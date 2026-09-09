/* SPDX-License-Identifier: GPL-2.0 */

// Translated from linux/objtool.h.  The original include dependencies are
// provided by the surrounding kernel translation.

#[cfg(objtool)]
#[cfg(not(assembler))]
#[macro_export]
macro_rules! unwind_hint {
    ($type:expr, $sp_reg:expr, $sp_offset:expr, $signal:expr) => {
        concat!(
            "987: \n\t",
            ".pushsection .discard.unwind_hints\n\t",
            /* ANNOTATE_DATA_SPECIAL */
            "\n\t",
            ".long 987b - .\n\t",
            ".short ", stringify!($sp_offset), "\n\t",
            ".byte ", stringify!($sp_reg), "\n\t",
            ".byte ", stringify!($type), "\n\t",
            ".byte ", stringify!($signal), "\n\t",
            ".balign 4 \n\t",
            ".popsection\n\t"
        )
    };
}

/*
 * This macro marks the given function's stack frame as "non-standard", which
 * tells objtool to ignore the function when doing stack metadata validation.
 * It should only be used in special cases where you're 100% sure it won't
 * affect the reliability of frame pointers and kernel stack traces.
 *
 * For more information, see tools/objtool/Documentation/objtool.txt.
 */
#[cfg(objtool)]
#[cfg(not(assembler))]
#[macro_export]
macro_rules! stack_frame_non_standard {
    ($func:expr) => {
        const _: *const () = $func as *const ()
    };
}

/* STACK_FRAME_NON_STANDARD_FP() is frame-pointer-specific. */
#[cfg(all(objtool, frame_pointer, not(assembler)))]
#[macro_export]
macro_rules! stack_frame_non_standard_fp {
    ($func:expr) => { $crate::stack_frame_non_standard!($func) };
}

#[cfg(all(objtool, not(frame_pointer), not(assembler)))]
#[macro_export]
macro_rules! stack_frame_non_standard_fp {
    ($func:expr) => {};
}

#[cfg(objtool)]
#[cfg(not(assembler))]
#[macro_export]
macro_rules! asm_reachable {
    () => {
        "998:\n\t.pushsection .discard.reachable\n\t.long 998b\n\t.popsection\n\t"
    };
}

#[macro_export]
macro_rules! asm_bref {
    ($label:ident) => { $label };
}

#[cfg(not(objtool))]
#[cfg(not(assembler))]
#[macro_export]
macro_rules! unwind_hint {
    ($type:expr, $sp_reg:expr, $sp_offset:expr, $signal:expr) => { "\n\t" };
}

#[cfg(not(objtool))]
#[cfg(not(assembler))]
#[macro_export]
macro_rules! stack_frame_non_standard {
    ($func:expr) => {};
}

#[cfg(not(objtool))]
#[cfg(not(assembler))]
#[macro_export]
macro_rules! stack_frame_non_standard_fp {
    ($func:expr) => {};
}

#[cfg(all(noinstr_validation, any(mitigation_unret_entry, mitigation_srso)))]
#[macro_export]
macro_rules! validate_unret_begin {
    () => {
        /* ANNOTATE_UNRET_BEGIN */
    };
}

#[cfg(not(all(noinstr_validation, any(mitigation_unret_entry, mitigation_srso))))]
#[macro_export]
macro_rules! validate_unret_begin {
    () => {};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
