//!
//! Anti Helmet
//! Advent of Code
//! Day 20: Trench Map
//!

use std::io::{BufRead, stdin};

/// Defines a infinite B/W image that consists of bitmap in the center 
/// surrounded by void pixels that stretch out infinitely.
#[derive(Debug)]
struct Image {
    bitmap: Vec<Vec<bool>>,
    void_pixel: bool,
}
impl Image {
    /// Performs convolution with void pixel padding on this image using the given
    /// filter with the given windows of the image with the given dimensions.
    /// Returns the convolved image.
    fn convolve<F: Fn(Vec<Vec<bool>>) -> bool>(self, filter: F, window_dim: (usize, usize)) -> Self {
        let (window_x, window_y) = window_dim;
        let (window_x, window_y) = (window_x as isize, window_y as isize);
        // apply convolution to image bitmap
        let bitmap = if self.bitmap.len() <= 0 {
            self.bitmap.clone()
        } else {
            // compute convolution bounds of convolving bitmap
            let (bitmap_x, bitmap_y) = (self.bitmap[0].len() as isize, self.bitmap.len() as isize);
            let (begin_x, end_x, begin_y, end_y) = (
                0 - window_x + 1,
                bitmap_x + (window_x - 1),
                0 - (window_y - 1),
                bitmap_y + (window_y - 1),
            );
        
            // convolute bitmap with with padding
            (begin_y..end_y).map(|offset_y|
                (begin_x..end_x).map(|offset_x| {
                    // collect pixels in convolution window offset by (x, y)
                    let window: Vec<Vec<_>> = (offset_y..(offset_y + window_y)).map(|y|
                        (offset_x..(offset_x + window_x)).map(|x|
                            // negative x, y when cast as usize should underflow
                            // and give a large usize value, which would not be 
                            // found in the bitmmap.
                            *self.bitmap.get(y as usize)
                                .map(|row| row.get(x as usize))
                                .flatten()
                                .unwrap_or(&self.void_pixel)
                        ).collect()
                    ).collect();

                    // apply convolution filter to window
                    filter(window)
                }).collect()
            ).collect()
        };

        // apply convolution to void pixel
        let void_window = (0..window_y).map(|_|
            (0..window_x).map(|_| self.void_pixel).collect()
        ).collect();
        let void_pixel = filter(void_window);

        Image { bitmap, void_pixel }
    }

    /// Enhance this image with the given enhancement algorithm.
    fn enhance(self, algorithm: &[bool]) -> Self {
        let filter_dim = (3, 3);
        self.convolve(|window| {
            // convert the window into a binary string
            let pos_binary = window.into_iter()
                .flatten()
                .map(|pixel|
                    match pixel {
                        true => "1",
                        false => "0",
                    }
                ).collect::<String>();
            // parse it for the position to lookup in the image enhancement algorithm.
            let position = usize::from_str_radix(&pos_binary, 2)
                .expect("Failed to parse position as binary string");
                
            algorithm[position]
        }, filter_dim)
    }
}


fn main() {
    // parse image enhancement algorithm from stdin.
    let input = stdin();
    let mut lines = input.lock().lines();
    let parse_pixel = |c| {
        match c {
            '#' => true,
            '.' => false,
            _ => panic!("Parsed unsupported character in algorithm: '{}'", c),
        }
    };
    let algorithm: Vec<_> = lines.next()
        .expect("Expected at least one line of input to be passed")
        .expect("Failed to read image enhancement algorithm from stdin")
        .chars()
        .map(parse_pixel)
        .collect();

    // skip empty line
    lines.next();

    // parse inital image from stdin
    let mut image = Image{
        bitmap: lines.map(|line| 
            line.expect("Failed to read inital image")
                .chars()
                .map(|c| parse_pixel(c))
                .collect()
        ).collect(),
        void_pixel: parse_pixel('.'),
    };
    
    // apply image enhancement algorithm to enhance image
    for _ in 0..50 {
        image = image.enhance(&algorithm);
    }

    // count no. of set pixels
    // check that void pixel is unset, otherwise there will be a infinite no. of set pixels
    assert_eq!(image.void_pixel, false); 
    let n_set = image.bitmap.into_iter().fold(0, |count, row|
        row.into_iter().fold(count, |count, pixel|
            count + if pixel { 1 } else { 0 }
        )
    );
    println!("No. of set pixels: {}", n_set);
}
