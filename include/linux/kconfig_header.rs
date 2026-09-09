/* SPDX-License-Identifier: GPL-2.0 */

// The generated configuration header is supplied by the surrounding build.

#[cfg(feature = "CONFIG_CPU_BIG_ENDIAN")]
pub const __BIG_ENDIAN: i32 = 4321;
#[cfg(not(feature = "CONFIG_CPU_BIG_ENDIAN"))]
pub const __LITTLE_ENDIAN: i32 = 1234;

// Rust has no preprocessor token-pasting equivalent in the language itself;
// these macros preserve the boolean expansion behavior for resolved options.
#[macro_export]
macro_rules! __ARG_PLACEHOLDER_1 {
    () => { 0 };
}

#[macro_export]
macro_rules! __take_second_arg {
    ($_ignored:expr, $val:expr $(, $rest:expr)*) => { $val };
}

/*
 * The use of "&&" / "||" is limited in certain expressions.
 * The following enable to calculate "and" / "or" with macro expansion only.
 */
#[macro_export]
macro_rules! __and {
    ($x:expr, $y:expr) => { (($x != 0) && ($y != 0)) as i32 };
}

#[macro_export]
macro_rules! ___and {
    ($x:expr, $y:expr) => { $crate::__and!($x, $y) };
}

#[macro_export]
macro_rules! ____and {
    ($x:expr, $y:expr) => { $crate::__and!($x, $y) };
}

#[macro_export]
macro_rules! __or {
    ($x:expr, $y:expr) => { (($x != 0) || ($y != 0)) as i32 };
}

#[macro_export]
macro_rules! ___or {
    ($x:expr, $y:expr) => { $crate::__or!($x, $y) };
}

#[macro_export]
macro_rules! ____or {
    ($x:expr, $y:expr) => { $crate::__or!($x, $y) };
}

/* Helper macros for resolved boolean and tristate configuration options. */
#[macro_export]
macro_rules! __is_defined {
    ($val:expr) => { (($val != 0) as i32) };
}

#[macro_export]
macro_rules! ___is_defined {
    ($val:expr) => { $crate::__is_defined!($val) };
}

#[macro_export]
macro_rules! ____is_defined {
    ($val:expr) => { $crate::__is_defined!($val) };
}

/* IS_BUILTIN(CONFIG_FOO) evaluates to 1 if CONFIG_FOO is set to 'y'. */
#[macro_export]
macro_rules! IS_BUILTIN {
    ($option:expr) => { $crate::__is_defined!($option) };
}

/* Pass the resolved CONFIG_FOO_MODULE value as the argument. */
#[macro_export]
macro_rules! IS_MODULE {
    ($option_module:expr) => { $crate::__is_defined!($option_module) };
}

#[macro_export]
macro_rules! IS_REACHABLE {
    ($builtin:expr, $module:expr, $current_module:expr) => {
        $crate::__or!($crate::IS_BUILTIN!($builtin),
                     $crate::__and!($crate::IS_MODULE!($module),
                                   $crate::__is_defined!($current_module)))
    };
}

#[macro_export]
macro_rules! IS_ENABLED {
    ($builtin:expr, $module:expr) => {
        $crate::__or!($crate::IS_BUILTIN!($builtin), $crate::IS_MODULE!($module))
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
