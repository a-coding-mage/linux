/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Some useful macros for LoongArch assembler code
 *
 * Copyright (C) 2020-2022 Loongson Technology Corporation Limited
 *
 * Derived from MIPS:
 * Copyright (C) 1995, 1996, 1997, 1999, 2001 by Ralf Baechle
 * Copyright (C) 1999 by Silicon Graphics, Inc.
 * Copyright (C) 2001 MIPS Technologies, Inc.
 * Copyright (C) 2002  Maciej W. Rozycki
 */

/* Build-time assembler configuration from the C header is preserved here. */

/* LoongArch pref instruction. */
#[cfg(feature = "CONFIG_CPU_HAS_PREFETCH")]
#[macro_export]
macro_rules! PREF { ($hint:tt, $addr:tt, $offs:tt) => { concat!("preld ", stringify!($hint), ", ", stringify!($addr), ", ", stringify!($offs), ";") }; }

#[cfg(feature = "CONFIG_CPU_HAS_PREFETCH")]
#[macro_export]
macro_rules! PREFX { ($hint:tt, $addr:tt, $index:tt) => { concat!("preldx ", stringify!($hint), ", ", stringify!($addr), ", ", stringify!($index), ";") }; }

#[cfg(not(feature = "CONFIG_CPU_HAS_PREFETCH"))]
#[macro_export]
macro_rules! PREF { ($hint:tt, $addr:tt, $offs:tt) => {}; }

#[cfg(not(feature = "CONFIG_CPU_HAS_PREFETCH"))]
#[macro_export]
macro_rules! PREFX { ($hint:tt, $addr:tt, $index:tt) => {}; }

/* Stack alignment */
pub const STACK_ALIGN: usize = !0xf;

/* Register size-dependent assembler spellings. */
#[cfg(not(feature = "__loongarch64"))]
pub const SZREG: usize = 4;
#[cfg(feature = "__loongarch64")]
pub const SZREG: usize = 8;

/* The following constants preserve the assembler instruction macros. */
#[cfg(not(feature = "__loongarch64"))]
pub mod reg {
    pub const L: &str = "ld.w"; pub const S: &str = "st.w";
    pub const ADD: &str = "add.w"; pub const SUB: &str = "sub.w";
}

#[cfg(target_pointer_width = "32")]
pub mod int {
    pub const ADD: &str = "add.w"; pub const ADDI: &str = "addi.w"; pub const SUB: &str = "sub.w";
    pub const L: &str = "ld.w"; pub const S: &str = "st.w"; pub const SLLI: &str = "slli.w";
    pub const SLLV: &str = "sll.w"; pub const SRLI: &str = "srli.w"; pub const SRLV: &str = "srl.w";
    pub const SRAI: &str = "srai.w"; pub const SRAV: &str = "sra.w";
}
#[cfg(target_pointer_width = "64")]
pub mod int {
    pub const ADD: &str = "add.d"; pub const ADDI: &str = "addi.d"; pub const SUB: &str = "sub.d";
    pub const L: &str = "ld.d"; pub const S: &str = "st.d"; pub const SLLI: &str = "slli.d";
    pub const SLLV: &str = "sll.d"; pub const SRLI: &str = "srli.d"; pub const SRLV: &str = "srl.d";
    pub const SRAI: &str = "srai.d"; pub const SRAV: &str = "sra.d";
}
#[cfg(feature = "__loongarch64")]
pub mod reg {
    pub const L: &str = "ld.d"; pub const S: &str = "st.d";
    pub const ADD: &str = "add.d"; pub const SUB: &str = "sub.d";
}

/* C int, long, and pointer instruction spellings are selected by the
 * corresponding C build-time size conditions. */
