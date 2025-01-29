//Le présent fichier "mod.rs" sera le module lu pour le module "module"
//Il permettra de proposer "functions" à l'extérieur grâce à l'instruction suivante
pub(crate) mod functions;

//"pub" permet d'exposer les différentes fonctions et modules à l'intérieur
//Pour utiliser une fonction ou module ne nous appartenant pas, il faut utiliser le mot clé "use", potentiellement avec un alias ("as")
//Pour utiliser une fonction d'un module parent, utiliser "super::nom_de_la_fonction" ou "super::nom_du_module::nom_de_la_fonction"


//use std::io;
//use std::io::Write;

// =

//use std::io::{self, Write};

//use std::collections::*;