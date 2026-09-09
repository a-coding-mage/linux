/* SPDX-License-Identifier: GPL-2.0 */

// Dependency: declarations corresponding to <uapi/linux/const.h> are supplied
// by other translated files.

macro_rules! UL {
    ($x:expr) => {
        _UL($x)
    };
}

macro_rules! ULL {
    ($x:expr) => {
        _ULL($x)
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
