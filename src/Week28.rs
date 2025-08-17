use std::fmt::Display;
fn main (){
    print_varibale(1);
    print_varibale("Hello World");      
    print_varibale(true);
}
fn print_varibale<T:Display>(x:T){
    println!("{}",x);   //if we want to print generic variable you should firt implement this Display traits and (a )needs to be  bound with this specifc traits,println cant be called with random custom  type like user struct ..means i doont told rust how to print my custom type so it ccant if i wantt to print user also so we have to implement  how to print for my custom type user bacuase its implementation is not present in display 
}


Generic over structs
problem statement  =>struct Rect is for rect(number) and for Rect1(float) we have to create another struct so for that what will do 
struct Rect {
    width: u32,
    height: u32,
}

impl Rect {
    fn area(&self) -> u32 {
      return  self.width * self.height
    }
}

fn main(){
    let rect= Rect{
        width:10,
        height:20
    };
    let rect1= Rect{
        width:10.0,
        height:20.0
    };
    
    println!("Area of rect is {}",rect.area());
    println!("Area of rect1 is {}",rect1.area());
}



Answer we can make struct also generaic so that we can use for both floaat as we as number

struct Rect <T>{
    width: T,
    height: T,
}
impl<T: std::ops::Mul<Output = T> + Copy> Rect<T> {
    pub fn area(&self) -> T {
        return self.height * self.width
 
    }
}

fn main(){
    let rect= Rect{
        width:10,
        height:20
    };
    let rect1= Rect{
        width:10.0,
        height:20.0
    };
    
    println!("Area of rect is {}",rect.area());
    println!("Area of rect1 is {}",rect1.area());
}


traits 

struct Rect{
    width:f32,
    height:f32
}

impl Shape for Rect {  //Rect implements the shape traits
    fn area(&self) -> f32 {
        return self.height * self.width
    }
}

struct Circle{
    radius:f32
}
impl Shape for Circle {  //Circle implements the shape traits
    fn area(&self) -> f32{
        return 3.14*self.radius*self.radius
    }
}
fn print_area_of_shape<T:Shape>(s:T){  //shape could be anything could be circle could be rect
    println!("Area of shape is {}",s.area());  //now this s variable should be bound with specific traits or area should be exist in s vaiable which is type generic
    //just like sum function bound with specific traits(ADD) it should also bound with specific traits(area)
    //jo jo sha pe traits ko implement kar rha hoga usko tm ish function ko de skte ho
}
//this is how we define traits
trait Shape{
    fn area(&self)->f32;  //aur uski shape kaisi dikhni chahiye just like interface
}

fn main (){
    let r = Rect{
        width:10.0,
        height:20.0
    };
    let c = Circle{
        radius:5.0
    };
    println!("Area of rect is {}",r.area());
    println!("Area of circle is {}",c.area());
    print_area_of_shape(r);
    print_area_of_shape(c);
}