#![allow(unused)]

mod variable;
mod types;
mod functions;
mod module;
mod test;
mod structures;
mod structures_impl;
mod pointers;
mod arrays;
mod strings;
mod ownership;
mod structures_generic;
mod structures_references;
mod trait_predefined;
mod enumerations;
mod pattern_matching;
mod traits;
mod trait_operator_overload;
mod char_string;
mod vectors;
mod collections;
mod closures;
mod threads;
mod trait_object_dyn;
mod unsafe_code;
mod generic;
mod control_flow;
mod errors;
mod lifetime;

fn main() {

    //TODO A refaire avec le summary du coup
    
    //no exec                               //1. Cargo command with parameters
    //no exec                               //2. Tests (how to write)

    //variable::using_variables();
    //types::init_var();                    //3. Types
    //functions::call();                    //4. Write functions and variables
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
    //ownership::using_ownership();         //10. Borrow / Owner

                                            //11. Structures :
    //structures_impl::using_impl();        // Impl
    //structures_generic::using_generic();  // Generics
    //structures_references::using_ref();   // References

    //trait_predefined::using_traits();     //12. Traits (prédéfinis)

    //enumerations::using_enum();           //13. Enumerations
    //pattern_matching::using_pattern_matching();     //14. Pattern matching

    //traits::using_traits();               //15. Traits
    //traits::using_traits_2();
    
    //trait_operator_overload::using_operators();     //16. Operators overload (linked to 12. Predefined Traits)

    //char_string::using_chars();           //17. Char, String (String & str)
    //char_string::using_strings();
    //char_string::using_regex();

    //vectors::using_vectors();             //18. Vectors
    //collections::using_collections();     //19. Collections
    //closures::using_closure();            //20. Closures
    //threads::using_threads();             //21. Threads
    //trait_object_dyn::using_trait_object(); //22. Trait Object
    unsafe_code::using_unsafe();          //23. Unsafe
}
