/* SPDX-License-Identifier: GPL-2.0 */
// Rust translation of x86/include/asm/bug.h.
// The original Linux includes and configuration conditions are supplied by
// the surrounding translation unit.

#[repr(C)]
pub struct bug_entry {
    _private: [u8; 0],
}

#[cfg(not(feature = "assembler"))]
unsafe extern "C" {
    pub fn __WARN_trap(bug: *mut bug_entry, ...);
}

/* Despite that some emulators terminate on UD2, we use it for WARN(). */
pub const ASM_UD2: &str = "ud2";
pub const INSN_UD2: u16 = 0x0b0f;
pub const LEN_UD2: usize = 2;

pub const ASM_UDB: &str = "\\xd6";
pub const INSN_UDB: u8 = 0xd6;
pub const LEN_UDB: usize = 1;

/* In clang we have UD1s reporting UBSAN failures on X86, 64 and 32bit. */
pub const INSN_ASOP: u8 = 0x67;
pub const INSN_LOCK: u8 = 0xf0;
pub const OPCODE_ESCAPE: u8 = 0x0f;
pub const SECOND_BYTE_OPCODE_UD1: u8 = 0xb9;
pub const SECOND_BYTE_OPCODE_UD2: u8 = 0x0b;

pub const BUG_NONE: u16 = 0xffff;
pub const BUG_UD2: u16 = 0xfffe;
pub const BUG_UD1: u16 = 0xfffd;
pub const BUG_UD1_UBSAN: u16 = 0xfffc;
pub const BUG_UD1_WARN: u16 = 0xfffb;
pub const BUG_UDB: u16 = 0xffd6;
pub const BUG_LOCK: u16 = 0xfff0;

// The following assembly-building macros preserve the source interface and
// intent.  Configuration-dependent expansions are represented by cfg blocks.
#[cfg(feature = "debug_bugverbose")]
#[macro_export]
macro_rules! __BUG_ENTRY_VERBOSE { ($file:expr, $line:expr) => { concat!("\\t.long ", $file, " - .\\t# bug_entry::file\\n", "\\t.word ", $line, "\\t# bug_entry::line\\n") }; }
#[cfg(not(feature = "debug_bugverbose"))]
#[macro_export]
macro_rules! __BUG_ENTRY_VERBOSE { ($file:expr, $line:expr) => { "" }; }

#[cfg(any(target_arch = "x86_64", feature = "debug_bugverbose_detailed"))]
#[macro_export]
macro_rules! __BUG_ENTRY_FORMAT { ($format:expr) => { concat!("\\t.long ", $format, " - .\\t# bug_entry::format\\n") }; }
#[cfg(not(any(target_arch = "x86_64", feature = "debug_bugverbose_detailed")))]
#[macro_export]
macro_rules! __BUG_ENTRY_FORMAT { ($format:expr) => { "" }; }

#[macro_export]
macro_rules! __BUG_ENTRY { ($format:expr, $file:expr, $line:expr, $flags:expr) => {
    concat!("\\t.long 1b - .\\t# bug_entry::bug_addr\\n", __BUG_ENTRY_FORMAT!($format), __BUG_ENTRY_VERBOSE!($file, $line), "\\t.word ", $flags, "\\t# bug_entry::flags\\n")
}; }

#[macro_export]
macro_rules! _BUG_FLAGS_ASM { ($format:expr, $file:expr, $line:expr, $flags:expr, $size:expr, $extra:expr) => {
    concat!(".pushsection __bug_table,\"aw\"\\n\\t", "ANNOTATE_DATA_SPECIAL", "\\n\\t2:\\n\\t", __BUG_ENTRY!($format, $file, $line, $flags), "\\t.org 2b + ", $size, "\\n.popsection\\n", $extra)
}; }

#[cfg(feature = "debug_bugverbose_detailed")]
#[macro_export]
macro_rules! WARN_CONDITION_STR { ($s:expr) => { $s }; }
#[cfg(not(feature = "debug_bugverbose_detailed"))]
#[macro_export]
macro_rules! WARN_CONDITION_STR { ($s:expr) => { "" }; }

#[macro_export]
macro_rules! _BUG_FLAGS { ($cond_str:expr, $ins:expr, $flags:expr, $extra:expr) => {{
    unsafe { core::arch::asm!("1:\t{ins}", ins = const $ins, options(nostack, preserves_flags)); }
}}; }

pub const HAVE_ARCH_BUG: bool = true;

#[macro_export]
macro_rules! BUG { () => {{
    instrumentation_begin!();
    _BUG_FLAGS!("", $crate::ASM_UD2, 0, "");
    unsafe { core::hint::unreachable_unchecked() }
}}; }

#[macro_export]
macro_rules! ARCH_WARN_REACHABLE { () => { "ANNOTATE_REACHABLE(1b)" }; }

#[macro_export]
macro_rules! __WARN_FLAGS { ($cond_str:expr, $flags:expr) => {{
    let __flags = BUGFLAG_WARNING | ($flags);
    instrumentation_begin!();
    _BUG_FLAGS!($cond_str, $crate::ASM_UD2, __flags, ARCH_WARN_REACHABLE!());
    instrumentation_end!();
}}; }

// HAVE_ARCH_BUG_FORMAT_ARGS is enabled on x86_64 in the original header.
#[repr(C)]
pub struct sysv_va_list {
    pub gp_offset: u32,
    pub fp_offset: u32,
    pub overflow_arg_area: *mut core::ffi::c_void,
    pub reg_save_area: *mut core::ffi::c_void,
}
#[repr(C)]
pub struct arch_va_list {
    pub regs: [u64; 6],
    pub args: sysv_va_list,
}

#[cfg(not(feature = "assembler"))]
unsafe extern "C" {
    pub fn __warn_args(args: *mut arch_va_list, regs: *mut pt_regs) -> *mut core::ffi::c_void;
}

#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

#[macro_export]
macro_rules! __WARN_bug_entry { ($flags:expr, $format:expr) => {{
    let mut bug: *mut $crate::bug_entry;
    unsafe { core::arch::asm!("lea (2f)(%rip), {addr}", "1:", addr = out(reg) bug); }
    bug
}}; }

#[macro_export]
macro_rules! __WARN_print_arg { ($flags:expr, $format:expr $(, $arg:expr)*) => {{
    let __flags: i32 = ($flags) | BUGFLAG_WARNING | BUGFLAG_ARGS;
    static_call_mod!(WARN_trap)(__WARN_bug_entry!(__flags, $format) $(, $arg)*);
    unsafe { core::arch::asm!(""); }
}}; }

#[macro_export]
macro_rules! __WARN_printf { ($taint:expr, $fmt:expr $(, $arg:expr)*) => {
    __WARN_print_arg!(BUGFLAG_TAINT!($taint), $fmt $(, $arg)*);
}; }

#[macro_export]
macro_rules! WARN_ONCE { ($cond:expr, $format:expr $(, $arg:expr)*) => {{
    let __ret_warn_on = (($cond) as i32) != 0;
    if __ret_warn_on { __WARN_print_arg!(BUGFLAG_ONCE | BUGFLAG_TAINT!(TAINT_WARN), $format $(, $arg)*); }
    __ret_warn_on
}}; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
