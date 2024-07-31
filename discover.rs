#![no_std]
#![no_main]

// use core::net::{Ipv4Addr, SocketAddr};
// use core::time::Duration;

// use cyw43::Runner;
use cyw43_pio::PioSpi;
use embassy_executor::Spawner;
use embassy_net::tcp::TcpSocket;
use embassy_net::{Config, Ipv4Address, Ipv4Cidr, Stack, StackResources, StaticConfigV4};
use embassy_rp::bind_interrupts;
use embassy_rp::gpio::{Level, Output};
use embassy_rp::peripherals::{DMA_CH0, PIO0, USB};
use embassy_rp::pio::{InterruptHandler, Pio};
use embassy_rp::usb::{Driver, InterruptHandler as USBInterruptHandler};
use embassy_time::Timer;
// use futures::future::ok;
use log::info;
use static_cell::StaticCell;

const _WIFI_NETWORK: &str = "Wyliodrin";
const _WIFI_PASSWORD: &str = "g3E2PjWy";

bind_interrupts!(struct Irqs {
	PIO0_IRQ_0 => InterruptHandler<PIO0>;
	USBCTRL_IRQ => USBInterruptHandler<USB>;
});

// Logging task on the USB serial output.
#[embassy_executor::task]
async fn logger_task(driver: Driver<'static, USB>) {
	embassy_usb_logger::run!(1024, log::LevelFilter::Info, driver);
}

// WiFi task to communicate with the CYW43 chip.
#[embassy_executor::task]
async fn wifi_task(
	runner: cyw43::Runner<'static, Output<'static>, PioSpi<'static, PIO0, 0, DMA_CH0>>,
) -> ! {
	runner.run().await
}

// Net task to process network events.
#[embassy_executor::task]
async fn net_task(stack: &'static Stack<cyw43::NetDriver<'static>>) -> ! {
	stack.run().await
}

// Basic panic handler.
#[panic_handler]
fn panic_handler(_pc: &core::panic::PanicInfo) -> ! {
	loop {}
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
	let p = embassy_rp::init(Default::default());

	// ************** WiFi Chip initialization - DO NOT MODIFY! *****************

	// Firmware.
	let fw = include_bytes!("../cyw43-firmware/43439A0.bin");
	let clm = include_bytes!("../cyw43-firmware/43439A0_clm.bin");

	// The CYW43 chip is connected through SPI.
	let pwr = Output::new(p.PIN_23, Level::Low);
	let cs = Output::new(p.PIN_25, Level::High);
	let mut pio = Pio::new(p.PIO0, Irqs);
	let spi = PioSpi::new(
		&mut pio.common,
		pio.sm0,
		pio.irq0,
		cs,
		p.PIN_24,
		p.PIN_29,
		p.DMA_CH0,
	);
	static STATE: StaticCell<cyw43::State> = StaticCell::new();
	let state = STATE.init(cyw43::State::new());
	let (_net_device, mut control, runner) = cyw43::new(state, pwr, spi, fw).await;
	// ************************************************************************

	// 1. TODO: Spawn `logger_task`.
	let driver1 = Driver::new(p.USB, Irqs);
	_spawner.spawn(logger_task(driver1)).unwrap();

	// 2. TODO: Spawn `wifi_task`.
	
	_spawner.spawn(wifi_task(runner)).unwrap();

	// Initialize the control peripheral on the CYW43 chip.
	control.init(clm).await;
	control
		.set_power_management(cyw43::PowerManagementMode::Performance)
		.await;

	// 3. TODO: Create a `Config` using a static address.
	let config = Config::ipv4_static(StaticConfigV4 {
		address: Ipv4Cidr::new(Ipv4Address::new(192, 168, 1, 57), 24),
		gateway: Some(Ipv4Address::new(192, 168, 1, 1)),
		dns_servers: heapless::Vec::new()
	});

	// Generate random seed
	let _seed: u64 = 0x0123_4567_89ba_cdef; // chosen by fair dice roll. guarenteed to be random.

	// 4. TODO: Init network stack
	static _STACK: StaticCell<Stack<cyw43::NetDriver<'static>>> = StaticCell::new();
	static _RESOURCES: StaticCell<StackResources<2>> = StaticCell::new();

	let stack = &*_STACK.init(Stack::new(
	    _net_device,
	    config,
	    _RESOURCES.init(StackResources::<2>::new()),
	    _seed,
	));

	// 5. TODO: Spawn `net_task`.
	_spawner.spawn(net_task(stack)).unwrap();

	// 6. TODO: Try to connect to the AP network.
	// **HINT**: Don't give up if it fails!
	loop {
		if control.join_wpa2(_WIFI_NETWORK, _WIFI_PASSWORD).await.is_ok() {
			break;
		}
	}
	info!("connected to !!!");


	// 7. TODO: Create a socket and connect to our server.
	// Send to the server the code on the box you were given,
	// and print the response receivedto the USB
	// serial output using `log::info!`.
	

	let endpoint = (Ipv4Address::new(192, 168, 1, 199), 3000);
	let mut rx_buffer = [0_u8;1244];
	let mut tx_buffer = [0;1244];
	let mut tcp_s = TcpSocket::new(stack, &mut rx_buffer, &mut tx_buffer);
	
	// tcp_s.set_timeout(Some(Duration::from_secs(10)));
	
	loop {
		if let Ok(_) = tcp_s.connect(endpoint).await {
			info!("S-a logat!");
			break;
		}
	}	
	

	loop {
		if let Ok(_) = tcp_s.write(b"11OP63D3").await {
			info!("S-a scris!");
			break;
		}
	}

	let mut buffer = [0_u8;1234];
	loop {
		if let Ok(_) = tcp_s.read(&mut buffer).await {
			info!("S-a primit! {}", core::str::from_utf8(&buffer).unwrap());
			break;
		}
	}

	loop {
		Timer::after_secs(1).await;
	}
}
