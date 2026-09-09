/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * Copyright (C) 2012 ARM Ltd.
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License version 2 as
 * published by the Free Software Foundation.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

/*
 * HWCAP flags - for AT_HWCAP
 *
 * Bits 62 and 63 are reserved for use by libc.
 * Bits 33-61 are unallocated for potential use by libc.
 */
pub const HWCAP_FP: u64 = 1u64 << 0;
pub const HWCAP_ASIMD: u64 = 1u64 << 1;
pub const HWCAP_EVTSTRM: u64 = 1u64 << 2;
pub const HWCAP_AES: u64 = 1u64 << 3;
pub const HWCAP_PMULL: u64 = 1u64 << 4;
pub const HWCAP_SHA1: u64 = 1u64 << 5;
pub const HWCAP_SHA2: u64 = 1u64 << 6;
pub const HWCAP_CRC32: u64 = 1u64 << 7;
pub const HWCAP_ATOMICS: u64 = 1u64 << 8;
pub const HWCAP_FPHP: u64 = 1u64 << 9;
pub const HWCAP_ASIMDHP: u64 = 1u64 << 10;
pub const HWCAP_CPUID: u64 = 1u64 << 11;
pub const HWCAP_ASIMDRDM: u64 = 1u64 << 12;
pub const HWCAP_JSCVT: u64 = 1u64 << 13;
pub const HWCAP_FCMA: u64 = 1u64 << 14;
pub const HWCAP_LRCPC: u64 = 1u64 << 15;
pub const HWCAP_DCPOP: u64 = 1u64 << 16;
pub const HWCAP_SHA3: u64 = 1u64 << 17;
pub const HWCAP_SM3: u64 = 1u64 << 18;
pub const HWCAP_SM4: u64 = 1u64 << 19;
pub const HWCAP_ASIMDDP: u64 = 1u64 << 20;
pub const HWCAP_SHA512: u64 = 1u64 << 21;
pub const HWCAP_SVE: u64 = 1u64 << 22;
pub const HWCAP_ASIMDFHM: u64 = 1u64 << 23;
pub const HWCAP_DIT: u64 = 1u64 << 24;
pub const HWCAP_USCAT: u64 = 1u64 << 25;
pub const HWCAP_ILRCPC: u64 = 1u64 << 26;
pub const HWCAP_FLAGM: u64 = 1u64 << 27;
pub const HWCAP_SSBS: u64 = 1u64 << 28;
pub const HWCAP_SB: u64 = 1u64 << 29;
pub const HWCAP_PACA: u64 = 1u64 << 30;
pub const HWCAP_PACG: u64 = 1u64 << 31;
pub const HWCAP_GCS: u64 = 1u64 << 32;
pub const HWCAP_CMPBR: u64 = 1u64 << 33;
pub const HWCAP_FPRCVT: u64 = 1u64 << 34;
pub const HWCAP_F8MM8: u64 = 1u64 << 35;
pub const HWCAP_F8MM4: u64 = 1u64 << 36;
pub const HWCAP_SVE_F16MM: u64 = 1u64 << 37;
pub const HWCAP_SVE_ELTPERM: u64 = 1u64 << 38;
pub const HWCAP_SVE_AES2: u64 = 1u64 << 39;
pub const HWCAP_SVE_BFSCALE: u64 = 1u64 << 40;
pub const HWCAP_SVE2P2: u64 = 1u64 << 41;
pub const HWCAP_SME2P2: u64 = 1u64 << 42;
pub const HWCAP_SME_SBITPERM: u64 = 1u64 << 43;
pub const HWCAP_SME_AES: u64 = 1u64 << 44;
pub const HWCAP_SME_SFEXPA: u64 = 1u64 << 45;
pub const HWCAP_SME_STMOP: u64 = 1u64 << 46;
pub const HWCAP_SME_SMOP4: u64 = 1u64 << 47;

