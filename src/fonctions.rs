
//"fn"      : fonction
//"-> type" : type retour de fonction
//Un paramètre de fonction sera par défaut constant
// dernière instruction pas besoin de "return" (mais on peut si besoin)
//pub : public. Une fonction appelable à l'extérieur du module
//const est stocké dans la mémoire du programme. Il doit être connu au moment de la compilation
//shadowing : une variable est re-définissable, avec le même nom. Si le shadow est fait dans un scope enfant,
//   l'ancienne variable sera de nouveau disponible à la fin de celui-ci.

const ONE_HOUR_IN_SECOND : i32 = 3600;

pub fn call(){
    let p = perimetre_cercle(5.0);
    let mut variable = 0;

    println!("Le périmètre du cercle est de {}cm.", p);    
    println!("Le volume de la sphère est de {}cm.", volume_sphere(5.0));

    // shadowing
    let p = 5;
}

fn perimetre_cercle(rayon : f64) -> f64 {
    assert!(rayon >= 0.0, "Le rayon ({}) devrait être positif.", rayon);

    2.0 * std::f64::consts::PI * rayon
}

fn volume_sphere(rayon : f64) -> f64 {
    4.0 / 3.0 * std::f64::consts::PI * f64::powf(rayon, 3.0)
}



#[test]
fn test_perimetre_cercle() {
    let perimetre = perimetre_cercle(5.0);
    assert!(perimetre > 31.0 && perimetre < 32.0);
}

#[test]
fn test_volume_sphere() {
    let volume = volume_sphere(5.0);
    assert!(volume > 523.0 && volume < 524.0);
}
