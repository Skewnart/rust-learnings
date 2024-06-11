// Actions ayant nécessité d'utiliser un code unsafe :
//  - Déréférencement d'un pointeur brut
//  - Modification d'une variable statique mutable (percue par Rust comme une constante)
//  - Implémenter des traits non sécurisés

pub fn using_unsafe() {
    dereferencing();
    static_mutable();
}

fn dereferencing () {
    let mut number : i64 = 42;

    let ptr1 = &number as *const i64;
    let ptr2 = &mut number as *mut i64;

    unsafe {
        println!("ptr1 : {}", *ptr1);
        println!("ptr1 : {}", *ptr2);
    }
}

fn static_mutable() {
    static mut PI: f64 = 3.1;

    unsafe {
        PI = 3.145;
        println!("{}", PI);
    }
}

unsafe trait Exemple {
    
}

unsafe impl Exemple for i32 {

}
