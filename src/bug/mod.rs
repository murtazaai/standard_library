use std::io::{Read, Error};

enum Result<T, E> {
    Ok(T),
    Err(E),
}

// fn rocket() -> Result<String, Error> {
//     let ipfs = std::fs::File::open("./Cargo.toml");

//     let file = match  ipfs {
//         Ok(f) => {
//             let mut buf = String::new;
//             f.read_to_string(buf);
//             Ok(buf)
//         },
//         Err(e) => {
//                 error
//         }
//     };
// }

fn diagonalDifference(arr: &[Vec<i32>]) -> i32 {
    
    let mut diagonal_x: i32 = 0;
    let mut diagonal_y: i32 = 0;

    let mut index_x = 0;
    let mut index_y = 0;

    let mut index_l = 0;
    
    let mut index_m = arr.len();

    while index_x < arr.len() {
        
        let index_y_l = arr[index_x].len();
        
        
        while index_y < arr[index_x].len() {
            if index_x == index_y {
                diagonal_x += arr[index_x][index_y];
            }

            if index_l < arr[index_x].len() && 0 <= index_m  {
                diagonal_y = arr[index_m-index_x][index_y+index_l];
                index_l -= 1;
                index_m += 1;
            }

            index_y += 1;
        }

        index_x += 1;
    }

    (diagonal_x - diagonal_y).abs()
}