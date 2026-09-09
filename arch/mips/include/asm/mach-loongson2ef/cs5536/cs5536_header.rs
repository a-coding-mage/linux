/* SPDX-License-Identifier: GPL-2.0 */
/*
 * The header file of cs5536 south bridge.
 *
 * Copyright (C) 2007 Lemote, Inc.
 * Author : jlliu <liujl@lemote.com>
 */

// C header guard: _CS5536_H
// Dependency: linux/types.h

unsafe extern "C" {
    pub fn _rdmsr(msr: u32, hi: *mut u32, lo: *mut u32);
    pub fn _wrmsr(msr: u32, hi: u32, lo: u32);
}

/* MSR module base */
pub const CS5536_SB_MSR_BASE: u32 = 0x00000000;
pub const CS5536_GLIU_MSR_BASE: u32 = 0x10000000;
pub const CS5536_ILLEGAL_MSR_BASE: u32 = 0x20000000;
pub const CS5536_USB_MSR_BASE: u32 = 0x40000000;
pub const CS5536_IDE_MSR_BASE: u32 = 0x60000000;
pub const CS5536_DIVIL_MSR_BASE: u32 = 0x80000000;
pub const CS5536_ACC_MSR_BASE: u32 = 0xa0000000;
pub const CS5536_UNUSED_MSR_BASE: u32 = 0xc0000000;
pub const CS5536_GLCP_MSR_BASE: u32 = 0xe0000000;

macro_rules! SB_MSR_REG { ($offset:expr) => { CS5536_SB_MSR_BASE | ($offset) }; }
macro_rules! GLIU_MSR_REG { ($offset:expr) => { CS5536_GLIU_MSR_BASE | ($offset) }; }
macro_rules! ILLEGAL_MSR_REG { ($offset:expr) => { CS5536_ILLEGAL_MSR_BASE | ($offset) }; }
macro_rules! USB_MSR_REG { ($offset:expr) => { CS5536_USB_MSR_BASE | ($offset) }; }
macro_rules! IDE_MSR_REG { ($offset:expr) => { CS5536_IDE_MSR_BASE | ($offset) }; }
macro_rules! DIVIL_MSR_REG { ($offset:expr) => { CS5536_DIVIL_MSR_BASE | ($offset) }; }
macro_rules! ACC_MSR_REG { ($offset:expr) => { CS5536_ACC_MSR_BASE | ($offset) }; }
macro_rules! UNUSED_MSR_REG { ($offset:expr) => { CS5536_UNUSED_MSR_BASE | ($offset) }; }
macro_rules! GLCP_MSR_REG { ($offset:expr) => { CS5536_GLCP_MSR_BASE | ($offset) }; }

/* BAR SPACE OF VIRTUAL PCI: range for pci probe use, length is the actual size. */
pub const CS5536_IRQ_RANGE: u32 = 0xffffffe0;
pub const CS5536_IRQ_LENGTH: u32 = 0x20;
pub const CS5536_SMB_RANGE: u32 = 0xfffffff8;
pub const CS5536_SMB_LENGTH: u32 = 0x08;
pub const CS5536_GPIO_RANGE: u32 = 0xffffff00;
pub const CS5536_GPIO_LENGTH: u32 = 0x100;
pub const CS5536_MFGPT_RANGE: u32 = 0xffffffc0;
pub const CS5536_MFGPT_LENGTH: u32 = 0x40;
pub const CS5536_ACPI_RANGE: u32 = 0xffffffe0;
pub const CS5536_ACPI_LENGTH: u32 = 0x20;
pub const CS5536_PMS_RANGE: u32 = 0xffffff80;
pub const CS5536_PMS_LENGTH: u32 = 0x80;
pub const CS5536_IDE_RANGE: u32 = 0xfffffff0;
pub const CS5536_IDE_LENGTH: u32 = 0x10;
pub const CS5536_ACC_RANGE: u32 = 0xffffff80;
pub const CS5536_ACC_LENGTH: u32 = 0x80;
pub const CS5536_OHCI_RANGE: u32 = 0xfffff000;
pub const CS5536_OHCI_LENGTH: u32 = 0x1000;
pub const CS5536_EHCI_RANGE: u32 = 0xfffff000;
pub const CS5536_EHCI_LENGTH: u32 = 0x1000;

