// 2 types :
// - Structure
// - Tuple

//Allow non snake case in property
#![allow(non_snake_case)]

//struct here
struct Person {
    name : String,
    age : i8,
    drivingLicense: bool
}

//struct tuple here
struct PersonTuple (String, i8, bool);

pub fn using_structures() {
    let person = Person { name : String::from("Jarod"), age : 21, drivingLicense : true };

    println!("Name : {}", person.name);
    println!("Age : {}", person.age);

    let mut person2 = Person { name : String::from("Leto"), age : 25, drivingLicense : false };
    person2.drivingLicense = true;              //First method
    person2 = Person { age : 31, ..person2};    //Second method (range operator)


    let persontuple = PersonTuple(String::from("John"), 23, true);
    println!("{}", persontuple.0);
}

//Why using structure tuple ? 
// -> Seen to define some units : (better than use the unit in name)

//Example :

struct Meter(f32);

pub fn using_structure_tuple() {
    let distance = Meter(12.5);

    //by destructuring :
    let Meter(m) = distance; 
    println!("{}", m);

    //by accessing directly, but less readable :
    let m2 = distance.0;
    println!("{}", m2);
}
