/* SPDX-License-Identifier: GPL-2.0 */

// C preprocessor aliases translated as forwarding Rust macros. The referenced
// macros are supplied by dependencies of this header.
macro_rules! LOCK {
    ($($args:tt)*) => { RSL!($($args)*) };
}

macro_rules! UNLOCK {
    ($($args:tt)*) => { RSU!($($args)*) };
}

macro_rules! RLOCK {
    ($($args:tt)*) => { RSL!($($args)*) };
}

macro_rules! WLOCK {
    ($($args:tt)*) => { WSL!($($args)*) };
}

macro_rules! INIT {
    ($($args:tt)*) => { RWSI!($($args)*) };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
