use specs::World;
use specs::Planner;
use std::mem;

pub trait WorldSteal {
  fn take_world(&mut self) -> World;
}

impl<T: 'static> WorldSteal for Planner<T> {
  fn take_world(&mut self) -> World {
    let mut world = World::new();
    mem::swap(self.mut_world(), &mut world);

    world
  }
}
