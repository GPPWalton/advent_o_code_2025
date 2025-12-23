pub mod day_3_solution {
    use std::{fs::File, path::Path};
        use std::io::{self, BufRead};
    struct Bank {
        battery_row: Vec<char>,
    }
    fn search_bank(battery_row: &Vec<char>) -> (char, usize)
    {
        let mut index = 0;
        let mut result = '0';
        for i in 0..battery_row.len() {
            let digitised_bat = battery_row[i];
            if  digitised_bat > result {
                result = digitised_bat;
                index = i;                
            }
        }
        (result, index)
    }
    fn find_max_joltage (bank: Bank) -> String {
        //linear search, find the highest value, if it is not the last element (second digit)
        //then save as the first,
        // slice list then repeat to find second
        let mut cur_row = bank.battery_row;
        let mut first_digit = '0';
        let mut second_digit = '0';
        for _digit in 0..2{
            let (result, index) = search_bank(&cur_row);
            if index == cur_row.len() -1 || first_digit != '0'  {
                second_digit = result;
                //slice row at second digit, since it is either the last one, or the loop will end anyway
                println!("second digit:     {}", second_digit);
                cur_row = cur_row[..index].to_vec();
                //replace last value with 1 to allow search to not duplicate choice
                cur_row.push('1');
            }
            else {
                first_digit = result;
                println!("first digit:     {}", first_digit);
                //slice row from after first digit
                 cur_row = cur_row[index+1..].to_vec();
                
            }
        }

        vec![first_digit,second_digit].iter().collect()

    }
    
    fn read_lines<P: AsRef<Path>>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
    }

    fn parse_bank(line: String) -> Bank {
        Bank { battery_row: line.chars().collect() }
    }
    pub fn read_cmd_file(p2_flag: bool) -> usize {
        //read file, process commands and output counter value
        let mut max_jolt = 0;
        if let Ok(lines) = read_lines("./day_3/src/input.txt") {
            // Consumes the iterator, returns an (Optional) String

            //will only run once, more efficient way to do this?
            for line in lines.map_while(Result::ok) {
                let bank: Bank = parse_bank(line);
                if !p2_flag {
                    let parsed_joltage: usize = find_max_joltage(bank).parse().unwrap();
                    max_jolt += parsed_joltage;                    
                }
                else{
                    //for part 2, make a new function, can't sort as it still needs to pick digits
                    // max_jolt += find_max_joltage_p2(bank);
                }
            }
            max_jolt
        }
        else{
            panic!("File not found")
        }
    }
}
