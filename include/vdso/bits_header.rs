/* SPDX-License-Identifier: GPL-2.0 */

// Dependency intent: definitions corresponding to UL and ULL are supplied by
// vdso/const.h in the original header.

macro_rules! BIT {
    ($nr:expr) => {
        (1usize) << ($nr)
    };
}

macro_rules! BIT_ULL {
    ($nr:expr) => {
        (1u64) << ($nr)
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
