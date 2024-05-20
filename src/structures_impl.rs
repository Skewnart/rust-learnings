
struct Rectangle {
    width: f64,
    height: f64
}

impl Rectangle {
    //Methode de classe (statique) (sans &self) qui fait office de constructeur
    pub fn new(w: f64, h: f64) -> Self {
        Self { width : w, height : h }
    }

    //Deux méthodes d'objets
    pub fn get_perimetre(&self) -> f64 {
        2.0 * (self.width + self.height)
    }

    pub fn get_aire(&self) -> f64 {
        self.width * self.height
    }
}

pub fn using_impl() {
    let rectangle = Rectangle::new(2.0, 3.0);
    println!("Rectangle ; width : {}, height : {}", rectangle.width, rectangle.height);
    println!("Perimetre : {}", rectangle.get_perimetre());
    println!("Aire : {}", rectangle.get_aire());
}
