use color_eyre::{Report, Result};
use crate::{exec_or_eyre, KeyCode};
use crate::utils::GradualMove;

pub type OS_Input_Coord = i32;

use mouse_keyboard_input::EventParams;

#[cfg(all(target_os = "linux", not(feature = "enigo-always"), not(feature = "use-tfc"), not(feature = "use-hidg")))]
use mouse_keyboard_input::{VirtualDevice};

#[cfg(all(target_os = "linux", not(feature = "enigo-always"), not(feature = "use-tfc"), feature = "use-hidg"))]
use hidg::{Class, Device, Keyboard, Key, Led, StateChange, Button, Mouse, ValueChange, KeyboardInput, MouseInput};

#[cfg(feature = "use-tfc")]
use tfc::{Context, Error, traits::*, MouseButton};
#[cfg(all(not(feature = "use-tfc"), any(target_os = "windows", all(target_os = "linux", feature = "enigo-always"))))]
use enigo::{Enigo, Settings, Coordinate, Mouse, Keyboard, Axis, Button, Key};
#[cfg(all(not(feature = "use-tfc"), any(target_os = "windows", all(target_os = "linux", feature = "enigo-always"))))]
use enigo::Direction::{Click, Press, Release};


#[cfg(all(target_os = "linux", not(feature = "enigo-always"), not(feature = "use-tfc"), not(feature = "use-hidg")))]
pub struct InputEmulator {
    // virtual_mouse: VirtualDevice,
    // virtual_keyboard: VirtualDevice,
    virtual_device: VirtualDevice,
}

#[cfg(all(target_os = "linux", not(feature = "enigo-always"), not(feature = "use-tfc"), feature = "use-hidg"))]
pub struct InputEmulator {
    virtual_keyboard: Device<Keyboard>,
    virtual_mouse: Device<Mouse>,
    keyboard_input: KeyboardInput,
    mouse_input: MouseInput,
}

#[cfg(feature = "use-tfc")]
pub struct InputEmulator {
    ctx: Context,
}

#[cfg(all(not(feature = "use-tfc"), any(target_os = "windows", all(target_os = "linux", feature = "enigo-always"))))]
pub struct InputEmulator {
    enigo: Enigo,
}

// TODO: remove
// pub struct InputEmulator {
//     #[cfg(all(target_os = "linux", not(feature = "enigo-always"), not(feature = "use-tfc"), not(feature = "use-hidg")))]
//     // virtual_mouse: VirtualDevice,
//     // virtual_keyboard: VirtualDevice,
//     virtual_device: VirtualDevice,
// 
//     #[cfg(all(target_os = "linux", not(feature = "enigo-always"), not(feature = "use-tfc"), feature = "use-hidg"))]
//     virtual_keyboard: Device<Keyboard>,
//     #[cfg(all(target_os = "linux", not(feature = "enigo-always"), not(feature = "use-tfc"), feature = "use-hidg"))]
//     virtual_mouse: Device<Mouse>,
//     #[cfg(all(target_os = "linux", not(feature = "enigo-always"), not(feature = "use-tfc"), feature = "use-hidg"))]
//     keyboard_input: KeyboardInput,
//     #[cfg(all(target_os = "linux", not(feature = "enigo-always"), not(feature = "use-tfc"), feature = "use-hidg"))]
//     mouse_input: MouseInput,
// 
//     #[cfg(feature = "use-tfc")]
//     ctx: Context,
// 
//     #[cfg(all(not(feature = "use-tfc"), any(target_os = "windows", all(target_os = "linux", feature = "enigo-always"))))]
//     enigo: Enigo,
// }

impl InputEmulator {
    #[cfg(all(target_os = "linux", not(feature = "enigo-always"), not(feature = "use-tfc"), not(feature = "use-hidg")))]
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

