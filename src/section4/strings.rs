fn strings() {
    // utf-8
    let s:&'static str = "hello there!"; // &str = string slice
    // dont work
    // s = "abc";
    // let h = s[0];

    for c in s.chars().rev() {
        println!("{}", c);
    }

    if let Some(first_char) = s.chars().nth(0) {
        println!("first letter {}", first_char);
    }

    // String
    // heap
    let mut letters = String::new();
    let mut a = 'a' as u8;
    while a <= ('z' as u8) {
        letters.push(a as char);
        letters.push_str(",");
        a += 1;
    }
    println!("{}", letters);

    // &str <> String
    let u:&str = &letters;
    println!("{}", u);

    let mut abc = String::from("hello world");
    let abd2 = "hello world".to_string();
    abc.remove(0);
    abc.push_str("!!!");
    println!("{}", abd2.replace("hello", "goodbye"));
}

fn main() {
    strings();    
}