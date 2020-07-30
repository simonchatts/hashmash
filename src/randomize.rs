///! Randomize a string, while maintaining character classes.
use rand::{thread_rng, Rng};

/// Macro for internal use: test a particular character range, and if so,
/// randomly substitute and continue.
macro_rules! char_range {
    ($char_lo:literal, $char_hi: literal, $cvar:ident, $rng:ident, $output:ident) => {
        if $cvar >= $char_lo && $cvar <= $char_hi {
            let uvar = $rng.gen_range($char_lo as u8, $char_hi as u8 + 1);
            $output.push(uvar as char);
            continue;
        }
    };
}

/// Randomize a string, while preserving major character classes. eg
/// "123-abc_DEF" might go to "973-qox_NAP".
pub fn randomize(input: &str) -> String {
    let mut rng = thread_rng();
    let mut output = String::with_capacity(input.len());
    for c in input.chars() {
        char_range!('0', '9', c, rng, output);
        char_range!('a', 'z', c, rng, output);
        char_range!('A', 'Z', c, rng, output);
        output.push(c);
    }
    output
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
        let input = "0123-4567-89ab-cdef-ghij-wxyz-ABCD-EFGH_WXYZ";
        let output = randomize(input);
        let mut changes = 0;
        for (c, d) in input.chars().zip(output.chars()) {
            assert_eq!(c >= '0' && c <= '9', d >= '0' && d <= '9');
            assert_eq!(c >= 'a' && c <= 'z', d >= 'a' && d <= 'z');
            assert_eq!(c >= 'A' && c <= 'Z', d >= 'A' && d <= 'Z');
            assert_eq!(c == '-', d == '-');
            if c != d {
                changes += 1;
            }
        }
        assert!(changes > 25);
    }
}
