use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let numbers: Vec<&str> = input.split_whitespace().collect();

    if numbers.len() < 2 {
        let first: i64 = numbers[0].parse().unwrap();
        println!("{}", first);
    } else {
        let first: i64 = numbers[0].parse().unwrap();
        let second: i64 = numbers[1].parse().unwrap();
        let result = first + second;
        println!("{}", result);
    }
}
