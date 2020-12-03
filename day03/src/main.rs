fn main() {
    println!("Hello, world!");
}

fn count_trees_with_slope(forest: &str, right: i32, down: i32) -> usize {
    unimplemented!();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        let input = "..##.......
        #...#...#..
        .#....#..#.
        ..#.#...#.#
        .#...##..#.
        ..#.##.....
        .#.#.#....#
        .#........#
        #.##...#...
        #...##....#
        .#..#...#.#";
        assert_eq!(count_trees_with_slope(input, 3, 1), 7);
    }
}
