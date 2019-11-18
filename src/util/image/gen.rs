use image::{DynamicImage, FilterType, GenericImage, GenericImageView, Rgba, RgbaImage};
use imageproc::drawing::{draw_hollow_rect_mut, draw_text_mut};
use imageproc::rect::Rect;
use rusttype::{Font, FontCollection, Scale};

#[derive(Clone, Debug)]
pub struct Dimension {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
}

#[derive(Clone)]
pub struct FontSettings {
    pub size: f32,
    pub color: [u8; 4],
}

#[cfg(debug_assertions)]
const DEBUG: bool = true;
#[cfg(not(debug_assertions))]
const DEBUG: bool = false;

const FONT: &'static [u8] = include_bytes!("Oswald.ttf");

fn get_text_width(text: &String, font: &Font, scale: Scale) -> f32 {
    let words = text.split_ascii_whitespace();
    let mut width = 0f32;
    for word in words {
        let mut word_with = 0f32;
        for char in word.chars() {
            word_with += font.glyph(char).scaled(scale).h_metrics().advance_width;
            //word_with += font.glyph(char).scaled(scale).scale().x
        }
        width += word_with;
    }
    return width;
}


pub fn generate_image_text(dimension: &Dimension, font_settings: &FontSettings, bg: &DynamicImage, text: &str) -> RgbaImage {
    let text = text.to_owned();

    let mut img = RgbaImage::new(bg.width(), bg.height());
    copy_image(bg, &mut img);

    let font = FontCollection::from_bytes(&FONT).expect("could not read font").into_font().unwrap();
    let font_size = font_settings.size;
    let mut scale = Scale {
        x: font_size,
        y: font_size,
    };

    if DEBUG {
        draw_hollow_rect_mut(&mut img, Rect::at(dimension.x as i32, dimension.y as i32).of_size(dimension.w, dimension.h), Rgba([0, 255, 0, 255]));
    }

    let mut final_font_height: f32;
    let mut lines: Vec<String>;
    let mut counter = 0;
    loop {
        final_font_height = font.v_metrics(scale).ascent - font.v_metrics(scale).descent;
        lines = Vec::new();
        lines.clear();

        if get_text_width(&text, &font, scale) as u32 > dimension.w {
            let words = text.split_ascii_whitespace();
            let mut t = String::new();
            let mut current_line_width = 0f32;
            for word in words {
                let word = format!("{} ", word);
                let mut word_width = 0f32;
                for char in word.chars() {
                    word_width += font.glyph(char).scaled(scale).h_metrics().advance_width;
                }
                current_line_width += word_width;

                if current_line_width.ceil() as u32 > dimension.w {
                    current_line_width = word_width;
                    lines.push(t);
                    t = String::new();
                }
                t.push_str(&word);
            }
            lines.push(t);
        } else {
            lines.push(text.to_owned());
        }

        if scale.y * lines.len() as f32 > dimension.h as f32 {
            scale = Scale {
                x: scale.x * 0.99f32,
                y: scale.y * 0.99f32,
            };
            counter += 1;
        } else {
            info!("Scaled: {} times", counter);
            break;
        }
    }

// SCALE TEXT DOWN
    /*let line_height = font_height;
    let total_height = line_height * lines.len() as f32;
    if total_height.ceil() as u32 > dimension.h {
        let scale_down_percentage = dimension.h as f32 / total_height;
        scale = Scale {
            x: scale.x * scale_down_percentage,
            y: scale.y * scale_down_percentage,
        }
    }*/


    for (i, line) in lines.into_iter().enumerate() {
        draw_text_mut(&mut img, Rgba(font_settings.color), dimension.x, (dimension.y as f32 + final_font_height * i as f32) as u32, scale, &font, &line);
    }

    img
}

pub fn generate_image_image(dimension: &Dimension, bg: &DynamicImage, other: &DynamicImage) -> RgbaImage {
    let mut img = RgbaImage::new(bg.width(), bg.height());
    copy_image(bg, &mut img);
    let other = other.resize(dimension.w, dimension.h, FilterType::Nearest);
    copy_image_with_offset(&other, &mut img, dimension.x, dimension.y, dimension.w, dimension.h);
    if DEBUG {
        draw_hollow_rect_mut(&mut img, Rect::at(dimension.x as i32, dimension.y as i32).of_size(dimension.w, dimension.h), Rgba([0, 255, 0, 255]));
    }
    img
}

fn copy_image<S: GenericImage<Pixel=D::Pixel>, D: GenericImage>(source: &S, dest: &mut D) {
    copy_image_with_offset(source, dest, 0, 0, source.width(), source.height());
}

fn copy_image_with_offset<S: GenericImage<Pixel=D::Pixel>, D: GenericImage>(source: &S, dest: &mut D, offset_x: u32, offset_y: u32, width_x: u32, width_y: u32) {
    for (x, y, pixel) in source.pixels() {
        let x1 = x + offset_x;
        let y1 = y + offset_y;

        if x <= width_x && y <= width_y {
            if x1 < dest.width() && y1 < dest.height() {
                dest.blend_pixel(x1, y1, pixel);
            }
        }
    }
}