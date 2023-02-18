pub mod graph {
    pub mod while_hole {
        pub mod milky_way {
            pub mod galaxy {
                pub mod solar_system {

                    pub enum Pole {
                        North,
                        East,
                        West,
                        South,
                    }
                    pub struct Coordinate {
                        pub longitude: f64,
                        pub latitude: f64,
                    }
                    pub struct Rhombus(bool);

                    // pub trait Sphere {
                    //     fn slice(do_it: bool) -> Result<Rhombus, ErrorKind>() {
                    //         match do_it {
                    //             true => Rhombus(true),
                    //             false => ErrorKind::NotFound,
                    //         }       
                    //     }
                    // }

                    /// pub struct Planet(pub bool);

                    // impl Sphere for Planet {
                    //     fn slice() -> Rhombus {
                    //         Rhombus(true)
                    //     }
                    // }
                
                    pub mod star {}

                    pub mod moon {}
                }
            }
        }
    }
}