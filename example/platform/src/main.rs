extern crate specs;
extern crate libloading;

use libloading::Library;
use std::fs;
use std::time::SystemTime;
use std::time::Duration;
use std::time::Instant;
use std::thread;
use specs::World;
use specs::Planner;


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
    // println!("end of tick");
  }
}

struct Platform {
  dylib: Option<Library>,
  planner: Option<Planner<Duration>>,
  dylib_last_modified: SystemTime,
}

impl Platform {
  pub fn new() -> Platform {
    let dylib = Library::new(LIB_PATH).unwrap();

    let mut world = World::new();

    let planner = {
      let fresh_fn =
        unsafe { dylib.get::<fn(World) -> Planner<Duration>>(b"fresh_planner\0").unwrap() };
      fresh_fn(world)
    };

    Platform {
      dylib: Some(dylib),
      planner: Some(planner),
      dylib_last_modified: SystemTime::now(),
    }
  }

  fn try_reload(&mut self) {
    let mut last_modified = fs::metadata(LIB_PATH).unwrap().modified().unwrap();

    if last_modified > self.dylib_last_modified {
      println!("reloading!");
      drop(self.dylib.take().unwrap());

      let dylib = Library::new(LIB_PATH).unwrap();

      let planner = {
        let hotload_fn = unsafe {
          dylib.get::<fn(Planner<Duration>) -> Planner<Duration>>(b"hotload_planner\0").unwrap()
        };
        hotload_fn(self.planner.take().unwrap())
      };

      self.dylib = Some(dylib);
      self.planner = Some(planner);
      self.dylib_last_modified = last_modified;
    } else {
      // println!("not reloading");
    }
  }

  pub fn run(&mut self, dt: Duration) {
    match self.planner {
      Some(ref mut p) => p.dispatch(dt),
      None => panic!("where did the planner go?"),
    }
    self.try_reload();
  }

  pub fn keep_running(&mut self) -> bool {
    true
  }
}
