fn match_statement() {
    let country_code = 7;

    let country = match country_code {
        44 => "UK",
        7 => "RUS",
       1...999 => "unknown",
       _ => "none"
    };

    /*
        range type
       1..99 - 1-98
       1...99 - 1-99
    */

    println!("{}", country);
}

fn main() {
    match_statement();
}