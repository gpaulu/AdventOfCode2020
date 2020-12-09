use std::{collections::HashSet, error::Error, fs::read_to_string, str::FromStr};

type BoxError = Box<dyn Error + Send + Sync + 'static>;

fn main() {
    let input = read_to_string("input.txt").expect("Error reading input file");
    println!("accumulator = {}", acc_val_before_loop(&input));
}

fn acc_val_before_loop(program: &str) -> i32 {
    let program = parse_program(program);
    let mut accumulator = 0;
    let mut counter = 0usize;
    let mut visited = HashSet::new();
    while counter < program.len() {
        let line = program[counter];
        if visited.contains(&line) {
            break;
        } else {
            visited.insert(line);
        }
        match line.instruction {
            Instruction::Nop(_) => counter += 1,
            Instruction::Acc(arg) => {
                accumulator += arg;
                counter += 1
            }
            Instruction::Jmp(arg) => {
                if arg < 0 {
                    counter -= arg.abs() as usize
                } else {
                    counter += arg as usize
                }
            }
        }
    }
    accumulator
}

fn parse_program(program: &str) -> Vec<ProgramLine> {
    program
        .lines()
        .enumerate()
        .map(|(num, line)| ProgramLine {
            line_number: num,
            instruction: line.parse().unwrap(),
        })
        .collect()
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct ProgramLine {
    line_number: usize,
    instruction: Instruction,
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
enum Instruction {
    Nop(i32),
    Acc(i32),
    Jmp(i32),
}

impl FromStr for Instruction {
    type Err = BoxError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let arg = s[4..].parse()?;
        Ok(match &s[..3] {
            "nop" => Instruction::Nop(arg),
            "acc" => Instruction::Acc(arg),
            "jmp" => Instruction::Jmp(arg),
            _ => panic!("Bad Instruction"),
        })
    }
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
