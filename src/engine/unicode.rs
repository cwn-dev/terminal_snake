#[derive(Clone, Debug, PartialEq)]
pub enum Unicode {
    Space,
    HeavyCircleWithCircleInside,
    BoxDoubleHorizontal,
    BoxDoubleVertical,
    BoxDoubleDownAndRight,
    BoxDoubleDownAndLeft,
    BoxDoubleUpAndRight,
    BoxDoubleUpAndLeft,
    BoxLightArcDownAndLeft,
    BoxLightArcUpAndLeft,
    BoxLightArcUpAndRight,
    BoxLightArcDownAndRight,
    BoxLightHorizontal,
    BoxLightVertical,
}

impl Unicode {
    pub fn to_char(&self) -> char {
        match self {
            Unicode::Space => '\u{0020}',                       // " " (space)
            Unicode::HeavyCircleWithCircleInside => '\u{2B57}', // ⭗
            Unicode::BoxDoubleHorizontal => '\u{2550}',         // ═
            Unicode::BoxDoubleVertical => '\u{2551}',           // ║
            Unicode::BoxDoubleDownAndRight => '\u{2554}',       // ╔
            Unicode::BoxDoubleDownAndLeft => '\u{2557}',        // ╗
            Unicode::BoxDoubleUpAndRight => '\u{255A}',         // ╚
            Unicode::BoxDoubleUpAndLeft => '\u{255D}',          // ╝
            Unicode::BoxLightArcDownAndLeft => '\u{256E}',      // ╮
            Unicode::BoxLightArcUpAndLeft => '\u{256F}',        // ╯
            Unicode::BoxLightArcUpAndRight => '\u{2570}',       // ╰
            Unicode::BoxLightArcDownAndRight => '\u{256D}',     // ╭
            Unicode::BoxLightHorizontal => '\u{2500}',          // ─
            Unicode::BoxLightVertical => '\u{2502}',            // │
        }
    }
}
