/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    println!("code is {:?}", code);
    let mut skipper = false;
    let nums: Vec<u32> = code
        .chars()
        .filter_map(|c| {
            println!("character is {:?}", c);
            println!("character digited is {:?}", c.to_digit(10));
            if c.to_digit(10).is_none() && c != ' ' {
                skipper = true;
            }
            c.to_digit(10)
        })
        .collect();
    if skipper {
        return false;
    }
    println!("nums initialized as {:?}", nums);
    let mut valid = false;
    println!("valid initialized as {:?}", valid);
    let mut nums_check = nums.clone();
    println!("nums check cloned as {:?}", nums_check);
    let mut switch = false;
    println!("switch initialized as {:?}", switch);
    for num in nums_check.iter_mut().rev() {
        switch = !switch;
        println!("switch is {:?}", switch);
        if switch {
            continue;
        }
        println!("doubling num {:?}", *num);
        *num *= 2;
        println!("num doubled is {:?}", *num);
        if *num > 9 {
            *num -= 9;
        }
        println!("num is now {:?}", *num);
    }
    println!("nums check is now {:?}", nums_check);
    let mut nums_sum = 0;
    println!("nums sum initialized as {:?}", nums_sum);
    for num in nums_check {
        nums_sum += num;
    }
    println!("nums sum summed as {:?}", nums_sum);
    if nums_sum % 10 == 0 && nums.len() > 1 {
        valid = true;
    }
    println!("nums sum is {:?}", nums_sum);
    println!("valid state is {:?}", valid);
    valid
}
