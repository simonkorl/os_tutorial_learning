// move_semantics2.rs
// Make me compile without changing line 13!
// Execute `rustlings hint move_semantics2` for hints :)


fn main() {
    let vec0 = Vec::new();
    // let tmp = vec0.clone(); //S1
    // let mut vec1 = fill_vec(tmp); // S1
    let mut vec1 = fill_vec(&vec0); // S2

    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

fn fill_vec(vec: &Vec<i32>) -> Vec<i32> { // vec: &Vec<i32> // S2
    // let mut vec = vec; // S1
    let mut vec = vec.clone(); // S2

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}
