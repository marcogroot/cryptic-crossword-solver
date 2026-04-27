use itertools::Itertools;

fn main() {
    let clue = "wet items on ground".to_lowercase();
    let clue_arr: Vec<String> = clue.split(" ").map(|f| f.to_string()).collect();

    let solver = Solver::new(clue_arr, 7);
}

fn is_anagram_indicator(word: &str) -> bool {
    let anagram_indicators = vec!["ground"];
    anagram_indicators.contains(&word)
}

fn generate_valid_anagrams(letters: Vec<char>) -> Vec<String> {
    vec![]
}

fn get_dictionary_definition(word: &str) -> Vec<String> {
    vec![]
}

enum IndicatorType {
    Anagram,
}

fn get_indicator_types(word: &str) -> Vec<IndicatorType> {
    return vec![IndicatorType::Anagram];
}

fn is_past_tense(word: &str) -> bool {
    true
}

struct PotentialAnswer {
    answer: String,
    confidence: u8,
}

struct Solver {
    hint: Vec<String>,
    number_of_letters: u8,
    potential_answers: Vec<PotentialAnswer>,
}

enum Position {
    Continue,
}

impl Solver {
    fn new(hint: Vec<String>, number_of_letters: u8) -> Self {
        Solver {
            hint,
            number_of_letters,
            potential_answers: vec![],
        }
    }

    fn recursive_solve(
        &mut self,
        indicator: Option<IndicatorType>,
        position: Position,
        fodder: Vec<String>,
        current_index: usize,
        mut current_answer: Vec<char>,
    ) {
        if current_index == self.hint.len() {
            println!("Reached the end of the word");
            let answer: String = current_answer.iter().collect();
            println!("Answer is {answer}");
            if current_answer.contains(&'?') {
                println!("Invalid answer, not enough letters");
                return;
            }
            println!("Valid answer!");
            let potential_answer = PotentialAnswer {
                answer,
                confidence: 100,
            };
            &self.potential_answers.push(potential_answer);

            return;
        }

        let word = &self.hint[current_index].clone();
        let next_index = current_index + 1;
        println!("Current word {word}");

        // always try the path where this is not an indicator, and is fodder
        let mut new_fodder = fodder.clone();
        new_fodder.push(word.clone());
        self.recursive_solve(None, position, new_fodder, next_index, current_answer);

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
                        let anagrams = get_anagrams(letters);
                        for anagram in anagrams {
                            let updated_answer =
                                updated_answer(&current_answer, &letters, &position);
                            self.recursive_solve(
                                None,
                                position,
                                new_fodder,
                                next_index,
                                updated_answer,
                            );
                        }
                    } else {
                        println!("todo: implement future indicator flow")
                    }
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
    let mut new_answer = current_answer.clone();
    match position {
        Position::Continue => {
            let start_index = current_answer.iter().position(|c| *c == '?');
            let a: String = current_answer.iter().collect();
            let b: String = letters.iter().collect();
            match start_index {
                Some(s) => {
                    for (i, letter) in letters.iter().enumerate() {
                        new_answer[s + i] = *letter;
                    }
                    return new_answer;
                }
                None => {
                    panic!("Could not fit {a} into {b}");
                }
            }
        }
    }
}

fn is_dictionary_word(word: &str) -> bool {
    webster::dictionary(word).is_some()
}

fn get_anagrams(letters: Vec<char>) -> Vec<String> {
    let anagrams = letters
        .iter()
        .permutations(letters.len())
        .filter_map(|perm| {
            let word: String = perm.into_iter().collect();
            is_dictionary_word(&word).then(|| {
                println!("Anagram: {word}");
                word
            })
        })
        .collect();
    anagrams
}
