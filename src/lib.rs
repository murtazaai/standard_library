mod bug;
mod fact;
mod geo;
mod ipfs;
mod launch;
mod listener;
mod origin;
mod process;
mod puzzle;
mod seed;
mod std;
mod store;
mod thread_pool;

/// Unit test cases [`tests`]
/// Verify and validate.
#[cfg(test)]
mod tests {
    use assert_type_eq::assert_type_eq;
    use http::Request;

    /// [`String`] is a [`Vec`] array of [`byte`].
    #[test]
    fn test_as_byte_array_from_string() {
        let value_string = String::from("value");
        let actual_byte_array = value_string.as_bytes();
        let current_byte_array = actual_byte_array.clone();
        assert_eq!(actual_byte_array, current_byte_array);
        assert_eq!(value_string.as_bytes(), current_byte_array);
    }

    /// Slice a [`String`]
    #[test]
    fn test_string_slice() {
        use std::string::String;

        let string = String::from("This is a string!");

        let string_1 = string[0..string.len()].to_string();

        assert_eq!(string_1, "This is a string!")
    }

    /// [`str`] is a [`u8`] array.
    #[test]
    fn test_str() {
        let str_1 = "ABCDEF";
        assert_eq!(str_1, "ABCDEF")
    }

    /// [`Hashmap`] test.
    #[test]
    fn test_human() {
        use std::collections::HashMap;

        use crate::origin::life::tree::{Body, Human, Kind};

        let body = Body {
            vertebral_column: true,
            arms: 2,
            legs: 2,
        };

        let human = Human { head: true, body };

        assert_eq!(human.is_humane(-1), false);

        let mut scores: HashMap<_, _> = HashMap::new();

        scores.insert(human.head, human.body);

        scores.entry(human.head);

        assert_ne!(scores.capacity(), usize::MIN);

        let panic = Some(8u8);

        assert_eq!(panic.unwrap(), 8);
    }

    /// Chinese language.
    /// [UTF-8] not [Unicode]
    #[test]
    fn test_unicode_transformation_format_8() {
        let 你好 = String::from("你好");
        assert_eq!(你好, "你好".to_string());
        // let 交換 = 你好.as_bytes();
        // assert_eq!(交換[0], 0x4EA4);
    }

    /// Parse
    #[test]
    fn test_parse_string_to_i32() {
        use crate::bug::parse_string_to_i32;

        assert_eq!(parse_string_to_i32(String::from("8")), 8);
    }

    /// Exponent
    /// For logarithm implementation.
    #[test]
    fn test_power() {
        assert_eq!(2i32.pow(2u32), 4i32);
    }

    /// Borrow.
    #[test]
    fn test_borrow_lifetime() {
        use crate::bug::{borrow_lifetime, function, Type};

        borrow_lifetime();

        let in_1 = Type {};
        // let mut out_1 = Type {};

        function(&in_1);

        // assert_eq!(return_1, &in_1.clone());
    }

    /// [&str] character [ASCII] confirmation.
    #[test]
    fn test_ascii() {
        let x = "a".as_bytes();

        assert_eq!(x[0], 97);
    }

    /// Stack and heap.
    #[test]
    fn test_smart_pointer() {
        let y = Box::new(&5);

        let z = *y;

        assert_eq!(*z, 5);
    }

    /// Dereference.
    #[test]
    fn test_deref() {
        use crate::bug::deref;

        assert_eq!(deref(), 8);
    }

    /// Eager and late binding.
    /// Weak reference count.
    #[test]
    fn test_ref_counter() {
        use std::rc::Rc;

        use crate::bug::List;

        let list_1 = Rc::new(List::Cons(8, Rc::new(List::Nil)));

        assert_eq!(Rc::strong_count(&list_1), 1);

        assert_eq!(Rc::weak_count(&list_1), 0);

        assert_eq!(Rc::strong_count(&list_1), 1);

        assert_eq!(Rc::weak_count(&list_1), 0);
    }

