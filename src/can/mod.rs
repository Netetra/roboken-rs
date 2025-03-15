use data::{Channel, Data};
use embedded_can::{ExtendedId, Frame, Id};
use id::{Command, IdFactory, NodeId};

use crate::robotics::motor::{Dir, Duty};

pub mod data;
pub mod id;

pub trait FrameFactory: Frame {
    fn parse(&self) -> Option<(NodeId, NodeId, Data)> {
        let (command, sender_id, receiver_id) = match self.id() {
            Id::Standard(_) => return None,
            Id::Extended(id) => id.parse()?,
        };

        let raw_data = self.data();

        let data = match command {
            Command::Stop => {
                if raw_data.len() != 0 {
                    return None;
                }
                Data::Stop
            }
            Command::Ping => {
                if raw_data.len() != 0 {
                    return None;
                }
                Data::Ping
            }
            Command::Pong => {
                if raw_data.len() != 0 {
                    return None;
                }
                Data::Pong
            }
            Command::SetDuty => {
                if raw_data.len() != 4 {
                    return None;
                }
                let duty = (raw_data[1] as u16) << 8 | raw_data[0] as u16;
                let dir = if raw_data[3] == 0 {
                    Dir::Cw
                } else if raw_data[3] == 1 {
                    Dir::Ccw
                } else {
                    return None;
                };
                Data::SetDuty(Channel::new(raw_data[0]), Duty::from(duty), dir)
            }
            Command::NotifySwitchState => {
                if raw_data.len() != 2 {
                    return None;
                }
                Data::NotifySwitchState(Channel::new(raw_data[0]), raw_data[1].try_into().ok()?)
            }
        };
        Some((sender_id, receiver_id, data))
    }
    fn build(sender_id: impl Into<NodeId>, receiver_id: impl Into<NodeId>, data: &Data) -> Self {
        let id = ExtendedId::build(data.command(), &sender_id.into(), &receiver_id.into());
        let raw_data = data.to_vec();
        Self::new(id, &raw_data).unwrap()
    }
}
