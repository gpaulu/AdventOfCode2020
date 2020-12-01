fn main() {
    println!("Hello, world!");
}

fn find_sum_2020(list: &[i32]) -> (i32, i32) {
    (0, 0)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        let found = find_sum_2020(&[1721, 979, 366, 299, 675, 1456]);
        assert_eq!(found, (1721, 299));
        assert_eq!(found.0 * found.1, 514579);
    }
}
