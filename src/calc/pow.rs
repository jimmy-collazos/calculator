pub fn pow(mut base:i32, mut exponent:i32) -> i32 {
  return 0;
}

pub mod pow_test {
  use super::*;

  #[test]
  fn cero_exponent() {
    let base = 10;
    let exponent = 0;
    let result = pow(base, exponent);
    let expect = 1;
    assert_eq!(expect, result);
  }

  #[test]
  fn one_exponent() {
    let base = 10;
    let exponent = 1;
    let result = pow(base, exponent);
    let expect = base;  
    assert_eq!(expect, result);
  }

  #[test]
  fn square_exponent() {
    let base = 2;
    let exponent = 2;
    let result = pow(base, exponent);
    let expect = base * base;  
    assert_eq!(expect, result);
  }

  #[test]
  fn cubic_exponent() {
    let base = 2;
    let exponent = 3;
    let result = pow(base, exponent);
    let expect = base * base * base;
    assert_eq!(expect, result);
  }
}