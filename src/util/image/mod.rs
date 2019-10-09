use std::error;
use std::path::Path;

use image::{DynamicImage, ImageFormat};

pub use gen::Dimension;
pub use gen::FontSettings;

use crate::util::image::feature::Feature;
use crate::util::image::partial::PartialTemplate;

mod gen;
mod parser;
mod partial;
pub mod feature;

pub struct ImageStorage {
    storage: Vec<PartialTemplate>,
}

impl ImageStorage {
    pub fn load(p: &Path) -> Result<Self, Box<dyn error::Error>> {
        let templates = match parser::parse(p) {
            Ok(k) => k,
            Err(e) => {
                return Err(Box::new(e));
            }
        };
        Ok(Self {
            storage: templates
        })
    }
    pub fn start_building(&self, key: &str) -> Option<PartialTemplate> {
        match self.storage.iter().find(|t| t.key == key) {
            Some(s) => Some(s.clone()),
            None => None
        }
    }
}


pub struct Template {
    pub name: String,
    pub base: DynamicImage,
    pub features: Vec<Box<dyn Feature>>,
}

impl Template {
    pub fn apply(&self) -> Result<Vec<u8>, Box<dyn error::Error>> {
        let mut last: DynamicImage = self.features[0].apply(&self.base);

        for feature in self.features.iter().skip(1) {
            last = feature.apply(&last);
        }

        let mut buf: Vec<u8> = Vec::new();
        last.write_to(&mut buf, ImageFormat::PNG)?;

        Ok(buf)
    }
}