/* SPDX-License-Identifier: GPL-2.0
 *
 * include/asm-sh/spinlock.h
 *
 * Copyright (C) 2002, 2003 Paul Mundt
 * Copyright (C) 2006, 2007 Akio Idehara
 */

// Configuration-dependent dependency:
// #if defined(CONFIG_CPU_SH4A)
// #include <asm/spinlock-llsc.h>
// #elif defined(CONFIG_CPU_J2)
// #include <asm/spinlock-cas.h>
// #else
// #error "The configured cpu type does not support spinlocks"
// #endif

#[cfg(CONFIG_CPU_SH4A)]
use crate::asm::spinlock_llsc::*;

#[cfg(all(not(CONFIG_CPU_SH4A), CONFIG_CPU_J2))]
use crate::asm::spinlock_cas::*;

#[cfg(all(not(CONFIG_CPU_SH4A), not(CONFIG_CPU_J2)))]
compile_error!("The configured cpu type does not support spinlocks");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
