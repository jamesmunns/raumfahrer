//! An application with one task
#![deny(unsafe_code)]
// #![deny(warnings)]
#![feature(proc_macro)]
#![no_std]

extern crate cortex_m;
extern crate cortex_m_rtfm as rtfm;
extern crate embedded_hal as ehal;
extern crate stm32f103xx_hal as hal;

use hal::stm32f103xx;

use cortex_m::peripheral::syst::SystClkSource;
use rtfm::{app, Threshold};
use stm32f103xx::{SPI1, GPIOC, USART1};

// These let me use the `.constrain()` method
use hal::rcc::RccExt;
use hal::gpio::GpioExt;
use hal::afio::AfioExt;
use hal::flash::FlashExt; // This probably isn't necessary...

// This lets me use the .mhz() method
use hal::time::U32Ext;

use hal::spi::Spi;

// Some imports to make type signatures shorter
use hal::gpio::gpioa::{PA4, PA5, PA6, PA7};
use hal::gpio::gpiob::{PB0};
use hal::gpio::{Alternate, Floating, Input, Output, PushPull};

// Do stuff with SPI
use ehal::blocking::spi::Transfer;

use hal::delay::Delay;
use ehal::digital::OutputPin;

use hal::serial::{Serial, Tx};
use ehal::serial::Write;

mod dw1000;

use dw1000::registers;

type DWM1000 = dw1000::Dw1000<
    Spi<
        SPI1,
        (
            PA5<Alternate<PushPull>>,
            PA6<Input<Floating>>,
            PA7<Alternate<PushPull>>,
        ),
    >,
    PA4<Output<PushPull>>,
>;

app! {
    device: stm32f103xx,

    // Here data resources are declared
    //
    // Data resources are static variables that are safe to share across tasks
    resources: {
        // Declaration of resources looks exactly like declaration of static
        // variables
        static ON: bool = false;
        static RADIO: DWM1000;
        static LOG: Tx<USART1>;
        static TRIGGER: PB0<Output<PushPull>>;
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
            resources: [
                ON,
                RADIO,
                LOG,
                TRIGGER
            ],
        },
    }
}

fn init(mut p: init::Peripherals, _r: init::Resources) -> init::LateResources {
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

    let mut delay = Delay::new(p.core.SYST, clocks);

    // // Trigger for debugging with the logic analyzer
    let mut gpiob = p.device.GPIOB.split(&mut rcc.apb2);
    let mut trig = gpiob.pb0.into_push_pull_output(&mut gpiob.crl);

    // // For now, set trigger on device boot
    // use ehal::blocking::delay::DelayMs;
    trig.set_low();
    // delay.delay_ms(1u8);
    trig.set_high();


    // SPI
    let nss = gpioa.pa4.into_push_pull_output(&mut gpioa.crl);
    let sck = gpioa.pa5.into_alternate_push_pull(&mut gpioa.crl);
    let miso = gpioa.pa6;
    let mosi = gpioa.pa7.into_alternate_push_pull(&mut gpioa.crl);

    let spi = Spi::spi1(
        p.device.SPI1,
        (sck, miso, mosi),
        &mut afio.mapr,
        dw1000::DEFAULT_SPI_MODE,
        1_u32.mhz(), // TODO, this could be higher (3-20Mhz), but okay for now
        clocks, // TODO - Is this right?
        &mut rcc.apb2,
    );

    // NOTE: 10ms delay expected here during boot sequence
    let mut dwm = dw1000::new(spi, nss, &mut delay).unwrap();

    // USART for logging output until I figure out how to log over semihosting
    let pa9 = gpioa.pa9.into_alternate_push_pull(&mut gpioa.crh);
    let pa10 = gpioa.pa10;

    let serial = Serial::usart1(
        p.device.USART1,
        (pa9, pa10),
        &mut afio.mapr,
        115_200.bps(),
        clocks,
        &mut rcc.apb2,
    );

    let (mut tx, _rx) = serial.split();

    // Say hello to test the interface
    for _ in 0..3 {
        let _ = tx.write(0xAC);
    }

    dwm.ncs.set_low();

    init::LateResources {
        RADIO: dwm,
        LOG: tx,
        TRIGGER: trig,
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

    let mut buf = [0x0u8; 4];

    buf[0] = registers::DEV_ID::BASE | registers::READ_MASK | registers::NO_SUB_INDEX_MASK;

    // r.RADIO.ncs.set_low();
    // TODO: Is delay necessary here?
    let _ = r.RADIO.spi.transfer(&mut buf[0..1]);
    buf[0] = 0u8;
    let _ = r.RADIO.spi.transfer(&mut buf);

    r.LOG.write(buf[3]); // TODO use blocking trait?

    // r.RADIO.ncs.set_high();

    // TODO assert decawave id in buf
}
