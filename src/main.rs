#[allow(non_snake_case)]

extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels;
use std::time;

mod mandelbrot;
mod levycurve;
mod dragoncurve;

pub fn main() -> Result<(), String> {
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
	
	let mut offset = num::complex::Complex::<f64>::new(0., 0.);
	let mut scale: f64 = 1.;
	let mut limit = 50;
	
	let threads = 12;
	
	// I'm sorry.
	let mut renderers: Vec<for<'r, 's> fn(num::complex::Complex::<f64>, f64, i32, i32, &'r mut sdl2::render::Texture<'s>)> = Vec::new();
	renderers.push(mandelbrot::render);
	renderers.push(levycurve::render);
	renderers.push(dragoncurve::render);
	
	let mut index = 0;
	let mut cur_renderer = renderers[index];
	
	let texc = canvas.texture_creator();
	let mut tex = texc.create_texture(sdl2::pixels::PixelFormatEnum::RGB24, sdl2::render::TextureAccess::Target, 1000, 1000).unwrap();
	println!("Everyone make fun of SDL2 for thinking that 8 bits times 3 equals {} bytes per pixel", sdl2::pixels::PixelFormatEnum::RGB888.byte_size_per_pixel());
	
	'running: loop {
		for e in events.poll_iter() {
			match e {
				Event::Quit { .. } => break 'running,
				Event::KeyDown { keycode, .. } => {
				match keycode {
					Some(Keycode::Up) => {
						offset += num::complex::Complex::<f64>::new(0., -1.) / (scale as f64);
					},
					Some(Keycode::Down) => {
						offset += num::complex::Complex::<f64>::new(0., 1.) / (scale as f64);
					},
					Some(Keycode::Left) => {
						offset += num::complex::Complex::<f64>::new(-1., 0.) / (scale as f64);
					},
					Some(Keycode::Right) => {
						offset += num::complex::Complex::<f64>::new(1., 0.) / (scale as f64);
					},
					Some(Keycode::KpPlus) => {
						scale =  2f64.powf(scale.log2() + 1.);
					},
					Some(Keycode::KpMinus) => {
						scale = 2f64.powf(scale.log2() - 1.);
					},
					Some(Keycode::Num9) => {
						limit += 1;
					},
					Some(Keycode::Num3) => {
						limit -= 1;
						if limit < 0 {
							limit = 0;
						}
					},
					Some(Keycode::Num7) => {
						index += 1;
						if index >= renderers.len() {
							index = 0;
						}
						cur_renderer = renderers[index];
					}
					_ => {}
					}
				},
				_ => {}
			}
		}
		canvas.set_draw_color(pixels::Color::RGB(0, 0, 0));
		canvas.clear();
		let now = time::SystemTime::now();
		
		cur_renderer(offset, scale, limit, threads, &mut tex);
		
		canvas.copy(&tex, None, None).unwrap();
		canvas.present();
		
		match now.elapsed() {
			Ok(elapsed) => {
				println!("Frametime: {}ms", elapsed.as_millis());
			},
			_ => {}
		}
	}
	Ok(())
}
