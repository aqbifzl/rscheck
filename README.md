# rscheck

🚀 rscheck - blazingly fast spell checker written in rust 🚀

rscheck is a tool designed to empower you to catch and correct spelling errors in any document or codebase. rscheck has 2 built-in functions to parse keywords in PascalCase, camelCase, snake_case and MACRO_CASE

Key Features:

🦀 Written in rust

## Usage
```
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
```
## Example
```
rscheck -t file.txt -t dir/ -w wordlist.txt -w wordlist2.txt -i to_be_ignored.txt -e rs -e cpp -xp dir/subdir/
```
