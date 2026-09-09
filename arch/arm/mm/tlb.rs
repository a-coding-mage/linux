// SPDX-License-Identifier: GPL-2.0-only
// Copyright 2024 Google LLC
// Author: Ard Biesheuvel <ardb@google.com>

// C dependencies supplied by the surrounding kernel translation unit.
use core::ffi::c_ulong;

#[repr(C)]
pub struct vm_area_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct cpu_tlb_fns {
    pub flush_user_range: Option<unsafe extern "C" fn(c_ulong, c_ulong, *mut vm_area_struct)>,
    pub flush_kern_range: Option<unsafe extern "C" fn(c_ulong, c_ulong)>,
    pub tlb_flags: c_ulong,
}

#[cfg(feature = "CONFIG_CPU_TLB_V4WT")]
unsafe extern "C" {
    fn v4_flush_user_tlb_range(start: c_ulong, end: c_ulong, vma: *mut vm_area_struct);
    fn v4_flush_kern_tlb_range(start: c_ulong, end: c_ulong);
    static v4_tlb_flags: c_ulong;
}

#[cfg(feature = "CONFIG_CPU_TLB_V4WT")]
#[no_mangle]
pub static v4_tlb_fns: cpu_tlb_fns = cpu_tlb_fns {
    flush_user_range: Some(v4_flush_user_tlb_range),
    flush_kern_range: Some(v4_flush_kern_tlb_range),
    tlb_flags: unsafe { v4_tlb_flags },
};

#[cfg(feature = "CONFIG_CPU_TLB_V4WB")]
unsafe extern "C" {
    fn v4wb_flush_user_tlb_range(start: c_ulong, end: c_ulong, vma: *mut vm_area_struct);
    fn v4wb_flush_kern_tlb_range(start: c_ulong, end: c_ulong);
    static v4wb_tlb_flags: c_ulong;
}

#[cfg(feature = "CONFIG_CPU_TLB_V4WB")]
#[no_mangle]
pub static v4wb_tlb_fns: cpu_tlb_fns = cpu_tlb_fns {
    flush_user_range: Some(v4wb_flush_user_tlb_range),
    flush_kern_range: Some(v4wb_flush_kern_tlb_range),
    tlb_flags: unsafe { v4wb_tlb_flags },
};

#[cfg(any(feature = "CONFIG_CPU_TLB_V4WBI", feature = "CONFIG_CPU_TLB_FEROCEON"))]
unsafe extern "C" {
    fn v4wbi_flush_user_tlb_range(start: c_ulong, end: c_ulong, vma: *mut vm_area_struct);
    fn v4wbi_flush_kern_tlb_range(start: c_ulong, end: c_ulong);
    static v4wbi_tlb_flags: c_ulong;
}

#[cfg(any(feature = "CONFIG_CPU_TLB_V4WBI", feature = "CONFIG_CPU_TLB_FEROCEON"))]
#[no_mangle]
pub static v4wbi_tlb_fns: cpu_tlb_fns = cpu_tlb_fns {
    flush_user_range: Some(v4wbi_flush_user_tlb_range),
    flush_kern_range: Some(v4wbi_flush_kern_tlb_range),
    tlb_flags: unsafe { v4wbi_tlb_flags },
};

#[cfg(feature = "CONFIG_CPU_TLB_V6")]
unsafe extern "C" {
    fn v6wbi_flush_user_tlb_range(start: c_ulong, end: c_ulong, vma: *mut vm_area_struct);
    fn v6wbi_flush_kern_tlb_range(start: c_ulong, end: c_ulong);
    static v6wbi_tlb_flags: c_ulong;
}

#[cfg(feature = "CONFIG_CPU_TLB_V6")]
#[no_mangle]
pub static v6wbi_tlb_fns: cpu_tlb_fns = cpu_tlb_fns {
    flush_user_range: Some(v6wbi_flush_user_tlb_range),
    flush_kern_range: Some(v6wbi_flush_kern_tlb_range),
    tlb_flags: unsafe { v6wbi_tlb_flags },
};

#[cfg(feature = "CONFIG_CPU_TLB_V7")]
unsafe extern "C" {
    fn v7wbi_flush_user_tlb_range(start: c_ulong, end: c_ulong, vma: *mut vm_area_struct);
    fn v7wbi_flush_kern_tlb_range(start: c_ulong, end: c_ulong);
    static v7wbi_tlb_flags_smp: c_ulong;
    static v7wbi_tlb_flags_up: c_ulong;
}

#[cfg(feature = "CONFIG_CPU_TLB_V7")]
#[no_mangle]
pub static v7wbi_tlb_fns: cpu_tlb_fns = cpu_tlb_fns {
    flush_user_range: Some(v7wbi_flush_user_tlb_range),
    flush_kern_range: Some(v7wbi_flush_kern_tlb_range),
    // C: IS_ENABLED(CONFIG_SMP) ? v7wbi_tlb_flags_smp : v7wbi_tlb_flags_up
    tlb_flags: unsafe {
        if cfg!(feature = "CONFIG_SMP") {
            v7wbi_tlb_flags_smp
        } else {
            v7wbi_tlb_flags_up
        }
    },
};

// CONFIG_SMP_ON_UP: the C implementation emits an alternative-patching
// assembly record for v7wbi_tlb_fns.tlb_flags at offset 8.

#[cfg(feature = "CONFIG_CPU_TLB_FA")]
unsafe extern "C" {
    fn fa_flush_user_tlb_range(start: c_ulong, end: c_ulong, vma: *mut vm_area_struct);
    fn fa_flush_kern_tlb_range(start: c_ulong, end: c_ulong);
    static fa_tlb_flags: c_ulong;
}

#[cfg(feature = "CONFIG_CPU_TLB_FA")]
#[no_mangle]
pub static fa_tlb_fns: cpu_tlb_fns = cpu_tlb_fns {
    flush_user_range: Some(fa_flush_user_tlb_range),
    flush_kern_range: Some(fa_flush_kern_tlb_range),
    tlb_flags: unsafe { fa_tlb_flags },
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
