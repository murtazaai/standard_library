pub mod life {
    pub mod tree {
        #[allow(dead_code)]
        pub fn sum_it(base: u8, exp: u32) -> u8 {
            if 1 < base && 1 < exp && (base.pow(exp) < u8::MAX) {
                base.pow(exp) - 1
            } else {
                1
            }
        }
        
        pub trait Kind {
            fn is_humane(&self, psychometric_trait_attribute: i8) -> bool;
            fn chop_header(order: bool) -> bool;
        }
        
        pub struct Body {
            pub vertebral_column: bool,
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

            #[allow(dead_code, unused_doc_comments)]
            fn chop_header(why: bool) -> bool {
                match why {
                    /// In every scenario: treemap all use cases: 1 result
                    _ => true
                }
            }
        }
        
    }
}