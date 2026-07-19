use std::env;
fn main() {
    let how_many_inputs_in_the_terminal_lol : Vec<String> = env::args().collect();
    
    let mut list : Vec<String> = Vec::new();
    for i in 1 .. how_many_inputs_in_the_terminal_lol.len() {
        let new_arg:String = how_many_inputs_in_the_terminal_lol[i].clone();
        list.push(new_arg);
    }

    for i in 0..list.len(){
        println!("[ ] {:?}", list[i]);
    }
}
