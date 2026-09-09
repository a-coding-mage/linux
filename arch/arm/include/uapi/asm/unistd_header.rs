/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 *  arch/arm/include/asm/unistd.h
 *
 *  Copyright (C) 2001-2005 Russell King
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License version 2 as
 * published by the Free Software Foundation.
 *
 * Please forward _all_ changes to this file to rmk@arm.linux.org.uk,
 * no matter what the change is.  Thanks!
 */

pub const __NR_OABI_SYSCALL_BASE: u32 = 0x900000;
pub const __NR_SYSCALL_MASK: u32 = 0x0fffff;

/* The original header selects the EABI or OABI syscall table at build time. */
#[cfg(any(target_feature = "thumb-mode", arm_eabi))]
pub const __NR_SYSCALL_BASE: u32 = 0;

#[cfg(not(any(target_feature = "thumb-mode", arm_eabi)))]
pub const __NR_SYSCALL_BASE: u32 = __NR_OABI_SYSCALL_BASE;

/* Supplied by the selected asm/unistd-eabi.h or asm/unistd-oabi.h table. */
#[cfg(any(target_feature = "thumb-mode", arm_eabi))]
pub const __NR_sync_file_range2: u32 = __NR_arm_sync_file_range;

#[cfg(not(any(target_feature = "thumb-mode", arm_eabi)))]
pub const __NR_sync_file_range2: u32 = __NR_arm_sync_file_range;

/*
 * The following SWIs are ARM private.
 */
pub const __ARM_NR_BASE: u32 = __NR_SYSCALL_BASE + 0x0f0000;
pub const __ARM_NR_breakpoint: u32 = __ARM_NR_BASE + 1;
pub const __ARM_NR_cacheflush: u32 = __ARM_NR_BASE + 2;
pub const __ARM_NR_usr26: u32 = __ARM_NR_BASE + 3;
pub const __ARM_NR_usr32: u32 = __ARM_NR_BASE + 4;
pub const __ARM_NR_set_tls: u32 = __ARM_NR_BASE + 5;
pub const __ARM_NR_get_tls: u32 = __ARM_NR_BASE + 6;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
