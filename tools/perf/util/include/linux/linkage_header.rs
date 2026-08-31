/* SPDX-License-Identifier: GPL-2.0 */

/* linkage.h ... for including arch/x86/lib/memcpy_64.S */

/* Header guard and C include/preprocessor syntax removed for Rust translation. */

/* Some toolchains use other characters (e.g. '`') to mark new line in macro */
/* Original C fallback: #ifndef ASM_NL */
#[macro_export]
macro_rules! ASM_NL {
    () => {
        ";"
    };
}

/* Original C fallback: #ifndef __ALIGN */
#[macro_export]
macro_rules! __ALIGN {
    () => {
        ".align 4,0x90"
    };
}

pub const __ALIGN_STR: &str = ".align 4,0x90";

/* SYM_T_FUNC -- type used by assembler to mark functions */
/* Original C fallback: #ifndef SYM_T_FUNC */
#[macro_export]
macro_rules! SYM_T_FUNC {
    () => {
        "STT_FUNC"
    };
}

/* SYM_A_* -- align the symbol? */
#[macro_export]
macro_rules! SYM_A_ALIGN {
    () => {
        ALIGN!()
    };
}

/* SYM_L_* -- linkage of symbols */
#[macro_export]
macro_rules! SYM_L_GLOBAL {
    ($name:ident) => {
        concat!(".globl ", stringify!($name))
    };
}

#[macro_export]
macro_rules! SYM_L_WEAK {
    ($name:ident) => {
        concat!(".weak ", stringify!($name))
    };
}

#[macro_export]
macro_rules! SYM_L_LOCAL {
    ($name:ident) => {
        ""
    };
}

#[macro_export]
macro_rules! ALIGN {
    () => {
        __ALIGN!()
    };
}

/* === generic annotations === */

/* SYM_ENTRY -- use only if you have to for non-paired symbols */
/* Original C fallback: #ifndef SYM_ENTRY */
#[macro_export]
macro_rules! SYM_ENTRY {
    ($name:ident, $linkage:ident $(, $align:ident)*) => {
        concat!(
            $linkage!($name),
            ASM_NL!(),
            $($align!(), ASM_NL!(),)*
            stringify!($name),
            ":"
        )
    };
}

/* SYM_START -- use only if you have to */
/* Original C fallback: #ifndef SYM_START */
#[macro_export]
macro_rules! SYM_START {
    ($name:ident, $linkage:ident $(, $align:ident)*) => {
        SYM_ENTRY!($name, $linkage $(, $align)*)
    };
}

/* SYM_END -- use only if you have to */
/* Original C fallback: #ifndef SYM_END */
#[macro_export]
macro_rules! SYM_END {
    ($name:ident, $sym_type:ident) => {
        concat!(
            ".type ",
            stringify!($name),
            " ",
            $sym_type!(),
            ASM_NL!(),
            ".set .L__sym_size_",
            stringify!($name),
            ", .-",
            stringify!($name),
            ASM_NL!(),
            ".size ",
            stringify!($name),
            ", .-",
            stringify!($name)
        )
    };
}

/* SYM_ALIAS -- use only if you have to */
/* Original C fallback: #ifndef SYM_ALIAS */
#[macro_export]
macro_rules! SYM_ALIAS {
    ($alias:ident, $name:ident, $sym_type:ident, $linkage:ident) => {
        concat!(
            $linkage!($alias),
            ASM_NL!(),
            ".set ",
            stringify!($alias),
            ", ",
            stringify!($name),
            ASM_NL!(),
            ".type ",
            stringify!($alias),
            " ",
            $sym_type!(),
            ASM_NL!(),
            ".set .L__sym_size_",
            stringify!($alias),
            ", .L__sym_size_",
            stringify!($name),
            ASM_NL!(),
            ".size ",
            stringify!($alias),
            ", .L__sym_size_",
            stringify!($alias)
        )
    };
}

/* SYM_FUNC_START -- use for global functions */
/* Original C fallback: #ifndef SYM_FUNC_START */
#[macro_export]
macro_rules! SYM_FUNC_START {
    ($name:ident) => {
        SYM_START!($name, SYM_L_GLOBAL, SYM_A_ALIGN)
    };
}

