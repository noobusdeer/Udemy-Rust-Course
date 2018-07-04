use std::mem;

fn main() {
    // u - usigned 0 + 0..255(1bite)
    // i - signed - 0 + -27...28(1bite)

    //i8 u8 i16 u16 i32 u32 i64 u64 - int types

    let a:u8 = 123; // 8bits 1bite
    println!("a = {}", a);

    let mut b:i8 = 0; //mutable
    println!("b = {}", b);
    b = 42;
    println!("b = {}", b);

    let mut c = 2; 
    println!("c = {}, size = {} bytes", c, mem::size_of_val(&c));
    c = -1;
    println!("c = {}, size = {} bytes", c, mem::size_of_val(&c));

    let z:isize = 123; // isize/usize
    let size_of_z = mem::size_of_val(&z);
    println!("z = {}, takes up {} bytes, {}-bit OS", z, size_of_z, size_of_z * 8);

    let d:char = 'x'; //char not need
    println!("d = {}, size = {} bytes", d, mem::size_of_val(&d));

    let e:f32 = 2.5; // double-precision, 8 bytes or 64 bits, f64 -- f32
    println!("e = {}, size = {} bytes", e, mem::size_of_val(&e));

    let g = false;
     println!("g = {}, size = {} bytes", g, mem::size_of_val(&g));
}
