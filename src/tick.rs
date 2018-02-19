use rtfm::Threshold;
use stm32f103xx::GPIOC;

// Do stuff with SPI
use ehal::blocking::spi::Transfer;

use ehal::serial::Write;

use dw1000::registers;

// This is the task handler of the SYS_TICK exception
//
// `_t` is the preemption threshold token. We won't use it in this program.
//
// `r` is the set of resources this task has access to. `SYS_TICK::Resources`
// has one field per resource declared in `app!`.
#[allow(unsafe_code)]
pub fn sys_tick(_t: &mut Threshold, mut r: ::SYS_TICK::Resources) {
    {
        let mut hstdout = hio::hstdout().unwrap();
        use core::fmt::Write;
        use sh::hio;
        writeln!(hstdout, "Hello, world!").unwrap();
    }

    // toggle state
    *r.ON = !*r.ON;

    if *r.ON {
        // set the pin PC13 high
        // NOTE(unsafe) atomic write to a stateless register
        unsafe {
            (*GPIOC::ptr()).bsrr.write(|w| w.bs13().set());
        }
    } else {
        // set the pin PC13 low
        // NOTE(unsafe) atomic write to a stateless register
        unsafe {
            (*GPIOC::ptr()).bsrr.write(|w| w.br13().reset());
        }
    }

    let mut buf = [0x0u8; 4];

    buf[0] = registers::DEV_ID::BASE | registers::READ_MASK | registers::NO_SUB_INDEX_MASK;

    // r.RADIO.ncs.set_low();
    // TODO: Is delay necessary here?
    let _ = r.RADIO.spi.transfer(&mut buf[0..1]);
    buf[0] = 0u8;
    let _ = r.RADIO.spi.transfer(&mut buf);

    let _ = r.LOG.write(buf[3]); // TODO use blocking trait?

    // r.RADIO.ncs.set_high();

    // TODO assert decawave id in buf
}
