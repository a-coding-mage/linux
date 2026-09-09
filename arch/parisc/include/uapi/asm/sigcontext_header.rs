/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

pub const PARISC_SC_FLAG_ONSTACK: usize = 1 << 0;
pub const PARISC_SC_FLAG_IN_SYSCALL: usize = 1 << 1;

/* We will add more stuff here as it becomes necessary, until we know
   it works. */
#[repr(C)]
pub struct sigcontext {
    pub sc_flags: usize,

    pub sc_gr: [usize; 32], /* PSW in sc_gr[0] */
    pub sc_fr: [u64; 32], /* FIXME, do we need other state info? */
    pub sc_iasq: [usize; 2],
    pub sc_iaoq: [usize; 2],
    pub sc_sar: usize, /* cr11 */
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
