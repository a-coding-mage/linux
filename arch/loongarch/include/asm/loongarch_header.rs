// SPDX-License-Identifier: GPL-2.0
// Rust translation of loongarch.h
#![allow(non_upper_case_globals, dead_code, unused_macros)]

pub const fn BIT(n: u32) -> u64 { 1u64 << n }
pub const fn BIT_ULL(n: u32) -> u64 { 1u64 << n }
pub const fn GENMASK(h: u32, l: u32) -> u64 { (u64::MAX >> (63 - h)) & (u64::MAX << l) }

/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2020-2022 Loongson Technology Corporation Limited
 */
// #ifndef _ASM_LOONGARCH_H
pub const _ASM_LOONGARCH_H: u64 = ;

// #include <linux/bits.h>
// #include <linux/linkage.h>
// #include <linux/types.h>

// #ifndef __ASSEMBLER__
// #include <larchintrin.h>

/* CPUCFG */
// #define read_cpucfg(reg) __cpucfg(reg)

// #endif /* !__ASSEMBLER__ */

// #ifdef __ASSEMBLER__

/* LoongArch Registers */
pub const REG_ZERO: u64 = 0x0;
pub const REG_RA: u64 = 0x1;
pub const REG_TP: u64 = 0x2;
pub const REG_SP: u64 = 0x3;
pub const REG_A0: u64 = 0x4 /* Reused as V0 for return value */;
pub const REG_A1: u64 = 0x5 /* Reused as V1 for return value */;
pub const REG_A2: u64 = 0x6;
pub const REG_A3: u64 = 0x7;
pub const REG_A4: u64 = 0x8;
pub const REG_A5: u64 = 0x9;
pub const REG_A6: u64 = 0xa;
pub const REG_A7: u64 = 0xb;
pub const REG_T0: u64 = 0xc;
pub const REG_T1: u64 = 0xd;
pub const REG_T2: u64 = 0xe;
pub const REG_T3: u64 = 0xf;
pub const REG_T4: u64 = 0x10;
pub const REG_T5: u64 = 0x11;
pub const REG_T6: u64 = 0x12;
pub const REG_T7: u64 = 0x13;
pub const REG_T8: u64 = 0x14;
pub const REG_U0: u64 = 0x15 /* Kernel uses it as percpu base */;
pub const REG_FP: u64 = 0x16;
pub const REG_S0: u64 = 0x17;
pub const REG_S1: u64 = 0x18;
pub const REG_S2: u64 = 0x19;
pub const REG_S3: u64 = 0x1a;
pub const REG_S4: u64 = 0x1b;
pub const REG_S5: u64 = 0x1c;
pub const REG_S6: u64 = 0x1d;
pub const REG_S7: u64 = 0x1e;
pub const REG_S8: u64 = 0x1f;

// #endif /* __ASSEMBLER__ */

/* Bit fields for CPUCFG registers */
pub const LOONGARCH_CPUCFG0: u64 = 0x0;
pub const CPUCFG0_PRID: u64 = GENMASK(31, 0);

pub const LOONGARCH_CPUCFG1: u64 = 0x1;
pub const CPUCFG1_ISGR32: u64 = BIT(0);
pub const CPUCFG1_ISGR64: u64 = BIT(1);
pub const CPUCFG1_ISA: u64 = GENMASK(1, 0);
pub const CPUCFG1_PAGING: u64 = BIT(2);
pub const CPUCFG1_IOCSR: u64 = BIT(3);
pub const CPUCFG1_PABITS: u64 = GENMASK(11, 4);
pub const CPUCFG1_VABITS: u64 = GENMASK(19, 12);
pub const CPUCFG1_UAL: u64 = BIT(20);
pub const CPUCFG1_RI: u64 = BIT(21);
pub const CPUCFG1_EP: u64 = BIT(22);
pub const CPUCFG1_RPLV: u64 = BIT(23);
pub const CPUCFG1_HUGEPG: u64 = BIT(24);
pub const CPUCFG1_CRC32: u64 = BIT(25);
pub const CPUCFG1_MSGINT: u64 = BIT(26);

pub const LOONGARCH_CPUCFG2: u64 = 0x2;
pub const CPUCFG2_FP: u64 = BIT(0);
pub const CPUCFG2_FPSP: u64 = BIT(1);
pub const CPUCFG2_FPDP: u64 = BIT(2);
pub const CPUCFG2_FPVERS: u64 = GENMASK(5, 3);
pub const CPUCFG2_LSX: u64 = BIT(6);
pub const CPUCFG2_LASX: u64 = BIT(7);
pub const CPUCFG2_COMPLEX: u64 = BIT(8);
pub const CPUCFG2_CRYPTO: u64 = BIT(9);
pub const CPUCFG2_LVZP: u64 = BIT(10);
pub const CPUCFG2_LVZVER: u64 = GENMASK(13, 11);
pub const CPUCFG2_LLFTP: u64 = BIT(14);
pub const CPUCFG2_LLFTPREV: u64 = GENMASK(17, 15);
pub const CPUCFG2_X86BT: u64 = BIT(18);
pub const CPUCFG2_ARMBT: u64 = BIT(19);
pub const CPUCFG2_MIPSBT: u64 = BIT(20);
pub const CPUCFG2_LSPW: u64 = BIT(21);
pub const CPUCFG2_LAM: u64 = BIT(22);
pub const CPUCFG2_PTW: u64 = BIT(24);
pub const CPUCFG2_FRECIPE: u64 = BIT(25);
pub const CPUCFG2_DIV32: u64 = BIT(26);
pub const CPUCFG2_LAM_BH: u64 = BIT(27);
pub const CPUCFG2_LAMCAS: u64 = BIT(28);
pub const CPUCFG2_LLACQ_SCREL: u64 = BIT(29);
pub const CPUCFG2_SCQ: u64 = BIT(30);

pub const LOONGARCH_CPUCFG3: u64 = 0x3;
pub const CPUCFG3_CCDMA: u64 = BIT(0);
pub const CPUCFG3_SFB: u64 = BIT(1);
pub const CPUCFG3_UCACC: u64 = BIT(2);
pub const CPUCFG3_LLEXC: u64 = BIT(3);
pub const CPUCFG3_SCDLY: u64 = BIT(4);
pub const CPUCFG3_LLDBAR: u64 = BIT(5);
pub const CPUCFG3_ITLBT: u64 = BIT(6);
pub const CPUCFG3_ICACHET: u64 = BIT(7);
pub const CPUCFG3_SPW_LVL: u64 = GENMASK(10, 8);
pub const CPUCFG3_SPW_HG_HF: u64 = BIT(11);
pub const CPUCFG3_RVA: u64 = BIT(12);
pub const CPUCFG3_RVAMAX: u64 = GENMASK(16, 13);
pub const CPUCFG3_DBAR_HINTS: u64 = BIT(17);
pub const CPUCFG3_ALDORDER_CAP: u64 = BIT(18) /* All address load ordered, capability */;
pub const CPUCFG3_ASTORDER_CAP: u64 = BIT(19) /* All address store ordered, capability */;
pub const CPUCFG3_ALDORDER_STA: u64 = BIT(20) /* All address load ordered, status */;
pub const CPUCFG3_ASTORDER_STA: u64 = BIT(21) /* All address store ordered, status */;
pub const CPUCFG3_SLDORDER_CAP: u64 = BIT(22) /* Same address load ordered, capability */;
pub const CPUCFG3_SLDORDER_STA: u64 = BIT(23) /* Same address load ordered, status */;

pub const LOONGARCH_CPUCFG4: u64 = 0x4;
pub const CPUCFG4_CCFREQ: u64 = GENMASK(31, 0);

pub const LOONGARCH_CPUCFG5: u64 = 0x5;
pub const CPUCFG5_CCMUL: u64 = GENMASK(15, 0);
pub const CPUCFG5_CCDIV: u64 = GENMASK(31, 16);

pub const LOONGARCH_CPUCFG6: u64 = 0x6;
pub const CPUCFG6_PMP: u64 = BIT(0);
pub const CPUCFG6_PAMVER: u64 = GENMASK(3, 1);
pub const CPUCFG6_PMNUM: u64 = GENMASK(7, 4);
pub const CPUCFG6_PMNUM_SHIFT: u64 = 4;
pub const CPUCFG6_PMBITS: u64 = GENMASK(13, 8);
pub const CPUCFG6_PMBITS_SHIFT: u64 = 8;
pub const CPUCFG6_UPM: u64 = BIT(14);

pub const LOONGARCH_CPUCFG16: u64 = 0x10;
pub const CPUCFG16_L1_IUPRE: u64 = BIT(0);
pub const CPUCFG16_L1_IUUNIFY: u64 = BIT(1);
pub const CPUCFG16_L1_DPRE: u64 = BIT(2);
pub const CPUCFG16_L2_IUPRE: u64 = BIT(3);
pub const CPUCFG16_L2_IUUNIFY: u64 = BIT(4);
pub const CPUCFG16_L2_IUPRIV: u64 = BIT(5);
pub const CPUCFG16_L2_IUINCL: u64 = BIT(6);
pub const CPUCFG16_L2_DPRE: u64 = BIT(7);
pub const CPUCFG16_L2_DPRIV: u64 = BIT(8);
pub const CPUCFG16_L2_DINCL: u64 = BIT(9);
pub const CPUCFG16_L3_IUPRE: u64 = BIT(10);
pub const CPUCFG16_L3_IUUNIFY: u64 = BIT(11);
pub const CPUCFG16_L3_IUPRIV: u64 = BIT(12);
pub const CPUCFG16_L3_IUINCL: u64 = BIT(13);
pub const CPUCFG16_L3_DPRE: u64 = BIT(14);
pub const CPUCFG16_L3_DPRIV: u64 = BIT(15);
pub const CPUCFG16_L3_DINCL: u64 = BIT(16);

pub const LOONGARCH_CPUCFG17: u64 = 0x11;
pub const LOONGARCH_CPUCFG18: u64 = 0x12;
pub const LOONGARCH_CPUCFG19: u64 = 0x13;
pub const LOONGARCH_CPUCFG20: u64 = 0x14;
pub const CPUCFG_CACHE_WAYS_M: u64 = GENMASK(15, 0);
pub const CPUCFG_CACHE_SETS_M: u64 = GENMASK(23, 16);
pub const CPUCFG_CACHE_LSIZE_M: u64 = GENMASK(30, 24);
pub const CPUCFG_CACHE_WAYS: u64 = 0;
pub const CPUCFG_CACHE_SETS: u64 = 16;
pub const CPUCFG_CACHE_LSIZE: u64 = 24;

pub const LOONGARCH_CPUCFG48: u64 = 0x30;
pub const CPUCFG48_MCSR_LCK: u64 = BIT(0);
pub const CPUCFG48_NAP_EN: u64 = BIT(1);
pub const CPUCFG48_VFPU_CG: u64 = BIT(2);
pub const CPUCFG48_RAM_CG: u64 = BIT(3);

/*
 * CPUCFG index area: 0x40000000 -- 0x400000ff
 * SW emulation for KVM hypervirsor, see arch/loongarch/include/uapi/asm/kvm_para.h
 */

// #ifndef __ASSEMBLER__

/* CSR */
// #define csr_read32(reg) __csrrd_w(reg)
// #define csr_read64(reg) __csrrd_d(reg)
// #define csr_write32(val, reg) __csrwr_w(val, reg)
// #define csr_write64(val, reg) __csrwr_d(val, reg)
// #define csr_xchg32(val, mask, reg) __csrxchg_w(val, mask, reg)
// #define csr_xchg64(val, mask, reg) __csrxchg_d(val, mask, reg)

// #ifdef CONFIG_32BIT
// #define csr_read(reg) csr_read32(reg)
// #define csr_write(val, reg) csr_write32(val, reg)
// #define csr_xchg(val, mask, reg) csr_xchg32(val, mask, reg)
// #else
// #define csr_read(reg) csr_read64(reg)
// #define csr_write(val, reg) csr_write64(val, reg)
// #define csr_xchg(val, mask, reg) csr_xchg64(val, mask, reg)
// #endif

/* IOCSR */
// #define iocsr_read32(reg) __iocsrrd_w(reg)
// #define iocsr_read64(reg) __iocsrrd_d(reg)
// #define iocsr_write32(val, reg) __iocsrwr_w(val, reg)
// #define iocsr_write64(val, reg) __iocsrwr_d(val, reg)

// #endif /* !__ASSEMBLER__ */

/* CSR register number */

/* Basic CSR registers */
pub const LOONGARCH_CSR_CRMD: u64 = 0x0	/* Current mode info */;
pub const CSR_CRMD_WE_SHIFT: u64 = 9;
// #define  CSR_CRMD_WE			(_ULCAST_(0x1) << CSR_CRMD_WE_SHIFT)
pub const CSR_CRMD_DACM_SHIFT: u64 = 7;
pub const CSR_CRMD_DACM_WIDTH: u64 = 2;
// #define  CSR_CRMD_DACM			(_ULCAST_(0x3) << CSR_CRMD_DACM_SHIFT)
pub const CSR_CRMD_DACF_SHIFT: u64 = 5;
pub const CSR_CRMD_DACF_WIDTH: u64 = 2;
// #define  CSR_CRMD_DACF			(_ULCAST_(0x3) << CSR_CRMD_DACF_SHIFT)
pub const CSR_CRMD_PG_SHIFT: u64 = 4;
// #define  CSR_CRMD_PG			(_ULCAST_(0x1) << CSR_CRMD_PG_SHIFT)
pub const CSR_CRMD_DA_SHIFT: u64 = 3;
// #define  CSR_CRMD_DA			(_ULCAST_(0x1) << CSR_CRMD_DA_SHIFT)
pub const CSR_CRMD_IE_SHIFT: u64 = 2;
// #define  CSR_CRMD_IE			(_ULCAST_(0x1) << CSR_CRMD_IE_SHIFT)
pub const CSR_CRMD_PLV_SHIFT: u64 = 0;
pub const CSR_CRMD_PLV_WIDTH: u64 = 2;
// #define  CSR_CRMD_PLV			(_ULCAST_(0x3) << CSR_CRMD_PLV_SHIFT)

pub const PLV_KERN: u64 = 0;
pub const PLV_USER: u64 = 3;
pub const PLV_MASK: u64 = 0x3;

pub const LOONGARCH_CSR_PRMD: u64 = 0x1	/* Prev-exception mode info */;
pub const CSR_PRMD_PWE_SHIFT: u64 = 3;
// #define  CSR_PRMD_PWE			(_ULCAST_(0x1) << CSR_PRMD_PWE_SHIFT)
pub const CSR_PRMD_PIE_SHIFT: u64 = 2;
// #define  CSR_PRMD_PIE			(_ULCAST_(0x1) << CSR_PRMD_PIE_SHIFT)
pub const CSR_PRMD_PPLV_SHIFT: u64 = 0;
pub const CSR_PRMD_PPLV_WIDTH: u64 = 2;
// #define  CSR_PRMD_PPLV			(_ULCAST_(0x3) << CSR_PRMD_PPLV_SHIFT)

