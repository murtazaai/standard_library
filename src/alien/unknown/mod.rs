/// For WebAssembly
#[allow(dead_code)]
pub mod vacuum_graphic {
    const METER: i32 = 1;
    const DEFAULT_DENSITY: i32 = 7680 * 4320;

    enum Color {
        Rgb(i32, i32, i32),
        Hsv(i32, i32, i32),
    }

    enum Pixel {
        Length(i32),
        Color,
    }

    enum Multiverse {}

    pub enum Shape {
        Coordinate(n),
    }

    pub enum Cube {}

    pub enum Sphere {}

    /// trait t = Generic(T);
    /// struct s = Generic(S);
    /// type tp = s impl t;

    #[allow(dead_code, unused_doc_comments)]
    /// to be!
    trait Density {
        #[allow(unused_variables)]
        fn pixel_per_meter(pixel: i32) {}
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
}
