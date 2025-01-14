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
// use std::fs::read_to_string;

// fn main() {
//     let file_data = read_to_string("src/a.txt");
//     match file_data {
//         Ok(data) => println!("Data of file is {}", data),
//         Err(err) => println!("Error occured {}", err),
//     }
// }

// fn main() {
//     let file_data = read_to_string("src/a.txt");
//     match file_data {
//         Ok(data) => println!("Data of file is {}", data),
//         Err(err) => println!("Error occured {}", err),
//     }
// }

// fn read_file() -> Result {

// }

// Vector
// fn main() {
//     let mut vec = Vec::new();

//     vec.push(1);
//     vec.push(2);
//     vec.push(3);
//     println!("{:?}", calculate_even_vec(&vec));
//     print!("{:?}", vec);
// }

// // create a function that takes a vector as an input and returns a vector with even value

// fn calculate_even_vec(vec: &Vec<i32>) -> Vec<i32> {
//     let mut new_vec: Vec<i32> = Vec::new();
//     for val in vec {
//         if val % 2 == 0 {
//             new_vec.push(*val);
//         }
//     }
//     return new_vec;
// }

// Hashmaps
// use ::std::collections::HashMap;

// fn main() {
//     // Create a vector with a tuple value
//     let mut vec: Vec<(String, i32)> = Vec::new();
//     vec.push((String::from("Ashutosh"), 22));
//     vec.push((String::from("Manavi"), 21));
//     println!("{:?}", group_values_by_keys(&vec));
// }

// fn group_values_by_keys(input_vec: &Vec<(String, i32)>) -> HashMap<String, i32> {
//     let mut hm = HashMap::new();
//     for (key, value) in input_vec {
//         hm.insert(key.clone(), *value);
//     }
//     return hm;
// }

// Iterator

// fn main() {
//     let vec = vec![1, 2, 3];

// Type 1: Simple
// for val in vec {
//     println!("{}", val);
// }

// Type 2: .iter() it is borrowing
// let vec_itr = vec.iter();
// for val in vec_itr {
//     println!("{}", val);
// }

// Type 3: .iter_mut() in this we can change the value, to do this variable should also
// be mutable
// let vec_itr = vec.iter_mut();
// for val in vec_itr {
//     *val = *val + 1;
//     println!("{}", val);
// }

// Type 4: while loop .next(), in this vec_itr should be mutable and the variable vec should not be mutable
// let mut vec_itr = vec.iter();
// while let Some(val) = vec_itr.next() {
//     // .next() returs Options<Some, None> if none occurs the while loop breaks
//     println!("{}", val);
// }

// Type 5: .into_iter() in this the ownership is transferred to the vec_iter of vec, vec variable
// will not be usable anymore
// let vec_iter = vec.into_iter();
// for val in vec_iter {
//     println!("{}", val);
// }

// This is same as .into_iter()
// for val in vec {
//     println!("{}", val);
// }
// }

// Consuming adapter and Iterator adaptor
// fn main() {
//     let vec = vec![1, 2, 3];

//     let vec_itr = vec.iter();

//     let itr = vec_itr.map(|x| x + 1);

//     for x in itr {
//         println!("{}", x);
//     }

// println!("{:?}", vec_itr);
// let total: i32 = vec_itr.sum(); // .sum() method consume the vec_itr you can't use again vec_itr

// println!("{}", total);

// println!("{:?}", vec);

// if you uncomment below line it will give error as vec_itr has been consumed on line 214, but
// you can use vec again
// println!("{:?}", vec_itr);
// }

// write the logic to first filter all odd values then double each value and create a new vector
fn main() {
    let vec = vec![1, 2, 3, 4, 5];
    // even_vec is an iterator vector
    let even_vec = vec.iter().filter(|x| *x % 2 == 0).map(|x| *x * 2);

    let new_vec: Vec<i32> = even_vec.collect();
    println!("{:?}", vec);
    println!("{:?}", new_vec);
}
