/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * IXP4XX cpu type detection
 *
 * Copyright (C) 2007 MontaVista Software, Inc.
 */

// C dependencies: linux/io.h, linux/regmap.h, and asm/cputype.h when CONFIG_ARM.

/* Processor id value in CP15 Register 0 */
pub const IXP42X_PROCESSOR_ID_VALUE: u32 = 0x6905_41c0; /* including unused 0x690541Ex */
pub const IXP42X_PROCESSOR_ID_MASK: u32 = 0xffff_ffc0;

pub const IXP43X_PROCESSOR_ID_VALUE: u32 = 0x6905_4040;
pub const IXP43X_PROCESSOR_ID_MASK: u32 = 0xffff_fff0;

pub const IXP46X_PROCESSOR_ID_VALUE: u32 = 0x6905_4200; /* including IXP455 */
pub const IXP46X_PROCESSOR_ID_MASK: u32 = 0xffff_fff0;

/* Feature register in the expansion bus controller */
pub const IXP4XX_EXP_CNFG2: u32 = 0x2c;

/* "fuse" bits of IXP_EXP_CFG2 */
/* All IXP4xx CPUs */
pub const IXP4XX_FEATURE_RCOMP: u32 = 1 << 0;
pub const IXP4XX_FEATURE_USB_DEVICE: u32 = 1 << 1;
pub const IXP4XX_FEATURE_HASH: u32 = 1 << 2;
pub const IXP4XX_FEATURE_AES: u32 = 1 << 3;
pub const IXP4XX_FEATURE_DES: u32 = 1 << 4;
pub const IXP4XX_FEATURE_HDLC: u32 = 1 << 5;
pub const IXP4XX_FEATURE_AAL: u32 = 1 << 6;
pub const IXP4XX_FEATURE_HSS: u32 = 1 << 7;
pub const IXP4XX_FEATURE_UTOPIA: u32 = 1 << 8;
pub const IXP4XX_FEATURE_NPEB_ETH0: u32 = 1 << 9;
pub const IXP4XX_FEATURE_NPEC_ETH: u32 = 1 << 10;
pub const IXP4XX_FEATURE_RESET_NPEA: u32 = 1 << 11;
pub const IXP4XX_FEATURE_RESET_NPEB: u32 = 1 << 12;
pub const IXP4XX_FEATURE_RESET_NPEC: u32 = 1 << 13;
pub const IXP4XX_FEATURE_PCI: u32 = 1 << 14;
pub const IXP4XX_FEATURE_UTOPIA_PHY_LIMIT: u32 = 3 << 16;
pub const IXP4XX_FEATURE_XSCALE_MAX_FREQ: u32 = 3 << 22;
pub const IXP42X_FEATURE_MASK: u32 = IXP4XX_FEATURE_RCOMP
    | IXP4XX_FEATURE_USB_DEVICE | IXP4XX_FEATURE_HASH | IXP4XX_FEATURE_AES
    | IXP4XX_FEATURE_DES | IXP4XX_FEATURE_HDLC | IXP4XX_FEATURE_AAL
    | IXP4XX_FEATURE_HSS | IXP4XX_FEATURE_UTOPIA | IXP4XX_FEATURE_NPEB_ETH0
    | IXP4XX_FEATURE_NPEC_ETH | IXP4XX_FEATURE_RESET_NPEA
    | IXP4XX_FEATURE_RESET_NPEB | IXP4XX_FEATURE_RESET_NPEC
    | IXP4XX_FEATURE_PCI | IXP4XX_FEATURE_UTOPIA_PHY_LIMIT
    | IXP4XX_FEATURE_XSCALE_MAX_FREQ;

/* IXP43x/46x CPUs */
pub const IXP4XX_FEATURE_ECC_TIMESYNC: u32 = 1 << 15;
pub const IXP4XX_FEATURE_USB_HOST: u32 = 1 << 18;
pub const IXP4XX_FEATURE_NPEA_ETH: u32 = 1 << 19;
pub const IXP43X_FEATURE_MASK: u32 = IXP42X_FEATURE_MASK
    | IXP4XX_FEATURE_ECC_TIMESYNC | IXP4XX_FEATURE_USB_HOST | IXP4XX_FEATURE_NPEA_ETH;

