extern crate sdl2;

use std::sync::mpsc;
use std::thread;


pub fn mandelbrot(x: f64, y: f64, limit: i32, scale: f64) -> u16 {
	let mut z = num::complex::Complex::<f64>::new(0., 0.);
	let x2 = x / 250. / scale - 2. / scale;
	let y2 = y / 250. / scale - 2. / scale;
	let c = num::complex::Complex::new(x2, y2);
	for i in 1..limit {
		z = z.powu(2) + c;
		if num::complex::Complex::norm(z) > 2. {
			return i as u16;
		}
	}
	return 0;
}

fn hsv2rgb(hin: i32) -> Vec<u8> {
	let h = hin % 360;
	let i = h / 60;
	let t = ((h % 60) as f32 * 4.25) as u8; // God awful conversion
	let q = 255 - t;
	match i {
		0 => {
		vec![255, t, 0]
		},
		1 => {
		vec![q, 255, 0]
		},
		2 => {
		vec![0, 255, t]
		},
		3 => {
		vec![0, q, 255]
		},
		4 => {
		vec![t, 0, 255]
		},
		5 => {
		vec![255, 0, q]
		},
		_ => {
		vec![0, 0, 0]
		}
	}
}

pub fn render(offset: num::complex::Complex::<f64>, scale: f64, limit: i32, threads: i32, tex: &mut sdl2::render::Texture) {
	// How do I freaking multithread in rust
	// Message passing is cool but why do I have to for i {tx.clone()go brrrr;} ???
	let (tx, rx)  = mpsc::channel();
	for i in 0..threads {
		let thread_tx = tx.clone();
		thread::spawn(move || {
			let mut results = vec![i as u8];
			for y_pos in 0..(1000 / threads) {
				let y = y_pos + 1000 / threads * i;
				for x in 0..1000 {
					let result = mandelbrot(x as f64 + offset.re * scale, y as f64 + offset.im * scale, limit, scale);
					if result == 0 {
						results.append(&mut vec![0, 0, 0]);
					}
					else {
						let mut color = hsv2rgb(((result as f64) / 50. * 360.) as i32);
						results.append(&mut color);
					}
				}
			}
			thread_tx.send(results).unwrap();
		});
	}
	
	let mut count = 0;
	for msg in rx {
		let y_offset = 1000 / threads * (msg[0] as i32);
		tex.update(sdl2::rect::Rect::new(0, y_offset, 1000, 1000 / threads as u32), &msg[1..], 3000).unwrap();
		count += 1;
		if count == threads {
			break;
		}
	}
}
