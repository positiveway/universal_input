use color_eyre::eyre::bail;
use color_eyre::{Report, Result};
use crate::{exec_or_eyre, KeyCode, OS_Input_Coord};

#[cfg(feature = "use-mki")]
use mouse_keyboard_input::{EventParams, key_codes, VirtualDevice, Button};

#[cfg(feature = "use-mki")]
pub struct InputEmulator {
    // virtual_mouse: VirtualDevice,
    // virtual_keyboard: VirtualDevice,
    pub virtual_device: VirtualDevice,
}

#[cfg(feature = "use-mki")]
impl InputEmulator {
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

    // Unique methods

    #[inline]
    pub fn finish_operation_mouse(&mut self) -> Result<()> {
        // exec_or_eyre!(self.virtual_mouse.synchronize())?;
        exec_or_eyre!(self.virtual_device.synchronize())?;
        Ok(())
    }

    #[inline]
    pub fn finish_operation_keyboard(&mut self) -> Result<()> {
        // exec_or_eyre!(self.virtual_keyboard.synchronize())?;
        exec_or_eyre!(self.virtual_device.synchronize())?;
        Ok(())
    }

    #[inline]
    pub fn write_buffer(&mut self, buffer: &[EventParams]) -> Result<()> {
        exec_or_eyre!(self.virtual_device.write_batch(buffer))?;
        Ok(())
    }

    // #[inline]
    // pub fn finish_operation_keyboard(&mut self) -> Result<()> {
    //     exec_or_eyre!(self.virtual_keyboard.synchronize())?;
    //     Ok(())
    // }

    #[inline]
    pub fn move_mouse_raw_x(&mut self, x: OS_Input_Coord) -> Result<()> {
        // exec_or_eyre!(self.virtual_mouse.move_mouse_raw_x(x))?;
        exec_or_eyre!(self.virtual_device.move_mouse_raw_x(x))?;
        Ok(())
    }

    #[inline]
    pub fn move_mouse_raw_y(&mut self, y: OS_Input_Coord) -> Result<()> {
        // exec_or_eyre!(self.virtual_mouse.move_mouse_raw_y(y))?;
        exec_or_eyre!(self.virtual_device.move_mouse_raw_y(y))?;
        Ok(())
    }

    #[inline]
    pub fn move_mouse_raw(&mut self, x: OS_Input_Coord, y: OS_Input_Coord) -> Result<()> {
        // exec_or_eyre!(self.virtual_mouse.move_mouse_raw(x, y))?;
        exec_or_eyre!(self.virtual_device.move_mouse_raw(x, y))?;
        Ok(())
    }

    #[inline]
    pub fn buffered_move_mouse_x(&mut self, x: OS_Input_Coord) -> Vec<EventParams> {
        self.virtual_device.buffered_move_mouse_x(x)
    }

    #[inline]
    pub fn buffered_move_mouse_y(&mut self, y: OS_Input_Coord) -> Vec<EventParams> {
        self.virtual_device.buffered_move_mouse_y(y)
    }

    #[inline]
    pub fn buffered_move_mouse(&mut self, x: OS_Input_Coord, y: OS_Input_Coord) -> Vec<EventParams> {
        self.virtual_device.buffered_move_mouse(x, y)
    }

    #[inline]
    pub fn buffered_gradual_move_mouse(&mut self, x: OS_Input_Coord, y: OS_Input_Coord) -> Vec<EventParams> {
        self.virtual_device.buffered_gradual_move_mouse(x, y)
    }

    #[inline]
    pub fn gradual_move_mouse(&mut self, x: OS_Input_Coord, y: OS_Input_Coord) -> Result<()> {
        self.gradual_move_mouse_raw(x, y)
    }

    #[inline]
    pub fn gradual_move_mouse_raw(&mut self, x: OS_Input_Coord, y: OS_Input_Coord) -> Result<()> {
        exec_or_eyre!(self.virtual_device.gradual_move_mouse_raw(x, y))?;
        Ok(())
    }

