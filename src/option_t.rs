#![allow(dead_code)]

pub fn option_t() {
  let x = 3.0;
  let y = 2.0;

  // Some(z) or None
  let result:Option<f64> =
    if y != 0.0 { Some(x/y) } else { None };

  println!("{:?}", result);

  match result {
    Some(z) => println!("{}/{} = {}", x, y, z),
    None => println!("Cannot divide {} by {}", x, y),
  }

  // if let / while let
  if let Some(z) = result { println!("z = {}", z); }
}