#[cfg(target_pointer_width = "32")]
pub mod long {
    pub const ADD: &str = "add.w"; pub const ADDI: &str = "addi.w";
    pub const ALSL: &str = "alsl.w"; pub const BSTRINS: &str = "bstrins.w";
    pub const BSTRPICK: &str = "bstrpick.w"; pub const SUB: &str = "sub.w";
    pub const L: &str = "ld.w"; pub const LI: &str = "li.w"; pub const LPTR: &str = "ld.w";
    pub const S: &str = "st.w"; pub const SPTR: &str = "st.w";
    pub const SLLI: &str = "slli.w"; pub const SLLV: &str = "sll.w";
    pub const SRLI: &str = "srli.w"; pub const SRLV: &str = "srl.w";
    pub const SRAI: &str = "srai.w"; pub const SRAV: &str = "sra.w";
    pub const ROTR: &str = "rotr.w"; pub const ROTRI: &str = "rotri.w";
    pub const LONGSIZE: usize = 4; pub const LONGMASK: usize = 3; pub const LONGLOG: usize = 2;
}
#[cfg(target_pointer_width = "64")]
pub mod long {
    pub const ADD: &str = "add.d"; pub const ADDI: &str = "addi.d";
    pub const ALSL: &str = "alsl.d"; pub const BSTRINS: &str = "bstrins.d";
    pub const BSTRPICK: &str = "bstrpick.d"; pub const SUB: &str = "sub.d";
    pub const L: &str = "ld.d"; pub const LI: &str = "li.d"; pub const LPTR: &str = "ldptr.d";
    pub const S: &str = "st.d"; pub const SPTR: &str = "stptr.d";
    pub const SLLI: &str = "slli.d"; pub const SLLV: &str = "sll.d";
    pub const SRLI: &str = "srli.d"; pub const SRLV: &str = "srl.d";
    pub const SRAI: &str = "srai.d"; pub const SRAV: &str = "sra.d";
    pub const ROTR: &str = "rotr.d"; pub const ROTRI: &str = "rotri.d";
    pub const LONGSIZE: usize = 8; pub const LONGMASK: usize = 7; pub const LONGLOG: usize = 3;
}

/* Pointer instruction spellings. */
#[cfg(target_pointer_width = "32")]
pub mod ptr {
    pub const ADD: &str = "add.w"; pub const ADDI: &str = "addi.w"; pub const ALSL: &str = "alsl.w";
    pub const BSTRINS: &str = "bstrins.w"; pub const BSTRPICK: &str = "bstrpick.w"; pub const SUB: &str = "sub.w";
    pub const L: &str = "ld.w"; pub const LI: &str = "li.w"; pub const LPTR: &str = "ld.w";
    pub const S: &str = "st.w"; pub const SPTR: &str = "st.w"; pub const SLLI: &str = "slli.w";
    pub const SLLV: &str = "sll.w"; pub const SRLI: &str = "srli.w"; pub const SRLV: &str = "srl.w";
    pub const SRAI: &str = "srai.w"; pub const SRAV: &str = "sra.w"; pub const ROTR: &str = "rotr.w";
    pub const ROTRI: &str = "rotri.w"; pub const SCALESHIFT: usize = 2; pub const PTRSIZE: usize = 4; pub const PTRLOG: usize = 2;
}
#[cfg(target_pointer_width = "64")]
pub mod ptr {
    pub const ADD: &str = "add.d"; pub const ADDI: &str = "addi.d"; pub const ALSL: &str = "alsl.d";
    pub const BSTRINS: &str = "bstrins.d"; pub const BSTRPICK: &str = "bstrpick.d"; pub const SUB: &str = "sub.d";
    pub const L: &str = "ld.d"; pub const LI: &str = "li.d"; pub const LPTR: &str = "ldptr.d";
    pub const S: &str = "st.d"; pub const SPTR: &str = "stptr.d"; pub const SLLI: &str = "slli.d";
    pub const SLLV: &str = "sll.d"; pub const SRLI: &str = "srli.d"; pub const SRLV: &str = "srl.d";
    pub const SRAI: &str = "srai.d"; pub const SRAV: &str = "sra.d"; pub const ROTR: &str = "rotr.d";
    pub const ROTRI: &str = "rotri.d"; pub const SCALESHIFT: usize = 3; pub const PTRSIZE: usize = 8; pub const PTRLOG: usize = 3;
}

/* Annotate a function as being unsuitable for kprobes. */
#[cfg(feature = "CONFIG_KPROBES")]
#[macro_export]
macro_rules! _ASM_NOKPROBE { ($name:tt) => { stringify!($name) }; }
#[cfg(not(feature = "CONFIG_KPROBES"))]
#[macro_export]
macro_rules! _ASM_NOKPROBE { ($name:tt) => {}; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
