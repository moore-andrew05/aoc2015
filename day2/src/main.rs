use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::cmp::min;

struct ElfBox {
    length: u32,
    width: u32,
    height: u32
}

fn main() {
    let mut total_area: u32 = 0;

    if let Ok(lines) = read_lines("input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.map_while(Result::ok) {
            let data: ElfBox = parse_line(&line); 
            total_area += calculate_ribbon_area(&data);
        }
    }
    print!("Total Area: {}", total_area);
}

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


fn calculate_area(box_data: &ElfBox) -> u32 {
    let side_one: u32 = box_data.length*box_data.height;
    let mut small_side = side_one;
    let side_two: u32 = box_data.height*box_data.width;
    if side_two < small_side {
        small_side = side_two;
    }
    let side_three: u32 = box_data.length*box_data.width;
    if side_three < small_side {
        small_side = side_three;
    }
    return 2*side_one + 2*side_two + 2*side_three + small_side;
}

fn calculate_ribbon_area(box_data: &ElfBox) -> u32 {
    let perim_lh = 2*box_data.length+2*box_data.height;
    let perim_hw = 2*box_data.height+2*box_data.width;
    let perim_lw = 2*box_data.length+2*box_data.width;

    let min_perim = min(perim_lh, min(perim_hw, perim_lw));
    let box_volume = box_data.length*box_data.width*box_data.height;

    return min_perim + box_volume;
}

fn parse_line(line: &str) -> ElfBox {
    let mut vals = line.split("x");
    return ElfBox {
        length: vals.next().unwrap().parse::<u32>().unwrap(), 
        width: vals.next().unwrap().parse::<u32>().unwrap(), 
        height: vals.next().unwrap().parse::<u32>().unwrap()
    }
}
