/* SPDX-License-Identifier: GPL-2.0
 *
 * Low-Level PCI Support for SH7751 targets
 *
 * C header translation. Build-time include and header-guard semantics are not
 * applicable to Rust.
 */

/* Platform Specific Values */
pub const SH7751_VENDOR_ID: u32 = 0x1054;
pub const SH7751_DEVICE_ID: u32 = 0x3505;
pub const SH7751R_DEVICE_ID: u32 = 0x350e;

/* SH7751 Specific Values */
pub const SH7751_PCI_CONFIG_BASE: u32 = 0xFD000000; // Config space base addr
pub const SH7751_PCI_CONFIG_SIZE: u32 = 0x1000000; // Config space size
pub const SH7751_PCI_MEMORY_BASE: u32 = 0xFD000000; // Memory space base addr
pub const SH7751_PCI_MEM_SIZE: u32 = 0x01000000; // Size of Memory window
pub const SH7751_PCI_IO_BASE: u32 = 0xFE240000; // IO space base address
pub const SH7751_PCI_IO_SIZE: u32 = 0x40000; // Size of IO window

pub const SH7751_PCIREG_BASE: u32 = 0xFE200000; // PCI regs base address

pub const SH7751_PCICONF0: u32 = 0x0; // PCI Config Reg 0
pub const SH7751_PCICONF0_DEVID: u32 = 0xFFFF0000; // Device ID
pub const SH7751_PCICONF0_VNDID: u32 = 0x0000FFFF; // Vendor ID
pub const SH7751_PCICONF1: u32 = 0x4; // PCI Config Reg 1
pub const SH7751_PCICONF1_DPE: u32 = 0x80000000; // Data Parity Error
pub const SH7751_PCICONF1_SSE: u32 = 0x40000000; // System Error Status
pub const SH7751_PCICONF1_RMA: u32 = 0x20000000; // Master Abort
pub const SH7751_PCICONF1_RTA: u32 = 0x10000000; // Target Abort Rx Status
pub const SH7751_PCICONF1_STA: u32 = 0x08000000; // Target Abort Exec Status
pub const SH7751_PCICONF1_DEV: u32 = 0x06000000; // Timing Status
pub const SH7751_PCICONF1_DPD: u32 = 0x01000000; // Data Parity Status
pub const SH7751_PCICONF1_FBBC: u32 = 0x00800000; // Back 2 Back Status
pub const SH7751_PCICONF1_UDF: u32 = 0x00400000; // User Defined Status
pub const SH7751_PCICONF1_66M: u32 = 0x00200000; // 66Mhz Operation Status
pub const SH7751_PCICONF1_PM: u32 = 0x00100000; // Power Management Status
pub const SH7751_PCICONF1_PBBE: u32 = 0x00000200; // Back 2 Back Control
pub const SH7751_PCICONF1_SER: u32 = 0x00000100; // SERR Output Control
pub const SH7751_PCICONF1_WCC: u32 = 0x00000080; // Wait Cycle Control
pub const SH7751_PCICONF1_PER: u32 = 0x00000040; // Parity Error Response
pub const SH7751_PCICONF1_VPS: u32 = 0x00000020; // VGA Pallet Snoop
pub const SH7751_PCICONF1_MWIE: u32 = 0x00000010; // Memory Write+Invalidate
pub const SH7751_PCICONF1_SPC: u32 = 0x00000008; // Special Cycle Control
pub const SH7751_PCICONF1_BUM: u32 = 0x00000004; // Bus Master Control
pub const SH7751_PCICONF1_MES: u32 = 0x00000002; // Memory Space Control
pub const SH7751_PCICONF1_IOS: u32 = 0x00000001; // I/O Space Control
pub const SH7751_PCICONF2: u32 = 0x8; // PCI Config Reg 2
pub const SH7751_PCICONF2_BCC: u32 = 0xFF000000; // Base Class Code
pub const SH7751_PCICONF2_SCC: u32 = 0x00FF0000; // Sub-Class Code
pub const SH7751_PCICONF2_RLPI: u32 = 0x0000FF00; // Programming Interface
pub const SH7751_PCICONF2_REV: u32 = 0x000000FF; // Revision ID
pub const SH7751_PCICONF3: u32 = 0xC; // PCI Config Reg 3
pub const SH7751_PCICONF3_BIST7: u32 = 0x80000000; // Bist Supported
pub const SH7751_PCICONF3_BIST6: u32 = 0x40000000; // Bist Executing
pub const SH7751_PCICONF3_BIST3_0: u32 = 0x0F000000; // Bist Passed
pub const SH7751_PCICONF3_HD7: u32 = 0x00800000; // Single Function device
pub const SH7751_PCICONF3_HD6_0: u32 = 0x007F0000; // Configuration Layout
pub const SH7751_PCICONF3_LAT: u32 = 0x0000FF00; // Latency Timer
pub const SH7751_PCICONF3_CLS: u32 = 0x000000FF; // Cache Line Size
pub const SH7751_PCICONF4: u32 = 0x10; // PCI Config Reg 4
pub const SH7751_PCICONF4_BASE: u32 = 0xFFFFFFFC; // I/O Space Base Addr
pub const SH7751_PCICONF4_ASI: u32 = 0x00000001; // Address Space Type
pub const SH7751_PCICONF5: u32 = 0x14; // PCI Config Reg 5
pub const SH7751_PCICONF5_BASE: u32 = 0xFFFFFFF0; // Mem Space Base Addr
pub const SH7751_PCICONF5_LAP: u32 = 0x00000008; // Prefetch Enabled
pub const SH7751_PCICONF5_LAT: u32 = 0x00000006; // Local Memory type
pub const SH7751_PCICONF5_ASI: u32 = 0x00000001; // Address Space Type
pub const SH7751_PCICONF6: u32 = 0x18; // PCI Config Reg 6
pub const SH7751_PCICONF6_BASE: u32 = 0xFFFFFFF0; // Mem Space Base Addr
pub const SH7751_PCICONF6_LAP: u32 = 0x00000008; // Prefetch Enabled
pub const SH7751_PCICONF6_LAT: u32 = 0x00000006; // Local Memory type
pub const SH7751_PCICONF6_ASI: u32 = 0x00000001; // Address Space Type
// PCICONF7 - PCICONF10 are undefined
pub const SH7751_PCICONF11: u32 = 0x2C; // PCI Config Reg 11
pub const SH7751_PCICONF11_SSID: u32 = 0xFFFF0000; // Subsystem ID
pub const SH7751_PCICONF11_SVID: u32 = 0x0000FFFF; // Subsystem Vendor ID
// PCICONF12 is undefined
pub const SH7751_PCICONF13: u32 = 0x34; // PCI Config Reg 13
pub const SH7751_PCICONF13_CPTR: u32 = 0x000000FF; // PM function pointer
// PCICONF14 is undefined
pub const SH7751_PCICONF15: u32 = 0x3C; // PCI Config Reg 15
pub const SH7751_PCICONF15_IPIN: u32 = 0x000000FF; // Interrupt Pin
pub const SH7751_PCICONF16: u32 = 0x40; // PCI Config Reg 16
pub const SH7751_PCICONF16_PMES: u32 = 0xF8000000; // PME Support
pub const SH7751_PCICONF16_D2S: u32 = 0x04000000; // D2 Support
pub const SH7751_PCICONF16_D1S: u32 = 0x02000000; // D1 Support
pub const SH7751_PCICONF16_DSI: u32 = 0x00200000; // Bit Device Init.
pub const SH7751_PCICONF16_PMCK: u32 = 0x00080000; // Clock for PME req.
pub const SH7751_PCICONF16_VER: u32 = 0x00070000; // PM Version
pub const SH7751_PCICONF16_NIP: u32 = 0x0000FF00; // Next Item Pointer
pub const SH7751_PCICONF16_CID: u32 = 0x000000FF; // Capability Identifier
pub const SH7751_PCICONF17: u32 = 0x44; // PCI Config Reg 17
pub const SH7751_PCICONF17_DATA: u32 = 0xFF000000; // Data field for PM
pub const SH7751_PCICONF17_PMES: u32 = 0x00800000; // PME Status
pub const SH7751_PCICONF17_DSCL: u32 = 0x00600000; // Data Scaling Value
pub const SH7751_PCICONF17_DSEL: u32 = 0x001E0000; // Data Select
pub const SH7751_PCICONF17_PMEN: u32 = 0x00010000; // PME Enable
pub const SH7751_PCICONF17_PWST: u32 = 0x00000003; // Power State

