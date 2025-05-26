// Le match doit être exhaustif pour ne pas provoquer d'erreur de compilation
// Placeholder "_"

struct Point {
    x: i32,
    y: i32,
}

enum Personnage{ 
    Heros,                                                          // Structure unité
    Fantome { points_de_vie : u32, indice_invisibilite : u32},      // Structure
    Combattant { points_de_vie : u32},                              // Structure
    Magicien(u32, u32),                                             // Structure tuple
}

impl Personnage {
    pub fn display(&self) -> String {
        match self {
            Personnage::Heros => String::from("Le personnage est un héros"),
            Personnage::Fantome { points_de_vie, indice_invisibilite }
                => format!("Fantome : {}, {}", points_de_vie, indice_invisibilite).to_string(),
            Personnage::Combattant { points_de_vie }
                => format!("Combattant : {}", points_de_vie).to_string(),
            Personnage::Magicien (a, b )
                => format!("Magicien : {}, {}", a, b).to_string(),
            _ => String::from("")
        }
    }
}

pub fn using_pattern_matching() {
    let h = Personnage::Heros;
    println!("{}", h.display());

    let f = Personnage::Fantome { points_de_vie: 10, indice_invisibilite: 1 };
    println!("{}", f.display());
    
    let c = Personnage::Combattant { points_de_vie: 5 };
    println!("{}", c.display());
    
    let m = Personnage::Magicien(4, 8);
    println!("{}", m.display());


    let annee = 2010;
    match annee {
        2000 => println!("C'est un 2000"),
        2001..=2009 => println!("Dans les 2000"),
        annee if (2010..2020).contains(&annee) => println!("Dans les 2010"),
        annee if annee >= 2020 && annee < 2030 => println!("Dans les 2020"),
        _ => println!("Autre")
    }

    let prenom = "Hector"; 
    match prenom { 
        "Arthur" | "Sophie" | "Hector" => println!("Quel joli prénom."), 
        _ => println!("Quel prénom joli.") 
    };

    let optional = Some(10);
    match optional {
        Some(value) if value > 5 => println!("Le nombre est supérieur à 5 : {}", value),
        Some(value) => println!("Le nombre est : {}", value),
        _ => println!("Aucun nombre fourni")
    }

    let x = 'c';
    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }

    let p = Point { x: 0, y: 7 };

    let Point { x, y } = p;
}
