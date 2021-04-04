use crate::pojazd;
use std::fs::File;
use std::io::{Write, BufReader, BufRead};
use std::path::Path;


pub fn zapis_katalogu_csv(katalog: &Vec<pojazd::Pojazd>){
    let mut plik = File::create("katalog.csv").expect("Brak uprawnień!");
    for samochod in katalog{
        let csv = format!("{}\n",samochod.to_csv());
        let wartosc = csv.as_bytes();
        match plik.write_all(wartosc){
            Err(e) => println!("Nie można było zapisać pliku! {}", e),
            Ok(_t) => {},
        };
    }
}

pub fn wczytanie_katalogu_csv() -> Vec<pojazd::Pojazd>{
    let sciezka = Path::new("katalog.csv"); 
    let mut katalog: Vec<pojazd::Pojazd> = Vec::new();

    let plik = match File::open(sciezka){
        Err(e) => {
            println!("Błąd wczytywania: {}", e);
            return katalog;
        },
        Ok(file) => file,
    };
    
    let linie = BufReader::new(plik).lines();
    for linia in linie{
        if let Ok(element) = linia {
            let skladowe:Vec<String> = element.split(",").map(|s| s.to_string()).collect(); 
            if skladowe.len() == 6{
                let marka = &skladowe[0];
                let model = &skladowe[1];
                let rocznik: i32 = (&skladowe[2]).parse().unwrap();
                let pojemnosc: i32 = (&skladowe[3]).parse().unwrap();
                let przebieg: i32 = (&skladowe[4]).parse().unwrap();
                let jaka_skrzynia = &skladowe[5];
                let mut skrzynia = pojazd::TypSkrzyni::Manualna;
                if jaka_skrzynia == "Automatyczna"{
                    skrzynia = pojazd::TypSkrzyni::Automatyczna;
                }
                katalog.push(pojazd::Pojazd::new(marka.to_string(), model.to_string(), rocznik, pojemnosc, przebieg, skrzynia));
            }
        }
    }
    return katalog;
}