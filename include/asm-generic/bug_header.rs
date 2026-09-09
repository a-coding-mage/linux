/* SPDX-License-Identifier: GPL-2.0 */

// Translated from asm-generic/bug.h.  C preprocessor configuration is
// preserved with Rust cfg attributes where a direct mapping is available.

pub const CUT_HERE: &str = "------------[ cut here ]------------\n";

#[cfg(CONFIG_GENERIC_BUG)]
pub const BUGFLAG_WARNING: u32 = 1 << 0;
#[cfg(CONFIG_GENERIC_BUG)]
pub const BUGFLAG_ONCE: u32 = 1 << 1;
#[cfg(CONFIG_GENERIC_BUG)]
pub const BUGFLAG_DONE: u32 = 1 << 2;
#[cfg(CONFIG_GENERIC_BUG)]
pub const BUGFLAG_NO_CUT_HERE: u32 = 1 << 3;
#[cfg(CONFIG_GENERIC_BUG)]
pub const BUGFLAG_ARGS: u32 = 1 << 4;
#[cfg(CONFIG_GENERIC_BUG)]
pub const BUGFLAG_TAINT_SHIFT: u32 = 8;
#[cfg(CONFIG_GENERIC_BUG)]
#[inline]
pub const fn BUGFLAG_TAINT(taint: u32) -> u32 { taint << BUGFLAG_TAINT_SHIFT }
#[cfg(CONFIG_GENERIC_BUG)]
#[inline]
pub const fn BUG_GET_TAINT(flags: u16) -> u16 { flags >> 8 }

#[repr(C)]
pub struct warn_args {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

extern "C" {
    pub fn __warn(
        file: *const core::ffi::c_char,
        line: i32,
        caller: *mut core::ffi::c_void,
        taint: u32,
        regs: *mut pt_regs,
        args: *mut warn_args,
    );

    pub fn warn_slowpath_fmt(
        file: *const core::ffi::c_char,
        line: i32,
        taint: u32,
        fmt: *const core::ffi::c_char,
        ...
    );

    pub fn __warn_printk(fmt: *const core::ffi::c_char, ...);
}

#[cfg(CONFIG_GENERIC_BUG)]
#[repr(C)]
pub struct bug_entry {
    #[cfg(not(CONFIG_GENERIC_BUG_RELATIVE_POINTERS))]
    pub bug_addr: u64,
    #[cfg(CONFIG_GENERIC_BUG_RELATIVE_POINTERS)]
    pub bug_addr_disp: i32,
    #[cfg(HAVE_ARCH_BUG_FORMAT)]
    #[cfg(not(CONFIG_GENERIC_BUG_RELATIVE_POINTERS))]
    pub format: *const core::ffi::c_char,
    #[cfg(HAVE_ARCH_BUG_FORMAT)]
    #[cfg(CONFIG_GENERIC_BUG_RELATIVE_POINTERS)]
    pub format_disp: i32,
    #[cfg(CONFIG_DEBUG_BUGVERBOSE)]
    #[cfg(not(CONFIG_GENERIC_BUG_RELATIVE_POINTERS))]
    pub file: *const core::ffi::c_char,
    #[cfg(CONFIG_DEBUG_BUGVERBOSE)]
    #[cfg(CONFIG_GENERIC_BUG_RELATIVE_POINTERS)]
    pub file_disp: i32,
    #[cfg(CONFIG_DEBUG_BUGVERBOSE)]
    pub line: u16,
    pub flags: u16,
}

#[cfg(CONFIG_BUG)]
#[macro_export]
macro_rules! BUG {
    () => {{
        unsafe {
            $crate::printk!("BUG: failure at %s:%d/%s()!\n", file!(), line!(), module_path!());
            $crate::barrier_before_unreachable();
            $crate::panic!("BUG!");
        }
    }};
}

#[cfg(not(CONFIG_BUG))]
#[macro_export]
macro_rules! BUG {
    () => {{
        loop {}
    }};
}

#[macro_export]
macro_rules! BUG_ON {
    ($condition:expr) => {{
        if $condition { $crate::BUG!(); }
    }};
}

#[macro_export]
macro_rules! WARN_ON {
    ($condition:expr) => {{
        let __ret_warn_on: i32 = if $condition { 1 } else { 0 };
        __ret_warn_on != 0
    }};
}

#[macro_export]
macro_rules! WARN {
    ($condition:expr $(, $format:expr $(, $arg:expr)*)?) => {{
        let __ret_warn_on: i32 = if $condition { 1 } else { 0 };
        let _ = (__ret_warn_on $(, $format $(, $arg)*)?);
        __ret_warn_on != 0
    }};
}

#[macro_export]
macro_rules! WARN_TAINT {
    ($condition:expr, $taint:expr $(, $format:expr $(, $arg:expr)*)?) => {{
        let __ret_warn_on: i32 = if $condition { 1 } else { 0 };
        let _ = ($taint, __ret_warn_on $(, $format $(, $arg)*)?);
        __ret_warn_on != 0
    }};
}

#[macro_export]
macro_rules! WARN_ON_ONCE {
    ($condition:expr) => { $crate::WARN_ON!($condition) };
}

#[macro_export]
macro_rules! WARN_ONCE {
    ($condition:expr $(, $format:expr $(, $arg:expr)*)?) => {
        $crate::WARN!($condition $(, $format $(, $arg)*)?)
    };
}

#[macro_export]
macro_rules! WARN_TAINT_ONCE {
    ($condition:expr, $taint:expr $(, $format:expr $(, $arg:expr)*)?) => {
        $crate::WARN_TAINT!($condition, $taint $(, $format $(, $arg)*)?)
    };
}

#[cfg(CONFIG_SMP)]
#[macro_export]
macro_rules! WARN_ON_SMP { ($x:expr) => { $crate::WARN_ON!($x) }; }

#[cfg(not(CONFIG_SMP))]
#[macro_export]
macro_rules! WARN_ON_SMP { ($x:expr) => {{ let _ = &$x; false }}; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
