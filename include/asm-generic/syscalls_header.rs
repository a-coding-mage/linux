/* SPDX-License-Identifier: GPL-2.0 */

// Calling conventions for these system calls can differ, so it is possible
// to override them. The declarations below correspond to the C declarations
// guarded by the respective `#ifndef` conditions.

// `struct pt_regs` is supplied by an external dependency.
pub struct pt_regs;

extern "C" {
    // #ifndef sys_mmap2
    pub fn sys_mmap2(
        addr: usize,
        len: usize,
        prot: usize,
        flags: usize,
        fd: usize,
        pgoff: usize,
    ) -> isize;

    // #ifndef sys_mmap
    pub fn sys_mmap(
        addr: usize,
        len: usize,
        prot: usize,
        flags: usize,
        fd: usize,
        off: usize,
    ) -> isize;

    // #ifndef sys_rt_sigreturn
    pub fn sys_rt_sigreturn(regs: *mut pt_regs) -> isize;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
