#[derive(PartialEq, Eq)]
pub enum TypSkrzyni {
    Manualna = 0,
    Automatyczna = 1,
}

pub struct Pojazd {
    marka: String,
    model: String,

    rocznik: i32,
    pojemnosc: i32,
    przebieg: i32,

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
