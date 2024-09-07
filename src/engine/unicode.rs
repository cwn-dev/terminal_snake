pub enum UnicodeCharacter {
    Food1,
    Body1Horizontal,
    Body1Vertical,
}

impl UnicodeCharacter {
    pub fn to_char(&self) -> char {
        match self {
            UnicodeCharacter::Food1 => '\u{2B57}',           // ⭗
            UnicodeCharacter::Body1Horizontal => '\u{2550}', // ═
            UnicodeCharacter::Body1Vertical => '\u{2551}',   // ║
        }
    }
}
