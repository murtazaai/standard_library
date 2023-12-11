#![feature(unboxed_closures)]
#![feature(tuple_trait)]

mod bug;
mod fact;
mod generic;
mod geo;
mod ipfs;
mod launch;
mod listener;
mod origin;
mod process;
mod puzzle;
mod reliability;
mod seed;
mod std;
mod store;
mod thread_pool;
mod macros;
mod file;
mod lifetime;
mod data_structure;
mod display;
// mod r#macro;

/// Unit test cases [`tests`]
/// Verify and validate.
#[cfg(test)]
mod tests {
    use crate::generic::{f0, Point};
    use crate::ipfs::btree_set_overload;
    use crate::reliability::fault_tolerance::tolerate_fault;
    use assert_type_eq::assert_type_eq;
    use http::Request;
    use crate::{and_or, create_function, find_min, print_result, to_bytes/*, vector*/};
    use crate::data_structure::AveragedCollection;
    use crate::display::coordinates;
    use crate::lifetime::{first_word, longest, longest_with_an_announcement, Rectangle};

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

        let current_path = String::from("./src/batch.rs");

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

    /// Test the shape of a [`Sphere`]
    #[test]
    fn test_sphere() {
        // let dimension = isize::MAX;
        // let sphere: Sphere = Sphere(dimension)
        // assert_type_eq!(Sphere, Sphere);
    }

    /// Data structures: [`BTreeSet`]
    #[test]
    // #[should_panic(expected = "assertion failed")]
    fn test_btree_set_overload() {
        let btree_set = btree_set_overload();

        let a = btree_set.get(&isize::MIN).unwrap();

        assert_eq!(a, &isize::MIN);
    }

    /// Fault tolerance
    /// fault
    #[test]
    fn negative_test_case_tolerate_fault() {
        let result = tolerate_fault("./invalid_dir/batch".to_string());

        let error = result.unwrap_err();

        assert_eq!(error.kind().to_string(), "entity not found".to_string());
    }

    /// Fault tolerance
    /// positive test case
    #[test]
    fn test_no_fault() {
        let content = tolerate_fault("./src/batch.rs".to_string()).unwrap();
        // .expect(&*"entity not found: Os { code: 2, kind: NotFound, message: \"No such file or directory\" }: Os { code: 2, kind: NotFound, message: \"No such file or directory\" }".to_string());

        if Some(&content).is_some() {
            assert_eq!(content, "trait Iterate {}\n\ntype stepX = Iterate;\n\nstruct Process {\n    a: stepX,\n}\n\ntrait Method {\n    fn plan() {}\n\n    fn execute() {}\n\n    fn monitor() {}\n\n    fn control() {}\n\n}".to_string());
        }
    }

    /// Test generic [`format!`]
    #[test]
    fn test_generic_f0() {
        let current_format = f0(10, 20);

        let expected_format = String::from("T: 10, U: 20");

        assert_eq!(current_format, expected_format);
    }

    // Procedural macros
    #[test]
    fn test_declarative_macros() {
        create_function!(func);

        func();

        print_result!("printf");

        and_or!(1i32 + 1 == 2i32; and 2i32 * 2 == 4i32);
        and_or!(true; or false);

        assert_eq!("A", find_min!("A", "B"));

        assert_eq!("A".as_bytes(), to_bytes!("A"));
    }

    #[test]
    fn test_random() {
        let hello = String::from("abc");
        assert_eq!(hello.as_bytes(), vec![97, 98, 99]);

    }

    #[test]
    fn test_hashmap() {
        use std::collections::HashMap;

        let mut scores = HashMap::new();

        let blue_0 = String::from("Blue");
        let yellow_0 = String::from("Yellow");

        scores.insert(blue_0, 10);
        scores.insert(yellow_0, 50);

        let blue_1 = String::from("Blue");
        let yellow_1 = String::from("Yellow");

        assert_eq!(scores.get(&blue_1), Some(&10));
        assert_eq!(scores.get(&yellow_1), Some(&50));
    }

    #[test]
    fn recoverable_errors() {
        use crate::file::read_username_from_file;

        let username = String::new();

        let username_1 = read_username_from_file("src/file/username", username).unwrap();

        assert_eq!(username_1, String::from("alfred"));
    }

    #[test]
    fn test_generics() {
        let p1 = Point { x: 5, y: 10.4 };
        let p2 = Point { x: "Hello", y: 'c' };

        let p3 = p1.mixup(p2);

        assert_eq!(p3.x, 5);
        assert_eq!(p3.y, 'c');
    }

    #[test]
    fn test_lifetime_longest() {
        let a = "abc";
        let b = "defg";
        let longest = longest(a, b);

        assert_eq!(longest, b);
    }

    #[test]
    fn test_lifetime_first_word() {
        let a = "First Last";

        assert_eq!(first_word(a), "First");
    }

    #[test]
    fn test_longest_with_announcement() {
        assert_eq!(longest_with_an_announcement("United Doom", "United Africa", "Destination Somalia"), "United Africa");
    }

    #[test]
    fn test_rectangle() {
        let rectangle = Rectangle {
            width: 2,
            height: 3,
        };

        assert!(!rectangle.can_hold(Rectangle {
            width: 3,
            height: 4,
        }));
    }

    #[test]
    fn test_average_collection() {
        let mut average_collection = AveragedCollection {
            list: vec![],
            average: 0.0000,
        };
        // test add
        average_collection.add(10);
        average_collection.add(11);
        average_collection.add(13);
        assert_eq!(average_collection.list, vec![10, 11, 13]);

        average_collection.remove();

        average_collection.average();
        assert_eq!(average_collection.average, 10.5);
    }

    #[test]
    fn test_coordinates() {
        assert_eq!(coordinates(&(2, 3)), 5);
    }

    // #[test]
    // fn functional_macro() {
    //     assert_eq!(vector!(vec![2, 3, 4]), vec![2, 3, 4]);
    // }
}