pub const LOONGARCH_CSR_EUEN: u64 = 0x2	/* Extended unit enable */;
pub const CSR_EUEN_LBTEN_SHIFT: u64 = 3;
// #define  CSR_EUEN_LBTEN			(_ULCAST_(0x1) << CSR_EUEN_LBTEN_SHIFT)
pub const CSR_EUEN_LASXEN_SHIFT: u64 = 2;
// #define  CSR_EUEN_LASXEN		(_ULCAST_(0x1) << CSR_EUEN_LASXEN_SHIFT)
pub const CSR_EUEN_LSXEN_SHIFT: u64 = 1;
// #define  CSR_EUEN_LSXEN			(_ULCAST_(0x1) << CSR_EUEN_LSXEN_SHIFT)
pub const CSR_EUEN_FPEN_SHIFT: u64 = 0;
// #define  CSR_EUEN_FPEN			(_ULCAST_(0x1) << CSR_EUEN_FPEN_SHIFT)

pub const LOONGARCH_CSR_MISC: u64 = 0x3	/* Misc config */;

pub const LOONGARCH_CSR_ECFG: u64 = 0x4	/* Exception config */;
pub const CSR_ECFG_VS_SHIFT: u64 = 16;
pub const CSR_ECFG_VS_WIDTH: u64 = 3;
// #define  CSR_ECFG_VS_SHIFT_END		(CSR_ECFG_VS_SHIFT + CSR_ECFG_VS_WIDTH - 1)
// #define  CSR_ECFG_VS			(_ULCAST_(0x7) << CSR_ECFG_VS_SHIFT)
pub const CSR_ECFG_IM_SHIFT: u64 = 0;
pub const CSR_ECFG_IM_WIDTH: u64 = 14;
// #define  CSR_ECFG_IM			(_ULCAST_(0x3fff) << CSR_ECFG_IM_SHIFT)

pub const LOONGARCH_CSR_ESTAT: u64 = 0x5	/* Exception status */;
pub const CSR_ESTAT_ESUBCODE_SHIFT: u64 = 22;
pub const CSR_ESTAT_ESUBCODE_WIDTH: u64 = 9;
// #define  CSR_ESTAT_ESUBCODE		(_ULCAST_(0x1ff) << CSR_ESTAT_ESUBCODE_SHIFT)
pub const CSR_ESTAT_EXC_SHIFT: u64 = 16;
pub const CSR_ESTAT_EXC_WIDTH: u64 = 6;
// #define  CSR_ESTAT_EXC			(_ULCAST_(0x3f) << CSR_ESTAT_EXC_SHIFT)
pub const CSR_ESTAT_IS_SHIFT: u64 = 0;
pub const CSR_ESTAT_IS_WIDTH: u64 = 15;
// #define  CSR_ESTAT_IS			(_ULCAST_(0x7fff) << CSR_ESTAT_IS_SHIFT)

pub const LOONGARCH_CSR_ERA: u64 = 0x6	/* Exception return address */;

pub const LOONGARCH_CSR_BADV: u64 = 0x7	/* Bad virtual address */;

pub const LOONGARCH_CSR_BADI: u64 = 0x8	/* Bad instruction */;

pub const LOONGARCH_CSR_EENTRY: u64 = 0xc	/* Exception entry */;

/* TLB related CSR registers */
pub const LOONGARCH_CSR_TLBIDX: u64 = 0x10	/* TLB Index, EHINV, PageSize, NP */;
pub const CSR_TLBIDX_EHINV_SHIFT: u64 = 31;
// #define  CSR_TLBIDX_EHINV		(_ULCAST_(1) << CSR_TLBIDX_EHINV_SHIFT)
pub const CSR_TLBIDX_PS_SHIFT: u64 = 24;
pub const CSR_TLBIDX_PS_WIDTH: u64 = 6;
// #define  CSR_TLBIDX_PS			(_ULCAST_(0x3f) << CSR_TLBIDX_PS_SHIFT)
pub const CSR_TLBIDX_IDX_SHIFT: u64 = 0;
pub const CSR_TLBIDX_IDX_WIDTH: u64 = 12;
// #define  CSR_TLBIDX_IDX			(_ULCAST_(0xfff) << CSR_TLBIDX_IDX_SHIFT)
pub const CSR_TLBIDX_SIZEM: u64 = 0x3f000000;
pub const CSR_TLBIDX_SIZE: u64 = CSR_TLBIDX_PS_SHIFT;
pub const CSR_TLBIDX_IDXM: u64 = 0xfff;
// #define  CSR_INVALID_ENTRY(e)		(CSR_TLBIDX_EHINV | e)

pub const LOONGARCH_CSR_TLBEHI: u64 = 0x11	/* TLB EntryHi */;

pub const LOONGARCH_CSR_TLBELO0: u64 = 0x12	/* TLB EntryLo0 */;
pub const CSR_TLBLO0_RPLV_SHIFT: u64 = 63;
// #define  CSR_TLBLO0_RPLV		(_ULCAST_(0x1) << CSR_TLBLO0_RPLV_SHIFT)
pub const CSR_TLBLO0_NX_SHIFT: u64 = 62;
// #define  CSR_TLBLO0_NX			(_ULCAST_(0x1) << CSR_TLBLO0_NX_SHIFT)
pub const CSR_TLBLO0_NR_SHIFT: u64 = 61;
// #define  CSR_TLBLO0_NR			(_ULCAST_(0x1) << CSR_TLBLO0_NR_SHIFT)
pub const CSR_TLBLO0_PFN_SHIFT: u64 = 12;
pub const CSR_TLBLO0_PFN_WIDTH: u64 = 36;
// #define  CSR_TLBLO0_PFN			(_ULCAST_(0xfffffffff) << CSR_TLBLO0_PFN_SHIFT)
pub const CSR_TLBLO0_GLOBAL_SHIFT: u64 = 6;
// #define  CSR_TLBLO0_GLOBAL		(_ULCAST_(0x1) << CSR_TLBLO0_GLOBAL_SHIFT)
pub const CSR_TLBLO0_CCA_SHIFT: u64 = 4;
pub const CSR_TLBLO0_CCA_WIDTH: u64 = 2;
// #define  CSR_TLBLO0_CCA			(_ULCAST_(0x3) << CSR_TLBLO0_CCA_SHIFT)
pub const CSR_TLBLO0_PLV_SHIFT: u64 = 2;
pub const CSR_TLBLO0_PLV_WIDTH: u64 = 2;
// #define  CSR_TLBLO0_PLV			(_ULCAST_(0x3) << CSR_TLBLO0_PLV_SHIFT)
pub const CSR_TLBLO0_WE_SHIFT: u64 = 1;
// #define  CSR_TLBLO0_WE			(_ULCAST_(0x1) << CSR_TLBLO0_WE_SHIFT)
pub const CSR_TLBLO0_V_SHIFT: u64 = 0;
// #define  CSR_TLBLO0_V			(_ULCAST_(0x1) << CSR_TLBLO0_V_SHIFT)

pub const LOONGARCH_CSR_TLBELO1: u64 = 0x13	/* TLB EntryLo1 */;
pub const CSR_TLBLO1_RPLV_SHIFT: u64 = 63;
// #define  CSR_TLBLO1_RPLV		(_ULCAST_(0x1) << CSR_TLBLO1_RPLV_SHIFT)
pub const CSR_TLBLO1_NX_SHIFT: u64 = 62;
// #define  CSR_TLBLO1_NX			(_ULCAST_(0x1) << CSR_TLBLO1_NX_SHIFT)
pub const CSR_TLBLO1_NR_SHIFT: u64 = 61;
// #define  CSR_TLBLO1_NR			(_ULCAST_(0x1) << CSR_TLBLO1_NR_SHIFT)
pub const CSR_TLBLO1_PFN_SHIFT: u64 = 12;
pub const CSR_TLBLO1_PFN_WIDTH: u64 = 36;
// #define  CSR_TLBLO1_PFN			(_ULCAST_(0xfffffffff) << CSR_TLBLO1_PFN_SHIFT)
pub const CSR_TLBLO1_GLOBAL_SHIFT: u64 = 6;
// #define  CSR_TLBLO1_GLOBAL		(_ULCAST_(0x1) << CSR_TLBLO1_GLOBAL_SHIFT)
pub const CSR_TLBLO1_CCA_SHIFT: u64 = 4;
pub const CSR_TLBLO1_CCA_WIDTH: u64 = 2;
// #define  CSR_TLBLO1_CCA			(_ULCAST_(0x3) << CSR_TLBLO1_CCA_SHIFT)
pub const CSR_TLBLO1_PLV_SHIFT: u64 = 2;
pub const CSR_TLBLO1_PLV_WIDTH: u64 = 2;
// #define  CSR_TLBLO1_PLV			(_ULCAST_(0x3) << CSR_TLBLO1_PLV_SHIFT)
pub const CSR_TLBLO1_WE_SHIFT: u64 = 1;
// #define  CSR_TLBLO1_WE			(_ULCAST_(0x1) << CSR_TLBLO1_WE_SHIFT)
pub const CSR_TLBLO1_V_SHIFT: u64 = 0;
// #define  CSR_TLBLO1_V			(_ULCAST_(0x1) << CSR_TLBLO1_V_SHIFT)

pub const LOONGARCH_CSR_GTLBC: u64 = 0x15	/* Guest TLB control */;
pub const CSR_GTLBC_TGID_SHIFT: u64 = 16;
pub const CSR_GTLBC_TGID_WIDTH: u64 = 8;
// #define  CSR_GTLBC_TGID_SHIFT_END	(CSR_GTLBC_TGID_SHIFT + CSR_GTLBC_TGID_WIDTH - 1)
// #define  CSR_GTLBC_TGID			(_ULCAST_(0xff) << CSR_GTLBC_TGID_SHIFT)
pub const CSR_GTLBC_TOTI_SHIFT: u64 = 13;
// #define  CSR_GTLBC_TOTI			(_ULCAST_(0x1) << CSR_GTLBC_TOTI_SHIFT)
pub const CSR_GTLBC_USETGID_SHIFT: u64 = 12;
// #define  CSR_GTLBC_USETGID		(_ULCAST_(0x1) << CSR_GTLBC_USETGID_SHIFT)
pub const CSR_GTLBC_GMTLBSZ_SHIFT: u64 = 0;
pub const CSR_GTLBC_GMTLBSZ_WIDTH: u64 = 6;
// #define  CSR_GTLBC_GMTLBSZ		(_ULCAST_(0x3f) << CSR_GTLBC_GMTLBSZ_SHIFT)

pub const LOONGARCH_CSR_TRGP: u64 = 0x16	/* TLBR read guest info */;
pub const CSR_TRGP_RID_SHIFT: u64 = 16;
pub const CSR_TRGP_RID_WIDTH: u64 = 8;
// #define  CSR_TRGP_RID			(_ULCAST_(0xff) << CSR_TRGP_RID_SHIFT)
pub const CSR_TRGP_GTLB_SHIFT: u64 = 0;
// #define  CSR_TRGP_GTLB			(1 << CSR_TRGP_GTLB_SHIFT)

pub const LOONGARCH_CSR_ASID: u64 = 0x18	/* ASID */;
pub const CSR_ASID_BIT_SHIFT: u64 = 16	/* ASIDBits */;
pub const CSR_ASID_BIT_WIDTH: u64 = 8;
// #define  CSR_ASID_BIT			(_ULCAST_(0xff) << CSR_ASID_BIT_SHIFT)
pub const CSR_ASID_ASID_SHIFT: u64 = 0;
pub const CSR_ASID_ASID_WIDTH: u64 = 10;
// #define  CSR_ASID_ASID			(_ULCAST_(0x3ff) << CSR_ASID_ASID_SHIFT)

pub const LOONGARCH_CSR_PGDL: u64 = 0x19	/* Page table base address when VA[VALEN-1] = 0 */;

pub const LOONGARCH_CSR_PGDH: u64 = 0x1a	/* Page table base address when VA[VALEN-1] = 1 */;

pub const LOONGARCH_CSR_PGD: u64 = 0x1b	/* Page table base */;

pub const LOONGARCH_CSR_PWCTL0: u64 = 0x1c	/* PWCtl0 */;
pub const CSR_PWCTL0_PTEW_SHIFT: u64 = 30;
pub const CSR_PWCTL0_PTEW_WIDTH: u64 = 2;
// #define  CSR_PWCTL0_PTEW		(_ULCAST_(0x3) << CSR_PWCTL0_PTEW_SHIFT)
pub const CSR_PWCTL0_DIR1WIDTH_SHIFT: u64 = 25;
pub const CSR_PWCTL0_DIR1WIDTH_WIDTH: u64 = 5;
// #define  CSR_PWCTL0_DIR1WIDTH		(_ULCAST_(0x1f) << CSR_PWCTL0_DIR1WIDTH_SHIFT)
pub const CSR_PWCTL0_DIR1BASE_SHIFT: u64 = 20;
pub const CSR_PWCTL0_DIR1BASE_WIDTH: u64 = 5;
// #define  CSR_PWCTL0_DIR1BASE		(_ULCAST_(0x1f) << CSR_PWCTL0_DIR1BASE_SHIFT)
pub const CSR_PWCTL0_DIR0WIDTH_SHIFT: u64 = 15;
pub const CSR_PWCTL0_DIR0WIDTH_WIDTH: u64 = 5;
// #define  CSR_PWCTL0_DIR0WIDTH		(_ULCAST_(0x1f) << CSR_PWCTL0_DIR0WIDTH_SHIFT)
pub const CSR_PWCTL0_DIR0BASE_SHIFT: u64 = 10;
pub const CSR_PWCTL0_DIR0BASE_WIDTH: u64 = 5;
// #define  CSR_PWCTL0_DIR0BASE		(_ULCAST_(0x1f) << CSR_PWCTL0_DIR0BASE_SHIFT)
pub const CSR_PWCTL0_PTWIDTH_SHIFT: u64 = 5;
pub const CSR_PWCTL0_PTWIDTH_WIDTH: u64 = 5;
// #define  CSR_PWCTL0_PTWIDTH		(_ULCAST_(0x1f) << CSR_PWCTL0_PTWIDTH_SHIFT)
pub const CSR_PWCTL0_PTBASE_SHIFT: u64 = 0;
pub const CSR_PWCTL0_PTBASE_WIDTH: u64 = 5;
// #define  CSR_PWCTL0_PTBASE		(_ULCAST_(0x1f) << CSR_PWCTL0_PTBASE_SHIFT)

