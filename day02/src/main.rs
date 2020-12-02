fn main() {
    println!("Hello, world!");
}

fn num_passwords_valid(passwords: &str) -> i32 {
    -1
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        let input = "1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc";
        assert_eq!(num_passwords_valid(input), 2);
    }
}
