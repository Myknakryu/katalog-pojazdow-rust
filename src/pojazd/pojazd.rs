use std::ops;

#[derive(PartialEq, Eq, Copy, Clone, Ord, PartialOrd)]
pub enum TypSkrzyni {
    Manualna = 0,
    Automatyczna = 1,
}

#[derive(Clone,Eq, Ord, PartialEq, PartialOrd)]
pub struct Pojazd {
    pub(in crate::pojazd) marka: String,
    pub(in crate::pojazd) model: String,

    pub(in crate::pojazd) rocznik: i32,
    pub(in crate::pojazd) pojemnosc: i32,
    pub(in crate::pojazd) przebieg: i32,

    skrzynia: TypSkrzyni,
}

impl Pojazd {
    pub fn new( marka: String, model: String, rocznik: i32, pojemnosc: i32, przebieg: i32, skrzynia: TypSkrzyni) -> Self {
        Pojazd {
            marka: marka,
            model: model,
            rocznik: rocznik,
            pojemnosc: pojemnosc,
            przebieg: przebieg,
            skrzynia: skrzynia,
        }
    }
    pub fn to_string(&self) -> String {
        return (format!(
            "{} {} - {} - {} cm³ - {} km - {}",
            self.marka,
            self.model,
            self.rocznik,
            self.pojemnosc,
            self.przebieg,
            if self.skrzynia == TypSkrzyni::Automatyczna {
                "Automatyczna"
            } else {
                "Manualna"
            }
        ))
        .to_string();
    }
    pub fn to_csv(&self) -> String{
        return (format!(
            "{},{},{},{},{},{}",
            self.marka,
            self.model,
            self.rocznik,
            self.pojemnosc,
            self.przebieg,
            if self.skrzynia == TypSkrzyni::Automatyczna {
                "Automatyczna"
            } else {
                "Manualna"
            }
        ))
        .to_string();
    }
}

impl ops::Not for &Pojazd{
    type Output = bool;
    fn not(self) -> bool{
        if self.skrzynia == TypSkrzyni::Automatyczna {
            return true;
        }
        else {
            return false;
        }
    }
}