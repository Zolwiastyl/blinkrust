//! # GPIO 'Blinky' Example
//!
//! This application demonstrates how to control a GPIO pin on the RP2040.
//!
//! It may need to be adapted to your particular board layout and/or pin assignment.
//!
//! See the `Cargo.toml` file for Copyright and license details.

#![no_std]
#![no_main]

use embedded_hal::adc::OneShot;
// Ensure we halt the program on panic (if we don't mention this crate it won't
// be linked)
use panic_halt as _;

// Alias for our HAL crate
use rp2040_hal as hal;

// A shorter alias for the Peripheral Access Crate, which provides low-level
// register access
use hal::pac;

// Some traits we need
use embedded_hal::digital::v2::{InputPin, OutputPin};
use rp2040_hal::adc::Adc;
use rp2040_hal::clocks::Clock;

/// The linker will place this boot block at the start of our program image. We
/// need this to help the ROM bootloader get our code up and running.
/// Note: This boot block is not necessary when using a rp-hal based BSP
/// as the BSPs already perform this step.
#[link_section = ".boot2"]
#[used]
pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER_GENERIC_03H;

/// External high-speed crystal on the Raspberry Pi Pico board is 12 MHz. Adjust
/// if your board has a different frequency
const XTAL_FREQ_HZ: u32 = 12_000_000u32;

/// Entry point to our bare-metal application.
///
/// The `#[rp2040_hal::entry]` macro ensures the Cortex-M start-up code calls this function
/// as soon as all global variables and the spinlock are initialised.
///
/// The function configures the RP2040 peripherals, then toggles a GPIO pin in
/// an infinite loop. If there is an LED connected to that pin, it will blink.
#[rp2040_hal::entry]
fn main() -> ! {
    // Grab our singleton objects
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();

    // Set up the watchdog driver - needed by the clock setup code
    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);

    // Configure the clocks
    let clocks = hal::clocks::init_clocks_and_plls(
        XTAL_FREQ_HZ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    // The single-cycle I/O block controls our GPIO pins
    let sio = hal::Sio::new(pac.SIO);

    // Set the pins to their default state
    let pins = hal::gpio::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    // Configure GPIO25 as an output
    let mut led_pin = pins.gpio25.into_push_pull_output();
    let mut external_led_pin = pins.gpio28.into_push_pull_output();
    let mut buzzer = pins.gpio18.into_push_pull_output();
    let mut button1 = pins.gpio20.into_pull_up_input();

    // let photoresistor = pins.gpio13.into_floating_input();
    let mut adc = Adc::new(pac.ADC, &mut pac.RESETS);
    let mut adc_pin_0 = pins.gpio26.into_floating_input();
    loop {
        let v0: u128 = adc.read(&mut adc_pin_0).unwrap();
        if v0 > 100 {
            led_pin.set_high().unwrap();
            delay.delay_ms(1000);
            led_pin.set_low().unwrap();
            delay.delay_ms(1000);
        } else if v0 == 0 {
            buzzer.set_high().unwrap();
            delay.delay_ms(600);
            buzzer.set_low().unwrap();
        } else {
            external_led_pin.set_high().unwrap();
            delay.delay_ms(1000);
            external_led_pin.set_low().unwrap();
            delay.delay_ms(1000);
        }
        // turn the LED on
        if button1.is_low().unwrap() {
            delay.delay_ms(500);

            led_pin.set_high().unwrap();
            // let pinStrength = external_led_pin.get_drive_strength();
            // TODO: Replace with proper 1s delays once we have clocks working
            external_led_pin.set_low().unwrap();
            delay.delay_ms(100);

            // turn the LED off
            external_led_pin.set_high().unwrap();
            led_pin.set_low().unwrap();
            delay.delay_ms(100);
            buzzer.set_high().unwrap();
            delay.delay_ms(600);
            buzzer.set_low().unwrap();
            delay.delay_ms(2000);
            buzzer.set_high().unwrap();
            delay.delay_ms(600);
            buzzer.set_low().unwrap();
            delay.delay_ms(2000);
        }
    }
}

// End of file
