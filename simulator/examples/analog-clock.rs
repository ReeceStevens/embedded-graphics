//! # Example: Analog Clock
//!
//! ![Screenshot of clock example]( data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAfwAAAH9AQMAAADFwFz1AAAABlBMVEUAAAD///+l2Z/dAAAFM0lEQVR42u3dYWrjMBAF4Afz1zBX8QEEc/WBvdAcQKBNussK3EbVpDNSt+SRYMWWPrBsybQJCK+88sp/EWm32BagN+/EcoBuLRUE8K2wA6B7c9yBN2I9QK3+2eItrS0HWv0L9c9LgV6f+p6lADdcAWq6AuiVO9DRhYBYty571wBc8RGApquApkM4H+A6lvOBpkM6H+A6tvOBpkM8H6A61BcAosMetnSA6olBzpYOsI0B0Wyg4cAgSjUZIPsEgCQDoqCRD7DlAvX2Hlyo414lFSD7HJAsoI/lc3An9xGdBFRMpCYCZDOAJAKsMwBrHiCYCVka0I+MI0GAvw97vSxAMBeyLKBiMjUJIJsFJAjwX8VeMwcQzIYsB6iYTk0ByOYBCQL8V7HXzQAE8yHLAAocSQHMA5QEgNQDcALAPkDjAYEnZPFAhSs1HjAfUMIBUh/A4QB7AY0GBL6QRQMFzoQD5gVKMEBuQKIB9QIcDDC8IY0FxA9YLFDgTjBQ/UCNBcwPlFCA1A9wKMBPAE1DgeI/AwoFxN+LJ1kkUHD655NQoLrntAOokYAB6h9NgQAZnohEAvoMwFHA+9HcHuUCaCCAZwAKBOQ5wOKAcgGU7q/WLoULgB8FXMZSlVqkisq1cAHKDwJIrzdSlXZvdysU6YUrwD8ZqHR/3VKpKeRvgVoawHgOIE0DfrX7S6X35q/bpkoaIM8CFgWUy4G3VnegSr21fSt8BOAnA2+75O9cet9WQG7lNMCeBUoW0B7kBwOkzwIcB/iSD+h2oKwGnv8XMWkSgHM7wNsB2Fqg4OlzsDQAuh0o2wHS3QDO7QCvBGxw7JOUTODYDpDuBlC2A6S7AZzbAd4OwBYB/XL5xxMnA6S7AZTtAG0HYNsB3g7AWDcDR6ubAfoGQFsAtGbPAdLal4Fv0gcjAK1+EfgGN9IXgcK6GVBsBhi7gXM7oDdjK8DYDZxrANioD8dASQYYO4F/bY+NAOlu4MBu4NwNkG4FekvKBUZ/WtkEYJkAYTdwbAesz6x7AMJu4FgJ8INDnwOkeQBhN1B68dwD6FKAdNCHQ4CRBpTtgO4G+DKws4H3T9dzEihpgO4GGKuBMujDIWBZgF6G9nKAsRyQQR+OAAoD+DJXTwOaAxxYCPTCaBLVwYyWAZDuBg5sAGDosXmgpACELUAv4XB8fWwpgO0B5NKHcwAFAtz70ANoIPCvaA6AA4E+O3q+QWfEAf2CeAAJBPAUUCKBCqgPOIAaCRQwfAApLBIQnHD+iOCkUIBJvQCThgIVXgAtFCD1A4xIYDyaPtZLLFD9QB0CbqD4AYsFxA1QMMB+QGOB/nH26coIBswLSDAAN1CigeIFLBoQJ0DhAKvv6coaDZAXQDQA8wElHqg+oMYD4gLI4gF2/daTNR4gH4B4AOYBSgZQPIBlAOIAKALw3En6vm4GQDYPCDIA1Hmg5gAyDZDlAKyzAGsOQDb7fBfkAKizQM0CZBIgywJY5wDWLIBsDhBkAZCp5ztZHsD6ALZxLT/g7UU9LicaALiGE4EulfIAUQD6wVXUfj3ZMgGy2xuPAep9mASgfg7UXEAUxwCAgi0XIMM5Ak4IcgE0nKNb+aSaDbDpaDgfotkAVR1NaUdDBOBfWkj7GeYDVEdAQz6Apo+frlxTgUGls+OpwONaZ7dXAFzfA51eAfR6HehyCOBfdu/o8BIAYleg710DUNP3ADesAnrlDnR0EYBWL0Dfkw90oQPU24cA/uVYqTV8Ffj/1rT9Buv6foO1jb/D+s6vvPKKI78B89G0YRkxkl8AAAAASUVORK5CYII=)
//!
//! This example shows some more advanced usage of Embedded Graphics. It draws a round clock face
//! with hour, minute and second hands. A digital clock is drawn in the middle of the clock. The
//! whole thing is updated with your computer's local time every 50ms.

use chrono::{Local, Timelike};
use core::f32::consts::{FRAC_PI_2, PI};
use embedded_graphics::egcircle;
use embedded_graphics::fonts::Font12x16;
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{Circle, Line, Rectangle};
use embedded_graphics_simulator::DisplayBuilder;
use std::thread;
use std::time::Duration;

/// The width and height of the simulated display
const DISP_SIZE: i32 = 256;

