#[cfg(test)]
mod tests {
    use std::ffi::CString;
    use calculator_backend::get_error_str;
    use calculator_backend::calculate_rpn;
    use calculator_backend::CReversePolishExpression;

    // Helper to convert a vector of Rust strings to C-style strings
    fn to_c_string_array(arr: &[&str]) -> (Vec<CString>, Vec<*const i8>) {
        let c_strings: Vec<CString> = arr.iter().map(|&s| CString::new(s).unwrap()).collect();
        let ptrs: Vec<*const i8> = c_strings.iter().map(|s| s.as_ptr()).collect();
        (c_strings, ptrs)
    }

    #[test]
    fn test_division_by_zero() {
        // Example test for division by zero error
        let (_c_strings, ptrs) = to_c_string_array(&["4", "0", "/"]);
        let rpn = CReversePolishExpression {
            crpn_expression: ptrs.as_ptr(),
            length: ptrs.len(),
        };

        let result = unsafe { calculate_rpn(&rpn) };
        let err_msg = get_error_str(result.error_code);
        assert_eq!(err_msg, "Division by zero");
    }

    #[test]
    fn test_invalid_operator() {
        // Example test for Undefined variable in expression error
        let (_c_strings, ptrs) = to_c_string_array(&["4", "2", "@"]);
        let rpn = CReversePolishExpression {
            crpn_expression: ptrs.as_ptr(),
            length: ptrs.len(),
        };

        let result = unsafe { calculate_rpn(&rpn) };
        let err_msg = get_error_str(result.error_code);
        assert_eq!(err_msg, "Undefined variable in expression");
    }

    #[test]
    fn test_undefined_variable() {
        // Example test for undefined variable error
        let (_c_strings, ptrs) = to_c_string_array(&["banana"]);
        let rpn = CReversePolishExpression {
            crpn_expression: ptrs.as_ptr(),
            length: ptrs.len(),
        };

        let result = unsafe { calculate_rpn(&rpn) };
        let err_msg = get_error_str(result.error_code);
        assert_eq!(err_msg, "Undefined variable in expression");
    }

    #[test]
    fn test_stack_underflow() {
        // Example test for stack underflow error
        let (_c_strings, ptrs) = to_c_string_array(&["+", "3"]);
        let rpn = CReversePolishExpression {
            crpn_expression: ptrs.as_ptr(),
            length: ptrs.len(),
        };

        let result = unsafe { calculate_rpn(&rpn) };
        let err_msg = get_error_str(result.error_code);
        assert_eq!(err_msg, "Stack underflow - invalid expression");
    }

    #[test]
    fn test_undefined_variable_in_expression() {
        // Example test for undefined variable in expression error
        let (_c_strings, ptrs) = to_c_string_array(&["x", "2", "+"]);
        let rpn = CReversePolishExpression {
            crpn_expression: ptrs.as_ptr(),
            length: ptrs.len(),
        };

        let result = unsafe { calculate_rpn(&rpn) };
        let err_msg = get_error_str(result.error_code);
        assert_eq!(err_msg, "Undefined variable in expression");
    }
    #[test]
    fn test_memory_error_stack_size() {
        // Simulate memory error by exceeding MAX_STACK_SIZE
        let mut tokens = vec!["1"; 101]; // MAX_STACK_SIZE is 100, so use 101 tokens
        tokens.push("+"); // Add an operator to trigger stack usage

        let (_c_strings, ptrs) = to_c_string_array(&tokens);
        let rpn = CReversePolishExpression {
            crpn_expression: ptrs.as_ptr(),
            length: ptrs.len(),
        };

        let result = unsafe { calculate_rpn(&rpn) };
        let err_msg = get_error_str(result.error_code);
        assert_eq!(err_msg, "Stack maximum exceeded");
    }

    #[test]
    fn test_memory_error_expression_length() {
        // Simulate memory error by exceeding MAX_EXPR_LENGTH
        let long_expression = "1 ".repeat(1001); // MAX_EXPR_LENGTH is 1000, so use 1001 tokens
        let tokens: Vec<&str> = long_expression.trim().split_whitespace().collect();

        let (_c_strings, ptrs) = to_c_string_array(&tokens);
        let rpn = CReversePolishExpression {
            crpn_expression: ptrs.as_ptr(),
            length: ptrs.len(),
        };

        let result = unsafe { calculate_rpn(&rpn) };
        let err_msg = get_error_str(result.error_code);
        assert_eq!(err_msg, "Exprestion lenght maximum exceeded");
    }
}
