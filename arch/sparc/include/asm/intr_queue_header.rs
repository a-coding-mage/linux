/* SPDX-License-Identifier: GPL-2.0 */

/* Sun4v interrupt queue registers, accessed via ASI_QUEUE. */

pub const INTRQ_CPU_MONDO_HEAD: usize = 0x3c0; /* CPU mondo head */
pub const INTRQ_CPU_MONDO_TAIL: usize = 0x3c8; /* CPU mondo tail */
pub const INTRQ_DEVICE_MONDO_HEAD: usize = 0x3d0; /* Device mondo head */
pub const INTRQ_DEVICE_MONDO_TAIL: usize = 0x3d8; /* Device mondo tail */
pub const INTRQ_RESUM_MONDO_HEAD: usize = 0x3e0; /* Resumable error mondo head */
pub const INTRQ_RESUM_MONDO_TAIL: usize = 0x3e8; /* Resumable error mondo tail */
pub const INTRQ_NONRESUM_MONDO_HEAD: usize = 0x3f0; /* Non-resumable error mondo head */
pub const INTRQ_NONRESUM_MONDO_TAIL: usize = 0x3f8; /* Non-resumable error mondo head */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
