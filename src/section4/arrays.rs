use std::mem;

fn arrays() {
    let mut a:[i32;5] = [1, 2, 3, 4, 5];
    println!("a has {} elements, first is {}", a.len(), a[0]);
    a[0] = 321;
    println!("a has {} elements, first is {}", a.len(), a[0]);
    println!("");

    let mut b = [0u16; 10];
    let mut val = 1;
    for i in 0..b.len() {
        val *= 2;
        b[i] = val;
        println!("{}", b[i]);
    }
    println!("array get {} bytes", mem::size_of_val(&b));
    println!("");

    let mtx:[[f32;3]; 2] = [
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0]
    ];
    println!("{:?}", mtx)
}

fn main() {
    arrays();    
}