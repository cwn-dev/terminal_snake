pub enum UnicodeCharacter {
    HeavyCircleWithCircleInside,
    BoxDoubleHorizontal,
    BoxDoubleVertical,
    BoxDoubleDownAndRight,
    BoxDoubleDownAndLeft,
    BoxDoubleUpAnRight,
    BoxDoubleUpAndLeft,
}

impl UnicodeCharacter {
    pub fn to_char(&self) -> char {
        match self {
            UnicodeCharacter::HeavyCircleWithCircleInside => '\u{2B57}', // ⭗
            UnicodeCharacter::BoxDoubleHorizontal => '\u{2550}',         // ═
            UnicodeCharacter::BoxDoubleVertical => '\u{2551}',           // ║
            UnicodeCharacter::BoxDoubleDownAndRight => '\u{2554}',       // ╔
            UnicodeCharacter::BoxDoubleDownAndLeft => '\u{2557}',        // ╗
            UnicodeCharacter::BoxDoubleUpAnRight => '\u{255A}',          // ╚
            UnicodeCharacter::BoxDoubleUpAndLeft => '\u{255D}',          // ╝
        }
    }
}
