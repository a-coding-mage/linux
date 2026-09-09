/* SPDX-License-Identifier: GPL-2.0 */

// The C header guard `_LINUX_EXTABLE_H` is omitted in Rust; module loading
// provides the equivalent single-definition behavior.

// Dependencies supplied by the surrounding kernel translation:
// `size_t`, `unsigned long`, and `NULL` are represented by Rust equivalents.

#[repr(C)]
pub struct module {
    _private: [u8; 0],
}

#[repr(C)]
pub struct exception_table_entry {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn search_extable(
        base: *const exception_table_entry,
        num: usize,
        value: ::core::ffi::c_ulong,
    ) -> *const exception_table_entry;

    pub fn sort_extable(
        start: *mut exception_table_entry,
        finish: *mut exception_table_entry,
    );

    pub fn sort_main_extable();

    pub fn trim_init_extable(m: *mut module);

    /* Given an address, look for it in the exception tables */
    pub fn search_exception_tables(add: ::core::ffi::c_ulong)
        -> *const exception_table_entry;

    pub fn search_kernel_exception_table(addr: ::core::ffi::c_ulong)
        -> *const exception_table_entry;
}

// For extable.c to search modules' exception tables.
// The CONFIG_MODULES condition is preserved as a Rust cfg feature.
#[cfg(feature = "CONFIG_MODULES")]
unsafe extern "C" {
    pub fn search_module_extables(addr: ::core::ffi::c_ulong)
        -> *const exception_table_entry;
}

#[cfg(not(feature = "CONFIG_MODULES"))]
#[inline]
pub unsafe fn search_module_extables(
    _addr: ::core::ffi::c_ulong,
) -> *const exception_table_entry {
    core::ptr::null()
}

#[cfg(feature = "CONFIG_BPF_JIT")]
unsafe extern "C" {
    pub fn search_bpf_extables(addr: ::core::ffi::c_ulong)
        -> *const exception_table_entry;
}

#[cfg(not(feature = "CONFIG_BPF_JIT"))]
#[inline]
pub unsafe fn search_bpf_extables(
    _addr: ::core::ffi::c_ulong,
) -> *const exception_table_entry {
    core::ptr::null()
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
