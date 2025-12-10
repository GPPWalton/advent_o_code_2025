pub mod day_2_solution{
    use std::{fs::File, path::Path};
    use std::io::{self, BufRead};

#[derive(Debug)]
    struct Range {
        first_id: u16,
        last_id: u16,
    }

     // TODO: make this more global
    fn read_lines<P: AsRef<Path>>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
    }

    fn split_digits(input: u16) -> Vec<u16> {
        let mut input = input;
        let mut split_num = Vec::with_capacity(10);
        println!("starting input:    {}", input);
        while input > 0 {
            let digit = input % 10;
            println!("digit:    {}", digit);
            input = input / 10;
            println!("input:    {}", input);
            split_num.push(digit);
        }
        split_num.reverse();
        println!("{:?}",split_num);
        split_num
        
    }

    //try a search algorithm?
    fn parse_range (raw_range: &String) -> Range {
        //split by hyphen
        let range: Vec<&str> = raw_range.split("-").collect();
        //conver string to u16
        println!("{:?}", range);
        //parse into struct
        Range {
            first_id: range[0].parse().unwrap(),
            last_id: range[1].parse().unwrap()
        }
    }

    fn find_invalid (range: Range) -> i32 {

    }
    //TODO: make this global
    pub fn read_cmd_file() -> i32 {
        //read file, process commands and output counter value
        // File hosts.txt must exist in the current path
        let ctr = 0;
        if let Ok(lines) = read_lines("./day_2/src/input.txt") {
            // Consumes the iterator, returns an (Optional) String

            //will only run once, more efficient way to do this?
            for line in lines.map_while(Result::ok) {
                let ranges: Vec<&str>  = line.split(',').collect();
                //for each range
                for range in ranges{
                    parse_range(&String::from(range));
                }
            }
            ctr
        }
        else{
            panic!("File not found")
        }
    }
}
