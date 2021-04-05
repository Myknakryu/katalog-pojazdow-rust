use crate::pojazd;
pub fn sortuj(katalog: &Vec<pojazd::Pojazd>, ktora_kolomna: i32, malejaca: bool, ) -> Vec<pojazd::Pojazd>{
    let mut katalog_kopia = katalog.to_vec();
    match ktora_kolomna{
        0 => {
            katalog_kopia.sort();
        } 
        1 => {
            katalog_kopia.sort_by(|a, b| b.model.cmp(&a.model));
        }
        2 => {
            katalog_kopia.sort_by(|a, b| b.rocznik.cmp(&a.rocznik));
        }
        3 => {
            katalog_kopia.sort_by(|a, b| b.pojemnosc.cmp(&a.pojemnosc));
        }
        4 => {
            katalog_kopia.sort_by(|a, b| b.przebieg.cmp(&a.przebieg));
        }
        _ => {
            return katalog_kopia;
        }
    }
    if malejaca { 
        katalog_kopia.reverse();
    }
    return katalog_kopia;
}