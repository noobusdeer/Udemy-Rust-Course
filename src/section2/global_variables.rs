const MEANING_OF_LIFE:u8 = 42; // no fixed address
static Z:i32 = 123;
static mut Z2:i32 = 321;

fn main() {
    println!("{}", MEANING_OF_LIFE);

    println!("{}", Z);

    unsafe  {
        println!("{}", Z2);
        Z2 = 333;
        println!("{}", Z2);
    }   
}