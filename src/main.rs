#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use rtt_target::{rtt_init_print, rprintln};
use panic_rtt_target as _;
use microbit::{
    board::Board,
    display::blocking::Display,
    hal::{prelude::*, Timer},
};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let mut board = Board::take().unwrap();

    let mut timer = Timer::new(board.TIMER0);

    let mut display = Display::new(board.display_pins);

    let mut light_mtx = [
        [0,0,0,0,0],
        [0,0,0,0,0],
        [0,0,0,0,0],
        [0,0,0,0,0],
        [0,0,0,0,0],
    ];
    let mut index = (0, 0);

    const PIXELS: [(usize, usize); 16] = [
        (0,0), (0, 1), (0, 2), (0, 3), (0, 4),
        (1, 4), (2, 4), (3, 4), (4, 4),
        (4, 3), (4, 2), (4, 1), (4, 0),
        (3, 0), (2, 0), (1,0)
    ];

    let mut last_led = (0,0);

    loop {
        for current_led in PIXELS.iter() {
            light_mtx[last_led.0][last_led.1] = 0;
            light_mtx[current_led.0][current_led.1] = 1;
            display.show(&mut timer, light_mtx, 30);
            last_led = *current_led;
        }
    }
}
