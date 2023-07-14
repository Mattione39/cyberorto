use crate::servo42::MotorError;
use core::fmt::Debug;
use core::time::Duration;
pub trait Motor
where
    Self: Sized,
{
    type PosUnit;
    type Info: Debug;
    type Builder: MotorBuilder<Self>;
    ///set a new objective
    fn goto(&mut self, pos: Self::PosUnit) -> Result<(), ()>;
    ///get printable info
    fn get_info(&mut self) -> Self::Info;
    ///function to call for an update
    fn update(&mut self, time_from_last: Duration) -> Result<(), MotorError>;
    ///find zero, and set
    fn reset(&mut self);
    ///set zero here
    fn set_zero(&mut self);
    ///Generic Function for set max speed, acceleration...
    fn new() -> Self::Builder;
}
pub trait MotorBuilder<T: Motor>
where
    Self: Sized,
{
    fn build(self) -> T;
}
