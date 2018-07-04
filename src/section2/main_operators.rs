
fn operators(){

    //arithmetic
    let mut a = 2+3*4;
    println!("a = {}", a); 
    a += 1;
    println!("a = {}", a); 

    let b = 3;
    let b_cubed = i32::pow(b, 3);
    println!("{} cubed is {}", b, b_cubed);

    let c = 2.5;
    let c_cubed = f64::powi(c, 3);
    let c_to_pi = f64::powf(c, std::f64::consts::PI);
    println!("{} cubed is {}, {}^pi = {}", c, c_cubed, c, c_to_pi);

    //bitwise
    let d = 1 | 2;  // | OR & AND ^  XOR ! NOR
                    // 01 OR 10 = 11 == 3
    println!("d = {}", d);
    let two_to_10 = 1 << 10;
    println!("2^10 = {}", two_to_10);

    //logical
    let pi_less_4 = std::f64::consts::PI < 4.0;
    println!("PI < 4 = {}", pi_less_4);
}

fn main() {
    operators();
}
