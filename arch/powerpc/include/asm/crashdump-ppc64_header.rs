/* SPDX-License-Identifier: GPL-2.0-only */

/*
 * Backup region - first 64KB of System RAM
 *
 * If ever the below macros are to be changed, please be judicious.
 * The implicit assumptions are:
 *     - start, end & size are less than UINT32_MAX.
 *     - start & size are at least 8 byte aligned.
 *
 * For implementation details: arch/powerpc/purgatory/trampoline_64.S
 */
pub const BACKUP_SRC_START: u32 = 0;
pub const BACKUP_SRC_END: u32 = 0xffff;
pub const BACKUP_SRC_SIZE: u32 = BACKUP_SRC_END - BACKUP_SRC_START + 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