    #[inline]
    pub fn scroll_raw_x(&mut self, value: OS_Input_Coord) -> Result<()> {
        // exec_or_eyre!(self.virtual_mouse.scroll_raw_x(value))?;
        exec_or_eyre!(self.virtual_device.scroll_raw_x(value))?;
        Ok(())
    }

    #[inline]
    pub fn scroll_raw_y(&mut self, value: OS_Input_Coord) -> Result<()> {
        // exec_or_eyre!(self.virtual_mouse.scroll_raw_y(value))?;
        exec_or_eyre!(self.virtual_device.scroll_raw_y(value))?;
        Ok(())
    }

    #[inline]
    pub fn buffered_scroll_x(&mut self, x: OS_Input_Coord) -> Vec<EventParams> {
        self.virtual_device.buffered_scroll_x(x)
    }

    #[inline]
    pub fn buffered_scroll_y(&mut self, y: OS_Input_Coord) -> Vec<EventParams> {
        self.virtual_device.buffered_scroll_y(y)
    }

    #[inline]
    pub fn buffered_gradual_scroll(&mut self, x: OS_Input_Coord, y: OS_Input_Coord) -> Vec<EventParams> {
        self.virtual_device.buffered_gradual_scroll(x, y)
    }

    #[inline]
    pub fn gradual_scroll(&mut self, x: OS_Input_Coord, y: OS_Input_Coord) -> Result<()> {
        self.gradual_scroll_raw(x, y)
    }

    #[inline]
    pub fn gradual_scroll_raw(&mut self, x: OS_Input_Coord, y: OS_Input_Coord) -> Result<()> {
        exec_or_eyre!(self.virtual_device.gradual_scroll_raw(x, y))?;
        Ok(())
    }

    #[inline]
    pub fn buffered_press(&mut self, key_code: KeyCode) -> Result<Vec<EventParams>> {
        let button = key_code.convert()?;
        Ok(self.virtual_device.buffered_press(button))
    }

    #[inline]
    pub fn buffered_release(&mut self, key_code: KeyCode) -> Result<Vec<EventParams>> {
        let button = key_code.convert()?;
        Ok(self.virtual_device.buffered_release(button))
    }

    // Common methods

    #[inline]
    pub fn move_mouse_x(&mut self, x: OS_Input_Coord) -> Result<()> {
        // exec_or_eyre!(self.virtual_mouse.move_mouse_x(x))?;
        exec_or_eyre!(self.virtual_device.move_mouse_x(x))?;
        Ok(())
    }

    #[inline]
    pub fn move_mouse_y(&mut self, y: OS_Input_Coord) -> Result<()> {
        // exec_or_eyre!(self.virtual_mouse.move_mouse_y(y))?;
        exec_or_eyre!(self.virtual_device.move_mouse_y(y))?;
        Ok(())
    }

    #[inline]
    pub fn move_mouse(&mut self, x: OS_Input_Coord, y: OS_Input_Coord) -> Result<()> {
        // exec_or_eyre!(self.virtual_mouse.move_mouse(x, y))?;
        exec_or_eyre!(self.virtual_device.move_mouse(x, y))?;
        Ok(())
    }

    #[inline]
    pub fn scroll_x(&mut self, value: OS_Input_Coord) -> Result<()> {
        // exec_or_eyre!(self.virtual_mouse.scroll_x(value))?;
        exec_or_eyre!(self.virtual_device.scroll_x(value))?;
        Ok(())
    }

    #[inline]
    pub fn scroll_y(&mut self, value: OS_Input_Coord) -> Result<()> {
        // exec_or_eyre!(self.virtual_mouse.scroll_y(value))?;
        exec_or_eyre!(self.virtual_device.scroll_y(value))?;
        Ok(())
    }

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
}

