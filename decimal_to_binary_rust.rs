fn main() {
    let a = 529;
    let b = to_binary(a);
    println!("Decimal {} is {} in binary.", a, b);
}

fn to_binary(mut n: u32) -> u32 {
    let mut result = String::new();
    while n>1 {
        result.push_str(&(n%2).to_string());
        n = n / 2;
    }
    result.push_str("1");
    result = format!("{}", result.chars().rev().collect::<String>());
    result.parse::<u32>().unwrap()
}
