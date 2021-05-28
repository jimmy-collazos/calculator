pub fn mul(a:i32, b:i32) -> i32 {
    return a * b;
}

pub mod mul_test {
    use super::*;

    #[test]
    fn cero() {
        assert_eq!(0, mul(0, 0));
    }

    #[test]
    fn multiplication() {
        assert_eq!(1, mul(1, 1));
    }

    #[test]
    fn negative_multiplication(){
        assert_eq!(-100, mul(-10, 10))
    }  
}