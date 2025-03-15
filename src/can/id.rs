use bit_field::BitField;
use embedded_can::ExtendedId;
use num_enum::{IntoPrimitive, TryFromPrimitive};

/// System Command:    0x0000 - 0x001F
/// Control Command:   0x0020 - 0x0AFF
/// Telemetry Command: 0x0B00 - 0x15DF
/// Config Command:    0x15E0 - 0x1FFF
#[derive(IntoPrimitive, TryFromPrimitive)]
#[repr(u16)]
pub enum Command {
    Stop = 0x0000,
    Ping = 0x0001,
    Pong = 0x0002,
    SetDuty = 0x0020,
    NotifySwitchState = 0x0B00,
}

#[derive(Debug)]
pub struct NodeId(u8);

impl NodeId {
    const BROADCAST_ADDRESS: u8 = 0xFF;

    pub fn new(raw_id: u8) -> Self {
        Self(raw_id)
    }
    pub fn broadcast() -> Self {
        Self(Self::BROADCAST_ADDRESS)
    }
    pub fn as_raw(&self) -> u8 {
        self.0
    }
    pub fn is_broadcast(&self) -> bool {
        self.0 == Self::BROADCAST_ADDRESS
    }
}

impl From<u8> for NodeId {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl From<NodeId> for u8 {
    fn from(value: NodeId) -> Self {
        value.as_raw()
    }
}

pub trait IdFactory {
    fn parse(self) -> Option<(Command, NodeId, NodeId)>;
    fn build(command: Command, sender_id: &NodeId, receiver_id: &NodeId) -> ExtendedId;
}

impl IdFactory for ExtendedId {
    fn parse(self) -> Option<(Command, NodeId, NodeId)> {
        let raw = self.as_raw();
        let command = Command::try_from(raw.get_bits(16..=28) as u16).ok()?;
        let sender_id = NodeId::new(raw.get_bits(8..=15) as u8);
        let receiver_id = NodeId::new(raw.get_bits(0..=7) as u8);
        Some((command, sender_id, receiver_id))
    }
    fn build(command: Command, sender_id: &NodeId, receiver_id: &NodeId) -> ExtendedId {
        let raw_command = u16::from(command) as u32;
        let raw_sender_id = sender_id.as_raw() as u32;
        let raw_receiver_id = receiver_id.as_raw() as u32;
        unsafe {
            ExtendedId::new_unchecked(raw_command << 16 | raw_sender_id << 8 | raw_receiver_id)
        }
    }
}
