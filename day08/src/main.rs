use std::{collections::HashSet, error::Error, fs::read_to_string, str::FromStr};

type BoxError = Box<dyn Error + Send + Sync + 'static>;

fn main() {
    let input = read_to_string("input.txt").expect("Error reading input file");
    println!("Part 1:");
    println!("accumulator = {}", acc_val_before_loop(&input));
    println!("Part 2:");
    println!("accumulator = {}", acc_fixed(&input));
}

fn acc_val_before_loop(program: &str) -> i32 {
    let mut program: Program = program.parse().unwrap();
    program.run().unwrap_err()
}

fn acc_fixed(program: &str) -> i32 {
    let program: Program = program.parse().unwrap();
    let results: Vec<i32> = program
        .possible_fixed()
        .map(|mut p| p.run())
        .filter_map(Result::ok)
        .collect();
    assert_eq!(results.len(), 1);
    results[0]
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

    fn possible_fixed(&self) -> FixedProgramIterator {
        FixedProgramIterator {
            program: self,
            iter: self.instructions.iter().enumerate(),
        }
    }
}

struct FixedProgramIterator<'a> {
    program: &'a Program,
    iter: std::iter::Enumerate<std::slice::Iter<'a, ProgramLine>>,
}

impl<'a> Iterator for FixedProgramIterator<'a> {
    type Item = Program;

    fn next(&mut self) -> Option<Self::Item> {
        let mut new_program = None;
        let (mut index, mut instruction) = self.iter.next()?;
        while new_program.is_none() {
            match instruction.instruction {
                Instruction::Acc(_) => {
                    let (idx, instr) = self.iter.next()?;
                    index = idx;
                    instruction = instr;
                }
                Instruction::Nop(arg) => {
                    let mut p = self.program.clone();
                    p.instructions[index].instruction = Instruction::Jmp(arg);
                    new_program = Some(p);
                }
                Instruction::Jmp(arg) => {
                    let mut p = self.program.clone();
                    p.instructions[index].instruction = Instruction::Nop(arg);
                    new_program = Some(p);
                }
            }
        }
        new_program
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
