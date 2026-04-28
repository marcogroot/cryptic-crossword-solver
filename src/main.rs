use itertools::Itertools;
use std::collections::HashSet;

fn main() {
    let clue = "wet items on ground".to_lowercase();
    let clue_arr: Vec<String> = clue.split(" ").map(|f| f.to_string()).collect();

    let dictionary = Dictionary::new();
    let mut solver = Solver::new(clue_arr, 7);
    let answers = solver.solve(&dictionary);

    for answer in answers {
        println!("{:?}", answer);
    }
}

#[derive(Debug)]
#[allow(dead_code)]
struct ValidAnswer {
    answer: String,
    confidence: f64,
}

struct Dictionary {
    words: HashSet<String>,
}

impl Dictionary {
    fn new() -> Self {
        let words: HashSet<String> = include_str!("../words.txt")
            .lines()
            .map(|l| l.to_lowercase())
            .collect();

        Dictionary { words }
    }

    fn contains(&self, word: &str) -> bool {
        return self.words.contains(word);
    }
}

#[derive(Debug)]
enum IndicatorType {
    Anagram,
}

fn get_indicator_types(_word: &str) -> Vec<IndicatorType> {
    return vec![IndicatorType::Anagram];
}

fn is_past_tense(_word: &str) -> bool {
    true
}

struct Solver {
    clue: Vec<String>,
    number_of_letters: usize,
}

enum Position {
    Continue,
}

impl Solver {
    fn new(hint: Vec<String>, number_of_letters: usize) -> Self {
        Solver {
            clue: hint,
            number_of_letters,
        }
    }

    fn solve(&mut self, dictionary: &Dictionary) -> Vec<ValidAnswer> {
        let mut answers = vec![];

        // TODO: I am assuming that the definition is either the first or last word... fix this

        // definition is first word case
        let clue_len = &self.clue.len();
        let definition = &self
            .clue
            .first()
            .expect("There should be atleast one word in the clue");

        let word_play = &self.clue[1..].to_vec();

        recursive_solve(
            word_play,
            definition,
            self.number_of_letters,
            None,
            Position::Continue,
            vec![],
            0,
            vec![],
            &mut answers,
            dictionary,
        );

        let definition = &self
            .clue
            .last()
            .expect("There should be atleast one word in the clue");

        let word_play = &self.clue[..clue_len - 1].to_vec();

        recursive_solve(
            word_play,
            definition,
            self.number_of_letters,
            None,
            Position::Continue,
            vec![],
            0,
            vec![],
            &mut answers,
            dictionary,
        );

        let python_args: String = answers
            .iter()
            .map(|a| format!("{},{};", a.definition, a.answer))
            .collect();

        let output = std::process::Command::new("python3")
            .args(["similarity.py", &python_args])
            .output();

        let scores: Vec<f64> = match output {
            Ok(res) => {
                let text = String::from_utf8_lossy(&res.stdout);
                text.lines()
                    .map(|line| line.trim().parse().unwrap())
                    .collect()
            }
            Err(e) => {
                panic!("Womp womp {e}");
            }
        };

        let filter_cutoff = 0.7;
        let valid_answers = answers
            .iter()
            .zip(scores)
            .filter(|it| it.1 >= filter_cutoff)
            .map(|f| ValidAnswer {
                answer: f.0.answer.to_string(),
                confidence: f.1,
            })
            .collect();

        valid_answers
    }
}

#[derive(Debug)]
struct PotentialAnswer {
    definition: String,
    answer: String,
}

fn recursive_solve(
    word_play: &Vec<String>,
    definition: &str,
    answer_length: usize,
    _indicator: Option<IndicatorType>,
    position: Position,
    fodder: Vec<String>,
    current_index: usize,
    current_answer: Vec<char>,
    answers: &mut Vec<PotentialAnswer>,
    dictionary: &Dictionary,
) {
    if current_index == word_play.len() {
        let answer: String = current_answer.iter().collect();
        if current_answer.len() != answer_length {
            return;
        }
        let potential_answer = PotentialAnswer {
            definition: definition.to_string(),
            answer,
        };
        answers.push(potential_answer);

        return;
    }

    let word = &word_play[current_index].clone();
    let next_index = current_index + 1;

    // always try the path where this is not an indicator, and is fodder
    let mut new_fodder = fodder.clone();
    new_fodder.push(word.clone());
    recursive_solve(
        word_play,
        definition,
        answer_length,
        None,
        Position::Continue,
        new_fodder,
        next_index,
        current_answer.clone(),
        answers,
        dictionary,
    );

    let indicators = get_indicator_types(word);
    let past_tense = is_past_tense(word);
    for indicator_type in indicators {
        match indicator_type {
            IndicatorType::Anagram => {
                if past_tense {
                    let mut letters: Vec<char> = vec![];
                    fodder.iter().for_each(|f| {
                        f.chars().for_each(|c| letters.push(c));
                    });
                    let anagrams = get_anagrams(dictionary, letters);
                    for anagram in anagrams {
                        let letters: Vec<char> = anagram.chars().into_iter().collect();
                        let updated_answer = updated_answer(&current_answer, &letters, &position);

                        recursive_solve(
                            word_play,
                            definition,
                            answer_length,
                            None,
                            Position::Continue,
                            vec![],
                            next_index,
                            updated_answer,
                            answers,
                            dictionary,
                        );
                    }
                } else {
                    println!("todo: implement future indicator flow")
                }
            }
        }
    }
}

fn updated_answer(
    current_answer: &Vec<char>,
    letters: &Vec<char>,
    position: &Position,
) -> Vec<char> {
    match position {
        Position::Continue => {
            let mut updated_answer = current_answer.clone();
            for letter in letters {
                updated_answer.push(*letter);
            }
            return updated_answer;
        }
    }
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
