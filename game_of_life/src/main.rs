use clap::Parser;
use std::fmt;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::time::{Duration, Instant};
use window_rs::WindowBuffer;
use graphic::{Graphic, Key, minifb::Minifb};

//CLI
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Optional name to operate on
    #[arg(long, default_value_t = 160)]
    width: usize,
    #[arg(long, default_value_t = 90)]
    height: usize,
    #[arg(long)]
    file_path: Option<String>,
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
    window_buffer: WindowBuffer,
    space_count: usize,
    small_break_timer: Instant,
    speed: u64,
}

impl World {
    pub fn new(
        window_buffer: WindowBuffer,
        space_count: usize,
        small_break_timer: Instant,
        speed: u64,
    ) -> Self {
        Self {
            window_buffer,
            space_count,
            small_break_timer,
            speed,
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

                if self.window_buffer.get(x - 1, y - 1) == Some(u32::MAX) {
                    colored_cells_counter += 1;
                }
                if self.window_buffer.get(x - 1, y) == Some(u32::MAX) {
                    colored_cells_counter += 1;
                }
                if self.window_buffer.get(x - 1, y + 1) == Some(u32::MAX) {
                    colored_cells_counter += 1;
                }
                if self.window_buffer.get(x, y - 1) == Some(u32::MAX) {
                    colored_cells_counter += 1;
                }
                if self.window_buffer.get(x, y + 1) == Some(u32::MAX) {
                    colored_cells_counter += 1;
                }
                if self.window_buffer.get(x + 1, y - 1) == Some(u32::MAX) {
                    colored_cells_counter += 1;
                }
                if (self.window_buffer.get(x + 1, y)) == Some(u32::MAX) {
                    colored_cells_counter += 1;
                }
                if self.window_buffer.get(x + 1, y + 1) == Some(u32::MAX) {
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
                    next_iteration[(x as usize, y as usize)] = u32::MAX;
                }

                colored_cells_counter = 0;
            }
        }
        self.window_buffer = next_iteration;
    }

    pub fn handle_user_input <W: Graphic>(&mut self, window: &W, cli: &Cli) -> std::io::Result<()> {
        if let Some((x, y)) = window.get_mouse_pos(graphic::Mouse::Discard) {
            if window.get_mouse_down(graphic::Mouse::Left) {
                self.window_buffer[(x as usize, y as usize)] = u32::MAX;
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

fn main() -> std::io::Result<()> {
    let cli = Cli::parse();

    let mut buffer = World::new(
        WindowBuffer::new(cli.width, cli.height),
        0,
        Instant::now(),
        2,
    );

    if cli.file_path != None {
        buffer.window_buffer.reset();
        buffer.speed = 0;

        let mut save_file = File::open(cli.file_path.clone().unwrap())?;

        let mut saved_chunk: [u8; 8] = [0; 8];

        save_file.read_exact(&mut saved_chunk)?;
        let new_width = usize::from_be_bytes(saved_chunk);

        if new_width != cli.width {
            panic!("width different from saved width");
        }

        save_file.read_exact(&mut saved_chunk)?;
        let new_height = usize::from_be_bytes(saved_chunk);

        if new_height != cli.height {
            panic!("height different from saved height");
        }

        save_file.read_exact(&mut saved_chunk)?;
        buffer.speed = u64::from_be_bytes(saved_chunk);

        let mut saved_chunk_2: [u8; 4] = [0; 4];

        for y in 0..buffer.window_buffer.height() {
            for x in 0..buffer.window_buffer.width() {
                save_file.read_exact(&mut saved_chunk_2)?;
                buffer.window_buffer[(x, y)] = u32::from_be_bytes(saved_chunk_2)
            }
        }
    }

    let mut window = Minifb::new("Game Of Life - ESC to exit", cli.width, cli.height);

    let mut instant = Instant::now();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let _ = buffer.handle_user_input(&window, &cli);
        let two_seconds = Duration::from_secs(buffer.speed());
        if instant.elapsed() >= two_seconds {
            buffer.update();
            instant = Instant::now();
        }

        window
            .update_with_buffer(&buffer)

    }

    Ok(())
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
        let mut buffer: World = World::new(WindowBuffer::new(5, 4), 0, Instant::now(), 0);
        buffer.window_buffer[(1, 1)] = u32::MAX;
        buffer.window_buffer[(1, 2)] = u32::MAX;
        buffer.window_buffer[(2, 1)] = u32::MAX;
        buffer.window_buffer[(2, 2)] = u32::MAX;
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
        let mut buffer = World::new(WindowBuffer::new(5, 4), 0, Instant::now(), 0);
        buffer.window_buffer[(1, 1)] = u32::MAX;
        buffer.window_buffer[(1, 2)] = u32::MAX;
        buffer.window_buffer[(1, 3)] = u32::MAX;
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
        let mut buffer = World::new(WindowBuffer::new(10, 10), 0, Instant::now(), 0);
        buffer.window_buffer[(2, 0)] = u32::MAX;
        buffer.window_buffer[(3, 1)] = u32::MAX;
        buffer.window_buffer[(1, 2)] = u32::MAX;
        buffer.window_buffer[(2, 2)] = u32::MAX;
        buffer.window_buffer[(3, 2)] = u32::MAX;
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
