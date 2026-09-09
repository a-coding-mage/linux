/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/**
 * struct ucontext - user context structure
 * @uc_flags:
 * @uc_link:
 * @uc_stack:
 * @uc_mcontext:    holds basic processor state
 * @uc_sigmask:
 * @uc_extcontext:  holds extended processor state
 */
#[repr(C)]
pub struct ucontext {
    pub uc_flags: core::ffi::c_ulong,
    pub uc_link: *mut ucontext,
    pub uc_stack: stack_t,
    pub uc_sigmask: sigset_t,
    /* There's some padding here to allow sigset_t to be expanded in the
     * future.  Though this is unlikely, other architectures put uc_sigmask
     * at the end of this structure and explicitly state it can be
     * expanded, so we didn't want to box ourselves in here. */
    pub __unused: [u8; 1024 / 8 - core::mem::size_of::<sigset_t>()],
    /* We can't put uc_sigmask at the end of this structure because we need
     * to be able to expand sigcontext in the future.  For example, the
     * vector ISA extension will almost certainly add ISA state.  We want
     * to ensure all user-visible ISA state can be saved and restored via a
     * ucontext, so we're putting this at the end in order to allow for
     * infinite extensibility.  Since we know this will be extended and we
     * assume sigset_t won't be extended an extreme amount, we're
     * prioritizing this. */
    pub uc_mcontext: sigcontext,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
