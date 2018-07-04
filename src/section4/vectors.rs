fn vectors() {
    let mut a = Vec::new();
    for i in 1..4 {
        a.push(i);
    }
    println!("a = {:?}", a);

    let idx:usize = 0;
    println!("a[{}] = {}", idx, a[idx]);

    match a.get(6) {
        Some(x) => println!("{}", x),
        None => println!("no such elem")
    }
    println!();
    for x in &a {
        println!("{}", x);
    }
    let last_elem = a.pop();
    println!("last elem {:?}, a = {:?}", last_elem, a);

    println!("");
    while let Some(x) = a.pop() {
        println!("{}", x);
    }
}

fn main() {
    vectors();    
}