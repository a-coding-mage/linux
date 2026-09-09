// Translated from cvmx-pci-defs.h.
#![allow(non_camel_case_types, non_upper_case_globals, dead_code)]
#[repr(C)] pub struct cvmx_pci_bitfield_placeholder { pub value: u64 }

/***********************license start***************
 * Author: Cavium Networks
 *
 * Contact: support@caviumnetworks.com
 * This file is part of the OCTEON SDK
 *
 * Copyright (c) 2003-2012 Cavium Networks
 *
 * This file is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License, Version 2, as
 * published by the Free Software Foundation.
 *
 * This file is distributed in the hope that it will be useful, but
 * AS-IS and WITHOUT ANY WARRANTY; without even the implied warranty
 * of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE, TITLE, or
 * NONINFRINGEMENT.  See the GNU General Public License for more
 * details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this file; if not, write to the Free Software
 * Foundation, Inc., 51 Franklin St, Fifth Floor, Boston, MA 02110-1301 USA
 * or visit http://www.gnu.org/licenses/.
 *
 * This file may also be available under a different license from Cavium.
 * Contact Cavium Networks for more information
 ***********************license end**************************************/

// #ifndef __CVMX_PCI_DEFS_H__
// #define __CVMX_PCI_DEFS_H__

#[inline] pub const fn CVMX_PCI_BAR1_INDEXX(offset: u64) -> u64 { (0x0000000000000100u64 + ((offset) & 31) * 4) }
pub const CVMX_PCI_BIST_REG: u64 = (0x00000000000001C0u64);
pub const CVMX_PCI_CFG00: u64 = (0x0000000000000000u64);
pub const CVMX_PCI_CFG01: u64 = (0x0000000000000004u64);
pub const CVMX_PCI_CFG02: u64 = (0x0000000000000008u64);
pub const CVMX_PCI_CFG03: u64 = (0x000000000000000Cu64);
pub const CVMX_PCI_CFG04: u64 = (0x0000000000000010u64);
pub const CVMX_PCI_CFG05: u64 = (0x0000000000000014u64);
pub const CVMX_PCI_CFG06: u64 = (0x0000000000000018u64);
pub const CVMX_PCI_CFG07: u64 = (0x000000000000001Cu64);
pub const CVMX_PCI_CFG08: u64 = (0x0000000000000020u64);
pub const CVMX_PCI_CFG09: u64 = (0x0000000000000024u64);
pub const CVMX_PCI_CFG10: u64 = (0x0000000000000028u64);
pub const CVMX_PCI_CFG11: u64 = (0x000000000000002Cu64);
pub const CVMX_PCI_CFG12: u64 = (0x0000000000000030u64);
pub const CVMX_PCI_CFG13: u64 = (0x0000000000000034u64);
pub const CVMX_PCI_CFG15: u64 = (0x000000000000003Cu64);
pub const CVMX_PCI_CFG16: u64 = (0x0000000000000040u64);
pub const CVMX_PCI_CFG17: u64 = (0x0000000000000044u64);
pub const CVMX_PCI_CFG18: u64 = (0x0000000000000048u64);
pub const CVMX_PCI_CFG19: u64 = (0x000000000000004Cu64);
pub const CVMX_PCI_CFG20: u64 = (0x0000000000000050u64);
pub const CVMX_PCI_CFG21: u64 = (0x0000000000000054u64);
pub const CVMX_PCI_CFG22: u64 = (0x0000000000000058u64);
pub const CVMX_PCI_CFG56: u64 = (0x00000000000000E0u64);
pub const CVMX_PCI_CFG57: u64 = (0x00000000000000E4u64);
pub const CVMX_PCI_CFG58: u64 = (0x00000000000000E8u64);
pub const CVMX_PCI_CFG59: u64 = (0x00000000000000ECu64);
pub const CVMX_PCI_CFG60: u64 = (0x00000000000000F0u64);
pub const CVMX_PCI_CFG61: u64 = (0x00000000000000F4u64);
pub const CVMX_PCI_CFG62: u64 = (0x00000000000000F8u64);
pub const CVMX_PCI_CFG63: u64 = (0x00000000000000FCu64);
pub const CVMX_PCI_CNT_REG: u64 = (0x00000000000001B8u64);
pub const CVMX_PCI_CTL_STATUS_2: u64 = (0x000000000000018Cu64);
#[inline] pub const fn CVMX_PCI_DBELL_X(offset: u64) -> u64 { (0x0000000000000080u64 + ((offset) & 3) * 8) }
pub const CVMX_PCI_DMA_CNT0: u64 = CVMX_PCI_DMA_CNTX(0);
pub const CVMX_PCI_DMA_CNT1: u64 = CVMX_PCI_DMA_CNTX(1);
#[inline] pub const fn CVMX_PCI_DMA_CNTX(offset: u64) -> u64 { (0x00000000000000A0u64 + ((offset) & 1) * 8) }
pub const CVMX_PCI_DMA_INT_LEV0: u64 = CVMX_PCI_DMA_INT_LEVX(0);
pub const CVMX_PCI_DMA_INT_LEV1: u64 = CVMX_PCI_DMA_INT_LEVX(1);
#[inline] pub const fn CVMX_PCI_DMA_INT_LEVX(offset: u64) -> u64 { (0x00000000000000A4u64 + ((offset) & 1) * 8) }
pub const CVMX_PCI_DMA_TIME0: u64 = CVMX_PCI_DMA_TIMEX(0);
pub const CVMX_PCI_DMA_TIME1: u64 = CVMX_PCI_DMA_TIMEX(1);
#[inline] pub const fn CVMX_PCI_DMA_TIMEX(offset: u64) -> u64 { (0x00000000000000B0u64 + ((offset) & 1) * 4) }
pub const CVMX_PCI_INSTR_COUNT0: u64 = CVMX_PCI_INSTR_COUNTX(0);
pub const CVMX_PCI_INSTR_COUNT1: u64 = CVMX_PCI_INSTR_COUNTX(1);
pub const CVMX_PCI_INSTR_COUNT2: u64 = CVMX_PCI_INSTR_COUNTX(2);
pub const CVMX_PCI_INSTR_COUNT3: u64 = CVMX_PCI_INSTR_COUNTX(3);
#[inline] pub const fn CVMX_PCI_INSTR_COUNTX(offset: u64) -> u64 { (0x0000000000000084u64 + ((offset) & 3) * 8) }
pub const CVMX_PCI_INT_ENB: u64 = (0x0000000000000038u64);
pub const CVMX_PCI_INT_ENB2: u64 = (0x00000000000001A0u64);
pub const CVMX_PCI_INT_SUM: u64 = (0x0000000000000030u64);
pub const CVMX_PCI_INT_SUM2: u64 = (0x0000000000000198u64);
pub const CVMX_PCI_MSI_RCV: u64 = (0x00000000000000F0u64);
pub const CVMX_PCI_PKTS_SENT0: u64 = CVMX_PCI_PKTS_SENTX(0);
pub const CVMX_PCI_PKTS_SENT1: u64 = CVMX_PCI_PKTS_SENTX(1);
pub const CVMX_PCI_PKTS_SENT2: u64 = CVMX_PCI_PKTS_SENTX(2);
pub const CVMX_PCI_PKTS_SENT3: u64 = CVMX_PCI_PKTS_SENTX(3);
#[inline] pub const fn CVMX_PCI_PKTS_SENTX(offset: u64) -> u64 { (0x0000000000000040u64 + ((offset) & 3) * 16) }
pub const CVMX_PCI_PKTS_SENT_INT_LEV0: u64 = CVMX_PCI_PKTS_SENT_INT_LEVX(0);
pub const CVMX_PCI_PKTS_SENT_INT_LEV1: u64 = CVMX_PCI_PKTS_SENT_INT_LEVX(1);
pub const CVMX_PCI_PKTS_SENT_INT_LEV2: u64 = CVMX_PCI_PKTS_SENT_INT_LEVX(2);
pub const CVMX_PCI_PKTS_SENT_INT_LEV3: u64 = CVMX_PCI_PKTS_SENT_INT_LEVX(3);
#[inline] pub const fn CVMX_PCI_PKTS_SENT_INT_LEVX(offset: u64) -> u64 { (0x0000000000000048u64 + ((offset) & 3) * 16) }
pub const CVMX_PCI_PKTS_SENT_TIME0: u64 = CVMX_PCI_PKTS_SENT_TIMEX(0);
pub const CVMX_PCI_PKTS_SENT_TIME1: u64 = CVMX_PCI_PKTS_SENT_TIMEX(1);
pub const CVMX_PCI_PKTS_SENT_TIME2: u64 = CVMX_PCI_PKTS_SENT_TIMEX(2);
pub const CVMX_PCI_PKTS_SENT_TIME3: u64 = CVMX_PCI_PKTS_SENT_TIMEX(3);
#[inline] pub const fn CVMX_PCI_PKTS_SENT_TIMEX(offset: u64) -> u64 { (0x000000000000004Cu64 + ((offset) & 3) * 16) }
pub const CVMX_PCI_PKT_CREDITS0: u64 = CVMX_PCI_PKT_CREDITSX(0);
pub const CVMX_PCI_PKT_CREDITS1: u64 = CVMX_PCI_PKT_CREDITSX(1);
pub const CVMX_PCI_PKT_CREDITS2: u64 = CVMX_PCI_PKT_CREDITSX(2);
pub const CVMX_PCI_PKT_CREDITS3: u64 = CVMX_PCI_PKT_CREDITSX(3);
#[inline] pub const fn CVMX_PCI_PKT_CREDITSX(offset: u64) -> u64 { (0x0000000000000044u64 + ((offset) & 3) * 16) }
pub const CVMX_PCI_READ_CMD_6: u64 = (0x0000000000000180u64);
pub const CVMX_PCI_READ_CMD_C: u64 = (0x0000000000000184u64);
pub const CVMX_PCI_READ_CMD_E: u64 = (0x0000000000000188u64);
pub const CVMX_PCI_READ_TIMEOUT: u64 = (cvmx_add_io_seg(0x00011F00000000B0u64));
pub const CVMX_PCI_SCM_REG: u64 = (0x00000000000001A8u64);
pub const CVMX_PCI_TSR_REG: u64 = (0x00000000000001B0u64);
pub const CVMX_PCI_WIN_RD_ADDR: u64 = (0x0000000000000008u64);
pub const CVMX_PCI_WIN_RD_DATA: u64 = (0x0000000000000020u64);
pub const CVMX_PCI_WIN_WR_ADDR: u64 = (0x0000000000000000u64);
pub const CVMX_PCI_WIN_WR_DATA: u64 = (0x0000000000000010u64);
pub const CVMX_PCI_WIN_WR_MASK: u64 = (0x0000000000000018u64);

#[repr(C)] pub union cvmx_pci_bar1_indexx {
    pub 32: u32,
#[repr(C)] pub struct cvmx_pci_bar1_indexx_s {
// #ifdef __BIG_ENDIAN_BITFIELD
    pub reserved_18_31: u32, // bitfield width 14
    pub addr_idx: u32, // bitfield width 14
    pub ca: u32, // bitfield width 1
    pub end_swp: u32, // bitfield width 2
    pub addr_v: u32, // bitfield width 1
// #else
    pub addr_v: u32, // bitfield width 1
    pub end_swp: u32, // bitfield width 2
    pub ca: u32, // bitfield width 1
    pub addr_idx: u32, // bitfield width 14
    pub reserved_18_31: u32, // bitfield width 14
// #endif
    pub s: cvmx_pci_bitfield_placeholder,
}

#[repr(C)] pub union cvmx_pci_bist_reg {
    pub 64: u64,
#[repr(C)] pub struct cvmx_pci_bist_reg_s {
// #ifdef __BIG_ENDIAN_BITFIELD
    pub reserved_10_63: u64, // bitfield width 54
    pub rsp_bs: u64, // bitfield width 1
    pub dma0_bs: u64, // bitfield width 1
    pub cmd0_bs: u64, // bitfield width 1
    pub cmd_bs: u64, // bitfield width 1
    pub csr2p_bs: u64, // bitfield width 1
    pub csrr_bs: u64, // bitfield width 1
    pub rsp2p_bs: u64, // bitfield width 1
    pub csr2n_bs: u64, // bitfield width 1
    pub dat2n_bs: u64, // bitfield width 1
    pub dbg2n_bs: u64, // bitfield width 1
