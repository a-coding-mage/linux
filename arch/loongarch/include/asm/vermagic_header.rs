/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2020-2022 Loongson Technology Corporation Limited
 */

pub const MODULE_PROC_FAMILY: &str = "LOONGARCH ";

// Build-time configuration equivalent to CONFIG_32BIT.
#[cfg(CONFIG_32BIT)]
pub const MODULE_KERNEL_TYPE: &str = "32BIT ";

// Build-time configuration equivalent to CONFIG_64BIT.
#[cfg(all(not(CONFIG_32BIT), CONFIG_64BIT))]
pub const MODULE_KERNEL_TYPE: &str = "64BIT ";

// MODULE_ARCH_VERMAGIC is the concatenation of MODULE_PROC_FAMILY and
// MODULE_KERNEL_TYPE, matching the C macro expansion.
#[cfg(CONFIG_32BIT)]
pub const MODULE_ARCH_VERMAGIC: &str = concat!(MODULE_PROC_FAMILY, MODULE_KERNEL_TYPE);

#[cfg(all(not(CONFIG_32BIT), CONFIG_64BIT))]
pub const MODULE_ARCH_VERMAGIC: &str = concat!(MODULE_PROC_FAMILY, MODULE_KERNEL_TYPE);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
