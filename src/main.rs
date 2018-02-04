//! An application with one task
#![deny(unsafe_code)]
// #![deny(warnings)]
#![feature(proc_macro)]
#![no_std]

extern crate cortex_m;
extern crate cortex_m_rtfm as rtfm;
extern crate stm32f103xx_hal as hal;
extern crate embedded_hal as ehal;

use hal::stm32f103xx;

use cortex_m::peripheral::syst::SystClkSource;
use rtfm::{app, Threshold};
use stm32f103xx::{GPIOC, SPI1};

// These let me use the `.constrain()` method
use hal::rcc::RccExt;
use hal::gpio::GpioExt;
use hal::afio::AfioExt;
use hal::flash::FlashExt; // This probably isn't necessary...

// This lets me use the .mhz() method
use hal::time::U32Ext;

use hal::spi::Spi;

// TODO, these probable should live in the driver
use ehal::spi::{Mode, Polarity, Phase};

// Some imports to make type signatures shorter
use hal::gpio::gpioa::{PA5, PA6, PA7};
use hal::gpio::{Alternate, Input, Floating, PushPull};

// Do stuff with SPI
use ehal::spi::FullDuplex;

mod dw1000;

app! {
    device: stm32f103xx,

    // Here data resources are declared
    //
    // Data resources are static variables that are safe to share across tasks
    resources: {
        // Declaration of resources looks exactly like declaration of static
        // variables
        static ON: bool = false;

        static SPI: Spi<SPI1, (
            PA5<Alternate<PushPull>>,
            PA6<Input<Floating>>,
            PA7<Alternate<PushPull>>
        )>;
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
    p.core.SYST.set_reload(64_000_000); // TODO - This changed with the `clocks` stuff below
    p.core.SYST.enable_interrupt();
    p.core.SYST.enable_counter();


    // AJM - DEMO SPI
    // TODO: Figure out what these settings are, instead of copy/paste
    //         from japaric's zen project
    let mut rcc = p.device.RCC.constrain();
    let mut gpioa = p.device.GPIOA.split(&mut rcc.apb2);
    let mut afio = p.device.AFIO.constrain(&mut rcc.apb2);
    let mut flash = p.device.FLASH.constrain();
    let clocks = rcc.cfgr
        .sysclk(64.mhz())
        .pclk1(32.mhz())
        .freeze(&mut flash.acr);

    // SPI
    let nss = gpioa.pa4.into_push_pull_output(&mut gpioa.crl);
    let sck = gpioa.pa5.into_alternate_push_pull(&mut gpioa.crl);
    let miso = gpioa.pa6;
    let mosi = gpioa.pa7.into_alternate_push_pull(&mut gpioa.crl);

    let spi = Spi::spi1(
        p.device.SPI1,
        (sck, miso, mosi),
        &mut afio.mapr,
        Mode {                                      // TODO - read datasheet for SPI settings
            polarity: Polarity::IdleLow,            // TODO - read datasheet for SPI settings
            phase: Phase::CaptureOnFirstTransition, // TODO - read datasheet for SPI settings
        },                                          // TODO - read datasheet for SPI settings
        1_u32.mhz(),                                // TODO - read datasheet for SPI settings
        clocks,                                     // TODO - Is this right?
        &mut rcc.apb2,
    );

    init::LateResources {
        SPI: spi,
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

    // TODO: This is only PoC code
    r.SPI.send(0x42);
    let _ = r.SPI.read();
}
