/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2020-2022 Loongson Technology Corporation Limited
 *
 * Derived from MIPS:
 * Copyright (C) 2003, 2004 Ralf Baechle
 * Copyright (C) 2004  Maciej W. Rozycki
 */

// C dependencies: <asm/cpu.h> and <asm/cpu-info.h>

macro_rules! cpu_opt {
    ($opt:expr) => {
        cpu_data[0].options & ($opt)
    };
}

macro_rules! cpu_has {
    ($feat:expr) => {
        cpu_data[0].options & BIT_ULL($feat)
    };
}

macro_rules! cpu_has_loongarch {
    () => {
        cpu_has_loongarch32!() | cpu_has_loongarch64!()
    };
}

macro_rules! cpu_has_loongarch32 {
    () => {
        cpu_data[0].isa_level & LOONGARCH_CPU_ISA_32BIT
    };
}

macro_rules! cpu_has_loongarch64 {
    () => {
        cpu_data[0].isa_level & LOONGARCH_CPU_ISA_64BIT
    };
}

#[cfg(CONFIG_32BIT)]
macro_rules! cpu_vabits {
    () => { 31 };
}

#[cfg(CONFIG_32BIT)]
macro_rules! cpu_pabits {
    () => { 31 };
}

#[cfg(CONFIG_64BIT)]
macro_rules! cpu_vabits {
    () => { cpu_data[0].vabits };
}

#[cfg(CONFIG_64BIT)]
macro_rules! cpu_pabits {
    () => { cpu_data[0].pabits };
}

/*
 * SMP assumption: Options of CPU 0 are a superset of all processors.
 * This is true for all known LoongArch systems.
 */
macro_rules! cpu_has_cpucfg { () => { cpu_opt!(LOONGARCH_CPU_CPUCFG) }; }
macro_rules! cpu_has_lam { () => { cpu_opt!(LOONGARCH_CPU_LAM) }; }
macro_rules! cpu_has_lam_bh { () => { cpu_opt!(LOONGARCH_CPU_LAM_BH) }; }
macro_rules! cpu_has_scq { () => { cpu_opt!(LOONGARCH_CPU_SCQ) }; }
macro_rules! cpu_has_ual { () => { cpu_opt!(LOONGARCH_CPU_UAL) }; }
macro_rules! cpu_has_fpu { () => { cpu_opt!(LOONGARCH_CPU_FPU) }; }
macro_rules! cpu_has_lsx { () => { cpu_opt!(LOONGARCH_CPU_LSX) }; }
macro_rules! cpu_has_lasx { () => { cpu_opt!(LOONGARCH_CPU_LASX) }; }
macro_rules! cpu_has_crc32 { () => { cpu_opt!(LOONGARCH_CPU_CRC32) }; }
macro_rules! cpu_has_complex { () => { cpu_opt!(LOONGARCH_CPU_COMPLEX) }; }
macro_rules! cpu_has_crypto { () => { cpu_opt!(LOONGARCH_CPU_CRYPTO) }; }
macro_rules! cpu_has_lvz { () => { cpu_opt!(LOONGARCH_CPU_LVZ) }; }
macro_rules! cpu_has_lbt_x86 { () => { cpu_opt!(LOONGARCH_CPU_LBT_X86) }; }
macro_rules! cpu_has_lbt_arm { () => { cpu_opt!(LOONGARCH_CPU_LBT_ARM) }; }
macro_rules! cpu_has_lbt_mips { () => { cpu_opt!(LOONGARCH_CPU_LBT_MIPS) }; }
macro_rules! cpu_has_lbt { () => { cpu_has_lbt_x86!() | cpu_has_lbt_arm!() | cpu_has_lbt_mips!() }; }
macro_rules! cpu_has_csr { () => { cpu_opt!(LOONGARCH_CPU_CSR) }; }
macro_rules! cpu_has_iocsr { () => { cpu_opt!(LOONGARCH_CPU_IOCSR) }; }
macro_rules! cpu_has_tlb { () => { cpu_opt!(LOONGARCH_CPU_TLB) }; }
macro_rules! cpu_has_watch { () => { cpu_opt!(LOONGARCH_CPU_WATCH) }; }
macro_rules! cpu_has_vint { () => { cpu_opt!(LOONGARCH_CPU_VINT) }; }
macro_rules! cpu_has_csripi { () => { cpu_opt!(LOONGARCH_CPU_CSRIPI) }; }
macro_rules! cpu_has_extioi { () => { cpu_opt!(LOONGARCH_CPU_EXTIOI) }; }
macro_rules! cpu_has_prefetch { () => { cpu_opt!(LOONGARCH_CPU_PREFETCH) }; }
macro_rules! cpu_has_pmp { () => { cpu_opt!(LOONGARCH_CPU_PMP) }; }
macro_rules! cpu_has_perf { () => { cpu_opt!(LOONGARCH_CPU_PMP) }; }
macro_rules! cpu_has_scalefreq { () => { cpu_opt!(LOONGARCH_CPU_SCALEFREQ) }; }
macro_rules! cpu_has_flatmode { () => { cpu_opt!(LOONGARCH_CPU_FLATMODE) }; }
macro_rules! cpu_has_eiodecode { () => { cpu_opt!(LOONGARCH_CPU_EIODECODE) }; }
macro_rules! cpu_has_guestid { () => { cpu_opt!(LOONGARCH_CPU_GUESTID) }; }
macro_rules! cpu_has_hypervisor { () => { cpu_opt!(LOONGARCH_CPU_HYPERVISOR) }; }
macro_rules! cpu_has_ptw { () => { cpu_opt!(LOONGARCH_CPU_PTW) }; }
macro_rules! cpu_has_lspw { () => { cpu_opt!(LOONGARCH_CPU_LSPW) }; }
macro_rules! cpu_has_msgint { () => { cpu_opt!(LOONGARCH_CPU_MSGINT) }; }
macro_rules! cpu_has_avecint { () => { cpu_opt!(LOONGARCH_CPU_AVECINT) }; }
macro_rules! cpu_has_redirectint { () => { cpu_opt!(LOONGARCH_CPU_REDIRECTINT) }; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