/* PCI MSR ACCESS */
pub const PCI_MSR_CTRL: u32 = 0xF0;
pub const PCI_MSR_ADDR: u32 = 0xF4;
pub const PCI_MSR_DATA_LO: u32 = 0xF8;
pub const PCI_MSR_DATA_HI: u32 = 0xFC;

/* GLIU STANDARD MSR */
pub const GLIU_CAP: u32 = 0x00;
pub const GLIU_CONFIG: u32 = 0x01;
pub const GLIU_SMI: u32 = 0x02;
pub const GLIU_ERROR: u32 = 0x03;
pub const GLIU_PM: u32 = 0x04;
pub const GLIU_DIAG: u32 = 0x05;

/* GLIU SPEC. MSR */
pub const GLIU_P2D_BM0: u32 = 0x20;
pub const GLIU_P2D_BM1: u32 = 0x21;
pub const GLIU_P2D_BM2: u32 = 0x22;
pub const GLIU_P2D_BMK0: u32 = 0x23;
pub const GLIU_P2D_BMK1: u32 = 0x24;
pub const GLIU_P2D_BM3: u32 = 0x25;
pub const GLIU_P2D_BM4: u32 = 0x26;
pub const GLIU_COH: u32 = 0x80;
pub const GLIU_PAE: u32 = 0x81;
pub const GLIU_ARB: u32 = 0x82;
pub const GLIU_ASMI: u32 = 0x83;
pub const GLIU_AERR: u32 = 0x84;
pub const GLIU_DEBUG: u32 = 0x85;
pub const GLIU_PHY_CAP: u32 = 0x86;
pub const GLIU_NOUT_RESP: u32 = 0x87;
pub const GLIU_NOUT_WDATA: u32 = 0x88;
pub const GLIU_WHOAMI: u32 = 0x8B;
pub const GLIU_SLV_DIS: u32 = 0x8C;
pub const GLIU_IOD_BM0: u32 = 0xE0;
pub const GLIU_IOD_BM1: u32 = 0xE1;
pub const GLIU_IOD_BM2: u32 = 0xE2;
pub const GLIU_IOD_BM3: u32 = 0xE3;
pub const GLIU_IOD_BM4: u32 = 0xE4;
pub const GLIU_IOD_BM5: u32 = 0xE5;
pub const GLIU_IOD_BM6: u32 = 0xE6;
pub const GLIU_IOD_BM7: u32 = 0xE7;
pub const GLIU_IOD_BM8: u32 = 0xE8;
pub const GLIU_IOD_BM9: u32 = 0xE9;
pub const GLIU_IOD_SC0: u32 = 0xEA;
pub const GLIU_IOD_SC1: u32 = 0xEB;
pub const GLIU_IOD_SC2: u32 = 0xEC;
pub const GLIU_IOD_SC3: u32 = 0xED;
pub const GLIU_IOD_SC4: u32 = 0xEE;
pub const GLIU_IOD_SC5: u32 = 0xEF;
pub const GLIU_IOD_SC6: u32 = 0xF0;
pub const GLIU_IOD_SC7: u32 = 0xF1;

