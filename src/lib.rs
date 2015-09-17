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
        let chars = alphabet.as_ref().chars().collect::<Vec<char>>();

        // Validate uniqueness
        let mut charset = HashSet::new();
        for ch in chars.iter() {
            if charset.contains(ch) {
                return Err(LuhnError::NotUnique(*ch));
            }

            charset.insert(*ch);
        }

        Ok(Luhn {
            alphabet: chars,
        })
    }

    #[inline]
    fn codepoint_from_character(&self, ch: char) -> Result<usize, LuhnError> {
        match self.alphabet.binary_search(&ch) {
            Ok(idx) => Ok(idx),
            Err(_)  => Err(LuhnError::InvalidCharacter(ch)),
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
        let n = s.len();

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
}


#[cfg(test)]
mod tests {
    use super::{Luhn, LuhnError};

    #[test]
    fn test_generate() {
        // Base 6
        let l = Luhn::new("abcdef").expect("valid alphabet");

        match l.generate("abcdef") {
            Ok(ch) => assert_eq!(ch, 'e'),
            Err(e) => panic!("unexpected generate error: {:?}", e),
        };

        let l = Luhn::new("0123456789").expect("valid alphabet");

        match l.generate("7992739871") {
            Ok(ch) => assert_eq!(ch, '3'),
            Err(e) => panic!("unexpected generate error: {:?}", e),
        };
    }

    #[test]
    fn test_invalid_alphabet() {
        match Luhn::new("abcdea") {
            Ok(_)  => panic!("unexpected success"),
            Err(e) => assert_eq!(e, LuhnError::NotUnique('a')),
        };
    }

    #[test]
    fn test_invalid_input() {
        let l = Luhn::new("abcdef").expect("valid alphabet");

        match l.generate("012345") {
            Ok(_)  => panic!("unexpected success"),
            Err(e) => assert_eq!(e, LuhnError::InvalidCharacter('0')),
        };
    }

    #[test]
    fn test_validate() {
        let l = Luhn::new("abcdef").expect("valid alphabet");

        assert!(l.validate("abcdefe").unwrap());
        assert!(! l.validate("abcdefd").unwrap());
    }
}
