use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input.txt").expect("Error reading input file");
    println!("First XMAS outlier: {}", first_xmas_outlier(&input, 25, 25));
}

fn first_xmas_outlier(sequence: &str, preamble: usize, search_window: usize) -> usize {
    let nums: Vec<usize> = sequence
        .lines()
        .map(|s| s.parse::<usize>().unwrap())
        .collect();
    *nums
        .iter()
        .skip(preamble)
        .zip(nums.windows(search_window))
        .find(|(elem, window)| is_outlier(**elem, window))
        .unwrap()
        .0
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
        assert_eq!(first_xmas_outlier(input, 5, 5), 127);
    }
}
