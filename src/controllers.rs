/// GPIO_FAN_PIN is the pin on the raspberry pi that controls the fan
const GPIO_FAN_PIN: u64 = 23;
/// GPIO_FLAME_PIN is the pin on the raspberry pi that controls the flame
const GPIO_FLAME_PIN: u64 = 24;

/// Fireplace describes the current fireplace condition
#[derive(Clone, Debug)]
pub struct Fireplace {
    fan_pin: sysfs_gpio::Pin,
    flame_pin: sysfs_gpio::Pin,
    fan_dir: sysfs_gpio::Direction,   // If true, the fan is on
    flame_dir: sysfs_gpio::Direction, // If true, the flame is on
}

impl Fireplace {
    // Returns the current fireplace state
    pub fn new() -> Result<Fireplace, sysfs_gpio::Error> {
        let mut state = Fireplace {
            fan_pin: sysfs_gpio::Pin::new(GPIO_FAN_PIN),
            flame_pin: sysfs_gpio::Pin::new(GPIO_FLAME_PIN),
            fan_dir: sysfs_gpio::Direction::Low,
            flame_dir: sysfs_gpio::Direction::Low,
        };
        state.fan_dir = Self::_get_direction(state.fan_pin)?;
        state.flame_dir = Self::_get_direction(state.fan_pin)?;
        Ok(state)
    }

    // Given a state, set the fireplace to match the struct's state
    pub fn set(&mut self, (fan, flame): (bool, bool)) -> Result<(), sysfs_gpio::Error> {
        self.fan_pin
            .with_exported(|| Self::_set(self.fan_pin, fan))?;
        self.flame_pin
            .with_exported(|| Self::_set(self.flame_pin, flame))?;
        self.fan_dir = Self::_get_direction(self.fan_pin)?;
        self.flame_dir = Self::_get_direction(self.flame_pin)?;
        Ok(())
    }

    // Given a pin and a desired state, sets that pins direction
    fn _set(pin: sysfs_gpio::Pin, enabled: bool) -> Result<(), sysfs_gpio::Error> {
        if enabled {
            pin.set_direction(sysfs_gpio::Direction::High)
        } else {
            pin.set_direction(sysfs_gpio::Direction::Low)
        }
    }

    // Given a pin, gets the direction
    fn _get_direction(pin: sysfs_gpio::Pin) -> Result<sysfs_gpio::Direction, sysfs_gpio::Error> {
        let mut direction = sysfs_gpio::Direction::Low;
        pin.with_exported(|| {
            direction = pin.get_direction()?;
            Ok(())
        })?;
        Ok(direction)
    }
}
