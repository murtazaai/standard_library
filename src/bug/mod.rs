use core::panic;
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::{Rc, Weak};
use std::{fs::File, io::Read, io::Error};

pub fn read_file(path: String) -> Result<String, Error>{
    let file_result = File::open(path);

    let mut buf = String::new();
    file_result.unwrap().read_to_string(&mut buf)?;
    Ok(buf)
}

pub fn result_okay() -> Result<(), ()>{
    Ok(())
}

// use std::net::IpAddr;

// pub fn validate_ipaddress() {
//     let ip_addr: IpAddr = "192.168.1.1".parse().expect("IP address should be valid!");
//     assert_eq!(ip_addr, IpAddr::from("192.168.1.1"));
// }

pub fn parse_string_to_i32(string: String) -> i32 {
    let result: i32 = match string.trim().parse() {
        Ok(integer) => integer,
        Err(err) => {
            panic!("{:?}", err);
        }
    };
    result
}

pub fn lifetime<'a>(x: &'a str) -> &'a str {
    x
}

pub fn borrow_lifetime() {
    let x = "hello!";
    let x = lifetime(x);
    println!("{}", x.to_string())
}

use std::fmt::Debug;

trait T {}

#[derive(Clone)]
pub struct S {}

impl T for S {}

impl Debug for S {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("S").finish()
    }
}

pub type Type = S;

pub fn function<'a>(in_1: &'a Type/* , mut out_1: &'a Type */) -> &'a Type
where  
    Type: Clone + Debug
{
    let out_1 = in_1;
    out_1
}

struct BlueBox<T>(T);

impl<T> BlueBox<T> {
    fn new(t: T) -> BlueBox<T> {
        BlueBox(t)
    }
}

impl<T> Deref for BlueBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> Drop for BlueBox<T> {
    fn drop(&mut self) {
        // println!("{}", &self.0);
    }
}

pub fn deref() -> u8 {
    let bluebox = BlueBox::new(8u8);
    *bluebox.deref()
}

pub enum List {
    Cons(i32, Box<List>), 
    Nil,
}

pub enum List1 {
    Cons(i32, Rc<List1>), 
    Nil,
}

pub enum ListDS {
    Val(i32, RefCell<Rc<ListDS>>),
    Nil,
}

impl ListDS {
    fn new(val :i32) -> ListDS {
        Self::Val(val, RefCell::new(Rc::new(Self::Nil)))
    }

    // fn val(&self) -> i32 {
    //     let mut val = 0;
    //     Self::Val(val, _<_<_>>);
    //     val
    // }

    fn iterate(&self) -> Option<&RefCell<Rc<ListDS>>> {
        match self {
            Self::Val(val, rc) => Some(rc),
            Self::Nil => None,
        }
    }
}

pub fn new_list_ds(val: i32) -> ListDS {
    ListDS::new(val)
}

pub struct Node {
    pub val: i32,
    pub parent: RefCell<Weak<Node>>,
    pub child: RefCell<Vec<Rc<Node>>>,
}

impl Node {

    pub fn new() -> Node {
        Node {
            val: 0,
            parent: RefCell::new(Weak::new()),
            child: RefCell::new(vec![]),
        }
    }
  
}

// pub fn list_ds_val(list_ds: ListDS) {
//     list_ds
// }

// pub fn iterate_list_ds(list_ds: ListDS) -> Option<RefCell<Rc<ListDS>>> {
//     let option = list_ds.iterate();

//     let rc = match option {
//         Some(x) => {
//             let y = x.as_ptr();
//         },
//         None => {
//             *RefCell::new(Rc::new(ListDS::Nil)).as_ptr()
//         }
//     };
//     Some(Ref)
// }

// pub fn str_array_zeroth_element(str: &str) -> u8 {
//     let u8_array = str.as_bytes();
//     let a = u8_array[0];
//     a
// }

// fn miniMaxSum(arr: &[i32]) {

//     let n = arr.len();
//     let sum: Vec<i32> = vec![];

//     let mut min_sum = arr[0];
//     let mut max_sum = arr[0];

//     let mut sum = 0;

//     for i in 0..n-1 {        
//         sum += arr[i];        
//     }

//     for i in 0..n-1 {
//         let num = sum - arr[i];

//         if num < min_sum {
//             min_sum = sum;
//         } else if num > max_sum {
//             max_sum = num;
//         }
//     } 
    
//     println!("{} {}", min_sum, max_sum);
// }