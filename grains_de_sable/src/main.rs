use minifb::{Key, Window, WindowOptions};
use std::fmt;
use std::slice::Chunks;

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
        // On va avoir besoin de connaître la taille totale du buffer AVANT d'entrer dans la boucle
        let buffer_len = buffer.buffer.len();

        // Ici grace au `.enumerate()` on récupère l'index auquel on se trouve dans la boucle en plus du pixel a modifier (précédemment appelé `i`)
        for (idx, pixel) in buffer.buffer.iter_mut().enumerate() {
            // On commence par convertir l'index en une valeur qui va de `0` à `1` où `1` sera renvoyé lorsque l'index atteint la taille du buffer.
            // Si on veut c'est un simple pourcentage qui indique notre progression dans tous les pixels à modifier.
            let progression = idx as f64 / buffer_len as f64;
        
            // En multipliant la `progression` par `u8::MAX` on fait passer cette valeur de `0` à `u8::MAX` (`255`). On peut convertir le tout en `u8`.
            let color = (progression * u8::MAX as f64) as u8;
        
            // Pour notre dégradé on utilise seulement le canal du rouge
            *pixel = rgb(0, 0, color);
        }

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&buffer.buffer, WIDTH, HEIGHT)
            .unwrap();

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

    proptest! {
        #[test]
        fn test_both_rgb(red in 0u8.., green in 0u8.., blue  in 0u8..) {
            assert_eq!(rgb(red, green, blue), rgb2(red, green, blue));
        }
    }
}