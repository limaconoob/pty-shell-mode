mod mouse;
mod key;

use ::libc;
use ::super::super::In;

pub use self::mouse::Mouse;
pub use self::key::Key;

#[derive(Copy, Clone)]
pub enum Operate {
  /// The mouse operate.
  Mouse(Mouse),
  /// The key operate.
  Key(Key),
}

impl Operate {
  /// The constructor method `new` returns evaluated Operate.
  pub fn new(buf: &In) -> Self {
    Operate::Mouse(Mouse::Left)
  }

  /// The accessor method `is_mouse` returns a Option for the Mouse Operate.
  pub fn is_mouse(&self) -> Option<Mouse> {
    match *self {
      Operate::Mouse(mouse) => Some(mouse),
      Operate::Key(_) => None,
    }
  }

  /// The accessor method `is_key` returns a Option for the Key Operate.
  pub fn is_key(&self) -> Option<Key> {
    match *self {
      Operate::Key(key) => Some(key),
      Operate::Mouse(_) => None,
    }
  }
}