    #[cfg(all(target_os = "linux", not(feature = "enigo-always"), not(feature = "use-tfc"), feature = "use-hidg"))]
    pub fn new() -> Result<Self> {
        Ok(Self{
            virtual_keyboard: Device::<Keyboard>::open("hidg0")?,
            virtual_mouse: Device::<Mouse>::open("hidg1")?, //TODO: check variants
            keyboard_input: Keyboard.input(),
            mouse_input: Mouse.input(),
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

    #[cfg(all(target_os = "linux", not(feature = "enigo-always"), not(feature = "use-tfc"), not(feature = "use-hidg")))]
    #[inline]
    pub fn finish_operation_mouse(&mut self) -> Result<()> {
        // exec_or_eyre!(self.virtual_mouse.synchronize())?;
        exec_or_eyre!(self.virtual_device.synchronize())?;
        Ok(())
    }

    #[cfg(any(target_os = "windows", feature = "use-tfc", feature = "enigo-always", feature = "use-hidg"))]
    #[inline]
    pub fn finish_operation_mouse(&mut self) -> Result<()> {
        Ok(())
    }

    #[cfg(all(target_os = "linux", not(feature = "enigo-always"), not(feature = "use-tfc"), not(feature = "use-hidg")))]
    #[inline]
    pub fn write_buffer(&mut self, buffer: Vec<EventParams>) -> Result<()> {
        exec_or_eyre!(self.virtual_device.write_batch(buffer))?;
        Ok(())
    }

    #[cfg(any(target_os = "windows", feature = "use-tfc", feature = "enigo-always", feature = "use-hidg"))]
    #[inline]
    pub fn write_buffer(&mut self, buffer: Vec<EventParams>) -> Result<()> {
        Ok(())
    }

    // #[cfg(all(target_os = "linux", not(feature = "enigo-always"), not(feature = "use-tfc"), not(feature = "use-hidg")))]
    // #[inline]
    // pub fn finish_operation_keyboard(&mut self) -> Result<()> {
    //     exec_or_eyre!(self.virtual_keyboard.synchronize())?;
    //     Ok(())
    // }

    #[cfg(all(target_os = "linux", not(feature = "enigo-always"), not(feature = "use-tfc"), not(feature = "use-hidg")))]
    #[inline]
    pub fn move_mouse_raw_x(&mut self, x: OS_Input_Coord) -> Result<()> {
        // exec_or_eyre!(self.virtual_mouse.move_mouse_raw_x(x))?;
        exec_or_eyre!(self.virtual_device.move_mouse_raw_x(x))?;
        Ok(())
    }

    #[cfg(any(target_os = "windows", feature = "use-tfc", feature = "enigo-always", feature = "use-hidg"))]
    #[inline]
    pub fn move_mouse_raw_x(&mut self, x: OS_Input_Coord) -> Result<()> {
        self.move_mouse_x(x)
    }

    #[cfg(all(target_os = "linux", not(feature = "enigo-always"), not(feature = "use-tfc"), not(feature = "use-hidg")))]
    #[inline]
    pub fn move_mouse_raw_y(&mut self, y: OS_Input_Coord) -> Result<()> {
        // exec_or_eyre!(self.virtual_mouse.move_mouse_raw_y(y))?;
        exec_or_eyre!(self.virtual_device.move_mouse_raw_y(y))?;
        Ok(())
    }

    #[cfg(any(target_os = "windows", feature = "use-tfc", feature = "enigo-always", feature = "use-hidg"))]
    #[inline]
    pub fn move_mouse_raw_y(&mut self, y: OS_Input_Coord) -> Result<()> {
        self.move_mouse_y(y)
    }

    #[cfg(all(target_os = "linux", not(feature = "enigo-always"), not(feature = "use-tfc"), not(feature = "use-hidg")))]
    #[inline]
    pub fn move_mouse_raw(&mut self, x: OS_Input_Coord, y: OS_Input_Coord) -> Result<()> {
        // exec_or_eyre!(self.virtual_mouse.move_mouse_raw(x, y))?;
        exec_or_eyre!(self.virtual_device.move_mouse_raw(x, y))?;
        Ok(())
    }

    #[cfg(any(target_os = "windows", feature = "use-tfc", feature = "enigo-always", feature = "use-hidg"))]
    #[inline]
    pub fn move_mouse_raw(&mut self, x: OS_Input_Coord, y: OS_Input_Coord) -> Result<()> {
        self.move_mouse(x, y)
    }

    #[cfg(all(target_os = "linux", not(feature = "enigo-always"), not(feature = "use-tfc"), not(feature = "use-hidg")))]
    #[inline]
    pub fn buffered_move_mouse_x(&mut self, x: OS_Input_Coord) -> Vec<EventParams> {
        self.virtual_device.buffered_move_mouse_x(x)
    }

    #[cfg(any(target_os = "windows", feature = "use-tfc", feature = "enigo-always", feature = "use-hidg"))]
    #[inline]
    pub fn buffered_move_mouse_x(&mut self, x: OS_Input_Coord) -> Vec<EventParams> {
        self.move_mouse_x(x).unwrap();
        vec![]
    }

    #[cfg(all(target_os = "linux", not(feature = "enigo-always"), not(feature = "use-tfc"), not(feature = "use-hidg")))]
    #[inline]
    pub fn buffered_move_mouse_y(&mut self, y: OS_Input_Coord) -> Vec<EventParams> {
        self.virtual_device.buffered_move_mouse_y(y)
    }

    #[cfg(any(target_os = "windows", feature = "use-tfc", feature = "enigo-always", feature = "use-hidg"))]
    #[inline]
    pub fn buffered_move_mouse_y(&mut self, y: OS_Input_Coord) -> Vec<EventParams> {
        self.move_mouse_y(y).unwrap();
        vec![]
    }

    #[cfg(all(target_os = "linux", not(feature = "enigo-always"), not(feature = "use-tfc"), not(feature = "use-hidg")))]
    #[inline]
    pub fn buffered_move_mouse(&mut self, x: OS_Input_Coord, y: OS_Input_Coord) -> Vec<EventParams> {
        self.virtual_device.buffered_move_mouse(x, y)
    }

    #[cfg(any(target_os = "windows", feature = "use-tfc", feature = "enigo-always", feature = "use-hidg"))]
    #[inline]
    pub fn buffered_move_mouse(&mut self, x: OS_Input_Coord, y: OS_Input_Coord) -> Vec<EventParams> {
        self.move_mouse(x, y).unwrap();
        vec![]
    }

    #[cfg(all(target_os = "linux", not(feature = "enigo-always"), not(feature = "use-tfc"), not(feature = "use-hidg")))]
    #[inline]
    pub fn buffered_gradual_move_mouse(&mut self, x: OS_Input_Coord, y: OS_Input_Coord) -> Vec<EventParams> {
        self.virtual_device.buffered_gradual_move_mouse(x, y)
    }

    #[cfg(any(target_os = "windows", feature = "use-tfc", feature = "enigo-always", feature = "use-hidg"))]
    #[inline]
    pub fn buffered_gradual_move_mouse(&mut self, x: OS_Input_Coord, y: OS_Input_Coord) -> Vec<EventParams> {
        self.gradual_move_mouse_raw(x, y).unwrap();
        vec![]
    }

    #[cfg(all(target_os = "linux", not(feature = "enigo-always"), not(feature = "use-tfc"), not(feature = "use-hidg")))]
    #[inline]
    pub fn gradual_move_mouse(&mut self, x: OS_Input_Coord, y: OS_Input_Coord) -> Result<()> {
        self.gradual_move_mouse_raw(x, y)
    }

    #[cfg(any(target_os = "windows", feature = "use-tfc", feature = "enigo-always", feature = "use-hidg"))]
    #[inline]
    pub fn gradual_move_mouse(&mut self, x: OS_Input_Coord, y: OS_Input_Coord) -> Result<()> {
        let gradual_move = GradualMove::calculate(x, y);

        for _ in 0..gradual_move.both_move {
            self.move_mouse(gradual_move.x_direction, gradual_move.y_direction)?;
        }
        for _ in 0..gradual_move.move_only_x {
            self.move_mouse_x(gradual_move.x_direction)?;
        }
        for _ in 0..gradual_move.move_only_y {
            self.move_mouse_y(gradual_move.y_direction)?;
        }

        Ok(())
    }

    #[cfg(all(target_os = "linux", not(feature = "enigo-always"), not(feature = "use-tfc"), not(feature = "use-hidg")))]
    #[inline]
    pub fn gradual_move_mouse_raw(&mut self, x: OS_Input_Coord, y: OS_Input_Coord) -> Result<()> {
        exec_or_eyre!(self.virtual_device.gradual_move_mouse_raw(x, y))?;
        Ok(())
    }

    #[cfg(any(target_os = "windows", feature = "use-tfc", feature = "enigo-always", feature = "use-hidg"))]
    #[inline]
    pub fn gradual_move_mouse_raw(&mut self, x: OS_Input_Coord, y: OS_Input_Coord) -> Result<()> {
        self.gradual_move_mouse(x, y)
    }

    #[cfg(all(target_os = "linux", not(feature = "enigo-always"), not(feature = "use-tfc"), not(feature = "use-hidg")))]
    #[inline]
    pub fn scroll_raw_x(&mut self, value: OS_Input_Coord) -> Result<()> {
        // exec_or_eyre!(self.virtual_mouse.scroll_raw_x(value))?;
        exec_or_eyre!(self.virtual_device.scroll_raw_x(value))?;
        Ok(())
    }

    #[cfg(any(target_os = "windows", feature = "use-tfc", feature = "enigo-always", feature = "use-hidg"))]
    #[inline]
    pub fn scroll_raw_x(&mut self, value: OS_Input_Coord) -> Result<()> {
        self.scroll_x(value)
    }

    #[cfg(all(target_os = "linux", not(feature = "enigo-always"), not(feature = "use-tfc"), not(feature = "use-hidg")))]
    #[inline]
    pub fn scroll_raw_y(&mut self, value: OS_Input_Coord) -> Result<()> {
        // exec_or_eyre!(self.virtual_mouse.scroll_raw_y(value))?;
        exec_or_eyre!(self.virtual_device.scroll_raw_y(value))?;
        Ok(())
    }

    #[cfg(any(target_os = "windows", feature = "use-tfc", feature = "enigo-always", feature = "use-hidg"))]
    #[inline]
    pub fn scroll_raw_y(&mut self, value: OS_Input_Coord) -> Result<()> {
        self.scroll_y(value)
    }

    #[cfg(all(target_os = "linux", not(feature = "enigo-always"), not(feature = "use-tfc"), not(feature = "use-hidg")))]
    #[inline]
    pub fn buffered_scroll_x(&mut self, x: OS_Input_Coord) -> Vec<EventParams> {
        self.virtual_device.buffered_scroll_x(x)
    }

    #[cfg(any(target_os = "windows", feature = "use-tfc", feature = "enigo-always", feature = "use-hidg"))]
    #[inline]
    pub fn buffered_scroll_x(&mut self, x: OS_Input_Coord) -> Vec<EventParams> {
        self.scroll_x(x).unwrap();
        vec![]
    }

    #[cfg(all(target_os = "linux", not(feature = "enigo-always"), not(feature = "use-tfc"), not(feature = "use-hidg")))]
    #[inline]
    pub fn buffered_scroll_y(&mut self, y: OS_Input_Coord) -> Vec<EventParams> {
        self.virtual_device.buffered_scroll_y(y)
    }

    #[cfg(any(target_os = "windows", feature = "use-tfc", feature = "enigo-always", feature = "use-hidg"))]
    #[inline]
    pub fn buffered_scroll_y(&mut self, y: OS_Input_Coord) -> Vec<EventParams> {
        self.scroll_y(y).unwrap();
        vec![]
    }

    #[cfg(all(target_os = "linux", not(feature = "enigo-always"), not(feature = "use-tfc"), not(feature = "use-hidg")))]
    #[inline]
    pub fn buffered_gradual_scroll(&mut self, x: OS_Input_Coord, y: OS_Input_Coord) -> Vec<EventParams> {
        self.virtual_device.buffered_gradual_scroll(x, y)
    }

    #[cfg(any(target_os = "windows", feature = "use-tfc", feature = "enigo-always", feature = "use-hidg"))]
    #[inline]
    pub fn buffered_gradual_scroll(&mut self, x: OS_Input_Coord, y: OS_Input_Coord) -> Vec<EventParams> {
        self.gradual_scroll(x, y).unwrap();
        vec![]
    }

    #[cfg(all(target_os = "linux", not(feature = "enigo-always"), not(feature = "use-tfc"), not(feature = "use-hidg")))]
    #[inline]
    pub fn gradual_scroll(&mut self, x: OS_Input_Coord, y: OS_Input_Coord) -> Result<()> {
        self.gradual_scroll_raw(x, y)
    }

    #[cfg(any(target_os = "windows", feature = "use-tfc", feature = "enigo-always", feature = "use-hidg"))]
    #[inline]
    pub fn gradual_scroll(&mut self, x: OS_Input_Coord, y: OS_Input_Coord) -> Result<()> {
        let gradual_scroll = GradualMove::calculate(x, y);

        for _ in 0..gradual_scroll.both_move {
            self.scroll_x(gradual_scroll.x_direction)?;
            self.scroll_y(gradual_scroll.y_direction)?;
        }
        for _ in 0..gradual_scroll.move_only_x {
            self.scroll_x(gradual_scroll.x_direction)?;
        }
        for _ in 0..gradual_scroll.move_only_y {
            self.scroll_y(gradual_scroll.y_direction)?;
        }

        Ok(())
    }

    #[cfg(all(target_os = "linux", not(feature = "enigo-always"), not(feature = "use-tfc"), not(feature = "use-hidg")))]
    #[inline]
    pub fn gradual_scroll_raw(&mut self, x: OS_Input_Coord, y: OS_Input_Coord) -> Result<()> {
        exec_or_eyre!(self.virtual_device.gradual_scroll_raw(x, y))?;
        Ok(())
    }

    #[cfg(any(target_os = "windows", feature = "use-tfc", feature = "enigo-always", feature = "use-hidg"))]
    #[inline]
    pub fn gradual_scroll_raw(&mut self, x: OS_Input_Coord, y: OS_Input_Coord) -> Result<()> {
        self.gradual_scroll(x, y)
    }

    #[cfg(all(target_os = "linux", not(feature = "enigo-always"), not(feature = "use-tfc"), not(feature = "use-hidg")))]
    #[inline]
    pub fn buffered_press(&mut self, key_code: KeyCode) -> Result<Vec<EventParams>> {
        let button = key_code.convert()?;
        Ok(self.virtual_device.buffered_press(button))
    }

    #[cfg(any(target_os = "windows", feature = "use-tfc", feature = "enigo-always", feature = "use-hidg"))]
    #[inline]
    pub fn buffered_press(&mut self, key_code: KeyCode) -> Result<Vec<EventParams>> {
        self.press(key_code)?;
        Ok(vec![])
    }

    #[cfg(all(target_os = "linux", not(feature = "enigo-always"), not(feature = "use-tfc"), not(feature = "use-hidg")))]
    #[inline]
    pub fn buffered_release(&mut self, key_code: KeyCode) -> Result<Vec<EventParams>> {
        let button = key_code.convert()?;
        Ok(self.virtual_device.buffered_release(button))
    }

    #[cfg(any(target_os = "windows", feature = "use-tfc", feature = "enigo-always", feature = "use-hidg"))]
    #[inline]
    pub fn buffered_release(&mut self, key_code: KeyCode) -> Result<Vec<EventParams>> {
        self.release(key_code)?;
        Ok(vec![])
    }


    // LIB SPECIFIC

    #[cfg(all(target_os = "linux", not(feature = "enigo-always"), not(feature = "use-tfc"), not(feature = "use-hidg")))]
    #[inline]
    pub fn scroll_x(&mut self, value: OS_Input_Coord) -> Result<()> {
        // exec_or_eyre!(self.virtual_mouse.scroll_x(value))?;
        exec_or_eyre!(self.virtual_device.scroll_x(value))?;
        Ok(())
    }

    #[cfg(all(target_os = "linux", not(feature = "enigo-always"), not(feature = "use-tfc"), not(feature = "use-hidg")))]
    #[inline]
    pub fn scroll_y(&mut self, value: OS_Input_Coord) -> Result<()> {
        // exec_or_eyre!(self.virtual_mouse.scroll_y(value))?;
        exec_or_eyre!(self.virtual_device.scroll_y(value))?;
        Ok(())
    }
    
    #[cfg(all(target_os = "linux", not(feature = "enigo-always"), not(feature = "use-tfc"), not(feature = "use-hidg")))]
    #[inline]
    pub fn move_mouse_x(&mut self, x: OS_Input_Coord) -> Result<()> {
        // exec_or_eyre!(self.virtual_mouse.move_mouse_x(x))?;
        exec_or_eyre!(self.virtual_device.move_mouse_x(x))?;
        Ok(())
    }

    #[cfg(all(target_os = "linux", not(feature = "enigo-always"), not(feature = "use-tfc"), not(feature = "use-hidg")))]
    #[inline]
    pub fn move_mouse_y(&mut self, y: OS_Input_Coord) -> Result<()> {
        // exec_or_eyre!(self.virtual_mouse.move_mouse_y(y))?;
        exec_or_eyre!(self.virtual_device.move_mouse_y(y))?;
        Ok(())
    }

    #[cfg(all(target_os = "linux", not(feature = "enigo-always"), not(feature = "use-tfc"), not(feature = "use-hidg")))]
    #[inline]
    pub fn move_mouse(&mut self, x: OS_Input_Coord, y: OS_Input_Coord) -> Result<()> {
        // exec_or_eyre!(self.virtual_mouse.move_mouse(x, y))?;
        exec_or_eyre!(self.virtual_device.move_mouse(x, y))?;
        Ok(())
    }

    #[cfg(feature = "use-tfc")]
    #[inline]
    pub fn move_mouse_x(&mut self, x: OS_Input_Coord) -> Result<()> {
        exec_or_eyre!(self.ctx.mouse_move_rel(x, 0))?;
        Ok(())
    }

    #[cfg(feature = "use-tfc")]
    #[inline]
    pub fn move_mouse_y(&mut self, y: OS_Input_Coord) -> Result<()> {
        exec_or_eyre!(self.ctx.mouse_move_rel(0, -y))?;
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

    #[cfg(all(target_os = "linux", not(feature = "enigo-always"), not(feature = "use-tfc"), not(feature = "use-hidg")))]
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

    #[cfg(all(target_os = "linux", not(feature = "enigo-always"), not(feature = "use-tfc"), not(feature = "use-hidg")))]
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