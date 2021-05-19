pub fn sqrt(x:i32) -> i32 {
    if x < 0 {
        panic!("Guess value must be positive number: {}", x);
    }
    return (x as f64).sqrt() as i32;
}

pub mod test {
    use super::*;

    #[test]
    fn cero() {
        assert_eq!(0, sqrt(0));
    }

    #[test]
    fn squareroot() {
        assert_eq!(3, sqrt(9));
    }

    #[test]
    #[should_panic]
    fn negative_squareroot(){
        sqrt(-9);
    }  
}