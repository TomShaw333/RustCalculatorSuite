# Rust-Interface-Wrappers-Spring-25

Rust Foreign Function Interface Wrapper Crate  
RI: Daniel Dusharm ddusharm@asrcfederal.com

External bindings make the (computer) world go round. For example, Python AI/ML projects that use performance-critical or efficient libraries do so by leveraging foreign function interfaces (FFI) to call native code. This project demonstrates the development of a wrapper crate in Rust using `std::ffi` to expose the functionality of an external C library while ensuring type safety. Students will gain hands-on experience with Rust fundamentals such as memory safety, ownership, and FFI integration.

## Project Overview

This project implements a calculator library that demonstrates Rust-C FFI integration. It consists of a C library for core calculations and a Rust wrapper providing a safe interface through a command-line interface (CLI).

### Key Features

- Infix to RPN (Reverse Polish Notation) conversion
- Mathematical expression evaluation
- RESTful API interface
- Memory-safe FFI implementation
- Robust error handling
- Support for complex mathematical expressions
- History saves all user input with answers or errors
- Support for advanced mathematical functions:
  - Trigonometric functions: `sin`, `cos`, `tan`, `asin`, `acos`, `atan`
  - Logarithmic functions: `log` (base-10), `ln` (natural logarithm)
  - Factorial and square root operations
  - Constants: `pi`, `e`
- Scientific notation support for numbers
- Conversion between RPN and infix notation

## Installation

### Prerequisites
- Rust (latest stable)
- C compiler (gcc/clang)
- Cargo

### Build Instructions
1. Clone the repository:
   git clone https://github.com/your-repo/Rust-Interface-Wrappers-Spring-25.git
   cd Rust-Interface-Wrappers-Spring-25/caculator-frontend

2. Build the rust code
   cargo build

### Running the Application

To start the calculator application, run the following command:

cargo run --bin main

## Project Alignment

This implementation aligns with the project goals in several ways:

1. **FFI Integration**
   - Uses `std::ffi` for C interop
   - Handles C strings and data types
   - Manages memory across language boundaries

2. **Memory Safety**
   - Safe wrapper around unsafe C code
   - Proper memory management
   - Rust ownership rules enforcement

3. **Error Handling**
   - Comprehensive error types
   - Safe error propagation
   - User-friendly error messages

4. **Type Safety**
   - Strong type checking
   - Safe type conversions
   - Rust type system utilization

## Technical Details
Memory Management
The project handles memory safety through:

- RAII principles in Rust
- Careful C memory allocation/deallocation
- Smart pointer usage
- Automatic cleanup on scope exit

**Error Handling**

Error types include:
- Division by zero
- Invalid operators
- Stack underflow
- Memory errors
- Undefined variables
- Stack maximum exceeded
- Expression length maximum exceeded
- Factorial errors
- Square root errors
- Logarithmic errors (log and ln)
- Type Conversions
- Handles conversions between:
- Rust strings ↔ C strings
- Rust numbers ↔ C numbers
- Complex data structures
- Error codes
### Memory Management

The project handles memory safety through:
- RAII principles in Rust
- Careful C memory allocation/deallocation
- Smart pointer usage
- Automatic cleanup on scope exit

### Type Conversions

Handles conversions between:
- Rust strings ↔ C strings
- Rust numbers ↔ C numbers
- Complex data structures
- Error codes

## Testing

### Run all tests
cargo test

### Run a specific test
cargo test --test <test_name>

### Test Coverage
The project includes comprehensive test coverage for: 
- Basic arithmetic operations
- Advanced mathematical functions (e.g., trigonometric, logarithmic)
- Error handling (e.g., division by zero, invalid operators)
- History logging
- RPN to infix conversion
- Scientific notation
- Edge cases (e.g., large expressions, deeply nested parentheses)

## Limitations and Future Work

Current limitations:

- Limited support for user-defined variables
- No support for custom functions

Planned Improvements

1. Additional Features
- Variable support
- User-defined functions
- Dynamic memory allocation for large expressions

2. Performance Optimization
- Memory allocation optimization
- Parser performance improvements
- Error handling efficiency

3. Enhanced Testing
- Add stress tests for large expressions
- Add tests for Unicode handling
- Add tests for edge cases in scientific notation


### Documentation Updates

1. **API Documentation**
   - Document all supported operators
   - Add examples for all edge cases
   - Update error messages

### Performance Improvements

1. **Optimization Opportunities**
   - Memory allocation optimization
   - Parser performance improvements
   - Error handling efficiency
   - String handling optimization

## License

This project is part of academic coursework and is subject to course guidelines.

## Acknowledgments

- Daniel Dusharm (Project Advisor)
- ASRC Federal
- Dr. Neil Toporski (Course Instructor)
- Project Team Members

