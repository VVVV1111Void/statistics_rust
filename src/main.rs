pub mod median;
pub mod mode;

fn test() {
    println!("\n Doing a test run of  functions.");
    let arr: Vec<i32> = vec![1, 2, 2, 4, 5, 6, 6, 7];
    println!(" Test array: {:?}", &arr);
    println!(" Median is: {:?}", median::median(&arr));
    println!(" Mode(s) is/are: {:?}\n", mode::mode(&arr));
}
fn main() {
    println!("Hello, world!");
    test()
}
