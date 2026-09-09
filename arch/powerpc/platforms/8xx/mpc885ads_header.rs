/*
 * A collection of structures, addresses, and values associated with
 * the Freescale MPC885ADS board.
 * Copied from the FADS stuff.
 *
 * Author: MontaVista Software, Inc.
 *         source@mvista.com
 *
 * 2005 (c) MontaVista Software, Inc.  This file is licensed under the
 * terms of the GNU General Public License version 2.  This program is licensed
 * "as is" without any warranty of any kind, whether express or implied.
 */

// The original declarations are guarded by __KERNEL__ and __ASM_MPC885ADS_H__.
// Dependency: <sysdev/fsl_soc.h>.

/* Bits of interest in the BCSRs.
 */
pub const BCSR1_ETHEN: u32 = 0x20000000u32;
pub const BCSR1_IRDAEN: u32 = 0x10000000u32;
pub const BCSR1_RS232EN_1: u32 = 0x01000000u32;
pub const BCSR1_PCCEN: u32 = 0x00800000u32;
pub const BCSR1_PCCVCC0: u32 = 0x00400000u32;
pub const BCSR1_PCCVPP0: u32 = 0x00200000u32;
pub const BCSR1_PCCVPP1: u32 = 0x00100000u32;
pub const BCSR1_PCCVPP_MASK: u32 = BCSR1_PCCVPP0 | BCSR1_PCCVPP1;
pub const BCSR1_RS232EN_2: u32 = 0x00040000u32;
pub const BCSR1_PCCVCC1: u32 = 0x00010000u32;
pub const BCSR1_PCCVCC_MASK: u32 = BCSR1_PCCVCC0 | BCSR1_PCCVCC1;

pub const BCSR4_ETH10_RST: u32 = 0x80000000u32; // 10Base-T PHY reset
pub const BCSR4_USB_LO_SPD: u32 = 0x04000000u32;
pub const BCSR4_USB_VCC: u32 = 0x02000000u32;
pub const BCSR4_USB_FULL_SPD: u32 = 0x00040000u32;
pub const BCSR4_USB_EN: u32 = 0x00020000u32;

pub const BCSR5_MII2_EN: u32 = 0x40u32;
pub const BCSR5_MII2_RST: u32 = 0x20u32;
pub const BCSR5_T1_RST: u32 = 0x10u32;
pub const BCSR5_ATM155_RST: u32 = 0x08u32;
pub const BCSR5_ATM25_RST: u32 = 0x04u32;
pub const BCSR5_MII1_EN: u32 = 0x02u32;
pub const BCSR5_MII1_RST: u32 = 0x01u32;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
