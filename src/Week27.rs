fn main(){
    let mut str :String = String::from("Hii i am anand");
    let str1 = & mut str;
    let str2 = &str;
    let str3 = &str;
    let str3= &str;
    let str4 = &str;
    println!("{}, {}, {}, {}, {}", str1, str2, str3, str4, str);
}

struct Rect {
    width:f32,
    height:f32
}
impl Rect {
    fn area (&self)->f32{
        return self.width *self.height;
    }
    fn print_somthing (){
        println!("hello world")
    }

}
 
fn main (){
    let r :Rect = Rect {
        width:10.0,
        height:10.0
    };
    println! ("{} {}",r.width,r.height);
    println!("{}",r.area()); 
    Rect::print_somthing();
   
}



enum Direction {
    East,
    West,
    North,
    South,
}

fn main() {
    let direction: Direction = Direction::East;
    steer(direction);
}

fn steer(dir: Direction) {
    match dir {
        Direction::North => println!("North direction"),
        Direction::South => println!("South direction"),
        _ => println!("Horizontal direction"),//catch anythiny 
    }
}


enums with values
enum Shape {
    Squre(f32),
    Rectange(f32,f32)
}

fn main(){
    let squre = Shape::Squre(10.0);
  
    let rect = Shape::Rectange(10.0,10.0);
    
    println!("{}",calculate_area(squre));
  
    println!("{}",calculate_area(rect));
 }

 fn calculate_area(shape:Shape)->f32{
    match shape{
        Shape::Squre(side)=>side*side,
        Shape::Rectange(width,height)=>width*height
    }
 }





enum Shape {
    Squre(f32),
    Circle(f32),
    Rectangle(f32, f32),
}

impl Shape {
    fn area(&self) -> f32 {
        match self {
            Shape::Circle(radius) => std::f32::consts::PI * radius * radius,
            Shape::Squre(side) => side * side,
            Shape::Rectangle(width, height) => width * height,
        }
    }
}

fn main() {
    let shape_circle: Shape = Shape::Circle(10.0);
    println!("Circle area: {}", shape_circle.area());

    let shape_square: Shape = Shape::Squre(5.0);
    println!("Square area: {}", shape_square.area());

    let shape_rectangle: Shape = Shape::Rectangle(10.0, 5.0);
    println!("Rectangle area: {}", shape_rectangle.area());
}


error handling
 use std::fs;

 fn main (){
    let contents = fs::read_to_string("a.txt");
    println!("{:?}", contents);
    match contents {
        Ok(contents)=>println!("{}",contents),
        Err(e)=>println!("Error while reading the file")
    }
 }

option Enum

pub enum Option<T> {
    None,
    Some(T),
}

fn main() {
    let ans = find_first_a(String::from("harkirat"));
    
    match ans {
        Option::None => println!("Value is not found"),
        Option::Some(val) => println!("'a' found at index {}", val),
    }
}

fn find_first_a(s: String) -> Option<u32> {
    let mut index = 0;
    for c in s.chars() {
        if c == 'a' {
            return Option::Some(index);
        }
        index += 1;
    }
    Option::None
}

