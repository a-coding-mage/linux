/* SPDX-License-Identifier: GPL-2.0 */

// C preprocessor directives translated as local macro aliases.
// #undef LOCK
macro_rules! LOCK {
    ($($args:tt)*) => { ML!($($args)*) };
}

// #define LOCK ML
// #undef UNLOCK
macro_rules! UNLOCK {
    ($($args:tt)*) => { MU!($($args)*) };
}

// #define UNLOCK MU
// #undef RLOCK
// #undef WLOCK

// #undef INIT
macro_rules! INIT {
    ($($args:tt)*) => { MI!($($args)*) };
}

// #define INIT MI

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
