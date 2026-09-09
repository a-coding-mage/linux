/* SPDX-License-Identifier: GPL-2.0 */

// Dependency intent: SZ_2M is supplied by linux/sizes.h.

/*
 * arm64 requires the DTB to be 8 byte aligned and
 * not exceed 2MB in size.
 */
pub const MIN_FDT_ALIGN: usize = 8;
pub const MAX_FDT_SIZE: usize = SZ_2M;

/*
 * arm64 requires the kernel image to placed at a 2 MB aligned base address
 */
pub const MIN_KIMG_ALIGN: usize = SZ_2M;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
