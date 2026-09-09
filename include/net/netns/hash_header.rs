/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by net/net_namespace.h.

#[inline]
pub unsafe fn net_hash_mix(net: *const net) -> u32 {
    (*net).hash_mix
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