/* SB STANDARD */
pub const SB_CAP: u32 = 0x00;
pub const SB_CONFIG: u32 = 0x01;
pub const SB_SMI: u32 = 0x02;
pub const SB_ERROR: u32 = 0x03;
pub const SB_MAR_ERR_EN: u32 = 0x00000001;
pub const SB_TAR_ERR_EN: u32 = 0x00000002;
pub const SB_RSVD_BIT1: u32 = 0x00000004;
pub const SB_EXCEP_ERR_EN: u32 = 0x00000008;
pub const SB_SYSE_ERR_EN: u32 = 0x00000010;
pub const SB_PARE_ERR_EN: u32 = 0x00000020;
pub const SB_TAS_ERR_EN: u32 = 0x00000040;
pub const SB_MAR_ERR_FLAG: u32 = 0x00010000;
pub const SB_TAR_ERR_FLAG: u32 = 0x00020000;
pub const SB_RSVD_BIT2: u32 = 0x00040000;
pub const SB_EXCEP_ERR_FLAG: u32 = 0x00080000;
pub const SB_SYSE_ERR_FLAG: u32 = 0x00100000;
pub const SB_PARE_ERR_FLAG: u32 = 0x00200000;
pub const SB_TAS_ERR_FLAG: u32 = 0x00400000;
pub const SB_PM: u32 = 0x04;
pub const SB_DIAG: u32 = 0x05;

/* SB SPEC. */
pub const SB_CTRL: u32 = 0x10;
pub const SB_R0: u32 = 0x20;
pub const SB_R1: u32 = 0x21;
pub const SB_R2: u32 = 0x22;
pub const SB_R3: u32 = 0x23;
pub const SB_R4: u32 = 0x24;
pub const SB_R5: u32 = 0x25;
pub const SB_R6: u32 = 0x26;
pub const SB_R7: u32 = 0x27;
pub const SB_R8: u32 = 0x28;
pub const SB_R9: u32 = 0x29;
pub const SB_R10: u32 = 0x2A;
pub const SB_R11: u32 = 0x2B;
pub const SB_R12: u32 = 0x2C;
pub const SB_R13: u32 = 0x2D;
pub const SB_R14: u32 = 0x2E;
pub const SB_R15: u32 = 0x2F;

/* GLCP STANDARD */
pub const GLCP_CAP: u32 = 0x00;
pub const GLCP_CONFIG: u32 = 0x01;
pub const GLCP_SMI: u32 = 0x02;
pub const GLCP_ERROR: u32 = 0x03;
pub const GLCP_PM: u32 = 0x04;
pub const GLCP_DIAG: u32 = 0x05;

/* GLCP SPEC. */
pub const GLCP_CLK_DIS_DELAY: u32 = 0x08;
pub const GLCP_PM_CLK_DISABLE: u32 = 0x09;
pub const GLCP_GLB_PM: u32 = 0x0B;
pub const GLCP_DBG_OUT: u32 = 0x0C;
pub const GLCP_RSVD1: u32 = 0x0D;
pub const GLCP_SOFT_COM: u32 = 0x0E;
pub const SOFT_BAR_SMB_FLAG: u32 = 0x00000001;
pub const SOFT_BAR_GPIO_FLAG: u32 = 0x00000002;
pub const SOFT_BAR_MFGPT_FLAG: u32 = 0x00000004;
pub const SOFT_BAR_IRQ_FLAG: u32 = 0x00000008;
pub const SOFT_BAR_PMS_FLAG: u32 = 0x00000010;
pub const SOFT_BAR_ACPI_FLAG: u32 = 0x00000020;
pub const SOFT_BAR_IDE_FLAG: u32 = 0x00000400;
pub const SOFT_BAR_ACC_FLAG: u32 = 0x00000800;
pub const SOFT_BAR_OHCI_FLAG: u32 = 0x00001000;
pub const SOFT_BAR_EHCI_FLAG: u32 = 0x00002000;
pub const GLCP_RSVD2: u32 = 0x0F;
pub const GLCP_CLK_OFF: u32 = 0x10;
pub const GLCP_CLK_ACTIVE: u32 = 0x11;
pub const GLCP_CLK_DISABLE: u32 = 0x12;
pub const GLCP_CLK4ACK: u32 = 0x13;
pub const GLCP_SYS_RST: u32 = 0x14;
pub const GLCP_RSVD3: u32 = 0x15;
pub const GLCP_DBG_CLK_CTRL: u32 = 0x16;
pub const GLCP_CHIP_REV_ID: u32 = 0x17;

