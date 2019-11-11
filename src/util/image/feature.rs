use image::DynamicImage;

use crate::util::image::gen::{Dimension, FontSettings};

pub trait Feature {
    fn apply(&self, bg: &DynamicImage) -> DynamicImage;
}


#[derive(Clone)]
pub struct TextFeature {
    pub dimension: Dimension,
    pub font: FontSettings,
    pub text: String,
}

impl Feature for TextFeature {
    fn apply(&self, bg: &DynamicImage) -> DynamicImage {
        let img = super::gen::generate_image_text(&self.dimension, &self.font, bg, &self.text);
        DynamicImage::ImageRgba8(img)
    }
}

#[derive(Clone)]
pub struct ImageFeature {
    pub dimension: Dimension,
    pub other: DynamicImage,
}

impl Feature for ImageFeature {
    fn apply(&self, bg: &DynamicImage) -> DynamicImage {
        let img = super::gen::generate_image_image(&self.dimension, bg, &self.other);
        DynamicImage::ImageRgba8(img)
    }
}

#[derive(PartialEq)]
#[derive(Clone)]
pub enum FeatureType {
    Text,
    SplitText,
    Image,
    UserImage,
}