use std::mem;
mod sh;
mod ifs;
mod while_loop;
mod for_loop;
mod match_state;
mod struct_type;
mod enumerations;
mod unions;
mod option_t;
mod array;
mod vectors;

// CONSTANTS

// Global variable with no fixed address
// Replaces the use at compilation, copied to each location that uses
const MEANING_OF_LIFE:u8 = 42;

// Fixed address, true global
static mut ANOTHER_CONST:i32 = 123;

fn integers() {
  // u = unsigned integer 0 or positive
  let a:u8 = 123; // 8bits
  println!("a = {}", a);

  // i = signed interger
  let mut b:i8 = -123;
  println!("b = {}", b);
  b = -12;
  println!("b = {}", b);

  let mut c = 123456789; // 32-bit signed i32
  println!("c = {}, size = {} bytes", c, mem::size_of_val(&c));

  // Interger types
  // i8 u8 i16 u16 i32 u32 i64 u64
  // isize, usize based on the os, 32 bit or 64 bit

  let e: f32 = 2.5; // double-precision, 8 bytes or 64 bits, f64
  println!("e = {}, size = {} bytes", e, mem::size_of_val(&e));

  let f = false;
  println!("f = {}, size = {} bytes", f, mem::size_of_val(&f));
  let g = 4 > 0;
  println!("g = {}, size = {} bytes", g, mem::size_of_val(&g));
}

fn operators() {
  let mut a = 2 + 2 * 2;
  println!("a = {}", a);

  // increment no ++ or --
  // -= += *= %=

  let a_cubed = i32::pow(a, 3);
  println!("{} cubed is {}", a, a_cubed);

  let b = 2.5;
  let b_cubed = f64::powi(b, 3);
  // const pi from f std lib
  // const are capitalized
  let b_to_pi = f64::powf(b, std::f64::consts::PI);
  println!("{} cubed = {}, {}^pi = {}", b, b_cubed, b, b_to_pi);

  // bitwise operators
  let c = 1 | 2; // | or & AND ^ XOR ! NOR
                 // 01 or 10 == 11 3_10
  println!("1|2 = {}", c)
}

fn scope() {
  let a = 123;

  let a = 456; // redeclare variable in rust

  {
    let b = 156;
    // a scoped inside this block
    let a = 456;
    println!("inside b = {}", b);
    // Can access outer scope
    println!("a = {}", a);
  }

  println!("a = {}", a);
  // println!("b = {}", b)
}

fn main() {
  // intergers();
  // operators();
  // scope();
  // sh::stack_and_heap();
  // ifs::if_statement();
  // while_loop::while_loop();
  // for_loop::for_loop();
  // match_state::match_state();
  // struct_type::struct_type();
  // enumerations::enums();
  // unions::unions();
  // option_t::option_t();
  // array::array();
  vectors::vectors();
}