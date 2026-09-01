/* SPDX-License-Identifier: GPL-2.0 */

/* Original header guard: __LINUX_COMPILER_TYPES_H */

/* Builtins */

/*
 * __has_builtin is supported on gcc >= 10, clang >= 3 and icc >= 21.
 * In the meantime, to support gcc < 10, we implement __has_builtin
 * by hand.
 */
/* Rust has no direct equivalent for the C preprocessor __has_builtin(x).
 * If not provided by the build environment, the original C macro expands to 0.
 */
#[macro_export]
macro_rules! __has_builtin {
    ($x:tt) => {
        0
    };
}

/* Dependency intent from C include:
 * #include <linux/compiler-context-analysis.h>
 */

/* Compiler specific macros.
 * Original condition:
 * #ifdef __GNUC__
 * #include <linux/compiler-gcc.h>
 * #endif
 */

/* If asm_goto_output is not defined, C maps it to: asm goto(x).
 * Rust has no stable direct equivalent for GNU C's asm goto macro form.
 */
#[macro_export]
macro_rules! asm_goto_output {
    ($($x:tt)*) => {
        core::arch::asm!($($x)*)
    };
}

/*
 * __unqual_scalar_typeof(x) - Declare an unqualified scalar type, leaving
 *			       non-scalar types unchanged.
 */
/*
 * Prefer C11 _Generic for better compile-times and simpler code. Note: 'char'
 * is not type-compatible with 'signed char', and we define a separate case.
 */
/* Original C helper macro:
 * #define __scalar_type_to_expr_cases(type) \
 *         unsigned type: (unsigned type)0, \
 *         signed type:   (signed type)0
 *
 * This is only meaningful inside the C11 _Generic association list below.
 */
#[macro_export]
macro_rules! __scalar_type_to_expr_cases {
    ($type:tt) => {
        /* C _Generic cases for unsigned $type and signed $type. */
    };
}

/* Original C macro:
 * #define __unqual_scalar_typeof(x) typeof( \
 *         _Generic((x), \
 *                  char: (char)0, \
 *                  __scalar_type_to_expr_cases(char), \
 *                  __scalar_type_to_expr_cases(short), \
 *                  __scalar_type_to_expr_cases(int), \
 *                  __scalar_type_to_expr_cases(long), \
 *                  __scalar_type_to_expr_cases(long long), \
 *                  default: (x)))
 *
 * Rust has no source-level equivalent for GNU typeof plus C11 _Generic.
 * The closest local translation preserves the macro interface and yields the
 * Rust expression's type without changing value category or qualifiers.
 */
#[macro_export]
macro_rules! __unqual_scalar_typeof {
    ($x:expr) => {
        $x
    };
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
