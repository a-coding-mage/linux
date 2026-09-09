/* SPDX-License-Identifier: GPL-2.0 */
/* Octeon CIU definitions
 *
 * Copyright (C) 2003-2018 Cavium, Inc.
 */

// Dependency: asm/bitfield.h

#[inline]
pub const fn CVMX_CIU_ADDR(addr: u64, coreid: u64, coremask: u64, offset: u64) -> u64 {
    CVMX_ADD_IO_SEG(0x0001_0700_0000_0000u64 + addr) + ((coreid & coremask) * offset)
}

#[inline] pub const fn CVMX_CIU_EN2_PPX_IP4(c: u64) -> u64 { CVMX_CIU_ADDR(0xA400, c, 0x0F, 8) }
#[inline] pub const fn CVMX_CIU_EN2_PPX_IP4_W1C(c: u64) -> u64 { CVMX_CIU_ADDR(0xCC00, c, 0x0F, 8) }
#[inline] pub const fn CVMX_CIU_EN2_PPX_IP4_W1S(c: u64) -> u64 { CVMX_CIU_ADDR(0xAC00, c, 0x0F, 8) }
pub const CVMX_CIU_FUSE: u64 = CVMX_CIU_ADDR(0x0728, 0, 0x00, 0);
pub const CVMX_CIU_INT_SUM1: u64 = CVMX_CIU_ADDR(0x0108, 0, 0x00, 0);
#[inline] pub const fn CVMX_CIU_INTX_EN0(c: u64) -> u64 { CVMX_CIU_ADDR(0x0200, c, 0x3F, 16) }
#[inline] pub const fn CVMX_CIU_INTX_EN0_W1C(c: u64) -> u64 { CVMX_CIU_ADDR(0x2200, c, 0x3F, 16) }
#[inline] pub const fn CVMX_CIU_INTX_EN0_W1S(c: u64) -> u64 { CVMX_CIU_ADDR(0x6200, c, 0x3F, 16) }
#[inline] pub const fn CVMX_CIU_INTX_EN1(c: u64) -> u64 { CVMX_CIU_ADDR(0x0208, c, 0x3F, 16) }
#[inline] pub const fn CVMX_CIU_INTX_EN1_W1C(c: u64) -> u64 { CVMX_CIU_ADDR(0x2208, c, 0x3F, 16) }
#[inline] pub const fn CVMX_CIU_INTX_EN1_W1S(c: u64) -> u64 { CVMX_CIU_ADDR(0x6208, c, 0x3F, 16) }
#[inline] pub const fn CVMX_CIU_INTX_SUM0(c: u64) -> u64 { CVMX_CIU_ADDR(0x0000, c, 0x3F, 8) }
pub const CVMX_CIU_NMI: u64 = CVMX_CIU_ADDR(0x0718, 0, 0x00, 0);
pub const CVMX_CIU_PCI_INTA: u64 = CVMX_CIU_ADDR(0x0750, 0, 0x00, 0);
pub const CVMX_CIU_PP_BIST_STAT: u64 = CVMX_CIU_ADDR(0x07E0, 0, 0x00, 0);
pub const CVMX_CIU_PP_DBG: u64 = CVMX_CIU_ADDR(0x0708, 0, 0x00, 0);
pub const CVMX_CIU_PP_RST: u64 = CVMX_CIU_ADDR(0x0700, 0, 0x00, 0);
pub const CVMX_CIU_QLM0: u64 = CVMX_CIU_ADDR(0x0780, 0, 0x00, 0);
pub const CVMX_CIU_QLM1: u64 = CVMX_CIU_ADDR(0x0788, 0, 0x00, 0);
pub const CVMX_CIU_QLM_JTGC: u64 = CVMX_CIU_ADDR(0x0768, 0, 0x00, 0);
pub const CVMX_CIU_QLM_JTGD: u64 = CVMX_CIU_ADDR(0x0770, 0, 0x00, 0);
pub const CVMX_CIU_SOFT_BIST: u64 = CVMX_CIU_ADDR(0x0738, 0, 0x00, 0);
pub const CVMX_CIU_SOFT_PRST1: u64 = CVMX_CIU_ADDR(0x0758, 0, 0x00, 0);
pub const CVMX_CIU_SOFT_PRST: u64 = CVMX_CIU_ADDR(0x0748, 0, 0x00, 0);
pub const CVMX_CIU_SOFT_RST: u64 = CVMX_CIU_ADDR(0x0740, 0, 0x00, 0);
#[inline] pub const fn CVMX_CIU_SUM2_PPX_IP4(c: u64) -> u64 { CVMX_CIU_ADDR(0x8C00, c, 0x0F, 8) }
pub const CVMX_CIU_TIM_MULTI_CAST: u64 = CVMX_CIU_ADDR(0xC200, 0, 0x00, 0);
#[inline] pub const fn CVMX_CIU_TIMX(c: u64) -> u64 { CVMX_CIU_ADDR(0x0480, c, 0x0F, 8) }

