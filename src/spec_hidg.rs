use color_eyre::eyre::bail;
use color_eyre::{Report, Result};
use crate::{exec_or_eyre, KeyCode, OS_Input_Coord};

#[cfg(feature = "use-hidg")]
use hidg::{Class, Device, Keyboard, Key, Led, StateChange, Button, Mouse, ValueChange, KeyboardInput, MouseInput};

#[cfg(feature = "use-hidg")]
pub struct InputEmulator {
    virtual_keyboard: Device<Keyboard>,
    virtual_mouse: Device<Mouse>,
    keyboard_input: KeyboardInput,
    mouse_input: MouseInput,
}

#[cfg(feature = "use-hidg")]
impl InputEmulator {
    pub fn new() -> Result<Self> {
        Ok(Self{
            virtual_keyboard: Device::<Keyboard>::open("hidg0")?,
            virtual_mouse: Device::<Mouse>::open("hidg1")?, //TODO: check variants
            keyboard_input: Keyboard.input(),
            mouse_input: Mouse.input(),
        })
    }


}

#[cfg(feature = "use-hidg")]
impl KeyCode {

}