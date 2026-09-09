#![allow(non_camel_case_types, non_snake_case, dead_code)]

// /***********************license start***************
//  * Author: Cavium Networks
//  *
//  * Contact: support@caviumnetworks.com
//  * This file is part of the OCTEON SDK
//  *
//  * Copyright (c) 2003-2012 Cavium Networks
//  *
//  * This file is free software; you can redistribute it and/or modify
//  * it under the terms of the GNU General Public License, Version 2, as
//  * published by the Free Software Foundation.
//  *
//  * This file is distributed in the hope that it will be useful, but
//  * AS-IS and WITHOUT ANY WARRANTY; without even the implied warranty
//  * of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE, TITLE, or
//  * NONINFRINGEMENT.  See the GNU General Public License for more
//  * details.
//  *
//  * You should have received a copy of the GNU General Public License
//  * along with this file; if not, write to the Free Software
//  * Foundation, Inc., 51 Franklin St, Fifth Floor, Boston, MA 02110-1301 USA
//  * or visit http://www.gnu.org/licenses/.
//  *
//  * This file may also be available under a different license from Cavium.
//  * Contact Cavium Networks for more information
//  ***********************license end**************************************/
// 
// #ifndef __CVMX_FPA_DEFS_H__
// #define __CVMX_FPA_DEFS_H__
// 
pub const CVMX_FPA_ADDR_RANGE_ERROR: u64 = (cvmx_add_io_seg(0x0001180028000458u64));
pub const CVMX_FPA_BIST_STATUS: u64 = (cvmx_add_io_seg(0x00011800280000E8u64));
pub const CVMX_FPA_CTL_STATUS: u64 = (cvmx_add_io_seg(0x0001180028000050u64));
pub const CVMX_FPA_FPF0_MARKS: u64 = (cvmx_add_io_seg(0x0001180028000000u64));
pub const CVMX_FPA_FPF0_SIZE: u64 = (cvmx_add_io_seg(0x0001180028000058u64));
pub const CVMX_FPA_FPF8_MARKS: u64 = (cvmx_add_io_seg(0x0001180028000240u64));
pub const CVMX_FPA_FPF8_SIZE: u64 = (cvmx_add_io_seg(0x0001180028000248u64));
pub const fn CVMX_FPA_FPFX_MARKS(offset: u64) -> u64 { (cvmx_add_io_seg(0x0001180028000008u64) + ((offset) & 7) * 8 - 8*1) }
pub const fn CVMX_FPA_FPFX_SIZE(offset: u64) -> u64 { (cvmx_add_io_seg(0x0001180028000060u64) + ((offset) & 7) * 8 - 8*1) }
pub const CVMX_FPA_INT_ENB: u64 = (cvmx_add_io_seg(0x0001180028000048u64));
pub const CVMX_FPA_INT_SUM: u64 = (cvmx_add_io_seg(0x0001180028000040u64));
pub const CVMX_FPA_PACKET_THRESHOLD: u64 = (cvmx_add_io_seg(0x0001180028000460u64));
pub const fn CVMX_FPA_POOLX_END_ADDR(offset: u64) -> u64 { (cvmx_add_io_seg(0x0001180028000358u64) + ((offset) & 15) * 8) }
pub const fn CVMX_FPA_POOLX_START_ADDR(offset: u64) -> u64 { (cvmx_add_io_seg(0x0001180028000258u64) + ((offset) & 15) * 8) }
pub const fn CVMX_FPA_POOLX_THRESHOLD(offset: u64) -> u64 { (cvmx_add_io_seg(0x0001180028000140u64) + ((offset) & 15) * 8) }
pub const CVMX_FPA_QUE8_PAGE_INDEX: u64 = (cvmx_add_io_seg(0x0001180028000250u64));
pub const fn CVMX_FPA_QUEX_AVAILABLE(offset: u64) -> u64 { (cvmx_add_io_seg(0x0001180028000098u64) + ((offset) & 15) * 8) }
pub const fn CVMX_FPA_QUEX_PAGE_INDEX(offset: u64) -> u64 { (cvmx_add_io_seg(0x00011800280000F0u64) + ((offset) & 7) * 8) }
pub const CVMX_FPA_QUE_ACT: u64 = (cvmx_add_io_seg(0x0001180028000138u64));
pub const CVMX_FPA_QUE_EXP: u64 = (cvmx_add_io_seg(0x0001180028000130u64));
pub const CVMX_FPA_WART_CTL: u64 = (cvmx_add_io_seg(0x00011800280000D8u64));
pub const CVMX_FPA_WART_STATUS: u64 = (cvmx_add_io_seg(0x00011800280000E0u64));
pub const CVMX_FPA_WQE_THRESHOLD: u64 = (cvmx_add_io_seg(0x0001180028000468u64));
pub const CVMX_FPA_CLK_COUNT: u64 = (cvmx_add_io_seg(0x00012800000000F0u64));
// 
#[repr(C)] pub union cvmx_fpa_addr_range_error { pub u64: u64, }
// 	uint64_t u64;
// 	struct cvmx_fpa_addr_range_error_s {
// #ifdef __BIG_ENDIAN_BITFIELD
// 		uint64_t reserved_38_63:26;
// 		uint64_t pool:5;
// 		uint64_t addr:33;
// #else
// 		uint64_t addr:33;
// 		uint64_t pool:5;
// 		uint64_t reserved_38_63:26;
// #endif
// 	} s;
// };
// 
#[repr(C)] pub union cvmx_fpa_bist_status { pub u64: u64, }
// 	uint64_t u64;
// 	struct cvmx_fpa_bist_status_s {
// #ifdef __BIG_ENDIAN_BITFIELD
// 		uint64_t reserved_5_63:59;
// 		uint64_t frd:1;
// 		uint64_t fpf0:1;
// 		uint64_t fpf1:1;
// 		uint64_t ffr:1;
// 		uint64_t fdr:1;
// #else
// 		uint64_t fdr:1;
// 		uint64_t ffr:1;
// 		uint64_t fpf1:1;
// 		uint64_t fpf0:1;
// 		uint64_t frd:1;
// 		uint64_t reserved_5_63:59;
// #endif
// 	} s;
// };
// 
#[repr(C)] pub union cvmx_fpa_ctl_status { pub u64: u64, }
// 	uint64_t u64;
// 	struct cvmx_fpa_ctl_status_s {
// #ifdef __BIG_ENDIAN_BITFIELD
// 		uint64_t reserved_21_63:43;
// 		uint64_t free_en:1;
// 		uint64_t ret_off:1;
// 		uint64_t req_off:1;
// 		uint64_t reset:1;
// 		uint64_t use_ldt:1;
// 		uint64_t use_stt:1;
// 		uint64_t enb:1;
// 		uint64_t mem1_err:7;
// 		uint64_t mem0_err:7;
// #else
// 		uint64_t mem0_err:7;
// 		uint64_t mem1_err:7;
// 		uint64_t enb:1;
// 		uint64_t use_stt:1;
// 		uint64_t use_ldt:1;
// 		uint64_t reset:1;
// 		uint64_t req_off:1;
// 		uint64_t ret_off:1;
// 		uint64_t free_en:1;
// 		uint64_t reserved_21_63:43;
// #endif
// 	} s;
// 	struct cvmx_fpa_ctl_status_cn30xx {
// #ifdef __BIG_ENDIAN_BITFIELD
// 		uint64_t reserved_18_63:46;
// 		uint64_t reset:1;
// 		uint64_t use_ldt:1;
// 		uint64_t use_stt:1;
// 		uint64_t enb:1;
// 		uint64_t mem1_err:7;
// 		uint64_t mem0_err:7;
// #else
// 		uint64_t mem0_err:7;
// 		uint64_t mem1_err:7;
// 		uint64_t enb:1;
// 		uint64_t use_stt:1;
// 		uint64_t use_ldt:1;
// 		uint64_t reset:1;
// 		uint64_t reserved_18_63:46;
// #endif
// 	} cn30xx;
// };
// 
#[repr(C)] pub union cvmx_fpa_fpfx_marks { pub u64: u64, }
// 	uint64_t u64;
// 	struct cvmx_fpa_fpfx_marks_s {
// #ifdef __BIG_ENDIAN_BITFIELD
// 		uint64_t reserved_22_63:42;
// 		uint64_t fpf_wr:11;
// 		uint64_t fpf_rd:11;
// #else
// 		uint64_t fpf_rd:11;
// 		uint64_t fpf_wr:11;
// 		uint64_t reserved_22_63:42;
// #endif
// 	} s;
// };
// 
#[repr(C)] pub union cvmx_fpa_fpfx_size { pub u64: u64, }
// 	uint64_t u64;
// 	struct cvmx_fpa_fpfx_size_s {
// #ifdef __BIG_ENDIAN_BITFIELD
// 		uint64_t reserved_11_63:53;
// 		uint64_t fpf_siz:11;
// #else
// 		uint64_t fpf_siz:11;
// 		uint64_t reserved_11_63:53;
// #endif
// 	} s;
// };
// 
#[repr(C)] pub union cvmx_fpa_fpf0_marks { pub u64: u64, }
// 	uint64_t u64;
// 	struct cvmx_fpa_fpf0_marks_s {
// #ifdef __BIG_ENDIAN_BITFIELD
// 		uint64_t reserved_24_63:40;
// 		uint64_t fpf_wr:12;
// 		uint64_t fpf_rd:12;
// #else
// 		uint64_t fpf_rd:12;
// 		uint64_t fpf_wr:12;
// 		uint64_t reserved_24_63:40;
// #endif
// 	} s;
// };
// 
#[repr(C)] pub union cvmx_fpa_fpf0_size { pub u64: u64, }
// 	uint64_t u64;
// 	struct cvmx_fpa_fpf0_size_s {
// #ifdef __BIG_ENDIAN_BITFIELD
// 		uint64_t reserved_12_63:52;
// 		uint64_t fpf_siz:12;
// #else
// 		uint64_t fpf_siz:12;
// 		uint64_t reserved_12_63:52;
// #endif
// 	} s;
// };
// 
#[repr(C)] pub union cvmx_fpa_fpf8_marks { pub u64: u64, }
// 	uint64_t u64;
// 	struct cvmx_fpa_fpf8_marks_s {
// #ifdef __BIG_ENDIAN_BITFIELD
// 		uint64_t reserved_22_63:42;
// 		uint64_t fpf_wr:11;
// 		uint64_t fpf_rd:11;
// #else
// 		uint64_t fpf_rd:11;
// 		uint64_t fpf_wr:11;
// 		uint64_t reserved_22_63:42;
// #endif
// 	} s;
// };
// 
#[repr(C)] pub union cvmx_fpa_fpf8_size { pub u64: u64, }
// 	uint64_t u64;
// 	struct cvmx_fpa_fpf8_size_s {
// #ifdef __BIG_ENDIAN_BITFIELD
// 		uint64_t reserved_12_63:52;
// 		uint64_t fpf_siz:12;
// #else
// 		uint64_t fpf_siz:12;
// 		uint64_t reserved_12_63:52;
// #endif
// 	} s;
// };
// 
#[repr(C)] pub union cvmx_fpa_int_enb { pub u64: u64, }
// 	uint64_t u64;
// 	struct cvmx_fpa_int_enb_s {
// #ifdef __BIG_ENDIAN_BITFIELD
// 		uint64_t reserved_50_63:14;
// 		uint64_t paddr_e:1;
// 		uint64_t reserved_44_48:5;
// 		uint64_t free7:1;
// 		uint64_t free6:1;
// 		uint64_t free5:1;
// 		uint64_t free4:1;
// 		uint64_t free3:1;
// 		uint64_t free2:1;
// 		uint64_t free1:1;
// 		uint64_t free0:1;
// 		uint64_t pool7th:1;
// 		uint64_t pool6th:1;
// 		uint64_t pool5th:1;
// 		uint64_t pool4th:1;
// 		uint64_t pool3th:1;
// 		uint64_t pool2th:1;
// 		uint64_t pool1th:1;
// 		uint64_t pool0th:1;
// 		uint64_t q7_perr:1;
// 		uint64_t q7_coff:1;
// 		uint64_t q7_und:1;
// 		uint64_t q6_perr:1;
// 		uint64_t q6_coff:1;
// 		uint64_t q6_und:1;
// 		uint64_t q5_perr:1;
// 		uint64_t q5_coff:1;
// 		uint64_t q5_und:1;
// 		uint64_t q4_perr:1;
// 		uint64_t q4_coff:1;
// 		uint64_t q4_und:1;
// 		uint64_t q3_perr:1;
// 		uint64_t q3_coff:1;
// 		uint64_t q3_und:1;
// 		uint64_t q2_perr:1;
// 		uint64_t q2_coff:1;
// 		uint64_t q2_und:1;
// 		uint64_t q1_perr:1;
// 		uint64_t q1_coff:1;
// 		uint64_t q1_und:1;
// 		uint64_t q0_perr:1;
// 		uint64_t q0_coff:1;
// 		uint64_t q0_und:1;
// 		uint64_t fed1_dbe:1;
// 		uint64_t fed1_sbe:1;
// 		uint64_t fed0_dbe:1;
// 		uint64_t fed0_sbe:1;
// #else
// 		uint64_t fed0_sbe:1;
// 		uint64_t fed0_dbe:1;
// 		uint64_t fed1_sbe:1;
// 		uint64_t fed1_dbe:1;
// 		uint64_t q0_und:1;
// 		uint64_t q0_coff:1;
// 		uint64_t q0_perr:1;
// 		uint64_t q1_und:1;
// 		uint64_t q1_coff:1;
// 		uint64_t q1_perr:1;
// 		uint64_t q2_und:1;
// 		uint64_t q2_coff:1;
// 		uint64_t q2_perr:1;
// 		uint64_t q3_und:1;
// 		uint64_t q3_coff:1;
// 		uint64_t q3_perr:1;
// 		uint64_t q4_und:1;
// 		uint64_t q4_coff:1;
// 		uint64_t q4_perr:1;
// 		uint64_t q5_und:1;
// 		uint64_t q5_coff:1;
// 		uint64_t q5_perr:1;
// 		uint64_t q6_und:1;
// 		uint64_t q6_coff:1;
// 		uint64_t q6_perr:1;
// 		uint64_t q7_und:1;
// 		uint64_t q7_coff:1;
// 		uint64_t q7_perr:1;
// 		uint64_t pool0th:1;
// 		uint64_t pool1th:1;
// 		uint64_t pool2th:1;
// 		uint64_t pool3th:1;
// 		uint64_t pool4th:1;
// 		uint64_t pool5th:1;
// 		uint64_t pool6th:1;
// 		uint64_t pool7th:1;
// 		uint64_t free0:1;
// 		uint64_t free1:1;
// 		uint64_t free2:1;
// 		uint64_t free3:1;
// 		uint64_t free4:1;
// 		uint64_t free5:1;
// 		uint64_t free6:1;
// 		uint64_t free7:1;
// 		uint64_t reserved_44_48:5;
// 		uint64_t paddr_e:1;
// 		uint64_t reserved_50_63:14;
// #endif
// 	} s;
// 	struct cvmx_fpa_int_enb_cn30xx {
// #ifdef __BIG_ENDIAN_BITFIELD
// 		uint64_t reserved_28_63:36;
// 		uint64_t q7_perr:1;
// 		uint64_t q7_coff:1;
// 		uint64_t q7_und:1;
// 		uint64_t q6_perr:1;
// 		uint64_t q6_coff:1;
// 		uint64_t q6_und:1;
// 		uint64_t q5_perr:1;
// 		uint64_t q5_coff:1;
// 		uint64_t q5_und:1;
// 		uint64_t q4_perr:1;
// 		uint64_t q4_coff:1;
// 		uint64_t q4_und:1;
// 		uint64_t q3_perr:1;
// 		uint64_t q3_coff:1;
// 		uint64_t q3_und:1;
// 		uint64_t q2_perr:1;
// 		uint64_t q2_coff:1;
// 		uint64_t q2_und:1;
// 		uint64_t q1_perr:1;
// 		uint64_t q1_coff:1;
// 		uint64_t q1_und:1;
// 		uint64_t q0_perr:1;
// 		uint64_t q0_coff:1;
// 		uint64_t q0_und:1;
// 		uint64_t fed1_dbe:1;
// 		uint64_t fed1_sbe:1;
// 		uint64_t fed0_dbe:1;
// 		uint64_t fed0_sbe:1;
// #else
// 		uint64_t fed0_sbe:1;
// 		uint64_t fed0_dbe:1;
// 		uint64_t fed1_sbe:1;
// 		uint64_t fed1_dbe:1;
// 		uint64_t q0_und:1;
// 		uint64_t q0_coff:1;
// 		uint64_t q0_perr:1;
// 		uint64_t q1_und:1;
// 		uint64_t q1_coff:1;
// 		uint64_t q1_perr:1;
// 		uint64_t q2_und:1;
// 		uint64_t q2_coff:1;
// 		uint64_t q2_perr:1;
// 		uint64_t q3_und:1;
// 		uint64_t q3_coff:1;
// 		uint64_t q3_perr:1;
// 		uint64_t q4_und:1;
// 		uint64_t q4_coff:1;
// 		uint64_t q4_perr:1;
// 		uint64_t q5_und:1;
// 		uint64_t q5_coff:1;
// 		uint64_t q5_perr:1;
// 		uint64_t q6_und:1;
// 		uint64_t q6_coff:1;
// 		uint64_t q6_perr:1;
// 		uint64_t q7_und:1;
// 		uint64_t q7_coff:1;
// 		uint64_t q7_perr:1;
// 		uint64_t reserved_28_63:36;
// #endif
// 	} cn30xx;
// 	struct cvmx_fpa_int_enb_cn61xx {
// #ifdef __BIG_ENDIAN_BITFIELD
// 		uint64_t reserved_50_63:14;
// 		uint64_t paddr_e:1;
// 		uint64_t res_44:5;
// 		uint64_t free7:1;
// 		uint64_t free6:1;
// 		uint64_t free5:1;
// 		uint64_t free4:1;
// 		uint64_t free3:1;
// 		uint64_t free2:1;
// 		uint64_t free1:1;
// 		uint64_t free0:1;
// 		uint64_t pool7th:1;
// 		uint64_t pool6th:1;
// 		uint64_t pool5th:1;
// 		uint64_t pool4th:1;
// 		uint64_t pool3th:1;
// 		uint64_t pool2th:1;
// 		uint64_t pool1th:1;
// 		uint64_t pool0th:1;
// 		uint64_t q7_perr:1;
// 		uint64_t q7_coff:1;
// 		uint64_t q7_und:1;
// 		uint64_t q6_perr:1;
// 		uint64_t q6_coff:1;
// 		uint64_t q6_und:1;
// 		uint64_t q5_perr:1;
// 		uint64_t q5_coff:1;
// 		uint64_t q5_und:1;
// 		uint64_t q4_perr:1;
// 		uint64_t q4_coff:1;
// 		uint64_t q4_und:1;
// 		uint64_t q3_perr:1;
// 		uint64_t q3_coff:1;
// 		uint64_t q3_und:1;
// 		uint64_t q2_perr:1;
// 		uint64_t q2_coff:1;
// 		uint64_t q2_und:1;
// 		uint64_t q1_perr:1;
// 		uint64_t q1_coff:1;
// 		uint64_t q1_und:1;
// 		uint64_t q0_perr:1;
// 		uint64_t q0_coff:1;
// 		uint64_t q0_und:1;
// 		uint64_t fed1_dbe:1;
// 		uint64_t fed1_sbe:1;
// 		uint64_t fed0_dbe:1;
// 		uint64_t fed0_sbe:1;
// #else
// 		uint64_t fed0_sbe:1;
// 		uint64_t fed0_dbe:1;
// 		uint64_t fed1_sbe:1;
// 		uint64_t fed1_dbe:1;
// 		uint64_t q0_und:1;
// 		uint64_t q0_coff:1;
// 		uint64_t q0_perr:1;
// 		uint64_t q1_und:1;
// 		uint64_t q1_coff:1;
// 		uint64_t q1_perr:1;
// 		uint64_t q2_und:1;
// 		uint64_t q2_coff:1;
// 		uint64_t q2_perr:1;
// 		uint64_t q3_und:1;
// 		uint64_t q3_coff:1;
// 		uint64_t q3_perr:1;
// 		uint64_t q4_und:1;
// 		uint64_t q4_coff:1;
// 		uint64_t q4_perr:1;
// 		uint64_t q5_und:1;
// 		uint64_t q5_coff:1;
// 		uint64_t q5_perr:1;
// 		uint64_t q6_und:1;
// 		uint64_t q6_coff:1;
// 		uint64_t q6_perr:1;
// 		uint64_t q7_und:1;
// 		uint64_t q7_coff:1;
// 		uint64_t q7_perr:1;
// 		uint64_t pool0th:1;
// 		uint64_t pool1th:1;
// 		uint64_t pool2th:1;
// 		uint64_t pool3th:1;
// 		uint64_t pool4th:1;
// 		uint64_t pool5th:1;
// 		uint64_t pool6th:1;
// 		uint64_t pool7th:1;
// 		uint64_t free0:1;
// 		uint64_t free1:1;
// 		uint64_t free2:1;
// 		uint64_t free3:1;
// 		uint64_t free4:1;
// 		uint64_t free5:1;
// 		uint64_t free6:1;
// 		uint64_t free7:1;
// 		uint64_t res_44:5;
// 		uint64_t paddr_e:1;
// 		uint64_t reserved_50_63:14;
// #endif
// 	} cn61xx;
// 	struct cvmx_fpa_int_enb_cn63xx {
// #ifdef __BIG_ENDIAN_BITFIELD
// 		uint64_t reserved_44_63:20;
// 		uint64_t free7:1;
// 		uint64_t free6:1;
// 		uint64_t free5:1;
// 		uint64_t free4:1;
// 		uint64_t free3:1;
// 		uint64_t free2:1;
// 		uint64_t free1:1;
// 		uint64_t free0:1;
// 		uint64_t pool7th:1;
// 		uint64_t pool6th:1;
// 		uint64_t pool5th:1;
// 		uint64_t pool4th:1;
// 		uint64_t pool3th:1;
// 		uint64_t pool2th:1;
// 		uint64_t pool1th:1;
// 		uint64_t pool0th:1;
// 		uint64_t q7_perr:1;
// 		uint64_t q7_coff:1;
// 		uint64_t q7_und:1;
// 		uint64_t q6_perr:1;
// 		uint64_t q6_coff:1;
// 		uint64_t q6_und:1;
// 		uint64_t q5_perr:1;
// 		uint64_t q5_coff:1;
// 		uint64_t q5_und:1;
// 		uint64_t q4_perr:1;
// 		uint64_t q4_coff:1;
// 		uint64_t q4_und:1;
// 		uint64_t q3_perr:1;
// 		uint64_t q3_coff:1;
// 		uint64_t q3_und:1;
// 		uint64_t q2_perr:1;
// 		uint64_t q2_coff:1;
// 		uint64_t q2_und:1;
// 		uint64_t q1_perr:1;
// 		uint64_t q1_coff:1;
// 		uint64_t q1_und:1;
// 		uint64_t q0_perr:1;
// 		uint64_t q0_coff:1;
// 		uint64_t q0_und:1;
// 		uint64_t fed1_dbe:1;
// 		uint64_t fed1_sbe:1;
// 		uint64_t fed0_dbe:1;
// 		uint64_t fed0_sbe:1;
// #else
// 		uint64_t fed0_sbe:1;
// 		uint64_t fed0_dbe:1;
// 		uint64_t fed1_sbe:1;
// 		uint64_t fed1_dbe:1;
// 		uint64_t q0_und:1;
// 		uint64_t q0_coff:1;
// 		uint64_t q0_perr:1;
// 		uint64_t q1_und:1;
// 		uint64_t q1_coff:1;
// 		uint64_t q1_perr:1;
// 		uint64_t q2_und:1;
// 		uint64_t q2_coff:1;
// 		uint64_t q2_perr:1;
// 		uint64_t q3_und:1;
// 		uint64_t q3_coff:1;
// 		uint64_t q3_perr:1;
// 		uint64_t q4_und:1;
// 		uint64_t q4_coff:1;
// 		uint64_t q4_perr:1;
// 		uint64_t q5_und:1;
// 		uint64_t q5_coff:1;
// 		uint64_t q5_perr:1;
// 		uint64_t q6_und:1;
// 		uint64_t q6_coff:1;
// 		uint64_t q6_perr:1;
// 		uint64_t q7_und:1;
// 		uint64_t q7_coff:1;
// 		uint64_t q7_perr:1;
// 		uint64_t pool0th:1;
// 		uint64_t pool1th:1;
// 		uint64_t pool2th:1;
// 		uint64_t pool3th:1;
// 		uint64_t pool4th:1;
// 		uint64_t pool5th:1;
// 		uint64_t pool6th:1;
// 		uint64_t pool7th:1;
// 		uint64_t free0:1;
// 		uint64_t free1:1;
// 		uint64_t free2:1;
// 		uint64_t free3:1;
// 		uint64_t free4:1;
// 		uint64_t free5:1;
// 		uint64_t free6:1;
// 		uint64_t free7:1;
// 		uint64_t reserved_44_63:20;
// #endif
// 	} cn63xx;
// 	struct cvmx_fpa_int_enb_cn68xx {
// #ifdef __BIG_ENDIAN_BITFIELD
// 		uint64_t reserved_50_63:14;
// 		uint64_t paddr_e:1;
// 		uint64_t pool8th:1;
// 		uint64_t q8_perr:1;
// 		uint64_t q8_coff:1;
// 		uint64_t q8_und:1;
// 		uint64_t free8:1;
// 		uint64_t free7:1;
// 		uint64_t free6:1;
// 		uint64_t free5:1;
// 		uint64_t free4:1;
// 		uint64_t free3:1;
// 		uint64_t free2:1;
// 		uint64_t free1:1;
// 		uint64_t free0:1;
// 		uint64_t pool7th:1;
// 		uint64_t pool6th:1;
// 		uint64_t pool5th:1;
// 		uint64_t pool4th:1;
// 		uint64_t pool3th:1;
// 		uint64_t pool2th:1;
// 		uint64_t pool1th:1;
// 		uint64_t pool0th:1;
// 		uint64_t q7_perr:1;
// 		uint64_t q7_coff:1;
// 		uint64_t q7_und:1;
// 		uint64_t q6_perr:1;
// 		uint64_t q6_coff:1;
// 		uint64_t q6_und:1;
// 		uint64_t q5_perr:1;
// 		uint64_t q5_coff:1;
// 		uint64_t q5_und:1;
// 		uint64_t q4_perr:1;
// 		uint64_t q4_coff:1;
// 		uint64_t q4_und:1;
// 		uint64_t q3_perr:1;
// 		uint64_t q3_coff:1;
// 		uint64_t q3_und:1;
// 		uint64_t q2_perr:1;
// 		uint64_t q2_coff:1;
// 		uint64_t q2_und:1;
// 		uint64_t q1_perr:1;
// 		uint64_t q1_coff:1;
// 		uint64_t q1_und:1;
// 		uint64_t q0_perr:1;
// 		uint64_t q0_coff:1;
// 		uint64_t q0_und:1;
// 		uint64_t fed1_dbe:1;
// 		uint64_t fed1_sbe:1;
// 		uint64_t fed0_dbe:1;
// 		uint64_t fed0_sbe:1;
// #else
// 		uint64_t fed0_sbe:1;
// 		uint64_t fed0_dbe:1;
// 		uint64_t fed1_sbe:1;
// 		uint64_t fed1_dbe:1;
// 		uint64_t q0_und:1;
// 		uint64_t q0_coff:1;
// 		uint64_t q0_perr:1;
// 		uint64_t q1_und:1;
// 		uint64_t q1_coff:1;
// 		uint64_t q1_perr:1;
// 		uint64_t q2_und:1;
// 		uint64_t q2_coff:1;
// 		uint64_t q2_perr:1;
// 		uint64_t q3_und:1;
// 		uint64_t q3_coff:1;
// 		uint64_t q3_perr:1;
// 		uint64_t q4_und:1;
// 		uint64_t q4_coff:1;
// 		uint64_t q4_perr:1;
// 		uint64_t q5_und:1;
// 		uint64_t q5_coff:1;
// 		uint64_t q5_perr:1;
// 		uint64_t q6_und:1;
// 		uint64_t q6_coff:1;
// 		uint64_t q6_perr:1;
// 		uint64_t q7_und:1;
// 		uint64_t q7_coff:1;
// 		uint64_t q7_perr:1;
// 		uint64_t pool0th:1;
// 		uint64_t pool1th:1;
// 		uint64_t pool2th:1;
// 		uint64_t pool3th:1;
// 		uint64_t pool4th:1;
// 		uint64_t pool5th:1;
// 		uint64_t pool6th:1;
// 		uint64_t pool7th:1;
// 		uint64_t free0:1;
// 		uint64_t free1:1;
// 		uint64_t free2:1;
// 		uint64_t free3:1;
// 		uint64_t free4:1;
// 		uint64_t free5:1;
// 		uint64_t free6:1;
// 		uint64_t free7:1;
// 		uint64_t free8:1;
// 		uint64_t q8_und:1;
// 		uint64_t q8_coff:1;
// 		uint64_t q8_perr:1;
// 		uint64_t pool8th:1;
// 		uint64_t paddr_e:1;
// 		uint64_t reserved_50_63:14;
// #endif
// 	} cn68xx;
// };
// 
#[repr(C)] pub union cvmx_fpa_int_sum { pub u64: u64, }
// 	uint64_t u64;
// 	struct cvmx_fpa_int_sum_s {
// #ifdef __BIG_ENDIAN_BITFIELD
// 		uint64_t reserved_50_63:14;
// 		uint64_t paddr_e:1;
// 		uint64_t pool8th:1;
// 		uint64_t q8_perr:1;
// 		uint64_t q8_coff:1;
// 		uint64_t q8_und:1;
// 		uint64_t free8:1;
// 		uint64_t free7:1;
// 		uint64_t free6:1;
// 		uint64_t free5:1;
// 		uint64_t free4:1;
// 		uint64_t free3:1;
// 		uint64_t free2:1;
// 		uint64_t free1:1;
// 		uint64_t free0:1;
// 		uint64_t pool7th:1;
// 		uint64_t pool6th:1;
// 		uint64_t pool5th:1;
// 		uint64_t pool4th:1;
// 		uint64_t pool3th:1;
// 		uint64_t pool2th:1;
// 		uint64_t pool1th:1;
// 		uint64_t pool0th:1;
// 		uint64_t q7_perr:1;
// 		uint64_t q7_coff:1;
// 		uint64_t q7_und:1;
// 		uint64_t q6_perr:1;
// 		uint64_t q6_coff:1;
// 		uint64_t q6_und:1;
// 		uint64_t q5_perr:1;
// 		uint64_t q5_coff:1;
// 		uint64_t q5_und:1;
// 		uint64_t q4_perr:1;
// 		uint64_t q4_coff:1;
// 		uint64_t q4_und:1;
// 		uint64_t q3_perr:1;
// 		uint64_t q3_coff:1;
// 		uint64_t q3_und:1;
// 		uint64_t q2_perr:1;
// 		uint64_t q2_coff:1;
// 		uint64_t q2_und:1;
// 		uint64_t q1_perr:1;
// 		uint64_t q1_coff:1;
// 		uint64_t q1_und:1;
// 		uint64_t q0_perr:1;
// 		uint64_t q0_coff:1;
// 		uint64_t q0_und:1;
// 		uint64_t fed1_dbe:1;
// 		uint64_t fed1_sbe:1;
// 		uint64_t fed0_dbe:1;
// 		uint64_t fed0_sbe:1;
// #else
// 		uint64_t fed0_sbe:1;
// 		uint64_t fed0_dbe:1;
// 		uint64_t fed1_sbe:1;
// 		uint64_t fed1_dbe:1;
// 		uint64_t q0_und:1;
// 		uint64_t q0_coff:1;
// 		uint64_t q0_perr:1;
// 		uint64_t q1_und:1;
// 		uint64_t q1_coff:1;
// 		uint64_t q1_perr:1;
// 		uint64_t q2_und:1;
// 		uint64_t q2_coff:1;
// 		uint64_t q2_perr:1;
// 		uint64_t q3_und:1;
// 		uint64_t q3_coff:1;
// 		uint64_t q3_perr:1;
// 		uint64_t q4_und:1;
// 		uint64_t q4_coff:1;
// 		uint64_t q4_perr:1;
// 		uint64_t q5_und:1;
// 		uint64_t q5_coff:1;
// 		uint64_t q5_perr:1;
// 		uint64_t q6_und:1;
// 		uint64_t q6_coff:1;
// 		uint64_t q6_perr:1;
// 		uint64_t q7_und:1;
// 		uint64_t q7_coff:1;
// 		uint64_t q7_perr:1;
// 		uint64_t pool0th:1;
// 		uint64_t pool1th:1;
// 		uint64_t pool2th:1;
// 		uint64_t pool3th:1;
// 		uint64_t pool4th:1;
// 		uint64_t pool5th:1;
// 		uint64_t pool6th:1;
// 		uint64_t pool7th:1;
// 		uint64_t free0:1;
// 		uint64_t free1:1;
// 		uint64_t free2:1;
// 		uint64_t free3:1;
// 		uint64_t free4:1;
// 		uint64_t free5:1;
// 		uint64_t free6:1;
// 		uint64_t free7:1;
// 		uint64_t free8:1;
// 		uint64_t q8_und:1;
// 		uint64_t q8_coff:1;
// 		uint64_t q8_perr:1;
// 		uint64_t pool8th:1;
// 		uint64_t paddr_e:1;
// 		uint64_t reserved_50_63:14;
// #endif
// 	} s;
// 	struct cvmx_fpa_int_sum_cn30xx {
// #ifdef __BIG_ENDIAN_BITFIELD
// 		uint64_t reserved_28_63:36;
// 		uint64_t q7_perr:1;
// 		uint64_t q7_coff:1;
// 		uint64_t q7_und:1;
// 		uint64_t q6_perr:1;
// 		uint64_t q6_coff:1;
// 		uint64_t q6_und:1;
// 		uint64_t q5_perr:1;
// 		uint64_t q5_coff:1;
// 		uint64_t q5_und:1;
// 		uint64_t q4_perr:1;
// 		uint64_t q4_coff:1;
// 		uint64_t q4_und:1;
// 		uint64_t q3_perr:1;
// 		uint64_t q3_coff:1;
// 		uint64_t q3_und:1;
// 		uint64_t q2_perr:1;
// 		uint64_t q2_coff:1;
// 		uint64_t q2_und:1;
// 		uint64_t q1_perr:1;
// 		uint64_t q1_coff:1;
// 		uint64_t q1_und:1;
// 		uint64_t q0_perr:1;
// 		uint64_t q0_coff:1;
// 		uint64_t q0_und:1;
// 		uint64_t fed1_dbe:1;
// 		uint64_t fed1_sbe:1;
// 		uint64_t fed0_dbe:1;
// 		uint64_t fed0_sbe:1;
// #else
// 		uint64_t fed0_sbe:1;
// 		uint64_t fed0_dbe:1;
// 		uint64_t fed1_sbe:1;
// 		uint64_t fed1_dbe:1;
// 		uint64_t q0_und:1;
// 		uint64_t q0_coff:1;
// 		uint64_t q0_perr:1;
// 		uint64_t q1_und:1;
// 		uint64_t q1_coff:1;
// 		uint64_t q1_perr:1;
// 		uint64_t q2_und:1;
// 		uint64_t q2_coff:1;
// 		uint64_t q2_perr:1;
// 		uint64_t q3_und:1;
// 		uint64_t q3_coff:1;
// 		uint64_t q3_perr:1;
// 		uint64_t q4_und:1;
// 		uint64_t q4_coff:1;
// 		uint64_t q4_perr:1;
// 		uint64_t q5_und:1;
// 		uint64_t q5_coff:1;
// 		uint64_t q5_perr:1;
// 		uint64_t q6_und:1;
// 		uint64_t q6_coff:1;
// 		uint64_t q6_perr:1;
// 		uint64_t q7_und:1;
// 		uint64_t q7_coff:1;
// 		uint64_t q7_perr:1;
// 		uint64_t reserved_28_63:36;
// #endif
// 	} cn30xx;
// 	struct cvmx_fpa_int_sum_cn61xx {
// #ifdef __BIG_ENDIAN_BITFIELD
// 		uint64_t reserved_50_63:14;
// 		uint64_t paddr_e:1;
// 		uint64_t reserved_44_48:5;
// 		uint64_t free7:1;
// 		uint64_t free6:1;
// 		uint64_t free5:1;
// 		uint64_t free4:1;
// 		uint64_t free3:1;
// 		uint64_t free2:1;
// 		uint64_t free1:1;
// 		uint64_t free0:1;
// 		uint64_t pool7th:1;
// 		uint64_t pool6th:1;
// 		uint64_t pool5th:1;
// 		uint64_t pool4th:1;
// 		uint64_t pool3th:1;
// 		uint64_t pool2th:1;
// 		uint64_t pool1th:1;
// 		uint64_t pool0th:1;
// 		uint64_t q7_perr:1;
// 		uint64_t q7_coff:1;
// 		uint64_t q7_und:1;
// 		uint64_t q6_perr:1;
// 		uint64_t q6_coff:1;
// 		uint64_t q6_und:1;
// 		uint64_t q5_perr:1;
// 		uint64_t q5_coff:1;
// 		uint64_t q5_und:1;
// 		uint64_t q4_perr:1;
// 		uint64_t q4_coff:1;
// 		uint64_t q4_und:1;
// 		uint64_t q3_perr:1;
// 		uint64_t q3_coff:1;
// 		uint64_t q3_und:1;
// 		uint64_t q2_perr:1;
// 		uint64_t q2_coff:1;
// 		uint64_t q2_und:1;
// 		uint64_t q1_perr:1;
// 		uint64_t q1_coff:1;
// 		uint64_t q1_und:1;
// 		uint64_t q0_perr:1;
// 		uint64_t q0_coff:1;
// 		uint64_t q0_und:1;
// 		uint64_t fed1_dbe:1;
// 		uint64_t fed1_sbe:1;
// 		uint64_t fed0_dbe:1;
// 		uint64_t fed0_sbe:1;
// #else
// 		uint64_t fed0_sbe:1;
// 		uint64_t fed0_dbe:1;
// 		uint64_t fed1_sbe:1;
// 		uint64_t fed1_dbe:1;
// 		uint64_t q0_und:1;
// 		uint64_t q0_coff:1;
// 		uint64_t q0_perr:1;
// 		uint64_t q1_und:1;
// 		uint64_t q1_coff:1;
// 		uint64_t q1_perr:1;
// 		uint64_t q2_und:1;
// 		uint64_t q2_coff:1;
// 		uint64_t q2_perr:1;
// 		uint64_t q3_und:1;
// 		uint64_t q3_coff:1;
// 		uint64_t q3_perr:1;
// 		uint64_t q4_und:1;
// 		uint64_t q4_coff:1;
// 		uint64_t q4_perr:1;
// 		uint64_t q5_und:1;
// 		uint64_t q5_coff:1;
// 		uint64_t q5_perr:1;
// 		uint64_t q6_und:1;
// 		uint64_t q6_coff:1;
// 		uint64_t q6_perr:1;
// 		uint64_t q7_und:1;
// 		uint64_t q7_coff:1;
// 		uint64_t q7_perr:1;
// 		uint64_t pool0th:1;
// 		uint64_t pool1th:1;
// 		uint64_t pool2th:1;
// 		uint64_t pool3th:1;
// 		uint64_t pool4th:1;
// 		uint64_t pool5th:1;
// 		uint64_t pool6th:1;
// 		uint64_t pool7th:1;
// 		uint64_t free0:1;
// 		uint64_t free1:1;
// 		uint64_t free2:1;
// 		uint64_t free3:1;
// 		uint64_t free4:1;
// 		uint64_t free5:1;
// 		uint64_t free6:1;
// 		uint64_t free7:1;
// 		uint64_t reserved_44_48:5;
// 		uint64_t paddr_e:1;
// 		uint64_t reserved_50_63:14;
// #endif
// 	} cn61xx;
// 	struct cvmx_fpa_int_sum_cn63xx {
// #ifdef __BIG_ENDIAN_BITFIELD
// 		uint64_t reserved_44_63:20;
// 		uint64_t free7:1;
// 		uint64_t free6:1;
// 		uint64_t free5:1;
// 		uint64_t free4:1;
// 		uint64_t free3:1;
// 		uint64_t free2:1;
// 		uint64_t free1:1;
// 		uint64_t free0:1;
// 		uint64_t pool7th:1;
// 		uint64_t pool6th:1;
// 		uint64_t pool5th:1;
// 		uint64_t pool4th:1;
// 		uint64_t pool3th:1;
// 		uint64_t pool2th:1;
// 		uint64_t pool1th:1;
// 		uint64_t pool0th:1;
// 		uint64_t q7_perr:1;
// 		uint64_t q7_coff:1;
// 		uint64_t q7_und:1;
// 		uint64_t q6_perr:1;
// 		uint64_t q6_coff:1;
// 		uint64_t q6_und:1;
// 		uint64_t q5_perr:1;
// 		uint64_t q5_coff:1;
// 		uint64_t q5_und:1;
// 		uint64_t q4_perr:1;
// 		uint64_t q4_coff:1;
// 		uint64_t q4_und:1;
// 		uint64_t q3_perr:1;
// 		uint64_t q3_coff:1;
// 		uint64_t q3_und:1;
// 		uint64_t q2_perr:1;
// 		uint64_t q2_coff:1;
// 		uint64_t q2_und:1;
// 		uint64_t q1_perr:1;
// 		uint64_t q1_coff:1;
// 		uint64_t q1_und:1;
// 		uint64_t q0_perr:1;
// 		uint64_t q0_coff:1;
// 		uint64_t q0_und:1;
// 		uint64_t fed1_dbe:1;
// 		uint64_t fed1_sbe:1;
// 		uint64_t fed0_dbe:1;
// 		uint64_t fed0_sbe:1;
// #else
// 		uint64_t fed0_sbe:1;
// 		uint64_t fed0_dbe:1;
// 		uint64_t fed1_sbe:1;
// 		uint64_t fed1_dbe:1;
// 		uint64_t q0_und:1;
// 		uint64_t q0_coff:1;
// 		uint64_t q0_perr:1;
// 		uint64_t q1_und:1;
// 		uint64_t q1_coff:1;
// 		uint64_t q1_perr:1;
// 		uint64_t q2_und:1;
// 		uint64_t q2_coff:1;
// 		uint64_t q2_perr:1;
// 		uint64_t q3_und:1;
// 		uint64_t q3_coff:1;
// 		uint64_t q3_perr:1;
// 		uint64_t q4_und:1;
// 		uint64_t q4_coff:1;
// 		uint64_t q4_perr:1;
// 		uint64_t q5_und:1;
// 		uint64_t q5_coff:1;
// 		uint64_t q5_perr:1;
// 		uint64_t q6_und:1;
// 		uint64_t q6_coff:1;
// 		uint64_t q6_perr:1;
// 		uint64_t q7_und:1;
// 		uint64_t q7_coff:1;
// 		uint64_t q7_perr:1;
// 		uint64_t pool0th:1;
// 		uint64_t pool1th:1;
// 		uint64_t pool2th:1;
// 		uint64_t pool3th:1;
// 		uint64_t pool4th:1;
// 		uint64_t pool5th:1;
// 		uint64_t pool6th:1;
// 		uint64_t pool7th:1;
// 		uint64_t free0:1;
// 		uint64_t free1:1;
// 		uint64_t free2:1;
// 		uint64_t free3:1;
// 		uint64_t free4:1;
// 		uint64_t free5:1;
// 		uint64_t free6:1;
// 		uint64_t free7:1;
// 		uint64_t reserved_44_63:20;
// #endif
// 	} cn63xx;
// };
// 
#[repr(C)] pub union cvmx_fpa_packet_threshold { pub u64: u64, }
// 	uint64_t u64;
// 	struct cvmx_fpa_packet_threshold_s {
// #ifdef __BIG_ENDIAN_BITFIELD
// 		uint64_t reserved_32_63:32;
// 		uint64_t thresh:32;
// #else
// 		uint64_t thresh:32;
// 		uint64_t reserved_32_63:32;
// #endif
// 	} s;
// };
// 
#[repr(C)] pub union cvmx_fpa_poolx_end_addr { pub u64: u64, }
// 	uint64_t u64;
// 	struct cvmx_fpa_poolx_end_addr_s {
// #ifdef __BIG_ENDIAN_BITFIELD
// 		uint64_t reserved_33_63:31;
// 		uint64_t addr:33;
// #else
// 		uint64_t addr:33;
// 		uint64_t reserved_33_63:31;
// #endif
// 	} s;
// };
// 
#[repr(C)] pub union cvmx_fpa_poolx_start_addr { pub u64: u64, }
// 	uint64_t u64;
// 	struct cvmx_fpa_poolx_start_addr_s {
// #ifdef __BIG_ENDIAN_BITFIELD
// 		uint64_t reserved_33_63:31;
// 		uint64_t addr:33;
// #else
// 		uint64_t addr:33;
// 		uint64_t reserved_33_63:31;
// #endif
// 	} s;
// };
// 
#[repr(C)] pub union cvmx_fpa_poolx_threshold { pub u64: u64, }
// 	uint64_t u64;
// 	struct cvmx_fpa_poolx_threshold_s {
// #ifdef __BIG_ENDIAN_BITFIELD
// 		uint64_t reserved_32_63:32;
// 		uint64_t thresh:32;
// #else
// 		uint64_t thresh:32;
// 		uint64_t reserved_32_63:32;
// #endif
// 	} s;
// 	struct cvmx_fpa_poolx_threshold_cn61xx {
// #ifdef __BIG_ENDIAN_BITFIELD
// 		uint64_t reserved_29_63:35;
// 		uint64_t thresh:29;
// #else
// 		uint64_t thresh:29;
// 		uint64_t reserved_29_63:35;
// #endif
// 	} cn61xx;
// };
// 
#[repr(C)] pub union cvmx_fpa_quex_available { pub u64: u64, }
// 	uint64_t u64;
// 	struct cvmx_fpa_quex_available_s {
// #ifdef __BIG_ENDIAN_BITFIELD
// 		uint64_t reserved_32_63:32;
// 		uint64_t que_siz:32;
// #else
// 		uint64_t que_siz:32;
// 		uint64_t reserved_32_63:32;
// #endif
// 	} s;
// 	struct cvmx_fpa_quex_available_cn30xx {
// #ifdef __BIG_ENDIAN_BITFIELD
// 		uint64_t reserved_29_63:35;
// 		uint64_t que_siz:29;
// #else
// 		uint64_t que_siz:29;
// 		uint64_t reserved_29_63:35;
// #endif
// 	} cn30xx;
// };
// 
#[repr(C)] pub union cvmx_fpa_quex_page_index { pub u64: u64, }
// 	uint64_t u64;
// 	struct cvmx_fpa_quex_page_index_s {
// #ifdef __BIG_ENDIAN_BITFIELD
// 		uint64_t reserved_25_63:39;
// 		uint64_t pg_num:25;
// #else
// 		uint64_t pg_num:25;
// 		uint64_t reserved_25_63:39;
// #endif
// 	} s;
// };
// 
#[repr(C)] pub union cvmx_fpa_que8_page_index { pub u64: u64, }
// 	uint64_t u64;
// 	struct cvmx_fpa_que8_page_index_s {
// #ifdef __BIG_ENDIAN_BITFIELD
// 		uint64_t reserved_25_63:39;
// 		uint64_t pg_num:25;
// #else
// 		uint64_t pg_num:25;
// 		uint64_t reserved_25_63:39;
// #endif
// 	} s;
// };
// 
#[repr(C)] pub union cvmx_fpa_que_act { pub u64: u64, }
// 	uint64_t u64;
// 	struct cvmx_fpa_que_act_s {
// #ifdef __BIG_ENDIAN_BITFIELD
// 		uint64_t reserved_29_63:35;
// 		uint64_t act_que:3;
// 		uint64_t act_indx:26;
// #else
// 		uint64_t act_indx:26;
// 		uint64_t act_que:3;
// 		uint64_t reserved_29_63:35;
// #endif
// 	} s;
// };
// 
#[repr(C)] pub union cvmx_fpa_que_exp { pub u64: u64, }
// 	uint64_t u64;
// 	struct cvmx_fpa_que_exp_s {
// #ifdef __BIG_ENDIAN_BITFIELD
// 		uint64_t reserved_29_63:35;
// 		uint64_t exp_que:3;
// 		uint64_t exp_indx:26;
// #else
// 		uint64_t exp_indx:26;
// 		uint64_t exp_que:3;
// 		uint64_t reserved_29_63:35;
// #endif
// 	} s;
// };
// 
#[repr(C)] pub union cvmx_fpa_wart_ctl { pub u64: u64, }
// 	uint64_t u64;
// 	struct cvmx_fpa_wart_ctl_s {
// #ifdef __BIG_ENDIAN_BITFIELD
// 		uint64_t reserved_16_63:48;
// 		uint64_t ctl:16;
// #else
// 		uint64_t ctl:16;
// 		uint64_t reserved_16_63:48;
// #endif
// 	} s;
// };
// 
#[repr(C)] pub union cvmx_fpa_wart_status { pub u64: u64, }
// 	uint64_t u64;
// 	struct cvmx_fpa_wart_status_s {
// #ifdef __BIG_ENDIAN_BITFIELD
// 		uint64_t reserved_32_63:32;
// 		uint64_t status:32;
// #else
// 		uint64_t status:32;
// 		uint64_t reserved_32_63:32;
// #endif
// 	} s;
// };
// 
#[repr(C)] pub union cvmx_fpa_wqe_threshold { pub u64: u64, }
// 	uint64_t u64;
// 	struct cvmx_fpa_wqe_threshold_s {
// #ifdef __BIG_ENDIAN_BITFIELD
// 		uint64_t reserved_32_63:32;
// 		uint64_t thresh:32;
// #else
// 		uint64_t thresh:32;
// 		uint64_t reserved_32_63:32;
// #endif
// 	} s;
// };
// 
// #endif

// Original C source retained below to preserve all bitfield declarations, comments, and conditional layout intent:

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
