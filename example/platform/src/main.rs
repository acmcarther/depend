extern crate specs;
extern crate libloading;

use std::io::Write;
use libloading::Library;
use std::fs;
use std::time::SystemTime;
use std::time::Duration;
use std::time::Instant;
use std::thread;
use specs::World;
use specs::Planner;
use specs::SystemInfo;


const LIB_PATH: &'static str = "../engine/target/debug/libengine.so";

pub fn main() {
  let mut platform = Platform::new();
  let mut last_time = Instant::now();

  while platform.keep_running() {
    let now_time = Instant::now();
    let dt = now_time.duration_since(last_time);
    last_time = now_time;

    platform.run(dt);
    thread::sleep(Duration::from_millis(500));
    println!("end of tick");
  }
}

struct Platform {
  dylib: Option<Library>,
  planner: Planner<Duration>,
  dylib_last_modified: SystemTime,
}

impl Platform {
  pub fn new() -> Platform {
    let dylib = Library::new(LIB_PATH).unwrap();

    let mut world = World::new();

    {
      let install_data_fn =
        unsafe { dylib.get::<fn(&mut World)>(b"install_data").unwrap() };
      install_data_fn(&mut world)
    };

    let systems = {
      let fetch_systems_fn = unsafe {
        dylib.get::<fn() -> Vec<SystemInfo<Duration>>>(b"fetch_systems\0").unwrap()
      };
      fetch_systems_fn()
    };

    let mut planner = Planner::new(world, 2);
    planner.systems = systems;


    Platform {
      dylib: Some(dylib),
      planner: planner,
      dylib_last_modified: SystemTime::now(),
    }
  }

  fn try_reload(&mut self) {
    let mut last_modified = fs::metadata(LIB_PATH).unwrap().modified().unwrap();

    if last_modified > self.dylib_last_modified {
      std::io::stdout().write(b"reloading!\n");
      std::io::stdout().flush();
      drop(self.dylib.take().unwrap());

      let dylib = Library::new(LIB_PATH).unwrap();

      self.planner.systems = Vec::new();
      self.planner.systems = {
        let fetch_systems_fn = unsafe {
          dylib.get::<fn() -> Vec<SystemInfo<Duration>>>(b"fetch_systems\0").unwrap()
        };
        fetch_systems_fn()
      };

      self.dylib = Some(dylib);
      self.dylib_last_modified = last_modified;
    } else {
      // println!("not reloading");
    }
  }

  pub fn run(&mut self, dt: Duration) {
    self.planner.wait();
    self.try_reload();
    println!("sys count: {}", self.planner.systems.len());
    self.planner.dispatch(dt);
  }

  pub fn keep_running(&mut self) -> bool {
    true
  }
}
