use std::fs;
use std::collections::HashMap;

fn calculate_coords_from_direction(coords: (i32, i32), dir: char) -> (i32, i32) {
    let direction_vector: (i32, i32) = match dir {
        '<' => (-1, 0),
        '>' => (1, 0),
        '^' => (0, 1),
        'v' => (0, -1),
        _ => todo!(),
    };

    return (direction_vector.0 + coords.0, direction_vector.1 + coords.1); 
}


fn main() {
    let filepath: String = String::from("input.txt");
    let content = fs::read_to_string(filepath)
        .expect("Should have read file");
    let content = content.replace("\n", "");

    let mut house_map: HashMap<(i32, i32), u16> = HashMap::new();
    let mut santa_coords: (i32, i32) = (0, 0);
    let mut robot_coords: (i32, i32) = (0, 0);
    let mut santa_turn: bool = true;

    for character in content.chars() {
        let current_coords = if santa_turn {
            &mut santa_coords
        } else {
            &mut robot_coords
        };

        let new_coords = calculate_coords_from_direction(*current_coords, character); 

        if house_map.get(&new_coords) == None {
            house_map.insert(new_coords, 1);
        }
        else {
            *house_map.get_mut(&new_coords).unwrap() += 1;
        }
        *current_coords = new_coords;
        santa_turn = !santa_turn;
    }
    println!("{}", house_map.len());
    println!("Robot Coords: x: {}, y: {}", robot_coords.0, robot_coords.1);
    println!("Santa Coords: x: {}, y: {}", santa_coords.0, santa_coords.1);
}



