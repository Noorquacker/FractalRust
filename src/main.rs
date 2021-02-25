#![allow(non_snake_case)]

extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels;

fn mandelbrot(x: i32, y: i32, limit: i32, scale: f32) -> u16 {
  let mut z = num::complex::Complex::<f32>::new(0., 0.);
  let x2 = (x as f32) / 250. / scale - 2. / scale;
  let y2 = (y as f32) / 250. / scale - 2. / scale;
  let c = num::complex::Complex::new(x2, y2);
  for i in 0..limit {
    z = z.powu(2) + c;
    if num::complex::Complex::norm(z) > 2. {
      return i as u16;
    }
  }
  return 0;
}

fn hsv2rgb(h: u16) -> sdl2::pixels::Color {
  let i = h / 60;
  let t = ((h % 60) as f32 * 4.25) as u8; // God awful conversion
  let q = 255 - t;
  match i {
    0 => {
      pixels::Color::RGB(255, t, 0)
    },
    1 => {
      pixels::Color::RGB(q, 255, 0)
    },
    2 => {
      pixels::Color::RGB(0, 255, t)
    },
    3 => {
      pixels::Color::RGB(0, q, 255)
    },
    4 => {
      pixels::Color::RGB(t, 0, 255)
    },
    5 => {
      pixels::Color::RGB(255, 0, q)
    },
    _ => {
      pixels::Color::RGB(0, 0, 0)
    }
  }
}

pub fn main() -> Result<(), String> {
    println!("aaaaaa");
    let sdl_context = sdl2::init()?;
    let video_subsys = sdl_context.video()?;
    let window = video_subsys.window("rust idk thing", 1000, 1000)
    .position_centered()
    .build()
    .map_err(|e| e.to_string())?;
    
    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    canvas.set_draw_color(pixels::Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();
    
    let mut events = sdl_context.event_pump()?;

    let mut offset = num::complex::Complex::new(0, 0);
    let mut scale: f32 = 1.;
    let mut limit = 5;
    
    'running: loop {
      for e in events.poll_iter() {
        match e {
          Event::Quit { .. } => break 'running,
          Event::KeyDown { keycode, .. } => {
            match keycode {
              Some(Keycode::Up) => {
                offset += num::complex::Complex::new(0, -1);
              },
              Some(Keycode::Down) => {
                offset += num::complex::Complex::new(0, 1);
              },
              Some(Keycode::Left) => {
                offset += num::complex::Complex::new(-1, 0);
              },
              Some(Keycode::Right) => {
                offset += num::complex::Complex::new(1, 0);
              },
              Some(Keycode::KpPlus) => {
                scale = (2. as f32).powf(scale.log2() + 1.);
              },
              Some(Keycode::KpMinus) => {
                scale = (2. as f32).powf(scale.log2() - 1.);
              },
              Some(Keycode::Num9) => {
                limit += 1;
              },
              Some(Keycode::Num3) => {
                limit -= 1;
              },
              _ => {}
            }
          },
          _ => {}
        }
      }
      
      canvas.clear();
      
      canvas.set_draw_color(pixels::Color::RGB(255, 255, 255));
      
      canvas.draw_point(sdl2::rect::Point::new(3,5))?;

      for x in 0..1000 {
        for y in 0..1000 {
          let result = mandelbrot(x + offset.re, y + offset.im, limit, scale);
          if result == 0 {
            canvas.set_draw_color(pixels::Color::RGB(0, 0, 0));
            canvas.draw_point(sdl2::rect::Point::new(x, y))?;
          }
          else {
            canvas.set_draw_color(hsv2rgb(((result as f32) / (limit as f32) * 360.) as u16));
            canvas.draw_point(sdl2::rect::Point::new(x, y))?;
          }
        }
      }
      // HSV debug line
      /*
      for x in 0..720 {
        let x2: u16 = x / 2;
        canvas.set_draw_color(hsv2rgb(x2 as u16));
        canvas.draw_point(sdl2::rect::Point::new((x as i32) + 10, 10))?;
      }
      */
      
      canvas.present();
      canvas.set_draw_color(pixels::Color::RGB(0, 0, 0));
    }
    
    Ok(())
}
