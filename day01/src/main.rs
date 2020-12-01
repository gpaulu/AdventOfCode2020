use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let f = File::open("input.txt").expect("No input.txt file found");
    let reader = BufReader::new(f);

    let nums: Vec<i32> = reader
        .lines()
        .map(|s| s.unwrap().parse().unwrap())
        .collect();

    match find_sum_2020(nums) {
        Some((x, y)) => println!("Answer: {} * {} = {}", x, y, x * y),
        None => println!("No pair adds to 2020"),
    }
}

// returns smaller number first
fn find_sum_2020(mut list: Vec<i32>) -> Option<(i32, i32)> {
    list.sort_unstable();
    for i in &list {
        let needle = 2020 - *i;
        if let Ok(index) = list.binary_search(&needle) {
            return Some((*i, list[index]));
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        let found = find_sum_2020([1721, 979, 366, 299, 675, 1456].into()).unwrap();
        assert_eq!(found, (299, 1721));
        assert_eq!(found.0 * found.1, 514579);
    }
}
