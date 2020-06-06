/// GPIO_FAN_PIN is the pin on the raspberry pi that controls the fan
const GPIO_FAN_PIN: u64 = 23;
/// GPIO_FLAME_PIN is the pin on the raspberry pi that controls the flame
const GPIO_FLAME_PIN: u64 = 24;

/// Fireplace describes the current fireplace condition
#[derive(Clone, Debug)]
pub struct Fireplace {
    fan_pin: sysfs_gpio::Pin,
    flame_pin: sysfs_gpio::Pin,
}

impl Fireplace {
    // Returns the current fireplace state
    pub fn new() -> Result<Fireplace, sysfs_gpio::Error> {
        let state = Fireplace {
            fan_pin: sysfs_gpio::Pin::new(GPIO_FAN_PIN),
            flame_pin: sysfs_gpio::Pin::new(GPIO_FLAME_PIN),
        };
        state.fan_pin.export()?;
        state.flame_pin.export()?;
        Ok(state)
    }

    // Given a state, set the fireplace to match the struct's state
    pub fn set(&mut self, (fan, flame): (bool, bool)) -> Result<(), sysfs_gpio::Error> {
        println!("setting fan: {}, flame: {}", fan, flame);
        Self::_set(self.fan_pin, fan)?;
        Self::_set(self.flame_pin, flame)?;
        Ok(())
    }

    // Given a pin and a desired state, sets that pins direction
    fn _set(pin: sysfs_gpio::Pin, enabled: bool) -> Result<(), sysfs_gpio::Error> {
        println!("setting {} to: {}", pin.get_pin_num(), enabled);
        let res = if enabled {
            pin.set_direction(sysfs_gpio::Direction::High)
        } else {
            pin.set_direction(sysfs_gpio::Direction::Low)
        };
        res
    }
}
