use color_eyre::{Report, Result};
use crate::{exec_or_eyre, KeyCode};

pub type OS_Input_Coord = i32;

#[cfg(all(target_os = "linux", not(feature = "enigo-always"), not(feature = "use-tfc")))]
use mouse_keyboard_input::VirtualDevice;

#[cfg(feature = "use-tfc")]
use tfc::{Context, Error, traits::*, MouseButton};

#[cfg(all(not(feature = "use-tfc"), any(target_os = "windows", all(target_os = "linux", feature = "enigo-always"))))]
use enigo::{Enigo, Settings, Coordinate, Mouse, Keyboard, Axis, Button, Key};

#[cfg(all(not(feature = "use-tfc"), any(target_os = "windows", all(target_os = "linux", feature = "enigo-always"))))]
use enigo::Direction::{Click, Press, Release};


pub struct InputEmulator {
    #[cfg(all(target_os = "linux", not(feature = "enigo-always"), not(feature = "use-tfc")))]
    // virtual_mouse: VirtualDevice,
    // virtual_keyboard: VirtualDevice,
    virtual_device: VirtualDevice,

    #[cfg(feature = "use-tfc")]
    ctx: Context,

    #[cfg(all(not(feature = "use-tfc"), any(target_os = "windows", all(target_os = "linux", feature = "enigo-always"))))]
    enigo: Enigo,
}

impl InputEmulator {
    #[cfg(all(target_os = "linux", not(feature = "enigo-always"), not(feature = "use-tfc")))]
    pub fn new() -> Result<Self> {
        // let (virtual_mouse, virtual_keyboard) = exec_or_eyre!(VirtualDevice::default())?;
        // Ok(Self {
        //     virtual_mouse,
        //     virtual_keyboard,
        // })

        Ok(Self{
            virtual_device: exec_or_eyre!(VirtualDevice::default())?,
        })
    }

    #[cfg(feature = "use-tfc")]
    pub fn new() -> Result<Self> {
        Ok(Self{
            ctx: Context::new()?,
        })
    }

    #[cfg(all(not(feature = "use-tfc"), any(target_os = "windows", all(target_os = "linux", feature = "enigo-always"))))]
    pub fn new() -> Result<Self> {
        Ok(Self {
            enigo: exec_or_eyre!(Enigo::new(&Settings::default()))?
        })
    }

    #[cfg(all(target_os = "linux", not(feature = "enigo-always"), not(feature = "use-tfc")))]
    #[inline]
    pub fn finish_operation_mouse(&mut self) -> Result<()> {
        // exec_or_eyre!(self.virtual_mouse.synchronize())?;
        exec_or_eyre!(self.virtual_device.synchronize())?;
        Ok(())
    }

    #[cfg(feature = "use-tfc")]
    #[inline]
    pub fn finish_operation_mouse(&mut self) -> Result<()> {
        Ok(())
    }

    // #[cfg(all(target_os = "linux", not(feature = "enigo-always"), not(feature = "use-tfc")))]
    // #[inline]
    // pub fn finish_operation_keyboard(&mut self) -> Result<()> {
    //     exec_or_eyre!(self.virtual_keyboard.synchronize())?;
    //     Ok(())
    // }

    #[cfg(all(not(feature = "use-tfc"), any(target_os = "windows", all(target_os = "linux", feature = "enigo-always"))))]
    #[inline]
    pub fn finish_operation_mouse(&mut self) -> Result<()> {
        Ok(())
    }

    // #[cfg(all(not(feature = "use-tfc"), any(target_os = "windows", all(target_os = "linux", feature = "enigo-always"))))]
    // #[inline]
    // pub fn finish_operation_keyboard(&mut self) -> Result<()> {
    //     Ok(())
    // }

    #[cfg(all(target_os = "linux", not(feature = "enigo-always"), not(feature = "use-tfc")))]
    #[inline]
    pub fn move_mouse_raw_x(&mut self, x: OS_Input_Coord) -> Result<()> {
        // exec_or_eyre!(self.virtual_mouse.move_mouse_raw_x(x))?;
        exec_or_eyre!(self.virtual_device.move_mouse_raw_x(x))?;
        Ok(())
    }

