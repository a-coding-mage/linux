/* SPDX-License-Identifier: GPL-2.0 */

// C preprocessor state:
// #undef LOCK
// #define LOCK L
#[macro_export]
macro_rules! LOCK {
    () => { L };
}

// C preprocessor state:
// #undef UNLOCK
// #define UNLOCK U
#[macro_export]
macro_rules! UNLOCK {
    () => { U };
}

// C preprocessor state:
// #undef RLOCK
// #undef WLOCK

// C preprocessor state:
// #undef INIT
// #define INIT SI
#[macro_export]
macro_rules! INIT {
    () => { SI };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
