/// Classify a word as hash-or-not. The word is expected to have been
/// pre-filtered to match the regex `[a-zA-Z0-9-]{8,}`, after which
/// we judge based on the prevalence of valid English letter trigrams.
pub fn is_hash(word: &str) -> bool {
    let mut num_digits = 0;
    let mut num_triples = 0;
    let mut num_letter_triples = 0;
    let mut num_trigrams = 0;

    // Go through each triple of characters, collecting some basic stats
    for (c0, (c1, c2)) in word
        .chars()
        .zip(word.chars().skip(1).zip(word.chars().skip(2)))
    {
        // Just the number of triples, ie number of chars minus two.
        num_triples += 1;

        // If the first character of the triple is an ascii digit, just count that.
        // (Note that ascii digit is not the same as `c0.is_digit()`.)
        if c0 >= '0' && c0 <= '9' {
            num_digits += 1;
        }
        // If this is a triple of ascii letters, then see if it's an English trigram.
        // (If it is a triple, it fits in a 15-bit usize, using 5 bits per letter.)
        else if let Some(bit) = triple_to_usize(c0, c1, c2) {
            num_letter_triples += 1;
            if is_trigram(bit) {
                num_trigrams += 1;
            }
        }
    }

    // We declare a word as a hash if there's at least one digit, and either:
    //  - there aren't many trigrams out of valid letter triples
    //  - or there really aren't many letter triples at all
    //
    // The coefficients here (1, 2, 3) were determined empirically, but didn't seem
    // super-sensitive when fiddling with the test data I had easily to hand. It
    // mis-classifies eg `gpl3Plus` as a hash, but that's acceptable.
    num_digits >= 1
        && (num_trigrams * 2 < num_letter_triples || num_letter_triples * 3 < num_triples)
}

/// Convert three chars into a 15-bit usize if they are just three ascii
/// letters.
fn triple_to_usize(c0: char, c1: char, c2: char) -> Option<usize> {
    let bit = char_to_usize(c0)? | char_to_usize(c1)? << 5 | char_to_usize(c2)? << 10;
    Some(bit)
}

/// Convert a single char into a 5-bit usize if it's just an ascii letter.
fn char_to_usize(c: char) -> Option<usize> {
    let c = c.to_ascii_lowercase();
    if c < 'a' || c > 'z' {
        None
    } else {
        Some(c as usize - 'a' as usize)
    }
}

/// 4kb bitmap of the top 15% of English letter trigrams.
/// See `build_tri.rs` for more details - that program constructs the file,
/// that here gets embedded as a flat read-only table in the executable itself.
static TRIGRAM_BITMAP: &[u8; 4096] = std::include_bytes!("trigrams.bitmap");

/// Lookup a potential letter trigram in the bitmap. Input is a 15-bit integer,
/// with three sets of 5 bits each determining a letter (from 'A' == 0 to 'Z' ==
/// 25).
fn is_trigram(bit: usize) -> bool {
    let byte = bit / 8;
    let mask = 1 << (bit % 8);
    TRIGRAM_BITMAP[byte] & mask != 0
}

/// Test a bunch of positive and negative samples.
#[cfg(test)]
mod tests {
    use super::is_hash;
    #[test]
    fn examples() {
        let test_hashes = [
            "4be1767e-fe51-4eba-9fe7-8118f4b1d888",
            "VuhA1t8McNh8LMje7Y0MXoWqEgI",
            "vFzxUN6mMuMFdYCJ9vZAZLBlYHJyJTQD2iI50oSZx",
            "AU6CRgE6nMwqBIxZKzzZZ4-bGatF",
            "7F9EC3B9-9450-49AE-9879-A446F0F4C4A0",
            "B2tGtnIORGjww4z",
            "01c941c71962c2d5127c0d61cb29c6a3a652489c5e",
            "A2F64146-F108-450E-8456-41559BD79E96",
            "AclBxxliwtUSfA1hyynGo6Z",
            "0198c5ab2686263bafc93ebd3ca599f927b83e60d3",
            "9d0c3ffe6783d59b427d018e8341e0084737fde9",
            "1wlkw8jw63vg1xa5hx63kshag71kl81ncdzfaxi3g1mq376m4bb0",
            "f73bf8d584148677b01859677a63191c31911eae",
            "0jlmrx633jvqrqlyhlzpvdrnim128gc81q5psz2lpp2af8p8q9qs",
            "c4662e662462e7bf3c2a968483478a665d00e717",
            "1npnx0h6bd0d7ql93ka7azhj40zgjp815fw2r6smg8ch9p7mzdlx",
            "70ec6ce85bb158151cae5e5c87f95a8e97d2c0c4b001223f33a334e3ce5de178",
            "ac3b87c63620426dd9b991e5ce0329eff545bccbbb34f3be09ff6fb6ab51b7b6",
            "5c839a674fcd7a98952e593242ea400abe93992746761e38641405d28b00f419",
            "b5a972e5669d67ba988ce3dc826706fb0a8b01471c088cb0b6110b805cc36aed",
            "8a5b4b77fdb63c1eca72173d68d24501c54ab1269409f6b672c85deb18af69de",
        ];
        let test_not_hashes = [
            "p37-sharedstreams",
            "photoGuids",
            "streamCtag",
            "sharedstreams",
            "webstream",
            "sharedalbum",
            "icloud-content",
            "locations",
            "template",
            "nixpkgs-channels",
            "nixpkgs-unstable",
            "hercules-ci",
            "gitignore",
            "description",
            "checksum",
            "registry",
            "winapi-x86_64-pc-windows-gnu",
            "winapi-util",
            "winapi-i686-pc-windows-gnu",
            "unicode-xid",
            "proc-macro2",
            "proc-macro-error-attr",
            "mapAttrs",
            "scrubOptionValue",
            "attrsets",
            "xiorcale",
            "wikipedia",
        ];
        for word in test_hashes.iter() {
            assert_eq!(is_hash(word), true, "testing {}", word);
        }
        for word in test_not_hashes.iter() {
            assert_eq!(is_hash(word), false, "testing {}", word);
        }
    }
}
