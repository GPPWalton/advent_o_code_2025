pub mod day_1_solution {
    use std::fs::File;
    use std::io::{self, BufRead};
    use std::path::Path;

    #[derive(Debug)]
    enum Direction {
        L,
        R
    }

    #[derive(Debug)]
    struct Cmd {
        direction: Direction,
        num_shift: u16,
    }

    fn parse_cmd(raw_cmd: &String)-> Cmd{
        let parsed_cmd: Cmd;
        let split_cmd  = raw_cmd.split_at(1);
        if split_cmd.0 == "L" {
            parsed_cmd = Cmd {
                direction: Direction::L,
               num_shift:  split_cmd.1.parse().unwrap(),
            };
            parsed_cmd
        } else if split_cmd.0 == "R" {
            parsed_cmd = Cmd {
                direction: Direction::R,
               num_shift:  split_cmd.1.parse().unwrap(),
            };
            parsed_cmd
        } else{
            panic!("Invalid command")
        }
    }

    fn shift_left(shift_value: u16 , cur_pos: i32, method_flag: bool, ctr: i32) -> (i32, i32){
        let shift = i32::from(shift_value);
        let mut true_pos = cur_pos - shift;
        let mut ctr = ctr;

        if true_pos.is_negative(){
            //if negative, subtract the the true position from the max dial value
            while true_pos.is_negative(){
                true_pos += 100 ;
                // if using the part 2 method, trigger this
                if method_flag{
                    ctr+=1;
                }
            }
            (true_pos, ctr)
        }else{
            (true_pos, ctr)
        }
    }

    fn shift_right(shift_value: u16 , cur_pos: i32, method_flag: bool, ctr: i32) -> (i32,i32){
        let shift = i32::from(shift_value);
        let mut true_pos = cur_pos + shift;
        let mut ctr = ctr;
        if true_pos > 99 {
            //if over 99, find the difference between the max and the true pos
            while true_pos > 99{
                true_pos -= 100;
                // if using the part 2 method, trigger this
                if method_flag {
                    ctr+=1;
                }
            }
            (true_pos, ctr)
        }else{
            (true_pos, ctr)
        }
    }
  
    fn shift_dial (cmd: String, cur_pos: i32, method_flag: bool, ctr: i32) -> (i32, i32) {
        //takes a command and returns the new position of the dial
        let parsed_cmd = parse_cmd(&cmd);
        let new_pos: i32;
        let mut cur_ctr = ctr;
        match parsed_cmd.direction {
            Direction::L => {
            (new_pos, cur_ctr) = shift_left(parsed_cmd.num_shift, cur_pos, method_flag, cur_ctr);
            },
            Direction::R => {
                (new_pos, cur_ctr) = shift_right(parsed_cmd.num_shift, cur_pos, method_flag, cur_ctr);
            }
            
        }
        (new_pos, cur_ctr)
    }
    // The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
    fn read_lines<P: AsRef<Path>>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
    }

    pub fn read_cmd_file(cur_pos: i32, method_flag: bool) -> i32 {
        //read file, process commands and output counter value
        // File hosts.txt must exist in the current path
        let mut pos = cur_pos;
        let mut ctr = 0;
        if let Ok(lines) = read_lines("./day_1/src/input.txt") {
            // Consumes the iterator, returns an (Optional) String
            for line in lines.map_while(Result::ok) {
                (pos, ctr) = shift_dial(line, pos, method_flag, ctr);
                println!("pos:  {} - ctr:   {}", pos, ctr);

                if pos == 0 && !method_flag{
                    ctr += 1;
                }
            }
            ctr
        }
        else{
            panic!("File not found")
        }
    }


}
