/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/* Direct Rust translation of the Linux PCI register definitions. */
#![allow(non_upper_case_globals, unused_macros)]

/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 *	PCI standard defines
 *	Copyright 1994, Drew Eckhardt
 *	Copyright 1997--1999 Martin Mares <mj@ucw.cz>
 *
 *	For more information, please consult the following manuals (look at
 *	http://www.pcisig.com/ for how to get them):
 *
 *	PCI BIOS Specification
 *	PCI Local Bus Specification
 *	PCI to PCI Bridge Specification
 *	PCI System Design Guide
 *
 *	For HyperTransport information, please consult the following manuals
 *	from http://www.hypertransport.org :
 *
 *	The HyperTransport I/O Link Specification
 */


/*
 * Conventional PCI and PCI-X Mode 1 devices have 256 bytes of
 * configuration space.  PCI-X Mode 2 and PCIe devices have 4096 bytes of
 * configuration space.
 */
pub const PCI_CFG_SPACE_SIZE: u64 = 256;
pub const PCI_CFG_SPACE_EXP_SIZE: u64 = 4096;

/*
 * Under PCI, each device has 256 bytes of configuration address space,
 * of which the first 64 bytes are standardized as follows:
 */
pub const PCI_STD_HEADER_SIZEOF: u64 = 64;
pub const PCI_STD_NUM_BARS: u64 = 6	/* Number of standard BARs */;
pub const PCI_VENDOR_ID: u64 = 0x00	/* 16 bits */;
pub const PCI_DEVICE_ID: u64 = 0x02	/* 16 bits */;
pub const PCI_COMMAND: u64 = 0x04	/* 16 bits */;
pub const PCI_COMMAND_IO: u64 = 0x1	/* Enable response in I/O space */;
pub const PCI_COMMAND_MEMORY: u64 = 0x2	/* Enable response in Memory space */;
pub const PCI_COMMAND_MASTER: u64 = 0x4	/* Enable bus mastering */;
pub const PCI_COMMAND_SPECIAL: u64 = 0x8	/* Enable response to special cycles */;
pub const PCI_COMMAND_INVALIDATE: u64 = 0x10	/* Use memory write and invalidate */;
pub const PCI_COMMAND_VGA_PALETTE: u64 = 0x20	/* Enable palette snooping */;
pub const PCI_COMMAND_PARITY: u64 = 0x40	/* Enable parity checking */;
pub const PCI_COMMAND_WAIT: u64 = 0x80	/* Enable address/data stepping */;
pub const PCI_COMMAND_SERR: u64 = 0x100	/* Enable SERR */;
pub const PCI_COMMAND_FAST_BACK: u64 = 0x200	/* Enable back-to-back writes */;
pub const PCI_COMMAND_INTX_DISABLE: u64 = 0x400 /* INTx Emulation Disable */;

pub const PCI_STATUS: u64 = 0x06	/* 16 bits */;
pub const PCI_STATUS_IMM_READY: u64 = 0x01	/* Immediate Readiness */;
pub const PCI_STATUS_INTERRUPT: u64 = 0x08	/* Interrupt status */;
pub const PCI_STATUS_CAP_LIST: u64 = 0x10	/* Support Capability List */;
pub const PCI_STATUS_66MHZ: u64 = 0x20	/* Support 66 MHz PCI 2.1 bus */;
pub const PCI_STATUS_UDF: u64 = 0x40	/* Support User Definable Features [obsolete] */;
pub const PCI_STATUS_FAST_BACK: u64 = 0x80	/* Accept fast-back to back */;
pub const PCI_STATUS_PARITY: u64 = 0x100	/* Detected parity error */;
pub const PCI_STATUS_DEVSEL_MASK: u64 = 0x600	/* DEVSEL timing */;
pub const PCI_STATUS_DEVSEL_FAST: u64 = 0x000;
pub const PCI_STATUS_DEVSEL_MEDIUM: u64 = 0x200;
pub const PCI_STATUS_DEVSEL_SLOW: u64 = 0x400;
pub const PCI_STATUS_SIG_TARGET_ABORT: u64 = 0x800 /* Set on target abort */;
pub const PCI_STATUS_REC_TARGET_ABORT: u64 = 0x1000 /* Master ack of " */;
pub const PCI_STATUS_REC_MASTER_ABORT: u64 = 0x2000 /* Set on master abort */;
pub const PCI_STATUS_SIG_SYSTEM_ERROR: u64 = 0x4000 /* Set when we drive SERR */;
pub const PCI_STATUS_DETECTED_PARITY: u64 = 0x8000 /* Set on parity error */;

pub const PCI_CLASS_REVISION: u64 = 0x08	/* High 24 bits are class, low 8 revision */;
pub const PCI_REVISION_ID: u64 = 0x08	/* Revision ID */;
pub const PCI_CLASS_PROG: u64 = 0x09	/* Reg. Level Programming Interface */;
pub const PCI_CLASS_DEVICE: u64 = 0x0a	/* Device class */;

pub const PCI_CACHE_LINE_SIZE: u64 = 0x0c	/* 8 bits */;
pub const PCI_LATENCY_TIMER: u64 = 0x0d	/* 8 bits */;
pub const PCI_HEADER_TYPE: u64 = 0x0e	/* 8 bits */;
pub const PCI_HEADER_TYPE_MASK: u64 = 0x7f;
pub const PCI_HEADER_TYPE_NORMAL: u64 = 0;
pub const PCI_HEADER_TYPE_BRIDGE: u64 = 1;
pub const PCI_HEADER_TYPE_CARDBUS: u64 = 2;
pub const PCI_HEADER_TYPE_MFD: u64 = 0x80	/* Multi-Function Device (possible) */;

pub const PCI_BIST: u64 = 0x0f	/* 8 bits */;
pub const PCI_BIST_CODE_MASK: u64 = 0x0f	/* Return result */;
pub const PCI_BIST_START: u64 = 0x40	/* 1 to start BIST, 2 secs or less */;
pub const PCI_BIST_CAPABLE: u64 = 0x80	/* 1 if BIST capable */;

/*
 * Base addresses specify locations in memory or I/O space.
 * Decoded size can be determined by writing a value of
 * 0xffffffff to the register, and reading it back.  Only
 * 1 bits are decoded.
 */
pub const PCI_BASE_ADDRESS_0: u64 = 0x10	/* 32 bits */;
pub const PCI_BASE_ADDRESS_1: u64 = 0x14	/* 32 bits [htype 0,1 only] */;
pub const PCI_BASE_ADDRESS_2: u64 = 0x18	/* 32 bits [htype 0 only] */;
pub const PCI_BASE_ADDRESS_3: u64 = 0x1c	/* 32 bits */;
pub const PCI_BASE_ADDRESS_4: u64 = 0x20	/* 32 bits */;
pub const PCI_BASE_ADDRESS_5: u64 = 0x24	/* 32 bits */;
pub const PCI_BASE_ADDRESS_SPACE: u64 = 0x01	/* 0 = memory, 1 = I/O */;
pub const PCI_BASE_ADDRESS_SPACE_IO: u64 = 0x01;
pub const PCI_BASE_ADDRESS_SPACE_MEMORY: u64 = 0x00;
pub const PCI_BASE_ADDRESS_MEM_TYPE_MASK: u64 = 0x06;
pub const PCI_BASE_ADDRESS_MEM_TYPE_32: u64 = 0x00	/* 32 bit address */;
pub const PCI_BASE_ADDRESS_MEM_TYPE_1M: u64 = 0x02	/* Below 1M [obsolete] */;
pub const PCI_BASE_ADDRESS_MEM_TYPE_64: u64 = 0x04	/* 64 bit address */;
pub const PCI_BASE_ADDRESS_MEM_PREFETCH: u64 = 0x08	/* prefetchable? */;
pub const PCI_BASE_ADDRESS_MEM_MASK: u64 = (~0x0f);
pub const PCI_BASE_ADDRESS_IO_MASK: u64 = (~0x03);
/* bit 1 is reserved if address_space = 1 */

/* Header type 0 (normal devices) */
pub const PCI_CARDBUS_CIS: u64 = 0x28;
pub const PCI_SUBSYSTEM_VENDOR_ID: u64 = 0x2c;
pub const PCI_SUBSYSTEM_ID: u64 = 0x2e;
pub const PCI_ROM_ADDRESS: u64 = 0x30	/* Bits 31..11 are address, 10..1 reserved */;
pub const PCI_ROM_ADDRESS_ENABLE: u64 = 0x01;
pub const PCI_ROM_ADDRESS_MASK: u64 = (~0x7ff);

pub const PCI_CAPABILITY_LIST: u64 = 0x34	/* Offset of first capability list entry */;

/* 0x35-0x3b are reserved */
pub const PCI_INTERRUPT_LINE: u64 = 0x3c	/* 8 bits */;
pub const PCI_INTERRUPT_PIN: u64 = 0x3d	/* 8 bits */;
pub const PCI_MIN_GNT: u64 = 0x3e	/* 8 bits */;
pub const PCI_MAX_LAT: u64 = 0x3f	/* 8 bits */;

/* Header type 1 (PCI-to-PCI bridges) */
pub const PCI_PRIMARY_BUS: u64 = 0x18	/* Primary bus number */;
pub const PCI_SECONDARY_BUS: u64 = 0x19	/* Secondary bus number */;
pub const PCI_SUBORDINATE_BUS: u64 = 0x1a	/* Highest bus number behind the bridge */;
pub const PCI_SEC_LATENCY_TIMER: u64 = 0x1b	/* Latency timer for secondary interface */;
/* Masks for dword-sized processing of Bus Number and Sec Latency Timer fields */
pub const PCI_PRIMARY_BUS_MASK: u64 = 0x000000ff;
pub const PCI_SECONDARY_BUS_MASK: u64 = 0x0000ff00;
pub const PCI_SUBORDINATE_BUS_MASK: u64 = 0x00ff0000;
pub const PCI_SEC_LATENCY_TIMER_MASK: u64 = 0xff000000;
pub const PCI_IO_BASE: u64 = 0x1c	/* I/O range behind the bridge */;
pub const PCI_IO_LIMIT: u64 = 0x1d;
pub const PCI_IO_RANGE_TYPE_MASK: u64 = 0x0f	/* I/O bridging type */;
pub const PCI_IO_RANGE_TYPE_16: u64 = 0x00;
pub const PCI_IO_RANGE_TYPE_32: u64 = 0x01;
pub const PCI_IO_RANGE_MASK: u64 = (~0x0f) /* Standard 4K I/O windows */;
pub const PCI_IO_1K_RANGE_MASK: u64 = (~0x03) /* Intel 1K I/O windows */;
pub const PCI_SEC_STATUS: u64 = 0x1e	/* Secondary status register, only bit 14 used */;
pub const PCI_MEMORY_BASE: u64 = 0x20	/* Memory range behind */;
pub const PCI_MEMORY_LIMIT: u64 = 0x22;
pub const PCI_MEMORY_RANGE_TYPE_MASK: u64 = 0x0f;
pub const PCI_MEMORY_RANGE_MASK: u64 = (~0x0f);
pub const PCI_PREF_MEMORY_BASE: u64 = 0x24	/* Prefetchable memory range behind */;
pub const PCI_PREF_MEMORY_LIMIT: u64 = 0x26;
pub const PCI_PREF_RANGE_TYPE_MASK: u64 = 0x0f;
pub const PCI_PREF_RANGE_TYPE_32: u64 = 0x00;
pub const PCI_PREF_RANGE_TYPE_64: u64 = 0x01;
pub const PCI_PREF_RANGE_MASK: u64 = (~0x0f);
pub const PCI_PREF_BASE_UPPER32: u64 = 0x28	/* Upper half of prefetchable memory range */;
pub const PCI_PREF_LIMIT_UPPER32: u64 = 0x2c;
pub const PCI_IO_BASE_UPPER16: u64 = 0x30	/* Upper half of I/O addresses */;
pub const PCI_IO_LIMIT_UPPER16: u64 = 0x32;
/* 0x34 same as for htype 0 */
/* 0x35-0x3b is reserved */
pub const PCI_ROM_ADDRESS1: u64 = 0x38	/* Same as PCI_ROM_ADDRESS, but for htype 1 */;
/* 0x3c-0x3d are same as for htype 0 */
pub const PCI_BRIDGE_CONTROL: u64 = 0x3e;
pub const PCI_BRIDGE_CTL_PARITY: u64 = 0x01	/* Enable parity detection on secondary interface */;
pub const PCI_BRIDGE_CTL_SERR: u64 = 0x02	/* The same for SERR forwarding */;
pub const PCI_BRIDGE_CTL_ISA: u64 = 0x04	/* Enable ISA mode */;
pub const PCI_BRIDGE_CTL_VGA: u64 = 0x08	/* Forward VGA addresses */;
pub const PCI_BRIDGE_CTL_MASTER_ABORT: u64 = 0x20  /* Report master aborts */;
pub const PCI_BRIDGE_CTL_BUS_RESET: u64 = 0x40	/* Secondary bus reset */;
pub const PCI_BRIDGE_CTL_FAST_BACK: u64 = 0x80	/* Fast Back2Back enabled on secondary interface */;

/* Header type 2 (CardBus bridges) */
pub const PCI_CB_CAPABILITY_LIST: u64 = 0x14;
/* 0x15 reserved */
pub const PCI_CB_SEC_STATUS: u64 = 0x16	/* Secondary status */;
pub const PCI_CB_PRIMARY_BUS: u64 = 0x18	/* PCI bus number */;
pub const PCI_CB_CARD_BUS: u64 = 0x19	/* CardBus bus number */;
pub const PCI_CB_SUBORDINATE_BUS: u64 = 0x1a	/* Subordinate bus number */;
pub const PCI_CB_LATENCY_TIMER: u64 = 0x1b	/* CardBus latency timer */;
pub const PCI_CB_MEMORY_BASE_0: u64 = 0x1c;
pub const PCI_CB_MEMORY_LIMIT_0: u64 = 0x20;
pub const PCI_CB_MEMORY_BASE_1: u64 = 0x24;
pub const PCI_CB_MEMORY_LIMIT_1: u64 = 0x28;
pub const PCI_CB_IO_BASE_0: u64 = 0x2c;
pub const PCI_CB_IO_BASE_0_HI: u64 = 0x2e;
pub const PCI_CB_IO_LIMIT_0: u64 = 0x30;
pub const PCI_CB_IO_LIMIT_0_HI: u64 = 0x32;
pub const PCI_CB_IO_BASE_1: u64 = 0x34;
pub const PCI_CB_IO_BASE_1_HI: u64 = 0x36;
pub const PCI_CB_IO_LIMIT_1: u64 = 0x38;
pub const PCI_CB_IO_LIMIT_1_HI: u64 = 0x3a;
pub const PCI_CB_IO_RANGE_MASK: u64 = (~0x03);
/* 0x3c-0x3d are same as for htype 0 */
pub const PCI_CB_BRIDGE_CONTROL: u64 = 0x3e;
pub const PCI_CB_BRIDGE_CTL_PARITY: u64 = 0x01	/* Similar to standard bridge control register */;
pub const PCI_CB_BRIDGE_CTL_SERR: u64 = 0x02;
pub const PCI_CB_BRIDGE_CTL_ISA: u64 = 0x04;
pub const PCI_CB_BRIDGE_CTL_VGA: u64 = 0x08;
pub const PCI_CB_BRIDGE_CTL_MASTER_ABORT: u64 = 0x20;
pub const PCI_CB_BRIDGE_CTL_CB_RESET: u64 = 0x40	/* CardBus reset */;
pub const PCI_CB_BRIDGE_CTL_16BIT_INT: u64 = 0x80	/* Enable interrupt for 16-bit cards */;
pub const PCI_CB_BRIDGE_CTL_PREFETCH_MEM0: u64 = 0x100	/* Prefetch enable for both memory regions */;
pub const PCI_CB_BRIDGE_CTL_PREFETCH_MEM1: u64 = 0x200;
pub const PCI_CB_BRIDGE_CTL_POST_WRITES: u64 = 0x400;
pub const PCI_CB_SUBSYSTEM_VENDOR_ID: u64 = 0x40;
pub const PCI_CB_SUBSYSTEM_ID: u64 = 0x42;
pub const PCI_CB_LEGACY_MODE_BASE: u64 = 0x44	/* 16-bit PC Card legacy mode base address (ExCa) */;
/* 0x48-0x7f reserved */

/* Capability lists */

pub const PCI_CAP_ID_MASK: u64 = 0x00ff	/* Capability ID mask */;
pub const PCI_CAP_LIST_NEXT_MASK: u64 = 0xff00	/* Next Capability Pointer mask */;

pub const PCI_CAP_LIST_ID: u64 = 0	/* Capability ID */;
pub const PCI_CAP_ID_PM: u64 = 0x01	/* Power Management */;
pub const PCI_CAP_ID_AGP: u64 = 0x02	/* Accelerated Graphics Port */;
pub const PCI_CAP_ID_VPD: u64 = 0x03	/* Vital Product Data */;
pub const PCI_CAP_ID_SLOTID: u64 = 0x04	/* Slot Identification */;
pub const PCI_CAP_ID_MSI: u64 = 0x05	/* Message Signalled Interrupts */;
pub const PCI_CAP_ID_CHSWP: u64 = 0x06	/* CompactPCI HotSwap */;
pub const PCI_CAP_ID_PCIX: u64 = 0x07	/* PCI-X */;
pub const PCI_CAP_ID_HT: u64 = 0x08	/* HyperTransport */;
pub const PCI_CAP_ID_VNDR: u64 = 0x09	/* Vendor-Specific */;
pub const PCI_CAP_ID_DBG: u64 = 0x0A	/* Debug port */;
pub const PCI_CAP_ID_CCRC: u64 = 0x0B	/* CompactPCI Central Resource Control */;
pub const PCI_CAP_ID_SHPC: u64 = 0x0C	/* PCI Standard Hot-Plug Controller */;
pub const PCI_CAP_ID_SSVID: u64 = 0x0D	/* Bridge subsystem vendor/device ID */;
pub const PCI_CAP_ID_AGP3: u64 = 0x0E	/* AGP Target PCI-PCI bridge */;
pub const PCI_CAP_ID_SECDEV: u64 = 0x0F	/* Secure Device */;
pub const PCI_CAP_ID_EXP: u64 = 0x10	/* PCI Express */;
pub const PCI_CAP_ID_MSIX: u64 = 0x11	/* MSI-X */;
pub const PCI_CAP_ID_SATA: u64 = 0x12	/* SATA Data/Index Conf. */;
pub const PCI_CAP_ID_AF: u64 = 0x13	/* PCI Advanced Features */;
pub const PCI_CAP_ID_EA: u64 = 0x14	/* PCI Enhanced Allocation */;
pub const PCI_CAP_ID_MAX: u64 = PCI_CAP_ID_EA;
pub const PCI_CAP_LIST_NEXT: u64 = 1	/* Next capability in the list */;
pub const PCI_CAP_FLAGS: u64 = 2	/* Capability defined flags (16 bits) */;
pub const PCI_CAP_SIZEOF: u64 = 4;

/* Power Management Registers */

pub const PCI_PM_PMC: u64 = 2	/* PM Capabilities Register */;
pub const PCI_PM_CAP_VER_MASK: u64 = 0x0007	/* Version */;
pub const PCI_PM_CAP_PME_CLOCK: u64 = 0x0008	/* PME clock required */;
pub const PCI_PM_CAP_RESERVED: u64 = 0x0010  /* Reserved field */;
pub const PCI_PM_CAP_DSI: u64 = 0x0020	/* Device specific initialization */;
pub const PCI_PM_CAP_AUX_POWER: u64 = 0x01C0	/* Auxiliary power support mask */;
pub const PCI_PM_CAP_D1: u64 = 0x0200	/* D1 power state support */;
pub const PCI_PM_CAP_D2: u64 = 0x0400	/* D2 power state support */;
pub const PCI_PM_CAP_PME: u64 = 0x0800	/* PME pin supported */;
pub const PCI_PM_CAP_PME_MASK: u64 = 0xF800	/* PME Mask of all supported states */;
pub const PCI_PM_CAP_PME_D0: u64 = 0x0800	/* PME# from D0 */;
pub const PCI_PM_CAP_PME_D1: u64 = 0x1000	/* PME# from D1 */;
pub const PCI_PM_CAP_PME_D2: u64 = 0x2000	/* PME# from D2 */;
pub const PCI_PM_CAP_PME_D3hot: u64 = 0x4000	/* PME# from D3 (hot) */;
pub const PCI_PM_CAP_PME_D3cold: u64 = 0x8000	/* PME# from D3 (cold) */;
pub const PCI_PM_CAP_PME_SHIFT: u64 = 11	/* Start of the PME Mask in PMC */;
pub const PCI_PM_CTRL: u64 = 4	/* PM control and status register */;
pub const PCI_PM_CTRL_STATE_MASK: u64 = 0x0003	/* Current power state (D0 to D3) */;
pub const PCI_PM_CTRL_NO_SOFT_RESET: u64 = 0x0008	/* No reset for D3hot->D0 */;
pub const PCI_PM_CTRL_PME_ENABLE: u64 = 0x0100	/* PME pin enable */;
pub const PCI_PM_CTRL_DATA_SEL_MASK: u64 = 0x1e00	/* Data select (??) */;
pub const PCI_PM_CTRL_DATA_SCALE_MASK: u64 = 0x6000	/* Data scale (??) */;
pub const PCI_PM_CTRL_PME_STATUS: u64 = 0x8000	/* PME pin status */;
pub const PCI_PM_PPB_EXTENSIONS: u64 = 6	/* PPB support extensions (??) */;
pub const PCI_PM_PPB_B2_B3: u64 = 0x40	/* Stop clock when in D3hot (??) */;
pub const PCI_PM_BPCC_ENABLE: u64 = 0x80	/* Bus power/clock control enable (??) */;
pub const PCI_PM_DATA_REGISTER: u64 = 7	/* (??) */;
pub const PCI_PM_SIZEOF: u64 = 8;

/* AGP registers */

pub const PCI_AGP_VERSION: u64 = 2	/* BCD version number */;
pub const PCI_AGP_RFU: u64 = 3	/* Rest of capability flags */;
pub const PCI_AGP_STATUS: u64 = 4	/* Status register */;
pub const PCI_AGP_STATUS_RQ_MASK: u64 = 0xff000000	/* Maximum number of requests - 1 */;
pub const PCI_AGP_STATUS_SBA: u64 = 0x0200	/* Sideband addressing supported */;
pub const PCI_AGP_STATUS_64BIT: u64 = 0x0020	/* 64-bit addressing supported */;
pub const PCI_AGP_STATUS_FW: u64 = 0x0010	/* FW transfers supported */;
pub const PCI_AGP_STATUS_RATE4: u64 = 0x0004	/* 4x transfer rate supported */;
pub const PCI_AGP_STATUS_RATE2: u64 = 0x0002	/* 2x transfer rate supported */;
pub const PCI_AGP_STATUS_RATE1: u64 = 0x0001	/* 1x transfer rate supported */;
pub const PCI_AGP_COMMAND: u64 = 8	/* Control register */;
pub const PCI_AGP_COMMAND_RQ_MASK: u64 = 0xff000000  /* Master: Maximum number of requests */;
pub const PCI_AGP_COMMAND_SBA: u64 = 0x0200	/* Sideband addressing enabled */;
pub const PCI_AGP_COMMAND_AGP: u64 = 0x0100	/* Allow processing of AGP transactions */;
pub const PCI_AGP_COMMAND_64BIT: u64 = 0x0020	/* Allow processing of 64-bit addresses */;
pub const PCI_AGP_COMMAND_FW: u64 = 0x0010	/* Force FW transfers */;
pub const PCI_AGP_COMMAND_RATE4: u64 = 0x0004	/* Use 4x rate */;
pub const PCI_AGP_COMMAND_RATE2: u64 = 0x0002	/* Use 2x rate */;
pub const PCI_AGP_COMMAND_RATE1: u64 = 0x0001	/* Use 1x rate */;
pub const PCI_AGP_SIZEOF: u64 = 12;

/* Vital Product Data */

pub const PCI_VPD_ADDR: u64 = 2	/* Address to access (15 bits!) */;
pub const PCI_VPD_ADDR_MASK: u64 = 0x7fff	/* Address mask */;
pub const PCI_VPD_ADDR_F: u64 = 0x8000	/* Write 0, 1 indicates completion */;
pub const PCI_VPD_DATA: u64 = 4	/* 32-bits of data returned here */;
pub const PCI_CAP_VPD_SIZEOF: u64 = 8;

/* Slot Identification */

pub const PCI_SID_ESR: u64 = 2	/* Expansion Slot Register */;
pub const PCI_SID_ESR_NSLOTS: u64 = 0x1f	/* Number of expansion slots available */;
pub const PCI_SID_ESR_FIC: u64 = 0x20	/* First In Chassis Flag */;
pub const PCI_SID_CHASSIS_NR: u64 = 3	/* Chassis Number */;

/* Message Signaled Interrupt registers */

pub const PCI_MSI_FLAGS: u64 = 0x02	/* Message Control */;
pub const PCI_MSI_FLAGS_ENABLE: u64 = 0x0001	/* MSI feature enabled */;
pub const PCI_MSI_FLAGS_QMASK: u64 = 0x000e	/* Maximum queue size available */;
pub const PCI_MSI_FLAGS_QSIZE: u64 = 0x0070	/* Message queue size configured */;
pub const PCI_MSI_FLAGS_64BIT: u64 = 0x0080	/* 64-bit addresses allowed */;
pub const PCI_MSI_FLAGS_MASKBIT: u64 = 0x0100	/* Per-vector masking capable */;
pub const PCI_MSI_RFU: u64 = 3	/* Rest of capability flags */;
pub const PCI_MSI_ADDRESS_LO: u64 = 0x04	/* Lower 32 bits */;
pub const PCI_MSI_ADDRESS_HI: u64 = 0x08	/* Upper 32 bits (if PCI_MSI_FLAGS_64BIT set) */;
pub const PCI_MSI_DATA_32: u64 = 0x08	/* 16 bits of data for 32-bit devices */;
pub const PCI_MSI_MASK_32: u64 = 0x0c	/* Mask bits register for 32-bit devices */;
pub const PCI_MSI_PENDING_32: u64 = 0x10	/* Pending intrs for 32-bit devices */;
pub const PCI_MSI_DATA_64: u64 = 0x0c	/* 16 bits of data for 64-bit devices */;
pub const PCI_MSI_MASK_64: u64 = 0x10	/* Mask bits register for 64-bit devices */;
pub const PCI_MSI_PENDING_64: u64 = 0x14	/* Pending intrs for 64-bit devices */;

/* MSI-X registers (in MSI-X capability) */
pub const PCI_MSIX_FLAGS: u64 = 2	/* Message Control */;
pub const PCI_MSIX_FLAGS_QSIZE: u64 = 0x07FF	/* Table size */;
pub const PCI_MSIX_FLAGS_MASKALL: u64 = 0x4000	/* Mask all vectors for this function */;
pub const PCI_MSIX_FLAGS_ENABLE: u64 = 0x8000	/* MSI-X enable */;
pub const PCI_MSIX_TABLE: u64 = 4	/* Table offset */;
pub const PCI_MSIX_TABLE_BIR: u64 = 0x00000007 /* BAR index */;
pub const PCI_MSIX_TABLE_OFFSET: u64 = 0xfffffff8 /* Offset into specified BAR */;
pub const PCI_MSIX_PBA: u64 = 8	/* Pending Bit Array offset */;
pub const PCI_MSIX_PBA_BIR: u64 = 0x00000007 /* BAR index */;
pub const PCI_MSIX_PBA_OFFSET: u64 = 0xfffffff8 /* Offset into specified BAR */;
pub const PCI_MSIX_FLAGS_BIRMASK: u64 = PCI_MSIX_PBA_BIR /* deprecated */;
pub const PCI_CAP_MSIX_SIZEOF: u64 = 12	/* size of MSIX registers */;

/* MSI-X Table entry format (in memory mapped by a BAR) */
pub const PCI_MSIX_ENTRY_SIZE: u64 = 16;
pub const PCI_MSIX_ENTRY_LOWER_ADDR: u64 = 0x0  /* Message Address */;
pub const PCI_MSIX_ENTRY_UPPER_ADDR: u64 = 0x4  /* Message Upper Address */;
pub const PCI_MSIX_ENTRY_DATA: u64 = 0x8  /* Message Data */;
pub const PCI_MSIX_ENTRY_VECTOR_CTRL: u64 = 0xc  /* Vector Control */;
pub const PCI_MSIX_ENTRY_CTRL_MASKBIT: u64 = 0x00000001  /* Mask Bit */;
pub const PCI_MSIX_ENTRY_CTRL_ST: u64 = 0xffff0000  /* Steering Tag */;

/* CompactPCI Hotswap Register */

pub const PCI_CHSWP_CSR: u64 = 2	/* Control and Status Register */;
pub const PCI_CHSWP_DHA: u64 = 0x01	/* Device Hiding Arm */;
pub const PCI_CHSWP_EIM: u64 = 0x02	/* ENUM# Signal Mask */;
pub const PCI_CHSWP_PIE: u64 = 0x04	/* Pending Insert or Extract */;
pub const PCI_CHSWP_LOO: u64 = 0x08	/* LED On / Off */;
pub const PCI_CHSWP_PI: u64 = 0x30	/* Programming Interface */;
pub const PCI_CHSWP_EXT: u64 = 0x40	/* ENUM# status - extraction */;
pub const PCI_CHSWP_INS: u64 = 0x80	/* ENUM# status - insertion */;

/* PCI Advanced Feature registers */

pub const PCI_AF_LENGTH: u64 = 2;
pub const PCI_AF_CAP: u64 = 3;
pub const PCI_AF_CAP_TP: u64 = 0x01;
pub const PCI_AF_CAP_FLR: u64 = 0x02;
pub const PCI_AF_CTRL: u64 = 4;
pub const PCI_AF_CTRL_FLR: u64 = 0x01;
pub const PCI_AF_STATUS: u64 = 5;
pub const PCI_AF_STATUS_TP: u64 = 0x01;
pub const PCI_CAP_AF_SIZEOF: u64 = 6	/* size of AF registers */;

/* PCI Enhanced Allocation registers */

pub const PCI_EA_NUM_ENT: u64 = 2	/* Number of Capability Entries */;
pub const PCI_EA_NUM_ENT_MASK: u64 = 0x3f	/* Num Entries Mask */;
pub const PCI_EA_FIRST_ENT: u64 = 4	/* First EA Entry in List */;
pub const PCI_EA_FIRST_ENT_BRIDGE: u64 = 8	/* First EA Entry for Bridges */;
pub const PCI_EA_ES: u64 = 0x00000007 /* Entry Size */;
pub const PCI_EA_BEI: u64 = 0x000000f0 /* BAR Equivalent Indicator */;

/* EA fixed Secondary and Subordinate bus numbers for Bridge */
pub const PCI_EA_SEC_BUS_MASK: u64 = 0xff;
pub const PCI_EA_SUB_BUS_MASK: u64 = 0xff00;
pub const PCI_EA_SUB_BUS_SHIFT: u64 = 8;

/* 0-5 map to BARs 0-5 respectively */
pub const PCI_EA_BEI_BAR0: u64 = 0;
pub const PCI_EA_BEI_BAR5: u64 = 5;
pub const PCI_EA_BEI_BRIDGE: u64 = 6	/* Resource behind bridge */;
pub const PCI_EA_BEI_ENI: u64 = 7	/* Equivalent Not Indicated */;
pub const PCI_EA_BEI_ROM: u64 = 8	/* Expansion ROM */;
/* 9-14 map to VF BARs 0-5 respectively */
pub const PCI_EA_BEI_VF_BAR0: u64 = 9;
pub const PCI_EA_BEI_VF_BAR5: u64 = 14;
pub const PCI_EA_BEI_RESERVED: u64 = 15	/* Reserved - Treat like ENI */;
pub const PCI_EA_PP: u64 = 0x0000ff00	/* Primary Properties */;
pub const PCI_EA_SP: u64 = 0x00ff0000	/* Secondary Properties */;
pub const PCI_EA_P_MEM: u64 = 0x00	/* Non-Prefetch Memory */;
pub const PCI_EA_P_MEM_PREFETCH: u64 = 0x01	/* Prefetchable Memory */;
pub const PCI_EA_P_IO: u64 = 0x02	/* I/O Space */;
pub const PCI_EA_P_VF_MEM_PREFETCH: u64 = 0x03	/* VF Prefetchable Memory */;
pub const PCI_EA_P_VF_MEM: u64 = 0x04	/* VF Non-Prefetch Memory */;
pub const PCI_EA_P_BRIDGE_MEM: u64 = 0x05	/* Bridge Non-Prefetch Memory */;
pub const PCI_EA_P_BRIDGE_MEM_PREFETCH: u64 = 0x06	/* Bridge Prefetchable Memory */;
pub const PCI_EA_P_BRIDGE_IO: u64 = 0x07	/* Bridge I/O Space */;
/* 0x08-0xfc reserved */
pub const PCI_EA_P_MEM_RESERVED: u64 = 0xfd	/* Reserved Memory */;
pub const PCI_EA_P_IO_RESERVED: u64 = 0xfe	/* Reserved I/O Space */;
pub const PCI_EA_P_UNAVAILABLE: u64 = 0xff	/* Entry Unavailable */;
pub const PCI_EA_WRITABLE: u64 = 0x40000000	/* Writable: 1 = RW, 0 = HwInit */;
pub const PCI_EA_ENABLE: u64 = 0x80000000	/* Enable for this entry */;
pub const PCI_EA_BASE: u64 = 4		/* Base Address Offset */;
pub const PCI_EA_MAX_OFFSET: u64 = 8		/* MaxOffset (resource length) */;
/* bit 0 is reserved */
pub const PCI_EA_IS_64: u64 = 0x00000002	/* 64-bit field flag */;
pub const PCI_EA_FIELD_MASK: u64 = 0xfffffffc	/* For Base & Max Offset */;

/* PCI-X registers (Type 0 (non-bridge) devices) */

pub const PCI_X_CMD: u64 = 2	/* Modes & Features */;
pub const PCI_X_CMD_DPERR_E: u64 = 0x0001	/* Data Parity Error Recovery Enable */;
pub const PCI_X_CMD_ERO: u64 = 0x0002	/* Enable Relaxed Ordering */;
pub const PCI_X_CMD_READ_512: u64 = 0x0000	/* 512 byte maximum read byte count */;
pub const PCI_X_CMD_READ_1K: u64 = 0x0004	/* 1Kbyte maximum read byte count */;
pub const PCI_X_CMD_READ_2K: u64 = 0x0008	/* 2Kbyte maximum read byte count */;
pub const PCI_X_CMD_READ_4K: u64 = 0x000c	/* 4Kbyte maximum read byte count */;
pub const PCI_X_CMD_MAX_READ: u64 = 0x000c	/* Max Memory Read Byte Count */;
				/* Max # of outstanding split transactions */
pub const PCI_X_CMD_SPLIT_1: u64 = 0x0000	/* Max 1 */;
pub const PCI_X_CMD_SPLIT_2: u64 = 0x0010	/* Max 2 */;
pub const PCI_X_CMD_SPLIT_3: u64 = 0x0020	/* Max 3 */;
pub const PCI_X_CMD_SPLIT_4: u64 = 0x0030	/* Max 4 */;
pub const PCI_X_CMD_SPLIT_8: u64 = 0x0040	/* Max 8 */;
pub const PCI_X_CMD_SPLIT_12: u64 = 0x0050	/* Max 12 */;
pub const PCI_X_CMD_SPLIT_16: u64 = 0x0060	/* Max 16 */;
pub const PCI_X_CMD_SPLIT_32: u64 = 0x0070	/* Max 32 */;
pub const PCI_X_CMD_MAX_SPLIT: u64 = 0x0070	/* Max Outstanding Split Transactions */;
macro_rules! PCI_X_CMD_VERSION { ($x:ident) => { (((($x) >> 12) & 3) /* Version */) }; }
pub const PCI_X_STATUS: u64 = 4	/* PCI-X capabilities */;
pub const PCI_X_STATUS_DEVFN: u64 = 0x000000ff	/* A copy of devfn */;
pub const PCI_X_STATUS_BUS: u64 = 0x0000ff00	/* A copy of bus nr */;
pub const PCI_X_STATUS_64BIT: u64 = 0x00010000	/* 64-bit device */;
pub const PCI_X_STATUS_133MHZ: u64 = 0x00020000	/* 133 MHz capable */;
pub const PCI_X_STATUS_SPL_DISC: u64 = 0x00040000	/* Split Completion Discarded */;
pub const PCI_X_STATUS_UNX_SPL: u64 = 0x00080000	/* Unexpected Split Completion */;
pub const PCI_X_STATUS_COMPLEX: u64 = 0x00100000	/* Device Complexity */;
pub const PCI_X_STATUS_MAX_READ: u64 = 0x00600000	/* Designed Max Memory Read Count */;
pub const PCI_X_STATUS_MAX_SPLIT: u64 = 0x03800000	/* Designed Max Outstanding Split Transactions */;
pub const PCI_X_STATUS_MAX_CUM: u64 = 0x1c000000	/* Designed Max Cumulative Read Size */;
pub const PCI_X_STATUS_SPL_ERR: u64 = 0x20000000	/* Rcvd Split Completion Error Msg */;
pub const PCI_X_STATUS_266MHZ: u64 = 0x40000000	/* 266 MHz capable */;
pub const PCI_X_STATUS_533MHZ: u64 = 0x80000000	/* 533 MHz capable */;
pub const PCI_X_ECC_CSR: u64 = 8	/* ECC control and status */;
pub const PCI_CAP_PCIX_SIZEOF_V0: u64 = 8	/* size of registers for Version 0 */;
pub const PCI_CAP_PCIX_SIZEOF_V1: u64 = 24	/* size for Version 1 */;
pub const PCI_CAP_PCIX_SIZEOF_V2: u64 = PCI_CAP_PCIX_SIZEOF_V1	/* Same for v2 */;

/* PCI-X registers (Type 1 (bridge) devices) */

pub const PCI_X_BRIDGE_SSTATUS: u64 = 2	/* Secondary Status */;
pub const PCI_X_SSTATUS_64BIT: u64 = 0x0001	/* Secondary AD interface is 64 bits */;
pub const PCI_X_SSTATUS_133MHZ: u64 = 0x0002	/* 133 MHz capable */;
pub const PCI_X_SSTATUS_FREQ: u64 = 0x03c0	/* Secondary Bus Mode and Frequency */;
pub const PCI_X_SSTATUS_VERS: u64 = 0x3000	/* PCI-X Capability Version */;
pub const PCI_X_SSTATUS_V1: u64 = 0x1000	/* Mode 2, not Mode 1 */;
pub const PCI_X_SSTATUS_V2: u64 = 0x2000	/* Mode 1 or Modes 1 and 2 */;
pub const PCI_X_SSTATUS_266MHZ: u64 = 0x4000	/* 266 MHz capable */;
pub const PCI_X_SSTATUS_533MHZ: u64 = 0x8000	/* 533 MHz capable */;
pub const PCI_X_BRIDGE_STATUS: u64 = 4	/* Bridge Status */;

/* PCI Bridge Subsystem ID registers */

pub const PCI_SSVID_VENDOR_ID: u64 = 4	/* PCI Bridge subsystem vendor ID */;
pub const PCI_SSVID_DEVICE_ID: u64 = 6	/* PCI Bridge subsystem device ID */;

/* PCI Express capability registers */

pub const PCI_EXP_FLAGS: u64 = 0x02	/* Capabilities register */;
pub const PCI_EXP_FLAGS_VERS: u64 = 0x000f	/* Capability version */;
pub const PCI_EXP_FLAGS_TYPE: u64 = 0x00f0	/* Device/Port type */;
pub const PCI_EXP_TYPE_ENDPOINT: u64 = 0x0	/* Express Endpoint */;
pub const PCI_EXP_TYPE_LEG_END: u64 = 0x1	/* Legacy Endpoint */;
pub const PCI_EXP_TYPE_ROOT_PORT: u64 = 0x4	/* Root Port */;
pub const PCI_EXP_TYPE_UPSTREAM: u64 = 0x5	/* Upstream Port */;
pub const PCI_EXP_TYPE_DOWNSTREAM: u64 = 0x6	/* Downstream Port */;
pub const PCI_EXP_TYPE_PCI_BRIDGE: u64 = 0x7	/* PCIe to PCI/PCI-X Bridge */;
pub const PCI_EXP_TYPE_PCIE_BRIDGE: u64 = 0x8	/* PCI/PCI-X to PCIe Bridge */;
pub const PCI_EXP_TYPE_RC_END: u64 = 0x9	/* Root Complex Integrated Endpoint */;
pub const PCI_EXP_TYPE_RC_EC: u64 = 0xa	/* Root Complex Event Collector */;
pub const PCI_EXP_FLAGS_SLOT: u64 = 0x0100	/* Slot implemented */;
pub const PCI_EXP_FLAGS_IRQ: u64 = 0x3e00	/* Interrupt message number */;
pub const PCI_EXP_FLAGS_FLIT: u64 = 0x8000	/* Flit Mode Supported */;
pub const PCI_EXP_DEVCAP: u64 = 0x04	/* Device capabilities */;
pub const PCI_EXP_DEVCAP_PAYLOAD: u64 = 0x00000007 /* Max_Payload_Size */;
pub const PCI_EXP_DEVCAP_PHANTOM: u64 = 0x00000018 /* Phantom functions */;
pub const PCI_EXP_DEVCAP_EXT_TAG: u64 = 0x00000020 /* Extended tags */;
pub const PCI_EXP_DEVCAP_L0S: u64 = 0x000001c0 /* L0s Acceptable Latency */;
pub const PCI_EXP_DEVCAP_L1: u64 = 0x00000e00 /* L1 Acceptable Latency */;
pub const PCI_EXP_DEVCAP_ATN_BUT: u64 = 0x00001000 /* Attention Button Present */;
pub const PCI_EXP_DEVCAP_ATN_IND: u64 = 0x00002000 /* Attention Indicator Present */;
pub const PCI_EXP_DEVCAP_PWR_IND: u64 = 0x00004000 /* Power Indicator Present */;
pub const PCI_EXP_DEVCAP_RBER: u64 = 0x00008000 /* Role-Based Error Reporting */;
pub const PCI_EXP_DEVCAP_PWR_VAL: u64 = 0x03fc0000 /* Slot Power Limit Value */;
pub const PCI_EXP_DEVCAP_PWR_SCL: u64 = 0x0c000000 /* Slot Power Limit Scale */;
pub const PCI_EXP_DEVCAP_FLR: u64 = 0x10000000 /* Function Level Reset */;
pub const PCI_EXP_DEVCAP_TEE: u64 = 0x40000000 /* TEE I/O (TDISP) Support */;
pub const PCI_EXP_DEVCTL: u64 = 0x08	/* Device Control */;
pub const PCI_EXP_DEVCTL_CERE: u64 = 0x0001	/* Correctable Error Reporting En. */;
pub const PCI_EXP_DEVCTL_NFERE: u64 = 0x0002	/* Non-Fatal Error Reporting Enable */;
pub const PCI_EXP_DEVCTL_FERE: u64 = 0x0004	/* Fatal Error Reporting Enable */;
pub const PCI_EXP_DEVCTL_URRE: u64 = 0x0008	/* Unsupported Request Reporting En. */;
pub const PCI_EXP_DEVCTL_RELAX_EN: u64 = 0x0010 /* Enable relaxed ordering */;
pub const PCI_EXP_DEVCTL_PAYLOAD: u64 = 0x00e0	/* Max_Payload_Size */;
pub const PCI_EXP_DEVCTL_PAYLOAD_128B: u64 = 0x0000 /* 128 Bytes */;
pub const PCI_EXP_DEVCTL_PAYLOAD_256B: u64 = 0x0020 /* 256 Bytes */;
pub const PCI_EXP_DEVCTL_PAYLOAD_512B: u64 = 0x0040 /* 512 Bytes */;
pub const PCI_EXP_DEVCTL_PAYLOAD_1024B: u64 = 0x0060 /* 1024 Bytes */;
pub const PCI_EXP_DEVCTL_PAYLOAD_2048B: u64 = 0x0080 /* 2048 Bytes */;
pub const PCI_EXP_DEVCTL_PAYLOAD_4096B: u64 = 0x00a0 /* 4096 Bytes */;
pub const PCI_EXP_DEVCTL_EXT_TAG: u64 = 0x0100	/* Extended Tag Field Enable */;
pub const PCI_EXP_DEVCTL_PHANTOM: u64 = 0x0200	/* Phantom Functions Enable */;
pub const PCI_EXP_DEVCTL_AUX_PME: u64 = 0x0400	/* Auxiliary Power PM Enable */;
pub const PCI_EXP_DEVCTL_NOSNOOP_EN: u64 = 0x0800  /* Enable No Snoop */;
pub const PCI_EXP_DEVCTL_READRQ: u64 = 0x7000	/* Max_Read_Request_Size */;
pub const PCI_EXP_DEVCTL_READRQ_128B: u64 = 0x0000 /* 128 Bytes */;
pub const PCI_EXP_DEVCTL_READRQ_256B: u64 = 0x1000 /* 256 Bytes */;
pub const PCI_EXP_DEVCTL_READRQ_512B: u64 = 0x2000 /* 512 Bytes */;
pub const PCI_EXP_DEVCTL_READRQ_1024B: u64 = 0x3000 /* 1024 Bytes */;
pub const PCI_EXP_DEVCTL_READRQ_2048B: u64 = 0x4000 /* 2048 Bytes */;
pub const PCI_EXP_DEVCTL_READRQ_4096B: u64 = 0x5000 /* 4096 Bytes */;
pub const PCI_EXP_DEVCTL_BCR_FLR: u64 = 0x8000  /* Bridge Configuration Retry / FLR */;
pub const PCI_EXP_DEVSTA: u64 = 0x0a	/* Device Status */;
pub const PCI_EXP_DEVSTA_CED: u64 = 0x0001	/* Correctable Error Detected */;
pub const PCI_EXP_DEVSTA_NFED: u64 = 0x0002	/* Non-Fatal Error Detected */;
pub const PCI_EXP_DEVSTA_FED: u64 = 0x0004	/* Fatal Error Detected */;
pub const PCI_EXP_DEVSTA_URD: u64 = 0x0008	/* Unsupported Request Detected */;
pub const PCI_EXP_DEVSTA_AUXPD: u64 = 0x0010	/* AUX Power Detected */;
pub const PCI_EXP_DEVSTA_TRPND: u64 = 0x0020	/* Transactions Pending */;
pub const PCI_CAP_EXP_RC_ENDPOINT_SIZEOF_V1: u64 = 12	/* v1 endpoints without link end here */;
pub const PCI_EXP_LNKCAP: u64 = 0x0c	/* Link Capabilities */;
pub const PCI_EXP_LNKCAP_SLS: u64 = 0x0000000f /* Max Link Speed (prior to PCIe r3.0: Supported Link Speeds) */;
pub const PCI_EXP_LNKCAP_SLS_2_5GB: u64 = 0x00000001 /* LNKCAP2 SLS Vector bit 0 */;
pub const PCI_EXP_LNKCAP_SLS_5_0GB: u64 = 0x00000002 /* LNKCAP2 SLS Vector bit 1 */;
pub const PCI_EXP_LNKCAP_SLS_8_0GB: u64 = 0x00000003 /* LNKCAP2 SLS Vector bit 2 */;
pub const PCI_EXP_LNKCAP_SLS_16_0GB: u64 = 0x00000004 /* LNKCAP2 SLS Vector bit 3 */;
pub const PCI_EXP_LNKCAP_SLS_32_0GB: u64 = 0x00000005 /* LNKCAP2 SLS Vector bit 4 */;
pub const PCI_EXP_LNKCAP_SLS_64_0GB: u64 = 0x00000006 /* LNKCAP2 SLS Vector bit 5 */;
pub const PCI_EXP_LNKCAP_MLW: u64 = 0x000003f0 /* Maximum Link Width */;
pub const PCI_EXP_LNKCAP_ASPMS: u64 = 0x00000c00 /* ASPM Support */;
pub const PCI_EXP_LNKCAP_ASPM_L0S: u64 = 0x00000400 /* ASPM L0s Support */;
pub const PCI_EXP_LNKCAP_ASPM_L1: u64 = 0x00000800 /* ASPM L1 Support */;
pub const PCI_EXP_LNKCAP_L0SEL: u64 = 0x00007000 /* L0s Exit Latency */;
pub const PCI_EXP_LNKCAP_L1EL: u64 = 0x00038000 /* L1 Exit Latency */;
pub const PCI_EXP_LNKCAP_CLKPM: u64 = 0x00040000 /* Clock Power Management */;
pub const PCI_EXP_LNKCAP_SDERC: u64 = 0x00080000 /* Surprise Down Error Reporting Capable */;
pub const PCI_EXP_LNKCAP_DLLLARC: u64 = 0x00100000 /* Data Link Layer Link Active Reporting Capable */;
pub const PCI_EXP_LNKCAP_LBNC: u64 = 0x00200000 /* Link Bandwidth Notification Capability */;
pub const PCI_EXP_LNKCAP_PN: u64 = 0xff000000 /* Port Number */;
pub const PCI_EXP_LNKCTL: u64 = 0x10	/* Link Control */;
pub const PCI_EXP_LNKCTL_ASPMC: u64 = 0x0003	/* ASPM Control */;
pub const PCI_EXP_LNKCTL_ASPM_L0S: u64 = 0x0001	/* L0s Enable */;
pub const PCI_EXP_LNKCTL_ASPM_L1: u64 = 0x0002	/* L1 Enable */;
pub const PCI_EXP_LNKCTL_RCB: u64 = 0x0008	/* Read Completion Boundary */;
pub const PCI_EXP_LNKCTL_LD: u64 = 0x0010	/* Link Disable */;
pub const PCI_EXP_LNKCTL_RL: u64 = 0x0020	/* Retrain Link */;
pub const PCI_EXP_LNKCTL_CCC: u64 = 0x0040	/* Common Clock Configuration */;
pub const PCI_EXP_LNKCTL_ES: u64 = 0x0080	/* Extended Synch */;
pub const PCI_EXP_LNKCTL_CLKREQ_EN: u64 = 0x0100 /* Enable clkreq */;
pub const PCI_EXP_LNKCTL_HAWD: u64 = 0x0200	/* Hardware Autonomous Width Disable */;
pub const PCI_EXP_LNKCTL_LBMIE: u64 = 0x0400	/* Link Bandwidth Management Interrupt Enable */;
pub const PCI_EXP_LNKCTL_LABIE: u64 = 0x0800	/* Link Autonomous Bandwidth Interrupt Enable */;
pub const PCI_EXP_LNKSTA: u64 = 0x12	/* Link Status */;
pub const PCI_EXP_LNKSTA_CLS: u64 = 0x000f	/* Current Link Speed */;
pub const PCI_EXP_LNKSTA_CLS_2_5GB: u64 = 0x0001 /* Current Link Speed 2.5GT/s */;
pub const PCI_EXP_LNKSTA_CLS_5_0GB: u64 = 0x0002 /* Current Link Speed 5.0GT/s */;
pub const PCI_EXP_LNKSTA_CLS_8_0GB: u64 = 0x0003 /* Current Link Speed 8.0GT/s */;
pub const PCI_EXP_LNKSTA_CLS_16_0GB: u64 = 0x0004 /* Current Link Speed 16.0GT/s */;
pub const PCI_EXP_LNKSTA_CLS_32_0GB: u64 = 0x0005 /* Current Link Speed 32.0GT/s */;
pub const PCI_EXP_LNKSTA_CLS_64_0GB: u64 = 0x0006 /* Current Link Speed 64.0GT/s */;
pub const PCI_EXP_LNKSTA_NLW: u64 = 0x03f0	/* Negotiated Link Width */;
pub const PCI_EXP_LNKSTA_NLW_X1: u64 = 0x0010	/* Current Link Width x1 */;
pub const PCI_EXP_LNKSTA_NLW_X2: u64 = 0x0020	/* Current Link Width x2 */;
pub const PCI_EXP_LNKSTA_NLW_X4: u64 = 0x0040	/* Current Link Width x4 */;
pub const PCI_EXP_LNKSTA_NLW_X8: u64 = 0x0080	/* Current Link Width x8 */;
pub const PCI_EXP_LNKSTA_NLW_SHIFT: u64 = 4	/* start of NLW mask in link status */;
pub const PCI_EXP_LNKSTA_LT: u64 = 0x0800	/* Link Training */;
pub const PCI_EXP_LNKSTA_SLC: u64 = 0x1000	/* Slot Clock Configuration */;
pub const PCI_EXP_LNKSTA_DLLLA: u64 = 0x2000	/* Data Link Layer Link Active */;
pub const PCI_EXP_LNKSTA_LBMS: u64 = 0x4000	/* Link Bandwidth Management Status */;
pub const PCI_EXP_LNKSTA_LABS: u64 = 0x8000	/* Link Autonomous Bandwidth Status */;
pub const PCI_CAP_EXP_ENDPOINT_SIZEOF_V1: u64 = 20	/* v1 endpoints with link end here */;
pub const PCI_EXP_SLTCAP: u64 = 0x14	/* Slot Capabilities */;
pub const PCI_EXP_SLTCAP_ABP: u64 = 0x00000001 /* Attention Button Present */;
pub const PCI_EXP_SLTCAP_PCP: u64 = 0x00000002 /* Power Controller Present */;
pub const PCI_EXP_SLTCAP_MRLSP: u64 = 0x00000004 /* MRL Sensor Present */;
pub const PCI_EXP_SLTCAP_AIP: u64 = 0x00000008 /* Attention Indicator Present */;
pub const PCI_EXP_SLTCAP_PIP: u64 = 0x00000010 /* Power Indicator Present */;
pub const PCI_EXP_SLTCAP_HPS: u64 = 0x00000020 /* Hot-Plug Surprise */;
pub const PCI_EXP_SLTCAP_HPC: u64 = 0x00000040 /* Hot-Plug Capable */;
pub const PCI_EXP_SLTCAP_SPLV: u64 = 0x00007f80 /* Slot Power Limit Value */;
pub const PCI_EXP_SLTCAP_SPLS: u64 = 0x00018000 /* Slot Power Limit Scale */;
pub const PCI_EXP_SLTCAP_EIP: u64 = 0x00020000 /* Electromechanical Interlock Present */;
pub const PCI_EXP_SLTCAP_NCCS: u64 = 0x00040000 /* No Command Completed Support */;
pub const PCI_EXP_SLTCAP_PSN: u64 = 0xfff80000 /* Physical Slot Number */;
pub const PCI_EXP_SLTCTL: u64 = 0x18	/* Slot Control */;
pub const PCI_EXP_SLTCTL_ABPE: u64 = 0x0001	/* Attention Button Pressed Enable */;
pub const PCI_EXP_SLTCTL_PFDE: u64 = 0x0002	/* Power Fault Detected Enable */;
pub const PCI_EXP_SLTCTL_MRLSCE: u64 = 0x0004	/* MRL Sensor Changed Enable */;
pub const PCI_EXP_SLTCTL_PDCE: u64 = 0x0008	/* Presence Detect Changed Enable */;
pub const PCI_EXP_SLTCTL_CCIE: u64 = 0x0010	/* Command Completed Interrupt Enable */;
pub const PCI_EXP_SLTCTL_HPIE: u64 = 0x0020	/* Hot-Plug Interrupt Enable */;
pub const PCI_EXP_SLTCTL_AIC: u64 = 0x00c0	/* Attention Indicator Control */;
pub const PCI_EXP_SLTCTL_ATTN_IND_SHIFT: u64 = 6      /* Attention Indicator shift */;
pub const PCI_EXP_SLTCTL_ATTN_IND_ON: u64 = 0x0040 /* Attention Indicator on */;
pub const PCI_EXP_SLTCTL_ATTN_IND_BLINK: u64 = 0x0080 /* Attention Indicator blinking */;
pub const PCI_EXP_SLTCTL_ATTN_IND_OFF: u64 = 0x00c0 /* Attention Indicator off */;
pub const PCI_EXP_SLTCTL_PIC: u64 = 0x0300	/* Power Indicator Control */;
pub const PCI_EXP_SLTCTL_PWR_IND_ON: u64 = 0x0100 /* Power Indicator on */;
pub const PCI_EXP_SLTCTL_PWR_IND_BLINK: u64 = 0x0200 /* Power Indicator blinking */;
pub const PCI_EXP_SLTCTL_PWR_IND_OFF: u64 = 0x0300 /* Power Indicator off */;
pub const PCI_EXP_SLTCTL_PCC: u64 = 0x0400	/* Power Controller Control */;
pub const PCI_EXP_SLTCTL_PWR_ON: u64 = 0x0000 /* Power On */;
pub const PCI_EXP_SLTCTL_PWR_OFF: u64 = 0x0400 /* Power Off */;
pub const PCI_EXP_SLTCTL_EIC: u64 = 0x0800	/* Electromechanical Interlock Control */;
pub const PCI_EXP_SLTCTL_DLLSCE: u64 = 0x1000	/* Data Link Layer State Changed Enable */;
pub const PCI_EXP_SLTCTL_ASPL_DISABLE: u64 = 0x2000 /* Auto Slot Power Limit Disable */;
pub const PCI_EXP_SLTCTL_IBPD_DISABLE: u64 = 0x4000 /* In-band PD disable */;
pub const PCI_EXP_SLTSTA: u64 = 0x1a	/* Slot Status */;
pub const PCI_EXP_SLTSTA_ABP: u64 = 0x0001	/* Attention Button Pressed */;
pub const PCI_EXP_SLTSTA_PFD: u64 = 0x0002	/* Power Fault Detected */;
pub const PCI_EXP_SLTSTA_MRLSC: u64 = 0x0004	/* MRL Sensor Changed */;
pub const PCI_EXP_SLTSTA_PDC: u64 = 0x0008	/* Presence Detect Changed */;
pub const PCI_EXP_SLTSTA_CC: u64 = 0x0010	/* Command Completed */;
pub const PCI_EXP_SLTSTA_MRLSS: u64 = 0x0020	/* MRL Sensor State */;
pub const PCI_EXP_SLTSTA_PDS: u64 = 0x0040	/* Presence Detect State */;
pub const PCI_EXP_SLTSTA_EIS: u64 = 0x0080	/* Electromechanical Interlock Status */;
pub const PCI_EXP_SLTSTA_DLLSC: u64 = 0x0100	/* Data Link Layer State Changed */;
pub const PCI_EXP_RTCTL: u64 = 0x1c	/* Root Control */;
pub const PCI_EXP_RTCTL_SECEE: u64 = 0x0001	/* System Error on Correctable Error */;
pub const PCI_EXP_RTCTL_SENFEE: u64 = 0x0002	/* System Error on Non-Fatal Error */;
pub const PCI_EXP_RTCTL_SEFEE: u64 = 0x0004	/* System Error on Fatal Error */;
pub const PCI_EXP_RTCTL_PMEIE: u64 = 0x0008	/* PME Interrupt Enable */;
pub const PCI_EXP_RTCTL_RRS_SVE: u64 = 0x0010	/* Config RRS Software Visibility Enable */;
pub const PCI_EXP_RTCTL_CRSSVE: u64 = PCI_EXP_RTCTL_RRS_SVE /* compatibility */;
pub const PCI_EXP_RTCAP: u64 = 0x1e	/* Root Capabilities */;
pub const PCI_EXP_RTCAP_RRS_SV: u64 = 0x0001	/* Config RRS Software Visibility */;
pub const PCI_EXP_RTCAP_CRSVIS: u64 = PCI_EXP_RTCAP_RRS_SV /* compatibility */;
pub const PCI_EXP_RTSTA: u64 = 0x20	/* Root Status */;
pub const PCI_EXP_RTSTA_PME_RQ_ID: u64 = 0x0000ffff /* PME Requester ID */;
pub const PCI_EXP_RTSTA_PME: u64 = 0x00010000 /* PME status */;
pub const PCI_EXP_RTSTA_PENDING: u64 = 0x00020000 /* PME pending */;
/*
 * The Device Capabilities 2, Device Status 2, Device Control 2,
 * Link Capabilities 2, Link Status 2, Link Control 2,
 * Slot Capabilities 2, Slot Status 2, and Slot Control 2 registers
 * are only present on devices with PCIe Capability version 2.
 * Use pcie_capability_read_word() and similar interfaces to use them
 * safely.
 */
pub const PCI_EXP_DEVCAP2: u64 = 0x24	/* Device Capabilities 2 */;
pub const PCI_EXP_DEVCAP2_COMP_TMOUT_DIS: u64 = 0x00000010 /* Completion Timeout Disable supported */;
pub const PCI_EXP_DEVCAP2_ARI: u64 = 0x00000020 /* Alternative Routing-ID */;
pub const PCI_EXP_DEVCAP2_ATOMIC_ROUTE: u64 = 0x00000040 /* Atomic Op routing */;
pub const PCI_EXP_DEVCAP2_ATOMIC_COMP32: u64 = 0x00000080 /* 32b AtomicOp completion */;
pub const PCI_EXP_DEVCAP2_ATOMIC_COMP64: u64 = 0x00000100 /* 64b AtomicOp completion */;
pub const PCI_EXP_DEVCAP2_ATOMIC_COMP128: u64 = 0x00000200 /* 128b AtomicOp completion */;
pub const PCI_EXP_DEVCAP2_LTR: u64 = 0x00000800 /* Latency tolerance reporting */;
pub const PCI_EXP_DEVCAP2_TPH_COMP_MASK: u64 = 0x00003000 /* TPH completer support */;
pub const PCI_EXP_DEVCAP2_OBFF_MASK: u64 = 0x000c0000 /* OBFF support mechanism */;
pub const PCI_EXP_DEVCAP2_OBFF_MSG: u64 = 0x00040000 /* New message signaling */;
pub const PCI_EXP_DEVCAP2_OBFF_WAKE: u64 = 0x00080000 /* Re-use WAKE# for OBFF */;
pub const PCI_EXP_DEVCAP2_EE_PREFIX: u64 = 0x00200000 /* End-End TLP Prefix */;
pub const PCI_EXP_DEVCAP2_EE_PREFIX_MAX: u64 = 0x00c00000 /* Max End-End TLP Prefixes */;
pub const PCI_EXP_DEVCTL2: u64 = 0x28	/* Device Control 2 */;
pub const PCI_EXP_DEVCTL2_COMP_TIMEOUT: u64 = 0x000f	/* Completion Timeout Value */;
pub const PCI_EXP_DEVCTL2_COMP_TMOUT_DIS: u64 = 0x0010	/* Completion Timeout Disable */;
pub const PCI_EXP_DEVCTL2_ARI: u64 = 0x0020	/* Alternative Routing-ID */;
pub const PCI_EXP_DEVCTL2_ATOMIC_REQ: u64 = 0x0040	/* Set Atomic requests */;
pub const PCI_EXP_DEVCTL2_ATOMIC_EGRESS_BLOCK: u64 = 0x0080 /* Block atomic egress */;
pub const PCI_EXP_DEVCTL2_IDO_REQ_EN: u64 = 0x0100	/* Allow IDO for requests */;
pub const PCI_EXP_DEVCTL2_IDO_CMP_EN: u64 = 0x0200	/* Allow IDO for completions */;
pub const PCI_EXP_DEVCTL2_LTR_EN: u64 = 0x0400	/* Enable LTR mechanism */;
pub const PCI_EXP_DEVCTL2_OBFF_MSGA_EN: u64 = 0x2000	/* Enable OBFF Message type A */;
pub const PCI_EXP_DEVCTL2_OBFF_MSGB_EN: u64 = 0x4000	/* Enable OBFF Message type B */;
pub const PCI_EXP_DEVCTL2_OBFF_WAKE_EN: u64 = 0x6000	/* OBFF using WAKE# signaling */;
pub const PCI_EXP_DEVSTA2: u64 = 0x2a	/* Device Status 2 */;
pub const PCI_CAP_EXP_RC_ENDPOINT_SIZEOF_V2: u64 = 0x2c	/* end of v2 EPs w/o link */;
pub const PCI_EXP_LNKCAP2: u64 = 0x2c	/* Link Capabilities 2 */;
pub const PCI_EXP_LNKCAP2_SLS: u64 = 0x000000fe /* Supported Link Speeds Vector */;
pub const PCI_EXP_LNKCAP2_SLS_2_5GB: u64 = 0x00000002 /* Supported Speed 2.5GT/s */;
pub const PCI_EXP_LNKCAP2_SLS_5_0GB: u64 = 0x00000004 /* Supported Speed 5GT/s */;
pub const PCI_EXP_LNKCAP2_SLS_8_0GB: u64 = 0x00000008 /* Supported Speed 8GT/s */;
pub const PCI_EXP_LNKCAP2_SLS_16_0GB: u64 = 0x00000010 /* Supported Speed 16GT/s */;
pub const PCI_EXP_LNKCAP2_SLS_32_0GB: u64 = 0x00000020 /* Supported Speed 32GT/s */;
pub const PCI_EXP_LNKCAP2_SLS_64_0GB: u64 = 0x00000040 /* Supported Speed 64GT/s */;
pub const PCI_EXP_LNKCAP2_CROSSLINK: u64 = 0x00000100 /* Crosslink supported */;
pub const PCI_EXP_LNKCTL2: u64 = 0x30	/* Link Control 2 */;
pub const PCI_EXP_LNKCTL2_TLS: u64 = 0x000f;
pub const PCI_EXP_LNKCTL2_TLS_2_5GT: u64 = 0x0001 /* Supported Speed 2.5GT/s */;
pub const PCI_EXP_LNKCTL2_TLS_5_0GT: u64 = 0x0002 /* Supported Speed 5GT/s */;
pub const PCI_EXP_LNKCTL2_TLS_8_0GT: u64 = 0x0003 /* Supported Speed 8GT/s */;
pub const PCI_EXP_LNKCTL2_TLS_16_0GT: u64 = 0x0004 /* Supported Speed 16GT/s */;
pub const PCI_EXP_LNKCTL2_TLS_32_0GT: u64 = 0x0005 /* Supported Speed 32GT/s */;
pub const PCI_EXP_LNKCTL2_TLS_64_0GT: u64 = 0x0006 /* Supported Speed 64GT/s */;
pub const PCI_EXP_LNKCTL2_ENTER_COMP: u64 = 0x0010 /* Enter Compliance */;
pub const PCI_EXP_LNKCTL2_TX_MARGIN: u64 = 0x0380 /* Transmit Margin */;
pub const PCI_EXP_LNKCTL2_HASD: u64 = 0x0020 /* HW Autonomous Speed Disable */;
pub const PCI_EXP_LNKSTA2: u64 = 0x32	/* Link Status 2 */;
pub const PCI_EXP_LNKSTA2_FLIT: u64 = 0x0400 /* Flit Mode Status */;
pub const PCI_CAP_EXP_ENDPOINT_SIZEOF_V2: u64 = 0x34	/* end of v2 EPs w/ link */;
pub const PCI_EXP_SLTCAP2: u64 = 0x34	/* Slot Capabilities 2 */;
pub const PCI_EXP_SLTCAP2_IBPD: u64 = 0x00000001 /* In-band PD Disable Supported */;
pub const PCI_EXP_SLTCTL2: u64 = 0x38	/* Slot Control 2 */;
pub const PCI_EXP_SLTSTA2: u64 = 0x3a	/* Slot Status 2 */;

/* Extended Capabilities (PCI-X 2.0 and Express) */
macro_rules! PCI_EXT_CAP_ID { ($header:ident) => { (($header & 0x0000ffff)) }; }
macro_rules! PCI_EXT_CAP_VER { ($header:ident) => { ((($header >> 16) & 0xf)) }; }
macro_rules! PCI_EXT_CAP_NEXT { ($header:ident) => { ((($header >> 20) & 0xffc)) }; }

pub const PCI_EXT_CAP_ID_ERR: u64 = 0x01	/* Advanced Error Reporting */;
pub const PCI_EXT_CAP_ID_VC: u64 = 0x02	/* Virtual Channel Capability */;
pub const PCI_EXT_CAP_ID_DSN: u64 = 0x03	/* Device Serial Number */;
pub const PCI_EXT_CAP_ID_PWR: u64 = 0x04	/* Power Budgeting */;
pub const PCI_EXT_CAP_ID_RCLD: u64 = 0x05	/* Root Complex Link Declaration */;
pub const PCI_EXT_CAP_ID_RCILC: u64 = 0x06	/* Root Complex Internal Link Control */;
pub const PCI_EXT_CAP_ID_RCEC: u64 = 0x07	/* Root Complex Event Collector */;
pub const PCI_EXT_CAP_ID_MFVC: u64 = 0x08	/* Multi-Function VC Capability */;
pub const PCI_EXT_CAP_ID_VC9: u64 = 0x09	/* same as _VC */;
pub const PCI_EXT_CAP_ID_RCRB: u64 = 0x0A	/* Root Complex RB? */;
pub const PCI_EXT_CAP_ID_VNDR: u64 = 0x0B	/* Vendor-Specific */;
pub const PCI_EXT_CAP_ID_CAC: u64 = 0x0C	/* Config Access - obsolete */;
pub const PCI_EXT_CAP_ID_ACS: u64 = 0x0D	/* Access Control Services */;
pub const PCI_EXT_CAP_ID_ARI: u64 = 0x0E	/* Alternate Routing ID */;
pub const PCI_EXT_CAP_ID_ATS: u64 = 0x0F	/* Address Translation Services */;
pub const PCI_EXT_CAP_ID_SRIOV: u64 = 0x10	/* Single Root I/O Virtualization */;
pub const PCI_EXT_CAP_ID_MRIOV: u64 = 0x11	/* Multi Root I/O Virtualization */;
pub const PCI_EXT_CAP_ID_MCAST: u64 = 0x12	/* Multicast */;
pub const PCI_EXT_CAP_ID_PRI: u64 = 0x13	/* Page Request Interface */;
pub const PCI_EXT_CAP_ID_AMD_XXX: u64 = 0x14	/* Reserved for AMD */;
pub const PCI_EXT_CAP_ID_REBAR: u64 = 0x15	/* Resizable BAR */;
pub const PCI_EXT_CAP_ID_DPA: u64 = 0x16	/* Dynamic Power Allocation */;
pub const PCI_EXT_CAP_ID_TPH: u64 = 0x17	/* TPH Requester */;
pub const PCI_EXT_CAP_ID_LTR: u64 = 0x18	/* Latency Tolerance Reporting */;
pub const PCI_EXT_CAP_ID_SECPCI: u64 = 0x19	/* Secondary PCIe Capability */;
pub const PCI_EXT_CAP_ID_PMUX: u64 = 0x1A	/* Protocol Multiplexing */;
pub const PCI_EXT_CAP_ID_PASID: u64 = 0x1B	/* Process Address Space ID */;
pub const PCI_EXT_CAP_ID_DPC: u64 = 0x1D	/* Downstream Port Containment */;
pub const PCI_EXT_CAP_ID_L1SS: u64 = 0x1E	/* L1 PM Substates */;
pub const PCI_EXT_CAP_ID_PTM: u64 = 0x1F	/* Precision Time Measurement */;
pub const PCI_EXT_CAP_ID_DVSEC: u64 = 0x23	/* Designated Vendor-Specific */;
pub const PCI_EXT_CAP_ID_VF_REBAR: u64 = 0x24	/* VF Resizable BAR */;
pub const PCI_EXT_CAP_ID_DLF: u64 = 0x25	/* Data Link Feature */;
pub const PCI_EXT_CAP_ID_PL_16GT: u64 = 0x26	/* Physical Layer 16.0 GT/s */;
pub const PCI_EXT_CAP_ID_NPEM: u64 = 0x29	/* Native PCIe Enclosure Management */;
pub const PCI_EXT_CAP_ID_PL_32GT: u64 = 0x2A    /* Physical Layer 32.0 GT/s */;
pub const PCI_EXT_CAP_ID_DOE: u64 = 0x2E	/* Data Object Exchange */;
pub const PCI_EXT_CAP_ID_DEV3: u64 = 0x2F	/* Device 3 Capability/Control/Status */;
pub const PCI_EXT_CAP_ID_IDE: u64 = 0x30    /* Integrity and Data Encryption */;
pub const PCI_EXT_CAP_ID_PL_64GT: u64 = 0x31	/* Physical Layer 64.0 GT/s */;
pub const PCI_EXT_CAP_ID_MAX: u64 = PCI_EXT_CAP_ID_PL_64GT;

pub const PCI_EXT_CAP_DSN_SIZEOF: u64 = 12;
pub const PCI_EXT_CAP_MCAST_ENDPOINT_SIZEOF: u64 = 40;

/* Advanced Error Reporting */
pub const PCI_ERR_UNCOR_STATUS: u64 = 0x04	/* Uncorrectable Error Status */;
pub const PCI_ERR_UNC_UND: u64 = 0x00000001	/* Undefined */;
pub const PCI_ERR_UNC_DLP: u64 = 0x00000010	/* Data Link Protocol */;
pub const PCI_ERR_UNC_SURPDN: u64 = 0x00000020	/* Surprise Down */;
pub const PCI_ERR_UNC_POISON_TLP: u64 = 0x00001000	/* Poisoned TLP */;
pub const PCI_ERR_UNC_FCP: u64 = 0x00002000	/* Flow Control Protocol */;
pub const PCI_ERR_UNC_COMP_TIME: u64 = 0x00004000	/* Completion Timeout */;
pub const PCI_ERR_UNC_COMP_ABORT: u64 = 0x00008000	/* Completer Abort */;
pub const PCI_ERR_UNC_UNX_COMP: u64 = 0x00010000	/* Unexpected Completion */;
pub const PCI_ERR_UNC_RX_OVER: u64 = 0x00020000	/* Receiver Overflow */;
pub const PCI_ERR_UNC_MALF_TLP: u64 = 0x00040000	/* Malformed TLP */;
pub const PCI_ERR_UNC_ECRC: u64 = 0x00080000	/* ECRC Error Status */;
pub const PCI_ERR_UNC_UNSUP: u64 = 0x00100000	/* Unsupported Request */;
pub const PCI_ERR_UNC_ACSV: u64 = 0x00200000	/* ACS Violation */;
pub const PCI_ERR_UNC_INTN: u64 = 0x00400000	/* internal error */;
pub const PCI_ERR_UNC_MCBTLP: u64 = 0x00800000	/* MC blocked TLP */;
pub const PCI_ERR_UNC_ATOMEG: u64 = 0x01000000	/* Atomic egress blocked */;
pub const PCI_ERR_UNC_TLPPRE: u64 = 0x02000000	/* TLP prefix blocked */;
pub const PCI_ERR_UNC_POISON_BLK: u64 = 0x04000000	/* Poisoned TLP Egress Blocked */;
pub const PCI_ERR_UNC_DMWR_BLK: u64 = 0x08000000	/* DMWr Request Egress Blocked */;
pub const PCI_ERR_UNC_IDE_CHECK: u64 = 0x10000000	/* IDE Check Failed */;
pub const PCI_ERR_UNC_MISR_IDE: u64 = 0x20000000	/* Misrouted IDE TLP */;
pub const PCI_ERR_UNC_PCRC_CHECK: u64 = 0x40000000	/* PCRC Check Failed */;
pub const PCI_ERR_UNC_XLAT_BLK: u64 = 0x80000000	/* TLP Translation Egress Blocked */;
pub const PCI_ERR_UNCOR_MASK: u64 = 0x08	/* Uncorrectable Error Mask */;
	/* Same bits as above */
pub const PCI_ERR_UNCOR_SEVER: u64 = 0x0c	/* Uncorrectable Error Severity */;
	/* Same bits as above */
pub const PCI_ERR_COR_STATUS: u64 = 0x10	/* Correctable Error Status */;
pub const PCI_ERR_COR_RCVR: u64 = 0x00000001	/* Receiver Error Status */;
pub const PCI_ERR_COR_BAD_TLP: u64 = 0x00000040	/* Bad TLP Status */;
pub const PCI_ERR_COR_BAD_DLLP: u64 = 0x00000080	/* Bad DLLP Status */;
pub const PCI_ERR_COR_REP_ROLL: u64 = 0x00000100	/* REPLAY_NUM Rollover */;
pub const PCI_ERR_COR_REP_TIMER: u64 = 0x00001000	/* Replay Timer Timeout */;
pub const PCI_ERR_COR_ADV_NFAT: u64 = 0x00002000	/* Advisory Non-Fatal */;
pub const PCI_ERR_COR_INTERNAL: u64 = 0x00004000	/* Corrected Internal */;
pub const PCI_ERR_COR_LOG_OVER: u64 = 0x00008000	/* Header Log Overflow */;
pub const PCI_ERR_COR_MASK: u64 = 0x14	/* Correctable Error Mask */;
	/* Same bits as above */
pub const PCI_ERR_CAP: u64 = 0x18	/* Advanced Error Capabilities & Ctrl*/;
macro_rules! PCI_ERR_CAP_FEP { ($x:ident) => { ((($x) & 0x1f)	/* First Error Pointer */) }; }
pub const PCI_ERR_CAP_ECRC_GENC: u64 = 0x00000020 /* ECRC Generation Capable */;
pub const PCI_ERR_CAP_ECRC_GENE: u64 = 0x00000040 /* ECRC Generation Enable */;
pub const PCI_ERR_CAP_ECRC_CHKC: u64 = 0x00000080 /* ECRC Check Capable */;
pub const PCI_ERR_CAP_ECRC_CHKE: u64 = 0x00000100 /* ECRC Check Enable */;
pub const PCI_ERR_CAP_PREFIX_LOG_PRESENT: u64 = 0x00000800 /* TLP Prefix Log Present */;
pub const PCI_ERR_CAP_COMP_TIME_LOG: u64 = 0x00001000 /* Completion Timeout Prefix/Header Log Capable */;
pub const PCI_ERR_CAP_TLP_LOG_FLIT: u64 = 0x00040000 /* TLP was logged in Flit Mode */;
pub const PCI_ERR_CAP_TLP_LOG_SIZE: u64 = 0x00f80000 /* Logged TLP Size (only in Flit mode) */;
pub const PCI_ERR_HEADER_LOG: u64 = 0x1c	/* Header Log Register (16 bytes) */;
pub const PCI_ERR_ROOT_COMMAND: u64 = 0x2c	/* Root Error Command */;
pub const PCI_ERR_ROOT_CMD_COR_EN: u64 = 0x00000001 /* Correctable Err Reporting Enable */;
pub const PCI_ERR_ROOT_CMD_NONFATAL_EN: u64 = 0x00000002 /* Non-Fatal Err Reporting Enable */;
pub const PCI_ERR_ROOT_CMD_FATAL_EN: u64 = 0x00000004 /* Fatal Err Reporting Enable */;
pub const PCI_ERR_ROOT_STATUS: u64 = 0x30;
pub const PCI_ERR_ROOT_COR_RCV: u64 = 0x00000001 /* ERR_COR Received */;
pub const PCI_ERR_ROOT_MULTI_COR_RCV: u64 = 0x00000002 /* Multiple ERR_COR */;
pub const PCI_ERR_ROOT_UNCOR_RCV: u64 = 0x00000004 /* ERR_FATAL/NONFATAL */;
pub const PCI_ERR_ROOT_MULTI_UNCOR_RCV: u64 = 0x00000008 /* Multiple FATAL/NONFATAL */;
pub const PCI_ERR_ROOT_FIRST_FATAL: u64 = 0x00000010 /* First UNC is Fatal */;
pub const PCI_ERR_ROOT_NONFATAL_RCV: u64 = 0x00000020 /* Non-Fatal Received */;
pub const PCI_ERR_ROOT_FATAL_RCV: u64 = 0x00000040 /* Fatal Received */;
pub const PCI_ERR_ROOT_AER_IRQ: u64 = 0xf8000000 /* Advanced Error Interrupt Message Number */;
pub const PCI_ERR_ROOT_ERR_SRC: u64 = 0x34	/* Error Source Identification */;
pub const PCI_ERR_PREFIX_LOG: u64 = 0x38	/* TLP Prefix LOG Register (up to 16 bytes) */;

/* Virtual Channel */
pub const PCI_VC_PORT_CAP1: u64 = 0x04;
pub const PCI_VC_CAP1_EVCC: u64 = 0x00000007	/* extended VC count */;
pub const PCI_VC_CAP1_LPEVCC: u64 = 0x00000070	/* low prio extended VC count */;
pub const PCI_VC_CAP1_ARB_SIZE: u64 = 0x00000c00;
pub const PCI_VC_PORT_CAP2: u64 = 0x08;
pub const PCI_VC_CAP2_32_PHASE: u64 = 0x00000002;
pub const PCI_VC_CAP2_64_PHASE: u64 = 0x00000004;
pub const PCI_VC_CAP2_128_PHASE: u64 = 0x00000008;
pub const PCI_VC_CAP2_ARB_OFF: u64 = 0xff000000;
pub const PCI_VC_PORT_CTRL: u64 = 0x0c;
pub const PCI_VC_PORT_CTRL_LOAD_TABLE: u64 = 0x00000001;
pub const PCI_VC_PORT_STATUS: u64 = 0x0e;
pub const PCI_VC_PORT_STATUS_TABLE: u64 = 0x00000001;
pub const PCI_VC_RES_CAP: u64 = 0x10;
pub const PCI_VC_RES_CAP_32_PHASE: u64 = 0x00000002;
pub const PCI_VC_RES_CAP_64_PHASE: u64 = 0x00000004;
pub const PCI_VC_RES_CAP_128_PHASE: u64 = 0x00000008;
pub const PCI_VC_RES_CAP_128_PHASE_TB: u64 = 0x00000010;
pub const PCI_VC_RES_CAP_256_PHASE: u64 = 0x00000020;
pub const PCI_VC_RES_CAP_ARB_OFF: u64 = 0xff000000;
pub const PCI_VC_RES_CTRL: u64 = 0x14;
pub const PCI_VC_RES_CTRL_LOAD_TABLE: u64 = 0x00010000;
pub const PCI_VC_RES_CTRL_ARB_SELECT: u64 = 0x000e0000;
pub const PCI_VC_RES_CTRL_ID: u64 = 0x07000000;
pub const PCI_VC_RES_CTRL_ENABLE: u64 = 0x80000000;
pub const PCI_VC_RES_STATUS: u64 = 0x1a;
pub const PCI_VC_RES_STATUS_TABLE: u64 = 0x00000001;
pub const PCI_VC_RES_STATUS_NEGO: u64 = 0x00000002;
pub const PCI_CAP_VC_BASE_SIZEOF: u64 = 0x10;
pub const PCI_CAP_VC_PER_VC_SIZEOF: u64 = 0x0c;

/* Power Budgeting */
pub const PCI_PWR_DSR: u64 = 0x04	/* Data Select Register */;
pub const PCI_PWR_DATA: u64 = 0x08	/* Data Register */;
macro_rules! PCI_PWR_DATA_BASE { ($x:ident) => { ((($x) & 0xff)	    /* Base Power */) }; }
macro_rules! PCI_PWR_DATA_SCALE { ($x:ident) => { (((($x) >> 8) & 3)    /* Data Scale */) }; }
macro_rules! PCI_PWR_DATA_PM_SUB { ($x:ident) => { (((($x) >> 10) & 7)   /* PM Sub State */) }; }
macro_rules! PCI_PWR_DATA_PM_STATE { ($x:ident) => { (((($x) >> 13) & 3) /* PM State */) }; }
macro_rules! PCI_PWR_DATA_TYPE { ($x:ident) => { (((($x) >> 15) & 7)   /* Type */) }; }
macro_rules! PCI_PWR_DATA_RAIL { ($x:ident) => { (((($x) >> 18) & 7)   /* Power Rail */) }; }
pub const PCI_PWR_CAP: u64 = 0x0c	/* Capability */;
macro_rules! PCI_PWR_CAP_BUDGET { ($x:ident) => { ((($x) & 1)	/* Included in system budget */) }; }
pub const PCI_EXT_CAP_PWR_SIZEOF: u64 = 0x10;

/* Root Complex Event Collector Endpoint Association  */
pub const PCI_RCEC_RCIEP_BITMAP: u64 = 4	/* Associated Bitmap for RCiEPs */;
pub const PCI_RCEC_BUSN: u64 = 8	/* RCEC Associated Bus Numbers */;
pub const PCI_RCEC_BUSN_REG_VER: u64 = 0x02	/* Least version with BUSN present */;
macro_rules! PCI_RCEC_BUSN_NEXT { ($x:ident) => { (((($x) >> 8) & 0xff)) }; }
macro_rules! PCI_RCEC_BUSN_LAST { ($x:ident) => { (((($x) >> 16) & 0xff)) }; }

/* Vendor-Specific (VSEC, PCI_EXT_CAP_ID_VNDR) */
pub const PCI_VNDR_HEADER: u64 = 4	/* Vendor-Specific Header */;
macro_rules! PCI_VNDR_HEADER_ID { ($x:ident) => { ((($x) & 0xffff)) }; }
macro_rules! PCI_VNDR_HEADER_REV { ($x:ident) => { (((($x) >> 16) & 0xf)) }; }
macro_rules! PCI_VNDR_HEADER_LEN { ($x:ident) => { (((($x) >> 20) & 0xfff)) }; }

/*
 * HyperTransport sub capability types
 *
 * Unfortunately there are both 3 bit and 5 bit capability types defined
 * in the HT spec, catering for that is a little messy. You probably don't
 * want to use these directly, just use pci_find_ht_capability() and it
 * will do the right thing for you.
 */
pub const HT_3BIT_CAP_MASK: u64 = 0xE0;
pub const HT_CAPTYPE_SLAVE: u64 = 0x00	/* Slave/Primary link configuration */;
pub const HT_CAPTYPE_HOST: u64 = 0x20	/* Host/Secondary link configuration */;

pub const HT_5BIT_CAP_MASK: u64 = 0xF8;
pub const HT_CAPTYPE_IRQ: u64 = 0x80	/* IRQ Configuration */;
pub const HT_CAPTYPE_REMAPPING_40: u64 = 0xA0	/* 40 bit address remapping */;
pub const HT_CAPTYPE_REMAPPING_64: u64 = 0xA2	/* 64 bit address remapping */;
pub const HT_CAPTYPE_UNITID_CLUMP: u64 = 0x90	/* Unit ID clumping */;
pub const HT_CAPTYPE_EXTCONF: u64 = 0x98	/* Extended Configuration Space Access */;
pub const HT_CAPTYPE_MSI_MAPPING: u64 = 0xA8	/* MSI Mapping Capability */;
pub const HT_MSI_FLAGS: u64 = 0x02		/* Offset to flags */;
pub const HT_MSI_FLAGS_ENABLE: u64 = 0x1		/* Mapping enable */;
pub const HT_MSI_FLAGS_FIXED: u64 = 0x2		/* Fixed mapping only */;
pub const HT_MSI_FIXED_ADDR: u64 = 0x00000000FEE00000	/* Fixed addr */;
pub const HT_MSI_ADDR_LO: u64 = 0x04		/* Offset to low addr bits */;
pub const HT_MSI_ADDR_LO_MASK: u64 = 0xFFF00000	/* Low address bit mask */;
pub const HT_MSI_ADDR_HI: u64 = 0x08		/* Offset to high addr bits */;
pub const HT_CAPTYPE_DIRECT_ROUTE: u64 = 0xB0	/* Direct routing configuration */;
pub const HT_CAPTYPE_VCSET: u64 = 0xB8	/* Virtual Channel configuration */;
pub const HT_CAPTYPE_ERROR_RETRY: u64 = 0xC0	/* Retry on error configuration */;
pub const HT_CAPTYPE_GEN3: u64 = 0xD0	/* Generation 3 HyperTransport configuration */;
pub const HT_CAPTYPE_PM: u64 = 0xE0	/* HyperTransport power management configuration */;
pub const HT_CAP_SIZEOF_LONG: u64 = 28	/* slave & primary */;
pub const HT_CAP_SIZEOF_SHORT: u64 = 24	/* host & secondary */;

/* Alternative Routing-ID Interpretation */
pub const PCI_ARI_CAP: u64 = 0x04	/* ARI Capability Register */;
pub const PCI_ARI_CAP_MFVC: u64 = 0x0001	/* MFVC Function Groups Capability */;
pub const PCI_ARI_CAP_ACS: u64 = 0x0002	/* ACS Function Groups Capability */;
macro_rules! PCI_ARI_CAP_NFN { ($x:ident) => { (((($x) >> 8) & 0xff) /* Next Function Number */) }; }
pub const PCI_ARI_CTRL: u64 = 0x06	/* ARI Control Register */;
pub const PCI_ARI_CTRL_MFVC: u64 = 0x0001	/* MFVC Function Groups Enable */;
pub const PCI_ARI_CTRL_ACS: u64 = 0x0002	/* ACS Function Groups Enable */;
macro_rules! PCI_ARI_CTRL_FG { ($x:ident) => { (((($x) >> 4) & 7) /* Function Group */) }; }
pub const PCI_EXT_CAP_ARI_SIZEOF: u64 = 8;

/* Address Translation Service */
pub const PCI_ATS_CAP: u64 = 0x04	/* ATS Capability Register */;
macro_rules! PCI_ATS_CAP_QDEP { ($x:ident) => { ((($x) & 0x1f)	/* Invalidate Queue Depth */) }; }
pub const PCI_ATS_MAX_QDEP: u64 = 32	/* Max Invalidate Queue Depth */;
pub const PCI_ATS_CAP_PAGE_ALIGNED: u64 = 0x0020 /* Page Aligned Request */;
pub const PCI_ATS_CTRL: u64 = 0x06	/* ATS Control Register */;
pub const PCI_ATS_CTRL_ENABLE: u64 = 0x8000	/* ATS Enable */;
macro_rules! PCI_ATS_CTRL_STU { ($x:ident) => { ((($x) & 0x1f)	/* Smallest Translation Unit */) }; }
pub const PCI_ATS_MIN_STU: u64 = 12	/* shift of minimum STU block */;
pub const PCI_EXT_CAP_ATS_SIZEOF: u64 = 8;

/* Page Request Interface */
pub const PCI_PRI_CTRL: u64 = 0x04	/* PRI control register */;
pub const PCI_PRI_CTRL_ENABLE: u64 = 0x0001	/* Enable */;
pub const PCI_PRI_CTRL_RESET: u64 = 0x0002	/* Reset */;
pub const PCI_PRI_STATUS: u64 = 0x06	/* PRI status register */;
pub const PCI_PRI_STATUS_RF: u64 = 0x0001	/* Response Failure */;
pub const PCI_PRI_STATUS_UPRGI: u64 = 0x0002	/* Unexpected PRG index */;
pub const PCI_PRI_STATUS_STOPPED: u64 = 0x0100	/* PRI Stopped */;
pub const PCI_PRI_STATUS_PASID: u64 = 0x8000	/* PRG Response PASID Required */;
pub const PCI_PRI_MAX_REQ: u64 = 0x08	/* PRI max reqs supported */;
pub const PCI_PRI_ALLOC_REQ: u64 = 0x0c	/* PRI max reqs allowed */;
pub const PCI_EXT_CAP_PRI_SIZEOF: u64 = 16;

/* Process Address Space ID */
pub const PCI_PASID_CAP: u64 = 0x04    /* PASID feature register */;
pub const PCI_PASID_CAP_EXEC: u64 = 0x0002	/* Exec permissions Supported */;
pub const PCI_PASID_CAP_PRIV: u64 = 0x0004	/* Privilege Mode Supported */;
pub const PCI_PASID_CAP_WIDTH: u64 = 0x1f00;
pub const PCI_PASID_CTRL: u64 = 0x06    /* PASID control register */;
pub const PCI_PASID_CTRL_ENABLE: u64 = 0x0001	/* Enable bit */;
pub const PCI_PASID_CTRL_EXEC: u64 = 0x0002	/* Exec permissions Enable */;
pub const PCI_PASID_CTRL_PRIV: u64 = 0x0004	/* Privilege Mode Enable */;
pub const PCI_EXT_CAP_PASID_SIZEOF: u64 = 8;

/* Single Root I/O Virtualization */
pub const PCI_SRIOV_CAP: u64 = 0x04	/* SR-IOV Capabilities */;
pub const PCI_SRIOV_CAP_VFM: u64 = 0x00000001  /* VF Migration Capable */;
macro_rules! PCI_SRIOV_CAP_INTR { ($x:ident) => { ((($x) >> 21) /* Interrupt Message Number */) }; }
pub const PCI_SRIOV_CTRL: u64 = 0x08	/* SR-IOV Control */;
pub const PCI_SRIOV_CTRL_VFE: u64 = 0x0001	/* VF Enable */;
pub const PCI_SRIOV_CTRL_VFM: u64 = 0x0002	/* VF Migration Enable */;
pub const PCI_SRIOV_CTRL_INTR: u64 = 0x0004	/* VF Migration Interrupt Enable */;
pub const PCI_SRIOV_CTRL_MSE: u64 = 0x0008	/* VF Memory Space Enable */;
pub const PCI_SRIOV_CTRL_ARI: u64 = 0x0010	/* ARI Capable Hierarchy */;
pub const PCI_SRIOV_STATUS: u64 = 0x0a	/* SR-IOV Status */;
pub const PCI_SRIOV_STATUS_VFM: u64 = 0x0001	/* VF Migration Status */;
pub const PCI_SRIOV_INITIAL_VF: u64 = 0x0c	/* Initial VFs */;
pub const PCI_SRIOV_TOTAL_VF: u64 = 0x0e	/* Total VFs */;
pub const PCI_SRIOV_NUM_VF: u64 = 0x10	/* Number of VFs */;
pub const PCI_SRIOV_FUNC_LINK: u64 = 0x12	/* Function Dependency Link */;
pub const PCI_SRIOV_VF_OFFSET: u64 = 0x14	/* First VF Offset */;
pub const PCI_SRIOV_VF_STRIDE: u64 = 0x16	/* Following VF Stride */;
pub const PCI_SRIOV_VF_DID: u64 = 0x1a	/* VF Device ID */;
pub const PCI_SRIOV_SUP_PGSIZE: u64 = 0x1c	/* Supported Page Sizes */;
pub const PCI_SRIOV_SYS_PGSIZE: u64 = 0x20	/* System Page Size */;
pub const PCI_SRIOV_BAR: u64 = 0x24	/* VF BAR0 */;
pub const PCI_SRIOV_NUM_BARS: u64 = 6	/* Number of VF BARs */;
pub const PCI_SRIOV_VFM: u64 = 0x3c	/* VF Migration State Array Offset*/;
macro_rules! PCI_SRIOV_VFM_BIR { ($x:ident) => { ((($x) & 7)	/* State BIR */) }; }
macro_rules! PCI_SRIOV_VFM_OFFSET { ($x:ident) => { ((($x) & ~7)	/* State Offset */) }; }
pub const PCI_SRIOV_VFM_UA: u64 = 0x0	/* Inactive.Unavailable */;
pub const PCI_SRIOV_VFM_MI: u64 = 0x1	/* Dormant.MigrateIn */;
pub const PCI_SRIOV_VFM_MO: u64 = 0x2	/* Active.MigrateOut */;
pub const PCI_SRIOV_VFM_AV: u64 = 0x3	/* Active.Available */;
pub const PCI_EXT_CAP_SRIOV_SIZEOF: u64 = 0x40;

pub const PCI_LTR_MAX_SNOOP_LAT: u64 = 0x4;
pub const PCI_LTR_MAX_NOSNOOP_LAT: u64 = 0x6;
pub const PCI_LTR_VALUE_MASK: u64 = 0x000003ff;
pub const PCI_LTR_SCALE_MASK: u64 = 0x00001c00;
pub const PCI_LTR_SCALE_SHIFT: u64 = 10;
pub const PCI_LTR_NOSNOOP_VALUE: u64 = 0x03ff0000 /* Max No-Snoop Latency Value */;
pub const PCI_LTR_NOSNOOP_SCALE: u64 = 0x1c000000 /* Scale for Max Value */;
pub const PCI_EXT_CAP_LTR_SIZEOF: u64 = 8;

/* Access Control Service */
pub const PCI_ACS_CAP: u64 = 0x04	/* ACS Capability Register */;
pub const PCI_ACS_SV: u64 = 0x0001	/* Source Validation */;
pub const PCI_ACS_TB: u64 = 0x0002	/* Translation Blocking */;
pub const PCI_ACS_RR: u64 = 0x0004	/* P2P Request Redirect */;
pub const PCI_ACS_CR: u64 = 0x0008	/* P2P Completion Redirect */;
pub const PCI_ACS_UF: u64 = 0x0010	/* Upstream Forwarding */;
pub const PCI_ACS_EC: u64 = 0x0020	/* P2P Egress Control */;
pub const PCI_ACS_DT: u64 = 0x0040	/* Direct Translated P2P */;
pub const PCI_ACS_EGRESS_BITS: u64 = 0x05	/* ACS Egress Control Vector Size */;
pub const PCI_ACS_CTRL: u64 = 0x06	/* ACS Control Register */;
pub const PCI_ACS_EGRESS_CTL_V: u64 = 0x08	/* ACS Egress Control Vector */;

/* SATA capability */
pub const PCI_SATA_REGS: u64 = 4	/* SATA REGs specifier */;
pub const PCI_SATA_REGS_MASK: u64 = 0xF	/* location - BAR#/inline */;
pub const PCI_SATA_REGS_INLINE: u64 = 0xF	/* REGS in config space */;
pub const PCI_SATA_SIZEOF_SHORT: u64 = 8;
pub const PCI_SATA_SIZEOF_LONG: u64 = 16;

/* Resizable BARs */
pub const PCI_REBAR_CAP: u64 = 4	/* capability register */;
pub const PCI_REBAR_CAP_SIZES: u64 = 0xFFFFFFF0  /* supported BAR sizes */;
pub const PCI_REBAR_CTRL: u64 = 8	/* control register */;
pub const PCI_REBAR_CTRL_BAR_IDX: u64 = 0x00000007  /* BAR index */;
pub const PCI_REBAR_CTRL_NBAR_MASK: u64 = 0x000000E0  /* # of resizable BARs */;
pub const PCI_REBAR_CTRL_NBAR_SHIFT: u64 = 5	    /* shift for # of BARs */;
pub const PCI_REBAR_CTRL_BAR_SIZE: u64 = 0x00001F00  /* BAR size */;
pub const PCI_REBAR_CTRL_BAR_SHIFT: u64 = 8	    /* shift for BAR size */;

/* Dynamic Power Allocation */
pub const PCI_DPA_CAP: u64 = 4	/* capability register */;
pub const PCI_DPA_CAP_SUBSTATE_MASK: u64 = 0x1F	/* # substates - 1 */;
pub const PCI_DPA_BASE_SIZEOF: u64 = 16	/* size with 0 substates */;

/* TPH Completer Support */
pub const PCI_EXP_DEVCAP2_TPH_COMP_NONE: u64 = 0x0 /* None */;
pub const PCI_EXP_DEVCAP2_TPH_COMP_TPH_ONLY: u64 = 0x1 /* TPH only */;
pub const PCI_EXP_DEVCAP2_TPH_COMP_EXT_TPH: u64 = 0x3 /* TPH and Extended TPH */;

/* TPH Requester */
pub const PCI_TPH_CAP: u64 = 4	/* capability register */;
pub const PCI_TPH_CAP_ST_NS: u64 = 0x00000001 /* No ST Mode Supported */;
pub const PCI_TPH_CAP_ST_IV: u64 = 0x00000002 /* Interrupt Vector Mode Supported */;
pub const PCI_TPH_CAP_ST_DS: u64 = 0x00000004 /* Device Specific Mode Supported */;
pub const PCI_TPH_CAP_EXT_TPH: u64 = 0x00000100 /* Ext TPH Requester Supported */;
pub const PCI_TPH_CAP_LOC_MASK: u64 = 0x00000600 /* ST Table Location */;
pub const PCI_TPH_LOC_NONE: u64 = 0x00000000 /* Not present */;
pub const PCI_TPH_LOC_CAP: u64 = 0x00000200 /* In capability */;
pub const PCI_TPH_LOC_MSIX: u64 = 0x00000400 /* In MSI-X */;
pub const PCI_TPH_CAP_ST_MASK: u64 = 0x07FF0000 /* ST Table Size */;
pub const PCI_TPH_CAP_ST_SHIFT: u64 = 16	/* ST Table Size shift */;
pub const PCI_TPH_BASE_SIZEOF: u64 = 0xc	/* Size with no ST table */;

pub const PCI_TPH_CTRL: u64 = 8	/* control register */;
pub const PCI_TPH_CTRL_MODE_SEL_MASK: u64 = 0x00000007 /* ST Mode Select */;
pub const PCI_TPH_ST_NS_MODE: u64 = 0x0 /* No ST Mode */;
pub const PCI_TPH_ST_IV_MODE: u64 = 0x1 /* Interrupt Vector Mode */;
pub const PCI_TPH_ST_DS_MODE: u64 = 0x2 /* Device Specific Mode */;
pub const PCI_TPH_CTRL_REQ_EN_MASK: u64 = 0x00000300 /* TPH Requester Enable */;
pub const PCI_TPH_REQ_DISABLE: u64 = 0x0 /* No TPH requests allowed */;
pub const PCI_TPH_REQ_TPH_ONLY: u64 = 0x1 /* TPH only requests allowed */;
pub const PCI_TPH_REQ_EXT_TPH: u64 = 0x3 /* Extended TPH requests allowed */;

/* Downstream Port Containment */
pub const PCI_EXP_DPC_CAP: u64 = 0x04	/* DPC Capability */;
pub const PCI_EXP_DPC_IRQ: u64 = 0x001F	/* Interrupt Message Number */;
pub const PCI_EXP_DPC_CAP_RP_EXT: u64 = 0x0020	/* Root Port Extensions */;
pub const PCI_EXP_DPC_CAP_POISONED_TLP: u64 = 0x0040	/* Poisoned TLP Egress Blocking Supported */;
pub const PCI_EXP_DPC_CAP_SW_TRIGGER: u64 = 0x0080	/* Software Triggering Supported */;
pub const PCI_EXP_DPC_RP_PIO_LOG_SIZE: u64 = 0x0F00	/* RP PIO Log Size [3:0] */;
pub const PCI_EXP_DPC_CAP_DL_ACTIVE: u64 = 0x1000	/* ERR_COR signal on DL_Active supported */;
pub const PCI_EXP_DPC_RP_PIO_LOG_SIZE4: u64 = 0x2000	/* RP PIO Log Size [4] */;

pub const PCI_EXP_DPC_CTL: u64 = 0x06	/* DPC control */;
pub const PCI_EXP_DPC_CTL_EN_FATAL: u64 = 0x0001	/* Enable trigger on ERR_FATAL message */;
pub const PCI_EXP_DPC_CTL_EN_NONFATAL: u64 = 0x0002	/* Enable trigger on ERR_NONFATAL message */;
pub const PCI_EXP_DPC_CTL_INT_EN: u64 = 0x0008	/* DPC Interrupt Enable */;

pub const PCI_EXP_DPC_STATUS: u64 = 0x08	/* DPC Status */;
pub const PCI_EXP_DPC_STATUS_TRIGGER: u64 = 0x0001 /* Trigger Status */;
pub const PCI_EXP_DPC_STATUS_TRIGGER_RSN: u64 = 0x0006 /* Trigger Reason */;
pub const PCI_EXP_DPC_STATUS_TRIGGER_RSN_UNCOR: u64 = 0x0000 /* Uncorrectable error */;
pub const PCI_EXP_DPC_STATUS_TRIGGER_RSN_NFE: u64 = 0x0002 /* Rcvd ERR_NONFATAL */;
pub const PCI_EXP_DPC_STATUS_TRIGGER_RSN_FE: u64 = 0x0004 /* Rcvd ERR_FATAL */;
pub const PCI_EXP_DPC_STATUS_TRIGGER_RSN_IN_EXT: u64 = 0x0006 /* Reason in Trig Reason Extension field */;
pub const PCI_EXP_DPC_STATUS_INTERRUPT: u64 = 0x0008 /* Interrupt Status */;
pub const PCI_EXP_DPC_RP_BUSY: u64 = 0x0010 /* Root Port Busy */;
pub const PCI_EXP_DPC_STATUS_TRIGGER_RSN_EXT: u64 = 0x0060 /* Trig Reason Extension */;
pub const PCI_EXP_DPC_STATUS_TRIGGER_RSN_RP_PIO: u64 = 0x0000	/* RP PIO error */;
pub const PCI_EXP_DPC_STATUS_TRIGGER_RSN_SW_TRIGGER: u64 = 0x0020	/* DPC SW Trigger bit */;
pub const PCI_EXP_DPC_RP_PIO_FEP: u64 = 0x1f00 /* RP PIO First Err Ptr */;

pub const PCI_EXP_DPC_SOURCE_ID: u64 = 0x0A	/* DPC Source Identifier */;

pub const PCI_EXP_DPC_RP_PIO_STATUS: u64 = 0x0C	/* RP PIO Status */;
pub const PCI_EXP_DPC_RP_PIO_MASK: u64 = 0x10	/* RP PIO Mask */;
pub const PCI_EXP_DPC_RP_PIO_SEVERITY: u64 = 0x14	/* RP PIO Severity */;
pub const PCI_EXP_DPC_RP_PIO_SYSERROR: u64 = 0x18	/* RP PIO SysError */;
pub const PCI_EXP_DPC_RP_PIO_EXCEPTION: u64 = 0x1C	/* RP PIO Exception */;
pub const PCI_EXP_DPC_RP_PIO_HEADER_LOG: u64 = 0x20	/* RP PIO Header Log */;
pub const PCI_EXP_DPC_RP_PIO_IMPSPEC_LOG: u64 = 0x30	/* RP PIO ImpSpec Log */;
pub const PCI_EXP_DPC_RP_PIO_TLPPREFIX_LOG: u64 = 0x34	/* RP PIO TLP Prefix Log */;

/* Precision Time Measurement */
pub const PCI_PTM_CAP: u64 = 0x04	    /* PTM Capability */;
pub const PCI_PTM_CAP_REQ: u64 = 0x00000001  /* Requester capable */;
pub const PCI_PTM_CAP_RES: u64 = 0x00000002  /* Responder capable */;
pub const PCI_PTM_CAP_ROOT: u64 = 0x00000004  /* Root capable */;
pub const PCI_PTM_GRANULARITY_MASK: u64 = 0x0000FF00  /* Clock granularity */;
pub const PCI_PTM_CTRL: u64 = 0x08	    /* PTM Control */;
pub const PCI_PTM_CTRL_ENABLE: u64 = 0x00000001  /* PTM enable */;
pub const PCI_PTM_CTRL_ROOT: u64 = 0x00000002  /* Root select */;

/* ASPM L1 PM Substates */
pub const PCI_L1SS_CAP: u64 = 0x04	/* Capabilities Register */;
pub const PCI_L1SS_CAP_PCIPM_L1_2: u64 = 0x00000001  /* PCI-PM L1.2 Supported */;
pub const PCI_L1SS_CAP_PCIPM_L1_1: u64 = 0x00000002  /* PCI-PM L1.1 Supported */;
pub const PCI_L1SS_CAP_ASPM_L1_2: u64 = 0x00000004  /* ASPM L1.2 Supported */;
pub const PCI_L1SS_CAP_ASPM_L1_1: u64 = 0x00000008  /* ASPM L1.1 Supported */;
pub const PCI_L1SS_CAP_L1_PM_SS: u64 = 0x00000010  /* L1 PM Substates Supported */;
pub const PCI_L1SS_CAP_CM_RESTORE_TIME: u64 = 0x0000ff00  /* Port Common_Mode_Restore_Time */;
pub const PCI_L1SS_CAP_P_PWR_ON_SCALE: u64 = 0x00030000  /* Port T_POWER_ON scale */;
pub const PCI_L1SS_CAP_P_PWR_ON_VALUE: u64 = 0x00f80000  /* Port T_POWER_ON value */;
pub const PCI_L1SS_CTL1: u64 = 0x08	/* Control 1 Register */;
pub const PCI_L1SS_CTL1_PCIPM_L1_2: u64 = 0x00000001  /* PCI-PM L1.2 Enable */;
pub const PCI_L1SS_CTL1_PCIPM_L1_1: u64 = 0x00000002  /* PCI-PM L1.1 Enable */;
pub const PCI_L1SS_CTL1_ASPM_L1_2: u64 = 0x00000004  /* ASPM L1.2 Enable */;
pub const PCI_L1SS_CTL1_ASPM_L1_1: u64 = 0x00000008  /* ASPM L1.1 Enable */;
pub const PCI_L1SS_CTL1_L1_2_MASK: u64 = 0x00000005;
pub const PCI_L1SS_CTL1_L1SS_MASK: u64 = 0x0000000f;
pub const PCI_L1SS_CTL1_CM_RESTORE_TIME: u64 = 0x0000ff00  /* Common_Mode_Restore_Time */;
pub const PCI_L1SS_CTL1_LTR_L12_TH_VALUE: u64 = 0x03ff0000  /* LTR_L1.2_THRESHOLD_Value */;
pub const PCI_L1SS_CTL1_LTR_L12_TH_SCALE: u64 = 0xe0000000  /* LTR_L1.2_THRESHOLD_Scale */;
pub const PCI_L1SS_CTL2: u64 = 0x0c	/* Control 2 Register */;
pub const PCI_L1SS_CTL2_T_PWR_ON_SCALE: u64 = 0x00000003  /* T_POWER_ON Scale */;
pub const PCI_L1SS_CTL2_T_PWR_ON_VALUE: u64 = 0x000000f8  /* T_POWER_ON Value */;

/* Designated Vendor-Specific (DVSEC, PCI_EXT_CAP_ID_DVSEC) */
pub const PCI_DVSEC_HEADER1: u64 = 0x4 /* Designated Vendor-Specific Header1 */;
macro_rules! PCI_DVSEC_HEADER1_VID { ($x:ident) => { ((($x) & 0xffff)) }; }
macro_rules! PCI_DVSEC_HEADER1_REV { ($x:ident) => { (((($x) >> 16) & 0xf)) }; }
macro_rules! PCI_DVSEC_HEADER1_LEN { ($x:ident) => { (((($x) >> 20) & 0xfff)) }; }
pub const PCI_DVSEC_HEADER2: u64 = 0x8 /* Designated Vendor-Specific Header2 */;
macro_rules! PCI_DVSEC_HEADER2_ID { ($x:ident) => { ((($x) & 0xffff)) }; }

/* VF Resizable BARs, same layout as PCI_REBAR */
pub const PCI_VF_REBAR_CAP: u64 = PCI_REBAR_CAP;
pub const PCI_VF_REBAR_CAP_SIZES: u64 = PCI_REBAR_CAP_SIZES;
pub const PCI_VF_REBAR_CTRL: u64 = PCI_REBAR_CTRL;
pub const PCI_VF_REBAR_CTRL_BAR_IDX: u64 = PCI_REBAR_CTRL_BAR_IDX;
pub const PCI_VF_REBAR_CTRL_NBAR_MASK: u64 = PCI_REBAR_CTRL_NBAR_MASK;
pub const PCI_VF_REBAR_CTRL_BAR_SIZE: u64 = PCI_REBAR_CTRL_BAR_SIZE;

/* Data Link Feature */
pub const PCI_DLF_CAP: u64 = 0x04	/* Capabilities Register */;
pub const PCI_DLF_EXCHANGE_ENABLE: u64 = 0x80000000  /* Data Link Feature Exchange Enable */;

/* Secondary PCIe Capability 8.0 GT/s */
pub const PCI_SECPCI_LE_CTRL: u64 = 0x0c /* Lane Equalization Control Register */;

/* Physical Layer 16.0 GT/s */
pub const PCI_PL_16GT_LE_CTRL: u64 = 0x20	/* Lane Equalization Control Register */;
pub const PCI_PL_16GT_LE_CTRL_DSP_TX_PRESET_MASK: u64 = 0x0000000F;
pub const PCI_PL_16GT_LE_CTRL_USP_TX_PRESET_MASK: u64 = 0x000000F0;
pub const PCI_PL_16GT_LE_CTRL_USP_TX_PRESET_SHIFT: u64 = 4;

/* Physical Layer 32.0 GT/s */
pub const PCI_PL_32GT_LE_CTRL: u64 = 0x20	/* Lane Equalization Control Register */;

/* Physical Layer 64.0 GT/s */
pub const PCI_PL_64GT_LE_CTRL: u64 = 0x20	/* Lane Equalization Control Register */;

/* Native PCIe Enclosure Management */
pub const PCI_NPEM_CAP: u64 = 0x04 /* NPEM capability register */;
pub const PCI_NPEM_CAP_CAPABLE: u64 = 0x00000001 /* NPEM Capable */;

pub const PCI_NPEM_CTRL: u64 = 0x08 /* NPEM control register */;
pub const PCI_NPEM_CTRL_ENABLE: u64 = 0x00000001 /* NPEM Enable */;

/*
 * Native PCIe Enclosure Management indication bits and Reset command bit
 * are corresponding for capability and control registers.
 */
pub const PCI_NPEM_CMD_RESET: u64 = 0x00000002 /* Reset Command */;
pub const PCI_NPEM_IND_OK: u64 = 0x00000004 /* OK */;
pub const PCI_NPEM_IND_LOCATE: u64 = 0x00000008 /* Locate */;
pub const PCI_NPEM_IND_FAIL: u64 = 0x00000010 /* Fail */;
pub const PCI_NPEM_IND_REBUILD: u64 = 0x00000020 /* Rebuild */;
pub const PCI_NPEM_IND_PFA: u64 = 0x00000040 /* Predicted Failure Analysis */;
pub const PCI_NPEM_IND_HOTSPARE: u64 = 0x00000080 /* Hot Spare */;
pub const PCI_NPEM_IND_ICA: u64 = 0x00000100 /* In Critical Array */;
pub const PCI_NPEM_IND_IFA: u64 = 0x00000200 /* In Failed Array */;
pub const PCI_NPEM_IND_IDT: u64 = 0x00000400 /* Device Type */;
pub const PCI_NPEM_IND_DISABLED: u64 = 0x00000800 /* Disabled */;
pub const PCI_NPEM_IND_SPEC_0: u64 = 0x01000000;
pub const PCI_NPEM_IND_SPEC_1: u64 = 0x02000000;
pub const PCI_NPEM_IND_SPEC_2: u64 = 0x04000000;
pub const PCI_NPEM_IND_SPEC_3: u64 = 0x08000000;
pub const PCI_NPEM_IND_SPEC_4: u64 = 0x10000000;
pub const PCI_NPEM_IND_SPEC_5: u64 = 0x20000000;
pub const PCI_NPEM_IND_SPEC_6: u64 = 0x40000000;
pub const PCI_NPEM_IND_SPEC_7: u64 = 0x80000000;

pub const PCI_NPEM_STATUS: u64 = 0x0c /* NPEM status register */;
pub const PCI_NPEM_STATUS_CC: u64 = 0x00000001 /* Command Completed */;

/* Data Object Exchange */
pub const PCI_DOE_CAP: u64 = 0x04    /* DOE Capabilities Register */;
pub const PCI_DOE_CAP_INT_SUP: u64 = 0x00000001  /* Interrupt Support */;
pub const PCI_DOE_CAP_INT_MSG_NUM: u64 = 0x00000ffe  /* Interrupt Message Number */;
pub const PCI_DOE_CTRL: u64 = 0x08    /* DOE Control Register */;
pub const PCI_DOE_CTRL_ABORT: u64 = 0x00000001  /* DOE Abort */;
pub const PCI_DOE_CTRL_INT_EN: u64 = 0x00000002  /* DOE Interrupt Enable */;
pub const PCI_DOE_CTRL_GO: u64 = 0x80000000  /* DOE Go */;
pub const PCI_DOE_STATUS: u64 = 0x0c    /* DOE Status Register */;
pub const PCI_DOE_STATUS_BUSY: u64 = 0x00000001  /* DOE Busy */;
pub const PCI_DOE_STATUS_INT_STATUS: u64 = 0x00000002  /* DOE Interrupt Status */;
pub const PCI_DOE_STATUS_ERROR: u64 = 0x00000004  /* DOE Error */;
pub const PCI_DOE_STATUS_DATA_OBJECT_READY: u64 = 0x80000000  /* Data Object Ready */;
pub const PCI_DOE_WRITE: u64 = 0x10    /* DOE Write Data Mailbox Register */;
pub const PCI_DOE_READ: u64 = 0x14    /* DOE Read Data Mailbox Register */;
pub const PCI_DOE_CAP_SIZEOF: u64 = 0x18	/* Size of DOE register block */;

/* DOE Data Object - note not actually registers */
pub const PCI_DOE_DATA_OBJECT_HEADER_1_VID: u64 = 0x0000ffff;
pub const PCI_DOE_DATA_OBJECT_HEADER_1_TYPE: u64 = 0x00ff0000;
pub const PCI_DOE_DATA_OBJECT_HEADER_2_LENGTH: u64 = 0x0003ffff;

pub const PCI_DOE_DATA_OBJECT_DISC_REQ_3_INDEX: u64 = 0x000000ff;
pub const PCI_DOE_DATA_OBJECT_DISC_REQ_3_VER: u64 = 0x0000ff00;
pub const PCI_DOE_DATA_OBJECT_DISC_RSP_3_VID: u64 = 0x0000ffff;
pub const PCI_DOE_DATA_OBJECT_DISC_RSP_3_TYPE: u64 = 0x00ff0000;
pub const PCI_DOE_DATA_OBJECT_DISC_RSP_3_NEXT_INDEX: u64 = 0xff000000;

/* Deprecated old name, replaced with PCI_DOE_DATA_OBJECT_DISC_RSP_3_TYPE */
pub const PCI_DOE_DATA_OBJECT_DISC_RSP_3_PROTOCOL: u64 = PCI_DOE_DATA_OBJECT_DISC_RSP_3_TYPE;

/* Device 3 Extended Capability */
pub const PCI_DEV3_CAP: u64 = 0x04	/* Device 3 Capabilities Register */;
pub const PCI_DEV3_CTL: u64 = 0x08	/* Device 3 Control Register */;
pub const PCI_DEV3_STA: u64 = 0x0c	/* Device 3 Status Register */;
pub const PCI_DEV3_STA_SEGMENT: u64 = 0x8	/* Segment Captured (end-to-end flit-mode detected) */;

/* Integrity and Data Encryption Extended Capability */
pub const PCI_IDE_CAP: u64 = 0x04;
pub const PCI_IDE_CAP_LINK: u64 = 0x1  /* Link IDE Stream Supported */;
pub const PCI_IDE_CAP_SELECTIVE: u64 = 0x2  /* Selective IDE Streams Supported */;
pub const PCI_IDE_CAP_FLOWTHROUGH: u64 = 0x4  /* Flow-Through IDE Stream Supported */;
pub const PCI_IDE_CAP_PARTIAL_HEADER_ENC: u64 = 0x8  /* Partial Header Encryption Supported */;
pub const PCI_IDE_CAP_AGGREGATION: u64 = 0x10 /* Aggregation Supported */;
pub const PCI_IDE_CAP_PCRC: u64 = 0x20 /* PCRC Supported */;
pub const PCI_IDE_CAP_IDE_KM: u64 = 0x40 /* IDE_KM Protocol Supported */;
pub const PCI_IDE_CAP_SEL_CFG: u64 = 0x80 /* Selective IDE for Config Request Support */;
pub const PCI_IDE_CAP_ALG: u64 = __GENMASK(12, 8) /* Supported Algorithms */;
pub const PCI_IDE_CAP_ALG_AES_GCM_256: u64 = 0    /* AES-GCM 256 key size, 96b MAC */;
pub const PCI_IDE_CAP_LINK_TC_NUM: u64 = __GENMASK(15, 13) /* Link IDE TCs */;
pub const PCI_IDE_CAP_SEL_NUM: u64 = __GENMASK(23, 16) /* Supported Selective IDE Streams */;
pub const PCI_IDE_CAP_TEE_LIMITED: u64 = 0x1000000 /* TEE-Limited Stream Supported */;
pub const PCI_IDE_CTL: u64 = 0x08;
pub const PCI_IDE_CTL_FLOWTHROUGH_IDE: u64 = 0x4  /* Flow-Through IDE Stream Enabled */;

pub const PCI_IDE_LINK_STREAM_0: u64 = 0xc  /* First Link Stream Register Block */;
pub const PCI_IDE_LINK_BLOCK_SIZE: u64 = 8;
/* Link IDE Stream block, up to PCI_IDE_CAP_LINK_TC_NUM */
pub const PCI_IDE_LINK_CTL_0: u64 = 0x00		  /* First Link Control Register Offset in block */;
pub const PCI_IDE_LINK_CTL_EN: u64 = 0x1		  /* Link IDE Stream Enable */;
pub const PCI_IDE_LINK_CTL_TX_AGGR_NPR: u64 = __GENMASK(3, 2)	  /* Tx Aggregation Mode NPR */;
pub const PCI_IDE_LINK_CTL_TX_AGGR_PR: u64 = __GENMASK(5, 4)	  /* Tx Aggregation Mode PR */;
pub const PCI_IDE_LINK_CTL_TX_AGGR_CPL: u64 = __GENMASK(7, 6)	  /* Tx Aggregation Mode CPL */;
pub const PCI_IDE_LINK_CTL_PCRC_EN: u64 = 0x100		  /* PCRC Enable */;
pub const PCI_IDE_LINK_CTL_PART_ENC: u64 = __GENMASK(13, 10) /* Partial Header Encryption Mode */;
pub const PCI_IDE_LINK_CTL_ALG: u64 = __GENMASK(18, 14) /* Selection from PCI_IDE_CAP_ALG */;
pub const PCI_IDE_LINK_CTL_TC: u64 = __GENMASK(21, 19) /* Traffic Class */;
pub const PCI_IDE_LINK_CTL_ID: u64 = __GENMASK(31, 24) /* Stream ID */;
pub const PCI_IDE_LINK_STS_0: u64 = 0x4               /* First Link Status Register Offset in block */;
pub const PCI_IDE_LINK_STS_STATE: u64 = __GENMASK(3, 0)   /* Link IDE Stream State */;
pub const PCI_IDE_LINK_STS_IDE_FAIL: u64 = 0x80000000	  /* IDE fail message received */;

/* Selective IDE Stream block, up to PCI_IDE_CAP_SELECTIVE_STREAMS_NUM */
/* Selective IDE Stream Capability Register */
pub const PCI_IDE_SEL_CAP: u64 = 0x00;
pub const PCI_IDE_SEL_CAP_ASSOC_NUM: u64 = __GENMASK(3, 0);
/* Selective IDE Stream Control Register */
pub const PCI_IDE_SEL_CTL: u64 = 0x04;
pub const PCI_IDE_SEL_CTL_EN: u64 = 0x1		  /* Selective IDE Stream Enable */;
pub const PCI_IDE_SEL_CTL_TX_AGGR_NPR: u64 = __GENMASK(3, 2)	  /* Tx Aggregation Mode NPR */;
pub const PCI_IDE_SEL_CTL_TX_AGGR_PR: u64 = __GENMASK(5, 4)   /* Tx Aggregation Mode PR */;
pub const PCI_IDE_SEL_CTL_TX_AGGR_CPL: u64 = __GENMASK(7, 6)	  /* Tx Aggregation Mode CPL */;
pub const PCI_IDE_SEL_CTL_PCRC_EN: u64 = 0x100		  /* PCRC Enable */;
pub const PCI_IDE_SEL_CTL_CFG_EN: u64 = 0x200		  /* Selective IDE for Configuration Requests */;
pub const PCI_IDE_SEL_CTL_PART_ENC: u64 = __GENMASK(13, 10) /* Partial Header Encryption Mode */;
pub const PCI_IDE_SEL_CTL_ALG: u64 = __GENMASK(18, 14) /* Selection from PCI_IDE_CAP_ALG */;
pub const PCI_IDE_SEL_CTL_TC: u64 = __GENMASK(21, 19) /* Traffic Class */;
pub const PCI_IDE_SEL_CTL_DEFAULT: u64 = 0x400000	  /* Default Stream */;
pub const PCI_IDE_SEL_CTL_TEE_LIMITED: u64 = 0x800000	  /* TEE-Limited Stream */;
pub const PCI_IDE_SEL_CTL_ID: u64 = __GENMASK(31, 24) /* Stream ID */;
pub const PCI_IDE_SEL_CTL_ID_MAX: u64 = 255;
/* Selective IDE Stream Status Register */
pub const PCI_IDE_SEL_STS: u64 = 0x08;
pub const PCI_IDE_SEL_STS_STATE: u64 = __GENMASK(3, 0) /* Selective IDE Stream State */;
pub const PCI_IDE_SEL_STS_STATE_INSECURE: u64 = 0;
pub const PCI_IDE_SEL_STS_STATE_SECURE: u64 = 2;
pub const PCI_IDE_SEL_STS_IDE_FAIL: u64 = 0x80000000	 /* IDE fail message received */;
/* IDE RID Association Register 1 */
pub const PCI_IDE_SEL_RID_1: u64 = 0x0c;
pub const PCI_IDE_SEL_RID_1_LIMIT: u64 = __GENMASK(23, 8);
/* IDE RID Association Register 2 */
pub const PCI_IDE_SEL_RID_2: u64 = 0x10;
pub const PCI_IDE_SEL_RID_2_VALID: u64 = 0x1;
pub const PCI_IDE_SEL_RID_2_BASE: u64 = __GENMASK(23, 8);
pub const PCI_IDE_SEL_RID_2_SEG: u64 = __GENMASK(31, 24);
/* Selective IDE Address Association Register Block, up to PCI_IDE_SEL_CAP_ASSOC_NUM */
pub const PCI_IDE_SEL_ADDR_BLOCK_SIZE: u64 = 12;
macro_rules! PCI_IDE_SEL_ADDR_1 { ($x:ident) => { ((20 + ($x) * PCI_IDE_SEL_ADDR_BLOCK_SIZE)) }; }
pub const PCI_IDE_SEL_ADDR_1_VALID: u64 = 0x1;
pub const PCI_IDE_SEL_ADDR_1_BASE_LOW: u64 = __GENMASK(19, 8);
pub const PCI_IDE_SEL_ADDR_1_LIMIT_LOW: u64 = __GENMASK(31, 20);
/* IDE Address Association Register 2 is "Memory Limit Upper" */
macro_rules! PCI_IDE_SEL_ADDR_2 { ($x:ident) => { ((24 + ($x) * PCI_IDE_SEL_ADDR_BLOCK_SIZE)) }; }
/* IDE Address Association Register 3 is "Memory Base Upper" */
macro_rules! PCI_IDE_SEL_ADDR_3 { ($x:ident) => { ((28 + ($x) * PCI_IDE_SEL_ADDR_BLOCK_SIZE)) }; }
macro_rules! PCI_IDE_SEL_BLOCK_SIZE { ($nr_assoc:ident) => { ((20 + PCI_IDE_SEL_ADDR_BLOCK_SIZE * ($nr_assoc))) }; }

/*
 * Compute Express Link (CXL r4.0, sec 8.1)
 *
 * Note that CXL DVSEC id 3 and 7 to be ignored when the CXL link state
 * is "disconnected" (CXL r4.0, sec 9.12.3). Re-enumerate these
 * registers on downstream link-up events.
 */

/* CXL r4.0, 8.1.3: PCIe DVSEC for CXL Device */
pub const PCI_DVSEC_CXL_DEVICE: u64 = 0;
pub const PCI_DVSEC_CXL_CAP: u64 = 0xA;
pub const PCI_DVSEC_CXL_CACHE_CAPABLE: u64 = _BITUL(0);
pub const PCI_DVSEC_CXL_MEM_CAPABLE: u64 = _BITUL(2);
pub const PCI_DVSEC_CXL_HDM_COUNT: u64 = __GENMASK(5, 4);
pub const PCI_DVSEC_CXL_CTRL: u64 = 0xC;
pub const PCI_DVSEC_CXL_MEM_ENABLE: u64 = _BITUL(2);
macro_rules! PCI_DVSEC_CXL_RANGE_SIZE_HIGH { ($i:ident) => { ((0x18 + ($i * 0x10))) }; }
macro_rules! PCI_DVSEC_CXL_RANGE_SIZE_LOW { ($i:ident) => { ((0x1C + ($i * 0x10))) }; }
pub const PCI_DVSEC_CXL_MEM_INFO_VALID: u64 = _BITUL(0);
pub const PCI_DVSEC_CXL_MEM_ACTIVE: u64 = _BITUL(1);
pub const PCI_DVSEC_CXL_MEM_ACTIVE_TIMEOUT: u64 = __GENMASK(15, 13);
pub const PCI_DVSEC_CXL_MEM_SIZE_LOW: u64 = __GENMASK(31, 28);
macro_rules! PCI_DVSEC_CXL_RANGE_BASE_HIGH { ($i:ident) => { ((0x20 + ($i * 0x10))) }; }
macro_rules! PCI_DVSEC_CXL_RANGE_BASE_LOW { ($i:ident) => { ((0x24 + ($i * 0x10))) }; }
pub const PCI_DVSEC_CXL_MEM_BASE_LOW: u64 = __GENMASK(31, 28);

pub const CXL_DVSEC_RANGE_MAX: u64 = 2;

/* CXL r4.0, 8.1.4: Non-CXL Function Map DVSEC */
pub const PCI_DVSEC_CXL_FUNCTION_MAP: u64 = 2;

/* CXL r4.0, 8.1.5: Extensions DVSEC for Ports */
pub const PCI_DVSEC_CXL_PORT: u64 = 3;
pub const PCI_DVSEC_CXL_PORT_CTL: u64 = 0x0c;
pub const PCI_DVSEC_CXL_PORT_CTL_UNMASK_SBR: u64 = 0x00000001;

/* CXL r4.0, 8.1.6: GPF DVSEC for CXL Port */
pub const PCI_DVSEC_CXL_PORT_GPF: u64 = 4;
pub const PCI_DVSEC_CXL_PORT_GPF_PHASE_1_CONTROL: u64 = 0x0C;
pub const PCI_DVSEC_CXL_PORT_GPF_PHASE_1_TMO_BASE: u64 = __GENMASK(3, 0);
pub const PCI_DVSEC_CXL_PORT_GPF_PHASE_1_TMO_SCALE: u64 = __GENMASK(11, 8);
pub const PCI_DVSEC_CXL_PORT_GPF_PHASE_2_CONTROL: u64 = 0xE;
pub const PCI_DVSEC_CXL_PORT_GPF_PHASE_2_TMO_BASE: u64 = __GENMASK(3, 0);
pub const PCI_DVSEC_CXL_PORT_GPF_PHASE_2_TMO_SCALE: u64 = __GENMASK(11, 8);

/* CXL r4.0, 8.1.7: GPF DVSEC for CXL Device */
pub const PCI_DVSEC_CXL_DEVICE_GPF: u64 = 5;

/* CXL r4.0, 8.1.8: Flex Bus DVSEC */
pub const PCI_DVSEC_CXL_FLEXBUS_PORT: u64 = 7;
pub const PCI_DVSEC_CXL_FLEXBUS_PORT_STATUS: u64 = 0xE;
pub const PCI_DVSEC_CXL_FLEXBUS_PORT_STATUS_CACHE: u64 = _BITUL(0);
pub const PCI_DVSEC_CXL_FLEXBUS_PORT_STATUS_MEM: u64 = _BITUL(2);

/* CXL r4.0, 8.1.9: Register Locator DVSEC */
pub const PCI_DVSEC_CXL_REG_LOCATOR: u64 = 8;
pub const PCI_DVSEC_CXL_REG_LOCATOR_BLOCK1: u64 = 0xC;
pub const PCI_DVSEC_CXL_REG_LOCATOR_BIR: u64 = __GENMASK(2, 0);
pub const PCI_DVSEC_CXL_REG_LOCATOR_BLOCK_ID: u64 = __GENMASK(15, 8);
pub const PCI_DVSEC_CXL_REG_LOCATOR_BLOCK_OFF_LOW: u64 = __GENMASK(31, 16);




// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
