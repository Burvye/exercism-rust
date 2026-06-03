pub fn recite(start_bottles: u32, take_down: u32) -> String {
    let mut times = vec![];
    let mut take_down = take_down;
    let mut start_bottles = start_bottles;
    while take_down > 0 {
        take_down -= 1;
        times.push(wall_bottles(start_bottles));
        start_bottles -= 1;
    }
    times.join("\n")
}

fn wall_bottles(bottles: u32) -> String {
    format!("{}{}And if one green bottle should accidentally fall,\nThere'll be {} green {} hanging on the wall.\n",
        bottle_sentence(bottles),
        bottle_sentence(bottles),
        number_word(bottles - 1, false),
        pluralizer(bottles - 1))
}
fn bottle_sentence(bottles: u32) -> String {
    format!(
        "{} green {} hanging on the wall,\n",
        number_word(bottles, true),
        pluralizer(bottles)
    )
}
fn pluralizer(bottles: u32) -> String {
    if bottles == 1 {
        return "bottle".to_string();
    }
    "bottles".to_string()
}
fn number_word(num: u32, cap: bool) -> String {
    let num_words = [
        "no", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten",
    ];
    if !cap {
        return num_words[num as usize].to_string();
    } else {
        return num_words[num as usize]
            .chars()
            .nth(0)
            .unwrap()
            .to_uppercase()
            .to_string()
            + &num_words[num as usize].chars().skip(1).collect::<String>();
    }
}
