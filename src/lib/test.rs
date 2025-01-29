/*
    #[cfg(test)] : TU uniquement A mettre devant un module, permet de compiler uniquement cette partie à l'utilisation de "cargo test"
    #[test] : À mettre devant une fonction "test"
    #[should_panic] : À mettre devant une fonction qui est censée panic
    #[ignore] : Ignorer un test qui serait trop long de base

    //assert!()
    //assert_eq!()
    //assert_ne!()
    //Result<(), String>    Il est possible d'utiliser Result pour retourner soit pas d'erreur (OK()), soit une erreur (String)
    pour utiliser "eq" et "ne", il faut que le premier objet implémente #[derive(PartialEq, Debug)]

    cargo test --help :     aide pour les arguments de la commande
    cargo test -- --help :  aide pour l'utilisation des tests en tant que tel

    cargo test -- --test-threads=1  : Tests consécutifs (mono-thread)
    cargo test -- --show-output     : Quand il y a des tests qui print, par défaut ça ne les affiche pas
    cargo test -- --ignored         : Tester les tests naturellement ignorés
    cargo test -- --include-ignored : Lancer tous les tests, même ceux ignorés de base
*/

/*
    Spécifique : Tests d'intégration
        # A faire que pour les libs (tester la lib comme si elle était importée dans un projet (d'où le fait que ça ne sert pas pour les binaires))

    PROJECT
    ├── Cargo.lock
    ├── Cargo.toml
    ├── src
    │   └── lib.rs
    └── tests                       //À mettre au même niveau que le dossier "src", automatiquement détecté à cet endroit
        ├── integration_test.rs     //Chaque fichier .rs est considéré comme un crate qui teste la lib
        └── blabla
            └── mod.rs              //Sauf les fichiers blabla/mod.rs
*/

#[derive(Clone, Copy)]
struct Color(u32, u32, u32);

enum Colors {
    Red,
    Blue,
    Yellow,
    TrueColor(Color)
}
impl Colors {
    fn get_color(&self) -> Color {
        match self {
            Colors::Red => Color(255, 0, 0),
            Colors::Blue => Color(0, 0, 255),
            Colors::Yellow => Color(255, 255, 0),
            Colors::TrueColor(color) => *color,
        }
    }

    fn get_panic(number : i32) -> Option<i32> {
        if number > 100 {
            panic!("Should be less or equal than 100");
        }

        Option::Some(number)
    }
}

#[cfg(test)]
mod tests {             //mod tests interne au module en cours
    use super::*;       //Permet notamment de tester les méthodes privées

    #[test]
    fn test_get_color() {
        let red = Colors::Red.get_color();
        assert_eq!(red.0, 255);
    }

    #[test]
    #[should_panic(expected = "Should be less or equal than 100")]
    fn test_get_panic() {
        Colors::get_panic(200);
    }

    #[test]
    fn it_works() -> Result<(), String> {
        let color = Colors::Blue.get_color();
        if color.0 == 0 {
            Ok(())
        }
        else {
            Err(String::from("Error from method"))
        }
    }

    #[test]
    #[ignore]               //on ignore le test s'il est trop long, lançable autrement (--ignored --include-ignored)
    fn too_long_test() {

    }
}
