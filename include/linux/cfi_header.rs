/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Clang Control Flow Integrity (CFI) support.
 *
 * Copyright (C) 2022 Google LLC
 */

// Dependency intent from <linux/bug.h>, <linux/module.h>, <linux/uaccess.h>,
// and <asm/cfi.h> is preserved through the referenced external types/functions.

#[cfg(CONFIG_CFI)]
extern "C" {
    pub static mut cfi_warn: bool;

    pub fn report_cfi_failure(
        regs: *mut pt_regs,
        addr: kernel::ffi::c_ulong,
        target: *mut kernel::ffi::c_ulong,
        type_: u32,
    ) -> bug_trap_type;
}

#[cfg(CONFIG_CFI)]
#[inline]
pub unsafe fn report_cfi_failure_noaddr(
    regs: *mut pt_regs,
    addr: kernel::ffi::c_ulong,
) -> bug_trap_type {
    report_cfi_failure(regs, addr, core::ptr::null_mut(), 0)
}

// The architecture may override cfi_get_offset when a patchable function
// entry changes the compiler-emitted CFI prefix offset.
#[cfg(CONFIG_CFI)]
#[inline]
pub fn cfi_get_offset() -> kernel::ffi::c_int {
    4
}

#[cfg(CONFIG_CFI)]
#[inline]
pub unsafe fn cfi_get_func_hash(func: *mut kernel::ffi::c_void) -> u32 {
    let mut hash: u32 = 0;
    // get_kernel_nofault is supplied by the kernel uaccess implementation.
    if get_kernel_nofault(
        &mut hash as *mut u32,
        (func as *mut u8).offset(-(cfi_get_offset() as isize)) as *mut kernel::ffi::c_void,
    ) != 0
    {
        return 0;
    }

    hash
}

#[cfg(CONFIG_CFI)]
extern "C" {
    pub static mut cfi_bpf_hash: u32;
    pub static mut cfi_bpf_subprog_hash: u32;
}

#[cfg(not(CONFIG_CFI))]
#[inline]
pub fn cfi_get_offset() -> kernel::ffi::c_int {
    0
}

#[cfg(not(CONFIG_CFI))]
#[inline]
pub fn cfi_get_func_hash(_func: *mut kernel::ffi::c_void) -> u32 {
    0
}

#[cfg(not(CONFIG_CFI))]
pub const cfi_bpf_hash: u32 = 0;

#[cfg(not(CONFIG_CFI))]
pub const cfi_bpf_subprog_hash: u32 = 0;

#[cfg(CONFIG_ARCH_USES_CFI_TRAPS)]
extern "C" {
    pub fn is_cfi_trap(addr: kernel::ffi::c_ulong) -> bool;
}

#[cfg(not(CONFIG_ARCH_USES_CFI_TRAPS))]
#[inline]
pub fn is_cfi_trap(_addr: kernel::ffi::c_ulong) -> bool {
    false
}

#[cfg(all(CONFIG_MODULES, CONFIG_ARCH_USES_CFI_TRAPS))]
extern "C" {
    pub fn module_cfi_finalize(
        hdr: *const Elf_Ehdr,
        sechdrs: *const Elf_Shdr,
        mod_: *mut module,
    );
}

#[cfg(all(CONFIG_MODULES, not(CONFIG_ARCH_USES_CFI_TRAPS)))]
#[inline]
pub fn module_cfi_finalize(
    _hdr: *const Elf_Ehdr,
    _sechdrs: *const Elf_Shdr,
    _mod: *mut module,
) {
}

#[cfg(not(CFI_NOSEAL))]
#[macro_export]
macro_rules! CFI_NOSEAL {
    ($x:expr) => {{ let _ = &$x; }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
