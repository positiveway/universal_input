use color_eyre::eyre::bail;
use color_eyre::{Report, Result};
use crate::{exec_or_eyre, KeyCode, OS_Input_Coord};

#[cfg(feature = "use_enigo")]
use enigo::{Enigo, Settings, Coordinate, Mouse, Keyboard, Axis, Button, Key};
#[cfg(feature = "use_enigo")]
use enigo::Direction::{Click, Press, Release};
#[cfg(feature = "use_enigo")]
use mouse_keyboard_input::key_codes;

#[cfg(feature = "use_enigo")]
pub struct InputEmulator {
    enigo: Enigo,
}

#[cfg(feature = "use_enigo")]
impl InputEmulator {
    pub fn new() -> Result<Self> {
        Ok(Self {
            enigo: exec_or_eyre!(Enigo::new(&Settings::default()))?
        })
    }

    #[inline]
    pub fn move_mouse_x(&mut self, x: OS_Input_Coord) -> Result<()> {
        exec_or_eyre!(self.enigo.move_mouse(x, 0, Coordinate::Rel))?;
        Ok(())
    }

    #[inline]
    pub fn move_mouse_y(&mut self, y: OS_Input_Coord) -> Result<()> {
        exec_or_eyre!(self.enigo.move_mouse(0, y, Coordinate::Rel))?;
        Ok(())
    }

    #[inline]
    pub fn move_mouse(&mut self, x: OS_Input_Coord, y: OS_Input_Coord) -> Result<()> {
        exec_or_eyre!(self.enigo.move_mouse(x, y, Coordinate::Rel))?;
        Ok(())
    }

    #[inline]
    pub fn scroll_x(&mut self, value: OS_Input_Coord) -> Result<()> {
        exec_or_eyre!(self.enigo.scroll(value, Axis::Horizontal))?;
        Ok(())
    }

    #[inline]
    pub fn scroll_y(&mut self, value: OS_Input_Coord) -> Result<()> {
        exec_or_eyre!(self.enigo.scroll(value, Axis::Vertical))?;
        Ok(())
    }

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

#[cfg(feature = "use_enigo")]
impl KeyCode {
    #[cfg(target_os = "linux")]
    pub fn convert(&self) -> Result<Key> {
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
            value => bail!("No such key code: {value}"),
        };

        Ok(Key::Other(result as u32))
    }

