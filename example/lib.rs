struct Flags;

impl Flags {
  fn get_flag<T>(name: &str) -> Option<T> {
    // Stub
    None
  }
}


#[no_mangle]
pub fn first_load(flags: Flags, thread_count: usize) -> Planner {
  let initial_world = specs::World::new();
  world.add_resource<Flags>(flags);

  let data_dependency_installed_world = add_data_dependencies(world);
  hotload(data_dependency_installed_world, thread_count);
}

fn add_data_dependencies(world: specs::World) -> specs::World {
  let data_installer = DataInstaller::<Delta>::new(world);

  install_all!(data_installer, [
    SomeSystem,
    SomeOtherSystem]);

  data_installer.take_world()
}

macro_rules! install_all {
  ($installer:expr, [$($system:ty),*]) => {
    $($installer.install::<$system>();)
  }
}

#[no_mangle]
pub fn hotload(world: specs::World, thread_count: usize) -> Planner {
  let system_installer = AutoInstaller::<Delta>::new(world);

  install_all!(system_installer, [
    SomeSystem,
    SomeOtherSystem])

  system_installer.apply(thread_count)
}

