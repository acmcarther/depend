#[macro_export]
macro_rules! standalone_installer_from_new {
  ($system:ty, $delta:ty) => {
    impl $crate::system::StandaloneInstaller<$delta> for $system {
      fn system() -> $system {
        Self::new()
      }
    }
  }
}

#[macro_export]
macro_rules! data_dependency_from_new {
  ($data:ty) => {
    impl $crate::data::DataDependency for $data {
      fn create(_: &mut ::specs::World) -> $data {
        Self::new()
      }
    }
  }
}


#[macro_export]
macro_rules! declare_data_dependencies {
  ($system:ty, [$($data:ty),*]) => {
    impl $crate::data::DataDependencyAware for $system {
      // Silence the unused mut world when no data dependencies are provided
      #[allow(unused_variables, unused_mut)]
      fn install_data_dependencies(mut world: &mut ::specs::World) {
        $(
          if !world.has_resource::<$data>() {
            let dep_value = <$data as $crate::data::DataDependency>::create(&mut world);
            world.add_resource::<$data>(dep_value);
          }
        )*
      }
    }
  }
}

#[macro_export]
macro_rules! declare_system_dependencies {
  ($system:ty, [$($other_system:ty),*]) => {
    impl $crate::system::SystemDependencyAware for $system {
      fn dependencies(&self) -> Vec<::std::any::TypeId> {
        vec![$(::std::any::TypeId::of::<$other_system>()),*]
      }

      fn identity(&self) -> String {
        format!("{}::{}", module_path!(), stringify!($system))
      }
    }
  }
}

#[macro_export]
macro_rules! fresh_planner {
  ($world:ident, $thread_count:expr, [$($system:ty),*]) => {
    {
      $($crate::data::install_data::<$system>(&mut $world);)*

      let mut auto_installer = $crate::system::AutoInstaller::with_world($world);
      $(auto_installer.auto_install::<$system>();)*

      auto_installer.apply($thread_count)
    }
  }
}
#[macro_export]
macro_rules! hotload_planner {
  ($planner:ident, $thread_count:expr, [$($system:ty),*]) => {
    {
      use $crate::hotload::WorldSteal;

      let old_world = $planner.take_world();
      drop($planner);

      let mut auto_installer = $crate::system::AutoInstaller::with_world(old_world);
      $(auto_installer.auto_install::<$system>();)*

      auto_installer.apply($thread_count)
    }
  }
}
