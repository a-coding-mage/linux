/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright 2024 Rivos, Inc
 */

// Dependency declarations supplied by the surrounding kernel translation.
// The C header's build-time IS_ENABLED(CONFIG_...) conditions are represented
// by the corresponding Rust feature names below.

pub const RISCV_ISA_VENDOR_EXT_MAX: usize = 32;

#[repr(C)]
pub struct riscv_isavendorinfo {
    pub isa: [core::ffi::c_ulong; 1],
}

#[repr(C)]
pub struct riscv_isa_vendor_ext_data_list {
    pub is_initialized: bool,
    pub ext_data_count: usize,
    pub ext_data: *const riscv_isa_ext_data,
    pub per_hart_isa_bitmap: [riscv_isavendorinfo; NR_CPUS],
    pub all_harts_isa_bitmap: riscv_isavendorinfo,
}

extern "C" {
    pub static mut riscv_isa_vendor_ext_list:
        *mut *mut riscv_isa_vendor_ext_data_list;
    pub static riscv_isa_vendor_ext_list_size: usize;

    pub fn __riscv_isa_vendor_extension_available(
        cpu: core::ffi::c_int,
        vendor: core::ffi::c_ulong,
        bit: core::ffi::c_uint,
    ) -> bool;
    pub fn __riscv_has_extension_likely(
        vendor: core::ffi::c_ulong,
        ext: core::ffi::c_ulong,
    ) -> bool;
    pub fn __riscv_has_extension_unlikely(
        vendor: core::ffi::c_ulong,
        ext: core::ffi::c_ulong,
    ) -> bool;
}

// Supplied by the surrounding translation of asm/cpufeature.h and linux/types.h.
extern "Rust" {
    static NR_CPUS: usize;
}

pub const RISCV_VENDOR_EXT_ALTERNATIVES_BASE: core::ffi::c_ulong = 0x8000;
pub const VENDOR_EXT_ALL_CPUS: core::ffi::c_int = -1;

pub unsafe fn riscv_has_vendor_extension_likely(
    vendor: core::ffi::c_ulong,
    ext: core::ffi::c_ulong,
) -> bool {
    if !cfg!(feature = "CONFIG_RISCV_ISA_VENDOR_EXT") {
        return false;
    }
    if cfg!(feature = "CONFIG_RISCV_ALTERNATIVE") {
        return __riscv_has_extension_likely(
            vendor,
            ext.wrapping_add(RISCV_VENDOR_EXT_ALTERNATIVES_BASE),
        );
    }
    __riscv_isa_vendor_extension_available(VENDOR_EXT_ALL_CPUS, vendor, ext as core::ffi::c_uint)
}

pub unsafe fn riscv_has_vendor_extension_unlikely(
    vendor: core::ffi::c_ulong,
    ext: core::ffi::c_ulong,
) -> bool {
    if !cfg!(feature = "CONFIG_RISCV_ISA_VENDOR_EXT") {
        return false;
    }
    if cfg!(feature = "CONFIG_RISCV_ALTERNATIVE") {
        return __riscv_has_extension_unlikely(
            vendor,
            ext.wrapping_add(RISCV_VENDOR_EXT_ALTERNATIVES_BASE),
        );
    }
    __riscv_isa_vendor_extension_available(VENDOR_EXT_ALL_CPUS, vendor, ext as core::ffi::c_uint)
}

pub unsafe fn riscv_cpu_has_vendor_extension_likely(
    vendor: core::ffi::c_ulong,
    cpu: core::ffi::c_int,
    ext: core::ffi::c_ulong,
) -> bool {
    if !cfg!(feature = "CONFIG_RISCV_ISA_VENDOR_EXT") {
        return false;
    }
    if cfg!(feature = "CONFIG_RISCV_ALTERNATIVE")
        && __riscv_has_extension_likely(
            vendor,
            ext.wrapping_add(RISCV_VENDOR_EXT_ALTERNATIVES_BASE),
        )
    {
        return true;
    }
    __riscv_isa_vendor_extension_available(cpu, vendor, ext as core::ffi::c_uint)
}

pub unsafe fn riscv_cpu_has_vendor_extension_unlikely(
    vendor: core::ffi::c_ulong,
    cpu: core::ffi::c_int,
    ext: core::ffi::c_ulong,
) -> bool {
    if !cfg!(feature = "CONFIG_RISCV_ISA_VENDOR_EXT") {
        return false;
    }
    if cfg!(feature = "CONFIG_RISCV_ALTERNATIVE")
        && __riscv_has_extension_unlikely(
            vendor,
            ext.wrapping_add(RISCV_VENDOR_EXT_ALTERNATIVES_BASE),
        )
    {
        return true;
    }
    __riscv_isa_vendor_extension_available(cpu, vendor, ext as core::ffi::c_uint)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
