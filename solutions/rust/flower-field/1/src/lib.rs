pub fn annotate(garden: &[&str]) -> Vec<String> {
    // initialize a vec
    let mut result = vec![];
    // copy garden as a Vec<String>
    for string in garden {
        result.push(string.to_string());
    }
    // for each row and column number (ignore column content)
    for (row, row_content) in garden.iter().enumerate() {
        for (column, column_content) in row_content.chars().enumerate() {
            // skip mines
            if garden[row].chars().nth(column).unwrap() == '*' {
                continue;
            }
            // initialize neighbor count
            let mut neighbors = 0;
            // iterate over nearby cells as long as they aren't edge cells
            // count each "row"
            for y in (row != 0) as i32 * -1..=(row != garden.len() - 1) as i32 {
                for x in (column != 0) as i32 * -1..=(column != garden[row].len() - 1) as i32 {
                    /*
                     * # # #
                     * # O #
                     * # # #
                     */
                    if y == 0 && x == 0 {
                        continue;
                    }
                    // increment neighbor count if this cell is a mine
                    if garden[(row as i32 + y) as usize]
                        .chars()
                        .nth((column as i32 + x) as usize)
                        .expect("DIEDIEDDE")
                        == '*'
                    {
                        neighbors += 1;
                    }
                }
            }
            // clone the row to be like a vec of chars to modify it
            let mut row_vec: Vec<char> = result[row].chars().collect();
            // modify that element in that row to be the number
            if neighbors > 0 {
                row_vec[column] = neighbors.to_string().chars().next().expect("please");
            }
            // modify that row in the result to be our row except as a string
            result[row] = row_vec.into_iter().collect();
            // acknowledge columncontent exists
            println!("ok i used {}", column_content);
        }
    }
    // our result vector!
    result
}
