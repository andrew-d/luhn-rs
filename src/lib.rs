/// ## Luhn
///
/// This create contains an implementation of the [Luhn checksum
/// algorithm](https://en.wikipedia.org/wiki/Luhn_mod_N_algorithm).  For more
/// information, see the documentation on the `Luhn` type.
use std::collections::HashSet;
use std::convert::AsRef;


/// The error type for this crate.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LuhnError {
    /// The given alphabet has a duplicated character.
    NotUnique(char),

    /// The input string has a character that is invalid for the alphabet.
    InvalidCharacter(char),

    /// The input was the empty string or a single character.
    EmptyString,
}

/// Luhn represents a thing that can generate or validate the Luhn character for
/// a given input.
#[derive(Debug)]
pub struct Luhn {
    alphabet: Vec<char>,
}

impl Luhn {
    /// Create a new Luhn instance from anything that can be coerced to a
    /// `&str`.
    pub fn new<S>(alphabet: S) -> Result<Luhn, LuhnError>
        where S: AsRef<str>
    {
        let mut chars = alphabet.as_ref().chars().collect::<Vec<char>>();
        if chars.len() < 1 {
            return Err(LuhnError::EmptyString);
        }

        // Need to sort so binary_search works.
        chars.sort();

        // Validate uniqueness
        let mut charset = HashSet::new();
        for ch in chars.iter() {
            if charset.contains(ch) {
                return Err(LuhnError::NotUnique(*ch));
            }

            charset.insert(*ch);
        }

        Ok(Luhn { alphabet: chars })
    }

    #[inline]
    fn codepoint_from_character(&self, ch: char) -> Result<usize, LuhnError> {
        match self.alphabet.binary_search(&ch) {
            Ok(idx) => Ok(idx),
            Err(_) => Err(LuhnError::InvalidCharacter(ch)),
        }
    }

    #[inline]
    fn character_from_codepoint(&self, cp: usize) -> char {
        self.alphabet[cp]
    }

    /// Given an input string, generate the Luhn character.
    ///
    /// Returns an error if the input string is empty, or contains a character
    /// that is not in the input alphabet.
    pub fn generate<S>(&self, s: S) -> Result<char, LuhnError>
        where S: AsRef<str>
    {
        let s = s.as_ref();
        if s.len() == 0 {
            return Err(LuhnError::EmptyString);
        }

        let mut factor = 1;
        let mut sum = 0;
        let n = self.alphabet.len();

        // Note: this is by-and-large a transliteration of the algorithm in the
        // Wikipedia article into Rust:
        //   https://en.wikipedia.org/wiki/Luhn_mod_N_algorithm
        for ch in s.chars() {
            let codepoint = try!(self.codepoint_from_character(ch));

            let mut addend = factor * codepoint;
            factor = if factor == 2 {
                1
            } else {
                2
            };
            addend = (addend / n) + (addend % n);
            sum += addend;
        }

        let remainder = sum % n;
        let check_codepoint = (n - remainder) % n;

        Ok(self.character_from_codepoint(check_codepoint))
    }

    /// Validates a Luhn check character.  This assumes that the final character
    /// of the input string is the Luhn character, and it will validate that the
    /// remainder of the string is correct.
    pub fn validate<S>(&self, s: S) -> Result<bool, LuhnError>
        where S: AsRef<str>
    {
        let s = s.as_ref();
        if s.len() <= 1 {
            return Err(LuhnError::EmptyString);
        }

        // Extract the check character and remainder of the string.
        // TODO: can we do this without allocating a new String?
        let head = s.char_indices()
                    .take_while(|&(index, _)| index < s.len() - 1)
                    .map(|(_, ch)| ch)
                    .collect::<String>();
        let luhn = s.chars().last().unwrap();

        let expected = try!(self.generate(head));
        Ok(luhn == expected)
    }

    /// Validates a Luhn check character.  This is the same as the `validate`
    /// method, but allows providing the Luhn check character out-of-band from
    /// the input to validate.
    pub fn validate_with<S>(&self, s: S, check: char) -> Result<bool, LuhnError>
        where S: AsRef<str>
    {
        let s = s.as_ref();
        if s.len() <= 1 {
            return Err(LuhnError::EmptyString);
        }

        let expected = try!(self.generate(s));
        Ok(check == expected)
    }
}


