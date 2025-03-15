use embedded_can::{Frame, Id};
use heapless::Vec;
use roboken_rs::{
    can::{
        FrameFactory,
        data::{Channel, Data},
    },
    robotics::switch::SwitchState,
};

#[derive(Debug)]
struct MockFrame {
    id: Id,
    data: Vec<u8, 8>,
}

impl FrameFactory for MockFrame {}

impl Frame for MockFrame {
    fn new(id: impl Into<Id>, data: &[u8]) -> Option<Self> {
        Some(Self {
            id: id.into(),
            data: Vec::from_slice(data).unwrap(),
        })
    }
    fn new_remote(_id: impl Into<Id>, _dlc: usize) -> Option<Self> {
        None
    }
    fn is_extended(&self) -> bool {
        true
    }
    fn is_standard(&self) -> bool {
        false
    }
    fn is_remote_frame(&self) -> bool {
        false
    }
    fn is_data_frame(&self) -> bool {
        true
    }
    fn id(&self) -> Id {
        self.id
    }
    fn dlc(&self) -> usize {
        self.data.len()
    }
    fn data(&self) -> &[u8] {
        &self.data
    }
}

fn main() {
    let frame = MockFrame::build(
        0x01,
        0x00,
        &Data::NotifySwitchState(Channel::new(0), SwitchState::On),
    );
    println!("{:?}", frame);
    println!("{:?}", frame.parse());
}