#[inline] pub unsafe fn CVMX_CIU_MBOX_CLRX(coreid: u32) -> u64 {
    if cvmx_get_octeon_family() == (OCTEON_CN68XX & OCTEON_FAMILY_MASK) { CVMX_CIU_ADDR(0x100100600, coreid as u64, 0x0F, 8) } else { CVMX_CIU_ADDR(0x000000680, coreid as u64, 0x0F, 8) }
}
#[inline] pub unsafe fn CVMX_CIU_MBOX_SETX(coreid: u32) -> u64 {
    if cvmx_get_octeon_family() == (OCTEON_CN68XX & OCTEON_FAMILY_MASK) { CVMX_CIU_ADDR(0x100100400, coreid as u64, 0x0F, 8) } else { CVMX_CIU_ADDR(0x000000600, coreid as u64, 0x0F, 8) }
}
#[inline] pub unsafe fn CVMX_CIU_PP_POKEX(coreid: u32) -> u64 {
    match cvmx_get_octeon_family() {
        x if x == (OCTEON_CN68XX & OCTEON_FAMILY_MASK) => CVMX_CIU_ADDR(0x100100200, coreid as u64, 0x0F, 8),
        x if x == (OCTEON_CNF75XX & OCTEON_FAMILY_MASK) || x == (OCTEON_CN73XX & OCTEON_FAMILY_MASK) || x == (OCTEON_CN78XX & OCTEON_FAMILY_MASK) => CVMX_CIU_ADDR(0x000030000, coreid as u64, 0x0F, 8).wrapping_sub(0x60000000000),
        _ => CVMX_CIU_ADDR(0x000000580, coreid as u64, 0x0F, 8),
    }
}
#[inline] pub unsafe fn CVMX_CIU_WDOGX(coreid: u32) -> u64 {
    match cvmx_get_octeon_family() {
        x if x == (OCTEON_CN68XX & OCTEON_FAMILY_MASK) => CVMX_CIU_ADDR(0x100100000, coreid as u64, 0x0F, 8),
        x if x == (OCTEON_CNF75XX & OCTEON_FAMILY_MASK) || x == (OCTEON_CN73XX & OCTEON_FAMILY_MASK) || x == (OCTEON_CN78XX & OCTEON_FAMILY_MASK) => CVMX_CIU_ADDR(0x000020000, coreid as u64, 0x0F, 8).wrapping_sub(0x60000000000),
        _ => CVMX_CIU_ADDR(0x000000500, coreid as u64, 0x0F, 8),
    }
}

// C bitfield layouts are represented by their underlying 64-bit storage.
#[repr(C)] pub union cvmx_ciu_qlm { pub u64_: u64, pub s: cvmx_ciu_qlm_s }
#[repr(C)] pub struct cvmx_ciu_qlm_s { pub bits: u64 }
#[repr(C)] pub union cvmx_ciu_qlm_jtgc { pub u64_: u64, pub s: cvmx_ciu_qlm_jtgc_s }
#[repr(C)] pub struct cvmx_ciu_qlm_jtgc_s { pub bits: u64 }
#[repr(C)] pub union cvmx_ciu_qlm_jtgd { pub u64_: u64, pub s: cvmx_ciu_qlm_jtgd_s }
#[repr(C)] pub struct cvmx_ciu_qlm_jtgd_s { pub bits: u64 }
#[repr(C)] pub union cvmx_ciu_soft_prst { pub u64_: u64, pub s: cvmx_ciu_soft_prst_s }
#[repr(C)] pub struct cvmx_ciu_soft_prst_s { pub bits: u64 }
#[repr(C)] pub union cvmx_ciu_timx { pub u64_: u64, pub s: cvmx_ciu_timx_s }
#[repr(C)] pub struct cvmx_ciu_timx_s { pub bits: u64 }
#[repr(C)] pub union cvmx_ciu_wdogx { pub u64_: u64, pub s: cvmx_ciu_wdogx_s }
#[repr(C)] pub struct cvmx_ciu_wdogx_s { pub bits: u64 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
