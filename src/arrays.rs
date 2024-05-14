// 3 types :
// - Array          array
// - Vector         list (C#, Java), vector (C++)
// - Slice          part of


// ARRAY (TABLEAU)
// Le tableau doit avoir une taille fixe et ça ne peut pas changer au cours de son existence
// Il est onc naturellement stocké dans la Stack par défaut
pub fn using_arrays() {
    let mut my_array : [i32; 8] = [2, 6, 1, 3, 5, 9, 7, 8];
    let second_array = [false, false, true];        //fonctionne puisque types cohérents (sans typer)

    println!("4e élément : {}", my_array[3]);
    my_array.sort();
    for num in my_array.iter() {
        print!("{}", num);
    }
    // for ii in 0..my_array.len() {
    //     print!("{}", my_array[ii]);
    // }
    println!();
}

// VECTOR (VECTEUR)
// Le vecteur n'a pas de taille fixe (c'est l'intérêt). Le vecteur est donc stocké dans le tas avec un pointeur dans la pile sur le premier élément
// La taille et la capacité du vecteur sont également stockés dans la pile avec le pointeur
// Tous les élements étant les uns à la suite des autres dans le tas, un ajout dans le vecteur provoque :
// - une réallocation mémoire du nouveau total
// - une copie
// - désallocation de l'ancien vecteur
pub fn using_vectors() {
    let mut my_vector = vec![1, 2, 3, 4];       //Vec::new()
    my_vector.push(5);
    my_vector.pop();
    let length = my_vector.len();

    //Création de l'itérateur:
    for value in my_vector.iter() {
        println!("{}", value);
    }

    //Depuis un itérateur on peut retransformer le résultat intermédiaire en vecteur :
    let mut little_vector : Vec<i32> = (10..16).collect();
    little_vector.pop();
    println!("Length : {}", little_vector.len());
    println!("Capacity : {}", little_vector.capacity());
}

// SLICE
// Section d'un tableau ou d'un vecteur
pub fn using_slice() {
    let array = [1, 2, 3, 4, 5];
    let slice = &array[0..2];

    println!("Slice length : {:?}", slice.len());
    println!("Slice : {:?}", slice);
}
