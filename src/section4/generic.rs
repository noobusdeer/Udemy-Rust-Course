/*
struct Point<T,V> {
    x: T,
    y: V
}
*/

struct Point<T> {
    x: T,
    y: T
}

struct Line<T> {
    start: Point<T>,
    end: Point<T>
}

fn generics() {
    let a:Point<f64> = Point { x: 0.0 , y: 0f64};
    let b = Point { x: 1.2 , y: 3.4};

    let myline = Line { start:a, end:b };

    //let c:Point<u16, i32> = Point { x: 0 , y: 0}; 
}

fn main() {
    generics();
}