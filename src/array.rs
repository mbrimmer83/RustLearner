#![allow(dead_code)]

use std::mem;

pub fn array() {
  let mut a:[i32;5] = [1, 2, 3, 4, 5];
  println!("s has {} elements, first is {}", a.len(), a[0]);

  a[0] = 312;
  println!("a[0] = {}", a[0]);

  // Print the entire array with debug statement
  println!("{:?}", a);

  if a == [1, 2, 3, 4, 5] {
    println!("match");
  }

  // ten elements equal to one
  let b = [1u16; 10];

  for i in 0..b.len() {
    println!("{}", b[i]);
  }

  println!("b took up {} bytes", mem::size_of_val(&b));

  // Multi demensional arrays

  let mtx: [[f32; 3]; 2] = [
    [1.0, 0.0, 0.0],
    [0.0, 2.0, 0.0]
  ];

  println!("{:?}", mtx);

  for i in 0..mtx.len() {
    for j in 0..mtx.len() {
      if i == j { println!("mtx[{}][{}] = {}", i, j, mtx[i][j])}
    }
  }
}