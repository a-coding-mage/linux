/* SPDX-License-Identifier: GPL-2.0 */

// C header guard: _ASM_S390_BUG_H
// Dependencies supplied by the surrounding translation unit:
// linux/compiler.h, linux/const.h, linux/stringify.h, asm-generic/bug.h

pub const MONCODE_BUG: u32 = 0;
pub const MONCODE_BUG_ARG: u32 = 1;

// The following items correspond to the CONFIG_BUG and
// CONFIG_CC_HAS_ASM_IMMEDIATE_STRINGS conditional section in the C header.

#[cfg(feature = "CONFIG_DEBUG_BUGVERBOSE")]
macro_rules! __BUG_ENTRY_VERBOSE {
    ($file:expr, $line:expr) => {
        concat!(
            "\t.long\t", $file, " - .\t# bug_entry::file\n",
            "\t.short\t", $line, "\t# bug_entry::line\n"
        )
    };
}

#[cfg(not(feature = "CONFIG_DEBUG_BUGVERBOSE"))]
macro_rules! __BUG_ENTRY_VERBOSE {
    ($file:expr, $line:expr) => { "" };
}

#[cfg(feature = "CONFIG_DEBUG_BUGVERBOSE_DETAILED")]
macro_rules! WARN_CONDITION_STR {
    ($cond_str:expr) => { $cond_str };
}

#[cfg(not(feature = "CONFIG_DEBUG_BUGVERBOSE_DETAILED"))]
macro_rules! WARN_CONDITION_STR {
    ($cond_str:expr) => { "" };
}

macro_rules! __BUG_ENTRY {
    ($format:expr, $file:expr, $line:expr, $flags:expr, $size:expr) => {
        concat!(
            "\t.section __bug_table,\"aw\"\n",
            "1:\t.long\t0b - .\t\t# bug_entry::bug_addr\n",
            "\t.long\t", $format, " - .\t# bug_entry::format\n",
            __BUG_ENTRY_VERBOSE!($file, $line),
            "\t.short\t", $flags, "\t\t# bug_entry::flags\n",
            "\t.org\t1b+", $size, "\n",
            "\t.previous"
        )
    };
}

// C inline assembly is retained as an explicit low-level operation for the
// target-specific s390 implementation.
macro_rules! __BUG_ASM {
    ($cond_str:expr, $flags:expr) => {{
        unsafe {
            core::arch::asm!(
                "0: mc {monc}(r0),0",
                monc = const MONCODE_BUG,
                options(nostack)
            );
        }
    }};
}

macro_rules! BUG {
    () => {{
        __BUG_ASM!("", 0);
        unsafe { core::hint::unreachable_unchecked() }
    }};
}

macro_rules! __WARN_FLAGS {
    ($cond_str:expr, $flags:expr) => {{
        __BUG_ASM!($cond_str, BUGFLAG_WARNING | ($flags));
    }};
}

#[repr(C)]
pub struct arch_va_list {
    pub __gpr: libc::c_long,
    pub __fpr: libc::c_long,
    pub __overflow_arg_area: *mut core::ffi::c_void,
    pub __reg_save_area: *mut core::ffi::c_void,
}

pub enum bug_entry {}
pub enum pt_regs {}

unsafe extern "C" {
    pub fn __warn_args(args: *mut arch_va_list, regs: *mut pt_regs) -> *mut core::ffi::c_void;
    pub fn __WARN_trap(bug: *mut bug_entry, ...);
}

macro_rules! __WARN_bug_entry {
    ($flags:expr, $format:expr) => {{
        // The C implementation obtains the bug_entry address through s390
        // inline assembly and emits a __bug_table entry.
        let bug: *mut bug_entry;
        unsafe {
            core::arch::asm!(
                "larl {bug},1f",
                "1:",
                bug = lateout(reg) bug,
                options(nostack)
            );
        }
        bug
    }};
}

macro_rules! __WARN_print_arg {
    ($flags:expr, $format:expr $(, $arg:expr)*) => {{
        let __flags = ($flags) | BUGFLAG_WARNING | BUGFLAG_ARGS;
        unsafe { __WARN_trap(__WARN_bug_entry!(__flags, $format) $(, $arg)*); }
        unsafe { core::arch::asm!("", options(nostack, preserves_flags)); }
    }};
}

macro_rules! __WARN_printf {
    ($taint:expr, $fmt:expr $(, $arg:expr)*) => {
        __WARN_print_arg!(BUGFLAG_TAINT($taint), $fmt $(, $arg)*);
    };
}

macro_rules! WARN_ONCE {
    ($cond:expr, $format:expr $(, $arg:expr)*) => {{
        let __ret_warn_on = (($cond) as i32 != 0) as i32;
        if __ret_warn_on != 0 {
            __WARN_print_arg!(BUGFLAG_ONCE | BUGFLAG_TAINT(TAINT_WARN), $format $(, $arg)*);
        }
        __ret_warn_on
    }};
}

pub const HAVE_ARCH_BUG: bool = true;
pub const HAVE_ARCH_BUG_FORMAT: bool = true;
pub const HAVE_ARCH_BUG_FORMAT_ARGS: bool = true;

macro_rules! ARCH_WARN_ASM {
    ($file:expr, $line:expr, $flags:expr, $size:expr) => {
        concat!(
            ".section .rodata.str,\"aMS\",@progbits,1\n",
            "9:\n",
            ".asciz \"\"\n",
            ".previous\n",
            "0:\n",
            "mc 0(%r0),0\n",
            __BUG_ENTRY!("9b", $file, $line, $flags, $size)
        )
    };
}

pub const ARCH_WARN_REACHABLE: bool = true;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
