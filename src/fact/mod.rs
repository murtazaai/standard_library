use std::vec;

fn solution(s: String) -> String {

    
    let mut longest_palindrome = String::new();

    let mut index_1 = 0;
    let mut index_n = s.len()-1;

    let vec: Vec<char> = s.chars().collect();

    // let mut pivot = index_n;

     while 0 <= index_n {
        while index_1 < index_n  {
            if vec[index_1] == vec[index_n] {
                index_1 += 1;
                index_n -= 1;    
            }
        }        
        // pivot -= 1;
        index_n -= 1;
     }
    longest_palindrome    
}