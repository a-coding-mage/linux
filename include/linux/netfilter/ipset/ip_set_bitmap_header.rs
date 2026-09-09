/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by the corresponding uapi header:
// #include <uapi/linux/netfilter/ipset/ip_set_bitmap.h>

pub const IPSET_BITMAP_MAX_RANGE: u32 = 0x0000_FFFF;

pub const IPSET_ADD_STORE_PLAIN_TIMEOUT: i32 = -1;
pub const IPSET_ADD_FAILED: i32 = 1;
pub const IPSET_ADD_START_STORED_TIMEOUT: i32 = 2;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
