use ::dag::{Dag, PriorityMap};
use std::any::{Any, TypeId};
use std::convert::From;
use specs;
use itertools::Itertools;


/// A system that can be instantiated and installed by the InstallPlugin
pub trait StandaloneInstaller<T: 'static>: specs::System<T> {
  fn system() -> Self;
}

/// A type that can install Installers and StandaloneInstallers
pub trait InstallPlugin<T: 'static> {
  fn install<U: StandaloneInstaller<T>>(&mut self, name: &str, priority: specs::Priority);
  fn install_instance<U: specs::System<T>>(&mut self,
                                           installer: U,
                                           name: &str,
                                           priority: specs::Priority);
}

/// Implemented by default for specs::Planner
impl<T: 'static> InstallPlugin<T> for specs::Planner<T> {
  fn install<U: StandaloneInstaller<T> + 'static>(&mut self,
                                                  name: &str,
                                                  priority: specs::Priority) {
    let i = U::system();
    self.install_instance(i, name, priority);
  }

  fn install_instance<U: specs::System<T> + 'static>(&mut self,
                                                     instance: U,
                                                     name: &str,
                                                     priority: specs::Priority) {
    self.add_system(instance, name, priority);
  }
}


pub type SystemDependency = TypeId;
type LazyInstallFn<T> = FnMut(&mut specs::Planner<T>, &PriorityMap<SystemDependency>);


pub trait SystemDependencyAware {
  fn dependencies(&self) -> Vec<SystemDependency>;
  fn identity(&self) -> String;
}

pub struct AutoInstaller<T: 'static> {
  install_thunks: Vec<Box<LazyInstallFn<T>>>,
  world: specs::World,
  dependency_set: Dag<SystemDependency>,
}

impl<T> AutoInstaller<T> {
  pub fn with_world(world: specs::World) -> AutoInstaller<T> {
    AutoInstaller {
      install_thunks: Vec::new(),
      world: world,
      dependency_set: Dag::new(),
    }
  }

  pub fn new() -> AutoInstaller<T> {
    AutoInstaller {
      install_thunks: Vec::new(),
      world: specs::World::new(),
      dependency_set: Dag::new(),
    }
  }

  pub fn take_dag(self) -> Dag<SystemDependency> {
    self.dependency_set
  }

  pub fn mut_world(&mut self) -> &mut specs::World {
    &mut self.world
  }

  pub fn auto_install<U: Any + StandaloneInstaller<T> + SystemDependencyAware>
    (&mut self)
     -> &mut AutoInstaller<T> {
    self.auto_install_instance(U::system());
    self
  }

  fn auto_install_instance<U: Any + specs::System<T> + SystemDependencyAware>
    (&mut self,
     installer: U)
     -> &mut AutoInstaller<T> {
    let own_type = SystemDependency::of::<U>();
    self.dependency_set.add_alias(&own_type, installer.identity());
    self.dependency_set.add_system(&own_type);
    self.dependency_set.add_dependency_set(&own_type, installer.dependencies().as_slice());
    let mut closure_own_type = Some(own_type);
    let mut closure_installer = Some(installer);

    self.install_thunks.push(Box::new(move |ref mut planner, ref priority_map| {
      // Bookkeeping to let this be a FnMut instead of a FnOnce
      // See: https://stackoverflow.com/questions/30411594/moving-a-boxed-function
      let installer = closure_installer.take().unwrap();
      let own_type = closure_own_type.take().unwrap();

      let name = format!("{:?}", own_type.clone());

      planner.install_instance(installer,
                               &name,
                               priority_map.get(&own_type).unwrap() as specs::Priority);
    }));
    self
  }

  pub fn apply(self, num_threads: usize) -> specs::Planner<T> {
    let priority_map = PriorityMap::from(self.dependency_set);
    let mut planner = specs::Planner::new(self.world, num_threads);
    self.install_thunks.into_iter().foreach(|mut thunk| thunk(&mut planner, &priority_map));

    planner
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use specs;

  macro_rules! null_system {
    ($thing:ty, $msg:expr) => {
      impl<T> specs::System<T> for $thing {
        fn run(&mut self, arg: specs::RunArg, _: T) {
          arg.fetch(|_| ());
          println!($msg);
        }
      }
    }
  }

  struct MockSystem1;
  null_system!(MockSystem1, "spin 1");
  declare_system_dependencies!(MockSystem1, []);
  standalone_installer_from_new!(MockSystem1, ());

  impl MockSystem1 {
    fn new() -> MockSystem1 {
      MockSystem1
    }
  }

  struct MockSystem2;

  null_system!(MockSystem2, "spin 2");
  declare_system_dependencies!(MockSystem2, []);
  standalone_installer_from_new!(MockSystem2, ());

  impl MockSystem2 {
    fn new() -> MockSystem2 {
      MockSystem2
    }
  }

  struct MockSystem3;
  null_system!(MockSystem3, "spin 3");
  declare_system_dependencies!(MockSystem3, [MockSystem2, MockSystem4]);
  standalone_installer_from_new!(MockSystem3, ());

  impl MockSystem3 {
    fn new() -> MockSystem3 {
      MockSystem3
    }
  }

  struct MockSystem4;
  null_system!(MockSystem4, "spin 4");
  declare_system_dependencies!(MockSystem4, []);
  standalone_installer_from_new!(MockSystem4, ());

  impl MockSystem4 {
    fn new() -> MockSystem4 {
      MockSystem4
    }
  }

  #[test]
  fn test() {
    let mut i = AutoInstaller::<()>::new();

    i.auto_install::<MockSystem1>();
    i.auto_install::<MockSystem2>();
    i.auto_install::<MockSystem3>();
    i.auto_install::<MockSystem4>();

    let mut planner = i.apply(2);

    planner.dispatch(());
    // panic!("testing!")
  }
}
