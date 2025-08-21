// // TODO: Fix the compiler error in this function.
// fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
//     let mut vec = vec;

//     vec.push(88);

//     vec
// }

// fn main() {
//     // You can optionally experiment here.
//          let vec0 = vec![22, 44, 66];
//     let vec1 = fill_vec(vec0);
//     println!("{:?}", vec1);
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn move_semantics1() {
//         let vec0 = vec![22, 44, 66];
//         let vec1 = fill_vec(vec0);
//         assert_eq!(vec1, vec![22, 44, 66, 88]);
//     }
// }


// fn main() {
//     // You can optionally experiment here.
// }

// #[cfg(test)]
// mod tests {
//     // TODO: Fix the compiler errors only by reordering the lines in the test.
//     // Don't add, change or remove any line.
//     #[test]
//     fn move_semantics4() {
//         let mut x = Vec::new();
//         let y = &mut x;
//         y.push(42);
//         let z = &mut x;
//         z.push(13);
//         assert_eq!(x, [42, 13]);
//     }
// }

// #![allow(clippy::ptr_arg)]

// // TODO: Fix the compiler errors without changing anything except adding or
// // removing references (the character `&`).

// // Shouldn't take ownership
// fn get_char(data: &String) -> char {
//     data.chars().last().unwrap()
// }

// // Should take ownership
// fn string_uppercase(mut data: String) {
//     data = data.to_uppercase();

//     println!("{data}");
// }

// fn main() {
//     let data = "Rust is great!".to_string();

//     get_char(&data);


//     string_uppercase(data);
// }


// #![allow(clippy::ptr_arg)]

// // TODO: Fix the compiler errors without changing anything except adding or
// // removing references (the character `&`).


// fn get_char(data: &String) -> char {
//     data.chars().last().unwrap()
// }


// fn string_uppercase(mut data: String) {
//     data = data.to_uppercase();

//     println!("{data}");
// }

// fn main() {
//     let data = "Rust is great!".to_string();

//     get_char(&data);

//     string_uppercase(data);
// }
