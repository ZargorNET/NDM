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

    let mut files: Vec<String> = Vec::new();

    for entry in dir {
        let entry = entry?;
        let name = entry.file_name();
        let name = match name.into_string() {
            Ok(s) => s,
            Err(_) => {
                return Err(Error::Other("could not cast OsString to String".to_owned()));
            }
        };
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

    let mut ret = Vec::new();
    'tomlLoop: for file_name in files {
        let toml_file_path = Path::new(path.as_os_str()).join(format!("{}.toml", file_name));
        let mut toml_file = match fs::File::open(toml_file_path) {
            Ok(k) => k,
            Err(e) => {
                warn!("TEMPLATE PARSER: could not open metadata file: {}", e);
                continue;
            }
        };

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

        let mut toml_file_content = String::new();
        toml_file.read_to_string(&mut toml_file_content)?;

        let metadata: TemplateMetadataFile = toml::from_str(&toml_file_content)?;
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
                }
                FeatureType::Image => {
                    if feat.overlay_image_path.is_none() {
                        warn!(r#"TEMPLATE PARSER: missing attribute "{}" for feature "{}" in template "{}" "#, "overlay_image_path", feat.key, &metadata.name);
                        continue 'tomlLoop; // SKIP THIS TEMPLATE
                    }
                }
                _ => {},
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
            });
        }
        ret.push(PartialTemplate::new(
            metadata.name,
            image::load_from_memory(&base_img_buf)?,
            features));
    }

    Ok(ret)
}

#[derive(Serialize, Deserialize)]
struct TemplateMetadataFile {
    name: String,
    features: Vec<TemplateFileFeature>,
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
}

#[derive(Debug)]
pub enum Error {
    PathNotDir,
    InvalidFeatureType,
    IoError(io::Error),
    TomlError(toml::de::Error),
    ImageError(image::ImageError),
    Other(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            Self::InvalidFeatureType => write!(f, "feature type/kind is unknown"),
            Self::PathNotDir => write!(f, "path ist not a directory"),
            Self::IoError(ref e) => e.fmt(f),
            Self::TomlError(ref e) => write!(f, "could not parse metadata file: {}", e),
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
            Self::TomlError(ref e) => Some(e),
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

impl From<toml::de::Error> for Error {
    fn from(e: toml::de::Error) -> Self {
        Error::TomlError(e)
    }
}

impl From<image::ImageError> for Error {
    fn from(e: image::ImageError) -> Self {
        Error::ImageError(e)
    }
}