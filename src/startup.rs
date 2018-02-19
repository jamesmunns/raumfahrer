use cortex_m::peripheral::syst::SystClkSource;

// These let me use the `.constrain()` method
use hal::rcc::RccExt;
use hal::gpio::GpioExt;
use hal::afio::AfioExt;
use hal::flash::FlashExt; // This probably isn't necessary...

// This lets me use the .mhz() method
use hal::time::U32Ext;

use hal::spi::Spi;

use hal::delay::Delay;
use ehal::digital::OutputPin;

use hal::serial::Serial;
use ehal::serial::Write;

use dw1000;

use dw1000::registers;
use ehal::blocking::spi::Transfer;

pub fn init(
    mut p: ::init::Peripherals // _r: ::init::Resources
                               // ) -> ::init::LateResources {
) {
    // Okay, first lets just get the clocks powered on, and do a short PoC
    let mut rcc = p.device.RCC.constrain();

    // TODO(AJM) - Why do I need flash for this?
    let mut flash = p.device.FLASH.constrain();
    let clocks = rcc.cfgr
        .sysclk(64.mhz())
        .pclk1(32.mhz())
        .freeze(&mut flash.acr);

    // AJM - I have to do this now, because we move p.core.SYST when creating a delay obj
    // configure the system timer to generate one interrupt every second
    p.core.SYST.set_clock_source(SystClkSource::Core);
    p.core.SYST.set_reload(64_000_000); // TODO - This changed with the `clocks` stuff below
    p.core.SYST.enable_interrupt();
    p.core.SYST.enable_counter();

    let mut delay = Delay::new(p.core.SYST, clocks);

    // Trigger for debugging with the logic analyzer
    let mut gpiob = p.device.GPIOB.split(&mut rcc.apb2);
    let mut trig_la = gpiob.pb0.into_push_pull_output(&mut gpiob.crl);

    // Trigger for LED
    let mut gpioc = p.device.GPIOC.split(&mut rcc.apb2);
    let mut trig_led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);

    // Serial port
    let mut gpioa = p.device.GPIOA.split(&mut rcc.apb2);
    let mut afio = p.device.AFIO.constrain(&mut rcc.apb2);
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

    // For now, set trigger on device boot
    // fn todo_main()
    {
        use ehal::blocking::delay::DelayMs;

        // This triggers the logic analyzer
        delay.delay_ms(100u8);
        trig_la.set_high();
        delay.delay_ms(1u8);
        trig_la.set_low();

        // Blinky to measure timing
        for _ in 0..10 {
            trig_led.set_low();
            delay.delay_ms(100u8);
            trig_led.set_high();
            delay.delay_ms(100u8);
        }

        for _ in 0..10 {
            block!(tx.write(0xAC)).unwrap();
        }

        block!(tx.flush()).unwrap();

        // Hiccup for timing
        trig_la.set_high();
        delay.delay_ms(1u8);
        trig_la.set_low();
        delay.delay_ms(1u8);

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
            clocks,      // TODO - Is this right?
            &mut rcc.apb2,
        );

        // NOTE: 10ms delay expected here during boot sequence
        let mut dwm = dw1000::new(spi, nss, gpiob.pb1, &mut gpiob.crl, &mut delay).unwrap();


        let mut buf = [0x0u8; 4];

        buf[0] = registers::DEV_ID::BASE | registers::READ_MASK | registers::NO_SUB_INDEX_MASK;

        dwm.ncs.set_low();
        delay.delay_ms(1u8);
        // TODO: Is delay necessary here?
        dwm.spi.transfer(&mut buf[0..1]).unwrap();
        buf[0] = 0u8;
        dwm.spi.transfer(&mut buf).unwrap();
        delay.delay_ms(1u8);
        dwm.ncs.set_high();

        for b in buf.iter() {
            block!(tx.write(*b)).unwrap();
        }
        block!(tx.flush()).unwrap();

        // Mark end of function in logic analyzer
        trig_la.set_high();
    }
}

// // power on GPIOC
// p.device.RCC.apb2enr.modify(|_, w| w.iopcen().enabled());

// // configure PC13 as output
// p.device.GPIOC.bsrr.write(|w| w.bs13().set());
// p.device
//     .GPIOC
//     .crh
//     .modify(|_, w| w.mode13().output().cnf13().push());

// // AJM - DEMO SPI
// // TODO: Figure out what these settings are, instead of copy/paste
// //         from japaric's zen project
// let mut rcc = p.device.RCC.constrain();


// // SPI
// let nss = gpioa.pa4.into_push_pull_output(&mut gpioa.crl);
// let sck = gpioa.pa5.into_alternate_push_pull(&mut gpioa.crl);
// let miso = gpioa.pa6;
// let mosi = gpioa.pa7.into_alternate_push_pull(&mut gpioa.crl);

// let spi = Spi::spi1(
//     p.device.SPI1,
//     (sck, miso, mosi),
//     &mut afio.mapr,
//     dw1000::DEFAULT_SPI_MODE,
//     1_u32.mhz(), // TODO, this could be higher (3-20Mhz), but okay for now
//     clocks, // TODO - Is this right?
//     &mut rcc.apb2,
// );

// // NOTE: 10ms delay expected here during boot sequence
// let mut dwm = dw1000::new(spi, nss, &mut delay).unwrap();



// dwm.ncs.set_low();

// ::init::LateResources {
//     RADIO: dwm,
//     LOG: tx,
//     TRIGGER: trig,
// }
