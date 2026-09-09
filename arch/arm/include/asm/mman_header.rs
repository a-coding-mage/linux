/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding architecture headers:
// `cpu_architecture()` and `CPU_ARCH_ARMv6`.

#[inline]
pub fn arch_memory_deny_write_exec_supported() -> bool {
    cpu_architecture() >= CPU_ARCH_ARMv6
}

// C macro alias:
// #define arch_memory_deny_write_exec_supported arch_memory_deny_write_exec_supported

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
