use std::fmt;
use std::ops::Add;
use std::ops::Sub;
use std::cmp::PartialEq;

#[derive(Clone, Copy)]
struct Complex {
    re: f64,         // nombre réel
    im: f64          // nombre imaginaire
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "z = {} + {} * i", self.re, self.im)
    }
}

impl Add for Complex {
    type Output = Complex;

    fn add(self, rhs: Self) -> Self::Output {
        Complex { re : self.re + rhs.re, im : self.im + rhs.im }
    }
}

impl Sub for Complex {
    type Output = Complex;

    fn sub(self, rhs: Self) -> Self::Output {
        Complex { re : self.re - rhs.re, im : self.im - rhs.im }
    }
}

// Can be derived (predefined -> default behavior is enough)
impl PartialEq for Complex {
    fn eq(&self, other: &Self) -> bool {
        self.re == other.re && self.im == other.im
    }
}

pub fn using_operators() {
    let comp_1 = Complex { re: 10.0, im : 5.0 };
    let comp_2 = Complex { re: 4.0, im : 3.0 };

    println!("comp_1 : {}", comp_1);
    println!("comp_2 : {}", comp_2);
    
    println!("Addition : {}", comp_1 + comp_2);
    println!("Soustraction : {}", comp_1 - comp_2);

    println!("Egalité : {}", comp_1 == comp_2);
}
