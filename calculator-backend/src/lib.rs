///./src/lib.rs 
/// This file contains the main logic for a calculator library that supports Reverse Polish Notation (RPN) and infix expressions.
/// It includes functions for tokenization, conversion between infix and RPN, and evaluation of expressions.
/// The library is designed to be used with a C library for evaluation, and it provides a C-compatible interface for integration.

//use serde::{Serialize, Deserialize};
use std::ffi::{CString, CStr, c_char};
use std::os::raw::{c_double, c_int};

// Error codes matching C
const SUCCESS: c_int = 0;
const DIVISION_BY_ZERO: c_int = 1;
const INVALID_OPERATOR: c_int = 2;
const STACK_UNDERFLOW: c_int = 3;
const MEMORY_ERROR: c_int = 4;
const UNDEFINED_VARIABLE: c_int = 5;
const STACK_MAXIMUM: c_int = 6;
const EXPR_LENGHT_MAXIMUM: c_int = 7;
const FACTORIAL_ERROR: c_int = 8;
const SQUARE_ROOT_ERROR: c_int = 9;
const LOG_ERROR: c_int = 10;
const LN_ERROR: c_int = 11;
const TAN_INVALID_OPERATOR: c_int = 12;
const INVALID_TRIG_OPERATOR: c_int = 13;

/// Stores the result of the calculation
/// S
/// # Fields
/// 
/// * `result_value`: The calculated result, a C-compatible double-precision floating-point number (`c_double`).
/// * `error_code`: An integer error code (`c_int`) indicating the success or failure of the calculation.
///     * `0` represents success.
///     * Other values indicate specific errors.
#[repr(C)]
#[derive(Debug)]
pub struct CCalculationResult {
    pub result_value: c_double,
    pub error_code: c_int,
}

/// Returns a human-readable error message based on the provided error code.
/// 
/// This function acts as a wrapper around the `get_error_message` function, providing
/// a simpler interface for retrieving error messages.
/// 
/// # Arguments
/// 
/// * `error_code`: An integer (`c_int`) representing the error code.
/// 
/// # Returns
/// 
/// A static string slice (`&'static str`) containing a human-readable description of the error.
/// 
/// # Supported Error Codes
/// 
/// * `0` (`SUCCESS`): "Success"
/// * `1` (`DIVISION_BY_ZERO`): "Division by zero"
/// * `2` (`INVALID_OPERATOR`): "Invalid operator"
/// * `3` (`STACK_UNDERFLOW`): "Stack underflow - invalid expression"
/// * `4` (`MEMORY_ERROR`): "Memory error"
/// * `5` (`UNDEFINED_VARIABLE`): "Undefined variable in expression"
/// * `6` (`STACK_MAXIMUM`): "Stack maximum exceeded"
/// * `7` (`EXPR_LENGHT_MAXIMUM`): "Expression length maximum exceeded"
/// * `8` (`FACTORIAL_ERROR`): "Factorial error"
/// * `9` (`SQUARE_ROOT_ERROR`): "Square root error"
/// * `10` (`LOG_ERROR`): "Log error"
/// * `11` (`LN_ERROR`): "Natural logarithm error"
/// * `12` (`TAN_INVALID_OPERATOR`): "Invalid operator for tangent"
/// * `13` (`INVALID_TRIG_OPERATOR`): "Invalid trigonometric operator"
/// * Any other value: "Unknown error"
/// 
pub fn get_error_str(error_code: c_int) -> &'static str {
    get_error_message(error_code)
}

/// Reverse Polish Notatio3n (RPN) as a C syle array
/// 
/// # Fields
/// 
/// * `crpn_expression`: A pointer (`*const *const c_char`) to a read-only array of C-style strings. Each string represents an element of the RPN expression.
/// * `length`: The number of elements in the `expression` array.
#[repr(C)]
#[derive(Debug)]
pub struct CReversePolishExpression {
    pub crpn_expression: *const *const c_char,
    pub length: usize,
}

