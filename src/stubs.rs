use color_eyre::eyre::bail;
use color_eyre::{Report, Result};
use crate::{exec_or_eyre, KeyCode, OS_Input_Coord};

use crate::utils::GradualMove;
use crate::InputEmulator;

pub type EventParams = (u16, u16, i32);

impl InputEmulator {
    #[cfg(all(not(feature = "use-mki"), not(feature = "use-hidg")))]
    #[inline]
    pub fn finish_operation_mouse(&mut self) -> Result<()> {
        Ok(())
    }

    #[cfg(all(not(feature = "use-mki"), not(feature = "use-hidg")))]
    #[inline]
    pub fn finish_operation_keyboard(&mut self) -> Result<()> {
        Ok(())
    }

    #[cfg(not(feature = "use-mki"))]
    #[inline]
    pub fn write_buffer(&mut self, buffer: Vec<EventParams>) -> Result<()> {
        Ok(())
    }

    // #[cfg(all(not(feature = "use-mki"), not(feature = "use-hidg")))]
    #[cfg(not(feature = "use-mki"))]
    #[inline]
    pub fn move_mouse_raw_x(&mut self, x: OS_Input_Coord) -> Result<()> {
        self.move_mouse_x(x)
    }

    // #[cfg(all(not(feature = "use-mki"), not(feature = "use-hidg")))]
    #[cfg(not(feature = "use-mki"))]
    #[inline]
    pub fn move_mouse_raw_y(&mut self, y: OS_Input_Coord) -> Result<()> {
        self.move_mouse_y(y)
    }

    // #[cfg(all(not(feature = "use-mki"), not(feature = "use-hidg")))]
    #[cfg(not(feature = "use-mki"))]
    #[inline]
    pub fn move_mouse_raw(&mut self, x: OS_Input_Coord, y: OS_Input_Coord) -> Result<()> {
        self.move_mouse(x, y)
    }

    #[cfg(not(feature = "use-mki"))]
    #[inline]
    pub fn buffered_move_mouse_x(&mut self, x: OS_Input_Coord) -> Vec<EventParams> {
        self.move_mouse_x(x).unwrap();
        vec![]
    }

    #[cfg(not(feature = "use-mki"))]
    #[inline]
    pub fn buffered_move_mouse_y(&mut self, y: OS_Input_Coord) -> Vec<EventParams> {
        self.move_mouse_y(y).unwrap();
        vec![]
    }

    #[cfg(not(feature = "use-mki"))]
    #[inline]
    pub fn buffered_move_mouse(&mut self, x: OS_Input_Coord, y: OS_Input_Coord) -> Vec<EventParams> {
        self.move_mouse(x, y).unwrap();
        vec![]
    }

    #[cfg(not(feature = "use-mki"))]
    #[inline]
    pub fn buffered_gradual_move_mouse(&mut self, x: OS_Input_Coord, y: OS_Input_Coord) -> Vec<EventParams> {
        self.gradual_move_mouse_raw(x, y).unwrap();
        vec![]
    }

    // #[cfg(not(feature = "use-mki"))]
    // #[inline]
    // pub fn gradual_move_mouse_raw(&mut self, x: OS_Input_Coord, y: OS_Input_Coord) -> Result<()> {
    //     let gradual_move = GradualMove::calculate(x, y);
    //
    //     for _ in 0..gradual_move.both_move {
    //         self.move_mouse(gradual_move.x_direction, gradual_move.y_direction)?;
    //     }
    //     for _ in 0..gradual_move.move_only_x {
    //         self.move_mouse_x(gradual_move.x_direction)?;
    //     }
    //     for _ in 0..gradual_move.move_only_y {
    //         self.move_mouse_y(gradual_move.y_direction)?;
    //     }
    //
    //     Ok(())
    // }

    #[cfg(not(feature = "use-mki"))]
    #[inline]
    pub fn gradual_move_mouse_raw(&mut self, x: OS_Input_Coord, y: OS_Input_Coord) -> Result<()> {
        let gradual_move = GradualMove::calculate(x, y);

        for _ in 0..gradual_move.both_move {
            self.move_mouse_raw(gradual_move.x_direction, gradual_move.y_direction)?;
        }
        for _ in 0..gradual_move.move_only_x {
            self.move_mouse_raw_x(gradual_move.x_direction)?;
        }
        for _ in 0..gradual_move.move_only_y {
            self.move_mouse_raw_y(gradual_move.y_direction)?;
        }

        self.finish_operation_mouse()?;

        Ok(())
    }

