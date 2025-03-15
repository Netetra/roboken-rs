use num_enum::{IntoPrimitive, TryFromPrimitive};

pub trait Motor {
    fn run_cw(&mut self, duty: impl Into<Duty>);
    fn run_ccw(&mut self, duty: impl Into<Duty>);
    fn run(&mut self, signed_duty: impl Into<SignedDuty>) -> Option<()> {
        let signed_duty: SignedDuty = signed_duty.into();
        if signed_duty.value() >= 0 {
            self.run_cw(signed_duty.value() as u16);
        } else {
            self.run_ccw(signed_duty.value().abs() as u16);
        }
        Some(())
    }
    fn run_with_dir(&mut self, duty: impl Into<Duty>, dir: Dir) {
        if dir == Dir::Cw {
            self.run_cw(duty);
        } else {
            self.run_ccw(duty);
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Duty(u16);

impl Duty {
    pub fn value(&self) -> u16 {
        self.0
    }
}

impl From<u16> for Duty {
    fn from(value: u16) -> Self {
        Self(value)
    }
}

impl From<Duty> for u16 {
    fn from(value: Duty) -> Self {
        value.0
    }
}

#[derive(Clone, Copy)]
pub struct SignedDuty(i32);

impl SignedDuty {
    pub fn value(&self) -> i32 {
        self.0
    }
}

impl TryFrom<i32> for SignedDuty {
    type Error = ();
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value.abs() > u16::MAX as i32 {
            return Err(());
        }
        Ok(SignedDuty(value))
    }
}

impl From<SignedDuty> for i32 {
    fn from(value: SignedDuty) -> Self {
        value.0
    }
}

impl From<Duty> for SignedDuty {
    fn from(value: Duty) -> Self {
        Self(value.0 as i32)
    }
}

impl TryFrom<SignedDuty> for Duty {
    type Error = ();
    fn try_from(value: SignedDuty) -> Result<Self, Self::Error> {
        if value.value() < 0 {
            return Err(());
        }
        Ok(Self(value.value() as u16))
    }
}

#[derive(PartialEq, IntoPrimitive, TryFromPrimitive, Copy, Clone, Debug)]
#[repr(u8)]
pub enum Dir {
    Cw = 0,
    Ccw = 1,
}