/// Represents the result of converting an infix expression to Reverse Polish Notation (RPN) in a C-compatible format.
/// 
/// # Fields
/// 
/// * `result_expression`: A fixed-size array of C-style characters (`c_char`) representing the converted RPN expression.
/// * `error_code`: An integer error code (`c_int`) indicating the success or failure of the conversion.
///     * `0` represents success.
///     * Other values indicate specific errors.
#[repr(C)]
pub struct CConversionResult {
    result_expression: [c_char; 1000],  // MAX_EXPR_LENGTH from C
    error_code: c_int,
}

// Externally defined C functions for Reverse Polish Notation (RPN) calculations and conversions.
// 
// These functions are implemented in the C library and are exposed to Rust using the `extern "C"` block.
// 
// # Functions
// 
// ## `calculate_rpn`
// Evaluates a Reverse Polish Notation (RPN) expression and returns the result.
// 
// ### Arguments
// * `expr`: A pointer to a `CReversePolishExpression` struct, which contains the RPN expression to be evaluated.
// 
// ### Returns
// A `CCalculationResult` struct containing:
// * `result_value`: The calculated result as a `c_double`.
// * `error_code`: An integer (`c_int`) indicating the success or failure of the calculation.
//     * `0` (`SUCCESS`) indicates success.
//     * Other values indicate specific errors (e.g., `DIVISION_BY_ZERO`, `INVALID_OPERATOR`).
// 
// ## `convert_rpn_to_infix`
// Converts a Reverse Polish Notation (RPN) expression to an infix expression.
// 
// ### Arguments
// * `expr`: A pointer to a `CReversePolishExpression` struct, which contains the RPN expression to be converted.
// 
// ### Returns
// A `CConversionResult` struct containing:
// * `result_expression`: A fixed-size array of `c_char` representing the converted infix expression.
// * `error_code`: An integer (`c_int`) indicating the success or failure of the conversion.
//     * `0` (`SUCCESS`) indicates success.
//     * Other values indicate specific errors.
// 
// # Safety
// 
// These functions are marked as `unsafe` because they involve raw pointers and interaction with a C library.
// Ensure that the pointers passed to these functions are valid and properly aligned.
extern "C" {
    pub fn calculate_rpn(expr: *const CReversePolishExpression) -> CCalculationResult;
    fn convert_rpn_to_infix(expr: *const CReversePolishExpression) -> CConversionResult;
}

/// Represents a history entry for a mathematical expression and its result.
///  # Fields
/// 
/// * `input`: A string containing the original mathematical expression provided by the user. 
/// * `result`: An optional floating-point number (`Option<f64>`) representing the calculated result.
/// * `error_message`: An optional string (`Option<String>`) containing an error message if the calculation failed.
/// * `None` if there was no error.
/// 
///  This struct is used to log the history of calculations performed by the calculator.
#[derive(Debug, Clone)]
pub struct HistoryEntry {
    pub input: String,
    pub result: Option<f64>, // `None` if there was an error
    pub error_message: Option<String>, // `None` if there was no error
}

/// Represents the history of calculations performed by the calculator.
/// 
/// # Fields
/// 
/// * `entries`: A vector of `HistoryEntry` structs, each representing a single calculation.
/// * `last_result`: An optional floating-point number (`Option<f64>`) representing the result of the most recent successful calculation.
/// 
/// # Methods
/// 
/// ## `new`
/// Creates a new, empty `History` instance.
/// 
/// ## `add_entry`
/// Adds a new entry to the history.
/// 
/// ### Arguments
/// * `input`: A string representing the mathematical expression provided by the user.
/// * `result`: An optional floating-point number representing the result of the calculation. Use `None` if the calculation failed.
/// * `error_message`: An optional string containing an error message if the calculation failed. Use `None` if there was no error.
/// 
/// ## `get_history`
/// Returns a reference to the vector of `HistoryEntry` structs.
/// 
/// ## `get_last_result`
/// Returns the result of the most recent successful calculation, or `None` if no successful calculation has been performed.
#[derive(Debug)]
pub struct History {
    pub entries: Vec<HistoryEntry>,
    pub last_result: Option<f64>,
}

