pub fn div(a:i32, b:i32) -> i32 {
    return a / b;
}

pub mod div_test {
    use super::*;

    #[test]
    #[should_panic]
    fn cero() {
        div(0,0);
    }

    #[test]
    fn division() {
        assert_eq!(1, div(1, 1));
    }

    #[test]
    fn negative_division(){
        assert_eq!(-1, div(-10, 10))
    }  
}