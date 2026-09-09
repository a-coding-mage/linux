/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License. See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2004-2012 Cavium Networks
 */

// Dependencies supplied by the kernel's MIPS/Octeon implementation.

#[repr(C)]
pub struct octeon_cop2_state {
    _private: [u8; 0],
}

extern "C" {
    fn preempt_disable();
    fn preempt_enable();
    fn local_irq_save(flags: *mut ::core::ffi::c_ulong);
    fn local_irq_restore(flags: ::core::ffi::c_ulong);
    fn read_c0_status() -> ::core::ffi::c_int;
    fn write_c0_status(status: ::core::ffi::c_int);
    fn octeon_cop2_save(state: *mut octeon_cop2_state);
    fn octeon_cop2_restore(state: *mut octeon_cop2_state);

    // Rust-facing forms of the kernel's KSTK_STATUS(current) access and
    // current->thread.cp2 access; their definitions are provided elsewhere.
    fn octeon_current_kstk_status() -> *mut ::core::ffi::c_int;
    fn octeon_current_thread_cp2() -> *mut octeon_cop2_state;
}

const ST0_CU2: ::core::ffi::c_int = 1 << 30;

/**
 * Enable access to Octeon's COP2 crypto hardware for kernel use. Wrap any
 * crypto operations in calls to octeon_crypto_enable/disable in order to make
 * sure the state of COP2 isn't corrupted if userspace is also performing
 * hardware crypto operations. Allocate the state parameter on the stack.
 * Returns with preemption disabled.
 *
 * @state: Pointer to state structure to store current COP2 state in.
 *
 * Returns: Flags to be passed to octeon_crypto_disable()
 */
pub unsafe fn octeon_crypto_enable(state: *mut octeon_cop2_state) -> ::core::ffi::c_ulong {
    let mut status: ::core::ffi::c_int;
    let mut flags: ::core::ffi::c_ulong = 0;

    preempt_disable();
    local_irq_save(&mut flags);
    status = read_c0_status();
    write_c0_status(status | ST0_CU2);
    let current_status = octeon_current_kstk_status();
    if (*current_status & ST0_CU2) != 0 {
        octeon_cop2_save(octeon_current_thread_cp2());
        *current_status &= !ST0_CU2;
        status &= !ST0_CU2;
    } else if (status & ST0_CU2) != 0 {
        octeon_cop2_save(state);
    }
    local_irq_restore(flags);
    (status & ST0_CU2) as ::core::ffi::c_ulong
}

/**
 * Disable access to Octeon's COP2 crypto hardware in the kernel. This must be
 * called after an octeon_crypto_enable() before any context switch or return
 * to userspace.
 *
 * @state:  Pointer to COP2 state to restore
 * @flags:  Return value from octeon_crypto_enable()
 */
pub unsafe fn octeon_crypto_disable(
    state: *mut octeon_cop2_state,
    crypto_flags: ::core::ffi::c_ulong,
) {
    let mut flags: ::core::ffi::c_ulong = 0;

    local_irq_save(&mut flags);
    if (crypto_flags & ST0_CU2 as ::core::ffi::c_ulong) != 0 {
        octeon_cop2_restore(state);
    } else {
        write_c0_status(read_c0_status() & !ST0_CU2);
    }
    local_irq_restore(flags);
    preempt_enable();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
