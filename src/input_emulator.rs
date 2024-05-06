use color_eyre::{Report, Result};
use crate::{exec_or_eyre, KeyCode};

#[cfg(target_os = "linux")]
use mouse_keyboard_input::VirtualDevice;

pub type Coord = i32;

#[cfg(target_os = "windows")]
use enigo::{Enigo, Settings};
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
    pub fn move_mouse(&mut self, x: Coord, y: Coord) -> Result<()>{
        exec_or_eyre!(self.virtual_device.move_mouse(x, y))?;
        Ok(())
    }

    #[cfg(target_os = "windows")]
    pub fn move_mouse(&mut self, x: Coord, y: Coord) -> Result<()>{
        exec_or_eyre!(self.enigo.move_mouse(x, y))?;
        Ok(())
    }

    #[cfg(target_os = "linux")]
    pub fn scroll_x(&mut self, value: Coord) -> Result<()>{
        exec_or_eyre!(self.virtual_device.scroll_x(value))?;
        Ok(())
    }

    #[cfg(target_os = "linux")]
    pub fn scroll_y(&mut self, value: Coord) -> Result<()>{
        exec_or_eyre!(self.virtual_device.scroll_y(value))?;
        Ok(())
    }

    #[cfg(target_os = "windows")]
    pub fn scroll_x(&mut self, value: Coord) -> Result<()>{
        exec_or_eyre!(self.enigo.mouse_scroll_x(value))?;
        Ok(())
    }

    #[cfg(target_os = "windows")]
    pub fn scroll_y(&mut self, value: Coord) -> Result<()>{
        exec_or_eyre!(self.enigo.mouse_scroll_y(value))?;
        Ok(())
    }

    #[cfg(target_os = "linux")]
    pub fn press(&mut self, key_code: KeyCode) -> Result<()>{
        let button = key_code.as_button()?;
        exec_or_eyre!(self.virtual_device.press(button))?;
        Ok(())
    }

    #[cfg(target_os = "linux")]
    pub fn release(&mut self, key_code: KeyCode) -> Result<()>{
        let button = key_code.as_button()?;
        exec_or_eyre!(self.virtual_device.release(button))?;
        Ok(())
    }

    #[cfg(target_os = "windows")]
    pub fn press(&mut self, key_code: KeyCode) -> Result<()>{
        let button = key_code.as_button()?;
        exec_or_eyre!(self.enigo.raw(button, Press))?;
        Ok(())
    }

    #[cfg(target_os = "windows")]
    pub fn release(&mut self, key_code: KeyCode) -> Result<()>{
        let button = key_code.as_button()?;
        exec_or_eyre!(self.enigo.raw(button, Release))?;
        Ok(())
    }
}