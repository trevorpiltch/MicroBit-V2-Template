#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use panic_halt as _;
use microbit::{
    board::Board,
    display::blocking::Display,
    hal::Timer
};

#[entry]
fn main() -> ! {
    let board = Board::take().unwrap();

    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);

    let mut light_mtx = [
        [0,0,0,0,0],
        [0,0,0,0,0],
        [0,0,0,0,0],
        [0,0,0,0,0],
        [0,0,0,0,0],
    ];

    loop {
        light_mtx[0][0] = (light_mtx[0][0] + 1) % 2;
        display.show(&mut timer, light_mtx, 1000);
    }
}