/* IXP46x CPU (including IXP455) only */
pub const IXP4XX_FEATURE_NPEB_ETH_1_TO_3: u32 = 1 << 20;
pub const IXP4XX_FEATURE_RSA: u32 = 1 << 21;
pub const IXP46X_FEATURE_MASK: u32 = IXP43X_FEATURE_MASK
    | IXP4XX_FEATURE_NPEB_ETH_1_TO_3 | IXP4XX_FEATURE_RSA;

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

#[cfg(feature = "CONFIG_ARCH_IXP4XX")]
extern "C" {
    pub fn read_cpuid_id() -> u32;
    pub fn regmap_read(rmap: *mut regmap, reg: u32, val: *mut u32) -> i32;
}

#[cfg(feature = "CONFIG_ARCH_IXP4XX")]
#[inline]
pub unsafe fn cpu_is_ixp42x_rev_a0() -> bool {
    (read_cpuid_id() & (IXP42X_PROCESSOR_ID_MASK | 0xF)) == IXP42X_PROCESSOR_ID_VALUE
}

#[cfg(feature = "CONFIG_ARCH_IXP4XX")]
#[inline]
pub unsafe fn cpu_is_ixp42x() -> bool {
    (read_cpuid_id() & IXP42X_PROCESSOR_ID_MASK) == IXP42X_PROCESSOR_ID_VALUE
}

#[cfg(feature = "CONFIG_ARCH_IXP4XX")]
#[inline]
pub unsafe fn cpu_is_ixp43x() -> bool {
    (read_cpuid_id() & IXP43X_PROCESSOR_ID_MASK) == IXP43X_PROCESSOR_ID_VALUE
}

#[cfg(feature = "CONFIG_ARCH_IXP4XX")]
#[inline]
pub unsafe fn cpu_is_ixp46x() -> bool {
    (read_cpuid_id() & IXP46X_PROCESSOR_ID_MASK) == IXP46X_PROCESSOR_ID_VALUE
}

#[cfg(feature = "CONFIG_ARCH_IXP4XX")]
#[inline]
pub unsafe fn cpu_ixp4xx_features(rmap: *mut regmap) -> u32 {
    let mut val: u32 = 0;
    regmap_read(rmap, IXP4XX_EXP_CNFG2, &mut val);
    /* For some reason this register is inverted */
    val = !val;
    if cpu_is_ixp42x_rev_a0() {
        return IXP42X_FEATURE_MASK & !(IXP4XX_FEATURE_RCOMP | IXP4XX_FEATURE_AES);
    }
    if cpu_is_ixp42x() {
        return val & IXP42X_FEATURE_MASK;
    }
    if cpu_is_ixp43x() {
        return val & IXP43X_FEATURE_MASK;
    }
    val & IXP46X_FEATURE_MASK
}

#[cfg(not(feature = "CONFIG_ARCH_IXP4XX"))]
#[inline]
pub const fn cpu_is_ixp42x_rev_a0() -> u32 { 0 }

#[cfg(not(feature = "CONFIG_ARCH_IXP4XX"))]
#[inline]
pub const fn cpu_is_ixp42x() -> u32 { 0 }

#[cfg(not(feature = "CONFIG_ARCH_IXP4XX"))]
#[inline]
pub const fn cpu_is_ixp43x() -> u32 { 0 }

#[cfg(not(feature = "CONFIG_ARCH_IXP4XX"))]
#[inline]
pub const fn cpu_is_ixp46x() -> u32 { 0 }

#[cfg(not(feature = "CONFIG_ARCH_IXP4XX"))]
#[inline]
pub unsafe fn cpu_ixp4xx_features(_rmap: *mut regmap) -> u32 { 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
