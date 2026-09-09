/* SPDX-License-Identifier: GPL-2.0 */

// #undef LOCK
macro_rules! LOCK {
    () => { WSL };
}

// #undef UNLOCK
macro_rules! UNLOCK {
    () => { WSU };
}

// #undef RLOCK
macro_rules! RLOCK {
    () => { RSL };
}

// #undef WLOCK
macro_rules! WLOCK {
    () => { WSL };
}

// #undef INIT
macro_rules! INIT {
    () => { RWSI };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
