fn print_array(vec: Vec<[usize; 2]>) {
    for items in vec.iter(){
        for item in items {
            print!("{} ", item);
        }
    }
}



fn main() {
    let v1: Vec<[usize; 2]> = vec![[1, 2], [3, 5], [8, 0]];
    print_array(v1);
}
