pub mod geo;
mod origin;
mod alien;
    
#[cfg(test)]
mod tests {
    use crate::{origin::life::tree::{{/*sum_it, */Human, Body, Kind}}, geo::graph::while_hole::milky_way::galaxy::solar_system::Planet};
    use crate::alien::unknown::vacuum::{Known};

    /// #[test]
    /// fn test_sum_it() {
    ///     let result = sum_it(2,
    ///          8);
    ///     assert_eq!(result, 255);
    /// }

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

        let some = Some(8u8);
        assert_eq!(some.unwrap(), 8)
    }

    #[test]
    fn test_alien_is_known() {
        let alien = Known(true);
        assert_eq!(alien.0, true);
    }

    fn test_planet() {
        let mercury = Planet(true);
        mercury.slice();
    }

}