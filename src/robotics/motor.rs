pub trait Motor {
    fn cw(&mut self, speed: u16);
    fn ccw(&mut self, speed: u16);
    fn run(&mut self, velocity: i32) {
        let speed = velocity.abs().try_into().unwrap();
        if velocity >= 0 {
            self.cw(speed);
        } else {
            self.ccw(speed);
        }
    }
}
