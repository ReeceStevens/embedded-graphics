//! # Example: Input Handling
//!
//! This example allows you to move a red circle to the location of a click on the simulator
//! screen. Although input handling is not a part of the embedded-graphics API, the simulator can
//! be used to emulate input controls in order to represent more complex UI systems such as touch
//! screens.
extern crate embedded_graphics;
extern crate embedded_graphics_simulator;

use embedded_graphics::prelude::*;
use embedded_graphics::primitives::Circle;
use embedded_graphics::pixelcolor::Rgb888;
use embedded_graphics_simulator::{DisplayBuilder, RgbDisplay};

let background_color = Some(Rgb888::new(0, 0, 0));
let foreground_color = Some(Rgb888::new(255, 0, 0));

fn main() {
    let mut display = DisplayBuilder::new().size(800, 480).build_rgb();

    let mut position = Point::new(200, 200);
    display.draw(Circle::new(position, 100).fill(foreground_color));

    loop {
        let end = display.run_once();

        if end {
            break;
        }

        if let Some((x, y)) = display.get_input_event() {
            // Clear old circle
            display.draw(Circle::new(position, 100).fill(background_color));
            position = Point::new(x, y);
            // Draw circle at new location
            display.draw(Circle::new(position, 100).fill(foreground_color));
        }
    }
}
