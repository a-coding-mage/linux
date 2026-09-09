/* SPDX-License-Identifier: GPL-2.0+ OR BSD-3-Clause */
/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 * All rights reserved.
 *
 * This source code is licensed under both the BSD-style license (found in the
 * LICENSE file in the root directory of this source tree) and the GPLv2 (found
 * in the COPYING file in the root directory of this source tree).
 * You may select, at your option, one of the above-listed licenses.
 */

/*
 * Implementation taken from folly/CpuId.h
 * https://github.com/facebook/folly/blob/master/folly/CpuId.h
 */

/* C dependency: U32 and MEM_STATIC are supplied by mem.h. */

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ZSTD_cpuid_t {
    pub f1c: U32,
    pub f1d: U32,
    pub f7b: U32,
    pub f7c: U32,
}

pub unsafe fn ZSTD_cpuid() -> ZSTD_cpuid_t {
    let mut f1c: U32 = 0;
    let mut f1d: U32 = 0;
    let mut f7b: U32 = 0;
    let mut f7c: U32 = 0;
    /* The original implementation has compiler-specific 32-bit PIC assembly.
     * Preserve that condition and intent; the inline assembly is expressed
     * using Rust's architecture-specific cpuid instruction below. */
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    {
        let r0 = core::arch::x86_64::__cpuid(0);
        let n = r0.eax;
        if n >= 1 {
            let r1 = core::arch::x86_64::__cpuid(1);
            f1c = r1.ecx;
            f1d = r1.edx;
        }
        if n >= 7 {
            let r7 = core::arch::x86_64::__cpuid_count(7, 0);
            f7b = r7.ebx;
            f7c = r7.ecx;
        }
    }
    ZSTD_cpuid_t { f1c, f1d, f7b, f7c }
}

macro_rules! zstd_cpuid_feature {
    ($name:ident, $field:ident, $bit:expr) => {
        pub fn $name(cpuid: ZSTD_cpuid_t) -> i32 {
            ((cpuid.$field & (1u32 << $bit)) != 0) as i32
        }
    };
}

/* cpuid(1): Processor Info and Feature Bits. */
zstd_cpuid_feature!(ZSTD_cpuid_sse3, f1c, 0);
zstd_cpuid_feature!(ZSTD_cpuid_pclmuldq, f1c, 1);
zstd_cpuid_feature!(ZSTD_cpuid_dtes64, f1c, 2);
zstd_cpuid_feature!(ZSTD_cpuid_monitor, f1c, 3);
zstd_cpuid_feature!(ZSTD_cpuid_dscpl, f1c, 4);
zstd_cpuid_feature!(ZSTD_cpuid_vmx, f1c, 5);
zstd_cpuid_feature!(ZSTD_cpuid_smx, f1c, 6);
zstd_cpuid_feature!(ZSTD_cpuid_eist, f1c, 7);
zstd_cpuid_feature!(ZSTD_cpuid_tm2, f1c, 8);
zstd_cpuid_feature!(ZSTD_cpuid_ssse3, f1c, 9);
zstd_cpuid_feature!(ZSTD_cpuid_cnxtid, f1c, 10);
zstd_cpuid_feature!(ZSTD_cpuid_fma, f1c, 12);
zstd_cpuid_feature!(ZSTD_cpuid_cx16, f1c, 13);
zstd_cpuid_feature!(ZSTD_cpuid_xtpr, f1c, 14);
zstd_cpuid_feature!(ZSTD_cpuid_pdcm, f1c, 15);
zstd_cpuid_feature!(ZSTD_cpuid_pcid, f1c, 17);
zstd_cpuid_feature!(ZSTD_cpuid_dca, f1c, 18);
zstd_cpuid_feature!(ZSTD_cpuid_sse41, f1c, 19);
zstd_cpuid_feature!(ZSTD_cpuid_sse42, f1c, 20);
zstd_cpuid_feature!(ZSTD_cpuid_x2apic, f1c, 21);
zstd_cpuid_feature!(ZSTD_cpuid_movbe, f1c, 22);
zstd_cpuid_feature!(ZSTD_cpuid_popcnt, f1c, 23);
zstd_cpuid_feature!(ZSTD_cpuid_tscdeadline, f1c, 24);
zstd_cpuid_feature!(ZSTD_cpuid_aes, f1c, 25);
zstd_cpuid_feature!(ZSTD_cpuid_xsave, f1c, 26);
zstd_cpuid_feature!(ZSTD_cpuid_osxsave, f1c, 27);
zstd_cpuid_feature!(ZSTD_cpuid_avx, f1c, 28);
zstd_cpuid_feature!(ZSTD_cpuid_f16c, f1c, 29);
zstd_cpuid_feature!(ZSTD_cpuid_rdrand, f1c, 30);

