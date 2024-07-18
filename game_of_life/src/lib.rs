use clap::Parser;
use std::fs::File;
use std::io::Write;
use web_time::{Duration, Instant};
use window_rs::WindowBuffer;
use graphic::Graphic;

//CLI
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Optional name to operate on
    #[arg(long, default_value_t = 30)]
    pub width: usize,
    #[arg(long, default_value_t = 30)]
    pub height: usize,
    #[arg(long)]
    pub file_path: Option<String>,
}
//CLI END

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
#[derive(Debug)]
pub struct World {
    pub window_buffer: WindowBuffer,
    pub space_count: usize,
    pub small_break_timer: Instant,
    pub speed: u64,
    pub colour_cell: u32,
}

impl World {
    pub fn new(
        window_buffer: WindowBuffer,
        space_count: usize,
        small_break_timer: Instant,
        speed: u64,
        colour_cell: u32,
    ) -> Self {
        Self {
            window_buffer,
            space_count,
            small_break_timer,
            speed,
            colour_cell,
        }
    }

    pub fn space_count(&mut self) -> usize {
        self.space_count
    }
    pub fn small_break_timer(&mut self) -> Instant {
        self.small_break_timer
    }
    pub fn speed(&mut self) -> u64 {
        self.speed
    }
    pub fn update(&mut self) {
        if self.space_count % 2 == 0 {
            self.check_surroundings()
        }
    }

    pub fn check_surroundings(&mut self) {
        let mut colored_cells_counter: usize = 0;
        let mut next_iteration =
            WindowBuffer::new(self.window_buffer.width(), self.window_buffer.height());

        for x in 0..self.window_buffer.width() {
            for y in 0..self.window_buffer.height() {
                let x = x as isize;
                let y = y as isize;

                if self.window_buffer.get(x - 1, y - 1) == Some(self.colour_cell) {
                    colored_cells_counter += 1;
                }
                if self.window_buffer.get(x - 1, y) == Some(self.colour_cell) {
                    colored_cells_counter += 1;
                }
                if self.window_buffer.get(x - 1, y + 1) == Some(self.colour_cell) {
                    colored_cells_counter += 1;
                }
                if self.window_buffer.get(x, y - 1) == Some(self.colour_cell) {
                    colored_cells_counter += 1;
                }
                if self.window_buffer.get(x, y + 1) == Some(self.colour_cell) {
                    colored_cells_counter += 1;
                }
                if self.window_buffer.get(x + 1, y - 1) == Some(self.colour_cell) {
                    colored_cells_counter += 1;
                }
                if (self.window_buffer.get(x + 1, y)) == Some(self.colour_cell) {
                    colored_cells_counter += 1;
                }
                if self.window_buffer.get(x + 1, y + 1) == Some(self.colour_cell) {
                    colored_cells_counter += 1;
                }

                if colored_cells_counter < 2 || colored_cells_counter > 3 {
                    next_iteration[(x as usize, y as usize)] = 0;
                }
                if colored_cells_counter == 2 || colored_cells_counter == 3 {
                    next_iteration[(x as usize, y as usize)] =
                        self.window_buffer[(x as usize, y as usize)]
                }
                if colored_cells_counter == 3 && self.window_buffer[(x as usize, y as usize)] == 0 {
                    next_iteration[(x as usize, y as usize)] = self.colour_cell;
                }

                colored_cells_counter = 0;
            }
        }
        self.window_buffer = next_iteration;
    }

    pub fn handle_user_input <W: Graphic>(&mut self, window: &W, cli: &Cli) -> std::io::Result<()> {
        if let Some((x, y)) = window.get_mouse_pos(graphic::Mouse::Discard) {
            if window.get_mouse_down(graphic::Mouse::Left) {
                self.window_buffer[(x as usize, y as usize)] = self.colour_cell;
            }
        }

        if window.is_key_pressed(graphic::Key::Quit,) {
            self.window_buffer.reset();
        }

        if window.is_key_pressed(graphic::Key::Save) {
            let mut save_file = File::create("save_file")?;

            if cli.file_path != None {
                save_file = File::create(cli.file_path.clone().unwrap())?;
            }
            save_file.write_all(&self.window_buffer.width().to_be_bytes())?;
            save_file.write_all(&self.window_buffer.height().to_be_bytes())?;
            save_file.write_all(&self.speed().to_be_bytes())?;

            for number in &self.window_buffer.buffer() {
                save_file.write_all(&number.to_be_bytes())?;
            }

            save_file.flush()?;
        }

        if window.is_key_pressed(graphic::Key::Up) {
            if self.speed > 0 {
                self.speed -= 1;
            }
        }

        if window.is_key_pressed(graphic::Key::Down) {
            self.speed += 1;
        }

        let small_break = Duration::from_millis(0);
        if self.small_break_timer.elapsed() >= small_break {
            window.get_keys_released().iter().for_each(|key| match key {
                graphic::Key::Space => self.space_count += 1,
                _ => (),
            });
            self.small_break_timer = Instant::now();
        }

        Ok(())
    }
}


//TESTS

#[cfg(test)]
mod test {
    use super::*;
    use insta::assert_snapshot;
    use proptest::bits::BitSetLike;

    #[test]
    fn test_rgb() {
        assert_eq!(rgb(0, 0, 0), 0x00_00_00_00);
        assert_eq!(rgb(255, 255, 255), 0x00_ff_ff_ff);
        assert_eq!(rgb(0x12, 0x34, 0x56), 0x00_12_34_56);
    }

    #[test]
    fn cells_life_square() {
        let mut buffer: World = World::new(WindowBuffer::new(5, 4), 0, Instant::now(), 0, 0x0066CC33);
        buffer.window_buffer[(1, 1)] = buffer.colour_cell;
        buffer.window_buffer[(1, 2)] = buffer.colour_cell;
        buffer.window_buffer[(2, 1)] = buffer.colour_cell;
        buffer.window_buffer[(2, 2)] = buffer.colour_cell;
        assert_snapshot!(
            buffer.window_buffer.to_string(),
            @r###"
        .....
        .##..
        .##..
        .....
        "###
        );
        buffer.update();
        assert_snapshot!(
            buffer.window_buffer.to_string(),
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
        let mut buffer = World::new(WindowBuffer::new(5, 4), 0, Instant::now(), 0, 0x0066CC33);
        buffer.window_buffer[(1, 1)] = buffer.colour_cell;
        buffer.window_buffer[(1, 2)] = buffer.colour_cell;
        buffer.window_buffer[(1, 3)] = buffer.colour_cell;
        assert_snapshot!(
            buffer.window_buffer.to_string(),
            @r###"
        .....
        .#...
        .#...
        .#...
        "###
        );
        buffer.update();
        assert_snapshot!(
            buffer.window_buffer.to_string(),
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
        let mut buffer = World::new(WindowBuffer::new(10, 10), 0, Instant::now(), 0, 0x0066CC33);
        buffer.window_buffer[(2, 0)] = buffer.colour_cell;
        buffer.window_buffer[(3, 1)] = buffer.colour_cell;
        buffer.window_buffer[(1, 2)] = buffer.colour_cell;
        buffer.window_buffer[(2, 2)] = buffer.colour_cell;
        buffer.window_buffer[(3, 2)] = buffer.colour_cell;
        assert_snapshot!(
            buffer.window_buffer.to_string(),
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
            buffer.window_buffer.to_string(),
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