    #[cfg(all(target_os = "linux", not(feature = "enigo-always"), not(feature = "use-tfc")))]
    #[inline]
    pub fn move_mouse_raw_y(&mut self, y: OS_Input_Coord) -> Result<()> {
        // exec_or_eyre!(self.virtual_mouse.move_mouse_raw_y(y))?;
        exec_or_eyre!(self.virtual_device.move_mouse_raw_y(y))?;
        Ok(())
    }

    #[cfg(all(target_os = "linux", not(feature = "enigo-always"), not(feature = "use-tfc")))]
    #[inline]
    pub fn move_mouse_raw(&mut self, x: OS_Input_Coord, y: OS_Input_Coord) -> Result<()> {
        // exec_or_eyre!(self.virtual_mouse.move_mouse_raw(x, y))?;
        exec_or_eyre!(self.virtual_device.move_mouse_raw(x, y))?;
        Ok(())
    }

    #[cfg(all(target_os = "linux", not(feature = "enigo-always"), not(feature = "use-tfc")))]
    #[inline]
    pub fn move_mouse_x(&mut self, x: OS_Input_Coord) -> Result<()> {
        // exec_or_eyre!(self.virtual_mouse.move_mouse_x(x))?;
        exec_or_eyre!(self.virtual_device.move_mouse_x(x))?;
        Ok(())
    }

    #[cfg(all(target_os = "linux", not(feature = "enigo-always"), not(feature = "use-tfc")))]
    #[inline]
    pub fn move_mouse_y(&mut self, y: OS_Input_Coord) -> Result<()> {
        // exec_or_eyre!(self.virtual_mouse.move_mouse_y(y))?;
        exec_or_eyre!(self.virtual_device.move_mouse_y(y))?;
        Ok(())
    }

    #[cfg(all(target_os = "linux", not(feature = "enigo-always"), not(feature = "use-tfc")))]
    #[inline]
    pub fn move_mouse(&mut self, x: OS_Input_Coord, y: OS_Input_Coord) -> Result<()> {
        // exec_or_eyre!(self.virtual_mouse.move_mouse(x, y))?;
        exec_or_eyre!(self.virtual_device.move_mouse(x, y))?;
        Ok(())
    }

    #[cfg(feature = "use-tfc")]
    #[inline]
    pub fn move_mouse_raw_x(&mut self, x: OS_Input_Coord) -> Result<()> {
        exec_or_eyre!(self.ctx.mouse_move_rel(x, 0))?;
        Ok(())
    }

    #[cfg(feature = "use-tfc")]
    #[inline]
    pub fn move_mouse_raw_y(&mut self, y: OS_Input_Coord) -> Result<()> {
        exec_or_eyre!(self.ctx.mouse_move_rel(0, -y))?;
        Ok(())
    }

    #[cfg(feature = "use-tfc")]
    #[inline]
    pub fn move_mouse_raw(&mut self, x: OS_Input_Coord, y: OS_Input_Coord) -> Result<()> {
        exec_or_eyre!(self.ctx.mouse_move_rel(x, -y))?;
        Ok(())
    }

    #[cfg(feature = "use-tfc")]
    #[inline]
    pub fn move_mouse(&mut self, x: OS_Input_Coord, y: OS_Input_Coord) -> Result<()> {
        exec_or_eyre!(self.ctx.mouse_move_rel(x, -y))?;
        Ok(())
    }

    #[cfg(all(not(feature = "use-tfc"), any(target_os = "windows", all(target_os = "linux", feature = "enigo-always"))))]
    #[inline]
    pub fn move_mouse_raw_x(&mut self, x: OS_Input_Coord) -> Result<()> {
        exec_or_eyre!(self.enigo.move_mouse(x, 0, Coordinate::Rel))?;
        Ok(())
    }

    #[cfg(all(not(feature = "use-tfc"), any(target_os = "windows", all(target_os = "linux", feature = "enigo-always"))))]
    #[inline]
    pub fn move_mouse_raw_y(&mut self, y: OS_Input_Coord) -> Result<()> {
        exec_or_eyre!(self.enigo.move_mouse(0, y, Coordinate::Rel))?;
        Ok(())
    }

    #[cfg(all(not(feature = "use-tfc"), any(target_os = "windows", all(target_os = "linux", feature = "enigo-always"))))]
    #[inline]
    pub fn move_mouse_raw(&mut self, x: OS_Input_Coord, y: OS_Input_Coord) -> Result<()> {
        exec_or_eyre!(self.enigo.move_mouse(x, y, Coordinate::Rel))?;
        Ok(())
    }

