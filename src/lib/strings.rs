// STRING
// Chaîne de caractères encodés en Unicode UTF-8

pub fn using_str() {
    //1st method
    let mut str_1 = String::new();
    str_1.push('a');
    str_1.push('b');
    str_1.push('c');
    println!("{}", str_1);

    //2nd method
    let str_2 = String::from("def");
    println!("{}", str_2);
    
    //3rd method
    let str_3 = "ghi".to_string();
    println!("{}", str_3);
    
    //Les slices
    let sliced_str = &str_3[1..];
    println!("{}", sliced_str);

    println!("contains \"hi\" : {}", sliced_str.contains("hi"));
    println!("replace \"i\" : {}", sliced_str.replace("i", "L"));

    for s in str_3.split("h") {
        println!("{}", s);
    }

    let mut s1 = String::from("Hello, ");
    let s2 = String::from("world!");

    //s1.push_str(&s2);     , ou
    s1 += &s2;              //Le add du String prend en paramètre une référence, ce qui permet de réutiliser s2 après
    println!("{}", s1);

    s1 = s1 + " - " + &s2;      //Aucune copie particulière n'est faite, il n'y a que des références

    let new_s = format!("{} - {}", s1, s2);     //Même utilisation qu'un println!, sauf que format! n'envoie rien dans la sortie standard, juste il renvoie un string

    //Un caractère Unicode peut être représenté par plusieurs bytes
    for b in "Зд".bytes() {
        println!("{b}");
    }
    // 208, 151, 208, 180

    
}
