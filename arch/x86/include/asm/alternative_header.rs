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
pub union AltInstrFlags {
    pub ft_flags: u32,
    pub cpuid_flags: AltInstrCpuidFlags,
}

#[repr(C)]
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

// The following assembly primitives are retained as token macros for consumers that provide
// the kernel assembler symbols and configuration constants.
#[macro_export]
macro_rules! OLDINSTR { ($oldinstr:tt) => { concat!("# ALT: oldinstr\n771:\n\t", $oldinstr, "\n772:\n") }; }
#[macro_export]
macro_rules! ALTINSTR_ENTRY { ($ft_flags:tt) => { $ft_flags }; }
#[macro_export]
macro_rules! ALTINSTR_REPLACEMENT { ($newinstr:tt) => { $newinstr }; }
#[macro_export]
macro_rules! ALTERNATIVE { ($oldinstr:tt, $newinstr:tt, $ft_flags:tt) => { ($oldinstr, $newinstr, $ft_flags) }; }
#[macro_export]
macro_rules! ALTERNATIVE_2 { ($oldinstr:tt, $newinstr1:tt, $ft_flags1:tt, $newinstr2:tt, $ft_flags2:tt) => { ($oldinstr, $newinstr1, $ft_flags1, $newinstr2, $ft_flags2) }; }
#[macro_export]
macro_rules! ALTERNATIVE_TERNARY { ($oldinstr:tt, $ft_flags:tt, $newinstr_yes:tt, $newinstr_no:tt) => { $crate::ALTERNATIVE_2!($oldinstr, $newinstr_no, X86_FEATURE_ALWAYS, $newinstr_yes, $ft_flags) }; }
#[macro_export]
macro_rules! ALTERNATIVE_3 { ($oldinstr:tt, $newinstr1:tt, $ft_flags1:tt, $newinstr2:tt, $ft_flags2:tt, $newinstr3:tt, $ft_flags3:tt) => { $crate::ALTERNATIVE!($crate::ALTERNATIVE_2!($oldinstr, $newinstr1, $ft_flags1, $newinstr2, $ft_flags2), $newinstr3, $ft_flags3) }; }

unsafe extern "C" {
    pub fn BUG_func();
    pub fn nop_func();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
