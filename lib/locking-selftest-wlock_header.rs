/* SPDX-License-Identifier: GPL-2.0 */

// #undef LOCK
macro_rules! LOCK {
    () => { WL };
}

// #undef UNLOCK
macro_rules! UNLOCK {
    () => { WU };
}

// #undef RLOCK
macro_rules! RLOCK {
    () => { RL };
}

// #undef WLOCK
macro_rules! WLOCK {
    () => { WL };
}

// #undef INIT
macro_rules! INIT {
    () => { RWI };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
