/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2017  ARM Limited
 *
 * C header translation.  The original include and configuration preprocessor
 * conditions are represented by external Rust dependencies and cfg comments.
 */

/* Supplied by the translated asm/brk-imm.h dependency. */
extern "Rust" {
    static BUG_BRK_IMM: i32;
}

/* CONFIG_DEBUG_BUGVERBOSE */
#[cfg(CONFIG_DEBUG_BUGVERBOSE)]
#[macro_export]
macro_rules! _BUGVERBOSE_LOCATION {
    ($file:expr, $line:expr) => {
        concat!(
            ".pushsection .rodata.str,\"aMS\",@progbits,1;",
            "14472:.string ", $file, ";",
            ".popsection;",
            ".long 14472b - .;",
            ".short ", $line, ";"
        )
    };
}

#[cfg(not(CONFIG_DEBUG_BUGVERBOSE))]
#[macro_export]
macro_rules! _BUGVERBOSE_LOCATION {
    ($file:expr, $line:expr) => { "" };
}

/* CONFIG_GENERIC_BUG */
#[cfg(CONFIG_GENERIC_BUG)]
#[macro_export]
macro_rules! __BUG_ENTRY_START {
    () => {
        ".pushsection __bug_table,\"aw\"; .align 2; 14470:.long 14471f - .;"
    };
}

#[cfg(CONFIG_GENERIC_BUG)]
#[macro_export]
macro_rules! __BUG_ENTRY_END {
    () => { ".align 2; .popsection; 14471:" };
}

#[cfg(CONFIG_GENERIC_BUG)]
#[macro_export]
macro_rules! __BUG_ENTRY {
    ($flags:expr) => {
        concat!(
            __BUG_ENTRY_START!(),
            _BUGVERBOSE_LOCATION!(file!(), line!()),
            ".short ", $flags, ";",
            __BUG_ENTRY_END!()
        )
    };
}

#[cfg(not(CONFIG_GENERIC_BUG))]
#[macro_export]
macro_rules! __BUG_ENTRY {
    ($flags:expr) => { "" };
}

#[macro_export]
macro_rules! ASM_BUG_FLAGS {
    ($flags:expr) => {
        concat!(__BUG_ENTRY!($flags), "brk ", BUG_BRK_IMM)
    };
}

#[macro_export]
macro_rules! ASM_BUG {
    () => { ASM_BUG_FLAGS!(0) };
}

/* CONFIG_DEBUG_BUGVERBOSE */
#[cfg(CONFIG_DEBUG_BUGVERBOSE)]
#[macro_export]
macro_rules! __BUG_LOCATION_STRING {
    ($file:expr, $line:expr) => {
        concat!(".long ", $file, "- .;", ".short ", $line, ";")
    };
}

#[cfg(not(CONFIG_DEBUG_BUGVERBOSE))]
#[macro_export]
macro_rules! __BUG_LOCATION_STRING {
    ($file:expr, $line:expr) => { "" };
}

#[macro_export]
macro_rules! __BUG_ENTRY_STRING {
    ($file:expr, $line:expr, $flags:expr) => {
        concat!(
            __BUG_ENTRY_START!(),
            __BUG_LOCATION_STRING!($file, $line),
            ".short ", $flags, ";",
            __BUG_ENTRY_END!()
        )
    };
}

#[macro_export]
macro_rules! ARCH_WARN_ASM {
    ($file:expr, $line:expr, $flags:expr, $size:expr) => {
        concat!(__BUG_ENTRY_STRING!($file, $line, $flags), "brk ", BUG_BRK_IMM)
    };
}

/* ARCH_WARN_REACHABLE */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
