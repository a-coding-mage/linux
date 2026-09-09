/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * ARM Power State and Coordination Interface (PSCI) header
 *
 * This header holds common PSCI defines and macros shared
 * by: ARM kernel, ARM64 kernel, KVM ARM/ARM64 and user space.
 *
 * Copyright (C) 2014 Linaro Ltd.
 * Author: Anup Patel <anup.patel@linaro.org>
 */

/* PSCI v0.1 interface
 *
 * The PSCI v0.1 function numbers are implementation defined.
 *
 * Only PSCI return values such as: SUCCESS, NOT_SUPPORTED,
 * INVALID_PARAMS, and DENIED defined below are applicable
 * to PSCI v0.1.
 */

/* PSCI v0.2 interface */
pub const PSCI_0_2_FN_BASE: u32 = 0x84000000;
pub const fn PSCI_0_2_FN(n: u32) -> u32 { PSCI_0_2_FN_BASE + n }
pub const PSCI_0_2_64BIT: u32 = 0x40000000;
pub const PSCI_0_2_FN64_BASE: u32 = PSCI_0_2_FN_BASE + PSCI_0_2_64BIT;
pub const fn PSCI_0_2_FN64(n: u32) -> u32 { PSCI_0_2_FN64_BASE + n }

pub const PSCI_0_2_FN_PSCI_VERSION: u32 = PSCI_0_2_FN(0);
pub const PSCI_0_2_FN_CPU_SUSPEND: u32 = PSCI_0_2_FN(1);
pub const PSCI_0_2_FN_CPU_OFF: u32 = PSCI_0_2_FN(2);
pub const PSCI_0_2_FN_CPU_ON: u32 = PSCI_0_2_FN(3);
pub const PSCI_0_2_FN_AFFINITY_INFO: u32 = PSCI_0_2_FN(4);
pub const PSCI_0_2_FN_MIGRATE: u32 = PSCI_0_2_FN(5);
pub const PSCI_0_2_FN_MIGRATE_INFO_TYPE: u32 = PSCI_0_2_FN(6);
pub const PSCI_0_2_FN_MIGRATE_INFO_UP_CPU: u32 = PSCI_0_2_FN(7);
pub const PSCI_0_2_FN_SYSTEM_OFF: u32 = PSCI_0_2_FN(8);
pub const PSCI_0_2_FN_SYSTEM_RESET: u32 = PSCI_0_2_FN(9);

pub const PSCI_0_2_FN64_CPU_SUSPEND: u32 = PSCI_0_2_FN64(1);
pub const PSCI_0_2_FN64_CPU_ON: u32 = PSCI_0_2_FN64(3);
pub const PSCI_0_2_FN64_AFFINITY_INFO: u32 = PSCI_0_2_FN64(4);
pub const PSCI_0_2_FN64_MIGRATE: u32 = PSCI_0_2_FN64(5);
pub const PSCI_0_2_FN64_MIGRATE_INFO_UP_CPU: u32 = PSCI_0_2_FN64(7);

pub const PSCI_1_0_FN_PSCI_FEATURES: u32 = PSCI_0_2_FN(10);
pub const PSCI_1_0_FN_CPU_FREEZE: u32 = PSCI_0_2_FN(11);
pub const PSCI_1_0_FN_CPU_DEFAULT_SUSPEND: u32 = PSCI_0_2_FN(12);
pub const PSCI_1_0_FN_NODE_HW_STATE: u32 = PSCI_0_2_FN(13);
pub const PSCI_1_0_FN_SYSTEM_SUSPEND: u32 = PSCI_0_2_FN(14);
pub const PSCI_1_0_FN_SET_SUSPEND_MODE: u32 = PSCI_0_2_FN(15);
pub const PSCI_1_0_FN_STAT_RESIDENCY: u32 = PSCI_0_2_FN(16);
pub const PSCI_1_0_FN_STAT_COUNT: u32 = PSCI_0_2_FN(17);

pub const PSCI_1_1_FN_SYSTEM_RESET2: u32 = PSCI_0_2_FN(18);
pub const PSCI_1_1_FN_MEM_PROTECT: u32 = PSCI_0_2_FN(19);
pub const PSCI_1_1_FN_MEM_PROTECT_CHECK_RANGE: u32 = PSCI_0_2_FN(20);
pub const PSCI_1_3_FN_SYSTEM_OFF2: u32 = PSCI_0_2_FN(21);

pub const PSCI_1_0_FN64_CPU_DEFAULT_SUSPEND: u32 = PSCI_0_2_FN64(12);
pub const PSCI_1_0_FN64_NODE_HW_STATE: u32 = PSCI_0_2_FN64(13);
pub const PSCI_1_0_FN64_SYSTEM_SUSPEND: u32 = PSCI_0_2_FN64(14);
pub const PSCI_1_0_FN64_STAT_RESIDENCY: u32 = PSCI_0_2_FN64(16);
pub const PSCI_1_0_FN64_STAT_COUNT: u32 = PSCI_0_2_FN64(17);

pub const PSCI_1_1_FN64_SYSTEM_RESET2: u32 = PSCI_0_2_FN64(18);
pub const PSCI_1_1_FN64_MEM_PROTECT_CHECK_RANGE: u32 = PSCI_0_2_FN64(20);
pub const PSCI_1_3_FN64_SYSTEM_OFF2: u32 = PSCI_0_2_FN64(21);

