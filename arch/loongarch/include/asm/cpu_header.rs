/* SPDX-License-Identifier: GPL-2.0 */
/*
 * cpu.h: Values of the PRID register used to match up
 *	  various LoongArch CPU types.
 *
 * Copyright (C) 2020-2022 Loongson Technology Corporation Limited
 */

/*
 * As described in LoongArch specs from Loongson Technology, the PRID register
 * (CPUCFG.00) has the following layout:
 *
 * +---------------+----------------+------------+--------------------+
 * | Reserved      | Company ID     | Series ID  |  Product ID        |
 * +---------------+----------------+------------+--------------------+
 *  31		 24 23		  16 15	       12 11		     0
 */

/*
 * Assigned Company values for bits 23:16 of the PRID register.
 */

pub const PRID_COMP_MASK: u32 = 0xff0000;

pub const PRID_COMP_LOONGSON: u32 = 0x140000;

/*
 * Assigned Series ID values for bits 15:12 of the PRID register. In order
 * to detect a certain CPU type exactly eventually additional registers may
 * need to be examined.
 */

pub const PRID_SERIES_MASK: u32 = 0xf000;

pub const PRID_SERIES_LA132: u32 = 0x8000; /* Loongson 32bit */
pub const PRID_SERIES_LA264: u32 = 0xa000; /* Loongson 64bit, 2-issue */
pub const PRID_SERIES_LA364: u32 = 0xb000; /* Loongson 64bit, 3-issue */
pub const PRID_SERIES_LA464: u32 = 0xc000; /* Loongson 64bit, 4-issue */
pub const PRID_SERIES_LA664: u32 = 0xd000; /* Loongson 64bit, 6-issue */

/* Particular Product ID values for bits 11:0 of the PRID register. */
pub const PRID_PRODUCT_MASK: u32 = 0x0fff;

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum cpu_type_enum {
    CPU_UNKNOWN,
    CPU_LOONGSON32,
    CPU_LOONGSON64,
    CPU_LAST,
}

pub fn id_to_core_name(id: u32) -> &'static str {
    if (id & PRID_COMP_MASK) != PRID_COMP_LOONGSON {
        return "Unknown";
    }

    match id & PRID_SERIES_MASK {
        PRID_SERIES_LA132 => "LA132",
        PRID_SERIES_LA264 => "LA264",
        PRID_SERIES_LA364 => "LA364",
        PRID_SERIES_LA464 => "LA464",
        PRID_SERIES_LA664 => "LA664",
        _ => "Unknown",
    }
}

/* ISA Level encodings */
pub const LOONGARCH_CPU_ISA_LA32R: u32 = 0x00000001;
pub const LOONGARCH_CPU_ISA_LA32S: u32 = 0x00000002;
pub const LOONGARCH_CPU_ISA_LA64: u32 = 0x00000004;

pub const LOONGARCH_CPU_ISA_32BIT: u32 = LOONGARCH_CPU_ISA_LA32R | LOONGARCH_CPU_ISA_LA32S;
pub const LOONGARCH_CPU_ISA_64BIT: u32 = LOONGARCH_CPU_ISA_LA64;

/* CPU Option encodings */
pub const CPU_FEATURE_CPUCFG: u32 = 0; /* CPU has CPUCFG */
pub const CPU_FEATURE_LAM: u32 = 1; /* CPU has Atomic instructions */
pub const CPU_FEATURE_LAM_BH: u32 = 2; /* CPU has AM{SWAP/ADD}[_DB].{B/H} instructions */
pub const CPU_FEATURE_SCQ: u32 = 3; /* CPU has SC.Q instruction */
pub const CPU_FEATURE_UAL: u32 = 4; /* CPU supports unaligned access */
pub const CPU_FEATURE_FPU: u32 = 5; /* CPU has FPU */
pub const CPU_FEATURE_LSX: u32 = 6; /* CPU has LSX (128-bit SIMD) */
pub const CPU_FEATURE_LASX: u32 = 7; /* CPU has LASX (256-bit SIMD) */
pub const CPU_FEATURE_CRC32: u32 = 8; /* CPU has CRC32 instructions */
pub const CPU_FEATURE_COMPLEX: u32 = 9; /* CPU has Complex instructions */
pub const CPU_FEATURE_CRYPTO: u32 = 10; /* CPU has Crypto instructions */
pub const CPU_FEATURE_LVZ: u32 = 11; /* CPU has Virtualization extension */
pub const CPU_FEATURE_LBT_X86: u32 = 12; /* CPU has X86 Binary Translation */
pub const CPU_FEATURE_LBT_ARM: u32 = 13; /* CPU has ARM Binary Translation */
pub const CPU_FEATURE_LBT_MIPS: u32 = 14; /* CPU has MIPS Binary Translation */
pub const CPU_FEATURE_TLB: u32 = 15; /* CPU has TLB */
pub const CPU_FEATURE_CSR: u32 = 16; /* CPU has CSR */
pub const CPU_FEATURE_IOCSR: u32 = 17; /* CPU has IOCSR */
pub const CPU_FEATURE_WATCH: u32 = 18; /* CPU has watchpoint registers */
pub const CPU_FEATURE_VINT: u32 = 19; /* CPU has vectored interrupts */
pub const CPU_FEATURE_CSRIPI: u32 = 20; /* CPU has CSR-IPI */
pub const CPU_FEATURE_EXTIOI: u32 = 21; /* CPU has EXT-IOI */
pub const CPU_FEATURE_PREFETCH: u32 = 22; /* CPU has prefetch instructions */
pub const CPU_FEATURE_PMP: u32 = 23; /* CPU has perfermance counter */
pub const CPU_FEATURE_SCALEFREQ: u32 = 24; /* CPU supports cpufreq scaling */
pub const CPU_FEATURE_FLATMODE: u32 = 25; /* CPU has flat mode */
pub const CPU_FEATURE_EIODECODE: u32 = 26; /* CPU has EXTIOI interrupt pin decode mode */
pub const CPU_FEATURE_GUESTID: u32 = 27; /* CPU has GuestID feature */
pub const CPU_FEATURE_HYPERVISOR: u32 = 28; /* CPU has hypervisor (running in VM) */
pub const CPU_FEATURE_PTW: u32 = 29; /* CPU has hardware page table walker */
pub const CPU_FEATURE_LSPW: u32 = 30; /* CPU has LSPW (lddir/ldpte instructions) */
pub const CPU_FEATURE_MSGINT: u32 = 31; /* CPU has MSG interrupt */
pub const CPU_FEATURE_AVECINT: u32 = 32; /* CPU has AVEC interrupt */
pub const CPU_FEATURE_REDIRECTINT: u32 = 33; /* CPU has interrupt remapping */

