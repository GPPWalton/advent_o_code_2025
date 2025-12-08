use std::{env, ffi::OsString};
use std::error::Error;
use day_1::day_1_solution::{read_cmd_file};

fn day_1 (method_flag: bool) {
    //arrow that shows current dial position, starts ar 50
    let  starting_pos = 50;
    let ctr = read_cmd_file(starting_pos, method_flag);

    println!("True code:    {}", ctr);
}

fn get_arg() -> Result<bool, Box<dyn Error>> {
    //check first flag
    match env::args_os().nth(1) {
        //if no flaf, run as normal
        None => Ok(false),
        Some(argument)=> {if argument == OsString::from("method")
                                    || argument == OsString::from("m")
            {
                //check value of flag
               match  env::args_os().nth(2) {
                None => Err(From::from("No value given for flag")),
                Some(value) => {if value == OsString::from("0x434C49434B")
                {
                    Ok(true)
                }
                else{
                    Err(From::from("Invalid method"))
                }}
               }
            }
        else{
            Err(From::from("Invalid argument"))
        }},
    }
}
fn main() {
    let method_flag = match get_arg(){
        Ok(flag) => flag,
        Err(err) => panic!("{:?}",err)
    };
    
    day_1(method_flag)
}
