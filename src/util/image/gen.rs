use image::{DynamicImage, GenericImage, GenericImageView, Rgba, RgbaImage};
use imageproc::drawing::{draw_hollow_rect_mut, draw_text_mut};
use imageproc::rect::Rect;
use rusttype::{FontCollection, Scale};

#[derive(Clone)]
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

const DEBUG: bool = true;
const FONT: &'static [u8] = include_bytes!("Oswald.ttf");

pub fn generate_image_text(dimension: &Dimension, font_settings: &FontSettings, bg: &DynamicImage, text: &str) -> RgbaImage {
    let mut img = RgbaImage::new(bg.width(), bg.height());
    copy_image(bg, &mut img);

    let font = FontCollection::from_bytes(&FONT).expect("could not read font").into_font().unwrap();

    if DEBUG {
        draw_hollow_rect_mut(&mut img, Rect::at(dimension.x as i32, dimension.y as i32).of_size(dimension.w, dimension.h), Rgba([0, 255, 0, 255]));
    }
    let mut font_size = font_settings.size;
    let mut char_width = font_size;
    let mut char_height = font_size;

    let width = char_width * text.chars().count() as f32;

    if width > dimension.w as f32 {
        font_size = (font_size * dimension.w as f32 / width).floor();
        char_width = font_size;
        char_height = font_size;
    }

    let scale = Scale {
        x: char_width,
        y: char_height,
    };

    draw_text_mut(&mut img, Rgba(font_settings.color), dimension.x, dimension.y, scale, &font, &text);

    img
}

pub fn generate_image_image(dimension: &Dimension, bg: &DynamicImage, other: &DynamicImage) -> RgbaImage {
    let mut img = RgbaImage::new(bg.width(), bg.height());
    copy_image(bg, &mut img);
    copy_image_with_offset(other, &mut img, dimension.x, dimension.y, dimension.w, dimension.h);
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