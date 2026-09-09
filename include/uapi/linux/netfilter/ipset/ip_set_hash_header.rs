/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Dependency: `linux/netfilter/ipset/ip_set.h` supplies `IPSET_ERR_TYPE_SPECIFIC`.

/* Hash type specific error codes */

/* Hash is full */
pub const IPSET_ERR_HASH_FULL: i32 = IPSET_ERR_TYPE_SPECIFIC;
/* Null-valued element */
pub const IPSET_ERR_HASH_ELEM: i32 = IPSET_ERR_HASH_FULL + 1;
/* Invalid protocol */
pub const IPSET_ERR_INVALID_PROTO: i32 = IPSET_ERR_HASH_ELEM + 1;
/* Protocol missing but must be specified */
pub const IPSET_ERR_MISSING_PROTO: i32 = IPSET_ERR_INVALID_PROTO + 1;
/* Range not supported */
pub const IPSET_ERR_HASH_RANGE_UNSUPPORTED: i32 = IPSET_ERR_MISSING_PROTO + 1;
/* Invalid range */
pub const IPSET_ERR_HASH_RANGE: i32 = IPSET_ERR_HASH_RANGE_UNSUPPORTED + 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