pub const LOONGARCH_CPU_CPUCFG: u64 = 1u64 << CPU_FEATURE_CPUCFG;
pub const LOONGARCH_CPU_LAM: u64 = 1u64 << CPU_FEATURE_LAM;
pub const LOONGARCH_CPU_LAM_BH: u64 = 1u64 << CPU_FEATURE_LAM_BH;
pub const LOONGARCH_CPU_SCQ: u64 = 1u64 << CPU_FEATURE_SCQ;
pub const LOONGARCH_CPU_UAL: u64 = 1u64 << CPU_FEATURE_UAL;
pub const LOONGARCH_CPU_FPU: u64 = 1u64 << CPU_FEATURE_FPU;
pub const LOONGARCH_CPU_LSX: u64 = 1u64 << CPU_FEATURE_LSX;
pub const LOONGARCH_CPU_LASX: u64 = 1u64 << CPU_FEATURE_LASX;
pub const LOONGARCH_CPU_CRC32: u64 = 1u64 << CPU_FEATURE_CRC32;
pub const LOONGARCH_CPU_COMPLEX: u64 = 1u64 << CPU_FEATURE_COMPLEX;
pub const LOONGARCH_CPU_CRYPTO: u64 = 1u64 << CPU_FEATURE_CRYPTO;
pub const LOONGARCH_CPU_LVZ: u64 = 1u64 << CPU_FEATURE_LVZ;
pub const LOONGARCH_CPU_LBT_X86: u64 = 1u64 << CPU_FEATURE_LBT_X86;
pub const LOONGARCH_CPU_LBT_ARM: u64 = 1u64 << CPU_FEATURE_LBT_ARM;
pub const LOONGARCH_CPU_LBT_MIPS: u64 = 1u64 << CPU_FEATURE_LBT_MIPS;
pub const LOONGARCH_CPU_TLB: u64 = 1u64 << CPU_FEATURE_TLB;
pub const LOONGARCH_CPU_IOCSR: u64 = 1u64 << CPU_FEATURE_IOCSR;
pub const LOONGARCH_CPU_CSR: u64 = 1u64 << CPU_FEATURE_CSR;
pub const LOONGARCH_CPU_WATCH: u64 = 1u64 << CPU_FEATURE_WATCH;
pub const LOONGARCH_CPU_VINT: u64 = 1u64 << CPU_FEATURE_VINT;
pub const LOONGARCH_CPU_CSRIPI: u64 = 1u64 << CPU_FEATURE_CSRIPI;
pub const LOONGARCH_CPU_EXTIOI: u64 = 1u64 << CPU_FEATURE_EXTIOI;
pub const LOONGARCH_CPU_PREFETCH: u64 = 1u64 << CPU_FEATURE_PREFETCH;
pub const LOONGARCH_CPU_PMP: u64 = 1u64 << CPU_FEATURE_PMP;
pub const LOONGARCH_CPU_SCALEFREQ: u64 = 1u64 << CPU_FEATURE_SCALEFREQ;
pub const LOONGARCH_CPU_FLATMODE: u64 = 1u64 << CPU_FEATURE_FLATMODE;
pub const LOONGARCH_CPU_EIODECODE: u64 = 1u64 << CPU_FEATURE_EIODECODE;
pub const LOONGARCH_CPU_GUESTID: u64 = 1u64 << CPU_FEATURE_GUESTID;
pub const LOONGARCH_CPU_HYPERVISOR: u64 = 1u64 << CPU_FEATURE_HYPERVISOR;
pub const LOONGARCH_CPU_PTW: u64 = 1u64 << CPU_FEATURE_PTW;
pub const LOONGARCH_CPU_LSPW: u64 = 1u64 << CPU_FEATURE_LSPW;
pub const LOONGARCH_CPU_MSGINT: u64 = 1u64 << CPU_FEATURE_MSGINT;
pub const LOONGARCH_CPU_AVECINT: u64 = 1u64 << CPU_FEATURE_AVECINT;
pub const LOONGARCH_CPU_REDIRECTINT: u64 = 1u64 << CPU_FEATURE_REDIRECTINT;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
