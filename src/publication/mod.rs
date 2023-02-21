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

// use std::fmt::{Display, Debug};

// pub struct T(u8);
// pub struct U(u8);

// pub fn implement_display_plus_clone_plus_debug<T: Display + Clone, U: Display + Debug>(t: &T, u: &U) {

//     impl Display for U {
//         fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//           write!(f, "{}", self.0)
//         }
//     }

//     println!("{:?}, {:?}", t, u);
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