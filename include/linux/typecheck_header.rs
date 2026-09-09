/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Check at compile time that something is of a particular type.
 * Always evaluates to 1 so you may use it easily in comparisons.
 *
 * The address comparison forces the two inferred types to agree, as in the
 * original GNU C typeof-based macro.
 */
#[macro_export]
macro_rules! typecheck {
    ($type:ty, $x:expr) => {{
        let __dummy: $type = unsafe { core::mem::MaybeUninit::uninit().assume_init() };
        let __dummy2 = $x;
        let _ = (&__dummy as *const $type, &__dummy2);
        1
    }};
}

/*
 * Check at compile time that 'function' is a certain type, or is a pointer
 * to that type (needs to use typedef for the function type.)
 */
#[macro_export]
macro_rules! typecheck_fn {
    ($type:ty, $function:expr) => {{
        let __tmp: $type = $function;
        let _ = __tmp;
    }};
}

/*
 * Check at compile time that something is a pointer type.
 */
#[macro_export]
macro_rules! typecheck_pointer {
    ($x:expr) => {{
        let __dummy = $x;
        let _ = unsafe { *__dummy };
        1
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
