use super::ImageFile;
use crate::drawable::{Drawable, Pixel};
use crate::geometry::{Dimensions, Point, Size};
use crate::pixelcolor::raw::{LittleEndian, RawData, RawDataIter};
use crate::pixelcolor::PixelColor;
use crate::transform::Transform;
use core::marker::PhantomData;
use tinybmp::Bmp;

/// BMP format image
///
/// `ImageBmp` is available with the `bmp` feature turned on
///
/// # Examples
///
/// ## Load a 16 bit per pixel image from a raw byte slice and draw it to a display
///
/// Note that images must be passed to `Display#draw` by reference, or by explicitly calling
/// `.into_iter()` on them, unlike other embedded_graphics objects.
///
/// ```rust
/// use embedded_graphics::prelude::*;
/// use embedded_graphics::image::ImageBmp;
/// # use embedded_graphics::mock_display::MockDisplay;
/// # use embedded_graphics::pixelcolor::Rgb565;
/// # let mut display: MockDisplay<Rgb565> = MockDisplay::default();
///
/// // Load `patch_16bpp.bmp`, a 16BPP 4x4px image
/// let image = ImageBmp::new(include_bytes!("../../../assets/patch_16bpp.bmp")).unwrap();
///
/// // Equivalent behavior
/// display.draw(&image);
/// display.draw(image.into_iter());
/// ```
#[derive(Debug, Clone)]
pub struct ImageBmp<'a, C>
where
    C: PixelColor + From<<C as PixelColor>::Raw>,
{
    bmp: Bmp<'a>,

    /// Top left corner offset from display origin (0,0)
    pub offset: Point,

    pixel_type: PhantomData<C>,
}

impl<'a, C> ImageBmp<'a, C>
where
    C: PixelColor + From<<C as PixelColor>::Raw>,
{
    /// Returns the row length in bytes.
    ///
    /// Each row in a BMP file is a multiple of 4 bytes long.
    fn bytes_per_row(&self) -> usize {
        let bits_per_row = self.bmp.width() as usize * self.bmp.bpp() as usize;

        (bits_per_row + 31) / 32 * (32 / 8)
    }
}

impl<'a, C> ImageFile<'a> for ImageBmp<'a, C>
where
    C: PixelColor + From<<C as PixelColor>::Raw>,
{
    /// Create a new BMP from a byte slice
    fn new(image_data: &'a [u8]) -> Result<Self, ()> {
        let im = Self {
            bmp: Bmp::from_slice(image_data)?,
            offset: Point::zero(),
            pixel_type: PhantomData,
        };

        Ok(im)
    }

    fn width(&self) -> u32 {
        self.bmp.width()
    }

    fn height(&self) -> u32 {
        self.bmp.height()
    }
}

impl<'a, C> Dimensions for ImageBmp<'a, C>
where
    C: PixelColor + From<<C as PixelColor>::Raw>,
{
    fn top_left(&self) -> Point {
        self.offset
    }

    fn bottom_right(&self) -> Point {
        self.top_left() + self.size()
    }

    fn size(&self) -> Size {
        Size::new(self.bmp.width(), self.bmp.height())
    }
}

impl<'a, C> Transform for ImageBmp<'a, C>
where
    C: PixelColor + From<<C as PixelColor>::Raw>,
{
    /// Translate the image from its current position to a new position by (x, y) pixels, returning
    /// a new `ImageBmp`. For a mutating transform, see `translate_mut`.
    fn translate(&self, by: Point) -> Self {
        Self {
            offset: self.offset + by,
            ..self.clone()
        }
    }

    /// Translate the image from its current position to a new position by (x, y) pixels.
    fn translate_mut(&mut self, by: Point) -> &mut Self {
        self.offset += by;

        self
    }
}

impl<'a, C> Drawable for ImageBmp<'a, C> where C: PixelColor + From<<C as PixelColor>::Raw> {}

impl<'a, C> IntoIterator for &'a ImageBmp<'a, C>
where
    C: PixelColor + From<<C as PixelColor>::Raw>,
{
    type Item = Pixel<C>;
    type IntoIter = ImageBmpIterator<'a, C>;

    // NOTE: `self` is a reference already, no copies here!
    fn into_iter(self) -> Self::IntoIter {
        // Check that image bpp is equal to required bpp for `C`.
        if self.bmp.bpp() as usize != C::Raw::BITS_PER_PIXEL {
            panic!("invalid bits per pixel");
        }

        ImageBmpIterator {
            data: RawDataIter::new(self.bmp.image_data()),
            x: 0,
            y: 0,
            image: self,
        }
    }
}

#[derive(Debug)]
pub struct ImageBmpIterator<'a, C>
where
    C: PixelColor + From<<C as PixelColor>::Raw>,
{
    data: RawDataIter<'a, C::Raw, LittleEndian>,

    x: u32,
    y: u32,

    image: &'a ImageBmp<'a, C>,
}

