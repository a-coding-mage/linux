/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (C) 2001 Mike Corrigan & Dave Engebretsen, IBM Corporation
 * Rewrite, cleanup:
 * Copyright (C) 2004 Olof Johansson <olof@lixom.net>, IBM Corporation
 */

/* C header guard: _ASM_POWERPC_TCE_H */
/* These definitions were enclosed by the C __KERNEL__ build condition. */

/*
 * Tces come in two formats, one for the virtual bus and a different
 * format for PCI.  PCI TCEs can have hardware or software maintianed
 * coherency.
 */
pub const TCE_VB: u64 = 0;
pub const TCE_PCI: u64 = 1;

pub const TCE_ENTRY_SIZE: u64 = 8; /* each TCE is 64 bits */
pub const TCE_VALID: u64 = 0x800; /* TCE valid */
pub const TCE_ALLIO: u64 = 0x400; /* TCE valid for all lpars */
pub const TCE_PCI_WRITE: u64 = 0x2; /* write from PCI allowed */
pub const TCE_PCI_READ: u64 = 0x1; /* read from PCI allowed */
pub const TCE_VB_WRITE: u64 = 0x1; /* write from VB allowed */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
