enum Color{
    Red,
    Green,
    Blue,
    RGBColor(u8,u8,u8), // tuple
    Cmyk {cyan:u8, magenta:u8, yello:u8, black:u8} //struct
}

fn enums() {
    //let c:Color = Color::RGBColor(0,1,0);
    let c:Color = Color::Cmyk { cyan:10, magenta:20, yello:30, black:255 };

    match c {
        Color::Red => println!("r"),
        Color::Green => println!("g"),
        Color::Blue => println!("b"),
        Color::RGBColor(0,0,0) => println!("black"),
        Color::RGBColor(r,g,b) => println!("rgb({},{},{})", r,g,b),
        Color::Cmyk { cyan:_, magenta:_, yello:_, black:255 } => println!("bbb"),
        _ => ()
    }
}

fn main() {
    enums();
}