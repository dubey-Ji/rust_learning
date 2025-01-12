fn main() {
    let ans = is_even(11);
    print!("{}", ans);
    // println!("Hello, world!");
}

fn is_even(num: i32) -> bool {
    if num % 2 == 0 {
        return true;
    }
    return false;
}
