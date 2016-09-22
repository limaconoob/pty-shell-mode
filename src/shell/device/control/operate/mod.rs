pub mod mouse;
pub mod key;

use super::In;
use ::libc;

pub use self::mouse::Mouse;
pub use self::key::Key;

#[derive(Clone, Copy, Debug)]
pub enum Operate {
  /// The mouse operate.
  Mouse(Mouse, libc::c_ushort, libc::c_ushort),
  /// The key operate.
  Key(Key),
}

fn info_arrow(buf: &In) -> (u16, u16)
{ let mut i = 0;
  let mut k: (u16, u16);
  k = (0, 0);
  buf.split(|cut| *cut == b';').all(|nbr|
  { if i == 0
    { i = 1; }
    else
    { for x in nbr
      { if *x < 58
        { k.0 = (k.0 * (10 as u16)) + (*x as u16) - 48; }
        else
        { k.1 = *x as u16; }}}
    true });
  k }

fn info_mouse(buf: &In) -> (u16, u16, u16)
{ let mut i = 0;
  let mut tup: (u16, u16, u16) = (0, 0, 0);
  buf.split(|cut| *cut == b';').all(|nbr|
  { if i == 0
    { for x in nbr
      { if *x < b'0' && *x > b'9'
        { tup.0 = (tup.0 * (10 as u16)) + (*x as u16) - 48; }}
      i = 1; }
    else if i == 1
    { for x in nbr
      { tup.1 = (tup.1 * (10 as u16)) + (*x as u16) - 48; }
      tup.1 -= 1;
      i = 2; }
    else
    { for x in nbr
      { if *x < b'0' && *x > b'9'
        { tup.2 = (tup.2 * (10 as u16)) + (*x as u16) - 48; }}
      tup.2 -= 1; }
    true });
  tup }

