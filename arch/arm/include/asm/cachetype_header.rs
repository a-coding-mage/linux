/* SPDX-License-Identifier: GPL-2.0 */

pub const CACHEID_VIVT: u32 = 1 << 0;
pub const CACHEID_VIPT_NONALIASING: u32 = 1 << 1;
pub const CACHEID_VIPT_ALIASING: u32 = 1 << 2;
pub const CACHEID_VIPT: u32 = CACHEID_VIPT_ALIASING | CACHEID_VIPT_NONALIASING;
pub const CACHEID_ASID_TAGGED: u32 = 1 << 3;
pub const CACHEID_VIPT_I_ALIASING: u32 = 1 << 4;
pub const CACHEID_PIPT: u32 = 1 << 5;

unsafe extern "C" {
    pub static mut cacheid: u32;
}

#[inline]
pub fn cacheid_is(mask: u32) -> u32 {
    (__CACHEID_ALWAYS & mask)
        | (!__CACHEID_NEVER & __CACHEID_ARCH_MIN & mask & unsafe { cacheid })
}

#[inline]
pub fn cache_is_vivt() -> bool { cacheid_is(CACHEID_VIVT) != 0 }
#[inline]
pub fn cache_is_vipt() -> bool { cacheid_is(CACHEID_VIPT) != 0 }
#[inline]
pub fn cache_is_vipt_nonaliasing() -> bool { cacheid_is(CACHEID_VIPT_NONALIASING) != 0 }
#[inline]
pub fn cache_is_vipt_aliasing() -> bool { cacheid_is(CACHEID_VIPT_ALIASING) != 0 }
#[inline]
pub fn icache_is_vivt_asid_tagged() -> bool { cacheid_is(CACHEID_ASID_TAGGED) != 0 }
#[inline]
pub fn icache_is_vipt_aliasing() -> bool { cacheid_is(CACHEID_VIPT_I_ALIASING) != 0 }
#[inline]
pub fn icache_is_pipt() -> bool { cacheid_is(CACHEID_PIPT) != 0 }
#[inline]
pub fn cpu_dcache_is_aliasing() -> bool { cache_is_vivt() || cache_is_vipt_aliasing() }

/* __LINUX_ARM_ARCH__ and CONFIG_CPU_CACHE_* are build-time C configuration conditions. */
#[cfg(feature = "linux_arm_arch_ge_7")]
const __CACHEID_ARCH_MIN: u32 = CACHEID_VIPT_NONALIASING | CACHEID_ASID_TAGGED |
    CACHEID_VIPT_I_ALIASING | CACHEID_PIPT;
#[cfg(all(not(feature = "linux_arm_arch_ge_7"), feature = "linux_arm_arch_ge_6"))]
const __CACHEID_ARCH_MIN: u32 = !CACHEID_VIVT;
#[cfg(not(any(feature = "linux_arm_arch_ge_7", feature = "linux_arm_arch_ge_6")))]
const __CACHEID_ARCH_MIN: u32 = !0;

#[cfg(all(feature = "CONFIG_CPU_CACHE_VIVT", not(feature = "CONFIG_CPU_CACHE_VIPT")))]
const __CACHEID_ALWAYS: u32 = CACHEID_VIVT;
#[cfg(all(feature = "CONFIG_CPU_CACHE_VIVT", not(feature = "CONFIG_CPU_CACHE_VIPT")))]
const __CACHEID_NEVER: u32 = !CACHEID_VIVT;
#[cfg(all(not(feature = "CONFIG_CPU_CACHE_VIVT"), feature = "CONFIG_CPU_CACHE_VIPT"))]
const __CACHEID_ALWAYS: u32 = 0;
#[cfg(all(not(feature = "CONFIG_CPU_CACHE_VIVT"), feature = "CONFIG_CPU_CACHE_VIPT"))]
const __CACHEID_NEVER: u32 = CACHEID_VIVT;
#[cfg(not(any(
    all(feature = "CONFIG_CPU_CACHE_VIVT", not(feature = "CONFIG_CPU_CACHE_VIPT")),
    all(not(feature = "CONFIG_CPU_CACHE_VIVT"), feature = "CONFIG_CPU_CACHE_VIPT")
)))]
const __CACHEID_ALWAYS: u32 = 0;
#[cfg(not(any(
    all(feature = "CONFIG_CPU_CACHE_VIVT", not(feature = "CONFIG_CPU_CACHE_VIPT")),
    all(not(feature = "CONFIG_CPU_CACHE_VIVT"), feature = "CONFIG_CPU_CACHE_VIPT")
)))]
const __CACHEID_NEVER: u32 = 0;

pub const CSSELR_ICACHE: u32 = 1;
pub const CSSELR_DCACHE: u32 = 0;
pub const CSSELR_L1: u32 = 0 << 1;
pub const CSSELR_L2: u32 = 1 << 1;
pub const CSSELR_L3: u32 = 2 << 1;
pub const CSSELR_L4: u32 = 3 << 1;
pub const CSSELR_L5: u32 = 4 << 1;
pub const CSSELR_L6: u32 = 5 << 1;
pub const CSSELR_L7: u32 = 6 << 1;

/* CONFIG_CPU_V7M selects the alternate MMIO implementations in the C header. */
#[cfg(not(feature = "CONFIG_CPU_V7M"))]
#[inline]
pub unsafe fn set_csselr(cache_selector: u32) {
    core::arch::asm!("mcr p15, 2, {0}, c0, c0, 0", in(reg) cache_selector);
}

#[cfg(not(feature = "CONFIG_CPU_V7M"))]
#[inline]
pub unsafe fn read_ccsidr() -> u32 {
    let val: u32;
    core::arch::asm!("mrc p15, 1, {0}, c0, c0, 0", out(reg) val);
    val
}

#[cfg(not(feature = "CONFIG_CPU_V7M"))]
#[inline]
pub unsafe fn read_clidr() -> u32 {
    let val: u32;
    core::arch::asm!("mrc p15, 1, {0}, c0, c0, 1", out(reg) val);
    val
}

/* CONFIG_CPU_V7M branch depends on external linux::io and asm::v7m declarations. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
