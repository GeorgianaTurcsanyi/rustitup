use rand::rng;
use rand::seq::IndexedRandom;
use std::{fmt, io};
use std::collections::HashSet;
use std::fmt::{ Formatter};

const COLOURS: [&str; 6] = ["Y", "B", "R", "P", "G", "W"];

fn get_secret() -> Vec<&'static str> {
    let mut rng = rng();
    let secret= COLOURS.choose_multiple(&mut rng, 4).copied().collect();
    secret
}

pub enum InputError {
    NotEnoughColours(usize, usize),
    IllegalChoice(String),
    ChoicesShouldBeUnique()
}

impl fmt::Display for InputError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            InputError::NotEnoughColours(given, expected) => write!(f,
            "Incorrect number of chosen colours, given {}, need {}", given, expected),
            InputError::IllegalChoice(choice) => write!(f,
            "Incorrect choice {}, please use any of {:?}", choice, COLOURS),
            InputError::ChoicesShouldBeUnique() => write!(f,
            "Choose unique colours please")
        }
    }
}

fn is_unique_arr<T>(vec: &[T]) -> bool where
T: Eq + std::hash::Hash
{
    let set: HashSet<&T> = vec.iter().collect();
    vec.len() == set.len()
}

fn validate_input(input: String) -> Result<Vec<String>, InputError> {
    let cleaned_input = input.trim_end();
    let parts: Vec<String> = cleaned_input.split(' ').map(str::to_string).collect();

    if parts.len() != 4 {
        return Err(InputError::NotEnoughColours(parts.len(), 4));
    }

    for part in &parts {
        if !COLOURS.contains(&part.to_uppercase().as_str())  {
            return Err(InputError::IllegalChoice(part.to_string()))
        }
    }

    if !is_unique_arr(&parts) {
        return Err(InputError::ChoicesShouldBeUnique())
    }

    Ok(parts)
}

fn check_guess(guess: &[String], secret: &[&str]) -> (i32, i32) {
    let mut is_match: i32 = 0;
    let mut is_match_position: i32 = 0;
    for (i, g) in guess.iter().enumerate() {
        for (j, s) in secret.iter().enumerate() {
            if g.to_uppercase() == *s {
                if i == j {
                    is_match_position += 1;
                } else {
                    is_match += 1;
                }
            }
        }
    }
    (is_match, is_match_position)
}

fn main() {
    let secret = get_secret();
    // println!("{:?}",&secret);

    let mut i = 0;
    loop {
        println!("Choose 4 colours from {:?}, separated by space", COLOURS);
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_n) => {
            }
            Err(error) => println!("error: {error}"),
        }

        match validate_input(input) {
            Ok(validate_input ) => {
                let (is_match, is_match_position) = check_guess(&validate_input, &secret);
                if is_match_position == 4 {
                    println!("YOU WIN!");
                    break;
                }
                println!("you matched {} colours, and matched {} colours in the correct position", is_match, is_match_position);
                i += 1;
                if i == 0 {
                    println!("YOU LOOSE!");
                    break;
                }
            }
            Err(error) => println!("error: {error}"),
        }
    }
}
