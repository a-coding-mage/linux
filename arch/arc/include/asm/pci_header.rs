/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2015-2016 Synopsys, Inc. (www.synopsys.com)
 */

/*
 * C header guard: _ASM_ARC_PCI_H
 *
 * The following declarations are present only when __KERNEL__ is defined in
 * the original header. The kernel I/O-port dependency is supplied externally.
 */

pub const PCIBIOS_MIN_IO: u32 = 0x100;
pub const PCIBIOS_MIN_MEM: u32 = 0x100000;

#[inline]
pub const fn pcibios_assign_all_busses() -> i32 {
    1
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
