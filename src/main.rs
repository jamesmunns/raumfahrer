//! An application with one task
// #![deny(unsafe_code)]
// #![deny(warnings)]
#![feature(proc_macro)]
#![no_std]

#![allow(unused_imports)]

extern crate cortex_m_semihosting as sh;

extern crate cortex_m;
extern crate cortex_m_rtfm as rtfm;
extern crate embedded_hal as ehal;

extern crate nrf52_hal as hal;

use hal::nrf52;

use rtfm::{app, Threshold};
// use stm32f103xx::{SPI1, USART1};

// use hal::spi::Spi;

// Some imports to make type signatures shorter
// use hal::gpio::gpioa::{PA4, PA5, PA6, PA7};
// use hal::gpio::gpiob::PB0;
// use hal::gpio::{Alternate, Floating, Input, Output, PushPull};

// use hal::serial::Tx;

#[macro_use]
extern crate nb;

mod dw1000;
mod tick;
mod startup;

// type DWM1000<'a> = dw1000::Dw1000<
//     'a,
//     Spi<
//         SPI1,
//         (
//             PA5<Alternate<PushPull>>,
//             PA6<Input<Floating>>,
//             PA7<Alternate<PushPull>>,
//         ),
//     >,
//     PA4<Output<PushPull>>,
// >;

app! {
    device: nrf52,

    // Here data resources are declared
    //
    // Data resources are static variables that are safe to share across tasks
    // resources: {
    // //     // Declaration of resources looks exactly like declaration of static
    // //     // variables
    // //     static ON: bool = false;
    // //     static RADIO: DWM1000;
    // //     static LOG: Tx<USART1>;
    // //     static TRIGGER: PB0<Output<PushPull>>;
    //     static LED: hal::gpio::p0::P0_17<hal::gpio::Output<hal::gpio::PushPull>>;
    // },

    init: {
        path: startup::init,
    },

    // Here tasks are declared
    //
    // Each task corresponds to an interrupt or an exception. Every time the
    // interrupt or exception becomes *pending* the corresponding task handler
    // will be executed.
    tasks: {
        // Here we declare that we'll use the SYS_TICK exception as a task
        SYS_TICK: {
            // Path to the task handler
            path: tick::sys_tick,

            // // These are the resources this task has access to.
            // //
            // // The resources listed here must also appear in `app.resources`
            // resources: [
            //     LED
            // //     ON,
            // //     RADIO,
            // //     LOG,
            // //     TRIGGER
            // ],
        },
    }
}

fn idle() -> ! {
    loop {
        rtfm::wfi();
    }
}
