use std::time::Duration;
use specs::World;
use specs::RunArg;
use specs::System;
use std::io::Write;

macro_rules! null_system {
  ($thing:ty, $message:expr) => {
    impl System<Duration> for $thing {
      fn run(&mut self, arg: RunArg, _: Duration) {
        arg.fetch(|_| ());
        ::std::io::stdout().write(format!("{} says: {}", stringify!($thing), $message).as_bytes());
        ::std::io::stdout().flush();
      }
    }
  }
}

struct SomeSystemData {
  pub num: i32
}
data_dependency_from_new!(SomeSystemData);

impl SomeSystemData {
  pub fn new() -> SomeSystemData {
    SomeSystemData {
      num: 10
    }
  }
}

struct SomeCommonData {
  pub oi: i32
}
data_dependency_from_new!(SomeCommonData);

impl SomeCommonData {
  pub fn new() -> SomeCommonData {
    SomeCommonData {
      oi: 5
    }
  }
}

pub struct SomeSystem;
null_system!(SomeSystem, "hello2\n");
declare_data_dependencies!(SomeSystem, [SomeSystemData, SomeCommonData]);
declare_system_dependencies!(SomeSystem, [SomeOtherSystem]);
standalone_installer_from_new!(SomeSystem, Duration);

impl SomeSystem {
  pub fn new() -> SomeSystem {
    SomeSystem
  }
}


pub struct SomeOtherSystem;
declare_data_dependencies!(SomeOtherSystem, [SomeCommonData]);
declare_system_dependencies!(SomeOtherSystem, []);
standalone_installer_from_new!(SomeOtherSystem, Duration);

impl System<Duration> for SomeOtherSystem {
  fn run(&mut self, arg: RunArg, _: Duration) {
    let mut commonData = arg.fetch(|w| {
      w.write_resource::<SomeCommonData>()
    });
    ::std::io::stdout().write(format!("SomeOtherSystem says: goodbye with {}\n", commonData.oi).as_bytes());
    ::std::io::stdout().flush();
    commonData.oi = commonData.oi + 1;
  }
}

impl SomeOtherSystem {
  pub fn new() -> SomeOtherSystem {
    SomeOtherSystem
  }
}

pub struct SomeThirdSystem;
null_system!(SomeThirdSystem, "ayyyy\n");
declare_data_dependencies!(SomeThirdSystem, []);
declare_system_dependencies!(SomeThirdSystem, []);
standalone_installer_from_new!(SomeThirdSystem, Duration);

impl SomeThirdSystem {
  pub fn new() -> SomeThirdSystem {
    SomeThirdSystem
  }
}
