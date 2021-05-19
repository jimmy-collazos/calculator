pub fn div(left:i32, right:i32) -> i32 {
    return left / right;
}

pub mod test {
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