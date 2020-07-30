///! Randomize a string, while maintaining character classes.
use rand::rngs::ThreadRng;
use rand::{thread_rng, Rng};

/// Randomize a string, while preserving major character classes. eg
/// "123-abc_DEF" might go to "973-qox_NAP". It doesn't matter that the random
/// number generation isn't cryptographic quality - from a security perspective,
/// it could randomize everyting to '000-aaa_AAA" and that would be fine.
pub fn randomize(input: &str) -> String {
    let mut rng = thread_rng();
    let mut output = String::with_capacity(input.len());
    for c in input.chars() {
        let new_c = None // Just to align next few lines
            .or_else(|| try_range('0', '9', c, &mut rng))
            .or_else(|| try_range('a', 'z', c, &mut rng))
            .or_else(|| try_range('A', 'Z', c, &mut rng))
            .unwrap_or(c);
        output.push(new_c);
    }
    output
}

/// Replace a char with a random alternative if within the specified range.
fn try_range(low: char, high: char, c: char, rng: &mut ThreadRng) -> Option<char> {
    if c < low || c > high {
        None
    } else {
        let new_u = rng.gen_range(low as u8, high as u8 + 1);
        Some(new_u as char)
    }
}

/// Tests
#[cfg(test)]
mod tests {
    use super::randomize;
    #[test]
    fn negative() {
        let input = "-& @!_#";
        assert_eq!(input, randomize(input));
    }

    #[test]
    fn positive() {
        let input = "0123-4567-89ab-cdef-ghij-wxyz-ABCD-EFGH-WXYZ";
        let output = randomize(input);
        let mut changes = 0;
        let char_classes: &[fn(char) -> bool] = &[
            |x| x >= '0' && x <= '9',
            |x| x >= 'a' && x <= 'z',
            |x| x >= 'A' && x <= 'Z',
            |x| x == '-',
        ];

        for (c, d) in input.chars().zip(output.chars()) {
            // If the input char is in a character class, so is the output char.
            for char_class in char_classes {
                assert_eq!(char_class(c), char_class(d));
            }

            // Given that constraint, count up how much shuffling we've done.
            if c != d {
                changes += 1;
            }
        }
        assert!(changes > 25);
    }
}
