use std::io::{self, Stdout, Write};

use crossterm::{cursor, queue, style::{self, Stylize}, terminal, ExecutableCommand};

use crate::vec::{Vector2, Vector3};

pub struct Canvas {
    pub stdout: Stdout,
    pub verticies: Vec<Vector3<f32>>,
    pub indicies: Vec<Vector2<usize>>
}

impl Canvas {
    pub fn new() -> Self {
        Self {
            stdout: io::stdout(),
            verticies: Vec::new(),
            indicies: Vec::new()
        }
    }

    pub fn clear(&mut self) -> io::Result<()> {
        self.stdout.execute(terminal::Clear(terminal::ClearType::All))?;
        Ok(())
    }

    pub fn set_pixel(&mut self, point: Vector2<i32>, char: char) -> io::Result<()> {
        let size: Vector2<u16> = self.get_screen_size()?;
        let centered_point: Vector2<i32> = Vector2::new(point.x + (size.x / 2) as i32, point.y + (size.y / 2) as i32);
        if centered_point.x < 0 || centered_point.y < 0 || centered_point.x > size.x.into() || centered_point.y > size.y.into() {
            return Ok(());
        }
        queue!(self.stdout, cursor::MoveTo(centered_point.x as u16, centered_point.y as u16), style::PrintStyledContent(char.white()))?;
        Ok(())
    }

    pub fn draw_line(&mut self, point1: Vector2<i32>, point2: Vector2<i32>) -> io::Result<()> {
        let mut current_x: i32 = point1.x;
        let mut current_y: i32 = point1.y;
        let delta_x: i32 = (point1.x - point2.x).abs();
        let delta_y: i32 = (point1.y - point2.y).abs();
        let slope_x: i32 = if point1.x < point2.x {
            1
        } else {
            -1
        };
        let slope_y: i32 = if point1.y < point2.y {
            1
        } else {
            -1
        };
        let mut error: i32 = if delta_x > delta_y {
            delta_x
        } else {
            -delta_y
        } / 2;
        let mut old_error: i32;
        loop {
            self.set_pixel(Vector2::new(current_x, current_y), '#')?;
            if current_x == point2.x && current_y == point2.y {
                return Ok(());
            }
            old_error = 2 * error;
            if old_error > -delta_x {
                error -= delta_y;
                current_x += slope_x;
            }
            if old_error < delta_y {
                error += delta_x;
                current_y += slope_y;
            }
        }
    }

    pub fn draw(&mut self, fov: f32) -> io::Result<()> {
        let verticies_len: usize = self.verticies.len();
        for index in self.indicies.clone().iter() {
            if index.x < verticies_len && index.y < verticies_len {
                let point1: Vector2<i32> = self.verticies[index.x].projected(fov).round();
                let point2: Vector2<i32> = self.verticies[index.y].projected(fov).round();
                self.draw_line(point1, point2)?;
            }
        }
        Ok(())
    }

    pub fn update(&mut self) -> io::Result<()> {
        self.stdout.flush()?;
        Ok(())
    }

    pub fn get_screen_size(&self) -> io::Result<Vector2<u16>> {
        let size = terminal::window_size()?;
        Ok(Vector2::new(size.columns, size.rows))
    }
}
