#[macro_use]
extern crate depend;
extern crate specs;

use std::time::Duration;
use specs::World;
use specs::Planner;

mod systems;

#[no_mangle]
pub fn fresh_planner(mut world: World) -> Planner<Duration> {
  println!("first fresh");
  fresh_planner!(world, 2, [systems::SomeSystem, systems::SomeOtherSystem])
}

#[no_mangle]
pub fn hotload_planner(mut world: World) -> Planner<Duration> {
  println!("second hotload");

  hotload_planner!(planner, 2, [systems::SomeSystem, systems::SomeOtherSystem])
}
