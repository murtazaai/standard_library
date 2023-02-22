// pub trait Summary {
//     fn summarize(&self) -> String;
// }

// pub struct Article {
//     pub headline: String,
//     pub location: String,
//     pub author: String,
//     pub content: String,
// }

// impl Summary for Article {
//     fn summarize(&self) -> String {
//         format!("{}, {}, {}, {}", self.headline, self.location, self.author, self.content)
//     }
// }

// impl Summary for Article {
//     fn summarize(&self) -> String {
//         format!("{}, {}, {}, {}", self.headline, self.location, self.author, self.content)
//     }
// }

// pub fn notify<T: Summary>(item: &dyn Summary) -> String {
//     item.summarize()
// }

// use std::fmt::{Display, Debug, Formatter, Result};

// #[derive(Debug, Clone)]
// pub struct Turkey {
//     pub val: u8,
// }

// #[derive(Debug, Clone)]
// pub struct Uganda {
//     pub val: u8,
// }

// impl Display for Turkey {
//     fn fmt(&self, f: &mut Formatter<'_>) -> Result {
//       write!(f, "{}", self.val)
//     }
// }

// impl Display for Uganda {
//     fn fmt(&self, f: &mut Formatter<'_>) -> Result {
//       write!(f, "{}", self.val)
//     }
// }

// pub fn implement_display_plus_clone_plus_debug::<T, U>(t: &T, u: &U) 
// where 
//     T: Turkey, 
//     U: Uganda,
// {
//     println!("{}, {}", t.val, u.val);
// }


// fn plusMinus(arr: &[i32]) {
//     let size = arr.len();
    
//     let mut i = 0;
    
//     let mut negative:f64 = 0.000000;
//     let mut positive: f64 = 0.000000;
//     let mut zero: f64 = 0.000000;

//     while i < size {
//         if arr[i] < 0 {
//             negative += 1.000000;
//         } else if arr[i] > 0 {
//             positive += 1.000000;
//         } else {
//             zero += 1.000000;
//         }       
//     }

//     let divider: f64 = size as f64;

//     let result: &[f64; 3] = &[positive / divider, negative / divider, zero / divider]; 

//     for element in result {
//         println!("{}", element);
//     }
// }