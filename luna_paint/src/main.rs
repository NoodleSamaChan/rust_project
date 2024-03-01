use minifb::{Key, Window, WindowOptions};
use std::fmt;
use insta::assert_snapshot;
use insta::assert_display_snapshot;

//GLOBAL VARIABLES
const WIDTH: usize = 640;
const HEIGHT: usize = 360;
//END GLOBAL VARIABLES

//COLOUR FUNCTION
pub fn rgb(red: u8, green: u8, blue: u8) -> u32 {
    let a = u32::from(red);
    let b: u32 = u32::from(green);
    let c = u32::from(blue);

    let new_red = a << 16;
    let new_green = b << 8;

    let final_number = new_red | new_green | c;

    return final_number
    
}
//END COLOUR FUNCTION

//WINDOW BUFFER SECTION ORGANISATION
pub struct WindowBuffer {
    width: usize,
    height: usize,

    buffer: Vec<u32>,
}

impl WindowBuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            buffer: vec![0; width * height],
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn buffer(&self) -> Vec<u32> {
        self.buffer.clone()
    }
}

impl fmt::Display for WindowBuffer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        let line_len = self.buffer.chunks(self.width);
        for i in line_len{
            for a in i{
                match a {
                    0 => f.write_str(".")?,
                    _ => f.write_str("#")?,
                    
                } 
            }
            f.write_str("\n")?;  
        }
        Ok(())
    }
}
impl std::ops::Index<(usize, usize)> for WindowBuffer {
    type Output = u32;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        if x >= self.width {
            panic!(
                "Tried to index in a buffer of width {} with a x of {}",
                self.width, x
            );
        }
        if y >= self.height {
            panic!(
                "Tried to index in a buffer of height {} with a y of {}",
                self.height, y
            );
        }

        &self.buffer[y * self.width + x]
    }
}
impl std::ops::IndexMut<(usize, usize)> for WindowBuffer {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        if x >= self.width {
            panic!(
                "Tried to index in a buffer of width {} with a x of {}",
                self.width, x
            );
        }
        if y >= self.height {
            panic!(
                "Tried to index in a buffer of height {} with a y of {}",
                self.height, y
            );
        }

        &mut self.buffer[y * self.width + x]
    }
}
////END WINDOW BUFFER SECTION ORGANISATION


//MAIN WINDOW FOR DRAWING
fn main() {
    let mut buffer = WindowBuffer::new(WIDTH, HEIGHT);

    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        for y in 0..buffer.height() {
            for x in 0..buffer.width() {
                let progression = x as f64 / buffer.width() as f64;

                let color = (progression * u8::MAX as f64) as u8;

                buffer[(x, y)] = rgb(color, 0, 0);

            }
        }

        window
            .update_with_buffer(&buffer.buffer(), WIDTH, HEIGHT)
            .unwrap();
    }  
}

//TESTS
#[cfg(test)]
mod test {
  use super::*;
  use proptest::prelude::*;
  
  #[test]
  fn test_rgb() {
    assert_eq!(rgb(0, 0, 0), 0x00_00_00_00);
    assert_eq!(rgb(255, 255, 255), 0x00_ff_ff_ff);
    assert_eq!(rgb(0x12, 0x34, 0x56), 0x00_12_34_56);
  }

  use insta::assert_snapshot;

  #[test]
  fn display_window_buffer() {
      let mut buffer = WindowBuffer::new(4, 4);
      assert_display_snapshot!(
          buffer.to_string(),
          @""
      );
      buffer.buffer[1] = 1;
      buffer.buffer[3] = 3;
      buffer.buffer[4] = 4;
      buffer.buffer[6] = 6;
      buffer.buffer[9] = 9;
      buffer.buffer[11] = 11;
      buffer.buffer[12] = 12;
      buffer.buffer[14] = 14;
      assert_display_snapshot!(
          buffer.to_string(),
          @""
      );
  }

  #[test]
  #[should_panic]
  fn test_bad_index_width() {
      let mut buffer = WindowBuffer::new(4, 4);
      buffer[(0, 5)] = 0;
  }

  #[test]
  #[should_panic]
  fn test_bad_index_height() {
      let mut buffer = WindowBuffer::new(4, 4);
      buffer[(5, 0)] = 0;
  }

  #[test]
  fn test_index() {
      let mut buffer = WindowBuffer::new(4, 4);
      buffer[(0, 1)] = 1;
      buffer[(0, 3)] = 3;
      buffer[(1, 0)] = 4;
      buffer[(1, 2)] = 6;
      buffer[(2, 1)] = 9;
      buffer[(2, 3)] = 11;
      buffer[(3, 0)] = 12;
      buffer[(3, 2)] = 14;
      assert_display_snapshot!(
          buffer.to_string(),
          @r###"
      .#.#
      #.#.
      .#.#
      #.#.
      "###
      );
  }
}