#[cfg(test)]
mod tests {
    extern crate rand;

    use self::rand::{Isaac64Rng, Rng, SeedableRng, sample, thread_rng};

    use super::{Luhn, LuhnError};

    #[test]
    fn test_generate() {
        // Base 6
        let l = Luhn::new("abcdef").ok().expect("valid alphabet");

        match l.generate("abcdef") {
            Ok(ch) => assert_eq!(ch, 'e'),
            Err(e) => panic!("unexpected generate error: {:?}", e),
        };

        let l = Luhn::new("0123456789").ok().expect("valid alphabet");

        match l.generate("7992739871") {
            Ok(ch) => assert_eq!(ch, '3'),
            Err(e) => panic!("unexpected generate error: {:?}", e),
        };
    }

    #[test]
    fn test_invalid_alphabet() {
        match Luhn::new("abcdea") {
            Ok(_) => panic!("unexpected success"),
            Err(e) => assert_eq!(e, LuhnError::NotUnique('a')),
        };
    }

    #[test]
    fn test_invalid_input() {
        let l = Luhn::new("abcdef").ok().expect("valid alphabet");

        match l.generate("012345") {
            Ok(_) => panic!("unexpected success"),
            Err(e) => assert_eq!(e, LuhnError::InvalidCharacter('0')),
        };
    }

    #[test]
    fn test_validate() {
        let l = Luhn::new("abcdef").ok().expect("valid alphabet");

        assert!(l.validate("abcdefe").unwrap());
        assert!(!l.validate("abcdefd").unwrap());
    }

    #[test]
    fn test_empty_strings() {
        // Alphabet must have at least one character.
        assert_eq!(Luhn::new("").unwrap_err(), LuhnError::EmptyString);

        let l = Luhn::new("abcdef").ok().expect("valid alphabet");

        // Cannot generate on an empty string.
        assert_eq!(l.generate("").unwrap_err(), LuhnError::EmptyString);

        // Cannot validate a string of length 1 (since the last character is the check digit).
        assert_eq!(l.validate("a").unwrap_err(), LuhnError::EmptyString);
    }

    #[test]
    fn test_validate_with() {
        let l = Luhn::new("abcdef").ok().expect("valid alphabet");

        assert!(l.validate_with("abcdef", 'e').unwrap());
        assert!(!l.validate_with("abcdef", 'd').unwrap());
    }

    #[test]
    fn test_longer_input() {
        // This test caught an out-of-bounds error.
        let l = Luhn::new("abcdef").ok().expect("valid alphabet");
        let _ = l.generate("aabbccdd");
    }

    #[test]
    fn test_random_input() {
        const NUM_TESTS: usize = 10000;
        const PRINTABLE: &'static str = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTU\
                                         VWXYZ";
        let printable_chars = PRINTABLE.chars().collect::<Vec<char>>();

        // Generate a random seed and print it
        let seed: u64 = thread_rng().gen();
        println!("Seed for this run: {}", seed);

        // Create the seedable RNG with this seed.
        let mut rng = Isaac64Rng::from_seed(&[seed]);

        for i in 1..NUM_TESTS {
            // Generate a random alphabet size
            let alphabet_size: u8 = rng.gen_range(1, printable_chars.len() as u8);

            // Create the alphabet by taking this many characters from our
            // printable characters Vec.
            let chars = sample(&mut rng, &printable_chars, alphabet_size as usize)
                            .into_iter()
                            .cloned()
                            .collect::<Vec<char>>();
            let alphabet = chars.iter().cloned().collect::<String>();

            // Generate a random input length.
            let input_length: u16 = rng.gen_range(1, 1024);

            // Generate this many random characters.
            let input = (0..input_length)
                            .map(|_| *rng.choose(&*chars).unwrap())
                            .collect::<String>();

            // Validate that this succeeds.
            let l = Luhn::new(&alphabet).expect("invalid alphabet given");
            if let Err(e) = l.generate(&input) {
                println!("Alphabet = {}", alphabet);
                println!("Input = {}", input);
                panic!("{}: Unexpected error: {:?}", i, e);
            }
        }
    }
}
