/* SPDX-License-Identifier: GPL-2.0 */
/*
 * fsl_ssi.h - ALSA SSI interface for the Freescale MPC8610 and i.MX SoC
 *
 * Author: Timur Tabi <timur@freescale.com>
 *
 * Copyright 2007-2008 Freescale Semiconductor, Inc.
 */

/* -- SSI Register Map -- */

/* SSI Transmit Data Register 0 */
pub const REG_SSI_STX0: u32 = 0x00;
/* SSI Transmit Data Register 1 */
pub const REG_SSI_STX1: u32 = 0x04;
/* SSI Receive Data Register 0 */
pub const REG_SSI_SRX0: u32 = 0x08;
/* SSI Receive Data Register 1 */
pub const REG_SSI_SRX1: u32 = 0x0c;
/* SSI Control Register */
pub const REG_SSI_SCR: u32 = 0x10;
/* SSI Interrupt Status Register */
pub const REG_SSI_SISR: u32 = 0x14;
/* SSI Interrupt Enable Register */
pub const REG_SSI_SIER: u32 = 0x18;
/* SSI Transmit Configuration Register */
pub const REG_SSI_STCR: u32 = 0x1c;
/* SSI Receive Configuration Register */
pub const REG_SSI_SRCR: u32 = 0x20;
pub const fn REG_SSI_SxCR(tx: u32) -> u32 {
    if tx != 0 { REG_SSI_STCR } else { REG_SSI_SRCR }
}
/* SSI Transmit Clock Control Register */
pub const REG_SSI_STCCR: u32 = 0x24;
/* SSI Receive Clock Control Register */
pub const REG_SSI_SRCCR: u32 = 0x28;
pub const fn REG_SSI_SxCCR(tx: u32) -> u32 {
    if tx != 0 { REG_SSI_STCCR } else { REG_SSI_SRCCR }
}
/* SSI FIFO Control/Status Register */
pub const REG_SSI_SFCSR: u32 = 0x2c;
/*
 * SSI Test Register (Intended for debugging purposes only)
 *
 * Note: STR is not documented in recent IMX datasheet, but
 * is described in IMX51 reference manual at section 56.3.3.14
 */
pub const REG_SSI_STR: u32 = 0x30;
/*
 * SSI Option Register (Intended for internal use only)
 *
 * Note: SOR is not documented in recent IMX datasheet, but
 * is described in IMX51 reference manual at section 56.3.3.15
 */
pub const REG_SSI_SOR: u32 = 0x34;
/* SSI AC97 Control Register */
pub const REG_SSI_SACNT: u32 = 0x38;
/* SSI AC97 Command Address Register */
pub const REG_SSI_SACADD: u32 = 0x3c;
/* SSI AC97 Command Data Register */
pub const REG_SSI_SACDAT: u32 = 0x40;
/* SSI AC97 Tag Register */
pub const REG_SSI_SATAG: u32 = 0x44;
/* SSI Transmit Time Slot Mask Register */
pub const REG_SSI_STMSK: u32 = 0x48;
/* SSI  Receive Time Slot Mask Register */
pub const REG_SSI_SRMSK: u32 = 0x4c;
pub const fn REG_SSI_SxMSK(tx: u32) -> u32 {
    if tx != 0 { REG_SSI_STMSK } else { REG_SSI_SRMSK }
}
/*
 * SSI AC97 Channel Status Register
 *
 * The status could be changed by:
 * 1) Writing a '1' bit at some position in SACCEN sets relevant bit in SACCST
 * 2) Writing a '1' bit at some position in SACCDIS unsets the relevant bit
 * 3) Receivng a '1' in SLOTREQ bit from external CODEC via AC Link
 */
pub const REG_SSI_SACCST: u32 = 0x50;
/* SSI AC97 Channel Enable Register -- Set bits in SACCST */
pub const REG_SSI_SACCEN: u32 = 0x54;
/* SSI AC97 Channel Disable Register -- Clear bits in SACCST */
pub const REG_SSI_SACCDIS: u32 = 0x58;

/* -- SSI Register Field Maps -- */

