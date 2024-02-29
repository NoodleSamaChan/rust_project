use minifb::{Key, Window, WindowOptions};
use std::fmt;

const WIDTH: usize = 640;
const HEIGHT: usize = 360;


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

struct Sand {
    x: usize,
    y: usize,
}

struct World {
    world: Vec<Sand>,
}
impl World {
    pub fn update(&mut self) {
        // on va modifier les grains donc on doit itérer en mode mutable sur les grains de sable
        for sand in self.world.iter_mut() {
            sand.y += 1;
        }
    }

    pub fn display(&self, buffer: &mut WindowBuffer) {
        for sand in self.world.iter() {
            buffer[(sand.x, sand.y)] = u32::MAX;
        }
    }
}

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

    let mut world = World {
        world: vec![Sand { x: WIDTH / 2, y: 0 }],
    };

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        for y in 0..buffer.height() {
            for x in 0..buffer.width() {
                            // On commence par convertir l'index en une valeur qui va de `0` à `1` où `1` sera renvoyé lorsqu’on est a droite de l’écran.
            // Si on veut c'est un simple pourcentage qui indique notre progression de la gauche vers la droite.
            let progression = x as f64 / buffer.width() as f64;

            // En multipliant la `progression` par `u8::MAX` on fait passer cette valeur de `0` à `u8::MAX` (`255`). On peut convertir le tout en `u8`.
            let color = (progression * u8::MAX as f64) as u8;

            // Pour notre dégradé on utilise seulement le canal du rouge
            buffer[(x, y)] = rgb(0, 0, color);

        }

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&buffer.buffer(), WIDTH, HEIGHT)
            .unwrap();

        }
    }
}

pub fn rgb(red: u8, green: u8, blue: u8) -> u32 {
    let a = u32::from(red);
    let b: u32 = u32::from(green);
    let c = u32::from(blue);

    let new_red = a << 16;
    let new_green = b << 8;

    let final_number = new_red | new_green | c;

    return final_number
    
}

pub fn rgb2(red: u8, green: u8, blue: u8) -> u32 {

    let a = red;
    let b = green;
    let c = blue;
    
    let value = u32::from_be_bytes([00, a, b, c]);

    u32::from(value)
}

#[cfg(test)]
mod test {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn test_rgb() {
        assert_eq!(rgb(0, 0, 0), 0x00_00_00_00);
        assert_eq!(rgb(255, 255, 255), 0x00_ff_ff_ff);
        assert_eq!(rgb(0x12, 0x34, 0x56), 0x00_12_34_56);
        assert_eq!(rgb2(0, 0, 0), 0x00_00_00_00);
        assert_eq!(rgb2(255, 255, 255), 0x00_ff_ff_ff);
        assert_eq!(rgb2(0x12, 0x34, 0x56), 0x00_12_34_56);
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
    fn display_window_buffer_v2() {
        let mut buffer = WindowBuffer::new(4, 4);
        assert_display_snapshot!(buffer.to_string(), @r###"
        ....
        ....
        ....
        ....
        "###);

        buffer.buffer[1] = 1;
        buffer.buffer[3] = 3;
        buffer.buffer[4] = 4;
        buffer.buffer[6] = 6;
        buffer.buffer[9] = 9;
        buffer.buffer[11] = 11;
        buffer.buffer[12] = 12;
        buffer.buffer[14] = 14;
        assert_display_snapshot!(buffer.to_string(), @r###"
        .#.#
        #.#.
        .#.#
        #.#.
        "###); 
    }

    proptest! {
        #[test]
        fn test_both_rgb(red in 0u8.., green in 0u8.., blue  in 0u8..) {
            assert_eq!(rgb(red, green, blue), rgb2(red, green, blue));
        }
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
