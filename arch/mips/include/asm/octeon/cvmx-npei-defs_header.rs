// Faithful source-level Rust translation of cvmx-npei-defs.h.
#![allow(non_camel_case_types, non_snake_case, dead_code)]
#[allow(non_upper_case_globals)]
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

// header guard omitted
// header guard omitted

#[inline]
pub const fn CVMX_NPEI_BAR1_INDEXX(offset: u64) -> u64 { (0x0000000000000000ull + ((offset) & 31) * 16) }
pub const CVMX_NPEI_BIST_STATUS: u64 = 0x0000000000000580;
pub const CVMX_NPEI_BIST_STATUS2: u64 = 0x0000000000000680;
pub const CVMX_NPEI_CTL_PORT0: u64 = 0x0000000000000250;
pub const CVMX_NPEI_CTL_PORT1: u64 = 0x0000000000000260;
pub const CVMX_NPEI_CTL_STATUS: u64 = 0x0000000000000570;
pub const CVMX_NPEI_CTL_STATUS2: u64 = 0x0000000000003C00;
pub const CVMX_NPEI_DATA_OUT_CNT: u64 = 0x00000000000005F0;
pub const CVMX_NPEI_DBG_DATA: u64 = 0x0000000000000510;
pub const CVMX_NPEI_DBG_SELECT: u64 = 0x0000000000000500;
pub const CVMX_NPEI_DMA0_INT_LEVEL: u64 = 0x00000000000005C0;
pub const CVMX_NPEI_DMA1_INT_LEVEL: u64 = 0x00000000000005D0;
#[inline]
pub const fn CVMX_NPEI_DMAX_COUNTS(offset: u64) -> u64 { (0x0000000000000450ull + ((offset) & 7) * 16) }
#[inline]
pub const fn CVMX_NPEI_DMAX_DBELL(offset: u64) -> u64 { (0x00000000000003B0ull + ((offset) & 7) * 16) }
#[inline]
pub const fn CVMX_NPEI_DMAX_IBUFF_SADDR(offset: u64) -> u64 { (0x0000000000000400ull + ((offset) & 7) * 16) }
#[inline]
pub const fn CVMX_NPEI_DMAX_NADDR(offset: u64) -> u64 { (0x00000000000004A0ull + ((offset) & 7) * 16) }
pub const CVMX_NPEI_DMA_CNTS: u64 = 0x00000000000005E0;
pub const CVMX_NPEI_DMA_CONTROL: u64 = 0x00000000000003A0;
pub const CVMX_NPEI_DMA_PCIE_REQ_NUM: u64 = 0x00000000000005B0;
pub const CVMX_NPEI_DMA_STATE1: u64 = 0x00000000000006C0;
pub const CVMX_NPEI_DMA_STATE1_P1: u64 = 0x0000000000000680;
pub const CVMX_NPEI_DMA_STATE2: u64 = 0x00000000000006D0;
pub const CVMX_NPEI_DMA_STATE2_P1: u64 = 0x0000000000000690;
pub const CVMX_NPEI_DMA_STATE3_P1: u64 = 0x00000000000006A0;
pub const CVMX_NPEI_DMA_STATE4_P1: u64 = 0x00000000000006B0;
pub const CVMX_NPEI_DMA_STATE5_P1: u64 = 0x00000000000006C0;
pub const CVMX_NPEI_INT_A_ENB: u64 = 0x0000000000000560;
pub const CVMX_NPEI_INT_A_ENB2: u64 = 0x0000000000003CE0;
pub const CVMX_NPEI_INT_A_SUM: u64 = 0x0000000000000550;
pub const CVMX_NPEI_INT_ENB: u64 = 0x0000000000000540;
pub const CVMX_NPEI_INT_ENB2: u64 = 0x0000000000003CD0;
pub const CVMX_NPEI_INT_INFO: u64 = 0x0000000000000590;
pub const CVMX_NPEI_INT_SUM: u64 = 0x0000000000000530;
pub const CVMX_NPEI_INT_SUM2: u64 = 0x0000000000003CC0;
pub const CVMX_NPEI_LAST_WIN_RDATA0: u64 = 0x0000000000000600;
pub const CVMX_NPEI_LAST_WIN_RDATA1: u64 = 0x0000000000000610;
pub const CVMX_NPEI_MEM_ACCESS_CTL: u64 = 0x00000000000004F0;
#[inline]
pub const fn CVMX_NPEI_MEM_ACCESS_SUBIDX(offset: u64) -> u64 { (0x0000000000000280ull + ((offset) & 31) * 16 - 16*12) }
pub const CVMX_NPEI_MSI_ENB0: u64 = 0x0000000000003C50;
pub const CVMX_NPEI_MSI_ENB1: u64 = 0x0000000000003C60;
pub const CVMX_NPEI_MSI_ENB2: u64 = 0x0000000000003C70;
pub const CVMX_NPEI_MSI_ENB3: u64 = 0x0000000000003C80;
pub const CVMX_NPEI_MSI_RCV0: u64 = 0x0000000000003C10;
pub const CVMX_NPEI_MSI_RCV1: u64 = 0x0000000000003C20;
pub const CVMX_NPEI_MSI_RCV2: u64 = 0x0000000000003C30;
pub const CVMX_NPEI_MSI_RCV3: u64 = 0x0000000000003C40;
pub const CVMX_NPEI_MSI_RD_MAP: u64 = 0x0000000000003CA0;
pub const CVMX_NPEI_MSI_W1C_ENB0: u64 = 0x0000000000003CF0;
pub const CVMX_NPEI_MSI_W1C_ENB1: u64 = 0x0000000000003D00;
pub const CVMX_NPEI_MSI_W1C_ENB2: u64 = 0x0000000000003D10;
pub const CVMX_NPEI_MSI_W1C_ENB3: u64 = 0x0000000000003D20;
pub const CVMX_NPEI_MSI_W1S_ENB0: u64 = 0x0000000000003D30;
pub const CVMX_NPEI_MSI_W1S_ENB1: u64 = 0x0000000000003D40;
pub const CVMX_NPEI_MSI_W1S_ENB2: u64 = 0x0000000000003D50;
pub const CVMX_NPEI_MSI_W1S_ENB3: u64 = 0x0000000000003D60;
pub const CVMX_NPEI_MSI_WR_MAP: u64 = 0x0000000000003C90;
pub const CVMX_NPEI_PCIE_CREDIT_CNT: u64 = 0x0000000000003D70;
pub const CVMX_NPEI_PCIE_MSI_RCV: u64 = 0x0000000000003CB0;
pub const CVMX_NPEI_PCIE_MSI_RCV_B1: u64 = 0x0000000000000650;
pub const CVMX_NPEI_PCIE_MSI_RCV_B2: u64 = 0x0000000000000660;
pub const CVMX_NPEI_PCIE_MSI_RCV_B3: u64 = 0x0000000000000670;
#[inline]
pub const fn CVMX_NPEI_PKTX_CNTS(offset: u64) -> u64 { (0x0000000000002400ull + ((offset) & 31) * 16) }
#[inline]
pub const fn CVMX_NPEI_PKTX_INSTR_BADDR(offset: u64) -> u64 { (0x0000000000002800ull + ((offset) & 31) * 16) }
#[inline]
pub const fn CVMX_NPEI_PKTX_INSTR_BAOFF_DBELL(offset: u64) -> u64 { (0x0000000000002C00ull + ((offset) & 31) * 16) }
#[inline]
pub const fn CVMX_NPEI_PKTX_INSTR_FIFO_RSIZE(offset: u64) -> u64 { (0x0000000000003000ull + ((offset) & 31) * 16) }
#[inline]
pub const fn CVMX_NPEI_PKTX_INSTR_HEADER(offset: u64) -> u64 { (0x0000000000003400ull + ((offset) & 31) * 16) }
#[inline]
pub const fn CVMX_NPEI_PKTX_IN_BP(offset: u64) -> u64 { (0x0000000000003800ull + ((offset) & 31) * 16) }
#[inline]
pub const fn CVMX_NPEI_PKTX_SLIST_BADDR(offset: u64) -> u64 { (0x0000000000001400ull + ((offset) & 31) * 16) }
#[inline]
pub const fn CVMX_NPEI_PKTX_SLIST_BAOFF_DBELL(offset: u64) -> u64 { (0x0000000000001800ull + ((offset) & 31) * 16) }
#[inline]
pub const fn CVMX_NPEI_PKTX_SLIST_FIFO_RSIZE(offset: u64) -> u64 { (0x0000000000001C00ull + ((offset) & 31) * 16) }
pub const CVMX_NPEI_PKT_CNT_INT: u64 = 0x0000000000001110;
pub const CVMX_NPEI_PKT_CNT_INT_ENB: u64 = 0x0000000000001130;
pub const CVMX_NPEI_PKT_DATA_OUT_ES: u64 = 0x00000000000010B0;
pub const CVMX_NPEI_PKT_DATA_OUT_NS: u64 = 0x00000000000010A0;
pub const CVMX_NPEI_PKT_DATA_OUT_ROR: u64 = 0x0000000000001090;
pub const CVMX_NPEI_PKT_DPADDR: u64 = 0x0000000000001080;
pub const CVMX_NPEI_PKT_INPUT_CONTROL: u64 = 0x0000000000001150;
pub const CVMX_NPEI_PKT_INSTR_ENB: u64 = 0x0000000000001000;
pub const CVMX_NPEI_PKT_INSTR_RD_SIZE: u64 = 0x0000000000001190;
pub const CVMX_NPEI_PKT_INSTR_SIZE: u64 = 0x0000000000001020;
pub const CVMX_NPEI_PKT_INT_LEVELS: u64 = 0x0000000000001100;
pub const CVMX_NPEI_PKT_IN_BP: u64 = 0x00000000000006B0;
#[inline]
pub const fn CVMX_NPEI_PKT_IN_DONEX_CNTS(offset: u64) -> u64 { (0x0000000000002000ull + ((offset) & 31) * 16) }
pub const CVMX_NPEI_PKT_IN_INSTR_COUNTS: u64 = 0x00000000000006A0;
pub const CVMX_NPEI_PKT_IN_PCIE_PORT: u64 = 0x00000000000011A0;
pub const CVMX_NPEI_PKT_IPTR: u64 = 0x0000000000001070;
pub const CVMX_NPEI_PKT_OUTPUT_WMARK: u64 = 0x0000000000001160;
pub const CVMX_NPEI_PKT_OUT_BMODE: u64 = 0x00000000000010D0;
pub const CVMX_NPEI_PKT_OUT_ENB: u64 = 0x0000000000001010;
pub const CVMX_NPEI_PKT_PCIE_PORT: u64 = 0x00000000000010E0;
pub const CVMX_NPEI_PKT_PORT_IN_RST: u64 = 0x0000000000000690;
pub const CVMX_NPEI_PKT_SLIST_ES: u64 = 0x0000000000001050;
pub const CVMX_NPEI_PKT_SLIST_ID_SIZE: u64 = 0x0000000000001180;
pub const CVMX_NPEI_PKT_SLIST_NS: u64 = 0x0000000000001040;
pub const CVMX_NPEI_PKT_SLIST_ROR: u64 = 0x0000000000001030;
pub const CVMX_NPEI_PKT_TIME_INT: u64 = 0x0000000000001120;
pub const CVMX_NPEI_PKT_TIME_INT_ENB: u64 = 0x0000000000001140;
pub const CVMX_NPEI_RSL_INT_BLOCKS: u64 = 0x0000000000000520;
pub const CVMX_NPEI_SCRATCH_1: u64 = 0x0000000000000270;
pub const CVMX_NPEI_STATE1: u64 = 0x0000000000000620;
pub const CVMX_NPEI_STATE2: u64 = 0x0000000000000630;
pub const CVMX_NPEI_STATE3: u64 = 0x0000000000000640;
pub const CVMX_NPEI_WINDOW_CTL: u64 = 0x0000000000000380;
pub const CVMX_NPEI_WIN_RD_ADDR: u64 = 0x0000000000000210;
pub const CVMX_NPEI_WIN_RD_DATA: u64 = 0x0000000000000240;
pub const CVMX_NPEI_WIN_WR_ADDR: u64 = 0x0000000000000200;
pub const CVMX_NPEI_WIN_WR_DATA: u64 = 0x0000000000000220;
pub const CVMX_NPEI_WIN_WR_MASK: u64 = 0x0000000000000230;

