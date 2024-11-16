#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    delay::Delay,
    prelude::*,
    gpio::Io,
    i2c::I2c
};

#[entry]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());
    let io = Io::new(peripherals.GPIO, peripherals.IO_MUX);

    let mut i2c = I2c::new(
        peripherals.I2C0,
        io.pins.gpio1,
        io.pins.gpio2,
        100.kHz(),
    );

    let delay = Delay::new();
    esp_println::logger::init_logger_from_env();

    let mut data = [0u8; 1];
    if i2c.write_read(0x68, &[0x75], &mut data).is_ok() {
        esp_println::println!("[MPU6050] ---> OKE addr : 0x{:x}", data[0]);
    } else {
        esp_println::println!("Failed to connect to MPU6050");
        return loop {};
    }

    i2c.write(0x68, &[0x6B, 0x00]).ok();
    i2c.write(0x68, &[0x1C, 0x00]).ok(); 

    loop {
        let mut accel_data = [0u8; 6];

        if i2c.write_read(0x68, &[0x3B], &mut accel_data).is_ok() {
            let ax = (accel_data[0] as i16) << 8 | accel_data[1] as i16;
            let ay = (accel_data[2] as i16) << 8 | accel_data[3] as i16;
            let az = (accel_data[4] as i16) << 8 | accel_data[5] as i16;

            esp_println::println!("Accelerometer - X: {}, Y: {}, Z: {}", ax, ay, az);
        } else {
            esp_println::println!("Failed to read accelerometer data");
        }

        delay.delay_millis(50u32);
    }
}
