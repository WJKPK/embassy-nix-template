#![no_std]
#![no_main]

use embassy_time::{Duration, Timer};
{% if chip contains "stm32" -%}
use embassy_executor::{Spawner, main};
use {defmt_rtt as _, panic_probe as _};
{% endif -%}
{% if chip contains "esp32" -%}
use embassy_executor::Spawner;
use esp_backtrace as _;
use esp_hal::{
    clock::ClockControl,
    embassy::{self},
    peripherals::Peripherals,
    prelude::*,
    timer::TimerGroup,
};
{% endif -%}

#[main]
async fn main(_spawner: Spawner) {
    {% if chip contains "stm32" -%}
    let _p = embassy_stm32::init(Default::default());
    {% endif -%}
    {% if chip contains "esp32" -%}
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    let timg0 = TimerGroup::new_async(peripherals.TIMG0, &clocks);
    embassy::init(&clocks, timg0);
    {% endif -%}
    loop {
        defmt::info!("Blink");
        Timer::after(Duration::from_millis(100)).await;
    }
}
