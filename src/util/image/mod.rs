use std::collections::HashMap;
use std::error;
use std::path::Path;

use image::{DynamicImage, ImageFormat};

pub use gen::Dimension;
pub use gen::FontSettings;

use crate::util::image::feature::Feature;
use crate::util::image::partial::{PartialFeature, PartialTemplate};

mod gen;
mod parser;
pub mod partial;
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
        let storage = &self.storage;
        match storage.iter().find(|t| t.key == key) {
            Some(s) => Some(s.clone()),
            None => None
        }
    }

    pub fn get_all_keys(&self) -> Vec<String> {
        let storage = &self.storage;
        let mut ret = Vec::with_capacity(storage.len());
        for pt in storage.iter() {
            ret.push(pt.key.clone());
        }
        ret
    }

    /// Returns None if the key cannot be found in the Vec
    pub fn get_required_features(&self, key: &str) -> Option<HashMap<String, PartialFeature>> {
        let pt = match self.storage.iter().find(|t| t.key == key) {
            Some(s) => s,
            None => return None
        };

        Some(pt.features.clone())
    }
}


pub struct Template {
    pub name: String,
    pub base: DynamicImage,
    pub features: Vec<Box<dyn Feature + Send + Sync>>,
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