#[repr(C)]
pub union cvmx_npei_bar1_indexx {
    pub u32: u32,
	struct cvmx_npei_bar1_indexx_s {
// #ifdef __BIG_ENDIAN_BITFIELD
        pub reserved_18_31: u32,
        pub addr_idx: u32,
        pub ca: u32,
        pub end_swp: u32,
        pub addr_v: u32,
// #else (bitfield order is target-dependent)
        pub addr_v: u32,
        pub end_swp: u32,
        pub ca: u32,
        pub addr_idx: u32,
        pub reserved_18_31: u32,
// header guard omitted
    pub s: cvmx_npei_bar1_indexx_s,
}

#[repr(C)]
pub union cvmx_npei_bist_status {
    pub u64: u64,
	struct cvmx_npei_bist_status_s {
// #ifdef __BIG_ENDIAN_BITFIELD
        pub pkt_rdf: u64,
        pub reserved_60_62: u64,
        pub pcr_gim: u64,
        pub pkt_pif: u64,
        pub pcsr_int: u64,
        pub pcsr_im: u64,
        pub pcsr_cnt: u64,
        pub pcsr_id: u64,
        pub pcsr_sl: u64,
        pub reserved_50_52: u64,
        pub pkt_ind: u64,
        pub pkt_slm: u64,
        pub reserved_36_47: u64,
        pub d0_pst: u64,
        pub d1_pst: u64,
        pub d2_pst: u64,
        pub d3_pst: u64,
        pub reserved_31_31: u64,
        pub n2p0_c: u64,
        pub n2p0_o: u64,
        pub n2p1_c: u64,
        pub n2p1_o: u64,
        pub cpl_p0: u64,
        pub cpl_p1: u64,
        pub p2n1_po: u64,
        pub p2n1_no: u64,
        pub p2n1_co: u64,
        pub p2n0_po: u64,
        pub p2n0_no: u64,
        pub p2n0_co: u64,
        pub p2n0_c0: u64,
        pub p2n0_c1: u64,
        pub p2n0_n: u64,
        pub p2n0_p0: u64,
        pub p2n0_p1: u64,
        pub p2n1_c0: u64,
        pub p2n1_c1: u64,
        pub p2n1_n: u64,
        pub p2n1_p0: u64,
        pub p2n1_p1: u64,
        pub csm0: u64,
        pub csm1: u64,
        pub dif0: u64,
        pub dif1: u64,
        pub dif2: u64,
        pub dif3: u64,
        pub reserved_2_2: u64,
        pub msi: u64,
        pub ncb_cmd: u64,
// #else (bitfield order is target-dependent)
        pub ncb_cmd: u64,
        pub msi: u64,
        pub reserved_2_2: u64,
        pub dif3: u64,
        pub dif2: u64,
        pub dif1: u64,
        pub dif0: u64,
        pub csm1: u64,
        pub csm0: u64,
        pub p2n1_p1: u64,
        pub p2n1_p0: u64,
        pub p2n1_n: u64,
        pub p2n1_c1: u64,
        pub p2n1_c0: u64,
        pub p2n0_p1: u64,
        pub p2n0_p0: u64,
        pub p2n0_n: u64,
        pub p2n0_c1: u64,
        pub p2n0_c0: u64,
        pub p2n0_co: u64,
        pub p2n0_no: u64,
        pub p2n0_po: u64,
        pub p2n1_co: u64,
        pub p2n1_no: u64,
        pub p2n1_po: u64,
        pub cpl_p1: u64,
        pub cpl_p0: u64,
        pub n2p1_o: u64,
        pub n2p1_c: u64,
        pub n2p0_o: u64,
        pub n2p0_c: u64,
        pub reserved_31_31: u64,
        pub d3_pst: u64,
        pub d2_pst: u64,
        pub d1_pst: u64,
        pub d0_pst: u64,
        pub reserved_36_47: u64,
        pub pkt_slm: u64,
        pub pkt_ind: u64,
        pub reserved_50_52: u64,
        pub pcsr_sl: u64,
        pub pcsr_id: u64,
        pub pcsr_cnt: u64,
        pub pcsr_im: u64,
        pub pcsr_int: u64,
        pub pkt_pif: u64,
        pub pcr_gim: u64,
        pub reserved_60_62: u64,
        pub pkt_rdf: u64,
// header guard omitted
    pub s: cvmx_npei_bar1_indexx_s,
	struct cvmx_npei_bist_status_cn52xx {
// #ifdef __BIG_ENDIAN_BITFIELD
        pub pkt_rdf: u64,
        pub reserved_60_62: u64,
        pub pcr_gim: u64,
        pub pkt_pif: u64,
        pub pcsr_int: u64,
        pub pcsr_im: u64,
        pub pcsr_cnt: u64,
        pub pcsr_id: u64,
        pub pcsr_sl: u64,
        pub pkt_imem: u64,
        pub pkt_pfm: u64,
        pub pkt_pof: u64,
        pub reserved_48_49: u64,
        pub pkt_pop0: u64,
        pub pkt_pop1: u64,
        pub d0_mem: u64,
        pub d1_mem: u64,
        pub d2_mem: u64,
        pub d3_mem: u64,
        pub d4_mem: u64,
        pub ds_mem: u64,
        pub reserved_36_39: u64,
        pub d0_pst: u64,
        pub d1_pst: u64,
        pub d2_pst: u64,
        pub d3_pst: u64,
        pub d4_pst: u64,
        pub n2p0_c: u64,
        pub n2p0_o: u64,
        pub n2p1_c: u64,
        pub n2p1_o: u64,
        pub cpl_p0: u64,
        pub cpl_p1: u64,
        pub p2n1_po: u64,
        pub p2n1_no: u64,
        pub p2n1_co: u64,
        pub p2n0_po: u64,
        pub p2n0_no: u64,
        pub p2n0_co: u64,
        pub p2n0_c0: u64,
        pub p2n0_c1: u64,
        pub p2n0_n: u64,
        pub p2n0_p0: u64,
        pub p2n0_p1: u64,
        pub p2n1_c0: u64,
        pub p2n1_c1: u64,
        pub p2n1_n: u64,
        pub p2n1_p0: u64,
        pub p2n1_p1: u64,
        pub csm0: u64,
        pub csm1: u64,
        pub dif0: u64,
        pub dif1: u64,
        pub dif2: u64,
        pub dif3: u64,
        pub dif4: u64,
        pub msi: u64,
        pub ncb_cmd: u64,
// #else (bitfield order is target-dependent)
        pub ncb_cmd: u64,
        pub msi: u64,
        pub dif4: u64,
        pub dif3: u64,
        pub dif2: u64,
        pub dif1: u64,
        pub dif0: u64,
        pub csm1: u64,
        pub csm0: u64,
        pub p2n1_p1: u64,
        pub p2n1_p0: u64,
        pub p2n1_n: u64,
        pub p2n1_c1: u64,
        pub p2n1_c0: u64,
        pub p2n0_p1: u64,
        pub p2n0_p0: u64,
        pub p2n0_n: u64,
        pub p2n0_c1: u64,
        pub p2n0_c0: u64,
        pub p2n0_co: u64,
        pub p2n0_no: u64,
        pub p2n0_po: u64,
        pub p2n1_co: u64,
        pub p2n1_no: u64,
        pub p2n1_po: u64,
        pub cpl_p1: u64,
        pub cpl_p0: u64,
        pub n2p1_o: u64,
        pub n2p1_c: u64,
        pub n2p0_o: u64,
        pub n2p0_c: u64,
        pub d4_pst: u64,
        pub d3_pst: u64,
        pub d2_pst: u64,
        pub d1_pst: u64,
        pub d0_pst: u64,
        pub reserved_36_39: u64,
        pub ds_mem: u64,
        pub d4_mem: u64,
        pub d3_mem: u64,
        pub d2_mem: u64,
        pub d1_mem: u64,
        pub d0_mem: u64,
        pub pkt_pop1: u64,
        pub pkt_pop0: u64,
        pub reserved_48_49: u64,
        pub pkt_pof: u64,
        pub pkt_pfm: u64,
        pub pkt_imem: u64,
        pub pcsr_sl: u64,
        pub pcsr_id: u64,
        pub pcsr_cnt: u64,
        pub pcsr_im: u64,
        pub pcsr_int: u64,
        pub pkt_pif: u64,
        pub pcr_gim: u64,
        pub reserved_60_62: u64,
        pub pkt_rdf: u64,
// header guard omitted
    pub cn52xx: cvmx_npei_bist_status_cn52xx,
	struct cvmx_npei_bist_status_cn52xxp1 {
// #ifdef __BIG_ENDIAN_BITFIELD
        pub reserved_46_63: u64,
        pub d0_mem0: u64,
        pub d1_mem1: u64,
        pub d2_mem2: u64,
        pub d3_mem3: u64,
        pub dr0_mem: u64,
        pub d0_mem: u64,
        pub d1_mem: u64,
        pub d2_mem: u64,
        pub d3_mem: u64,
        pub dr1_mem: u64,
        pub d0_pst: u64,
        pub d1_pst: u64,
        pub d2_pst: u64,
        pub d3_pst: u64,
        pub dr2_mem: u64,
        pub n2p0_c: u64,
        pub n2p0_o: u64,
        pub n2p1_c: u64,
        pub n2p1_o: u64,
        pub cpl_p0: u64,
        pub cpl_p1: u64,
        pub p2n1_po: u64,
        pub p2n1_no: u64,
        pub p2n1_co: u64,
        pub p2n0_po: u64,
        pub p2n0_no: u64,
        pub p2n0_co: u64,
        pub p2n0_c0: u64,
        pub p2n0_c1: u64,
        pub p2n0_n: u64,
        pub p2n0_p0: u64,
        pub p2n0_p1: u64,
        pub p2n1_c0: u64,
        pub p2n1_c1: u64,
        pub p2n1_n: u64,
        pub p2n1_p0: u64,
        pub p2n1_p1: u64,
        pub csm0: u64,
        pub csm1: u64,
        pub dif0: u64,
        pub dif1: u64,
        pub dif2: u64,
        pub dif3: u64,
        pub dr3_mem: u64,
        pub msi: u64,
        pub ncb_cmd: u64,
// #else (bitfield order is target-dependent)
        pub ncb_cmd: u64,
        pub msi: u64,
        pub dr3_mem: u64,
        pub dif3: u64,
        pub dif2: u64,
        pub dif1: u64,
        pub dif0: u64,
        pub csm1: u64,
        pub csm0: u64,
        pub p2n1_p1: u64,
        pub p2n1_p0: u64,
        pub p2n1_n: u64,
        pub p2n1_c1: u64,
        pub p2n1_c0: u64,
        pub p2n0_p1: u64,
        pub p2n0_p0: u64,
        pub p2n0_n: u64,
        pub p2n0_c1: u64,
        pub p2n0_c0: u64,
        pub p2n0_co: u64,
        pub p2n0_no: u64,
        pub p2n0_po: u64,
        pub p2n1_co: u64,
        pub p2n1_no: u64,
        pub p2n1_po: u64,
        pub cpl_p1: u64,
        pub cpl_p0: u64,
        pub n2p1_o: u64,
        pub n2p1_c: u64,
        pub n2p0_o: u64,
        pub n2p0_c: u64,
        pub dr2_mem: u64,
        pub d3_pst: u64,
        pub d2_pst: u64,
        pub d1_pst: u64,
        pub d0_pst: u64,
        pub dr1_mem: u64,
        pub d3_mem: u64,
        pub d2_mem: u64,
        pub d1_mem: u64,
        pub d0_mem: u64,
        pub dr0_mem: u64,
        pub d3_mem3: u64,
        pub d2_mem2: u64,
        pub d1_mem1: u64,
        pub d0_mem0: u64,
        pub reserved_46_63: u64,
// header guard omitted
    pub cn52xxp1: cvmx_npei_bist_status_cn52xxp1,
	struct cvmx_npei_bist_status_cn56xxp1 {
// #ifdef __BIG_ENDIAN_BITFIELD
        pub reserved_58_63: u64,
        pub pcsr_int: u64,
        pub pcsr_im: u64,
        pub pcsr_cnt: u64,
        pub pcsr_id: u64,
        pub pcsr_sl: u64,
        pub pkt_pout: u64,
        pub pkt_imem: u64,
        pub pkt_cntm: u64,
        pub pkt_ind: u64,
        pub pkt_slm: u64,
        pub pkt_odf: u64,
        pub pkt_oif: u64,
        pub pkt_out: u64,
        pub pkt_i0: u64,
        pub pkt_i1: u64,
        pub pkt_s0: u64,
        pub pkt_s1: u64,
        pub d0_mem: u64,
        pub d1_mem: u64,
        pub d2_mem: u64,
        pub d3_mem: u64,
        pub d4_mem: u64,
        pub d0_pst: u64,
        pub d1_pst: u64,
        pub d2_pst: u64,
        pub d3_pst: u64,
        pub d4_pst: u64,
        pub n2p0_c: u64,
        pub n2p0_o: u64,
        pub n2p1_c: u64,
        pub n2p1_o: u64,
        pub cpl_p0: u64,
        pub cpl_p1: u64,
        pub p2n1_po: u64,
        pub p2n1_no: u64,
        pub p2n1_co: u64,
        pub p2n0_po: u64,
        pub p2n0_no: u64,
        pub p2n0_co: u64,
        pub p2n0_c0: u64,
        pub p2n0_c1: u64,
        pub p2n0_n: u64,
        pub p2n0_p0: u64,
        pub p2n0_p1: u64,
        pub p2n1_c0: u64,
        pub p2n1_c1: u64,
        pub p2n1_n: u64,
        pub p2n1_p0: u64,
        pub p2n1_p1: u64,
        pub csm0: u64,
        pub csm1: u64,
        pub dif0: u64,
        pub dif1: u64,
        pub dif2: u64,
        pub dif3: u64,
        pub dif4: u64,
        pub msi: u64,
        pub ncb_cmd: u64,
// #else (bitfield order is target-dependent)
        pub ncb_cmd: u64,
        pub msi: u64,
        pub dif4: u64,
        pub dif3: u64,
        pub dif2: u64,
        pub dif1: u64,
        pub dif0: u64,
        pub csm1: u64,
        pub csm0: u64,
        pub p2n1_p1: u64,
        pub p2n1_p0: u64,
        pub p2n1_n: u64,
        pub p2n1_c1: u64,
        pub p2n1_c0: u64,
        pub p2n0_p1: u64,
        pub p2n0_p0: u64,
        pub p2n0_n: u64,
        pub p2n0_c1: u64,
        pub p2n0_c0: u64,
        pub p2n0_co: u64,
        pub p2n0_no: u64,
        pub p2n0_po: u64,
        pub p2n1_co: u64,
        pub p2n1_no: u64,
        pub p2n1_po: u64,
        pub cpl_p1: u64,
        pub cpl_p0: u64,
        pub n2p1_o: u64,
        pub n2p1_c: u64,
        pub n2p0_o: u64,
        pub n2p0_c: u64,
        pub d4_pst: u64,
        pub d3_pst: u64,
        pub d2_pst: u64,
        pub d1_pst: u64,
        pub d0_pst: u64,
        pub d4_mem: u64,
        pub d3_mem: u64,
        pub d2_mem: u64,
        pub d1_mem: u64,
        pub d0_mem: u64,
        pub pkt_s1: u64,
        pub pkt_s0: u64,
        pub pkt_i1: u64,
        pub pkt_i0: u64,
        pub pkt_out: u64,
        pub pkt_oif: u64,
        pub pkt_odf: u64,
        pub pkt_slm: u64,
        pub pkt_ind: u64,
        pub pkt_cntm: u64,
        pub pkt_imem: u64,
        pub pkt_pout: u64,
        pub pcsr_sl: u64,
        pub pcsr_id: u64,
        pub pcsr_cnt: u64,
        pub pcsr_im: u64,
        pub pcsr_int: u64,
        pub reserved_58_63: u64,
// header guard omitted
    pub cn56xxp1: cvmx_npei_bist_status_cn56xxp1,
}

#[repr(C)]
pub union cvmx_npei_bist_status2 {
    pub u64: u64,
	struct cvmx_npei_bist_status2_s {
// #ifdef __BIG_ENDIAN_BITFIELD
        pub reserved_14_63: u64,
        pub prd_tag: u64,
        pub prd_st0: u64,
        pub prd_st1: u64,
        pub prd_err: u64,
        pub nrd_st: u64,
        pub nwe_st: u64,
        pub nwe_wr0: u64,
        pub nwe_wr1: u64,
        pub pkt_rd: u64,
        pub psc_p0: u64,
        pub psc_p1: u64,
        pub pkt_gd: u64,
        pub pkt_gl: u64,
        pub pkt_blk: u64,
// #else (bitfield order is target-dependent)
        pub pkt_blk: u64,
        pub pkt_gl: u64,
        pub pkt_gd: u64,
        pub psc_p1: u64,
        pub psc_p0: u64,
        pub pkt_rd: u64,
        pub nwe_wr1: u64,
        pub nwe_wr0: u64,
        pub nwe_st: u64,
        pub nrd_st: u64,
        pub prd_err: u64,
        pub prd_st1: u64,
        pub prd_st0: u64,
        pub prd_tag: u64,
        pub reserved_14_63: u64,
// header guard omitted
    pub s: cvmx_npei_bar1_indexx_s,
}

#[repr(C)]
pub union cvmx_npei_ctl_port0 {
    pub u64: u64,
	struct cvmx_npei_ctl_port0_s {
// #ifdef __BIG_ENDIAN_BITFIELD
        pub reserved_21_63: u64,
        pub waitl_com: u64,
        pub intd: u64,
        pub intc: u64,
        pub intb: u64,
        pub inta: u64,
        pub intd_map: u64,
        pub intc_map: u64,
        pub intb_map: u64,
        pub inta_map: u64,
        pub ctlp_ro: u64,
        pub reserved_6_6: u64,
        pub ptlp_ro: u64,
        pub bar2_enb: u64,
        pub bar2_esx: u64,
        pub bar2_cax: u64,
        pub wait_com: u64,
// #else (bitfield order is target-dependent)
        pub wait_com: u64,
        pub bar2_cax: u64,
        pub bar2_esx: u64,
        pub bar2_enb: u64,
        pub ptlp_ro: u64,
        pub reserved_6_6: u64,
        pub ctlp_ro: u64,
        pub inta_map: u64,
        pub intb_map: u64,
        pub intc_map: u64,
        pub intd_map: u64,
        pub inta: u64,
        pub intb: u64,
        pub intc: u64,
        pub intd: u64,
        pub waitl_com: u64,
        pub reserved_21_63: u64,
// header guard omitted
    pub s: cvmx_npei_bar1_indexx_s,
}

#[repr(C)]
pub union cvmx_npei_ctl_port1 {
    pub u64: u64,
	struct cvmx_npei_ctl_port1_s {
// #ifdef __BIG_ENDIAN_BITFIELD
        pub reserved_21_63: u64,
        pub waitl_com: u64,
        pub intd: u64,
        pub intc: u64,
        pub intb: u64,
        pub inta: u64,
        pub intd_map: u64,
        pub intc_map: u64,
        pub intb_map: u64,
        pub inta_map: u64,
        pub ctlp_ro: u64,
        pub reserved_6_6: u64,
        pub ptlp_ro: u64,
        pub bar2_enb: u64,
        pub bar2_esx: u64,
        pub bar2_cax: u64,
        pub wait_com: u64,
// #else (bitfield order is target-dependent)
        pub wait_com: u64,
        pub bar2_cax: u64,
        pub bar2_esx: u64,
        pub bar2_enb: u64,
        pub ptlp_ro: u64,
        pub reserved_6_6: u64,
        pub ctlp_ro: u64,
        pub inta_map: u64,
        pub intb_map: u64,
        pub intc_map: u64,
        pub intd_map: u64,
        pub inta: u64,
        pub intb: u64,
        pub intc: u64,
        pub intd: u64,
        pub waitl_com: u64,
        pub reserved_21_63: u64,
// header guard omitted
    pub s: cvmx_npei_bar1_indexx_s,
}

#[repr(C)]
pub union cvmx_npei_ctl_status {
    pub u64: u64,
	struct cvmx_npei_ctl_status_s {
// #ifdef __BIG_ENDIAN_BITFIELD
        pub reserved_44_63: u64,
        pub p1_ntags: u64,
        pub p0_ntags: u64,
        pub cfg_rtry: u64,
        pub ring_en: u64,
        pub lnk_rst: u64,
        pub arb: u64,
        pub pkt_bp: u64,
        pub host_mode: u64,
        pub chip_rev: u64,
// #else (bitfield order is target-dependent)
        pub chip_rev: u64,
        pub host_mode: u64,
        pub pkt_bp: u64,
        pub arb: u64,
        pub lnk_rst: u64,
        pub ring_en: u64,
        pub cfg_rtry: u64,
        pub p0_ntags: u64,
        pub p1_ntags: u64,
        pub reserved_44_63: u64,
// header guard omitted
    pub s: cvmx_npei_bar1_indexx_s,
	struct cvmx_npei_ctl_status_cn52xxp1 {
// #ifdef __BIG_ENDIAN_BITFIELD
        pub reserved_44_63: u64,
        pub p1_ntags: u64,
        pub p0_ntags: u64,
        pub cfg_rtry: u64,
        pub reserved_15_15: u64,
        pub lnk_rst: u64,
        pub arb: u64,
        pub reserved_9_12: u64,
        pub host_mode: u64,
        pub chip_rev: u64,
// #else (bitfield order is target-dependent)
        pub chip_rev: u64,
        pub host_mode: u64,
        pub reserved_9_12: u64,
        pub arb: u64,
        pub lnk_rst: u64,
        pub reserved_15_15: u64,
        pub cfg_rtry: u64,
        pub p0_ntags: u64,
        pub p1_ntags: u64,
        pub reserved_44_63: u64,
// header guard omitted
    pub cn52xxp1: cvmx_npei_bist_status_cn52xxp1,
	struct cvmx_npei_ctl_status_cn56xxp1 {
// #ifdef __BIG_ENDIAN_BITFIELD
        pub reserved_15_63: u64,
        pub lnk_rst: u64,
        pub arb: u64,
        pub pkt_bp: u64,
        pub host_mode: u64,
        pub chip_rev: u64,
// #else (bitfield order is target-dependent)
        pub chip_rev: u64,
        pub host_mode: u64,
        pub pkt_bp: u64,
        pub arb: u64,
        pub lnk_rst: u64,
        pub reserved_15_63: u64,
// header guard omitted
    pub cn56xxp1: cvmx_npei_bist_status_cn56xxp1,
}

#[repr(C)]
pub union cvmx_npei_ctl_status2 {
    pub u64: u64,
	struct cvmx_npei_ctl_status2_s {
// #ifdef __BIG_ENDIAN_BITFIELD
        pub reserved_16_63: u64,
        pub mps: u64,
        pub mrrs: u64,
        pub c1_w_flt: u64,
        pub c0_w_flt: u64,
        pub c1_b1_s: u64,
        pub c0_b1_s: u64,
        pub c1_wi_d: u64,
        pub c1_b0_d: u64,
        pub c0_wi_d: u64,
        pub c0_b0_d: u64,
// #else (bitfield order is target-dependent)
        pub c0_b0_d: u64,
        pub c0_wi_d: u64,
        pub c1_b0_d: u64,
        pub c1_wi_d: u64,
        pub c0_b1_s: u64,
        pub c1_b1_s: u64,
        pub c0_w_flt: u64,
        pub c1_w_flt: u64,
        pub mrrs: u64,
        pub mps: u64,
        pub reserved_16_63: u64,
// header guard omitted
    pub s: cvmx_npei_bar1_indexx_s,
}

#[repr(C)]
pub union cvmx_npei_data_out_cnt {
    pub u64: u64,
	struct cvmx_npei_data_out_cnt_s {
// #ifdef __BIG_ENDIAN_BITFIELD
        pub reserved_44_63: u64,
        pub p1_ucnt: u64,
        pub p1_fcnt: u64,
        pub p0_ucnt: u64,
        pub p0_fcnt: u64,
// #else (bitfield order is target-dependent)
        pub p0_fcnt: u64,
        pub p0_ucnt: u64,
        pub p1_fcnt: u64,
        pub p1_ucnt: u64,
        pub reserved_44_63: u64,
// header guard omitted
    pub s: cvmx_npei_bar1_indexx_s,
}

#[repr(C)]
pub union cvmx_npei_dbg_data {
    pub u64: u64,
	struct cvmx_npei_dbg_data_s {
// #ifdef __BIG_ENDIAN_BITFIELD
        pub reserved_28_63: u64,
        pub qlm0_rev_lanes: u64,
        pub reserved_25_26: u64,
        pub qlm1_spd: u64,
        pub c_mul: u64,
        pub dsel_ext: u64,
        pub data: u64,
// #else (bitfield order is target-dependent)
        pub data: u64,
        pub dsel_ext: u64,
        pub c_mul: u64,
        pub qlm1_spd: u64,
        pub reserved_25_26: u64,
        pub qlm0_rev_lanes: u64,
        pub reserved_28_63: u64,
// header guard omitted
    pub s: cvmx_npei_bar1_indexx_s,
	struct cvmx_npei_dbg_data_cn52xx {
// #ifdef __BIG_ENDIAN_BITFIELD
        pub reserved_29_63: u64,
        pub qlm0_link_width: u64,
        pub qlm0_rev_lanes: u64,
        pub qlm1_mode: u64,
        pub qlm1_spd: u64,
        pub c_mul: u64,
        pub dsel_ext: u64,
        pub data: u64,
// #else (bitfield order is target-dependent)
        pub data: u64,
        pub dsel_ext: u64,
        pub c_mul: u64,
        pub qlm1_spd: u64,
        pub qlm1_mode: u64,
        pub qlm0_rev_lanes: u64,
        pub qlm0_link_width: u64,
        pub reserved_29_63: u64,
// header guard omitted
    pub cn52xx: cvmx_npei_bist_status_cn52xx,
	struct cvmx_npei_dbg_data_cn56xx {
// #ifdef __BIG_ENDIAN_BITFIELD
        pub reserved_29_63: u64,
        pub qlm2_rev_lanes: u64,
        pub qlm0_rev_lanes: u64,
        pub qlm3_spd: u64,
        pub qlm1_spd: u64,
        pub c_mul: u64,
        pub dsel_ext: u64,
        pub data: u64,
// #else (bitfield order is target-dependent)
        pub data: u64,
        pub dsel_ext: u64,
        pub c_mul: u64,
        pub qlm1_spd: u64,
        pub qlm3_spd: u64,
        pub qlm0_rev_lanes: u64,
        pub qlm2_rev_lanes: u64,
        pub reserved_29_63: u64,
// header guard omitted
    pub cn56xx: cvmx_npei_bist_status_cn56xx,
}

#[repr(C)]
pub union cvmx_npei_dbg_select {
    pub u64: u64,
	struct cvmx_npei_dbg_select_s {
// #ifdef __BIG_ENDIAN_BITFIELD
        pub reserved_16_63: u64,
        pub dbg_sel: u64,
// #else (bitfield order is target-dependent)
        pub dbg_sel: u64,
        pub reserved_16_63: u64,
// header guard omitted
    pub s: cvmx_npei_bar1_indexx_s,
}

#[repr(C)]
pub union cvmx_npei_dmax_counts {
    pub u64: u64,
	struct cvmx_npei_dmax_counts_s {
// #ifdef __BIG_ENDIAN_BITFIELD
        pub reserved_39_63: u64,
        pub fcnt: u64,
        pub dbell: u64,
// #else (bitfield order is target-dependent)
        pub dbell: u64,
        pub fcnt: u64,
        pub reserved_39_63: u64,
// header guard omitted
    pub s: cvmx_npei_bar1_indexx_s,
}

#[repr(C)]
pub union cvmx_npei_dmax_dbell {
    pub u32: u32,
	struct cvmx_npei_dmax_dbell_s {
// #ifdef __BIG_ENDIAN_BITFIELD
        pub reserved_16_31: u32,
        pub dbell: u32,
// #else (bitfield order is target-dependent)
        pub dbell: u32,
        pub reserved_16_31: u32,
// header guard omitted
    pub s: cvmx_npei_bar1_indexx_s,
}

#[repr(C)]
pub union cvmx_npei_dmax_ibuff_saddr {
    pub u64: u64,
	struct cvmx_npei_dmax_ibuff_saddr_s {
// #ifdef __BIG_ENDIAN_BITFIELD
        pub reserved_37_63: u64,
        pub idle: u64,
        pub saddr: u64,
        pub reserved_0_6: u64,
// #else (bitfield order is target-dependent)
        pub reserved_0_6: u64,
        pub saddr: u64,
        pub idle: u64,
        pub reserved_37_63: u64,
// header guard omitted
    pub s: cvmx_npei_bar1_indexx_s,
	struct cvmx_npei_dmax_ibuff_saddr_cn52xxp1 {
// #ifdef __BIG_ENDIAN_BITFIELD
        pub reserved_36_63: u64,
        pub saddr: u64,
        pub reserved_0_6: u64,
// #else (bitfield order is target-dependent)
        pub reserved_0_6: u64,
        pub saddr: u64,
        pub reserved_36_63: u64,
// header guard omitted
    pub cn52xxp1: cvmx_npei_bist_status_cn52xxp1,
}

#[repr(C)]
pub union cvmx_npei_dmax_naddr {
    pub u64: u64,
	struct cvmx_npei_dmax_naddr_s {
// #ifdef __BIG_ENDIAN_BITFIELD
        pub reserved_36_63: u64,
        pub addr: u64,
// #else (bitfield order is target-dependent)
        pub addr: u64,
        pub reserved_36_63: u64,
// header guard omitted
    pub s: cvmx_npei_bar1_indexx_s,
}

#[repr(C)]
pub union cvmx_npei_dma0_int_level {
    pub u64: u64,
	struct cvmx_npei_dma0_int_level_s {
// #ifdef __BIG_ENDIAN_BITFIELD
        pub time: u64,
        pub cnt: u64,
// #else (bitfield order is target-dependent)
        pub cnt: u64,
        pub time: u64,
// header guard omitted
    pub s: cvmx_npei_bar1_indexx_s,
}

#[repr(C)]
pub union cvmx_npei_dma1_int_level {
    pub u64: u64,
	struct cvmx_npei_dma1_int_level_s {
// #ifdef __BIG_ENDIAN_BITFIELD
        pub time: u64,
        pub cnt: u64,
// #else (bitfield order is target-dependent)
        pub cnt: u64,
        pub time: u64,
// header guard omitted
    pub s: cvmx_npei_bar1_indexx_s,
}

#[repr(C)]
pub union cvmx_npei_dma_cnts {
    pub u64: u64,
	struct cvmx_npei_dma_cnts_s {
// #ifdef __BIG_ENDIAN_BITFIELD
        pub dma1: u64,
        pub dma0: u64,
// #else (bitfield order is target-dependent)
        pub dma0: u64,
        pub dma1: u64,
// header guard omitted
    pub s: cvmx_npei_bar1_indexx_s,
}

#[repr(C)]
pub union cvmx_npei_dma_control {
    pub u64: u64,
	struct cvmx_npei_dma_control_s {
// #ifdef __BIG_ENDIAN_BITFIELD
        pub reserved_40_63: u64,
        pub p_32b_m: u64,
        pub dma4_enb: u64,
        pub dma3_enb: u64,
        pub dma2_enb: u64,
        pub dma1_enb: u64,
        pub dma0_enb: u64,
        pub b0_lend: u64,
        pub dwb_denb: u64,
        pub dwb_ichk: u64,
        pub fpa_que: u64,
        pub o_add1: u64,
        pub o_ro: u64,
        pub o_ns: u64,
        pub o_es: u64,
        pub o_mode: u64,
        pub csize: u64,
// #else (bitfield order is target-dependent)
        pub csize: u64,
        pub o_mode: u64,
        pub o_es: u64,
        pub o_ns: u64,
        pub o_ro: u64,
        pub o_add1: u64,
        pub fpa_que: u64,
        pub dwb_ichk: u64,
        pub dwb_denb: u64,
        pub b0_lend: u64,
        pub dma0_enb: u64,
        pub dma1_enb: u64,
        pub dma2_enb: u64,
        pub dma3_enb: u64,
        pub dma4_enb: u64,
        pub p_32b_m: u64,
        pub reserved_40_63: u64,
// header guard omitted
    pub s: cvmx_npei_bar1_indexx_s,
	struct cvmx_npei_dma_control_cn52xxp1 {
// #ifdef __BIG_ENDIAN_BITFIELD
        pub reserved_38_63: u64,
        pub dma3_enb: u64,
        pub dma2_enb: u64,
        pub dma1_enb: u64,
        pub dma0_enb: u64,
        pub b0_lend: u64,
        pub dwb_denb: u64,
        pub dwb_ichk: u64,
        pub fpa_que: u64,
        pub o_add1: u64,
        pub o_ro: u64,
        pub o_ns: u64,
        pub o_es: u64,
        pub o_mode: u64,
        pub csize: u64,
// #else (bitfield order is target-dependent)
        pub csize: u64,
        pub o_mode: u64,
        pub o_es: u64,
        pub o_ns: u64,
        pub o_ro: u64,
        pub o_add1: u64,
        pub fpa_que: u64,
        pub dwb_ichk: u64,
        pub dwb_denb: u64,
        pub b0_lend: u64,
        pub dma0_enb: u64,
        pub dma1_enb: u64,
        pub dma2_enb: u64,
        pub dma3_enb: u64,
        pub reserved_38_63: u64,
// header guard omitted
    pub cn52xxp1: cvmx_npei_bist_status_cn52xxp1,
	struct cvmx_npei_dma_control_cn56xxp1 {
// #ifdef __BIG_ENDIAN_BITFIELD
        pub reserved_39_63: u64,
        pub dma4_enb: u64,
        pub dma3_enb: u64,
        pub dma2_enb: u64,
        pub dma1_enb: u64,
        pub dma0_enb: u64,
        pub b0_lend: u64,
        pub dwb_denb: u64,
        pub dwb_ichk: u64,
        pub fpa_que: u64,
        pub o_add1: u64,
        pub o_ro: u64,
        pub o_ns: u64,
        pub o_es: u64,
        pub o_mode: u64,
        pub csize: u64,
// #else (bitfield order is target-dependent)
        pub csize: u64,
        pub o_mode: u64,
        pub o_es: u64,
        pub o_ns: u64,
        pub o_ro: u64,
        pub o_add1: u64,
        pub fpa_que: u64,
        pub dwb_ichk: u64,
        pub dwb_denb: u64,
        pub b0_lend: u64,
        pub dma0_enb: u64,
        pub dma1_enb: u64,
        pub dma2_enb: u64,
        pub dma3_enb: u64,
        pub dma4_enb: u64,
        pub reserved_39_63: u64,
// header guard omitted
    pub cn56xxp1: cvmx_npei_bist_status_cn56xxp1,
}

#[repr(C)]
pub union cvmx_npei_dma_pcie_req_num {
    pub u64: u64,
	struct cvmx_npei_dma_pcie_req_num_s {
// #ifdef __BIG_ENDIAN_BITFIELD
        pub dma_arb: u64,
        pub reserved_53_62: u64,
        pub pkt_cnt: u64,
        pub reserved_45_47: u64,
        pub dma4_cnt: u64,
        pub reserved_37_39: u64,
        pub dma3_cnt: u64,
        pub reserved_29_31: u64,
        pub dma2_cnt: u64,
        pub reserved_21_23: u64,
        pub dma1_cnt: u64,
        pub reserved_13_15: u64,
        pub dma0_cnt: u64,
        pub reserved_5_7: u64,
        pub dma_cnt: u64,
// #else (bitfield order is target-dependent)
        pub dma_cnt: u64,
        pub reserved_5_7: u64,
        pub dma0_cnt: u64,
        pub reserved_13_15: u64,
        pub dma1_cnt: u64,
        pub reserved_21_23: u64,
        pub dma2_cnt: u64,
        pub reserved_29_31: u64,
        pub dma3_cnt: u64,
        pub reserved_37_39: u64,
        pub dma4_cnt: u64,
        pub reserved_45_47: u64,
        pub pkt_cnt: u64,
        pub reserved_53_62: u64,
        pub dma_arb: u64,
// header guard omitted
    pub s: cvmx_npei_bar1_indexx_s,
}

#[repr(C)]
pub union cvmx_npei_dma_state1 {
    pub u64: u64,
	struct cvmx_npei_dma_state1_s {
// #ifdef __BIG_ENDIAN_BITFIELD
        pub reserved_40_63: u64,
        pub d4_dwe: u64,
        pub d3_dwe: u64,
        pub d2_dwe: u64,
        pub d1_dwe: u64,
        pub d0_dwe: u64,
// #else (bitfield order is target-dependent)
        pub d0_dwe: u64,
        pub d1_dwe: u64,
        pub d2_dwe: u64,
        pub d3_dwe: u64,
        pub d4_dwe: u64,
        pub reserved_40_63: u64,
// header guard omitted
    pub s: cvmx_npei_bar1_indexx_s,
}

#[repr(C)]
pub union cvmx_npei_dma_state1_p1 {
    pub u64: u64,
	struct cvmx_npei_dma_state1_p1_s {
// #ifdef __BIG_ENDIAN_BITFIELD
        pub reserved_60_63: u64,
        pub d0_difst: u64,
        pub d1_difst: u64,
        pub d2_difst: u64,
        pub d3_difst: u64,
        pub d4_difst: u64,
        pub d0_reqst: u64,
        pub d1_reqst: u64,
        pub d2_reqst: u64,
        pub d3_reqst: u64,
        pub d4_reqst: u64,
// #else (bitfield order is target-dependent)
        pub d4_reqst: u64,
        pub d3_reqst: u64,
        pub d2_reqst: u64,
        pub d1_reqst: u64,
        pub d0_reqst: u64,
        pub d4_difst: u64,
        pub d3_difst: u64,
        pub d2_difst: u64,
        pub d1_difst: u64,
        pub d0_difst: u64,
        pub reserved_60_63: u64,
// header guard omitted
    pub s: cvmx_npei_bar1_indexx_s,
	struct cvmx_npei_dma_state1_p1_cn52xxp1 {
// #ifdef __BIG_ENDIAN_BITFIELD
        pub reserved_60_63: u64,
        pub d0_difst: u64,
        pub d1_difst: u64,
        pub d2_difst: u64,
        pub d3_difst: u64,
        pub reserved_25_31: u64,
        pub d0_reqst: u64,
        pub d1_reqst: u64,
        pub d2_reqst: u64,
        pub d3_reqst: u64,
        pub reserved_0_4: u64,
// #else (bitfield order is target-dependent)
        pub reserved_0_4: u64,
        pub d3_reqst: u64,
        pub d2_reqst: u64,
        pub d1_reqst: u64,
        pub d0_reqst: u64,
        pub reserved_25_31: u64,
        pub d3_difst: u64,
        pub d2_difst: u64,
        pub d1_difst: u64,
        pub d0_difst: u64,
        pub reserved_60_63: u64,
// header guard omitted
    pub cn52xxp1: cvmx_npei_bist_status_cn52xxp1,
}

#[repr(C)]
pub union cvmx_npei_dma_state2 {
    pub u64: u64,
	struct cvmx_npei_dma_state2_s {
// #ifdef __BIG_ENDIAN_BITFIELD
        pub reserved_28_63: u64,
        pub ndwe: u64,
        pub reserved_21_23: u64,
        pub ndre: u64,
        pub reserved_10_15: u64,
        pub prd: u64,
// #else (bitfield order is target-dependent)
        pub prd: u64,
        pub reserved_10_15: u64,
        pub ndre: u64,
        pub reserved_21_23: u64,
        pub ndwe: u64,
        pub reserved_28_63: u64,
// header guard omitted
    pub s: cvmx_npei_bar1_indexx_s,
}

#[repr(C)]
pub union cvmx_npei_dma_state2_p1 {
    pub u64: u64,
	struct cvmx_npei_dma_state2_p1_s {
// #ifdef __BIG_ENDIAN_BITFIELD
        pub reserved_45_63: u64,
        pub d0_dffst: u64,
        pub d1_dffst: u64,
        pub d2_dffst: u64,
        pub d3_dffst: u64,
        pub d4_dffst: u64,
// #else (bitfield order is target-dependent)
        pub d4_dffst: u64,
        pub d3_dffst: u64,
        pub d2_dffst: u64,
        pub d1_dffst: u64,
        pub d0_dffst: u64,
        pub reserved_45_63: u64,
// header guard omitted
    pub s: cvmx_npei_bar1_indexx_s,
	struct cvmx_npei_dma_state2_p1_cn52xxp1 {
// #ifdef __BIG_ENDIAN_BITFIELD
        pub reserved_45_63: u64,
        pub d0_dffst: u64,
        pub d1_dffst: u64,
        pub d2_dffst: u64,
        pub d3_dffst: u64,
        pub reserved_0_8: u64,
// #else (bitfield order is target-dependent)
        pub reserved_0_8: u64,
        pub d3_dffst: u64,
        pub d2_dffst: u64,
        pub d1_dffst: u64,
        pub d0_dffst: u64,
        pub reserved_45_63: u64,
// header guard omitted
    pub cn52xxp1: cvmx_npei_bist_status_cn52xxp1,
}

#[repr(C)]
pub union cvmx_npei_dma_state3_p1 {
    pub u64: u64,
	struct cvmx_npei_dma_state3_p1_s {
// #ifdef __BIG_ENDIAN_BITFIELD
        pub reserved_60_63: u64,
        pub d0_drest: u64,
        pub d1_drest: u64,
        pub d2_drest: u64,
        pub d3_drest: u64,
// #else (bitfield order is target-dependent)
        pub d3_drest: u64,
        pub d2_drest: u64,
        pub d1_drest: u64,
        pub d0_drest: u64,
        pub reserved_60_63: u64,
// header guard omitted
    pub s: cvmx_npei_bar1_indexx_s,
}

#[repr(C)]
pub union cvmx_npei_dma_state4_p1 {
    pub u64: u64,
	struct cvmx_npei_dma_state4_p1_s {
// #ifdef __BIG_ENDIAN_BITFIELD
        pub reserved_52_63: u64,
        pub d0_dwest: u64,
        pub d1_dwest: u64,
        pub d2_dwest: u64,
        pub d3_dwest: u64,
// #else (bitfield order is target-dependent)
        pub d3_dwest: u64,
        pub d2_dwest: u64,
        pub d1_dwest: u64,
        pub d0_dwest: u64,
        pub reserved_52_63: u64,
// header guard omitted
    pub s: cvmx_npei_bar1_indexx_s,
}

#[repr(C)]
pub union cvmx_npei_dma_state5_p1 {
    pub u64: u64,
	struct cvmx_npei_dma_state5_p1_s {
// #ifdef __BIG_ENDIAN_BITFIELD
        pub reserved_28_63: u64,
        pub d4_drest: u64,
        pub d4_dwest: u64,
// #else (bitfield order is target-dependent)
        pub d4_dwest: u64,
        pub d4_drest: u64,
        pub reserved_28_63: u64,
// header guard omitted
    pub s: cvmx_npei_bar1_indexx_s,
}

#[repr(C)]
pub union cvmx_npei_int_a_enb {
    pub u64: u64,
	struct cvmx_npei_int_a_enb_s {
// #ifdef __BIG_ENDIAN_BITFIELD
        pub reserved_10_63: u64,
        pub pout_err: u64,
        pub pin_bp: u64,
        pub p1_rdlk: u64,
        pub p0_rdlk: u64,
        pub pgl_err: u64,
        pub pdi_err: u64,
        pub pop_err: u64,
        pub pins_err: u64,
        pub dma1_cpl: u64,
        pub dma0_cpl: u64,
// #else (bitfield order is target-dependent)
        pub dma0_cpl: u64,
        pub dma1_cpl: u64,
        pub pins_err: u64,
        pub pop_err: u64,
        pub pdi_err: u64,
        pub pgl_err: u64,
        pub p0_rdlk: u64,
        pub p1_rdlk: u64,
        pub pin_bp: u64,
        pub pout_err: u64,
        pub reserved_10_63: u64,
// header guard omitted
    pub s: cvmx_npei_bar1_indexx_s,
	struct cvmx_npei_int_a_enb_cn52xxp1 {
// #ifdef __BIG_ENDIAN_BITFIELD
        pub reserved_2_63: u64,
        pub dma1_cpl: u64,
        pub dma0_cpl: u64,
// #else (bitfield order is target-dependent)
        pub dma0_cpl: u64,
        pub dma1_cpl: u64,
        pub reserved_2_63: u64,
// header guard omitted
    pub cn52xxp1: cvmx_npei_bist_status_cn52xxp1,
}

#[repr(C)]
pub union cvmx_npei_int_a_enb2 {
    pub u64: u64,
	struct cvmx_npei_int_a_enb2_s {
// #ifdef __BIG_ENDIAN_BITFIELD
        pub reserved_10_63: u64,
        pub pout_err: u64,
        pub pin_bp: u64,
        pub p1_rdlk: u64,
        pub p0_rdlk: u64,
        pub pgl_err: u64,
        pub pdi_err: u64,
        pub pop_err: u64,
        pub pins_err: u64,
        pub dma1_cpl: u64,
        pub dma0_cpl: u64,
// #else (bitfield order is target-dependent)
        pub dma0_cpl: u64,
        pub dma1_cpl: u64,
        pub pins_err: u64,
        pub pop_err: u64,
        pub pdi_err: u64,
        pub pgl_err: u64,
        pub p0_rdlk: u64,
        pub p1_rdlk: u64,
        pub pin_bp: u64,
        pub pout_err: u64,
        pub reserved_10_63: u64,
// header guard omitted
    pub s: cvmx_npei_bar1_indexx_s,
	struct cvmx_npei_int_a_enb2_cn52xxp1 {
// #ifdef __BIG_ENDIAN_BITFIELD
        pub reserved_2_63: u64,
        pub dma1_cpl: u64,
        pub dma0_cpl: u64,
// #else (bitfield order is target-dependent)
        pub dma0_cpl: u64,
        pub dma1_cpl: u64,
        pub reserved_2_63: u64,
// header guard omitted
    pub cn52xxp1: cvmx_npei_bist_status_cn52xxp1,
}

#[repr(C)]
pub union cvmx_npei_int_a_sum {
    pub u64: u64,
	struct cvmx_npei_int_a_sum_s {
// #ifdef __BIG_ENDIAN_BITFIELD
        pub reserved_10_63: u64,
        pub pout_err: u64,
        pub pin_bp: u64,
        pub p1_rdlk: u64,
        pub p0_rdlk: u64,
        pub pgl_err: u64,
        pub pdi_err: u64,
        pub pop_err: u64,
        pub pins_err: u64,
        pub dma1_cpl: u64,
        pub dma0_cpl: u64,
// #else (bitfield order is target-dependent)
        pub dma0_cpl: u64,
        pub dma1_cpl: u64,
        pub pins_err: u64,
        pub pop_err: u64,
        pub pdi_err: u64,
        pub pgl_err: u64,
        pub p0_rdlk: u64,
        pub p1_rdlk: u64,
        pub pin_bp: u64,
        pub pout_err: u64,
        pub reserved_10_63: u64,
// header guard omitted
    pub s: cvmx_npei_bar1_indexx_s,
	struct cvmx_npei_int_a_sum_cn52xxp1 {
// #ifdef __BIG_ENDIAN_BITFIELD
        pub reserved_2_63: u64,
        pub dma1_cpl: u64,
        pub dma0_cpl: u64,
// #else (bitfield order is target-dependent)
        pub dma0_cpl: u64,
        pub dma1_cpl: u64,
        pub reserved_2_63: u64,
// header guard omitted
    pub cn52xxp1: cvmx_npei_bist_status_cn52xxp1,
}

#[repr(C)]
pub union cvmx_npei_int_enb {
    pub u64: u64,
	struct cvmx_npei_int_enb_s {
// #ifdef __BIG_ENDIAN_BITFIELD
        pub mio_inta: u64,
        pub reserved_62_62: u64,
        pub int_a: u64,
        pub c1_ldwn: u64,
        pub c0_ldwn: u64,
        pub c1_exc: u64,
        pub c0_exc: u64,
        pub c1_up_wf: u64,
        pub c0_up_wf: u64,
        pub c1_un_wf: u64,
        pub c0_un_wf: u64,
        pub c1_un_bx: u64,
        pub c1_un_wi: u64,
        pub c1_un_b2: u64,
        pub c1_un_b1: u64,
        pub c1_un_b0: u64,
        pub c1_up_bx: u64,
        pub c1_up_wi: u64,
        pub c1_up_b2: u64,
        pub c1_up_b1: u64,
        pub c1_up_b0: u64,
        pub c0_un_bx: u64,
        pub c0_un_wi: u64,
        pub c0_un_b2: u64,
        pub c0_un_b1: u64,
        pub c0_un_b0: u64,
        pub c0_up_bx: u64,
        pub c0_up_wi: u64,
        pub c0_up_b2: u64,
        pub c0_up_b1: u64,
        pub c0_up_b0: u64,
        pub c1_hpint: u64,
        pub c1_pmei: u64,
        pub c1_wake: u64,
        pub crs1_dr: u64,
        pub c1_se: u64,
        pub crs1_er: u64,
        pub c1_aeri: u64,
        pub c0_hpint: u64,
        pub c0_pmei: u64,
        pub c0_wake: u64,
        pub crs0_dr: u64,
        pub c0_se: u64,
        pub crs0_er: u64,
        pub c0_aeri: u64,
        pub ptime: u64,
        pub pcnt: u64,
        pub pidbof: u64,
        pub psldbof: u64,
        pub dtime1: u64,
        pub dtime0: u64,
        pub dcnt1: u64,
        pub dcnt0: u64,
        pub dma1fi: u64,
        pub dma0fi: u64,
        pub dma4dbo: u64,
        pub dma3dbo: u64,
        pub dma2dbo: u64,
        pub dma1dbo: u64,
        pub dma0dbo: u64,
        pub iob2big: u64,
        pub bar0_to: u64,
        pub rml_wto: u64,
        pub rml_rto: u64,
// #else (bitfield order is target-dependent)
        pub rml_rto: u64,
        pub rml_wto: u64,
        pub bar0_to: u64,
        pub iob2big: u64,
        pub dma0dbo: u64,
        pub dma1dbo: u64,
        pub dma2dbo: u64,
        pub dma3dbo: u64,
        pub dma4dbo: u64,
        pub dma0fi: u64,
        pub dma1fi: u64,
        pub dcnt0: u64,
        pub dcnt1: u64,
        pub dtime0: u64,
        pub dtime1: u64,
        pub psldbof: u64,
        pub pidbof: u64,
        pub pcnt: u64,
        pub ptime: u64,
        pub c0_aeri: u64,
        pub crs0_er: u64,
        pub c0_se: u64,
        pub crs0_dr: u64,
        pub c0_wake: u64,
        pub c0_pmei: u64,
        pub c0_hpint: u64,
        pub c1_aeri: u64,
        pub crs1_er: u64,
        pub c1_se: u64,
        pub crs1_dr: u64,
        pub c1_wake: u64,
        pub c1_pmei: u64,
        pub c1_hpint: u64,
        pub c0_up_b0: u64,
        pub c0_up_b1: u64,
        pub c0_up_b2: u64,
        pub c0_up_wi: u64,
        pub c0_up_bx: u64,
        pub c0_un_b0: u64,
        pub c0_un_b1: u64,
        pub c0_un_b2: u64,
        pub c0_un_wi: u64,
        pub c0_un_bx: u64,
        pub c1_up_b0: u64,
        pub c1_up_b1: u64,
        pub c1_up_b2: u64,
        pub c1_up_wi: u64,
        pub c1_up_bx: u64,
        pub c1_un_b0: u64,
        pub c1_un_b1: u64,
        pub c1_un_b2: u64,
        pub c1_un_wi: u64,
        pub c1_un_bx: u64,
        pub c0_un_wf: u64,
        pub c1_un_wf: u64,
        pub c0_up_wf: u64,
        pub c1_up_wf: u64,
        pub c0_exc: u64,
        pub c1_exc: u64,
        pub c0_ldwn: u64,
        pub c1_ldwn: u64,
        pub int_a: u64,
        pub reserved_62_62: u64,
        pub mio_inta: u64,
// header guard omitted
    pub s: cvmx_npei_bar1_indexx_s,
	struct cvmx_npei_int_enb_cn52xxp1 {
// #ifdef __BIG_ENDIAN_BITFIELD
        pub mio_inta: u64,
        pub reserved_62_62: u64,
        pub int_a: u64,
        pub c1_ldwn: u64,
        pub c0_ldwn: u64,
        pub c1_exc: u64,
        pub c0_exc: u64,
        pub c1_up_wf: u64,
        pub c0_up_wf: u64,
        pub c1_un_wf: u64,
        pub c0_un_wf: u64,
        pub c1_un_bx: u64,
        pub c1_un_wi: u64,
        pub c1_un_b2: u64,
        pub c1_un_b1: u64,
        pub c1_un_b0: u64,
        pub c1_up_bx: u64,
        pub c1_up_wi: u64,
        pub c1_up_b2: u64,
        pub c1_up_b1: u64,
        pub c1_up_b0: u64,
        pub c0_un_bx: u64,
        pub c0_un_wi: u64,
        pub c0_un_b2: u64,
        pub c0_un_b1: u64,
        pub c0_un_b0: u64,
        pub c0_up_bx: u64,
        pub c0_up_wi: u64,
        pub c0_up_b2: u64,
        pub c0_up_b1: u64,
        pub c0_up_b0: u64,
        pub c1_hpint: u64,
        pub c1_pmei: u64,
        pub c1_wake: u64,
        pub crs1_dr: u64,
        pub c1_se: u64,
        pub crs1_er: u64,
        pub c1_aeri: u64,
        pub c0_hpint: u64,
        pub c0_pmei: u64,
        pub c0_wake: u64,
        pub crs0_dr: u64,
        pub c0_se: u64,
        pub crs0_er: u64,
        pub c0_aeri: u64,
        pub ptime: u64,
        pub pcnt: u64,
        pub pidbof: u64,
        pub psldbof: u64,
        pub dtime1: u64,
        pub dtime0: u64,
        pub dcnt1: u64,
        pub dcnt0: u64,
        pub dma1fi: u64,
        pub dma0fi: u64,
        pub reserved_8_8: u64,
        pub dma3dbo: u64,
        pub dma2dbo: u64,
        pub dma1dbo: u64,
        pub dma0dbo: u64,
        pub iob2big: u64,
        pub bar0_to: u64,
        pub rml_wto: u64,
        pub rml_rto: u64,
// #else (bitfield order is target-dependent)
        pub rml_rto: u64,
        pub rml_wto: u64,
        pub bar0_to: u64,
        pub iob2big: u64,
        pub dma0dbo: u64,
        pub dma1dbo: u64,
        pub dma2dbo: u64,
        pub dma3dbo: u64,
        pub reserved_8_8: u64,
        pub dma0fi: u64,
        pub dma1fi: u64,
        pub dcnt0: u64,
        pub dcnt1: u64,
        pub dtime0: u64,
        pub dtime1: u64,
        pub psldbof: u64,
        pub pidbof: u64,
        pub pcnt: u64,
        pub ptime: u64,
        pub c0_aeri: u64,
        pub crs0_er: u64,
        pub c0_se: u64,
        pub crs0_dr: u64,
        pub c0_wake: u64,
        pub c0_pmei: u64,
        pub c0_hpint: u64,
        pub c1_aeri: u64,
        pub crs1_er: u64,
        pub c1_se: u64,
        pub crs1_dr: u64,
        pub c1_wake: u64,
        pub c1_pmei: u64,
        pub c1_hpint: u64,
        pub c0_up_b0: u64,
        pub c0_up_b1: u64,
        pub c0_up_b2: u64,
        pub c0_up_wi: u64,
        pub c0_up_bx: u64,
        pub c0_un_b0: u64,
        pub c0_un_b1: u64,
        pub c0_un_b2: u64,
        pub c0_un_wi: u64,
        pub c0_un_bx: u64,
        pub c1_up_b0: u64,
        pub c1_up_b1: u64,
        pub c1_up_b2: u64,
        pub c1_up_wi: u64,
        pub c1_up_bx: u64,
        pub c1_un_b0: u64,
        pub c1_un_b1: u64,
        pub c1_un_b2: u64,
        pub c1_un_wi: u64,
        pub c1_un_bx: u64,
        pub c0_un_wf: u64,
        pub c1_un_wf: u64,
        pub c0_up_wf: u64,
        pub c1_up_wf: u64,
        pub c0_exc: u64,
        pub c1_exc: u64,
        pub c0_ldwn: u64,
        pub c1_ldwn: u64,
        pub int_a: u64,
        pub reserved_62_62: u64,
        pub mio_inta: u64,
// header guard omitted
    pub cn52xxp1: cvmx_npei_bist_status_cn52xxp1,
	struct cvmx_npei_int_enb_cn56xxp1 {
// #ifdef __BIG_ENDIAN_BITFIELD
        pub mio_inta: u64,
        pub reserved_61_62: u64,
        pub c1_ldwn: u64,
        pub c0_ldwn: u64,
        pub c1_exc: u64,
        pub c0_exc: u64,
        pub c1_up_wf: u64,
        pub c0_up_wf: u64,
        pub c1_un_wf: u64,
        pub c0_un_wf: u64,
        pub c1_un_bx: u64,
        pub c1_un_wi: u64,
        pub c1_un_b2: u64,
        pub c1_un_b1: u64,
        pub c1_un_b0: u64,
        pub c1_up_bx: u64,
        pub c1_up_wi: u64,
        pub c1_up_b2: u64,
        pub c1_up_b1: u64,
        pub c1_up_b0: u64,
        pub c0_un_bx: u64,
        pub c0_un_wi: u64,
        pub c0_un_b2: u64,
        pub c0_un_b1: u64,
        pub c0_un_b0: u64,
        pub c0_up_bx: u64,
        pub c0_up_wi: u64,
        pub c0_up_b2: u64,
        pub c0_up_b1: u64,
        pub c0_up_b0: u64,
        pub c1_hpint: u64,
        pub c1_pmei: u64,
        pub c1_wake: u64,
        pub reserved_29_29: u64,
        pub c1_se: u64,
        pub reserved_27_27: u64,
        pub c1_aeri: u64,
        pub c0_hpint: u64,
        pub c0_pmei: u64,
        pub c0_wake: u64,
        pub reserved_22_22: u64,
        pub c0_se: u64,
        pub reserved_20_20: u64,
        pub c0_aeri: u64,
        pub ptime: u64,
        pub pcnt: u64,
        pub pidbof: u64,
        pub psldbof: u64,
        pub dtime1: u64,
        pub dtime0: u64,
        pub dcnt1: u64,
        pub dcnt0: u64,
        pub dma1fi: u64,
        pub dma0fi: u64,
        pub dma4dbo: u64,
        pub dma3dbo: u64,
        pub dma2dbo: u64,
        pub dma1dbo: u64,
        pub dma0dbo: u64,
        pub iob2big: u64,
        pub bar0_to: u64,
        pub rml_wto: u64,
        pub rml_rto: u64,
// #else (bitfield order is target-dependent)
        pub rml_rto: u64,
        pub rml_wto: u64,
        pub bar0_to: u64,
        pub iob2big: u64,
        pub dma0dbo: u64,
        pub dma1dbo: u64,
        pub dma2dbo: u64,
        pub dma3dbo: u64,
        pub dma4dbo: u64,
        pub dma0fi: u64,
        pub dma1fi: u64,
        pub dcnt0: u64,
        pub dcnt1: u64,
        pub dtime0: u64,
        pub dtime1: u64,
        pub psldbof: u64,
        pub pidbof: u64,
        pub pcnt: u64,
        pub ptime: u64,
        pub c0_aeri: u64,
        pub reserved_20_20: u64,
        pub c0_se: u64,
        pub reserved_22_22: u64,
        pub c0_wake: u64,
        pub c0_pmei: u64,
        pub c0_hpint: u64,
        pub c1_aeri: u64,
        pub reserved_27_27: u64,
        pub c1_se: u64,
        pub reserved_29_29: u64,
        pub c1_wake: u64,
        pub c1_pmei: u64,
        pub c1_hpint: u64,
        pub c0_up_b0: u64,
        pub c0_up_b1: u64,
        pub c0_up_b2: u64,
        pub c0_up_wi: u64,
        pub c0_up_bx: u64,
        pub c0_un_b0: u64,
        pub c0_un_b1: u64,
        pub c0_un_b2: u64,
        pub c0_un_wi: u64,
        pub c0_un_bx: u64,
        pub c1_up_b0: u64,
        pub c1_up_b1: u64,
        pub c1_up_b2: u64,
        pub c1_up_wi: u64,
        pub c1_up_bx: u64,
        pub c1_un_b0: u64,
        pub c1_un_b1: u64,
        pub c1_un_b2: u64,
        pub c1_un_wi: u64,
        pub c1_un_bx: u64,
        pub c0_un_wf: u64,
        pub c1_un_wf: u64,
        pub c0_up_wf: u64,
        pub c1_up_wf: u64,
        pub c0_exc: u64,
        pub c1_exc: u64,
        pub c0_ldwn: u64,
        pub c1_ldwn: u64,
        pub reserved_61_62: u64,
        pub mio_inta: u64,
// header guard omitted
    pub cn56xxp1: cvmx_npei_bist_status_cn56xxp1,
}

#[repr(C)]
pub union cvmx_npei_int_enb2 {
    pub u64: u64,
	struct cvmx_npei_int_enb2_s {
// #ifdef __BIG_ENDIAN_BITFIELD
        pub reserved_62_63: u64,
        pub int_a: u64,
        pub c1_ldwn: u64,
        pub c0_ldwn: u64,
        pub c1_exc: u64,
        pub c0_exc: u64,
        pub c1_up_wf: u64,
        pub c0_up_wf: u64,
        pub c1_un_wf: u64,
        pub c0_un_wf: u64,
        pub c1_un_bx: u64,
        pub c1_un_wi: u64,
        pub c1_un_b2: u64,
        pub c1_un_b1: u64,
        pub c1_un_b0: u64,
        pub c1_up_bx: u64,
        pub c1_up_wi: u64,
        pub c1_up_b2: u64,
        pub c1_up_b1: u64,
        pub c1_up_b0: u64,
        pub c0_un_bx: u64,
        pub c0_un_wi: u64,
        pub c0_un_b2: u64,
        pub c0_un_b1: u64,
        pub c0_un_b0: u64,
        pub c0_up_bx: u64,
        pub c0_up_wi: u64,
        pub c0_up_b2: u64,
        pub c0_up_b1: u64,
        pub c0_up_b0: u64,
        pub c1_hpint: u64,
        pub c1_pmei: u64,
        pub c1_wake: u64,
        pub crs1_dr: u64,
        pub c1_se: u64,
        pub crs1_er: u64,
        pub c1_aeri: u64,
        pub c0_hpint: u64,
        pub c0_pmei: u64,
        pub c0_wake: u64,
        pub crs0_dr: u64,
        pub c0_se: u64,
        pub crs0_er: u64,
        pub c0_aeri: u64,
        pub ptime: u64,
        pub pcnt: u64,
        pub pidbof: u64,
        pub psldbof: u64,
        pub dtime1: u64,
        pub dtime0: u64,
        pub dcnt1: u64,
        pub dcnt0: u64,
        pub dma1fi: u64,
        pub dma0fi: u64,
        pub dma4dbo: u64,
        pub dma3dbo: u64,
        pub dma2dbo: u64,
        pub dma1dbo: u64,
        pub dma0dbo: u64,
        pub iob2big: u64,
        pub bar0_to: u64,
        pub rml_wto: u64,
        pub rml_rto: u64,
// #else (bitfield order is target-dependent)
        pub rml_rto: u64,
        pub rml_wto: u64,
        pub bar0_to: u64,
        pub iob2big: u64,
        pub dma0dbo: u64,
        pub dma1dbo: u64,
        pub dma2dbo: u64,
        pub dma3dbo: u64,
        pub dma4dbo: u64,
        pub dma0fi: u64,
        pub dma1fi: u64,
        pub dcnt0: u64,
        pub dcnt1: u64,
        pub dtime0: u64,
        pub dtime1: u64,
        pub psldbof: u64,
        pub pidbof: u64,
        pub pcnt: u64,
        pub ptime: u64,
        pub c0_aeri: u64,
        pub crs0_er: u64,
        pub c0_se: u64,
        pub crs0_dr: u64,
        pub c0_wake: u64,
        pub c0_pmei: u64,
        pub c0_hpint: u64,
        pub c1_aeri: u64,
        pub crs1_er: u64,
        pub c1_se: u64,
        pub crs1_dr: u64,
        pub c1_wake: u64,
        pub c1_pmei: u64,
        pub c1_hpint: u64,
        pub c0_up_b0: u64,
        pub c0_up_b1: u64,
        pub c0_up_b2: u64,
        pub c0_up_wi: u64,
        pub c0_up_bx: u64,
        pub c0_un_b0: u64,
        pub c0_un_b1: u64,
        pub c0_un_b2: u64,
        pub c0_un_wi: u64,
        pub c0_un_bx: u64,
        pub c1_up_b0: u64,
        pub c1_up_b1: u64,
        pub c1_up_b2: u64,
        pub c1_up_wi: u64,
        pub c1_up_bx: u64,
        pub c1_un_b0: u64,
        pub c1_un_b1: u64,
        pub c1_un_b2: u64,
        pub c1_un_wi: u64,
        pub c1_un_bx: u64,
        pub c0_un_wf: u64,
        pub c1_un_wf: u64,
        pub c0_up_wf: u64,
        pub c1_up_wf: u64,
        pub c0_exc: u64,
        pub c1_exc: u64,
        pub c0_ldwn: u64,
        pub c1_ldwn: u64,
        pub int_a: u64,
        pub reserved_62_63: u64,
// header guard omitted
    pub s: cvmx_npei_bar1_indexx_s,
	struct cvmx_npei_int_enb2_cn52xxp1 {
// #ifdef __BIG_ENDIAN_BITFIELD
        pub reserved_62_63: u64,
        pub int_a: u64,
        pub c1_ldwn: u64,
        pub c0_ldwn: u64,
        pub c1_exc: u64,
        pub c0_exc: u64,
        pub c1_up_wf: u64,
        pub c0_up_wf: u64,
        pub c1_un_wf: u64,
        pub c0_un_wf: u64,
        pub c1_un_bx: u64,
        pub c1_un_wi: u64,
        pub c1_un_b2: u64,
        pub c1_un_b1: u64,
        pub c1_un_b0: u64,
        pub c1_up_bx: u64,
        pub c1_up_wi: u64,
        pub c1_up_b2: u64,
        pub c1_up_b1: u64,
        pub c1_up_b0: u64,
        pub c0_un_bx: u64,
        pub c0_un_wi: u64,
        pub c0_un_b2: u64,
        pub c0_un_b1: u64,
        pub c0_un_b0: u64,
        pub c0_up_bx: u64,
        pub c0_up_wi: u64,
        pub c0_up_b2: u64,
        pub c0_up_b1: u64,
        pub c0_up_b0: u64,
        pub c1_hpint: u64,
        pub c1_pmei: u64,
        pub c1_wake: u64,
        pub crs1_dr: u64,
        pub c1_se: u64,
        pub crs1_er: u64,
        pub c1_aeri: u64,
        pub c0_hpint: u64,
        pub c0_pmei: u64,
        pub c0_wake: u64,
        pub crs0_dr: u64,
        pub c0_se: u64,
        pub crs0_er: u64,
        pub c0_aeri: u64,
        pub ptime: u64,
        pub pcnt: u64,
        pub pidbof: u64,
        pub psldbof: u64,
        pub dtime1: u64,
        pub dtime0: u64,
        pub dcnt1: u64,
        pub dcnt0: u64,
        pub dma1fi: u64,
        pub dma0fi: u64,
        pub reserved_8_8: u64,
        pub dma3dbo: u64,
        pub dma2dbo: u64,
        pub dma1dbo: u64,
        pub dma0dbo: u64,
        pub iob2big: u64,
        pub bar0_to: u64,
        pub rml_wto: u64,
        pub rml_rto: u64,
// #else (bitfield order is target-dependent)
        pub rml_rto: u64,
        pub rml_wto: u64,
        pub bar0_to: u64,
        pub iob2big: u64,
        pub dma0dbo: u64,
        pub dma1dbo: u64,
        pub dma2dbo: u64,
        pub dma3dbo: u64,
        pub reserved_8_8: u64,
        pub dma0fi: u64,
        pub dma1fi: u64,
        pub dcnt0: u64,
        pub dcnt1: u64,
        pub dtime0: u64,
        pub dtime1: u64,
        pub psldbof: u64,
        pub pidbof: u64,
        pub pcnt: u64,
        pub ptime: u64,
        pub c0_aeri: u64,
        pub crs0_er: u64,
        pub c0_se: u64,
        pub crs0_dr: u64,
        pub c0_wake: u64,
        pub c0_pmei: u64,
        pub c0_hpint: u64,
        pub c1_aeri: u64,
        pub crs1_er: u64,
        pub c1_se: u64,
        pub crs1_dr: u64,
        pub c1_wake: u64,
        pub c1_pmei: u64,
        pub c1_hpint: u64,
        pub c0_up_b0: u64,
        pub c0_up_b1: u64,
        pub c0_up_b2: u64,
        pub c0_up_wi: u64,
        pub c0_up_bx: u64,
        pub c0_un_b0: u64,
        pub c0_un_b1: u64,
        pub c0_un_b2: u64,
        pub c0_un_wi: u64,
        pub c0_un_bx: u64,
        pub c1_up_b0: u64,
        pub c1_up_b1: u64,
        pub c1_up_b2: u64,
        pub c1_up_wi: u64,
        pub c1_up_bx: u64,
        pub c1_un_b0: u64,
        pub c1_un_b1: u64,
        pub c1_un_b2: u64,
        pub c1_un_wi: u64,
        pub c1_un_bx: u64,
        pub c0_un_wf: u64,
        pub c1_un_wf: u64,
        pub c0_up_wf: u64,
        pub c1_up_wf: u64,
        pub c0_exc: u64,
        pub c1_exc: u64,
        pub c0_ldwn: u64,
        pub c1_ldwn: u64,
        pub int_a: u64,
        pub reserved_62_63: u64,
// header guard omitted
    pub cn52xxp1: cvmx_npei_bist_status_cn52xxp1,
	struct cvmx_npei_int_enb2_cn56xxp1 {
// #ifdef __BIG_ENDIAN_BITFIELD
        pub reserved_61_63: u64,
        pub c1_ldwn: u64,
        pub c0_ldwn: u64,
        pub c1_exc: u64,
        pub c0_exc: u64,
        pub c1_up_wf: u64,
        pub c0_up_wf: u64,
        pub c1_un_wf: u64,
        pub c0_un_wf: u64,
        pub c1_un_bx: u64,
        pub c1_un_wi: u64,
        pub c1_un_b2: u64,
        pub c1_un_b1: u64,
        pub c1_un_b0: u64,
        pub c1_up_bx: u64,
        pub c1_up_wi: u64,
        pub c1_up_b2: u64,
        pub c1_up_b1: u64,
        pub c1_up_b0: u64,
        pub c0_un_bx: u64,
        pub c0_un_wi: u64,
        pub c0_un_b2: u64,
        pub c0_un_b1: u64,
        pub c0_un_b0: u64,
        pub c0_up_bx: u64,
        pub c0_up_wi: u64,
        pub c0_up_b2: u64,
        pub c0_up_b1: u64,
        pub c0_up_b0: u64,
        pub c1_hpint: u64,
        pub c1_pmei: u64,
        pub c1_wake: u64,
        pub reserved_29_29: u64,
        pub c1_se: u64,
        pub reserved_27_27: u64,
        pub c1_aeri: u64,
        pub c0_hpint: u64,
        pub c0_pmei: u64,
        pub c0_wake: u64,
        pub reserved_22_22: u64,
        pub c0_se: u64,
        pub reserved_20_20: u64,
        pub c0_aeri: u64,
        pub ptime: u64,
        pub pcnt: u64,
        pub pidbof: u64,
        pub psldbof: u64,
        pub dtime1: u64,
        pub dtime0: u64,
        pub dcnt1: u64,
        pub dcnt0: u64,
        pub dma1fi: u64,
        pub dma0fi: u64,
        pub dma4dbo: u64,
        pub dma3dbo: u64,
        pub dma2dbo: u64,
        pub dma1dbo: u64,
        pub dma0dbo: u64,
        pub iob2big: u64,
        pub bar0_to: u64,
        pub rml_wto: u64,
        pub rml_rto: u64,
// #else (bitfield order is target-dependent)
        pub rml_rto: u64,
        pub rml_wto: u64,
        pub bar0_to: u64,
        pub iob2big: u64,
        pub dma0dbo: u64,
        pub dma1dbo: u64,
        pub dma2dbo: u64,
        pub dma3dbo: u64,
        pub dma4dbo: u64,
        pub dma0fi: u64,
        pub dma1fi: u64,
        pub dcnt0: u64,
        pub dcnt1: u64,
        pub dtime0: u64,
        pub dtime1: u64,
        pub psldbof: u64,
        pub pidbof: u64,
        pub pcnt: u64,
        pub ptime: u64,
        pub c0_aeri: u64,
        pub reserved_20_20: u64,
        pub c0_se: u64,
        pub reserved_22_22: u64,
        pub c0_wake: u64,
        pub c0_pmei: u64,
        pub c0_hpint: u64,
        pub c1_aeri: u64,
        pub reserved_27_27: u64,
        pub c1_se: u64,
        pub reserved_29_29: u64,
        pub c1_wake: u64,
        pub c1_pmei: u64,
        pub c1_hpint: u64,
        pub c0_up_b0: u64,
        pub c0_up_b1: u64,
        pub c0_up_b2: u64,
        pub c0_up_wi: u64,
        pub c0_up_bx: u64,
        pub c0_un_b0: u64,
        pub c0_un_b1: u64,
        pub c0_un_b2: u64,
        pub c0_un_wi: u64,
        pub c0_un_bx: u64,
        pub c1_up_b0: u64,
        pub c1_up_b1: u64,
        pub c1_up_b2: u64,
        pub c1_up_wi: u64,
        pub c1_up_bx: u64,
        pub c1_un_b0: u64,
        pub c1_un_b1: u64,
        pub c1_un_b2: u64,
        pub c1_un_wi: u64,
        pub c1_un_bx: u64,
        pub c0_un_wf: u64,
        pub c1_un_wf: u64,
        pub c0_up_wf: u64,
        pub c1_up_wf: u64,
        pub c0_exc: u64,
        pub c1_exc: u64,
        pub c0_ldwn: u64,
        pub c1_ldwn: u64,
        pub reserved_61_63: u64,
// header guard omitted
    pub cn56xxp1: cvmx_npei_bist_status_cn56xxp1,
}

#[repr(C)]
pub union cvmx_npei_int_info {
    pub u64: u64,
	struct cvmx_npei_int_info_s {
// #ifdef __BIG_ENDIAN_BITFIELD
        pub reserved_12_63: u64,
        pub pidbof: u64,
        pub psldbof: u64,
// #else (bitfield order is target-dependent)
        pub psldbof: u64,
        pub pidbof: u64,
        pub reserved_12_63: u64,
// header guard omitted
    pub s: cvmx_npei_bar1_indexx_s,
}

#[repr(C)]
pub union cvmx_npei_int_sum {
    pub u64: u64,
	struct cvmx_npei_int_sum_s {
// #ifdef __BIG_ENDIAN_BITFIELD
        pub mio_inta: u64,
        pub reserved_62_62: u64,
        pub int_a: u64,
        pub c1_ldwn: u64,
        pub c0_ldwn: u64,
        pub c1_exc: u64,
        pub c0_exc: u64,
        pub c1_up_wf: u64,
        pub c0_up_wf: u64,
        pub c1_un_wf: u64,
        pub c0_un_wf: u64,
        pub c1_un_bx: u64,
        pub c1_un_wi: u64,
        pub c1_un_b2: u64,
        pub c1_un_b1: u64,
        pub c1_un_b0: u64,
        pub c1_up_bx: u64,
        pub c1_up_wi: u64,
        pub c1_up_b2: u64,
        pub c1_up_b1: u64,
        pub c1_up_b0: u64,
        pub c0_un_bx: u64,
        pub c0_un_wi: u64,
        pub c0_un_b2: u64,
        pub c0_un_b1: u64,
        pub c0_un_b0: u64,
        pub c0_up_bx: u64,
        pub c0_up_wi: u64,
        pub c0_up_b2: u64,
        pub c0_up_b1: u64,
        pub c0_up_b0: u64,
        pub c1_hpint: u64,
        pub c1_pmei: u64,
        pub c1_wake: u64,
        pub crs1_dr: u64,
        pub c1_se: u64,
        pub crs1_er: u64,
        pub c1_aeri: u64,
        pub c0_hpint: u64,
        pub c0_pmei: u64,
        pub c0_wake: u64,
        pub crs0_dr: u64,
        pub c0_se: u64,
        pub crs0_er: u64,
        pub c0_aeri: u64,
        pub ptime: u64,
        pub pcnt: u64,
        pub pidbof: u64,
        pub psldbof: u64,
        pub dtime1: u64,
        pub dtime0: u64,
        pub dcnt1: u64,
        pub dcnt0: u64,
        pub dma1fi: u64,
        pub dma0fi: u64,
        pub dma4dbo: u64,
        pub dma3dbo: u64,
        pub dma2dbo: u64,
        pub dma1dbo: u64,
        pub dma0dbo: u64,
        pub iob2big: u64,
        pub bar0_to: u64,
        pub rml_wto: u64,
        pub rml_rto: u64,
// #else (bitfield order is target-dependent)
        pub rml_rto: u64,
        pub rml_wto: u64,
        pub bar0_to: u64,
        pub iob2big: u64,
        pub dma0dbo: u64,
        pub dma1dbo: u64,
        pub dma2dbo: u64,
        pub dma3dbo: u64,
        pub dma4dbo: u64,
        pub dma0fi: u64,
        pub dma1fi: u64,
        pub dcnt0: u64,
        pub dcnt1: u64,
        pub dtime0: u64,
        pub dtime1: u64,
        pub psldbof: u64,
        pub pidbof: u64,
        pub pcnt: u64,
        pub ptime: u64,
        pub c0_aeri: u64,
        pub crs0_er: u64,
        pub c0_se: u64,
        pub crs0_dr: u64,
        pub c0_wake: u64,
        pub c0_pmei: u64,
        pub c0_hpint: u64,
        pub c1_aeri: u64,
        pub crs1_er: u64,
        pub c1_se: u64,
        pub crs1_dr: u64,
        pub c1_wake: u64,
        pub c1_pmei: u64,
        pub c1_hpint: u64,
        pub c0_up_b0: u64,
        pub c0_up_b1: u64,
        pub c0_up_b2: u64,
        pub c0_up_wi: u64,
        pub c0_up_bx: u64,
        pub c0_un_b0: u64,
        pub c0_un_b1: u64,
        pub c0_un_b2: u64,
        pub c0_un_wi: u64,
        pub c0_un_bx: u64,
        pub c1_up_b0: u64,
        pub c1_up_b1: u64,
        pub c1_up_b2: u64,
        pub c1_up_wi: u64,
        pub c1_up_bx: u64,
        pub c1_un_b0: u64,
        pub c1_un_b1: u64,
        pub c1_un_b2: u64,
        pub c1_un_wi: u64,
        pub c1_un_bx: u64,
        pub c0_un_wf: u64,
        pub c1_un_wf: u64,
        pub c0_up_wf: u64,
        pub c1_up_wf: u64,
        pub c0_exc: u64,
        pub c1_exc: u64,
        pub c0_ldwn: u64,
        pub c1_ldwn: u64,
        pub int_a: u64,
        pub reserved_62_62: u64,
        pub mio_inta: u64,
// header guard omitted
    pub s: cvmx_npei_bar1_indexx_s,
	struct cvmx_npei_int_sum_cn52xxp1 {
// #ifdef __BIG_ENDIAN_BITFIELD
        pub mio_inta: u64,
        pub reserved_62_62: u64,
        pub int_a: u64,
        pub c1_ldwn: u64,
        pub c0_ldwn: u64,
        pub c1_exc: u64,
        pub c0_exc: u64,
        pub c1_up_wf: u64,
        pub c0_up_wf: u64,
        pub c1_un_wf: u64,
        pub c0_un_wf: u64,
        pub c1_un_bx: u64,
        pub c1_un_wi: u64,
        pub c1_un_b2: u64,
        pub c1_un_b1: u64,
        pub c1_un_b0: u64,
        pub c1_up_bx: u64,
        pub c1_up_wi: u64,
        pub c1_up_b2: u64,
        pub c1_up_b1: u64,
        pub c1_up_b0: u64,
        pub c0_un_bx: u64,
        pub c0_un_wi: u64,
        pub c0_un_b2: u64,
        pub c0_un_b1: u64,
        pub c0_un_b0: u64,
        pub c0_up_bx: u64,
        pub c0_up_wi: u64,
        pub c0_up_b2: u64,
        pub c0_up_b1: u64,
        pub c0_up_b0: u64,
        pub c1_hpint: u64,
        pub c1_pmei: u64,
        pub c1_wake: u64,
        pub crs1_dr: u64,
        pub c1_se: u64,
        pub crs1_er: u64,
        pub c1_aeri: u64,
        pub c0_hpint: u64,
        pub c0_pmei: u64,
        pub c0_wake: u64,
        pub crs0_dr: u64,
        pub c0_se: u64,
        pub crs0_er: u64,
        pub c0_aeri: u64,
        pub reserved_15_18: u64,
        pub dtime1: u64,
        pub dtime0: u64,
        pub dcnt1: u64,
        pub dcnt0: u64,
        pub dma1fi: u64,
        pub dma0fi: u64,
        pub reserved_8_8: u64,
        pub dma3dbo: u64,
        pub dma2dbo: u64,
        pub dma1dbo: u64,
        pub dma0dbo: u64,
        pub iob2big: u64,
        pub bar0_to: u64,
        pub rml_wto: u64,
        pub rml_rto: u64,
// #else (bitfield order is target-dependent)
        pub rml_rto: u64,
        pub rml_wto: u64,
        pub bar0_to: u64,
        pub iob2big: u64,
        pub dma0dbo: u64,
        pub dma1dbo: u64,
        pub dma2dbo: u64,
        pub dma3dbo: u64,
        pub reserved_8_8: u64,
        pub dma0fi: u64,
        pub dma1fi: u64,
        pub dcnt0: u64,
        pub dcnt1: u64,
        pub dtime0: u64,
        pub dtime1: u64,
        pub reserved_15_18: u64,
        pub c0_aeri: u64,
        pub crs0_er: u64,
        pub c0_se: u64,
        pub crs0_dr: u64,
        pub c0_wake: u64,
        pub c0_pmei: u64,
        pub c0_hpint: u64,
        pub c1_aeri: u64,
        pub crs1_er: u64,
        pub c1_se: u64,
        pub crs1_dr: u64,
        pub c1_wake: u64,
        pub c1_pmei: u64,
        pub c1_hpint: u64,
        pub c0_up_b0: u64,
        pub c0_up_b1: u64,
        pub c0_up_b2: u64,
        pub c0_up_wi: u64,
        pub c0_up_bx: u64,
        pub c0_un_b0: u64,
        pub c0_un_b1: u64,
        pub c0_un_b2: u64,
        pub c0_un_wi: u64,
        pub c0_un_bx: u64,
        pub c1_up_b0: u64,
        pub c1_up_b1: u64,
        pub c1_up_b2: u64,
        pub c1_up_wi: u64,
        pub c1_up_bx: u64,
        pub c1_un_b0: u64,
        pub c1_un_b1: u64,
        pub c1_un_b2: u64,
        pub c1_un_wi: u64,
        pub c1_un_bx: u64,
        pub c0_un_wf: u64,
        pub c1_un_wf: u64,
        pub c0_up_wf: u64,
        pub c1_up_wf: u64,
        pub c0_exc: u64,
        pub c1_exc: u64,
        pub c0_ldwn: u64,
        pub c1_ldwn: u64,
        pub int_a: u64,
        pub reserved_62_62: u64,
        pub mio_inta: u64,
// header guard omitted
    pub cn52xxp1: cvmx_npei_bist_status_cn52xxp1,
	struct cvmx_npei_int_sum_cn56xxp1 {
// #ifdef __BIG_ENDIAN_BITFIELD
        pub mio_inta: u64,
        pub reserved_61_62: u64,
        pub c1_ldwn: u64,
        pub c0_ldwn: u64,
        pub c1_exc: u64,
        pub c0_exc: u64,
        pub c1_up_wf: u64,
        pub c0_up_wf: u64,
        pub c1_un_wf: u64,
        pub c0_un_wf: u64,
        pub c1_un_bx: u64,
        pub c1_un_wi: u64,
        pub c1_un_b2: u64,
        pub c1_un_b1: u64,
        pub c1_un_b0: u64,
        pub c1_up_bx: u64,
        pub c1_up_wi: u64,
        pub c1_up_b2: u64,
        pub c1_up_b1: u64,
        pub c1_up_b0: u64,
        pub c0_un_bx: u64,
        pub c0_un_wi: u64,
        pub c0_un_b2: u64,
        pub c0_un_b1: u64,
        pub c0_un_b0: u64,
        pub c0_up_bx: u64,
        pub c0_up_wi: u64,
        pub c0_up_b2: u64,
        pub c0_up_b1: u64,
        pub c0_up_b0: u64,
        pub c1_hpint: u64,
        pub c1_pmei: u64,
        pub c1_wake: u64,
        pub reserved_29_29: u64,
        pub c1_se: u64,
        pub reserved_27_27: u64,
        pub c1_aeri: u64,
        pub c0_hpint: u64,
        pub c0_pmei: u64,
        pub c0_wake: u64,
        pub reserved_22_22: u64,
        pub c0_se: u64,
        pub reserved_20_20: u64,
        pub c0_aeri: u64,
        pub reserved_15_18: u64,
        pub dtime1: u64,
        pub dtime0: u64,
        pub dcnt1: u64,
        pub dcnt0: u64,
        pub dma1fi: u64,
        pub dma0fi: u64,
        pub dma4dbo: u64,
        pub dma3dbo: u64,
        pub dma2dbo: u64,
        pub dma1dbo: u64,
        pub dma0dbo: u64,
        pub iob2big: u64,
        pub bar0_to: u64,
        pub rml_wto: u64,
        pub rml_rto: u64,
// #else (bitfield order is target-dependent)
        pub rml_rto: u64,
        pub rml_wto: u64,
        pub bar0_to: u64,
        pub iob2big: u64,
        pub dma0dbo: u64,
        pub dma1dbo: u64,
        pub dma2dbo: u64,
        pub dma3dbo: u64,
        pub dma4dbo: u64,
        pub dma0fi: u64,
        pub dma1fi: u64,
        pub dcnt0: u64,
        pub dcnt1: u64,
        pub dtime0: u64,
        pub dtime1: u64,
        pub reserved_15_18: u64,
        pub c0_aeri: u64,
        pub reserved_20_20: u64,
        pub c0_se: u64,
        pub reserved_22_22: u64,
        pub c0_wake: u64,
        pub c0_pmei: u64,
        pub c0_hpint: u64,
        pub c1_aeri: u64,
        pub reserved_27_27: u64,
        pub c1_se: u64,
        pub reserved_29_29: u64,
        pub c1_wake: u64,
        pub c1_pmei: u64,
        pub c1_hpint: u64,
        pub c0_up_b0: u64,
        pub c0_up_b1: u64,
        pub c0_up_b2: u64,
        pub c0_up_wi: u64,
        pub c0_up_bx: u64,
        pub c0_un_b0: u64,
        pub c0_un_b1: u64,
        pub c0_un_b2: u64,
        pub c0_un_wi: u64,
        pub c0_un_bx: u64,
        pub c1_up_b0: u64,
        pub c1_up_b1: u64,
        pub c1_up_b2: u64,
        pub c1_up_wi: u64,
        pub c1_up_bx: u64,
        pub c1_un_b0: u64,
        pub c1_un_b1: u64,
        pub c1_un_b2: u64,
        pub c1_un_wi: u64,
        pub c1_un_bx: u64,
        pub c0_un_wf: u64,
        pub c1_un_wf: u64,
        pub c0_up_wf: u64,
        pub c1_up_wf: u64,
        pub c0_exc: u64,
        pub c1_exc: u64,
        pub c0_ldwn: u64,
        pub c1_ldwn: u64,
        pub reserved_61_62: u64,
        pub mio_inta: u64,
// header guard omitted
    pub cn56xxp1: cvmx_npei_bist_status_cn56xxp1,
}

#[repr(C)]
pub union cvmx_npei_int_sum2 {
    pub u64: u64,
	struct cvmx_npei_int_sum2_s {
// #ifdef __BIG_ENDIAN_BITFIELD
        pub mio_inta: u64,
        pub reserved_62_62: u64,
        pub int_a: u64,
        pub c1_ldwn: u64,
        pub c0_ldwn: u64,
        pub c1_exc: u64,
        pub c0_exc: u64,
        pub c1_up_wf: u64,
        pub c0_up_wf: u64,
        pub c1_un_wf: u64,
        pub c0_un_wf: u64,
        pub c1_un_bx: u64,
        pub c1_un_wi: u64,
        pub c1_un_b2: u64,
        pub c1_un_b1: u64,
        pub c1_un_b0: u64,
        pub c1_up_bx: u64,
        pub c1_up_wi: u64,
        pub c1_up_b2: u64,
        pub c1_up_b1: u64,
        pub c1_up_b0: u64,
        pub c0_un_bx: u64,
        pub c0_un_wi: u64,
        pub c0_un_b2: u64,
        pub c0_un_b1: u64,
        pub c0_un_b0: u64,
        pub c0_up_bx: u64,
        pub c0_up_wi: u64,
        pub c0_up_b2: u64,
        pub c0_up_b1: u64,
        pub c0_up_b0: u64,
        pub c1_hpint: u64,
        pub c1_pmei: u64,
        pub c1_wake: u64,
        pub crs1_dr: u64,
        pub c1_se: u64,
        pub crs1_er: u64,
        pub c1_aeri: u64,
        pub c0_hpint: u64,
        pub c0_pmei: u64,
        pub c0_wake: u64,
        pub crs0_dr: u64,
        pub c0_se: u64,
        pub crs0_er: u64,
        pub c0_aeri: u64,
        pub reserved_15_18: u64,
        pub dtime1: u64,
        pub dtime0: u64,
        pub dcnt1: u64,
        pub dcnt0: u64,
        pub dma1fi: u64,
        pub dma0fi: u64,
        pub reserved_8_8: u64,
        pub dma3dbo: u64,
        pub dma2dbo: u64,
        pub dma1dbo: u64,
        pub dma0dbo: u64,
        pub iob2big: u64,
        pub bar0_to: u64,
        pub rml_wto: u64,
        pub rml_rto: u64,
// #else (bitfield order is target-dependent)
        pub rml_rto: u64,
        pub rml_wto: u64,
        pub bar0_to: u64,
        pub iob2big: u64,
        pub dma0dbo: u64,
        pub dma1dbo: u64,
        pub dma2dbo: u64,
        pub dma3dbo: u64,
        pub reserved_8_8: u64,
        pub dma0fi: u64,
        pub dma1fi: u64,
        pub dcnt0: u64,
        pub dcnt1: u64,
        pub dtime0: u64,
        pub dtime1: u64,
        pub reserved_15_18: u64,
        pub c0_aeri: u64,
        pub crs0_er: u64,
        pub c0_se: u64,
        pub crs0_dr: u64,
        pub c0_wake: u64,
        pub c0_pmei: u64,
        pub c0_hpint: u64,
        pub c1_aeri: u64,
        pub crs1_er: u64,
        pub c1_se: u64,
        pub crs1_dr: u64,
        pub c1_wake: u64,
        pub c1_pmei: u64,
        pub c1_hpint: u64,
        pub c0_up_b0: u64,
        pub c0_up_b1: u64,
        pub c0_up_b2: u64,
        pub c0_up_wi: u64,
        pub c0_up_bx: u64,
        pub c0_un_b0: u64,
        pub c0_un_b1: u64,
        pub c0_un_b2: u64,
        pub c0_un_wi: u64,
        pub c0_un_bx: u64,
        pub c1_up_b0: u64,
        pub c1_up_b1: u64,
        pub c1_up_b2: u64,
        pub c1_up_wi: u64,
        pub c1_up_bx: u64,
        pub c1_un_b0: u64,
        pub c1_un_b1: u64,
        pub c1_un_b2: u64,
        pub c1_un_wi: u64,
        pub c1_un_bx: u64,
        pub c0_un_wf: u64,
        pub c1_un_wf: u64,
        pub c0_up_wf: u64,
        pub c1_up_wf: u64,
        pub c0_exc: u64,
        pub c1_exc: u64,
        pub c0_ldwn: u64,
        pub c1_ldwn: u64,
        pub int_a: u64,
        pub reserved_62_62: u64,
        pub mio_inta: u64,
// header guard omitted
    pub s: cvmx_npei_bar1_indexx_s,
}

#[repr(C)]
pub union cvmx_npei_last_win_rdata0 {
    pub u64: u64,
	struct cvmx_npei_last_win_rdata0_s {
// #ifdef __BIG_ENDIAN_BITFIELD
        pub data: u64,
// #else (bitfield order is target-dependent)
        pub data: u64,
// header guard omitted
    pub s: cvmx_npei_bar1_indexx_s,
}

#[repr(C)]
pub union cvmx_npei_last_win_rdata1 {
    pub u64: u64,
	struct cvmx_npei_last_win_rdata1_s {
// #ifdef __BIG_ENDIAN_BITFIELD
        pub data: u64,
// #else (bitfield order is target-dependent)
        pub data: u64,
// header guard omitted
    pub s: cvmx_npei_bar1_indexx_s,
}

#[repr(C)]
pub union cvmx_npei_mem_access_ctl {
    pub u64: u64,
	struct cvmx_npei_mem_access_ctl_s {
// #ifdef __BIG_ENDIAN_BITFIELD
        pub reserved_14_63: u64,
        pub max_word: u64,
        pub timer: u64,
// #else (bitfield order is target-dependent)
        pub timer: u64,
        pub max_word: u64,
        pub reserved_14_63: u64,
// header guard omitted
    pub s: cvmx_npei_bar1_indexx_s,
}

#[repr(C)]
pub union cvmx_npei_mem_access_subidx {
    pub u64: u64,
	struct cvmx_npei_mem_access_subidx_s {
// #ifdef __BIG_ENDIAN_BITFIELD
        pub reserved_42_63: u64,
        pub zero: u64,
        pub port: u64,
        pub nmerge: u64,
        pub esr: u64,
        pub esw: u64,
        pub nsr: u64,
        pub nsw: u64,
        pub ror: u64,
        pub row: u64,
        pub ba: u64,
// #else (bitfield order is target-dependent)
        pub ba: u64,
        pub row: u64,
        pub ror: u64,
        pub nsw: u64,
        pub nsr: u64,
        pub esw: u64,
        pub esr: u64,
        pub nmerge: u64,
        pub port: u64,
        pub zero: u64,
        pub reserved_42_63: u64,
// header guard omitted
    pub s: cvmx_npei_bar1_indexx_s,
}

#[repr(C)]
pub union cvmx_npei_msi_enb0 {
    pub u64: u64,
	struct cvmx_npei_msi_enb0_s {
// #ifdef __BIG_ENDIAN_BITFIELD
        pub enb: u64,
// #else (bitfield order is target-dependent)
        pub enb: u64,
// header guard omitted
    pub s: cvmx_npei_bar1_indexx_s,
}

#[repr(C)]
pub union cvmx_npei_msi_enb1 {
    pub u64: u64,
	struct cvmx_npei_msi_enb1_s {
// #ifdef __BIG_ENDIAN_BITFIELD
        pub enb: u64,
// #else (bitfield order is target-dependent)
        pub enb: u64,
// header guard omitted
    pub s: cvmx_npei_bar1_indexx_s,
}

#[repr(C)]
pub union cvmx_npei_msi_enb2 {
    pub u64: u64,
	struct cvmx_npei_msi_enb2_s {
// #ifdef __BIG_ENDIAN_BITFIELD
        pub enb: u64,
// #else (bitfield order is target-dependent)
        pub enb: u64,
// header guard omitted
    pub s: cvmx_npei_bar1_indexx_s,
}

#[repr(C)]
pub union cvmx_npei_msi_enb3 {
    pub u64: u64,
	struct cvmx_npei_msi_enb3_s {
// #ifdef __BIG_ENDIAN_BITFIELD
        pub enb: u64,
// #else (bitfield order is target-dependent)
        pub enb: u64,
// header guard omitted
    pub s: cvmx_npei_bar1_indexx_s,
}

#[repr(C)]
pub union cvmx_npei_msi_rcv0 {
    pub u64: u64,
	struct cvmx_npei_msi_rcv0_s {
// #ifdef __BIG_ENDIAN_BITFIELD
        pub intr: u64,
// #else (bitfield order is target-dependent)
        pub intr: u64,
// header guard omitted
    pub s: cvmx_npei_bar1_indexx_s,
}

#[repr(C)]
pub union cvmx_npei_msi_rcv1 {
    pub u64: u64,
	struct cvmx_npei_msi_rcv1_s {
// #ifdef __BIG_ENDIAN_BITFIELD
        pub intr: u64,
// #else (bitfield order is target-dependent)
        pub intr: u64,
// header guard omitted
    pub s: cvmx_npei_bar1_indexx_s,
}

#[repr(C)]
pub union cvmx_npei_msi_rcv2 {
    pub u64: u64,
	struct cvmx_npei_msi_rcv2_s {
// #ifdef __BIG_ENDIAN_BITFIELD
        pub intr: u64,
// #else (bitfield order is target-dependent)
        pub intr: u64,
// header guard omitted
    pub s: cvmx_npei_bar1_indexx_s,
}

#[repr(C)]
pub union cvmx_npei_msi_rcv3 {
    pub u64: u64,
	struct cvmx_npei_msi_rcv3_s {
// #ifdef __BIG_ENDIAN_BITFIELD
        pub intr: u64,
// #else (bitfield order is target-dependent)
        pub intr: u64,
// header guard omitted
    pub s: cvmx_npei_bar1_indexx_s,
}

#[repr(C)]
pub union cvmx_npei_msi_rd_map {
    pub u64: u64,
	struct cvmx_npei_msi_rd_map_s {
// #ifdef __BIG_ENDIAN_BITFIELD
        pub reserved_16_63: u64,
        pub rd_int: u64,
        pub msi_int: u64,
// #else (bitfield order is target-dependent)
        pub msi_int: u64,
        pub rd_int: u64,
        pub reserved_16_63: u64,
// header guard omitted
    pub s: cvmx_npei_bar1_indexx_s,
}

#[repr(C)]
pub union cvmx_npei_msi_w1c_enb0 {
    pub u64: u64,
	struct cvmx_npei_msi_w1c_enb0_s {
// #ifdef __BIG_ENDIAN_BITFIELD
        pub clr: u64,
// #else (bitfield order is target-dependent)
        pub clr: u64,
// header guard omitted
    pub s: cvmx_npei_bar1_indexx_s,
}

#[repr(C)]
pub union cvmx_npei_msi_w1c_enb1 {
    pub u64: u64,
	struct cvmx_npei_msi_w1c_enb1_s {
// #ifdef __BIG_ENDIAN_BITFIELD
        pub clr: u64,
// #else (bitfield order is target-dependent)
        pub clr: u64,
// header guard omitted
    pub s: cvmx_npei_bar1_indexx_s,
}

#[repr(C)]
pub union cvmx_npei_msi_w1c_enb2 {
    pub u64: u64,
	struct cvmx_npei_msi_w1c_enb2_s {
// #ifdef __BIG_ENDIAN_BITFIELD
        pub clr: u64,
// #else (bitfield order is target-dependent)
        pub clr: u64,
// header guard omitted
    pub s: cvmx_npei_bar1_indexx_s,
}

#[repr(C)]
pub union cvmx_npei_msi_w1c_enb3 {
    pub u64: u64,
	struct cvmx_npei_msi_w1c_enb3_s {
// #ifdef __BIG_ENDIAN_BITFIELD
        pub clr: u64,
// #else (bitfield order is target-dependent)
        pub clr: u64,
// header guard omitted
    pub s: cvmx_npei_bar1_indexx_s,
}

#[repr(C)]
pub union cvmx_npei_msi_w1s_enb0 {
    pub u64: u64,
	struct cvmx_npei_msi_w1s_enb0_s {
// #ifdef __BIG_ENDIAN_BITFIELD
        pub set: u64,
// #else (bitfield order is target-dependent)
        pub set: u64,
// header guard omitted
    pub s: cvmx_npei_bar1_indexx_s,
}

#[repr(C)]
pub union cvmx_npei_msi_w1s_enb1 {
    pub u64: u64,
	struct cvmx_npei_msi_w1s_enb1_s {
// #ifdef __BIG_ENDIAN_BITFIELD
        pub set: u64,
// #else (bitfield order is target-dependent)
        pub set: u64,
// header guard omitted
    pub s: cvmx_npei_bar1_indexx_s,
}

#[repr(C)]
pub union cvmx_npei_msi_w1s_enb2 {
    pub u64: u64,
	struct cvmx_npei_msi_w1s_enb2_s {
// #ifdef __BIG_ENDIAN_BITFIELD
        pub set: u64,
// #else (bitfield order is target-dependent)
        pub set: u64,
// header guard omitted
    pub s: cvmx_npei_bar1_indexx_s,
}

#[repr(C)]
pub union cvmx_npei_msi_w1s_enb3 {
    pub u64: u64,
	struct cvmx_npei_msi_w1s_enb3_s {
// #ifdef __BIG_ENDIAN_BITFIELD
        pub set: u64,
// #else (bitfield order is target-dependent)
        pub set: u64,
// header guard omitted
    pub s: cvmx_npei_bar1_indexx_s,
}

#[repr(C)]
pub union cvmx_npei_msi_wr_map {
    pub u64: u64,
	struct cvmx_npei_msi_wr_map_s {
// #ifdef __BIG_ENDIAN_BITFIELD
        pub reserved_16_63: u64,
        pub ciu_int: u64,
        pub msi_int: u64,
// #else (bitfield order is target-dependent)
        pub msi_int: u64,
        pub ciu_int: u64,
        pub reserved_16_63: u64,
// header guard omitted
    pub s: cvmx_npei_bar1_indexx_s,
}

#[repr(C)]
pub union cvmx_npei_pcie_credit_cnt {
    pub u64: u64,
	struct cvmx_npei_pcie_credit_cnt_s {
// #ifdef __BIG_ENDIAN_BITFIELD
        pub reserved_48_63: u64,
        pub p1_ccnt: u64,
        pub p1_ncnt: u64,
        pub p1_pcnt: u64,
        pub p0_ccnt: u64,
        pub p0_ncnt: u64,
        pub p0_pcnt: u64,
// #else (bitfield order is target-dependent)
        pub p0_pcnt: u64,
        pub p0_ncnt: u64,
        pub p0_ccnt: u64,
        pub p1_pcnt: u64,
        pub p1_ncnt: u64,
        pub p1_ccnt: u64,
        pub reserved_48_63: u64,
// header guard omitted
    pub s: cvmx_npei_bar1_indexx_s,
}

#[repr(C)]
pub union cvmx_npei_pcie_msi_rcv {
    pub u64: u64,
	struct cvmx_npei_pcie_msi_rcv_s {
// #ifdef __BIG_ENDIAN_BITFIELD
        pub reserved_8_63: u64,
        pub intr: u64,
// #else (bitfield order is target-dependent)
        pub intr: u64,
        pub reserved_8_63: u64,
// header guard omitted
    pub s: cvmx_npei_bar1_indexx_s,
}

#[repr(C)]
pub union cvmx_npei_pcie_msi_rcv_b1 {
    pub u64: u64,
	struct cvmx_npei_pcie_msi_rcv_b1_s {
// #ifdef __BIG_ENDIAN_BITFIELD
        pub reserved_16_63: u64,
        pub intr: u64,
        pub reserved_0_7: u64,
// #else (bitfield order is target-dependent)
        pub reserved_0_7: u64,
        pub intr: u64,
        pub reserved_16_63: u64,
// header guard omitted
    pub s: cvmx_npei_bar1_indexx_s,
}

#[repr(C)]
pub union cvmx_npei_pcie_msi_rcv_b2 {
    pub u64: u64,
	struct cvmx_npei_pcie_msi_rcv_b2_s {
// #ifdef __BIG_ENDIAN_BITFIELD
        pub reserved_24_63: u64,
        pub intr: u64,
        pub reserved_0_15: u64,
// #else (bitfield order is target-dependent)
        pub reserved_0_15: u64,
        pub intr: u64,
        pub reserved_24_63: u64,
// header guard omitted
    pub s: cvmx_npei_bar1_indexx_s,
}

#[repr(C)]
pub union cvmx_npei_pcie_msi_rcv_b3 {
    pub u64: u64,
	struct cvmx_npei_pcie_msi_rcv_b3_s {
// #ifdef __BIG_ENDIAN_BITFIELD
        pub reserved_32_63: u64,
        pub intr: u64,
        pub reserved_0_23: u64,
// #else (bitfield order is target-dependent)
        pub reserved_0_23: u64,
        pub intr: u64,
        pub reserved_32_63: u64,
// header guard omitted
    pub s: cvmx_npei_bar1_indexx_s,
}

#[repr(C)]
pub union cvmx_npei_pktx_cnts {
    pub u64: u64,
	struct cvmx_npei_pktx_cnts_s {
// #ifdef __BIG_ENDIAN_BITFIELD
        pub reserved_54_63: u64,
        pub timer: u64,
        pub cnt: u64,
// #else (bitfield order is target-dependent)
        pub cnt: u64,
        pub timer: u64,
        pub reserved_54_63: u64,
// header guard omitted
    pub s: cvmx_npei_bar1_indexx_s,
}

#[repr(C)]
pub union cvmx_npei_pktx_in_bp {
    pub u64: u64,
	struct cvmx_npei_pktx_in_bp_s {
// #ifdef __BIG_ENDIAN_BITFIELD
        pub wmark: u64,
        pub cnt: u64,
// #else (bitfield order is target-dependent)
        pub cnt: u64,
        pub wmark: u64,
// header guard omitted
    pub s: cvmx_npei_bar1_indexx_s,
}

#[repr(C)]
pub union cvmx_npei_pktx_instr_baddr {
    pub u64: u64,
	struct cvmx_npei_pktx_instr_baddr_s {
// #ifdef __BIG_ENDIAN_BITFIELD
        pub addr: u64,
        pub reserved_0_2: u64,
// #else (bitfield order is target-dependent)
        pub reserved_0_2: u64,
        pub addr: u64,
// header guard omitted
    pub s: cvmx_npei_bar1_indexx_s,
}

#[repr(C)]
pub union cvmx_npei_pktx_instr_baoff_dbell {
    pub u64: u64,
	struct cvmx_npei_pktx_instr_baoff_dbell_s {
// #ifdef __BIG_ENDIAN_BITFIELD
        pub aoff: u64,
        pub dbell: u64,
// #else (bitfield order is target-dependent)
        pub dbell: u64,
        pub aoff: u64,
// header guard omitted
    pub s: cvmx_npei_bar1_indexx_s,
}

#[repr(C)]
pub union cvmx_npei_pktx_instr_fifo_rsize {
    pub u64: u64,
	struct cvmx_npei_pktx_instr_fifo_rsize_s {
// #ifdef __BIG_ENDIAN_BITFIELD
        pub max: u64,
        pub rrp: u64,
        pub wrp: u64,
        pub fcnt: u64,
        pub rsize: u64,
// #else (bitfield order is target-dependent)
        pub rsize: u64,
        pub fcnt: u64,
        pub wrp: u64,
        pub rrp: u64,
        pub max: u64,
// header guard omitted
    pub s: cvmx_npei_bar1_indexx_s,
}

#[repr(C)]
pub union cvmx_npei_pktx_instr_header {
    pub u64: u64,
	struct cvmx_npei_pktx_instr_header_s {
// #ifdef __BIG_ENDIAN_BITFIELD
        pub reserved_44_63: u64,
        pub pbp: u64,
        pub reserved_38_42: u64,
        pub rparmode: u64,
        pub reserved_35_35: u64,
        pub rskp_len: u64,
        pub reserved_22_27: u64,
        pub use_ihdr: u64,
        pub reserved_16_20: u64,
        pub par_mode: u64,
        pub reserved_13_13: u64,
        pub skp_len: u64,
        pub reserved_0_5: u64,
// #else (bitfield order is target-dependent)
        pub reserved_0_5: u64,
        pub skp_len: u64,
        pub reserved_13_13: u64,
        pub par_mode: u64,
        pub reserved_16_20: u64,
        pub use_ihdr: u64,
        pub reserved_22_27: u64,
        pub rskp_len: u64,
        pub reserved_35_35: u64,
        pub rparmode: u64,
        pub reserved_38_42: u64,
        pub pbp: u64,
        pub reserved_44_63: u64,
// header guard omitted
    pub s: cvmx_npei_bar1_indexx_s,
}

#[repr(C)]
pub union cvmx_npei_pktx_slist_baddr {
    pub u64: u64,
	struct cvmx_npei_pktx_slist_baddr_s {
// #ifdef __BIG_ENDIAN_BITFIELD
        pub addr: u64,
        pub reserved_0_3: u64,
// #else (bitfield order is target-dependent)
        pub reserved_0_3: u64,
        pub addr: u64,
// header guard omitted
    pub s: cvmx_npei_bar1_indexx_s,
}

#[repr(C)]
pub union cvmx_npei_pktx_slist_baoff_dbell {
    pub u64: u64,
	struct cvmx_npei_pktx_slist_baoff_dbell_s {
// #ifdef __BIG_ENDIAN_BITFIELD
        pub aoff: u64,
        pub dbell: u64,
// #else (bitfield order is target-dependent)
        pub dbell: u64,
        pub aoff: u64,
// header guard omitted
    pub s: cvmx_npei_bar1_indexx_s,
}

#[repr(C)]
pub union cvmx_npei_pktx_slist_fifo_rsize {
    pub u64: u64,
	struct cvmx_npei_pktx_slist_fifo_rsize_s {
// #ifdef __BIG_ENDIAN_BITFIELD
        pub reserved_32_63: u64,
        pub rsize: u64,
// #else (bitfield order is target-dependent)
        pub rsize: u64,
        pub reserved_32_63: u64,
// header guard omitted
    pub s: cvmx_npei_bar1_indexx_s,
}

#[repr(C)]
pub union cvmx_npei_pkt_cnt_int {
    pub u64: u64,
	struct cvmx_npei_pkt_cnt_int_s {
// #ifdef __BIG_ENDIAN_BITFIELD
        pub reserved_32_63: u64,
        pub port: u64,
// #else (bitfield order is target-dependent)
        pub port: u64,
        pub reserved_32_63: u64,
// header guard omitted
    pub s: cvmx_npei_bar1_indexx_s,
}

#[repr(C)]
pub union cvmx_npei_pkt_cnt_int_enb {
    pub u64: u64,
	struct cvmx_npei_pkt_cnt_int_enb_s {
// #ifdef __BIG_ENDIAN_BITFIELD
        pub reserved_32_63: u64,
        pub port: u64,
// #else (bitfield order is target-dependent)
        pub port: u64,
        pub reserved_32_63: u64,
// header guard omitted
    pub s: cvmx_npei_bar1_indexx_s,
}

#[repr(C)]
pub union cvmx_npei_pkt_data_out_es {
    pub u64: u64,
	struct cvmx_npei_pkt_data_out_es_s {
// #ifdef __BIG_ENDIAN_BITFIELD
        pub es: u64,
// #else (bitfield order is target-dependent)
        pub es: u64,
// header guard omitted
    pub s: cvmx_npei_bar1_indexx_s,
}

#[repr(C)]
pub union cvmx_npei_pkt_data_out_ns {
    pub u64: u64,
	struct cvmx_npei_pkt_data_out_ns_s {
// #ifdef __BIG_ENDIAN_BITFIELD
        pub reserved_32_63: u64,
        pub nsr: u64,
// #else (bitfield order is target-dependent)
        pub nsr: u64,
        pub reserved_32_63: u64,
// header guard omitted
    pub s: cvmx_npei_bar1_indexx_s,
}

#[repr(C)]
pub union cvmx_npei_pkt_data_out_ror {
    pub u64: u64,
	struct cvmx_npei_pkt_data_out_ror_s {
// #ifdef __BIG_ENDIAN_BITFIELD
        pub reserved_32_63: u64,
        pub ror: u64,
// #else (bitfield order is target-dependent)
        pub ror: u64,
        pub reserved_32_63: u64,
// header guard omitted
    pub s: cvmx_npei_bar1_indexx_s,
}

#[repr(C)]
pub union cvmx_npei_pkt_dpaddr {
    pub u64: u64,
	struct cvmx_npei_pkt_dpaddr_s {
// #ifdef __BIG_ENDIAN_BITFIELD
        pub reserved_32_63: u64,
        pub dptr: u64,
// #else (bitfield order is target-dependent)
        pub dptr: u64,
        pub reserved_32_63: u64,
// header guard omitted
    pub s: cvmx_npei_bar1_indexx_s,
}

#[repr(C)]
pub union cvmx_npei_pkt_in_bp {
    pub u64: u64,
	struct cvmx_npei_pkt_in_bp_s {
// #ifdef __BIG_ENDIAN_BITFIELD
        pub reserved_32_63: u64,
        pub bp: u64,
// #else (bitfield order is target-dependent)
        pub bp: u64,
        pub reserved_32_63: u64,
// header guard omitted
    pub s: cvmx_npei_bar1_indexx_s,
}

#[repr(C)]
pub union cvmx_npei_pkt_in_donex_cnts {
    pub u64: u64,
	struct cvmx_npei_pkt_in_donex_cnts_s {
// #ifdef __BIG_ENDIAN_BITFIELD
        pub reserved_32_63: u64,
        pub cnt: u64,
// #else (bitfield order is target-dependent)
        pub cnt: u64,
        pub reserved_32_63: u64,
// header guard omitted
    pub s: cvmx_npei_bar1_indexx_s,
}

#[repr(C)]
pub union cvmx_npei_pkt_in_instr_counts {
    pub u64: u64,
	struct cvmx_npei_pkt_in_instr_counts_s {
// #ifdef __BIG_ENDIAN_BITFIELD
        pub wr_cnt: u64,
        pub rd_cnt: u64,
// #else (bitfield order is target-dependent)
        pub rd_cnt: u64,
        pub wr_cnt: u64,
// header guard omitted
    pub s: cvmx_npei_bar1_indexx_s,
}

#[repr(C)]
pub union cvmx_npei_pkt_in_pcie_port {
    pub u64: u64,
	struct cvmx_npei_pkt_in_pcie_port_s {
// #ifdef __BIG_ENDIAN_BITFIELD
        pub pp: u64,
// #else (bitfield order is target-dependent)
        pub pp: u64,
// header guard omitted
    pub s: cvmx_npei_bar1_indexx_s,
}

#[repr(C)]
pub union cvmx_npei_pkt_input_control {
    pub u64: u64,
	struct cvmx_npei_pkt_input_control_s {
// #ifdef __BIG_ENDIAN_BITFIELD
        pub reserved_23_63: u64,
        pub pkt_rr: u64,
        pub pbp_dhi: u64,
        pub d_nsr: u64,
        pub d_esr: u64,
        pub d_ror: u64,
        pub use_csr: u64,
        pub nsr: u64,
        pub esr: u64,
        pub ror: u64,
// #else (bitfield order is target-dependent)
        pub ror: u64,
        pub esr: u64,
        pub nsr: u64,
        pub use_csr: u64,
        pub d_ror: u64,
        pub d_esr: u64,
        pub d_nsr: u64,
        pub pbp_dhi: u64,
        pub pkt_rr: u64,
        pub reserved_23_63: u64,
// header guard omitted
    pub s: cvmx_npei_bar1_indexx_s,
}

#[repr(C)]
pub union cvmx_npei_pkt_instr_enb {
    pub u64: u64,
	struct cvmx_npei_pkt_instr_enb_s {
// #ifdef __BIG_ENDIAN_BITFIELD
        pub reserved_32_63: u64,
        pub enb: u64,
// #else (bitfield order is target-dependent)
        pub enb: u64,
        pub reserved_32_63: u64,
// header guard omitted
    pub s: cvmx_npei_bar1_indexx_s,
}

#[repr(C)]
pub union cvmx_npei_pkt_instr_rd_size {
    pub u64: u64,
	struct cvmx_npei_pkt_instr_rd_size_s {
// #ifdef __BIG_ENDIAN_BITFIELD
        pub rdsize: u64,
// #else (bitfield order is target-dependent)
        pub rdsize: u64,
// header guard omitted
    pub s: cvmx_npei_bar1_indexx_s,
}

#[repr(C)]
pub union cvmx_npei_pkt_instr_size {
    pub u64: u64,
	struct cvmx_npei_pkt_instr_size_s {
// #ifdef __BIG_ENDIAN_BITFIELD
        pub reserved_32_63: u64,
        pub is_64b: u64,
// #else (bitfield order is target-dependent)
        pub is_64b: u64,
        pub reserved_32_63: u64,
// header guard omitted
    pub s: cvmx_npei_bar1_indexx_s,
}

#[repr(C)]
pub union cvmx_npei_pkt_int_levels {
    pub u64: u64,
	struct cvmx_npei_pkt_int_levels_s {
// #ifdef __BIG_ENDIAN_BITFIELD
        pub reserved_54_63: u64,
        pub time: u64,
        pub cnt: u64,
// #else (bitfield order is target-dependent)
        pub cnt: u64,
        pub time: u64,
        pub reserved_54_63: u64,
// header guard omitted
    pub s: cvmx_npei_bar1_indexx_s,
}

#[repr(C)]
pub union cvmx_npei_pkt_iptr {
    pub u64: u64,
	struct cvmx_npei_pkt_iptr_s {
// #ifdef __BIG_ENDIAN_BITFIELD
        pub reserved_32_63: u64,
        pub iptr: u64,
// #else (bitfield order is target-dependent)
        pub iptr: u64,
        pub reserved_32_63: u64,
// header guard omitted
    pub s: cvmx_npei_bar1_indexx_s,
}

#[repr(C)]
pub union cvmx_npei_pkt_out_bmode {
    pub u64: u64,
	struct cvmx_npei_pkt_out_bmode_s {
// #ifdef __BIG_ENDIAN_BITFIELD
        pub reserved_32_63: u64,
        pub bmode: u64,
// #else (bitfield order is target-dependent)
        pub bmode: u64,
        pub reserved_32_63: u64,
// header guard omitted
    pub s: cvmx_npei_bar1_indexx_s,
}

#[repr(C)]
pub union cvmx_npei_pkt_out_enb {
    pub u64: u64,
	struct cvmx_npei_pkt_out_enb_s {
// #ifdef __BIG_ENDIAN_BITFIELD
        pub reserved_32_63: u64,
        pub enb: u64,
// #else (bitfield order is target-dependent)
        pub enb: u64,
        pub reserved_32_63: u64,
// header guard omitted
    pub s: cvmx_npei_bar1_indexx_s,
}

#[repr(C)]
pub union cvmx_npei_pkt_output_wmark {
    pub u64: u64,
	struct cvmx_npei_pkt_output_wmark_s {
// #ifdef __BIG_ENDIAN_BITFIELD
        pub reserved_32_63: u64,
        pub wmark: u64,
// #else (bitfield order is target-dependent)
        pub wmark: u64,
        pub reserved_32_63: u64,
// header guard omitted
    pub s: cvmx_npei_bar1_indexx_s,
}

#[repr(C)]
pub union cvmx_npei_pkt_pcie_port {
    pub u64: u64,
	struct cvmx_npei_pkt_pcie_port_s {
// #ifdef __BIG_ENDIAN_BITFIELD
        pub pp: u64,
// #else (bitfield order is target-dependent)
        pub pp: u64,
// header guard omitted
    pub s: cvmx_npei_bar1_indexx_s,
}

#[repr(C)]
pub union cvmx_npei_pkt_port_in_rst {
    pub u64: u64,
	struct cvmx_npei_pkt_port_in_rst_s {
// #ifdef __BIG_ENDIAN_BITFIELD
        pub in_rst: u64,
        pub out_rst: u64,
// #else (bitfield order is target-dependent)
        pub out_rst: u64,
        pub in_rst: u64,
// header guard omitted
    pub s: cvmx_npei_bar1_indexx_s,
}

#[repr(C)]
pub union cvmx_npei_pkt_slist_es {
    pub u64: u64,
	struct cvmx_npei_pkt_slist_es_s {
// #ifdef __BIG_ENDIAN_BITFIELD
        pub es: u64,
// #else (bitfield order is target-dependent)
        pub es: u64,
// header guard omitted
    pub s: cvmx_npei_bar1_indexx_s,
}

#[repr(C)]
pub union cvmx_npei_pkt_slist_id_size {
    pub u64: u64,
	struct cvmx_npei_pkt_slist_id_size_s {
// #ifdef __BIG_ENDIAN_BITFIELD
        pub reserved_23_63: u64,
        pub isize: u64,
        pub bsize: u64,
// #else (bitfield order is target-dependent)
        pub bsize: u64,
        pub isize: u64,
        pub reserved_23_63: u64,
// header guard omitted
    pub s: cvmx_npei_bar1_indexx_s,
}

#[repr(C)]
pub union cvmx_npei_pkt_slist_ns {
    pub u64: u64,
	struct cvmx_npei_pkt_slist_ns_s {
// #ifdef __BIG_ENDIAN_BITFIELD
        pub reserved_32_63: u64,
        pub nsr: u64,
// #else (bitfield order is target-dependent)
        pub nsr: u64,
        pub reserved_32_63: u64,
// header guard omitted
    pub s: cvmx_npei_bar1_indexx_s,
}

#[repr(C)]
pub union cvmx_npei_pkt_slist_ror {
    pub u64: u64,
	struct cvmx_npei_pkt_slist_ror_s {
// #ifdef __BIG_ENDIAN_BITFIELD
        pub reserved_32_63: u64,
        pub ror: u64,
// #else (bitfield order is target-dependent)
        pub ror: u64,
        pub reserved_32_63: u64,
// header guard omitted
    pub s: cvmx_npei_bar1_indexx_s,
}

#[repr(C)]
pub union cvmx_npei_pkt_time_int {
    pub u64: u64,
	struct cvmx_npei_pkt_time_int_s {
// #ifdef __BIG_ENDIAN_BITFIELD
        pub reserved_32_63: u64,
        pub port: u64,
// #else (bitfield order is target-dependent)
        pub port: u64,
        pub reserved_32_63: u64,
// header guard omitted
    pub s: cvmx_npei_bar1_indexx_s,
}

#[repr(C)]
pub union cvmx_npei_pkt_time_int_enb {
    pub u64: u64,
	struct cvmx_npei_pkt_time_int_enb_s {
// #ifdef __BIG_ENDIAN_BITFIELD
        pub reserved_32_63: u64,
        pub port: u64,
// #else (bitfield order is target-dependent)
        pub port: u64,
        pub reserved_32_63: u64,
// header guard omitted
    pub s: cvmx_npei_bar1_indexx_s,
}

#[repr(C)]
pub union cvmx_npei_rsl_int_blocks {
    pub u64: u64,
	struct cvmx_npei_rsl_int_blocks_s {
// #ifdef __BIG_ENDIAN_BITFIELD
        pub reserved_31_63: u64,
        pub iob: u64,
        pub lmc1: u64,
        pub agl: u64,
        pub reserved_24_27: u64,
        pub asxpcs1: u64,
        pub asxpcs0: u64,
        pub reserved_21_21: u64,
        pub pip: u64,
        pub spx1: u64,
        pub spx0: u64,
        pub lmc0: u64,
        pub l2c: u64,
        pub usb1: u64,
        pub rad: u64,
        pub usb: u64,
        pub pow: u64,
        pub tim: u64,
        pub pko: u64,
        pub ipd: u64,
        pub reserved_8_8: u64,
        pub zip: u64,
        pub dfa: u64,
        pub fpa: u64,
        pub key: u64,
        pub npei: u64,
        pub gmx1: u64,
        pub gmx0: u64,
        pub mio: u64,
// #else (bitfield order is target-dependent)
        pub mio: u64,
        pub gmx0: u64,
        pub gmx1: u64,
        pub npei: u64,
        pub key: u64,
        pub fpa: u64,
        pub dfa: u64,
        pub zip: u64,
        pub reserved_8_8: u64,
        pub ipd: u64,
        pub pko: u64,
        pub tim: u64,
        pub pow: u64,
        pub usb: u64,
        pub rad: u64,
        pub usb1: u64,
        pub l2c: u64,
        pub lmc0: u64,
        pub spx0: u64,
        pub spx1: u64,
        pub pip: u64,
        pub reserved_21_21: u64,
        pub asxpcs0: u64,
        pub asxpcs1: u64,
        pub reserved_24_27: u64,
        pub agl: u64,
        pub lmc1: u64,
        pub iob: u64,
        pub reserved_31_63: u64,
// header guard omitted
    pub s: cvmx_npei_bar1_indexx_s,
}

#[repr(C)]
pub union cvmx_npei_scratch_1 {
    pub u64: u64,
	struct cvmx_npei_scratch_1_s {
// #ifdef __BIG_ENDIAN_BITFIELD
        pub data: u64,
// #else (bitfield order is target-dependent)
        pub data: u64,
// header guard omitted
    pub s: cvmx_npei_bar1_indexx_s,
}

#[repr(C)]
pub union cvmx_npei_state1 {
    pub u64: u64,
	struct cvmx_npei_state1_s {
// #ifdef __BIG_ENDIAN_BITFIELD
        pub cpl1: u64,
        pub cpl0: u64,
        pub arb: u64,
        pub csr: u64,
// #else (bitfield order is target-dependent)
        pub csr: u64,
        pub arb: u64,
        pub cpl0: u64,
        pub cpl1: u64,
// header guard omitted
    pub s: cvmx_npei_bar1_indexx_s,
}

#[repr(C)]
pub union cvmx_npei_state2 {
    pub u64: u64,
	struct cvmx_npei_state2_s {
// #ifdef __BIG_ENDIAN_BITFIELD
        pub reserved_48_63: u64,
        pub npei: u64,
        pub rac: u64,
        pub csm1: u64,
        pub csm0: u64,
        pub nnp0: u64,
        pub nnd: u64,
// #else (bitfield order is target-dependent)
        pub nnd: u64,
        pub nnp0: u64,
        pub csm0: u64,
        pub csm1: u64,
        pub rac: u64,
        pub npei: u64,
        pub reserved_48_63: u64,
// header guard omitted
    pub s: cvmx_npei_bar1_indexx_s,
}

#[repr(C)]
pub union cvmx_npei_state3 {
    pub u64: u64,
	struct cvmx_npei_state3_s {
// #ifdef __BIG_ENDIAN_BITFIELD
        pub reserved_56_63: u64,
        pub psm1: u64,
        pub psm0: u64,
        pub nsm1: u64,
        pub nsm0: u64,
// #else (bitfield order is target-dependent)
        pub nsm0: u64,
        pub nsm1: u64,
        pub psm0: u64,
        pub psm1: u64,
        pub reserved_56_63: u64,
// header guard omitted
    pub s: cvmx_npei_bar1_indexx_s,
}

#[repr(C)]
pub union cvmx_npei_win_rd_addr {
    pub u64: u64,
	struct cvmx_npei_win_rd_addr_s {
// #ifdef __BIG_ENDIAN_BITFIELD
        pub reserved_51_63: u64,
        pub ld_cmd: u64,
        pub iobit: u64,
        pub rd_addr: u64,
// #else (bitfield order is target-dependent)
        pub rd_addr: u64,
        pub iobit: u64,
        pub ld_cmd: u64,
        pub reserved_51_63: u64,
// header guard omitted
    pub s: cvmx_npei_bar1_indexx_s,
}

#[repr(C)]
pub union cvmx_npei_win_rd_data {
    pub u64: u64,
	struct cvmx_npei_win_rd_data_s {
// #ifdef __BIG_ENDIAN_BITFIELD
        pub rd_data: u64,
// #else (bitfield order is target-dependent)
        pub rd_data: u64,
// header guard omitted
    pub s: cvmx_npei_bar1_indexx_s,
}

#[repr(C)]
pub union cvmx_npei_win_wr_addr {
    pub u64: u64,
	struct cvmx_npei_win_wr_addr_s {
// #ifdef __BIG_ENDIAN_BITFIELD
        pub reserved_49_63: u64,
        pub iobit: u64,
        pub wr_addr: u64,
        pub reserved_0_1: u64,
// #else (bitfield order is target-dependent)
        pub reserved_0_1: u64,
        pub wr_addr: u64,
        pub iobit: u64,
        pub reserved_49_63: u64,
// header guard omitted
    pub s: cvmx_npei_bar1_indexx_s,
}

#[repr(C)]
pub union cvmx_npei_win_wr_data {
    pub u64: u64,
	struct cvmx_npei_win_wr_data_s {
// #ifdef __BIG_ENDIAN_BITFIELD
        pub wr_data: u64,
// #else (bitfield order is target-dependent)
        pub wr_data: u64,
// header guard omitted
    pub s: cvmx_npei_bar1_indexx_s,
}

#[repr(C)]
pub union cvmx_npei_win_wr_mask {
    pub u64: u64,
	struct cvmx_npei_win_wr_mask_s {
// #ifdef __BIG_ENDIAN_BITFIELD
        pub reserved_8_63: u64,
        pub wr_mask: u64,
// #else (bitfield order is target-dependent)
        pub wr_mask: u64,
        pub reserved_8_63: u64,
// header guard omitted
    pub s: cvmx_npei_bar1_indexx_s,
}

#[repr(C)]
pub union cvmx_npei_window_ctl {
    pub u64: u64,
	struct cvmx_npei_window_ctl_s {
// #ifdef __BIG_ENDIAN_BITFIELD
        pub reserved_32_63: u64,
        pub time: u64,
// #else (bitfield order is target-dependent)
        pub time: u64,
        pub reserved_32_63: u64,
// header guard omitted
    pub s: cvmx_npei_bar1_indexx_s,
}

// header guard omitted

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
