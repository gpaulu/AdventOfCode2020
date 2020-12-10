use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input.txt").expect("Error reading input file");
    println!("First XMAS outlier: {}", first_xmas_outlier(&input, 25));
}

fn first_xmas_outlier(sequence: &str, preamble: usize) -> usize {
    let nums = to_vec(sequence);
    find_xmas_outlier(&nums, preamble)
}

fn find_xmas_outlier(nums: &[usize], preamble: usize) -> usize {
    *nums
        .iter()
        .skip(preamble)
        .zip(nums.windows(preamble))
        .find(|(elem, window)| is_outlier(**elem, window))
        .unwrap()
        .0
}

fn to_vec(sequence: &str) -> Vec<usize> {
    sequence
        .lines()
        .map(|s| s.parse::<usize>().unwrap())
        .collect()
}

fn is_outlier(num: usize, window: &[usize]) -> bool {
    let mut compliant = false;
    for (i, i_elem) in window.iter().enumerate() {
        for j_elem in window.iter().skip(i + 1) {
            compliant |= i_elem + j_elem == num;
        }
    }
    !compliant
}

fn encryption_weakness(sequence: &str, preamble: usize) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";
        assert_eq!(first_xmas_outlier(input, 5), 127);
    }

    #[test]
    fn part2_example() {
        let input = "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";
        assert_eq!(encryption_weakness(input, 5), 62);
    }
}
