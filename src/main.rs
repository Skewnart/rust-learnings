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
mod structures_generic;
mod structures_references;
mod trait_predefined;
mod enumerations;
mod motifs;
mod traits;
mod trait_operator_overload;
mod char_string;
mod vectors;
mod collections;
mod closures;
mod threads;
mod trait_object_dyn;
mod unsafe_code;

fn main() {
    //no exec                               //1. Cargo command with parameters
    //no exec                               //2. Tests (how to write)

    //types::init_var();                    //3. Types
    //fonctions::call();                    //4. Write functions and variables
    //module::functions::function();        //5. Module management

    //structures::using_structures();       //6. Structures (structure, tuple)
    //structures::using_structure_tuple();
    
    //pointers::using_refs();               //7. References (reference, box, pointer)
    //pointers::using_box();
    //pointers::using_pointers();
    
    //arrays::using_arrays();               //8. Arrays (array, vector, slice)
    //arrays::using_vectors();
    //arrays::using_slice();
    
    //strings::using_str();                 //9. Strings
    //borrow::example_borrow_owner();       //10. Borrow / Owner

                                            //11. Structures :
    //structures_impl::using_impl();        // Impl
    //structures_generic::using_generic();  // Generics
    //structures_references::using_ref();   // References

    //trait_predefined::using_traits();     //12 a. Traits (prédéfinis)

    //enumerations::using_enum();           //13. Enumerations
    //motifs::using_pattern_matching();     // Pattern matching

    //traits::using_traits();               //14. Traits
    //traits::using_traits_2();
    
    //trait_operator_overload::using_operators();     //12 b. Operators overload

    //char_string::using_chars();           //15. Char, String (String & str)
    //char_string::using_strings();
    //char_string::using_regex();

    //vectors::using_vectors();             //16. Vectors
    //collections::using_collections();     //17. Collections
    //closures::using_closure();            //18. Closures
    //threads::using_threads();             //19. Threads
    //trait_object_dyn::using_trait_object(); //20. Trait Object
    unsafe_code::using_unsafe();          //21. Unsafe
}
