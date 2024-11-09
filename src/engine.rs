use std::{io, time::{Duration, Instant}};

use crossterm::{cursor, event::{self, Event, KeyCode}, execute, terminal, ExecutableCommand};

use crate::canvas::Canvas;

pub struct Engine {
    pub canvas: Canvas,
    time: Instant,
    fov: f32,
    max_fps: f32,
    should_stop: bool,
    show_ui: bool
}

impl Engine {
    pub fn new() -> Self {
        Self {
            canvas: Canvas::new(),
            time: Instant::now(),
            fov: 90.0,
            max_fps: 60.0,
            should_stop: false,
            show_ui: true
        }
    }

    pub fn run(&mut self) -> io::Result<()> {
        terminal::enable_raw_mode()?;
        self.canvas.stdout.execute(cursor::Hide)?;
        loop {
            let delta_time: f32 = self.time.elapsed().as_secs_f32();
            if delta_time >= 1.0 / self.max_fps {
                self.update(delta_time);
                if self.should_stop {
                    break;
                }
                self.render(delta_time)?;
                self.time = Instant::now();
            }
        }
        terminal::disable_raw_mode()?;
        self.canvas.stdout.execute(cursor::Show)?;
        self.canvas.clear()?;
        self.canvas.update()?;
        Ok(())
    }

    fn update(&mut self, delta_time: f32) {
        if event::poll(Duration::from_secs(0)).is_ok_and(|has_event| has_event) {
            if let Ok(Event::Key(event)) = event::read() {
                match event.code {
                    KeyCode::Char('q') => {
                        self.should_stop = true;
                        return;
                    },
                    KeyCode::Up => {
                        self.canvas.verticies.iter_mut().for_each(|v| (*v) = v.scale(1.01f32));
                    },
                    KeyCode::Down => {
                        self.canvas.verticies.iter_mut().for_each(|v| (*v) = v.scale(0.99f32));
                    },
                    KeyCode::Left => {
                        self.canvas.verticies.iter_mut().for_each(|v| *v = v.rotate_y(delta_time * 4.0));
                    },
                    KeyCode::Right => {
                        self.canvas.verticies.iter_mut().for_each(|v| *v = v.rotate_y(-delta_time * 4.0));
                    },
                    KeyCode::Char('j') => {
                        self.canvas.verticies.iter_mut().for_each(|v| *v = v.rotate_x(delta_time * 4.0));
                    },
                    KeyCode::Char('k') => {
                        self.canvas.verticies.iter_mut().for_each(|v| *v = v.rotate_x(-delta_time * 4.0));
                    },
                    KeyCode::Char('h') => {
                        self.canvas.verticies.iter_mut().for_each(|v| *v = v.rotate_z(delta_time * 4.0));
                    },
                    KeyCode::Char('l') => {
                        self.canvas.verticies.iter_mut().for_each(|v| *v = v.rotate_z(-delta_time * 4.0));
                    },
                    KeyCode::Tab => {
                        self.show_ui = !self.show_ui;
                    }
                    _ => {}
                }
            }
        }
    }

    fn render(&mut self, delta_time: f32) -> io::Result<()> {
        self.canvas.clear()?;
        self.canvas.draw(self.fov)?;
        if self.show_ui {
            execute!(self.canvas.stdout, cursor::MoveTo(0, 0))?;
            print!("TDEngine by Killar. Press 'q' to quit. FPS: {}", (1.0 / delta_time).round());
        }
        self.canvas.update()?;
        Ok(())
    }
}