/* HWCAP2 flags - for AT_HWCAP2 */
pub const HWCAP2_DCPODP: u64 = 1u64 << 0;
pub const HWCAP2_SVE2: u64 = 1u64 << 1;
pub const HWCAP2_SVEAES: u64 = 1u64 << 2;
pub const HWCAP2_SVEPMULL: u64 = 1u64 << 3;
pub const HWCAP2_SVEBITPERM: u64 = 1u64 << 4;
pub const HWCAP2_SVESHA3: u64 = 1u64 << 5;
pub const HWCAP2_SVESM4: u64 = 1u64 << 6;
pub const HWCAP2_FLAGM2: u64 = 1u64 << 7;
pub const HWCAP2_FRINT: u64 = 1u64 << 8;
pub const HWCAP2_SVEI8MM: u64 = 1u64 << 9;
pub const HWCAP2_SVEF32MM: u64 = 1u64 << 10;
pub const HWCAP2_SVEF64MM: u64 = 1u64 << 11;
pub const HWCAP2_SVEBF16: u64 = 1u64 << 12;
pub const HWCAP2_I8MM: u64 = 1u64 << 13;
pub const HWCAP2_BF16: u64 = 1u64 << 14;
pub const HWCAP2_DGH: u64 = 1u64 << 15;
pub const HWCAP2_RNG: u64 = 1u64 << 16;
pub const HWCAP2_BTI: u64 = 1u64 << 17;
pub const HWCAP2_MTE: u64 = 1u64 << 18;
pub const HWCAP2_ECV: u64 = 1u64 << 19;
pub const HWCAP2_AFP: u64 = 1u64 << 20;
pub const HWCAP2_RPRES: u64 = 1u64 << 21;
pub const HWCAP2_MTE3: u64 = 1u64 << 22;
pub const HWCAP2_SME: u64 = 1u64 << 23;
pub const HWCAP2_SME_I16I64: u64 = 1u64 << 24;
pub const HWCAP2_SME_F64F64: u64 = 1u64 << 25;
pub const HWCAP2_SME_I8I32: u64 = 1u64 << 26;
pub const HWCAP2_SME_F16F32: u64 = 1u64 << 27;
pub const HWCAP2_SME_B16F32: u64 = 1u64 << 28;
pub const HWCAP2_SME_F32F32: u64 = 1u64 << 29;
pub const HWCAP2_SME_FA64: u64 = 1u64 << 30;
pub const HWCAP2_WFXT: u64 = 1u64 << 31;
pub const HWCAP2_EBF16: u64 = 1u64 << 32;
pub const HWCAP2_SVE_EBF16: u64 = 1u64 << 33;
pub const HWCAP2_CSSC: u64 = 1u64 << 34;
pub const HWCAP2_RPRFM: u64 = 1u64 << 35;
pub const HWCAP2_SVE2P1: u64 = 1u64 << 36;
pub const HWCAP2_SME2: u64 = 1u64 << 37;
pub const HWCAP2_SME2P1: u64 = 1u64 << 38;
pub const HWCAP2_SME_I16I32: u64 = 1u64 << 39;
pub const HWCAP2_SME_BI32I32: u64 = 1u64 << 40;
pub const HWCAP2_SME_B16B16: u64 = 1u64 << 41;
pub const HWCAP2_SME_F16F16: u64 = 1u64 << 42;
pub const HWCAP2_MOPS: u64 = 1u64 << 43;
pub const HWCAP2_HBC: u64 = 1u64 << 44;
pub const HWCAP2_SVE_B16B16: u64 = 1u64 << 45;
pub const HWCAP2_LRCPC3: u64 = 1u64 << 46;
pub const HWCAP2_LSE128: u64 = 1u64 << 47;
pub const HWCAP2_FPMR: u64 = 1u64 << 48;
pub const HWCAP2_LUT: u64 = 1u64 << 49;
pub const HWCAP2_FAMINMAX: u64 = 1u64 << 50;
pub const HWCAP2_F8CVT: u64 = 1u64 << 51;
pub const HWCAP2_F8FMA: u64 = 1u64 << 52;
pub const HWCAP2_F8DP4: u64 = 1u64 << 53;
pub const HWCAP2_F8DP2: u64 = 1u64 << 54;
pub const HWCAP2_F8E4M3: u64 = 1u64 << 55;
pub const HWCAP2_F8E5M2: u64 = 1u64 << 56;
pub const HWCAP2_SME_LUTV2: u64 = 1u64 << 57;
pub const HWCAP2_SME_F8F16: u64 = 1u64 << 58;
pub const HWCAP2_SME_F8F32: u64 = 1u64 << 59;
pub const HWCAP2_SME_SF8FMA: u64 = 1u64 << 60;
pub const HWCAP2_SME_SF8DP4: u64 = 1u64 << 61;
pub const HWCAP2_SME_SF8DP2: u64 = 1u64 << 62;
pub const HWCAP2_POE: u64 = 1u64 << 63;

/* HWCAP3 flags - for AT_HWCAP3 */
pub const HWCAP3_MTE_FAR: u64 = 1u64 << 0;
pub const HWCAP3_MTE_STORE_ONLY: u64 = 1u64 << 1;
pub const HWCAP3_LSFE: u64 = 1u64 << 2;
pub const HWCAP3_LS64: u64 = 1u64 << 3;
pub const HWCAP3_SVE_B16MM: u64 = 1u64 << 4;
pub const HWCAP3_SVE2P3: u64 = 1u64 << 5;
pub const HWCAP3_SME_LUT6: u64 = 1u64 << 6;
pub const HWCAP3_SME2P3: u64 = 1u64 << 7;
pub const HWCAP3_F16MM: u64 = 1u64 << 8;
pub const HWCAP3_F16F32DOT: u64 = 1u64 << 9;
pub const HWCAP3_F16F32MM: u64 = 1u64 << 10;
pub const HWCAP3_SVE_LUT6: u64 = 1u64 << 11;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
