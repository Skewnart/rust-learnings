/*
    #[cfg(test)] : A mettre devant un module, permet de compiler uniquement cette partie à l'utilisation de "cargo test"
    #[test] : À mettre devant une fonction pour les tests unitaires
    #[should_panic] : À mettre devant une fonction qui est censée panic

    //assert!()
    //assert_eq!()
    //assert_ne!()
    //Result<(), String>    Il est possible d'utiliser Result pour retourner soit pas d'erreur (OK()), soit une erreur (String)

    pour utiliser "eq" et "ne", il faut que le premier objet implémente #[derive(PartialEq, Debug)]
*/

struct Color(u32, u32, u32);

enum Colors {
    RED,
    BLUE,
    YELLOW,
    TrueColor(Color)
}
impl Colors {
    fn get_color(&self) -> Color {
        match self {
            Colors::RED => Color(255, 0, 0),
            Colors::BLUE => Color(0, 0, 255),
            Colors::YELLOW => Color(255, 255, 0),
            Colors::TrueColor(color) => color,
        }
    }

    fn get_panic(number : i8) -> Option<i8> {
        if number > 100 {
            panic!("Error");
        }

        Option::Some(number)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_color() {
        let red = Colors::RED.get_color();
        assert_eq!(red.0, 255);
    }

    #[test]
    #[should_panic(expected = "Should be less or equal than 100")]
    fn test_get_panic() {
        Colors::get_panic(200);
    }

    #[test]
    fn it_works() -> Result<(), String> {
        let color = Colors::BLUE.get_color();
        if color.0 == 0 {
            Ok(())
        }
        else {
            Err(String::from("Error from method"))
        }
    }
}