// #else
    pub dbg2n_bs: u64, // bitfield width 1
    pub dat2n_bs: u64, // bitfield width 1
    pub csr2n_bs: u64, // bitfield width 1
    pub rsp2p_bs: u64, // bitfield width 1
    pub csrr_bs: u64, // bitfield width 1
    pub csr2p_bs: u64, // bitfield width 1
    pub cmd_bs: u64, // bitfield width 1
    pub cmd0_bs: u64, // bitfield width 1
    pub dma0_bs: u64, // bitfield width 1
    pub rsp_bs: u64, // bitfield width 1
    pub reserved_10_63: u64, // bitfield width 54
// #endif
    pub s: cvmx_pci_bitfield_placeholder,
}

#[repr(C)] pub union cvmx_pci_cfg00 {
    pub 32: u32,
#[repr(C)] pub struct cvmx_pci_cfg00_s {
// #ifdef __BIG_ENDIAN_BITFIELD
    pub devid: u32, // bitfield width 16
    pub vendid: u32, // bitfield width 16
// #else
    pub vendid: u32, // bitfield width 16
    pub devid: u32, // bitfield width 16
// #endif
    pub s: cvmx_pci_bitfield_placeholder,
}

#[repr(C)] pub union cvmx_pci_cfg01 {
    pub 32: u32,
#[repr(C)] pub struct cvmx_pci_cfg01_s {
// #ifdef __BIG_ENDIAN_BITFIELD
    pub dpe: u32, // bitfield width 1
    pub sse: u32, // bitfield width 1
    pub rma: u32, // bitfield width 1
    pub rta: u32, // bitfield width 1
    pub sta: u32, // bitfield width 1
    pub devt: u32, // bitfield width 2
    pub mdpe: u32, // bitfield width 1
    pub fbb: u32, // bitfield width 1
    pub reserved_22_22: u32, // bitfield width 1
    pub m66: u32, // bitfield width 1
    pub cle: u32, // bitfield width 1
    pub i_stat: u32, // bitfield width 1
    pub reserved_11_18: u32, // bitfield width 8
    pub i_dis: u32, // bitfield width 1
    pub fbbe: u32, // bitfield width 1
    pub see: u32, // bitfield width 1
    pub ads: u32, // bitfield width 1
    pub pee: u32, // bitfield width 1
    pub vps: u32, // bitfield width 1
    pub mwice: u32, // bitfield width 1
    pub scse: u32, // bitfield width 1
    pub me: u32, // bitfield width 1
    pub msae: u32, // bitfield width 1
    pub isae: u32, // bitfield width 1
// #else
    pub isae: u32, // bitfield width 1
    pub msae: u32, // bitfield width 1
    pub me: u32, // bitfield width 1
    pub scse: u32, // bitfield width 1
    pub mwice: u32, // bitfield width 1
    pub vps: u32, // bitfield width 1
    pub pee: u32, // bitfield width 1
    pub ads: u32, // bitfield width 1
    pub see: u32, // bitfield width 1
    pub fbbe: u32, // bitfield width 1
    pub i_dis: u32, // bitfield width 1
    pub reserved_11_18: u32, // bitfield width 8
    pub i_stat: u32, // bitfield width 1
    pub cle: u32, // bitfield width 1
    pub m66: u32, // bitfield width 1
    pub reserved_22_22: u32, // bitfield width 1
    pub fbb: u32, // bitfield width 1
    pub mdpe: u32, // bitfield width 1
    pub devt: u32, // bitfield width 2
    pub sta: u32, // bitfield width 1
    pub rta: u32, // bitfield width 1
    pub rma: u32, // bitfield width 1
    pub sse: u32, // bitfield width 1
    pub dpe: u32, // bitfield width 1
// #endif
    pub s: cvmx_pci_bitfield_placeholder,
}

#[repr(C)] pub union cvmx_pci_cfg02 {
    pub 32: u32,
#[repr(C)] pub struct cvmx_pci_cfg02_s {
// #ifdef __BIG_ENDIAN_BITFIELD
    pub cc: u32, // bitfield width 24
    pub rid: u32, // bitfield width 8
// #else
    pub rid: u32, // bitfield width 8
    pub cc: u32, // bitfield width 24
// #endif
    pub s: cvmx_pci_bitfield_placeholder,
}

#[repr(C)] pub union cvmx_pci_cfg03 {
    pub 32: u32,
#[repr(C)] pub struct cvmx_pci_cfg03_s {
// #ifdef __BIG_ENDIAN_BITFIELD
    pub bcap: u32, // bitfield width 1
    pub brb: u32, // bitfield width 1
    pub reserved_28_29: u32, // bitfield width 2
    pub bcod: u32, // bitfield width 4
    pub ht: u32, // bitfield width 8
    pub lt: u32, // bitfield width 8
    pub cls: u32, // bitfield width 8
// #else
    pub cls: u32, // bitfield width 8
    pub lt: u32, // bitfield width 8
    pub ht: u32, // bitfield width 8
    pub bcod: u32, // bitfield width 4
    pub reserved_28_29: u32, // bitfield width 2
    pub brb: u32, // bitfield width 1
    pub bcap: u32, // bitfield width 1
// #endif
    pub s: cvmx_pci_bitfield_placeholder,
}

#[repr(C)] pub union cvmx_pci_cfg04 {
    pub 32: u32,
#[repr(C)] pub struct cvmx_pci_cfg04_s {
// #ifdef __BIG_ENDIAN_BITFIELD
    pub lbase: u32, // bitfield width 20
    pub lbasez: u32, // bitfield width 8
    pub pf: u32, // bitfield width 1
    pub typ: u32, // bitfield width 2
    pub mspc: u32, // bitfield width 1
// #else
    pub mspc: u32, // bitfield width 1
    pub typ: u32, // bitfield width 2
    pub pf: u32, // bitfield width 1
    pub lbasez: u32, // bitfield width 8
    pub lbase: u32, // bitfield width 20
// #endif
    pub s: cvmx_pci_bitfield_placeholder,
}

#[repr(C)] pub union cvmx_pci_cfg05 {
    pub 32: u32,
#[repr(C)] pub struct cvmx_pci_cfg05_s {
// #ifdef __BIG_ENDIAN_BITFIELD
    pub hbase: u32, // bitfield width 32
// #else
    pub hbase: u32, // bitfield width 32
// #endif
    pub s: cvmx_pci_bitfield_placeholder,
}

#[repr(C)] pub union cvmx_pci_cfg06 {
    pub 32: u32,
#[repr(C)] pub struct cvmx_pci_cfg06_s {
// #ifdef __BIG_ENDIAN_BITFIELD
    pub lbase: u32, // bitfield width 5
    pub lbasez: u32, // bitfield width 23
    pub pf: u32, // bitfield width 1
    pub typ: u32, // bitfield width 2
    pub mspc: u32, // bitfield width 1
// #else
    pub mspc: u32, // bitfield width 1
    pub typ: u32, // bitfield width 2
    pub pf: u32, // bitfield width 1
    pub lbasez: u32, // bitfield width 23
    pub lbase: u32, // bitfield width 5
// #endif
    pub s: cvmx_pci_bitfield_placeholder,
}

#[repr(C)] pub union cvmx_pci_cfg07 {
    pub 32: u32,
#[repr(C)] pub struct cvmx_pci_cfg07_s {
// #ifdef __BIG_ENDIAN_BITFIELD
    pub hbase: u32, // bitfield width 32
// #else
    pub hbase: u32, // bitfield width 32
// #endif
    pub s: cvmx_pci_bitfield_placeholder,
}

#[repr(C)] pub union cvmx_pci_cfg08 {
    pub 32: u32,
#[repr(C)] pub struct cvmx_pci_cfg08_s {
// #ifdef __BIG_ENDIAN_BITFIELD
    pub lbasez: u32, // bitfield width 28
    pub pf: u32, // bitfield width 1
    pub typ: u32, // bitfield width 2
    pub mspc: u32, // bitfield width 1
// #else
    pub mspc: u32, // bitfield width 1
    pub typ: u32, // bitfield width 2
    pub pf: u32, // bitfield width 1
    pub lbasez: u32, // bitfield width 28
// #endif
    pub s: cvmx_pci_bitfield_placeholder,
}

#[repr(C)] pub union cvmx_pci_cfg09 {
    pub 32: u32,
#[repr(C)] pub struct cvmx_pci_cfg09_s {
// #ifdef __BIG_ENDIAN_BITFIELD
    pub hbase: u32, // bitfield width 25
    pub hbasez: u32, // bitfield width 7
// #else
    pub hbasez: u32, // bitfield width 7
    pub hbase: u32, // bitfield width 25
// #endif
    pub s: cvmx_pci_bitfield_placeholder,
}

#[repr(C)] pub union cvmx_pci_cfg10 {
    pub 32: u32,
#[repr(C)] pub struct cvmx_pci_cfg10_s {
// #ifdef __BIG_ENDIAN_BITFIELD
    pub cisp: u32, // bitfield width 32
// #else
    pub cisp: u32, // bitfield width 32
// #endif
    pub s: cvmx_pci_bitfield_placeholder,
}

#[repr(C)] pub union cvmx_pci_cfg11 {
    pub 32: u32,
#[repr(C)] pub struct cvmx_pci_cfg11_s {
// #ifdef __BIG_ENDIAN_BITFIELD
    pub ssid: u32, // bitfield width 16
    pub ssvid: u32, // bitfield width 16
// #else
    pub ssvid: u32, // bitfield width 16
    pub ssid: u32, // bitfield width 16
// #endif
    pub s: cvmx_pci_bitfield_placeholder,
}

#[repr(C)] pub union cvmx_pci_cfg12 {
    pub 32: u32,
#[repr(C)] pub struct cvmx_pci_cfg12_s {
// #ifdef __BIG_ENDIAN_BITFIELD
    pub erbar: u32, // bitfield width 16
    pub erbarz: u32, // bitfield width 5
    pub reserved_1_10: u32, // bitfield width 10
    pub erbar_en: u32, // bitfield width 1
// #else
    pub erbar_en: u32, // bitfield width 1
    pub reserved_1_10: u32, // bitfield width 10
    pub erbarz: u32, // bitfield width 5
    pub erbar: u32, // bitfield width 16
// #endif
    pub s: cvmx_pci_bitfield_placeholder,
}

#[repr(C)] pub union cvmx_pci_cfg13 {
    pub 32: u32,
#[repr(C)] pub struct cvmx_pci_cfg13_s {
// #ifdef __BIG_ENDIAN_BITFIELD
    pub reserved_8_31: u32, // bitfield width 24
    pub cp: u32, // bitfield width 8
// #else
    pub cp: u32, // bitfield width 8
    pub reserved_8_31: u32, // bitfield width 24
// #endif
    pub s: cvmx_pci_bitfield_placeholder,
}

#[repr(C)] pub union cvmx_pci_cfg15 {
    pub 32: u32,
#[repr(C)] pub struct cvmx_pci_cfg15_s {
// #ifdef __BIG_ENDIAN_BITFIELD
    pub ml: u32, // bitfield width 8
    pub mg: u32, // bitfield width 8
    pub inta: u32, // bitfield width 8
    pub il: u32, // bitfield width 8
// #else
    pub il: u32, // bitfield width 8
    pub inta: u32, // bitfield width 8
    pub mg: u32, // bitfield width 8
    pub ml: u32, // bitfield width 8
// #endif
    pub s: cvmx_pci_bitfield_placeholder,
}

#[repr(C)] pub union cvmx_pci_cfg16 {
    pub 32: u32,
#[repr(C)] pub struct cvmx_pci_cfg16_s {
// #ifdef __BIG_ENDIAN_BITFIELD
    pub trdnpr: u32, // bitfield width 1
    pub trdard: u32, // bitfield width 1
    pub rdsati: u32, // bitfield width 1
    pub trdrs: u32, // bitfield width 1
    pub trtae: u32, // bitfield width 1
    pub twsei: u32, // bitfield width 1
    pub twsen: u32, // bitfield width 1
    pub twtae: u32, // bitfield width 1
    pub tmae: u32, // bitfield width 1
    pub tslte: u32, // bitfield width 3
    pub tilt: u32, // bitfield width 4
    pub pbe: u32, // bitfield width 12
    pub dppmr: u32, // bitfield width 1
    pub reserved_2_2: u32, // bitfield width 1
    pub tswc: u32, // bitfield width 1
    pub mltd: u32, // bitfield width 1
