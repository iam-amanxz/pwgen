use rand::prelude::SliceRandom;
use rand::Rng;

const MIN_LENGTH: usize = 8;
const MAX_LENGTH: usize = 128;

fn main() {
    let g = Generator::new(Options {
        length: 8,
        allow_symbols: true,
        allow_numbers: true,
        allow_uppercase: true,
        allow_lowercase: true,
    })
    .expect("Failed to create generator");

    let result = g.generate();

    println!("{}", result);
}

struct State {
    length: usize,
    symbols: Vec<char>,
    numbers: Vec<char>,
    uppercase: Vec<char>,
    lowercase: Vec<char>,
    chars: Vec<char>,
}

struct Options {
    length: usize,
    allow_symbols: bool,
    allow_numbers: bool,
    allow_uppercase: bool,
    allow_lowercase: bool,
}

struct Generator {
    state: State,
}

impl Generator {
    fn new(options: Options) -> Result<Generator, String> {
        if options.length < MIN_LENGTH || options.length > MAX_LENGTH {
            return Err(format!(
                "Length must be between {} and {}.",
                MIN_LENGTH, MAX_LENGTH
            ));
        }

        if !options.allow_symbols
            && !options.allow_numbers
            && !options.allow_uppercase
            && !options.allow_lowercase
        {
            return Err(
                "At least one of symbols, numbers, uppercase, or lowercase must be set to true."
                    .to_string(),
            );
        }

        let mut chars = Vec::new();
        let symbols: Vec<char> = "!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~".chars().collect();
        let numbers: Vec<char> = "0123456789".chars().collect();
        let uppercase: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();
        let lowercase: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();

        if options.allow_symbols {
            chars.extend_from_slice(&symbols);
        }
        if options.allow_numbers {
            chars.extend_from_slice(&numbers);
        }
        if options.allow_uppercase {
            chars.extend_from_slice(&uppercase);
        }
        if options.allow_lowercase {
            chars.extend_from_slice(&lowercase);
        }

        Ok(Generator {
            state: State {
                length: options.length,
                symbols,
                numbers,
                uppercase,
                lowercase,
                chars,
            },
        })
    }

    fn get_random_string(&self, base: &mut Vec<char>) -> String {
        let mut rng = rand::thread_rng();
        while base.len() < self.state.length {
            base.push(self.state.chars[rng.gen_range(0..self.state.chars.len())]);
        }
        base.shuffle(&mut rng); // Randomize the positions of characters
        base.iter().collect()
    }

    fn generate(&self) -> String {
        let mut base: Vec<char> = Vec::new();

        // Ensure at least one char from each selected category is included
        let mut rng = rand::thread_rng();
        if !self.state.symbols.is_empty() {
            base.push(self.state.symbols[rng.gen_range(0..self.state.symbols.len())]);
        }
        if !self.state.numbers.is_empty() {
            base.push(self.state.numbers[rng.gen_range(0..self.state.numbers.len())]);
        }
        if !self.state.uppercase.is_empty() {
            base.push(self.state.uppercase[rng.gen_range(0..self.state.uppercase.len())]);
        }
        if !self.state.lowercase.is_empty() {
            base.push(self.state.lowercase[rng.gen_range(0..self.state.lowercase.len())]);
        }

        // Generate the rest of the characters
        self.get_random_string(&mut base)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length_out_of_range() {
        let options = Options {
            length: 7,
            allow_symbols: true,
            allow_numbers: true,
            allow_uppercase: true,
            allow_lowercase: true,
        };
        let result = Generator::new(options);
        assert!(result.is_err());
    }

    #[test]
    fn test_no_categories_selected() {
        let options = Options {
            length: 8,
            allow_symbols: false,
            allow_numbers: false,
            allow_uppercase: false,
            allow_lowercase: false,
        };
        let result = Generator::new(options);
        assert!(result.is_err());
    }

    #[test]
    fn test_generate() {
        let g = Generator::new(Options {
            length: 18,
            allow_symbols: true,
            allow_numbers: true,
            allow_uppercase: true,
            allow_lowercase: true,
        })
        .expect("Failed to create generator");

        let result = g.generate();
        assert_eq!(result.len(), 18);
    }
}

