use num_enum::{IntoPrimitive, TryFromPrimitive};

#[derive(IntoPrimitive, TryFromPrimitive, Clone, Copy, Debug)]
#[repr(u8)]
pub enum SwitchState {
    Off = 0,
    On = 1,
}
