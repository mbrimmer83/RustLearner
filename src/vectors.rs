#![allow(dead_code)]

pub fn vectors() {
  // Like a dynamic array with no known size
  let mut a = Vec::new();
  a.push(1);
  a.push(2);
  a.push(3);

  println!("a = {:?}", a);
  println!("a[0] = {}", a[0]);

  // usize
  let idx:usize = 0;
  a[idx] = 123;
  println!("a[0] = {}", a[idx]);

  // Vector length
  match a.get(6) {
    Some(x) => println!("a[6] = {}", x),
    None => println!("No such element!"),
  };

  // Iterate of vector
  for x in &a { println!("{}", x)}
  a.push(45);

  // Remove last element and store in last_elem
  let last_elem = a.pop(); // Option
  println!("last elem is {:?}, a = {:?}", last_elem, a);

  while let Some(x) = a.pop() {
    println!("{}", x);
  }
}