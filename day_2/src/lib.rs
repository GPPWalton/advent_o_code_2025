pub mod day_2_solution{
    use std::{fs::File, path::Path};
    use std::io::{self, BufRead};

#[derive(Debug)]
    struct Range {
        first_id: usize,
        last_id: usize,
    }

     // TODO: make this more global
    fn read_lines<P: AsRef<Path>>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
    }

    fn split_digits(input: usize) -> usize{
        let mut input = input;
        let mut split_num = Vec::with_capacity(10);
        while input > 0 {
            let digit = input % 10;
            input = input / 10;
            split_num.push(digit);
        }
        split_num.reverse();
        split_num.len()
        
    }

    //try a search algorithm?
    fn parse_range (raw_range: &String) -> Range {
        //split by hyphen
        let range: Vec<&str> = raw_range.split("-").collect();
        //parse into struct
        Range {
            first_id: range[0].parse().unwrap(),
            last_id: range[1].parse().unwrap()
        }
    }
    //take a number split into digits, find a repeating pattern if it exists
    fn find_pattern (id_len: usize,id: usize, p2_flag: bool) -> bool {
        //convert id into string,
        let str_id = id.to_string();
        if !p2_flag {
            if (str_id[..id_len/2].to_string() == str_id[id_len/2..].to_string()) {
                true
            }
            else {
                false
            }
        }
        else {
        // Try all possible pattern lengths from 1 to n/2
            for pattern_len in 1..=id_len / 2 {
                //if the pattern divides evenly with the id length
                if id_len % pattern_len == 0 {
                    //extract pattern
                    let pattern: String = str_id[0..pattern_len].to_string();
                    //repeat pattern to length of original id.
                    let repeated = pattern.repeat(id_len / pattern_len);
                    if repeated == id.to_string() {
                        //if it matches, then return true
                         return true
                    }
                }
            }
            false
        }

    }
    fn find_invalid (range: Range, ctr: usize, p2_flag: bool) -> usize {
        let mut ctr =  ctr;
        for id in range.first_id..(range.last_id +1){

            let id_len = split_digits(id);

            if find_pattern(id_len, id, p2_flag){
                ctr += id;
            }
        }
        ctr
    }
    //TODO: make this global
    pub fn read_cmd_file(p2_flag: bool) -> usize {
        //read file, process commands and output counter value
        // File hosts.txt must exist in the current path
        let mut ctr = 0;
        if let Ok(lines) = read_lines("./day_2/src/input.txt") {
            // Consumes the iterator, returns an (Optional) String

            //will only run once, more efficient way to do this?
            for line in lines.map_while(Result::ok) {
                let ranges: Vec<&str>  = line.split(',').collect();
                //for each range
                for range in ranges{
                    let  parsed_range = parse_range(&String::from(range));
                    ctr = find_invalid(parsed_range, ctr, p2_flag);
                }
            }
            ctr
        }
        else{
            panic!("File not found")
        }
    }
}