/* SSI Control Register -- REG_SSI_SCR 0x10 */
pub const SSI_SCR_SYNC_TX_FS: u32 = 0x00001000;
pub const SSI_SCR_RFR_CLK_DIS: u32 = 0x00000800;
pub const SSI_SCR_TFR_CLK_DIS: u32 = 0x00000400;
pub const SSI_SCR_TCH_EN: u32 = 0x00000100;
pub const SSI_SCR_SYS_CLK_EN: u32 = 0x00000080;
pub const SSI_SCR_I2S_MODE_MASK: u32 = 0x00000060;
pub const SSI_SCR_I2S_MODE_NORMAL: u32 = 0x00000000;
pub const SSI_SCR_I2S_MODE_MASTER: u32 = 0x00000020;
pub const SSI_SCR_I2S_MODE_SLAVE: u32 = 0x00000040;
pub const SSI_SCR_SYN: u32 = 0x00000010;
pub const SSI_SCR_NET: u32 = 0x00000008;
pub const SSI_SCR_I2S_NET_MASK: u32 = SSI_SCR_NET | SSI_SCR_I2S_MODE_MASK;
pub const SSI_SCR_RE: u32 = 0x00000004;
pub const SSI_SCR_TE: u32 = 0x00000002;
pub const SSI_SCR_SSIEN: u32 = 0x00000001;

/* SSI Interrupt Status Register -- REG_SSI_SISR 0x14 */
pub const SSI_SISR_RFRC: u32 = 0x01000000;
pub const SSI_SISR_TFRC: u32 = 0x00800000;
pub const SSI_SISR_CMDAU: u32 = 0x00040000;
pub const SSI_SISR_CMDDU: u32 = 0x00020000;
pub const SSI_SISR_RXT: u32 = 0x00010000;
pub const SSI_SISR_RDR1: u32 = 0x00008000;
pub const SSI_SISR_RDR0: u32 = 0x00004000;
pub const SSI_SISR_TDE1: u32 = 0x00002000;
pub const SSI_SISR_TDE0: u32 = 0x00001000;
pub const SSI_SISR_ROE1: u32 = 0x00000800;
pub const SSI_SISR_ROE0: u32 = 0x00000400;
pub const SSI_SISR_TUE1: u32 = 0x00000200;
pub const SSI_SISR_TUE0: u32 = 0x00000100;
pub const SSI_SISR_TFS: u32 = 0x00000080;
pub const SSI_SISR_RFS: u32 = 0x00000040;
pub const SSI_SISR_TLS: u32 = 0x00000020;
pub const SSI_SISR_RLS: u32 = 0x00000010;
pub const SSI_SISR_RFF1: u32 = 0x00000008;
pub const SSI_SISR_RFF0: u32 = 0x00000004;
pub const SSI_SISR_TFE1: u32 = 0x00000002;
pub const SSI_SISR_TFE0: u32 = 0x00000001;

/* SSI Interrupt Enable Register -- REG_SSI_SIER 0x18 */
pub const SSI_SIER_RFRC_EN: u32 = 0x01000000;
pub const SSI_SIER_TFRC_EN: u32 = 0x00800000;
pub const SSI_SIER_RDMAE: u32 = 0x00400000;
pub const SSI_SIER_RIE: u32 = 0x00200000;
pub const SSI_SIER_TDMAE: u32 = 0x00100000;
pub const SSI_SIER_TIE: u32 = 0x00080000;
pub const SSI_SIER_CMDAU_EN: u32 = 0x00040000;
pub const SSI_SIER_CMDDU_EN: u32 = 0x00020000;
pub const SSI_SIER_RXT_EN: u32 = 0x00010000;
pub const SSI_SIER_RDR1_EN: u32 = 0x00008000;
pub const SSI_SIER_RDR0_EN: u32 = 0x00004000;
pub const SSI_SIER_TDE1_EN: u32 = 0x00002000;
pub const SSI_SIER_TDE0_EN: u32 = 0x00001000;
pub const SSI_SIER_ROE1_EN: u32 = 0x00000800;
pub const SSI_SIER_ROE0_EN: u32 = 0x00000400;
pub const SSI_SIER_TUE1_EN: u32 = 0x00000200;
pub const SSI_SIER_TUE0_EN: u32 = 0x00000100;
pub const SSI_SIER_TFS_EN: u32 = 0x00000080;
pub const SSI_SIER_RFS_EN: u32 = 0x00000040;
pub const SSI_SIER_TLS_EN: u32 = 0x00000020;
pub const SSI_SIER_RLS_EN: u32 = 0x00000010;
pub const SSI_SIER_RFF1_EN: u32 = 0x00000008;
pub const SSI_SIER_RFF0_EN: u32 = 0x00000004;
pub const SSI_SIER_TFE1_EN: u32 = 0x00000002;
pub const SSI_SIER_TFE0_EN: u32 = 0x00000001;

