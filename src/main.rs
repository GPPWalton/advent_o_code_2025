use day_1::day_1_solution::{read_cmd_file};

fn day_1 () {
    //arrow that shows current dial position, starts ar 50
    let  starting_pos = 50;
    let ctr = read_cmd_file(starting_pos);

    println!("True code:    {}", ctr);
}
fn main() {
    day_1()    
}
