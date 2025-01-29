/*
    TYPE primitifs :
    - i8, i16, i32, i64, i128     integers
    - u8, u16, u32, u64, u128     unsigned integers
    - f32, f64                    float / double
    - isize, usize                size of target
    - bool                        boolean
    - char                        character 

    - tuple                 ! warning, tuple non nommées -> voir "structures" pour les nommées
    - unit tuple            Tuple vide : ()

    - Destructuration
    - Arrays
    - assignation en Decimal, Hexadecimal, Octal, Binary, Byte
*/

pub fn init_var() {
    let var_i32 : i32 = 0;
    let var_f64 : f64 = 0.1;
    let var_isize : isize = 64;
    let var_bool : bool = false;
    let var_char : char = '\0';

    let var = 10;       //Il est bien sûr possible de faire de l'inférence
    let var : i32 = "10".parse().unwrap();     //Le parse fonctionne uniquement parce que la variable est déjà typée. Sinon erreur de compilation
    let var = "10".parse::<i32>().unwrap();    //Dans le cas où la variable n'est pas typée, il faut donner le type explicite pour le parse.

    let var_underscore = 0_i32;

    let var = 98_222;           //Decimal
    let var = 0xff;             //Hexadecimal
    let var = 0o77;             //Octal
    let var = 0b1111_0000;      //Binary
    let var = b'A';             //Byte (u8)

    let var_tuple = (10, false, 15.42);
    println!("{}, {}", var_tuple.0, var_tuple.1);

    let (a, b, c) = var_tuple;      //Destructuration
    
    let var_unit_tuple = ();

    print_type_of(&var_isize);

    let a = [1, 2, 3, 4, 5];                //array
    let a: [i32; 5] = [1, 2, 3, 4, 5];      //Pareil avec le type
    let a = [3; 5];                         // = [3, 3, 3, 3, 3]
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}