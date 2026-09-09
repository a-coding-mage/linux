/* SPDX-License-Identifier: GPL-2.0 */

// Assembly-only kuap_user_restore, kuap_kernel_restore, kuap_check_amr, and
// kuap_save_amr_and_lock macros are intentionally represented by this note:
// their bodies consist of PowerPC assembler instructions and feature-section
// directives, which have no standalone Rust syntax.

pub const AMR_KUAP_BLOCK_READ: u64 = 0x5455_5555_5555_5555;
pub const AMR_KUAP_BLOCK_WRITE: u64 = 0xa8aa_aaaa_aaaa_aaaa;
pub const AMR_KUEP_BLOCKED: u64 = 0x5455_5555_5555_5555;
pub const AMR_KUAP_BLOCKED: u64 = AMR_KUAP_BLOCK_READ | AMR_KUAP_BLOCK_WRITE;

extern "C" {
    pub static mut uaccess_flush_key: core::ffi::c_ulong;
}

#[cfg(feature = "CONFIG_PPC_PKEY")]
extern "C" {
    pub static mut default_uamor: u64;
    pub static mut default_amr: u64;
    pub static mut default_iamr: u64;
}

// The following functions and types are supplied by the surrounding kernel
// translation. Their declarations are retained as external dependencies.
extern "C" {
    fn mmu_has_feature(feature: u64) -> bool;
    fn mfspr(spr: u64) -> u64;
    fn mtspr(spr: u64, value: u64);
    fn isync();
    fn do_uaccess_flush();
    fn static_branch_unlikely(key: *mut core::ffi::c_ulong) -> bool;
    fn current_thread_amr() -> u64;
    fn warn_on_once(condition: bool);
}

#[repr(C)]
pub struct pt_regs {
    pub amr: u64,
    pub iamr: u64,
}

#[inline(always)]
pub unsafe fn kuap_user_restore(regs: *const pt_regs) {
    if !mmu_has_feature(MMU_FTR_PKEY) { return; }
    let mut restore_amr = false;
    let mut restore_iamr = false;
    if !mmu_has_feature(MMU_FTR_KUAP) {
        if mfspr(SPRN_AMR) != (*regs).amr { restore_amr = true; }
    } else { restore_amr = true; }
    if !mmu_has_feature(MMU_FTR_BOOK3S_KUEP) {
        if mfspr(SPRN_IAMR) != (*regs).iamr { restore_iamr = true; }
    } else { restore_iamr = true; }
    if restore_amr || restore_iamr {
        isync();
        if restore_amr { mtspr(SPRN_AMR, (*regs).amr); }
        if restore_iamr { mtspr(SPRN_IAMR, (*regs).iamr); }
    }
    // No isync is required: execution is about to rfi to the prior context.
}

#[inline(always)]
pub unsafe fn __kuap_kernel_restore(regs: *const pt_regs, amr: u64) {
    if (*regs).amr == amr { return; }
    isync();
    mtspr(SPRN_AMR, (*regs).amr);
    // No IAMR restore is required when returning to kernel space.
}

#[inline(always)]
pub unsafe fn __kuap_get_and_assert_locked() -> u64 {
    let amr = mfspr(SPRN_AMR);
    #[cfg(feature = "CONFIG_PPC_KUAP_DEBUG")]
    warn_on_once(amr != AMR_KUAP_BLOCKED);
    amr
}

#[inline(always)]
pub unsafe fn get_kuap() -> u64 {
    if !mmu_has_feature(MMU_FTR_KUAP) { return AMR_KUAP_BLOCKED; }
    mfspr(SPRN_AMR)
}

#[inline(always)]
pub unsafe fn set_kuap(value: u64) {
    if !mmu_has_feature(MMU_FTR_KUAP) { return; }
    isync(); mtspr(SPRN_AMR, value); isync();
}

#[inline(always)]
pub unsafe fn __bad_kuap_fault(regs: *const pt_regs, _address: u64, is_write: bool) -> bool {
    if is_write {
        ((*regs).amr & AMR_KUAP_BLOCK_WRITE) == AMR_KUAP_BLOCK_WRITE
    } else {
        ((*regs).amr & AMR_KUAP_BLOCK_READ) == AMR_KUAP_BLOCK_READ
    }
}

#[inline(always)]
pub unsafe fn allow_user_access(_to: *mut core::ffi::c_void, dir: u64) {
    let mut thread_amr = 0;
    if mmu_has_feature(MMU_FTR_PKEY) { thread_amr = current_thread_amr(); }
    if dir == KUAP_READ { set_kuap(thread_amr | AMR_KUAP_BLOCK_WRITE); }
    else if dir == KUAP_WRITE { set_kuap(thread_amr | AMR_KUAP_BLOCK_READ); }
    else if dir == KUAP_READ_WRITE { set_kuap(thread_amr); }
    else { panic!("BUILD_BUG"); }
}

#[inline(always)]
pub unsafe fn prevent_user_access(_dir: u64) {
    set_kuap(AMR_KUAP_BLOCKED);
    if static_branch_unlikely(&raw mut uaccess_flush_key) { do_uaccess_flush(); }
}

#[inline(always)]
pub unsafe fn prevent_user_access_return() -> u64 {
    let flags = get_kuap();
    set_kuap(AMR_KUAP_BLOCKED);
    if static_branch_unlikely(&raw mut uaccess_flush_key) { do_uaccess_flush(); }
    flags
}

#[inline(always)]
pub unsafe fn restore_user_access(flags: u64) {
    set_kuap(flags);
    if static_branch_unlikely(&raw mut uaccess_flush_key) && flags == AMR_KUAP_BLOCKED {
        do_uaccess_flush();
    }
}

// External constants supplied by asm/reg.h and related kernel headers.
extern "C" {
    static MMU_FTR_PKEY: u64;
    static MMU_FTR_KUAP: u64;
    static MMU_FTR_BOOK3S_KUEP: u64;
    static SPRN_AMR: u64;
    static SPRN_IAMR: u64;
    static KUAP_READ: u64;
    static KUAP_WRITE: u64;
    static KUAP_READ_WRITE: u64;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
