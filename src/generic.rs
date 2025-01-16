/*

*/

pub fn using_generic() {
    let p1 = Point { x: 5, y: 4 };
    let p2 = Point<i8> { x: 10, y: 8 };
    
    println!("p1.x = {}", p1.x());

}


//Exemple :

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}