use itertools::Itertools;
use std::collections::HashSet;
use std::rc::Rc;

#[derive(Clone)]
struct Node {
    indicator: Indicator,
    value: String,
    parent: Option<Rc<Node>>,
}

struct RootNode {
    definition: String,
    root: Node,
}

#[derive(Clone, Debug, Copy)]
enum Indicator {
    Anagram,
    Reverse,
    Hidden,
    Fodder,
}

fn recursive_parse_node(node: Option<Rc<Node>>, result: &mut Vec<Rc<Node>>) {
    match node {
        Some(node) => {
            result.push(node.clone());
            recursive_parse_node(node.parent.clone(), result)
        }
        None => {
            println!("Collected {} results", result.len());
            return;
        }
    }
}

fn parse_leaf_node(leaf_node: Rc<Node>) {
    let mut res = vec![];
    recursive_parse_node(Some(leaf_node), &mut res);
    res.iter().rev().for_each(|node| {
        println!("{}, {:?}", node.value, node.indicator);
    });
}

fn recursive_solve(
    word_play: &Vec<String>,
    root_node: &RootNode,
    current_index: usize,
    previous_node: Option<Rc<Node>>,
    leaf_nodes: &mut Vec<Rc<Node>>,
    dictionary: &Dictionary,
) {
    if current_index == word_play.len() {
        match previous_node {
            Some(node) => {
                leaf_nodes.push(node);
            }
            None => {
                panic!("Previous node is missing");
            }
        }
        return;
    }

    let word = &word_play[current_index].clone();
    let next_index = current_index + 1;

    if dictionary.is_anagram_indicator(word) {
        let anagram_node = Some(Rc::new(Node {
            indicator: Indicator::Anagram,
            value: word.clone(),
            parent: previous_node.clone(),
        }));
        recursive_solve(
            word_play,
            root_node,
            next_index,
            anagram_node,
            leaf_nodes,
            dictionary,
        );
    }

    let fodder_node = Some(Rc::new(Node {
        indicator: Indicator::Fodder,
        value: word.clone(),
        parent: previous_node.clone(),
    }));

    recursive_solve(
        word_play,
        root_node,
        next_index,
        fodder_node,
        leaf_nodes,
        dictionary,
    );
}

fn get_anagrams(dictionary: &Dictionary, letters: Vec<char>) -> Vec<String> {
    let anagrams = letters
        .iter()
        .permutations(letters.len())
        .unique()
        .filter_map(|perm| {
            let word: String = perm.into_iter().collect();
            let res = dictionary.contains(&word);
            res.then(|| word)
        })
        .collect();
    anagrams
}

fn get_indicator_types(dictionary: &Dictionary, word: &str) -> Vec<Indicator> {
    let mut indicator_types = vec![];

    if dictionary.anagram_indicators.contains(word) {
        indicator_types.push(Indicator::Anagram);
    }

    if dictionary.reverse_indicators.contains(word) {
        indicator_types.push(Indicator::Reverse);
    }

    if dictionary.inside_indicators.contains(word) {
        indicator_types.push(Indicator::Hidden);
    }

    return vec![Indicator::Anagram];
}

struct Dictionary {
    words: HashSet<String>,
    anagram_indicators: HashSet<String>,
    inside_indicators: HashSet<String>,
    reverse_indicators: HashSet<String>,
}

impl Dictionary {
    fn new() -> Self {
        let words: HashSet<String> = include_str!("../words.txt")
            .lines()
            .map(|l| l.to_lowercase())
            .collect();

        let anagram_indicators: HashSet<String> = include_str!("../anagram_indicators.txt")
            .lines()
            .map(|l| l.to_lowercase())
            .collect();

        let inside_indicators: HashSet<String> = include_str!("../inside_indicators.txt")
            .lines()
            .map(|l| l.to_lowercase())
            .collect();

        let reverse_indicators: HashSet<String> = include_str!("../reverse_indicators.txt")
            .lines()
            .map(|l| l.to_lowercase())
            .collect();

        Dictionary {
            words,
            anagram_indicators,
            inside_indicators,
            reverse_indicators,
        }
    }

    fn contains(&self, word: &str) -> bool {
        return self.words.contains(word);
    }

    fn is_anagram_indicator(&self, word: &str) -> bool {
        self.anagram_indicators.contains(word)
    }
    // a
    // abc
}
