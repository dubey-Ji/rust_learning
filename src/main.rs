fn main() {
    let ans = fib(4);
    print!("{}", ans);
    // println!("Hello, world!");
}
// 0 1 1 2 3 5
// fn is_even(num: i32) -> bool {
//     if num % 2 == 0 {
//         return true;
//     }
//     return false;
// }

fn fib(num: u32) -> u32 {
    let mut first_num: u32 = 0;
    let mut second_num: u32 = 1;
    let mut count = 2;
    if num == 0 {
        return first_num;
    }
    if num == 1 {
        return second_num;
    }
    // let mut new_num: u32;
    while count <= num {
        let new_num = first_num + second_num;
        first_num = second_num;
        second_num = new_num;
        count = count + 1;
    }
    return second_num;
}
