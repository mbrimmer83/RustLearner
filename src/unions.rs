#![allow(dead_code)]

union IntOrFloat {
  i: i32,
  f: f32
}

fn process_value(iof: IntOrFloat) {
  unsafe {
    match iof {
      IntOrFloat { i: 42} => println!("Meaning o flife value!"),
      IntOrFloat { f } => println!("value  = {}", f),
    }
  }
}

pub fn unions() {
  let mut iof = IntOrFloat{ i: 123 };
  iof.i = 234;

  let value = unsafe { iof.i };
  println!("iof.i = {}", value);

  process_value(IntOrFloat{i:5});
}