// Copyright (c) 2024 Remi A. Godin
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use cgmath::{Vector3,vec3};
/// A struct containing the pixel data for a single image
///
/// # Arguments
///
/// `data`          The actual data, stored as a single vec of Vector3<u8>
/// `height`        the height of the image in pixels
/// `file_name`     the name of the image
/// `image_data`    holds the raw image data until it is written to an actual file 
/// `max_size`      the number of pixels the image can hold 
/// `current_size`  the current number of pixel that have been written
///
/// # Note
/// The canvas is basically an inline matrix representation of an image. Each subsequent element of
/// the vector starts from the top left corner, and goes from left to write, and up to down. You can
/// write directly to the `data` field of the canvas if you wish, but methods have been provided to
/// allow writing to pixels using rows and column as with a regular 2x2 matrix.
#[derive(Clone)]
pub struct Canvas {
    pub data: Vec<Vector3<u8>>,
    pub name: String,
    pub width: usize,
    pub height: usize,
    pub size: usize
}

impl Canvas {
    pub fn new(name: &str, width: usize, height: usize) -> Canvas {
        let mut canvas = Canvas {
            name: name.to_string(),
            width,
            height,
            size: width * height,
            data: Vec::with_capacity(width * height)
        };
        let zero = vec3(0,0,0);
        for _i in 0..canvas.data.capacity() {
            canvas.data.push(zero);
        }
        canvas
    }


    pub fn get(&self, row: usize, col: usize) -> Option<&Vector3<u8>> {
        self.data.get(row * self.width + col)
    }

    pub fn get_mut(&mut self, row: usize, col: usize) -> Option<&mut Vector3<u8>> {
        self.data.get_mut(row * self.width + col)
    }

    pub fn iter(&self) -> core::slice::Iter<Vector3<u8>> {
        self.data.iter()
    }

    pub fn iter_mut(&mut self) -> core::slice::IterMut<Vector3<u8>> {
        self.data.iter_mut()
    }
}