    #[cfg(target_os = "windows")]
    pub fn convert(&self) -> Result<Key> {
        let result = match self {
            KeyCode::KEY_ESC => Key::Escape,
            KeyCode::KEY_1 => Key::Num1,
            KeyCode::KEY_2 => Key::Num2,
            KeyCode::KEY_3 => Key::Num3,
            KeyCode::KEY_4 => Key::Num4,
            KeyCode::KEY_5 => Key::Num5,
            KeyCode::KEY_6 => Key::Num6,
            KeyCode::KEY_7 => Key::Num7,
            KeyCode::KEY_8 => Key::Num8,
            KeyCode::KEY_9 => Key::Num9,
            KeyCode::KEY_10 => Key::Num0,
            KeyCode::KEY_MINUS => Key::OEMMinus,
            KeyCode::KEY_EQUAL => Key::OEMNECEqual,
            KeyCode::KEY_BACKSPACE => Key::Backspace,
            KeyCode::KEY_TAB => Key::Tab,
            KeyCode::KEY_Q => Key::Q,
            KeyCode::KEY_W => Key::W,
            KeyCode::KEY_E => Key::E,
            KeyCode::KEY_R => Key::R,
            KeyCode::KEY_T => Key::T,
            KeyCode::KEY_Y => Key::Y,
            KeyCode::KEY_U => Key::U,
            KeyCode::KEY_I => Key::I,
            KeyCode::KEY_O => Key::O,
            KeyCode::KEY_P => Key::P,
            KeyCode::KEY_LEFTBRACE => Key::Unicode('['),
            KeyCode::KEY_RIGHTBRACE => Key::Unicode(']'),
            KeyCode::KEY_ENTER => Key::Return,
            KeyCode::KEY_LEFTCTRL => Key::LControl,
            KeyCode::KEY_A => Key::A,
            KeyCode::KEY_S => Key::S,
            KeyCode::KEY_D => Key::D,
            KeyCode::KEY_F => Key::F,
            KeyCode::KEY_G => Key::G,
            KeyCode::KEY_H => Key::H,
            KeyCode::KEY_J => Key::J,
            KeyCode::KEY_K => Key::K,
            KeyCode::KEY_L => Key::L,
            KeyCode::KEY_SEMICOLON => Key::Unicode(';'),
            KeyCode::KEY_APOSTROPHE => Key::Unicode('\''),
            KeyCode::KEY_GRAVE => Key::Unicode('`'),
            KeyCode::KEY_LEFTSHIFT => Key::LShift,
            KeyCode::KEY_BACKSLASH => Key::Unicode('\\'),
            KeyCode::KEY_Z => Key::Z,
            KeyCode::KEY_X => Key::X,
            KeyCode::KEY_C => Key::C,
            KeyCode::KEY_V => Key::V,
            KeyCode::KEY_B => Key::B,
            KeyCode::KEY_N => Key::N,
            KeyCode::KEY_M => Key::M,
            KeyCode::KEY_COMMA => Key::OEMComma,
            KeyCode::KEY_DOT => Key::OEMPeriod,
            KeyCode::KEY_SLASH => Key::Unicode('/'),
            KeyCode::KEY_RIGHTSHIFT => Key::RShift,
            KeyCode::KEY_LEFTALT => Key::Alt,
            KeyCode::KEY_SPACE => Key::Space,
            KeyCode::KEY_CAPSLOCK => Key::CapsLock,
            KeyCode::KEY_F1 => Key::F1,
            KeyCode::KEY_F2 => Key::F2,
            KeyCode::KEY_F3 => Key::F3,
            KeyCode::KEY_F4 => Key::F4,
            KeyCode::KEY_F5 => Key::F5,
            KeyCode::KEY_F6 => Key::F6,
            KeyCode::KEY_F7 => Key::F7,
            KeyCode::KEY_F8 => Key::F8,
            KeyCode::KEY_F9 => Key::F9,
            KeyCode::KEY_F10 => Key::F10,
            KeyCode::KEY_NUMLOCK => Key::Numlock,
            KeyCode::KEY_KP7 => Key::Numpad7,
            KeyCode::KEY_KP8 => Key::Numpad8,
            KeyCode::KEY_KP9 => Key::Numpad9,
            KeyCode::KEY_KPMINUS => Key::OEMMinus,
            KeyCode::KEY_KP4 => Key::Numpad4,
            KeyCode::KEY_KP5 => Key::Numpad5,
            KeyCode::KEY_KP6 => Key::Numpad6,
            KeyCode::KEY_KPPLUS => Key::OEMPlus,
            KeyCode::KEY_KP1 => Key::Numpad1,
            KeyCode::KEY_KP2 => Key::Numpad2,
            KeyCode::KEY_KP3 => Key::Numpad3,
            KeyCode::KEY_KP0 => Key::Numpad0,
            // KeyCode::KEY_KPDOT => Key::,
            KeyCode::KEY_F11 => Key::F11,
            KeyCode::KEY_F12 => Key::F12,
            KeyCode::KEY_RIGHTCTRL => Key::RControl,
            KeyCode::KEY_UP => Key::UpArrow,
            KeyCode::KEY_PAGEUP => Key::PageUp,
            KeyCode::KEY_LEFT => Key::LeftArrow,
            KeyCode::KEY_RIGHT => Key::RightArrow,
            KeyCode::KEY_END => Key::End,
            KeyCode::KEY_DOWN => Key::DownArrow,
            KeyCode::KEY_PAGEDOWN => Key::PageDown,
            KeyCode::KEY_INSERT => Key::Insert,
            KeyCode::KEY_DELETE => Key::Delete,
            KeyCode::KEY_LEFTMETA => Key::LWin,
            KeyCode::KEY_RIGHTMETA => Key::RWin,
            value => bail!("No such key code: {value}"),
        };

        Ok(result)
    }
}