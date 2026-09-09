/* SPDX-License-Identifier: GPL-2.0 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum implver_enum {
    IMPLVER_EV4,
    IMPLVER_EV5,
    IMPLVER_EV6,
}

#[cfg(feature = "CONFIG_ALPHA_GENERIC")]
#[inline]
pub unsafe fn implver() -> implver_enum {
    let mut __implver: u64;
    core::arch::asm!("implver {0}", out(reg) __implver);
    __implver as implver_enum
}

/* Try to eliminate some dead code.  */
#[cfg(all(not(feature = "CONFIG_ALPHA_GENERIC"), feature = "CONFIG_ALPHA_EV56"))]
#[inline]
pub const fn implver() -> implver_enum {
    implver_enum::IMPLVER_EV5
}

#[cfg(all(
    not(feature = "CONFIG_ALPHA_GENERIC"),
    not(feature = "CONFIG_ALPHA_EV56"),
    feature = "CONFIG_ALPHA_EV6"
))]
#[inline]
pub const fn implver() -> implver_enum {
    implver_enum::IMPLVER_EV6
}

#[repr(u64)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum amask_enum {
    AMASK_BWX = 1u64 << 0,
    AMASK_FIX = 1u64 << 1,
    AMASK_CIX = 1u64 << 2,
    AMASK_MAX = 1u64 << 8,
    AMASK_PRECISE_TRAP = 1u64 << 9,
}

#[inline]
pub unsafe fn amask(mask: u64) -> u64 {
    let mut __amask: u64;
    let __input = mask;
    core::arch::asm!("amask {1},{0}", out(reg) __amask, in(reg) __input);
    __amask
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
