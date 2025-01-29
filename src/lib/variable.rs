
/*
    - "let" immuable par défaut
    - "let mut" muable 
    - &str (string slice) =/= String
    - Shadowing
    - const
*/

use rand::Rng;

pub fn using_variables () {
    let a = 10;         //Variable classique, immuable par défaut
    let mut b = 20;     //Variable muable avec le mot-clé mut

    println!("a = {}", a);
    println!("a = {a}");

    //Peut-être à déplacer au futur dans une partie String
    let string = "Hello, World!";                   //&str (AKA string slice)

    let string = String::from("Hello, World!");     //String    (&str -> String)
    let string = "Hello, World!".to_string();       //String

    let string = &string[..];                       //re &str   (String -> &str)

    // ^ SHADOWING : une variable est re-définissable, avec le même nom. Si le shadow est fait dans un scope enfant
    //   L'ancienne variable sera de nouveau disponible à la fin de celui-ci.
    //   Il n'est pas obligatoire d'utiliser le même type que la variable de base.

    //Lecture de l'entrée utilisateur
    let mut user_input = String::new();
    std::io::stdin().read_line(&mut user_input).expect("Erreur lors de la saisie");     //expect implique un "panic" avec un message custom. -> supposé pas le choix de faire planter.

    //Random number generation :
    //Ajouter la dépendance "rand" au Cargo.toml
    let random_number: u32 = rand::thread_rng().gen_range(1..=100);     // [1, 100], voir le rang operator pour plus de possibilités

    //const est stocké dans la mémoire du programme. Il doit être connu au moment de la compilation
    // Utiliser une const voit sa valeur attribuée au moment de la compilation.
    // Valeur immuable
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
}
