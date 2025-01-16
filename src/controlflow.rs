/*
    - if, else if, else
    - if est une expression, il peut assigner une valeur
    - loop (+ expression + loop labels)
    - while
    - for
*/

pub fn using_controlflows() {
    let number = 10;

    if number < 5 {
        println!("Condition true");
    } else if number == 5 {
        println!("Other condition true (number equals 5)");
    } else {
        println!("Condition false");
    }

    //if est une expression
    let number = if number > 10 { 15 } else { 5 };

    //Boucle à break
    loop {
        for i in 1..=10 {
            println!("Number : {number}");
        }
        break;
    }

    //Loop peut renvoyer une valeur
    let mut counter = 0;
    let number = loop {
        counter += 1;
        if counter == 10 {
            break counter;
        }
    }

    //Loop labels
    'looplabel : loop {
        loop {
            break 'looplabel;
        }
    }

    //While
    let mut number = 3;
    while number != 0 {
        println!("{number}!");
        number -= 1;
    }
    
    //For
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}
