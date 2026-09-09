/* SPDX-License-Identifier: GPL-2.0 */

// Translated from linux/buildid.h. The original include supplies the kernel
// integer and opaque type definitions used by this interface.

pub const BUILD_ID_SIZE_MAX: usize = 20;

#[repr(C)]
pub struct vm_area_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct file {
    _private: [u8; 0],
}

#[repr(C)]
pub struct folio {
    _private: [u8; 0],
}

pub type loff_t = i64;

extern "C" {
    pub fn build_id_parse(
        vma: *mut vm_area_struct,
        build_id: *mut u8,
        size: *mut u32,
    ) -> i32;
    pub fn build_id_parse_file(
        file: *mut file,
        build_id: *mut u8,
        size: *mut u32,
    ) -> i32;
    pub fn build_id_parse_nofault(
        vma: *mut vm_area_struct,
        build_id: *mut u8,
        size: *mut u32,
    ) -> i32;
    pub fn build_id_parse_buf(
        buf: *const core::ffi::c_void,
        build_id: *mut u8,
        buf_size: u32,
    ) -> i32;
}

// The following declarations are enabled when CONFIG_STACKTRACE_BUILD_ID or
// CONFIG_VMCORE_INFO is enabled in the kernel build.
#[cfg(any(feature = "CONFIG_STACKTRACE_BUILD_ID", feature = "CONFIG_VMCORE_INFO"))]
extern "C" {
    pub static mut vmlinux_build_id: [u8; BUILD_ID_SIZE_MAX];
    pub fn init_vmlinux_build_id();
}

// When neither configuration option is enabled, the C header provides an
// empty inline function.
#[cfg(not(any(feature = "CONFIG_STACKTRACE_BUILD_ID", feature = "CONFIG_VMCORE_INFO")))]
#[inline]
pub unsafe fn init_vmlinux_build_id() {}

#[repr(C)]
pub struct freader_file {
    pub file: *mut file,
    pub folio: *mut folio,
    pub addr: *mut core::ffi::c_void,
    pub folio_off: loff_t,
    pub may_fault: bool,
}

#[repr(C)]
pub struct freader_mem {
    pub data: *const core::ffi::c_char,
    pub data_sz: u64,
}

#[repr(C)]
pub union freader_source {
    pub file: freader_file,
    pub mem: freader_mem,
}

#[repr(C)]
pub struct freader {
    pub buf: *mut core::ffi::c_void,
    pub buf_sz: u32,
    pub err: i32,
    pub source: freader_source,
}

extern "C" {
    pub fn freader_init_from_file(
        r: *mut freader,
        buf: *mut core::ffi::c_void,
        buf_sz: u32,
        file: *mut file,
        may_fault: bool,
    );
    pub fn freader_init_from_mem(
        r: *mut freader,
        data: *const core::ffi::c_char,
        data_sz: u64,
    );
    pub fn freader_fetch(
        r: *mut freader,
        file_off: loff_t,
        sz: usize,
    ) -> *const core::ffi::c_void;
    pub fn freader_cleanup(r: *mut freader);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
