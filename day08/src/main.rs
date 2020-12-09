use std::{collections::HashSet, error::Error, fs::read_to_string, str::FromStr};

type BoxError = Box<dyn Error + Send + Sync + 'static>;

fn main() {
    let input = read_to_string("input.txt").expect("Error reading input file");
    println!("accumulator = {}", acc_val_before_loop(&input));
}

fn acc_val_before_loop(program: &str) -> i32 {
    let mut program: Program = program.parse().unwrap();
    program.run().unwrap_err()
}

fn acc_fixed(program: &str) -> i32 {
    todo!()
}

#[derive(Debug, Clone, Default)]
struct Program {
    instructions: Vec<ProgramLine>,
    counter: usize,
    accumulator: i32,
}

impl Program {
    fn execute_instruction(&mut self) {
        let line = self.instructions[self.counter];
        match line.instruction {
            Instruction::Nop(_) => self.counter += 1,
            Instruction::Acc(arg) => {
                self.accumulator += arg;
                self.counter += 1
            }
            Instruction::Jmp(arg) => {
                if arg < 0 {
                    self.counter -= arg.abs() as usize
                } else {
                    self.counter += arg as usize
                }
            }
        }
    }

    //returns Ok(accumulator) if program terminates successfully
    //else returns Err(accumulator) if a loop is detected
    fn run(&mut self) -> Result<i32, i32> {
        let mut visited = HashSet::new();
        while self.counter < self.instructions.len() {
            let line = self.instructions[self.counter];
            if visited.contains(&line) {
                return Err(self.accumulator);
            } else {
                visited.insert(line);
            }
            self.execute_instruction();
        }
        Ok(self.accumulator)
    }
}

impl FromStr for Program {
    type Err = BoxError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let instructions = s
            .lines()
            .enumerate()
            .map(|(num, line)| ProgramLine {
                line_number: num,
                instruction: line.parse().unwrap(),
            })
            .collect();
        Ok(Program {
            instructions,
            ..Default::default()
        })
    }
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

    #[test]
    fn part2_example() {
        let input = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";
        assert_eq!(acc_fixed(input), 8);
    }
}
