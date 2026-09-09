/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2020-2022 Loongson Technology Corporation Limited
 */

pub const Pref_Load: i32 = 0;
pub const Pref_Store: i32 = 8;

// The following macros are assembler-only in the original header.  Their
// intent is preserved here; CONFIG_CPU_HAS_PREFETCH controls whether the
// prefetch instruction is emitted by the assembler implementation.
//
// .macro __pref hint addr
// #ifdef CONFIG_CPU_HAS_PREFETCH
// preld \hint, \addr, 0
// #endif
// .endm
//
// .macro pref_load addr
// __pref Pref_Load, \addr
// .endm
//
// .macro pref_store addr
// __pref Pref_Store, \addr
// .endm

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
