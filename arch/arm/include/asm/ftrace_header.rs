/* SPDX-License-Identifier: GPL-2.0 */

// #define HAVE_FUNCTION_GRAPH_FP_TEST
pub const HAVE_FUNCTION_GRAPH_FP_TEST: bool = true;

// CONFIG_DYNAMIC_FTRACE_WITH_REGS controls this declaration in the original header.
pub const ARCH_SUPPORTS_FTRACE_OPS: usize = 1;

// CONFIG_FUNCTION_TRACER controls the following declarations.
pub const MCOUNT_INSN_SIZE: usize = 4; // sizeof mcount call

unsafe extern "C" {
    pub fn __gnu_mcount_nc();
}

// CONFIG_DYNAMIC_FTRACE controls this type and function.
#[repr(C)]
pub struct dyn_arch_ftrace {
    // CONFIG_ARM_MODULE_PLTS controls this field in the original header.
    pub mod_: *mut core::ffi::c_void,
}

#[inline]
pub const fn ftrace_call_adjust(addr: usize) -> usize {
    // With Thumb-2, the recorded addresses have the lsb set.
    addr & !1
}

pub const MCOUNT_ADDR: usize = __gnu_mcount_nc as usize;

// CONFIG_FRAME_POINTER && !CONFIG_ARM_UNWIND selects the external declaration.
unsafe extern "C" {
    pub fn return_address(level: u32) -> *mut core::ffi::c_void;
}

// The alternative inline definition when the frame-pointer/unwind condition is
// not selected is represented here as a separate implementation.
#[inline]
pub fn return_address_null(_level: u32) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

#[inline]
pub unsafe fn ftrace_return_address(n: u32) -> *mut core::ffi::c_void {
    return_address(n)
}

unsafe extern "C" {
    fn strcmp(lhs: *const core::ffi::c_char, rhs: *const core::ffi::c_char) -> i32;
    fn strcasecmp(lhs: *const core::ffi::c_char, rhs: *const core::ffi::c_char) -> i32;
}

#[inline]
pub unsafe fn arch_syscall_match_sym_name(
    mut sym: *const core::ffi::c_char,
    name: *const core::ffi::c_char,
) -> bool {
    if strcmp(sym, c"sys_mmap2".as_ptr()) == 0 {
        sym = c"sys_mmap_pgoff".as_ptr();
    } else if strcmp(sym, c"sys_statfs64_wrapper".as_ptr()) == 0 {
        sym = c"sys_statfs64".as_ptr();
    } else if strcmp(sym, c"sys_fstatfs64_wrapper".as_ptr()) == 0 {
        sym = c"sys_fstatfs64".as_ptr();
    } else if strcmp(sym, c"sys_arm_fadvise64_64".as_ptr()) == 0 {
        sym = c"sys_fadvise64_64".as_ptr();
    }

    // Ignore case since sym may start with "SyS" instead of "sys".
    strcasecmp(sym, name) != 0
}

unsafe extern "C" {
    pub fn prepare_ftrace_return(
        parent: *mut usize,
        self_: usize,
        frame_pointer: usize,
        stack_pointer: usize,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
