/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation.

pub const ALT_FLAGS_SHIFT: u32 = 16;
pub const ALT_FLAG_NOT: u32 = 1 << 0;
pub const ALT_FLAG_DIRECT_CALL: u32 = 1 << 1;

#[inline]
pub const fn ALT_NOT(feature: u32) -> u32 {
    (ALT_FLAG_NOT << ALT_FLAGS_SHIFT) | feature
}

#[inline]
pub const fn ALT_DIRECT_CALL(feature: u32) -> u32 {
    (ALT_FLAG_DIRECT_CALL << ALT_FLAGS_SHIFT) | feature
}

// X86_FEATURE_ALWAYS is supplied by the architecture feature definitions.
pub const ALT_CALL_ALWAYS: u32 = ALT_DIRECT_CALL(X86_FEATURE_ALWAYS);

#[repr(C)]
#[derive(Clone, Copy)]
pub union AltInstrFlags {
    pub ft_flags: u32,
    pub cpuid_flags: AltInstrCpuidFlags,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct AltInstrCpuidFlags {
    // C bit-fields: cpuid:16, flags:16.
    pub cpuid: u16,
    pub flags: u16,
}

#[repr(C, packed)]
pub struct alt_instr {
    pub instr_offset: i32,
    pub repl_offset: i32,
    pub flags: AltInstrFlags,
    pub instrlen: u8,
    pub replacementlen: u8,
}

unsafe extern "C" {
    pub static mut __alt_instructions: [alt_instr; 0];
    pub static mut __alt_instructions_end: [alt_instr; 0];
    pub static mut __retpoline_sites: [i32; 0];
    pub static mut __retpoline_sites_end: [i32; 0];
    pub static mut __return_sites: [i32; 0];
    pub static mut __return_sites_end: [i32; 0];
    pub static mut __cfi_sites: [i32; 0];
    pub static mut __cfi_sites_end: [i32; 0];
    pub static mut __ibt_endbr_seal: [i32; 0];
    pub static mut __ibt_endbr_seal_end: [i32; 0];
    pub static mut alternatives_patched: i32;

    pub fn alternative_instructions();
    pub fn apply_alternatives(start: *mut alt_instr, end: *mut alt_instr);
    pub fn apply_retpolines(start: *mut i32, end: *mut i32);
    pub fn apply_returns(start: *mut i32, end: *mut i32);
    pub fn apply_seal_endbr(start: *mut i32, end: *mut i32);
    pub fn apply_fineibt(start_retpoline: *mut i32, end_retpoine: *mut i32,
                         start_cfi: *mut i32, end_cfi: *mut i32);
}

pub struct module;

#[repr(C)]
pub struct callthunk_sites {
    pub call_start: *mut i32,
    pub call_end: *mut i32,
}

// CONFIG_CALL_THUNKS selects the external declarations; otherwise these are inline no-ops.
#[inline]
pub unsafe fn callthunks_patch_builtin_calls() {}
#[inline]
pub unsafe fn callthunks_patch_module_calls(_sites: *mut callthunk_sites, _mod: *mut module) {}
#[inline]
pub unsafe fn callthunks_translate_call_dest(dest: *mut core::ffi::c_void) -> *mut core::ffi::c_void { dest }
#[inline]
pub unsafe fn x86_call_depth_emit_accounting(_pprog: *mut *mut u8, _func: *mut core::ffi::c_void, _ip: *mut core::ffi::c_void) -> i32 { 0 }

// CONFIG_MITIGATION_ITS selects the external declarations; otherwise these are inline stubs.
#[inline]
pub unsafe fn its_init_mod(_mod: *mut module) {}
#[inline]
pub unsafe fn its_fini_mod(_mod: *mut module) {}
#[inline]
pub unsafe fn its_free_mod(_mod: *mut module) {}
#[inline]
pub unsafe fn its_static_thunk(_reg: i32) -> *mut u8 { core::ptr::null_mut() }

// CONFIG_MITIGATION_RETHUNK and CONFIG_OBJTOOL select the external declarations.
#[inline]
pub unsafe fn cpu_wants_rethunk() -> bool { false }
#[inline]
pub unsafe fn cpu_wants_rethunk_at(_addr: *mut core::ffi::c_void) -> bool { false }

pub const ALT_CALL_INSTR: &str = "call BUG_func";
pub const alt_slen: &str = "772b-771b";
pub const alt_total_slen: &str = "773b-771b";
pub const alt_rlen: &str = "775f-774f";

/*
 * Assembler text for the alternative-instruction macros.  Rust's `concat!`
 * cannot splice constants, so each feature/flags word is an asm `const`
 * operand whose name is passed as a string literal, e.g.:
 *
 *     asm!(ALTERNATIVE!("rep movsb", "call rep_movs_alternative", "ft"),
 *          ft = const ALT_NOT(X86_FEATURE_FSRM), ...)
 *
 * ALT_INSTR_SIZE is 14 (sizeof(struct alt_instr), asm-offsets).
 */
#[macro_export]
macro_rules! OLDINSTR {
    ($oldinstr:expr) => {
        concat!(
            "# ALT: oldinstr\n",
            "771:\n\t", $oldinstr, "\n772:\n",
            "# ALT: padding\n",
            ".skip -(((775f-774f)-(772b-771b)) > 0) * ((775f-774f)-(772b-771b)),0x90\n",
            "773:\n"
        )
    };
}

#[macro_export]
macro_rules! ALTINSTR_ENTRY {
    ($ft_flags:literal) => {
        concat!(
            ".pushsection .altinstructions, \"aM\", @progbits, 14\n",
            " .long 771b - .\n",
            " .long 774f - .\n",
            " .4byte {", $ft_flags, "}\n",
            " .byte 773b-771b\n",
            " .byte 775f-774f\n",
            ".popsection\n"
        )
    };
}

#[cfg(CONFIG_OBJTOOL)]
#[macro_export]
macro_rules! ANNOTATE_DATA_SPECIAL {
    () => {
        "912: .pushsection .discard.annotate_data, \"M\", @progbits, 8; .long 912b - ., 1; .popsection"
    };
}
#[cfg(not(CONFIG_OBJTOOL))]
#[macro_export]
macro_rules! ANNOTATE_DATA_SPECIAL {
    () => {
        ""
    };
}

#[macro_export]
macro_rules! ALTINSTR_REPLACEMENT {
    ($newinstr:expr) => {
        concat!(
            ".pushsection .altinstr_replacement, \"ax\"\n",
            ANNOTATE_DATA_SPECIAL!(), "\n",
            "# ALT: replacement\n",
            "774:\n\t", $newinstr, "\n775:\n",
            ".popsection\n"
        )
    };
}

#[macro_export]
macro_rules! ALTERNATIVE {
    ($oldinstr:expr, $newinstr:expr, $ft_flags:literal) => {
        concat!(
            OLDINSTR!($oldinstr),
            ALTINSTR_ENTRY!($ft_flags),
            ALTINSTR_REPLACEMENT!($newinstr)
        )
    };
}

#[macro_export]
macro_rules! ALTERNATIVE_2 {
    ($oldinstr:expr, $newinstr1:expr, $ft_flags1:literal, $newinstr2:expr, $ft_flags2:literal) => {
        ALTERNATIVE!(ALTERNATIVE!($oldinstr, $newinstr1, $ft_flags1), $newinstr2, $ft_flags2)
    };
}

/// The caller binds `$ft_always` to `const X86_FEATURE_ALWAYS`.
#[macro_export]
macro_rules! ALTERNATIVE_TERNARY {
    ($oldinstr:expr, $ft_flags:literal, $newinstr_yes:expr, $newinstr_no:expr, $ft_always:literal) => {
        ALTERNATIVE_2!($oldinstr, $newinstr_no, $ft_always, $newinstr_yes, $ft_flags)
    };
}

#[macro_export]
macro_rules! ALTERNATIVE_3 {
    ($oldinstr:expr, $newinstr1:expr, $ft_flags1:literal, $newinstr2:expr, $ft_flags2:literal,
     $newinstr3:expr, $ft_flags3:literal) => {
        ALTERNATIVE!(
            ALTERNATIVE_2!($oldinstr, $newinstr1, $ft_flags1, $newinstr2, $ft_flags2),
            $newinstr3,
            $ft_flags3
        )
    };
}

unsafe extern "C" {
    pub fn BUG_func();
    pub fn nop_func();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
