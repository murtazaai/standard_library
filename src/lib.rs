mod std;
mod process;
mod listener;
mod fact;
mod thread_pool;
mod store;
mod config;
mod publication;
mod bug;
mod geo;
mod origin;
mod alien;
    
#[cfg(test)]
mod tests {
    use std::{collections::HashMap, cell::RefCell, rc::Weak, sync::{mpsc::{self, Sender, Receiver}, Mutex}};

    use crate::{origin::life::tree::{Body, Human, Kind}, bug::{parse_string_to_i32, borrow_lifetime, function, Type, deref, Node, List1}, config::read_config, std::Character, alien::unknown::vacuum::Known, thread_pool::{send_message_on_channel, receive_message_on_channel, mutex_thread_handle_join}};
    
    use std::rc::Rc;

    use crate::thread_pool::thread_pool;


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

        let mut scores: HashMap<_, _> = HashMap::new();

        scores.insert(human.head, human.body);

        scores.entry(human.head);

        assert_ne!(scores.capacity(), usize::MIN);

        let panic = Some(8u8);

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
    fn testparse_string_to_i32() {
        assert_eq!(parse_string_to_i32(String::from("8")), 8);
    }

    #[test]
    fn test_power() {
        assert_eq!(2i32.pow(2u32), 4i32);       
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
        // let args = 
        read_config();
        // assert_eq!(args[0], "/home/murtaza/Workspace/source/rust/rust/rust-lib/target/debug/deps/rust_lib-0826646801856ef5".to_string())
    }

    #[test]
    fn test_smart_pointer() {
        let y = Box::new(&5);
        let z = *y;

        assert_eq!(*z, 5);
    }

    #[test]
    fn test_deref() {
        assert_eq!(deref(), 8);
    }

    #[test]
    fn test_ref_counter() {

        let list_1 = Rc::new(List1::Cons(8, Rc::new(List1::Nil)));
        
        assert_eq!(Rc::strong_count(&list_1), 1);
        assert_eq!(Rc::weak_count(&list_1), 0);

        assert_eq!(Rc::strong_count(&list_1), 1);
        assert_eq!(Rc::weak_count(&list_1), 0);

    }

    #[test]
    fn test_ref_cell() {
        let ref_cell = RefCell::new(vec!["Hello!".to_string()]);
        ref_cell.borrow_mut().push(String::from("Hello 1"));
        let string = &ref_cell.borrow()[1];
        assert_eq!(*string, String::from("Hello 1"));

        // let ref_cell_1 = new_list_ds(8);
    }

    #[test]
    fn test_node() {
        let actual_node = Node::new();

        let expected_node = Node {
            val: 0,
            parent: RefCell::new(Weak::new()),
            child: RefCell::new(vec![]),
        };

        assert_eq!(actual_node.val, expected_node.val);
    }

    #[test]
    fn test_thread_join() {
        let (sender, receiver): (Sender<i32>, Receiver<i32>) = mpsc::channel();

        send_message_on_channel(sender);

        let actual = receive_message_on_channel(receiver);

        let expected = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

        for i in 0..actual.len()-1 {
            let actual = actual[i];
            assert!(expected.contains(&actual));
        } 
    }   

    #[test]
    fn test_mutex() {
        let mutex = Mutex::new(1);

        let mut locked_mutex = mutex.lock().unwrap();

        *locked_mutex = 2;

        assert_eq!(*locked_mutex, 2);

    }

    #[test]
    fn test_mutex_thread_handle_join() {

        let arc = mutex_thread_handle_join();

        let expected = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

        let actual = *arc.lock().unwrap();

        assert!(expected.contains(&actual));

    }    

    #[test]
    fn test_character() {
        let character = Character('a');
        assert_eq!(character.0, 'a');
    }

    #[test]
    fn test_filter() {

        let vec = vec![0, 2, 4, 6, 8];

        let vec_i32: Vec<i32> = vec.iter().map(|f| {
            f / 2
        }).collect();

        for i in 0..4 {
            assert_eq!(vec_i32[i], i as i32);
        }
    }

    #[test]
    fn test_tcp_listener() {
        // tcp_listener();
        assert!(!false);
    }

    #[test]
    fn test_thread_pool() {
        thread_pool();
    }

}