// #else
    pub mltd: u32, // bitfield width 1
    pub tswc: u32, // bitfield width 1
    pub reserved_2_2: u32, // bitfield width 1
    pub dppmr: u32, // bitfield width 1
    pub pbe: u32, // bitfield width 12
    pub tilt: u32, // bitfield width 4
    pub tslte: u32, // bitfield width 3
    pub tmae: u32, // bitfield width 1
    pub twtae: u32, // bitfield width 1
    pub twsen: u32, // bitfield width 1
    pub twsei: u32, // bitfield width 1
    pub trtae: u32, // bitfield width 1
    pub trdrs: u32, // bitfield width 1
    pub rdsati: u32, // bitfield width 1
    pub trdard: u32, // bitfield width 1
    pub trdnpr: u32, // bitfield width 1
// #endif
    pub s: cvmx_pci_bitfield_placeholder,
}

#[repr(C)] pub union cvmx_pci_cfg17 {
    pub 32: u32,
#[repr(C)] pub struct cvmx_pci_cfg17_s {
// #ifdef __BIG_ENDIAN_BITFIELD
    pub tscme: u32, // bitfield width 32
// #else
    pub tscme: u32, // bitfield width 32
// #endif
    pub s: cvmx_pci_bitfield_placeholder,
}

#[repr(C)] pub union cvmx_pci_cfg18 {
    pub 32: u32,
#[repr(C)] pub struct cvmx_pci_cfg18_s {
// #ifdef __BIG_ENDIAN_BITFIELD
    pub tdsrps: u32, // bitfield width 32
// #else
    pub tdsrps: u32, // bitfield width 32
// #endif
    pub s: cvmx_pci_bitfield_placeholder,
}

#[repr(C)] pub union cvmx_pci_cfg19 {
    pub 32: u32,
#[repr(C)] pub struct cvmx_pci_cfg19_s {
// #ifdef __BIG_ENDIAN_BITFIELD
    pub mrbcm: u32, // bitfield width 1
    pub mrbci: u32, // bitfield width 1
    pub mdwe: u32, // bitfield width 1
    pub mdre: u32, // bitfield width 1
    pub mdrimc: u32, // bitfield width 1
    pub mdrrmc: u32, // bitfield width 3
    pub tmes: u32, // bitfield width 8
    pub teci: u32, // bitfield width 1
    pub tmei: u32, // bitfield width 1
    pub tmse: u32, // bitfield width 1
    pub tmdpes: u32, // bitfield width 1
    pub tmapes: u32, // bitfield width 1
    pub reserved_9_10: u32, // bitfield width 2
    pub tibcd: u32, // bitfield width 1
    pub tibde: u32, // bitfield width 1
    pub reserved_6_6: u32, // bitfield width 1
    pub tidomc: u32, // bitfield width 1
    pub tdomc: u32, // bitfield width 5
// #else
    pub tdomc: u32, // bitfield width 5
    pub tidomc: u32, // bitfield width 1
    pub reserved_6_6: u32, // bitfield width 1
    pub tibde: u32, // bitfield width 1
    pub tibcd: u32, // bitfield width 1
    pub reserved_9_10: u32, // bitfield width 2
    pub tmapes: u32, // bitfield width 1
    pub tmdpes: u32, // bitfield width 1
    pub tmse: u32, // bitfield width 1
    pub tmei: u32, // bitfield width 1
    pub teci: u32, // bitfield width 1
    pub tmes: u32, // bitfield width 8
    pub mdrrmc: u32, // bitfield width 3
    pub mdrimc: u32, // bitfield width 1
    pub mdre: u32, // bitfield width 1
    pub mdwe: u32, // bitfield width 1
    pub mrbci: u32, // bitfield width 1
    pub mrbcm: u32, // bitfield width 1
// #endif
    pub s: cvmx_pci_bitfield_placeholder,
}

#[repr(C)] pub union cvmx_pci_cfg20 {
    pub 32: u32,
#[repr(C)] pub struct cvmx_pci_cfg20_s {
// #ifdef __BIG_ENDIAN_BITFIELD
    pub mdsp: u32, // bitfield width 32
// #else
    pub mdsp: u32, // bitfield width 32
// #endif
    pub s: cvmx_pci_bitfield_placeholder,
}

#[repr(C)] pub union cvmx_pci_cfg21 {
    pub 32: u32,
#[repr(C)] pub struct cvmx_pci_cfg21_s {
// #ifdef __BIG_ENDIAN_BITFIELD
    pub scmre: u32, // bitfield width 32
// #else
    pub scmre: u32, // bitfield width 32
// #endif
    pub s: cvmx_pci_bitfield_placeholder,
}

#[repr(C)] pub union cvmx_pci_cfg22 {
    pub 32: u32,
#[repr(C)] pub struct cvmx_pci_cfg22_s {
// #ifdef __BIG_ENDIAN_BITFIELD
    pub mac: u32, // bitfield width 7
    pub reserved_19_24: u32, // bitfield width 6
    pub flush: u32, // bitfield width 1
    pub mra: u32, // bitfield width 1
    pub mtta: u32, // bitfield width 1
    pub mrv: u32, // bitfield width 8
    pub mttv: u32, // bitfield width 8
// #else
    pub mttv: u32, // bitfield width 8
    pub mrv: u32, // bitfield width 8
    pub mtta: u32, // bitfield width 1
    pub mra: u32, // bitfield width 1
    pub flush: u32, // bitfield width 1
    pub reserved_19_24: u32, // bitfield width 6
    pub mac: u32, // bitfield width 7
// #endif
    pub s: cvmx_pci_bitfield_placeholder,
}

#[repr(C)] pub union cvmx_pci_cfg56 {
    pub 32: u32,
#[repr(C)] pub struct cvmx_pci_cfg56_s {
// #ifdef __BIG_ENDIAN_BITFIELD
    pub reserved_23_31: u32, // bitfield width 9
    pub most: u32, // bitfield width 3
    pub mmbc: u32, // bitfield width 2
    pub roe: u32, // bitfield width 1
    pub dpere: u32, // bitfield width 1
    pub ncp: u32, // bitfield width 8
    pub pxcid: u32, // bitfield width 8
// #else
    pub pxcid: u32, // bitfield width 8
    pub ncp: u32, // bitfield width 8
    pub dpere: u32, // bitfield width 1
    pub roe: u32, // bitfield width 1
    pub mmbc: u32, // bitfield width 2
    pub most: u32, // bitfield width 3
    pub reserved_23_31: u32, // bitfield width 9
// #endif
    pub s: cvmx_pci_bitfield_placeholder,
}

#[repr(C)] pub union cvmx_pci_cfg57 {
    pub 32: u32,
#[repr(C)] pub struct cvmx_pci_cfg57_s {
// #ifdef __BIG_ENDIAN_BITFIELD
    pub reserved_30_31: u32, // bitfield width 2
    pub scemr: u32, // bitfield width 1
    pub mcrsd: u32, // bitfield width 3
    pub mostd: u32, // bitfield width 3
    pub mmrbcd: u32, // bitfield width 2
    pub dc: u32, // bitfield width 1
    pub usc: u32, // bitfield width 1
    pub scd: u32, // bitfield width 1
    pub m133: u32, // bitfield width 1
    pub w64: u32, // bitfield width 1
    pub bn: u32, // bitfield width 8
    pub dn: u32, // bitfield width 5
    pub fn: u32, // bitfield width 3
// #else
    pub fn: u32, // bitfield width 3
    pub dn: u32, // bitfield width 5
    pub bn: u32, // bitfield width 8
    pub w64: u32, // bitfield width 1
    pub m133: u32, // bitfield width 1
    pub scd: u32, // bitfield width 1
    pub usc: u32, // bitfield width 1
    pub dc: u32, // bitfield width 1
    pub mmrbcd: u32, // bitfield width 2
    pub mostd: u32, // bitfield width 3
    pub mcrsd: u32, // bitfield width 3
    pub scemr: u32, // bitfield width 1
    pub reserved_30_31: u32, // bitfield width 2
// #endif
    pub s: cvmx_pci_bitfield_placeholder,
}

#[repr(C)] pub union cvmx_pci_cfg58 {
    pub 32: u32,
#[repr(C)] pub struct cvmx_pci_cfg58_s {
// #ifdef __BIG_ENDIAN_BITFIELD
    pub pmes: u32, // bitfield width 5
    pub d2s: u32, // bitfield width 1
    pub d1s: u32, // bitfield width 1
    pub auxc: u32, // bitfield width 3
    pub dsi: u32, // bitfield width 1
    pub reserved_20_20: u32, // bitfield width 1
    pub pmec: u32, // bitfield width 1
    pub pcimiv: u32, // bitfield width 3
    pub ncp: u32, // bitfield width 8
    pub pmcid: u32, // bitfield width 8
// #else
    pub pmcid: u32, // bitfield width 8
    pub ncp: u32, // bitfield width 8
    pub pcimiv: u32, // bitfield width 3
    pub pmec: u32, // bitfield width 1
    pub reserved_20_20: u32, // bitfield width 1
    pub dsi: u32, // bitfield width 1
    pub auxc: u32, // bitfield width 3
    pub d1s: u32, // bitfield width 1
    pub d2s: u32, // bitfield width 1
    pub pmes: u32, // bitfield width 5
// #endif
    pub s: cvmx_pci_bitfield_placeholder,
}

#[repr(C)] pub union cvmx_pci_cfg59 {
    pub 32: u32,
#[repr(C)] pub struct cvmx_pci_cfg59_s {
// #ifdef __BIG_ENDIAN_BITFIELD
    pub pmdia: u32, // bitfield width 8
    pub bpccen: u32, // bitfield width 1
    pub bd3h: u32, // bitfield width 1
    pub reserved_16_21: u32, // bitfield width 6
    pub pmess: u32, // bitfield width 1
    pub pmedsia: u32, // bitfield width 2
    pub pmds: u32, // bitfield width 4
    pub pmeens: u32, // bitfield width 1
    pub reserved_2_7: u32, // bitfield width 6
    pub ps: u32, // bitfield width 2
// #else
    pub ps: u32, // bitfield width 2
    pub reserved_2_7: u32, // bitfield width 6
    pub pmeens: u32, // bitfield width 1
    pub pmds: u32, // bitfield width 4
    pub pmedsia: u32, // bitfield width 2
    pub pmess: u32, // bitfield width 1
    pub reserved_16_21: u32, // bitfield width 6
    pub bd3h: u32, // bitfield width 1
    pub bpccen: u32, // bitfield width 1
    pub pmdia: u32, // bitfield width 8
// #endif
    pub s: cvmx_pci_bitfield_placeholder,
}

#[repr(C)] pub union cvmx_pci_cfg60 {
    pub 32: u32,
#[repr(C)] pub struct cvmx_pci_cfg60_s {
// #ifdef __BIG_ENDIAN_BITFIELD
    pub reserved_24_31: u32, // bitfield width 8
    pub m64: u32, // bitfield width 1
    pub mme: u32, // bitfield width 3
    pub mmc: u32, // bitfield width 3
    pub msien: u32, // bitfield width 1
    pub ncp: u32, // bitfield width 8
    pub msicid: u32, // bitfield width 8
// #else
    pub msicid: u32, // bitfield width 8
    pub ncp: u32, // bitfield width 8
    pub msien: u32, // bitfield width 1
    pub mmc: u32, // bitfield width 3
    pub mme: u32, // bitfield width 3
    pub m64: u32, // bitfield width 1
    pub reserved_24_31: u32, // bitfield width 8
// #endif
    pub s: cvmx_pci_bitfield_placeholder,
}

#[repr(C)] pub union cvmx_pci_cfg61 {
    pub 32: u32,
#[repr(C)] pub struct cvmx_pci_cfg61_s {
// #ifdef __BIG_ENDIAN_BITFIELD
    pub msi31t2: u32, // bitfield width 30
    pub reserved_0_1: u32, // bitfield width 2
// #else
    pub reserved_0_1: u32, // bitfield width 2
    pub msi31t2: u32, // bitfield width 30
// #endif
    pub s: cvmx_pci_bitfield_placeholder,
}

#[repr(C)] pub union cvmx_pci_cfg62 {
    pub 32: u32,
#[repr(C)] pub struct cvmx_pci_cfg62_s {
// #ifdef __BIG_ENDIAN_BITFIELD
    pub msi: u32, // bitfield width 32
// #else
    pub msi: u32, // bitfield width 32
// #endif
    pub s: cvmx_pci_bitfield_placeholder,
}

