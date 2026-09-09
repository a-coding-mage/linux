/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Dependency: <linux/netfilter/ipset/ip_set.h>

/* Bitmap type specific error codes */
pub const IPSET_ERR_BITMAP_RANGE: i32 = IPSET_ERR_TYPE_SPECIFIC;
/* The range exceeds the size limit of the set type */
pub const IPSET_ERR_BITMAP_RANGE_SIZE: i32 = IPSET_ERR_BITMAP_RANGE + 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
