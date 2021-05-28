// @see https://www.geeksforgeeks.org/bakhshali-approximation-computing-square-roots/
pub fn sqrt(x:i32) -> i32 {

    // reject negative values
    if x < 0 {
        panic!("Guess value must be positive number: {}", x);
    }

    // This will be the nearest perfect square to s
    let mut nearest_perfect_square = 0;
     
    // This is the sqrt of nearest_perfect_square
    let mut sqrt_near_perfect_square = 0;
 
    // Find the nearest perfect square to s
    let mut i = x;
    while i > 0 {
        for j in 1..i {
            if j*j == i {
                nearest_perfect_square = i;
                sqrt_near_perfect_square = j;
                break;
            }
        }
        if nearest_perfect_square > 0 {
            break;
        }
        i -= 1;
    }
     
    // calculate d   
    let d = x - nearest_perfect_square;    
     
    // calculate P
    let p = d / 2 * sqrt_near_perfect_square;
     
    // calculate A
    let a = sqrt_near_perfect_square + p;
      
    // calculate sqrt(S).
    let sqrt_of_x = a - (p * p / 2 * a);
    return sqrt_of_x;
}

pub mod sqrt_test {
    use super::*;

    #[test]
    fn cero() {
        assert_eq!(0, sqrt(0));
    }

    #[test]
    fn squareroot() {
        assert_eq!(3, sqrt(9));
        assert_eq!(5, sqrt(25));
        assert_eq!(75, sqrt(75*75));
    }

    #[test]
    #[should_panic]
    fn negative_squareroot(){
        sqrt(-9);
    }  
}