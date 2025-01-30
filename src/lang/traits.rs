#[derive(Clone)]
pub enum Language {
    English,
    Hindi,
    Marathi,
    Sanskrit,
}

impl Language {
    pub fn get_print_keyword(&self) -> &str {
        match self {
            Language::English => "print",
            Language::Hindi => "likho",
            Language::Marathi => "liha",
            Language::Sanskrit => "likh",
        }
    }

    pub fn from_code(code: &str) -> Self {
        match code {
            "Hindi" => Language::Hindi,
            "Marathi" => Language::Marathi,
            "Sanskrit" => Language::Sanskrit,
            _ => Language::English,
        }
    }
}
