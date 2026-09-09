/* SPDX-License-Identifier: GPL-2.0 */

// PowerPC bug support.  The original header is active only in the kernel
// build; assembler-only sections and compiler configuration conditionals are
// retained here as comments where Rust has no direct equivalent.

#[cfg(feature = "config_bug")]
#[macro_export]
macro_rules! bug_entry {
    ($cond_str:expr, $insn:expr, $flags:expr $(, $extra:expr)*) => {{
        // The C implementation emits a volatile inline-assembly bug-table
        // entry containing the instruction, source file, line, flags, and
        // bug_entry size.  Inline assembly is supplied by the target kernel.
        let _ = ($cond_str, $insn, $flags);
        $(let _ = $extra;)*
    }};
}

#[cfg(feature = "config_bug")]
#[macro_export]
macro_rules! bug {
    () => {{
        $crate::bug_entry!("", "twi 31, 0, 0", 0);
        unsafe { core::hint::unreachable_unchecked() }
    }};
}

#[cfg(feature = "config_bug")]
#[macro_export]
macro_rules! warn_flags {
    ($cond_str:expr, $flags:expr) => {
        $crate::bug_entry!($cond_str, "twi 31, 0, 0", BUGFLAG_WARNING | ($flags))
    };
}

// BUG_ON and WARN_ON have architecture-specific constant-folding and inline
// assembly behavior in C.  Their Rust equivalents preserve the observable
// control flow; target-specific constants and warning helpers are external.
#[cfg(all(feature = "config_bug", target_pointer_width = "64"))]
#[macro_export]
macro_rules! bug_on {
    ($x:expr) => {{
        let __bug_on_value = $x;
        if __bug_on_value {
            $crate::bug!();
        }
    }};
}

#[cfg(all(feature = "config_bug", target_pointer_width = "64"))]
#[macro_export]
macro_rules! warn_on {
    ($x:expr) => {{
        let __ret_warn_on: i32 = if $x { 1 } else { 0 };
        if __ret_warn_on != 0 {
            // __WARN() / BUGFLAG_TAINT(TAINT_WARN) are supplied by the
            // generic kernel warning implementation.
        }
        __ret_warn_on != 0
    }};
}

// The C header aliases EMIT_WARN_ENTRY to EMIT_BUG_ENTRY.  The assembler
// implementation emits __bug_table records (and, in verbose builds, the
// source file and line); the Rust build provides this through its target ABI.
#[macro_export]
macro_rules! emit_warn_entry {
    ($($args:tt)*) => { $crate::emit_bug_entry!($($args)*) };
}

// Declarations from the non-assembler portion of the header.
#[repr(C)]
pub struct PtRegs {
    _private: [u8; 0],
}

extern "C" {
    pub fn hash__do_page_fault(regs: *mut PtRegs);
    pub fn bad_page_fault(regs: *mut PtRegs, err: i32);
    pub fn emulate_single_step(regs: *mut PtRegs);
    pub fn _exception(regs: *mut PtRegs, err: i32, address: usize);
    pub fn _exception_pkey(regs: *mut PtRegs, address: usize, err: i32);
    pub fn die(str_: *const core::ffi::c_char, regs: *mut PtRegs, err: isize);
    pub fn die_mce(str_: *const core::ffi::c_char, regs: *mut PtRegs, err: isize);
    pub fn die_will_crash() -> bool;
    pub fn panic_flush_kmsg_start();
    pub fn panic_flush_kmsg_end();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
