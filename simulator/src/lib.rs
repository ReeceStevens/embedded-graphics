//! # Embedded graphics simulator
//!
//! ![It can display all sorts of embedded-graphics test code.](https://raw.githubusercontent.com/jamwaffles/embedded-graphics/master/assets/simulator-demo.png)
//!
//! The simulator can be used to test and debug
//! [embedded-graphics](https://crates.io/crates/embedded-graphics) code, or produce snazzy examples
//! for people to try drivers out without needing physical hardware to run on.
//!
//! # Setup
//!
//! The simulator uses SDL and its development libraries which must be installed to build and run
//! it.
//!
//! ## Linux (`apt`)
//!
//! ```bash
//! sudo apt install libsdl2-dev
//! ```
//!
//! ## macOS (`brew`)
//!
//! ```bash
//! brew install sdl2
//! ```
//!
//! ## Windows
//!
//! The Windows install process is a bit more involved, but it _does_ work. See [the SDL2
//! wiki](https://wiki.libsdl.org/Installation#WinRT.2FWindows_8.2FWinPhone) for instructions.
//!
//! # Examples
//!
//! ## Simulate a 128x64 SSD1306 OLED
//!
//! ```rust,no_run
//! use embedded_graphics::prelude::*;
//! use embedded_graphics::{egcircle, egline, text_6x8};
//! use embedded_graphics::pixelcolor::BinaryColor;
//! use embedded_graphics_simulator::{DisplayBuilder, BinaryColorTheme, SimulatorEvent};
//! use std::thread;
//! use std::time::Duration;
//!
//! fn main() {
//!     let mut display = DisplayBuilder::new()
//!         .theme(BinaryColorTheme::OledBlue)
//!         .size(128, 64)
//!         .build_binary();
//!
//!     text_6x8!("Hello World!").draw(&mut display);
//!
//!     egcircle!((96, 32), 31, stroke_color = Some(BinaryColor::On)).draw(&mut display);
//!
//!     egline!((32, 32), (1, 32), stroke_color = Some(BinaryColor::On)).translate(Point::new(64, 0)).draw(&mut display);
//!     egline!((32, 32), (40, 40), stroke_color = Some(BinaryColor::On)) .translate(Point::new(64, 0)).draw(&mut display);
//!
//!     loop {
//!         let end = display.run_once();
//!
//!         if end {
//!             break;
//!         }
//!
//!         for event in display.get_input_events() {
//!             if let SimulatorEvent::MouseButtonUp { point, ..} = event {
//!                 println!("Click event at ({}, {})", point.x, point.y);
//!             }
//!         }
//!         thread::sleep(Duration::from_millis(200));
//!     }
//! }
//! ```

#![deny(missing_docs)]

mod display_builder;
mod display_theme;
mod window;

pub use crate::display_builder::DisplayBuilder;
pub use crate::display_theme::BinaryColorTheme;
pub use crate::window::SimulatorEvent;
use crate::window::Window;
use embedded_graphics::drawable::Pixel;
use embedded_graphics::pixelcolor::{BinaryColor, Rgb888, RgbColor};
use embedded_graphics::prelude::*;

struct PixelData<C> {
    pub width: usize,
    pub height: usize,
    data: Box<[C]>,
}

impl<C> PixelData<C>
where
    C: PixelColor + From<BinaryColor>,
{
    fn new(width: usize, height: usize) -> Self {
        let data = vec![BinaryColor::Off.into(); width * height];

        Self {
            width,
            height,
            data: data.into_boxed_slice(),
        }
    }

    fn get(&self, x: usize, y: usize) -> C {
        self.data[x + y * self.width]
    }

    fn set(&mut self, x: usize, y: usize, color: C) {
        if x < self.width && y < self.height {
            self.data[x + y * self.width] = color;
        }
    }
}

/// Simulated binary color display
///
/// You should use [`DisplayBuilder`] to create an instance of `BinaryDisplay`
///
/// [`DisplayBuilder`]: ./display_builder/struct.DisplayBuilder.html
pub struct BinaryDisplay {
    pixels: PixelData<BinaryColor>,
    theme: BinaryColorTheme,
    window: Window,
}

impl BinaryDisplay {
    /// Clear all pixels to black (empty the pixel buffer)
    pub fn clear(&mut self) {
        self.pixels = PixelData::<BinaryColor>::new(self.pixels.width, self.pixels.height);
    }

    /// Update the display to show drawn pixels
    pub fn run_once(&mut self) -> bool {
        if self.window.handle_events() {
            return true;
        }

        self.window.clear(self.theme.convert(BinaryColor::Off));

        for y in 0..self.pixels.height {
            for x in 0..self.pixels.width {
                let color = self.pixels.get(x, y);
                let color = self.theme.convert(color);
                self.window.draw_pixel(x, y, color);
            }
        }

        self.window.present();
        false
    }

    /// Get a vector of detected input events
    pub fn get_input_events(&mut self) -> Vec<SimulatorEvent> {
        self.window.get_input_events()
    }
}

impl DrawTarget<BinaryColor> for BinaryDisplay {
    fn draw_pixel(&mut self, pixel: Pixel<BinaryColor>) {
        let Pixel(coord, color) = pixel;
        let x = coord[0] as usize;
        let y = coord[1] as usize;
        self.pixels.set(x, y, color);
    }

    fn size(&self) -> Size {
        Size::new(self.pixels.width as u32, self.pixels.height as u32)
    }
}

/// Simulated RGB display
///
/// You should use [`DisplayBuilder`] to create an instance of `RgbDisplay`
///
/// [`DisplayBuilder`]: ./display_builder/struct.DisplayBuilder.html
pub struct RgbDisplay {
    pixels: PixelData<Rgb888>,
    window: Window,
}

impl RgbDisplay {
    /// Clear all pixels to black (empty the pixel buffer)
    pub fn clear(&mut self) {
        self.pixels = PixelData::<Rgb888>::new(self.pixels.width, self.pixels.height);
    }

    /// Update the display to show drawn pixels
    pub fn run_once(&mut self) -> bool {
        if self.window.handle_events() {
            return true;
        }

        self.window.clear(Rgb888::BLACK);

        for y in 0..self.pixels.height {
            for x in 0..self.pixels.width {
                let color = self.pixels.get(x, y);
                self.window.draw_pixel(x, y, color);
            }
        }

        self.window.present();
        false
    }

    /// Get a vector of detected input events
    pub fn get_input_events(&mut self) -> Vec<SimulatorEvent> {
        self.window.get_input_events()
    }
}

impl<C> DrawTarget<C> for RgbDisplay
where
    C: PixelColor + Into<Rgb888>,
{
    fn draw_pixel(&mut self, pixel: Pixel<C>) {
        let Pixel(coord, color) = pixel;
        let x = coord[0] as usize;
        let y = coord[1] as usize;
        self.pixels.set(x, y, color.into());
    }

    fn size(&self) -> Size {
        Size::new(self.pixels.width as u32, self.pixels.height as u32)
    }
}
