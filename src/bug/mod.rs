use core::panic;
use std::ops::Deref;
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

// struct BlueBox<T>(T);

// impl Deref for BlueBox<T> {
//     fn new(box: BlueBox<T>) {

//     }
// }

pub fn deref() {
    
}

// pub fn str_array_zeroth_element(str: &str) -> u8 {
//     let u8_array = str.as_bytes();
//     let a = u8_array[0];
//     a
// }