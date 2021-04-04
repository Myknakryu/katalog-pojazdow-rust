use crate::pojazd;
use crate::menu;
use crate::io::io;

fn menu() {
    const ELEMENTY: [&str; 9] = [
        "Wczytanie z katalogu",
        "Zapis katalogu z pliku",
        "Wprowadzanie nowego samochodu",
        "Wyświetlenie listy pojazdów",
        "Wyświetlenie warunkowe",
        "Wyświetlenie pojedynczego samochodu",
        "Sortowanie",
        "Usuniecie z katalogu",
        "Wyjście",
    ];

    let mut i = 1;

    for element in ELEMENTY.iter() {
        println!("{} {}", i, element);
        i += 1;
    }
}

fn wyswietl_katalog(katalog: &Vec<pojazd::Pojazd>){
    let mut i = 0;
    for pojazd in katalog.iter() {
        println!("{}. {}", i, pojazd.to_string());
        i += 1;
    }
}

pub fn wyswietlanie_warunkowe(katalog: &Vec<pojazd::Pojazd>, czy_automat: i32){
    for element in katalog.iter() {
        if czy_automat == 1 {
            if !element {
                println!("{}", element.to_string());
            }
        }
        else {
            if !(!element){
                println!("{}", element.to_string());
            }
        }
    }
}

pub fn obsluga_menu() {
    let mut katalog: Vec<pojazd::Pojazd> = Vec::new();
    loop {
        menu();
        println!("Wprowadź opcje: ");
        let x = menu::console_input::pobierz_int();
        match x {
            1 => {
                katalog = io::wczytanie_katalogu_csv();
            }
            2 => {
                io::zapis_katalogu_csv(&katalog);
            }
            3 => {
                katalog.push(menu::console_input::wczytaj_pojazd());
            }
            4 => {
                wyswietl_katalog(&katalog);
            }
            5 => {
                println!("Czy skrzynia jest automatyczna (1 - automatyczna): ");
                let czy_automat = menu::console_input::pobierz_int();
                wyswietlanie_warunkowe(&katalog, czy_automat);
            }
            6 => {
                println!("Podaj element do wydrukowania: ");
                let x = menu::console_input::wybierz_zakres(0, (katalog.len() as i32) -1);
                let element = katalog.get(x as usize).expect("Brak elementu");
                println!("{}", element.to_string());
            }
            8 => {
                wyswietl_katalog(&katalog);
                println!("Podaj element do usunięcia: ");
                let x = menu::console_input::wybierz_zakres(0, (katalog.len() as i32) -1);
                katalog.remove(x as usize);
            }
            9 => {
                std::process::exit(0);
            }
            _ => {
                continue;
            }
        }
    }
}
