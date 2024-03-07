use std::collections::HashMap;

pub struct TrieNode {
    children: HashMap<char, TrieNode>,
    end: bool,
}

pub struct Trie {
    root: TrieNode,
}

impl Default for Trie {
    fn default() -> Self {
        Trie::new()
    }
}

impl Trie {
    pub fn new() -> Self {
        Self {
            root: TrieNode {
                children: HashMap::new(),
                end: false,
            },
        }
    }
    pub fn insert(&mut self, word: &str) {
        let mut node = &mut self.root;

        for ch in word.chars() {
            node = node.children.entry(ch).or_insert(TrieNode {
                children: HashMap::new(),
                end: false,
            });
        }
        node.end = true;
    }
    pub fn search(&self, word: &str) -> bool {
        let mut node = &self.root;

        for ch in word.chars() {
            match node.children.get(&ch) {
                Some(entry) => {
                    node = entry;
                }
                None => {
                    return false;
                }
            }
        }
        node.end
    }
}
