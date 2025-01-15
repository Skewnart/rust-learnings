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

    // ^ Utilisation du shadowing : Il est possible de déclarer une variable avec le même nom dans un scope enfant ou courant. Celui-ci prendra la main
    //   jusqu'à la fermeture du scope enfant, où la valeur précédente sera de nouveau disponible.
    //   Le shadowing permet de ne pas utiliser le même type que la variable de base.

    //Lecture de l'entrée utilisateur
    let mut user_input = String::new();
    std::io::stdin().read_line(&mut user_input).expect("Erreur lors de la saisie");     //expect implique un "panic" avec un message custom. -> supposé pas le choix de faire planter.

    //Random number generation :
    //Ajouter la dépendance "rand" au Cargo.toml
    let random_number: u32 = rand::thread_rng().gen_range(1..=100);     // [1, 100], voir le rang operator pour plus de possibilités

    //Une variable constante est une variable qui doit avoir une valeur fixe au moment de la compilation.
    // Tout usage d'une constante se voit attribuer la valeur de la constante directement au moment de la compilation. Il se retrouve donc directement dans le binaire.
    // Cette valeur ne peut pas changer dans le code.
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
}