/* SYM_FUNC_START_LOCAL -- use for local functions */
/* Original C fallback: #ifndef SYM_FUNC_START_LOCAL */
#[macro_export]
macro_rules! SYM_FUNC_START_LOCAL {
    ($name:ident) => {
        SYM_START!($name, SYM_L_LOCAL, SYM_A_ALIGN)
    };
}

/* SYM_FUNC_START_WEAK -- use for weak functions */
/* Original C fallback: #ifndef SYM_FUNC_START_WEAK */
#[macro_export]
macro_rules! SYM_FUNC_START_WEAK {
    ($name:ident) => {
        SYM_START!($name, SYM_L_WEAK, SYM_A_ALIGN)
    };
}

/*
 * SYM_FUNC_END -- the end of SYM_FUNC_START_LOCAL, SYM_FUNC_START,
 * SYM_FUNC_START_WEAK, ...
 */
/* Original C fallback: #ifndef SYM_FUNC_END */
#[macro_export]
macro_rules! SYM_FUNC_END {
    ($name:ident) => {
        SYM_END!($name, SYM_T_FUNC)
    };
}

/*
 * SYM_FUNC_ALIAS -- define a global alias for an existing function
 */
/* Original C fallback: #ifndef SYM_FUNC_ALIAS */
#[macro_export]
macro_rules! SYM_FUNC_ALIAS {
    ($alias:ident, $name:ident) => {
        SYM_ALIAS!($alias, $name, SYM_T_FUNC, SYM_L_GLOBAL)
    };
}

/*
 * SYM_FUNC_ALIAS_LOCAL -- define a local alias for an existing function
 */
/* Original C fallback: #ifndef SYM_FUNC_ALIAS_LOCAL */
#[macro_export]
macro_rules! SYM_FUNC_ALIAS_LOCAL {
    ($alias:ident, $name:ident) => {
        SYM_ALIAS!($alias, $name, SYM_T_FUNC, SYM_L_LOCAL)
    };
}

/*
 * SYM_FUNC_ALIAS_WEAK -- define a weak global alias for an existing function
 */
/* Original C fallback: #ifndef SYM_FUNC_ALIAS_WEAK */
#[macro_export]
macro_rules! SYM_FUNC_ALIAS_WEAK {
    ($alias:ident, $name:ident) => {
        SYM_ALIAS!($alias, $name, SYM_T_FUNC, SYM_L_WEAK)
    };
}

/* Original C fallback: #ifndef SYM_FUNC_ALIAS_MEMFUNC */
#[macro_export]
macro_rules! SYM_FUNC_ALIAS_MEMFUNC {
    ($alias:ident, $name:ident) => {
        SYM_FUNC_ALIAS!($alias, $name)
    };
}

// In the kernel sources (include/linux/cfi_types.h), this has a different
// definition when CONFIG_CFI is used, for tools/ just use the !cfi
// definition:
/* Original C fallback: #ifndef SYM_TYPED_START */
#[macro_export]
macro_rules! SYM_TYPED_START {
    ($name:ident, $linkage:ident $(, $align:ident)*) => {
        SYM_START!($name, $linkage $(, $align)*)
    };
}

/* Original C fallback: #ifndef SYM_TYPED_FUNC_START */
#[macro_export]
macro_rules! SYM_TYPED_FUNC_START {
    ($name:ident) => {
        SYM_TYPED_START!($name, SYM_L_GLOBAL, SYM_A_ALIGN)
    };
}

/* Original C fallback: #ifndef SYM_PIC_ALIAS */
#[macro_export]
macro_rules! SYM_PIC_ALIAS {
    ($sym:ident) => {
        concat!(
            ".globl __pi_",
            stringify!($sym),
            ASM_NL!(),
            ".set __pi_",
            stringify!($sym),
            ", ",
            stringify!($sym),
            ASM_NL!(),
            ".type __pi_",
            stringify!($sym),
            " ",
            SYM_T_FUNC!(),
            ASM_NL!(),
            ".set .L__sym_size___pi_",
            stringify!($sym),
            ", .L__sym_size_",
            stringify!($sym),
            ASM_NL!(),
            ".size __pi_",
            stringify!($sym),
            ", .L__sym_size___pi_",
            stringify!($sym)
        )
    };
}
