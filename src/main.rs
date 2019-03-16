use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::screen::*;
use std::io::{Write, stdout, stdin};
use std::{str, f32};

use nalgebra::{Matrix4, Vector4, Perspective3};

mod sloth;
use sloth::Triangle;

fn rot(x: f32, y: f32) -> Matrix4<f32> {
    Matrix4::new(x.cos(), 0.0, x.sin(), 0.0, 
                            0.0,      y.cos(), -y.sin(),     0.0,
                            -x.sin(), y.sin(), x.cos()*y.cos(), 0.0,
                            0.0,      0.0, 0.0,     1.0)
}

fn main() {
    let mut stdout = AlternateScreen::from(stdout().into_raw_mode().unwrap());

    let triangle = Triangle {
        v1: Vector4::new(0.0, 0.0, 0.0, 1.0),
        v2: Vector4::new(-10.0, 10.0, 10.0, 1.0),
        v3: Vector4::new(10.0, 10.0, 10.0, 1.0)
    };
    let persp = Perspective3::new(1.0, 3.14 / 4.0, 1.0, 10.0);
    let persp = persp.as_matrix();
    let t = Matrix4::new(1.0, 0.0, 0.0, 10.0, 
                         0.0, 1.0, 0.0, 0.0,
                         0.0, 0.0, 1.0, 0.0,
                         0.0, 0.0, 0.0, 1.0);
    let mut x:f32 = 0.0;
    let mut y:f32 = 0.0;
    loop {
        let size = termion::terminal_size().unwrap();
        let mut frame_buffer = vec![0x0020u8; ((size.0-1)*(size.1-1)) as usize];
        let mut z_buffer     = vec![f32::MAX; ((size.0-1)*(size.1-1)) as usize];
        sloth::draw_triangle(&mut frame_buffer, &mut z_buffer, &triangle, persp*&t*rot(x, y), size.0 as usize, size.1 as usize);
        sloth::draw_triangle(&mut frame_buffer, &mut z_buffer, &triangle, persp*&t*rot(std::f32::consts::PI/2.0 + x, y), size.0 as usize, size.1 as usize);
        sloth::draw_triangle(&mut frame_buffer, &mut z_buffer, &triangle, persp*&t*rot(std::f32::consts::PI + x, y), size.0 as usize, size.1 as usize);
        sloth::draw_triangle(&mut frame_buffer, &mut z_buffer, &triangle, persp*&t*rot(3.0*std::f32::consts::PI/2.0 + x, y), size.0 as usize, size.1 as usize);
        match str::from_utf8(&frame_buffer) {
            Ok(v) => println!("{}{}{}", termion::clear::All, termion::cursor::Goto(1,1), v),
            Err(e) => panic!("Invalid UTF-8 shade chosen: {}", e),
        };
        stdout.flush().unwrap();

        let c = stdin().keys().next().unwrap();
        match c.unwrap() {
            Key::Char('q') => break,
            Key::Right => x += 0.05,
            Key::Left => x -= 0.05,
            Key::Up => y += 0.05,
            Key::Down => y -= 0.05,
            _=> {
            }
        }
    }
    stdout.flush().unwrap();
}
