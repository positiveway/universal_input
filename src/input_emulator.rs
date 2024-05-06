use color_eyre::{Report, Result};
use crate::{exec_or_eyre, KeyCode};

#[cfg(target_os = "linux")]
use mouse_keyboard_input::VirtualDevice;

pub type Coord = i32;

#[cfg(target_os = "windows")]
use enigo::{Enigo, Settings, Coordinate, Mouse, Keyboard, Axis, Button, Key};

#[cfg(target_os = "windows")]
use enigo::Direction::{Click, Press, Release};


pub struct InputEmulator {
    #[cfg(target_os = "linux")]
    virtual_device: VirtualDevice,

    #[cfg(target_os = "windows")]
    enigo: Enigo,
}

impl InputEmulator {
    #[cfg(target_os = "linux")]
    pub fn new() -> Result<Self> {
        Ok(Self {
            virtual_device: exec_or_eyre!(VirtualDevice::default())?
        })
    }

    #[cfg(target_os = "windows")]
    pub fn new() -> Result<Self> {
        Ok(Self {
            enigo: exec_or_eyre!(Enigo::new(&Settings::default()))?
        })
    }

    #[cfg(target_os = "linux")]
    pub fn move_mouse(&mut self, x: Coord, y: Coord) -> Result<()> {
        exec_or_eyre!(self.virtual_device.move_mouse(x, y))?;
        Ok(())
    }

    #[cfg(target_os = "windows")]
    pub fn move_mouse(&mut self, x: Coord, y: Coord) -> Result<()> {
        exec_or_eyre!(self.enigo.move_mouse(x, y, Coordinate::Rel))?;
        Ok(())
    }

    #[cfg(target_os = "linux")]
    pub fn scroll_x(&mut self, value: Coord) -> Result<()> {
        exec_or_eyre!(self.virtual_device.scroll_x(value))?;
        Ok(())
    }

    #[cfg(target_os = "linux")]
    pub fn scroll_y(&mut self, value: Coord) -> Result<()> {
        exec_or_eyre!(self.virtual_device.scroll_y(value))?;
        Ok(())
    }

    #[cfg(target_os = "windows")]
    pub fn scroll_x(&mut self, value: Coord) -> Result<()> {
        exec_or_eyre!(self.enigo.scroll(value, Axis::Horizontal))?;
        Ok(())
    }

    #[cfg(target_os = "windows")]
    pub fn scroll_y(&mut self, value: Coord) -> Result<()> {
        exec_or_eyre!(self.enigo.scroll(value, Axis::Vertical))?;
        Ok(())
    }

    #[cfg(target_os = "linux")]
    pub fn press(&mut self, key_code: KeyCode) -> Result<()> {
        let button = key_code.convert()?;
        exec_or_eyre!(self.virtual_device.press(button))?;
        Ok(())
    }

    #[cfg(target_os = "linux")]
    pub fn release(&mut self, key_code: KeyCode) -> Result<()> {
        let button = key_code.convert()?;
        exec_or_eyre!(self.virtual_device.release(button))?;
        Ok(())
    }

    #[cfg(target_os = "windows")]
    pub fn press(&mut self, key_code: KeyCode) -> Result<()> {
        match key_code {
            KeyCode::MOUSE_LEFT => { exec_or_eyre!(self.enigo.button(Button::Left, Press))? }
            KeyCode::MOUSE_RIGHT => { exec_or_eyre!(self.enigo.button(Button::Right, Press))? }
            KeyCode::MOUSE_MIDDLE => { exec_or_eyre!(self.enigo.button(Button::Middle, Press))? }
            _ => {
                let button = key_code.convert()?;
                exec_or_eyre!(self.enigo.key(button, Press))?;
            }
        };
        Ok(())
    }

    #[cfg(target_os = "windows")]
    pub fn release(&mut self, key_code: KeyCode) -> Result<()> {
        match key_code {
            KeyCode::MOUSE_LEFT => { exec_or_eyre!(self.enigo.button(Button::Left, Release))? }
            KeyCode::MOUSE_RIGHT => { exec_or_eyre!(self.enigo.button(Button::Right, Release))? }
            KeyCode::MOUSE_MIDDLE => { exec_or_eyre!(self.enigo.button(Button::Middle, Release))? }
            _ => {
                let button = key_code.convert()?;
                exec_or_eyre!(self.enigo.key(button, Release))?;
            }
        };
        Ok(())
    }
}