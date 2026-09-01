/* SPDX-License-Identifier: GPL-2.0 */

/* Subset of include/linux/kconfig.h */

macro_rules! __ARG_PLACEHOLDER_1 {
    () => {
        0,
    };
}

macro_rules! __take_second_arg {
    ($__ignored:tt, $val:expr $(, $rest:tt)*) => {
        $val
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
macro_rules! __is_defined {
    (1) => {
        1
    };
    ($($x:tt)*) => {
        0
    };
}

macro_rules! ___is_defined {
    ($val:tt) => {
        __is_defined!($val)
    };
}

macro_rules! ____is_defined {
    ($arg1_or_junk:tt) => {
        __take_second_arg!($arg1_or_junk 1, 0)
    };
}

/*
 * IS_BUILTIN(CONFIG_FOO) evaluates to 1 if CONFIG_FOO is set to 'y', 0
 * otherwise. For boolean options, this is equivalent to
 * IS_ENABLED(CONFIG_FOO).
 */
macro_rules! IS_BUILTIN {
    ($option:tt) => {
        __is_defined!($option)
    };
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