/* PSCI v0.2 power state encoding for CPU_SUSPEND function */
pub const PSCI_0_2_POWER_STATE_ID_MASK: u32 = 0xffff;
pub const PSCI_0_2_POWER_STATE_ID_SHIFT: u32 = 0;
pub const PSCI_0_2_POWER_STATE_TYPE_SHIFT: u32 = 16;
pub const PSCI_0_2_POWER_STATE_TYPE_MASK: u32 = 0x1 << PSCI_0_2_POWER_STATE_TYPE_SHIFT;
pub const PSCI_0_2_POWER_STATE_AFFL_SHIFT: u32 = 24;
pub const PSCI_0_2_POWER_STATE_AFFL_MASK: u32 = 0x3 << PSCI_0_2_POWER_STATE_AFFL_SHIFT;

/* PSCI extended power state encoding for CPU_SUSPEND function */
pub const PSCI_1_0_EXT_POWER_STATE_ID_MASK: u32 = 0xfffffff;
pub const PSCI_1_0_EXT_POWER_STATE_ID_SHIFT: u32 = 0;
pub const PSCI_1_0_EXT_POWER_STATE_TYPE_SHIFT: u32 = 30;
pub const PSCI_1_0_EXT_POWER_STATE_TYPE_MASK: u32 = 0x1 << PSCI_1_0_EXT_POWER_STATE_TYPE_SHIFT;

/* PSCI v0.2 affinity level state returned by AFFINITY_INFO */
pub const PSCI_0_2_AFFINITY_LEVEL_ON: u32 = 0;
pub const PSCI_0_2_AFFINITY_LEVEL_OFF: u32 = 1;
pub const PSCI_0_2_AFFINITY_LEVEL_ON_PENDING: u32 = 2;

/* PSCI v0.2 multicore support in Trusted OS returned by MIGRATE_INFO_TYPE */
pub const PSCI_0_2_TOS_UP_MIGRATE: u32 = 0;
pub const PSCI_0_2_TOS_UP_NO_MIGRATE: u32 = 1;
pub const PSCI_0_2_TOS_MP: u32 = 2;

/* PSCI v1.1 reset type encoding for SYSTEM_RESET2 */
pub const PSCI_1_1_RESET_TYPE_SYSTEM_WARM_RESET: u32 = 0;
pub const PSCI_1_1_RESET_TYPE_VENDOR_START: u32 = 0x80000000;

/* PSCI v1.3 hibernate type for SYSTEM_OFF2 */
pub const PSCI_1_3_OFF_TYPE_HIBERNATE_OFF: u32 = 1u32 << 0;

/* PSCI version decoding (independent of PSCI version) */
pub const PSCI_VERSION_MAJOR_SHIFT: u32 = 16;
pub const PSCI_VERSION_MINOR_MASK: u32 = (1u32 << PSCI_VERSION_MAJOR_SHIFT) - 1;
pub const PSCI_VERSION_MAJOR_MASK: u32 = !PSCI_VERSION_MINOR_MASK;
pub const fn PSCI_VERSION_MAJOR(ver: u32) -> u32 {
    (ver & PSCI_VERSION_MAJOR_MASK) >> PSCI_VERSION_MAJOR_SHIFT
}
pub const fn PSCI_VERSION_MINOR(ver: u32) -> u32 { ver & PSCI_VERSION_MINOR_MASK }
pub const fn PSCI_VERSION(maj: u32, min: u32) -> u32 {
    ((maj << PSCI_VERSION_MAJOR_SHIFT) & PSCI_VERSION_MAJOR_MASK) |
        (min & PSCI_VERSION_MINOR_MASK)
}

/* PSCI features decoding (>=1.0) */
pub const PSCI_1_0_FEATURES_CPU_SUSPEND_PF_SHIFT: u32 = 1;
pub const PSCI_1_0_FEATURES_CPU_SUSPEND_PF_MASK: u32 = 0x1 << PSCI_1_0_FEATURES_CPU_SUSPEND_PF_SHIFT;

pub const PSCI_1_0_OS_INITIATED: u32 = 1u32 << 0;
pub const PSCI_1_0_SUSPEND_MODE_PC: u32 = 0;
pub const PSCI_1_0_SUSPEND_MODE_OSI: u32 = 1;

/* PSCI return values (inclusive of all PSCI versions) */
pub const PSCI_RET_SUCCESS: i32 = 0;
pub const PSCI_RET_NOT_SUPPORTED: i32 = -1;
pub const PSCI_RET_INVALID_PARAMS: i32 = -2;
pub const PSCI_RET_DENIED: i32 = -3;
pub const PSCI_RET_ALREADY_ON: i32 = -4;
pub const PSCI_RET_ON_PENDING: i32 = -5;
pub const PSCI_RET_INTERNAL_FAILURE: i32 = -6;
pub const PSCI_RET_NOT_PRESENT: i32 = -7;
pub const PSCI_RET_DISABLED: i32 = -8;
pub const PSCI_RET_INVALID_ADDRESS: i32 = -9;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