pub const LOONGARCH_CSR_PWCTL1: u64 = 0x1d	/* PWCtl1 */;
pub const CSR_PWCTL1_PTW_SHIFT: u64 = 24;
pub const CSR_PWCTL1_PTW_WIDTH: u64 = 1;
// #define  CSR_PWCTL1_PTW			(_ULCAST_(0x1) << CSR_PWCTL1_PTW_SHIFT)
pub const CSR_PWCTL1_DIR3WIDTH_SHIFT: u64 = 18;
pub const CSR_PWCTL1_DIR3WIDTH_WIDTH: u64 = 5;
// #define  CSR_PWCTL1_DIR3WIDTH		(_ULCAST_(0x1f) << CSR_PWCTL1_DIR3WIDTH_SHIFT)
pub const CSR_PWCTL1_DIR3BASE_SHIFT: u64 = 12;
pub const CSR_PWCTL1_DIR3BASE_WIDTH: u64 = 5;
// #define  CSR_PWCTL1_DIR3BASE		(_ULCAST_(0x1f) << CSR_PWCTL0_DIR3BASE_SHIFT)
pub const CSR_PWCTL1_DIR2WIDTH_SHIFT: u64 = 6;
pub const CSR_PWCTL1_DIR2WIDTH_WIDTH: u64 = 5;
// #define  CSR_PWCTL1_DIR2WIDTH		(_ULCAST_(0x1f) << CSR_PWCTL1_DIR2WIDTH_SHIFT)
pub const CSR_PWCTL1_DIR2BASE_SHIFT: u64 = 0;
pub const CSR_PWCTL1_DIR2BASE_WIDTH: u64 = 5;
// #define  CSR_PWCTL1_DIR2BASE		(_ULCAST_(0x1f) << CSR_PWCTL0_DIR2BASE_SHIFT)

pub const LOONGARCH_CSR_STLBPGSIZE: u64 = 0x1e;
pub const CSR_STLBPGSIZE_PS_WIDTH: u64 = 6;
// #define  CSR_STLBPGSIZE_PS		(_ULCAST_(0x3f))

pub const LOONGARCH_CSR_RVACFG: u64 = 0x1f;
pub const CSR_RVACFG_RDVA_WIDTH: u64 = 4;
// #define  CSR_RVACFG_RDVA		(_ULCAST_(0xf))

/* Config CSR registers */
pub const LOONGARCH_CSR_CPUID: u64 = 0x20	/* CPU core id */;
pub const CSR_CPUID_COREID_WIDTH: u64 = 11;
pub const CSR_CPUID_COREID: u64 = (0x7ff);

pub const LOONGARCH_CSR_PRCFG1: u64 = 0x21	/* Config1 */;
pub const CSR_CONF1_VSMAX_SHIFT: u64 = 12;
pub const CSR_CONF1_VSMAX_WIDTH: u64 = 3;
// #define  CSR_CONF1_VSMAX		(_ULCAST_(7) << CSR_CONF1_VSMAX_SHIFT)
pub const CSR_CONF1_TMRBITS_SHIFT: u64 = 4;
pub const CSR_CONF1_TMRBITS_WIDTH: u64 = 8;
// #define  CSR_CONF1_TMRBITS		(_ULCAST_(0xff) << CSR_CONF1_TMRBITS_SHIFT)
pub const CSR_CONF1_KSNUM_WIDTH: u64 = 4;
pub const CSR_CONF1_KSNUM: u64 = (0xf);

pub const LOONGARCH_CSR_PRCFG2: u64 = 0x22	/* Config2 */;
pub const CSR_CONF2_PGMASK_SUPP: u64 = 0x3ffff000;

pub const LOONGARCH_CSR_PRCFG3: u64 = 0x23	/* Config3 */;
pub const CSR_CONF3_STLBIDX_SHIFT: u64 = 20;
pub const CSR_CONF3_STLBIDX_WIDTH: u64 = 6;
// #define  CSR_CONF3_STLBIDX		(_ULCAST_(0x3f) << CSR_CONF3_STLBIDX_SHIFT)
pub const CSR_CONF3_STLBWAYS_SHIFT: u64 = 12;
pub const CSR_CONF3_STLBWAYS_WIDTH: u64 = 8;
// #define  CSR_CONF3_STLBWAYS		(_ULCAST_(0xff) << CSR_CONF3_STLBWAYS_SHIFT)
pub const CSR_CONF3_MTLBSIZE_SHIFT: u64 = 4;
pub const CSR_CONF3_MTLBSIZE_WIDTH: u64 = 8;
// #define  CSR_CONF3_MTLBSIZE		(_ULCAST_(0xff) << CSR_CONF3_MTLBSIZE_SHIFT)
pub const CSR_CONF3_TLBTYPE_SHIFT: u64 = 0;
pub const CSR_CONF3_TLBTYPE_WIDTH: u64 = 4;
// #define  CSR_CONF3_TLBTYPE		(_ULCAST_(0xf) << CSR_CONF3_TLBTYPE_SHIFT)

/* KSave registers */
pub const LOONGARCH_CSR_KS0: u64 = 0x30;
pub const LOONGARCH_CSR_KS1: u64 = 0x31;
pub const LOONGARCH_CSR_KS2: u64 = 0x32;
pub const LOONGARCH_CSR_KS3: u64 = 0x33;
pub const LOONGARCH_CSR_KS4: u64 = 0x34;
pub const LOONGARCH_CSR_KS5: u64 = 0x35;
pub const LOONGARCH_CSR_KS6: u64 = 0x36;
pub const LOONGARCH_CSR_KS7: u64 = 0x37;
pub const LOONGARCH_CSR_KS8: u64 = 0x38;
pub const LOONGARCH_CSR_KS9: u64 = 0x39;
pub const LOONGARCH_CSR_KS10: u64 = 0x3a;
pub const LOONGARCH_CSR_KS11: u64 = 0x3b;
pub const LOONGARCH_CSR_KS12: u64 = 0x3c;
pub const LOONGARCH_CSR_KS13: u64 = 0x3d;
pub const LOONGARCH_CSR_KS14: u64 = 0x3e;
pub const LOONGARCH_CSR_KS15: u64 = 0x3f;

/* Exception allocated KS0, KS1 and KS2 statically */
pub const EXCEPTION_KS0: u64 = LOONGARCH_CSR_KS0;
pub const EXCEPTION_KS1: u64 = LOONGARCH_CSR_KS1;
pub const EXCEPTION_KS2: u64 = LOONGARCH_CSR_KS2;
// #define EXC_KSAVE_MASK			(1 << 0 | 1 << 1 | 1 << 2)

/* Percpu-data base allocated KS3 statically */
pub const PERCPU_BASE_KS: u64 = LOONGARCH_CSR_KS3;
// #define PERCPU_KSAVE_MASK		(1 << 3)

/* KVM allocated KS4 and KS5 statically */
pub const KVM_VCPU_KS: u64 = LOONGARCH_CSR_KS4;
pub const KVM_TEMP_KS: u64 = LOONGARCH_CSR_KS5;
// #define KVM_KSAVE_MASK			(1 << 4 | 1 << 5)

/* Timer registers */
pub const LOONGARCH_CSR_TMID: u64 = 0x40	/* Timer ID */;

pub const LOONGARCH_CSR_TCFG: u64 = 0x41	/* Timer config */;
pub const CSR_TCFG_VAL_SHIFT: u64 = 2;
// #define  CSR_TCFG_VAL			(_ULCAST_(0x3fffffffffff) << CSR_TCFG_VAL_SHIFT)
pub const CSR_TCFG_PERIOD_SHIFT: u64 = 1;
// #define  CSR_TCFG_PERIOD		(_ULCAST_(0x1) << CSR_TCFG_PERIOD_SHIFT)
// #define  CSR_TCFG_EN			(_ULCAST_(0x1))

pub const LOONGARCH_CSR_TVAL: u64 = 0x42	/* Timer value */;

pub const LOONGARCH_CSR_CNTC: u64 = 0x43	/* Timer offset */;

pub const LOONGARCH_CSR_TINTCLR: u64 = 0x44	/* Timer interrupt clear */;
pub const CSR_TINTCLR_TI_SHIFT: u64 = 0;
// #define  CSR_TINTCLR_TI			(1 << CSR_TINTCLR_TI_SHIFT)

/* Guest registers */
pub const LOONGARCH_CSR_GSTAT: u64 = 0x50	/* Guest status */;
pub const CSR_GSTAT_GID_SHIFT: u64 = 16;
pub const CSR_GSTAT_GID_WIDTH: u64 = 8;
// #define  CSR_GSTAT_GID_SHIFT_END	(CSR_GSTAT_GID_SHIFT + CSR_GSTAT_GID_WIDTH - 1)
// #define  CSR_GSTAT_GID			(_ULCAST_(0xff) << CSR_GSTAT_GID_SHIFT)
pub const CSR_GSTAT_GIDBIT_SHIFT: u64 = 4;
pub const CSR_GSTAT_GIDBIT_WIDTH: u64 = 6;
// #define  CSR_GSTAT_GIDBIT		(_ULCAST_(0x3f) << CSR_GSTAT_GIDBIT_SHIFT)
pub const CSR_GSTAT_PVM_SHIFT: u64 = 1;
// #define  CSR_GSTAT_PVM			(_ULCAST_(0x1) << CSR_GSTAT_PVM_SHIFT)
pub const CSR_GSTAT_VM_SHIFT: u64 = 0;
// #define  CSR_GSTAT_VM			(_ULCAST_(0x1) << CSR_GSTAT_VM_SHIFT)

pub const LOONGARCH_CSR_GCFG: u64 = 0x51	/* Guest config */;
pub const CSR_GCFG_GPERF_SHIFT: u64 = 24;
pub const CSR_GCFG_GPERF_WIDTH: u64 = 3;
// #define  CSR_GCFG_GPERF			(_ULCAST_(0x7) << CSR_GCFG_GPERF_SHIFT)
pub const CSR_GCFG_GPMP_SHIFT: u64 = 23;
// #define  CSR_GCFG_GPMP			(_ULCAST_(0x1) << CSR_GCFG_GPMP_SHIFT)
pub const CSR_GCFG_GCI_SHIFT: u64 = 20;
pub const CSR_GCFG_GCI_WIDTH: u64 = 2;
// #define  CSR_GCFG_GCI			(_ULCAST_(0x3) << CSR_GCFG_GCI_SHIFT)
// #define  CSR_GCFG_GCI_ALL		(_ULCAST_(0x0) << CSR_GCFG_GCI_SHIFT)
// #define  CSR_GCFG_GCI_HIT		(_ULCAST_(0x1) << CSR_GCFG_GCI_SHIFT)
// #define  CSR_GCFG_GCI_SECURE		(_ULCAST_(0x2) << CSR_GCFG_GCI_SHIFT)
pub const CSR_GCFG_GCIP_SHIFT: u64 = 16;
// #define  CSR_GCFG_GCIP			(_ULCAST_(0xf) << CSR_GCFG_GCIP_SHIFT)
// #define  CSR_GCFG_GCIP_ALL		(_ULCAST_(0x1) << CSR_GCFG_GCIP_SHIFT)
// #define  CSR_GCFG_GCIP_HIT		(_ULCAST_(0x1) << (CSR_GCFG_GCIP_SHIFT + 1))
// #define  CSR_GCFG_GCIP_SECURE		(_ULCAST_(0x1) << (CSR_GCFG_GCIP_SHIFT + 2))
pub const CSR_GCFG_TORU_SHIFT: u64 = 15;
// #define  CSR_GCFG_TORU			(_ULCAST_(0x1) << CSR_GCFG_TORU_SHIFT)
pub const CSR_GCFG_TORUP_SHIFT: u64 = 14;
// #define  CSR_GCFG_TORUP			(_ULCAST_(0x1) << CSR_GCFG_TORUP_SHIFT)
pub const CSR_GCFG_TOP_SHIFT: u64 = 13;
// #define  CSR_GCFG_TOP			(_ULCAST_(0x1) << CSR_GCFG_TOP_SHIFT)
pub const CSR_GCFG_TOPP_SHIFT: u64 = 12;
// #define  CSR_GCFG_TOPP			(_ULCAST_(0x1) << CSR_GCFG_TOPP_SHIFT)
pub const CSR_GCFG_TOE_SHIFT: u64 = 11;
// #define  CSR_GCFG_TOE			(_ULCAST_(0x1) << CSR_GCFG_TOE_SHIFT)
pub const CSR_GCFG_TOEP_SHIFT: u64 = 10;
// #define  CSR_GCFG_TOEP			(_ULCAST_(0x1) << CSR_GCFG_TOEP_SHIFT)
pub const CSR_GCFG_TIT_SHIFT: u64 = 9;
// #define  CSR_GCFG_TIT			(_ULCAST_(0x1) << CSR_GCFG_TIT_SHIFT)
pub const CSR_GCFG_TITP_SHIFT: u64 = 8;
// #define  CSR_GCFG_TITP			(_ULCAST_(0x1) << CSR_GCFG_TITP_SHIFT)
pub const CSR_GCFG_SIT_SHIFT: u64 = 7;
// #define  CSR_GCFG_SIT			(_ULCAST_(0x1) << CSR_GCFG_SIT_SHIFT)
pub const CSR_GCFG_SITP_SHIFT: u64 = 6;
// #define  CSR_GCFG_SITP			(_ULCAST_(0x1) << CSR_GCFG_SITP_SHIFT)
pub const CSR_GCFG_MATC_SHITF: u64 = 4;
pub const CSR_GCFG_MATC_WIDTH: u64 = 2;
// #define  CSR_GCFG_MATC_MASK		(_ULCAST_(0x3) << CSR_GCFG_MATC_SHITF)
// #define  CSR_GCFG_MATC_GUEST		(_ULCAST_(0x0) << CSR_GCFG_MATC_SHITF)
// #define  CSR_GCFG_MATC_ROOT		(_ULCAST_(0x1) << CSR_GCFG_MATC_SHITF)
// #define  CSR_GCFG_MATC_NEST		(_ULCAST_(0x2) << CSR_GCFG_MATC_SHITF)
pub const CSR_GCFG_MATP_NEST_SHIFT: u64 = 2;
// #define  CSR_GCFG_MATP_NEST		(_ULCAST_(0x1) << CSR_GCFG_MATP_NEST_SHIFT)
pub const CSR_GCFG_MATP_ROOT_SHIFT: u64 = 1;
// #define  CSR_GCFG_MATP_ROOT		(_ULCAST_(0x1) << CSR_GCFG_MATP_ROOT_SHIFT)
pub const CSR_GCFG_MATP_GUEST_SHIFT: u64 = 0;
// #define  CSR_GCFG_MATP_GUEST		(_ULCAST_(0x1) << CSR_GCFG_MATP_GUEST_SHIFT)

pub const LOONGARCH_CSR_GINTC: u64 = 0x52	/* Guest interrupt control */;
pub const CSR_GINTC_HC_SHIFT: u64 = 16;
pub const CSR_GINTC_HC_WIDTH: u64 = 8;
// #define  CSR_GINTC_HC			(_ULCAST_(0xff) << CSR_GINTC_HC_SHIFT)
pub const CSR_GINTC_PIP_SHIFT: u64 = 8;
pub const CSR_GINTC_PIP_WIDTH: u64 = 8;
// #define  CSR_GINTC_PIP			(_ULCAST_(0xff) << CSR_GINTC_PIP_SHIFT)
pub const CSR_GINTC_VIP_SHIFT: u64 = 0;
pub const CSR_GINTC_VIP_WIDTH: u64 = 8;
// #define  CSR_GINTC_VIP			(_ULCAST_(0xff))

