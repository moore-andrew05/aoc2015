use md5;

fn main() {
    let key: &str = "iwrupvqb";
    let mut key_num: u32 = 1;

    loop {
        let num_str: String = key_num.to_string();
        let full_key: String = String::with_capacity(
            key.len() + num_str.len()) + key + &num_str; 
        if check_key(&full_key) {
            println!("{}", num_str);
            break;
        } else {
            key_num += 1;
        }
    }
}

fn check_key(key: &str) -> bool {
    let digest = md5::compute(key);
    let dslice = &digest[0..3];

    if dslice[0] == 0 && dslice[1] == 0 && dslice[2] == 0 {
        return true;
    } else {
        return false;
    }
}
