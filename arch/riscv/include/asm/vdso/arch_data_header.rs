/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the corresponding Linux/VDSO headers:
// linux/types.h, vdso/datapage.h, and asm/hwprobe.h.

#[repr(C)]
pub struct vdso_arch_data {
    /* Stash static answers to the hwprobe queries when all CPUs are selected. */
    pub all_cpu_hwprobe_values: [u64; RISCV_HWPROBE_MAX_KEY as usize + 1],

    /* Boolean indicating all CPUs have the same static hwprobe values. */
    pub homogeneous_cpus: u8,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
