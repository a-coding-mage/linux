/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2012 ARM Ltd.
 */

// Dependencies supplied by the corresponding UAPI and CPU-feature headers.

pub const COMPAT_HWCAP_SWP: u32 = 1 << 0;
pub const COMPAT_HWCAP_HALF: u32 = 1 << 1;
pub const COMPAT_HWCAP_THUMB: u32 = 1 << 2;
pub const COMPAT_HWCAP_26BIT: u32 = 1 << 3;
pub const COMPAT_HWCAP_FAST_MULT: u32 = 1 << 4;
pub const COMPAT_HWCAP_FPA: u32 = 1 << 5;
pub const COMPAT_HWCAP_VFP: u32 = 1 << 6;
pub const COMPAT_HWCAP_EDSP: u32 = 1 << 7;
pub const COMPAT_HWCAP_JAVA: u32 = 1 << 8;
pub const COMPAT_HWCAP_IWMMXT: u32 = 1 << 9;
pub const COMPAT_HWCAP_CRUNCH: u32 = 1 << 10; // Obsolete
pub const COMPAT_HWCAP_THUMBEE: u32 = 1 << 11;
pub const COMPAT_HWCAP_NEON: u32 = 1 << 12;
pub const COMPAT_HWCAP_VFPv3: u32 = 1 << 13;
pub const COMPAT_HWCAP_VFPV3D16: u32 = 1 << 14;
pub const COMPAT_HWCAP_TLS: u32 = 1 << 15;
pub const COMPAT_HWCAP_VFPv4: u32 = 1 << 16;
pub const COMPAT_HWCAP_IDIVA: u32 = 1 << 17;
pub const COMPAT_HWCAP_IDIVT: u32 = 1 << 18;
pub const COMPAT_HWCAP_IDIV: u32 = COMPAT_HWCAP_IDIVA | COMPAT_HWCAP_IDIVT;
pub const COMPAT_HWCAP_VFPD32: u32 = 1 << 19;
pub const COMPAT_HWCAP_LPAE: u32 = 1 << 20;
pub const COMPAT_HWCAP_EVTSTRM: u32 = 1 << 21;
pub const COMPAT_HWCAP_FPHP: u32 = 1 << 22;
pub const COMPAT_HWCAP_ASIMDHP: u32 = 1 << 23;
pub const COMPAT_HWCAP_ASIMDDP: u32 = 1 << 24;
pub const COMPAT_HWCAP_ASIMDFHM: u32 = 1 << 25;
pub const COMPAT_HWCAP_ASIMDBF16: u32 = 1 << 26;
pub const COMPAT_HWCAP_I8MM: u32 = 1 << 27;

pub const COMPAT_HWCAP2_AES: u32 = 1 << 0;
pub const COMPAT_HWCAP2_PMULL: u32 = 1 << 1;
pub const COMPAT_HWCAP2_SHA1: u32 = 1 << 2;
pub const COMPAT_HWCAP2_SHA2: u32 = 1 << 3;
pub const COMPAT_HWCAP2_CRC32: u32 = 1 << 4;
pub const COMPAT_HWCAP2_SB: u32 = 1 << 5;
pub const COMPAT_HWCAP2_SSBS: u32 = 1 << 6;

/*
 * For userspace we represent hwcaps as a collection of HWCAP{,2}_x bitfields
 * as described in uapi/asm/hwcap.h. For the kernel we represent hwcaps as
 * natural numbers (in a single range of size MAX_CPU_FEATURES) defined here
 * with prefix KERNEL_HWCAP_ mapped to their HWCAP{,2}_x counterpart.
 *
 * Hwcaps should be set and tested within the kernel via the
 * cpu_{set,have}_named_feature(feature) where feature is the unique suffix
 * of KERNEL_HWCAP_{feature}.
 */
// The __khwcap_feature, __khwcap2_feature, and __khwcap3_feature C token-
// pasting macros depend on constants and const_ilog2 supplied by other headers.

// Local declarations from asm/kernel-hwcap.h are supplied externally.

/*
 * This yields a mask that user programs can use to figure out what
 * instruction set this cpu supports.
 */
extern "C" {
    pub fn cpu_get_elf_hwcap() -> usize;
    pub fn cpu_get_elf_hwcap2() -> usize;
    pub fn cpu_get_elf_hwcap3() -> usize;
}

#[macro_export]
macro_rules! ELF_HWCAP { () => { $crate::cpu_get_elf_hwcap() }; }
#[macro_export]
macro_rules! ELF_HWCAP2 { () => { $crate::cpu_get_elf_hwcap2() }; }
#[macro_export]
macro_rules! ELF_HWCAP3 { () => { $crate::cpu_get_elf_hwcap3() }; }

// CONFIG_COMPAT is a build-time configuration condition from the kernel.
#[cfg(feature = "CONFIG_COMPAT")]
macro_rules! COMPAT_ELF_HWCAP { () => { $crate::compat_elf_hwcap }; }
#[cfg(feature = "CONFIG_COMPAT")]
macro_rules! COMPAT_ELF_HWCAP2 { () => { $crate::compat_elf_hwcap2 }; }
#[cfg(feature = "CONFIG_COMPAT")]
macro_rules! COMPAT_ELF_HWCAP3 { () => { $crate::compat_elf_hwcap3 }; }

#[cfg(feature = "CONFIG_COMPAT")]
extern "C" {
    pub static mut compat_elf_hwcap: u32;
    pub static mut compat_elf_hwcap2: u32;
    pub static mut compat_elf_hwcap3: u32;
}

#[repr(u32)]
pub enum Cap {
    CAP_HWCAP = 1,
    #[cfg(feature = "CONFIG_COMPAT")]
    CAP_COMPAT_HWCAP,
    #[cfg(feature = "CONFIG_COMPAT")]
    CAP_COMPAT_HWCAP2,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