#[repr(C)] pub union cvmx_pci_cfg63 {
    pub 32: u32,
#[repr(C)] pub struct cvmx_pci_cfg63_s {
// #ifdef __BIG_ENDIAN_BITFIELD
    pub reserved_16_31: u32, // bitfield width 16
    pub msimd: u32, // bitfield width 16
// #else
    pub msimd: u32, // bitfield width 16
    pub reserved_16_31: u32, // bitfield width 16
// #endif
    pub s: cvmx_pci_bitfield_placeholder,
}

#[repr(C)] pub union cvmx_pci_cnt_reg {
    pub 64: u64,
#[repr(C)] pub struct cvmx_pci_cnt_reg_s {
// #ifdef __BIG_ENDIAN_BITFIELD
    pub reserved_38_63: u64, // bitfield width 26
    pub hm_pcix: u64, // bitfield width 1
    pub hm_speed: u64, // bitfield width 2
    pub ap_pcix: u64, // bitfield width 1
    pub ap_speed: u64, // bitfield width 2
    pub pcicnt: u64, // bitfield width 32
// #else
    pub pcicnt: u64, // bitfield width 32
    pub ap_speed: u64, // bitfield width 2
    pub ap_pcix: u64, // bitfield width 1
    pub hm_speed: u64, // bitfield width 2
    pub hm_pcix: u64, // bitfield width 1
    pub reserved_38_63: u64, // bitfield width 26
// #endif
    pub s: cvmx_pci_bitfield_placeholder,
}

#[repr(C)] pub union cvmx_pci_ctl_status_2 {
    pub 32: u32,
#[repr(C)] pub struct cvmx_pci_ctl_status_2_s {
// #ifdef __BIG_ENDIAN_BITFIELD
    pub reserved_29_31: u32, // bitfield width 3
    pub bb1_hole: u32, // bitfield width 3
    pub bb1_siz: u32, // bitfield width 1
    pub bb_ca: u32, // bitfield width 1
    pub bb_es: u32, // bitfield width 2
    pub bb1: u32, // bitfield width 1
    pub bb0: u32, // bitfield width 1
    pub erst_n: u32, // bitfield width 1
    pub bar2pres: u32, // bitfield width 1
    pub scmtyp: u32, // bitfield width 1
    pub scm: u32, // bitfield width 1
    pub en_wfilt: u32, // bitfield width 1
    pub reserved_14_14: u32, // bitfield width 1
    pub ap_pcix: u32, // bitfield width 1
    pub ap_64ad: u32, // bitfield width 1
    pub b12_bist: u32, // bitfield width 1
    pub pmo_amod: u32, // bitfield width 1
    pub pmo_fpc: u32, // bitfield width 3
    pub tsr_hwm: u32, // bitfield width 3
    pub bar2_enb: u32, // bitfield width 1
    pub bar2_esx: u32, // bitfield width 2
    pub bar2_cax: u32, // bitfield width 1
// #else
    pub bar2_cax: u32, // bitfield width 1
    pub bar2_esx: u32, // bitfield width 2
    pub bar2_enb: u32, // bitfield width 1
    pub tsr_hwm: u32, // bitfield width 3
    pub pmo_fpc: u32, // bitfield width 3
    pub pmo_amod: u32, // bitfield width 1
    pub b12_bist: u32, // bitfield width 1
    pub ap_64ad: u32, // bitfield width 1
    pub ap_pcix: u32, // bitfield width 1
    pub reserved_14_14: u32, // bitfield width 1
    pub en_wfilt: u32, // bitfield width 1
    pub scm: u32, // bitfield width 1
    pub scmtyp: u32, // bitfield width 1
    pub bar2pres: u32, // bitfield width 1
    pub erst_n: u32, // bitfield width 1
    pub bb0: u32, // bitfield width 1
    pub bb1: u32, // bitfield width 1
    pub bb_es: u32, // bitfield width 2
    pub bb_ca: u32, // bitfield width 1
    pub bb1_siz: u32, // bitfield width 1
    pub bb1_hole: u32, // bitfield width 3
    pub reserved_29_31: u32, // bitfield width 3
// #endif
    pub s: cvmx_pci_bitfield_placeholder,
#[repr(C)] pub struct cvmx_pci_ctl_status_2_cn31xx {
// #ifdef __BIG_ENDIAN_BITFIELD
    pub reserved_20_31: u32, // bitfield width 12
    pub erst_n: u32, // bitfield width 1
    pub bar2pres: u32, // bitfield width 1
    pub scmtyp: u32, // bitfield width 1
    pub scm: u32, // bitfield width 1
    pub en_wfilt: u32, // bitfield width 1
    pub reserved_14_14: u32, // bitfield width 1
    pub ap_pcix: u32, // bitfield width 1
    pub ap_64ad: u32, // bitfield width 1
    pub b12_bist: u32, // bitfield width 1
    pub pmo_amod: u32, // bitfield width 1
    pub pmo_fpc: u32, // bitfield width 3
    pub tsr_hwm: u32, // bitfield width 3
    pub bar2_enb: u32, // bitfield width 1
    pub bar2_esx: u32, // bitfield width 2
    pub bar2_cax: u32, // bitfield width 1
// #else
    pub bar2_cax: u32, // bitfield width 1
    pub bar2_esx: u32, // bitfield width 2
    pub bar2_enb: u32, // bitfield width 1
    pub tsr_hwm: u32, // bitfield width 3
    pub pmo_fpc: u32, // bitfield width 3
    pub pmo_amod: u32, // bitfield width 1
    pub b12_bist: u32, // bitfield width 1
    pub ap_64ad: u32, // bitfield width 1
    pub ap_pcix: u32, // bitfield width 1
    pub reserved_14_14: u32, // bitfield width 1
    pub en_wfilt: u32, // bitfield width 1
    pub scm: u32, // bitfield width 1
    pub scmtyp: u32, // bitfield width 1
    pub bar2pres: u32, // bitfield width 1
    pub erst_n: u32, // bitfield width 1
    pub reserved_20_31: u32, // bitfield width 12
// #endif
	} cn31xx;
}

#[repr(C)] pub union cvmx_pci_dbellx {
    pub 32: u32,
#[repr(C)] pub struct cvmx_pci_dbellx_s {
// #ifdef __BIG_ENDIAN_BITFIELD
    pub reserved_16_31: u32, // bitfield width 16
    pub inc_val: u32, // bitfield width 16
// #else
    pub inc_val: u32, // bitfield width 16
    pub reserved_16_31: u32, // bitfield width 16
// #endif
    pub s: cvmx_pci_bitfield_placeholder,
}

#[repr(C)] pub union cvmx_pci_dma_cntx {
    pub 32: u32,
#[repr(C)] pub struct cvmx_pci_dma_cntx_s {
// #ifdef __BIG_ENDIAN_BITFIELD
    pub dma_cnt: u32, // bitfield width 32
// #else
    pub dma_cnt: u32, // bitfield width 32
// #endif
    pub s: cvmx_pci_bitfield_placeholder,
}

#[repr(C)] pub union cvmx_pci_dma_int_levx {
    pub 32: u32,
#[repr(C)] pub struct cvmx_pci_dma_int_levx_s {
// #ifdef __BIG_ENDIAN_BITFIELD
    pub pkt_cnt: u32, // bitfield width 32
// #else
    pub pkt_cnt: u32, // bitfield width 32
// #endif
    pub s: cvmx_pci_bitfield_placeholder,
}

#[repr(C)] pub union cvmx_pci_dma_timex {
    pub 32: u32,
#[repr(C)] pub struct cvmx_pci_dma_timex_s {
// #ifdef __BIG_ENDIAN_BITFIELD
    pub dma_time: u32, // bitfield width 32
// #else
    pub dma_time: u32, // bitfield width 32
// #endif
    pub s: cvmx_pci_bitfield_placeholder,
}

#[repr(C)] pub union cvmx_pci_instr_countx {
    pub 32: u32,
#[repr(C)] pub struct cvmx_pci_instr_countx_s {
// #ifdef __BIG_ENDIAN_BITFIELD
    pub icnt: u32, // bitfield width 32
// #else
    pub icnt: u32, // bitfield width 32
// #endif
    pub s: cvmx_pci_bitfield_placeholder,
}

#[repr(C)] pub union cvmx_pci_int_enb {
    pub 64: u64,
#[repr(C)] pub struct cvmx_pci_int_enb_s {
// #ifdef __BIG_ENDIAN_BITFIELD
    pub reserved_34_63: u64, // bitfield width 30
    pub ill_rd: u64, // bitfield width 1
    pub ill_wr: u64, // bitfield width 1
    pub win_wr: u64, // bitfield width 1
    pub dma1_fi: u64, // bitfield width 1
    pub dma0_fi: u64, // bitfield width 1
    pub idtime1: u64, // bitfield width 1
    pub idtime0: u64, // bitfield width 1
    pub idcnt1: u64, // bitfield width 1
    pub idcnt0: u64, // bitfield width 1
    pub iptime3: u64, // bitfield width 1
    pub iptime2: u64, // bitfield width 1
    pub iptime1: u64, // bitfield width 1
    pub iptime0: u64, // bitfield width 1
    pub ipcnt3: u64, // bitfield width 1
    pub ipcnt2: u64, // bitfield width 1
    pub ipcnt1: u64, // bitfield width 1
    pub ipcnt0: u64, // bitfield width 1
    pub irsl_int: u64, // bitfield width 1
    pub ill_rrd: u64, // bitfield width 1
    pub ill_rwr: u64, // bitfield width 1
    pub idperr: u64, // bitfield width 1
    pub iaperr: u64, // bitfield width 1
    pub iserr: u64, // bitfield width 1
    pub itsr_abt: u64, // bitfield width 1
    pub imsc_msg: u64, // bitfield width 1
    pub imsi_mabt: u64, // bitfield width 1
    pub imsi_tabt: u64, // bitfield width 1
    pub imsi_per: u64, // bitfield width 1
    pub imr_tto: u64, // bitfield width 1
    pub imr_abt: u64, // bitfield width 1
    pub itr_abt: u64, // bitfield width 1
    pub imr_wtto: u64, // bitfield width 1
    pub imr_wabt: u64, // bitfield width 1
    pub itr_wabt: u64, // bitfield width 1
// #else
    pub itr_wabt: u64, // bitfield width 1
    pub imr_wabt: u64, // bitfield width 1
    pub imr_wtto: u64, // bitfield width 1
    pub itr_abt: u64, // bitfield width 1
    pub imr_abt: u64, // bitfield width 1
    pub imr_tto: u64, // bitfield width 1
    pub imsi_per: u64, // bitfield width 1
    pub imsi_tabt: u64, // bitfield width 1
    pub imsi_mabt: u64, // bitfield width 1
    pub imsc_msg: u64, // bitfield width 1
    pub itsr_abt: u64, // bitfield width 1
    pub iserr: u64, // bitfield width 1
    pub iaperr: u64, // bitfield width 1
    pub idperr: u64, // bitfield width 1
    pub ill_rwr: u64, // bitfield width 1
    pub ill_rrd: u64, // bitfield width 1
    pub irsl_int: u64, // bitfield width 1
    pub ipcnt0: u64, // bitfield width 1
    pub ipcnt1: u64, // bitfield width 1
    pub ipcnt2: u64, // bitfield width 1
    pub ipcnt3: u64, // bitfield width 1
    pub iptime0: u64, // bitfield width 1
    pub iptime1: u64, // bitfield width 1
    pub iptime2: u64, // bitfield width 1
    pub iptime3: u64, // bitfield width 1
    pub idcnt0: u64, // bitfield width 1
    pub idcnt1: u64, // bitfield width 1
    pub idtime0: u64, // bitfield width 1
    pub idtime1: u64, // bitfield width 1
    pub dma0_fi: u64, // bitfield width 1
    pub dma1_fi: u64, // bitfield width 1
    pub win_wr: u64, // bitfield width 1
    pub ill_wr: u64, // bitfield width 1
    pub ill_rd: u64, // bitfield width 1
    pub reserved_34_63: u64, // bitfield width 30
// #endif
    pub s: cvmx_pci_bitfield_placeholder,
