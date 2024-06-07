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

    #[inline]
    pub fn finish_operation_mouse(&mut self) -> Result<()> {
        self.virtual_mouse.input(&self.mouse_input)?;
        Ok(())
    }

    #[inline]
    pub fn finish_operation_keyboard(&mut self) -> Result<()> {
        self.virtual_keyboard.input(&self.keyboard_input)?;
        Ok(())
    }

    // #[inline]
    // pub fn move_mouse_raw_x(&mut self, x: OS_Input_Coord) -> Result<()> {
    //     self.mouse_input.change_pointer((x as i16, 0), true);
    //     Ok(())
    // }
    //
    // #[inline]
    // pub fn move_mouse_raw_y(&mut self, y: OS_Input_Coord) -> Result<()> {
    //     self.mouse_input.change_pointer((0, y as i16), true);
    //     Ok(())
    // }
    //
    // #[inline]
    // pub fn move_raw_mouse(&mut self, x: OS_Input_Coord, y: OS_Input_Coord) -> Result<()> {
    //     self.mouse_input.change_pointer((x as i16, y as i16), true);
    //     Ok(())
    // }

    #[inline]
    pub fn move_mouse_x(&mut self, x: OS_Input_Coord) -> Result<()> {
        self.mouse_input.change_pointer((x as i16, 0), true);
        self.finish_operation_mouse()?;
        Ok(())
    }

    #[inline]
    pub fn move_mouse_y(&mut self, y: OS_Input_Coord) -> Result<()> {
        self.mouse_input.change_pointer((0, y as i16), true);
        self.finish_operation_mouse()?;
        Ok(())
    }

    #[inline]
    pub fn move_mouse(&mut self, x: OS_Input_Coord, y: OS_Input_Coord) -> Result<()> {
        self.mouse_input.change_pointer((x as i16, y as i16), true);
        self.finish_operation_mouse()?;
        Ok(())
    }

    // #[inline]
    // pub fn scroll_raw_x(&mut self, value: OS_Input_Coord) -> Result<()> {
    //     Ok(())
    // }
    //
    // #[inline]
    // pub fn scroll_raw_y(&mut self, value: OS_Input_Coord) -> Result<()> {
    //     self.mouse_input.change_wheel(value as i8, true);
    //     Ok(())
    // }

    #[inline]
    pub fn scroll_x(&mut self, value: OS_Input_Coord) -> Result<()> {
        Ok(())
    }

    #[inline]
    pub fn scroll_y(&mut self, value: OS_Input_Coord) -> Result<()> {
        self.mouse_input.change_wheel(value as i8, true);
        self.finish_operation_mouse()?;
        Ok(())
    }

    #[inline]
    pub fn press(&mut self, key_code: KeyCode) -> Result<()> {
        match key_code {
            KeyCode::MOUSE_LEFT => { self.mouse_input.press_button(Button::Primary) }
            KeyCode::MOUSE_RIGHT => { self.mouse_input.press_button(Button::Secondary) }
            KeyCode::MOUSE_MIDDLE => { self.mouse_input.press_button(Button::Tertiary) }
            _ => {
                let button = key_code.convert()?;
                self.keyboard_input.press_key(button);
            }
        };
        
        self.finish_operation_mouse()?;
        self.finish_operation_keyboard()?;
        
        Ok(())
    }

    #[inline]
    pub fn release(&mut self, key_code: KeyCode) -> Result<()> {
        match key_code {
            KeyCode::MOUSE_LEFT => { self.mouse_input.release_button(Button::Primary) }
            KeyCode::MOUSE_RIGHT => { self.mouse_input.release_button(Button::Secondary) }
            KeyCode::MOUSE_MIDDLE => { self.mouse_input.release_button(Button::Tertiary) }
            _ => {
                let button = key_code.convert()?;
                self.keyboard_input.release_key(button);
            }
        };

        self.finish_operation_mouse()?;
        self.finish_operation_keyboard()?;

        Ok(())
    }
}

