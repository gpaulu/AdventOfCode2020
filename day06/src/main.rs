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
    answers
        .split("\n\n")
        .map(num_all_yes_to_questions_in_group)
        .sum()
}

fn num_all_yes_to_questions_in_group(answers: &str) -> usize {
    let sets = answers
        .lines()
        .map(|s| s.chars().collect::<HashSet<_>>())
        .collect::<Vec<_>>();
    intersection_many_sets(&sets).unwrap().count()
}

// `HashSet` has fn `intersection` but only gives the result for 2 sets
// This fn takes an iterator of `HashSet`s and returns `Some` iterator of intersecting elements
// returns `None` if input iterator `sets` results in 0 `HashSet`s
fn intersection_many_sets<'iter, T>(
    sets: &'iter [HashSet<T>],
) -> Option<impl Iterator<Item = &'iter T>>
where
    T: std::cmp::Eq,
    T: std::hash::Hash,
    T: 'iter,
{
    sets.iter().next().map(move |first_set| {
        first_set
            .iter()
            .filter(move |item| sets.iter().all(|s| s.contains(*item)))
    })
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
        assert_eq!(num_all_yes_to_questions_in_group(input), 3);
        let input = "a
b
c";
        assert_eq!(num_all_yes_to_questions_in_group(input), 0);
        let input = "ab
ac";
        assert_eq!(num_all_yes_to_questions_in_group(input), 1);
        let input = "a
a
a
a";
        assert_eq!(num_all_yes_to_questions_in_group(input), 1);
        let input = "b";
        assert_eq!(num_all_yes_to_questions_in_group(input), 1);
    }

    #[test]
    fn set_intersection() {
        let sets: &[HashSet<i32>] = &[
            [1, 2, 3].iter().cloned().collect(),
            [1, 4, 5].iter().cloned().collect(),
            [1, 6, 7].iter().cloned().collect(),
        ];
        let intersection = intersection_many_sets(sets);
        assert!(intersection.is_some());
        let mut intersection = intersection.unwrap();
        assert_eq!(intersection.next(), Some(&1));
        assert_eq!(intersection.next(), None);
    }
}
