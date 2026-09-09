/* SPDX-License-Identifier: GPL-2.0 */

// The macros in this file implement an exponential backoff facility
// for atomic operations.  The original header contains SPARC assembler
// macros; their source-level form is retained below as Rust macros.

pub const BACKOFF_LIMIT: u32 = 4 * 1024;

#[cfg(feature = "CONFIG_SMP")]
macro_rules! BACKOFF_SETUP {
    ($reg:ident) => {
        // Original SPARC instruction: mov 1, $reg
        $reg = 1;
    };
}

#[cfg(not(feature = "CONFIG_SMP"))]
macro_rules! BACKOFF_SETUP {
    ($reg:ident) => {};
}

#[cfg(feature = "CONFIG_SMP")]
macro_rules! BACKOFF_LABEL {
    ($spin_label:ident, $continue_label:ident) => {
        $spin_label
    };
}

#[cfg(not(feature = "CONFIG_SMP"))]
macro_rules! BACKOFF_LABEL {
    ($spin_label:ident, $continue_label:ident) => {
        $continue_label
    };
}

#[cfg(feature = "CONFIG_SMP")]
macro_rules! BACKOFF_SPIN {
    ($reg:ident, $tmp:ident, $label:ident) => {{
        // Original SPARC sequence:
        // mov $reg, $tmp;
        // 88: rd %ccr, %g0; rd %ccr, %g0; rd %ccr, %g0;
        // .section .pause_3insn_patch,"ax"; .word 88b;
        // sllx $tmp, 7, $tmp; wr $tmp, 0, %asr27; clr $tmp; .previous;
        // brnz,pt $tmp, 88b; sub $tmp, 1, $tmp;
        // set BACKOFF_LIMIT, $tmp; cmp $reg, $tmp;
        // bg,pn %xcc, $label; nop; ba,pt %xcc, $label;
        // sllx $reg, 1, $reg;
        $tmp = $reg;
        while $tmp != 0 {
            $tmp = $tmp.wrapping_sub(1);
        }
        if $reg > BACKOFF_LIMIT {
            $reg = $reg;
        } else {
            $reg = $reg.wrapping_shl(1);
        }
    }};
}

#[cfg(not(feature = "CONFIG_SMP"))]
macro_rules! BACKOFF_SPIN {
    ($reg:ident, $tmp:ident, $label:ident) => {};
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
