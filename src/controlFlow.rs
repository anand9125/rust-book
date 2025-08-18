// use std::io;
// fn main(){
//     println!("Hello, world!");
//     let mut numb = String::new();
//     println!("Give me a number");
//     io::stdin()
//        .read_line(&mut numb)  //read_line() requires a mutable reference to a String.
//        .expect("Faild to read line");

//     let numb: usize = numb
//         .trim()
//         .parse()
//         .expect("Index entered was not a number");

//     if numb>5 {
//         println!("number is greated then 5")
//     }else {
//         println!("number is less then 5")
//     }

// }


// loop
// fn main(){
//     loop {
//         println!("Hii i am anand")
//     }
// }

// returning value from the loop

// fn main(){
//     let mut counter = 0;
//     let result = loop {
//         counter +=1;
//         println!("value of {}",counter);
//         if counter ==10{
//             break counter*2;
//         }
//     };
//     println!("the result is {result}");
// }

// Loop Labels to Disambiguate Between Multiple Loopsfn main() {
// fn main (){
//     let mut count = 0;
//     'counting_up:loop {
//         println!("count={count}" );
//         let mut remaining = 10;
//         loop {
//             println!("remaining={remaining}");
//             if remaining == 9 {
//                 break ;
//             }
//             if count == 2 {
//                 break 'counting_up;
//             }
//             remaining -=1;
//         }
//         count +=1;
        
//     }
//     println!("loops end {count}");
// }

// while loop
// fn main(){
//     let mut number = 5;
//     while number!=0 {
//         println!("{number}");
//         number -=1;
//     }
//     println!("Liftofff");
//   }
// }