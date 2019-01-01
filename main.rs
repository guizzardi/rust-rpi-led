extern crate rppal;

use std::thread;
use std::time::Duration;
use rppal::gpio::{Gpio, Mode, Level};
use rppal::system::DeviceInfo;

// Viene usata la numerazione BCM quindi si tratta della GPIO27 che corrisponde al pin 13.
const GPIO_LED: u8 = 27;

fn main() {
   let device_info = DeviceInfo::new().unwrap();
   println!("Modello: {} (SoC: {})", device_info.model(), device_info.soc());
   letmut gpio = Gpio::new().unwrap();
   gpio.set_mode(GPIO_LED, Mode::Output);

   // Accende e spegne il LED ROSSO
   gpio.write(GPIO_LED, Level::High);
   thread::sleep(Duration::from_millis(500));
   gpio.write(GPIO_LED, Level::Low);
}