/// Represents the history of calculations performed by the calculator.
/// 
/// # Fields
/// 
/// * `entries`: A vector of `HistoryEntry` structs, each representing a single calculation.
/// * `last_result`: An optional floating-point number (`Option<f64>`) representing the result of the most recent successful calculation.
/// 
/// # Methods
/// 
/// ## `new`
/// Creates a new, empty `History` instance.
/// 
/// ## `add_entry`
/// Adds a new entry to the history.
/// 
/// ### Arguments
/// * `input`: A string representing the mathematical expression provided by the user.
/// * `result`: An optional floating-point number representing the result of the calculation. Use `None` if the calculation failed.
/// * `error_message`: An optional string containing an error message if the calculation failed. Use `None` if there was no error.
/// 
/// ## `get_history`
/// Returns a reference to the vector of `HistoryEntry` structs.
/// 
/// ## `get_last_result`
/// Returns the result of the most recent successful calculation, or `None` if no successful calculation has been performed.
impl History {
    pub fn new() -> Self {
        History {
            entries: Vec::new(),
            last_result: None,
        }
    }

    pub fn add_entry(&mut self, input: String, result: Option<f64>, error_message: Option<String>) {
        if let Some(res) = result {
            self.last_result = Some(res); // Update the last result
        }
        self.entries.push(HistoryEntry {
            input,
            result,
            error_message,
        });
    }

    pub fn get_history(&self) -> &Vec<HistoryEntry> {
        &self.entries
    }

    pub fn get_last_result(&self) -> Option<f64> {
        self.last_result
    }
}

/// Displays the history of calculations in a human-readable format.
/// 
/// # Arguments
/// 
/// * `history`: A reference to the `History` instance to be displayed.
pub fn display_history(history: &History) {
    for (i, entry) in history.get_history().iter().enumerate() {
        println!("Entry {}:", i + 1);
        println!("  Input: {}", entry.input);
        match &entry.result {
            Some(result) => println!("  Result: {}", result),
            None => println!("  Error: {}", entry.error_message.as_ref().unwrap()),
        }
    }
}


/// Reverse Polish Notation (RPN) expression represented as a vector of strings
/// 
/// # Fields
/// 
/// * `rp_expression`: A vector of strings (`Vec<String>`) representing the expression in Reverse Polish Notation (RPN).
///     * Each string in the vector is a single token of the RPN expression (e.g., "2", "3", "+").
pub struct ReversePolish {
    rp_expression: Vec<String>
}

/// Result of the calculations done through the C Library.
/// 
/// # Fields
/// 
/// * `success`: A boolean indicating whether the calculation was successful.
/// * `expression`: The original mathematical expression provided by the user.
/// * `rpn_expression`: The corresponding expression in Reverse Polish Notation (RPN).
/// * `result`: The numerical result of the calculation.
/// * `message`: An additional message providing details about the calculation outcome, 
///              such as errors or warnings.
impl ReversePolish {
    pub fn to_string(&self) -> String {
        self.rp_expression.join(" ")
    }

    pub fn to_c_expr(&self) -> Result<(Vec<CString>, Vec<*const c_char>), String> {
        let mut expr_cstrings = Vec::with_capacity(self.rp_expression.len());
        let mut expr_ptrs = Vec::with_capacity(self.rp_expression.len());
        
        for s in &self.rp_expression {
            let cstring = CString::new(s.as_str())
                .map_err(|_| "Failed to convert expression to CString".to_string())?;
            expr_ptrs.push(cstring.as_ptr());
            expr_cstrings.push(cstring);
        }

        Ok((expr_cstrings, expr_ptrs))
    }
}

/// Represents the result of a calculation performed using the C Library.
///
/// # Fields
///
/// * `success`: A boolean indicating whether the calculation was successful.
/// * `expression`: The original mathematical expression provided by the user.
/// * `rpn_expression`: The corresponding expression in Reverse Polish Notation (RPN).
/// * `result`: The numerical result of the calculation.
/// * `message`: A string containing additional details about the calculation outcome, 
///              such as errors or warnings.
pub struct CalculationResult {
    pub success: bool,
    pub expression: String,
    pub rpn_expression: String,
    pub result: f64,
    pub message: String,
}


