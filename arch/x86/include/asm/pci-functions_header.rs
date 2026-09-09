/* SPDX-License-Identifier: GPL-2.0 */
/*
 *	PCI BIOS function numbering for conventional PCI BIOS
 *	systems
 */

// The C macro uses `XX` as a notation for the variable low byte.
pub const PCIBIOS_PCI_FUNCTION_ID: u16 = 0xb100;
pub const PCIBIOS_PCI_BIOS_PRESENT: u16 = 0xb101;
pub const PCIBIOS_FIND_PCI_DEVICE: u16 = 0xb102;
pub const PCIBIOS_FIND_PCI_CLASS_CODE: u16 = 0xb103;
pub const PCIBIOS_GENERATE_SPECIAL_CYCLE: u16 = 0xb106;
pub const PCIBIOS_READ_CONFIG_BYTE: u16 = 0xb108;
pub const PCIBIOS_READ_CONFIG_WORD: u16 = 0xb109;
pub const PCIBIOS_READ_CONFIG_DWORD: u16 = 0xb10a;
pub const PCIBIOS_WRITE_CONFIG_BYTE: u16 = 0xb10b;
pub const PCIBIOS_WRITE_CONFIG_WORD: u16 = 0xb10c;
pub const PCIBIOS_WRITE_CONFIG_DWORD: u16 = 0xb10d;
pub const PCIBIOS_GET_ROUTING_OPTIONS: u16 = 0xb10e;
pub const PCIBIOS_SET_PCI_HW_INT: u16 = 0xb10f;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
