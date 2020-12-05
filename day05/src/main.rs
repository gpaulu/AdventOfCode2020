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
    todo!()
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
}
