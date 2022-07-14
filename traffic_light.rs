#[derive(Debug)]
enum TrafficLight {
    Red,
    Green,
    Yellow,
}
pub trait TurnOnable {
    fn show_time(&self) -> usize;
}
impl TurnOnable for TrafficLight {
    fn show_time(&self) -> usize {
        match *self {
            TrafficLight::Red => 10,
            TrafficLight::Green => 15,
            TrafficLight::Yellow => 5,
        }
    }
}
fn turn_on(light: &(impl TurnOnable + std::fmt::Debug)) {
    println!(
        "{:?} is turning on for {} seconds.",
        light,
        light.show_time()
    );
}
fn main() {
    turn_on(&TrafficLight::Red);
    turn_on(&TrafficLight::Green);
    turn_on(&TrafficLight::Yellow);
}
