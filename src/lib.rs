mod publication;
mod bug;
pub mod geo;
mod origin;
mod alien;
    
#[cfg(test)]
mod tests {
    use std::{collections::HashMap};

    use crate::{origin::life::tree::{{/*sum_it, */Human, Body, Kind}}, bug::{read_file, result_okay, parse_string_to_i32, }, publication::{Article, notify, implement_display_plus_clone_plus_debug}};
    use crate::alien::unknown::vacuum::Known;

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

        // assert_eq!(human.chopp_header(), true);

        let mut scores: HashMap<_, _> = HashMap::new();

        scores.insert(human.head, human.body);

        scores.entry(human.head);//.or_insert(human.head, human.body);

        assert_ne!(scores.capacity(), usize::MIN);

        let panic = Some(8u8);

        // let f = match panic {
        //     _ => 8
        // };
        assert_eq!(panic.unwrap(), 8);

    }

    #[test]
    fn test_alien_is_known() {
        let alien = Known(true);
        assert_eq!(alien.0, true);
    }

    #[test]
    fn test_unicode_transformation_format_8() {
        let 你好 = String::from("你好");
        assert_eq!(你好, "你好".to_string());
        // let 交換 = 你好.as_bytes();
        // assert_eq!(交換[0], 0x4EA4);
    }

    #[test]
    fn read_file_gitignore() {
        let path = String::from("./.gitignore");
        
        let result = read_file(path.clone());
        let content = result.unwrap();

        // assert_eq!(content.as_str().lines().chars().last(), '\n');

        assert_eq!(content.clone(), "/target\n/Cargo.lock\n.vscode\n");

        let result_empty = Ok(());

        assert_eq!(result_okay(), result_empty);
    }

    #[test]
    fn testparse_string_to_i32() {
        assert_eq!(parse_string_to_i32(String::from("8")), 8);
    }

    #[test]
    fn test_power() {
        assert_eq!(2i32.pow(2u32), 4i32);       
    }

    // fn test_planet() {
    //     let mercury = Planet(true);
    //     mercury.slice();
    // }

    #[test]
    fn test_str_array_zeroth_element() {
        // let expected: u8 = b"48";
        // assert_eq!(str_array_zeroth_element("01"), expected);
    }

    // #[test]
    // fn test_article() {
    //     let article = Article {
    //         headline: String::from("Article 1"),
    //         location: String::from("Citation 1"),
    //         author: String::from("Author 1"),
    //         content: String::from("Content 1"), 
    //     };
    
    //     assert_eq!(notify(article), format!("Article 1, Citation 1, Author 1, Content 1"));
    // }

    #[test]
    fn test_implement_display_plus_clone_plus_debug() {
        struct S1(u8);
        struct S2(u8);

        let s1 = S1(1);
        let s2 = S2(2);

        implement_display_plus_clone_plus_debug<>();
    }


}