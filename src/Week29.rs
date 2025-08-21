// #[derive(Debug)]  //this is what called debug macro, it is used to print the struct
// //why we use this debug macro cause  `Debug` is not implemented for `User` what  this will do ?they will add some code that implement debug for user(you can check by expanding macro)
// struct User {
//      username:String,
//      password:String,
//      email:String,
// }



// fn main(){
//      let u = User{
//           username:String::from("Sar"),
//           password:String::from("1234"),
//           email:String::from("sar@gmail.com"),
//      };

//      print!("{:?}",u); //debug 
//   //   println!("{}",u); //display
// }

// Practice macros

// Debug and Display trait and macros

// Debug for formating as string an object for debug purpose
 
// Display will display the object as string userfriendly

// struct User{
//      username:String,
//      password:String,  
// }

// fn main(){
//      let user = User {
//           username:String::from("Sar"),
//           password:String::from("1234"),
//      };
//      println!("{}",user.password);
//      println!("{:?}",user.username);
//      print!("{:?}",user);
      
//  }


// Copy and Clone traits
// fn main(){
//      let user1 = 1;

//      let user2 = user1;  //it will copy the original value here ownershpi is not transfered its jiust cloned 
//      //why? caused its very easy to copy from the stack and value will stay inside the stack very hard to copy from the heap
//      //Now thew question is how will rust knows which is in heap and which is in stack 
//      //using copy and clone traits
//      //copy traits =>that number imple means this thing exist on stack copy it
//      //clone traits =>that string imple means thsi thing exist on heap we have to pass the ownsership or pass the reference or we can clone using .clone
//      println!("{}",user1);

//      println!("{:?}",user2);
// }

// now the problme is 
// in below code struct is simple and exist in stack but rust complainging about the ownership rule
// cause rust is dumb rust doesnot knwo these are simple variable and you havenot implemeted the copy clone traits for struct
// rust understand there is datastructer that haave copy clone traits then understand everything  so we can use copy clone marcos
// to derive the copy macro you need clone macro as well as soon as i will do rust will understand that this struct is simple and exist in stack and should not worry about the ownership rule
// if we add string will get error  string do not impl copy num and bool do impl copy traits[#derive(Copy,Clone)

// #[derive(Debug,Copy,Clone)]
// struct User {
//      is_male:bool,
//      age:u8,
// }

// fn main (){
//      let u1 = User {
//           is_male:true,
//           age:18,
//      };
//      let u2 = u1;
//      print!("{:?},{:?}",u1,u2);
// }
