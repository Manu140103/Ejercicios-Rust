pub struct Libro {
    titulo: String,
    autor: String,
    agno: u32,
}


impl Libro {
    // Constructor
    pub fn new(titulo: String, autor: String, agno: u32) -> Self {
        Libro {
            titulo,
            autor,
            agno,
        }
    }

    pub fn resumen(&self) -> String {
        format!("{}, escrito por {} en el agno {}", self.titulo, self.autor, self.agno)
    }

    pub fn titulo(&self) -> &str {
        &self.titulo
    }

    pub fn autor(&self) -> &str {
        &self.autor
    }

    pub fn agno(&self) -> u32 {
        self.agno
    }
}


fn main() {
    let libro = Libro::new(
        String::from("El Principito"), 
        String::from("Antoine de Saint-Exupéry"), 
        1943
    );
    
    println!("{}", libro.resumen());
}