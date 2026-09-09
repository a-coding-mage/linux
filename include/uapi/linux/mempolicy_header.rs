/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * NUMA memory policies for Linux.
 * Copyright 2003,2004 Andi Kleen SuSE Labs
 */

// Dependency supplied by the surrounding UAPI translation: <linux/errno.h>

/*
 * Both the MPOL_* mempolicy mode and the MPOL_F_* optional mode flags are
 * passed by the user to either set_mempolicy() or mbind() in an 'int' actual.
 * The MPOL_MODE_FLAGS macro determines the legal set of optional mode flags.
 */

/* Policies */
#[repr(i32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum mempolicy_mode {
    MPOL_DEFAULT,
    MPOL_PREFERRED,
    MPOL_BIND,
    MPOL_INTERLEAVE,
    MPOL_LOCAL,
    MPOL_PREFERRED_MANY,
    MPOL_WEIGHTED_INTERLEAVE,
    MPOL_MAX, /* always last member of enum */
}

/* Flags for set_mempolicy */
pub const MPOL_F_STATIC_NODES: i32 = 1 << 15;
pub const MPOL_F_RELATIVE_NODES: i32 = 1 << 14;
pub const MPOL_F_NUMA_BALANCING: i32 = 1 << 13; /* Optimize with NUMA balancing if possible */

/*
 * MPOL_MODE_FLAGS is the union of all possible optional mode flags passed to
 * either set_mempolicy() or mbind().
 */
pub const MPOL_MODE_FLAGS: i32 =
    MPOL_F_STATIC_NODES | MPOL_F_RELATIVE_NODES | MPOL_F_NUMA_BALANCING;

/* Whether the nodemask is specified by users */
pub const MPOL_USER_NODEMASK_FLAGS: i32 = MPOL_F_STATIC_NODES | MPOL_F_RELATIVE_NODES;

/* Flags for get_mempolicy */
pub const MPOL_F_NODE: i32 = 1 << 0; /* return next IL mode instead of node mask */
pub const MPOL_F_ADDR: i32 = 1 << 1; /* look up vma using address */
pub const MPOL_F_MEMS_ALLOWED: i32 = 1 << 2; /* return allowed memories */

/* Flags for mbind */
pub const MPOL_MF_STRICT: i32 = 1 << 0; /* Verify existing pages in the mapping */
pub const MPOL_MF_MOVE: i32 = 1 << 1; /* Move pages owned by this process to conform
                                         to policy */
pub const MPOL_MF_MOVE_ALL: i32 = 1 << 2; /* Move every page to conform to policy */
pub const MPOL_MF_LAZY: i32 = 1 << 3; /* UNSUPPORTED FLAG: Lazy migrate on fault */
pub const MPOL_MF_INTERNAL: i32 = 1 << 4; /* Internal flags start here */

pub const MPOL_MF_VALID: i32 = MPOL_MF_STRICT | MPOL_MF_MOVE | MPOL_MF_MOVE_ALL;

/*
 * Internal flags that share the struct mempolicy flags word with
 * "mode flags".  These flags are allocated from bit 0 up, as they
 * are never OR'ed into the mode in mempolicy API arguments.
 */
pub const MPOL_F_SHARED: i32 = 1 << 0; /* identify shared policies */
pub const MPOL_F_MOF: i32 = 1 << 3; /* this policy wants migrate on fault */
pub const MPOL_F_MORON: i32 = 1 << 4; /* Migrate On protnone Reference On Node */

/*
 * Enabling zone reclaim means the page allocator will attempt to fulfill the
 * allocation request on the current node by triggering reclaim and
 * trying to shrink the current node.
 * Fallback allocations on the next candidates in the zonelist are considered
 * when reclaim fails to free up enough memory in the current node/zone.
 *
 * These bit locations are exposed in the vm.zone_reclaim_mode sysctl.
 * New bits are OK, but existing bits should not be changed.
 */
pub const RECLAIM_ZONE: i32 = 1 << 0; /* Enable zone reclaim */
pub const RECLAIM_WRITE: i32 = 1 << 1; /* Writeout pages during reclaim */
pub const RECLAIM_UNMAP: i32 = 1 << 2; /* Unmap pages during reclaim */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
