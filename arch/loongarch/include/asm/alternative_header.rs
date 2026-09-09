/* SPDX-License-Identifier: GPL-2.0 */

/* C header guard: _ASM_ALTERNATIVE_H */
/* C conditional: declarations are omitted for assembler builds. */

#[repr(C, packed)]
pub struct alt_instr {
    pub instr_offset: i32,    /* offset to original instruction */
    pub replace_offset: i32,  /* offset to replacement instruction */
    pub feature: u16,         /* feature bit set for replacement */
    pub instrlen: u8,         /* length of original instruction */
    pub replacementlen: u8,   /* length of new instruction */
}

/*
 * Debug flag that can be tested to see whether alternative
 * instructions were patched in already:
 */
extern "C" {
    pub static mut alternatives_patched: i32;
    pub static mut __alt_instructions: alt_instr;
    pub static mut __alt_instructions_end: alt_instr;

    pub fn alternative_instructions();
    pub fn apply_alternatives(start: *mut alt_instr, end: *mut alt_instr);
}

macro_rules! b_replacement {
    ($num:tt) => { concat!("664", stringify!($num)) };
}

macro_rules! e_replacement {
    ($num:tt) => { concat!("665", stringify!($num)) };
}

macro_rules! alt_end_marker {
    () => { "663" };
}

macro_rules! alt_slen {
    () => { "662b-661b" };
}

macro_rules! alt_total_slen {
    () => { concat!(alt_end_marker!(), "b-661b") };
}

macro_rules! alt_rlen {
    ($num:tt) => { concat!(e_replacement!($num), "f-", b_replacement!($num), "f") };
}

macro_rules! __OLDINSTR {
    ($oldinstr:expr, $num:tt) => {
        concat!(
            "661:\n\t", $oldinstr, "\n662:\n",
            ".fill -(((", alt_rlen!($num), ")-(", alt_slen!(), ")) > 0) * ",
            "((", alt_rlen!($num), ")-(", alt_slen!(), ")) / 4, 4, 0x03400000\n"
        )
    };
}

macro_rules! OLDINSTR {
    ($oldinstr:expr, $num:tt) => { concat!(__OLDINSTR!($oldinstr, $num), alt_end_marker!(), ":\n") };
}

macro_rules! alt_max_short {
    ($a:expr, $b:expr) => {
        concat!(
            "((", $a, ") ^ (((", $a, ") ^ (", $b,
            ")) & -(-((", $a, ") < (", $b, ")))))"
        )
    };
}

/* Pad the second replacement alternative with additional NOPs if needed. */
macro_rules! OLDINSTR_2 {
    ($oldinstr:expr, $num1:tt, $num2:tt) => {
        concat!(
            "661:\n\t", $oldinstr, "\n662:\n",
            ".fill -(\(", alt_max_short!(alt_rlen!($num1), alt_rlen!($num2)),
            " - (", alt_slen!(), ")) > 0) * (", alt_max_short!(alt_rlen!($num1), alt_rlen!($num2)),
            " - (", alt_slen!(), ")) / 4, 4, 0x03400000\n",
            alt_end_marker!(), ":\n"
        )
    };
}

macro_rules! ALTINSTR_ENTRY {
    ($feature:expr, $num:tt) => {
        concat!(
            " .long 661b - .\n",
            " .long ", b_replacement!($num), "f - .\n",
            " .short ", stringify!($feature), "\n",
            " .byte ", alt_total_slen!(), "\n",
            " .byte ", alt_rlen!($num), "\n"
        )
    };
}

macro_rules! ALTINSTR_REPLACEMENT {
    ($newinstr:expr, $feature:expr, $num:tt) => {
        concat!(b_replacement!($num), ":\n\t", $newinstr, "\n", e_replacement!($num), ":\n\t")
    };
}

/* alternative assembly primitive */
macro_rules! ALTERNATIVE {
    ($oldinstr:expr, $newinstr:expr, $feature:expr) => {
        concat!(
            OLDINSTR!($oldinstr, 1),
            ".pushsection .altinstructions,\"a\"\n",
            ALTINSTR_ENTRY!($feature, 1),
            ".popsection\n.subsection 1\n",
            ALTINSTR_REPLACEMENT!($newinstr, $feature, 1),
            ".previous\n"
        )
    };
}

macro_rules! ALTERNATIVE_2 {
    ($oldinstr:expr, $newinstr1:expr, $feature1:expr, $newinstr2:expr, $feature2:expr) => {
        concat!(
            OLDINSTR_2!($oldinstr, 1, 2),
            ".pushsection .altinstructions,\"a\"\n",
            ALTINSTR_ENTRY!($feature1, 1), ALTINSTR_ENTRY!($feature2, 2),
            ".popsection\n.subsection 1\n",
            ALTINSTR_REPLACEMENT!($newinstr1, $feature1, 1),
            ALTINSTR_REPLACEMENT!($newinstr2, $feature2, 2),
            ".previous\n"
        )
    };
}

/*
 * Alternative instructions for different CPU types or capabilities.
 * The old instruction must be at least as long as each replacement.
 */
macro_rules! alternative {
    ($oldinstr:expr, $newinstr:expr, $feature:expr) => {{
        unsafe { core::arch::asm!(ALTERNATIVE!($oldinstr, $newinstr, $feature), options(nostack, preserves_flags)) }
    }};
}

macro_rules! alternative_2 {
    ($oldinstr:expr, $newinstr1:expr, $feature1:expr, $newinstr2:expr, $feature2:expr) => {{
        unsafe { core::arch::asm!(ALTERNATIVE_2!($oldinstr, $newinstr1, $feature1, $newinstr2, $feature2), options(nostack, preserves_flags)) }
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
