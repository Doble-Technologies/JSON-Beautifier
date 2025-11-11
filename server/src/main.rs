
fn print_arr(param_arr: [i32;15]) {
    for x in 0..param_arr.len() {
        print!("{x} ");
    }
}

fn main() {
    let array: [i32; 15] = [0; 15];//add mut
    print_arr(array)
}
