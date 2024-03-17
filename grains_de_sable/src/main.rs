use minifb::{Key, Window, WindowOptions};
use std::fmt;
use rand::Rng;
use rand::rngs::StdRng;
use rand::SeedableRng;
use minifb::MouseMode;
use minifb::MouseButton;
use std::cmp::Reverse;
use window_rs::WindowBuffer;

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

#[derive(Clone)]
#[derive(Debug)]
pub struct Sand {
    x: usize,
    y: usize,

    color: u32,
}


pub struct World {
    world: Vec<Sand>,
    colors: Box<dyn Iterator<Item = (u8, u8, u8)>>,
}
impl World {
    pub fn update(&mut self, buffer: &WindowBuffer) {

        self.world.sort_unstable_by_key(|sand| Reverse(sand.y));

        for index in 0..self.world.len() {
            let mut sand = self.world[index].clone();
            // On ne mets à jour que les grains de sable qui sont sur la ligne observée
            sand.y += 1;
            if sand.y >= buffer.height() {
                continue;
            }
                sand.y += 1;
                if self.world.iter().any(|s| (sand.x, sand.y) == (s.x, s.y)) {
                    if sand.x < buffer.width() && sand.x > 0 {
                        let mut rng = StdRng::seed_from_u64(0);
                        let n: u32 = rng.gen_range(0..=1);

                        if n == 0 {
                            if self.world.iter().all(|s| (sand.x-1, sand.y) != (s.x, s.y)){
                                sand.x -= 1;
                            } else if self.world.iter().all(|s| (sand.x+1, sand.y) != (s.x, s.y)) {
                                sand.x += 1;
                            } else {
                                continue
                            }
 
                        } else {
                            if self.world.iter().all(|s| (sand.x+1, sand.y) != (s.x, s.y)){
                                sand.x += 1;
                            } else if self.world.iter().all(|s| (sand.x-1, sand.y) != (s.x, s.y)) {
                                sand.x -= 1;
                            } else {
                                continue
                            }
                        }
                    };
                }
                if sand.y < buffer.height() {
                    self.world[index] = sand;
                } 
        }
    }

    pub fn display(&self, buffer: &mut WindowBuffer) {

        buffer.reset();

        for sand in self.world.iter() {
            buffer[(sand.x, sand.y)] = sand.color;
        }
    }

    pub fn handle_user_input(&mut self, window: &Window) {
        if let Some((x, y)) = window.get_mouse_pos(MouseMode::Discard) {
            if window.get_mouse_down(MouseButton::Left) {
                let (x, y) = (x as usize, y as usize);
                let thickness = 2;

                for x in (x - thickness)..(x + thickness) {
                    for y in (y - thickness)..(y + thickness) {

                        let (r, g, b) = self.colors.next().unwrap();
                                let sand = Sand {
                                    x: x as usize,
                                    y: y as usize,
                                    color: rgb(r, g, b),
                                };

                        self.world.push(sand);
                    }
                }
            }   
        }
    }

    pub fn color_generator() -> impl Iterator<Item = (u8, u8, u8)> {
        let channel = (0..u8::MAX) // On monte jusqu’à u8::MAX de 1 en 1
            .chain(std::iter::repeat(u8::MAX).take(u8::MAX as usize * 2)) // On reste a u8::MAX PENDANT u8::MAX itération pour que l’autre channel puisse nous rejoindre
            .chain((0..=u8::MAX).rev()) // On redescend jusqu’à 0
            .chain(std::iter::repeat(0).take(u8::MAX as usize * 2)) // On reste a 0 pendant u8::MAX * 2
            .cycle(); // On répète tout ça a l’infini
    
        let colors = channel
            .clone()
            .skip(u8::MAX as usize * 2)
            .zip(channel.clone())
            .zip(channel.clone().skip(u8::MAX as usize * 4))
            .map(|((r, g), b)| (r, g, b));
    
        colors
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
        world: Vec::new(),
        colors: Box::new(World::color_generator()),
    };

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        world.handle_user_input(&window);
        world.update(&buffer);
        world.display(&mut buffer);

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&buffer.buffer(), buffer.width(), buffer.height())
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
    use insta::assert_snapshot;

    #[test]
    fn test_rgb() {
        assert_eq!(rgb(0, 0, 0), 0x00_00_00_00);
        assert_eq!(rgb(255, 255, 255), 0x00_ff_ff_ff);
        assert_eq!(rgb(0x12, 0x34, 0x56), 0x00_12_34_56);
        assert_eq!(rgb2(0, 0, 0), 0x00_00_00_00);
        assert_eq!(rgb2(255, 255, 255), 0x00_ff_ff_ff);
        assert_eq!(rgb2(0x12, 0x34, 0x56), 0x00_12_34_56);
    }

    proptest! {
        #[test]
        fn test_both_rgb(red in 0u8.., green in 0u8.., blue  in 0u8..) {
            assert_eq!(rgb(red, green, blue), rgb2(red, green, blue));
        }
    }

    #[test]
    fn simple_sand_drop() {
        let mut buffer = WindowBuffer::new(5, 4);
        let mut world = World {
            world: vec![Sand { x: 3, y: 0, color: 250}],
            colors: Box::new(World::color_generator()),
        };
        world.display(&mut buffer);
        assert_snapshot!(
            buffer.to_string(),
            @r###"
        ...#.
        .....
        .....
        .....
        "###
        );

        world.update(&buffer);
        world.display(&mut buffer);
        assert_snapshot!(
            buffer.to_string(),
            @r###"
        .....
        ...#.
        .....
        .....
        "###
        );

        world.update(&buffer);
        world.display(&mut buffer);
        assert_snapshot!(
            buffer.to_string(),
            @r###"
        .....
        .....
        ...#.
        .....
        "###
        );
    }

    #[test]
    #[should_panic]
    fn test_y_bigger_than_buffer() {
        let mut buffer = WindowBuffer::new(5, 4);
        let mut world = World {
            world: vec![Sand { x: WIDTH / 2, y: 3, color: rgb(u8::MAX, u8::MAX, 0) }],
            colors: Box::new(World::color_generator()),
        };

        world.update(&buffer);
        world.display(&mut buffer);
        assert_snapshot!(
            buffer.to_string(),
            @r###""###
        );
    }
    #[test]
    fn sand_physic() {
        let mut buffer = WindowBuffer::new(5, 4);
        let mut world = World {
            world: vec![
                Sand { x: 2, y: 2, color: rgb(u8::MAX, u8::MAX, 0) },
                Sand { x: 2, y: 1, color: rgb(u8::MAX, u8::MAX, 0) },
                Sand { x: 2, y: 0, color: rgb(u8::MAX, u8::MAX, 0) },
            ],
            colors: Box::new(World::color_generator()),
        };
        world.display(&mut buffer);
        assert_snapshot!(
            buffer.to_string(),
            @r###"
        ..#..
        ..#..
        ..#..
        .....
        "###
        );
        world.update(&buffer);
        world.display(&mut buffer);
        assert_snapshot!(
            buffer.to_string(),
            @r###"
        .....
        ..#..
        ..#..
        ..#..
        "###
        );
        world.update(&buffer);
        world.display(&mut buffer);
        assert_snapshot!(
            buffer.to_string(),
            @r###"
        .....
        .....
        ..#..
        ..##.
        "###
        );
        world.update(&buffer);
        world.display(&mut buffer);
        assert_snapshot!(
            buffer.to_string(),
            @r###"
        .....
        .....
        .....
        .###.
        "###
        );
    }
}
