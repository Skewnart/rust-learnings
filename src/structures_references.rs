
// REGLES IMMUABLES
// 1. Une ref ne peut pas vivre plus longtemps que l'objet qu'elle pointe
// 2. &mut est exclusive, une et une seule ref mutable

//Cela met en exergue le besoin d'avoir l'information de la durée de vie quand une structure intègre une reference.
// Exemple ci-dessous, avec le "lifetime specifier" : '
// Avec son nom ici : a

struct Exemple<'a> {
    ii: &'a i64
}

struct Exemple2 {
    ii: i32
}

struct Exemple3 {
    ii: &'static i32
}

pub fn using_ref() {
    //Dans cet exemple, "integer" et "exmpl" qui seront dans la pile (cas classique), exmpl sera bien détruit en premier, et donc la référence
    // vers "integer" avant l'objet lui-même

    let integer = 1;
    let exmpl = Exemple { ii : &integer };
    let ref_exmpl = &exmpl;

    // Ci-après, l'exemple du lifetime specifier "static" qui énonce une durée de vie égale à celle du programme

    static EXMPL_2 : Exemple2 = Exemple2 { ii : 10 };
    let ref_static_valeur2: &'static Exemple2 = &EXMPL_2; 

    // Ci-après un exemple de propriété static
    static INT_STATIC : i32 = 10;       // A noter qu'une variable statique doit être typée sans inférence
    let exmpl_3 = Exemple3 { ii : &INT_STATIC };
    let ref_exmpl_3 = &exmpl_3;

}
