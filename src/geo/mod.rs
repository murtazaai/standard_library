pub mod graph {
    pub mod while_hole {
        pub mod milky_way {
            pub mod galaxy {
                pub mod solar_system {

                    #[derive(Debug)]
                    pub struct Coordinate<X> {
                        pub longitude: X,
                        pub latitude: X,
                    }

                    impl Coordinate<i32> {
                        #[allow(dead_code)]
                        pub fn new(longitude: i32, latitude: i32) -> Coordinate<i32> {
                            Coordinate {
                                longitude,
                                latitude,
                            }
                        }
                    }
                    
                    #[derive(Debug)]
                    pub struct Planet {
                        pub coordinate: Coordinate<i32>
                    }

                    impl Planet {
                        #[allow(dead_code)]
                        pub fn new(coordinate: Coordinate<i32>) -> Planet {
                            Planet {
                                coordinate
                            }
                        }
                    }

                }
            }
        }
    }
}