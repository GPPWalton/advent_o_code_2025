pub mod day_1_solution {
    enum Direction {
        L,
        R
    }

    struct cmd {
        direction: Direction,
        num_shift: u32,
    }
    pub(self) fn parse_cmd(cmd: &String)-> cmd{
        let mut parsed_cmd: cmd;
        let split_cmd  = cmd.split_at(1);
        if split_cmd.0 == "L" {
            parsed_cmd = cmd {
                direction: Direction::R,
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

        
    fn shift_dial (cmd: String) -> i32 {

    }
}
