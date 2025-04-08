use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filepath = &args[1];
    let content = fs::read_to_string(filepath)
        .expect("Should have read file");
    let content = content.replace("\n", "");
    //dbg!(content);
    let mut count: i32 = 0;
    let mut idx: u16 = 0;
    let mut found_basement_idx: u16 = 0;
    let mut found_basement: bool = false;

    for character in content.chars() {
        idx += 1;
        if character == '(' {
            count += 1;
        } 
        else if character == ')' {
            count -= 1;
        }

        if count < 0 && !found_basement {
             found_basement_idx = idx;
             found_basement = true;
        }
    }

    print!("Final Floor: {}\n", count);
    print!("Basement Reached: {}\n", found_basement_idx);
}
