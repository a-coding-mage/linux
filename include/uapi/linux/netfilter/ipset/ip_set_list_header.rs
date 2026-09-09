/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Dependency: IPSET_ERR_TYPE_SPECIFIC is provided by
// <linux/netfilter/ipset/ip_set.h>.

/* List type specific error codes */
pub const IPSET_ERR_NAME: i32 = IPSET_ERR_TYPE_SPECIFIC;
pub const IPSET_ERR_LOOP: i32 = IPSET_ERR_NAME + 1;
pub const IPSET_ERR_BEFORE: i32 = IPSET_ERR_LOOP + 1;
pub const IPSET_ERR_NAMEREF: i32 = IPSET_ERR_BEFORE + 1;
pub const IPSET_ERR_LIST_FULL: i32 = IPSET_ERR_NAMEREF + 1;
pub const IPSET_ERR_REF_EXIST: i32 = IPSET_ERR_LIST_FULL + 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
