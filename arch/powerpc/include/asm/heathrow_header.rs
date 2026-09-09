/* SPDX-License-Identifier: GPL-2.0 */
/*
 * heathrow.h: definitions for using the "Heathrow" I/O controller chip.
 *
 * Grabbed from Open Firmware definitions on a PowerBook G3 Series
 *
 * Copyright (C) 1997 Paul Mackerras.
 *
 * Original declarations were guarded by __KERNEL__.
 */

/* Front light color on Yikes/B&W G3. 32 bits */
pub const HEATHROW_FRONT_LIGHT: u32 = 0x32; /* (set to 0 or 0xffffffff) */

/* Brightness/contrast (gossamer iMac ?). 8 bits */
pub const HEATHROW_BRIGHTNESS_CNTL: u32 = 0x32;
pub const HEATHROW_CONTRAST_CNTL: u32 = 0x33;

/* offset from ohare base for feature control register */
pub const HEATHROW_MBCR: u32 = 0x34; /* Media bay control */
pub const HEATHROW_FCR: u32 = 0x38; /* Feature control */
pub const HEATHROW_AUX_CNTL_REG: u32 = 0x3c; /* Aux control */

/*
 * Bits in feature control register.
 * Bits postfixed with a _N are in inverse logic
 */
pub const HRW_SCC_TRANS_EN_N: u32 = 0x00000001; /* Also controls modem power */
pub const HRW_BAY_POWER_N: u32 = 0x00000002;
pub const HRW_BAY_PCI_ENABLE: u32 = 0x00000004;
pub const HRW_BAY_IDE_ENABLE: u32 = 0x00000008;
pub const HRW_BAY_FLOPPY_ENABLE: u32 = 0x00000010;
pub const HRW_IDE0_ENABLE: u32 = 0x00000020;
pub const HRW_IDE0_RESET_N: u32 = 0x00000040;
pub const HRW_BAY_DEV_MASK: u32 = 0x0000001c;
pub const HRW_BAY_RESET_N: u32 = 0x00000080;
pub const HRW_IOBUS_ENABLE: u32 = 0x00000100; /* Internal IDE ? */
pub const HRW_SCC_ENABLE: u32 = 0x00000200;
pub const HRW_MESH_ENABLE: u32 = 0x00000400;
pub const HRW_SWIM_ENABLE: u32 = 0x00000800;
pub const HRW_SOUND_POWER_N: u32 = 0x00001000;
pub const HRW_SOUND_CLK_ENABLE: u32 = 0x00002000;
pub const HRW_SCCA_IO: u32 = 0x00004000;
pub const HRW_SCCB_IO: u32 = 0x00008000;
pub const HRW_PORT_OR_DESK_VIA_N: u32 = 0x00010000; /* This one is 0 on PowerBook */
pub const HRW_PWM_MON_ID_N: u32 = 0x00020000; /* ??? (0) */
pub const HRW_HOOK_MB_CNT_N: u32 = 0x00040000; /* ??? (0) */
pub const HRW_SWIM_CLONE_FLOPPY: u32 = 0x00080000; /* ??? (0) */
pub const HRW_AUD_RUN22: u32 = 0x00100000; /* ??? (1) */
pub const HRW_SCSI_LINK_MODE: u32 = 0x00200000; /* Read ??? (1) */
pub const HRW_ARB_BYPASS: u32 = 0x00400000; /* Disable internal PCI arbitrer */
pub const HRW_IDE1_RESET_N: u32 = 0x00800000; /* Media bay */
pub const HRW_SLOW_SCC_PCLK: u32 = 0x01000000; /* ??? (0) */
pub const HRW_RESET_SCC: u32 = 0x02000000;
pub const HRW_MFDC_CELL_ENABLE: u32 = 0x04000000; /* ??? (0) */
pub const HRW_USE_MFDC: u32 = 0x08000000; /* ??? (0) */
pub const HRW_BMAC_IO_ENABLE: u32 = 0x60000000; /* two bits, not documented in OF */
pub const HRW_BMAC_RESET: u32 = 0x80000000; /* not documented in OF */

/* We OR those features at boot on desktop G3s */
pub const HRW_DEFAULTS: u32 = HRW_SCCA_IO | HRW_SCCB_IO | HRW_SCC_ENABLE;

/* Looks like Heathrow has some sort of GPIOs as well... */
pub const HRW_GPIO_MODEM_RESET: u32 = 0x6d;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
