pub mod day_3_solution {
    use std::{fs::File, path::Path};
        use std::io::{self, BufRead};
    struct Bank {
        battery_row: Vec<char>,
    }
    fn search_bank(battery_row: &Vec<char>) -> (bool, u32, usize)
    {
        let mut index = 0;
        let mut result = 0;
        for i in 0..battery_row.len() {
        let digitised_bat = battery_row[i].to_digit(10).unwrap();
        // println!("result:   {}", result);
        // println!("digitised:    {}", digitised_bat);
        if  digitised_bat > result {
            //check if battery is last in row
            if i + 1 == battery_row.len() {
                result = digitised_bat;
                println!("last digit is highest:    {}", result);
                return (true, result, i)
            }
            else{
                result = digitised_bat;
                println!("highest found:    {}", result);
                index = i;
            }
            
        }
        }
        (false, result, index)
    }
    fn find_max_joltage (bank: Bank) -> u32 {
        //linear search, find the highest value, if it is not the last element (second digit)
        //then save as the first,
        // slice list then repeat to find second
        let mut cur_row = bank.battery_row;
        let mut first_digit = 0;
        let mut second_digit = 0;
        for _digit in 0..2{
            let (is_second, result, index) = search_bank(&cur_row);
            if is_second || first_digit != 0  {
                second_digit = result;
                //slice row at second digit, since it is either the last one, or the loop will end anyway
                // is this even worth it?
                println!("second digit:     {}", second_digit);
                cur_row = cur_row[..index].to_vec();
                //replace last value with 1 to allow search to not duplicate choice
                cur_row.push('1');
            }
            else {
                first_digit = result * 10;
                println!("first digit:     {}", first_digit);
                //slice row from after first digit
                 cur_row = cur_row[index+1..].to_vec()
            }
            // println!("passing slice:    {:?}", cur_row)
        }

        first_digit + second_digit

    }
    
    fn read_lines<P: AsRef<Path>>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
    }

    fn parse_bank(line: String) -> Bank {
        Bank { battery_row: line.chars().collect() }
    }
    pub fn read_cmd_file(p2_flag: bool) -> u32 {
        //read file, process commands and output counter value
        let mut max_jolt = 0;
        if let Ok(lines) = read_lines("./day_3/src/input.txt") {
            // Consumes the iterator, returns an (Optional) String

            //will only run once, more efficient way to do this?
            for line in lines.map_while(Result::ok) {
                let bank: Bank = parse_bank(line);
                if !p2_flag {
                max_jolt += find_max_joltage(bank);                    
                }
                else{
                    //for part 2, make a new function, can't sort as it still needs to pick digits
                }
            }
            max_jolt
        }
        else{
            panic!("File not found")
        }
    }
}
