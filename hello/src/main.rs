fn main() {
    println!("Hello, world!");
}

// The isize and usize types hold pointer-sized signed
// and unsigned integers, 32 bits long on 32-bit platforms, and 64 bits long on
// 64-bit platforms.

//The ! character marks this as a macro invocation,
// not a function call.
//
// Unlike C and C++, in which
// assertions can be skipped, Rust always checks assertions regardless of how
// the program was compiled. There is also a debug_assert! macro, whose
// assertions are skipped when the program is compiled for speed.

//A let statement declares a local variable, like t in our function. We don’t
// need to write out t’s type, as long as Rust can infer it from how the variable
// is used.
//
// Rust only infers types within function bodies: you must write out the
// types of function parameters and return values, as we did before. If we
// wanted to spell out t’s type, we could write:
// let t:

//Rust has a return statement, but the gcd function doesn’t need one. If a
// function body ends with an expression that is not followed by a semicolon,
// that’s the function’s return value. In fact, any block surrounded by curly
// braces can function as an expression.
