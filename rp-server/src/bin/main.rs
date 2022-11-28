use rppal as hal;

use hal::uart::{Parity, Uart};
use std::error::Error;

const BAUD_RATE: u32 = 115200;
const DATA_BITS_LENGTH: u8 = 8;
const STOP_BITS_LENGTH: u8 = 1;

fn main() -> Result<(), Box<dyn Error>> {
    let mut uart = Uart::new(BAUD_RATE, Parity::None, DATA_BITS_LENGTH, STOP_BITS_LENGTH)?;

    let buffer = [0u8; 2];

    loop {
        uart.write(&buffer).expect("UART Transmit error");
        println!("Send task is done!");
    }
}
