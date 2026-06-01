// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;
#[derive(Debug)]
pub struct Duration(u64);

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration(s)
    }
}

pub trait Planet {
    const MULT: f64;
    fn years_during(d: &Duration) -> f64 {
        d.0 as f64 / 31557600 as f64 / Self::MULT
    }
}
macro_rules! plimple {
    ($planet:ident, $mult:expr) => {
        impl Planet for $planet {
            const MULT: f64 = $mult;
        }
    };
}
plimple!(Mercury, 0.2408467);
plimple!(Venus, 0.61519726);
plimple!(Earth, 1.0);
plimple!(Mars, 1.8808158);
plimple!(Jupiter, 11.862615);
plimple!(Saturn, 29.447498);
plimple!(Uranus, 84.016846);
plimple!(Neptune, 164.79132);
