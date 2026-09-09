/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2008-2009 Michal Simek <monstr@monstr.eu>
 * Copyright (C) 2008-2009 PetaLogix
 * Copyright (C) 2006 Atmark Techno, Inc.
 */

pub const MSR_BE: u32 = 1 << 0; /* 0x001 */
pub const MSR_IE: u32 = 1 << 1; /* 0x002 */
pub const MSR_C: u32 = 1 << 2; /* 0x004 */
pub const MSR_BIP: u32 = 1 << 3; /* 0x008 */
pub const MSR_FSL: u32 = 1 << 4; /* 0x010 */
pub const MSR_ICE: u32 = 1 << 5; /* 0x020 */
pub const MSR_DZ: u32 = 1 << 6; /* 0x040 */
pub const MSR_DCE: u32 = 1 << 7; /* 0x080 */
pub const MSR_EE: u32 = 1 << 8; /* 0x100 */
pub const MSR_EIP: u32 = 1 << 9; /* 0x200 */
pub const MSR_CC: u32 = 1 << 31;

/* Floating Point Status Register (FSR) Bits */
pub const FSR_IO: u32 = 1 << 4; /* Invalid operation */
pub const FSR_DZ: u32 = 1 << 3; /* Divide-by-zero */
pub const FSR_OF: u32 = 1 << 2; /* Overflow */
pub const FSR_UF: u32 = 1 << 1; /* Underflow */
pub const FSR_DO: u32 = 1 << 0; /* Denormalized operand error */

/* Machine State Register (MSR) Fields */
pub const MSR_UM: u32 = 1 << 11; /* User Mode */
pub const MSR_UMS: u32 = 1 << 12; /* User Mode Save */
pub const MSR_VM: u32 = 1 << 13; /* Virtual Mode */
pub const MSR_VMS: u32 = 1 << 14; /* Virtual Mode Save */

pub const MSR_KERNEL: u32 = MSR_EE | MSR_VM;
/* pub const MSR_USER: u32 = MSR_KERNEL | MSR_UM | MSR_IE; */
pub const MSR_KERNEL_VMS: u32 = MSR_EE | MSR_VMS;
/* pub const MSR_USER_VMS: u32 = MSR_KERNEL_VMS | MSR_UMS | MSR_IE; */

/* Exception State Register (ESR) Fields */
pub const ESR_DIZ: u32 = 1 << 11; /* Zone Protection */
pub const ESR_S: u32 = 1 << 10; /* Store instruction */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
