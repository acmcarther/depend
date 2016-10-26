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
        ::std::io::stdout().write($message);
        ::std::io::stdout().flush();
      }
    }
  }
}

pub struct SomeSystem;
null_system!(SomeSystem, b"hello2\n");
declare_data_dependencies!(SomeSystem, []);
declare_system_dependencies!(SomeSystem, []);
standalone_installer_from_new!(SomeSystem, Duration);

impl SomeSystem {
  pub fn new() -> SomeSystem {
    SomeSystem
  }
}


pub struct SomeOtherSystem;
null_system!(SomeOtherSystem, b"goodbye\n");
declare_data_dependencies!(SomeOtherSystem, []);
declare_system_dependencies!(SomeOtherSystem, []);
standalone_installer_from_new!(SomeOtherSystem, Duration);

impl SomeOtherSystem {
  pub fn new() -> SomeOtherSystem {
    SomeOtherSystem
  }
}