#[cfg(feature = "use-hidg")]
impl KeyCode {
    pub fn convert(&self) -> Result<Key> {
        let result = match self {
            KeyCode::KEY_ESC => Key::Esc,
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
            KeyCode::KEY_MINUS => Key::Minus,
            KeyCode::KEY_EQUAL => Key::Equal,
            KeyCode::KEY_BACKSPACE => Key::BackSpace,
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
            KeyCode::KEY_LEFTBRACE => Key::LeftBrace,
            KeyCode::KEY_RIGHTBRACE => Key::RightBrace,
            KeyCode::KEY_ENTER => Key::Enter,
            KeyCode::KEY_LEFTCTRL => Key::LeftCtrl,
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
            KeyCode::KEY_APOSTROPHE => Key::Apostrophe,
            KeyCode::KEY_GRAVE => Key::Grave,
            KeyCode::KEY_LEFTSHIFT => Key::LeftShift,
            KeyCode::KEY_BACKSLASH => Key::BackSlash,
            KeyCode::KEY_Z => Key::Z,
            KeyCode::KEY_X => Key::X,
            KeyCode::KEY_C => Key::C,
            KeyCode::KEY_V => Key::V,
            KeyCode::KEY_B => Key::B,
            KeyCode::KEY_N => Key::N,
            KeyCode::KEY_M => Key::M,
            KeyCode::KEY_COMMA => Key::Comma,
            KeyCode::KEY_DOT => Key::Dot,
            KeyCode::KEY_SLASH => Key::Slash,
            KeyCode::KEY_RIGHTSHIFT => Key::RightShift,
            KeyCode::KEY_LEFTALT => Key::LeftAlt,
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
            KeyCode::KEY_KP7 => Key::KeyPad7,
            KeyCode::KEY_KP8 => Key::KeyPad8,
            KeyCode::KEY_KP9 => Key::KeyPad9,
            KeyCode::KEY_KPMINUS => Key::KeyPadMinus,
            KeyCode::KEY_KP4 => Key::KeyPad4,
            KeyCode::KEY_KP5 => Key::KeyPad5,
            KeyCode::KEY_KP6 => Key::KeyPad6,
            KeyCode::KEY_KPPLUS => Key::KeyPadPlus,
            KeyCode::KEY_KP1 => Key::KeyPad1,
            KeyCode::KEY_KP2 => Key::KeyPad2,
            KeyCode::KEY_KP3 => Key::KeyPad3,
            KeyCode::KEY_KP0 => Key::KeyPad0,
            KeyCode::KEY_KPDOT => Key::KeyPadDot,
            KeyCode::KEY_F11 => Key::F11,
            KeyCode::KEY_F12 => Key::F12,
            KeyCode::KEY_KPENTER => Key::KeyPadEnter,
            KeyCode::KEY_RIGHTCTRL => Key::RightCtrl,
            KeyCode::KEY_KPSLASH => Key::KeyPadSlash,
            KeyCode::KEY_RIGHTALT => Key::RightAlt,
            KeyCode::KEY_HOME => Key::Home,
            KeyCode::KEY_UP => Key::Up,
            KeyCode::KEY_PAGEUP => Key::PageUp,
            KeyCode::KEY_LEFT => Key::Left,
            KeyCode::KEY_RIGHT => Key::Right,
            KeyCode::KEY_END => Key::End,
            KeyCode::KEY_DOWN => Key::Down,
            KeyCode::KEY_PAGEDOWN => Key::PageDown,
            KeyCode::KEY_INSERT => Key::Insert,
            KeyCode::KEY_DELETE => Key::Delete,
            KeyCode::KEY_MUTE => Key::Mute,
            KeyCode::KEY_VOLUMEDOWN => Key::VolumeDown,
            KeyCode::KEY_VOLUMEUP => Key::VolumeUp,
            KeyCode::KEY_KPEQUAL => Key::KeyPadEqual,
            KeyCode::KEY_KPPLUSMINUS => Key::KeyPadMinus,
            KeyCode::KEY_LEFTMETA => Key::LeftMeta,
            KeyCode::KEY_RIGHTMETA => Key::RightMeta,
            value => bail!("No such key code: {value}"),
        };
        
        Ok(result)
    }
}