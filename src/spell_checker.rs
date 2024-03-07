pub mod file_utils;
pub mod options;
pub mod parse_variables;
pub mod stats;

use crate::trie::Trie;
use file_utils::get_files;
use io::BufReader;
use std::fs::{canonicalize, File};
use std::io::{self, BufRead, ErrorKind};
use std::path::{Path, PathBuf};
use std::process;

use self::file_utils::{get_words_from_line, read_lines};
use self::options::Options;
use self::stats::CheckStats;

fn feed_trie(
    path: &Path,
    ignore_list: &Option<&mut Trie>,
    trie: &mut Trie,
    options: &Options,
) -> io::Result<()> {
    let lines = read_lines(path)?;

    match ignore_list {
        Some(ignore_list) => {
            for line in lines.map_while(Result::ok) {
                if line.len() < options.min.into()
                    || line.len() > options.max.into()
                    || ignore_list.search(&line)
                {
                    continue;
                }

                trie.insert(&line);
            }
        }
        None => {
            for line in lines.map_while(Result::ok) {
                if line.len() >= 3 {
                    trie.insert(&line);
                }
            }
        }
    }

    Ok(())
}

fn check_word(
    word: &str,
    trie: &Trie,
    ignore_list: &Trie,
    counter: &mut u64,
    options: &Options,
    line_num: &usize,
) {
    if !trie.search(word) && is_word_correct(word, ignore_list, options) {
        println!("  * {}:{}", word, line_num);
        *counter += 1;
    }
}

fn is_word_correct(word: &str, ignore_list: &Trie, options: &Options) -> bool {
    if word.len() < options.min.into()
        || word.len() > options.max.into()
        || ignore_list.search(word)
    {
        return false;
    }

    true
}

fn check_correctness(
    path: &Path,
    trie: &Trie,
    ignore_list: &Trie,
    options: &Options,
    stats: &mut CheckStats,
) -> Result<(), io::Error> {
    let file = File::open(path)?;
    let lines = BufReader::new(file).lines();

    let mut counter: u64 = 0;

    let parsing_functions = [
        parse_variables::parse_camel_case,
        parse_variables::parse_snake_case,
    ];

    for (num, line) in lines.map_while(Result::ok).enumerate() {
        let line: &str = line.as_str();
        let words = get_words_from_line(line);

        for word in words {
            let mut parsed = false;
            for parsing_func in parsing_functions {
                if let Some(parsed_words) = parsing_func(&word) {
                    for word in parsed_words {
                        check_word(&word, trie, ignore_list, &mut counter, options, &num);
                    }
                    parsed = true;
                }
            }

            if !parsed {
                check_word(
                    &word.to_lowercase(),
                    trie,
                    ignore_list,
                    &mut counter,
                    options,
                    &num,
                )
            }
        }
    }

    stats.typos_num += counter;

    Ok(())
}

fn handle_feed_trie(
    trie: &mut Trie,
    ignore_list: Option<&mut Trie>,
    files: &Vec<PathBuf>,
    options: &Options,
) {
    for wordlist in files {
        feed_trie(wordlist, &ignore_list, trie, options).unwrap_or_else(|error| {
            if error.kind() == ErrorKind::NotFound {
                println!(
                    "Error occurred reading {} file not found",
                    wordlist.display()
                );
            } else if error.kind() == ErrorKind::PermissionDenied {
                println!(
                    "Error occurred reading {} permission denied",
                    wordlist.display()
                );
            } else {
                println!("Unknow error occurred reading {}", wordlist.display());
            }
            process::exit(1);
        });
    }
}

fn handle_correctness_check(
    path: &Path,
    trie: &Trie,
    ignore_list: &Trie,
    stats: &mut CheckStats,
    options: &Options,
) {
    if let Err(error) = check_correctness(path, trie, ignore_list, options, stats) {
        match error.kind() {
            ErrorKind::PermissionDenied => {
                println!(
                    "Error occurred reading {} permission denied",
                    path.display()
                )
            }
            _ => println!("Unknown error occurred reading {}", path.display()),
        };
        stats.errors += 1;
    }
}

fn show_result(stats: &CheckStats) {
    println!("===SUCCESSFULLY FINISHED===");
    println!("->Files checked: {}", stats.files_checked);
    println!("->Dirs checked: {}", stats.dirs_checked);
    println!("->Typos found: {}", stats.typos_num);
    println!("->Errors: {}", stats.errors);
    println!("===THANKS FOR USING THIS SOFTWARE!===");
}

fn skip_file(file: &Path, options: &Options) -> Result<bool, io::Error> {
    let canonicalized_target = canonicalize(file)?;

    for path_to_exclude in options.paths_to_exclude.iter() {
        let canonicalized_file = canonicalize(path_to_exclude)?;
        if canonicalized_file == canonicalized_target {
            return Ok(true);
        }
    }

    let file_extension = file.extension();

    match file_extension {
        Some(extension) => {
            let extension = extension.to_string_lossy();
            if !options.extensions_to_exclude.is_empty()
                && options
                    .extensions_to_exclude
                    .contains(&extension.to_string())
            {
                return Ok(true);
            }

            if !options.extensions.is_empty()
                && !options.extensions.contains(&extension.to_string())
            {
                return Ok(true);
            }
        }
        None => {
            if !options.extensions.is_empty() {
                return Ok(true);
            }
        }
    }

    Ok(false)
}

pub fn spell_check(options: &Options) -> Result<(), io::Error> {
    let mut words_trie = Trie::new();
    let mut ignore_list = Trie::new();
    let mut stats = CheckStats::new();

    for target in &options.targets {
        handle_feed_trie(&mut ignore_list, None, &options.ignore, options);
        handle_feed_trie(
            &mut words_trie,
            Some(&mut ignore_list),
            &options.wordlists,
            options,
        );

        if target.is_file() {
            if skip_file(target, options)? {
                continue;
            }
            stats.files_checked += 1;
            handle_correctness_check(target, &words_trie, &ignore_list, &mut stats, options)
        } else if target.is_dir() {
            let files = get_files(target);
            for file in files.into_iter().filter_map(|x| x.ok()) {
                let file = file.path();

                if skip_file(file, options)? {
                    continue;
                }

                if file.is_file() {
                    println!("-> {}", file.display());
                    handle_correctness_check(file, &words_trie, &ignore_list, &mut stats, options);
                    stats.files_checked += 1;
                } else if file.is_dir() {
                    stats.dirs_checked += 1;
                }
                println!();
            }
        }
    }

    show_result(&stats);

    Ok(())
}