#[cfg(feature = "use-mki")]
impl KeyCode {
    pub fn convert(&self) -> Result<Button> {
        let result = match self {
            KeyCode::KEY_ESC => key_codes::KEY_ESC,
            KeyCode::KEY_1 => key_codes::KEY_1,
            KeyCode::KEY_2 => key_codes::KEY_2,
            KeyCode::KEY_3 => key_codes::KEY_3,
            KeyCode::KEY_4 => key_codes::KEY_4,
            KeyCode::KEY_5 => key_codes::KEY_5,
            KeyCode::KEY_6 => key_codes::KEY_6,
            KeyCode::KEY_7 => key_codes::KEY_7,
            KeyCode::KEY_8 => key_codes::KEY_8,
            KeyCode::KEY_9 => key_codes::KEY_9,
            KeyCode::KEY_10 => key_codes::KEY_10,
            KeyCode::KEY_MINUS => key_codes::KEY_MINUS,
            KeyCode::KEY_EQUAL => key_codes::KEY_EQUAL,
            KeyCode::KEY_BACKSPACE => key_codes::KEY_BACKSPACE,
            KeyCode::KEY_TAB => key_codes::KEY_TAB,
            KeyCode::KEY_Q => key_codes::KEY_Q,
            KeyCode::KEY_W => key_codes::KEY_W,
            KeyCode::KEY_E => key_codes::KEY_E,
            KeyCode::KEY_R => key_codes::KEY_R,
            KeyCode::KEY_T => key_codes::KEY_T,
            KeyCode::KEY_Y => key_codes::KEY_Y,
            KeyCode::KEY_U => key_codes::KEY_U,
            KeyCode::KEY_I => key_codes::KEY_I,
            KeyCode::KEY_O => key_codes::KEY_O,
            KeyCode::KEY_P => key_codes::KEY_P,
            KeyCode::KEY_LEFTBRACE => key_codes::KEY_LEFTBRACE,
            KeyCode::KEY_RIGHTBRACE => key_codes::KEY_RIGHTBRACE,
            KeyCode::KEY_ENTER => key_codes::KEY_ENTER,
            KeyCode::KEY_LEFTCTRL => key_codes::KEY_LEFTCTRL,
            KeyCode::KEY_A => key_codes::KEY_A,
            KeyCode::KEY_S => key_codes::KEY_S,
            KeyCode::KEY_D => key_codes::KEY_D,
            KeyCode::KEY_F => key_codes::KEY_F,
            KeyCode::KEY_G => key_codes::KEY_G,
            KeyCode::KEY_H => key_codes::KEY_H,
            KeyCode::KEY_J => key_codes::KEY_J,
            KeyCode::KEY_K => key_codes::KEY_K,
            KeyCode::KEY_L => key_codes::KEY_L,
            KeyCode::KEY_SEMICOLON => key_codes::KEY_SEMICOLON,
            KeyCode::KEY_APOSTROPHE => key_codes::KEY_APOSTROPHE,
            KeyCode::KEY_GRAVE => key_codes::KEY_GRAVE,
            KeyCode::KEY_LEFTSHIFT => key_codes::KEY_LEFTSHIFT,
            KeyCode::KEY_BACKSLASH => key_codes::KEY_BACKSLASH,
            KeyCode::KEY_Z => key_codes::KEY_Z,
            KeyCode::KEY_X => key_codes::KEY_X,
            KeyCode::KEY_C => key_codes::KEY_C,
            KeyCode::KEY_V => key_codes::KEY_V,
            KeyCode::KEY_B => key_codes::KEY_B,
            KeyCode::KEY_N => key_codes::KEY_N,
            KeyCode::KEY_M => key_codes::KEY_M,
            KeyCode::KEY_COMMA => key_codes::KEY_COMMA,
            KeyCode::KEY_DOT => key_codes::KEY_DOT,
            KeyCode::KEY_SLASH => key_codes::KEY_SLASH,
            KeyCode::KEY_RIGHTSHIFT => key_codes::KEY_RIGHTSHIFT,
            KeyCode::KEY_KPASTERISK => key_codes::KEY_KPASTERISK,
            KeyCode::KEY_LEFTALT => key_codes::KEY_LEFTALT,
            KeyCode::KEY_SPACE => key_codes::KEY_SPACE,
            KeyCode::KEY_CAPSLOCK => key_codes::KEY_CAPSLOCK,
            KeyCode::KEY_F1 => key_codes::KEY_F1,
            KeyCode::KEY_F2 => key_codes::KEY_F2,
            KeyCode::KEY_F3 => key_codes::KEY_F3,
            KeyCode::KEY_F4 => key_codes::KEY_F4,
            KeyCode::KEY_F5 => key_codes::KEY_F5,
            KeyCode::KEY_F6 => key_codes::KEY_F6,
            KeyCode::KEY_F7 => key_codes::KEY_F7,
            KeyCode::KEY_F8 => key_codes::KEY_F8,
            KeyCode::KEY_F9 => key_codes::KEY_F9,
            KeyCode::KEY_F10 => key_codes::KEY_F10,
            KeyCode::KEY_NUMLOCK => key_codes::KEY_NUMLOCK,
            KeyCode::KEY_SCROLLLOCK => key_codes::KEY_SCROLLLOCK,
            KeyCode::KEY_KP7 => key_codes::KEY_KP7,
            KeyCode::KEY_KP8 => key_codes::KEY_KP8,
            KeyCode::KEY_KP9 => key_codes::KEY_KP9,
            KeyCode::KEY_KPMINUS => key_codes::KEY_KPMINUS,
            KeyCode::KEY_KP4 => key_codes::KEY_KP4,
            KeyCode::KEY_KP5 => key_codes::KEY_KP5,
            KeyCode::KEY_KP6 => key_codes::KEY_KP6,
            KeyCode::KEY_KPPLUS => key_codes::KEY_KPPLUS,
            KeyCode::KEY_KP1 => key_codes::KEY_KP1,
            KeyCode::KEY_KP2 => key_codes::KEY_KP2,
            KeyCode::KEY_KP3 => key_codes::KEY_KP3,
            KeyCode::KEY_KP0 => key_codes::KEY_KP0,
            KeyCode::KEY_KPDOT => key_codes::KEY_KPDOT,
            KeyCode::KEY_ZENKAKUHANKAKU => key_codes::KEY_ZENKAKUHANKAKU,
            KeyCode::KEY_102ND => key_codes::KEY_102ND,
            KeyCode::KEY_F11 => key_codes::KEY_F11,
            KeyCode::KEY_F12 => key_codes::KEY_F12,
            KeyCode::KEY_RO => key_codes::KEY_RO,
            KeyCode::KEY_KATAKANA => key_codes::KEY_KATAKANA,
            KeyCode::KEY_HIRAGANA => key_codes::KEY_HIRAGANA,
            KeyCode::KEY_HENKAN => key_codes::KEY_HENKAN,
            KeyCode::KEY_KATAKANAHIRAGANA => key_codes::KEY_KATAKANAHIRAGANA,
            KeyCode::KEY_MUHENKAN => key_codes::KEY_MUHENKAN,
            KeyCode::KEY_KPJPCOMMA => key_codes::KEY_KPJPCOMMA,
            KeyCode::KEY_KPENTER => key_codes::KEY_KPENTER,
            KeyCode::KEY_RIGHTCTRL => key_codes::KEY_RIGHTCTRL,
            KeyCode::KEY_KPSLASH => key_codes::KEY_KPSLASH,
            KeyCode::KEY_SYSRQ => key_codes::KEY_SYSRQ,
            KeyCode::KEY_RIGHTALT => key_codes::KEY_RIGHTALT,
            KeyCode::KEY_LINEFEED => key_codes::KEY_LINEFEED,
            KeyCode::KEY_HOME => key_codes::KEY_HOME,
            KeyCode::KEY_UP => key_codes::KEY_UP,
            KeyCode::KEY_PAGEUP => key_codes::KEY_PAGEUP,
            KeyCode::KEY_LEFT => key_codes::KEY_LEFT,
            KeyCode::KEY_RIGHT => key_codes::KEY_RIGHT,
            KeyCode::KEY_END => key_codes::KEY_END,
            KeyCode::KEY_DOWN => key_codes::KEY_DOWN,
            KeyCode::KEY_PAGEDOWN => key_codes::KEY_PAGEDOWN,
            KeyCode::KEY_INSERT => key_codes::KEY_INSERT,
            KeyCode::KEY_DELETE => key_codes::KEY_DELETE,
            KeyCode::KEY_MACRO => key_codes::KEY_MACRO,
            KeyCode::KEY_MUTE => key_codes::KEY_MUTE,
            KeyCode::KEY_VOLUMEDOWN => key_codes::KEY_VOLUMEDOWN,
            KeyCode::KEY_VOLUMEUP => key_codes::KEY_VOLUMEUP,
            KeyCode::KEY_POWER => key_codes::KEY_POWER,
            KeyCode::KEY_KPEQUAL => key_codes::KEY_KPEQUAL,
            KeyCode::KEY_KPPLUSMINUS => key_codes::KEY_KPPLUSMINUS,
            KeyCode::KEY_PAUSE => key_codes::KEY_PAUSE,
            KeyCode::KEY_SCALE => key_codes::KEY_SCALE,
            KeyCode::KEY_KPCOMMA => key_codes::KEY_KPCOMMA,
            KeyCode::KEY_HANGEUL => key_codes::KEY_HANGEUL,
            KeyCode::KEY_HANJA => key_codes::KEY_HANJA,
            KeyCode::KEY_YEN => key_codes::KEY_YEN,
            KeyCode::KEY_LEFTMETA => key_codes::KEY_LEFTMETA,
            KeyCode::KEY_RIGHTMETA => key_codes::KEY_RIGHTMETA,
            KeyCode::KEY_COMPOSE => key_codes::KEY_COMPOSE,
            KeyCode::KEY_STOP => key_codes::KEY_STOP,
            KeyCode::KEY_AGAIN => key_codes::KEY_AGAIN,
            KeyCode::KEY_PROPS => key_codes::KEY_PROPS,
            KeyCode::KEY_UNDO => key_codes::KEY_UNDO,
            KeyCode::KEY_FRONT => key_codes::KEY_FRONT,
            KeyCode::KEY_COPY => key_codes::KEY_COPY,
            KeyCode::KEY_OPEN => key_codes::KEY_OPEN,
            KeyCode::KEY_PASTE => key_codes::KEY_PASTE,
            KeyCode::KEY_FIND => key_codes::KEY_FIND,
            KeyCode::KEY_CUT => key_codes::KEY_CUT,
            KeyCode::KEY_HELP => key_codes::KEY_HELP,
            KeyCode::KEY_MENU => key_codes::KEY_MENU,
            KeyCode::KEY_CALC => key_codes::KEY_CALC,
            KeyCode::KEY_SETUP => key_codes::KEY_SETUP,
            KeyCode::KEY_SLEEP => key_codes::KEY_SLEEP,
            KeyCode::KEY_WAKEUP => key_codes::KEY_WAKEUP,
            KeyCode::KEY_FILE => key_codes::KEY_FILE,
            KeyCode::KEY_SENDFILE => key_codes::KEY_SENDFILE,
            KeyCode::KEY_DELETEFILE => key_codes::KEY_DELETEFILE,
            KeyCode::KEY_XFER => key_codes::KEY_XFER,
            KeyCode::KEY_PROG1 => key_codes::KEY_PROG1,
            KeyCode::KEY_PROG2 => key_codes::KEY_PROG2,
            KeyCode::KEY_WWW => key_codes::KEY_WWW,
            KeyCode::KEY_MSDOS => key_codes::KEY_MSDOS,
            KeyCode::KEY_SCREENLOCK => key_codes::KEY_SCREENLOCK,
            KeyCode::KEY_ROTATE_DISPLAY => key_codes::KEY_ROTATE_DISPLAY,
            KeyCode::KEY_CYCLEWINDOWS => key_codes::KEY_CYCLEWINDOWS,
            KeyCode::KEY_MAIL => key_codes::KEY_MAIL,
            KeyCode::KEY_BOOKMARKS => key_codes::KEY_BOOKMARKS,
            KeyCode::KEY_COMPUTER => key_codes::KEY_COMPUTER,
            KeyCode::KEY_BACK => key_codes::KEY_BACK,
            KeyCode::KEY_FORWARD => key_codes::KEY_FORWARD,
            KeyCode::KEY_CLOSECD => key_codes::KEY_CLOSECD,
            KeyCode::KEY_EJECTCD => key_codes::KEY_EJECTCD,
            KeyCode::KEY_EJECTCLOSECD => key_codes::KEY_EJECTCLOSECD,
            KeyCode::KEY_NEXTSONG => key_codes::KEY_NEXTSONG,
            KeyCode::KEY_PLAYPAUSE => key_codes::KEY_PLAYPAUSE,
            KeyCode::KEY_PREVIOUSSONG => key_codes::KEY_PREVIOUSSONG,
            KeyCode::KEY_STOPCD => key_codes::KEY_STOPCD,
            KeyCode::KEY_RECORD => key_codes::KEY_RECORD,
            KeyCode::KEY_REWIND => key_codes::KEY_REWIND,
            KeyCode::KEY_PHONE => key_codes::KEY_PHONE,
            KeyCode::KEY_ISO => key_codes::KEY_ISO,
            KeyCode::KEY_CONFIG => key_codes::KEY_CONFIG,
            KeyCode::KEY_HOMEPAGE => key_codes::KEY_HOMEPAGE,
            KeyCode::KEY_REFRESH => key_codes::KEY_REFRESH,
            KeyCode::KEY_EXIT => key_codes::KEY_EXIT,
            KeyCode::KEY_MOVE => key_codes::KEY_MOVE,
            KeyCode::KEY_EDIT => key_codes::KEY_EDIT,
            KeyCode::KEY_SCROLLUP => key_codes::KEY_SCROLLUP,
            KeyCode::KEY_SCROLLDOWN => key_codes::KEY_SCROLLDOWN,
            KeyCode::KEY_KPLEFTPAREN => key_codes::KEY_KPLEFTPAREN,
            KeyCode::KEY_KPRIGHTPAREN => key_codes::KEY_KPRIGHTPAREN,
            KeyCode::KEY_NEW => key_codes::KEY_NEW,
            KeyCode::KEY_REDO => key_codes::KEY_REDO,
            KeyCode::MOUSE_LEFT => key_codes::BTN_LEFT,
            KeyCode::MOUSE_RIGHT => key_codes::BTN_RIGHT,
            KeyCode::MOUSE_MIDDLE => key_codes::BTN_MIDDLE,
            KeyCode::MOUSE_SIDE => key_codes::BTN_SIDE,
            KeyCode::MOUSE_EXTRA => key_codes::BTN_EXTRA,
            KeyCode::MOUSE_FORWARD => key_codes::BTN_FORWARD,
            KeyCode::MOUSE_BACK => key_codes::BTN_BACK,
            KeyCode::MOUSE_TASK => key_codes::BTN_TASK,
            value => bail!("No such key code: {value}"),
        };

        Ok(result)
    }
}