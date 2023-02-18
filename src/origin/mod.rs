pub mod life {
    pub mod tree {
        /// pub fn sum_it(base: u8, exp: u32) -> u8 {
        ///     if 1 < base && 1 < exp && (base.pow(exp) < u8::MAX) {
        ///         base.pow(exp) - 1
        ///     } else {
        ///         1
        ///     }
        /// }
        
        pub trait Kind {
            fn is_humane(&self, psychometric_trait_attribute: i8) -> bool;
        }
        
        pub struct Body {
            pub vertibral_column: bool,
            pub arms: u8,
            pub legs: u8,
        }
        
        pub struct Human {
            pub head: bool,
            pub body: Body,
        }
        
        impl Kind for Human {
            fn is_humane(&self, psychometric_trait_attribute: i8) -> bool {
                if psychometric_trait_attribute > 0 {
                    true
                } else {
                    false
                }
            }
        }
        
    }
}