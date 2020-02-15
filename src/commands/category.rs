use std::fmt::Display;

use serde::export::fmt::Error;
use serde::export::Formatter;

#[derive(Clone, PartialEq)]
pub enum Category {
    GeneratedImage,
    Fun,
    Misc,
    Animals,
}

impl Category {
    /*pub fn show_on_help(&self) -> bool {
        match *self {
            Self::GeneratedImage => false,
            Self::Fun => true,
            Self::Misc => true,
            Self::Animals => true,
        }
    }*/

    pub fn get_category_emoji(&self) -> &str {
        match *self {
            Self::GeneratedImage => "🖼",
            Self::Fun => "😄",
            Self::Misc => "❔",
            Self::Animals => "😺",
        }
    }
}

impl Display for Category {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match *self {
            Self::GeneratedImage => write!(f, "Images"),
            Self::Fun => write!(f, "Fun"),
            Self::Misc => write!(f, "Misc"),
            Self::Animals => write!(f, "Animals"),
        }
    }
}