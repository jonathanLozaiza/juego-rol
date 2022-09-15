use csv::{ReaderBuilder, StringRecord};
use std::collections::HashMap;
use std::fs;

const FILENAME: &str = "history.csv";

#[derive(Debug)]
struct DatoHistoria {
    tipo_dato: String,
    tag: String,
    texto: String,
    vida: i32,
}

impl DatoHistoria {
    fn new(row: StringRecord) -> DatoHistoria {
        let mut dato: DatoHistoria = DatoHistoria {
            tipo_dato: row.get(0).unwrap().trim().to_string(),
            tag: row.get(1).unwrap().trim().to_string(),
            texto: row.get(2).unwrap().trim().to_string(),
            vida: row.get(3).unwrap().trim().parse().unwrap_or(0),
        };
        dato
    }
}

fn main() {
    let content = fs::read_to_string(FILENAME).unwrap();
    let mut rdr = ReaderBuilder::new()
        .delimiter(b';')
        .from_reader(content.as_bytes());
    let mut dato_historias: HashMap<String, DatoHistoria> = HashMap::new();
    for result in rdr.records() {
        let result = result.unwrap();
        let dato = DatoHistoria::new(result);
        dato_historias.insert(dato.tag.clone(), dato);
    }
    println!("{:?}", dato_historias);
}
