// SPDX-License-Identifier: GPL-2.0-only
// Copyright 2023 Google LLC
// Author: Ard Biesheuvel <ardb@google.com>

// C dependency: <linux/types.h>

// C section attribute: __section(".init.rodata.prel64")

/// A signed, volatile 64-bit relative pointer offset.
pub type prel64_t = core::ffi::c_long;

/// C equivalent of `PREL64(type, name)`.
#[macro_export]
macro_rules! PREL64 {
    ($type:ty, $name:ident) => {
        union $name {
            pub $name: *mut $type,
            pub prel: $crate::prel64_t,
        }
    };
}

/// C equivalent of `prel64_pointer(__d)`.
#[macro_export]
macro_rules! prel64_pointer {
    ($d:expr) => {
        unsafe { $crate::prel64_to_pointer(&$d) }
    };
}

#[inline]
pub unsafe fn prel64_to_pointer(offset: *const prel64_t) -> *mut core::ffi::c_void {
    let value = core::ptr::read_volatile(offset);
    if value == 0 {
        core::ptr::null_mut()
    } else {
        (offset as *mut u8).offset(value as isize) as *mut core::ffi::c_void
    }
}

extern "C" {
    pub static mut dynamic_scs_is_enabled: bool;

    pub static mut init_idmap_pg_dir: [pgd_t; 0];
    pub static mut init_idmap_pg_end: [pgd_t; 0];
    pub static mut init_pg_dir: [pgd_t; 0];
    pub static mut init_pg_end: [pgd_t; 0];

    pub fn init_feature_override(boot_status: u64, fdt: *const core::ffi::c_void, chosen: i32);
    pub fn kaslr_early_init(fdt: *mut core::ffi::c_void, chosen: i32) -> u64;
    pub fn relocate_kernel(offset: u64);
    pub fn scs_patch(eh_frame: *const u8, size: i32, skip_dry_run: bool) -> i32;

    pub fn map_range(
        pte: *mut phys_addr_t,
        start: u64,
        end: u64,
        pa: phys_addr_t,
        prot: pgprot_t,
        level: i32,
        tbl: *mut pte_t,
        may_use_cont: bool,
        va_offset: u64,
    );

    // `asmlinkage` is a C calling-convention attribute with no direct Rust spelling.
    pub fn early_map_kernel(boot_status: u64, fdt: phys_addr_t);
    pub fn create_init_idmap(pgd: *mut pgd_t, clrmask: ptval_t) -> phys_addr_t;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
