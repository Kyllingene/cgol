#![warn(clippy::all)]

use errata::FallibleExt;
use minifb::{Key, Window, WindowOptions};
use rayon::prelude::*;

mod pixel;
use pixel::Pixel;
use rand::{thread_rng, Rng};

const WIDTH: usize = 680;
const HEIGHT: usize = 360;

fn pix(r: u8, g: u8, b: u8) -> Pixel {
    Pixel { r, g, b, _pad: 0 }
}

fn cell(live: bool) -> Pixel {
    if live {
        pix(255, 255, 255)
    } else {
        pix(0, 0, 0)
    }
}

fn pos(idx: usize) -> (usize, usize) {
    (idx % WIDTH, idx / WIDTH)
}

fn idx(x: usize, y: usize) -> usize {
    x + (y * WIDTH)
}

fn incx(x: usize) -> usize {
    if x == WIDTH - 1 {
        0
    } else {
        x + 1
    }
}

fn incy(x: usize) -> usize {
    if x == HEIGHT - 1 {
        0
    } else {
        x + 1
    }
}

fn decx(x: usize) -> usize {
    if x == 0 {
        WIDTH - 1
    } else {
        x - 1
    }
}

fn decy(y: usize) -> usize {
    if y == 0 {
        HEIGHT - 1
    } else {
        y - 1
    }
}

fn neighbors(x: usize, y: usize, board: &[Pixel]) -> u8 {
    [
        (decx(x), decy(y)),
        (x, decy(y)),
        (incx(x), decy(y)),
        (decx(x), y),
        (incx(x), y),
        (decx(x), incy(y)),
        (x, incy(y)),
        (incx(x), incy(y)),
    ]
    .into_iter()
    .map(|(x, y)| board[idx(x, y)].is_live() as u8)
    .sum()
}

#[errata::catch]
fn main() {
    let mut front = vec![cell(false); WIDTH * HEIGHT];
    let mut back = front.clone();

    front
        .par_iter_mut()
        .for_each(|p| *p = cell(thread_rng().gen_bool(0.5)));

    let mut window = Window::new("Game of Life", WIDTH, HEIGHT, WindowOptions::default())
        .fail("failed to create window");

    window.limit_update_rate(Some(std::time::Duration::from_millis(50)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        std::mem::swap(&mut front, &mut back);

        front.par_iter_mut().enumerate().for_each(|(i, p)| {
            let (x, y) = pos(i);
            let live = back[i].is_live();

            let neighbors = neighbors(x, y, &back);
            *p = cell((live && neighbors > 1 && neighbors < 4) || (!live && neighbors == 3));
        });

        window
            .update_with_buffer(bytemuck::cast_slice(&front), WIDTH, HEIGHT)
            .fail("failed to write to window");
    }
}
