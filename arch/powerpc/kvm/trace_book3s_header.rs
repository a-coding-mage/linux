/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Common defines used by the trace macros in trace_pr.h and trace_hv.h
 *
 * The C macro expands to a comma-separated list of (exit code, symbol)
 * initializers.  This Rust slice preserves that table as a directly usable
 * local equivalent.
 */
pub const KVM_TRACE_SYMBOL_EXIT: &[(u32, &str)] = &[
    (0x100, "SYSTEM_RESET"),
    (0x200, "MACHINE_CHECK"),
    (0x300, "DATA_STORAGE"),
    (0x380, "DATA_SEGMENT"),
    (0x400, "INST_STORAGE"),
    (0x480, "INST_SEGMENT"),
    (0x500, "EXTERNAL"),
    (0x502, "EXTERNAL_HV"),
    (0x600, "ALIGNMENT"),
    (0x700, "PROGRAM"),
    (0x800, "FP_UNAVAIL"),
    (0x900, "DECREMENTER"),
    (0x980, "HV_DECREMENTER"),
    (0xc00, "SYSCALL"),
    (0xd00, "TRACE"),
    (0xe00, "H_DATA_STORAGE"),
    (0xe20, "H_INST_STORAGE"),
    (0xe40, "H_EMUL_ASSIST"),
    (0xea0, "H_VIRT"),
    (0xf00, "PERFMON"),
    (0xf20, "ALTIVEC"),
    (0xf40, "VSX"),
];

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
