use cortex_m::peripheral::syst::SystClkSource;
use rtfm::bkpt;

// These let me use the `.constrain()` method
// use hal::rcc::RccExt;
// use hal::gpio::GpioExt;
// use hal::afio::AfioExt;
// use hal::flash::FlashExt; // This probably isn't necessary...

// This lets me use the .mhz() method
// use hal::time::U32Ext;

// use hal::spi::Spi;

// use hal::delay::Delay;

// use hal::serial::Serial;
use ehal::serial::Write;

use dw1000;

// use dw1000::registers;
// use ehal::blocking::spi::Transfer;
use ehal::spi::FullDuplex;

use ehal::digital::{InputPin, OutputPin};

#[allow(dead_code)]
fn omatch<I: InputPin, O: OutputPin>(in_pin: &I, out_pin: &mut O)
{
    if in_pin.is_high() {
        out_pin.set_high()
    } else {
        out_pin.set_low()
    }
}

pub fn init(
    mut p: ::init::Peripherals // _r: ::init::Resources
                               // ) -> ::init::LateResources {
) {

    use hal;
    use hal::gpio::GpioExt;

    let mut p0 = p.device.P0.split();

    if false { // LED/GPIO TEST
        // let mut p0_11 = p0.p0_11.into_push_pull_output();
        // let p0_12 = p0.p0_12.into_floating_input();
        // let mut p0_13 = p0.p0_13.into_push_pull_output();

        // bkpt();

        // for _ in 0..10 {
        //     p0_11.set_low();

        //     omatch(&p0_12, &mut p0_13);

        //     p0_11.set_high();

        //     omatch(&p0_12, &mut p0_13);
        // }
    }

    if false { // Delay Test
        // use hal::clocks::ClocksExt;
        // use ehal::blocking::delay::DelayMs;
        // use hal::delay::Delay;

        // let mut clks = p.device.CLOCK.constrain().freeze();

        // let mut delay = Delay::new(p.core.SYST, clks);

        // let mut led0 = p0.p0_17.into_push_pull_output();

        // for _ in 0..10 {
        //     led0.set_low();
        //     delay.delay_ms(1000u16);
        //     led0.set_high();
        //     delay.delay_ms(1000u16);
        // }

        // let mut x = delay.free();

        // x.set_clock_source(SystClkSource::Core);
        // x.set_reload(8_000_000);
        // x.enable_interrupt();
        // x.enable_counter();
    }

    // if true { // SPIM test
        use hal::spim::Spi1;
        use hal::time::U32Ext;

        use hal::clocks::ClocksExt;
        use ehal::blocking::delay::DelayMs;
        use hal::delay::Delay;

        let mut clks = p.device.CLOCK.constrain().freeze();

        let mut delay = Delay::new(p.core.SYST, clks);

        // For now, use the following hardcoded pins to SPI1:
        // SPI1_MISO:  P0.18
        // SPI1_MOSI:  P0.20
        // SPI1_CLK:   P0.16
        // DW_IRQ:     P0.19
        // DW_CS:      P0.17
        // DW_RST:     P0.24

        let mut nss_pin = p0.p0_17.into_push_pull_output().degrade();

        nss_pin.set_high();

        // bkpt();

        let mut spi = Spi1::new(
            dw1000::DEFAULT_SPI_MODE,
            p0.p0_18.into_floating_input().degrade(), // miso_pin,
            p0.p0_20.into_push_pull_output().degrade(), // mosi_pin,
            p0.p0_16.into_push_pull_output().degrade(), // clk_pin,
            125_000u32.hz(), // freq
        );

        let mut y = 0xAC;

        // bkpt();

        for i in 0..5 {
            nss_pin.set_low();
            let _ = block!(spi.send(i));
            if let Ok(x) = spi.read() {
                y = x;
            }
            nss_pin.set_high();
            delay.delay_ms(10u16);
        }

        // bkpt();
    // }


    // ::init::LateResources {
    //     LED: led0,
    // }

    // ===== OLD STM32 stuff





    // // Okay, first lets just get the clocks powered on, and do a short PoC
    // let mut rcc = p.device.RCC.constrain();

    // // TODO(AJM) - Why do I need flash for this?
    // let mut flash = p.device.FLASH.constrain();
    // let clocks = rcc.cfgr
    //     .sysclk(64.mhz())
    //     .pclk1(32.mhz())
    //     .freeze(&mut flash.acr);

    // // AJM - I have to do this now, because we move p.core.SYST when creating a delay obj
    // // configure the system timer to generate one interrupt every second
    // p.core.SYST.set_clock_source(SystClkSource::Core);
    // p.core.SYST.set_reload(64_000_000); // TODO - This changed with the `clocks` stuff below
    // p.core.SYST.enable_interrupt();
    // p.core.SYST.enable_counter();

    // let mut delay = Delay::new(p.core.SYST, clocks);

    // // Trigger for debugging with the logic analyzer
    // let mut gpiob = p.device.GPIOB.split(&mut rcc.apb2);
    // let mut trig_la = gpiob.pb0.into_push_pull_output(&mut gpiob.crl);

    // // Trigger for LED
    // let mut gpioc = p.device.GPIOC.split(&mut rcc.apb2);
    // let mut trig_led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);

    // // Serial port
    // let mut gpioa = p.device.GPIOA.split(&mut rcc.apb2);
    // let mut afio = p.device.AFIO.constrain(&mut rcc.apb2);
    // let pa9 = gpioa.pa9.into_alternate_push_pull(&mut gpioa.crh);
    // let pa10 = gpioa.pa10;

    // let serial = Serial::usart1(
    //     p.device.USART1,
    //     (pa9, pa10),
    //     &mut afio.mapr,
    //     115_200.bps(),
    //     clocks,
    //     &mut rcc.apb2,
    // );

    // let (mut tx, _rx) = serial.split();

    // // For now, set trigger on device boot
    // // fn todo_main()
    // {
    //     use ehal::blocking::delay::DelayMs;

    //     // This triggers the logic analyzer
    //     delay.delay_ms(100u8);
    //     trig_la.set_high();
    //     delay.delay_ms(1u8);
    //     trig_la.set_low();

    //     // Blinky to measure timing
    //     for _ in 0..2 {
    //         trig_led.set_low();
    //         delay.delay_ms(100u8);
    //         trig_led.set_high();
    //         delay.delay_ms(100u8);
    //     }

    //     for _ in 0..2 {
    //         block!(tx.write(0xAC)).unwrap();
    //     }

    //     block!(tx.flush()).unwrap();

    //     // Hiccup for timing
    //     trig_la.set_high();
    //     delay.delay_ms(1u8);
    //     trig_la.set_low();
    //     delay.delay_ms(1u8);

    //     // SPI
    //     let nss = gpioa.pa4.into_push_pull_output(&mut gpioa.crl);
    //     let sck = gpioa.pa5.into_alternate_push_pull(&mut gpioa.crl);
    //     let miso = gpioa.pa6;
    //     let mosi = gpioa.pa7.into_alternate_push_pull(&mut gpioa.crl);

    //     let spi = Spi::spi1(
    //         p.device.SPI1,
    //         (sck, miso, mosi),
    //         &mut afio.mapr,
    //         dw1000::DEFAULT_SPI_MODE,
    //         1_u32.mhz(), // TODO, this could be higher (3-20Mhz), but okay for now
    //         clocks,      // TODO - Is this right?
    //         &mut rcc.apb2,
    //     );

    //     // NOTE: 10ms delay expected here during boot sequence
    //     let mut dwm = dw1000::new(spi, nss, gpiob.pb1, &mut gpiob.crl, &mut delay).unwrap();


    //     for _ in 0..5 {
    //         {
    //             let mut buf = [0x0u8; 5];
    //             buf[0] = 0x8D; // SYS_CTRL
    //             buf[1] = 0x00;
    //             buf[2] = 0x00;
    //             buf[3] = 0x00;
    //             buf[4] = 0x40; // TRX OFF

    //             dwm.ncs.set_low();
    //             delay.delay_ms(1u8);
    //             // TODO: Is delay necessary here?
    //             dwm.spi.transfer(&mut buf).unwrap();
    //             delay.delay_ms(1u8);
    //             dwm.ncs.set_high();
    //         }

    //         delay.delay_ms(1u8);

    //         {
    //             let mut buf = [0x0u8; 4];

    //             buf[0] = registers::DEV_ID::BASE | registers::READ_MASK | registers::NO_SUB_INDEX_MASK;

    //             dwm.ncs.set_low();
    //             delay.delay_ms(1u8);
    //             // TODO: Is delay necessary here?
    //             dwm.spi.transfer(&mut buf[0..1]).unwrap();
    //             buf[0] = 0u8;
    //             dwm.spi.transfer(&mut buf).unwrap();
    //             delay.delay_ms(1u8);
    //             dwm.ncs.set_high();

    //             for b in buf.iter() {
    //                 block!(tx.write(*b)).unwrap();
    //             }
    //             block!(tx.flush()).unwrap();
    //         }

    //         delay.delay_ms(1u8);
    //     }

    //     // Mark end of function in logic analyzer
    //     trig_la.set_high();
    // }
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
