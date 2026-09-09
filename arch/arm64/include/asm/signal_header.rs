/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the corresponding architecture and UAPI headers.

#[inline]
unsafe fn arch_untagged_si_addr(
    addr: *mut core::ffi::c_void,
    sig: u64,
    si_code: u64,
) -> *mut core::ffi::c_void {
    /*
     * For historical reasons, all bits of the fault address are exposed as
     * address bits for watchpoint exceptions. New architectures should
     * handle the tag bits consistently.
     */
    if sig == SIGTRAP && si_code == TRAP_BRKPT {
        return addr;
    }

    untagged_addr(addr)
}

// C macro: arch_untagged_si_addr arch_untagged_si_addr

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
