extern crate phrases;

fn main() {
    println!("English: {}, {}", 
        phrases::greetings::english::hello(),
        phrases::greetings::english::goodbye()
    );

    println!("Русский: {}, {}", 
        phrases::greetings::russian::hello(),
        phrases::greetings::russian::goodbye()
    );
}