/*
    - Result<Ok, Err>
    - '?' operateur de propagation
    - Quand on doit recouvrir d'une erreur, utiliser Result 
    - Quand on peut pas (pas censé), panic! et assimilté (unwrap)
    - unwrap_or, unwrap_or_else, unwrap_err
*/

use std::{fs::File, io::Error};

pub fn using_errors() -> Result<File, Error>{
    let file_result = File::open("hello.txt");

    let file = match file_result {
        Ok(_file) => _file,
        Err(error) => panic!("Problem opening the file: {error:?}"),
    };

    // Pareil que :
    let file = File::open("hello.txt").unwrap_or_else(|error| { panic!("Problem opening the file: {error:?}"); });

    //Pareil que :
    let file = File::open("hello.txt").expect("Failed to open the file");

    //plus ou moins pareil que :
    let file = File::open("hello.txt")?;

    Ok(file)
}