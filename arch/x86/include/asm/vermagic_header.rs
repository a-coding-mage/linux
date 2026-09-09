// SPDX-License-Identifier: GPL-2.0

// Translated from the C header guard _ASM_VERMAGIC_H.

// X86_64 does not define MODULE_PROC_FAMILY.
#[cfg(CONFIG_X86_64)]
pub const MODULE_PROC_FAMILY: &str = "";

#[cfg(all(not(CONFIG_X86_64), CONFIG_M586TSC))]
pub const MODULE_PROC_FAMILY: &str = "586TSC ";
#[cfg(all(not(CONFIG_X86_64), CONFIG_M586MMX))]
pub const MODULE_PROC_FAMILY: &str = "586MMX ";
#[cfg(all(not(CONFIG_X86_64), CONFIG_MATOM))]
pub const MODULE_PROC_FAMILY: &str = "ATOM ";
#[cfg(all(not(CONFIG_X86_64), CONFIG_M686))]
pub const MODULE_PROC_FAMILY: &str = "686 ";
#[cfg(all(not(CONFIG_X86_64), CONFIG_MPENTIUMII))]
pub const MODULE_PROC_FAMILY: &str = "PENTIUMII ";
#[cfg(all(not(CONFIG_X86_64), CONFIG_MPENTIUMIII))]
pub const MODULE_PROC_FAMILY: &str = "PENTIUMIII ";
#[cfg(all(not(CONFIG_X86_64), CONFIG_MPENTIUMM))]
pub const MODULE_PROC_FAMILY: &str = "PENTIUMM ";
#[cfg(all(not(CONFIG_X86_64), CONFIG_MPENTIUM4))]
pub const MODULE_PROC_FAMILY: &str = "PENTIUM4 ";
#[cfg(all(not(CONFIG_X86_64), CONFIG_MK6))]
pub const MODULE_PROC_FAMILY: &str = "K6 ";
#[cfg(all(not(CONFIG_X86_64), CONFIG_MK7))]
pub const MODULE_PROC_FAMILY: &str = "K7 ";
#[cfg(all(not(CONFIG_X86_64), CONFIG_MCRUSOE))]
pub const MODULE_PROC_FAMILY: &str = "CRUSOE ";
#[cfg(all(not(CONFIG_X86_64), CONFIG_MEFFICEON))]
pub const MODULE_PROC_FAMILY: &str = "EFFICEON ";
#[cfg(all(not(CONFIG_X86_64), CONFIG_MCYRIXIII))]
pub const MODULE_PROC_FAMILY: &str = "CYRIXIII ";
#[cfg(all(not(CONFIG_X86_64), CONFIG_MVIAC3_2))]
pub const MODULE_PROC_FAMILY: &str = "VIAC3-2 ";
#[cfg(all(not(CONFIG_X86_64), CONFIG_MVIAC7))]
pub const MODULE_PROC_FAMILY: &str = "VIAC7 ";
#[cfg(all(not(CONFIG_X86_64), CONFIG_MGEODEGX1))]
pub const MODULE_PROC_FAMILY: &str = "GEODEGX1 ";
#[cfg(all(not(CONFIG_X86_64), CONFIG_MGEODE_LX))]
pub const MODULE_PROC_FAMILY: &str = "GEODE ";

// C source: #error unknown processor family
#[cfg(all(
    not(CONFIG_X86_64),
    not(CONFIG_M586TSC),
    not(CONFIG_M586MMX),
    not(CONFIG_MATOM),
    not(CONFIG_M686),
    not(CONFIG_MPENTIUMII),
    not(CONFIG_MPENTIUMIII),
    not(CONFIG_MPENTIUMM),
    not(CONFIG_MPENTIUM4),
    not(CONFIG_MK6),
    not(CONFIG_MK7),
    not(CONFIG_MCRUSOE),
    not(CONFIG_MEFFICEON),
    not(CONFIG_MCYRIXIII),
    not(CONFIG_MVIAC3_2),
    not(CONFIG_MVIAC7),
    not(CONFIG_MGEODEGX1),
    not(CONFIG_MGEODE_LX),
))]
compile_error!("unknown processor family");

#[cfg(CONFIG_X86_32)]
pub const MODULE_ARCH_VERMAGIC: &str = MODULE_PROC_FAMILY;

#[cfg(not(CONFIG_X86_32))]
pub const MODULE_ARCH_VERMAGIC: &str = "";

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
