/* SPDX-License-Identifier: GPL-2.0 */

// Dependency intent from <linux/compiler.h> is preserved by the C ABI
// declarations below. The source declarations are __attribute_const__.

use core::ffi::c_ulong;

extern "C" {
    pub fn lcm(a: c_ulong, b: c_ulong) -> c_ulong;
    pub fn lcm_not_zero(a: c_ulong, b: c_ulong) -> c_ulong;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
