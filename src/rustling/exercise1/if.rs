// fn bigger(a: i32, b: i32) -> i32 {
//     // TODO: Complete this function to return the bigger number!
//     // If both numbers are equal, any of them can be returned.
//     // Do not use:
//     // - another function call
//     // - additional variables
//     if a>b {
//         return a;
//     }
//     else{
//         return b;
//     }
// }


// fn main() {
//     // You can optionally experiment here.
//    let ans =  bigger(8,10);
//    println!("The bigger number is {}",ans);

// }

// // Don't mind this for now :)
// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn ten_is_bigger_than_eight() {
//         assert_eq!(10, bigger(10, 8));
//     }

//     #[test]
//     fn fortytwo_is_bigger_than_thirtytwo() {
//         assert_eq!(42, bigger(32, 42));
//     }

//     #[test]
//     fn equal_numbers() {
//         assert_eq!(42, bigger(42, 42));
//     }
// }



// // TODO: Fix the compiler error on this function.
// fn picky_eater(food: &str) -> &str {
//     if food == "strawberry" {
//         "Yummy!"
//     } else {
//         "nick"
//     }
// }

// fn main() {
//     // You can optionally experiment here.
//     let ans = picky_eater("strawberry");
//     println!("The answer is {}",ans);
// }

// // TODO: Read the tests to understand the desired behavior.
// // Make all tests pass without changing them.
// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn yummy_food() {
//         // This means that calling `picky_eater` with the argument "strawberry" should return "Yummy!".
//         assert_eq!(picky_eater("strawberry"), "Yummy!");
//     }

//     #[test]
//     fn neutral_food() {
//         assert_eq!(picky_eater("potato"), "I guess I can eat that.");
//     }

//     #[test]
//     fn default_disliked_food() {
//         assert_eq!(picky_eater("broccoli"), "No thanks!");
//         assert_eq!(picky_eater("gummy bears"), "No thanks!");
//         assert_eq!(picky_eater("literally anything"), "No thanks!");
//     }
// }


// You’ve got mixed types in the identifier if expression (int, float, and &str). In Rust, all branches must return the same type
