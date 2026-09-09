/*
 *  include/asm-ppc/hydra.h -- Mac I/O `Hydra' definitions
 *
 *  Copyright (C) 1997 Geert Uytterhoeven
 *
 *  This file is based on the following documentation:
 *
 *\tMacintosh Technology in the Common Hardware Reference Platform
 *\tApple Computer, Inc.
 *
 *  © Copyright 1995 Apple Computer, Inc. All rights reserved.
 *
 *  It's available online from https://www.cpu.lu/~mlan/ftp/MacTech.pdf
 *  You can obtain paper copies of this book from computer bookstores or by
 *  writing Morgan Kaufmann Publishers, Inc., 340 Pine Street, Sixth Floor, San
 *  Francisco, CA 94104. Reference ISBN 1-55860-393-X.
 *
 *  This file is subject to the terms and conditions of the GNU General Public
 *  License.  See the file COPYING in the main directory of this archive
 *  for more details.
 */

// The original declarations were enabled only under __KERNEL__.

#[repr(C)]
pub struct Hydra {
    /* DBDMA Controller Register Space */
    pub Pad1: [u8; 0x30],
    pub CachePD: u32,
    pub IDs: u32,
    pub Feature_Control: u32,
    pub Pad2: [u8; 0x7fc4],
    /* DBDMA Channel Register Space */
    pub SCSI_DMA: [u8; 0x100],
    pub Pad3: [u8; 0x300],
    pub SCCA_Tx_DMA: [u8; 0x100],
    pub SCCA_Rx_DMA: [u8; 0x100],
    pub SCCB_Tx_DMA: [u8; 0x100],
    pub SCCB_Rx_DMA: [u8; 0x100],
    pub Pad4: [u8; 0x7800],
    /* Device Register Space */
    pub SCSI: [u8; 0x1000],
    pub ADB: [u8; 0x1000],
    pub SCC_Legacy: [u8; 0x1000],
    pub SCC: [u8; 0x1000],
    pub Pad9: [u8; 0x2000],
    pub VIA: [u8; 0x2000],
    pub Pad10: [u8; 0x28000],
    pub OpenPIC: [u8; 0x40000],
}

extern "C" {
    pub static mut Hydra: *mut Hydra;
}

/*
 *  Feature Control Register
 */

pub const HYDRA_FC_SCC_CELL_EN: u32 = 0x00000001; /* Enable SCC Clock */
pub const HYDRA_FC_SCSI_CELL_EN: u32 = 0x00000002; /* Enable SCSI Clock */
pub const HYDRA_FC_SCCA_ENABLE: u32 = 0x00000004; /* Enable SCC A Lines */
pub const HYDRA_FC_SCCB_ENABLE: u32 = 0x00000008; /* Enable SCC B Lines */
pub const HYDRA_FC_ARB_BYPASS: u32 = 0x00000010; /* Bypass Internal Arbiter */
pub const HYDRA_FC_RESET_SCC: u32 = 0x00000020; /* Reset SCC */
pub const HYDRA_FC_MPIC_ENABLE: u32 = 0x00000040; /* Enable OpenPIC */
pub const HYDRA_FC_SLOW_SCC_PCLK: u32 = 0x00000080; /* 1=15.6672, 0=25 MHz */
pub const HYDRA_FC_MPIC_IS_MASTER: u32 = 0x00000100; /* OpenPIC Master Mode */

/*
 *  OpenPIC Interrupt Sources
 */

pub const HYDRA_INT_SIO: u32 = 0;
pub const HYDRA_INT_SCSI_DMA: u32 = 1;
pub const HYDRA_INT_SCCA_TX_DMA: u32 = 2;
pub const HYDRA_INT_SCCA_RX_DMA: u32 = 3;
pub const HYDRA_INT_SCCB_TX_DMA: u32 = 4;
pub const HYDRA_INT_SCCB_RX_DMA: u32 = 5;
pub const HYDRA_INT_SCSI: u32 = 6;
pub const HYDRA_INT_SCCA: u32 = 7;
pub const HYDRA_INT_SCCB: u32 = 8;
pub const HYDRA_INT_VIA: u32 = 9;
pub const HYDRA_INT_ADB: u32 = 10;
pub const HYDRA_INT_ADB_NMI: u32 = 11;
pub const HYDRA_INT_EXT1: u32 = 12; /* PCI IRQW */
pub const HYDRA_INT_EXT2: u32 = 13; /* PCI IRQX */
pub const HYDRA_INT_EXT3: u32 = 14; /* PCI IRQY */
pub const HYDRA_INT_EXT4: u32 = 15; /* PCI IRQZ */
pub const HYDRA_INT_EXT5: u32 = 16; /* IDE Primary/Secondary */
pub const HYDRA_INT_EXT6: u32 = 17; /* IDE Secondary */
pub const HYDRA_INT_EXT7: u32 = 18; /* Power Off Request */
pub const HYDRA_INT_SPARE: u32 = 19;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
