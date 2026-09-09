/* SPDX-License-Identifier: GPL-2.0 */

// The original header undefines and redefines these object-like C macros.
// Rust has no direct object-like macro alias syntax; these macros preserve
// the token substitution intent and require invocation with `!()`.
macro_rules! LOCK {
    () => { RL };
}

macro_rules! UNLOCK {
    () => { RU };
}

macro_rules! RLOCK {
    () => { RL };
}

macro_rules! WLOCK {
    () => { WL };
}

macro_rules! INIT {
    () => { RWI };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
