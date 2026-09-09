/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Each alternative has a 32 bit feature field. The four low context bits,
 * eight type bits, and twenty data bits are represented by the bitfield view.
 */

pub const ALT_CTX_EARLY: u32 = 1;
pub const ALT_CTX_LATE: u32 = 2;
pub const ALT_CTX_ALL: u32 = ALT_CTX_EARLY | ALT_CTX_LATE;

pub const ALT_TYPE_FACILITY: u32 = 0;
pub const ALT_TYPE_FEATURE: u32 = 1;
pub const ALT_TYPE_SPEC: u32 = 2;

pub const ALT_DATA_SHIFT: u32 = 0;
pub const ALT_TYPE_SHIFT: u32 = 20;
pub const ALT_CTX_SHIFT: u32 = 28;

#[macro_export]
macro_rules! ALT_FACILITY {
    ($facility:expr) => {
        (ALT_CTX_EARLY << ALT_CTX_SHIFT)
            | (ALT_TYPE_FACILITY << ALT_TYPE_SHIFT)
            | (($facility as u32) << ALT_DATA_SHIFT)
    };
}

#[macro_export]
macro_rules! ALT_FEATURE {
    ($feature:expr) => {
        (ALT_CTX_EARLY << ALT_CTX_SHIFT)
            | (ALT_TYPE_FEATURE << ALT_TYPE_SHIFT)
            | (($feature as u32) << ALT_DATA_SHIFT)
    };
}

#[macro_export]
macro_rules! ALT_SPEC {
    ($facility:expr) => {
        (ALT_CTX_LATE << ALT_CTX_SHIFT)
            | (ALT_TYPE_SPEC << ALT_TYPE_SHIFT)
            | (($facility as u32) << ALT_DATA_SHIFT)
    };
}

#[repr(C)]
pub struct AltInstrBits {
    pub ctx: u32,
    pub type_: u32,
    pub data: u32,
}

#[repr(C)]
pub union AltInstrFeature {
    pub feature: u32,
    pub bits: AltInstrBits,
}

#[repr(C, packed)]
pub struct AltInstr {
    pub instr_offset: i32,
    pub repl_offset: i32,
    pub feature: AltInstrFeature,
    pub instrlen: u8,
}

unsafe extern "C" {
    pub static mut __alt_instructions: AltInstr;
    pub static mut __alt_instructions_end: AltInstr;
    pub fn __apply_alternatives(start: *mut AltInstr, end: *mut AltInstr, ctx: u32);
}

#[inline]
pub unsafe fn apply_alternative_instructions() {
    __apply_alternatives(
        &raw mut __alt_instructions,
        &raw mut __alt_instructions_end,
        ALT_CTX_LATE,
    );
}

#[inline]
pub unsafe fn apply_alternatives(start: *mut AltInstr, end: *mut AltInstr) {
    __apply_alternatives(start, end, ALT_CTX_ALL);
}

/* Assembly label and length helpers from the original header. */
#[macro_export]
macro_rules! b_altinstr { ($num:tt) => { concat!("664", stringify!($num)) }; }
#[macro_export]
macro_rules! e_altinstr { ($num:tt) => { concat!("665", stringify!($num)) }; }
#[macro_export]
macro_rules! oldinstr_len { () => { "662b-661b" }; }
#[macro_export]
macro_rules! altinstr_len { ($num:tt) => { concat!(e_altinstr!($num), "b-", b_altinstr!($num), "b") }; }

/* The following macros preserve the original inline-assembly construction. */
#[macro_export]
macro_rules! OLDINSTR {
    ($oldinstr:expr) => { concat!("661:\n\t", $oldinstr, "\n662:\n") };
}

#[macro_export]
macro_rules! ALTINSTR_REPLACEMENT {
    ($altinstr:expr, $num:tt) => { concat!(b_altinstr!($num), ":\n\t", $altinstr, "\n", e_altinstr!($num), ":\n") };
}

/* C inline-assembly primitives; assembler directives are retained as text. */
#[macro_export]
macro_rules! ALTERNATIVE {
    ($oldinstr:expr, $altinstr:expr, $feature:expr) => {
        concat!(
            ".pushsection .altinstr_replacement, \"ax\"\n",
            ALTINSTR_REPLACEMENT!($altinstr, 1),
            ".popsection\n",
            OLDINSTR!($oldinstr),
            ".pushsection .altinstructions,\"a\"\n",
            ".popsection\n"
        )
    };
}

#[macro_export]
macro_rules! ALTERNATIVE_2 {
    ($oldinstr:expr, $altinstr1:expr, $feature1:expr, $altinstr2:expr, $feature2:expr) => {
        concat!(
            ".pushsection .altinstr_replacement, \"ax\"\n",
            ALTINSTR_REPLACEMENT!($altinstr1, 1),
            ALTINSTR_REPLACEMENT!($altinstr2, 2),
            ".popsection\n",
            OLDINSTR!($oldinstr),
            ".pushsection .altinstructions,\"a\"\n",
            ".popsection\n"
        )
    };
}

/* The assembler-only alt_entry, ALTERNATIVE, and ALTERNATIVE_2 macros are
 * preserved above as string-producing Rust macros; their section layout,
 * labels, and padding directives remain assembler source semantics. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
