// // fn array_and_vec() -> ([i32; 4], Vec<i32>) {
// //     let a = [10, 20, 30, 40]; // Array

// //     // TODO: Create a vector called `v` which contains the exact same elements as in the array `a`.
// //     // Use the vector macro.
// //      let v = vec![10, 20, 30, 40];
   
// //     (a, v)
// // }

// // fn main() {
// //     // You can optionally experiment here.
// // }

// // #[cfg(test)]
// // mod tests {
// //     use super::*;

// //     #[test]
// //     fn test_array_and_vec_similarity() {
// //         let (a, v) = array_and_vec();
// //         assert_eq!(a, *v);
// //     }
// // } 

// fn vec_loop(input: &[i32]) -> Vec<i32> {
//     let mut output = Vec::new();

//     for element in input {
//         // TODO: Multiply each element in the `input` slice by 2 and push it to
//         // the `output` vector.
//     }

//     output
// }

// fn vec_map_example(input: &[i32]) -> Vec<i32> {
//     // Example: add 1 to each element
//     input.iter().map(|element| element + 1).collect()
// }

// fn vec_map(input: &[i32]) -> Vec<i32> {
//     // Multiply each element by 2 using map
//     input
//         .iter()
//         .map(|element| element * 2)
//         .collect()
// }

// fn main() {
//     // You can optionally experiment here.
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_vec_loop() {
//         let input = [2, 4, 6, 8, 10];
//         let ans = vec_loop(&input);
//         assert_eq!(ans, [4, 8, 12, 16, 20]);
//     }

//     #[test]
//     fn test_vec_map_example() {
//         let input = [1, 2, 3];
//         let ans = vec_map_example(&input);
//         assert_eq!(ans, [2, 3, 4]);
//     }

//     #[test]
//     fn test_vec_map() {
//         let input = [2, 4, 6, 8, 10];
//         let ans = vec_map(&input);
//         assert_eq!(ans, [4, 8, 12, 16, 20]);
//     }
// }