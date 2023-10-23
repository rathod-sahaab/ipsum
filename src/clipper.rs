use core::{
    iter::Iterator,
    option::Option,
    option::Option::{None, Some},
};

struct WordsClipper {
    words: &'static str,
}

struct Words {
    words: &'static str,
    index: usize,
    formed_words: i32,
    words_count: i32,
}

impl WordsClipper {
    fn new(words: &'static str) -> WordsClipper {
        WordsClipper { words }
    }

    // TODO: Trailing space feature on last formed word
    fn words(&self, words: i32) -> Words {
        Words {
            words: self.words,
            index: 0,
            formed_words: 0,
            words_count: words,
        }
    }
}

impl Iterator for Words {
    type Item = &'static str;

    // TODO: wrap around and repeat
    fn next(&mut self) -> Option<Self::Item> {
        if self.formed_words >= self.words_count || self.index >= self.words.len() {
            return None;
        }

        let word_start = self.index;
        loop {
            // collects a word and breaks or loops around if only whitespace
            let local_word_start = self.index;
            loop {
                // loop to collect a word

                let mut point = self.index;
                while point < self.words.len() {
                    // collects UTF-8 code points of a character
                    point += 1;

                    if self.words.is_char_boundary(point) {
                        break;
                    }
                }

                // got a UTF-8 character, checking if it is a whitespace
                if self.words[self.index..point]
                    .chars()
                    .all(|ch| ch.is_whitespace())
                {
                    // found a whitespace, forming a word
                    self.formed_words += 1;
                    // leaving, so updating index
                    self.index = point;
                    break;
                }

                // saw till this point in our string so updating index
                self.index = point;

                if self.index >= self.words.len() {
                    break;
                }
            }

            if self.words[local_word_start..self.index]
                .chars()
                .all(|c| c.is_whitespace())
            {
                continue;
            } else {
                break;
            }
        }

        return Some(&self.words[word_start..self.index]);
    }
}

#[cfg(test)]
mod tests {
    use core::assert_eq;
    use core::prelude::rust_2021::test_case;
    use core::{
        iter::Iterator,
        option::Option::{None, Some},
    };

    #[test_case]
    fn test_words_clipper() {
        let clipper = super::WordsClipper::new("Hello World");
        let mut words = clipper.words(2);
        assert_eq!(words.next(), Some("Hello "));
        assert_eq!(words.next(), Some("World"));
        assert_eq!(words.next(), None);
    }

    #[test_case]
    fn test_punctuation() {
        let clipper = super::WordsClipper::new("Hello World, this is. a sample and a test");
        let mut words = clipper.words(8);
        assert_eq!(words.next(), Some("Hello "));
        assert_eq!(words.next(), Some("World, "));
        assert_eq!(words.next(), Some("this "));
        assert_eq!(words.next(), Some("is. "));
    }

    #[test_case]
    fn test_reusability() {
        let clipper = super::WordsClipper::new("Hello World this is a sample and a test.");
        let mut words = clipper.words(8);
        assert_eq!(words.next(), Some("Hello "));
        assert_eq!(words.next(), Some("World "));
        assert_eq!(words.next(), Some("this "));
        assert_eq!(words.next(), Some("is "));
        let mut words = clipper.words(2);
        assert_eq!(words.next(), Some("Hello "));
        assert_eq!(words.next(), Some("World "));
        assert_eq!(words.next(), None);
    }

    #[test_case]
    fn test_refrence_preservation() {
        let clipper = super::WordsClipper::new("Hello World this is a sample and a test.");
        let mut words = clipper.words(8);
        let reference = words.next();
        let mut words = clipper.words(2);
        let another_reference = words.next();

        assert_eq!(reference, another_reference);
    }

    #[test_case]
    fn test_newline() {
        let clipper = super::WordsClipper::new("Hello\nWorld");
        let mut words = clipper.words(2);
        assert_eq!(words.next(), Some("Hello\n"));
        assert_eq!(words.next(), Some("World"));
        assert_eq!(words.next(), None);
    }

    #[test_case]
    fn test_multiple_spaces() {
        let clipper = super::WordsClipper::new("Hello   World");
        let mut words = clipper.words(2);
        assert_eq!(words.next(), Some("Hello "));
        assert_eq!(words.next(), Some("  World"));
        assert_eq!(words.next(), None);
    }

    #[test_case]
    fn test_internationalization() {
        let clipper = super::WordsClipper::new("こんに ちは 世界");
        let mut words = clipper.words(2);
        assert_eq!(words.next(), Some("こんに "));
        assert_eq!(words.next(), Some("ちは "));
        assert_eq!(words.next(), None);
    }

    #[test_case]
    fn test_internationalization_hindi() {
        let clipper = super::WordsClipper::new("नमस्ते दुनिया");
        let mut words = clipper.words(2);
        assert_eq!(words.next(), Some("नमस्ते "));
        assert_eq!(words.next(), Some("दुनिया"));
        assert_eq!(words.next(), None);
    }

    #[test_case]
    fn test_internationalization_arabic() {
        // TODO: Fix this test
        let clipper = super::WordsClipper::new("مرحبا العالم");
        let mut words = clipper.words(2);
        assert_eq!(words.next(), Some("مرحبا "));
        assert_eq!(words.next(), Some("العالم"));
        assert_eq!(words.next(), None);
    }
}
