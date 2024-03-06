use insta::assert_snapshot;
use minifb::{Key, Window, WindowOptions};
use minifb::{MouseButton, MouseMode};
use proptest::strategy::W;
use std::fmt;
use std::time::{Instant, Duration};

const WIDTH: usize = 160;
const HEIGHT: usize = 90;

//COLOURS MANAGEMENT
pub fn rgb(red: u8, green: u8, blue: u8) -> u32 {
    let a = u32::from(red);
    let b: u32 = u32::from(green);
    let c = u32::from(blue);

    let new_red = a << 16;
    let new_green = b << 8;

    let final_number = new_red | new_green | c;

    return final_number;
}
//COLOURS MANAGEMENT END

// GRID CREATION
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
    pub fn update(&mut self) {
        self.check_surroundings()
    }

    pub fn get(&self, x: isize, y: isize) -> Option<u32> {
        if (x >= 0) && ((x as usize) < self.width()) && (y >= 0) && ((y as usize) < self.height()) {
            Some(self[(x as usize, y as usize)])
        } else {
            None
        }
    }

    pub fn check_surroundings(&mut self) {
        let mut colored_cells_counter: usize = 0;
        let mut next_iteration = WindowBuffer{width: self.width(), height: self.height(), buffer: self.buffer()};

        for x in 0..self.width {
            for y in 0..self.height {
                let x = x as isize;
                let y = y as isize;

                if self.get(x - 1, y - 1) == Some(u32::MAX) {
                    colored_cells_counter += 1;
                }
                if self.get(x - 1, y) == Some(u32::MAX) {
                    colored_cells_counter += 1;
                }
                if self.get(x - 1, y + 1) == Some(u32::MAX) {
                    colored_cells_counter += 1;
                }
                if self.get(x, y - 1) == Some(u32::MAX) {
                    colored_cells_counter += 1;
                }
                if self.get(x, y + 1) == Some(u32::MAX) {
                    colored_cells_counter += 1;
                }
                if self.get(x + 1, y - 1) == Some(u32::MAX) {
                    colored_cells_counter += 1;
                }
                if (self.get(x + 1, y)) == Some(u32::MAX) {
                    colored_cells_counter += 1;
                }
                if self.get(x + 1, y + 1) == Some(u32::MAX) {
                    colored_cells_counter += 1;
                }

                if colored_cells_counter < 2 || colored_cells_counter > 3 {
                    next_iteration[(x as usize, y as usize)] = 0;
                } if colored_cells_counter == 2 || colored_cells_counter == 3 {
                    next_iteration[(x as usize, y as usize)] = self[(x as usize, y as usize)]
                } if colored_cells_counter == 3 && self[(x as usize, y as usize)] == 0 {
                    next_iteration[(x as usize, y as usize)] = u32::MAX;
                }

                if (x, y) == (1, 1) {
                    dbg!(colored_cells_counter);
                  }

                colored_cells_counter = 0;
            }
        }
        *self = next_iteration;
    }

    pub fn handle_user_input(&mut self, window: &Window) {
        if let Some((x, y)) = window.get_mouse_pos(MouseMode::Discard) {
            if window.get_mouse_down(MouseButton::Left) {
                self[(x as usize, y as usize)] = u32::MAX;
            }
        }
    }
}

impl fmt::Display for WindowBuffer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let line_len = self.buffer.chunks(self.width);
        for i in line_len {
            for a in i {
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
// GRID CREATION END

fn main() {
    let mut buffer = WindowBuffer::new(WIDTH, HEIGHT);

    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions {
            scale: minifb::Scale::X8,
            ..WindowOptions::default()
        },
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    let mut instant = Instant::now();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        buffer.handle_user_input(&window);
        let two_seconds = Duration::from_secs(2);
        if instant.elapsed() >= two_seconds {
            buffer.update();
            instant = Instant::now();
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

    #[test]
    fn display_window_buffer() {
        let mut buffer = WindowBuffer::new(4, 4);
        assert_eq!(
            buffer.to_string(),
            "....
....
....
....
"
        );
        buffer.buffer[1] = 1;
        buffer.buffer[3] = 3;
        buffer.buffer[4] = 4;
        buffer.buffer[6] = 6;
        buffer.buffer[9] = 9;
        buffer.buffer[11] = 11;
        buffer.buffer[12] = 12;
        buffer.buffer[14] = 14;
        assert_eq!(
            buffer.to_string(),
            ".#.#
#.#.
.#.#
#.#.
"
        );
    }

    #[test]
    fn display_window_buffer2() {
        let mut buffer = WindowBuffer::new(4, 4);
        assert_snapshot!(
            buffer.to_string(),
            @r###"
        ....
        ....
        ....
        ....
        "###
        );
        buffer.buffer[1] = 1;
        buffer.buffer[3] = 3;
        buffer.buffer[4] = 4;
        buffer.buffer[6] = 6;
        buffer.buffer[9] = 9;
        buffer.buffer[11] = 11;
        buffer.buffer[12] = 12;
        buffer.buffer[14] = 14;
        assert_snapshot!(
            buffer.to_string(),
            @r###"
        .#.#
        #.#.
        .#.#
        #.#.
        "###
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
        assert_snapshot!(
            buffer.to_string(),
            @r###"
        .#.#
        #.#.
        .#.#
        #.#.
        "###
        );
    }

    #[test]
    fn cells_life_square() {
        let mut buffer = WindowBuffer::new(5, 4);
        buffer[(1, 1)] = u32::MAX;
        buffer[(1, 2)] = u32::MAX;
        buffer[(2, 1)] = u32::MAX;
        buffer[(2, 2)] = u32::MAX;
        assert_snapshot!(
            buffer.to_string(),
            @r###"
        .....
        .##..
        .##..
        .....
        "###
        );
        buffer.update();
        assert_snapshot!(
            buffer.to_string(),
            @r###"
        .....
        .##..
        .##..
        .....
        "###
        );
    }

    #[test]
    fn cells_life_line() {
        let mut buffer = WindowBuffer::new(5, 4);
        buffer[(1, 1)] = u32::MAX;
        buffer[(1, 2)] = u32::MAX;
        buffer[(1, 3)] = u32::MAX;
        assert_snapshot!(
            buffer.to_string(),
            @r###"
        .....
        .#...
        .#...
        .#...
        "###
        );
        buffer.update();
        assert_snapshot!(
            buffer.to_string(),
            @r###"
        .....
        .....
        ###..
        .....
        "###
        );
    }

    #[test]
    fn cells_life_strange_shape() {
        let mut buffer = WindowBuffer::new(10, 10);
        buffer[(2, 0)] = u32::MAX;
        buffer[(3, 1)] = u32::MAX;
        buffer[(1, 2)] = u32::MAX;
        buffer[(2, 2)] = u32::MAX;
        buffer[(3, 2)] = u32::MAX;
        assert_snapshot!(
            buffer.to_string(),
            @r###"
        ..#.......
        ...#......
        .###......
        ..........
        ..........
        ..........
        ..........
        ..........
        ..........
        ..........
        "###
        );
        buffer.update();
        assert_snapshot!(
            buffer.to_string(),
            @r###"
        ..........
        .#.#......
        ..##......
        ..#.......
        ..........
        ..........
        ..........
        ..........
        ..........
        ..........
        "###
        );
    }
}
