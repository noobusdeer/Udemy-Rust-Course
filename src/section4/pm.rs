enum Color{
    Red,
    Green,
    Blue,
    RGBColor(u8,u8,u8), // tuple
    Cmyk {cyan:u8, magenta:u8, yello:u8, black:u8} //struct
}

fn how_many(x:i32) -> &'static str {
    match x {
        0 => "no",
        1 | 2 => "one or two",
        z @ 9...11 => "lots",
        12 => "a dozer",
        _ if (x % 2 == 0) => "some",
        _ => "a few"
    }
}

pub fn pattern_matching() {
    for x in 0..13 {
        println!("{}: I have {} oranges", x, how_many(x));
    }

    let mut point = (3,4);

    match (point) {
        (0,0) => println!("origin"),
        (0,y) => println!("x axis y = {}", y),
        (ref mut x,0) => println!("y axis x = {}", x),
        (_,y) => println!("?, y = {}",  y)
    }

    let c:Color = Color::Cmyk { cyan:10, magenta:20, yello:30, black:255 };

    match c {
        Color::Red => println!("r"),
        Color::Green => println!("g"),
        Color::Blue => println!("b"),
        Color::RGBColor(0,0,0) => println!("black"),
        Color::RGBColor(r,g,b) => println!("rgb({},{},{})", r,g,b),
        Color::Cmyk { black:255, .. } => println!("blck"),
        _ => ()
    }
}