impl<'a, C> Iterator for ImageBmpIterator<'a, C>
where
    C: PixelColor + From<<C as PixelColor>::Raw>,
{
    type Item = Pixel<C>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.y < self.image.bmp.height() {
            if self.x == 0 {
                let row_index = (self.image.height() - 1) - self.y;
                let row_start = self.image.bytes_per_row() * row_index as usize;
                self.data.set_byte_position(row_start);
            }

            let data = self.data.next()?;
            let mut point = Point::new(self.x as i32, self.y as i32);
            point += self.image.offset;

            self.x += 1;
            if self.x >= self.image.bmp.width() {
                self.y += 1;
                self.x = 0;
            }

            Some(Pixel(point, data.into()))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mock_display::MockDisplay;
    use crate::pixelcolor::{BinaryColor, Gray8, GrayColor, Rgb555, Rgb565, Rgb888, RgbColor};
    use crate::Drawing;

    #[test]
    fn negative_top_left() {
        let image: ImageBmp<Rgb565> = ImageBmp::new(include_bytes!(
            "../../tests/chessboard-4px-colour-16bit.bmp"
        ))
        .unwrap()
        .translate(Point::new(-1, -1));

        assert_eq!(image.top_left(), Point::new(-1, -1));
        assert_eq!(image.bottom_right(), Point::new(3, 3));
        assert_eq!(image.size(), Size::new(4, 4));
    }

    #[test]
    fn dimensions() {
        let image: ImageBmp<Rgb565> = ImageBmp::new(include_bytes!(
            "../../tests/chessboard-4px-colour-16bit.bmp"
        ))
        .unwrap()
        .translate(Point::new(100, 200));

        assert_eq!(image.top_left(), Point::new(100, 200));
        assert_eq!(image.bottom_right(), Point::new(104, 204));
        assert_eq!(image.size(), Size::new(4, 4));
    }

    #[test]
    #[ignore]
    fn it_can_have_negative_offsets() {
        let image: ImageBmp<Rgb565> = ImageBmp::new(include_bytes!(
            "../../tests/chessboard-4px-colour-16bit.bmp"
        ))
        .unwrap()
        .translate(Point::new(-1, -1));
        let it = image.into_iter();

        let expected: [Pixel<Rgb565>; 9] = [
            Pixel(Point::new(0, 0), Rgb565::RED),
            Pixel(Point::new(1, 0), Rgb565::BLACK),
            Pixel(Point::new(2, 0), Rgb565::GREEN),
            //
            Pixel(Point::new(0, 1), Rgb565::BLACK),
            Pixel(Point::new(1, 1), Rgb565::BLUE),
            Pixel(Point::new(2, 1), Rgb565::BLACK),
            //
            Pixel(Point::new(0, 2), Rgb565::WHITE),
            Pixel(Point::new(1, 2), Rgb565::BLACK),
            Pixel(Point::new(2, 2), Rgb565::WHITE),
        ];

        assert_eq!(image.into_iter().count(), 9);

        for (idx, pixel) in it.enumerate() {
            assert_eq!(pixel, expected[idx]);
        }
    }

    fn create_color_pattern<C>() -> [[C; 4]; 2]
    where
        C: RgbColor,
    {
        [
            [C::BLACK, C::RED, C::GREEN, C::YELLOW],
            [C::BLUE, C::MAGENTA, C::CYAN, C::WHITE],
        ]
    }

    macro_rules! test_pattern {
        ($color_type:ident, $image_data:expr) => {
            let image: ImageBmp<$color_type> = ImageBmp::new($image_data).unwrap();

            let pattern = create_color_pattern();

            assert_eq!(image.size(), Size::new(4, 2));

            let mut iter = image.into_iter();
            for (y, row) in pattern.iter().enumerate() {
                for (x, &expected_color) in row.iter().enumerate() {
                    let pos = Point::new(x as i32, y as i32);
                    let pixel = iter.next().unwrap();

                    assert_eq!(pixel, Pixel(pos, expected_color));
                }
            }

            assert!(iter.next().is_none());
        };
    }

    #[test]
    fn colors_rgb555() {
        test_pattern!(Rgb555, include_bytes!("../../tests/colors_rgb555.bmp"));
    }

    #[test]
    fn colors_rgb565() {
        test_pattern!(Rgb565, include_bytes!("../../tests/colors_rgb565.bmp"));
    }

    #[test]
    fn colors_rgb888_24bit() {
        test_pattern!(
            Rgb888,
            include_bytes!("../../tests/colors_rgb888_24bit.bmp")
        );
    }

    #[test]
    #[ignore]
    fn colors_rgb888_32bit() {
        test_pattern!(
            Rgb888,
            include_bytes!("../../tests/colors_rgb888_32bit.bmp")
        );
    }

    #[test]
    fn colors_grey8() {
        let image: ImageBmp<Gray8> =
            ImageBmp::new(include_bytes!("../../tests/colors_grey8.bmp")).unwrap();

        assert_eq!(image.size(), Size::new(3, 1));

        let mut iter = image.into_iter();

        let p = iter.next().unwrap();
        assert_eq!(p.0, Point::new(0, 0));
        assert_eq!(p.1, Gray8::BLACK);

        let p = iter.next().unwrap();
        assert_eq!(p.0, Point::new(1, 0));
        assert_eq!(p.1, Gray8::new(128));

        let p = iter.next().unwrap();
        assert_eq!(p.0, Point::new(2, 0));
        assert_eq!(p.1, Gray8::WHITE);

        assert!(iter.next().is_none());
    }

    /// Test for issue #136
    #[test]
    fn issue_136_row_size_is_multiple_of_4_bytes() {
        let image: ImageBmp<Rgb565> =
            ImageBmp::new(include_bytes!("../../tests/issue_136.bmp")).unwrap();

        let mut display = MockDisplay::new();
        display.draw(image.into_iter().map(|Pixel(p, c)| {
            Pixel(
                p,
                match c {
                    Rgb565::BLACK => BinaryColor::Off,
                    Rgb565::WHITE => BinaryColor::On,
                    _ => panic!("Unexpected color in image"),
                },
            )
        }));

        assert_eq!(
            display,
            MockDisplay::from_pattern(&[
                "####.####",
                "#....#...",
                "####.#.##",
                "#....#..#",
                "####.####",
            ])
        );
    }
}
