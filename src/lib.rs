pub fn sum_it(left: isize, right: isize) -> isize {
    if (left <= isize::MAX || right <= isize::MAX) && left + right < isize::MAX {
        left + right
    } else {
        //panic!("Boundary value exceeded!");
        -1
    }
}

trait Kind {
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

#[cfg(test)]
mod tests {
    use crate::{sum_it, Human, Body, Kind};
    
    #[test]
    fn it_works() {
        let result = sum_it(2,
             2);
        assert_eq!(result, 4);
              
    }

    #[test]
    fn it_works_nevative_add_negative_must_result_negative() {
        assert_eq!(sum_it(-2, -2), -4);
    }

    // #[test]
    // #[should_panic(expected = "Override default panic handler!")]
    // fn boundary_value_analysis() {
    //     let result: isize = add(isize::MAX, isize::MAX);
    //     assert_eq!(result, -1);
    // }

    #[test]
    fn test_as_byte_array_from_string() {
        let value_string = String::from("value");
        let actual_byte_array = value_string.as_bytes();
        let expected_byte_array = actual_byte_array.clone();
        assert_eq!(actual_byte_array, expected_byte_array);
        assert_eq!(value_string.as_bytes(), expected_byte_array);
    }

    #[test]
    fn test_string_slice() {
        let string = String::from("This is a string!");
        let string_1 = string[0..string.len()].to_string();
        assert_eq!(string_1, "This is a string!")

    }

    #[test]
    fn test_str() {
        let str_1 = "ABCDEF";
        assert_eq!(str_1, "ABCDEF")
    }

    #[test]
    fn test_human() {
        let body = Body {
            vertibral_column: true,
            arms: 2,
            legs: 2,
        };

        let human = Human {
            head: true,
            body,
        };

        assert_eq!(human.is_humane(-1), false);
    }

}