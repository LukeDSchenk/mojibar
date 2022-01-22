use std::collections::HashMap;

use crate::emoji::Emoji;

// fix this nonsense
enum TrieRef<'a> {
    Node(&'a Node<'a>),
    Trie(&'a SearchTrie<'a>),
}

/// A SearchTrie node; each containing a letter, a hashmap of pointers to children, and an optional vec of emoji data.
#[derive(Clone)]
pub struct Node<'a> {
    letter: char,
    children: HashMap<char, Node<'a>>,
    data: Option<Vec<&'a Emoji>>,
}

/// A searchable trie-like data structure.
/// In practice, this is basically just a HashSet containing each of the root nodes.
pub struct SearchTrie<'a> {
    children: HashMap<char, Node<'a>>,
}

impl<'a> SearchTrie<'a> {
    pub fn new() -> SearchTrie<'a> {
        SearchTrie {
            children: HashMap::new(),
        }
    }

    pub fn add_keyword<'b>(&self, word: &str, emoji: &Emoji) {
        let mut pointer = TrieRef::Trie(&self);
        let chars = word.chars();
        let len = chars.clone().count();
        for (i, l) in chars.enumerate() {
            let mut p = match pointer {
                TrieRef::Node(mut x) => x,
                TrieRef::Trie(_) => {
                    let n: Node = Node {
                        letter: '0',
                        children: self.children.clone(),
                        data: None,
                    };
                    &n
                },
            };

            // check if this is the last letter in the word
            let mut d: Option<&Emoji> = None;
            if i == (len-1) {
                d = Some(emoji);
            } else {
                d = None;
            }

            if !p.children.contains_key(&l) {
                let new_node = Node {
                    letter: l,
                    children: HashMap::new(),
                    data: match d {
                        Some(data) => Some(vec![data]),
                        None => None,
                    }
                };
                p.children.insert(l, new_node);
            } else {
                match d {
                    Some(data) => {
                        match &p.children[&l].data {
                            Some(mut emojis) => { &emojis.push(data); },
                            None => { p.children[&l].data = Some(vec![data]); }
                        };
                    },
                    None => {},
                }
            }
            pointer = TrieRef::Node(&p.children[&l]);
        }
    }

    /// Search for the given keyword and return any associated emojis.
    pub fn keyword_search(&self, word: &str) -> Option<&Vec<&Emoji>> {
        let mut pointer = TrieRef::Trie(&self);
        for l in word.chars() {
            let p = match pointer {
                TrieRef::Node(x) => x,
                TrieRef::Trie(_) => {
                    &Node {
                        letter: '0',
                        children: self.children.clone(),
                        data: None,
                    }
                },
            };
            if p.children.contains_key(&l) {
                pointer = TrieRef::Node(&p.children[&l]);
            } else {
                return None
            }
        }
        match pointer {
            TrieRef::Node(node) => {
                match &node.data {
                    Some(emojis) => Some(&emojis),
                    None => None,
                }
            },
            TrieRef::Trie(_) => None,
        }
    }
}
