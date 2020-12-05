fn main() {
    println!("Hello, world!");
}

#[derive(Debug, Eq, PartialEq)]
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
}
