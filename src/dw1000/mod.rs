// NOTES:
//
// "The maximum SPI frequency is 20 MHz when the CLKPLL is locked,
// otherwise the maximum SPI frequency is 3 MHz."
//   The CLKPLL is locked once the device is in IDLE state
//   The wakeup process goes OFF -> WAKEUP -> INIT -> IDLE
//     OFF -> WAKEUP happens when the device is powered
//     WAKEUP -> INIT takes up to 4ms (User Manual Sec 2.3.2)
//     INIT -> IDLE takes 5us, and no SPI comms should happen (User Manual Sec 2.3.2)
//
//   I will probably start with a 10ms delay after power on to allow the radio to init properly
//   In the future, we can probably detect this more intelligently, or just defer initialization

// NOTE: Initially borrowing heavily from https://github.com/japaric/mpu9250/blob/master/src/lib.rs
// for how to write an embedded-hal SPI device driver

use ehal::blocking::delay::DelayMs;
use ehal::blocking::spi;
use ehal::digital::OutputPin;
use ehal::spi::{Mode, Phase, Polarity};

pub mod registers;

pub struct Dw1000<SPI, NCS> {
    pub spi: SPI, // TODO - remove pub once theres a good accessor fn
    pub ncs: NCS, // TODO - remove pub once theres a good accessor fn
}

pub fn new<SPI, NCS, D, E>(spi: SPI, ncs: NCS, delay: &mut D) -> Result<Dw1000<SPI, NCS>, E>
where
    D: DelayMs<u8>,
    NCS: OutputPin,
    SPI: spi::Write<u8, Error = E> + spi::Transfer<u8, Error = E>,
{
    // Give the radio time to power on and lock PLLs
    // (User Manual Sec 2.3.2)
    // delay.delay_ms(10); // TODO AJM TROUBLESHOOTING

    // TODO - Make sure registers::DEV_ID::BASE reads 0xDECA0130

    let dw1000 = Dw1000 { spi, ncs };

    // TODO - Any init necessary?

    Ok(dw1000)
}

// With dw1000 GPIO5 and GPIO6 floating or pulled low, the default SPI mode is:
// "Data is sampled on the rising (first) edge of the clock and launched on the
// falling (second) edge."
//
// SPI MOSI has an internal pulldown resistor, so IDLE should probably
// be low
pub const DEFAULT_SPI_MODE: Mode = Mode {
    polarity: Polarity::IdleLow,
    phase: Phase::CaptureOnFirstTransition,
};
