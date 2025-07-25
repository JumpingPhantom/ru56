use ru56_lib::traits::Object2D;
use ru56_lib::traits::Renderer;
use std::process::Command;

pub struct Renderer2D {
    pub viewport_size: (usize, usize),
    pub resolution: (usize, usize),
}

impl Renderer2D {
    pub fn new() -> Self {
        Renderer2D {
            viewport_size: (80, 30),
            resolution: (80, 30),
        }
    }
}

impl Renderer for Renderer2D {
    fn render(&self, data: &Vec<Box<dyn Object2D>>) {
        clear_terminal();

        let (screen_w, screen_h) = self.resolution;
        let mut screen = vec![vec![' '; screen_w]; screen_h];

        // Choose a scale factor to fit the viewport in the screen
        let scale_x = screen_w as f32 / self.viewport_size.0 as f32;
        let scale_y = screen_h as f32 / self.viewport_size.1 as f32;
        let scale = scale_x.min(scale_y);

        for object in data {
            let (sx, sy) = world_to_screen(
                object.position().x,
                object.position().y,
                0,
                0,
                scale,
                screen_w,
                screen_h,
            );
            let x = sx.round() as isize;
            let y = sy.round() as isize;
            if x >= 0 && x < screen_w as isize && y >= 0 && y < screen_h as isize {
                screen[y as usize][x as usize] = '*';
            }
        }

        for row in screen {
            let line: String = row.into_iter().collect();
            println!("{}", line);
        }
    }
}

pub fn world_to_screen(
    px: f32,
    py: f32,
    viewport_x: usize,
    viewport_y: usize,
    scale: f32,
    screen_w: usize,
    screen_h: usize,
) -> (f32, f32) {
    let sx = (px - viewport_x as f32) * scale + screen_w as f32 / 2.0;
    let sy = (py - viewport_y as f32) * scale + screen_h as f32 / 2.0;
    (sx, sy)
}

fn clear_terminal() {
    if cfg!(target_os = "windows") {
        // For Windows
        Command::new("cmd").args(&["/C", "cls"]).status().unwrap();
    } else {
        // For Unix-like systems
        Command::new("clear").status().unwrap();
    }
}