pub const LOONGARCH_CSR_GCNTC: u64 = 0x53	/* Guest timer offset */;

/* LLBCTL register */
pub const LOONGARCH_CSR_LLBCTL: u64 = 0x60	/* LLBit control */;
pub const CSR_LLBCTL_ROLLB_SHIFT: u64 = 0;
// #define  CSR_LLBCTL_ROLLB		(_ULCAST_(1) << CSR_LLBCTL_ROLLB_SHIFT)
pub const CSR_LLBCTL_WCLLB_SHIFT: u64 = 1;
// #define  CSR_LLBCTL_WCLLB		(_ULCAST_(1) << CSR_LLBCTL_WCLLB_SHIFT)
pub const CSR_LLBCTL_KLO_SHIFT: u64 = 2;
// #define  CSR_LLBCTL_KLO			(_ULCAST_(1) << CSR_LLBCTL_KLO_SHIFT)

/* Implement dependent */
pub const LOONGARCH_CSR_IMPCTL1: u64 = 0x80	/* Loongson config1 */;
pub const CSR_LDSTORDER_SHIFT: u64 = 28;
pub const CSR_LDSTORDER_WIDTH: u64 = 3;
// #define  CSR_LDSTORDER_MASK		(_ULCAST_(0x7) << CSR_LDSTORDER_SHIFT)
// #define  CSR_LDSTORDER_NLD_NST		(_ULCAST_(0x0) << CSR_LDSTORDER_SHIFT) /* 000 = No Load No Store */
// #define  CSR_LDSTORDER_ALD_NST		(_ULCAST_(0x1) << CSR_LDSTORDER_SHIFT) /* 001 = All Load No Store */
// #define  CSR_LDSTORDER_SLD_NST		(_ULCAST_(0x3) << CSR_LDSTORDER_SHIFT) /* 011 = Same Load No Store */
// #define  CSR_LDSTORDER_NLD_AST		(_ULCAST_(0x4) << CSR_LDSTORDER_SHIFT) /* 100 = No Load All Store */
// #define  CSR_LDSTORDER_ALD_AST		(_ULCAST_(0x5) << CSR_LDSTORDER_SHIFT) /* 101 = All Load All Store */
// #define  CSR_LDSTORDER_SLD_AST		(_ULCAST_(0x7) << CSR_LDSTORDER_SHIFT) /* 111 = Same Load All Store */
pub const CSR_MISPEC_SHIFT: u64 = 20;
pub const CSR_MISPEC_WIDTH: u64 = 8;
// #define  CSR_MISPEC			(_ULCAST_(0xff) << CSR_MISPEC_SHIFT)
pub const CSR_SSEN_SHIFT: u64 = 18;
// #define  CSR_SSEN			(_ULCAST_(1) << CSR_SSEN_SHIFT)
pub const CSR_SCRAND_SHIFT: u64 = 17;
// #define  CSR_SCRAND			(_ULCAST_(1) << CSR_SCRAND_SHIFT)
pub const CSR_LLEXCL_SHIFT: u64 = 16;
// #define  CSR_LLEXCL			(_ULCAST_(1) << CSR_LLEXCL_SHIFT)
pub const CSR_DISVC_SHIFT: u64 = 15;
// #define  CSR_DISVC			(_ULCAST_(1) << CSR_DISVC_SHIFT)
pub const CSR_VCLRU_SHIFT: u64 = 14;
// #define  CSR_VCLRU			(_ULCAST_(1) << CSR_VCLRU_SHIFT)
pub const CSR_DCLRU_SHIFT: u64 = 13;
// #define  CSR_DCLRU			(_ULCAST_(1) << CSR_DCLRU_SHIFT)
pub const CSR_FASTLDQ_SHIFT: u64 = 12;
// #define  CSR_FASTLDQ			(_ULCAST_(1) << CSR_FASTLDQ_SHIFT)
pub const CSR_USERCAC_SHIFT: u64 = 11;
// #define  CSR_USERCAC			(_ULCAST_(1) << CSR_USERCAC_SHIFT)
pub const CSR_ANTI_MISPEC_SHIFT: u64 = 10;
// #define  CSR_ANTI_MISPEC		(_ULCAST_(1) << CSR_ANTI_MISPEC_SHIFT)
pub const CSR_AUTO_FLUSHSFB_SHIFT: u64 = 9;
// #define  CSR_AUTO_FLUSHSFB		(_ULCAST_(1) << CSR_AUTO_FLUSHSFB_SHIFT)
pub const CSR_STFILL_SHIFT: u64 = 8;
// #define  CSR_STFILL			(_ULCAST_(1) << CSR_STFILL_SHIFT)
pub const CSR_LIFEP_SHIFT: u64 = 7;
// #define  CSR_LIFEP			(_ULCAST_(1) << CSR_LIFEP_SHIFT)
pub const CSR_LLSYNC_SHIFT: u64 = 6;
// #define  CSR_LLSYNC			(_ULCAST_(1) << CSR_LLSYNC_SHIFT)
pub const CSR_BRBTDIS_SHIFT: u64 = 5;
// #define  CSR_BRBTDIS			(_ULCAST_(1) << CSR_BRBTDIS_SHIFT)
pub const CSR_RASDIS_SHIFT: u64 = 4;
// #define  CSR_RASDIS			(_ULCAST_(1) << CSR_RASDIS_SHIFT)
pub const CSR_STPRE_SHIFT: u64 = 2;
pub const CSR_STPRE_WIDTH: u64 = 2;
// #define  CSR_STPRE			(_ULCAST_(3) << CSR_STPRE_SHIFT)
pub const CSR_INSTPRE_SHIFT: u64 = 1;
// #define  CSR_INSTPRE			(_ULCAST_(1) << CSR_INSTPRE_SHIFT)
pub const CSR_DATAPRE_SHIFT: u64 = 0;
// #define  CSR_DATAPRE			(_ULCAST_(1) << CSR_DATAPRE_SHIFT)

pub const LOONGARCH_CSR_IMPCTL2: u64 = 0x81	/* Loongson config2 */;
pub const CSR_FLUSH_MTLB_SHIFT: u64 = 0;
// #define  CSR_FLUSH_MTLB			(_ULCAST_(1) << CSR_FLUSH_MTLB_SHIFT)
pub const CSR_FLUSH_STLB_SHIFT: u64 = 1;
// #define  CSR_FLUSH_STLB			(_ULCAST_(1) << CSR_FLUSH_STLB_SHIFT)
pub const CSR_FLUSH_DTLB_SHIFT: u64 = 2;
// #define  CSR_FLUSH_DTLB			(_ULCAST_(1) << CSR_FLUSH_DTLB_SHIFT)
pub const CSR_FLUSH_ITLB_SHIFT: u64 = 3;
// #define  CSR_FLUSH_ITLB			(_ULCAST_(1) << CSR_FLUSH_ITLB_SHIFT)
pub const CSR_FLUSH_BTAC_SHIFT: u64 = 4;
// #define  CSR_FLUSH_BTAC			(_ULCAST_(1) << CSR_FLUSH_BTAC_SHIFT)

pub const LOONGARCH_CSR_GNMI: u64 = 0x82;

/* TLB Refill registers */
pub const LOONGARCH_CSR_TLBRENTRY: u64 = 0x88	/* TLB refill exception entry */;
pub const LOONGARCH_CSR_TLBRBADV: u64 = 0x89	/* TLB refill badvaddr */;
pub const LOONGARCH_CSR_TLBRERA: u64 = 0x8a	/* TLB refill ERA */;
pub const LOONGARCH_CSR_TLBRSAVE: u64 = 0x8b	/* KSave for TLB refill exception */;
pub const LOONGARCH_CSR_TLBRELO0: u64 = 0x8c	/* TLB refill entrylo0 */;
pub const LOONGARCH_CSR_TLBRELO1: u64 = 0x8d	/* TLB refill entrylo1 */;
pub const LOONGARCH_CSR_TLBREHI: u64 = 0x8e	/* TLB refill entryhi */;
pub const CSR_TLBREHI_PS_SHIFT: u64 = 0;
// #define  CSR_TLBREHI_PS			(_ULCAST_(0x3f) << CSR_TLBREHI_PS_SHIFT)
pub const LOONGARCH_CSR_TLBRPRMD: u64 = 0x8f	/* TLB refill mode info */;

/* Machine Error registers */
pub const LOONGARCH_CSR_MERRCTL: u64 = 0x90	/* MERRCTL */;
pub const LOONGARCH_CSR_MERRINFO1: u64 = 0x91	/* MError info1 */;
pub const LOONGARCH_CSR_MERRINFO2: u64 = 0x92	/* MError info2 */;
pub const LOONGARCH_CSR_MERRENTRY: u64 = 0x93	/* MError exception entry */;
pub const LOONGARCH_CSR_MERRERA: u64 = 0x94	/* MError exception ERA */;
pub const LOONGARCH_CSR_MERRSAVE: u64 = 0x95	/* KSave for machine error exception */;

pub const LOONGARCH_CSR_CTAG: u64 = 0x98	/* TagLo + TagHi */;

pub const LOONGARCH_CSR_ISR0: u64 = 0xa0;
pub const LOONGARCH_CSR_ISR1: u64 = 0xa1;
pub const LOONGARCH_CSR_ISR2: u64 = 0xa2;
pub const LOONGARCH_CSR_ISR3: u64 = 0xa3;

pub const LOONGARCH_CSR_IRR: u64 = 0xa4;
pub const LOONGARCH_CSR_IPR: u64 = 0xa5;

pub const LOONGARCH_CSR_PRID: u64 = 0xc0;

/* Shadow MCSR : 0xc0 ~ 0xff */
pub const LOONGARCH_CSR_MCSR0: u64 = 0xc0	/* CPUCFG0 and CPUCFG1 */;
pub const MCSR0_INT_IMPL_SHIFT: u64 = 58;
pub const MCSR0_INT_IMPL: u64 = 0;
pub const MCSR0_IOCSR_BRD_SHIFT: u64 = 57;
// #define  MCSR0_IOCSR_BRD		(_ULCAST_(1) << MCSR0_IOCSR_BRD_SHIFT)
pub const MCSR0_HUGEPG_SHIFT: u64 = 56;
// #define  MCSR0_HUGEPG			(_ULCAST_(1) << MCSR0_HUGEPG_SHIFT)
pub const MCSR0_RPLMTLB_SHIFT: u64 = 55;
// #define  MCSR0_RPLMTLB			(_ULCAST_(1) << MCSR0_RPLMTLB_SHIFT)
pub const MCSR0_EP_SHIFT: u64 = 54;
// #define  MCSR0_EP			(_ULCAST_(1) << MCSR0_EP_SHIFT)
pub const MCSR0_RI_SHIFT: u64 = 53;
// #define  MCSR0_RI			(_ULCAST_(1) << MCSR0_RI_SHIFT)
pub const MCSR0_UAL_SHIFT: u64 = 52;
// #define  MCSR0_UAL			(_ULCAST_(1) << MCSR0_UAL_SHIFT)
pub const MCSR0_VABIT_SHIFT: u64 = 44;
pub const MCSR0_VABIT_WIDTH: u64 = 8;
// #define  MCSR0_VABIT			(_ULCAST_(0xff) << MCSR0_VABIT_SHIFT)
pub const VABIT_DEFAULT: u64 = 0x2f;
pub const MCSR0_PABIT_SHIFT: u64 = 36;
pub const MCSR0_PABIT_WIDTH: u64 = 8;
// #define  MCSR0_PABIT			(_ULCAST_(0xff) << MCSR0_PABIT_SHIFT)
pub const PABIT_DEFAULT: u64 = 0x2f;
pub const MCSR0_IOCSR_SHIFT: u64 = 35;
// #define  MCSR0_IOCSR			(_ULCAST_(1) << MCSR0_IOCSR_SHIFT)
pub const MCSR0_PAGING_SHIFT: u64 = 34;
// #define  MCSR0_PAGING			(_ULCAST_(1) << MCSR0_PAGING_SHIFT)
pub const MCSR0_GR64_SHIFT: u64 = 33;
// #define  MCSR0_GR64			(_ULCAST_(1) << MCSR0_GR64_SHIFT)
pub const GR64_DEFAULT: u64 = 1;
pub const MCSR0_GR32_SHIFT: u64 = 32;
// #define  MCSR0_GR32			(_ULCAST_(1) << MCSR0_GR32_SHIFT)
pub const GR32_DEFAULT: u64 = 0;
pub const MCSR0_PRID_WIDTH: u64 = 32;
pub const MCSR0_PRID: u64 = 0x14C010;

