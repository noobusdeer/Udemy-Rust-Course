fn if_statement(temp:i32) {
    if temp > 30 {
        println!("more 30");
    }
    else if temp <10 {
        println!("less 10");
    }
    else {
        println!("10 < x < 30");
    }
}

fn main() {
    if_statement(30);
}