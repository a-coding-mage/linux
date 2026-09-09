/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *  arch/arm/include/asm/domain.h
 *
 *  Copyright (C) 1999 Russell King.
 */

/* Domain numbers.  The CONFIG_IO_36 condition is preserved from the C header. */
#[cfg(not(CONFIG_IO_36))]
pub const DOMAIN_KERNEL: u32 = 0;
#[cfg(not(CONFIG_IO_36))]
pub const DOMAIN_USER: u32 = 1;
#[cfg(not(CONFIG_IO_36))]
pub const DOMAIN_IO: u32 = 2;

#[cfg(CONFIG_IO_36)]
pub const DOMAIN_KERNEL: u32 = 2;
#[cfg(CONFIG_IO_36)]
pub const DOMAIN_USER: u32 = 1;
#[cfg(CONFIG_IO_36)]
pub const DOMAIN_IO: u32 = 0;

pub const DOMAIN_VECTORS: u32 = 3;

/* Domain types. */
pub const DOMAIN_NOACCESS: u32 = 0;
pub const DOMAIN_CLIENT: u32 = 1;
#[cfg(CONFIG_CPU_USE_DOMAINS)]
pub const DOMAIN_MANAGER: u32 = 3;
#[cfg(not(CONFIG_CPU_USE_DOMAINS))]
pub const DOMAIN_MANAGER: u32 = 1;

#[macro_export]
macro_rules! domain_mask {
    ($dom:expr) => { 3u32 << (2 * ($dom)) };
}

#[macro_export]
macro_rules! domain_val {
    ($dom:expr, $ty:expr) => { ($ty) << (2 * ($dom)) };
}

#[cfg(CONFIG_CPU_SW_DOMAIN_PAN)]
pub const DACR_INIT: u32 =
    domain_val!(DOMAIN_USER, DOMAIN_NOACCESS) |
    domain_val!(DOMAIN_KERNEL, DOMAIN_MANAGER) |
    domain_val!(DOMAIN_IO, DOMAIN_CLIENT) |
    domain_val!(DOMAIN_VECTORS, DOMAIN_CLIENT);

#[cfg(not(CONFIG_CPU_SW_DOMAIN_PAN))]
pub const DACR_INIT: u32 =
    domain_val!(DOMAIN_USER, DOMAIN_CLIENT) |
    domain_val!(DOMAIN_KERNEL, DOMAIN_MANAGER) |
    domain_val!(DOMAIN_IO, DOMAIN_CLIENT) |
    domain_val!(DOMAIN_VECTORS, DOMAIN_CLIENT);

pub const __DACR_DEFAULT: u32 =
    domain_val!(DOMAIN_KERNEL, DOMAIN_CLIENT) |
    domain_val!(DOMAIN_IO, DOMAIN_CLIENT) |
    domain_val!(DOMAIN_VECTORS, DOMAIN_CLIENT);

pub const DACR_UACCESS_DISABLE: u32 =
    __DACR_DEFAULT | domain_val!(DOMAIN_USER, DOMAIN_NOACCESS);
pub const DACR_UACCESS_ENABLE: u32 =
    __DACR_DEFAULT | domain_val!(DOMAIN_USER, DOMAIN_CLIENT);

#[cfg(CONFIG_CPU_CP15_MMU)]
#[inline(always)]
pub unsafe fn get_domain() -> u32 {
    let domain: u32;
    core::arch::asm!(
        "mrc p15, 0, {domain}, c3, c0",
        domain = out(reg) domain,
        options(nostack)
    );
    domain
}

#[cfg(CONFIG_CPU_CP15_MMU)]
#[inline(always)]
pub unsafe fn set_domain(val: u32) {
    core::arch::asm!(
        "mcr p15, 0, {val}, c3, c0",
        val = in(reg) val,
        options(nostack)
    );
    core::arch::asm!("isb", options(nostack));
}

#[cfg(not(CONFIG_CPU_CP15_MMU))]
#[inline(always)]
pub unsafe fn get_domain() -> u32 {
    0
}

#[cfg(not(CONFIG_CPU_CP15_MMU))]
#[inline(always)]
pub unsafe fn set_domain(_val: u32) {}

/* Generate the T (user) versions of LDR/STR and related instructions. */
#[cfg(CONFIG_CPU_USE_DOMAINS)]
#[macro_export]
macro_rules! TUSER {
    ($instr:ident) => { concat!(stringify!($instr), "t") };
}
#[cfg(not(CONFIG_CPU_USE_DOMAINS))]
#[macro_export]
macro_rules! TUSER {
    ($instr:ident) => { stringify!($instr) };
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
