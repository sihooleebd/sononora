//! Folding text for search, so a query matches however the letters are spelt.
//!
//! Tags and catalogs carry accents, curly quotes and full-width letters that nobody types into
//! a search box. Matching folded text against folded text is what lets "sigur ros" find
//! "Sigur Rós" and "beyonce" find "Beyoncé". Non-Latin scripts transliterate the same way on
//! both sides, so pasting the original title keeps working too.

use deunicode::deunicode_char;

/// Lowercases `text` and strips it to plain ASCII where a letter has an ASCII reading, with
/// runs of whitespace collapsed to single spaces. Both sides of a match go through here, and
/// the result is only for matching, never for display.
pub fn fold(text: &str) -> String {
    let mut folded = String::with_capacity(text.len());
    for letter in text.chars() {
        if letter.is_ascii() {
            folded.push(letter.to_ascii_lowercase());
            continue;
        }
        match deunicode_char(letter) {
            Some(ascii) => folded.extend(ascii.chars().map(|letter| letter.to_ascii_lowercase())),
            None => folded.extend(letter.to_lowercase()),
        }
    }
    let mut spaced = String::with_capacity(folded.len());
    for word in folded.split_whitespace() {
        if !spaced.is_empty() {
            spaced.push(' ');
        }
        spaced.push_str(word);
    }
    spaced
}

/// Whether every whitespace-separated term of `query` appears in at least one of `fields`,
/// matching folded against folded. An empty query matches everything.
pub fn matches(fields: &[&str], query: &str) -> bool {
    let folded: Vec<String> = fields.iter().map(|field| fold(field)).collect();
    fold(query)
        .split_whitespace()
        .all(|term| folded.iter().any(|field| field.contains(term)))
}
