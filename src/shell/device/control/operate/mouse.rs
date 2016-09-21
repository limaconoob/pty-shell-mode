#[derive(Clone, Copy, Debug)]
/// Note that the button "Command" is like the function button 
/// between Ctrl and Alt on Azerty keyboards
pub enum Mouse {
  /// The left mouse button is pressed.
  Left,
  /// The right mouse button is pressed.
  Right,
  /// The mouse wheel button is pressed.
  Wheel,
  /// The mouse wheel is going up.
  WheelUp,
  /// The mouse wheel is going down.
  WheelDown,
  /// The left mouse button is held while moving pointer.
  LeftDrag,
  /// The wheel mouse button is held while moving pointer.
  WheelDrag,
  /// The right mouse button is held while moving pointer.
  RightDrag,
  /// The left mouse button is pressed while helding Shift.
  ShiftLeft,
  /// The wheel mouse button is pressed while helding Shift.
  ShiftWheel,
  /// The right mouse button is pressed while helding Shift.
  ShiftRight,
  /// The left mouse button and Shift are held while moving pointer.
  ShiftLeftDrag,
  /// The wheel mouse button and Shift are held while moving pointer.
  ShiftWheelDrag,
  /// The right mouse button and Shift are held while moving pointer.
  ShiftRightDrag,
  /// The left mouse button is pressed while helding Ctrl
  CtrlLeft,
  /// The wheel mouse button is pressed while helding Ctrl
  CtrlWheel,
  /// The right mouse button is pressed while helding Ctrl 
  CtrlRight,
  /// The mouse wheel is going up while helding Ctrl 
  CtrlWheelUp,
  /// The mouse wheel is going down while helding Ctrl 
  CtrlWheelDown,
  /// The left mouse button and Ctrl are held while moving pointer
  CtrlLeftDrag,
  /// The wheel mouse button and Ctrl are held while moving pointer
  CtrlWheelDrag,
  /// The right mouse button and Ctrl are held while moving pointer
  CtrlRightDrag,
  /// The left mouse button is pressed while Ctrl and Shift are held
  ShiftCtrlLeft,
  /// The wheel mouse button is pressed while Ctrl and Shift are held
  ShiftCtrlWheel,
  /// The right mouse button is pressed while Ctrl and Shift are held
  ShiftCtrlRight,
  /// The left mouse button, Ctrl and Shift are held while moving pointer
  ShiftCtrlLeftDrag,
  /// The wheel mouse button, Ctrl and Shift are held while moving pointer
  ShiftCtrlWheelDrag,
  /// The right mouse button, Ctrl and Shift are held while moving pointer
  ShiftCtrlRightDrag,
  /// The left mouse button is pressed while helding Command
  CmdLeft,
  /// The wheel mouse button is pressed while helding Command
  CmdWheel,
  /// The right mouse button is pressed while helding Command
  CmdRight,
  /// The mouse wheel is going up while helding Command
  CmdWheelUp,
  /// The mouse wheel is going down while helding Command
  CmdWheelDown,
  /// The left mouse button and Command are held while moving pointer
  CmdLeftDrag,
  /// The wheel mouse button and Command are held while moving pointer
  CmdWheelDrag,
  /// The right mouse button and Command are held while moving pointer
  CmdRightDrag,
  /// The left mouse button is pressed while helding Command and Shift
  CmdShiftLeft,
  /// The wheel mouse button is pressed while helding Command and Shift
  CmdShiftWheel,
  /// The right mouse button is pressed while helding Command and Shift
  CmdShiftRight,
  /// The left mouse button, Shift and Command are held while moving pointer
  CmdShiftLeftDrag,
  /// The wheel mouse button, Shift and Command are held while moving pointer
  CmdShiftWheelDrag,
  /// The right mouse button, Shift and Command are held while moving pointer
  CmdShiftRightDrag,
  /// The left mouse button is pressed while helding Command and Ctrl
  CmdCtrlLeft,
  /// The wheel mouse button is pressed while helding Command and Ctrl
  CmdCtrlWheel,
  /// The right mouse button is pressed while helding Command and Ctrl
  CmdCtrlRight,
  /// The mouse wheel is going up while helding Command and Ctrl
  CmdCtrlWheelUp,
  /// The mouse wheel is going down while helding Command and Ctrl
  CmdCtrlWheelDown,
  /// The left mouse button, Ctrl and Command are held while moving pointer
  CmdCtrlLeftDrag,
  /// The wheel mouse button, Ctrl and Command are held while moving pointer
  CmdCtrlWheelDrag,
  /// The right mouse button, Ctrl and Command are held while moving pointer
  CmdCtrlRightDrag,

  /// The left mouse button is pressed while helding Command, Shift and Ctrl
  CmdShiftCtrlLeft,
  /// The wheel mouse button is pressed while helding Command, Shift and Ctrl
  CmdShiftCtrlWheel,
  /// The right mouse button is pressed while helding Command, Shift and Ctrl
  CmdShiftCtrlRight,
  /// The left mouse button, Ctrl, Shift and Command are held while moving pointer
  CmdShiftCtrlLeftDrag,
  /// The wheel mouse button, Ctrl, Shift and Command are held while moving pointer
  CmdShiftCtrlWheelDrag,
  /// The right mouse button, Ctrl, Shift and Command are held while moving pointer
  CmdShiftCtrlRightDrag,
}
