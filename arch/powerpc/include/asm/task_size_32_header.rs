/* SPDX-License-Identifier: GPL-2.0 */

/*
 * The original header guard and include directive are C-only.  The size
 * constants and configuration constants referenced below are supplied by
 * other translated headers.
 */

/*
 * #if CONFIG_TASK_SIZE > CONFIG_KERNEL_START
 * #error User TASK_SIZE overlaps with KERNEL_START address
 * #endif
 *
 * This build-time diagnostic is preserved as conditional intent; Rust builds
 * should enforce the same configuration invariant.
 */

#[cfg(feature = "CONFIG_PPC_8xx")]
pub const MODULES_END: usize = CONFIG_PAGE_OFFSET;

#[cfg(feature = "CONFIG_PPC_8xx")]
pub const MODULES_SIZE: usize = CONFIG_MODULES_SIZE * SZ_1M;

#[cfg(feature = "CONFIG_PPC_8xx")]
pub const MODULES_VADDR: usize = MODULES_END - MODULES_SIZE;

#[cfg(feature = "CONFIG_PPC_8xx")]
pub const MODULES_BASE: usize = MODULES_VADDR & !(SZ_4M - 1);

#[cfg(feature = "CONFIG_PPC_8xx")]
pub const USER_TOP: usize = MODULES_BASE - SZ_4M;

#[cfg(feature = "CONFIG_PPC_BOOK3S_32")]
pub const MODULES_END: usize = CONFIG_PAGE_OFFSET & !(SZ_256M - 1);

#[cfg(feature = "CONFIG_PPC_BOOK3S_32")]
pub const MODULES_SIZE: usize = CONFIG_MODULES_SIZE * SZ_1M;

#[cfg(feature = "CONFIG_PPC_BOOK3S_32")]
pub const MODULES_VADDR: usize = MODULES_END - MODULES_SIZE;

#[cfg(feature = "CONFIG_PPC_BOOK3S_32")]
pub const MODULES_BASE: usize = MODULES_VADDR & !(SZ_256M - 1);

#[cfg(feature = "CONFIG_PPC_BOOK3S_32")]
pub const USER_TOP: usize = MODULES_BASE - SZ_4M;

#[cfg(not(any(feature = "CONFIG_PPC_8xx", feature = "CONFIG_PPC_BOOK3S_32")))]
pub const USER_TOP: usize = (CONFIG_PAGE_OFFSET - SZ_128K) & !(SZ_128K - 1);

#[cfg(feature = "CONFIG_TASK_SIZE_LT_USER_TOP")]
pub const TASK_SIZE: usize = CONFIG_TASK_SIZE;

#[cfg(not(feature = "CONFIG_TASK_SIZE_LT_USER_TOP"))]
pub const TASK_SIZE: usize = USER_TOP;

/*
 * This decides where the kernel will search for a free chunk of vm space
 * during mmap's.
 */
pub const TASK_UNMAPPED_BASE: usize = TASK_SIZE / 8 * 3;

pub const DEFAULT_MAP_WINDOW: usize = TASK_SIZE;
pub const STACK_TOP: usize = TASK_SIZE;
pub const STACK_TOP_MAX: usize = STACK_TOP;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
