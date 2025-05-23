#[cfg(test)]
mod tests {
    use calculator_backend::{calculate_expression, convert_rpn, History};

    // Helper function to avoid repeating code
    fn test_case(description: &str, data: &str, expected_result: f64) {
        println!("\nTest: {}", description);
        let mut history = History::new();
        let result = calculate_expression(data, &mut history);
        assert!(result.success, "Failed to calculate expression: {}", description);
        assert_eq!(result.result, expected_result, "Expression: {}", data);

        // Use a tolerance for floating-point comparison
        let tolerance = match expected_result.abs() {
            x if x > 1.0e50 => x * 1.0e-6,  // Large numbers need larger tolerance
            x if x < 1.0e-50 && x > 0.0 => x * 1.0e-6,  // Tiny numbers need proportional tolerance
            _ => 1.0e-9,  // Default tolerance for normal-sized numbers
        };
        
        assert!(
            (result.result - expected_result).abs() < tolerance,
            "Expression: {}\nExpected: {}, Got: {}, Diff: {}",
            data,
            expected_result,
            result.result,
            (result.result - expected_result).abs()
        );
    }

    #[test]
    fn test_string_handling() {
        // Very long expressions
        test_case("Long expression with repeated operations", 
                  "1 + 2 + 3 + 4 + 5 + 6 + 7 + 8 + 9 + 10 + 11 + 12 + 13 + 14 + 15 + 16 + 17 + 18 + 19 + 20", 
                  210.0);

        test_case ("Expression with White spaces", 
                  "1                       +                              2                           *                  3", 
                  7.0);
       
    }

    #[test]
    fn test_numeric_edge_cases() {
        // Large numbers and precision
        test_case("Very large numbers", 
                  "999999999999999999999999 + 1", 
                  1000000000000000000000000.0);

        // Very small decimals
        test_case("Very small decimals", 
                  "0.0000000000000000000001 * 10000000000000000000000", 
                  1.0);

        // Floating point precision
        test_case("Floating point precision", 
                  "0.1 + 0.2", 
                  0.3);
    }

    #[test]
    fn test_complex_expression_structure() {
        // Deeply nested expressions
        test_case("Deeply nested parentheses", 
                  "(((((1 + 2) * 3) + 4) * 5) / 6)", 
                  10.833333333);

        // Mixed operators with different precedence
        test_case("Complex operator precedence", 
                  "1 + 2 * 3 ^ 4 / 5 - 6", 
                  27.4);

        // Right-associative operations
        test_case("Multiple exponents (right associative)", 
                  "2 ^ 3 ^ 2", 
                  512.0);
    }

    #[test]
    fn test_rpn_edge_cases() {
        // Complex RPN expressions
        let result = convert_rpn("1 2 3 4 5 6 7 + + + + + +".to_string());
        assert!(result.success);
        assert_eq!(result.infix_expression, "1 + 2 + 3 + 4 + 5 + 6 + 7");

        let result = convert_rpn("1 2 + 3 + 4 + 5 + 6 +".to_string());
        assert!(result.success);
        assert_eq!(result.infix_expression, "1 + 2 + 3 + 4 + 5 + 6");

        // RPN stack manipulation edge cases
        let result = convert_rpn("1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 + + + + + + + + + + + + + + + + + + +".to_string());
        assert!(result.success);
    }

    #[test]
    fn test_memory_safety_edge_cases() {
        // Large expressions
        let mut max_expr = "1".to_string();
        for _ in 1..500 {
            max_expr.push_str(" + 1");
        }
        test_case("Maximum length expression", &max_expr, 500.0);
    }

    #[test]
    fn test_special_cases() {
        // Operator combinations
        test_case("Multiple exponents in sequence", 
                  "2 ^ 3 ^ 2 ^ 1", 
                  512.0);

        test_case("Negative exponents", 
                  "2 ^ -3", 
                  0.125);

        // Parentheses edge cases
        test_case("Multiple nested parentheses with operators", 
                  "((1 + 2) * (3 + 4)) / ((5 + 6) * (7 + 8))", 
                  0.127272727);

        // Mixed decimal and integer operations
        test_case("Mixed decimal and integer operations", 
                  "1.5 * 2 + 3.7 / 2.5", 
                  4.48);
    }

    #[test]
    fn test_scientific_notation() {
        // Scientific notation with small positive exponent
        test_case(
            "Scientific notation with small positive exponent",
            "1.23e5 + 4.56e-7",
            123000.000000456,
        );

        // Scientific notation with large positive exponent
        test_case(
            "Scientific notation with large positive exponent",
            "1.23e5 + 4.56e7",
            45723000.0,
        );

        // Scientific notation with small negative exponent
        test_case(
            "Scientific notation with small negative exponent",
            "1.23e-5 + 4.56e-7",
            0.000012756,
        );

        // Scientific notation with large negative exponent
        test_case(
            "Scientific notation with large negative exponent",
            "1.23e-5 - 4.56e-7",
            0.000011844,
        );

        // Scientific notation with mixed positive and negative exponents
        test_case(
            "Scientific notation with mixed exponents",
            "1.23e5 - 4.56e-7",
             122999.999999544,
        );

        // Scientific notation with very large numbers
        test_case(
            "Scientific notation with very large numbers",
            "1.23e100 + 4.56e99",
            1.686e100,
        );
    }
}
