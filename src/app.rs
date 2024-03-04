use image::{Rgba, RgbaImage};

use crate::shot::{capture, generate_border};
use crate::utils::str_to_color;
use crate::Config;

pub struct MainApp {
    alt: bool,
    pos: (i32, i32),
    size: (u32, u32),
    scale_size: i32,
    scale_factor: i32,
    config: Config,
    border_color: Option<Rgba<u8>>,
    img_origin: RgbaImage,
    img: RgbaImage,
}

#[derive(Debug, Clone)]
pub enum MainMessage {
    Move(i32, i32),
    Resize((u32, u32)),
    ZoomIn,
    ZoomOut,
    AltKey(bool),
}

#[derive(Debug, Clone)]
pub enum Command {
    None,
    Resize(i32, i32),
}

impl MainApp {
    pub fn new(config: Config, (x, y): (i32, i32)) -> Self {
        #[cfg(feature = "wayland")]
        let wayshot = libwayshot::WayshotConnection::new().unwrap();
        let border_color = config.border_color.as_deref().and_then(str_to_color);

        let img = capture(
            #[cfg(feature = "wayland")]
            &wayshot,
            (x, y),
        );
        Self {
            img_origin: img.clone(),
            img,
            alt: false,
            pos: (0, 0),
            scale_factor: 0,
            scale_size: 0,
            size: (config.width.unwrap_or(400), config.height.unwrap_or(200)),
            config,
            border_color,
        }
    }

    pub fn update(&mut self, msg: &MainMessage) -> Command {
        match msg {
            MainMessage::Move(x, y) => self.pos = (*x, *y),
            MainMessage::Resize(size) => {
                self.size = *size;
                self.img =
                    image::imageops::crop_imm(&self.img_origin, 0, 0, size.0, size.1).to_image();
            }
            MainMessage::AltKey(pressed) => self.alt = *pressed,
            MainMessage::ZoomIn => {
                if self.alt {
                    self.scale_size += 1;
                    return Command::Resize(
                        self.size.0 as i32 + self.scale_size,
                        self.size.1 as i32 + self.scale_size,
                    );
                } else {
                    if self.scale_factor < 20 {
                        self.scale_factor += 1;
                    }
                }
            }
            MainMessage::ZoomOut => {
                if self.alt {
                    self.scale_size -= 1;
                    return Command::Resize(
                        self.size.0 as i32 + self.scale_size,
                        self.size.1 as i32 + self.scale_size,
                    );
                } else {
                    if self.scale_factor > -20 {
                        self.scale_factor -= 1;
                    }
                }
            }
        }
        Command::None
    }

    pub fn render(&self) -> Option<RgbaImage> {
        let (x, y) = self.pos;
        let zoom_range = (self.config.zoom_area.unwrap_or(50) as i32 + self.scale_factor) as u32;
        let mut img = self.img.clone();
        let res = image::imageops::crop_imm(
            &img,
            x as u32 - (zoom_range / 2),
            y as u32 - (zoom_range / 2),
            zoom_range,
            zoom_range,
        )
        .to_image();

        let mut res =
            image::imageops::resize(&res, 400, 200, image::imageops::FilterType::Gaussian);
        generate_border(&mut res, self.border_color.clone());
        image::imageops::overlay(&mut img, &res, (x - 200).into(), (y - 100).into());
        Some(img)
    }
}
