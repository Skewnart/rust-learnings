// Toute valeur a un propriétaire et un seul
// Quand le propriétaire sort de la portée, la valeur est supprimée

// -> Le propriétaire peut changer : cession
// -> une valeur peut être empruntée (borrowing)
// -> concerne autant une valeur sur la pile que dans le tas

pub fn example_borrow_owner() {
    change_owner();

    transfert_propriete();
    emprunt();
}

fn change_owner() {
    let str = String::from("string");
    let str_2 = str;

    // println!("{}", str);         //Ne va pas compiler, puisque la valeur a changé de propriétaire

    let number = 10;
    let number_2 = number;
    println!("{}", number);         //Compile car le type i32 implémente le trait Clone
                                    //Tout comme : u32, i64, etc, f64, etc, boolean, char, tuples
}


fn transfert_propriete() {
    let str = String::from("chaine");
    let (str_return, size) = obtenir_taille(str);

    println!("{}", str_return);
}

fn obtenir_taille(ch: String) -> (String, usize) { 
    let taille = ch.len(); 
    (ch, taille) 
} 

fn emprunt() {
    let str_ex = String::from("chaine");
    let size = obtenir_taille_2(&str_ex);

    println!("{} : {}", str_ex, size);

    let mut chaine = String::from("hello");
    change_str(&mut chaine);

    println!("{}", chaine);
}

fn obtenir_taille_2(borrow_str : &String) -> usize {
    borrow_str.len()
}

fn change_str(str : &mut String) {
    str.push_str(". Oui.");
}