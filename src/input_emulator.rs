use color_eyre::{Report, Result};
use crate::{exec_or_eyre, KeyCode};

#[cfg(all(target_os = "linux", not(feature = "enigo-always")))]
use mouse_keyboard_input::VirtualDevice;

pub type OS_Input_Coord = i32;

#[cfg(any(target_os = "windows", all(target_os = "linux", feature = "enigo-always")))]
use enigo::{Enigo, Settings, Coordinate, Mouse, Keyboard, Axis, Button, Key};

#[cfg(any(target_os = "windows", all(target_os = "linux", feature = "enigo-always")))]
use enigo::Direction::{Click, Press, Release};


pub struct InputEmulator {
    #[cfg(all(target_os = "linux", not(feature = "enigo-always")))]
    virtual_device: VirtualDevice,

    #[cfg(any(target_os = "windows", all(target_os = "linux", feature = "enigo-always")))]
    enigo: Enigo,
}

impl InputEmulator {
    #[cfg(all(target_os = "linux", not(feature = "enigo-always")))]
    pub fn new() -> Result<Self> {
        Ok(Self {
            virtual_device: exec_or_eyre!(VirtualDevice::default())?
        })
    }

    #[cfg(any(target_os = "windows", all(target_os = "linux", feature = "enigo-always")))]
    pub fn new() -> Result<Self> {
        Ok(Self {
            enigo: exec_or_eyre!(Enigo::new(&Settings::default()))?
        })
    }

    #[cfg(all(target_os = "linux", not(feature = "enigo-always")))]
    #[inline]
    pub fn finish_operation(&mut self) -> Result<()> {
        exec_or_eyre!(self.virtual_device.synchronize())?;
        Ok(())
    }

    #[cfg(any(target_os = "windows", all(target_os = "linux", feature = "enigo-always")))]
    #[inline]
    pub fn finish_operation(&mut self) -> Result<()> {
        Ok(())
    }

    #[cfg(all(target_os = "linux", not(feature = "enigo-always")))]
    #[inline]
    pub fn move_mouse_raw_x(&mut self, x: OS_Input_Coord) -> Result<()> {
        exec_or_eyre!(self.virtual_device.move_mouse_raw_x(x))?;
        Ok(())
    }

    #[cfg(all(target_os = "linux", not(feature = "enigo-always")))]
    #[inline]
    pub fn move_mouse_raw_y(&mut self, y: OS_Input_Coord) -> Result<()> {
        exec_or_eyre!(self.virtual_device.move_mouse_raw_y(y))?;
        Ok(())
    }

    #[cfg(all(target_os = "linux", not(feature = "enigo-always")))]
    #[inline]
    pub fn move_mouse_raw(&mut self, x: OS_Input_Coord, y: OS_Input_Coord) -> Result<()> {
        exec_or_eyre!(self.virtual_device.move_mouse_raw(x, y))?;
        Ok(())
    }

    #[cfg(all(target_os = "linux", not(feature = "enigo-always")))]
    #[inline]
    pub fn move_mouse_x(&mut self, x: OS_Input_Coord) -> Result<()> {
        exec_or_eyre!(self.virtual_device.move_mouse_x(x))?;
        Ok(())
    }

    #[cfg(all(target_os = "linux", not(feature = "enigo-always")))]
    #[inline]
    pub fn move_mouse_y(&mut self, y: OS_Input_Coord) -> Result<()> {
        exec_or_eyre!(self.virtual_device.move_mouse_y(y))?;
        Ok(())
    }

    #[cfg(all(target_os = "linux", not(feature = "enigo-always")))]
    #[inline]
    pub fn move_mouse(&mut self, x: OS_Input_Coord, y: OS_Input_Coord) -> Result<()> {
        exec_or_eyre!(self.virtual_device.move_mouse(x, y))?;
        Ok(())
    }

    #[cfg(any(target_os = "windows", all(target_os = "linux", feature = "enigo-always")))]
    #[inline]
    pub fn move_mouse_raw_x(&mut self, x: OS_Input_Coord) -> Result<()> {
        exec_or_eyre!(self.enigo.move_mouse(x, 0, Coordinate::Rel))?;
        Ok(())
    }

    #[cfg(any(target_os = "windows", all(target_os = "linux", feature = "enigo-always")))]
    #[inline]
    pub fn move_mouse_raw_y(&mut self, y: OS_Input_Coord) -> Result<()> {
        exec_or_eyre!(self.enigo.move_mouse(0, y, Coordinate::Rel))?;
        Ok(())
    }

