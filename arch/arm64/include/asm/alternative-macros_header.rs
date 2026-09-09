/* SPDX-License-Identifier: GPL-2.0 */

/* Translated from asm/alternative-macros.h. */

pub const ARM64_CB_SHIFT: u32 = 15;
pub const ARM64_CB_BIT: u64 = 1u64 << ARM64_CB_SHIFT;

/* The original build-time check is retained as intent: ARM64_NCAPS must be
 * less than ARM64_CB_BIT. */

#[cfg(not(feature = "assembler"))]
#[macro_export]
macro_rules! ALTINSTR_ENTRY {
    ($cpucap:expr) => {
        concat!(
            " .word 661b - .\n",
            " .word 663f - .\n",
            " .hword ", stringify!($cpucap), "\n",
            " .byte 662b-661b\n",
            " .byte 664f-663f\n"
        )
    };
}

#[cfg(not(feature = "assembler"))]
#[macro_export]
macro_rules! ALTINSTR_ENTRY_CB {
    ($cpucap:expr, $cb:expr) => {
        concat!(
            " .word 661b - .\n",
            " .word ", stringify!($cb), "- .\n",
            " .hword ", stringify!($cpucap), "\n",
            " .byte 662b-661b\n",
            " .byte 664f-663f\n"
        )
    };
}

/* Rust has no direct equivalent of the C preprocessor's assembly-string
 * concatenation. These macros preserve the original interfaces and emitted
 * assembly text. */
#[macro_export]
macro_rules! __ALTERNATIVE_CFG {
    ($oldinstr:expr, $newinstr:expr, $cpucap:expr, $cfg_enabled:expr) => {
        concat!(
            ".if ", stringify!($cfg_enabled), " == 1\n",
            "661:\n\t", $oldinstr, "\n",
            "662:\n",
            ".pushsection .altinstructions,\"a\"\n",
            ALTINSTR_ENTRY!($cpucap),
            ".popsection\n.subsection 1\n",
            "663:\n\t", $newinstr, "\n",
            "664:\n\t",
            ".org\t. - (664b-663b) + (662b-661b)\n\t",
            ".org\t. - (662b-661b) + (664b-663b)\n.previous\n.endif\n"
        )
    };
}

#[macro_export]
macro_rules! __ALTERNATIVE_CFG_CB {
    ($oldinstr:expr, $cpucap:expr, $cfg_enabled:expr, $cb:expr) => {
        concat!(
            ".if ", stringify!($cfg_enabled), " == 1\n",
            "661:\n\t", $oldinstr, "\n662:\n",
            ".pushsection .altinstructions,\"a\"\n",
            ALTINSTR_ENTRY_CB!($cpucap, $cb),
            ".popsection\n663:\n\t664:\n.endif\n"
        )
    };
}

#[macro_export]
macro_rules! _ALTERNATIVE_CFG {
    ($oldinstr:expr, $newinstr:expr, $cpucap:expr, $cfg:expr $(, $rest:expr)*) => {
        __ALTERNATIVE_CFG!($oldinstr, $newinstr, $cpucap, $cfg)
    };
}

#[macro_export]
macro_rules! ALTERNATIVE_CB {
    ($oldinstr:expr, $cpucap:expr, $cb:expr) => {
        __ALTERNATIVE_CFG_CB!(
            $oldinstr,
            (1u64 << ARM64_CB_SHIFT) | ($cpucap),
            1,
            $cb
        )
    };
}

/* The __ASSEMBLER__ branch consists solely of GNU assembler .macro
 * definitions (alternative_insn, alternative_if_not, alternative_if,
 * alternative_cb, alternative_else, alternative_endif, alternative_cb_end,
 * and alternative_else_nop_endif). They are preserved as assembly intent in
 * the source header and have no executable Rust representation. */

#[macro_export]
macro_rules! ALTERNATIVE {
    ($oldinstr:expr, $newinstr:expr, $($cpucap:expr),+ $(,)?) => {
        _ALTERNATIVE_CFG!($oldinstr, $newinstr, $($cpucap),+, 1)
    };
}

/* External dependencies supplied by the surrounding kernel translation. */
extern "C" {
    fn cpucap_is_possible(cpucap: libc::c_ulong) -> bool;
}

#[inline(always)]
pub unsafe fn alternative_has_cap_likely(cpucap: libc::c_ulong) -> bool {
    if !cpucap_is_possible(cpucap) {
        return false;
    }

    /* Rust has no stable asm-goto equivalent; the labels and ALTERNATIVE
     * sequence are retained above as assembly text, with this branch left as
     * the required external low-level integration point. */
    todo!("translate asm goto alternative_has_cap_likely")
}

#[inline(always)]
pub unsafe fn alternative_has_cap_unlikely(cpucap: libc::c_ulong) -> bool {
    if !cpucap_is_possible(cpucap) {
        return false;
    }

    todo!("translate asm goto alternative_has_cap_unlikely")
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