#[repr(C)] pub struct cvmx_pci_int_enb_cn30xx {
// #ifdef __BIG_ENDIAN_BITFIELD
    pub reserved_34_63: u64, // bitfield width 30
    pub ill_rd: u64, // bitfield width 1
    pub ill_wr: u64, // bitfield width 1
    pub win_wr: u64, // bitfield width 1
    pub dma1_fi: u64, // bitfield width 1
    pub dma0_fi: u64, // bitfield width 1
    pub idtime1: u64, // bitfield width 1
    pub idtime0: u64, // bitfield width 1
    pub idcnt1: u64, // bitfield width 1
    pub idcnt0: u64, // bitfield width 1
    pub reserved_22_24: u64, // bitfield width 3
    pub iptime0: u64, // bitfield width 1
    pub reserved_18_20: u64, // bitfield width 3
    pub ipcnt0: u64, // bitfield width 1
    pub irsl_int: u64, // bitfield width 1
    pub ill_rrd: u64, // bitfield width 1
    pub ill_rwr: u64, // bitfield width 1
    pub idperr: u64, // bitfield width 1
    pub iaperr: u64, // bitfield width 1
    pub iserr: u64, // bitfield width 1
    pub itsr_abt: u64, // bitfield width 1
    pub imsc_msg: u64, // bitfield width 1
    pub imsi_mabt: u64, // bitfield width 1
    pub imsi_tabt: u64, // bitfield width 1
    pub imsi_per: u64, // bitfield width 1
    pub imr_tto: u64, // bitfield width 1
    pub imr_abt: u64, // bitfield width 1
    pub itr_abt: u64, // bitfield width 1
    pub imr_wtto: u64, // bitfield width 1
    pub imr_wabt: u64, // bitfield width 1
    pub itr_wabt: u64, // bitfield width 1
// #else
    pub itr_wabt: u64, // bitfield width 1
    pub imr_wabt: u64, // bitfield width 1
    pub imr_wtto: u64, // bitfield width 1
    pub itr_abt: u64, // bitfield width 1
    pub imr_abt: u64, // bitfield width 1
    pub imr_tto: u64, // bitfield width 1
    pub imsi_per: u64, // bitfield width 1
    pub imsi_tabt: u64, // bitfield width 1
    pub imsi_mabt: u64, // bitfield width 1
    pub imsc_msg: u64, // bitfield width 1
    pub itsr_abt: u64, // bitfield width 1
    pub iserr: u64, // bitfield width 1
    pub iaperr: u64, // bitfield width 1
    pub idperr: u64, // bitfield width 1
    pub ill_rwr: u64, // bitfield width 1
    pub ill_rrd: u64, // bitfield width 1
    pub irsl_int: u64, // bitfield width 1
    pub ipcnt0: u64, // bitfield width 1
    pub reserved_18_20: u64, // bitfield width 3
    pub iptime0: u64, // bitfield width 1
    pub reserved_22_24: u64, // bitfield width 3
    pub idcnt0: u64, // bitfield width 1
    pub idcnt1: u64, // bitfield width 1
    pub idtime0: u64, // bitfield width 1
    pub idtime1: u64, // bitfield width 1
    pub dma0_fi: u64, // bitfield width 1
    pub dma1_fi: u64, // bitfield width 1
    pub win_wr: u64, // bitfield width 1
    pub ill_wr: u64, // bitfield width 1
    pub ill_rd: u64, // bitfield width 1
    pub reserved_34_63: u64, // bitfield width 30
// #endif
	} cn30xx;
#[repr(C)] pub struct cvmx_pci_int_enb_cn31xx {
// #ifdef __BIG_ENDIAN_BITFIELD
    pub reserved_34_63: u64, // bitfield width 30
    pub ill_rd: u64, // bitfield width 1
    pub ill_wr: u64, // bitfield width 1
    pub win_wr: u64, // bitfield width 1
    pub dma1_fi: u64, // bitfield width 1
    pub dma0_fi: u64, // bitfield width 1
    pub idtime1: u64, // bitfield width 1
    pub idtime0: u64, // bitfield width 1
    pub idcnt1: u64, // bitfield width 1
    pub idcnt0: u64, // bitfield width 1
    pub reserved_23_24: u64, // bitfield width 2
    pub iptime1: u64, // bitfield width 1
    pub iptime0: u64, // bitfield width 1
    pub reserved_19_20: u64, // bitfield width 2
    pub ipcnt1: u64, // bitfield width 1
    pub ipcnt0: u64, // bitfield width 1
    pub irsl_int: u64, // bitfield width 1
    pub ill_rrd: u64, // bitfield width 1
    pub ill_rwr: u64, // bitfield width 1
    pub idperr: u64, // bitfield width 1
    pub iaperr: u64, // bitfield width 1
    pub iserr: u64, // bitfield width 1
    pub itsr_abt: u64, // bitfield width 1
    pub imsc_msg: u64, // bitfield width 1
    pub imsi_mabt: u64, // bitfield width 1
    pub imsi_tabt: u64, // bitfield width 1
    pub imsi_per: u64, // bitfield width 1
    pub imr_tto: u64, // bitfield width 1
    pub imr_abt: u64, // bitfield width 1
    pub itr_abt: u64, // bitfield width 1
    pub imr_wtto: u64, // bitfield width 1
    pub imr_wabt: u64, // bitfield width 1
    pub itr_wabt: u64, // bitfield width 1
// #else
    pub itr_wabt: u64, // bitfield width 1
    pub imr_wabt: u64, // bitfield width 1
    pub imr_wtto: u64, // bitfield width 1
    pub itr_abt: u64, // bitfield width 1
    pub imr_abt: u64, // bitfield width 1
    pub imr_tto: u64, // bitfield width 1
    pub imsi_per: u64, // bitfield width 1
    pub imsi_tabt: u64, // bitfield width 1
    pub imsi_mabt: u64, // bitfield width 1
    pub imsc_msg: u64, // bitfield width 1
    pub itsr_abt: u64, // bitfield width 1
    pub iserr: u64, // bitfield width 1
    pub iaperr: u64, // bitfield width 1
    pub idperr: u64, // bitfield width 1
    pub ill_rwr: u64, // bitfield width 1
    pub ill_rrd: u64, // bitfield width 1
    pub irsl_int: u64, // bitfield width 1
    pub ipcnt0: u64, // bitfield width 1
    pub ipcnt1: u64, // bitfield width 1
    pub reserved_19_20: u64, // bitfield width 2
    pub iptime0: u64, // bitfield width 1
    pub iptime1: u64, // bitfield width 1
    pub reserved_23_24: u64, // bitfield width 2
    pub idcnt0: u64, // bitfield width 1
    pub idcnt1: u64, // bitfield width 1
    pub idtime0: u64, // bitfield width 1
    pub idtime1: u64, // bitfield width 1
    pub dma0_fi: u64, // bitfield width 1
    pub dma1_fi: u64, // bitfield width 1
    pub win_wr: u64, // bitfield width 1
    pub ill_wr: u64, // bitfield width 1
    pub ill_rd: u64, // bitfield width 1
    pub reserved_34_63: u64, // bitfield width 30
// #endif
	} cn31xx;
}

#[repr(C)] pub union cvmx_pci_int_enb2 {
    pub 64: u64,
#[repr(C)] pub struct cvmx_pci_int_enb2_s {
// #ifdef __BIG_ENDIAN_BITFIELD
    pub reserved_34_63: u64, // bitfield width 30
    pub ill_rd: u64, // bitfield width 1
    pub ill_wr: u64, // bitfield width 1
    pub win_wr: u64, // bitfield width 1
    pub dma1_fi: u64, // bitfield width 1
    pub dma0_fi: u64, // bitfield width 1
    pub rdtime1: u64, // bitfield width 1
    pub rdtime0: u64, // bitfield width 1
    pub rdcnt1: u64, // bitfield width 1
    pub rdcnt0: u64, // bitfield width 1
    pub rptime3: u64, // bitfield width 1
    pub rptime2: u64, // bitfield width 1
    pub rptime1: u64, // bitfield width 1
    pub rptime0: u64, // bitfield width 1
    pub rpcnt3: u64, // bitfield width 1
    pub rpcnt2: u64, // bitfield width 1
    pub rpcnt1: u64, // bitfield width 1
    pub rpcnt0: u64, // bitfield width 1
    pub rrsl_int: u64, // bitfield width 1
    pub ill_rrd: u64, // bitfield width 1
    pub ill_rwr: u64, // bitfield width 1
    pub rdperr: u64, // bitfield width 1
    pub raperr: u64, // bitfield width 1
    pub rserr: u64, // bitfield width 1
    pub rtsr_abt: u64, // bitfield width 1
    pub rmsc_msg: u64, // bitfield width 1
    pub rmsi_mabt: u64, // bitfield width 1
    pub rmsi_tabt: u64, // bitfield width 1
    pub rmsi_per: u64, // bitfield width 1
    pub rmr_tto: u64, // bitfield width 1
    pub rmr_abt: u64, // bitfield width 1
    pub rtr_abt: u64, // bitfield width 1
    pub rmr_wtto: u64, // bitfield width 1
    pub rmr_wabt: u64, // bitfield width 1
    pub rtr_wabt: u64, // bitfield width 1
// #else
    pub rtr_wabt: u64, // bitfield width 1
    pub rmr_wabt: u64, // bitfield width 1
    pub rmr_wtto: u64, // bitfield width 1
    pub rtr_abt: u64, // bitfield width 1
    pub rmr_abt: u64, // bitfield width 1
    pub rmr_tto: u64, // bitfield width 1
    pub rmsi_per: u64, // bitfield width 1
    pub rmsi_tabt: u64, // bitfield width 1
    pub rmsi_mabt: u64, // bitfield width 1
    pub rmsc_msg: u64, // bitfield width 1
    pub rtsr_abt: u64, // bitfield width 1
    pub rserr: u64, // bitfield width 1
    pub raperr: u64, // bitfield width 1
    pub rdperr: u64, // bitfield width 1
    pub ill_rwr: u64, // bitfield width 1
    pub ill_rrd: u64, // bitfield width 1
    pub rrsl_int: u64, // bitfield width 1
    pub rpcnt0: u64, // bitfield width 1
    pub rpcnt1: u64, // bitfield width 1
    pub rpcnt2: u64, // bitfield width 1
    pub rpcnt3: u64, // bitfield width 1
    pub rptime0: u64, // bitfield width 1
    pub rptime1: u64, // bitfield width 1
    pub rptime2: u64, // bitfield width 1
    pub rptime3: u64, // bitfield width 1
    pub rdcnt0: u64, // bitfield width 1
    pub rdcnt1: u64, // bitfield width 1
    pub rdtime0: u64, // bitfield width 1
    pub rdtime1: u64, // bitfield width 1
    pub dma0_fi: u64, // bitfield width 1
    pub dma1_fi: u64, // bitfield width 1
    pub win_wr: u64, // bitfield width 1
    pub ill_wr: u64, // bitfield width 1
    pub ill_rd: u64, // bitfield width 1
    pub reserved_34_63: u64, // bitfield width 30
// #endif
    pub s: cvmx_pci_bitfield_placeholder,
#[repr(C)] pub struct cvmx_pci_int_enb2_cn30xx {
// #ifdef __BIG_ENDIAN_BITFIELD
    pub reserved_34_63: u64, // bitfield width 30
    pub ill_rd: u64, // bitfield width 1
    pub ill_wr: u64, // bitfield width 1
    pub win_wr: u64, // bitfield width 1
    pub dma1_fi: u64, // bitfield width 1
    pub dma0_fi: u64, // bitfield width 1
    pub rdtime1: u64, // bitfield width 1
    pub rdtime0: u64, // bitfield width 1
    pub rdcnt1: u64, // bitfield width 1
    pub rdcnt0: u64, // bitfield width 1
    pub reserved_22_24: u64, // bitfield width 3
    pub rptime0: u64, // bitfield width 1
    pub reserved_18_20: u64, // bitfield width 3
    pub rpcnt0: u64, // bitfield width 1
    pub rrsl_int: u64, // bitfield width 1
    pub ill_rrd: u64, // bitfield width 1
    pub ill_rwr: u64, // bitfield width 1
    pub rdperr: u64, // bitfield width 1
    pub raperr: u64, // bitfield width 1
    pub rserr: u64, // bitfield width 1
    pub rtsr_abt: u64, // bitfield width 1
    pub rmsc_msg: u64, // bitfield width 1
    pub rmsi_mabt: u64, // bitfield width 1
    pub rmsi_tabt: u64, // bitfield width 1
    pub rmsi_per: u64, // bitfield width 1
    pub rmr_tto: u64, // bitfield width 1
    pub rmr_abt: u64, // bitfield width 1
    pub rtr_abt: u64, // bitfield width 1
    pub rmr_wtto: u64, // bitfield width 1
    pub rmr_wabt: u64, // bitfield width 1
    pub rtr_wabt: u64, // bitfield width 1
