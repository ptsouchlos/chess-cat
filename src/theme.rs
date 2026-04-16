#[derive(Clone, Copy)]
pub struct Theme {
    pub light_square: (u8, u8, u8),
    pub dark_square: (u8, u8, u8),
    pub white_piece: (u8, u8, u8),
    pub black_piece: (u8, u8, u8),
}

impl Theme {
    pub const CLASSIC: Self = Self {
        light_square: (231, 195, 139),
        dark_square: (155, 111, 74),
        white_piece: (250, 255, 255),
        black_piece: (20, 20, 20),
    };

    pub const GREEN: Self = Self {
        light_square: (238, 238, 210),
        dark_square: (118, 150, 86),
        white_piece: (255, 255, 255),
        black_piece: (20, 20, 20),
    };

    pub const OCEAN: Self = Self {
        light_square: (157, 172, 255),
        dark_square: (50, 100, 164),
        white_piece: (255, 255, 255),
        black_piece: (10, 20, 40),
    };

    // Darker squares improve contrast for white pieces in terminals.
    pub const HIGH_CONTRAST: Self = Self {
        light_square: (220, 200, 155),
        dark_square: (90, 60, 30),
        white_piece: (255, 255, 255),
        black_piece: (20, 20, 20),
    };

    pub const MONO: Self = Self {
        light_square: (175, 175, 175),
        dark_square: (70, 70, 70),
        white_piece: (255, 255, 255),
        black_piece: (10, 10, 10),
    };
}
