pub enum UnicodeCharacter {
    Food1,
    Body1Horizontal,
    Body1Vertical,
    Body1TopLeftCorner,
    Body1TopRightCorner,
    Body1BottomLeftCorner,
    Body1BottomRightCorner,
}

impl UnicodeCharacter {
    pub fn to_char(&self) -> char {
        match self {
            UnicodeCharacter::Food1 => '\u{2B57}',                  // ⭗
            UnicodeCharacter::Body1Horizontal => '\u{2550}',        // ═
            UnicodeCharacter::Body1Vertical => '\u{2551}',          // ║
            UnicodeCharacter::Body1TopLeftCorner => '\u{2554}',     // ╔
            UnicodeCharacter::Body1TopRightCorner => '\u{2557}',    // ╗
            UnicodeCharacter::Body1BottomLeftCorner => '\u{255A}',  // ╚
            UnicodeCharacter::Body1BottomRightCorner => '\u{255D}', // ╝
        }
    }
}
