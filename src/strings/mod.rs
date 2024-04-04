

#[allow(dead_code)]
pub fn string_concat() {
    let s1 = String::from("Hello");

    let s2 = " Universe";

    let s3 = s1+s2;

    // println!("{}", s1); // This is an error as s1 no more exists and String s3 exists with value "Hello Universe" because of ownership transfer

    println!("s3: {}", s3);

    let s4 = String::from("\t Welcome");

    let s5 = s3 + &s4;

    println!("s5: {}", s5);
}