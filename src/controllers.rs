use log::{debug};
use gpio_cdev::{Chip, LineRequestFlags};
use gpio_cdev::errors::Error;

/// GPIO_FAN_PIN is the pin on the raspberry pi that controls the fan
const GPIO_FAN_PIN: u32 = 23;
/// GPIO_FLAME_PIN is the pin on the raspberry pi that controls the flame
const GPIO_FLAME_PIN: u32 = 24;

/// Fireplace describes the current fireplace condition
#[derive(Clone, Debug)]
pub struct Fireplace {
    fan_val: u8,   // If 0 the fan is off, if 1 the fan is on
    flame_val: u8, // If 0 the fan is off, if 1 the flame is on
}

impl Fireplace {
    // Returns the current fireplace state
    pub fn new() -> Result<Fireplace, Error> {
        let state = Fireplace {
            fan_val: 0,
            flame_val: 0,
        };
        Ok(state)
    }

    // Given a state, set the fireplace to match the struct's state
    pub fn set(&mut self, (fan, flame): (bool, bool)) -> Result<(), Error> {
        debug!("getting gpio chip /dev/gpiochip0");
        let mut chip = Chip::new("/dev/gpiochip0")?;

        debug!("getting fan line");
        let fan_line = chip.get_line(GPIO_FAN_PIN)?;
        debug!("getting and setting fan handle to {}", fan);
        fan_line.request(LineRequestFlags::OUTPUT, fan as u8, "fireplace-rs")?;

        debug!("getting flame line");
        let flame_line = chip.get_line(GPIO_FLAME_PIN)?;
        debug!("getting and setting flame handle to {}", flame);
        flame_line.request(LineRequestFlags::OUTPUT, flame as u8, "fireplace-rs")?;
        Ok(())
    }
}
