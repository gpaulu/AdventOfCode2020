use std::{collections::HashSet, fs::read_to_string};

fn main() {
    let input = read_to_string("input.txt").expect("error reading input file");
    println!("Total yes answers: {}", sum_groups_yes(&input));
}

fn sum_groups_yes(answers: &str) -> usize {
    answers
        .split("\n\n")
        .map(num_yes_to_questions_in_group)
        .sum()
}

fn num_yes_to_questions_in_group(answers: &str) -> usize {
    let mut set = HashSet::new();
    for question in answers.chars().filter(|c| !c.is_whitespace()) {
        set.insert(question);
    }
    set.len()
}

fn sum_groups_all_yes(answers: &str) -> usize {
    todo!()
}

fn num_all_yes_to_questions_in_group(answers: &str) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "abc

a
b
c

ab
ac

a
a
a
a

b";
        assert_eq!(sum_groups_yes(input), 11);
    }
    #[test]
    fn single_group() {
        let input = "abc";
        assert_eq!(num_yes_to_questions_in_group(input), 3);
        let input = "a
b
c";
        assert_eq!(num_yes_to_questions_in_group(input), 3);
        let input = "ab
ac";
        assert_eq!(num_yes_to_questions_in_group(input), 3);
        let input = "a
a
a
a";
        assert_eq!(num_yes_to_questions_in_group(input), 1);
        let input = "b";
        assert_eq!(num_yes_to_questions_in_group(input), 1);
    }

    #[test]
    fn part2_example() {
        let input = "abc

a
b
c

ab
ac

a
a
a
a

b";
        assert_eq!(sum_groups_all_yes(input), 6);
    }
    #[test]
    fn part2_single_group() {
        let input = "abc";
        assert_eq!(num_yes_to_questions_in_group(input), 3);
        let input = "a
b
c";
        assert_eq!(num_yes_to_questions_in_group(input), 0);
        let input = "ab
ac";
        assert_eq!(num_yes_to_questions_in_group(input), 1);
        let input = "a
a
a
a";
        assert_eq!(num_yes_to_questions_in_group(input), 1);
        let input = "b";
        assert_eq!(num_yes_to_questions_in_group(input), 1);
    }
}
