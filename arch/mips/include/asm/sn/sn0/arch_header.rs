/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * SGI IP27 specific setup.
 *
 * Copyright (C) 1995 - 1997, 1999 Silcon Graphics, Inc.
 * Copyright (C) 1999 Ralf Baechle (ralf@gnu.org)
 */

/*
 * MAXCPUS refers to the maximum number of CPUs in a single kernel.
 * This is not necessarily the same as MAXNODES * CPUS_PER_NODE
 */
pub const MAXCPUS: usize = MAX_NUMNODES * CPUS_PER_NODE;

/*
 * This is the maximum number of NASIDS that can be present in a system.
 * (Highest NASID plus one.)
 */
pub const MAX_NASIDS: usize = 256;

/*
 * MAX_REGIONS refers to the maximum number of hardware partitioned regions.
 */
pub const MAX_REGIONS: usize = 64;
pub const MAX_NONPREMIUM_REGIONS: usize = 16;
pub const MAX_PREMIUM_REGIONS: usize = MAX_REGIONS;

/*
 * MAX_PARITIONS refers to the maximum number of logically defined
 * partitions the system can support.
 */
pub const MAX_PARTITIONS: usize = MAX_REGIONS;

pub const NASID_MASK_BYTES: usize = (MAX_NASIDS + 7) / 8;

/*
 * Slot constants for SN0
 *
 * The C source selects the value at build time using
 * CONFIG_SGI_SN_N_MODE; the Rust cfg feature preserves that condition.
 */
#[cfg(feature = "CONFIG_SGI_SN_N_MODE")]
pub const MAX_MEM_SLOTS: usize = 16; /* max slots per node */

#[cfg(not(feature = "CONFIG_SGI_SN_N_MODE"))]
pub const MAX_MEM_SLOTS: usize = 32; /* max slots per node */

pub const SLOT_SHIFT: usize = 27;
pub const SLOT_MIN_MEM_SIZE: usize = 32 * 1024 * 1024;

pub const CPUS_PER_NODE: usize = 2; /* CPUs on a single hub */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
