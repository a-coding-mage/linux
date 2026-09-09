/* SPDX-License-Identifier: GPL-2.0 */

use core::ffi::c_void;

/* References to section boundaries.  These symbols are supplied by the linker. */
extern "C" {
    pub static _text: u8;
    pub static _stext: u8;
    pub static _etext: u8;
    pub static _data: u8;
    pub static _sdata: u8;
    pub static _edata: u8;
    pub static __bss_start: u8;
    pub static __bss_stop: u8;
    pub static __init_begin: u8;
    pub static __init_end: u8;
    pub static _sinittext: u8;
    pub static _einittext: u8;
    pub static __start_ro_after_init: u8;
    pub static __end_ro_after_init: u8;
    pub static _end: u8;
    pub static __per_cpu_start: u8;
    pub static __per_cpu_end: u8;
    pub static __kprobes_text_start: u8;
    pub static __kprobes_text_end: u8;
    pub static __entry_text_start: u8;
    pub static __entry_text_end: u8;
    pub static __start_rodata: u8;
    pub static __end_rodata: u8;
    pub static __irqentry_text_start: u8;
    pub static __irqentry_text_end: u8;
    pub static __softirqentry_text_start: u8;
    pub static __softirqentry_text_end: u8;
    pub static __start_once: u8;
    pub static __end_once: u8;
    pub static __ctors_start: u8;
    pub static __ctors_end: u8;
    pub static __start_opd: u8;
    pub static __end_opd: u8;
    pub static __noinstr_text_start: u8;
    pub static __noinstr_text_end: u8;
    pub static __nosave_begin: u8;
    pub static __nosave_end: u8;
}

/* Function descriptor handling (if any) is selected by CONFIG_HAVE_FUNCTION_DESCRIPTORS. */
#[cfg(feature = "CONFIG_HAVE_FUNCTION_DESCRIPTORS")]
extern "C" {
    pub fn dereference_function_descriptor(ptr: *mut c_void) -> *mut c_void;
    pub fn dereference_kernel_function_descriptor(ptr: *mut c_void) -> *mut c_void;
}

#[cfg(not(feature = "CONFIG_HAVE_FUNCTION_DESCRIPTORS"))]
#[repr(C)]
pub struct func_desc_t {
    pub addr: usize,
}

#[cfg(not(feature = "CONFIG_HAVE_FUNCTION_DESCRIPTORS"))]
#[inline]
pub unsafe fn dereference_function_descriptor(ptr: *mut c_void) -> *mut c_void {
    ptr
}

#[cfg(not(feature = "CONFIG_HAVE_FUNCTION_DESCRIPTORS"))]
#[inline]
pub unsafe fn dereference_kernel_function_descriptor(ptr: *mut c_void) -> *mut c_void {
    ptr
}

#[inline]
pub const fn have_function_descriptors() -> bool {
    cfg!(feature = "CONFIG_HAVE_FUNCTION_DESCRIPTORS")
}

#[inline]
pub unsafe fn memory_contains(begin: *mut c_void, end: *mut c_void,
                              virt: *mut c_void, size: usize) -> bool {
    let begin = begin as usize;
    let end = end as usize;
    let virt = virt as usize;
    virt >= begin && virt.wrapping_add(size) <= end
}

#[inline]
pub unsafe fn memory_intersects(begin: *mut c_void, end: *mut c_void,
                                virt: *mut c_void, size: usize) -> bool {
    let begin = begin as usize;
    let end = end as usize;
    let virt = virt as usize;
    let vend = virt.wrapping_add(size);
    if virt < end && vend > begin { true } else { false }
}

#[inline]
pub unsafe fn init_section_contains(virt: *mut c_void, size: usize) -> bool {
    memory_contains(core::ptr::addr_of!(__init_begin) as *mut c_void,
                    core::ptr::addr_of!(__init_end) as *mut c_void, virt, size)
}

#[inline]
pub unsafe fn init_section_intersects(virt: *mut c_void, size: usize) -> bool {
    memory_intersects(core::ptr::addr_of!(__init_begin) as *mut c_void,
                      core::ptr::addr_of!(__init_end) as *mut c_void, virt, size)
}

#[inline]
pub unsafe fn is_kernel_core_data(addr: usize) -> bool {
    (addr >= core::ptr::addr_of!(_sdata) as usize && addr < core::ptr::addr_of!(_edata) as usize) ||
    (addr >= core::ptr::addr_of!(__bss_start) as usize && addr < core::ptr::addr_of!(__bss_stop) as usize)
}

#[inline]
pub unsafe fn is_kernel_rodata(addr: usize) -> bool {
    addr >= core::ptr::addr_of!(__start_rodata) as usize && addr < core::ptr::addr_of!(__end_rodata) as usize
}

#[inline]
pub unsafe fn is_kernel_ro_after_init(addr: usize) -> bool {
    addr >= core::ptr::addr_of!(__start_ro_after_init) as usize && addr < core::ptr::addr_of!(__end_ro_after_init) as usize
}

#[inline]
pub unsafe fn is_kernel_inittext(addr: usize) -> bool {
    addr >= core::ptr::addr_of!(_sinittext) as usize && addr < core::ptr::addr_of!(_einittext) as usize
}

#[inline]
pub unsafe fn __is_kernel_text(addr: usize) -> bool {
    addr >= core::ptr::addr_of!(_stext) as usize && addr < core::ptr::addr_of!(_etext) as usize
}

#[inline]
pub unsafe fn __is_kernel(addr: usize) -> bool {
    (addr >= core::ptr::addr_of!(_stext) as usize && addr < core::ptr::addr_of!(_end) as usize) ||
    (addr >= core::ptr::addr_of!(__init_begin) as usize && addr < core::ptr::addr_of!(__init_end) as usize)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