// #else
    pub rtr_wabt: u64, // bitfield width 1
    pub rmr_wabt: u64, // bitfield width 1
    pub rmr_wtto: u64, // bitfield width 1
    pub rtr_abt: u64, // bitfield width 1
    pub rmr_abt: u64, // bitfield width 1
    pub rmr_tto: u64, // bitfield width 1
    pub rmsi_per: u64, // bitfield width 1
    pub rmsi_tabt: u64, // bitfield width 1
    pub rmsi_mabt: u64, // bitfield width 1
    pub rmsc_msg: u64, // bitfield width 1
    pub rtsr_abt: u64, // bitfield width 1
    pub rserr: u64, // bitfield width 1
    pub raperr: u64, // bitfield width 1
    pub rdperr: u64, // bitfield width 1
    pub ill_rwr: u64, // bitfield width 1
    pub ill_rrd: u64, // bitfield width 1
    pub rrsl_int: u64, // bitfield width 1
    pub rpcnt0: u64, // bitfield width 1
    pub reserved_18_20: u64, // bitfield width 3
    pub rptime0: u64, // bitfield width 1
    pub reserved_22_24: u64, // bitfield width 3
    pub rdcnt0: u64, // bitfield width 1
    pub rdcnt1: u64, // bitfield width 1
    pub rdtime0: u64, // bitfield width 1
    pub rdtime1: u64, // bitfield width 1
    pub dma0_fi: u64, // bitfield width 1
    pub dma1_fi: u64, // bitfield width 1
    pub win_wr: u64, // bitfield width 1
    pub ill_wr: u64, // bitfield width 1
    pub ill_rd: u64, // bitfield width 1
    pub reserved_34_63: u64, // bitfield width 30
// #endif
	} cn30xx;
#[repr(C)] pub struct cvmx_pci_int_enb2_cn31xx {
// #ifdef __BIG_ENDIAN_BITFIELD
    pub reserved_34_63: u64, // bitfield width 30
    pub ill_rd: u64, // bitfield width 1
    pub ill_wr: u64, // bitfield width 1
    pub win_wr: u64, // bitfield width 1
    pub dma1_fi: u64, // bitfield width 1
    pub dma0_fi: u64, // bitfield width 1
    pub rdtime1: u64, // bitfield width 1
    pub rdtime0: u64, // bitfield width 1
    pub rdcnt1: u64, // bitfield width 1
    pub rdcnt0: u64, // bitfield width 1
    pub reserved_23_24: u64, // bitfield width 2
    pub rptime1: u64, // bitfield width 1
    pub rptime0: u64, // bitfield width 1
    pub reserved_19_20: u64, // bitfield width 2
    pub rpcnt1: u64, // bitfield width 1
    pub rpcnt0: u64, // bitfield width 1
    pub rrsl_int: u64, // bitfield width 1
    pub ill_rrd: u64, // bitfield width 1
    pub ill_rwr: u64, // bitfield width 1
    pub rdperr: u64, // bitfield width 1
    pub raperr: u64, // bitfield width 1
    pub rserr: u64, // bitfield width 1
    pub rtsr_abt: u64, // bitfield width 1
    pub rmsc_msg: u64, // bitfield width 1
    pub rmsi_mabt: u64, // bitfield width 1
    pub rmsi_tabt: u64, // bitfield width 1
    pub rmsi_per: u64, // bitfield width 1
    pub rmr_tto: u64, // bitfield width 1
    pub rmr_abt: u64, // bitfield width 1
    pub rtr_abt: u64, // bitfield width 1
    pub rmr_wtto: u64, // bitfield width 1
    pub rmr_wabt: u64, // bitfield width 1
    pub rtr_wabt: u64, // bitfield width 1
// #else
    pub rtr_wabt: u64, // bitfield width 1
    pub rmr_wabt: u64, // bitfield width 1
    pub rmr_wtto: u64, // bitfield width 1
    pub rtr_abt: u64, // bitfield width 1
    pub rmr_abt: u64, // bitfield width 1
    pub rmr_tto: u64, // bitfield width 1
    pub rmsi_per: u64, // bitfield width 1
    pub rmsi_tabt: u64, // bitfield width 1
    pub rmsi_mabt: u64, // bitfield width 1
    pub rmsc_msg: u64, // bitfield width 1
    pub rtsr_abt: u64, // bitfield width 1
    pub rserr: u64, // bitfield width 1
    pub raperr: u64, // bitfield width 1
    pub rdperr: u64, // bitfield width 1
    pub ill_rwr: u64, // bitfield width 1
    pub ill_rrd: u64, // bitfield width 1
    pub rrsl_int: u64, // bitfield width 1
    pub rpcnt0: u64, // bitfield width 1
    pub rpcnt1: u64, // bitfield width 1
    pub reserved_19_20: u64, // bitfield width 2
    pub rptime0: u64, // bitfield width 1
    pub rptime1: u64, // bitfield width 1
    pub reserved_23_24: u64, // bitfield width 2
    pub rdcnt0: u64, // bitfield width 1
    pub rdcnt1: u64, // bitfield width 1
    pub rdtime0: u64, // bitfield width 1
    pub rdtime1: u64, // bitfield width 1
    pub dma0_fi: u64, // bitfield width 1
    pub dma1_fi: u64, // bitfield width 1
    pub win_wr: u64, // bitfield width 1
    pub ill_wr: u64, // bitfield width 1
    pub ill_rd: u64, // bitfield width 1
    pub reserved_34_63: u64, // bitfield width 30
// #endif
	} cn31xx;
}

#[repr(C)] pub union cvmx_pci_int_sum {
    pub 64: u64,
#[repr(C)] pub struct cvmx_pci_int_sum_s {
// #ifdef __BIG_ENDIAN_BITFIELD
    pub reserved_34_63: u64, // bitfield width 30
    pub ill_rd: u64, // bitfield width 1
    pub ill_wr: u64, // bitfield width 1
    pub win_wr: u64, // bitfield width 1
    pub dma1_fi: u64, // bitfield width 1
    pub dma0_fi: u64, // bitfield width 1
    pub dtime1: u64, // bitfield width 1
    pub dtime0: u64, // bitfield width 1
    pub dcnt1: u64, // bitfield width 1
    pub dcnt0: u64, // bitfield width 1
    pub ptime3: u64, // bitfield width 1
    pub ptime2: u64, // bitfield width 1
    pub ptime1: u64, // bitfield width 1
    pub ptime0: u64, // bitfield width 1
    pub pcnt3: u64, // bitfield width 1
    pub pcnt2: u64, // bitfield width 1
    pub pcnt1: u64, // bitfield width 1
    pub pcnt0: u64, // bitfield width 1
    pub rsl_int: u64, // bitfield width 1
    pub ill_rrd: u64, // bitfield width 1
    pub ill_rwr: u64, // bitfield width 1
    pub dperr: u64, // bitfield width 1
    pub aperr: u64, // bitfield width 1
    pub serr: u64, // bitfield width 1
    pub tsr_abt: u64, // bitfield width 1
    pub msc_msg: u64, // bitfield width 1
    pub msi_mabt: u64, // bitfield width 1
    pub msi_tabt: u64, // bitfield width 1
    pub msi_per: u64, // bitfield width 1
    pub mr_tto: u64, // bitfield width 1
    pub mr_abt: u64, // bitfield width 1
    pub tr_abt: u64, // bitfield width 1
    pub mr_wtto: u64, // bitfield width 1
    pub mr_wabt: u64, // bitfield width 1
    pub tr_wabt: u64, // bitfield width 1
// #else
    pub tr_wabt: u64, // bitfield width 1
    pub mr_wabt: u64, // bitfield width 1
    pub mr_wtto: u64, // bitfield width 1
    pub tr_abt: u64, // bitfield width 1
    pub mr_abt: u64, // bitfield width 1
    pub mr_tto: u64, // bitfield width 1
    pub msi_per: u64, // bitfield width 1
    pub msi_tabt: u64, // bitfield width 1
    pub msi_mabt: u64, // bitfield width 1
    pub msc_msg: u64, // bitfield width 1
    pub tsr_abt: u64, // bitfield width 1
    pub serr: u64, // bitfield width 1
    pub aperr: u64, // bitfield width 1
    pub dperr: u64, // bitfield width 1
    pub ill_rwr: u64, // bitfield width 1
    pub ill_rrd: u64, // bitfield width 1
    pub rsl_int: u64, // bitfield width 1
    pub pcnt0: u64, // bitfield width 1
    pub pcnt1: u64, // bitfield width 1
    pub pcnt2: u64, // bitfield width 1
    pub pcnt3: u64, // bitfield width 1
    pub ptime0: u64, // bitfield width 1
    pub ptime1: u64, // bitfield width 1
    pub ptime2: u64, // bitfield width 1
    pub ptime3: u64, // bitfield width 1
    pub dcnt0: u64, // bitfield width 1
    pub dcnt1: u64, // bitfield width 1
    pub dtime0: u64, // bitfield width 1
    pub dtime1: u64, // bitfield width 1
    pub dma0_fi: u64, // bitfield width 1
    pub dma1_fi: u64, // bitfield width 1
    pub win_wr: u64, // bitfield width 1
    pub ill_wr: u64, // bitfield width 1
    pub ill_rd: u64, // bitfield width 1
    pub reserved_34_63: u64, // bitfield width 30
// #endif
    pub s: cvmx_pci_bitfield_placeholder,
#[repr(C)] pub struct cvmx_pci_int_sum_cn30xx {
// #ifdef __BIG_ENDIAN_BITFIELD
    pub reserved_34_63: u64, // bitfield width 30
    pub ill_rd: u64, // bitfield width 1
    pub ill_wr: u64, // bitfield width 1
    pub win_wr: u64, // bitfield width 1
    pub dma1_fi: u64, // bitfield width 1
    pub dma0_fi: u64, // bitfield width 1
    pub dtime1: u64, // bitfield width 1
    pub dtime0: u64, // bitfield width 1
    pub dcnt1: u64, // bitfield width 1
    pub dcnt0: u64, // bitfield width 1
    pub reserved_22_24: u64, // bitfield width 3
    pub ptime0: u64, // bitfield width 1
    pub reserved_18_20: u64, // bitfield width 3
    pub pcnt0: u64, // bitfield width 1
    pub rsl_int: u64, // bitfield width 1
    pub ill_rrd: u64, // bitfield width 1
    pub ill_rwr: u64, // bitfield width 1
    pub dperr: u64, // bitfield width 1
    pub aperr: u64, // bitfield width 1
    pub serr: u64, // bitfield width 1
    pub tsr_abt: u64, // bitfield width 1
    pub msc_msg: u64, // bitfield width 1
    pub msi_mabt: u64, // bitfield width 1
    pub msi_tabt: u64, // bitfield width 1
    pub msi_per: u64, // bitfield width 1
    pub mr_tto: u64, // bitfield width 1
    pub mr_abt: u64, // bitfield width 1
    pub tr_abt: u64, // bitfield width 1
    pub mr_wtto: u64, // bitfield width 1
    pub mr_wabt: u64, // bitfield width 1
    pub tr_wabt: u64, // bitfield width 1
// #else
    pub tr_wabt: u64, // bitfield width 1
    pub mr_wabt: u64, // bitfield width 1
    pub mr_wtto: u64, // bitfield width 1
    pub tr_abt: u64, // bitfield width 1
    pub mr_abt: u64, // bitfield width 1
    pub mr_tto: u64, // bitfield width 1
    pub msi_per: u64, // bitfield width 1
    pub msi_tabt: u64, // bitfield width 1
    pub msi_mabt: u64, // bitfield width 1
    pub msc_msg: u64, // bitfield width 1
    pub tsr_abt: u64, // bitfield width 1
    pub serr: u64, // bitfield width 1
    pub aperr: u64, // bitfield width 1
    pub dperr: u64, // bitfield width 1
    pub ill_rwr: u64, // bitfield width 1
    pub ill_rrd: u64, // bitfield width 1
    pub rsl_int: u64, // bitfield width 1
    pub pcnt0: u64, // bitfield width 1
    pub reserved_18_20: u64, // bitfield width 3
    pub ptime0: u64, // bitfield width 1
    pub reserved_22_24: u64, // bitfield width 3
    pub dcnt0: u64, // bitfield width 1
    pub dcnt1: u64, // bitfield width 1
    pub dtime0: u64, // bitfield width 1
    pub dtime1: u64, // bitfield width 1
    pub dma0_fi: u64, // bitfield width 1
    pub dma1_fi: u64, // bitfield width 1
    pub win_wr: u64, // bitfield width 1
    pub ill_wr: u64, // bitfield width 1
    pub ill_rd: u64, // bitfield width 1
    pub reserved_34_63: u64, // bitfield width 30
