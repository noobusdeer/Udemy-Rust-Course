struct Person {
    name: String
}

impl Person {
    //fn get_ref_name(&self) -> &String {
    fn get_ref_name<'a>(&'a self) -> &'a String {
        &self.name
    }
}

struct Company<'z> {
    name: String,
    ceo: &'z Person
}

fn main() {

    // memory beh
    let print_vector = |x:&Vec<i32>|{
        println!("{:?}", x);
    };
    let v = vec![3,2,1];
    print_vector(&v);
    println!("{}", v[0]);

    let mut a = 40;
    {
        let b = &mut a;
        *b += 2;
    }
    println!("a = {}", a);

    //lifetime
    let boss = Person { name: String::from("Elon Musk") };
    let tesla = Company { name: String::from("Tesla"), ceo: &boss };

}