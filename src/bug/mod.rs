use core::panic;
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::{Rc, Weak};

#[allow(warnings)]
pub fn parse_string_to_i32(string: String) -> i32 {
    let result: i32 = match string.trim().parse() {
        Ok(integer) => integer,
        Err(err) => {
            panic!("{:?}", err);
        }
    };
    result
}

#[allow(warnings)]
pub fn lifetime<'a>(x: &'a str) -> &'a str {
    x
}

#[allow(warnings)]
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

#[allow(warnings)]
pub type Type = S;

#[allow(warnings)]
pub fn function<'a>(in_1: &'a Type/* , mut out_1: &'a Type */) -> &'a Type
where  
    Type: Clone + Debug
{
    let out_1 = in_1;
    out_1
}

struct BlueBox<T>(T);

#[allow(warnings)]
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

#[allow(warnings)]
pub fn deref() -> u8 {
    let bluebox = BlueBox::new(8u8);
    *bluebox.deref()
}

#[allow(warnings)]
pub enum List {
    Cons(i32, Rc<List>), 
    Nil,
}

// pub enum ListDS {
//     Val(i32, RefCell<Rc<ListDS>>),
//     Nil,
// }

// impl ListDS {
//     fn new(val :i32) -> ListDS {
//         Self::Val(val, RefCell::new(Rc::new(Self::Nil)))
//     }

//     fn iterate(&self) -> Option<&RefCell<Rc<ListDS>>> {
//         match self {
//             Self::Val(val, rc) => Some(rc),
//             Self::Nil => None,
//         }
//     }
// }

// pub fn new_list_ds(val: i32) -> ListDS {
//     ListDS::new(val)
// }

#[allow(dead_code)]
pub struct Node {

    pub val: i32,
    
    parent: RefCell<Weak<Node>>,
    
    child: RefCell<Vec<Rc<Node>>>,

}

#[allow(warnings)]
impl Node {

    pub fn new(i: i32) -> Node {

        Node {

            val: i,

            parent: RefCell::new(Weak::new()),

            child: RefCell::new(vec![]),

        }

    }
  
}