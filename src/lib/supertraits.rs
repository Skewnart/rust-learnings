// Les supertraits sont des traits qui ont BESOIN que d'autres traits soient implémentés pour fonctionner
// Ils sont utilisés pour contraindre les types génériques à implémenter certains traits avant de pouvoir utiliser le trait qui les utilise
// Ils sont déclarés dans la définition du trait, et sont séparés par un +

use std::fmt::{Display, Result};

pub fn using_supertraits() {
    let point = Point {x: 5, y: 8};
    point.outline_print();
}

trait OutlinePrint : Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {output} *");
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

struct Point {
    x: i32,
    y: i32
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result {
        write!(f, "Point({}, {})", self.x, self.y)
    }
}

impl OutlinePrint for Point {}
