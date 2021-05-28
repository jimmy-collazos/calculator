pub fn add(a:i32, b:i32) -> i32 {
    return a + b;
}

pub mod add_test {
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