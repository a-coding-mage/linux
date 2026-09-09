/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/* C header guard: _XT_CPU_H */
/* Dependency: <linux/types.h> */

#[repr(C)]
pub struct xt_cpu_info {
    pub cpu: u32,
    pub invert: u32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
