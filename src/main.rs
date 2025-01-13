// fn main() {
//     let ans = fib(4);
//     print!("{}", ans);
//     // println!("Hello, world!");
// }
// 0 1 1 2 3 5
// fn is_even(num: i32) -> bool {
//     if num % 2 == 0 {
//         return true;
//     }
//     return false;
// }

// fn fib(num: u32) -> u32 {
//     let mut first_num: u32 = 0;
//     let mut second_num: u32 = 1;
//     let mut count = 2;
//     if num == 0 {
//         return first_num;
//     }
//     if num == 1 {
//         return second_num;
//     }
//     // let mut new_num: u32;
//     while count <= num {
//         let new_num = first_num + second_num;
//         first_num = second_num;
//         second_num = new_num;
//         count = count + 1;
//     }
//     return second_num;
// }

// struct Rect {
//     width: u32,
//     height: u32,
// }

// impl Rect {
//     fn area(&self) -> u32 {
//         return self.width * self.height;
//     }

//     fn debug() -> u32 {
//         return 1;
//     }
// }

// fn main() {
//     let rect = Rect {
//         width: 10,
//         height: 20,
//     };
//     print!("Area of the rectangle is {}\n", rect.area());
//     print!("Debug functin {}", Rect::debug());
// }

// Enum

// enum Shape {
//     Rectangle(f64, f64), // width, height
//     Circle(f64),         // radius
// }

// fn main() {
//     let rect = Shape::Rectangle(1.2, 3.2);
//     let circle = Shape::Circle(4.0);
//     print!("Area of rectangle {}", calculate_area(rect));
//     print!("\n");
//     print!("Area of circle {}", calculate_area(circle));
// }

// fn calculate_area(shape: Shape) -> f64 {
//     match shape {
//         Shape::Rectangle(a, b) => a * b,
//         Shape::Circle(r) => 3.14 * r * r,
//     }
// }

// Options

// fn main() {
//     let index_of_a = first_char_a(String::from("manavi"));
//     match index_of_a {
//         Some(index) => print!("Index of a is {}", index),
//         None => print!("a not found"),
//     }
// }

// fn first_char_a(str: String) -> Option<i32> {
//     for (index, character) in str.chars().enumerate() {
//         if character == 'a' {
//             return Some(index as i32);
//         }
//     }
//     return None;
// }

// Results enum (inbuild)
use std::fs::read_to_string;

fn main() {
    let file_data = read_to_string("src/a.txt");
    match file_data {
        Ok(data) => println!("Data of file is {}", data),
        Err(err) => println!("Error occured {}", err),
    }
}

// fn read_file() -> Result {

// }
