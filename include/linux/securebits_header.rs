/* SPDX-License-Identifier: GPL-2.0 */

// Declarations from <uapi/linux/securebits.h> are supplied by another
// translation unit.

macro_rules! issecure {
    ($x:expr) => {
        issecure_mask($x) & current_cred_xxx!(securebits)
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
