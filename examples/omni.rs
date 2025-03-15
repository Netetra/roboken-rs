use core::f32;

use roboken_rs::robotics::{
    motor::Motor,
    omni::{OmniWheel, OmniWheels},
};

struct MockMotor(u8);

impl Motor for MockMotor {
    fn cw(&mut self, speed: u16) {
        print!("[{}] Speed: {}, ", self.0, speed as i32);
    }
    fn ccw(&mut self, speed: u16) {
        print!("[{}] Speed: {}, ", self.0, -1 * speed as i32);
    }
}

fn main() {
    let mut omni: OmniWheels<_, 3> = [
        OmniWheel::new(MockMotor(0), f32::consts::FRAC_PI_6 * 5., 1.),
        OmniWheel::new(MockMotor(1), f32::consts::FRAC_PI_6 * 7., 1.),
        OmniWheel::new(MockMotor(2), 0., 1.),
    ]
    .into();
    let x = 100.;
    let y = 0.;
    let rotation = 0.;
    omni.run(x, y, rotation);
    println!();

    let x = 0.;
    let y = 100.;
    omni.run(x, y, rotation);
    println!();

    let x = 100.;
    let y = 100.;
    omni.run(x, y, rotation);
    println!();
}
