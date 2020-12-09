fn main() {
    println!("Hello, world!");
}

fn acc_val_before_loop(program: &str) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";
        assert_eq!(acc_val_before_loop(input), 5);
    }
}
