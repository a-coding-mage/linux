/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2020-2021 ARM Ltd.
 */

// The original declarations are assembler-only and are active only when
// CONFIG_ARM64_MTE is enabled. They are preserved here as Rust macro items.

#[cfg(feature = "CONFIG_ARM64_MTE")]
macro_rules! mte_switch_to_guest {
    ($g_ctxt:expr, $h_ctxt:expr, $reg1:expr) => {{
        // alternative_if_not ARM64_MTE
        // b .L__skip_switch\@
        // alternative_else_nop_endif
        // mrs $reg1, hcr_el2
        // tbz $reg1, #(HCR_ATA_SHIFT), .L__skip_switch\@

        // mrs_s $reg1, SYS_RGSR_EL1
        // str $reg1, [$h_ctxt, #CPU_RGSR_EL1]
        // mrs_s $reg1, SYS_GCR_EL1
        // str $reg1, [$h_ctxt, #CPU_GCR_EL1]

        // ldr $reg1, [$g_ctxt, #CPU_RGSR_EL1]
        // msr_s SYS_RGSR_EL1, $reg1
        // ldr $reg1, [$g_ctxt, #CPU_GCR_EL1]
        // msr_s SYS_GCR_EL1, $reg1

        // .L__skip_switch\@:
    }};
}

#[cfg(feature = "CONFIG_ARM64_MTE")]
macro_rules! mte_switch_to_hyp {
    ($g_ctxt:expr, $h_ctxt:expr, $reg1:expr) => {{
        // alternative_if_not ARM64_MTE
        // b .L__skip_switch\@
        // alternative_else_nop_endif
        // mrs $reg1, hcr_el2
        // tbz $reg1, #(HCR_ATA_SHIFT), .L__skip_switch\@

        // mrs_s $reg1, SYS_RGSR_EL1
        // str $reg1, [$g_ctxt, #CPU_RGSR_EL1]
        // mrs_s $reg1, SYS_GCR_EL1
        // str $reg1, [$g_ctxt, #CPU_GCR_EL1]

        // ldr $reg1, [$h_ctxt, #CPU_RGSR_EL1]
        // msr_s SYS_RGSR_EL1, $reg1
        // ldr $reg1, [$h_ctxt, #CPU_GCR_EL1]
        // msr_s SYS_GCR_EL1, $reg1

        // isb

        // .L__skip_switch\@:
    }};
}

#[cfg(not(feature = "CONFIG_ARM64_MTE"))]
macro_rules! mte_switch_to_guest {
    ($g_ctxt:expr, $h_ctxt:expr, $reg1:expr) => {};
}

#[cfg(not(feature = "CONFIG_ARM64_MTE"))]
macro_rules! mte_switch_to_hyp {
    ($g_ctxt:expr, $h_ctxt:expr, $reg1:expr) => {};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
