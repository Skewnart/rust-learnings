#[derive(Debug)]
enum Departement {
    Ain,
    Aisne,
    Allier,
    Alpes
}

impl Departement {
    pub fn get_id(id: i64) -> Option<Departement> {
        match id {
            0 => Some(Departement::Ain),
            1 => Some(Departement::Aisne),
            _ => None
        }
    }
}

pub fn using_enum() {
    let ain : Departement = Departement::Ain;
    println!("{:?}", ain);

    let allier_i64 = Departement::Allier as i64;
    println!("{}", allier_i64);

    println!("{:?}", Departement::get_id(1));
    println!("{:?}", Departement::get_id(10));
}



