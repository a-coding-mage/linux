/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Copyright (C) 2023 Google LLC.
 */

// The C header includes <linux/args.h>, which supplies COUNT_ARGS.
// Build-time CONFIG_* conditions are represented here by Cargo cfg features.

/*
 * Macros to count the number of LSMs enabled in the kernel at compile time.
 */

/*
 * Capabilities is enabled when CONFIG_SECURITY is enabled.
 */
#[cfg(feature = "CONFIG_SECURITY")]
pub const CAPABILITIES_ENABLED: usize = 1;
#[cfg(not(feature = "CONFIG_SECURITY"))]
pub const CAPABILITIES_ENABLED: usize = 0;

#[cfg(feature = "CONFIG_SECURITY_SELINUX")]
pub const SELINUX_ENABLED: usize = 1;
#[cfg(not(feature = "CONFIG_SECURITY_SELINUX"))]
pub const SELINUX_ENABLED: usize = 0;

#[cfg(feature = "CONFIG_SECURITY_SMACK")]
pub const SMACK_ENABLED: usize = 1;
#[cfg(not(feature = "CONFIG_SECURITY_SMACK"))]
pub const SMACK_ENABLED: usize = 0;

#[cfg(feature = "CONFIG_SECURITY_APPARMOR")]
pub const APPARMOR_ENABLED: usize = 1;
#[cfg(not(feature = "CONFIG_SECURITY_APPARMOR"))]
pub const APPARMOR_ENABLED: usize = 0;

#[cfg(feature = "CONFIG_SECURITY_TOMOYO")]
pub const TOMOYO_ENABLED: usize = 1;
#[cfg(not(feature = "CONFIG_SECURITY_TOMOYO"))]
pub const TOMOYO_ENABLED: usize = 0;

#[cfg(feature = "CONFIG_SECURITY_YAMA")]
pub const YAMA_ENABLED: usize = 1;
#[cfg(not(feature = "CONFIG_SECURITY_YAMA"))]
pub const YAMA_ENABLED: usize = 0;

#[cfg(feature = "CONFIG_SECURITY_LOADPIN")]
pub const LOADPIN_ENABLED: usize = 1;
#[cfg(not(feature = "CONFIG_SECURITY_LOADPIN"))]
pub const LOADPIN_ENABLED: usize = 0;

#[cfg(feature = "CONFIG_SECURITY_LOCKDOWN_LSM")]
pub const LOCKDOWN_ENABLED: usize = 1;
#[cfg(not(feature = "CONFIG_SECURITY_LOCKDOWN_LSM"))]
pub const LOCKDOWN_ENABLED: usize = 0;

#[cfg(feature = "CONFIG_SECURITY_SAFESETID")]
pub const SAFESETID_ENABLED: usize = 1;
#[cfg(not(feature = "CONFIG_SECURITY_SAFESETID"))]
pub const SAFESETID_ENABLED: usize = 0;

#[cfg(feature = "CONFIG_BPF_LSM")]
pub const BPF_LSM_ENABLED: usize = 1;
#[cfg(not(feature = "CONFIG_BPF_LSM"))]
pub const BPF_LSM_ENABLED: usize = 0;

#[cfg(feature = "CONFIG_SECURITY_LANDLOCK")]
pub const LANDLOCK_ENABLED: usize = 1;
#[cfg(not(feature = "CONFIG_SECURITY_LANDLOCK"))]
pub const LANDLOCK_ENABLED: usize = 0;

#[cfg(feature = "CONFIG_IMA")]
pub const IMA_ENABLED: usize = 1;
#[cfg(not(feature = "CONFIG_IMA"))]
pub const IMA_ENABLED: usize = 0;

#[cfg(feature = "CONFIG_EVM")]
pub const EVM_ENABLED: usize = 1;
#[cfg(not(feature = "CONFIG_EVM"))]
pub const EVM_ENABLED: usize = 0;

#[cfg(feature = "CONFIG_SECURITY_IPE")]
pub const IPE_ENABLED: usize = 1;
#[cfg(not(feature = "CONFIG_SECURITY_IPE"))]
pub const IPE_ENABLED: usize = 0;

#[cfg(feature = "CONFIG_SECURITY")]
pub const MAX_LSM_COUNT: usize = CAPABILITIES_ENABLED
    + SELINUX_ENABLED
    + SMACK_ENABLED
    + APPARMOR_ENABLED
    + TOMOYO_ENABLED
    + YAMA_ENABLED
    + LOADPIN_ENABLED
    + LOCKDOWN_ENABLED
    + SAFESETID_ENABLED
    + BPF_LSM_ENABLED
    + LANDLOCK_ENABLED
    + IMA_ENABLED
    + EVM_ENABLED
    + IPE_ENABLED;

#[cfg(not(feature = "CONFIG_SECURITY"))]
pub const MAX_LSM_COUNT: usize = 0;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