/// Returns an error message based on given error code.
/// 
/// # Arguments
/// 
/// * 'error_code': An error code represented by a C-style integer.
/// 
/// # Returns
/// 
/// A static string slice containing a human-readable message describing the error.
pub fn get_error_message(error_code: c_int) -> &'static str {
    match error_code {
        SUCCESS => "Success",
        DIVISION_BY_ZERO => "Division by zero",
        INVALID_OPERATOR => "Invalid operator",
        STACK_UNDERFLOW => "Stack underflow - invalid expression",
        MEMORY_ERROR => "Memory error",
        UNDEFINED_VARIABLE => "Undefined variable in expression",
        STACK_MAXIMUM => "Stack maximum exceeded",
        EXPR_LENGHT_MAXIMUM => "Exprestion lenght maximum exceeded",
        FACTORIAL_ERROR => "Factorial error",
        SQUARE_ROOT_ERROR => "Square root error",
        LOG_ERROR => "Log error",
        LN_ERROR => "Natural logarithm error",
        TAN_INVALID_OPERATOR => "Invalid operator for tangent",
        INVALID_TRIG_OPERATOR => "Invalid trigonometric operator",
        _ => "Unknown error"
    }
}

/// Token types for the tokenization process.
///
/// * `Operator`: Represents mathematical operators such as `+`, `-`, `*`, `/`, `^`, `!`, and `√`.
///     - Trigonometric functions: "sin", "cos", "tan", "arcsin", "arccos", "arctan"
///     - Square root: "sqrt", "√"
///     - Logarithmic functions: "log" (base-10 logarithm), "ln" (natural logarithm)
///     - Special constants or functions: "ans"
/// * `Operand`: Represents numeric values, including integers and floating-point numbers.
///     * Example values: "2", "3.14", "10", "0"
/// * `Variable`: Represents alphanumeric symbols used as variables or identifiers.
///     * Example values: "x", "y", "result", "my_variable"
/// * `Bracket`: Represents parentheses used to group expressions.
///     * Example values: "(", ")"
#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Operator,
    Operand,
    Variable,
    Bracket,
    Function
}

/// Represents a single token produced during the tokenization process.
///
/// Each `Token` consists of a string `value` and its corresponding `TokenType`, which categorizes the token.
///
/// # Fields
///
/// * `token_value`: A string containing the actual content of the token.
/// * `token_type`: The type of the token, as defined by the `TokenType` enum.
#[derive(Debug, Clone)]
pub struct Token {
    token_value: String,
    token_type: TokenType,
}

/// Classifies an identifier as either an operator or a variable.
/// 
/// # Arguments
/// 
/// * `ident` - A string slice representing the identifier to be classified.
/// 
/// # Returns
/// 
/// A `TokenType` enum value:
/// - `TokenType::Operator` if the identifier matches a known operator name.
/// - `TokenType::Variable` if the identifier does not match any known operator name.
/// 
/// # Known Operators
/// 
/// The function recognizes the following operators:
/// - Trigonometric functions: `"sin"`, `"cos"`, `"tan"`, `"arcsin"`, `"arccos"`, `"arctan"`
/// - Square root: `"sqrt"`
/// - Logarithmic functions: `"log"` (base-10 logarithm), `"ln"` (natural logarithm)
/// - Special constants or functions: `"ans"`
fn classify_identifier(ident: &str, history: &History) -> TokenType {
    if ident == "ans" {
        // Always resolve `ans` to the last result
        if let Some(_last_result) = history.get_last_result() {

            TokenType::Operand
        } else {
            TokenType::Operand // Default to 0.0 if no previous result exists
        }
    } else {
        // Known operators
        const OPERATORS_NAMES: &[&str] = &[
            "sin", "cos", "tan", "arcsin", "arccos", "arctan", "sqrt", "log", "ln",
        ];
        if OPERATORS_NAMES.contains(&ident) {
            TokenType::Operator
        } else {
            TokenType::Variable
        }
    }
}

