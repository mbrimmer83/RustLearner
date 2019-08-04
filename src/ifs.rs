#![allow(dead_code)]

pub fn if_statement() {
  let temp = 35;

  if temp > 30 {
    println!("It's really hot outside!");
  } else if temp < 10 {
    println!("It's really cold outside!");
  } else {
    println!("It's really nice outside!");
  }

  let day = if temp > 20 {"sunny"} else {"cloudy"};

  println!("Today is {}", day);
}