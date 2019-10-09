use std::collections::HashMap;

use image::DynamicImage;

use crate::util::image::{Dimension, FontSettings};
use crate::util::image::feature::FeatureType;

pub struct PartialTemplate {
    pub(super) key: String,
    pub(super) base: DynamicImage,
    pub(super) features: HashMap<String, PartialFeature>,
    pub(super) built_features: Vec<Box<dyn super::feature::Feature + Send + Sync>>,
}

#[derive(Clone)]
pub struct PartialFeature {
    pub kind: FeatureType,
    pub dimension: Dimension,
    pub font_size: Option<f32>,
    pub font_color: Option<[u8; 4]>,
}

impl PartialTemplate {
    pub fn new(key: String, base: DynamicImage, features: HashMap<String, PartialFeature>) -> Self {
        Self {
            key,
            base,
            features,
            built_features: vec![],
        }
    }

    pub fn set_text(&mut self, key: &str, text: String) -> Result<(), error::Error> {
        let f = match self.features.remove(key) {
            Some(s) => s,
            None => {
                return Err(error::Error::KeyNotFound);
            }
        };

        if f.kind != FeatureType::Text {
            return Err(error::Error::WrongType);
        }

        let font_size = match f.font_size {
            Some(s) => s,
            None => 24f32
        };

        let font_color = match f.font_color {
            Some(s) => s,
            None => [255, 255, 255, 255]
        };
        self.built_features.push(Box::new(super::feature::TextFeature {
            dimension: f.dimension,
            font: FontSettings {
                size: font_size,
                color: font_color,
            },
            text,
        }));

        Ok(())
    }

    pub fn set_image(&mut self, key: &str, other: DynamicImage) -> Result<(), error::Error> {
        let f = match self.features.remove(key) {
            Some(s) => s,
            None => {
                return Err(error::Error::KeyNotFound);
            }
        };

        if f.kind != FeatureType::Image {
            return Err(error::Error::WrongType);
        }

        self.built_features.push(Box::new(super::feature::ImageFeature {
            dimension: f.dimension,
            other,
        }));

        Ok(())
    }

    pub fn build(self) -> Result<super::Template, error::Error> {
        if !self.features.is_empty() {
            return Err(error::Error::NotAllFeaturesSatisfied);
        }

        Ok(super::Template {
            name: self.key,
            base: self.base,
            features: self.built_features,
        })
    }
}

impl Clone for PartialTemplate {
    fn clone(&self) -> Self {
        PartialTemplate {
            key: self.key.clone(),
            base: self.base.clone(),
            features: self.features.clone(),
            built_features: vec![], // LEAVE BLANK
        }
    }
}

mod error {
    use std::fmt;

    use serde::export::Formatter;

    #[derive(Debug)]
    pub enum Error {
        KeyNotFound,
        WrongType,
        NotAllFeaturesSatisfied,
    }

    impl std::error::Error for Error {}

    impl fmt::Display for Error {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            match *self {
                Self::KeyNotFound => write!(f, "key not found"),
                Self::WrongType => write!(f, "template types do not match"),
                Self::NotAllFeaturesSatisfied => write!(f, "not all features were built"),
            }
        }
    }
}