impl Operate {
  /// The constructor method `new` returns evaluated Operate.
  pub fn new(buf: &In) -> Self {
    match buf {
      &[b'\x1B', b'[', b'<', ..] => {
        let tmp: (u16, u16, u16) = info_mouse(buf);
        println!("Bouton::{}, Coord::({}, {})", tmp.0, tmp.1, tmp.2);
        match tmp.0 {

            ///Click
            0 => Operate::Mouse(Mouse::Left, tmp.1, tmp.2),
            1 => Operate::Mouse(Mouse::Right, tmp.1, tmp.2),
            2 => Operate::Mouse(Mouse::Wheel, tmp.1, tmp.2),
            64 => Operate::Mouse(Mouse::WheelUp, tmp.1, tmp.2),
            65 => Operate::Mouse(Mouse::WheelDown, tmp.1, tmp.2),

            ///Drag
            32 => Operate::Mouse(Mouse::LeftDrag, tmp.1, tmp.2),
            33 => Operate::Mouse(Mouse::WheelDrag, tmp.1, tmp.2),
            34 => Operate::Mouse(Mouse::RightDrag, tmp.1, tmp.2),

            ///Shift Click
            4 => Operate::Mouse(Mouse::ShiftLeft, tmp.1, tmp.2),
            5 => Operate::Mouse(Mouse::ShiftWheel, tmp.1, tmp.2),
            6 => Operate::Mouse(Mouse::ShiftRight, tmp.1, tmp.2),

            ///Shift Drag
            36 => Operate::Mouse(Mouse::ShiftLeftDrag, tmp.1, tmp.2),
            37 => Operate::Mouse(Mouse::ShiftWheelDrag, tmp.1, tmp.2),
            38 => Operate::Mouse(Mouse::ShiftRightDrag, tmp.1, tmp.2),

            ///Control Click
            16 => Operate::Mouse(Mouse::CtrlLeft, tmp.1, tmp.2),
            17 => Operate::Mouse(Mouse::CtrlWheel, tmp.1, tmp.2),
            18 => Operate::Mouse(Mouse::CtrlRight, tmp.1, tmp.2),
            80 => Operate::Mouse(Mouse::CtrlWheelUp, tmp.1, tmp.2),
            81 => Operate::Mouse(Mouse::CtrlWheelDown, tmp.1, tmp.2),

            ///Control Drag
            48 => Operate::Mouse(Mouse::CtrlLeftDrag, tmp.1, tmp.2),
            49 => Operate::Mouse(Mouse::CtrlWheelDrag, tmp.1, tmp.2),
            50 => Operate::Mouse(Mouse::CtrlRightDrag, tmp.1, tmp.2),

            ///Control Shift Click
            20 => Operate::Mouse(Mouse::ShiftCtrlLeft, tmp.1, tmp.2),
            21 => Operate::Mouse(Mouse::ShiftCtrlWheel, tmp.1, tmp.2),
            22 => Operate::Mouse(Mouse::ShiftCtrlRight, tmp.1, tmp.2),

            ///Control Shift Drag
            52 => Operate::Mouse(Mouse::ShiftCtrlLeftDrag, tmp.1, tmp.2),
            53 => Operate::Mouse(Mouse::ShiftCtrlWheelDrag, tmp.1, tmp.2),
            54 => Operate::Mouse(Mouse::ShiftCtrlRightDrag, tmp.1, tmp.2),

            ///Command Click
            8 => Operate::Mouse(Mouse::CmdLeft, tmp.1, tmp.2),
            9 => Operate::Mouse(Mouse::CmdWheel, tmp.1, tmp.2),
            10 => Operate::Mouse(Mouse::CmdRight, tmp.1, tmp.2),
            72 => Operate::Mouse(Mouse::CmdWheelUp, tmp.1, tmp.2),
            73 => Operate::Mouse(Mouse::CmdWheelDown, tmp.1, tmp.2),

            ///Command Drag
            40 => Operate::Mouse(Mouse::CmdLeftDrag, tmp.1, tmp.2),
            41 => Operate::Mouse(Mouse::CmdWheelDrag, tmp.1, tmp.2),
            42 => Operate::Mouse(Mouse::CmdRightDrag, tmp.1, tmp.2),

            ///Command Shift Click
            12 => Operate::Mouse(Mouse::CmdShiftLeft, tmp.1, tmp.2),
            13 => Operate::Mouse(Mouse::CmdShiftWheel, tmp.1, tmp.2),
            14 => Operate::Mouse(Mouse::CmdShiftRight, tmp.1, tmp.2),

            ///Command Shift Drag
            44 => Operate::Mouse(Mouse::CmdShiftLeftDrag, tmp.1, tmp.2),
            45 => Operate::Mouse(Mouse::CmdShiftWheelDrag, tmp.1, tmp.2),
            46 => Operate::Mouse(Mouse::CmdShiftRightDrag, tmp.1, tmp.2),

            ///Command Control Click
            24 => Operate::Mouse(Mouse::CmdCtrlLeft, tmp.1, tmp.2),
            25 => Operate::Mouse(Mouse::CmdCtrlWheel, tmp.1, tmp.2),
            26 => Operate::Mouse(Mouse::CmdCtrlRight, tmp.1, tmp.2),
            88 => Operate::Mouse(Mouse::CmdWheelUp, tmp.1, tmp.2),
            89 => Operate::Mouse(Mouse::CmdWheelDown, tmp.1, tmp.2),

            ///Command Control Drag
            56 => Operate::Mouse(Mouse::CmdCtrlLeftDrag, tmp.1, tmp.2),
            57 => Operate::Mouse(Mouse::CmdCtrlWheelDrag, tmp.1, tmp.2),
            58 => Operate::Mouse(Mouse::CmdCtrlRightDrag, tmp.1, tmp.2),

            ///Command Shift Control Click
            28 => Operate::Mouse(Mouse::CmdShiftCtrlLeft, tmp.1, tmp.2),
            29 => Operate::Mouse(Mouse::CmdShiftCtrlWheel, tmp.1, tmp.2),
            30 => Operate::Mouse(Mouse::CmdShiftCtrlRight, tmp.1, tmp.2),

            ///Command Shift Control Drag
            60 => Operate::Mouse(Mouse::CmdShiftCtrlLeftDrag, tmp.1, tmp.2),
            61 => Operate::Mouse(Mouse::CmdShiftCtrlWheelDrag, tmp.1, tmp.2),
            62 => Operate::Mouse(Mouse::CmdShiftCtrlRightDrag, tmp.1, tmp.2),

            _ => unimplemented!(),
        }
      }
      
      &[b'\r', b'\0', ..] | &[b'\n', b'\0', ..] | 
      &[b'\n', b'\r', b'\0', ..] => Operate::Key(Key::Enter),

      &[b'\t', b'\0', ..] => Operate::Key(Key::Tab),
      &[27, b'\0', ..] => Operate::Key(Key::Esc),
      &[127, b'\0', ..] => Operate::Key(Key::Backspace),
      
      &[c, b'\0', ..] => {
        if c > 0 && c < 27
        { Operate::Key(Key::Ctrl(c + b'a' - 0x01)) }
        else if c > 27 && c < 32
        { Operate::Key(Key::Ctrl(c + b'4' - 0x1C)) }
        else
        { Operate::Key(Key::Char(c)) }
      }

      &[27, b'[', c, b'\0', ..] => {
        match c {
          b'A' => Operate::Key(Key::Up),
          b'B' => Operate::Key(Key::Down),
          b'C' => Operate::Key(Key::Right),
          b'D' => Operate::Key(Key::Left),
          b'F' => Operate::Key(Key::End),
          _ => unimplemented!(),
        }
      }

      &[27, b'[', b'1', b';', ..] => {
        let c: (u16, u16) = info_arrow(buf);
        match c.0 {
          2 => {
            match c.1 {
              b'A' => Operate::Key(Key::ShiftUp),
              b'B' => Operate::Key(Key::ShiftDown),
              b'C' => Operate::Key(Key::ShiftRight),
              b'D' => Operate::Key(Key::ShiftLeft),
              b'P' => Operate::Key(Key::F(13)),
              b'Q' => Operate::Key(Key::F(14)),
              b'R' => Operate::Key(Key::F(15)),
              b'S' => Operate::Key(Key::F(16)),
              _ => unimplemented!(),
            }
          }
          9 => {
            match c.1 {
              b'A' => Operate::Key(Key::AltUp),
              b'B' => Operate::Key(Key::AltDown),
              b'C' => Operate::Key(Key::AltRight),
              b'D' => Operate::Key(Key::AltLeft),
              _ => unimplemented!(),
            }
          }
          5 => {
            match c.1 {
              b'A' => Operate::Key(Key::CtrlUp),
              b'B' => Operate::Key(Key::CtrlDown),
              b'C' => Operate::Key(Key::CtrlRight),
              b'D' => Operate::Key(Key::CtrlLeft),
              _ => unimplemented!(),
            }
          }
          10 => {
            match c.1 {
              b'A' => Operate::Key(Key::AltShiftUp),
              b'B' => Operate::Key(Key::AltShiftDown),
              b'C' => Operate::Key(Key::AltShiftRight),
              b'D' => Operate::Key(Key::AltShiftLeft),
              _ => unimplemented!(),
            }
          }
          6 => {
            match c.1 {
              b'A' => Operate::Key(Key::CtrlShiftUp),
              b'B' => Operate::Key(Key::CtrlShiftDown),
              b'C' => Operate::Key(Key::CtrlShiftRight),
              b'D' => Operate::Key(Key::CtrlShiftLeft),
              _ => unimplemented!(),
            }
          }
          _ => unimplemented!(),
        }
      }
      
      &[27, b'[', c, b'~', b'\0', ..] => {
        match c {
          b'3' => Operate::Key(Key::Delete),
          b'5' => Operate::Key(Key::PageUp),
          b'6' => Operate::Key(Key::PageDown),
        }
      }

      &[27, b'O', c, b'\0', ..] => {
        match c {
          b'P' => Operate::Key(Key::F(1)),
          b'Q' => Operate::Key(Key::F(2)),
          b'R' => Operate::Key(Key::F(3)),
          b'S' => Operate::Key(Key::F(4)),
        }
      }

      &[27, b'[', c, d, b'~', b'\0', ..] => {
        match c {
          b'1' => {
            match d {
              b'5' => Operate::Key(Key::F(5)),
              b'7' => Operate::Key(Key::F(6)),
              b'8' => Operate::Key(Key::F(7)),
              b'9' => Operate::Key(Key::F(8)),
            }
          }
          b'1' => {
            match d {
              b'0' => Operate::Key(Key::F(9)),
              b'1' => Operate::Key(Key::F(10)),
              b'3' => Operate::Key(Key::F(11)),
              b'4' => Operate::Key(Key::F(12)),
            }
          }
          _ => unimplemented!(),
        }
      }

      _ => unimplemented!(),
    }
  }

  /// The accessor method `is_mouse` returns a Option for the Mouse Operate.
  pub fn is_mouse(&self) -> Option<Mouse> {
    match *self {
      Operate::Mouse(mouse, _, _) => Some(mouse),
      Operate::Key(_) => None,
    }
  }

  /// The accessor method `is_key` returns a Option for the Key Operate.
  pub fn is_key(&self) -> Option<Key> {
    match *self {
      Operate::Key(key) => Some(key),
      Operate::Mouse(_, _, _) => None,
    }
  }
}
