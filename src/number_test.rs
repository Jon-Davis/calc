mod number;
use number::*;

mod number_test {
    #[test]
    fn test_add() {
        let num1 = Number::new(10);
        let num2 = Number::new(5);
        let num3 = Number::new(15);

        assert!((&num1 + &num2) == &num3, format!("{:X} + {:X} == {:X}",&num1,&num2,&num3));
    }
}
