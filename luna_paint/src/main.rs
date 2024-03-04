use minifb::{Key, MouseButton, MouseMode, Scale, Window, WindowOptions};


const WIDTH: usize = 620;
const HEIGHT: usize = 360;

fn main() {
    let mut buffer = vec![0u32; WIDTH * HEIGHT];

    let mut window = Window::new(
        "Mouse drawing example - press ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions {
            scale: Scale::X2,
            ..WindowOptions::default()
        },
    )
    .expect("Unable to create the window");

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    let (mut width, mut height) = (WIDTH, HEIGHT);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let (new_width, new_height) = window.get_size();
        if new_width != width || new_height != height {
            let mut new_buffer = vec![0; (new_width) * (new_height)];

            // copy valid bits of old buffer to new buffer
            for y in 0..(height).min(new_height) {
                for x in 0..(width).min(new_width) {
                    new_buffer[y * (new_width) + x] = buffer[y * (width) + x];
                }
            }

            buffer = new_buffer;
            width = new_width;
            height = new_height;
        }

        if let Some((x, y)) = window.get_mouse_pos(MouseMode::Discard) {
            let screen_pos = ((y as usize) * (width / 2)) + x as usize;

            if window.get_mouse_down(MouseButton::Left) {
                buffer[screen_pos] = 0x00ffffff; // white
            }
        }

        // We unwrap here as we want this code to exit if it fails
        window
            .update_with_buffer(&buffer, width / 2, height / 2)
            .unwrap();
    }
}