// #endif
	} cn30xx;
#[repr(C)] pub struct cvmx_pci_int_sum_cn31xx {
// #ifdef __BIG_ENDIAN_BITFIELD
    pub reserved_34_63: u64, // bitfield width 30
    pub ill_rd: u64, // bitfield width 1
    pub ill_wr: u64, // bitfield width 1
    pub win_wr: u64, // bitfield width 1
    pub dma1_fi: u64, // bitfield width 1
    pub dma0_fi: u64, // bitfield width 1
    pub dtime1: u64, // bitfield width 1
    pub dtime0: u64, // bitfield width 1
    pub dcnt1: u64, // bitfield width 1
    pub dcnt0: u64, // bitfield width 1
    pub reserved_23_24: u64, // bitfield width 2
    pub ptime1: u64, // bitfield width 1
    pub ptime0: u64, // bitfield width 1
    pub reserved_19_20: u64, // bitfield width 2
    pub pcnt1: u64, // bitfield width 1
    pub pcnt0: u64, // bitfield width 1
    pub rsl_int: u64, // bitfield width 1
    pub ill_rrd: u64, // bitfield width 1
    pub ill_rwr: u64, // bitfield width 1
    pub dperr: u64, // bitfield width 1
    pub aperr: u64, // bitfield width 1
    pub serr: u64, // bitfield width 1
    pub tsr_abt: u64, // bitfield width 1
    pub msc_msg: u64, // bitfield width 1
    pub msi_mabt: u64, // bitfield width 1
    pub msi_tabt: u64, // bitfield width 1
    pub msi_per: u64, // bitfield width 1
    pub mr_tto: u64, // bitfield width 1
    pub mr_abt: u64, // bitfield width 1
    pub tr_abt: u64, // bitfield width 1
    pub mr_wtto: u64, // bitfield width 1
    pub mr_wabt: u64, // bitfield width 1
    pub tr_wabt: u64, // bitfield width 1
// #else
    pub tr_wabt: u64, // bitfield width 1
    pub mr_wabt: u64, // bitfield width 1
    pub mr_wtto: u64, // bitfield width 1
    pub tr_abt: u64, // bitfield width 1
    pub mr_abt: u64, // bitfield width 1
    pub mr_tto: u64, // bitfield width 1
    pub msi_per: u64, // bitfield width 1
    pub msi_tabt: u64, // bitfield width 1
    pub msi_mabt: u64, // bitfield width 1
    pub msc_msg: u64, // bitfield width 1
    pub tsr_abt: u64, // bitfield width 1
    pub serr: u64, // bitfield width 1
    pub aperr: u64, // bitfield width 1
    pub dperr: u64, // bitfield width 1
    pub ill_rwr: u64, // bitfield width 1
    pub ill_rrd: u64, // bitfield width 1
    pub rsl_int: u64, // bitfield width 1
    pub pcnt0: u64, // bitfield width 1
    pub pcnt1: u64, // bitfield width 1
    pub reserved_19_20: u64, // bitfield width 2
    pub ptime0: u64, // bitfield width 1
    pub ptime1: u64, // bitfield width 1
    pub reserved_23_24: u64, // bitfield width 2
    pub dcnt0: u64, // bitfield width 1
    pub dcnt1: u64, // bitfield width 1
    pub dtime0: u64, // bitfield width 1
    pub dtime1: u64, // bitfield width 1
    pub dma0_fi: u64, // bitfield width 1
    pub dma1_fi: u64, // bitfield width 1
    pub win_wr: u64, // bitfield width 1
    pub ill_wr: u64, // bitfield width 1
    pub ill_rd: u64, // bitfield width 1
    pub reserved_34_63: u64, // bitfield width 30
// #endif
	} cn31xx;
}

#[repr(C)] pub union cvmx_pci_int_sum2 {
    pub 64: u64,
#[repr(C)] pub struct cvmx_pci_int_sum2_s {
// #ifdef __BIG_ENDIAN_BITFIELD
    pub reserved_34_63: u64, // bitfield width 30
    pub ill_rd: u64, // bitfield width 1
    pub ill_wr: u64, // bitfield width 1
    pub win_wr: u64, // bitfield width 1
    pub dma1_fi: u64, // bitfield width 1
    pub dma0_fi: u64, // bitfield width 1
    pub dtime1: u64, // bitfield width 1
    pub dtime0: u64, // bitfield width 1
    pub dcnt1: u64, // bitfield width 1
    pub dcnt0: u64, // bitfield width 1
    pub ptime3: u64, // bitfield width 1
    pub ptime2: u64, // bitfield width 1
    pub ptime1: u64, // bitfield width 1
    pub ptime0: u64, // bitfield width 1
    pub pcnt3: u64, // bitfield width 1
    pub pcnt2: u64, // bitfield width 1
    pub pcnt1: u64, // bitfield width 1
    pub pcnt0: u64, // bitfield width 1
    pub rsl_int: u64, // bitfield width 1
    pub ill_rrd: u64, // bitfield width 1
    pub ill_rwr: u64, // bitfield width 1
    pub dperr: u64, // bitfield width 1
    pub aperr: u64, // bitfield width 1
    pub serr: u64, // bitfield width 1
    pub tsr_abt: u64, // bitfield width 1
    pub msc_msg: u64, // bitfield width 1
    pub msi_mabt: u64, // bitfield width 1
    pub msi_tabt: u64, // bitfield width 1
    pub msi_per: u64, // bitfield width 1
    pub mr_tto: u64, // bitfield width 1
    pub mr_abt: u64, // bitfield width 1
    pub tr_abt: u64, // bitfield width 1
    pub mr_wtto: u64, // bitfield width 1
    pub mr_wabt: u64, // bitfield width 1
    pub tr_wabt: u64, // bitfield width 1
// #else
    pub tr_wabt: u64, // bitfield width 1
    pub mr_wabt: u64, // bitfield width 1
    pub mr_wtto: u64, // bitfield width 1
    pub tr_abt: u64, // bitfield width 1
    pub mr_abt: u64, // bitfield width 1
    pub mr_tto: u64, // bitfield width 1
    pub msi_per: u64, // bitfield width 1
    pub msi_tabt: u64, // bitfield width 1
    pub msi_mabt: u64, // bitfield width 1
    pub msc_msg: u64, // bitfield width 1
    pub tsr_abt: u64, // bitfield width 1
    pub serr: u64, // bitfield width 1
    pub aperr: u64, // bitfield width 1
    pub dperr: u64, // bitfield width 1
    pub ill_rwr: u64, // bitfield width 1
    pub ill_rrd: u64, // bitfield width 1
    pub rsl_int: u64, // bitfield width 1
    pub pcnt0: u64, // bitfield width 1
    pub pcnt1: u64, // bitfield width 1
    pub pcnt2: u64, // bitfield width 1
    pub pcnt3: u64, // bitfield width 1
    pub ptime0: u64, // bitfield width 1
    pub ptime1: u64, // bitfield width 1
    pub ptime2: u64, // bitfield width 1
    pub ptime3: u64, // bitfield width 1
    pub dcnt0: u64, // bitfield width 1
    pub dcnt1: u64, // bitfield width 1
    pub dtime0: u64, // bitfield width 1
    pub dtime1: u64, // bitfield width 1
    pub dma0_fi: u64, // bitfield width 1
    pub dma1_fi: u64, // bitfield width 1
    pub win_wr: u64, // bitfield width 1
    pub ill_wr: u64, // bitfield width 1
    pub ill_rd: u64, // bitfield width 1
    pub reserved_34_63: u64, // bitfield width 30
// #endif
    pub s: cvmx_pci_bitfield_placeholder,
#[repr(C)] pub struct cvmx_pci_int_sum2_cn30xx {
// #ifdef __BIG_ENDIAN_BITFIELD
    pub reserved_34_63: u64, // bitfield width 30
    pub ill_rd: u64, // bitfield width 1
    pub ill_wr: u64, // bitfield width 1
    pub win_wr: u64, // bitfield width 1
    pub dma1_fi: u64, // bitfield width 1
    pub dma0_fi: u64, // bitfield width 1
    pub dtime1: u64, // bitfield width 1
    pub dtime0: u64, // bitfield width 1
    pub dcnt1: u64, // bitfield width 1
    pub dcnt0: u64, // bitfield width 1
    pub reserved_22_24: u64, // bitfield width 3
    pub ptime0: u64, // bitfield width 1
    pub reserved_18_20: u64, // bitfield width 3
    pub pcnt0: u64, // bitfield width 1
    pub rsl_int: u64, // bitfield width 1
    pub ill_rrd: u64, // bitfield width 1
    pub ill_rwr: u64, // bitfield width 1
    pub dperr: u64, // bitfield width 1
    pub aperr: u64, // bitfield width 1
    pub serr: u64, // bitfield width 1
    pub tsr_abt: u64, // bitfield width 1
    pub msc_msg: u64, // bitfield width 1
    pub msi_mabt: u64, // bitfield width 1
    pub msi_tabt: u64, // bitfield width 1
    pub msi_per: u64, // bitfield width 1
    pub mr_tto: u64, // bitfield width 1
    pub mr_abt: u64, // bitfield width 1
    pub tr_abt: u64, // bitfield width 1
    pub mr_wtto: u64, // bitfield width 1
    pub mr_wabt: u64, // bitfield width 1
    pub tr_wabt: u64, // bitfield width 1
// #else
    pub tr_wabt: u64, // bitfield width 1
    pub mr_wabt: u64, // bitfield width 1
    pub mr_wtto: u64, // bitfield width 1
    pub tr_abt: u64, // bitfield width 1
    pub mr_abt: u64, // bitfield width 1
    pub mr_tto: u64, // bitfield width 1
    pub msi_per: u64, // bitfield width 1
    pub msi_tabt: u64, // bitfield width 1
    pub msi_mabt: u64, // bitfield width 1
    pub msc_msg: u64, // bitfield width 1
    pub tsr_abt: u64, // bitfield width 1
    pub serr: u64, // bitfield width 1
    pub aperr: u64, // bitfield width 1
    pub dperr: u64, // bitfield width 1
    pub ill_rwr: u64, // bitfield width 1
    pub ill_rrd: u64, // bitfield width 1
    pub rsl_int: u64, // bitfield width 1
    pub pcnt0: u64, // bitfield width 1
    pub reserved_18_20: u64, // bitfield width 3
    pub ptime0: u64, // bitfield width 1
    pub reserved_22_24: u64, // bitfield width 3
    pub dcnt0: u64, // bitfield width 1
    pub dcnt1: u64, // bitfield width 1
    pub dtime0: u64, // bitfield width 1
    pub dtime1: u64, // bitfield width 1
    pub dma0_fi: u64, // bitfield width 1
    pub dma1_fi: u64, // bitfield width 1
    pub win_wr: u64, // bitfield width 1
    pub ill_wr: u64, // bitfield width 1
    pub ill_rd: u64, // bitfield width 1
    pub reserved_34_63: u64, // bitfield width 30
// #endif
	} cn30xx;
#[repr(C)] pub struct cvmx_pci_int_sum2_cn31xx {
// #ifdef __BIG_ENDIAN_BITFIELD
    pub reserved_34_63: u64, // bitfield width 30
    pub ill_rd: u64, // bitfield width 1
    pub ill_wr: u64, // bitfield width 1
    pub win_wr: u64, // bitfield width 1
    pub dma1_fi: u64, // bitfield width 1
    pub dma0_fi: u64, // bitfield width 1
    pub dtime1: u64, // bitfield width 1
    pub dtime0: u64, // bitfield width 1
    pub dcnt1: u64, // bitfield width 1
    pub dcnt0: u64, // bitfield width 1
    pub reserved_23_24: u64, // bitfield width 2
    pub ptime1: u64, // bitfield width 1
    pub ptime0: u64, // bitfield width 1
    pub reserved_19_20: u64, // bitfield width 2
    pub pcnt1: u64, // bitfield width 1
    pub pcnt0: u64, // bitfield width 1
    pub rsl_int: u64, // bitfield width 1
    pub ill_rrd: u64, // bitfield width 1
    pub ill_rwr: u64, // bitfield width 1
    pub dperr: u64, // bitfield width 1
    pub aperr: u64, // bitfield width 1
    pub serr: u64, // bitfield width 1
    pub tsr_abt: u64, // bitfield width 1
    pub msc_msg: u64, // bitfield width 1
    pub msi_mabt: u64, // bitfield width 1
    pub msi_tabt: u64, // bitfield width 1
    pub msi_per: u64, // bitfield width 1
    pub mr_tto: u64, // bitfield width 1
    pub mr_abt: u64, // bitfield width 1
    pub tr_abt: u64, // bitfield width 1
    pub mr_wtto: u64, // bitfield width 1
    pub mr_wabt: u64, // bitfield width 1
    pub tr_wabt: u64, // bitfield width 1
