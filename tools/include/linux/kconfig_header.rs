/* SPDX-License-Identifier: GPL-2.0 */

/* CONFIG_CC_VERSION_TEXT (Do not delete this comment. See help in Kconfig) */

#[macro_export]
macro_rules! __ARG_PLACEHOLDER_1 {
    () => {
        0,
    };
}

#[macro_export]
macro_rules! __take_second_arg {
    ($ignored:expr, $val:expr $(, $rest:expr)*) => {
        $val
    };
}

/*
 * The use of "&&" / "||" is limited in certain expressions.
 * The following enable to calculate "and" / "or" with macro expansion only.
 */
#[macro_export]
macro_rules! __and {
    ($x:expr, $y:expr) => {
        (($x) != 0 && ($y) != 0) as i32
    };
}

#[macro_export]
macro_rules! ___and {
    ($x:expr, $y:expr) => {
        $crate::__and!($x, $y)
    };
}

#[macro_export]
macro_rules! ____and {
    ($arg1_or_junk:expr, $y:expr) => {
        (($arg1_or_junk) != 0 && ($y) != 0) as i32
    };
}

#[macro_export]
macro_rules! __or {
    ($x:expr, $y:expr) => {
        (($x) != 0 || ($y) != 0) as i32
    };
}

#[macro_export]
macro_rules! ___or {
    ($x:expr, $y:expr) => {
        $crate::__or!($x, $y)
    };
}

#[macro_export]
macro_rules! ____or {
    ($arg1_or_junk:expr, $y:expr) => {
        (($arg1_or_junk) != 0 || ($y) != 0) as i32
    };
}

/*
 * Helper macros to use CONFIG_ options in C/CPP expressions. Note that
 * these only work with boolean and tristate options.
 */

/*
 * Getting something that works in C and CPP for an arg that may or may
 * not be defined is tricky.  Here, if we have "#define CONFIG_BOOGER 1"
 * we match on the placeholder define, insert the "0," for arg1 and generate
 * the triplet (0, 1, 0).  Then the last step cherry picks the 2nd arg (a one).
 * When CONFIG_BOOGER is not defined, we generate a (... 1, 0) pair, and when
 * the last step cherry picks the 2nd arg, we get a zero.
 */
#[macro_export]
macro_rules! __is_defined {
    (1) => {
        1
    };
    ($x:tt) => {
        0
    };
}

#[macro_export]
macro_rules! ___is_defined {
    ($val:tt) => {
        $crate::__is_defined!($val)
    };
}

#[macro_export]
macro_rules! ____is_defined {
    ($arg1_or_junk:tt) => {
        $crate::__is_defined!($arg1_or_junk)
    };
}

/*
 * IS_BUILTIN(CONFIG_FOO) evaluates to 1 if CONFIG_FOO is set to 'y', 0
 * otherwise. For boolean options, this is equivalent to
 * IS_ENABLED(CONFIG_FOO).
 */
#[macro_export]
macro_rules! IS_BUILTIN {
    ($option:tt) => {
        $crate::__is_defined!($option)
    };
}

/*
 * IS_MODULE(CONFIG_FOO) evaluates to 1 if CONFIG_FOO is set to 'm', 0
 * otherwise.
 *
 * The original C macro probes option##_MODULE using preprocessor token
 * concatenation. Rust macro_rules! cannot form and resolve a new identifier
 * from an input token without an external dependency, so callers must pass the
 * module-state token directly when using this translated header in isolation.
 */
#[macro_export]
macro_rules! IS_MODULE {
    ($option_module:tt) => {
        $crate::__is_defined!($option_module)
    };
}

/*
 * IS_REACHABLE(CONFIG_FOO) evaluates to 1 if the currently compiled
 * code can call a function defined in code compiled based on CONFIG_FOO.
 * This is similar to IS_ENABLED(), but returns false when invoked from
 * built-in code when CONFIG_FOO is set to 'm'.
 */
#[macro_export]
macro_rules! IS_REACHABLE {
    ($option:tt) => {
        $crate::__or!(
            $crate::IS_BUILTIN!($option),
            $crate::__and!($crate::IS_MODULE!($option), $crate::__is_defined!(MODULE))
        )
    };
}

/*
 * IS_ENABLED(CONFIG_FOO) evaluates to 1 if CONFIG_FOO is set to 'y' or 'm',
 * 0 otherwise.
 */
#[macro_export]
macro_rules! IS_ENABLED {
    ($option:tt) => {
        $crate::__or!($crate::IS_BUILTIN!($option), $crate::IS_MODULE!($option))
    };
}