///Determines if token is a valid number. 
/// 
/// # Fields 
/// 
/// *`s`: A string slice representing the token
/// 
/// # Returns
/// 
/// A boolean:
/// -'true' if valid number (e.g 123,1.23,-1.23,)
/// -'false` otherwise
fn is_numeric(s: &str) -> bool {
    s.parse::<f64>().is_ok() //Standard rust parser that does a way better job than my implementation did :(
}

/// Tokenizes an input string into a vector of tokens.
/// 
/// # Arguments
/// 
/// * `input` - A string slice representing the input to be tokenized.
/// 
/// # Returns
/// 
/// A vector of `Token` structs, each representing a part of the input.
/// 
/// # Token Types
///
/// - `Operator`: Symbols representing mathematical operations (`+`, `-`, `*`, `/`, `^`, '!').
///     - Trigonometric functions: "sin", "cos", "tan", "arcsin", "arccos", "arctan"
///     - Square root: "sqrt"
///     - Logarithmic functions: "log" (base-10 logarithm), "ln" (natural logarithm)
///     - Special constants or functions: "ans"
/// - `Operand`: Numeric values, which can include decimal points (`2`, `3`, `4`, `9.23`, `0`).
/// - `Variable`: Alphanumeric symbols representing unknowns (`x`, `y`, `z`, `_`). (UNUSED CURRENTLY)
/// - `Bracket`: Parentheses used in expressions (`(`, `)`).
pub fn tokenize(input: &str, history: &History) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let binding = input.to_lowercase();
    let mut chars = binding.chars().peekable();
    let mut current_pos = 0; //Keeps track of where the tokenizer is in the expression
    let _input = input.replace("√", "sqrt");

    while let Some(&c) = chars.peek() {
        current_pos += 1; // Increment position conceptually for error messages

        match c {
            // === Whitespace ===
            w if w.is_whitespace() => {
                chars.next(); // Consume whitespace
            }

            // === Brackets ==
            '(' | ')' => {
                tokens.push(Token {
                    token_value: c.to_string(),
                    token_type: TokenType::Bracket,
                });
                chars.next(); // Consume bracket
            }

            // === Operators (excluding '-') ===
            '+' | '*' | '/' | '^' | '!' => {
            
                tokens.push(Token {
                    token_value: c.to_string(),
                    token_type: TokenType::Operator,
                });
                chars.next(); // Consume operator
            }
            '√' => {
                // Handle square root as a 
                tokens.push(Token {
                    token_value: "sqrt".to_string(),
                    token_type: TokenType::Operator,
                });
                chars.next(); // Consume '√'
            }
            // === Minus Sign or Negative Number(Operator or start of Number) ===
            '-' => {
                let is_unary = tokens.is_empty() || matches!(tokens.last(), Some(Token { token_type: TokenType::Operator | TokenType::Bracket, ..}));

                if is_unary {
                    // Potentially start of a new negative number
                    let mut num_string = String::new();
                    num_string.push(chars.next().unwrap()); // Consume '-'

                    //Consume rest of digits and one optional '.'
                    while let Some(&next_c) = chars.peek() {
                         if next_c.is_digit(10) || (next_c == '.' && !num_string.contains('.')) {
                            // Consume the character *only if* it's part of the number
                            num_string.push(chars.next().unwrap());
                         } else {
                             break; // Not part of number
                         }
                    }

                    //Validate if a negative number was formed
                    if is_numeric(&num_string) {
                        tokens.push(Token {
                            token_value: num_string, // num_string owns the value now
                            token_type: TokenType::Operand,
                        });
                    } else {
                        //If just "-". gets treated as an operator
                        if num_string == "-" {
                             tokens.push(Token {
                                token_value: "-".to_string(), 
                                token_type: TokenType::Operator,
                             });
                        } else {
                            eprintln!("Warning: Invalid sequence starting with '-' ('{}') at position {}", num_string, current_pos);
                             if num_string != "-" { // Avoid double-pushing if it was exactly "-"
                                tokens.push(Token {
                                    token_value: "-".to_string(),
                                    token_type: TokenType::Operator,
                                 });
                                 // The non-digit/dot character after '-' will be handled in the next loop iteration.
                             }

                        }
                    }
                } else {
                    //Binary operator
                    tokens.push(Token {
                        token_value: c.to_string(),
                        token_type: TokenType::Operator,
                    });
                    chars.next(); // Consume '-' operator
                }
            }

            // === Numbers (starting with digit or '.') ===
            d if d.is_digit(10) || d == '.' => {
                let mut num_str = String::new();
                let mut has_decimal = d == '.';
                num_str.push(chars.next().unwrap()); // Consume first digit or '.'

                while let Some(&next_c) = chars.peek() {
                    if next_c.is_digit(10) {
                        num_str.push(chars.next().unwrap());
                    } else if next_c == '.' && !has_decimal {
                        has_decimal = true;
                        num_str.push(chars.next().unwrap());
                    } else {
                        break; // End of number
                    }
                }

                // === Scientific Notation ===
                if let Some(&'e') = chars.peek() {
                    num_str.push(chars.next().unwrap()); // Consume e
                    
                    //Catch direction decimal is moving in
                    if let Some(&next_c) = chars.peek() {
                        if next_c == '-' || next_c == '+' {
                            num_str.push(chars.next().unwrap());
                        }

                        //Consume the rest of the number
                        while let Some(&next_c) = chars.peek() {
                            if next_c.is_digit(10) {
                                num_str.push(chars.next().unwrap());
                            } else {
                                break; 
                            }

                        }
                    }
                }

                // Validate if number was formed
                if is_numeric(&num_str) {
                    tokens.push(Token {
                        token_value: num_str,
                        token_type: TokenType::Operand,
                    });
                } else {
                    // Handle error: Invalid sequence like "." or "1.2.3"
                    eprintln!("Warning: Invalid numeric sequence '{}' at position {}", num_str, current_pos);
                    // How to recover? Skip? Push as unknown? For now, just warns.
                }
            }

            // ==== Identifier (Variables or Functions) ===
            // Original code only checked is_alphabetic(), let's add `|| a == '_'`
             a if a.is_alphabetic() => {
                let mut ident_str = String::new();
                ident_str.push(chars.next().unwrap());

                while let Some(&next_c) = chars.peek() {
                    if next_c.is_alphanumeric() {
                        ident_str.push(chars.next().unwrap());
                    } else {
                        break;
                    }
                }

                if ident_str == "ans" {
                    // Replace `ans` with the last result from history
                    let last_result = history.get_last_result().unwrap_or(0.0);
                    tokens.push(Token {
                        token_value: last_result.to_string(),
                        token_type: TokenType::Operand,
                    });
                } else {
                    let token_type = classify_identifier(&ident_str, history);
                    tokens.push(Token {
                        token_value: ident_str,
                        token_type,
                    });
                }
            }


            // == Unknown Character ===
            _ => {
                eprintln!("Warning: Skipping unknown character '{}' at position {}", c, current_pos);
                chars.next(); // Consume the unknown character
            }
        }
    }
    println!("Tokens: {:?}", tokens);
    tokens
}

