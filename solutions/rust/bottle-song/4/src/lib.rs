static NUM_WORDS: [&str; 11] = [
    "no", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten",
];

struct Botinfo {
    nsb: usize,
    ntd: usize,
}

pub fn recite(start_bottles: usize, take_down: usize) -> String {
    let mut times = Vec::with_capacity(take_down as usize);
    let mut botinfo = Botinfo {
        nsb: start_bottles,
        ntd: take_down,
    };
    while botinfo.ntd > 0 {
        times.push(wall_bottles(botinfo.nsb));
        botinfo.nsb = botinfo.nsb.saturating_sub(1);
        botinfo.ntd = botinfo.ntd.saturating_sub(1);
    }
    times.join("\n")
}

fn wall_bottles(bottles: usize) -> String {
    format!(
        "{0} green {1} hanging on the wall,\n\
         {0} green {1} hanging on the wall,\n\
         And if one green bottle should accidentally fall,\n\
         There'll be {2} green {3} hanging on the wall.\n",
        capitalize(NUM_WORDS[bottles as usize]),
        if bottles == 1 { "bottle" } else { "bottles" },
        NUM_WORDS[bottles as usize - 1],
        if bottles == 2 { "bottle" } else { "bottles" }
    )
}
fn capitalize(word: &str) -> String {
    word.chars()
        .enumerate()
        .map(|(index, character)| {
            if index == 0 {
                character.to_ascii_uppercase()
            } else {
                character
            }
        })
        .collect()
}
