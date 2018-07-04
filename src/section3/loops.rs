fn while_and_loop(count:i32) {
    let mut x = 1;

    while x < count {
        x *= 2;
        println!("x = {}", x);
        if  x == 64 { continue; }
    }
    println!("");
    let mut y = 1;
    loop
    {
        y *= 2;
        println!("y = {}", y);
        if  y == 1<<10 { break; }  
    }
    println!("");
    for z in 0..11 {
        println!("z = {}", z);
    }
    println!("");
    for (pos, b) in (30..41).enumerate() {
        println!("{}: {}", pos, b);
    }
}

fn main() {
    while_and_loop(1000);
}