/// The center of the clock face
const CENTER: Point = Point::new(DISP_SIZE / 2, DISP_SIZE / 2);

/// The radius of the clock face
const SIZE: u32 = 120;

/// Start at the top of the circle
const START: f32 = -FRAC_PI_2;

/// Convert a polar coordinate (angle/distance) into an (X, Y) coordinate centered around `CENTER`
fn polar(angle: f32, radius: f32) -> Point {
    CENTER + Point::new((angle.cos() * radius) as i32, (angle.sin() * radius) as i32)
}

/// Draw a circle and 12 tics as a simple clock face
fn draw_face() -> impl Iterator<Item = Pixel<BinaryColor>> {
    let tic_len = 10.0;

    // Use the circle macro to create the outer face
    let face = egcircle!(
        CENTER,
        SIZE,
        stroke_color = Some(BinaryColor::On),
        stroke_width = 2
    );

    // Create 12 `Line`s starting from the outer edge and drawing inwards by `tic_len` pixels
    let tics = (0..12).into_iter().map(move |index| {
        // Start angle around the circle, in radians
        let angle = START + (PI * 2.0 / 12.0) * index as f32;

        // Start point on circumference
        let start = polar(angle, SIZE as f32);

        // End point; start point offset by `tic_len` pixels towards the circle center
        let end = polar(angle, SIZE as f32 - tic_len);

        Line::new(start, end)
            .stroke_color(Some(BinaryColor::On))
            .into_iter()
    });

    // Create a single iterator of pixels, first iterating over the circle, then over the 12 lines
    // generated
    face.into_iter().chain(tics.flatten())
}

/// Draw the seconds hand given a seconds value (0 - 59)
fn draw_seconds_hand(seconds: u32) -> impl Iterator<Item = Pixel<BinaryColor>> {
    // Convert seconds into a position around the circle in radians
    let seconds_radians = ((seconds as f32 / 60.0) * 2.0 * PI) + START;

    let end = polar(seconds_radians, SIZE as f32);

    // Basic line hand
    let hand = Line::new(CENTER, end).stroke_color(Some(BinaryColor::On));

    // Decoration position
    let decoration_position = polar(seconds_radians, SIZE as f32 - 20.0);

    // Add a fancy circle near the end of the hand
    let decoration = Circle::new(decoration_position, 5)
        .fill_color(Some(BinaryColor::Off))
        .stroke_color(Some(BinaryColor::On));

    hand.into_iter().chain(decoration)
}

/// Draw the hour hand (0-11)
fn draw_hour_hand(hour: u32) -> Line<BinaryColor> {
    // Convert hour into a position around the circle in radians
    let hour_radians = ((hour as f32 / 12.0) * 2.0 * PI) + START;

    let hand_len = SIZE as f32 - 60.0;

    let end = polar(hour_radians, hand_len);

    // Basic line hand
    Line::new(CENTER, end).stroke_color(Some(BinaryColor::On))
}

/// Draw the minute hand (0-59)
fn draw_minute_hand(minute: u32) -> Line<BinaryColor> {
    // Convert minute into a position around the circle in radians
    let minute_radians = ((minute as f32 / 60.0) * 2.0 * PI) + START;

    let hand_len = SIZE as f32 - 30.0;

    let end = polar(minute_radians, hand_len);

    // Basic line hand
    Line::new(CENTER, end).stroke_color(Some(BinaryColor::On))
}

/// Draw digital clock just above center with black text on a white background
///
/// NOTE: The formatted time str must be passed in as references to temporary values in a
/// function can't be returned.
fn draw_digital_clock<'a>(time_str: &'a str) -> impl Iterator<Item = Pixel<BinaryColor>> + 'a {
    let text = Font12x16::render_str(&time_str)
        .stroke_color(Some(BinaryColor::Off))
        .translate(CENTER - Size::new(48, 48));

    // Add a background around the time digits. Note that there is no bottom-right padding as this
    // is added by the font renderer itself
    let background = Rectangle::new(text.top_left() - Size::new(3, 3), text.bottom_right())
        .fill_color(Some(BinaryColor::On));

    // Draw the white background first, then the black text. Order matters here
    background.into_iter().chain(text)
}

fn main() {
    let mut display = DisplayBuilder::new()
        .title("Clock")
        .size(DISP_SIZE as usize, DISP_SIZE as usize)
        .scale(2)
        .build_binary();

    loop {
        let time = Local::now();

        // NOTE: In no-std environments, consider using
        // [arrayvec](https://stackoverflow.com/a/39491059/383609) and a fixed size buffer
        let digital_clock_text = format!(
            "{:02}:{:02}:{:02}",
            time.hour(),
            time.minute(),
            time.second()
        );

        display.clear();

        display.draw(draw_face());
        display.draw(draw_hour_hand(time.hour()));
        display.draw(draw_minute_hand(time.minute()));
        display.draw(draw_seconds_hand(time.second()));

        // Draw digital clock just above center
        display.draw(draw_digital_clock(&digital_clock_text));

        // Draw a small circle over the hands in the center of the clock face. This has to happen
        // after the hands are drawn so they're covered up
        display.draw(Circle::new(CENTER, 4).fill_color(Some(BinaryColor::On)));

        let end = display.run_once();

        if end {
            break;
        }

        thread::sleep(Duration::from_millis(50));
    }
}