// #else
    pub tr_wabt: u64, // bitfield width 1
    pub mr_wabt: u64, // bitfield width 1
    pub mr_wtto: u64, // bitfield width 1
    pub tr_abt: u64, // bitfield width 1
    pub mr_abt: u64, // bitfield width 1
    pub mr_tto: u64, // bitfield width 1
    pub msi_per: u64, // bitfield width 1
    pub msi_tabt: u64, // bitfield width 1
    pub msi_mabt: u64, // bitfield width 1
    pub msc_msg: u64, // bitfield width 1
    pub tsr_abt: u64, // bitfield width 1
    pub serr: u64, // bitfield width 1
    pub aperr: u64, // bitfield width 1
    pub dperr: u64, // bitfield width 1
    pub ill_rwr: u64, // bitfield width 1
    pub ill_rrd: u64, // bitfield width 1
    pub rsl_int: u64, // bitfield width 1
    pub pcnt0: u64, // bitfield width 1
    pub pcnt1: u64, // bitfield width 1
    pub reserved_19_20: u64, // bitfield width 2
    pub ptime0: u64, // bitfield width 1
    pub ptime1: u64, // bitfield width 1
    pub reserved_23_24: u64, // bitfield width 2
    pub dcnt0: u64, // bitfield width 1
    pub dcnt1: u64, // bitfield width 1
    pub dtime0: u64, // bitfield width 1
    pub dtime1: u64, // bitfield width 1
    pub dma0_fi: u64, // bitfield width 1
    pub dma1_fi: u64, // bitfield width 1
    pub win_wr: u64, // bitfield width 1
    pub ill_wr: u64, // bitfield width 1
    pub ill_rd: u64, // bitfield width 1
    pub reserved_34_63: u64, // bitfield width 30
// #endif
	} cn31xx;
}

#[repr(C)] pub union cvmx_pci_msi_rcv {
    pub 32: u32,
#[repr(C)] pub struct cvmx_pci_msi_rcv_s {
// #ifdef __BIG_ENDIAN_BITFIELD
    pub reserved_6_31: u32, // bitfield width 26
    pub intr: u32, // bitfield width 6
// #else
    pub intr: u32, // bitfield width 6
    pub reserved_6_31: u32, // bitfield width 26
// #endif
    pub s: cvmx_pci_bitfield_placeholder,
}

#[repr(C)] pub union cvmx_pci_pkt_creditsx {
    pub 32: u32,
#[repr(C)] pub struct cvmx_pci_pkt_creditsx_s {
// #ifdef __BIG_ENDIAN_BITFIELD
    pub pkt_cnt: u32, // bitfield width 16
    pub ptr_cnt: u32, // bitfield width 16
// #else
    pub ptr_cnt: u32, // bitfield width 16
    pub pkt_cnt: u32, // bitfield width 16
// #endif
    pub s: cvmx_pci_bitfield_placeholder,
}

#[repr(C)] pub union cvmx_pci_pkts_sentx {
    pub 32: u32,
#[repr(C)] pub struct cvmx_pci_pkts_sentx_s {
// #ifdef __BIG_ENDIAN_BITFIELD
    pub pkt_cnt: u32, // bitfield width 32
// #else
    pub pkt_cnt: u32, // bitfield width 32
// #endif
    pub s: cvmx_pci_bitfield_placeholder,
}

#[repr(C)] pub union cvmx_pci_pkts_sent_int_levx {
    pub 32: u32,
#[repr(C)] pub struct cvmx_pci_pkts_sent_int_levx_s {
// #ifdef __BIG_ENDIAN_BITFIELD
    pub pkt_cnt: u32, // bitfield width 32
// #else
    pub pkt_cnt: u32, // bitfield width 32
// #endif
    pub s: cvmx_pci_bitfield_placeholder,
}

#[repr(C)] pub union cvmx_pci_pkts_sent_timex {
    pub 32: u32,
#[repr(C)] pub struct cvmx_pci_pkts_sent_timex_s {
// #ifdef __BIG_ENDIAN_BITFIELD
    pub pkt_time: u32, // bitfield width 32
// #else
    pub pkt_time: u32, // bitfield width 32
// #endif
    pub s: cvmx_pci_bitfield_placeholder,
}

#[repr(C)] pub union cvmx_pci_read_cmd_6 {
    pub 32: u32,
#[repr(C)] pub struct cvmx_pci_read_cmd_6_s {
// #ifdef __BIG_ENDIAN_BITFIELD
    pub reserved_9_31: u32, // bitfield width 23
    pub min_data: u32, // bitfield width 6
    pub prefetch: u32, // bitfield width 3
// #else
    pub prefetch: u32, // bitfield width 3
    pub min_data: u32, // bitfield width 6
    pub reserved_9_31: u32, // bitfield width 23
// #endif
    pub s: cvmx_pci_bitfield_placeholder,
}

#[repr(C)] pub union cvmx_pci_read_cmd_c {
    pub 32: u32,
#[repr(C)] pub struct cvmx_pci_read_cmd_c_s {
// #ifdef __BIG_ENDIAN_BITFIELD
    pub reserved_9_31: u32, // bitfield width 23
    pub min_data: u32, // bitfield width 6
    pub prefetch: u32, // bitfield width 3
// #else
    pub prefetch: u32, // bitfield width 3
    pub min_data: u32, // bitfield width 6
    pub reserved_9_31: u32, // bitfield width 23
// #endif
    pub s: cvmx_pci_bitfield_placeholder,
}

#[repr(C)] pub union cvmx_pci_read_cmd_e {
    pub 32: u32,
#[repr(C)] pub struct cvmx_pci_read_cmd_e_s {
// #ifdef __BIG_ENDIAN_BITFIELD
    pub reserved_9_31: u32, // bitfield width 23
    pub min_data: u32, // bitfield width 6
    pub prefetch: u32, // bitfield width 3
// #else
    pub prefetch: u32, // bitfield width 3
    pub min_data: u32, // bitfield width 6
    pub reserved_9_31: u32, // bitfield width 23
// #endif
    pub s: cvmx_pci_bitfield_placeholder,
}

#[repr(C)] pub union cvmx_pci_read_timeout {
    pub 64: u64,
#[repr(C)] pub struct cvmx_pci_read_timeout_s {
// #ifdef __BIG_ENDIAN_BITFIELD
    pub reserved_32_63: u64, // bitfield width 32
    pub enb: u64, // bitfield width 1
    pub cnt: u64, // bitfield width 31
// #else
    pub cnt: u64, // bitfield width 31
    pub enb: u64, // bitfield width 1
    pub reserved_32_63: u64, // bitfield width 32
// #endif
    pub s: cvmx_pci_bitfield_placeholder,
}

#[repr(C)] pub union cvmx_pci_scm_reg {
    pub 64: u64,
#[repr(C)] pub struct cvmx_pci_scm_reg_s {
// #ifdef __BIG_ENDIAN_BITFIELD
    pub reserved_32_63: u64, // bitfield width 32
    pub scm: u64, // bitfield width 32
// #else
    pub scm: u64, // bitfield width 32
    pub reserved_32_63: u64, // bitfield width 32
// #endif
    pub s: cvmx_pci_bitfield_placeholder,
}

#[repr(C)] pub union cvmx_pci_tsr_reg {
    pub 64: u64,
#[repr(C)] pub struct cvmx_pci_tsr_reg_s {
// #ifdef __BIG_ENDIAN_BITFIELD
    pub reserved_36_63: u64, // bitfield width 28
    pub tsr: u64, // bitfield width 36
// #else
    pub tsr: u64, // bitfield width 36
    pub reserved_36_63: u64, // bitfield width 28
// #endif
    pub s: cvmx_pci_bitfield_placeholder,
}

#[repr(C)] pub union cvmx_pci_win_rd_addr {
    pub 64: u64,
#[repr(C)] pub struct cvmx_pci_win_rd_addr_s {
// #ifdef __BIG_ENDIAN_BITFIELD
    pub reserved_49_63: u64, // bitfield width 15
    pub iobit: u64, // bitfield width 1
    pub reserved_0_47: u64, // bitfield width 48
// #else
    pub reserved_0_47: u64, // bitfield width 48
    pub iobit: u64, // bitfield width 1
    pub reserved_49_63: u64, // bitfield width 15
// #endif
    pub s: cvmx_pci_bitfield_placeholder,
#[repr(C)] pub struct cvmx_pci_win_rd_addr_cn30xx {
// #ifdef __BIG_ENDIAN_BITFIELD
    pub reserved_49_63: u64, // bitfield width 15
    pub iobit: u64, // bitfield width 1
    pub rd_addr: u64, // bitfield width 46
    pub reserved_0_1: u64, // bitfield width 2
// #else
    pub reserved_0_1: u64, // bitfield width 2
    pub rd_addr: u64, // bitfield width 46
    pub iobit: u64, // bitfield width 1
    pub reserved_49_63: u64, // bitfield width 15
// #endif
	} cn30xx;
#[repr(C)] pub struct cvmx_pci_win_rd_addr_cn38xx {
// #ifdef __BIG_ENDIAN_BITFIELD
    pub reserved_49_63: u64, // bitfield width 15
    pub iobit: u64, // bitfield width 1
    pub rd_addr: u64, // bitfield width 45
    pub reserved_0_2: u64, // bitfield width 3
// #else
    pub reserved_0_2: u64, // bitfield width 3
    pub rd_addr: u64, // bitfield width 45
    pub iobit: u64, // bitfield width 1
    pub reserved_49_63: u64, // bitfield width 15
// #endif
	} cn38xx;
}

#[repr(C)] pub union cvmx_pci_win_rd_data {
    pub 64: u64,
#[repr(C)] pub struct cvmx_pci_win_rd_data_s {
// #ifdef __BIG_ENDIAN_BITFIELD
    pub rd_data: u64, // bitfield width 64
// #else
    pub rd_data: u64, // bitfield width 64
// #endif
    pub s: cvmx_pci_bitfield_placeholder,
}

#[repr(C)] pub union cvmx_pci_win_wr_addr {
    pub 64: u64,
#[repr(C)] pub struct cvmx_pci_win_wr_addr_s {
// #ifdef __BIG_ENDIAN_BITFIELD
    pub reserved_49_63: u64, // bitfield width 15
    pub iobit: u64, // bitfield width 1
    pub wr_addr: u64, // bitfield width 45
    pub reserved_0_2: u64, // bitfield width 3
// #else
    pub reserved_0_2: u64, // bitfield width 3
    pub wr_addr: u64, // bitfield width 45
    pub iobit: u64, // bitfield width 1
    pub reserved_49_63: u64, // bitfield width 15
// #endif
    pub s: cvmx_pci_bitfield_placeholder,
}

#[repr(C)] pub union cvmx_pci_win_wr_data {
    pub 64: u64,
#[repr(C)] pub struct cvmx_pci_win_wr_data_s {
// #ifdef __BIG_ENDIAN_BITFIELD
    pub wr_data: u64, // bitfield width 64
// #else
    pub wr_data: u64, // bitfield width 64
// #endif
    pub s: cvmx_pci_bitfield_placeholder,
}

#[repr(C)] pub union cvmx_pci_win_wr_mask {
    pub 64: u64,
#[repr(C)] pub struct cvmx_pci_win_wr_mask_s {
// #ifdef __BIG_ENDIAN_BITFIELD
    pub reserved_8_63: u64, // bitfield width 56
    pub wr_mask: u64, // bitfield width 8
// #else
    pub wr_mask: u64, // bitfield width 8
    pub reserved_8_63: u64, // bitfield width 56
// #endif
    pub s: cvmx_pci_bitfield_placeholder,
}

// #endif

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
