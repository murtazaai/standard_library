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
    use std::{collections::HashMap, cell::RefCell, rc::Weak, thread, time::Duration, sync::{Arc, mpsc::{self, Sender, Receiver}, Mutex, MutexGuard}, any::Any};

    use crate::{origin::life::tree::{Body, Human, Kind}, alien::unknown::{vacuum::Known, graphic::Character}, bug::{read_file, result_okay, parse_string_to_i32, borrow_lifetime, function, Type, deref, List, List1, ListDS, new_list_ds, Node}, config::read_config, thread_pool::{send_message_on_channel, receive_message_on_channel}};
    use std::rc::Rc;

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

        // assert_eq!(content.clone(), "/target\n/Cargo.lock\n.vscode\n");

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

        let list = List::Cons(8, Box::new(List::Nil));

        let list_1 = Rc::new(List1::Cons(8, Rc::new(List1::Nil)));
        
        assert_eq!(Rc::strong_count(&list_1), 1);
        assert_eq!(Rc::weak_count(&list_1), 0);

        let list_2 = &list_1.clone();

        assert_eq!(Rc::strong_count(&list_1), 2);
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

        let arc = Arc::new(Mutex::new(0));

        for _ in 0..10 {

            let arc_clone = arc.clone();

            let join_handle = thread::spawn(move || {

                let mut number = arc_clone.lock().unwrap();                

                *number += 1;

            });
        }

        let expected = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

        let actual = *arc.lock().unwrap();

        assert!(expected.contains(&actual));
 

    }    

    #[test]
    fn test_singleton() {
        trait SingletonTrait {
            fn singleton() -> Self;
        }

        struct Singleton(u8);

        // impl SingletonTrait for Singleton {
        //     fn singleton() -> Self {
                
        //     }
    
        // }
    }

    #[test]
    fn test_character() {
        let character = Character('a');
        assert_eq!(character.0, 'a');
    }

    #[test]
    fn test_heap() {
        /// Singleton implemented
        type Think = Box<dyn FnOnce() + Send + 'static>;

        // assert_eq!(Any::type_id(&self, ))

    }

    // fn test_closure() -> Box<dyn Fn() -> i32> {

    //     struct S1(i32);

    //     impl S1 {
    //         fn funciton() -> i32 {
    //             1i32
    //         }
    //     }
        
    //     let s1 = S1;

    //     let x = Box::new(s1);
    //     x
    // }

    // #[test]
    // fn test_largest() {

    //     let x = &[X{x: 0}, X{x: 1}, X{x: 2}, X{x: 3}, X{x: 4}, X{x: 5}, X{x: 6}, X{x: 7}, X{x: 8}, X{x: 9}];
    //     assert_eq!(largest(x), 6);
    // }

    // fn miniMaxSum(arr: &[i32]) {
        
    //     let n = arr.len();

    //     let mut min_sum = i32::MAX;
    //     let mut max_sum = i32::MIN;

    //     for i in 0..n-1 {
    //         let sum = arr.iter().skip(arr[i].try_into().unwrap()).sum::<i32>();


    //          if sum < min_sum {
    //             min_sum = sum;
    //          }
    //          if sum > max_sum {
    //             max_sum = sum;
    //          }
    //     }
        
    //     println!("{} {}", min_sum, max_sum);
    // }
}