/// Determines the precedence of an operator.
/// 
/// # Arguments
/// 
/// * `op`- A string slice representing the operator.
/// 
/// # Returns
/// 
/// An integer representing the precedence of the operator.
/// Returns 0 for unsupported or invalid operators.
pub fn get_precedence(op: &str) -> i32 {
    match op {
        "+" | "-" => 1,
        "*" | "/" => 2,
        "^" => 3,
        "!" | "√" | "sqrt" | "log" | "ln" => 4, 
        "sin"| "cos"| "tan" | "arctan"| "arcsin"| "arccos" => 5,
        _ => 0
    }
}

/// Determines if operator is a right associative.
/// 
/// #Arguments
/// 
/// * 'op': A string slice representing the operator being checked.
/// 
/// #Returns
/// 
/// A boolean value:
/// - `true` if the operator is right-associative (e.g., `^`, '!').
/// - `false` otherwise.
pub fn is_right_associative(op: &str) -> bool {
    op == "^" || op == "!" 
}

/// Takes an infix expression and converts it into a Reverse Polish Notation (RPN) expression.
///
/// # Arguments
///
/// * `input`: A string slice representing the infix expression to be converted to Reverse Polish Notation (RPN).
///
/// # Returns
///
/// * `Ok(ReversePolish)`: If the conversion is successful, where `ReversePolish` holds the Reverse Polish Notation (RPN) expression.
/// * `Err(String)`: If the conversion fails.
///
/// # Errors
///
/// This function returns an `Err(String)` with a descriptive error message in the following cases:
///
/// * "Mismatched parentheses": If the input expression has an unbalanced number of opening and closing parentheses.
/// * "Invalid token: ..." : If the tokenizer encounters an unexpected character or sequence of characters that cannot be recognized as a valid token.
pub fn infix_to_rpn(input: &str, _history: &History) -> Result<ReversePolish, String> {
    let tokens = tokenize(input, _history);
    let mut output = Vec::new();
    let mut op_stack: Vec<Token> = Vec::new();

    fn is_prefix_unary(op: &str) -> bool {
        op == "sqrt"
    }

    for token in tokens {
        match token.token_type {
            TokenType::Operand | TokenType::Variable => {
                output.push(token.token_value.clone());
                if let Some(last_op) = op_stack.last() {
                    if is_prefix_unary(&last_op.token_value) {
                        let op = op_stack.pop().unwrap();
                        output.push(op.token_value);
                    }
                }
            }
            TokenType::Operator => {
                if is_prefix_unary(&token.token_value) {
                    // Delay adding to output until we see the operand
                    op_stack.push(token);
                }else {
                    while let Some(top) = op_stack.last() {
                    if top.token_value == "(" {
                        break;
                    }
                    let curr_prec = get_precedence(&token.token_value);
                    let top_prec = get_precedence(&top.token_value);

                    if curr_prec > top_prec || (curr_prec == top_prec && is_right_associative(&token.token_value)) {
                        break;
                    }
                    if let Some(op) = op_stack.pop() {
                        output.push(op.token_value);
                    }
                    }
                    op_stack.push(token);
                }
            }
            TokenType::Bracket => {
                if token.token_value == "(" {
                    op_stack.push(token);
                } else {
                    while let Some(top) = op_stack.pop() {
                        if top.token_value == "(" {
                            break;
                        }
                        output.push(top.token_value);
                    }
                }
            }
            _ => {}
        }
    }

    while let Some(op) = op_stack.pop() {
        output.push(op.token_value);
    }

    Ok(ReversePolish {
        rp_expression: output,
    })
}