pub const LOONGARCH_CSR_MCSR1: u64 = 0xc1	/* CPUCFG2 and CPUCFG3 */;
pub const MCSR1_HPFOLD_SHIFT: u64 = 43;
// #define  MCSR1_HPFOLD			(_ULCAST_(1) << MCSR1_HPFOLD_SHIFT)
pub const MCSR1_SPW_LVL_SHIFT: u64 = 40;
pub const MCSR1_SPW_LVL_WIDTH: u64 = 3;
// #define  MCSR1_SPW_LVL			(_ULCAST_(7) << MCSR1_SPW_LVL_SHIFT)
pub const MCSR1_ICACHET_SHIFT: u64 = 39;
// #define  MCSR1_ICACHET			(_ULCAST_(1) << MCSR1_ICACHET_SHIFT)
pub const MCSR1_ITLBT_SHIFT: u64 = 38;
// #define  MCSR1_ITLBT			(_ULCAST_(1) << MCSR1_ITLBT_SHIFT)
pub const MCSR1_LLDBAR_SHIFT: u64 = 37;
// #define  MCSR1_LLDBAR			(_ULCAST_(1) << MCSR1_LLDBAR_SHIFT)
pub const MCSR1_SCDLY_SHIFT: u64 = 36;
// #define  MCSR1_SCDLY			(_ULCAST_(1) << MCSR1_SCDLY_SHIFT)
pub const MCSR1_LLEXC_SHIFT: u64 = 35;
// #define  MCSR1_LLEXC			(_ULCAST_(1) << MCSR1_LLEXC_SHIFT)
pub const MCSR1_UCACC_SHIFT: u64 = 34;
// #define  MCSR1_UCACC			(_ULCAST_(1) << MCSR1_UCACC_SHIFT)
pub const MCSR1_SFB_SHIFT: u64 = 33;
// #define  MCSR1_SFB			(_ULCAST_(1) << MCSR1_SFB_SHIFT)
pub const MCSR1_CCDMA_SHIFT: u64 = 32;
// #define  MCSR1_CCDMA			(_ULCAST_(1) << MCSR1_CCDMA_SHIFT)
pub const MCSR1_LAMO_SHIFT: u64 = 22;
// #define  MCSR1_LAMO			(_ULCAST_(1) << MCSR1_LAMO_SHIFT)
pub const MCSR1_LSPW_SHIFT: u64 = 21;
// #define  MCSR1_LSPW			(_ULCAST_(1) << MCSR1_LSPW_SHIFT)
pub const MCSR1_MIPSBT_SHIFT: u64 = 20;
// #define  MCSR1_MIPSBT			(_ULCAST_(1) << MCSR1_MIPSBT_SHIFT)
pub const MCSR1_ARMBT_SHIFT: u64 = 19;
// #define  MCSR1_ARMBT			(_ULCAST_(1) << MCSR1_ARMBT_SHIFT)
pub const MCSR1_X86BT_SHIFT: u64 = 18;
// #define  MCSR1_X86BT			(_ULCAST_(1) << MCSR1_X86BT_SHIFT)
pub const MCSR1_LLFTPVERS_SHIFT: u64 = 15;
pub const MCSR1_LLFTPVERS_WIDTH: u64 = 3;
// #define  MCSR1_LLFTPVERS		(_ULCAST_(7) << MCSR1_LLFTPVERS_SHIFT)
pub const MCSR1_LLFTP_SHIFT: u64 = 14;
// #define  MCSR1_LLFTP			(_ULCAST_(1) << MCSR1_LLFTP_SHIFT)
pub const MCSR1_VZVERS_SHIFT: u64 = 11;
pub const MCSR1_VZVERS_WIDTH: u64 = 3;
// #define  MCSR1_VZVERS			(_ULCAST_(7) << MCSR1_VZVERS_SHIFT)
pub const MCSR1_VZ_SHIFT: u64 = 10;
// #define  MCSR1_VZ			(_ULCAST_(1) << MCSR1_VZ_SHIFT)
pub const MCSR1_CRYPTO_SHIFT: u64 = 9;
// #define  MCSR1_CRYPTO			(_ULCAST_(1) << MCSR1_CRYPTO_SHIFT)
pub const MCSR1_COMPLEX_SHIFT: u64 = 8;
// #define  MCSR1_COMPLEX			(_ULCAST_(1) << MCSR1_COMPLEX_SHIFT)
pub const MCSR1_LASX_SHIFT: u64 = 7;
// #define  MCSR1_LASX			(_ULCAST_(1) << MCSR1_LASX_SHIFT)
pub const MCSR1_LSX_SHIFT: u64 = 6;
// #define  MCSR1_LSX			(_ULCAST_(1) << MCSR1_LSX_SHIFT)
pub const MCSR1_FPVERS_SHIFT: u64 = 3;
pub const MCSR1_FPVERS_WIDTH: u64 = 3;
// #define  MCSR1_FPVERS			(_ULCAST_(7) << MCSR1_FPVERS_SHIFT)
pub const MCSR1_FPDP_SHIFT: u64 = 2;
// #define  MCSR1_FPDP			(_ULCAST_(1) << MCSR1_FPDP_SHIFT)
pub const MCSR1_FPSP_SHIFT: u64 = 1;
// #define  MCSR1_FPSP			(_ULCAST_(1) << MCSR1_FPSP_SHIFT)
pub const MCSR1_FP_SHIFT: u64 = 0;
// #define  MCSR1_FP			(_ULCAST_(1) << MCSR1_FP_SHIFT)

pub const LOONGARCH_CSR_MCSR2: u64 = 0xc2	/* CPUCFG4 and CPUCFG5 */;
pub const MCSR2_CCDIV_SHIFT: u64 = 48;
pub const MCSR2_CCDIV_WIDTH: u64 = 16;
// #define  MCSR2_CCDIV			(_ULCAST_(0xffff) << MCSR2_CCDIV_SHIFT)
pub const MCSR2_CCMUL_SHIFT: u64 = 32;
pub const MCSR2_CCMUL_WIDTH: u64 = 16;
// #define  MCSR2_CCMUL			(_ULCAST_(0xffff) << MCSR2_CCMUL_SHIFT)
pub const MCSR2_CCFREQ_WIDTH: u64 = 32;
// #define  MCSR2_CCFREQ			(_ULCAST_(0xffffffff))
pub const CCFREQ_DEFAULT: u64 = 0x5f5e100	/* 100MHz */;

pub const LOONGARCH_CSR_MCSR3: u64 = 0xc3	/* CPUCFG6 */;
pub const MCSR3_UPM_SHIFT: u64 = 14;
// #define  MCSR3_UPM			(_ULCAST_(1) << MCSR3_UPM_SHIFT)
pub const MCSR3_PMBITS_SHIFT: u64 = 8;
pub const MCSR3_PMBITS_WIDTH: u64 = 6;
// #define  MCSR3_PMBITS			(_ULCAST_(0x3f) << MCSR3_PMBITS_SHIFT)
pub const PMBITS_DEFAULT: u64 = 0x40;
pub const MCSR3_PMNUM_SHIFT: u64 = 4;
pub const MCSR3_PMNUM_WIDTH: u64 = 4;
// #define  MCSR3_PMNUM			(_ULCAST_(0xf) << MCSR3_PMNUM_SHIFT)
pub const MCSR3_PAMVER_SHIFT: u64 = 1;
pub const MCSR3_PAMVER_WIDTH: u64 = 3;
// #define  MCSR3_PAMVER			(_ULCAST_(0x7) << MCSR3_PAMVER_SHIFT)
pub const MCSR3_PMP_SHIFT: u64 = 0;
// #define  MCSR3_PMP			(_ULCAST_(1) << MCSR3_PMP_SHIFT)

pub const LOONGARCH_CSR_MCSR8: u64 = 0xc8	/* CPUCFG16 and CPUCFG17 */;
pub const MCSR8_L1I_SIZE_SHIFT: u64 = 56;
pub const MCSR8_L1I_SIZE_WIDTH: u64 = 7;
// #define  MCSR8_L1I_SIZE			(_ULCAST_(0x7f) << MCSR8_L1I_SIZE_SHIFT)
pub const MCSR8_L1I_IDX_SHIFT: u64 = 48;
pub const MCSR8_L1I_IDX_WIDTH: u64 = 8;
// #define  MCSR8_L1I_IDX			(_ULCAST_(0xff) << MCSR8_L1I_IDX_SHIFT)
pub const MCSR8_L1I_WAY_SHIFT: u64 = 32;
pub const MCSR8_L1I_WAY_WIDTH: u64 = 16;
// #define  MCSR8_L1I_WAY			(_ULCAST_(0xffff) << MCSR8_L1I_WAY_SHIFT)
pub const MCSR8_L3DINCL_SHIFT: u64 = 16;
// #define  MCSR8_L3DINCL			(_ULCAST_(1) << MCSR8_L3DINCL_SHIFT)
pub const MCSR8_L3DPRIV_SHIFT: u64 = 15;
// #define  MCSR8_L3DPRIV			(_ULCAST_(1) << MCSR8_L3DPRIV_SHIFT)
pub const MCSR8_L3DPRE_SHIFT: u64 = 14;
// #define  MCSR8_L3DPRE			(_ULCAST_(1) << MCSR8_L3DPRE_SHIFT)
pub const MCSR8_L3IUINCL_SHIFT: u64 = 13;
// #define  MCSR8_L3IUINCL			(_ULCAST_(1) << MCSR8_L3IUINCL_SHIFT)
pub const MCSR8_L3IUPRIV_SHIFT: u64 = 12;
// #define  MCSR8_L3IUPRIV			(_ULCAST_(1) << MCSR8_L3IUPRIV_SHIFT)
pub const MCSR8_L3IUUNIFY_SHIFT: u64 = 11;
// #define  MCSR8_L3IUUNIFY		(_ULCAST_(1) << MCSR8_L3IUUNIFY_SHIFT)
pub const MCSR8_L3IUPRE_SHIFT: u64 = 10;
// #define  MCSR8_L3IUPRE			(_ULCAST_(1) << MCSR8_L3IUPRE_SHIFT)
pub const MCSR8_L2DINCL_SHIFT: u64 = 9;
// #define  MCSR8_L2DINCL			(_ULCAST_(1) << MCSR8_L2DINCL_SHIFT)
pub const MCSR8_L2DPRIV_SHIFT: u64 = 8;
// #define  MCSR8_L2DPRIV			(_ULCAST_(1) << MCSR8_L2DPRIV_SHIFT)
pub const MCSR8_L2DPRE_SHIFT: u64 = 7;
// #define  MCSR8_L2DPRE			(_ULCAST_(1) << MCSR8_L2DPRE_SHIFT)
pub const MCSR8_L2IUINCL_SHIFT: u64 = 6;
// #define  MCSR8_L2IUINCL			(_ULCAST_(1) << MCSR8_L2IUINCL_SHIFT)
pub const MCSR8_L2IUPRIV_SHIFT: u64 = 5;
// #define  MCSR8_L2IUPRIV			(_ULCAST_(1) << MCSR8_L2IUPRIV_SHIFT)
pub const MCSR8_L2IUUNIFY_SHIFT: u64 = 4;
// #define  MCSR8_L2IUUNIFY		(_ULCAST_(1) << MCSR8_L2IUUNIFY_SHIFT)
pub const MCSR8_L2IUPRE_SHIFT: u64 = 3;
// #define  MCSR8_L2IUPRE			(_ULCAST_(1) << MCSR8_L2IUPRE_SHIFT)
pub const MCSR8_L1DPRE_SHIFT: u64 = 2;
// #define  MCSR8_L1DPRE			(_ULCAST_(1) << MCSR8_L1DPRE_SHIFT)
pub const MCSR8_L1IUUNIFY_SHIFT: u64 = 1;
// #define  MCSR8_L1IUUNIFY		(_ULCAST_(1) << MCSR8_L1IUUNIFY_SHIFT)
pub const MCSR8_L1IUPRE_SHIFT: u64 = 0;
// #define  MCSR8_L1IUPRE			(_ULCAST_(1) << MCSR8_L1IUPRE_SHIFT)

pub const LOONGARCH_CSR_MCSR9: u64 = 0xc9	/* CPUCFG18 and CPUCFG19 */;
pub const MCSR9_L2U_SIZE_SHIFT: u64 = 56;
pub const MCSR9_L2U_SIZE_WIDTH: u64 = 7;
// #define  MCSR9_L2U_SIZE			(_ULCAST_(0x7f) << MCSR9_L2U_SIZE_SHIFT)
pub const MCSR9_L2U_IDX_SHIFT: u64 = 48;
pub const MCSR9_L2U_IDX_WIDTH: u64 = 8;
// #define  MCSR9_L2U_IDX			(_ULCAST_(0xff) << MCSR9_IDX_LOG_SHIFT)
pub const MCSR9_L2U_WAY_SHIFT: u64 = 32;
pub const MCSR9_L2U_WAY_WIDTH: u64 = 16;
// #define  MCSR9_L2U_WAY			(_ULCAST_(0xffff) << MCSR9_L2U_WAY_SHIFT)
pub const MCSR9_L1D_SIZE_SHIFT: u64 = 24;
pub const MCSR9_L1D_SIZE_WIDTH: u64 = 7;
// #define  MCSR9_L1D_SIZE			(_ULCAST_(0x7f) << MCSR9_L1D_SIZE_SHIFT)
pub const MCSR9_L1D_IDX_SHIFT: u64 = 16;
pub const MCSR9_L1D_IDX_WIDTH: u64 = 8;
// #define  MCSR9_L1D_IDX			(_ULCAST_(0xff) << MCSR9_L1D_IDX_SHIFT)
pub const MCSR9_L1D_WAY_SHIFT: u64 = 0;
pub const MCSR9_L1D_WAY_WIDTH: u64 = 16;
// #define  MCSR9_L1D_WAY			(_ULCAST_(0xffff) << MCSR9_L1D_WAY_SHIFT)

pub const LOONGARCH_CSR_MCSR10: u64 = 0xca	/* CPUCFG20 */;
pub const MCSR10_L3U_SIZE_SHIFT: u64 = 24;
pub const MCSR10_L3U_SIZE_WIDTH: u64 = 7;
// #define  MCSR10_L3U_SIZE		(_ULCAST_(0x7f) << MCSR10_L3U_SIZE_SHIFT)
pub const MCSR10_L3U_IDX_SHIFT: u64 = 16;
pub const MCSR10_L3U_IDX_WIDTH: u64 = 8;
// #define  MCSR10_L3U_IDX			(_ULCAST_(0xff) << MCSR10_L3U_IDX_SHIFT)
pub const MCSR10_L3U_WAY_SHIFT: u64 = 0;
pub const MCSR10_L3U_WAY_WIDTH: u64 = 16;
// #define  MCSR10_L3U_WAY			(_ULCAST_(0xffff) << MCSR10_L3U_WAY_SHIFT)

pub const LOONGARCH_CSR_MCSR24: u64 = 0xf0	/* cpucfg48 */;
pub const MCSR24_RAMCG_SHIFT: u64 = 3;
// #define  MCSR24_RAMCG			(_ULCAST_(1) << MCSR24_RAMCG_SHIFT)
pub const MCSR24_VFPUCG_SHIFT: u64 = 2;
// #define  MCSR24_VFPUCG			(_ULCAST_(1) << MCSR24_VFPUCG_SHIFT)
pub const MCSR24_NAPEN_SHIFT: u64 = 1;
// #define  MCSR24_NAPEN			(_ULCAST_(1) << MCSR24_NAPEN_SHIFT)
pub const MCSR24_MCSRLOCK_SHIFT: u64 = 0;
// #define  MCSR24_MCSRLOCK		(_ULCAST_(1) << MCSR24_MCSRLOCK_SHIFT)

/* Uncached accelerate windows registers */
pub const LOONGARCH_CSR_UCAWIN: u64 = 0x100;
pub const LOONGARCH_CSR_UCAWIN0_LO: u64 = 0x102;
pub const LOONGARCH_CSR_UCAWIN0_HI: u64 = 0x103;
pub const LOONGARCH_CSR_UCAWIN1_LO: u64 = 0x104;
pub const LOONGARCH_CSR_UCAWIN1_HI: u64 = 0x105;
pub const LOONGARCH_CSR_UCAWIN2_LO: u64 = 0x106;
pub const LOONGARCH_CSR_UCAWIN2_HI: u64 = 0x107;
pub const LOONGARCH_CSR_UCAWIN3_LO: u64 = 0x108;
pub const LOONGARCH_CSR_UCAWIN3_HI: u64 = 0x109;

/* Direct Map windows registers */
pub const LOONGARCH_CSR_DMWIN0: u64 = 0x180	/* 64 direct map win0: MEM & IF */;
pub const LOONGARCH_CSR_DMWIN1: u64 = 0x181	/* 64 direct map win1: MEM & IF */;
pub const LOONGARCH_CSR_DMWIN2: u64 = 0x182	/* 64 direct map win2: MEM */;
pub const LOONGARCH_CSR_DMWIN3: u64 = 0x183	/* 64 direct map win3: MEM */;

/* Direct Map window 0/1/2/3 */

// #ifdef CONFIG_32BIT

// #define CSR_DMW0_PLV0		(1 << 0)
// #define CSR_DMW0_VSEG		(0x4)
// #define CSR_DMW0_BASE		(CSR_DMW0_VSEG << DMW_PABITS)
// #define CSR_DMW0_INIT		(CSR_DMW0_BASE | CSR_DMW0_PLV0)

