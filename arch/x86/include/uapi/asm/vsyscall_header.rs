/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

#[repr(C)]
pub enum vsyscall_num {
    __NR_vgettimeofday,
    __NR_vtime,
    __NR_vgetcpu,
}

pub const VSYSCALL_ADDR: usize = 10usize.wrapping_neg() << 20;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
