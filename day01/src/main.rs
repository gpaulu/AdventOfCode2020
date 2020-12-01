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

    println!("Part 1: ");

    match find2_sum(2020, nums.clone()) {
        Some((x, y)) => println!("Answer: {} * {} = {}", x, y, x * y),
        None => println!("No pair adds to 2020"),
    }

    println!("Part 2: ");

    match find3_sum(2020, nums.clone()) {
        Some((x, y, z)) => println!("Answer: {} * {} * {} = {}", x, y, z, x * y * z),
        None => println!("No trip adds to 2020"),
    }
}

// returns smaller number first
fn find3_sum(sum: i32, mut list: Vec<i32>) -> Option<(i32, i32, i32)> {
    list.sort_unstable();
    for i in &list {
        let new_sum = sum - *i;
        if let Some((x, y)) = find2_sum_sorted(new_sum, &list) {
            return Some((*i, x, y));
        }
    }
    None
}

// returns smaller number first
fn find2_sum(sum: i32, mut list: Vec<i32>) -> Option<(i32, i32)> {
    list.sort_unstable();
    find2_sum_sorted(sum, &list)
}

// list must be sorted
fn find2_sum_sorted(sum: i32, list: &[i32]) -> Option<(i32, i32)> {
    for i in list {
        let needle = sum - *i;
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
        let found = find2_sum(2020, [1721, 979, 366, 299, 675, 1456].into()).unwrap();
        assert_eq!(found, (299, 1721));
        assert_eq!(found.0 * found.1, 514579);
    }

    #[test]
    fn example_part2() {
        let found = find3_sum(2020, [1721, 979, 366, 299, 675, 1456].into()).unwrap();
        assert_eq!(found, (366, 675, 979));
        assert_eq!(found.0 * found.1 * found.2, 241861950);
    }
}