/// Endpoint to process a mathematical expression and return the result as a string.
/// 
/// #Arguments
/// * 'input': A string representation of the infix expression to be processed.
/// 
/// #Returns
/// 
/// 'CalculationResult' structure containing:
/// - `success`: A boolean indicating whether the calculation was successful.
/// - `expression`: The original mathematical expression provided by the user.
/// - `rpn_expression`: The corresponding expression in Reverse Polish Notation (RPN).
/// - `result`: The numerical result of the calculation.
/// - `message`: An additional message providing details about the calculation outcome, 
///              such as errors or warnings.
/// 
/// #Errors
/// 
/// This function returns an error code in the following cases:
/// - If the expression contains mismatched parentheses or cannot be tokenized, an error message is returned.
/// - If the conversion to C-compatible format fails, an error message is returned.
/// - If the C function for evaluation fails, an error message with details is included in the response.
pub fn calculate_expression(input: &str, history: &mut History) -> CalculationResult {
    let input = input.trim_matches('"');

    match infix_to_rpn(input, history) {
        Ok(rpn) => {
            let rpn_str = rpn.to_string();

            let (_expr_cstrings, expr_ptrs) = match rpn.to_c_expr() {
                Ok(data) => data,
                Err(e) => {
                    history.add_entry(
                        input.to_string(),
                        None,
                        Some(format!("Failed to convert to RPN: {}", e)),
                    );
                    return CalculationResult {
                        success: false,
                        expression: input.to_string(),
                        rpn_expression: rpn_str,
                        result: 0.0,
                        message: e,
                    };
                }
            };

            let c_expr = CReversePolishExpression {
                crpn_expression: expr_ptrs.as_ptr(),
                length: expr_ptrs.len(),
            };

            let result = unsafe { calculate_rpn(&c_expr) };

            let success = result.error_code == SUCCESS;
            let message = get_error_message(result.error_code).to_string();

            history.add_entry(
                input.to_string(),
                if success { Some(result.result_value) } else { None },
                if success { None } else { Some(message.clone()) },
            );

            CalculationResult {
                success,
                expression: input.to_string(),
                rpn_expression: rpn_str,
                result: result.result_value,
                message,
            }
        }
        Err(e) => {
            history.add_entry(
                input.to_string(),
                None,
                Some(format!("Failed to parse expression: {}", e)),
            );
            CalculationResult {
                success: false,
                expression: input.to_string(),
                rpn_expression: String::new(),
                result: 0.0,
                message: format!("Failed to parse expression: {}", e),
            }
        }
    }
}

