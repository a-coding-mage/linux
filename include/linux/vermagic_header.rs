/* SPDX-License-Identifier: GPL-2.0 */

// This header can be included from kernel/module.c or *.mod.c only.
// The C preprocessor include dependencies are supplied by other files:
// <generated/utsrelease.h> and <asm/vermagic.h>.

/* Simply sanity version stamp for modules. */
#[cfg(feature = "CONFIG_SMP")]
pub const MODULE_VERMAGIC_SMP: &str = "SMP ";
#[cfg(not(feature = "CONFIG_SMP"))]
pub const MODULE_VERMAGIC_SMP: &str = "";

#[cfg(feature = "CONFIG_PREEMPT_RT")]
pub const MODULE_VERMAGIC_PREEMPT: &str = "preempt_rt ";
#[cfg(all(
    not(feature = "CONFIG_PREEMPT_RT"),
    feature = "CONFIG_PREEMPT_BUILD"
))]
pub const MODULE_VERMAGIC_PREEMPT: &str = "preempt ";
#[cfg(all(
    not(feature = "CONFIG_PREEMPT_RT"),
    not(feature = "CONFIG_PREEMPT_BUILD")
))]
pub const MODULE_VERMAGIC_PREEMPT: &str = "";

#[cfg(feature = "CONFIG_MODULE_UNLOAD")]
pub const MODULE_VERMAGIC_MODULE_UNLOAD: &str = "mod_unload ";
#[cfg(not(feature = "CONFIG_MODULE_UNLOAD"))]
pub const MODULE_VERMAGIC_MODULE_UNLOAD: &str = "";

#[cfg(feature = "CONFIG_MODVERSIONS")]
pub const MODULE_VERMAGIC_MODVERSIONS: &str = "modversions ";
#[cfg(not(feature = "CONFIG_MODVERSIONS"))]
pub const MODULE_VERMAGIC_MODVERSIONS: &str = "";

#[cfg(feature = "RANDSTRUCT")]
pub const MODULE_RANDSTRUCT: &str = concat!("RANDSTRUCT_", RANDSTRUCT_HASHED_SEED);
#[cfg(not(feature = "RANDSTRUCT"))]
pub const MODULE_RANDSTRUCT: &str = "";

// UTS_RELEASE and MODULE_ARCH_VERMAGIC are supplied by the generated
// dependency headers represented above.
pub const VERMAGIC_STRING: &str = concat!(
    UTS_RELEASE,
    " ",
    MODULE_VERMAGIC_SMP,
    MODULE_VERMAGIC_PREEMPT,
    MODULE_VERMAGIC_MODULE_UNLOAD,
    MODULE_VERMAGIC_MODVERSIONS,
    MODULE_ARCH_VERMAGIC,
    MODULE_RANDSTRUCT,
);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
