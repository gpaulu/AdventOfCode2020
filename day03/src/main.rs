fn main() {
    println!("Hello, world!");
}

fn count_trees_with_slope(forest: &str, right: i32, down: i32) -> usize {
    forest::Forest::new(forest)
        .iter(right, down)
        .filter(|item| **item == forest::Lot::Tree)
        .count()
}

mod forest {
    #[derive(Debug, PartialEq, Eq)]
    pub enum Lot {
        Open,
        Tree,
    }

    #[derive(Debug)]
    pub struct Forest {
        matrix: Vec<Vec<Lot>>,
    }

    impl Forest {
        // Should probably return Result<Forest>
        pub fn new(s: &str) -> Forest {
            let matrix = s
                .lines()
                .map(|line| {
                    let mut row = Vec::new();
                    for c in line.as_bytes() {
                        match c {
                            b'.' => row.push(Lot::Open),
                            b'#' => row.push(Lot::Tree),
                            _ => panic!("Bad Forest input! {}", c),
                        }
                    }
                    row
                })
                .collect();
            Forest { matrix }
        }

        pub fn iter(&self, right_step: i32, down_step: i32) -> ForestIter {
            ForestIter {
                forest: self,
                right_step,
                down_step,
                current_row: 0,
                current_col: 0,
            }
        }
    }

    #[derive(Debug)]
    pub struct ForestIter<'a> {
        forest: &'a Forest,
        right_step: i32,
        down_step: i32,
        current_row: i32,
        current_col: i32,
    }

    impl<'a> ForestIter<'a> {
        fn get(&self) -> &'a Lot {
            &self.forest.matrix[self.current_row as usize][self.current_col as usize]
        }

        fn step(&mut self) -> Option<()> {
            let width = self.forest.matrix[0].len() as i32;
            let mut new_col = self.current_col + self.right_step;
            if new_col >= width {
                new_col %= width;
            } else if new_col < 0 {
                //unneeded for positive rightward step
                unreachable!();
            }
            let new_row = self.current_row + self.down_step;
            if new_row as usize >= self.forest.matrix.len() {
                return None;
            }
            self.current_col = new_col;
            self.current_row = new_row;
            Some(())
        }
    }

    impl<'a> Iterator for ForestIter<'a> {
        type Item = &'a Lot;

        fn next(&mut self) -> Option<Self::Item> {
            self.step()?;
            Some(self.get())
        }
    }
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
    #[test]
    fn forest_iter() {
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
        let forest = forest::Forest::new(input);
        let mut iter = forest.iter(3, 1);
        assert_eq!(*iter.next().unwrap(), forest::Lot::Open);
        assert_eq!(*iter.next().unwrap(), forest::Lot::Tree);
        assert_eq!(*iter.next().unwrap(), forest::Lot::Open);
        assert_eq!(*iter.next().unwrap(), forest::Lot::Tree);
        assert_eq!(*iter.next().unwrap(), forest::Lot::Tree);
        assert_eq!(*iter.next().unwrap(), forest::Lot::Open);
        assert_eq!(*iter.next().unwrap(), forest::Lot::Tree);
        assert_eq!(*iter.next().unwrap(), forest::Lot::Tree);
        assert_eq!(*iter.next().unwrap(), forest::Lot::Tree);
        assert_eq!(*iter.next().unwrap(), forest::Lot::Tree);
        assert!(iter.next().is_none());
    }
}
