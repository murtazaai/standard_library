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