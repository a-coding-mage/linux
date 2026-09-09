/* SPDX-License-Identifier: GPL-2.0 */

// Translated from linux/bug.h.  The included architecture/compiler/build
// interfaces are supplied by other translated headers.

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum bug_trap_type {
    BUG_TRAP_TYPE_NONE = 0,
    BUG_TRAP_TYPE_WARN = 1,
    BUG_TRAP_TYPE_BUG = 2,
}

#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

#[cfg(feature = "CONFIG_GENERIC_BUG")]
extern "C" {
    pub fn bug_get_file_line(
        bug: *mut bug_entry,
        file: *mut *const core::ffi::c_char,
        line: *mut u32,
    );
    pub fn find_bug(bugaddr: u64) -> *mut bug_entry;
    pub fn report_bug(bug_addr: u64, regs: *mut pt_regs) -> bug_trap_type;
    pub fn report_bug_entry(bug: *mut bug_entry, regs: *mut pt_regs) -> bug_trap_type;
    pub fn is_valid_bugaddr(addr: u64) -> i32;
    pub fn generic_bug_clear_once();
}

#[cfg(feature = "CONFIG_GENERIC_BUG")]
#[repr(C)]
pub struct bug_entry {
    _private: [u8; 0],
}

#[cfg(feature = "CONFIG_GENERIC_BUG")]
pub unsafe fn is_warning_bug(_bug: *const bug_entry) -> i32 {
    // The bug_entry layout and BUGFLAG_WARNING are supplied by asm-generic/bug.h.
    // Field access is therefore deferred to the architecture-specific binding.
    unimplemented!()
}

#[cfg(not(feature = "CONFIG_GENERIC_BUG"))]
pub unsafe fn find_bug(_bugaddr: u64) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

#[cfg(not(feature = "CONFIG_GENERIC_BUG"))]
pub unsafe fn report_bug(_bug_addr: u64, _regs: *mut pt_regs) -> bug_trap_type {
    bug_trap_type::BUG_TRAP_TYPE_BUG
}

#[cfg(not(feature = "CONFIG_GENERIC_BUG"))]
pub struct bug_entry {
    _private: [u8; 0],
}

#[cfg(not(feature = "CONFIG_GENERIC_BUG"))]
pub unsafe fn report_bug_entry(_bug: *mut bug_entry, _regs: *mut pt_regs) -> bug_trap_type {
    bug_trap_type::BUG_TRAP_TYPE_BUG
}

#[cfg(not(feature = "CONFIG_GENERIC_BUG"))]
pub unsafe fn bug_get_file_line(
    _bug: *mut bug_entry,
    file: *mut *const core::ffi::c_char,
    line: *mut u32,
) {
    *file = core::ptr::null();
    *line = 0;
}

#[cfg(not(feature = "CONFIG_GENERIC_BUG"))]
pub unsafe fn generic_bug_clear_once() {}

#[cfg(feature = "CONFIG_PRINTK")]
extern "C" {
    pub fn mem_dump_obj(object: *mut core::ffi::c_void);
}

#[cfg(not(feature = "CONFIG_PRINTK"))]
pub unsafe fn mem_dump_obj(_object: *mut core::ffi::c_void) {}

/*
 * Since detected data corruption should stop operation on the affected
 * structures. Return value must be checked and sanely acted on by caller.
 */
#[must_use]
pub const fn check_data_corruption(v: bool) -> bool {
    v
}

// MAYBE_BUILD_BUG_ON and CHECK_DATA_CORRUPTION depend on compiler/build and
// printk/BUG/WARN macros supplied by other headers; their conditional intent
// is preserved here rather than inventing those external implementations.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
