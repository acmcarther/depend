use specs::World;

pub trait DataDependency {
  fn create(&mut World) -> Self;
}

pub trait DataDependencyAware {
  fn install_data_dependencies(world: &mut World);
}

pub fn install_data<T: DataDependencyAware>(mut world: &mut World) {
  T::install_data_dependencies(&mut world);
}

#[cfg(test)]
mod tests {
  use super::*;
  use specs;

  struct TestData {
    some_field: f32,
  }
  data_dependency_from_new!(TestData);

  impl TestData {
    pub fn new() -> TestData {
      TestData { some_field: 1.0 }
    }
  }

  struct TestSystem;
  standalone_installer_from_new!(TestSystem, ());
  declare_data_dependencies!(TestSystem, [TestData]);

  impl<T> specs::System<T> for TestSystem {
    fn run(&mut self, arg: specs::RunArg, _: T) {
      let dep = arg.fetch(|w| w.write_resource::<TestData>());
    }
  }

  impl TestSystem {
    pub fn new() -> TestSystem {
      TestSystem
    }
  }

  #[test]
  fn end_to_end() {
    let mut world = specs::World::new();

    install_data::<TestSystem>(&mut world);

    let mut planner = specs::Planner::new(world, 2);
    planner.dispatch(());
  }
}