// #define CSR_DMW1_PLV0		(1 << 0)
// #define CSR_DMW1_MAT		(1 << 4)
// #define CSR_DMW1_VSEG		(0x5)
// #define CSR_DMW1_BASE		(CSR_DMW1_VSEG << DMW_PABITS)
// #define CSR_DMW1_INIT		(CSR_DMW1_BASE | CSR_DMW1_MAT | CSR_DMW1_PLV0)

pub const CSR_DMW2_INIT: u64 = 0x0;

pub const CSR_DMW3_INIT: u64 = 0x0;

// #else

pub const CSR_DMW0_PLV0: u64 = (1 << 0);
pub const CSR_DMW0_VSEG: u64 = (0x8000);
// #define CSR_DMW0_BASE		(CSR_DMW0_VSEG << DMW_PABITS)
// #define CSR_DMW0_INIT		(CSR_DMW0_BASE | CSR_DMW0_PLV0)

pub const CSR_DMW1_PLV0: u64 = (1 << 0);
pub const CSR_DMW1_MAT: u64 = (1 << 4);
pub const CSR_DMW1_VSEG: u64 = (0x9000);
// #define CSR_DMW1_BASE		(CSR_DMW1_VSEG << DMW_PABITS)
// #define CSR_DMW1_INIT		(CSR_DMW1_BASE | CSR_DMW1_MAT | CSR_DMW1_PLV0)

pub const CSR_DMW2_PLV0: u64 = (1 << 0);
pub const CSR_DMW2_MAT: u64 = (2 << 4);
pub const CSR_DMW2_VSEG: u64 = (0xa000);
// #define CSR_DMW2_BASE		(CSR_DMW2_VSEG << DMW_PABITS)
// #define CSR_DMW2_INIT		(CSR_DMW2_BASE | CSR_DMW2_MAT | CSR_DMW2_PLV0)

pub const CSR_DMW3_INIT: u64 = 0x0;

// #endif

/* Performance Counter registers */
pub const LOONGARCH_CSR_PERFCTRL0: u64 = 0x200	/* 32 perf event 0 config */;
pub const LOONGARCH_CSR_PERFCNTR0: u64 = 0x201	/* 64 perf event 0 count value */;
pub const LOONGARCH_CSR_PERFCTRL1: u64 = 0x202	/* 32 perf event 1 config */;
pub const LOONGARCH_CSR_PERFCNTR1: u64 = 0x203	/* 64 perf event 1 count value */;
pub const LOONGARCH_CSR_PERFCTRL2: u64 = 0x204	/* 32 perf event 2 config */;
pub const LOONGARCH_CSR_PERFCNTR2: u64 = 0x205	/* 64 perf event 2 count value */;
pub const LOONGARCH_CSR_PERFCTRL3: u64 = 0x206	/* 32 perf event 3 config */;
pub const LOONGARCH_CSR_PERFCNTR3: u64 = 0x207	/* 64 perf event 3 count value */;
// #define  CSR_PERFCTRL_PLV0		(_ULCAST_(1) << 16)
// #define  CSR_PERFCTRL_PLV1		(_ULCAST_(1) << 17)
// #define  CSR_PERFCTRL_PLV2		(_ULCAST_(1) << 18)
// #define  CSR_PERFCTRL_PLV3		(_ULCAST_(1) << 19)
// #define  CSR_PERFCTRL_IE		(_ULCAST_(1) << 20)
pub const CSR_PERFCTRL_EVENT: u64 = 0x3ff;

/* Debug registers */
pub const LOONGARCH_CSR_MWPC: u64 = 0x300	/* data breakpoint config */;
pub const LOONGARCH_CSR_MWPS: u64 = 0x301	/* data breakpoint status */;

pub const LOONGARCH_CSR_DB0ADDR: u64 = 0x310	/* data breakpoint 0 address */;
pub const LOONGARCH_CSR_DB0MASK: u64 = 0x311	/* data breakpoint 0 mask */;
pub const LOONGARCH_CSR_DB0CTRL: u64 = 0x312	/* data breakpoint 0 control */;
pub const LOONGARCH_CSR_DB0ASID: u64 = 0x313	/* data breakpoint 0 asid */;

pub const LOONGARCH_CSR_DB1ADDR: u64 = 0x318	/* data breakpoint 1 address */;
pub const LOONGARCH_CSR_DB1MASK: u64 = 0x319	/* data breakpoint 1 mask */;
pub const LOONGARCH_CSR_DB1CTRL: u64 = 0x31a	/* data breakpoint 1 control */;
pub const LOONGARCH_CSR_DB1ASID: u64 = 0x31b	/* data breakpoint 1 asid */;

pub const LOONGARCH_CSR_DB2ADDR: u64 = 0x320	/* data breakpoint 2 address */;
pub const LOONGARCH_CSR_DB2MASK: u64 = 0x321	/* data breakpoint 2 mask */;
pub const LOONGARCH_CSR_DB2CTRL: u64 = 0x322	/* data breakpoint 2 control */;
pub const LOONGARCH_CSR_DB2ASID: u64 = 0x323	/* data breakpoint 2 asid */;

pub const LOONGARCH_CSR_DB3ADDR: u64 = 0x328	/* data breakpoint 3 address */;
pub const LOONGARCH_CSR_DB3MASK: u64 = 0x329	/* data breakpoint 3 mask */;
pub const LOONGARCH_CSR_DB3CTRL: u64 = 0x32a	/* data breakpoint 3 control */;
pub const LOONGARCH_CSR_DB3ASID: u64 = 0x32b	/* data breakpoint 3 asid */;

pub const LOONGARCH_CSR_DB4ADDR: u64 = 0x330	/* data breakpoint 4 address */;
pub const LOONGARCH_CSR_DB4MASK: u64 = 0x331	/* data breakpoint 4 maks */;
pub const LOONGARCH_CSR_DB4CTRL: u64 = 0x332	/* data breakpoint 4 control */;
pub const LOONGARCH_CSR_DB4ASID: u64 = 0x333	/* data breakpoint 4 asid */;

pub const LOONGARCH_CSR_DB5ADDR: u64 = 0x338	/* data breakpoint 5 address */;
pub const LOONGARCH_CSR_DB5MASK: u64 = 0x339	/* data breakpoint 5 mask */;
pub const LOONGARCH_CSR_DB5CTRL: u64 = 0x33a	/* data breakpoint 5 control */;
pub const LOONGARCH_CSR_DB5ASID: u64 = 0x33b	/* data breakpoint 5 asid */;

pub const LOONGARCH_CSR_DB6ADDR: u64 = 0x340	/* data breakpoint 6 address */;
pub const LOONGARCH_CSR_DB6MASK: u64 = 0x341	/* data breakpoint 6 mask */;
pub const LOONGARCH_CSR_DB6CTRL: u64 = 0x342	/* data breakpoint 6 control */;
pub const LOONGARCH_CSR_DB6ASID: u64 = 0x343	/* data breakpoint 6 asid */;

pub const LOONGARCH_CSR_DB7ADDR: u64 = 0x348	/* data breakpoint 7 address */;
pub const LOONGARCH_CSR_DB7MASK: u64 = 0x349	/* data breakpoint 7 mask */;
pub const LOONGARCH_CSR_DB7CTRL: u64 = 0x34a	/* data breakpoint 7 control */;
pub const LOONGARCH_CSR_DB7ASID: u64 = 0x34b	/* data breakpoint 7 asid */;

pub const LOONGARCH_CSR_DB8ADDR: u64 = 0x350	/* data breakpoint 8 address */;
pub const LOONGARCH_CSR_DB8MASK: u64 = 0x351	/* data breakpoint 8 mask */;
pub const LOONGARCH_CSR_DB8CTRL: u64 = 0x352	/* data breakpoint 8 control */;
pub const LOONGARCH_CSR_DB8ASID: u64 = 0x353	/* data breakpoint 8 asid */;

pub const LOONGARCH_CSR_DB9ADDR: u64 = 0x358	/* data breakpoint 9 address */;
pub const LOONGARCH_CSR_DB9MASK: u64 = 0x359	/* data breakpoint 9 mask */;
pub const LOONGARCH_CSR_DB9CTRL: u64 = 0x35a	/* data breakpoint 9 control */;
pub const LOONGARCH_CSR_DB9ASID: u64 = 0x35b	/* data breakpoint 9 asid */;

pub const LOONGARCH_CSR_DB10ADDR: u64 = 0x360	/* data breakpoint 10 address */;
pub const LOONGARCH_CSR_DB10MASK: u64 = 0x361	/* data breakpoint 10 mask */;
pub const LOONGARCH_CSR_DB10CTRL: u64 = 0x362	/* data breakpoint 10 control */;
pub const LOONGARCH_CSR_DB10ASID: u64 = 0x363	/* data breakpoint 10 asid */;

pub const LOONGARCH_CSR_DB11ADDR: u64 = 0x368	/* data breakpoint 11 address */;
pub const LOONGARCH_CSR_DB11MASK: u64 = 0x369	/* data breakpoint 11 mask */;
pub const LOONGARCH_CSR_DB11CTRL: u64 = 0x36a	/* data breakpoint 11 control */;
pub const LOONGARCH_CSR_DB11ASID: u64 = 0x36b	/* data breakpoint 11 asid */;

pub const LOONGARCH_CSR_DB12ADDR: u64 = 0x370	/* data breakpoint 12 address */;
pub const LOONGARCH_CSR_DB12MASK: u64 = 0x371	/* data breakpoint 12 mask */;
pub const LOONGARCH_CSR_DB12CTRL: u64 = 0x372	/* data breakpoint 12 control */;
pub const LOONGARCH_CSR_DB12ASID: u64 = 0x373	/* data breakpoint 12 asid */;

pub const LOONGARCH_CSR_DB13ADDR: u64 = 0x378	/* data breakpoint 13 address */;
pub const LOONGARCH_CSR_DB13MASK: u64 = 0x379	/* data breakpoint 13 mask */;
pub const LOONGARCH_CSR_DB13CTRL: u64 = 0x37a	/* data breakpoint 13 control */;
pub const LOONGARCH_CSR_DB13ASID: u64 = 0x37b	/* data breakpoint 13 asid */;

pub const LOONGARCH_CSR_FWPC: u64 = 0x380	/* instruction breakpoint config */;
pub const LOONGARCH_CSR_FWPS: u64 = 0x381	/* instruction breakpoint status */;

pub const LOONGARCH_CSR_IB0ADDR: u64 = 0x390	/* inst breakpoint 0 address */;
pub const LOONGARCH_CSR_IB0MASK: u64 = 0x391	/* inst breakpoint 0 mask */;
pub const LOONGARCH_CSR_IB0CTRL: u64 = 0x392	/* inst breakpoint 0 control */;
pub const LOONGARCH_CSR_IB0ASID: u64 = 0x393	/* inst breakpoint 0 asid */;

pub const LOONGARCH_CSR_IB1ADDR: u64 = 0x398	/* inst breakpoint 1 address */;
pub const LOONGARCH_CSR_IB1MASK: u64 = 0x399	/* inst breakpoint 1 mask */;
pub const LOONGARCH_CSR_IB1CTRL: u64 = 0x39a	/* inst breakpoint 1 control */;
pub const LOONGARCH_CSR_IB1ASID: u64 = 0x39b	/* inst breakpoint 1 asid */;

pub const LOONGARCH_CSR_IB2ADDR: u64 = 0x3a0	/* inst breakpoint 2 address */;
pub const LOONGARCH_CSR_IB2MASK: u64 = 0x3a1	/* inst breakpoint 2 mask */;
pub const LOONGARCH_CSR_IB2CTRL: u64 = 0x3a2	/* inst breakpoint 2 control */;
pub const LOONGARCH_CSR_IB2ASID: u64 = 0x3a3	/* inst breakpoint 2 asid */;

pub const LOONGARCH_CSR_IB3ADDR: u64 = 0x3a8	/* inst breakpoint 3 address */;
pub const LOONGARCH_CSR_IB3MASK: u64 = 0x3a9	/* breakpoint 3 mask */;
pub const LOONGARCH_CSR_IB3CTRL: u64 = 0x3aa	/* inst breakpoint 3 control */;
pub const LOONGARCH_CSR_IB3ASID: u64 = 0x3ab	/* inst breakpoint 3 asid */;

pub const LOONGARCH_CSR_IB4ADDR: u64 = 0x3b0	/* inst breakpoint 4 address */;
pub const LOONGARCH_CSR_IB4MASK: u64 = 0x3b1	/* inst breakpoint 4 mask */;
pub const LOONGARCH_CSR_IB4CTRL: u64 = 0x3b2	/* inst breakpoint 4 control */;
pub const LOONGARCH_CSR_IB4ASID: u64 = 0x3b3	/* inst breakpoint 4 asid */;

pub const LOONGARCH_CSR_IB5ADDR: u64 = 0x3b8	/* inst breakpoint 5 address */;
pub const LOONGARCH_CSR_IB5MASK: u64 = 0x3b9	/* inst breakpoint 5 mask */;
pub const LOONGARCH_CSR_IB5CTRL: u64 = 0x3ba	/* inst breakpoint 5 control */;
pub const LOONGARCH_CSR_IB5ASID: u64 = 0x3bb	/* inst breakpoint 5 asid */;

pub const LOONGARCH_CSR_IB6ADDR: u64 = 0x3c0	/* inst breakpoint 6 address */;
pub const LOONGARCH_CSR_IB6MASK: u64 = 0x3c1	/* inst breakpoint 6 mask */;
pub const LOONGARCH_CSR_IB6CTRL: u64 = 0x3c2	/* inst breakpoint 6 control */;
pub const LOONGARCH_CSR_IB6ASID: u64 = 0x3c3	/* inst breakpoint 6 asid */;

pub const LOONGARCH_CSR_IB7ADDR: u64 = 0x3c8	/* inst breakpoint 7 address */;
pub const LOONGARCH_CSR_IB7MASK: u64 = 0x3c9	/* inst breakpoint 7 mask */;
pub const LOONGARCH_CSR_IB7CTRL: u64 = 0x3ca	/* inst breakpoint 7 control */;
pub const LOONGARCH_CSR_IB7ASID: u64 = 0x3cb	/* inst breakpoint 7 asid */;

pub const LOONGARCH_CSR_IB8ADDR: u64 = 0x3d0	/* inst breakpoint 8 address */;
pub const LOONGARCH_CSR_IB8MASK: u64 = 0x3d1	/* inst breakpoint 8 mask */;
pub const LOONGARCH_CSR_IB8CTRL: u64 = 0x3d2	/* inst breakpoint 8 control */;
pub const LOONGARCH_CSR_IB8ASID: u64 = 0x3d3	/* inst breakpoint 8 asid */;

pub const LOONGARCH_CSR_IB9ADDR: u64 = 0x3d8	/* inst breakpoint 9 address */;
pub const LOONGARCH_CSR_IB9MASK: u64 = 0x3d9	/* inst breakpoint 9 mask */;
pub const LOONGARCH_CSR_IB9CTRL: u64 = 0x3da	/* inst breakpoint 9 control */;
pub const LOONGARCH_CSR_IB9ASID: u64 = 0x3db	/* inst breakpoint 9 asid */;

