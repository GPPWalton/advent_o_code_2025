pub mod day_1_solution {
    #[derive(Debug)]
    enum Direction {
        L,
        R
    }

    #[derive(Debug)]
    struct cmd {
        direction: Direction,
        num_shift: u16,
    }
    pub(self) fn parse_cmd(cmd: &String)-> cmd{
        let mut parsed_cmd: cmd;
        let split_cmd  = cmd.split_at(1);
        if split_cmd.0 == "L" {
            parsed_cmd = cmd {
                direction: Direction::L,
               num_shift:  split_cmd.1.parse().unwrap(),
            };
            parsed_cmd
        } else if split_cmd.0 == "R" {
            parsed_cmd = cmd {
                direction: Direction::R,
               num_shift:  split_cmd.1.parse().unwrap(),
            };
            parsed_cmd
        } else{
            panic!("Invalid command")
        }
    }

    fn shift_left(shift_value: u16 , cur_pos: i32) -> i32{
        let shift = i32::from(shift_value);
        let true_pos = cur_pos - shift;

        if true_pos.is_negative(){
            //if negative, subtract the the true position from the max dial value
            99 + true_pos
        }else{
            true_pos
        }
    }

    fn shift_right(shift_value: u16 , cur_pos: i32) ->i32{
        let shift = i32::from(shift_value);
        let true_pos = cur_pos + shift;

        if true_pos > 99 {
            //if over 99, find the difference between the max and the true pos
            true_pos - 99
        }else{
            true_pos
        }
    }
  
    fn shift_dial (cmd: String, cur_pos: i32) -> i32 {
        //takes a command and returns the new position of the dial
        let parsed_cmd = parse_cmd(&cmd);
        let mut new_pos: i32;
        match parsed_cmd.direction {
            Direction::L => {
            new_pos = shift_left(parsed_cmd.num_shift, cur_pos);
            },
            Direction::R => {
                new_pos = shift_right(parsed_cmd.num_shift, cur_pos);
            }
            
        }
        new_pos

    }
}
