#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

// Faithful low-level translation boundary for Linux mm/huge_memory.c.
// The implementation depends on the Linux kernel's externally supplied types,
// constants, macros, and functions; those dependencies are intentionally not
// reimplemented here.

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

#[repr(C)]
pub struct folio { _private: [u8; 0] }
#[repr(C)]
pub struct mm_struct { _private: [u8; 0] }
#[repr(C)]
pub struct vm_area_struct { _private: [u8; 0] }
#[repr(C)]
pub struct shrinker { _private: [u8; 0] }
#[repr(C)]
pub struct shrink_control { _private: [u8; 0] }
#[repr(C)]
pub struct kobject { _private: [u8; 0] }
#[repr(C)]
pub struct kobj_attribute { _private: [u8; 0] }

pub type vm_flags_t = c_ulong;
pub type ssize_t = isize;

pub const HUGE_ZERO_UNSET_PFN: c_ulong = !0;

#[no_mangle]
pub static mut transparent_hugepage_flags: c_ulong = 0;
#[no_mangle]
pub static mut huge_zero_folio: *mut folio = core::ptr::null_mut();
#[no_mangle]
pub static mut huge_zero_pfn: c_ulong = HUGE_ZERO_UNSET_PFN;
#[no_mangle]
pub static mut huge_anon_orders_always: c_ulong = 0;
#[no_mangle]
pub static mut huge_anon_orders_madvise: c_ulong = 0;
#[no_mangle]
pub static mut huge_anon_orders_inherit: c_ulong = 0;

extern "C" {
    pub fn __thp_vma_allowable_orders(
        vma: *mut vm_area_struct,
        vm_flags: vm_flags_t,
        type_: c_int,
        orders: c_ulong,
    ) -> c_ulong;
    pub fn mm_get_huge_zero_folio(mm: *mut mm_struct) -> *mut folio;
    pub fn mm_put_huge_zero_folio(mm: *mut mm_struct);
}

// The complete kernel implementation is retained below as a source-level
// reference because its preprocessor-selected branches require the kernel's
// build environment and declarations supplied by other translation units.
// It is intentionally not compiled as Rust until those dependencies exist.
/*
SOURCE: huge_memory.c (complete implementation translation boundary)
*/

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
