/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let mut nums_sum = 0;
    let mut count = 0;
    for (index, num) in code
        .chars()
        .filter(|c| !c.is_whitespace())
        .rev()
        .enumerate()
    {
        let Some(mut num) = num.to_digit(10) else {
            return false;
        };
        if index % 2 == 1 {
            num *= 2;
            if num > 9 {
                num -= 9;
            }
        }
        count += 1;
        nums_sum += num;
    }
    nums_sum % 10 == 0 && count > 1
}