/* SH7715 Internal PCI Registers */

/* Memory Control Registers */
pub const SH7751_BCR1: u32 = 0xFF800000; // Memory BCR1 Register
pub const SH7751_BCR2: u32 = 0xFF800004; // Memory BCR2 Register
pub const SH7751_BCR3: u32 = 0xFF800050; // Memory BCR3 Register
pub const SH7751_BCR4: u32 = 0xFE0A00F0; // Memory BCR4 Register
pub const SH7751_WCR1: u32 = 0xFF800008; // Wait Control 1 Register
pub const SH7751_WCR2: u32 = 0xFF80000C; // Wait Control 2 Register
pub const SH7751_WCR3: u32 = 0xFF800010; // Wait Control 3 Register
pub const SH7751_MCR: u32 = 0xFF800014; // Memory Control Register

/* General Memory Config Addresses */
pub const SH7751_CS0_BASE_ADDR: u32 = 0x0;
pub const SH7751_MEM_REGION_SIZE: u32 = 0x04000000;
pub const SH7751_CS1_BASE_ADDR: u32 = SH7751_CS0_BASE_ADDR + SH7751_MEM_REGION_SIZE;
pub const SH7751_CS2_BASE_ADDR: u32 = SH7751_CS1_BASE_ADDR + SH7751_MEM_REGION_SIZE;
pub const SH7751_CS3_BASE_ADDR: u32 = SH7751_CS2_BASE_ADDR + SH7751_MEM_REGION_SIZE;
pub const SH7751_CS4_BASE_ADDR: u32 = SH7751_CS3_BASE_ADDR + SH7751_MEM_REGION_SIZE;
pub const SH7751_CS5_BASE_ADDR: u32 = SH7751_CS4_BASE_ADDR + SH7751_MEM_REGION_SIZE;
pub const SH7751_CS6_BASE_ADDR: u32 = SH7751_CS5_BASE_ADDR + SH7751_MEM_REGION_SIZE;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
