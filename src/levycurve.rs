extern crate sdl2;

#[derive(Clone, Copy, Debug)]
struct Rect3 {
	x: i32,
	y: i32,
	orient: u8	// Right, down, left, up
}

#[derive(Clone, Copy, Debug)]
struct Point(i32, i32);

fn rect3_rot(rect3_in: Rect3, anchor: Point) -> Rect3 {
	let new_x = anchor.0 - (anchor.1 - rect3_in.y);
	let new_y = anchor.1 + (anchor.0 - rect3_in.x);
	if rect3_in.orient == 3 {
		return Rect3 {x: new_x, y: new_y, orient: 0};
	}
	Rect3 {x: new_x, y: new_y, orient: rect3_in.orient + 1}
}

fn rect3_endpoint(rect3_in: Rect3) -> Point {
	match rect3_in.orient {
		0 => {
// 			x = rect3_in.x + 2;
// 			y = rect3_in.y;
			Point(rect3_in.x + 2, rect3_in.y)
		},
		1 => {
			Point(rect3_in.x, rect3_in.y + 2)
		},
		2 => {
			Point(rect3_in.x - 2, rect3_in.y)
		},
		3 => {
			Point(rect3_in.x, rect3_in.y - 2)
		},
		_ => {
			println!("WARNING: Unknown rect! {:?}", rect3_in);
			Point(0, 0)
		}
	}
}

// Yes, we are literally just moving the rects a little
fn push_rect(rect3_in: Rect3, anchor: Point, endpoint: Point) -> Rect3 {
	Rect3 {x:  anchor.0 - rect3_in.x + endpoint.0, y: anchor.1 - rect3_in.y + endpoint.1, orient: rect3_in.orient}
}

pub fn render(_offset: num::complex::Complex::<f64>, _scale: f64, limit: i32, _threads: i32, tex: &mut sdl2::render::Texture) {
	let mut whitevec: Vec<u8> = vec![];
	for _i in 0..1000000 {
		whitevec.push(255);
		whitevec.push(255);
		whitevec.push(255);
	}
	tex.update(sdl2::rect::Rect::new(0, 0, 1000, 1000), &whitevec, 3000).unwrap();
	
	let mut rects = vec![Rect3 {x: 500, y: 500, orient: 0}];
	
	
	//tex.update(sdl2::rect::Rect::new(500, 500, 3, 1), &[0 as u8, 0 as u8, 0 as u8], 3).unwrap();
	let anchor = Point(500, 500);
	for _i in 0..limit {
		let mut new_rects = Vec::<Rect3>::new();
		let endpoint = rect3_endpoint(rects[rects.len() - 1]);
		for j in 0..rects.len() {
			new_rects.push(rect3_rot(rects[j], anchor));
		}
		for j in 0..new_rects.len() {
			rects.push(push_rect(new_rects[j], anchor, endpoint));
		}
	}
	
	for i in 0..rects.len() {
		let rect = &rects[i];
		match rect.orient {
			0 => {
				tex.update(sdl2::rect::Rect::new(rect.x, rect.y, 3, 1), &[0 as u8, 0 as u8, 0 as u8], 3).unwrap();
			},
			1 => {
				tex.update(sdl2::rect::Rect::new(rect.x, rect.y, 1, 3), &[0 as u8, 0 as u8, 0 as u8], 1).unwrap();
			},
			2 => {
				tex.update(sdl2::rect::Rect::new(rect.x - 2, rect.y, 3, 1), &[0 as u8, 0 as u8, 0 as u8], 3).unwrap();
			},
			3 => {
				tex.update(sdl2::rect::Rect::new(rect.x, rect.y - 2, 1, 3), &[0 as u8, 0 as u8, 0 as u8], 1).unwrap();
			},
			_ => {
				println!("WARNING: Invalid orientation {} at rect number {}: {:?}", rect.orient, i, rect);
			}
		}
	}
}
