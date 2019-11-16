#[derive(Clone)]
pub enum Category {
    GeneratedImage,
    Fun,
    Misc,
    Animals,
}

impl ToString for Category {
    fn to_string(&self) -> String {
        match *self {
            Category::GeneratedImage => "Images",
            Category::Fun => "Fun",
            Category::Misc => "Misc",
            Category::Animals => "Animals",
        }.to_string()
    }
}