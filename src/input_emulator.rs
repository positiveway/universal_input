use color_eyre::{Report, Result};
use crate::{exec_or_eyre, KeyCode};

#[cfg(target_os = "linux")]
use mouse_keyboard_input::VirtualDevice;

pub type OS_Input_Coord = i32;

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
    #[inline]
    pub fn move_mouse_raw_x(&mut self, x: OS_Input_Coord) -> Result<()> {
        exec_or_eyre!(self.virtual_device.move_mouse_raw_x(x))?;
        Ok(())
    }

    #[cfg(target_os = "linux")]
    #[inline]
    pub fn move_mouse_raw_y(&mut self, y: OS_Input_Coord) -> Result<()> {
        exec_or_eyre!(self.virtual_device.move_mouse_raw_y(y))?;
        Ok(())
    }

    #[cfg(target_os = "linux")]
    #[inline]
    pub fn move_mouse_raw(&mut self, x: OS_Input_Coord, y: OS_Input_Coord) -> Result<()> {
        exec_or_eyre!(self.virtual_device.move_mouse_raw(x, y))?;
        Ok(())
    }

    #[cfg(target_os = "linux")]
    #[inline]
    pub fn finish_operation(&mut self) -> Result<()> {
        exec_or_eyre!(self.virtual_device.synchronize())?;
        Ok(())
    }

    #[cfg(target_os = "linux")]
    #[inline]
    pub fn move_mouse_x(&mut self, x: OS_Input_Coord) -> Result<()> {
        exec_or_eyre!(self.virtual_device.move_mouse_x(x))?;
        Ok(())
    }

    #[cfg(target_os = "linux")]
    #[inline]
    pub fn move_mouse_y(&mut self, y: OS_Input_Coord) -> Result<()> {
        exec_or_eyre!(self.virtual_device.move_mouse_y(y))?;
        Ok(())
    }

    #[cfg(target_os = "linux")]
    #[inline]
    pub fn move_mouse(&mut self, x: OS_Input_Coord, y: OS_Input_Coord) -> Result<()> {
        exec_or_eyre!(self.virtual_device.move_mouse(x, y))?;
        Ok(())
    }

    #[cfg(target_os = "windows")]
    #[inline]
    pub fn move_mouse_raw_x(&mut self, x: OS_Input_Coord) -> Result<()> {
        exec_or_eyre!(self.enigo.move_mouse(x, 0, Coordinate::Rel))?;
        Ok(())
    }

    #[cfg(target_os = "windows")]
    #[inline]
    pub fn move_mouse_raw_y(&mut self, y: OS_Input_Coord) -> Result<()> {
        exec_or_eyre!(self.enigo.move_mouse(0, y, Coordinate::Rel))?;
        Ok(())
    }

    #[cfg(target_os = "windows")]
    #[inline]
    pub fn move_mouse_raw(&mut self, x: OS_Input_Coord, y: OS_Input_Coord) -> Result<()> {
        exec_or_eyre!(self.enigo.move_mouse(x, y, Coordinate::Rel))?;
        Ok(())
    }

    #[cfg(target_os = "windows")]
    #[inline]
    pub fn finish_operation(&mut self) -> Result<()> {
        Ok(())
    }

    #[cfg(target_os = "windows")]
    #[inline]
    pub fn move_mouse_x(&mut self, x: OS_Input_Coord) -> Result<()> {
        exec_or_eyre!(self.enigo.move_mouse(x, 0, Coordinate::Rel))?;
        Ok(())
    }

    #[cfg(target_os = "windows")]
    #[inline]
    pub fn move_mouse_y(&mut self, y: OS_Input_Coord) -> Result<()> {
        exec_or_eyre!(self.enigo.move_mouse(0, y, Coordinate::Rel))?;
        Ok(())
    }

    #[cfg(target_os = "windows")]
    #[inline]
    pub fn move_mouse(&mut self, x: OS_Input_Coord, y: OS_Input_Coord) -> Result<()> {
        exec_or_eyre!(self.enigo.move_mouse(x, y, Coordinate::Rel))?;
        Ok(())
    }

    #[cfg(target_os = "linux")]
    #[inline]
    pub fn scroll_x(&mut self, value: OS_Input_Coord) -> Result<()> {
        exec_or_eyre!(self.virtual_device.scroll_x(value))?;
        Ok(())
    }

    #[cfg(target_os = "linux")]
    #[inline]
    pub fn scroll_y(&mut self, value: OS_Input_Coord) -> Result<()> {
        exec_or_eyre!(self.virtual_device.scroll_y(value))?;
        Ok(())
    }

    #[cfg(target_os = "windows")]
    #[inline]
    pub fn scroll_x(&mut self, value: OS_Input_Coord) -> Result<()> {
        exec_or_eyre!(self.enigo.scroll(value, Axis::Horizontal))?;
        Ok(())
    }

    #[cfg(target_os = "windows")]
    #[inline]
    pub fn scroll_y(&mut self, value: OS_Input_Coord) -> Result<()> {
        exec_or_eyre!(self.enigo.scroll(value, Axis::Vertical))?;
        Ok(())
    }

    #[cfg(target_os = "linux")]
    #[inline]
    pub fn press(&mut self, key_code: KeyCode) -> Result<()> {
        let button = key_code.convert()?;
        exec_or_eyre!(self.virtual_device.press(button))?;
        Ok(())
    }

    #[cfg(target_os = "linux")]
    #[inline]
    pub fn release(&mut self, key_code: KeyCode) -> Result<()> {
        let button = key_code.convert()?;
        exec_or_eyre!(self.virtual_device.release(button))?;
        Ok(())
    }

    #[cfg(target_os = "windows")]
    #[inline]
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
    #[inline]
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