/* SSI Transmit Configuration Register -- REG_SSI_STCR 0x1C */
pub const SSI_STCR_TXBIT0: u32 = 0x00000200;
pub const SSI_STCR_TFEN1: u32 = 0x00000100;
pub const SSI_STCR_TFEN0: u32 = 0x00000080;
pub const SSI_STCR_TFDIR: u32 = 0x00000040;
pub const SSI_STCR_TXDIR: u32 = 0x00000020;
pub const SSI_STCR_TSHFD: u32 = 0x00000010;
pub const SSI_STCR_TSCKP: u32 = 0x00000008;
pub const SSI_STCR_TFSI: u32 = 0x00000004;
pub const SSI_STCR_TFSL: u32 = 0x00000002;
pub const SSI_STCR_TEFS: u32 = 0x00000001;

/* SSI Receive Configuration Register -- REG_SSI_SRCR 0x20 */
pub const SSI_SRCR_RXEXT: u32 = 0x00000400;
pub const SSI_SRCR_RXBIT0: u32 = 0x00000200;
pub const SSI_SRCR_RFEN1: u32 = 0x00000100;
pub const SSI_SRCR_RFEN0: u32 = 0x00000080;
pub const SSI_SRCR_RFDIR: u32 = 0x00000040;
pub const SSI_SRCR_RXDIR: u32 = 0x00000020;
pub const SSI_SRCR_RSHFD: u32 = 0x00000010;
pub const SSI_SRCR_RSCKP: u32 = 0x00000008;
pub const SSI_SRCR_RFSI: u32 = 0x00000004;
pub const SSI_SRCR_RFSL: u32 = 0x00000002;
pub const SSI_SRCR_REFS: u32 = 0x00000001;

/*
 * SSI Transmit Clock Control Register -- REG_SSI_STCCR 0x24
 * SSI Receive Clock Control Register -- REG_SSI_SRCCR 0x28
 */
pub const SSI_SxCCR_DIV2_SHIFT: u32 = 18;
pub const SSI_SxCCR_DIV2: u32 = 0x00040000;
pub const SSI_SxCCR_PSR_SHIFT: u32 = 17;
pub const SSI_SxCCR_PSR: u32 = 0x00020000;
pub const SSI_SxCCR_WL_SHIFT: u32 = 13;
pub const SSI_SxCCR_WL_MASK: u32 = 0x0001E000;
pub const fn SSI_SxCCR_WL(x: u32) -> u32 {
    (((x / 2).wrapping_sub(1)) << SSI_SxCCR_WL_SHIFT) & SSI_SxCCR_WL_MASK
}
pub const SSI_SxCCR_DC_SHIFT: u32 = 8;
pub const SSI_SxCCR_DC_MASK: u32 = 0x00001F00;
pub const fn SSI_SxCCR_DC(x: u32) -> u32 {
    (x.wrapping_sub(1) << SSI_SxCCR_DC_SHIFT) & SSI_SxCCR_DC_MASK
}
pub const SSI_SxCCR_PM_SHIFT: u32 = 0;
pub const SSI_SxCCR_PM_MASK: u32 = 0x000000FF;
pub const fn SSI_SxCCR_PM(x: u32) -> u32 {
    (x.wrapping_sub(1) << SSI_SxCCR_PM_SHIFT) & SSI_SxCCR_PM_MASK
}

/*
 * SSI FIFO Control/Status Register -- REG_SSI_SFCSR 0x2c
 *
 * Tx or Rx FIFO Counter -- SSI_SFCSR_xFCNTy Read-Only
 * Tx or Rx FIFO Watermarks -- SSI_SFCSR_xFWMy Read/Write
 */
