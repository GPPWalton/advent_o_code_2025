use std::{env, ffi::OsString};
use std::error::Error;
use day_1::day_1_solution;
use day_2::day_2_solution;

enum Advent {
    Day1,
    Day2,
    Day3,
    Day4,
    Day5,
    Day6,
    Day7,
    Day8,
    Day9,
    Day10,
    Day11,
    Day12
}
fn day_1 () {
    //current dial position, starts at 50
    let  starting_pos = 50;
    let ctr: i32;

    //check value of flag
    match  env::args_os().nth(2) {
    //if no flag for day_1 run as normal.
    None => {
        //run without flag
        ctr = day_1_solution::read_cmd_file(starting_pos, false);
    },
    Some(argument) => { if argument == OsString::from("method")
                        || argument == OsString::from("m")
    {
        ctr =  day_1_solution::read_cmd_file(starting_pos, true);
    }
    else{
        panic!("2nd argument is invalid, if using new protocols, try 'method'/'m'")
    }}
    }
    println!("True code:    {}", ctr);
}
fn day_2(){
    let ctr: usize;

    ctr = day_2_solution::read_cmd_file();

    println!("The sum of invalid ids is:    {}", ctr);
}
fn select_day (cmd: Advent) {
    match cmd {
        Advent::Day1 => day_1(),
        Advent::Day2 =>day_2(),
        Advent::Day3 =>{},
        Advent::Day4 =>{},
        Advent::Day5 =>{},
        Advent::Day6 =>{},
        Advent::Day7 =>{},
        Advent::Day8 =>{},
        Advent::Day9 =>{},
        Advent::Day10 =>{},
        Advent::Day11 =>{},
        Advent::Day12 =>{},
    }
}

fn get_arg() -> Result<Advent, Box<dyn Error>> {
    //check first flag
    match env::args_os().nth(1) {
        //if no flaf, run as normal
        None => Err(From::from("No argument, please input a day")),
        Some(argument)=> {
            if argument == OsString::from("day_1") {
                Ok(Advent::Day1)
            }
            else if argument == OsString::from("day_2") {
                Ok(Advent::Day2)
            }
            else{
                Err(From::from("Invalid argument"))
            }},
    }
}
fn main() {
    select_day(match get_arg(){
        Ok(cmd) => cmd,
        Err(err) => panic!("{:?}",err)
    });

}
