extern crate itertools;
extern crate daggy;
extern crate specs;

#[macro_use]
pub mod macros;

pub mod dag;
pub mod system;
pub mod data;
pub mod hotload;

#[cfg(test)]
mod tests {
  macro_rules! null_system {
    ($thing:ty) => {
      impl<T> specs::System<T> for $thing {
        fn run(&mut self, arg: specs::RunArg, _: T) {
          arg.fetch(|_| ());
        }
      }
    }
  }

  use super::*;
  use specs::*;
  use specs;

  struct SomeSystemState {
    some_field: f32,
  }
  data_dependency_from_new!(SomeSystemState);

  impl SomeSystemState {
    pub fn new() -> SomeSystemState {
      SomeSystemState { some_field: 1.0 }
    }
  }

  struct SomeSystem;
  null_system!(SomeSystem);
  declare_data_dependencies!(SomeSystem, [SomeSystemState]);
  declare_system_dependencies!(SomeSystem, [SomeOtherSystem]);
  standalone_installer_from_new!(SomeSystem, ());
  impl SomeSystem {
    pub fn new() -> SomeSystem {
      SomeSystem
    }
  }

  struct SomeOtherSystem;
  null_system!(SomeOtherSystem);
  declare_data_dependencies!(SomeOtherSystem, []);
  declare_system_dependencies!(SomeOtherSystem, []);
  standalone_installer_from_new!(SomeOtherSystem, ());
  impl SomeOtherSystem {
    pub fn new() -> SomeOtherSystem {
      SomeOtherSystem
    }
  }

  #[test]
  fn end_to_end() {
    let mut initial_world = World::new();
    let mut planner = fresh_planner!(initial_world, 2, [SomeSystem, SomeOtherSystem]);

    planner.dispatch(());
  }

  #[test]
  fn hotload() {
    let mut initial_world = World::new();
    let mut planner = fresh_planner!(initial_world, 2, [SomeSystem, SomeOtherSystem]);

    planner.dispatch(());

    let mut new_planner = hotload_planner!(planner, 2, [SomeSystem, SomeOtherSystem]);
  }
}
