use std::{error, fmt, fs, io};
use std::io::Read;
use std::path::Path;

use serde::export::Formatter;

use crate::util::image::Dimension;
use crate::util::image::feature::FeatureType;
use crate::util::image::partial::{PartialFeature, PartialTemplate};

const IMAGE_EXTENSIONS: [&'static str; 3] = [".jpg", ".jpeg", ".png"];

pub fn parse(path: &Path) -> Result<Vec<PartialTemplate>, Error> {
    if !path.is_dir() {
        return Err(Error::PathNotDir);
    }
    let dir = path.read_dir()?;

    let mut ret = Vec::new();

    let mut files: Vec<String> = Vec::new();

    for entry in dir {
        let entry = entry?;

        if entry.path().is_dir() {
            ret.append(&mut parse(entry.path().as_path())?);
            continue;
        }

        let name = entry.file_name();
        let name = match name.into_string() {
            Ok(s) => s,
            Err(_) => {
                return Err(Error::Other("could not cast OsString to String".to_owned()));
            }
        };

        if name.starts_with("_") {
            continue;
        }

        let split: Vec<&str> = name.split(".").collect();
        let file_name = split[0];
        let extension: &str = match split.get(1) {
            Some(s) => s,
            None => continue,
        };

        if extension.to_lowercase() == "toml" {
            files.push(file_name.to_owned());
        }
    }


    'tomlLoop: for file_name in files {
        let toml_file_path = Path::new(path.as_os_str()).join(format!("{}.toml", file_name));
        let mut toml_file = match fs::File::open(toml_file_path) {
            Ok(k) => k,
            Err(e) => {
                warn!("TEMPLATE PARSER: could not open metadata file: {}", e);
                continue;
            }
        };

        let mut toml_file_content = String::new();
        toml_file.read_to_string(&mut toml_file_content)?;
        let metadata: TemplateMetadataFile = match toml::from_str(&toml_file_content) {
            Ok(k) => k,
            Err(e) => {
                warn!(r#"TEMPLATE PARSER: template file "{}" could not be parsed! Error: {}"#, &file_name, e);
                continue 'tomlLoop; // SKIP TEMPLATE
            }
        };

        let mut features: Vec<PartialFeature> = Vec::new();

        for feat in metadata.features {
            let kind = match feat.kind.as_str() {
                "text" => FeatureType::Text,
                "split_text" => FeatureType::SplitText,
                "image" => FeatureType::Image,
                "user_image" => FeatureType::UserImage,
                _ => {
                    return Err(Error::InvalidFeatureType);
                }
            };

            if let Some(f) = features.iter().find(|f| &f.key == &feat.key) {
                if f.kind != kind {
                    warn!(r#"TEMPLATE PARSER: template "{}" has at least two features with the same key but with different types!"#, &metadata.name);
                    continue 'tomlLoop;
                }
            }

            // Check if required attributes exist
            match kind {
                FeatureType::Text | FeatureType::SplitText => {
                    let mut skip = false;
                    if feat.font_color.is_none() {
                        warn!(r#"TEMPLATE PARSER: missing attribute "{}" for feature "{}" in template "{}" "#, "font_color", feat.key, &metadata.name);
                        skip = true;
                    }

                    if feat.font_size.is_none() {
                        warn!(r#"TEMPLATE PARSER: missing attribute "{}" for feature "{}" in template "{}" "#, "font_size", feat.key, &metadata.name);
                        skip = true;
                    }

                    if skip {
                        continue 'tomlLoop; // SKIP THIS TEMPLATE
                    }
                },
                FeatureType::Image => {
                    if feat.overlay_image_path.is_none() {
                        warn!(r#"TEMPLATE PARSER: missing attribute "{}" for feature "{}" in template "{}" "#, "overlay_image_path", feat.key, &metadata.name);
                        continue 'tomlLoop; // SKIP THIS TEMPLATE
                    }
                },
                FeatureType::UserImage => {
                    if feat.default_user.unwrap_or_default() == true {
                        if features.len() != 0 {
                            warn!(r#"TEMPLATE PARSER: user_image feature "{}" in template "{}" with attribute default_user = true must be the first feature!"#, feat.key, &metadata.name);
                            continue 'tomlLoop; // SKIP THIS TEMPLATE
                        }
                    }
                }
            }

            let dimension = Dimension {
                x: feat.x,
                y: feat.y,
                w: feat.w,
                h: feat.h,
            };

            features.push(PartialFeature {
                key: feat.key,
                kind,
                dimension,
                font_size: feat.font_size,
                font_color: feat.font_color,
                overlay_image_path: feat.overlay_image_path,
                default_user: feat.default_user,
                grayscale: feat.grayscale,
            });
        }


        let base_img;

        if metadata.empty.is_none() {
            let mut base_img_file = None;
            'extLoop: for extension in IMAGE_EXTENSIONS.iter() {
                let base_img_path = Path::new(path.as_os_str()).join(format!("{}{}", file_name, extension));
                match fs::File::open(base_img_path) {
                    Ok(k) => {
                        base_img_file = Some(k);
                        break 'extLoop;
                    }
                    Err(_e) => {
                        continue 'extLoop;
                    }
                };
            }

            if base_img_file.is_none() {
                warn!("TEMPLATE PARSER: could not find base image to metadata file {}", &file_name);
                continue;
            }
            let mut base_img_file = base_img_file.unwrap();

            let mut base_img_buf = Vec::new();
            base_img_file.read_to_end(&mut base_img_buf)?;

            base_img = image::load_from_memory(&base_img_buf)?;
        } else {
            let empty = metadata.empty.unwrap();
            base_img = image::DynamicImage::ImageRgba8(image::RgbaImage::new(empty.w, empty.h));
        }

        ret.push(PartialTemplate::new(
            metadata.name,
            base_img,
            features));
    }

    Ok(ret)
}

#[derive(Serialize, Deserialize)]
struct TemplateMetadataFile {
    name: String,
    #[serde(default)]
    empty: Option<TemplateFileEmpty>,
    features: Vec<TemplateFileFeature>,
}

#[derive(Serialize, Deserialize)]
struct TemplateFileEmpty {
    w: u32,
    h: u32,
}

#[derive(Serialize, Deserialize)]
struct TemplateFileFeature {
    key: String,
    kind: String,
    x: u32,
    y: u32,
    w: u32,
    h: u32,
    #[serde(default)]
    font_size: Option<f32>,
    #[serde(default)]
    font_color: Option<[u8; 4]>,
    #[serde(default)]
    overlay_image_path: Option<String>,
    #[serde(default)]
    default_user: Option<bool>,
    #[serde(default)]
    grayscale: Option<bool>,
}

#[derive(Debug)]
pub enum Error {
    PathNotDir,
    InvalidFeatureType,
    IoError(io::Error),
    ImageError(image::ImageError),
    Other(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            Self::InvalidFeatureType => write!(f, "feature type/kind is unknown"),
            Self::PathNotDir => write!(f, "path ist not a directory"),
            Self::IoError(ref e) => e.fmt(f),
            Self::ImageError(ref e) => e.fmt(f),
            Self::Other(ref e) => write!(f, "{}", e),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            Self::InvalidFeatureType => None,
            Self::PathNotDir => None,
            Self::IoError(ref e) => Some(e),
            Self::ImageError(ref e) => Some(e),
            Self::Other(_) => None
        }
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::IoError(e)
    }
}

impl From<image::ImageError> for Error {
    fn from(e: image::ImageError) -> Self {
        Error::ImageError(e)
    }
}