    /// [`RefCell`] verification.
    #[test]
    fn test_ref_cell() {
        use std::cell::RefCell;

        let ref_cell = RefCell::new(vec!["Hello!".to_string()]);

        ref_cell.borrow_mut().push(String::from("Hello 1"));

        let string = &ref_cell.borrow()[1];

        assert_eq!(*string, String::from("Hello 1"));

        // let ref_cell_1 = new_list_ds(8);
    }

    /// Container.
    #[test]
    fn test_node() {
        use crate::bug::Node;

        let actual_node = Node::new(0);

        let current_node = Node::new(0);

        assert_eq!(actual_node.val, current_node.val);
    }

    /// Message queuing
    #[test]
    fn test_thread_join() {
        use std::sync::mpsc::{self, Receiver, Sender};

        use crate::thread_pool::{receive_message_on_channel, send_message_on_channel};

        let (sender, receiver): (Sender<i32>, Receiver<i32>) = mpsc::channel();

        send_message_on_channel(sender);

        let actual = receive_message_on_channel(receiver);

        let current = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

        for i in 0..actual.len() - 1 {
            let actual = actual[i];
            assert!(current.contains(&actual));
        }
    }

    /// [`MutexGuard`]
    #[test]
    fn test_mutex() {
        use std::sync::Mutex;

        let mutex = Mutex::new(1);

        let mut locked_mutex = mutex.lock().unwrap();

        *locked_mutex = 2;

        assert_eq!(*locked_mutex, 2);
    }

    /// Custom [`Character`] type.
    #[test]
    fn test_character() {
        use crate::std::Character;

        let character = Character('a');

        assert_eq!(character.0, 'a');
    }

    /// [`vec_i32`]
    #[test]
    fn test_filter() {
        let vec = vec![0, 2, 4, 6, 8];

        let vec_i32: Vec<i32> = vec.iter().map(|f| f / 2).collect();

        for i in 0..4 {
            assert_eq!(vec_i32[i], i as i32);
        }
    }

    /// Probe into byte and u8 array
    /// String = Vec<u8>, str = &[u8]
    /// Probe into size of u8
    #[test]
    fn test_binary_octal_decimal_hex() {
        assert_eq!(format!("{:b}", 0b11111111), String::from("11111111"));

        assert_eq!(format!("{:o}", 0o77777777), String::from("77777777"));

        assert_eq!(format!("{:x}", 0xfffffff), String::from("fffffff"));

        assert_eq!(format!("{:X}", 0xFFFFFFF), String::from("FFFFFFF"));
    }

    /// Test [`Planet`] root reset.
    #[test]
    fn test_planet() {
        use crate::geo::graph::while_hole::milky_way::galaxy::solar_system::{Coordinate, Planet};

        let x = 88888888;

        let y = 88888888;

        let coordinate = Coordinate::new(x, y);

        let earth = Planet::new(coordinate);

        let string = format!("{:?}", earth);

        assert_eq!(
            string,
            String::from(
                "Planet { coordinate: Coordinate { longitude: 88888888, latitude: 88888888 } }"
            )
        );
    }

    /// [`read_file_content`]
    #[test]
    #[allow(unused_doc_comments)]
    fn read_file_content() {
        use crate::ipfs::read_file_content;

        let current_path = String::from("./src/methodology.rs");

        /// let invalid_path = String::from("./abc/abc.rs");
        let current_result = read_file_content(current_path);

        let expected_content = String::from("trait Iterate {}\n\ntype stepX = Iterate;\n\nstruct Process {\n    a: stepX,\n}\n\ntrait Method {\n    fn plan() {}\n\n    fn execute() {}\n\n    fn monitor() {}\n\n    fn control() {}\n\n}");

        match current_result {
            Ok(current_content) => assert_eq!(current_content, expected_content),
            Err(e) => assert_eq!(e.to_string(), String::from("")), // panic with the error code and error message
        };
    }

    /// [`assert_type_eq`]
    #[test]
    fn test_generate_request_header() {
        assert_type_eq!(Request<()>, Request<()>);
    }

    /// [`calendar`]
    #[test]
    fn test_calendar() {
        // Solar to Lunar
        // convert_from_solar_to_lunar(a, 'يقطع الرأس');
    }
}
