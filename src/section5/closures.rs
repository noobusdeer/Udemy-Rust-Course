fn say_hello() {
    println!("hello");
}

fn closures() {
    let sh = say_hello;
    sh();

    let plus_one = |x:i32| -> i32 { x + 1 };
    let a = 6;
    println!("{} + 1 = {}", a, plus_one(a));

    /*
    let plus_two = |x| 
    { 
        let mut z;
        z += 2;
        z
    };
    println!("{} + 2 = {}", a, plus_two(a));
    */

    let plus_three = |x:&mut i32| *x += 3;
    let mut f = 12;
    plus_three(&mut f);
    println!("f = {}", f);
}

fn main() {
    closures();
}