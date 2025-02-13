use std::{collections::BTreeMap, process};

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

pub fn run(number : i8) {

    let mut btreemap: BTreeMap<i8, (&str, Box<dyn Fn()>)> = BTreeMap::new();

    btreemap.insert(1, ("Cargo", Box::new(|| {println!("Go check Cargo file")})));
    btreemap.insert(2, ("Variables", Box::new(variable::using_variables)));
    btreemap.insert(3, ("Types", Box::new(types::init_var)));
    btreemap.insert(4, ("Arrays", Box::new(|| { arrays::using_arrays(); arrays::using_vectors(); arrays::using_slice(); })));
    btreemap.insert(5, ("Functions", Box::new(functions::call)));
    btreemap.insert(6, ("Control flow", Box::new(control_flow::using_controlflows)));
    btreemap.insert(7, ("Ownership", Box::new(ownership::using_ownership)));
    btreemap.insert(8, ("Structure", Box::new(|| { structures::using_structures(); structures::using_structure_tuple(); })));
    btreemap.insert(9, ("Structure impl", Box::new(|| { structures_impl::using_impl(); structures_generic::using_generic(); structures_references::using_ref(); })));
    btreemap.insert(10, ("Enum", Box::new(enumerations::using_enum)));
    btreemap.insert(11, ("Pattern matching", Box::new(pattern_matching::using_pattern_matching)));
    btreemap.insert(12, ("Modules", Box::new(module::functions::function)));
    btreemap.insert(13, ("Vectors", Box::new(vectors::using_vectors)));
    btreemap.insert(14, ("String", Box::new(strings::using_str)));
    btreemap.insert(15, ("Char / String", Box::new(|| { char_string::using_chars(); char_string::using_strings(); char_string::using_regex(); })));
    btreemap.insert(16, ("Collections", Box::new(collections::using_collections)));
    btreemap.insert(17, ("Errors", Box::new(|| { errors::using_errors(); })));
    btreemap.insert(18, ("Generic", Box::new(generic::using_generic)));
    btreemap.insert(19, ("Traits", Box::new(|| { traits::using_traits(); traits::using_traits_2(); })));
    btreemap.insert(20, ("Predefined traits", Box::new(trait_predefined::using_traits)));
    btreemap.insert(21, ("Operator overload", Box::new(trait_operator_overload::using_operators)));
    btreemap.insert(22, ("Lifetime", Box::new(lifetime::using_lifetimes)));
    btreemap.insert(23, ("Tests", Box::new(|| { println!("Go check Tests file"); } )));

    // À remettre ou enlever :
    //closures
    //pointers
    //threads
    //trait_object_dyn
    //unsafe_code

    if !btreemap.contains_key(&number) {
        eprintln!("Merci de mettre un numéro correct en paramètre.");

        for (index, tuple) in btreemap {
            eprintln!("{} : {}", index, tuple.0);
        }
        process::exit(1);
    }

    btreemap[&number].1();
}