/* cpuid(1): Processor Info and Feature Bits. */
zstd_cpuid_feature!(ZSTD_cpuid_fpu, f1d, 0);
zstd_cpuid_feature!(ZSTD_cpuid_vme, f1d, 1);
zstd_cpuid_feature!(ZSTD_cpuid_de, f1d, 2);
zstd_cpuid_feature!(ZSTD_cpuid_pse, f1d, 3);
zstd_cpuid_feature!(ZSTD_cpuid_tsc, f1d, 4);
zstd_cpuid_feature!(ZSTD_cpuid_msr, f1d, 5);
zstd_cpuid_feature!(ZSTD_cpuid_pae, f1d, 6);
zstd_cpuid_feature!(ZSTD_cpuid_mce, f1d, 7);
zstd_cpuid_feature!(ZSTD_cpuid_cx8, f1d, 8);
zstd_cpuid_feature!(ZSTD_cpuid_apic, f1d, 9);
zstd_cpuid_feature!(ZSTD_cpuid_sep, f1d, 11);
zstd_cpuid_feature!(ZSTD_cpuid_mtrr, f1d, 12);
zstd_cpuid_feature!(ZSTD_cpuid_pge, f1d, 13);
zstd_cpuid_feature!(ZSTD_cpuid_mca, f1d, 14);
zstd_cpuid_feature!(ZSTD_cpuid_cmov, f1d, 15);
zstd_cpuid_feature!(ZSTD_cpuid_pat, f1d, 16);
zstd_cpuid_feature!(ZSTD_cpuid_pse36, f1d, 17);
zstd_cpuid_feature!(ZSTD_cpuid_psn, f1d, 18);
zstd_cpuid_feature!(ZSTD_cpuid_clfsh, f1d, 19);
zstd_cpuid_feature!(ZSTD_cpuid_ds, f1d, 21);
zstd_cpuid_feature!(ZSTD_cpuid_acpi, f1d, 22);
zstd_cpuid_feature!(ZSTD_cpuid_mmx, f1d, 23);
zstd_cpuid_feature!(ZSTD_cpuid_fxsr, f1d, 24);
zstd_cpuid_feature!(ZSTD_cpuid_sse, f1d, 25);
zstd_cpuid_feature!(ZSTD_cpuid_sse2, f1d, 26);
zstd_cpuid_feature!(ZSTD_cpuid_ss, f1d, 27);
zstd_cpuid_feature!(ZSTD_cpuid_htt, f1d, 28);
zstd_cpuid_feature!(ZSTD_cpuid_tm, f1d, 29);
zstd_cpuid_feature!(ZSTD_cpuid_pbe, f1d, 31);

/* cpuid(7): Extended Features. */
zstd_cpuid_feature!(ZSTD_cpuid_bmi1, f7b, 3);
zstd_cpuid_feature!(ZSTD_cpuid_hle, f7b, 4);
zstd_cpuid_feature!(ZSTD_cpuid_avx2, f7b, 5);
zstd_cpuid_feature!(ZSTD_cpuid_smep, f7b, 7);
zstd_cpuid_feature!(ZSTD_cpuid_bmi2, f7b, 8);
zstd_cpuid_feature!(ZSTD_cpuid_erms, f7b, 9);
zstd_cpuid_feature!(ZSTD_cpuid_invpcid, f7b, 10);
zstd_cpuid_feature!(ZSTD_cpuid_rtm, f7b, 11);
zstd_cpuid_feature!(ZSTD_cpuid_mpx, f7b, 14);
zstd_cpuid_feature!(ZSTD_cpuid_avx512f, f7b, 16);
zstd_cpuid_feature!(ZSTD_cpuid_avx512dq, f7b, 17);
zstd_cpuid_feature!(ZSTD_cpuid_rdseed, f7b, 18);
zstd_cpuid_feature!(ZSTD_cpuid_adx, f7b, 19);
zstd_cpuid_feature!(ZSTD_cpuid_smap, f7b, 20);
zstd_cpuid_feature!(ZSTD_cpuid_avx512ifma, f7b, 21);
zstd_cpuid_feature!(ZSTD_cpuid_pcommit, f7b, 22);
zstd_cpuid_feature!(ZSTD_cpuid_clflushopt, f7b, 23);
zstd_cpuid_feature!(ZSTD_cpuid_clwb, f7b, 24);
zstd_cpuid_feature!(ZSTD_cpuid_avx512pf, f7b, 26);
zstd_cpuid_feature!(ZSTD_cpuid_avx512er, f7b, 27);
zstd_cpuid_feature!(ZSTD_cpuid_avx512cd, f7b, 28);
zstd_cpuid_feature!(ZSTD_cpuid_sha, f7b, 29);
zstd_cpuid_feature!(ZSTD_cpuid_avx512bw, f7b, 30);
zstd_cpuid_feature!(ZSTD_cpuid_avx512vl, f7b, 31);
zstd_cpuid_feature!(ZSTD_cpuid_prefetchwt1, f7c, 0);
zstd_cpuid_feature!(ZSTD_cpuid_avx512vbmi, f7c, 1);


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
