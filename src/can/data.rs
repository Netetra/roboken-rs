use heapless::Vec;

use crate::robotics::{
    motor::{Dir, Duty},
    switch::SwitchState,
};

use super::id::Command;

#[derive(Debug)]
pub enum Data {
    Stop,
    Ping,
    Pong,
    SetDuty(Channel, Duty, Dir),
    NotifySwitchState(Channel, SwitchState),
}

impl Data {
    pub fn command(&self) -> Command {
        match self {
            Data::Stop => Command::Stop,
            Data::Ping => Command::Ping,
            Data::Pong => Command::Pong,
            Data::SetDuty(_, _, _) => Command::SetDuty,
            Data::NotifySwitchState(_, _) => Command::NotifySwitchState,
        }
    }
    pub fn to_vec(&self) -> Vec<u8, 8> {
        match self {
            Data::Stop => Vec::new(),
            Data::Ping => Vec::new(),
            Data::Pong => Vec::new(),
            Data::SetDuty(channel, duty, dir) => {
                let duty_bytes = u16::from(*duty).to_be_bytes();
                Vec::from_iter([
                    channel.value(),
                    duty_bytes[0],
                    duty_bytes[1],
                    u8::from(*dir),
                ])
            }
            Data::NotifySwitchState(channel, state) => {
                Vec::from_iter([channel.value(), u8::from(*state)])
            }
        }
    }
}

#[derive(Debug)]
pub struct Channel(u8);

impl Channel {
    pub fn new(value: u8) -> Self {
        Self(value)
    }
    pub fn value(&self) -> u8 {
        self.0
    }
}

impl From<u8> for Channel {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl From<Channel> for u8 {
    fn from(value: Channel) -> Self {
        value.0
    }
}
