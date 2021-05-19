pub fn add(left:i32, right:i32) -> i32 {
    return left + right;
}

pub mod test {
    use super::*;

    #[test]
    fn cero() {
        assert_eq!(0, add(0,0));
    }

    #[test]
    fn addition() {
        assert_eq!(2, add(1, 1));
    }

    #[test]
    fn negative_addition(){
        assert_eq!(0, add(1, -1))
    }  
}