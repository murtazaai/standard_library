pub mod vacuum {

    trait Photon {
        fn is_digital(&self, conduct: bool) -> bool;
    }

    pub struct Known(pub bool);

    impl Photon for Known {
        fn is_digital(&self, conduct: bool) -> bool {
            match conduct {
                true => {
                    true
                },
                false => {
                    false
                },
            }
        }
    }
}

pub mod graphic {
    use std::fmt::write;

    const METER: i32 = 1;
    const DEFAULT_DENSITY: i32 = 7680*4320;

    enum Color {
        Rgb(i32, i32, i32),
        Hsv(i32, i32, i32),
    }

    enum Pixel {
        Length(i32),
        Color
    }

    trait Density {
        fn pixel_per_meter(pixel: i32) {

        }
    }

    enum Display {
        Width(i32),
        Height(i32),
    }

    enum Origin {
        X(i32),
        Y(i32),
        Z(i32),
    }

    enum Position {
        X(i32),
        Y(i32),
    }

    pub struct Character(pub char);

    impl std::fmt::Display for Character {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.0)
        }
    }
}