/* PIC */
pub const PIC_YSEL_LOW: u32 = 0x20;
pub const PIC_YSEL_LOW_USB_SHIFT: u32 = 8;
pub const PIC_YSEL_LOW_ACC_SHIFT: u32 = 16;
pub const PIC_YSEL_LOW_FLASH_SHIFT: u32 = 24;
pub const PIC_YSEL_HIGH: u32 = 0x21;
pub const PIC_ZSEL_LOW: u32 = 0x22;
pub const PIC_ZSEL_HIGH: u32 = 0x23;
pub const PIC_IRQM_PRIM: u32 = 0x24;
pub const PIC_IRQM_LPC: u32 = 0x25;
pub const PIC_XIRR_STS_LOW: u32 = 0x26;
pub const PIC_XIRR_STS_HIGH: u32 = 0x27;
pub const PCI_SHDW: u32 = 0x34;

/* DIVIL STANDARD */
pub const DIVIL_CAP: u32 = 0x00;
pub const DIVIL_CONFIG: u32 = 0x01;
pub const DIVIL_SMI: u32 = 0x02;
pub const DIVIL_ERROR: u32 = 0x03;
pub const DIVIL_PM: u32 = 0x04;
pub const DIVIL_DIAG: u32 = 0x05;

/* DIVIL SPEC. */
pub const DIVIL_LBAR_IRQ: u32 = 0x08;
pub const DIVIL_LBAR_KEL: u32 = 0x09;
pub const DIVIL_LBAR_SMB: u32 = 0x0B;
pub const DIVIL_LBAR_GPIO: u32 = 0x0C;
pub const DIVIL_LBAR_MFGPT: u32 = 0x0D;
pub const DIVIL_LBAR_ACPI: u32 = 0x0E;
pub const DIVIL_LBAR_PMS: u32 = 0x0F;
pub const DIVIL_LEG_IO: u32 = 0x14;
pub const DIVIL_BALL_OPTS: u32 = 0x15;
pub const DIVIL_SOFT_IRQ: u32 = 0x16;
pub const DIVIL_SOFT_RESET: u32 = 0x17;

/* MFGPT */
pub const MFGPT_IRQ: u32 = 0x28;

/* IDE STANDARD */
pub const IDE_CAP: u32 = 0x00;
pub const IDE_CONFIG: u32 = 0x01;
pub const IDE_SMI: u32 = 0x02;
pub const IDE_ERROR: u32 = 0x03;
pub const IDE_PM: u32 = 0x04;
pub const IDE_DIAG: u32 = 0x05;

/* IDE SPEC. */
pub const IDE_IO_BAR: u32 = 0x08;
pub const IDE_CFG: u32 = 0x10;
pub const IDE_DTC: u32 = 0x12;
pub const IDE_CAST: u32 = 0x13;
pub const IDE_ETC: u32 = 0x14;
pub const IDE_INTERNAL_PM: u32 = 0x15;

/* ACC STANDARD */
pub const ACC_CAP: u32 = 0x00;
pub const ACC_CONFIG: u32 = 0x01;
pub const ACC_SMI: u32 = 0x02;
pub const ACC_ERROR: u32 = 0x03;
pub const ACC_PM: u32 = 0x04;
pub const ACC_DIAG: u32 = 0x05;

/* USB STANDARD */
pub const USB_CAP: u32 = 0x00;
pub const USB_CONFIG: u32 = 0x01;
pub const USB_SMI: u32 = 0x02;
pub const USB_ERROR: u32 = 0x03;
pub const USB_PM: u32 = 0x04;
pub const USB_DIAG: u32 = 0x05;

/* USB SPEC. */
pub const USB_OHCI: u32 = 0x08;
pub const USB_EHCI: u32 = 0x09;

/****************** NATIVE ***************************/
/* GPIO : I/O SPACE; REG : 32BITS */
pub const GPIOL_OUT_VAL: u32 = 0x00;
pub const GPIOL_OUT_EN: u32 = 0x04;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
