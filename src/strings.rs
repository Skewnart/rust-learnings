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
}
