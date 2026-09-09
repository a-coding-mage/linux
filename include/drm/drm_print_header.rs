/* Rust translation of drm_print.h. C includes and build-time configuration are external dependencies. */

use core::ffi::{c_char, c_int, c_ulong, c_void};

#[repr(C)]
pub struct debugfs_regset32;
#[repr(C)]
pub struct drm_device {
    pub dev: *mut device,
}
#[repr(C)]
pub struct seq_file;
#[repr(C)]
pub struct device;
#[repr(C)]
pub struct va_format {
    pub fmt: *const c_char,
    pub va: *mut va_list,
}
#[repr(C)]
pub struct va_list;
#[repr(C)]
pub struct _ddebug;

pub type ssize_t = isize;
pub type size_t = usize;
pub type u8 = u8;

extern "C" {
    pub static mut __drm_debug: c_ulong;
    pub fn __drm_printfn_coredump(p: *mut drm_printer, vaf: *mut va_format);
    pub fn __drm_puts_coredump(p: *mut drm_printer, str_: *const c_char);
    pub fn __drm_printfn_seq_file(p: *mut drm_printer, vaf: *mut va_format);
    pub fn __drm_puts_seq_file(p: *mut drm_printer, str_: *const c_char);
    pub fn __drm_printfn_info(p: *mut drm_printer, vaf: *mut va_format);
    pub fn __drm_printfn_dbg(p: *mut drm_printer, vaf: *mut va_format);
    pub fn __drm_printfn_err(p: *mut drm_printer, vaf: *mut va_format);
    pub fn __drm_printfn_line(p: *mut drm_printer, vaf: *mut va_format);
    pub fn drm_printf(p: *mut drm_printer, f: *const c_char, ...);
    pub fn drm_puts(p: *mut drm_printer, str_: *const c_char);
    pub fn drm_print_regset32(p: *mut drm_printer, regset: *mut debugfs_regset32);
    pub fn drm_print_bits(p: *mut drm_printer, value: c_ulong, bits: *const *const c_char, nbits: u32);
    pub fn drm_print_hex_dump(p: *mut drm_printer, prefix: *const c_char, buf: *const u8, len: size_t);
    pub fn drm_dev_printk(dev: *const device, level: *const c_char, format: *const c_char, ...);
    pub fn __drm_dev_dbg(desc: *mut _ddebug, dev: *const device, category: drm_debug_category, format: *const c_char, ...);
    pub fn __drm_err(format: *const c_char, ...);
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum drm_debug_category {
    DRM_UT_CORE,
    DRM_UT_DRIVER,
    DRM_UT_KMS,
    DRM_UT_PRIME,
    DRM_UT_ATOMIC,
    DRM_UT_VBL,
    DRM_UT_STATE,
    DRM_UT_LEASE,
    DRM_UT_DP,
    DRM_UT_DRMRES,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_printer_line {
    pub series: u32,
    pub counter: u32,
}

#[repr(C)]
pub struct drm_printer {
    pub printfn: Option<unsafe extern "C" fn(*mut drm_printer, *mut va_format)>,
    pub puts: Option<unsafe extern "C" fn(*mut drm_printer, *const c_char)>,
    pub arg: *mut c_void,
    pub origin: *const c_void,
    pub prefix: *const c_char,
    pub line: drm_printer_line,
    pub category: drm_debug_category,
}

#[inline]
pub unsafe fn drm_debug_enabled_raw(category: drm_debug_category) -> bool {
    (__drm_debug & (1u64 << (category as u32))) != 0
}

#[inline]
pub unsafe fn drm_debug_enabled_instrumented(category: drm_debug_category) -> bool {
    // C macro emits pr_debug here; the external logging dependency remains unresolved.
    drm_debug_enabled_raw(category)
}

#[repr(C)]
pub struct drm_print_iterator {
    pub data: *mut c_void,
    pub start: ssize_t,
    pub remain: ssize_t,
    pub offset: ssize_t,
}

#[inline]
pub unsafe fn drm_vprintf(p: *mut drm_printer, fmt: *const c_char, va: *mut va_list) {
    let mut vaf = va_format { fmt, va };
    ((*p).printfn.unwrap())(p, &mut vaf);
}

#[inline]
pub unsafe fn drm_coredump_printer(iter: *mut drm_print_iterator) -> drm_printer {
    (*iter).offset = 0;
    drm_printer {
        printfn: Some(__drm_printfn_coredump), puts: Some(__drm_puts_coredump),
        arg: iter.cast(), origin: core::ptr::null(), prefix: core::ptr::null(),
        line: drm_printer_line { series: 0, counter: 0 }, category: drm_debug_category::DRM_UT_CORE,
    }
}

#[inline]
pub unsafe fn drm_coredump_printer_is_full(p: *mut drm_printer) -> bool {
    if (*p).printfn != Some(__drm_printfn_coredump) { return true; }
    (*( (*p).arg as *mut drm_print_iterator)).remain == 0
}

#[inline]
pub unsafe fn drm_seq_file_printer(f: *mut seq_file) -> drm_printer {
    drm_printer { printfn: Some(__drm_printfn_seq_file), puts: Some(__drm_puts_seq_file), arg: f.cast(), origin: core::ptr::null(), prefix: core::ptr::null(), line: drm_printer_line { series: 0, counter: 0 }, category: drm_debug_category::DRM_UT_CORE }
}

#[inline]
pub unsafe fn drm_info_printer(dev: *mut device) -> drm_printer {
    drm_printer { printfn: Some(__drm_printfn_info), puts: None, arg: dev.cast(), origin: core::ptr::null(), prefix: core::ptr::null(), line: drm_printer_line { series: 0, counter: 0 }, category: drm_debug_category::DRM_UT_CORE }
}

#[inline]
pub unsafe fn drm_dbg_printer(drm: *mut drm_device, category: drm_debug_category, prefix: *const c_char) -> drm_printer {
    drm_printer { printfn: Some(__drm_printfn_dbg), puts: None, arg: drm.cast(), origin: core::ptr::null(), prefix, line: drm_printer_line { series: 0, counter: 0 }, category }
}

#[inline]
pub unsafe fn drm_err_printer(drm: *mut drm_device, prefix: *const c_char) -> drm_printer {
    drm_printer { printfn: Some(__drm_printfn_err), puts: None, arg: drm.cast(), origin: core::ptr::null(), prefix, line: drm_printer_line { series: 0, counter: 0 }, category: drm_debug_category::DRM_UT_CORE }
}

#[inline]
pub unsafe fn drm_line_printer(p: *mut drm_printer, prefix: *const c_char, series: u32) -> drm_printer {
    drm_printer { printfn: Some(__drm_printfn_line), puts: None, arg: p.cast(), origin: core::ptr::null(), prefix, line: drm_printer_line { series, counter: 0 }, category: drm_debug_category::DRM_UT_CORE }
}

#[inline]
pub unsafe fn __drm_to_dev(drm: *const drm_device) -> *mut device {
    if drm.is_null() { core::ptr::null_mut() } else { (*drm).dev }
}

// C preprocessor logging interfaces, retained as Rust macro equivalents.
#[macro_export] macro_rules! drm_printf_indent { ($printer:expr, $indent:expr, $fmt:expr $(, $arg:expr)*) => { unsafe { $crate::drm_printf($printer, $fmt $(, $arg)*) } }; }
#[macro_export] macro_rules! drm_dbg { ($drm:expr, $fmt:expr $(, $arg:expr)*) => { $crate::drm_dbg_driver($drm, $fmt $(, $arg)*) }; }
#[macro_export] macro_rules! drm_dbg_core { ($drm:expr, $fmt:expr $(, $arg:expr)*) => { unsafe { $crate::__drm_dev_dbg(core::ptr::null_mut(), $crate::__drm_to_dev($drm), $crate::drm_debug_category::DRM_UT_CORE, $fmt $(, $arg)*) } }; }
#[macro_export] macro_rules! drm_dbg_driver { ($drm:expr, $fmt:expr $(, $arg:expr)*) => { unsafe { $crate::__drm_dev_dbg(core::ptr::null_mut(), $crate::__drm_to_dev($drm), $crate::drm_debug_category::DRM_UT_DRIVER, $fmt $(, $arg)*) } }; }
#[macro_export] macro_rules! drm_dbg_kms { ($drm:expr, $fmt:expr $(, $arg:expr)*) => { unsafe { $crate::__drm_dev_dbg(core::ptr::null_mut(), $crate::__drm_to_dev($drm), $crate::drm_debug_category::DRM_UT_KMS, $fmt $(, $arg)*) } }; }
#[macro_export] macro_rules! drm_dbg_prime { ($drm:expr, $fmt:expr $(, $arg:expr)*) => { unsafe { $crate::__drm_dev_dbg(core::ptr::null_mut(), $crate::__drm_to_dev($drm), $crate::drm_debug_category::DRM_UT_PRIME, $fmt $(, $arg)*) } }; }
#[macro_export] macro_rules! drm_dbg_atomic { ($drm:expr, $fmt:expr $(, $arg:expr)*) => { unsafe { $crate::__drm_dev_dbg(core::ptr::null_mut(), $crate::__drm_to_dev($drm), $crate::drm_debug_category::DRM_UT_ATOMIC, $fmt $(, $arg)*) } }; }
#[macro_export] macro_rules! drm_dbg_vbl { ($drm:expr, $fmt:expr $(, $arg:expr)*) => { unsafe { $crate::__drm_dev_dbg(core::ptr::null_mut(), $crate::__drm_to_dev($drm), $crate::drm_debug_category::DRM_UT_VBL, $fmt $(, $arg)*) } }; }
#[macro_export] macro_rules! drm_dbg_state { ($drm:expr, $fmt:expr $(, $arg:expr)*) => { unsafe { $crate::__drm_dev_dbg(core::ptr::null_mut(), $crate::__drm_to_dev($drm), $crate::drm_debug_category::DRM_UT_STATE, $fmt $(, $arg)*) } }; }
#[macro_export] macro_rules! drm_dbg_lease { ($drm:expr, $fmt:expr $(, $arg:expr)*) => { unsafe { $crate::__drm_dev_dbg(core::ptr::null_mut(), $crate::__drm_to_dev($drm), $crate::drm_debug_category::DRM_UT_LEASE, $fmt $(, $arg)*) } }; }
#[macro_export] macro_rules! drm_dbg_dp { ($drm:expr, $fmt:expr $(, $arg:expr)*) => { unsafe { $crate::__drm_dev_dbg(core::ptr::null_mut(), $crate::__drm_to_dev($drm), $crate::drm_debug_category::DRM_UT_DP, $fmt $(, $arg)*) } }; }
#[macro_export] macro_rules! drm_dbg_drmres { ($drm:expr, $fmt:expr $(, $arg:expr)*) => { unsafe { $crate::__drm_dev_dbg(core::ptr::null_mut(), $crate::__drm_to_dev($drm), $crate::drm_debug_category::DRM_UT_DRMRES, $fmt $(, $arg)*) } }; }
#[macro_export] macro_rules! DRM_DEBUG { ($fmt:expr $(, $arg:expr)*) => { unsafe { $crate::__drm_dev_dbg(core::ptr::null_mut(), core::ptr::null(), $crate::drm_debug_category::DRM_UT_CORE, $fmt $(, $arg)*) } }; }
#[macro_export] macro_rules! DRM_DEBUG_DRIVER { ($fmt:expr $(, $arg:expr)*) => { unsafe { $crate::__drm_dev_dbg(core::ptr::null_mut(), core::ptr::null(), $crate::drm_debug_category::DRM_UT_DRIVER, $fmt $(, $arg)*) } }; }
#[macro_export] macro_rules! DRM_DEBUG_KMS { ($fmt:expr $(, $arg:expr)*) => { unsafe { $crate::__drm_dev_dbg(core::ptr::null_mut(), core::ptr::null(), $crate::drm_debug_category::DRM_UT_KMS, $fmt $(, $arg)*) } }; }
#[macro_export] macro_rules! DRM_DEBUG_PRIME { ($fmt:expr $(, $arg:expr)*) => { unsafe { $crate::__drm_dev_dbg(core::ptr::null_mut(), core::ptr::null(), $crate::drm_debug_category::DRM_UT_PRIME, $fmt $(, $arg)*) } }; }
#[macro_export] macro_rules! DRM_DEBUG_ATOMIC { ($fmt:expr $(, $arg:expr)*) => { unsafe { $crate::__drm_dev_dbg(core::ptr::null_mut(), core::ptr::null(), $crate::drm_debug_category::DRM_UT_ATOMIC, $fmt $(, $arg)*) } }; }
#[macro_export] macro_rules! DRM_DEBUG_VBL { ($fmt:expr $(, $arg:expr)*) => { unsafe { $crate::__drm_dev_dbg(core::ptr::null_mut(), core::ptr::null(), $crate::drm_debug_category::DRM_UT_VBL, $fmt $(, $arg)*) } }; }
#[macro_export] macro_rules! DRM_DEBUG_LEASE { ($fmt:expr $(, $arg:expr)*) => { unsafe { $crate::__drm_dev_dbg(core::ptr::null_mut(), core::ptr::null(), $crate::drm_debug_category::DRM_UT_LEASE, $fmt $(, $arg)*) } }; }
#[macro_export] macro_rules! DRM_DEBUG_DP { ($fmt:expr $(, $arg:expr)*) => { unsafe { $crate::__drm_dev_dbg(core::ptr::null_mut(), core::ptr::null(), $crate::drm_debug_category::DRM_UT_DP, $fmt $(, $arg)*) } }; }
#[macro_export] macro_rules! DRM_ERROR { ($fmt:expr $(, $arg:expr)*) => { unsafe { $crate::__drm_err($fmt $(, $arg)*) } }; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
