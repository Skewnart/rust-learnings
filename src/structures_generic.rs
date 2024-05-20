use std::{default, ptr::null};

struct VectorGeneric<T> {
    vector : Vec<T>
}

impl<T> VectorGeneric<T> {
    fn new() -> Self {
        Self { vector: Vec::new() }
    }

    fn get_length(&self) -> usize {
        self.vector.len()
    }

    fn add(&mut self, element : T) {
        self.vector.push(element);
    }

    // fn get_with_index(&self, index:usize) -> i32 {
    //     if index < self.vector.len() {
    //         self.vector[index]
    //     }
    //     else {
    //         -1
    //     }
    // }
}


pub fn using_generic() {
    let vec_gen = VectorGeneric::<i32>::new();
    
    //On lui laisse deviner le type avec le premier ajout
    let mut vec = VectorGeneric::new();

    println!("{}", vec.get_length());
    vec.add('\0');
    println!("{}", vec.get_length());
    // println!("{}", vec.get_with_index(0));
    // println!("{}", vec.get_with_index(1));
}