pub const SSI_SFCSR_RFCNT1_SHIFT: u32 = 28;
pub const SSI_SFCSR_RFCNT1_MASK: u32 = 0xF0000000;
pub const fn SSI_SFCSR_RFCNT1(x: u32) -> u32 {
    (x & SSI_SFCSR_RFCNT1_MASK) >> SSI_SFCSR_RFCNT1_SHIFT
}
pub const SSI_SFCSR_TFCNT1_SHIFT: u32 = 24;
pub const SSI_SFCSR_TFCNT1_MASK: u32 = 0x0F000000;
pub const fn SSI_SFCSR_TFCNT1(x: u32) -> u32 {
    (x & SSI_SFCSR_TFCNT1_MASK) >> SSI_SFCSR_TFCNT1_SHIFT
}
pub const SSI_SFCSR_RFWM1_SHIFT: u32 = 20;
pub const SSI_SFCSR_RFWM1_MASK: u32 = 0x00F00000;
pub const fn SSI_SFCSR_RFWM1(x: u32) -> u32 {
    (x << SSI_SFCSR_RFWM1_SHIFT) & SSI_SFCSR_RFWM1_MASK
}
pub const SSI_SFCSR_TFWM1_SHIFT: u32 = 16;
pub const SSI_SFCSR_TFWM1_MASK: u32 = 0x000F0000;
pub const fn SSI_SFCSR_TFWM1(x: u32) -> u32 {
    (x << SSI_SFCSR_TFWM1_SHIFT) & SSI_SFCSR_TFWM1_MASK
}
pub const SSI_SFCSR_RFCNT0_SHIFT: u32 = 12;
pub const SSI_SFCSR_RFCNT0_MASK: u32 = 0x0000F000;
pub const fn SSI_SFCSR_RFCNT0(x: u32) -> u32 {
    (x & SSI_SFCSR_RFCNT0_MASK) >> SSI_SFCSR_RFCNT0_SHIFT
}
pub const SSI_SFCSR_TFCNT0_SHIFT: u32 = 8;
pub const SSI_SFCSR_TFCNT0_MASK: u32 = 0x00000F00;
pub const fn SSI_SFCSR_TFCNT0(x: u32) -> u32 {
    (x & SSI_SFCSR_TFCNT0_MASK) >> SSI_SFCSR_TFCNT0_SHIFT
}
pub const SSI_SFCSR_RFWM0_SHIFT: u32 = 4;
pub const SSI_SFCSR_RFWM0_MASK: u32 = 0x000000F0;
pub const fn SSI_SFCSR_RFWM0(x: u32) -> u32 {
    (x << SSI_SFCSR_RFWM0_SHIFT) & SSI_SFCSR_RFWM0_MASK
}
pub const SSI_SFCSR_TFWM0_SHIFT: u32 = 0;
pub const SSI_SFCSR_TFWM0_MASK: u32 = 0x0000000F;
pub const fn SSI_SFCSR_TFWM0(x: u32) -> u32 {
    (x << SSI_SFCSR_TFWM0_SHIFT) & SSI_SFCSR_TFWM0_MASK
}

/* SSI Test Register -- REG_SSI_STR 0x30 */
pub const SSI_STR_TEST: u32 = 0x00008000;
pub const SSI_STR_RCK2TCK: u32 = 0x00004000;
pub const SSI_STR_RFS2TFS: u32 = 0x00002000;
pub const fn SSI_STR_RXSTATE(x: u32) -> u32 {
    (x >> 8) & 0x1F
}
pub const SSI_STR_TXD2RXD: u32 = 0x00000080;
pub const SSI_STR_TCK2RCK: u32 = 0x00000040;
pub const SSI_STR_TFS2RFS: u32 = 0x00000020;
pub const fn SSI_STR_TXSTATE(x: u32) -> u32 {
    x & 0x1F
}

