/*Copyright (c) 2023 Collin Ogren

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/

use std::fs::File;
use png::{Decoder};

pub fn png_to_rgba(path: &str) -> (Vec<u8>, u32, u32) {
    let decoder = Decoder::new(
        match File::open(path) {
        Ok(v) => v,
        Err(err) => {
            eprintln!("Failed to read icon: {}", err);
            return (vec![], 0, 0);
        }
    });
    let mut reader = match decoder.read_info() {
        Ok(v) => v,
        Err(err) => {
            eprintln!("Failed to read icon: {}", err);
            return (vec![], 0, 0);
        }
    };

    let mut buffer = vec![0; reader.output_buffer_size()];

    let info = match reader.next_frame(&mut buffer) {
        Ok(v) => v,
        Err(err) => {
            eprintln!("Failed to read icon: {}", err);
            return (vec![], 0, 0);
        }
    };

    (buffer[..info.buffer_size()].to_vec(), info.width, info.height)
}