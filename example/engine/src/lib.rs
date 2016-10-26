#[macro_use]
extern crate depend;
extern crate specs;

use std::time::Duration;
use std::io::Write;
use specs::World;
use specs::Planner;
use specs::SystemInfo;

mod systems;

#[no_mangle]
pub fn install_data(mut world: &mut World) {
  install_data!(world, [systems::SomeSystem, systems::SomeOtherSystem]);

}

#[no_mangle]
pub fn fetch_systems() -> Vec<SystemInfo<Duration>> {
  fetch_systems!([systems::SomeSystem, systems::SomeOtherSystem, systems::SomeThirdSystem])

}