    #[cfg(not(feature = "use-mki"))]
    #[inline]
    pub fn gradual_move_mouse(&mut self, x: OS_Input_Coord, y: OS_Input_Coord) -> Result<()> {
        self.gradual_move_mouse_raw(x, y)
    }

    // #[cfg(all(not(feature = "use-mki"), not(feature = "use-hidg")))]
    #[cfg(not(feature = "use-mki"))]
    #[inline]
    pub fn scroll_raw_x(&mut self, value: OS_Input_Coord) -> Result<()> {
        self.scroll_x(value)
    }

    // #[cfg(all(not(feature = "use-mki"), not(feature = "use-hidg")))]
    #[cfg(not(feature = "use-mki"))]
    #[inline]
    pub fn scroll_raw_y(&mut self, value: OS_Input_Coord) -> Result<()> {
        self.scroll_y(value)
    }

    #[cfg(not(feature = "use-mki"))]
    #[inline]
    pub fn buffered_scroll_x(&mut self, x: OS_Input_Coord) -> Vec<EventParams> {
        self.scroll_x(x).unwrap();
        vec![]
    }

    #[cfg(not(feature = "use-mki"))]
    #[inline]
    pub fn buffered_scroll_y(&mut self, y: OS_Input_Coord) -> Vec<EventParams> {
        self.scroll_y(y).unwrap();
        vec![]
    }

    #[cfg(not(feature = "use-mki"))]
    #[inline]
    pub fn buffered_gradual_scroll(&mut self, x: OS_Input_Coord, y: OS_Input_Coord) -> Vec<EventParams> {
        self.gradual_scroll(x, y).unwrap();
        vec![]
    }

    // #[cfg(not(feature = "use-mki"))]
    // #[inline]
    // pub fn gradual_scroll_raw(&mut self, x: OS_Input_Coord, y: OS_Input_Coord) -> Result<()> {
    //     let gradual_scroll = GradualMove::calculate(x, y);
    //
    //     for _ in 0..gradual_scroll.both_move {
    //         self.scroll_x(gradual_scroll.x_direction)?;
    //         self.scroll_y(gradual_scroll.y_direction)?;
    //     }
    //     for _ in 0..gradual_scroll.move_only_x {
    //         self.scroll_x(gradual_scroll.x_direction)?;
    //     }
    //     for _ in 0..gradual_scroll.move_only_y {
    //         self.scroll_y(gradual_scroll.y_direction)?;
    //     }
    //
    //     Ok(())
    // }

    #[cfg(not(feature = "use-mki"))]
    #[inline]
    pub fn gradual_scroll_raw(&mut self, x: OS_Input_Coord, y: OS_Input_Coord) -> Result<()> {
        let gradual_scroll = GradualMove::calculate(x, y);

        for _ in 0..gradual_scroll.both_move {
            self.scroll_raw_x(gradual_scroll.x_direction)?;
            self.scroll_raw_y(gradual_scroll.y_direction)?;
        }
        for _ in 0..gradual_scroll.move_only_x {
            self.scroll_raw_x(gradual_scroll.x_direction)?;
        }
        for _ in 0..gradual_scroll.move_only_y {
            self.scroll_raw_y(gradual_scroll.y_direction)?;
        }

        self.finish_operation_mouse()?;

        Ok(())
    }

    #[cfg(not(feature = "use-mki"))]
    #[inline]
    pub fn gradual_scroll(&mut self, x: OS_Input_Coord, y: OS_Input_Coord) -> Result<()> {
        self.gradual_scroll_raw(x, y)
    }

    #[cfg(not(feature = "use-mki"))]
    #[inline]
    pub fn buffered_press(&mut self, key_code: KeyCode) -> Result<Vec<EventParams>> {
        self.press(key_code)?;
        Ok(vec![])
    }

    #[cfg(not(feature = "use-mki"))]
    #[inline]
    pub fn buffered_release(&mut self, key_code: KeyCode) -> Result<Vec<EventParams>> {
        self.release(key_code)?;
        Ok(vec![])
    }
}