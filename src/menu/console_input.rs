use crate::pojazd;
use std::io;

pub fn pobierz_string() -> String {
    let mut wejscie = String::new();
    io::stdin().read_line(&mut wejscie).expect("Błąd odczytu!");
    return wejscie.trim().to_string();
}

pub fn pobierz_int() -> i32 {
    loop {
        let przyciety = pobierz_string().trim().to_string();
        match przyciety.parse::<i32>() {
            Ok(i) => {
                return i;
            }
            Err(..) => {
                continue;
            }
        };
    }
}

pub fn wczytaj_pojazd() -> pojazd::Pojazd {
    println!("Podaj markę: ");
    let marka = pobierz_string();
    println!("Podaj model: ");
    let model = pobierz_string();

    println!("Podaj rocznik: ");
    let rocznik = pobierz_int();
    println!("Podaj pojemność: ");
    let pojemnosc = pobierz_int();
    println!("Podaj przebieg: ");
    let przebieg = pobierz_int();

    println!("Czy skrzynia jest automatyczna (1 - automatyczna): ");
    let czy_automat = pobierz_int();
    let mut skrzynia = pojazd::TypSkrzyni::Manualna;
    if czy_automat == 1 {
        skrzynia = pojazd::TypSkrzyni::Automatyczna;
    }
    let nowy_pojazd = pojazd::Pojazd::new(marka, model, rocznik, pojemnosc, przebieg, skrzynia);

    return nowy_pojazd;
}

pub fn wybierz_zakres(min: i32, max: i32) -> i32 {
    loop {
        println!("Wprowadź liczbę z zakresu {} - {}", min, max);
        let x = pobierz_int();
        if x >= min && x <= max {
            return x;
        }
    }
}

pub fn wybierz_sortowanie() -> i32 {
    println!("Wybierz sposób sortowania: ");
    const ELEMENTY: [&str; 5] = [
        "Marka", 
        "Model", 
        "Rocznik", 
        "Pojemność", 
        "Przebieg"
        ];
    let mut i = 0;

    for element in ELEMENTY.iter() {
        println!("{} {}", i, element);
        i += 1;
    }
    let selekcja = wybierz_zakres(0, 4);
    return selekcja;
}

pub fn wybierz_malejaca() -> bool {
    println!("Czy program ma sortować malejąco\n0. Rosnąco \n1. Malejąco");
    let selekcja = wybierz_zakres(0, 1);
    if selekcja == 1{
        return true;
    }
    return false;
}
