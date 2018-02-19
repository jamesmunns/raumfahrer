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

// use dw1000::registers;

pub fn init(
    mut p: ::init::Peripherals,
    // _r: ::init::Resources
// ) -> ::init::LateResources {
) {

}

    // // power on GPIOC
    // p.device.RCC.apb2enr.modify(|_, w| w.iopcen().enabled());

    // // configure PC13 as output
    // p.device.GPIOC.bsrr.write(|w| w.bs13().set());
    // p.device
    //     .GPIOC
    //     .crh
    //     .modify(|_, w| w.mode13().output().cnf13().push());

    // // configure the system timer to generate one interrupt every second
    // p.core.SYST.set_clock_source(SystClkSource::Core);
    // p.core.SYST.set_reload(64_000_000); // TODO - This changed with the `clocks` stuff below
    // p.core.SYST.enable_interrupt();
    // p.core.SYST.enable_counter();

    // // AJM - DEMO SPI
    // // TODO: Figure out what these settings are, instead of copy/paste
    // //         from japaric's zen project
    // let mut rcc = p.device.RCC.constrain();
    // let mut gpioa = p.device.GPIOA.split(&mut rcc.apb2);
    // let mut afio = p.device.AFIO.constrain(&mut rcc.apb2);
    // let mut flash = p.device.FLASH.constrain();
    // let clocks = rcc.cfgr
    //     .sysclk(64.mhz())
    //     .pclk1(32.mhz())
    //     .freeze(&mut flash.acr);

    // let mut delay = Delay::new(p.core.SYST, clocks);

    // // // Trigger for debugging with the logic analyzer
    // let mut gpiob = p.device.GPIOB.split(&mut rcc.apb2);
    // let mut trig = gpiob.pb0.into_push_pull_output(&mut gpiob.crl);

    // // // For now, set trigger on device boot
    // // use ehal::blocking::delay::DelayMs;
    // trig.set_low();
    // // delay.delay_ms(1u8);
    // trig.set_high();


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

    // // USART for logging output until I figure out how to log over semihosting
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

    // // Say hello to test the interface
    // for _ in 0..3 {
    //     let _ = tx.write(0xAC);
    // }

    // dwm.ncs.set_low();

    // ::init::LateResources {
    //     RADIO: dwm,
    //     LOG: tx,
    //     TRIGGER: trig,
    // }
