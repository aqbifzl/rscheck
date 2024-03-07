use std::{collections::HashMap, env::Args, path::PathBuf};

pub struct Options {
    pub targets: Vec<PathBuf>,
    pub wordlists: Vec<PathBuf>,
    pub ignore: Vec<PathBuf>,
    pub paths_to_exclude: Vec<PathBuf>,
    pub extensions: Vec<String>,
    pub extensions_to_exclude: Vec<String>,
    pub min: u16,
    pub max: u16,
}

struct ArgsPair {
    long: &'static str,
    short: &'static str,
}

enum Arg {
    Single(&'static str),
    Pair(ArgsPair),
}

impl Arg {
    pub fn get(str: &str) -> Result<Self, String> {
        match str {
            "--min" => Ok(Arg::Single("--min")),
            "--max" => Ok(Arg::Single("--max")),
            "-t" => Ok(Arg::Pair(ArgsPair {
                long: "--target",
                short: "-t",
            })),
            "-w" => Ok(Arg::Pair(ArgsPair {
                long: "--wordlist",
                short: "-w",
            })),
            "-i" => Ok(Arg::Pair(ArgsPair {
                long: "--ignore",
                short: "-i",
            })),
            "-e" => Ok(Arg::Pair(ArgsPair {
                long: "--extension",
                short: "-e",
            })),
            "-xe" => Ok(Arg::Pair(ArgsPair {
                long: "--exclude-extension",
                short: "-xe",
            })),
            "-xp" => Ok(Arg::Pair(ArgsPair {
                long: "--exclude-path",
                short: "-xp",
            })),
            _ => {
                Err("you tried to get invalid arg name, misconfiguration in your code".to_string())
            }
        }
    }
}

fn is_arg_valid(str: &str) -> bool {
    Arg::get(str).is_ok()
}

fn handle_file_extension(
    arg: &str,
    options_hashmap: &HashMap<String, Vec<String>>,
    target_string: &mut Vec<String>,
) {
    if let Some(entries) = options_hashmap.get(arg) {
        for ext in entries {
            target_string.push(ext.to_string());
        }
    }
}

fn handle_file_extensions(
    arg: &Arg,
    options_hashmap: &HashMap<String, Vec<String>>,
    target_string: &mut Vec<String>,
) {
    match arg {
        Arg::Single(arg) => handle_file_extension(arg, options_hashmap, target_string),
        Arg::Pair(arg) => {
            handle_file_extension(arg.short, options_hashmap, target_string);
            handle_file_extension(arg.long, options_hashmap, target_string);
        }
    }
}

fn handle_path_buf_arg(
    arg: &str,
    options_hashmap: &HashMap<String, Vec<String>>,
    path_bufs: &mut Vec<PathBuf>,
) -> Result<(), String> {
    if let Some(entries) = options_hashmap.get(arg) {
        for entry in entries.iter() {
            let mut path_buf = PathBuf::new();
            path_buf.push(entry);
            if !path_buf.exists() {
                return Err(format!("path {} doesnt exist", entry));
            }

            path_bufs.push(path_buf);
        }
    }
    Ok(())
}

fn handle_path_buf_args(
    arg: &Arg,
    options_hashmap: &HashMap<String, Vec<String>>,
    path_bufs: &mut Vec<PathBuf>,
) -> Result<(), String> {
    match arg {
        Arg::Single(arg) => handle_path_buf_arg(arg, options_hashmap, path_bufs)?,
        Arg::Pair(arg) => {
            handle_path_buf_arg(arg.short, options_hashmap, path_bufs)?;
            handle_path_buf_arg(arg.long, options_hashmap, path_bufs)?;
        }
    }

    Ok(())
}

fn handle_int_arg(
    arg: &str,
    options_hashmap: &HashMap<String, Vec<String>>,
    target_int: &mut u16,
) -> Result<(), String> {
    if let Some(entries) = options_hashmap.get(arg) {
        let first_num_arg = &entries[0];
        match first_num_arg.trim().parse::<u16>() {
            Ok(arg) => {
                *target_int = arg;
                return Ok(());
            }
            Err(_) => {
                return Err(format!("error parsing {arg} ({first_num_arg}) to int"));
            }
        }
    }

    Ok(())
}

fn handle_int_args(
    arg: &Arg,
    options_hashmap: &HashMap<String, Vec<String>>,
    target_int: &mut u16,
) -> Result<(), String> {
    match arg {
        Arg::Single(arg) => handle_int_arg(arg, options_hashmap, target_int)?,
        Arg::Pair(arg) => {
            handle_int_arg(arg.short, options_hashmap, target_int)?;
            handle_int_arg(arg.long, options_hashmap, target_int)?;
        }
    }

    Ok(())
}

fn push_args_into_struct(
    options: &mut Options,
    options_hashmap: &mut HashMap<String, Vec<String>>,
) -> Result<(), String> {
    handle_path_buf_args(&Arg::get("-t")?, options_hashmap, &mut options.targets)?;
    handle_path_buf_args(&Arg::get("-w")?, options_hashmap, &mut options.wordlists)?;
    handle_path_buf_args(&Arg::get("-i")?, options_hashmap, &mut options.ignore)?;
    handle_path_buf_args(
        &Arg::get("-xp")?,
        options_hashmap,
        &mut options.paths_to_exclude,
    )?;

    handle_file_extensions(&Arg::get("-e")?, options_hashmap, &mut options.extensions);
    handle_file_extensions(
        &Arg::get("-xe")?,
        options_hashmap,
        &mut options.extensions_to_exclude,
    );

    handle_int_args(&Arg::get("--min")?, options_hashmap, &mut options.min)?;
    handle_int_args(&Arg::get("--max")?, options_hashmap, &mut options.max)?;

    Ok(())
}

pub fn get_options_with_argv(argv: Args) -> Result<Options, String> {
    let argv: Vec<String> = argv.collect();
    let argv_len = argv.len();
    if argv_len <= 1 || argv_len % 2 == 0 {
        return Err("number of arguments is invalid".to_string());
    }
    let mut options = Options::new();
    let mut options_hashmap: HashMap<String, Vec<String>> = HashMap::new();

    for i in (1..argv_len).step_by(2) {
        let arg = &argv[i];

        if !is_arg_valid(arg) {
            return Err("invalid argument provided".to_string());
        }

        options_hashmap
            .entry(arg.to_string())
            .or_default()
            .push(argv[i + 1].to_string());
    }

    push_args_into_struct(&mut options, &mut options_hashmap)?;

    if options.targets.is_empty() {
        return Err("no target was provided".to_string());
    }

    if options.wordlists.is_empty() {
        return Err("no wordlists was provided".to_string());
    }

    Ok(options)
}

impl Default for Options {
    fn default() -> Self {
        Self::new()
    }
}

impl Options {
    fn new() -> Self {
        Self {
            targets: Vec::new(),
            wordlists: Vec::new(),
            ignore: Vec::new(),
            paths_to_exclude: Vec::new(),
            extensions: Vec::new(),
            extensions_to_exclude: Vec::new(),
            min: 2,
            max: 20,
        }
    }
    pub fn create(argv: Args) -> Result<Options, String> {
        get_options_with_argv(argv)
    }
}

pub fn show_manual() {
    let msg = r###"===USAGE===
rscheck -t [target] -w [wordlist]
-t or --target - set a target file or directory
-w or --wordlist - set a wordlist with valid words (optional)
-i or --ignore - set wordlist of words to be ignored (optional)
-e or --extension - set extension to scan (optional)
-xe or --exclude-extension - exclude specific extension (optional)
-xp or --exclude-path - exclude specific directory or file (optional)
--min and --max - minimum and maximum length of word (optional)

Args can be combined like
rscheck -t file.txt -t dir/ -w wordlist.txt -w wordlist2.txt -i to_be_ignored.txt -e rs -e cpp -xp dir/subdir/

--min and --max are the only args that can be used only once"###;
    println!("{msg}");
}
