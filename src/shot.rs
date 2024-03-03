use std::env::var_os;

use image::imageops::overlay;
use image::{Rgb, Rgba, RgbaImage};
#[cfg(feature = "wayland")]
use libwayshot::output::OutputPositioning;
#[cfg(feature = "wayland")]
use libwayshot::reexport::Transform;
#[cfg(feature = "wayland")]
use libwayshot::WayshotConnection;

#[derive(Clone, Copy, Debug, Default)]
pub struct Area {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

// #[cfg(feature = "wayland")]
// type ScreenImage = ((Area, Transform), RgbaImage);
// #[cfg(feature = "x11")]
// type ScreenImage = ((Area, ()), RgbaImage);
// 
// #[cfg(feature = "wayland")]
// fn rotate(screen: &RgbaImage, t: Transform) -> RgbaImage {
//     use image::imageops::{rotate180, rotate270, rotate90};
// 
//     match t {
//         Transform::_90 => rotate90(screen),
//         Transform::_180 => rotate180(screen),
//         Transform::_270 => rotate270(screen),
//         _ => screen.clone(),
//     }
// }
// 
// fn make_all_screens(screens: &[ScreenImage]) -> RgbaImage {
//     let max_w = screens.iter().map(|(a, _)| a.0.width).sum();
//     let max_h = screens
//         .iter()
//         .map(|(a, _)| a.0.height)
//         .max()
//         .unwrap_or_default();
//     let mut res = RgbaImage::from_pixel(max_w, max_h, Rgba([0, 0, 0, 255]));
// 
//     for (a, screen_img) in screens {
//         #[cfg(feature = "wayland")]
//         let screen_img = &rotate(screen_img, a.1);
//         overlay(&mut res, screen_img, (a.0.x).into(), (a.0.y).into());
//     }
// 
//     res
// }

pub fn capture(#[cfg(feature = "wayland")] wayshot: &WayshotConnection, area: Area) -> RgbaImage {
    println!("Area: {area:?}");

    let mut x = None;

    #[cfg(feature = "x11")]{
    x = xcap::Monitor::from_point(area.x, area.y)
        .expect(&format!("Not monitor from point ({}, {})", area.x, area.y))
        .capture_image()
        .map(|out| {
            image::imageops::crop_imm(&out, area.x as u32, area.y as u32, area.width, area.height)
                .to_image()
        })
        .ok();
    }

    #[cfg(feature = "x11")]
    return x.expect("Cannot get image from point");

    #[cfg(feature = "wayland")]
    wayshot
        .screenshot(
            libwayshot::CaptureRegion {
                x_coordinate: area.x,
                y_coordinate: area.y,
                width: area.width as i32,
                height: area.height as i32,
            },
            false,
        )
        .ok()
        .or(x)
        .unwrap()
        .into()
}

pub fn generate_border(img: &mut RgbaImage, color: Option<Rgba<u8>>) {
    let (width, height) = img.dimensions();
    // TODO: extract from image
    let color = color.unwrap_or(Rgba([0, 0, 0, 0]));
    let border_thickness = 5;

    // Dibujar el borde en los lados de la imagen
    for x in 0..width {
        for y in 0..border_thickness {
            img.put_pixel(x, y, color);
            img.put_pixel(x, height - y - 1, color);
        }
    }

    for y in 0..height {
        for x in 0..border_thickness {
            img.put_pixel(x, y, color);
            img.put_pixel(width - x - 1, y, color);
        }
    }
}

pub fn round_image(img: &RgbaImage, color: Option<Rgba<u8>>) -> RgbaImage {
    let (width, height) = img.dimensions();
    let border_thickness = 5.;
    let border_radius = (width.max(height) as f32 / 2.) - border_thickness;
    let mut new_img = RgbaImage::new(width, height);

    for x in 0..width {
        for y in 0..height {
            let dx = (x as f32 - width as f32 / 2.0).abs();
            let dy = (y as f32 - height as f32 / 2.0).abs();
            let distance = (dx.powi(2) + dy.powi(2)).sqrt();

            if distance < border_radius {
                let pixel = img.get_pixel(x, y);
                new_img.put_pixel(x, y, pixel.clone());
            }
            if let Some(color) = color {
                if distance >= border_radius - border_thickness
                    && distance < border_radius + border_thickness
                {
                    new_img.put_pixel(x, y, color);
                }
            }
        }
    }

    new_img
}
