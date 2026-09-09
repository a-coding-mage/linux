// SPDX-License-Identifier: GPL-2.0-or-later
/* Rewritten by Rusty Russell, on the backs of many others...
   Copyright (C) 2001 Rusty Russell, 2002 Rusty Russell IBM.
*/

// Linux kernel headers and build-time configuration dependencies are supplied
// by other translation units.

#[repr(C)]
pub struct exception_table_entry {
    _private: [u8; 0],
}

// mutex protecting text section modification (dynamic code patching).
// Some users need to sleep while they hold this lock.
// Also protects SMP-alternatives modification on x86; not exported to modules.
extern "C" {
    pub static mut text_mutex: core::ffi::c_void;

    pub static mut __start___ex_table: exception_table_entry;
    pub static mut __stop___ex_table: exception_table_entry;

    pub static mut system_state: i32;
    pub static SYSTEM_FREEING_INITMEM: i32;

    pub fn sort_extable(start: *mut exception_table_entry, stop: *mut exception_table_entry);
    pub fn search_extable(
        start: *const exception_table_entry,
        num: usize,
        addr: usize,
    ) -> *const exception_table_entry;
    pub fn search_module_extables(addr: usize) -> *const exception_table_entry;
    pub fn search_bpf_extables(addr: usize) -> *const exception_table_entry;

    pub fn is_kernel_text(addr: usize) -> bool;
    pub fn is_kernel_inittext(addr: usize) -> bool;
    pub fn is_module_text_address(addr: usize) -> bool;
    pub fn is_ftrace_trampoline(addr: usize) -> bool;
    pub fn is_kprobe_optinsn_slot(addr: usize) -> bool;
    pub fn is_kprobe_insn_slot(addr: usize) -> bool;
    pub fn is_bpf_text_address(addr: usize) -> bool;
    pub fn rcu_is_watching() -> bool;
    pub fn ct_nmi_enter();
    pub fn ct_nmi_exit();
    pub fn pr_notice(fmt: *const core::ffi::c_char, ...);
}

// Cleared by build-time tools if the table is already sorted.
#[no_mangle]
pub static mut main_extable_sort_needed: u32 = 1;

// Sort the kernel's built-in exception table.
#[no_mangle]
pub unsafe extern "C" fn sort_main_extable() {
    if main_extable_sort_needed != 0
        && (&__stop___ex_table as *const _ as usize)
            > (&__start___ex_table as *const _ as usize)
    {
        let message = b"Sorting __ex_table...\0";
        pr_notice(message.as_ptr() as *const core::ffi::c_char);
        sort_extable(
            &mut __start___ex_table,
            &mut __stop___ex_table,
        );
    }
}

// Given an address, look for it in the kernel exception table.
#[no_mangle]
pub unsafe extern "C" fn search_kernel_exception_table(
    addr: usize,
) -> *const exception_table_entry {
    let start = &__start___ex_table as *const exception_table_entry;
    let stop = &__stop___ex_table as *const exception_table_entry;
    search_extable(start, stop.offset_from(start) as usize, addr)
}

// Given an address, look for it in the exception tables.
#[no_mangle]
pub unsafe extern "C" fn search_exception_tables(
    addr: usize,
) -> *const exception_table_entry {
    let mut e = search_kernel_exception_table(addr);
    if e.is_null() {
        e = search_module_extables(addr);
    }
    if e.is_null() {
        e = search_bpf_extables(addr);
    }
    e
}

#[no_mangle]
pub unsafe extern "C" fn core_kernel_text(addr: usize) -> i32 {
    if is_kernel_text(addr) {
        return 1;
    }
    if system_state < SYSTEM_FREEING_INITMEM && is_kernel_inittext(addr) {
        return 1;
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn __kernel_text_address(addr: usize) -> i32 {
    if kernel_text_address(addr) != 0 {
        return 1;
    }
    if is_kernel_inittext(addr) {
        return 1;
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn kernel_text_address(addr: usize) -> i32 {
    let no_rcu = !rcu_is_watching();
    let mut ret = 1;

    if core_kernel_text(addr) != 0 {
        return 1;
    }
    if no_rcu {
        ct_nmi_enter();
    }
    if is_module_text_address(addr)
        || is_ftrace_trampoline(addr)
        || is_kprobe_optinsn_slot(addr)
        || is_kprobe_insn_slot(addr)
        || is_bpf_text_address(addr)
    {
        // C control flow jumps to out.
    } else {
        ret = 0;
    }
    if no_rcu {
        ct_nmi_exit();
    }
    ret
}

// CONFIG_HAVE_FUNCTION_DESCRIPTORS is a build-time condition from the kernel.
#[cfg(feature = "CONFIG_HAVE_FUNCTION_DESCRIPTORS")]
#[repr(C)]
pub struct func_desc_t {
    pub addr: *mut core::ffi::c_void,
}

#[cfg(feature = "CONFIG_HAVE_FUNCTION_DESCRIPTORS")]
extern "C" {
    pub static __start_opd: u8;
    pub static __end_opd: u8;
    pub fn get_kernel_nofault(dst: *mut *mut core::ffi::c_void, src: *const core::ffi::c_void) -> i32;
}

#[cfg(feature = "CONFIG_HAVE_FUNCTION_DESCRIPTORS")]
#[no_mangle]
pub unsafe extern "C" fn dereference_function_descriptor(
    mut ptr: *mut core::ffi::c_void,
) -> *mut core::ffi::c_void {
    let desc = ptr as *mut func_desc_t;
    let mut p: *mut core::ffi::c_void = core::ptr::null_mut();
    if get_kernel_nofault(&mut p, &(*desc).addr) == 0 {
        ptr = p;
    }
    ptr
}

#[cfg(feature = "CONFIG_HAVE_FUNCTION_DESCRIPTORS")]
#[no_mangle]
pub unsafe extern "C" fn dereference_kernel_function_descriptor(
    ptr: *mut core::ffi::c_void,
) -> *mut core::ffi::c_void {
    let start = &__start_opd as *const u8 as usize;
    let end = &__end_opd as *const u8 as usize;
    if (ptr as usize) < start || (ptr as usize) >= end {
        return ptr;
    }
    dereference_function_descriptor(ptr)
}

#[no_mangle]
pub unsafe extern "C" fn func_ptr_is_kernel_text(ptr: *mut core::ffi::c_void) -> i32 {
    #[cfg(feature = "CONFIG_HAVE_FUNCTION_DESCRIPTORS")]
    let addr = dereference_function_descriptor(ptr) as usize;
    #[cfg(not(feature = "CONFIG_HAVE_FUNCTION_DESCRIPTORS"))]
    let addr = ptr as usize;
    if core_kernel_text(addr) != 0 {
        return 1;
    }
    if is_module_text_address(addr) { 1 } else { 0 }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
