use std::fs::read_to_string;

fn main() {
    let passes = read_to_string("input.txt").expect("error reading input file");
    println!("Max seat id: {}", max_seat_id(&passes));
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
struct Seat {
    row: i32,
    column: i32,
    id: i32,
}

fn decode_boarding_pass(pass: &str) -> Seat {
    // This algorithm is obtuse and inefficient in Rust
    // But, hey, it works!
    // And it was fun to figure out how to make it work
    fn range_binary_search(
        range: Box<dyn ExactSizeIterator<Item = u16>>,
        c: char,
        left_char: char,
        right_char: char,
    ) -> Box<dyn ExactSizeIterator<Item = u16>> {
        let len = range.len();
        if c == left_char {
            Box::new(range.take(len / 2))
        } else if c == right_char {
            Box::new(range.skip(len / 2))
        } else {
            panic!("Bad encoding")
        }
    }
    let row = pass
        .chars()
        .take(7)
        .fold(
            Box::new(0..=127u16) as Box<dyn ExactSizeIterator<Item = u16>>,
            |range, c| range_binary_search(range, c, 'F', 'B'),
        )
        .next()
        .unwrap() as i32;
    let column = pass
        .chars()
        .skip(7)
        .fold(
            Box::new(0..=7u16) as Box<dyn ExactSizeIterator<Item = u16>>,
            |range, c| range_binary_search(range, c, 'L', 'R'),
        )
        .next()
        .unwrap() as i32;
    let id = row * 8 + column;
    Seat { row, column, id }
}

fn max_seat_id(passes: &str) -> i32 {
    passes
        .lines()
        .map(decode_boarding_pass)
        .max_by_key(|seat| seat.id)
        .unwrap()
        .id
}

fn find_empty_seat_id(passes: &str) -> Option<i32> {
    let ids: Vec<_> = passes.lines().map(|s| decode_boarding_pass(s).id).collect();
    find_missing_in_list(ids)
}

fn find_missing_in_list(mut list: Vec<i32>) -> Option<i32> {
    list.sort_unstable();
    let mut prev: Option<i32> = None;
    let mut found: Option<i32> = None;
    for current in list {
        if let Some(prev) = prev {
            if current - 1 != prev {
                if found.is_some() {
                    panic!("multiple numbers missing");
                }
                found = Some(prev + 1);
            }
        }
        prev = Some(current);
    }
    found
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        let pass = "FBFBBFFRLR";
        assert_eq!(
            Seat {
                row: 44,
                column: 5,
                id: 357
            },
            decode_boarding_pass(pass)
        );
        let pass = "BFFFBBFRRR";
        assert_eq!(
            Seat {
                row: 70,
                column: 7,
                id: 567
            },
            decode_boarding_pass(pass)
        );
        let pass = "FFFBBBFRRR";
        assert_eq!(
            Seat {
                row: 14,
                column: 7,
                id: 119
            },
            decode_boarding_pass(pass)
        );
        let pass = "BBFFBBFRLL";
        assert_eq!(
            Seat {
                row: 102,
                column: 4,
                id: 820
            },
            decode_boarding_pass(pass)
        );
    }
    #[test]
    fn found_max_seat_id() {
        let passes = "FBFBBFFRLR\nBFFFBBFRRR\nFFFBBBFRRR\nBBFFBBFRLL\n";
        assert_eq!(max_seat_id(passes), 820);
    }

    #[test]
    fn found_missing_number() {
        assert_eq!(find_missing_in_list(vec![2, 3, 4, 6, 7]).unwrap(), 5);
        assert_eq!(find_missing_in_list(vec![7, 3, 6, 4, 2]).unwrap(), 5);
        assert!(find_missing_in_list(vec![2, 3, 4, 6, 5]).is_none());
    }
    #[test]
    #[should_panic(expected = "multiple numbers missing")]
    fn multiple_missing_numbers() {
        find_missing_in_list(vec![7, 3, 6, 9, 4, 2]);
    }
}
