
//"let"     : constant
//"let mut" : variable

//TYPE primitifs :
//i8, i16, i32, i64     integers
//u8, u16, u32, u64     unsigned integers
//f32, f64              float / double
//isize, usize          size of target
//bool                  boolean
//char                  character

//tuple                 ! warning, tuple non nommées -> voir "structures" pour les nommées
//unit tuple            Tuple vide : ()

pub fn init_var() {
    let var_i32 : i32 = 0;
    let var_f64: f64 = 0.1;
    let var_isize : isize = 64;
    let var_bool : bool = false;
    let var_char: char = '\0';

    let var_tuple = (10, false, 15.42);
    println!("{}, {}", var_tuple.0, var_tuple.1);
    
    let var_unit_tuple = ();

    print_type_of(&var_isize);
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}