/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Tell the user there is some problem.
 * The offending file and line are encoded in the __bug_table section.
 *
 * The C header guard and include directives are intentionally omitted.
 * Configuration conditions are retained as Rust cfg conditions.
 */

#[cfg(feature = "CONFIG_BUG")]
pub const HAVE_ARCH_BUG: bool = true;
#[cfg(feature = "CONFIG_BUG")]
pub const HAVE_ARCH_WARN_ON: bool = true;

/* the break instruction is used as BUG() marker. */
#[cfg(feature = "CONFIG_BUG")]
pub const PARISC_BUG_BREAK_ASM: &str = "break 0x1f, 0x1fff";
#[cfg(feature = "CONFIG_BUG")]
pub const PARISC_BUG_BREAK_INSN: u32 = 0x03ffe01f;

#[cfg(all(feature = "CONFIG_BUG", feature = "CONFIG_GENERIC_BUG_RELATIVE_POINTERS"))]
#[macro_export]
macro_rules! __BUG_REL {
    ($val:expr) => {
        concat!(".word ", stringify!($val), " - .")
    };
}

#[cfg(all(feature = "CONFIG_BUG", not(feature = "CONFIG_GENERIC_BUG_RELATIVE_POINTERS")))]
#[macro_export]
macro_rules! __BUG_REL {
    ($val:expr) => {
        concat!(".word ", stringify!($val))
    };
}

#[cfg(all(feature = "CONFIG_BUG", feature = "CONFIG_DEBUG_BUGVERBOSE"))]
#[macro_export]
macro_rules! BUG {
    () => {{
        unsafe {
            core::arch::asm!("break 0x1f, 0x1fff", options(noreturn));
        }
    }};
}

#[cfg(all(feature = "CONFIG_BUG", not(feature = "CONFIG_DEBUG_BUGVERBOSE")))]
#[macro_export]
macro_rules! BUG {
    () => {{
        unsafe {
            core::arch::asm!("break 0x1f, 0x1fff", options(noreturn));
        }
    }};
}

#[cfg(all(feature = "CONFIG_BUG", feature = "CONFIG_DEBUG_BUGVERBOSE"))]
#[macro_export]
macro_rules! __WARN_FLAGS {
    ($cond_str:expr, $flags:expr) => {{
        unsafe {
            core::arch::asm!(
                "break 0x1f, 0x1fff",
                options(nostack)
            );
        }
    }};
}

#[cfg(all(feature = "CONFIG_BUG", not(feature = "CONFIG_DEBUG_BUGVERBOSE")))]
#[macro_export]
macro_rules! __WARN_FLAGS {
    ($cond_str:expr, $flags:expr) => {{
        unsafe {
            core::arch::asm!(
                "break 0x1f, 0x1fff",
                options(nostack)
            );
        }
    }};
}

#[cfg(feature = "CONFIG_BUG")]
#[macro_export]
macro_rules! WARN_ON {
    ($x:expr) => {{
        let __ret_warn_on: i32 = (($x) != 0) as i32;
        if __ret_warn_on != 0 {
            $crate::__WARN_FLAGS!(stringify!($x), 0);
        }
        __ret_warn_on != 0
    }};
}

/* Declarations supplied by asm-generic/bug.h remain external dependencies. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
