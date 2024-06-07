use color_eyre::eyre::bail;
use color_eyre::{Report, Result};
use crate::{exec_or_eyre, KeyCode, OS_Input_Coord};

#[cfg(feature = "use-tfc")]
use tfc::{Context, Error, traits::*, MouseButton, Key};

#[cfg(feature = "use-tfc")]
pub struct InputEmulator {
    ctx: Context,
}

#[cfg(feature = "use-tfc")]
impl InputEmulator {
    pub fn new() -> Result<Self> {
        Ok(Self{
            ctx: Context::new()?,
        })
    }

    #[inline]
    pub fn move_mouse_x(&mut self, x: OS_Input_Coord) -> Result<()> {
        exec_or_eyre!(self.ctx.mouse_move_rel(x, 0))?;
        Ok(())
    }

    #[inline]
    pub fn move_mouse_y(&mut self, y: OS_Input_Coord) -> Result<()> {
        exec_or_eyre!(self.ctx.mouse_move_rel(0, -y))?;
        Ok(())
    }

    #[inline]
    pub fn move_mouse(&mut self, x: OS_Input_Coord, y: OS_Input_Coord) -> Result<()> {
        exec_or_eyre!(self.ctx.mouse_move_rel(x, -y))?;
        Ok(())
    }

    #[inline]
    pub fn scroll_x(&mut self, value: OS_Input_Coord) -> Result<()> {
        exec_or_eyre!(self.ctx.mouse_scroll(value, 0))?;
        Ok(())
    }

    #[inline]
    pub fn scroll_y(&mut self, value: OS_Input_Coord) -> Result<()> {
        exec_or_eyre!(self.ctx.mouse_scroll(0, -value))?;
        Ok(())
    }

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
}

#[cfg(feature = "use-tfc")]
impl KeyCode {
    pub fn convert(&self) -> Result<Key> {
        let result = match self {
            KeyCode::KEY_ESC => Key::Escape,
            KeyCode::KEY_1 => Key::N1,
            KeyCode::KEY_2 => Key::N2,
            KeyCode::KEY_3 => Key::N3,
            KeyCode::KEY_4 => Key::N4,
            KeyCode::KEY_5 => Key::N5,
            KeyCode::KEY_6 => Key::N6,
            KeyCode::KEY_7 => Key::N7,
            KeyCode::KEY_8 => Key::N8,
            KeyCode::KEY_9 => Key::N9,
            KeyCode::KEY_10 => Key::N0,
            KeyCode::KEY_MINUS => Key::Minus,
            KeyCode::KEY_EQUAL => Key::Equal,
            KeyCode::KEY_BACKSPACE => Key::DeleteOrBackspace,
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
            KeyCode::KEY_LEFTBRACE => Key::LeftBracket,
            KeyCode::KEY_RIGHTBRACE => Key::RightBracket,
            KeyCode::KEY_ENTER => Key::ReturnOrEnter,
            KeyCode::KEY_LEFTCTRL => Key::Control,
            KeyCode::KEY_A => Key::A,
            KeyCode::KEY_S => Key::S,
            KeyCode::KEY_D => Key::D,
            KeyCode::KEY_F => Key::F,
            KeyCode::KEY_G => Key::G,
            KeyCode::KEY_H => Key::H,
            KeyCode::KEY_J => Key::J,
            KeyCode::KEY_K => Key::K,
            KeyCode::KEY_L => Key::L,
            KeyCode::KEY_SEMICOLON => Key::Semicolon,
            KeyCode::KEY_APOSTROPHE => Key::Quote,
            KeyCode::KEY_GRAVE => Key::Grave,
            KeyCode::KEY_LEFTSHIFT => Key::Shift,
            KeyCode::KEY_BACKSLASH => Key::Backslash,
            KeyCode::KEY_Z => Key::Z,
            KeyCode::KEY_X => Key::X,
            KeyCode::KEY_C => Key::C,
            KeyCode::KEY_V => Key::V,
            KeyCode::KEY_B => Key::B,
            KeyCode::KEY_N => Key::N,
            KeyCode::KEY_M => Key::M,
            KeyCode::KEY_COMMA => Key::Comma,
            KeyCode::KEY_DOT => Key::Period,
            KeyCode::KEY_SLASH => Key::Slash,
            KeyCode::KEY_RIGHTSHIFT => Key::RightShift,
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
            KeyCode::KEY_KP7 => Key::Numpad7,
            KeyCode::KEY_KP8 => Key::Numpad8,
            KeyCode::KEY_KP9 => Key::Numpad9,
            KeyCode::KEY_KPMINUS => Key::NumpadMinus,
            KeyCode::KEY_KP4 => Key::Numpad4,
            KeyCode::KEY_KP5 => Key::Numpad5,
            KeyCode::KEY_KP6 => Key::Numpad6,
            KeyCode::KEY_KPPLUS => Key::NumpadPlus,
            KeyCode::KEY_KP1 => Key::Numpad1,
            KeyCode::KEY_KP2 => Key::Numpad2,
            KeyCode::KEY_KP3 => Key::Numpad3,
            KeyCode::KEY_KP0 => Key::Numpad0,
            KeyCode::KEY_KPDOT => Key::NumpadDecimal,
            KeyCode::KEY_F11 => Key::F11,
            KeyCode::KEY_F12 => Key::F12,
            KeyCode::KEY_KPENTER => Key::NumpadEnter,
            KeyCode::KEY_RIGHTCTRL => Key::RightControl,
            KeyCode::KEY_KPSLASH => Key::NumpadDivide,
            KeyCode::KEY_RIGHTALT => Key::RightAlt,
            KeyCode::KEY_HOME => Key::Home,
            KeyCode::KEY_UP => Key::UpArrow,
            KeyCode::KEY_PAGEUP => Key::PageUp,
            KeyCode::KEY_LEFT => Key::LeftArrow,
            KeyCode::KEY_RIGHT => Key::RightArrow,
            KeyCode::KEY_END => Key::End,
            KeyCode::KEY_DOWN => Key::DownArrow,
            KeyCode::KEY_PAGEDOWN => Key::PageDown,
            KeyCode::KEY_INSERT => Key::Insert,
            KeyCode::KEY_DELETE => Key::ForwardDelete,
            KeyCode::KEY_MUTE => Key::Mute,
            KeyCode::KEY_VOLUMEDOWN => Key::VolumeDown,
            KeyCode::KEY_VOLUMEUP => Key::VolumeUp,
            KeyCode::KEY_KPEQUAL => Key::NumpadEquals,
            KeyCode::KEY_KPPLUSMINUS => Key::NumpadMinus,
            KeyCode::KEY_LEFTMETA => Key::Meta,
            KeyCode::KEY_RIGHTMETA => Key::RightMeta,
            value => bail!("No such key code: {value}"),
        };

        Ok(result)
    }
}