mod store;
mod config;
mod publication;
mod bug;
mod geo;
mod origin;
mod alien;
    
#[cfg(test)]
mod tests {
    use std::{collections::HashMap};

    use crate::{origin::life::tree::{Body, Human, Kind}, alien::unknown::vacuum::Known, bug::{read_file, result_okay, parse_string_to_i32}, publication::{borrow_lifetime, function, Type}, config::read_config};

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

    #[test]
    fn test_borrow_lifetime() {
        borrow_lifetime();

        let in_1 = Type {};
        // let mut out_1 = Type {};

        function(&in_1);

        // assert_eq!(return_1, &in_1.clone());
    }

    #[test]
    fn test_ascii() {


        let x = "a".as_bytes();
        assert_eq!(x[0], 97);
    }

    #[test]
    fn test_config() {
        // let config = 
        let args = read_config();
        assert_eq!(args[0], "/home/murtaza/Workspace/source/rust/rust/rust-lib/target/debug/deps/rust_lib-0826646801856ef5".to_string())
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

    // #[test]
    // fn test_implement_display_plus_clone_plus_debug() {
    //     let t = Turkey {
    //         val: 1,
    //     };
    //     let u = Uganda {
    //         val: 2,
    //     };

    //     implement_display_plus_clone_plus_debug<Turkey, Uganda>(t, u);
    // }

    // #[test]
    // fn test_largest() {

    //     let x = &[X{x: 0}, X{x: 1}, X{x: 2}, X{x: 3}, X{x: 4}, X{x: 5}, X{x: 6}, X{x: 7}, X{x: 8}, X{x: 9}];
    //     assert_eq!(largest(x), 6);
    // }



}