use clap::Parser;
use game_of_life::Cli;
use game_of_life::World;
use window_rs::WindowBuffer;
use std::fs::File;
use std::io::Read;
use web_time::{Duration, Instant};
use graphic::{Graphic, Key, minifb::Minifb};


fn main() -> std::io::Result<()> {
    let cli = Cli::parse();

    let mut buffer = World::new(
        WindowBuffer::new(cli.width, cli.height),
        0,
        Instant::now(),
        2,
        0x0066CC33,
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
            .update_with_buffer(&buffer.window_buffer)

    }

    Ok(())
}

