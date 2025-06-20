/*
    3 types de closures :

    - FnOnce : Closure qui ne peut être appelée qu'une fois. Peut prendre la propriété des variables        T
    - Fn : Closure qui "emprunte" les variables de façon immuable. Rend la propriété à la fin.              &T
    - FnMut : Closure qui "emprunte" les variables de façon muable. Rend la propriété à la fin.             &mut T
*/

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: i32
}

pub fn using_closure() {
    let mut vector_1 = vec![1, 4, 3, 6, 1];
    vector_1.retain(|&el| el % 2 == 0);

    let est_nombre_pair = | x : i32 | x % 2 == 0;
    println!("{}", est_nombre_pair(2));
    println!("{}", est_nombre_pair(3));

    let mut vector_2 = [Person {name : "John", age : 12 }, Person {name : "Dim", age : 25 }, Person {name : "Doe", age : 8 }];
    vector_2.sort_by_key(| el | el.age);
    println!("{:?}", vector_2);

    let mul_by_2 = | x: i32 | -> i32 { x * 2 };
    println!("10 : {}", mul_by_2(10));
    println!("40 : {}", mul_by_2(40));

    let vecteur_cible = vec![10, 20, 30, 40, 50];  
    let comparaison_vecteur_cible = move |vect: Vec<i32>| vect == vecteur_cible;
    
    let vecteur_1 = vec![10, 20, 30, 45, 50]; 
    println!("Comparaison vecteur_1 : {}", comparaison_vecteur_cible(vecteur_1)); 
    
    let vecteur_2 = vec![10, 20, 30, 40, 50]; 
    println!("Comparaison vecteur_2 : {}", comparaison_vecteur_cible(vecteur_2)); 
}

pub fn using_function_pointers() {
    println!("{}", do_twice(add_one, 5));
    println!("{}", return_closure_add_one()(5));
}

fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(f(arg))
}

fn return_closure_add_one() -> impl Fn(i32) -> i32 {
    |x| x + 1
}