#![allow(dead_code)]

pub fn match_state() {
  let country_code = 1;
  let country = match country_code {
    44 => "UK",
    46 => "Sweden",
    7 => "Russia",
    1 => "USA",
    1...999 => "Unkown", // Range between 1 and 999 inclusive and is required
    _ => "Invalid"
  };

  println!("The country with code {} is {}", country_code, country);
}