pub const LOONGARCH_CSR_IB10ADDR: u64 = 0x3e0	/* inst breakpoint 10 address */;
pub const LOONGARCH_CSR_IB10MASK: u64 = 0x3e1	/* inst breakpoint 10 mask */;
pub const LOONGARCH_CSR_IB10CTRL: u64 = 0x3e2	/* inst breakpoint 10 control */;
pub const LOONGARCH_CSR_IB10ASID: u64 = 0x3e3	/* inst breakpoint 10 asid */;

pub const LOONGARCH_CSR_IB11ADDR: u64 = 0x3e8	/* inst breakpoint 11 address */;
pub const LOONGARCH_CSR_IB11MASK: u64 = 0x3e9	/* inst breakpoint 11 mask */;
pub const LOONGARCH_CSR_IB11CTRL: u64 = 0x3ea	/* inst breakpoint 11 control */;
pub const LOONGARCH_CSR_IB11ASID: u64 = 0x3eb	/* inst breakpoint 11 asid */;

pub const LOONGARCH_CSR_IB12ADDR: u64 = 0x3f0	/* inst breakpoint 12 address */;
pub const LOONGARCH_CSR_IB12MASK: u64 = 0x3f1	/* inst breakpoint 12 mask */;
pub const LOONGARCH_CSR_IB12CTRL: u64 = 0x3f2	/* inst breakpoint 12 control */;
pub const LOONGARCH_CSR_IB12ASID: u64 = 0x3f3	/* inst breakpoint 12 asid */;

pub const LOONGARCH_CSR_IB13ADDR: u64 = 0x3f8	/* inst breakpoint 13 address */;
pub const LOONGARCH_CSR_IB13MASK: u64 = 0x3f9	/* inst breakpoint 13 mask */;
pub const LOONGARCH_CSR_IB13CTRL: u64 = 0x3fa	/* inst breakpoint 13 control */;
pub const LOONGARCH_CSR_IB13ASID: u64 = 0x3fb	/* inst breakpoint 13 asid */;

pub const LOONGARCH_CSR_DEBUG: u64 = 0x500	/* debug config */;
pub const LOONGARCH_CSR_DERA: u64 = 0x501	/* debug era */;
pub const LOONGARCH_CSR_DESAVE: u64 = 0x502	/* debug save */;

pub const CSR_FWPC_SKIP_SHIFT: u64 = 16;
// #define CSR_FWPC_SKIP			(_ULCAST_(1) << CSR_FWPC_SKIP_SHIFT)

/*
 * CSR_ECFG IM
 */
pub const ECFG0_IM: u64 = 0x00005fff;
pub const ECFGB_SIP0: u64 = 0;
// #define ECFGF_SIP0		(_ULCAST_(1) << ECFGB_SIP0)
pub const ECFGB_SIP1: u64 = 1;
// #define ECFGF_SIP1		(_ULCAST_(1) << ECFGB_SIP1)
pub const ECFGB_IP0: u64 = 2;
// #define ECFGF_IP0		(_ULCAST_(1) << ECFGB_IP0)
pub const ECFGB_IP1: u64 = 3;
// #define ECFGF_IP1		(_ULCAST_(1) << ECFGB_IP1)
pub const ECFGB_IP2: u64 = 4;
// #define ECFGF_IP2		(_ULCAST_(1) << ECFGB_IP2)
pub const ECFGB_IP3: u64 = 5;
// #define ECFGF_IP3		(_ULCAST_(1) << ECFGB_IP3)
pub const ECFGB_IP4: u64 = 6;
// #define ECFGF_IP4		(_ULCAST_(1) << ECFGB_IP4)
pub const ECFGB_IP5: u64 = 7;
// #define ECFGF_IP5		(_ULCAST_(1) << ECFGB_IP5)
pub const ECFGB_IP6: u64 = 8;
// #define ECFGF_IP6		(_ULCAST_(1) << ECFGB_IP6)
pub const ECFGB_IP7: u64 = 9;
// #define ECFGF_IP7		(_ULCAST_(1) << ECFGB_IP7)
pub const ECFGB_PMC: u64 = 10;
// #define ECFGF_PMC		(_ULCAST_(1) << ECFGB_PMC)
pub const ECFGB_TIMER: u64 = 11;
// #define ECFGF_TIMER		(_ULCAST_(1) << ECFGB_TIMER)
pub const ECFGB_IPI: u64 = 12;
// #define ECFGF_IPI		(_ULCAST_(1) << ECFGB_IPI)
// #define ECFGF(hwirq)		(_ULCAST_(1) << hwirq)

pub const ESTATF_IP: u64 = 0x00003fff;

pub const LOONGARCH_IOCSR_FEATURES: u64 = 0x8;
pub const IOCSRF_TEMP: u64 = BIT_(0);
pub const IOCSRF_NODECNT: u64 = BIT_(1);
pub const IOCSRF_MSI: u64 = BIT_(2);
pub const IOCSRF_EXTIOI: u64 = BIT_(3);
pub const IOCSRF_CSRIPI: u64 = BIT_(4);
pub const IOCSRF_FREQCSR: u64 = BIT_(5);
pub const IOCSRF_FREQSCALE: u64 = BIT_(6);
pub const IOCSRF_DVFSV1: u64 = BIT_(7);
pub const IOCSRF_EIODECODE: u64 = BIT_(9);
pub const IOCSRF_FLATMODE: u64 = BIT_(10);
pub const IOCSRF_VM: u64 = BIT_(11);
pub const IOCSRF_AVEC: u64 = BIT_(15);
pub const IOCSRF_REDIRECT: u64 = BIT_(16);

pub const LOONGARCH_IOCSR_VENDOR: u64 = 0x10;

pub const LOONGARCH_IOCSR_CPUNAME: u64 = 0x20;

pub const LOONGARCH_IOCSR_NODECNT: u64 = 0x408;

pub const LOONGARCH_IOCSR_MISC_FUNC: u64 = 0x420;
pub const IOCSR_MISC_FUNC_SOFT_INT: u64 = BIT_(10);
pub const IOCSR_MISC_FUNC_TIMER_RESET: u64 = BIT_(21);
pub const IOCSR_MISC_FUNC_EXT_IOI_EN: u64 = BIT_(48);
pub const IOCSR_MISC_FUNC_AVEC_EN: u64 = BIT_(51);

pub const LOONGARCH_IOCSR_CPUTEMP: u64 = 0x428;

pub const LOONGARCH_IOCSR_SMCMBX: u64 = 0x51c;

/* PerCore CSR, only accessible by local cores */
pub const LOONGARCH_IOCSR_IPI_STATUS: u64 = 0x1000;
pub const LOONGARCH_IOCSR_IPI_EN: u64 = 0x1004;
pub const LOONGARCH_IOCSR_IPI_SET: u64 = 0x1008;
pub const LOONGARCH_IOCSR_IPI_CLEAR: u64 = 0x100c;
pub const LOONGARCH_IOCSR_MBUF0: u64 = 0x1020;
pub const LOONGARCH_IOCSR_MBUF1: u64 = 0x1028;
pub const LOONGARCH_IOCSR_MBUF2: u64 = 0x1030;
pub const LOONGARCH_IOCSR_MBUF3: u64 = 0x1038;

pub const LOONGARCH_IOCSR_IPI_SEND: u64 = 0x1040;
pub const IOCSR_IPI_SEND_IP_SHIFT: u64 = 0;
pub const IOCSR_IPI_SEND_CPU_SHIFT: u64 = 16;
pub const IOCSR_IPI_SEND_BLOCKING: u64 = BIT(31);

pub const LOONGARCH_IOCSR_MBUF_SEND: u64 = 0x1048;
pub const IOCSR_MBUF_SEND_BLOCKING: u64 = BIT_(31);
pub const IOCSR_MBUF_SEND_BOX_SHIFT: u64 = 2;
// #define  IOCSR_MBUF_SEND_BOX_LO(box)	(box << 1)
// #define  IOCSR_MBUF_SEND_BOX_HI(box)	((box << 1) + 1)
pub const IOCSR_MBUF_SEND_CPU_SHIFT: u64 = 16;
pub const IOCSR_MBUF_SEND_BUF_SHIFT: u64 = 32;
pub const IOCSR_MBUF_SEND_H32_MASK: u64 = 0xFFFFFFFF00000000;

pub const LOONGARCH_IOCSR_ANY_SEND: u64 = 0x1158;
pub const IOCSR_ANY_SEND_BLOCKING: u64 = BIT_(31);
pub const IOCSR_ANY_SEND_CPU_SHIFT: u64 = 16;
pub const IOCSR_ANY_SEND_MASK_SHIFT: u64 = 27;
pub const IOCSR_ANY_SEND_BUF_SHIFT: u64 = 32;
pub const IOCSR_ANY_SEND_H32_MASK: u64 = 0xFFFFFFFF00000000;

/* Register offset and bit definition for CSR access */
pub const LOONGARCH_IOCSR_TIMER_CFG: u64 = 0x1060;
pub const LOONGARCH_IOCSR_TIMER_TICK: u64 = 0x1070;
// #define  IOCSR_TIMER_CFG_RESERVED       (_ULCAST_(1) << 63)
// #define  IOCSR_TIMER_CFG_PERIODIC       (_ULCAST_(1) << 62)
// #define  IOCSR_TIMER_CFG_EN             (_ULCAST_(1) << 61)
pub const IOCSR_TIMER_MASK: u64 = 0x0ffffffffffff;
// #define  IOCSR_TIMER_INITVAL_RST        (_ULCAST_(0xffff) << 48)

pub const LOONGARCH_IOCSR_EXTIOI_NODEMAP_BASE: u64 = 0x14a0;
pub const LOONGARCH_IOCSR_EXTIOI_IPMAP_BASE: u64 = 0x14c0;
pub const LOONGARCH_IOCSR_EXTIOI_EN_BASE: u64 = 0x1600;
pub const LOONGARCH_IOCSR_EXTIOI_BOUNCE_BASE: u64 = 0x1680;
pub const LOONGARCH_IOCSR_EXTIOI_ISR_BASE: u64 = 0x1800;
pub const LOONGARCH_IOCSR_EXTIOI_ROUTE_BASE: u64 = 0x1c00;
pub const IOCSR_EXTIOI_VECTOR_NUM: u64 = 256;

// #ifndef __ASSEMBLER__

// #ifdef CONFIG_32BIT

// static __always_inline u32 rdtime_h(void)
// {
// 	u32 val = 0;

// 	__asm__ __volatile__(
// 		"rdtimeh.w %0, $zero\n\t"
// 		: "=r"(val)
// 		:
// 		);
// 	return val;
// }

// static __always_inline u32 rdtime_l(void)
// {
// 	u32 val = 0;

// 	__asm__ __volatile__(
// 		"rdtimel.w %0, $zero\n\t"
// 		: "=r"(val)
// 		:
// 		);
// 	return val;
// }

// #else

// static __always_inline u64 rdtime_d(void)
// {
// 	u64 val = 0;

// 	__asm__ __volatile__(
// 		"rdtime.d %0, $zero\n\t"
// 		: "=r"(val)
// 		:
// 		);
// 	return val;
// }

// #endif

// static inline unsigned int get_csr_cpuid(void)
// {
// 	return csr_read32(LOONGARCH_CSR_CPUID);
// }

// #ifdef CONFIG_64BIT
// static inline void csr_any_send(unsigned int addr, unsigned int data,
// 				unsigned int data_mask, unsigned int cpu)
// {
// 	uint64_t val = 0;

// 	val = IOCSR_ANY_SEND_BLOCKING | addr;
// 	val |= (cpu << IOCSR_ANY_SEND_CPU_SHIFT);
// 	val |= (data_mask << IOCSR_ANY_SEND_MASK_SHIFT);
// 	val |= ((uint64_t)data << IOCSR_ANY_SEND_BUF_SHIFT);
// 	iocsr_write64(val, LOONGARCH_IOCSR_ANY_SEND);
// }
// #endif

// static inline unsigned int read_csr_excode(void)
// {
// 	return (csr_read32(LOONGARCH_CSR_ESTAT) & CSR_ESTAT_EXC) >> CSR_ESTAT_EXC_SHIFT;
// }

// static inline void write_csr_index(unsigned int idx)
// {
// 	csr_xchg32(idx, CSR_TLBIDX_IDXM, LOONGARCH_CSR_TLBIDX);
// }

// static inline unsigned int read_csr_pagesize(void)
// {
// 	return (csr_read32(LOONGARCH_CSR_TLBIDX) & CSR_TLBIDX_SIZEM) >> CSR_TLBIDX_SIZE;
// }

// static inline void write_csr_pagesize(unsigned int size)
// {
// 	csr_xchg32(size << CSR_TLBIDX_SIZE, CSR_TLBIDX_SIZEM, LOONGARCH_CSR_TLBIDX);
// }

// static inline unsigned int read_csr_tlbrefill_pagesize(void)
// {
// 	return (csr_read(LOONGARCH_CSR_TLBREHI) & CSR_TLBREHI_PS) >> CSR_TLBREHI_PS_SHIFT;
// }

// static inline void write_csr_tlbrefill_pagesize(unsigned int size)
// {
// 	csr_xchg(size << CSR_TLBREHI_PS_SHIFT, CSR_TLBREHI_PS, LOONGARCH_CSR_TLBREHI);
// }

// #define read_csr_asid()			csr_read32(LOONGARCH_CSR_ASID)
// #define write_csr_asid(val)		csr_write32(val, LOONGARCH_CSR_ASID)
// #define read_csr_entryhi()		csr_read(LOONGARCH_CSR_TLBEHI)
// #define write_csr_entryhi(val)		csr_write(val, LOONGARCH_CSR_TLBEHI)
// #define read_csr_entrylo0()		csr_read(LOONGARCH_CSR_TLBELO0)
// #define write_csr_entrylo0(val)		csr_write(val, LOONGARCH_CSR_TLBELO0)
// #define read_csr_entrylo1()		csr_read(LOONGARCH_CSR_TLBELO1)
// #define write_csr_entrylo1(val)		csr_write(val, LOONGARCH_CSR_TLBELO1)
// #define read_csr_ecfg()			csr_read32(LOONGARCH_CSR_ECFG)
// #define write_csr_ecfg(val)		csr_write32(val, LOONGARCH_CSR_ECFG)
// #define read_csr_estat()		csr_read32(LOONGARCH_CSR_ESTAT)
// #define write_csr_estat(val)		csr_write32(val, LOONGARCH_CSR_ESTAT)
// #define read_csr_tlbidx()		csr_read32(LOONGARCH_CSR_TLBIDX)
// #define write_csr_tlbidx(val)		csr_write32(val, LOONGARCH_CSR_TLBIDX)
// #define read_csr_euen()			csr_read32(LOONGARCH_CSR_EUEN)
// #define write_csr_euen(val)		csr_write32(val, LOONGARCH_CSR_EUEN)
// #define read_csr_cpuid()		csr_read32(LOONGARCH_CSR_CPUID)
// #define read_csr_prcfg1()		csr_read(LOONGARCH_CSR_PRCFG1)
// #define write_csr_prcfg1(val)		csr_write(val, LOONGARCH_CSR_PRCFG1)
// #define read_csr_prcfg2()		csr_read(LOONGARCH_CSR_PRCFG2)
// #define write_csr_prcfg2(val)		csr_write(val, LOONGARCH_CSR_PRCFG2)
// #define read_csr_prcfg3()		csr_read(LOONGARCH_CSR_PRCFG3)
// #define write_csr_prcfg3(val)		csr_write(val, LOONGARCH_CSR_PRCFG3)
// #define read_csr_stlbpgsize()		csr_read32(LOONGARCH_CSR_STLBPGSIZE)
// #define write_csr_stlbpgsize(val)	csr_write32(val, LOONGARCH_CSR_STLBPGSIZE)
// #define read_csr_rvacfg()		csr_read32(LOONGARCH_CSR_RVACFG)
// #define write_csr_rvacfg(val)		csr_write32(val, LOONGARCH_CSR_RVACFG)
// #define write_csr_tintclear(val)	csr_write32(val, LOONGARCH_CSR_TINTCLR)
// #define read_csr_impctl1()		csr_read(LOONGARCH_CSR_IMPCTL1)
// #define write_csr_impctl1(val)		csr_write(val, LOONGARCH_CSR_IMPCTL1)
// #define write_csr_impctl2(val)		csr_write(val, LOONGARCH_CSR_IMPCTL2)

