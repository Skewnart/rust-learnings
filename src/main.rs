#![allow(unused)]

mod types;
mod fonctions;
mod module;
mod test;
mod structures;
mod structures_impl;
mod pointers;
mod arrays;
mod strings;
mod borrow;

fn main() {
    //no exec                               //1. Cargo command with parameters
    //no exec                               //2. Tests (how to write)

    //types::init_var();                    //3. Types
    //fonctions_variables::call();          //4. Write functions and variables
    //module::functions::function();        //5. Module management

    //structures::using_structures();       //6. Structures (structure, tuple)
    //structures::using_structure_tuple();
    structures_impl::using_impl();             //6.5 Impl with structures
    
    //pointers::using_refs();               //7. References (reference, box, pointer)
    //pointers::using_box();
    //pointers::using_pointers();
    
    //arrays::using_arrays();               //8. Arrays (array, vector, slice)
    //arrays::using_vectors();
    //arrays::using_slice();

    //strings::using_str();                 //9. Strings
    //borrow::example_borrow_owner();       //10. Borrow / Owner
}
