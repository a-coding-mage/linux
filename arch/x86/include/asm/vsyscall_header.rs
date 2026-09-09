/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation:
// linux/seqlock.h, uapi/asm/vsyscall.h, and asm/page_types.h.

// The following declarations and definitions are conditional on
// CONFIG_X86_VSYSCALL_EMULATION.
#[cfg(CONFIG_X86_VSYSCALL_EMULATION)]
unsafe extern "C" {
    pub fn map_vsyscall();
    pub fn set_vsyscall_pgtable_user_bits(root: *mut pgd_t);

    /*
     * Called on instruction fetch fault in vsyscall page.
     * Returns true if handled.
     */
    pub fn emulate_vsyscall_pf(
        error_code: ::core::ffi::c_ulong,
        regs: *mut pt_regs,
        address: ::core::ffi::c_ulong,
    ) -> bool;
    pub fn emulate_vsyscall_gp(regs: *mut pt_regs) -> bool;
}

#[cfg(not(CONFIG_X86_VSYSCALL_EMULATION))]
#[inline]
pub fn map_vsyscall() {}

#[cfg(not(CONFIG_X86_VSYSCALL_EMULATION))]
#[inline]
pub fn emulate_vsyscall_pf(
    _error_code: ::core::ffi::c_ulong,
    _regs: *mut pt_regs,
    _address: ::core::ffi::c_ulong,
) -> bool {
    false
}

#[cfg(not(CONFIG_X86_VSYSCALL_EMULATION))]
#[inline]
pub fn emulate_vsyscall_gp(_regs: *mut pt_regs) -> bool {
    false
}

/*
 * The (legacy) vsyscall page is the long page in the kernel portion
 * of the address space that has user-accessible permissions.
 */
#[inline]
pub fn is_vsyscall_vaddr(vaddr: ::core::ffi::c_ulong) -> bool {
    // C's unlikely() branch prediction hint has no required Rust equivalent.
    (vaddr & PAGE_MASK) == VSYSCALL_ADDR
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