    #[cfg(any(target_os = "windows", all(target_os = "linux", feature = "enigo-always")))]
    #[inline]
    pub fn move_mouse_raw(&mut self, x: OS_Input_Coord, y: OS_Input_Coord) -> Result<()> {
        exec_or_eyre!(self.enigo.move_mouse(x, y, Coordinate::Rel))?;
        Ok(())
    }

    #[cfg(any(target_os = "windows", all(target_os = "linux", feature = "enigo-always")))]
    #[inline]
    pub fn move_mouse_x(&mut self, x: OS_Input_Coord) -> Result<()> {
        exec_or_eyre!(self.enigo.move_mouse(x, 0, Coordinate::Rel))?;
        Ok(())
    }

    #[cfg(any(target_os = "windows", all(target_os = "linux", feature = "enigo-always")))]
    #[inline]
    pub fn move_mouse_y(&mut self, y: OS_Input_Coord) -> Result<()> {
        exec_or_eyre!(self.enigo.move_mouse(0, y, Coordinate::Rel))?;
        Ok(())
    }

    #[cfg(any(target_os = "windows", all(target_os = "linux", feature = "enigo-always")))]
    #[inline]
    pub fn move_mouse(&mut self, x: OS_Input_Coord, y: OS_Input_Coord) -> Result<()> {
        exec_or_eyre!(self.enigo.move_mouse(x, y, Coordinate::Rel))?;
        Ok(())
    }

    #[cfg(all(target_os = "linux", not(feature = "enigo-always")))]
    #[inline]
    pub fn scroll_raw_x(&mut self, value: OS_Input_Coord) -> Result<()> {
        exec_or_eyre!(self.virtual_device.scroll_raw_x(value))?;
        Ok(())
    }

    #[cfg(all(target_os = "linux", not(feature = "enigo-always")))]
    #[inline]
    pub fn scroll_raw_y(&mut self, value: OS_Input_Coord) -> Result<()> {
        exec_or_eyre!(self.virtual_device.scroll_raw_y(value))?;
        Ok(())
    }

    #[cfg(all(target_os = "linux", not(feature = "enigo-always")))]
    #[inline]
    pub fn scroll_x(&mut self, value: OS_Input_Coord) -> Result<()> {
        exec_or_eyre!(self.virtual_device.scroll_x(value))?;
        Ok(())
    }

    #[cfg(all(target_os = "linux", not(feature = "enigo-always")))]
    #[inline]
    pub fn scroll_y(&mut self, value: OS_Input_Coord) -> Result<()> {
        exec_or_eyre!(self.virtual_device.scroll_y(value))?;
        Ok(())
    }

    #[cfg(any(target_os = "windows", all(target_os = "linux", feature = "enigo-always")))]
    #[inline]
    pub fn scroll_raw_x(&mut self, value: OS_Input_Coord) -> Result<()> {
        exec_or_eyre!(self.enigo.scroll(value, Axis::Horizontal))?;
        Ok(())
    }

    #[cfg(any(target_os = "windows", all(target_os = "linux", feature = "enigo-always")))]
    #[inline]
    pub fn scroll_raw_y(&mut self, value: OS_Input_Coord) -> Result<()> {
        exec_or_eyre!(self.enigo.scroll(value, Axis::Vertical))?;
        Ok(())
    }

    #[cfg(any(target_os = "windows", all(target_os = "linux", feature = "enigo-always")))]
    #[inline]
    pub fn scroll_x(&mut self, value: OS_Input_Coord) -> Result<()> {
        exec_or_eyre!(self.enigo.scroll(value, Axis::Horizontal))?;
        Ok(())
    }

    #[cfg(any(target_os = "windows", all(target_os = "linux", feature = "enigo-always")))]
    #[inline]
    pub fn scroll_y(&mut self, value: OS_Input_Coord) -> Result<()> {
        exec_or_eyre!(self.enigo.scroll(value, Axis::Vertical))?;
        Ok(())
    }

    #[cfg(all(target_os = "linux", not(feature = "enigo-always")))]
    #[inline]
    pub fn press(&mut self, key_code: KeyCode) -> Result<()> {
        let button = key_code.convert()?;
        exec_or_eyre!(self.virtual_device.press(button))?;
        Ok(())
    }

    #[cfg(all(target_os = "linux", not(feature = "enigo-always")))]
    #[inline]
    pub fn release(&mut self, key_code: KeyCode) -> Result<()> {
        let button = key_code.convert()?;
        exec_or_eyre!(self.virtual_device.release(button))?;
        Ok(())
    }

    #[cfg(any(target_os = "windows", all(target_os = "linux", feature = "enigo-always")))]
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

    #[cfg(any(target_os = "windows", all(target_os = "linux", feature = "enigo-always")))]
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