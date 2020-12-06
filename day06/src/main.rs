fn main() {
    println!("Hello, world!");
}

fn sum_groups_yes(answers: &str) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "abc

a
b
c

ab
ac

a
a
a
a

b";
        assert_eq!(sum_groups_yes(input), 11);
    }
}