/// Represents the result of a conversion between Reverse Polish Notation (RPN) and infix notation.
///
/// # Fields
///
/// * `success` - A boolean indicating whether the conversion was successful.
/// * `rpn_expression` - A string containing the RPN representation of the expression.
/// * `infix_expression` - A string containing the infix representation of the expression.
/// * `message` - A string with additional information, such as error messages or status notes.
pub struct ConversionResult {
    pub success: bool,
    pub rpn_expression: String,
    pub infix_expression: String,
    pub message: String,
}

// Make it accessible in tests
#[cfg(test)]
impl CalculationResult {
    pub fn success(&self) -> bool {
        self.success
    }

    pub fn result(&self) -> f64 {
        self.result
    }
}

/// Endpoint to convert a Reverse Polish Notation (RPN) expression to an infix expression.
/// 
/// #Arguments
/// 
/// * 'input': A string containing the RPN expression to be converted
/// 
/// #Returns
/// 
/// A 'CalculationResult' structure containing:
///  - `success`: A boolean indicating whether the calculation was successful.
/// - `expression`: The original mathematical expression provided by the user.
/// - `rpn_expression`: The corresponding expression in Reverse Polish Notation (RPN).
/// - `result`: The numerical result of the calculation.
/// - `message`: An additional message providing details about the calculation outcome, 
///              such as errors or warnings.
/// 
/// #Errors
/// 
/// This function returns an error code in the following cases:
/// - If the expression contains mismatched parentheses or cannot be tokenized, an error message is returned.
/// - If the conversion to C-compatible format fails, an error message is returned.
/// - If the C function for evaluation fails, an error message with details is included in the response.
pub fn convert_rpn(input: String) -> ConversionResult {
    let input = input.trim_matches('"');
    let tokens: Vec<String> = input.split_whitespace().map(String::from).collect();
    let rpn = ReversePolish { rp_expression: tokens };

    let (_expr_cstrings, expr_ptrs) = match rpn.to_c_expr() {
        Ok(data) => data,
        Err(e) => {
            return ConversionResult {
                success: false,
                rpn_expression: input.to_string(),
                infix_expression: String::new(),
                message: e
            };
        }
    };

    let c_expr = CReversePolishExpression {
        crpn_expression: expr_ptrs.as_ptr(),
        length: expr_ptrs.len(),
    };

    let result = unsafe {
        let c_result = convert_rpn_to_infix(&c_expr);
        let infix_expression = match CStr::from_ptr(c_result.result_expression.as_ptr()).to_str() {
            Ok(s) => s.to_owned(),
            Err(_) => return ConversionResult {
                success: false,
                rpn_expression: input.to_string(),
                infix_expression: String::new(),
                message: "Invalid UTF-8 in result".to_string()
            }
        };
        (c_result.error_code, infix_expression)
    };

    let (error_code, infix_expression) = result;
    let success = error_code == SUCCESS;
    let message = get_error_message(error_code).to_string();

    ConversionResult {
        success,
        rpn_expression: input.to_string(),
        infix_expression,
        message
    }
}
