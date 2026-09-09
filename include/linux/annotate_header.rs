/* SPDX-License-Identifier: GPL-2.0 */

// Translation of linux/annotate.h. The original header provides assembler
// annotations when CONFIG_OBJTOOL is enabled and empty annotations otherwise.

#[cfg(feature = "CONFIG_OBJTOOL")]
#[macro_export]
macro_rules! __ASM_ANNOTATE {
    ($section:tt, $label:tt, $type:expr) => {
        concat!(
            ".pushsection ", stringify!($section), ", \"M\", @progbits, 8;\n",
            ".long ", stringify!($label), " - ., ", stringify!($type), ";\n",
            ".popsection"
        )
    };
}

#[cfg(feature = "CONFIG_OBJTOOL")]
#[macro_export]
macro_rules! ASM_ANNOTATE_LABEL {
    ($label:tt, $type:expr) => {
        $crate::__ASM_ANNOTATE!(.discard.annotate_insn, $label, $type)
    };
}

#[cfg(feature = "CONFIG_OBJTOOL")]
#[macro_export]
macro_rules! ASM_ANNOTATE {
    ($type:expr) => {
        concat!(
            "911: ",
            $crate::__ASM_ANNOTATE!(.discard.annotate_insn, 911b, $type)
        )
    };
}

#[cfg(feature = "CONFIG_OBJTOOL")]
#[macro_export]
macro_rules! ASM_ANNOTATE_DATA {
    ($type:expr) => {
        concat!(
            "912: ",
            $crate::__ASM_ANNOTATE!(.discard.annotate_data, 912b, $type)
        )
    };
}

#[cfg(not(feature = "CONFIG_OBJTOOL"))]
#[macro_export]
macro_rules! ASM_ANNOTATE_LABEL {
    ($label:tt, $type:expr) => { "" };
}

#[cfg(not(feature = "CONFIG_OBJTOOL"))]
#[macro_export]
macro_rules! ASM_ANNOTATE {
    ($type:expr) => { "" };
}

#[cfg(not(feature = "CONFIG_OBJTOOL"))]
#[macro_export]
macro_rules! ASM_ANNOTATE_DATA {
    ($type:expr) => { "" };
}

/*
 * Annotate away the various 'relocation to !ENDBR` complaints; knowing that
 * these relocations will never be used for indirect calls.
 */
#[macro_export]
macro_rules! ANNOTATE_NOENDBR {
    () => { $crate::ASM_ANNOTATE!(ANNOTYPE_NOENDBR) };
}

#[macro_export]
macro_rules! ANNOTATE_NOENDBR_SYM {
    ($sym:tt) => { core::arch::asm!($crate::ASM_ANNOTATE_LABEL!($sym, ANNOTYPE_NOENDBR)) };
}

/* This should be used immediately before an indirect jump/call. */
#[macro_export]
macro_rules! ANNOTATE_RETPOLINE_SAFE {
    () => { $crate::ASM_ANNOTATE!(ANNOTYPE_RETPOLINE_SAFE) };
}

#[macro_export]
macro_rules! ANNOTATE_INSTR_BEGIN {
    ($label:tt) => { $crate::ASM_ANNOTATE_LABEL!($label, ANNOTYPE_INSTR_BEGIN) };
}

#[macro_export]
macro_rules! ANNOTATE_INSTR_END {
    ($label:tt) => { $crate::ASM_ANNOTATE_LABEL!($label, ANNOTYPE_INSTR_END) };
}

#[macro_export]
macro_rules! ANNOTATE_IGNORE_ALTERNATIVE {
    () => { $crate::ASM_ANNOTATE!(ANNOTYPE_IGNORE_ALTS) };
}

#[macro_export]
macro_rules! ANNOTATE_INTRA_FUNCTION_CALL {
    () => { $crate::ASM_ANNOTATE!(ANNOTYPE_INTRA_FUNCTION_CALL) };
}

#[macro_export]
macro_rules! ANNOTATE_UNRET_BEGIN {
    () => { $crate::ASM_ANNOTATE!(ANNOTYPE_UNRET_BEGIN) };
}

#[macro_export]
macro_rules! ANNOTATE_REACHABLE {
    ($label:tt) => { $crate::ASM_ANNOTATE_LABEL!($label, ANNOTYPE_REACHABLE) };
}

#[macro_export]
macro_rules! ANNOTATE_NOCFI_SYM {
    ($sym:tt) => { core::arch::asm!($crate::ASM_ANNOTATE_LABEL!($sym, ANNOTYPE_NOCFI)) };
}

#[macro_export]
macro_rules! ANNOTATE_DATA_SPECIAL {
    () => { $crate::ASM_ANNOTATE_DATA!(ANNOTYPE_DATA_SPECIAL) };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
