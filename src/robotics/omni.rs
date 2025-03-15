#[allow(unused_imports)]
use micromath::F32Ext;
use micromath::vector::{Vector, Vector2d};

use super::motor::{Motor, SignedDuty};

pub struct OmniWheel<M: Motor> {
    motor: M,
    vector: Vector2d<f32>,
    radius: f32,
}

impl<M: Motor> OmniWheel<M> {
    pub fn new(motor: M, angle: f32, radius: f32) -> Self {
        let vector = Vector2d::from_slice(&[angle.cos(), angle.sin()]);
        Self {
            motor,
            vector,
            radius,
        }
    }
    pub fn run(&mut self, x: f32, y: f32, rotation: f32) {
        let velocity = self.vector.dot((x, y).into()) + self.radius * rotation;
        let signed_duty = SignedDuty::try_from(velocity as i32).unwrap();
        self.motor.run(signed_duty);
    }
}

pub struct OmniWheels<M: Motor, const N: usize>([OmniWheel<M>; N]);

impl<M: Motor, const N: usize> OmniWheels<M, N> {
    pub fn new(omni_wheels: [OmniWheel<M>; N]) -> Self {
        Self(omni_wheels)
    }

    pub fn run(&mut self, x: f32, y: f32, rotation: f32) {
        for omni_wheel in self.0.iter_mut() {
            omni_wheel.run(x, y, rotation);
        }
    }
}

impl<M: Motor, const N: usize> From<[OmniWheel<M>; N]> for OmniWheels<M, N> {
    fn from(value: [OmniWheel<M>; N]) -> Self {
        Self(value)
    }
}
