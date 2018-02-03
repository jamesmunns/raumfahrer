//! An application with one task
#![deny(unsafe_code)]
// #![deny(warnings)]
#![feature(proc_macro)]
#![no_std]

extern crate cortex_m;
extern crate cortex_m_rtfm as rtfm;
extern crate stm32f103xx;

use cortex_m::peripheral::syst::SystClkSource;
use rtfm::{app, Threshold};
use stm32f103xx::{GPIOC, SPI1};

pub struct FakeSPI {
    foo: u32,
    bar: bool,
}

app! {
    device: stm32f103xx,

    // Here data resources are declared
    //
    // Data resources are static variables that are safe to share across tasks
    resources: {
        // Declaration of resources looks exactly like declaration of static
        // variables
        static ON: bool = false;

        static SPI: FakeSPI;
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
            path: sys_tick,

            // These are the resources this task has access to.
            //
            // The resources listed here must also appear in `app.resources`
            resources: [ON, SPI],
        },
    }
}

fn init(mut p: init::Peripherals, r: init::Resources) -> init::LateResources {
    // `init` can modify all the `resources` declared in `app!`
    r.ON;

    // power on GPIOC
    p.device.RCC.apb2enr.modify(|_, w| w.iopcen().enabled());

    // configure PC13 as output
    p.device.GPIOC.bsrr.write(|w| w.bs13().set());
    p.device
        .GPIOC
        .crh
        .modify(|_, w| w.mode13().output().cnf13().push());

    // configure the system timer to generate one interrupt every second
    p.core.SYST.set_clock_source(SystClkSource::Core);
    p.core.SYST.set_reload(4_000_000); // 0.5s
    p.core.SYST.enable_interrupt();
    p.core.SYST.enable_counter();

    init::LateResources {
        SPI: FakeSPI{ foo: 42u32, bar: true },
    }
}

fn idle() -> ! {
    loop {
        rtfm::wfi();
    }
}

// This is the task handler of the SYS_TICK exception
//
// `_t` is the preemption threshold token. We won't use it in this program.
//
// `r` is the set of resources this task has access to. `SYS_TICK::Resources`
// has one field per resource declared in `app!`.
#[allow(unsafe_code)]
fn sys_tick(_t: &mut Threshold, mut r: SYS_TICK::Resources) {
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
}
