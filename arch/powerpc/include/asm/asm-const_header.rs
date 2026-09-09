// Translation of powerpc/include/asm/asm-const.h.
//
// The C header provides assembler/C preprocessor macros.  Rust has no
// equivalent of the assembler preprocessor branch; these macros preserve the
// corresponding expression and stringification behavior for Rust callers.

/// Equivalent of the C `stringify_in_c(...)` macro.
#[macro_export]
macro_rules! stringify_in_c {
    ($($value:tt)*) => {
        concat!(stringify!($($value)*), " ")
    };
}

/// Equivalent of the C `__ASM_CONST(x)` macro (`x##UL` in C).
///
/// The unsigned-long suffix is represented by the caller's Rust expression;
/// Rust does not support token-pasting an integer suffix in `macro_rules!`.
#[macro_export]
macro_rules! __ASM_CONST {
    ($value:expr) => {
        $value
    };
}

/// Equivalent of the C `ASM_CONST(x)` macro.
#[macro_export]
macro_rules! ASM_CONST {
    ($value:expr) => {
        $crate::__ASM_CONST!($value)
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