    #[cfg(all(not(feature = "use-tfc"), any(target_os = "windows", all(target_os = "linux", feature = "enigo-always"))))]
    #[inline]
    pub fn move_mouse_x(&mut self, x: OS_Input_Coord) -> Result<()> {
        exec_or_eyre!(self.enigo.move_mouse(x, 0, Coordinate::Rel))?;
        Ok(())
    }

    #[cfg(all(not(feature = "use-tfc"), any(target_os = "windows", all(target_os = "linux", feature = "enigo-always"))))]
    #[inline]
    pub fn move_mouse_y(&mut self, y: OS_Input_Coord) -> Result<()> {
        exec_or_eyre!(self.enigo.move_mouse(0, y, Coordinate::Rel))?;
        Ok(())
    }

    #[cfg(all(not(feature = "use-tfc"), any(target_os = "windows", all(target_os = "linux", feature = "enigo-always"))))]
    #[inline]
    pub fn move_mouse(&mut self, x: OS_Input_Coord, y: OS_Input_Coord) -> Result<()> {
        exec_or_eyre!(self.enigo.move_mouse(x, y, Coordinate::Rel))?;
        Ok(())
    }

    #[cfg(all(target_os = "linux", not(feature = "enigo-always"), not(feature = "use-tfc")))]
    #[inline]
    pub fn scroll_raw_x(&mut self, value: OS_Input_Coord) -> Result<()> {
        // exec_or_eyre!(self.virtual_mouse.scroll_raw_x(value))?;
        exec_or_eyre!(self.virtual_device.scroll_raw_x(value))?;
        Ok(())
    }

    #[cfg(all(target_os = "linux", not(feature = "enigo-always"), not(feature = "use-tfc")))]
    #[inline]
    pub fn scroll_raw_y(&mut self, value: OS_Input_Coord) -> Result<()> {
        // exec_or_eyre!(self.virtual_mouse.scroll_raw_y(value))?;
        exec_or_eyre!(self.virtual_device.scroll_raw_y(value))?;
        Ok(())
    }

    #[cfg(all(target_os = "linux", not(feature = "enigo-always"), not(feature = "use-tfc")))]
    #[inline]
    pub fn scroll_x(&mut self, value: OS_Input_Coord) -> Result<()> {
        // exec_or_eyre!(self.virtual_mouse.scroll_x(value))?;
        exec_or_eyre!(self.virtual_device.scroll_x(value))?;
        Ok(())
    }

    #[cfg(all(target_os = "linux", not(feature = "enigo-always"), not(feature = "use-tfc")))]
    #[inline]
    pub fn scroll_y(&mut self, value: OS_Input_Coord) -> Result<()> {
        // exec_or_eyre!(self.virtual_mouse.scroll_y(value))?;
        exec_or_eyre!(self.virtual_device.scroll_y(value))?;
        Ok(())
    }

    #[cfg(feature = "use-tfc")]
    #[inline]
    pub fn scroll_raw_x(&mut self, value: OS_Input_Coord) -> Result<()> {
        exec_or_eyre!(self.ctx.mouse_scroll(value, 0))?;
        Ok(())
    }

    #[cfg(feature = "use-tfc")]
    #[inline]
    pub fn scroll_raw_y(&mut self, value: OS_Input_Coord) -> Result<()> {
        exec_or_eyre!(self.ctx.mouse_scroll(0, -value))?;
        Ok(())
    }

    #[cfg(feature = "use-tfc")]
    #[inline]
    pub fn scroll_x(&mut self, value: OS_Input_Coord) -> Result<()> {
        exec_or_eyre!(self.ctx.mouse_scroll(value, 0))?;
        Ok(())
    }

    #[cfg(feature = "use-tfc")]
    #[inline]
    pub fn scroll_y(&mut self, value: OS_Input_Coord) -> Result<()> {
        exec_or_eyre!(self.ctx.mouse_scroll(0, -value))?;
        Ok(())
    }

    #[cfg(all(not(feature = "use-tfc"), any(target_os = "windows", all(target_os = "linux", feature = "enigo-always"))))]
    #[inline]
    pub fn scroll_raw_x(&mut self, value: OS_Input_Coord) -> Result<()> {
        exec_or_eyre!(self.enigo.scroll(value, Axis::Horizontal))?;
        Ok(())
    }

