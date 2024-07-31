//! This task's purpose is to make two LEDs blink.
//!
//! One LED should toggle when the button located on the Pico Base Explorer
//! is pressed, and the other one will toggle every second.

#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

// use core::slice::sort::TimSortRun;
// use cortex_m::prelude::_embedded_hal_digital_ToggleableOutputPin;
use embassy_executor::Spawner;
use embassy_futures::yield_now;
use embassy_rp::bind_interrupts;
use embassy_rp::gpio::{Input, Level, Output, Pull};
use embassy_rp::peripherals::USB;
use embassy_rp::usb::{Driver, InterruptHandler as USBInterruptHandler};
use embassy_time::Timer;
use log::info;

bind_interrupts!(struct Irqs {
	USBCTRL_IRQ => USBInterruptHandler<USB>;
});

// Basic panic handler.
#[panic_handler]
fn panic_handler(_pc: &core::panic::PanicInfo) -> ! {
	loop {}
}

#[embassy_executor::task]
async fn logger_task(driver: Driver<'static, USB>) {
	embassy_usb_logger::run!(1024, log::LevelFilter::Info, driver);
}

#[embassy_executor::task]
async fn blink_on_timer(mut _led: Output<'static>) {
	loop {
		// 4. Sets a timer that will wait for 1 second, then toggle the LED
		Timer::after_secs(1).await;
		_led.toggle();
	}
}

#[embassy_executor::task]
async fn blink_on_button_press(mut _led: Output<'static>, mut _btn: Input<'static>) {
	loop {
		// 5. Waits for the button to be pressed, then toggle the LED
		_btn.wait_for_falling_edge().await;
		_led.toggle();
	}
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
	// Initializes the peripherals
	let p = embassy_rp::init(Default::default());

	// 1. Creates two output channels for the Red and Green LEDs
	let r = Output::new(p.PIN_0, Level::High);
	let g = Output::new(p.PIN_1, Level::High);

	// 2. Creates an input channel for Button A
	let a = Input::new(p.PIN_12, Pull::Up);
	

	// Initializes the USB driver and spawns the logger task
	let driver = Driver::new(p.USB, Irqs);
	spawner.spawn(logger_task(driver)).unwrap();

	// 3. Similarly, spawns another two tasks for `blink_on_timer` and `blink_on_press`
	spawner.spawn(blink_on_timer(r)).unwrap();
	spawner.spawn(blink_on_button_press(g, a)).unwrap();

	info!("Hello from embassy!");

	loop {
		yield_now().await;
	}
}
