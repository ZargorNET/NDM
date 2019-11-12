use image::DynamicImage;

use crate::util::image::{Dimension, FontSettings};
use crate::util::image::feature::FeatureType;

pub struct PartialTemplate {
    pub(super) key: String,
    pub(super) base: DynamicImage,
    pub(super) features: Vec<PartialFeature>,
    pub(super) built_features: Vec<Box<dyn super::feature::Feature + Send + Sync>>,
}

#[derive(Clone)]
pub struct PartialFeature {
    pub key: String,
    pub kind: FeatureType,
    pub dimension: Dimension,
    pub font_size: Option<f32>,
    pub font_color: Option<[u8; 4]>,
    pub overlay_image_path: Option<String>,
    pub default_user: Option<bool>,
    pub grayscale: Option<bool>,
}

impl PartialTemplate {
    pub fn new(key: String, base: DynamicImage, features: Vec<PartialFeature>) -> Self {
        Self {
            key,
            base,
            features,
            built_features: vec![],
        }
    }

    pub fn set_text(&mut self, key: &str, text: String) -> Result<(), error::Error> {
        let pfeatures: Vec<PartialFeature> = self.features.iter().filter(|tp| tp.key == key).cloned().collect();
        self.features.retain(|f| pfeatures.iter().any(|pf| pf.key != f.key));

        if pfeatures.is_empty() {
            return Err(error::Error::KeyNotFound);
        }

        for f in pfeatures {
            if f.kind != FeatureType::Text && f.kind != FeatureType::SplitText {
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
                dimension: f.dimension.clone(),
                font: FontSettings {
                    size: font_size,
                    color: font_color,
                },
                text: text.clone(),
            }));
        }

        Ok(())
    }

    pub fn set_user_image(&mut self, key: &str, other: DynamicImage) -> Result<(), error::Error> {
        let pfeatures: Vec<PartialFeature> = self.features.iter().filter(|tp| tp.key == key).cloned().collect();
        self.features.retain(|f| pfeatures.iter().any(|pf| pf.key != f.key));

        if pfeatures.is_empty() {
            return Err(error::Error::KeyNotFound);
        }

        for f in pfeatures {
            if f.kind != FeatureType::UserImage {
                return Err(error::Error::WrongType);
            }

            let img: DynamicImage;
            if f.grayscale.unwrap_or_default() == true {
                img = other.grayscale();
            } else {
                img = other.clone();
            }

            self.built_features.push(Box::new(super::feature::ImageFeature {
                dimension: f.dimension.clone(),
                other: img,
            }));
        }


        Ok(())
    }

    pub fn set_image(&mut self, key: &str) -> Result<(), error::Error> {
        let pfeatures: Vec<PartialFeature> = self.features.iter().filter(|tp| tp.key == key).cloned().collect();
        self.features.retain(|f| pfeatures.iter().any(|pf| pf.key != f.key));

        if pfeatures.is_empty() {
            return Err(error::Error::KeyNotFound);
        }

        for f in pfeatures {
            if f.kind != FeatureType::Image {
                return Err(error::Error::WrongType);
            }

            let path = match f.overlay_image_path {
                Some(s) => s,
                None => {
                    return Err(error::Error::FeatureAttributeMissing("overlay_image_path"));
                }
            };

            let path = std::path::Path::new(&path);
            if !path.exists() {
                return Err(error::Error::AttributeError("overlay_image_path", "overlay image not found"));
            }

            let file_buf = match std::fs::read(path) {
                Ok(k) => k,
                Err(e) => return Err(error::Error::IoError("overlay_image_path", e))
            };

            let mut img = match image::load_from_memory(&file_buf) {
                Ok(k) => k,
                Err(_) => return Err(error::Error::AttributeError("overlay_image_path", "could not load image with library"))
            };

            if f.grayscale.unwrap_or_default() == true {
                img = img.grayscale();
            }

            self.built_features.push(Box::new(super::feature::ImageFeature {
                dimension: f.dimension.clone(),
                other: img,
            }));
        }


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
        // Attribute name
        FeatureAttributeMissing(&'static str),

        // Attribute name, Error msg
        AttributeError(&'static str, &'static str),
        // Attribute name, Error
        IoError(&'static str, std::io::Error),
    }

    impl std::error::Error for Error {
        fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
            match *self {
                Self::KeyNotFound => None,
                Self::WrongType => None,
                Self::NotAllFeaturesSatisfied => None,
                Self::FeatureAttributeMissing(_) => None,
                Self::AttributeError(_, _) => None,
                Self::IoError(_, ref e) => Some(e)
            }
        }
    }

    impl fmt::Display for Error {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            match *self {
                Self::KeyNotFound => write!(f, "key not found"),
                Self::WrongType => write!(f, "template types do not match"),
                Self::NotAllFeaturesSatisfied => write!(f, "not all features were built"),
                Self::FeatureAttributeMissing(s) => write!(f, "feature attribute missing: {}", s),
                Self::AttributeError(s, ref e) => write!(f, "feature attribute error: {} => {}", s, e),
                Self::IoError(s, ref e) => write!(f, "io error while building feature: {} => {}", s, e.to_string()),
            }
        }
    }
}