    #[cfg(all(not(feature = "use-tfc"), any(target_os = "windows", all(target_os = "linux", feature = "enigo-always"))))]
    #[inline]
    pub fn scroll_raw_y(&mut self, value: OS_Input_Coord) -> Result<()> {
        exec_or_eyre!(self.enigo.scroll(value, Axis::Vertical))?;
        Ok(())
    }

    #[cfg(all(not(feature = "use-tfc"), any(target_os = "windows", all(target_os = "linux", feature = "enigo-always"))))]
    #[inline]
    pub fn scroll_x(&mut self, value: OS_Input_Coord) -> Result<()> {
        exec_or_eyre!(self.enigo.scroll(value, Axis::Horizontal))?;
        Ok(())
    }

    #[cfg(all(not(feature = "use-tfc"), any(target_os = "windows", all(target_os = "linux", feature = "enigo-always"))))]
    #[inline]
    pub fn scroll_y(&mut self, value: OS_Input_Coord) -> Result<()> {
        exec_or_eyre!(self.enigo.scroll(value, Axis::Vertical))?;
        Ok(())
    }

    #[cfg(all(target_os = "linux", not(feature = "enigo-always"), not(feature = "use-tfc")))]
    #[inline]
    pub fn press(&mut self, key_code: KeyCode) -> Result<()> {
        // let virtual_device = match key_code {
        //     KeyCode::MOUSE_LEFT => &mut self.virtual_mouse,
        //     KeyCode::MOUSE_RIGHT => &mut self.virtual_mouse,
        //     KeyCode::MOUSE_MIDDLE => &mut self.virtual_mouse,
        //     _ => &mut self.virtual_keyboard,
        // };
        let button = key_code.convert()?;
        // exec_or_eyre!(virtual_device.press(button))?;
        exec_or_eyre!(self.virtual_device.press(button))?;
        Ok(())
    }

    #[cfg(all(target_os = "linux", not(feature = "enigo-always"), not(feature = "use-tfc")))]
    #[inline]
    pub fn release(&mut self, key_code: KeyCode) -> Result<()> {
        // let virtual_device = match key_code {
        //     KeyCode::MOUSE_LEFT => &mut self.virtual_mouse,
        //     KeyCode::MOUSE_RIGHT => &mut self.virtual_mouse,
        //     KeyCode::MOUSE_MIDDLE => &mut self.virtual_mouse,
        //     _ => &mut self.virtual_keyboard,
        // };
        let button = key_code.convert()?;
        // exec_or_eyre!(virtual_device.release(button))?;
        exec_or_eyre!(self.virtual_device.release(button))?;
        Ok(())
    }

    #[cfg(feature = "use-tfc")]
    #[inline]
    pub fn press(&mut self, key_code: KeyCode) -> Result<()> {
        match key_code {
            KeyCode::MOUSE_LEFT => { exec_or_eyre!(self.ctx.mouse_down(MouseButton::Left))? }
            KeyCode::MOUSE_RIGHT => { exec_or_eyre!(self.ctx.mouse_down(MouseButton::Right))? }
            KeyCode::MOUSE_MIDDLE => { exec_or_eyre!(self.ctx.mouse_down(MouseButton::Middle))? }
            _ => {
                let button = key_code.convert()?;
                exec_or_eyre!(self.ctx.key_down(button))?;
            }
        };
        Ok(())
    }

    #[cfg(feature = "use-tfc")]
    #[inline]
    pub fn release(&mut self, key_code: KeyCode) -> Result<()> {
        match key_code {
            KeyCode::MOUSE_LEFT => { exec_or_eyre!(self.ctx.mouse_up(MouseButton::Left))? }
            KeyCode::MOUSE_RIGHT => { exec_or_eyre!(self.ctx.mouse_up(MouseButton::Right))? }
            KeyCode::MOUSE_MIDDLE => { exec_or_eyre!(self.ctx.mouse_up(MouseButton::Middle))? }
            _ => {
                let button = key_code.convert()?;
                exec_or_eyre!(self.ctx.key_up(button))?;
            }
        };
        Ok(())
    }

    #[cfg(all(not(feature = "use-tfc"), any(target_os = "windows", all(target_os = "linux", feature = "enigo-always"))))]
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

    #[cfg(all(not(feature = "use-tfc"), any(target_os = "windows", all(target_os = "linux", feature = "enigo-always"))))]
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