/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2025 Rivos Inc.
 *
 * Authors:
 *     Clément Léger <cleger@rivosinc.com>
 */

// Translated from kvm_vcpu_sbi_fwft.h. The asm/sbi.h dependency is supplied
// externally by the surrounding translation unit.

use core::ffi::c_ulong;

pub struct KvmSbiFwftFeature;

#[repr(C)]
pub struct KvmSbiFwftConfig {
    pub feature: *const KvmSbiFwftFeature,
    pub supported: bool,
    pub enabled: bool,
    pub flags: c_ulong,
}

/* FWFT data structure per vcpu */
#[repr(C)]
pub struct KvmSbiFwft {
    pub configs: *mut KvmSbiFwftConfig,
    // Corresponds to !CONFIG_32BIT in the C header; the build system may
    // provide a different configuration condition than Rust's target width.
    #[cfg(not(target_pointer_width = "32"))]
    pub have_vs_pmlen_7: bool,
    #[cfg(not(target_pointer_width = "32"))]
    pub have_vs_pmlen_16: bool,
}

// C macro: (&(vcpu)->arch.fwft_context)
#[macro_export]
macro_rules! vcpu_to_fwft {
    ($vcpu:expr) => {{
        unsafe { &mut (*($vcpu)).arch.fwft_context }
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
