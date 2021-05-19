pub fn sub(left:i32, right:i32) -> i32 {
    return 0;
}

pub mod test {
    use super::*;

    #[test]
    fn cero() {
        assert_eq!(0, sub(0, 0));
    }

    #[test]
    fn substraction() {
        assert_eq!(1, sub(2, 1));
    }

    #[test]
    fn negative_substraction(){
        assert_eq!(-2, sub(-1, 1))
    }  
}