/* SPDX-License-Identifier: GPL-2.0 */

// The C header includes linux/linkage.h, linux/types.h, asm/opcodes.h, and
// asm-generic/bug.h. Their declarations are supplied by other translations.

/*
 * Use a suitable undefined instruction to use for ARM/Thumb2 bug handling.
 * We need to be careful not to conflict with those used by other modules and
 * the register_undef_hook() system.
 */
#[cfg(CONFIG_THUMB2_KERNEL)]
pub const BUG_INSTR_VALUE: u32 = 0xde02;
#[cfg(not(CONFIG_THUMB2_KERNEL))]
pub const BUG_INSTR_VALUE: u32 = 0xe7f001f2;

#[cfg(CONFIG_THUMB2_KERNEL)]
macro_rules! BUG_INSTR {
    ($value:expr) => { __inst_thumb16($value) };
}
#[cfg(not(CONFIG_THUMB2_KERNEL))]
macro_rules! BUG_INSTR {
    ($value:expr) => { __inst_arm($value) };
}

macro_rules! BUG {
    () => { _BUG!(file!(), line!(), BUG_INSTR_VALUE) };
}

macro_rules! _BUG {
    ($file:expr, $line:expr, $value:expr) => { __BUG!($file, $line, $value) };
}

// The original implementation emits architecture-specific inline assembly
// and records verbose bug information when CONFIG_DEBUG_BUGVERBOSE is set.
#[cfg(CONFIG_DEBUG_BUGVERBOSE)]
macro_rules! __BUG {
    ($file:expr, $line:expr, $value:expr) => {{
        unsafe {
            core::arch::asm!(
                "1:",
                "{instr}",
                ".pushsection .rodata.str, \"aMS\", %progbits, 1",
                "2:\t.asciz {file}",
                ".popsection",
                ".pushsection __bug_table,\"aw\"",
                ".align 2",
                "3:\t.word 1b, 2b",
                "\t.hword {line}, 0",
                ".popsection",
                instr = const BUG_INSTR!($value),
                file = sym $file,
                line = const $line,
            );
        }
        core::hint::unreachable_unchecked()
    }};
}

#[cfg(not(CONFIG_DEBUG_BUGVERBOSE))]
macro_rules! __BUG {
    ($file:expr, $line:expr, $value:expr) => {{
        unsafe {
            core::arch::asm!("{instr}", instr = const BUG_INSTR!($value));
        }
        core::hint::unreachable_unchecked()
    }};
}

pub const HAVE_ARCH_BUG: bool = true;

#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn die(msg: *const core::ffi::c_char, regs: *mut pt_regs, err: i32);
    pub fn arm_notify_die(
        str_: *const core::ffi::c_char,
        regs: *mut pt_regs,
        signo: i32,
        si_code: i32,
        addr: *mut core::ffi::c_void,
        err: usize,
        trap: usize,
    );
}

#[cfg(CONFIG_ARM_LPAE)]
pub const FAULT_CODE_ALIGNMENT: i32 = 33;
#[cfg(not(CONFIG_ARM_LPAE))]
pub const FAULT_CODE_ALIGNMENT: i32 = 1;
#[cfg(CONFIG_ARM_LPAE)]
pub const FAULT_CODE_DEBUG: i32 = 34;
#[cfg(not(CONFIG_ARM_LPAE))]
pub const FAULT_CODE_DEBUG: i32 = 2;

pub type FaultHandler = unsafe extern "C" fn(usize, u32, *mut pt_regs) -> i32;

unsafe extern "C" {
    pub fn hook_fault_code(nr: i32, f: FaultHandler, sig: i32, code: i32, name: *const core::ffi::c_char);
    pub fn hook_ifault_code(nr: i32, f: FaultHandler, sig: i32, code: i32, name: *const core::ffi::c_char);
    pub fn c_backtrace(fp: usize, pmode: i32, loglvl: *const core::ffi::c_char);
}

#[repr(C)]
pub struct mm_struct {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn show_pte(lvl: *const core::ffi::c_char, mm: *mut mm_struct, addr: usize);
    pub fn __show_regs(regs: *mut pt_regs);
    pub fn __show_regs_alloc_free(regs: *mut pt_regs);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