/* SSI Option Register -- REG_SSI_SOR 0x34 */
pub const SSI_SOR_CLKOFF: u32 = 0x00000040;
pub const SSI_SOR_RX_CLR: u32 = 0x00000020;
pub const SSI_SOR_TX_CLR: u32 = 0x00000010;
pub const fn SSI_SOR_xX_CLR(tx: u32) -> u32 {
    if tx != 0 { SSI_SOR_TX_CLR } else { SSI_SOR_RX_CLR }
}
pub const SSI_SOR_INIT: u32 = 0x00000008;
pub const SSI_SOR_WAIT_SHIFT: u32 = 1;
pub const SSI_SOR_WAIT_MASK: u32 = 0x00000006;
pub const fn SSI_SOR_WAIT(x: u32) -> u32 {
    (x & 3) << SSI_SOR_WAIT_SHIFT
}
pub const SSI_SOR_SYNRST: u32 = 0x00000001;

/* SSI AC97 Control Register -- REG_SSI_SACNT 0x38 */
pub const fn SSI_SACNT_FRDIV(x: u32) -> u32 {
    (x & 0x3f) << 5
}
pub const SSI_SACNT_WR: u32 = 0x00000010;
pub const SSI_SACNT_RD: u32 = 0x00000008;
pub const SSI_SACNT_RDWR_MASK: u32 = 0x00000018;
pub const SSI_SACNT_TIF: u32 = 0x00000004;
pub const SSI_SACNT_FV: u32 = 0x00000002;
pub const SSI_SACNT_AC97EN: u32 = 0x00000001;

#[repr(C)]
pub struct device {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct dentry {
    _unused: [u8; 0],
}

/* C conditional: IS_ENABLED(CONFIG_DEBUG_FS). */
#[cfg(CONFIG_DEBUG_FS)]
#[repr(C)]
pub struct fsl_ssi_dbg_stats {
    pub rfrc: ::core::ffi::c_uint,
    pub tfrc: ::core::ffi::c_uint,
    pub cmdau: ::core::ffi::c_uint,
    pub cmddu: ::core::ffi::c_uint,
    pub rxt: ::core::ffi::c_uint,
    pub rdr1: ::core::ffi::c_uint,
    pub rdr0: ::core::ffi::c_uint,
    pub tde1: ::core::ffi::c_uint,
    pub tde0: ::core::ffi::c_uint,
    pub roe1: ::core::ffi::c_uint,
    pub roe0: ::core::ffi::c_uint,
    pub tue1: ::core::ffi::c_uint,
    pub tue0: ::core::ffi::c_uint,
    pub tfs: ::core::ffi::c_uint,
    pub rfs: ::core::ffi::c_uint,
    pub tls: ::core::ffi::c_uint,
    pub rls: ::core::ffi::c_uint,
    pub rff1: ::core::ffi::c_uint,
    pub rff0: ::core::ffi::c_uint,
    pub tfe1: ::core::ffi::c_uint,
    pub tfe0: ::core::ffi::c_uint,
}

#[cfg(CONFIG_DEBUG_FS)]
#[repr(C)]
pub struct fsl_ssi_dbg {
    pub dbg_dir: *mut dentry,
    pub stats: fsl_ssi_dbg_stats,
}

#[cfg(CONFIG_DEBUG_FS)]
unsafe extern "C" {
    pub fn fsl_ssi_dbg_isr(ssi_dbg: *mut fsl_ssi_dbg, sisr: u32);
    pub fn fsl_ssi_debugfs_create(ssi_dbg: *mut fsl_ssi_dbg, dev: *mut device);
    pub fn fsl_ssi_debugfs_remove(ssi_dbg: *mut fsl_ssi_dbg);
}

/* C conditional else branch: !IS_ENABLED(CONFIG_DEBUG_FS). */
#[cfg(not(CONFIG_DEBUG_FS))]
#[repr(C)]
pub struct fsl_ssi_dbg {
    _unused: [u8; 0],
}

#[cfg(not(CONFIG_DEBUG_FS))]
pub unsafe fn fsl_ssi_dbg_isr(_stats: *mut fsl_ssi_dbg, _sisr: u32) {}

#[cfg(not(CONFIG_DEBUG_FS))]
pub unsafe fn fsl_ssi_debugfs_create(_ssi_dbg: *mut fsl_ssi_dbg, _dev: *mut device) {}

#[cfg(not(CONFIG_DEBUG_FS))]
pub unsafe fn fsl_ssi_debugfs_remove(_ssi_dbg: *mut fsl_ssi_dbg) {}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
