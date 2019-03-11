use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::screen::*;
use rulinalg::vector::Vector;
use rulinalg::matrix::Matrix;
use std::io::{Write, stdout, stdin};
use std::{str, f32};

mod sloth;
use sloth::Triangle;

fn rot(x: f32, y: f32) -> Matrix<f32> {
    Matrix::new(4, 4, vec![ x.cos(), 0.0, x.sin(), 0.0, 
                            0.0,      y.cos(), -y.sin(),     0.0,
                            -x.sin(), y.sin(), x.cos()*y.cos(), 0.0,
                            0.0,      0.0, 0.0,     1.0])
}

fn main() {
    let mut stdout = AlternateScreen::from(stdout().into_raw_mode().unwrap());

    let mut triangle = Triangle {
        v1: Vector::new(vec![0.0, 0.0, 0.0, 1.0]),
        v2: Vector::new(vec![-40.0, 40.0, 40.0, 1.0]),
        v3: Vector::new(vec![40.0, 40.0, 40.0, 1.0])
    };
    let t = Matrix::new(4, 4, vec![1.0, 0.0, 0.0, 50.0, 
                            0.0, 1.0, 0.0, 0.0,
                            0.0, 0.0, 1.0, 0.0,
                            0.0, 0.0, 0.0, 1.0]); // For whatever reason, transforming the rotation matrix with this breaks machine broke???
    let mut x:f32 = 0.0;
    let mut y:f32 = 0.0;
    loop {
        x += 0.03;
        y += 0.01;
        let size = termion::terminal_size().unwrap();
        let mut frame_buffer = vec![0x0020u8; ((size.0-1)*(size.1-1)) as usize];
        let mut z_buffer     = vec![f32::MAX; ((size.0-1)*(size.1-1)) as usize];
        sloth::draw_triangle(&mut frame_buffer, &mut z_buffer, &triangle, rot(x, y), size.0 as usize, size.1 as usize);
        sloth::draw_triangle(&mut frame_buffer, &mut z_buffer, &triangle, rot(std::f32::consts::PI/2.0 + x, y), size.0 as usize, size.1 as usize);
        sloth::draw_triangle(&mut frame_buffer, &mut z_buffer, &triangle, rot(std::f32::consts::PI + x, y), size.0 as usize, size.1 as usize);
        sloth::draw_triangle(&mut frame_buffer, &mut z_buffer, &triangle, rot(3.0*std::f32::consts::PI/2.0 + x, y), size.0 as usize, size.1 as usize);
        match str::from_utf8(&frame_buffer) {
            Ok(v) => println!("{}{}{}", termion::clear::All, termion::cursor::Goto(1,1), v),
            Err(e) => panic!("Invalid UTF-8 shade chosen: {}", e),
        };
        stdout.flush().unwrap();

        let c = stdin().keys().next().unwrap();
        match c.unwrap() {
            Key::Char('q') => break,
            _=> {
            }
        }
    }
    stdout.flush().unwrap();
}
