// //Serialization and decerlization
// use serde::{Serialize, Deserialize};
// use serde_json::{self};

// #[derive(Serialize, Deserialize, Debug)]
// struct Person {
//     name: String,
//     age: u32,
// }

// fn main() { 
//     let person = Person {
//         name: String::from("John Doe"),
//         age: 30,
//     };

  
//     let json_str =  match serde_json::to_string(&person){   
//          Ok(s)=>{
//              println!("Serialized JSON: {}", s);
//              s
//          },
//          Err(e)=>{
//              println!("Error: {}", e);
//              return ;
//          }
//      }; 
 


//     let deserialized_person: Person = serde_json::from_str(&json_str).unwrap();  ////unwrap means =>if this result succed and will get acces directly and if this result failed process ends here
//     println!("Deserialized Person: {:?}", deserialized_person);
// }


// Borsh
// what borsh deos its convert rust data type to byte array and vice versa
// Serialization = Converting Rust data structures → byte arrays
// Deserialization = Converting byte arrays back → Rust data structures
// why we have do this because in blockchain we store data manually we did not depened on compiler


// use borsh::{BorshSerialize, BorshDeserialize};

// #[derive(BorshSerialize, BorshDeserialize, Debug, PartialEq)]

// struct User{
//     username:String,
//     password:String
// }

// fn main(){
//     let user = User{
//         username:String::from("Anand"),
//         password:String::from("123456")
//     };
//     let mut v:Vec<u8> = Vec::new();
//     let ans = user.serialize(&mut v);  //u.serialize puts the data here in vector
//     match ans{
//         Ok(_)=>{
//             println!("Serialized data: {:?}", v);
//         }
//         Err(e)=>{
//             println!("Error: {}", e);
//         }
//     }
//     let deseralize = User::deserialize(&mut v.as_slice());
    
//     match deseralize {
//         Ok(u)=>{
//             println!("Deserialized data: {:?}", u);
//             print!("Username:{} Password:{}", u.username, u.password);  

//         }
//         Err(e)=>{
//             println!("Error: {}", e);
//         }
//     }

// }

// LifeTime

// Problem
// fn main(){
//   let str1 = String::from("Hello");
//   let ans ;
//   {
//       let str2 = String::from("Worllsdfsdfsdfs");
//       ans = longest_string(&str1, &str2);
//   }
// //why dangling error cause ans will point to the str2 which is not in scope (we are returning refrences) so will get lifetime error
// //ans will be a dangling pointer if str2>str1 cause str2 is not in scope and will be dropped
// //why str2 is not in scope is because of the block scope in rust and str2 is dropped after the block ends we can not print str2 after block kyuki wo heap se drop ho jayega block ke bahar kam nhi karta
//   println!("The longest string is {}", ans);
// }

// fn longest_string(a: &String, b: &String) -> &String {
//   if a.len() > b.len() {
//       a.to_string()
//   } else {
//       b.to_string()
//   }
// }

// so what can i do 

// if we do this then will get error cause we are returning b which is not in scope means str2 is not in scope or we can not print outside the block cause it will be dropped after the block
// fn main(){
//   let str1 = String::from("Hello");
//   let ans ;
//   {
//       let str2 = String::from("Worllsdfsdfsdfs");
//       ans = longest_string(&str1, &str2); //we need to define lifetime of ans ki wo depend kispe kar rha h what is the lifetime of return value using lifetime specifer or annotation
//   }

//   println!("The longest string is {}", ans);
// }
// //<'a,'b> lifetime specifiers (a is lifetime of str1 and b is lifetime of str2)
// fn longest_string<'a,'b>(a: &'a String, b: &'b String) -> &'b String {
//    return b;
// }


// fn main(){
//   let str1 = String::from("Hello");
//   let ans ;
//   {
//       let str2 = String::from("sdfs");
//       ans = longest_string(&str1, &str2);
//       println!("The longest string is {}", ans);

//   }
// // println!("The longest string is {}", ans); so we cant not use ans outside the block

// }

// fn longest_string<'a>(a: &'a String, b: &'a String) -> &'a String {  //giving both the varibale the same life time a and a so rust will consider smaller lifetime automatically in case of this s2
//     if a.len() > b.len() {
//         a
//     } else {
//         b
//     }
// }

// fn main (){
//     let str1 = String::from("Hello");
//     let str2 = String::from("sdfs");
//      let ans ;
//     {
//          let str3 = String::from("sdfs");
//          ans = longest_string(&str1,&str2,&str3);
//        print!("The longest string is {}", ans);

//     }
// }

// fn longest_string<'a ,'b>(a:&'a String,b: &'a String,c: &'b String) -> &'a String {
//     if a.len() > b.len(){
//         return a;
//     }
//     return b;;
    
// }

// Lifetime on struct
// rust say there might be spaces this username or password refrence (where its pointed) goes out of scope 
// so you have to tell it how long user will be valid for will it be as long as username is valid or password is valid
// struct User {  
//     username: &String,  
//     password: &String,
// }
// fn main(){
//     let str1 = String::from("Anand");
//     let str2 = String::from("1234");
//     let user = User{
//         username: &str1,
//         password: &str2,
//     };
//     print!("{},{}",user.username,user.password);
// }   

// so use lifetuim in struct
// in this case user lifetime is shorter lifetime amongest both of them  => when you have used same lifetime specifer for both
// struct User<'a> {  
//     username: &'a String,  
//     password: &'a String,
// }
// fn main(){
//     let str1 = String::from("Anand");
//     let str2 = String::from("1234");
//     let user = User{
//         username: &str1,
//         password: &str2,
//     };
//     print!("{},{}",user.username,user.password);
// }   

