#[cfg(test)]
mod tests {
    use calculator_backend::{calculate_expression, convert_rpn, History};

    #[test]
    fn test_calculate_basic_arithmetic() {
        let mut history = History::new();
        let result = calculate_expression("1 + 2", &mut history );
        assert!(result.success);
        assert_eq!(result.result, 3.0);
    }

    #[test]
    fn test_calculate_multiplication_division() {
        let mut history = History::new();
        let result = calculate_expression("3 * 4 / 2", &mut history );
        assert!(result.success);
        assert_eq!(result.result, 6.0);
    }

    #[test]
    fn test_calculate_complex_expression() {
        let mut history = History::new();
        let result = calculate_expression("(1 + 2) * (3 + 4)", &mut history );
        assert!(result.success);
        assert_eq!(result.result, 21.0);
    }

    #[test]
    fn test_calculate_exponents() {
        let mut history = History::new();
        let result = calculate_expression("2 ^ 3 + 1", &mut history );
        assert!(result.success);
        assert_eq!(result.result, 9.0);
    }

    #[test]
    fn test_calculate_division_by_zero() {
        let mut history = History::new();
        let result = calculate_expression("1 / 0", &mut history );
        assert!(!result.success);
    }

    #[test]
    fn test_calculate_invalid_expression() {
        let mut history = History::new();
        let result = calculate_expression("(1 + 2", &mut history );
        assert!(!result.success);
    }

    #[test]
    fn test_calculate_invalid_operator() {
        let mut history = History::new();
        let result = calculate_expression("1 $ 2", &mut history );
        assert!(!result.success);
    }

    #[test]
    fn test_convert_rpn_basic() {
        let result = convert_rpn("1 2 +".to_string());
        assert!(result.success);
        assert_eq!(result.infix_expression, "1 + 2");
    }

    #[test]
    fn test_convert_rpn_complex() {
        let result = convert_rpn("3 4 2 * +".to_string());
        assert!(result.success);
        assert_eq!(result.infix_expression, "3 + 4 * 2");
    }

    #[test]
    fn test_convert_rpn_multiple_operators() {
        let result = convert_rpn("1 2 + 3 4 + *".to_string());
        assert!(result.success);
        assert_eq!(result.infix_expression, "(1 + 2) * (3 + 4)");
    }

    #[test]
    fn test_convert_rpn_invalid_too_many_operators() {
        let result = convert_rpn("1 2 + +".to_string());
        assert!(!result.success);
    }

    #[test]
    fn test_convert_rpn_invalid_too_many_operands() {
        let result = convert_rpn("1 2 3 +".to_string());
        assert!(!result.success);
    }
}
