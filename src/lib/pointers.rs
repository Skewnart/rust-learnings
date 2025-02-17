// 3 types :
// - Reference
// - Boxing
// - Raw pointers (classical, not safe) (pointeurs bruts)

use std::env::consts;

// REFERENCE
//Une référence ne peut pas être nulle.
pub fn using_refs() {
    let pi = std::f64::consts::PI;
    let ref_pi = &pi;

    println!("Référence de pi (vue 0x) : {:p}", &pi);
    println!("Valeur de pi : {}", pi);
    
    println!("Référence de \"ref_pi\" (vue 0x) : {:p}", &ref_pi);
    println!("Valeur de \"ref_pi\" (vue 0x) : {:p}", ref_pi);
    println!("Valeur de \"ref_pi\" : {}", ref_pi);

    //On peut apercevoir que la valeur de ref_pi est en fait la référence de pi (que ce soit dans la pile ou dans le tas)
    //Et que même si on affiche la référence de pi tel quel ({}), elle sera "transcendée" et la valeur de pi est affiché

    let mut pi2 = std::f64::consts::PI;
    let ref_pi2 = &mut pi2;     //&mut se trouve dans l'opérande de droite car ce n'est pas la ref qui est mutable en soi

    /* ! Attention, UNE des règles de base du borrow checker :
        let -> &            :   immutable -> immutable  :   OK normal
        let -> &mut         :   immutable -> mutable    :   NOT OK, DOES NOT COMPILE
        let mut -> &        :   mutable -> immutable    :   OK less permissive
        let mut -> &mut     :   mutable -> mutable      :   OK normal
    */
}

// BOX
// Permet globalement d'utiliser la heap (le tas) plutôt que la stack (pile)
//    même si on connaît la taille mémoire (principe d'utilisation de la stack)
pub fn using_box() {
    let val_ex = 32.0;                              //Stack
    let val_ex_boxed = Box::new(val_ex);       //Heap

    let val_boxed = Box::new(123);             //Heap directly
    //Même si la Box pointe sur une valeur, l'utilisation directe emmène sur la valeur pointée,
    //  on peut donc utiliser la box directement :
    println!("{}", val_boxed);
}

// RAW POINTERS
//  pour du déréférencement, ou accès mémoire de façon non sécurisée
//  faire donc attention, voire ne pas du tout l'utiliser en fait

pub fn using_pointers() {
    //la variable doit être mutable
    let mut var_f64 = 125.0;
    let var_f64_pointer_const = &var_f64 as *const f64;
    let var_f64_pointer_mut = &mut var_f64 as *mut f64;

    unsafe {
        println!("{}", *var_f64_pointer_const); //Déréférencement, on se retrouve donc dans le même cas d'incertitude qu'en C++
        println!("{}", *var_f64_pointer_mut);
    }
}
