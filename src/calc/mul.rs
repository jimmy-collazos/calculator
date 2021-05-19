pub fn mul(left:i32, right:i32) -> i32 {
    return left * right;
}

pub mod test {
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