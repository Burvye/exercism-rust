pub fn is_armstrong_number(num: u32) -> bool {
    digit_pow_sum(&split_digits(num), count_digits(num)) == num
}
fn count_digits(num: u32) -> u32 {
    num.to_string().len() as u32
}
fn split_digits(num: u32) -> Vec<u32> {
    num.to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect()
}
fn digit_pow_sum(digits: &[u32], power: u32) -> u32 {
    digits.iter().map(|d| d.pow(power)).sum()
}
