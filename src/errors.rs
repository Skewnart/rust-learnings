/*
    - Result<Ok, Err>
    - '?' operateur de propagation
    - Quand on doit recouvrir d'une erreur, utiliser Result 
    - Quand on peut pas (pas censé), panic! et assimilté (unwrap)
    - unwrap_or, unwrap_or_else, unwrap_err
*/

use std::fs::File;

pub fn using_errors() {
    let file_result = File::open("hello.txt");

    let file = match file_result {
        Ok(_file) => _file,
        Err(error) => panic!("Problem opening the file: {error:?}"),
    };

    // Pareil que :
    let file = file_result.unwrap_or_else(|error| { panic!("Problem opening the file: {error:?}"); });

    //Pareil que :
    let file = file_result.expect("Failed to open the file");

    //plus ou moins pareil que :
    let file = file_result?;
}