// // Booleans (`bool`)

// fn main() {
//     let is_morning = true;
    
//     if is_morning {
//         println!("Good morning!");
//     }

//     // TODO: Define a boolean variable with the name `is_evening` before the `if` statement below.
//     // The value of the variable should be the negation (opposite) of `is_morning`.
//     // let …
//     let is_evening = false;
//     if is_evening {
//         println!("Good evening!");
//     }
// }

// Characters (`char`)

// fn main() {
//     // Note the _single_ quotes, these are different from the double quotes
//     // you've been seeing around.
//     let my_first_initial = 'A';
//     if my_first_initial.is_alphabetic() {
//         println!("Alphabetical!");
//     } else if my_first_initial.is_numeric() {
//         println!("Numerical!");
//     } else {
//         println!("Neither alphabetic nor numeric!");
//     }

//     // TODO: Analogous to the example before, declare a variable called `your_character`
//     // below with your favorite character.
//     // Try a letter, try a digit (in single quotes), try a special character, try a character
//     // from a different language than your own, try an emoji 😉
//      let your_character = '';
//     if your_character.is_alphabetic() {
//         println!("Alphabetical!");
//     } else if your_character.is_numeric() {
//         println!("Numerical!");
//     } else {
//         println!("Neither alphabetic nor numeric!");
//     }
// }

// fn main() {
//     let cat = ("Furry McFurson", 3.5); //its tuple (name, age)
//     let (name, age) = cat; we can desturcture it into name and age

//     // TODO: Destructure the `cat` tuple in one statement so that the println works.
//     // let /* your pattern here */ = cat;

//     println!("{name} is {age} years old");
// }

// fn main() {
//     // You can optionally experiment here
// }

// #[cfg(test)]
// mod tests {
//     use std::array;

//     #[test]
//     fn indexing_tuple() {
//         let numbers = (1, 2, 3);

//         // TODO: Use a tuple index to access the second element of `numbers`
//         // and assign it to a variable called `second`.
//         // let second = ???;
     
//         let second = numbers.1;
//         assert_eq!(second, 2, "This is not the 2nd number in the tuple!");
//     }
// }