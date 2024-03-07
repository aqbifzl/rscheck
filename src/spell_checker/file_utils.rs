use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};
use walkdir::WalkDir;

fn is_char_valid(ch: &char) -> bool {
    ch.is_ascii_alphanumeric() || *ch == '_'
}

pub fn get_words_from_line(line: &str) -> Vec<String> {
    let mut word_vec: Vec<String> = Vec::new();
    let mut current_word = String::new();
    let mut in_word = false;
    let mut has_letter = false;

    for ch in line.chars() {
        if is_char_valid(&ch) {
            current_word.push(ch);
            in_word = true;
            if ch.is_alphabetic() {
                has_letter = true;
            }
        } else {
            if has_letter {
                word_vec.push(current_word.clone());
            }
            current_word.clear();
            in_word = false;
            has_letter = false;
        }
    }

    if in_word {
        word_vec.push(current_word.clone());
    }

    word_vec
}

pub fn get_files(path: &Path) -> WalkDir {
    let walk_dir: WalkDir = WalkDir::new(path);

    walk_dir
}

pub fn read_lines(filename: &Path) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
