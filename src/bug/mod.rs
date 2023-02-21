use core::panic;
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

pub fn parse_string_to_i32(string: String) -> i32{
    let result: i32 = match string.trim().parse() {
        Ok(integer) => integer,
        Err(err) => {
            panic!("{:?}", err);
        }
    };
    result
}


// pub fn str_array_zeroth_element(str: &str) -> u8 {
//     let u8_array = str.as_bytes();
//     let a = u8_array[0];
//     a
// }