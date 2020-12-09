fn main() {
    println!("Hello, world!");
}

fn first_xmas_outlier(sequence: &str, preamble: usize, search_window: usize) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";
        assert_eq!(first_xmas_outlier(input, 5, 5), 127);
    }
}
