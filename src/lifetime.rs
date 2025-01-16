/*
    - Ne jamais renvoyer une référence en dehors de son scope (dangling reference)

    - Elision : Dans 95% des cas, les développeurs font tout le temps la même chose. A été décidé de générer les lifetimes tout seul suivant certains cas
    3 règles d'élision :
        input :
        - Tous les refs inputs prennent des lifetimes différents
        output :
        - si l'output est une ref et qu'il y a qu'un input, il prend son lifetime
        - si l'output est une ref et qu'il y a plusieurs inputs dont un Self, il prend le lifetime du Self
    Si après ces trois règles, un lifetime est encore manquant, erreur de compilation

    - static lifetime : 'static. Sa durée de vie est celle du programme.
*/

pub fn using_lifetimes() {
    let str1 = "Hello";
    let str2 = "World !";

    let str3 = longest(str1, str2);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn function_mut(x: &'a mut i32, y: &'a mut i32) -> &'a mut i32 {
    return x;
}