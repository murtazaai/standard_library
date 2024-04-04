use regex::Regex;

#[allow(dead_code)]
pub fn regex() {
    let regex = Regex::new(r"[prt].ain").unwrap();

    let text = "rrrain spain none";

    println!("The text match {:?}", regex.is_match(text));

    println!("The text has a match: {:?}", regex.find(text));

    for cap in regex.captures_iter(text) {
        println!("Match {:?}", &cap[0]);
    }

    let regex = Regex::new(r"gr[ae]y").unwrap();

    let text = "gray grey graye";

    for cap in regex.captures_iter(text) {
        println!("Match: {:?}", &cap[0]);
    }

    let regex = Regex::new(r"[^A-Z][a-z]ain").unwrap();

    let text = "main pain rain but not 0ain";

    for cap in regex.captures_iter(text) {
        println!("Match: {:?}", &cap[0]);
    }

    let regex = Regex::new(r"\d\d\d\d\d\d\d\d").unwrap();
    // let regex = Regex::new(r"\d........");

    let text = "My phone number is 12345678";

    for cap in regex.captures_iter(text) {
        println!("Match: {:?}", &cap[0]);
    }
}

#[allow(dead_code)]
pub fn string_literals() {
    let str = r"_Hello world_ \n \t";

    println!("str: {}", str);

    let json_str = r#"{
        "name": "Johana",
        "age": "40",
        "gender": female,
    }"#;

    println!("JSON string: {:?}", json_str);

    let str = r###"Hello"# World"###;

    println!("str: {}", str);
}