// #define read_csr_perfctrl0()		csr_read64(LOONGARCH_CSR_PERFCTRL0)
// #define read_csr_perfcntr0()		csr_read64(LOONGARCH_CSR_PERFCNTR0)
// #define read_csr_perfctrl1()		csr_read64(LOONGARCH_CSR_PERFCTRL1)
// #define read_csr_perfcntr1()		csr_read64(LOONGARCH_CSR_PERFCNTR1)
// #define read_csr_perfctrl2()		csr_read64(LOONGARCH_CSR_PERFCTRL2)
// #define read_csr_perfcntr2()		csr_read64(LOONGARCH_CSR_PERFCNTR2)
// #define read_csr_perfctrl3()		csr_read64(LOONGARCH_CSR_PERFCTRL3)
// #define read_csr_perfcntr3()		csr_read64(LOONGARCH_CSR_PERFCNTR3)
// #define write_csr_perfctrl0(val)	csr_write64(val, LOONGARCH_CSR_PERFCTRL0)
// #define write_csr_perfcntr0(val)	csr_write64(val, LOONGARCH_CSR_PERFCNTR0)
// #define write_csr_perfctrl1(val)	csr_write64(val, LOONGARCH_CSR_PERFCTRL1)
// #define write_csr_perfcntr1(val)	csr_write64(val, LOONGARCH_CSR_PERFCNTR1)
// #define write_csr_perfctrl2(val)	csr_write64(val, LOONGARCH_CSR_PERFCTRL2)
// #define write_csr_perfcntr2(val)	csr_write64(val, LOONGARCH_CSR_PERFCNTR2)
// #define write_csr_perfctrl3(val)	csr_write64(val, LOONGARCH_CSR_PERFCTRL3)
// #define write_csr_perfcntr3(val)	csr_write64(val, LOONGARCH_CSR_PERFCNTR3)

/*
 * Manipulate bits in a register.
 */
// #define __BUILD_CSR_COMMON(name)				\
// static inline unsigned long					\
// set_##name(unsigned long set)					\
// {								\
// 	unsigned long res, new;					\
// 								\
// 	res = read_##name();					\
// 	new = res | set;					\
// 	write_##name(new);					\
// 								\
// 	return res;						\
// }								\
// 								\
// static inline unsigned long					\
// clear_##name(unsigned long clear)				\
// {								\
// 	unsigned long res, new;					\
// 								\
// 	res = read_##name();					\
// 	new = res & ~clear;					\
// 	write_##name(new);					\
// 								\
// 	return res;						\
// }								\
// 								\
// static inline unsigned long					\
// change_##name(unsigned long change, unsigned long val)		\
// {								\
// 	unsigned long res, new;					\
// 								\
// 	res = read_##name();					\
// 	new = res & ~change;					\
// 	new |= (val & change);					\
// 	write_##name(new);					\
// 								\
// 	return res;						\
// }

// #define __BUILD_CSR_OP(name)	__BUILD_CSR_COMMON(csr_##name)

// __BUILD_CSR_OP(euen)
// __BUILD_CSR_OP(ecfg)
// __BUILD_CSR_OP(tlbidx)

// #define set_csr_estat(val)	\
// 	csr_xchg32(val, val, LOONGARCH_CSR_ESTAT)
// #define clear_csr_estat(val)	\
// 	csr_xchg32(~(val), val, LOONGARCH_CSR_ESTAT)

// #endif /* __ASSEMBLER__ */

/* Generic EntryLo bit definitions */
// #define ENTRYLO_V		(_ULCAST_(1) << 0)
// #define ENTRYLO_D		(_ULCAST_(1) << 1)
pub const ENTRYLO_PLV_SHIFT: u64 = 2;
// #define ENTRYLO_PLV		(_ULCAST_(3) << ENTRYLO_PLV_SHIFT)
pub const ENTRYLO_C_SHIFT: u64 = 4;
// #define ENTRYLO_C		(_ULCAST_(3) << ENTRYLO_C_SHIFT)
// #define ENTRYLO_G		(_ULCAST_(1) << 6)
// #ifdef CONFIG_64BIT
// #define ENTRYLO_NR		(_ULCAST_(1) << 61)
// #define ENTRYLO_NX		(_ULCAST_(1) << 62)
// #endif

/* Values for PageSize register */
pub const PS_4K: u64 = 0x0000000c;
pub const PS_8K: u64 = 0x0000000d;
pub const PS_16K: u64 = 0x0000000e;
pub const PS_32K: u64 = 0x0000000f;
pub const PS_64K: u64 = 0x00000010;
pub const PS_128K: u64 = 0x00000011;
pub const PS_256K: u64 = 0x00000012;
pub const PS_512K: u64 = 0x00000013;
pub const PS_1M: u64 = 0x00000014;
pub const PS_2M: u64 = 0x00000015;
pub const PS_4M: u64 = 0x00000016;
pub const PS_8M: u64 = 0x00000017;
pub const PS_16M: u64 = 0x00000018;
pub const PS_32M: u64 = 0x00000019;
pub const PS_64M: u64 = 0x0000001a;
pub const PS_128M: u64 = 0x0000001b;
pub const PS_256M: u64 = 0x0000001c;
pub const PS_512M: u64 = 0x0000001d;
pub const PS_1G: u64 = 0x0000001e;

/* Default page size for a given kernel configuration */
// #ifdef CONFIG_PAGE_SIZE_4KB
pub const PS_DEFAULT_SIZE: u64 = PS_4K;
// #elif defined(CONFIG_PAGE_SIZE_16KB)
pub const PS_DEFAULT_SIZE: u64 = PS_16K;
// #elif defined(CONFIG_PAGE_SIZE_64KB)
pub const PS_DEFAULT_SIZE: u64 = PS_64K;
// #else
// #error Bad page size configuration!
// #endif

/* Default huge tlb size for a given kernel configuration */
// #ifdef CONFIG_PAGE_SIZE_4KB
pub const PS_HUGE_SIZE: u64 = PS_1M;
// #elif defined(CONFIG_PAGE_SIZE_16KB)
pub const PS_HUGE_SIZE: u64 = PS_16M;
// #elif defined(CONFIG_PAGE_SIZE_64KB)
pub const PS_HUGE_SIZE: u64 = PS_256M;
// #else
// #error Bad page size configuration for hugetlbfs!
// #endif

/* ExStatus.ExcCode */
pub const EXCCODE_RSV: u64 = 0	/* Reserved */;
pub const EXCCODE_TLBL: u64 = 1	/* TLB miss on a load */;
pub const EXCCODE_TLBS: u64 = 2	/* TLB miss on a store */;
pub const EXCCODE_TLBI: u64 = 3	/* TLB miss on a ifetch */;
pub const EXCCODE_TLBM: u64 = 4	/* TLB modified fault */;
pub const EXCCODE_TLBNR: u64 = 5	/* TLB Read-Inhibit exception */;
pub const EXCCODE_TLBNX: u64 = 6	/* TLB Execution-Inhibit exception */;
pub const EXCCODE_TLBPE: u64 = 7	/* TLB Privilege Error */;
pub const EXCCODE_ADE: u64 = 8	/* Address Error */;
// 	#define EXSUBCODE_ADEF		0	/* Fetch Instruction */
// 	#define EXSUBCODE_ADEM		1	/* Access Memory*/
pub const EXCCODE_ALE: u64 = 9	/* Unalign Access */;
pub const EXCCODE_BCE: u64 = 10	/* Bounds Check Error */;
pub const EXCCODE_SYS: u64 = 11	/* System call */;
pub const EXCCODE_BP: u64 = 12	/* Breakpoint */;
pub const EXCCODE_INE: u64 = 13	/* Inst. Not Exist */;
pub const EXCCODE_IPE: u64 = 14	/* Inst. Privileged Error */;
pub const EXCCODE_FPDIS: u64 = 15	/* FPU Disabled */;
pub const EXCCODE_LSXDIS: u64 = 16	/* LSX Disabled */;
pub const EXCCODE_LASXDIS: u64 = 17	/* LASX Disabled */;
pub const EXCCODE_FPE: u64 = 18	/* Floating Point Exception */;
// 	#define EXCSUBCODE_FPE		0	/* Floating Point Exception */
// 	#define EXCSUBCODE_VFPE		1	/* Vector Exception */
pub const EXCCODE_WATCH: u64 = 19	/* WatchPoint Exception */;
// 	#define EXCSUBCODE_WPEF		0	/* ... on Instruction Fetch */
// 	#define EXCSUBCODE_WPEM		1	/* ... on Memory Accesses */
pub const EXCCODE_BTDIS: u64 = 20	/* Binary Trans. Disabled */;
pub const EXCCODE_BTE: u64 = 21	/* Binary Trans. Exception */;
pub const EXCCODE_GSPR: u64 = 22	/* Guest Privileged Error */;
pub const EXCCODE_HVC: u64 = 23	/* Hypercall */;
pub const EXCCODE_GCM: u64 = 24	/* Guest CSR modified */;
// 	#define EXCSUBCODE_GCSC		0	/* Software caused */
// 	#define EXCSUBCODE_GCHC		1	/* Hardware caused */
pub const EXCCODE_SE: u64 = 25	/* Security */;

/* Interrupt numbers */
pub const INT_SWI0: u64 = 0	/* Software Interrupts */;
pub const INT_SWI1: u64 = 1;
pub const INT_HWI0: u64 = 2	/* Hardware Interrupts */;
pub const INT_HWI1: u64 = 3;
pub const INT_HWI2: u64 = 4;
pub const INT_HWI3: u64 = 5;
pub const INT_HWI4: u64 = 6;
pub const INT_HWI5: u64 = 7;
pub const INT_HWI6: u64 = 8;
pub const INT_HWI7: u64 = 9;
pub const INT_PCOV: u64 = 10	/* Performance Counter Overflow */;
pub const INT_TI: u64 = 11	/* Timer */;
pub const INT_IPI: u64 = 12;
pub const INT_NMI: u64 = 13;
pub const INT_AVEC: u64 = 14;

/* ExcCodes corresponding to interrupts */
// #define EXCCODE_INT_NUM		(INT_AVEC + 1)
pub const EXCCODE_INT_START: u64 = 64;
// #define EXCCODE_INT_END		(EXCCODE_INT_START + EXCCODE_INT_NUM - 1)

/* FPU Status Register Names */
// #ifndef CONFIG_AS_HAS_FCSR_CLASS
pub const LOONGARCH_FCSR0: u64 = $r0;
pub const LOONGARCH_FCSR1: u64 = $r1;
pub const LOONGARCH_FCSR2: u64 = $r2;
pub const LOONGARCH_FCSR3: u64 = $r3;
// #else
pub const LOONGARCH_FCSR0: u64 = $fcsr0;
pub const LOONGARCH_FCSR1: u64 = $fcsr1;
pub const LOONGARCH_FCSR2: u64 = $fcsr2;
pub const LOONGARCH_FCSR3: u64 = $fcsr3;
// #endif

/* FPU Status Register Values */
pub const FPU_CSR_RSVD: u64 = 0xe0e0fce0;

/*
 * X the exception cause indicator
 * E the exception enable
 * S the sticky/flag bit
 */
pub const FPU_CSR_ALL_X: u64 = 0x1f000000;
pub const FPU_CSR_INV_X: u64 = 0x10000000;
pub const FPU_CSR_DIV_X: u64 = 0x08000000;
pub const FPU_CSR_OVF_X: u64 = 0x04000000;
pub const FPU_CSR_UDF_X: u64 = 0x02000000;
pub const FPU_CSR_INE_X: u64 = 0x01000000;

pub const FPU_CSR_ALL_S: u64 = 0x001f0000;
pub const FPU_CSR_INV_S: u64 = 0x00100000;
pub const FPU_CSR_DIV_S: u64 = 0x00080000;
pub const FPU_CSR_OVF_S: u64 = 0x00040000;
pub const FPU_CSR_UDF_S: u64 = 0x00020000;
pub const FPU_CSR_INE_S: u64 = 0x00010000;

pub const FPU_CSR_ALL_E: u64 = 0x0000001f;
pub const FPU_CSR_INV_E: u64 = 0x00000010;
pub const FPU_CSR_DIV_E: u64 = 0x00000008;
pub const FPU_CSR_OVF_E: u64 = 0x00000004;
pub const FPU_CSR_UDF_E: u64 = 0x00000002;
pub const FPU_CSR_INE_E: u64 = 0x00000001;

/* Bits 8 and 9 of FPU Status Register specify the rounding mode */
pub const FPU_CSR_RM: u64 = 0x300;
pub const FPU_CSR_RN: u64 = 0x000	/* nearest */;
pub const FPU_CSR_RZ: u64 = 0x100	/* towards zero */;
pub const FPU_CSR_RU: u64 = 0x200	/* towards +Infinity */;
pub const FPU_CSR_RD: u64 = 0x300	/* towards -Infinity */;

/* Bit 6 of FPU Status Register specify the LBT TOP simulation mode */
pub const FPU_CSR_TM_SHIFT: u64 = 0x6;
// #define FPU_CSR_TM		(_ULCAST_(1) << FPU_CSR_TM_SHIFT)

// #define read_fcsr(source)	\
// ({	\
// 	unsigned int __res;	\
// \
// 	__asm__ __volatile__(	\
// 	"	movfcsr2gr	%0, "__stringify(source)" \n"	\
// 	: "=r" (__res));	\
// 	__res;	\
// })

// #define write_fcsr(dest, val) \
// do {	\
// 	__asm__ __volatile__(	\
// 	"	movgr2fcsr	"__stringify(dest)", %0	\n"	\
// 	: : "r" (val));	\
// } while (0)

// #endif /* _ASM_LOONGARCH_H */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
