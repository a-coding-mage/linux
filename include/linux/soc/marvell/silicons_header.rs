/* SPDX-License-Identifier: GPL-2.0-only
 * Copyright (C) 2024 Marvell.
 */

// External dependencies supplied by the surrounding kernel translation:
// `pci_dev` and its `subsystem_device` field are declared in linux/pci.h.

#[cfg(target_arch = "aarch64")]
pub const CN20K_CHIPID: u8 = 0x20;

#[cfg(target_arch = "aarch64")]
/*
 * Silicon check for CN20K family
 */
#[inline]
pub unsafe fn is_cn20k(pdev: *const pci_dev) -> bool {
	((*pdev).subsystem_device & 0xFF) == CN20K_CHIPID
}

#[cfg(not(target_arch = "aarch64"))]
#[inline]
pub unsafe fn is_cn20k(pdev: *const pci_dev) -> i32 {
	let _ = pdev;
	0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
