/* SPDX-License-Identifier: GPL-2.0-only */
/* Copyright (c) 2011-2012, The Linux Foundation. All rights reserved. */

/* Rust translation of the CP14 register accessor header. */

#[macro_export]
macro_rules! MRC14 {
    ($op1:tt, $crn:tt, $crm:tt, $op2:tt) => {{
        let mut val: u32 = 0;
        unsafe {
            core::arch::asm!(
                concat!("mrc p14, ", stringify!($op1), ", {0}, ", stringify!($crn), ", ", stringify!($crm), ", ", stringify!($op2)),
                out(reg) val,
                options(nostack, preserves_flags)
            );
        }
        val
    }};
}

#[macro_export]
macro_rules! MCR14 {
    ($val:expr, $op1:tt, $crn:tt, $crm:tt, $op2:tt) => {{
        unsafe {
            core::arch::asm!(
                concat!("mcr p14, ", stringify!($op1), ", {0}, ", stringify!($crn), ", ", stringify!($crm), ", ", stringify!($op2)),
                in(reg) $val,
                options(nostack, preserves_flags)
            );
        }
    }};
}

#[macro_export]
macro_rules! dbg_read { ($reg:ident) => { $reg!() }; }
#[macro_export]
macro_rules! dbg_write { ($val:expr, $reg:ident) => { $reg!($val) }; }
#[macro_export]
macro_rules! etm_read { ($reg:ident) => { $reg!() }; }
#[macro_export]
macro_rules! etm_write { ($val:expr, $reg:ident) => { $reg!($val) }; }

/* SPDX-License-Identifier: GPL-2.0-only */
/* Copyright (c) 2011-2012, The Linux Foundation. All rights reserved.
 */
/* Accessors for CP14 registers */
/* MRC14 and MCR14 */
/*
 * Debug Registers
 *
 * Available only in DBGv7
 * DBGECR, DBGDSCCR, DBGDSMCR, DBGDRCR
 *
 * Available only in DBGv7.1
 * DBGBXVRm, DBGOSDLR, DBGDEVID2, DBGDEVID1
 *
 * Read only
 * DBGDIDR, DBGDSCRint, DBGDTRRXint, DBGDRAR, DBGOSLSR, DBGOSSRR, DBGPRSR,
 * DBGPRSR, DBGDSAR, DBGAUTHSTATUS, DBGDEVID2, DBGDEVID1, DBGDEVID
 *
 * Write only
 * DBGDTRTXint, DBGOSLAR
 */
#[macro_export]
macro_rules! RCP14_DBGDIDR { () => { MRC14!(0, c0, c0, 0) }; }
#[macro_export]
macro_rules! RCP14_DBGDSCRint { () => { MRC14!(0, c0, c1, 0) }; }
#[macro_export]
macro_rules! RCP14_DBGDTRRXint { () => { MRC14!(0, c0, c5, 0) }; }
#[macro_export]
macro_rules! RCP14_DBGWFAR { () => { MRC14!(0, c0, c6, 0) }; }
#[macro_export]
macro_rules! RCP14_DBGVCR { () => { MRC14!(0, c0, c7, 0) }; }
#[macro_export]
macro_rules! RCP14_DBGECR { () => { MRC14!(0, c0, c9, 0) }; }
#[macro_export]
macro_rules! RCP14_DBGDSCCR { () => { MRC14!(0, c0, c10, 0) }; }
#[macro_export]
macro_rules! RCP14_DBGDSMCR { () => { MRC14!(0, c0, c11, 0) }; }
#[macro_export]
macro_rules! RCP14_DBGDTRRXext { () => { MRC14!(0, c0, c0, 2) }; }
#[macro_export]
macro_rules! RCP14_DBGDSCRext { () => { MRC14!(0, c0, c2, 2) }; }
#[macro_export]
macro_rules! RCP14_DBGDTRTXext { () => { MRC14!(0, c0, c3, 2) }; }
#[macro_export]
macro_rules! RCP14_DBGDRCR { () => { MRC14!(0, c0, c4, 2) }; }
#[macro_export]
macro_rules! RCP14_DBGBVR0 { () => { MRC14!(0, c0, c0, 4) }; }
#[macro_export]
macro_rules! RCP14_DBGBVR1 { () => { MRC14!(0, c0, c1, 4) }; }
#[macro_export]
macro_rules! RCP14_DBGBVR2 { () => { MRC14!(0, c0, c2, 4) }; }
#[macro_export]
macro_rules! RCP14_DBGBVR3 { () => { MRC14!(0, c0, c3, 4) }; }
#[macro_export]
macro_rules! RCP14_DBGBVR4 { () => { MRC14!(0, c0, c4, 4) }; }
#[macro_export]
macro_rules! RCP14_DBGBVR5 { () => { MRC14!(0, c0, c5, 4) }; }
#[macro_export]
macro_rules! RCP14_DBGBVR6 { () => { MRC14!(0, c0, c6, 4) }; }
#[macro_export]
macro_rules! RCP14_DBGBVR7 { () => { MRC14!(0, c0, c7, 4) }; }
#[macro_export]
macro_rules! RCP14_DBGBVR8 { () => { MRC14!(0, c0, c8, 4) }; }
#[macro_export]
macro_rules! RCP14_DBGBVR9 { () => { MRC14!(0, c0, c9, 4) }; }
#[macro_export]
macro_rules! RCP14_DBGBVR10 { () => { MRC14!(0, c0, c10, 4) }; }
#[macro_export]
macro_rules! RCP14_DBGBVR11 { () => { MRC14!(0, c0, c11, 4) }; }
#[macro_export]
macro_rules! RCP14_DBGBVR12 { () => { MRC14!(0, c0, c12, 4) }; }
#[macro_export]
macro_rules! RCP14_DBGBVR13 { () => { MRC14!(0, c0, c13, 4) }; }
#[macro_export]
macro_rules! RCP14_DBGBVR14 { () => { MRC14!(0, c0, c14, 4) }; }
#[macro_export]
macro_rules! RCP14_DBGBVR15 { () => { MRC14!(0, c0, c15, 4) }; }
#[macro_export]
macro_rules! RCP14_DBGBCR0 { () => { MRC14!(0, c0, c0, 5) }; }
#[macro_export]
macro_rules! RCP14_DBGBCR1 { () => { MRC14!(0, c0, c1, 5) }; }
#[macro_export]
macro_rules! RCP14_DBGBCR2 { () => { MRC14!(0, c0, c2, 5) }; }
#[macro_export]
macro_rules! RCP14_DBGBCR3 { () => { MRC14!(0, c0, c3, 5) }; }
#[macro_export]
macro_rules! RCP14_DBGBCR4 { () => { MRC14!(0, c0, c4, 5) }; }
#[macro_export]
macro_rules! RCP14_DBGBCR5 { () => { MRC14!(0, c0, c5, 5) }; }
#[macro_export]
macro_rules! RCP14_DBGBCR6 { () => { MRC14!(0, c0, c6, 5) }; }
#[macro_export]
macro_rules! RCP14_DBGBCR7 { () => { MRC14!(0, c0, c7, 5) }; }
#[macro_export]
macro_rules! RCP14_DBGBCR8 { () => { MRC14!(0, c0, c8, 5) }; }
#[macro_export]
macro_rules! RCP14_DBGBCR9 { () => { MRC14!(0, c0, c9, 5) }; }
#[macro_export]
macro_rules! RCP14_DBGBCR10 { () => { MRC14!(0, c0, c10, 5) }; }
#[macro_export]
macro_rules! RCP14_DBGBCR11 { () => { MRC14!(0, c0, c11, 5) }; }
#[macro_export]
macro_rules! RCP14_DBGBCR12 { () => { MRC14!(0, c0, c12, 5) }; }
#[macro_export]
macro_rules! RCP14_DBGBCR13 { () => { MRC14!(0, c0, c13, 5) }; }
#[macro_export]
macro_rules! RCP14_DBGBCR14 { () => { MRC14!(0, c0, c14, 5) }; }
#[macro_export]
macro_rules! RCP14_DBGBCR15 { () => { MRC14!(0, c0, c15, 5) }; }
#[macro_export]
macro_rules! RCP14_DBGWVR0 { () => { MRC14!(0, c0, c0, 6) }; }
#[macro_export]
macro_rules! RCP14_DBGWVR1 { () => { MRC14!(0, c0, c1, 6) }; }
#[macro_export]
macro_rules! RCP14_DBGWVR2 { () => { MRC14!(0, c0, c2, 6) }; }
#[macro_export]
macro_rules! RCP14_DBGWVR3 { () => { MRC14!(0, c0, c3, 6) }; }
#[macro_export]
macro_rules! RCP14_DBGWVR4 { () => { MRC14!(0, c0, c4, 6) }; }
#[macro_export]
macro_rules! RCP14_DBGWVR5 { () => { MRC14!(0, c0, c5, 6) }; }
#[macro_export]
macro_rules! RCP14_DBGWVR6 { () => { MRC14!(0, c0, c6, 6) }; }
#[macro_export]
macro_rules! RCP14_DBGWVR7 { () => { MRC14!(0, c0, c7, 6) }; }
#[macro_export]
macro_rules! RCP14_DBGWVR8 { () => { MRC14!(0, c0, c8, 6) }; }
#[macro_export]
macro_rules! RCP14_DBGWVR9 { () => { MRC14!(0, c0, c9, 6) }; }
#[macro_export]
macro_rules! RCP14_DBGWVR10 { () => { MRC14!(0, c0, c10, 6) }; }
#[macro_export]
macro_rules! RCP14_DBGWVR11 { () => { MRC14!(0, c0, c11, 6) }; }
#[macro_export]
macro_rules! RCP14_DBGWVR12 { () => { MRC14!(0, c0, c12, 6) }; }
#[macro_export]
macro_rules! RCP14_DBGWVR13 { () => { MRC14!(0, c0, c13, 6) }; }
#[macro_export]
macro_rules! RCP14_DBGWVR14 { () => { MRC14!(0, c0, c14, 6) }; }
#[macro_export]
macro_rules! RCP14_DBGWVR15 { () => { MRC14!(0, c0, c15, 6) }; }
#[macro_export]
macro_rules! RCP14_DBGWCR0 { () => { MRC14!(0, c0, c0, 7) }; }
#[macro_export]
macro_rules! RCP14_DBGWCR1 { () => { MRC14!(0, c0, c1, 7) }; }
#[macro_export]
macro_rules! RCP14_DBGWCR2 { () => { MRC14!(0, c0, c2, 7) }; }
#[macro_export]
macro_rules! RCP14_DBGWCR3 { () => { MRC14!(0, c0, c3, 7) }; }
#[macro_export]
macro_rules! RCP14_DBGWCR4 { () => { MRC14!(0, c0, c4, 7) }; }
#[macro_export]
macro_rules! RCP14_DBGWCR5 { () => { MRC14!(0, c0, c5, 7) }; }
#[macro_export]
macro_rules! RCP14_DBGWCR6 { () => { MRC14!(0, c0, c6, 7) }; }
#[macro_export]
macro_rules! RCP14_DBGWCR7 { () => { MRC14!(0, c0, c7, 7) }; }
#[macro_export]
macro_rules! RCP14_DBGWCR8 { () => { MRC14!(0, c0, c8, 7) }; }
#[macro_export]
macro_rules! RCP14_DBGWCR9 { () => { MRC14!(0, c0, c9, 7) }; }
#[macro_export]
macro_rules! RCP14_DBGWCR10 { () => { MRC14!(0, c0, c10, 7) }; }
#[macro_export]
macro_rules! RCP14_DBGWCR11 { () => { MRC14!(0, c0, c11, 7) }; }
#[macro_export]
macro_rules! RCP14_DBGWCR12 { () => { MRC14!(0, c0, c12, 7) }; }
#[macro_export]
macro_rules! RCP14_DBGWCR13 { () => { MRC14!(0, c0, c13, 7) }; }
#[macro_export]
macro_rules! RCP14_DBGWCR14 { () => { MRC14!(0, c0, c14, 7) }; }
#[macro_export]
macro_rules! RCP14_DBGWCR15 { () => { MRC14!(0, c0, c15, 7) }; }
#[macro_export]
macro_rules! RCP14_DBGDRAR { () => { MRC14!(0, c1, c0, 0) }; }
#[macro_export]
macro_rules! RCP14_DBGBXVR0 { () => { MRC14!(0, c1, c0, 1) }; }
#[macro_export]
macro_rules! RCP14_DBGBXVR1 { () => { MRC14!(0, c1, c1, 1) }; }
#[macro_export]
macro_rules! RCP14_DBGBXVR2 { () => { MRC14!(0, c1, c2, 1) }; }
#[macro_export]
macro_rules! RCP14_DBGBXVR3 { () => { MRC14!(0, c1, c3, 1) }; }
#[macro_export]
macro_rules! RCP14_DBGBXVR4 { () => { MRC14!(0, c1, c4, 1) }; }
#[macro_export]
macro_rules! RCP14_DBGBXVR5 { () => { MRC14!(0, c1, c5, 1) }; }
#[macro_export]
macro_rules! RCP14_DBGBXVR6 { () => { MRC14!(0, c1, c6, 1) }; }
#[macro_export]
macro_rules! RCP14_DBGBXVR7 { () => { MRC14!(0, c1, c7, 1) }; }
#[macro_export]
macro_rules! RCP14_DBGBXVR8 { () => { MRC14!(0, c1, c8, 1) }; }
#[macro_export]
macro_rules! RCP14_DBGBXVR9 { () => { MRC14!(0, c1, c9, 1) }; }
#[macro_export]
macro_rules! RCP14_DBGBXVR10 { () => { MRC14!(0, c1, c10, 1) }; }
#[macro_export]
macro_rules! RCP14_DBGBXVR11 { () => { MRC14!(0, c1, c11, 1) }; }
#[macro_export]
macro_rules! RCP14_DBGBXVR12 { () => { MRC14!(0, c1, c12, 1) }; }
#[macro_export]
macro_rules! RCP14_DBGBXVR13 { () => { MRC14!(0, c1, c13, 1) }; }
#[macro_export]
macro_rules! RCP14_DBGBXVR14 { () => { MRC14!(0, c1, c14, 1) }; }
#[macro_export]
macro_rules! RCP14_DBGBXVR15 { () => { MRC14!(0, c1, c15, 1) }; }
#[macro_export]
macro_rules! RCP14_DBGOSLSR { () => { MRC14!(0, c1, c1, 4) }; }
#[macro_export]
macro_rules! RCP14_DBGOSSRR { () => { MRC14!(0, c1, c2, 4) }; }
#[macro_export]
macro_rules! RCP14_DBGOSDLR { () => { MRC14!(0, c1, c3, 4) }; }
#[macro_export]
macro_rules! RCP14_DBGPRCR { () => { MRC14!(0, c1, c4, 4) }; }
#[macro_export]
macro_rules! RCP14_DBGPRSR { () => { MRC14!(0, c1, c5, 4) }; }
#[macro_export]
macro_rules! RCP14_DBGDSAR { () => { MRC14!(0, c2, c0, 0) }; }
#[macro_export]
macro_rules! RCP14_DBGITCTRL { () => { MRC14!(0, c7, c0, 4) }; }
#[macro_export]
macro_rules! RCP14_DBGCLAIMSET { () => { MRC14!(0, c7, c8, 6) }; }
#[macro_export]
macro_rules! RCP14_DBGCLAIMCLR { () => { MRC14!(0, c7, c9, 6) }; }
#[macro_export]
macro_rules! RCP14_DBGAUTHSTATUS { () => { MRC14!(0, c7, c14, 6) }; }
#[macro_export]
macro_rules! RCP14_DBGDEVID2 { () => { MRC14!(0, c7, c0, 7) }; }
#[macro_export]
macro_rules! RCP14_DBGDEVID1 { () => { MRC14!(0, c7, c1, 7) }; }
#[macro_export]
macro_rules! RCP14_DBGDEVID { () => { MRC14!(0, c7, c2, 7) }; }
#[macro_export]
macro_rules! WCP14_DBGDTRTXint { ($val:expr) => { MCR14!(val, 0, c0, c5, 0) }; }
#[macro_export]
macro_rules! WCP14_DBGWFAR { ($val:expr) => { MCR14!(val, 0, c0, c6, 0) }; }
#[macro_export]
macro_rules! WCP14_DBGVCR { ($val:expr) => { MCR14!(val, 0, c0, c7, 0) }; }
#[macro_export]
macro_rules! WCP14_DBGECR { ($val:expr) => { MCR14!(val, 0, c0, c9, 0) }; }
#[macro_export]
macro_rules! WCP14_DBGDSCCR { ($val:expr) => { MCR14!(val, 0, c0, c10, 0) }; }
#[macro_export]
macro_rules! WCP14_DBGDSMCR { ($val:expr) => { MCR14!(val, 0, c0, c11, 0) }; }
#[macro_export]
macro_rules! WCP14_DBGDTRRXext { ($val:expr) => { MCR14!(val, 0, c0, c0, 2) }; }
#[macro_export]
macro_rules! WCP14_DBGDSCRext { ($val:expr) => { MCR14!(val, 0, c0, c2, 2) }; }
#[macro_export]
macro_rules! WCP14_DBGDTRTXext { ($val:expr) => { MCR14!(val, 0, c0, c3, 2) }; }
#[macro_export]
macro_rules! WCP14_DBGDRCR { ($val:expr) => { MCR14!(val, 0, c0, c4, 2) }; }
#[macro_export]
macro_rules! WCP14_DBGBVR0 { ($val:expr) => { MCR14!(val, 0, c0, c0, 4) }; }
#[macro_export]
macro_rules! WCP14_DBGBVR1 { ($val:expr) => { MCR14!(val, 0, c0, c1, 4) }; }
#[macro_export]
macro_rules! WCP14_DBGBVR2 { ($val:expr) => { MCR14!(val, 0, c0, c2, 4) }; }
#[macro_export]
macro_rules! WCP14_DBGBVR3 { ($val:expr) => { MCR14!(val, 0, c0, c3, 4) }; }
#[macro_export]
macro_rules! WCP14_DBGBVR4 { ($val:expr) => { MCR14!(val, 0, c0, c4, 4) }; }
#[macro_export]
macro_rules! WCP14_DBGBVR5 { ($val:expr) => { MCR14!(val, 0, c0, c5, 4) }; }
#[macro_export]
macro_rules! WCP14_DBGBVR6 { ($val:expr) => { MCR14!(val, 0, c0, c6, 4) }; }
#[macro_export]
macro_rules! WCP14_DBGBVR7 { ($val:expr) => { MCR14!(val, 0, c0, c7, 4) }; }
#[macro_export]
macro_rules! WCP14_DBGBVR8 { ($val:expr) => { MCR14!(val, 0, c0, c8, 4) }; }
#[macro_export]
macro_rules! WCP14_DBGBVR9 { ($val:expr) => { MCR14!(val, 0, c0, c9, 4) }; }
#[macro_export]
macro_rules! WCP14_DBGBVR10 { ($val:expr) => { MCR14!(val, 0, c0, c10, 4) }; }
#[macro_export]
macro_rules! WCP14_DBGBVR11 { ($val:expr) => { MCR14!(val, 0, c0, c11, 4) }; }
#[macro_export]
macro_rules! WCP14_DBGBVR12 { ($val:expr) => { MCR14!(val, 0, c0, c12, 4) }; }
#[macro_export]
macro_rules! WCP14_DBGBVR13 { ($val:expr) => { MCR14!(val, 0, c0, c13, 4) }; }
#[macro_export]
macro_rules! WCP14_DBGBVR14 { ($val:expr) => { MCR14!(val, 0, c0, c14, 4) }; }
#[macro_export]
macro_rules! WCP14_DBGBVR15 { ($val:expr) => { MCR14!(val, 0, c0, c15, 4) }; }
#[macro_export]
macro_rules! WCP14_DBGBCR0 { ($val:expr) => { MCR14!(val, 0, c0, c0, 5) }; }
#[macro_export]
macro_rules! WCP14_DBGBCR1 { ($val:expr) => { MCR14!(val, 0, c0, c1, 5) }; }
#[macro_export]
macro_rules! WCP14_DBGBCR2 { ($val:expr) => { MCR14!(val, 0, c0, c2, 5) }; }
#[macro_export]
macro_rules! WCP14_DBGBCR3 { ($val:expr) => { MCR14!(val, 0, c0, c3, 5) }; }
#[macro_export]
macro_rules! WCP14_DBGBCR4 { ($val:expr) => { MCR14!(val, 0, c0, c4, 5) }; }
#[macro_export]
macro_rules! WCP14_DBGBCR5 { ($val:expr) => { MCR14!(val, 0, c0, c5, 5) }; }
#[macro_export]
macro_rules! WCP14_DBGBCR6 { ($val:expr) => { MCR14!(val, 0, c0, c6, 5) }; }
#[macro_export]
macro_rules! WCP14_DBGBCR7 { ($val:expr) => { MCR14!(val, 0, c0, c7, 5) }; }
#[macro_export]
macro_rules! WCP14_DBGBCR8 { ($val:expr) => { MCR14!(val, 0, c0, c8, 5) }; }
#[macro_export]
macro_rules! WCP14_DBGBCR9 { ($val:expr) => { MCR14!(val, 0, c0, c9, 5) }; }
#[macro_export]
macro_rules! WCP14_DBGBCR10 { ($val:expr) => { MCR14!(val, 0, c0, c10, 5) }; }
#[macro_export]
macro_rules! WCP14_DBGBCR11 { ($val:expr) => { MCR14!(val, 0, c0, c11, 5) }; }
#[macro_export]
macro_rules! WCP14_DBGBCR12 { ($val:expr) => { MCR14!(val, 0, c0, c12, 5) }; }
#[macro_export]
macro_rules! WCP14_DBGBCR13 { ($val:expr) => { MCR14!(val, 0, c0, c13, 5) }; }
#[macro_export]
macro_rules! WCP14_DBGBCR14 { ($val:expr) => { MCR14!(val, 0, c0, c14, 5) }; }
#[macro_export]
macro_rules! WCP14_DBGBCR15 { ($val:expr) => { MCR14!(val, 0, c0, c15, 5) }; }
#[macro_export]
macro_rules! WCP14_DBGWVR0 { ($val:expr) => { MCR14!(val, 0, c0, c0, 6) }; }
#[macro_export]
macro_rules! WCP14_DBGWVR1 { ($val:expr) => { MCR14!(val, 0, c0, c1, 6) }; }
#[macro_export]
macro_rules! WCP14_DBGWVR2 { ($val:expr) => { MCR14!(val, 0, c0, c2, 6) }; }
#[macro_export]
macro_rules! WCP14_DBGWVR3 { ($val:expr) => { MCR14!(val, 0, c0, c3, 6) }; }
#[macro_export]
macro_rules! WCP14_DBGWVR4 { ($val:expr) => { MCR14!(val, 0, c0, c4, 6) }; }
#[macro_export]
macro_rules! WCP14_DBGWVR5 { ($val:expr) => { MCR14!(val, 0, c0, c5, 6) }; }
#[macro_export]
macro_rules! WCP14_DBGWVR6 { ($val:expr) => { MCR14!(val, 0, c0, c6, 6) }; }
#[macro_export]
macro_rules! WCP14_DBGWVR7 { ($val:expr) => { MCR14!(val, 0, c0, c7, 6) }; }
#[macro_export]
macro_rules! WCP14_DBGWVR8 { ($val:expr) => { MCR14!(val, 0, c0, c8, 6) }; }
#[macro_export]
macro_rules! WCP14_DBGWVR9 { ($val:expr) => { MCR14!(val, 0, c0, c9, 6) }; }
#[macro_export]
macro_rules! WCP14_DBGWVR10 { ($val:expr) => { MCR14!(val, 0, c0, c10, 6) }; }
#[macro_export]
macro_rules! WCP14_DBGWVR11 { ($val:expr) => { MCR14!(val, 0, c0, c11, 6) }; }
#[macro_export]
macro_rules! WCP14_DBGWVR12 { ($val:expr) => { MCR14!(val, 0, c0, c12, 6) }; }
#[macro_export]
macro_rules! WCP14_DBGWVR13 { ($val:expr) => { MCR14!(val, 0, c0, c13, 6) }; }
#[macro_export]
macro_rules! WCP14_DBGWVR14 { ($val:expr) => { MCR14!(val, 0, c0, c14, 6) }; }
#[macro_export]
macro_rules! WCP14_DBGWVR15 { ($val:expr) => { MCR14!(val, 0, c0, c15, 6) }; }
#[macro_export]
macro_rules! WCP14_DBGWCR0 { ($val:expr) => { MCR14!(val, 0, c0, c0, 7) }; }
#[macro_export]
macro_rules! WCP14_DBGWCR1 { ($val:expr) => { MCR14!(val, 0, c0, c1, 7) }; }
#[macro_export]
macro_rules! WCP14_DBGWCR2 { ($val:expr) => { MCR14!(val, 0, c0, c2, 7) }; }
#[macro_export]
macro_rules! WCP14_DBGWCR3 { ($val:expr) => { MCR14!(val, 0, c0, c3, 7) }; }
#[macro_export]
macro_rules! WCP14_DBGWCR4 { ($val:expr) => { MCR14!(val, 0, c0, c4, 7) }; }
#[macro_export]
macro_rules! WCP14_DBGWCR5 { ($val:expr) => { MCR14!(val, 0, c0, c5, 7) }; }
#[macro_export]
macro_rules! WCP14_DBGWCR6 { ($val:expr) => { MCR14!(val, 0, c0, c6, 7) }; }
#[macro_export]
macro_rules! WCP14_DBGWCR7 { ($val:expr) => { MCR14!(val, 0, c0, c7, 7) }; }
#[macro_export]
macro_rules! WCP14_DBGWCR8 { ($val:expr) => { MCR14!(val, 0, c0, c8, 7) }; }
#[macro_export]
macro_rules! WCP14_DBGWCR9 { ($val:expr) => { MCR14!(val, 0, c0, c9, 7) }; }
#[macro_export]
macro_rules! WCP14_DBGWCR10 { ($val:expr) => { MCR14!(val, 0, c0, c10, 7) }; }
#[macro_export]
macro_rules! WCP14_DBGWCR11 { ($val:expr) => { MCR14!(val, 0, c0, c11, 7) }; }
#[macro_export]
macro_rules! WCP14_DBGWCR12 { ($val:expr) => { MCR14!(val, 0, c0, c12, 7) }; }
#[macro_export]
macro_rules! WCP14_DBGWCR13 { ($val:expr) => { MCR14!(val, 0, c0, c13, 7) }; }
#[macro_export]
macro_rules! WCP14_DBGWCR14 { ($val:expr) => { MCR14!(val, 0, c0, c14, 7) }; }
#[macro_export]
macro_rules! WCP14_DBGWCR15 { ($val:expr) => { MCR14!(val, 0, c0, c15, 7) }; }
#[macro_export]
macro_rules! WCP14_DBGBXVR0 { ($val:expr) => { MCR14!(val, 0, c1, c0, 1) }; }
#[macro_export]
macro_rules! WCP14_DBGBXVR1 { ($val:expr) => { MCR14!(val, 0, c1, c1, 1) }; }
#[macro_export]
macro_rules! WCP14_DBGBXVR2 { ($val:expr) => { MCR14!(val, 0, c1, c2, 1) }; }
#[macro_export]
macro_rules! WCP14_DBGBXVR3 { ($val:expr) => { MCR14!(val, 0, c1, c3, 1) }; }
#[macro_export]
macro_rules! WCP14_DBGBXVR4 { ($val:expr) => { MCR14!(val, 0, c1, c4, 1) }; }
#[macro_export]
macro_rules! WCP14_DBGBXVR5 { ($val:expr) => { MCR14!(val, 0, c1, c5, 1) }; }
#[macro_export]
macro_rules! WCP14_DBGBXVR6 { ($val:expr) => { MCR14!(val, 0, c1, c6, 1) }; }
#[macro_export]
macro_rules! WCP14_DBGBXVR7 { ($val:expr) => { MCR14!(val, 0, c1, c7, 1) }; }
#[macro_export]
macro_rules! WCP14_DBGBXVR8 { ($val:expr) => { MCR14!(val, 0, c1, c8, 1) }; }
#[macro_export]
macro_rules! WCP14_DBGBXVR9 { ($val:expr) => { MCR14!(val, 0, c1, c9, 1) }; }
#[macro_export]
macro_rules! WCP14_DBGBXVR10 { ($val:expr) => { MCR14!(val, 0, c1, c10, 1) }; }
#[macro_export]
macro_rules! WCP14_DBGBXVR11 { ($val:expr) => { MCR14!(val, 0, c1, c11, 1) }; }
#[macro_export]
macro_rules! WCP14_DBGBXVR12 { ($val:expr) => { MCR14!(val, 0, c1, c12, 1) }; }
#[macro_export]
macro_rules! WCP14_DBGBXVR13 { ($val:expr) => { MCR14!(val, 0, c1, c13, 1) }; }
#[macro_export]
macro_rules! WCP14_DBGBXVR14 { ($val:expr) => { MCR14!(val, 0, c1, c14, 1) }; }
#[macro_export]
macro_rules! WCP14_DBGBXVR15 { ($val:expr) => { MCR14!(val, 0, c1, c15, 1) }; }
#[macro_export]
macro_rules! WCP14_DBGOSLAR { ($val:expr) => { MCR14!(val, 0, c1, c0, 4) }; }
#[macro_export]
macro_rules! WCP14_DBGOSSRR { ($val:expr) => { MCR14!(val, 0, c1, c2, 4) }; }
#[macro_export]
macro_rules! WCP14_DBGOSDLR { ($val:expr) => { MCR14!(val, 0, c1, c3, 4) }; }
#[macro_export]
macro_rules! WCP14_DBGPRCR { ($val:expr) => { MCR14!(val, 0, c1, c4, 4) }; }
#[macro_export]
macro_rules! WCP14_DBGITCTRL { ($val:expr) => { MCR14!(val, 0, c7, c0, 4) }; }
#[macro_export]
macro_rules! WCP14_DBGCLAIMSET { ($val:expr) => { MCR14!(val, 0, c7, c8, 6) }; }
#[macro_export]
macro_rules! WCP14_DBGCLAIMCLR { ($val:expr) => { MCR14!(val, 0, c7, c9, 6) }; }
/*
 * ETM Registers
 *
 * Available only in ETMv3.3, 3.4, 3.5
 * ETMASICCR, ETMTECR2, ETMFFRR, ETMVDEVR, ETMVDCR1, ETMVDCR2, ETMVDCR3,
 * ETMDCVRn, ETMDCMRn
 *
 * Available only in ETMv3.5 as read only
 * ETMIDR2
 *
 * Available only in ETMv3.5, PFTv1.0, 1.1
 * ETMTSEVR, ETMVMIDCVR, ETMPDCR
 *
 * Read only
 * ETMCCR, ETMSCR, ETMIDR, ETMCCER, ETMOSLSR
 * ETMLSR, ETMAUTHSTATUS, ETMDEVID, ETMDEVTYPE, ETMPIDR4, ETMPIDR5, ETMPIDR6,
 * ETMPIDR7, ETMPIDR0, ETMPIDR1, ETMPIDR2, ETMPIDR2, ETMPIDR3, ETMCIDR0,
 * ETMCIDR1, ETMCIDR2, ETMCIDR3
 *
 * Write only
 * ETMOSLAR, ETMLAR
 * Note: ETMCCER[11] controls WO nature of certain regs. Refer ETM arch spec.
 */
#[macro_export]
macro_rules! RCP14_ETMCR { () => { MRC14!(1, c0, c0, 0) }; }
#[macro_export]
macro_rules! RCP14_ETMCCR { () => { MRC14!(1, c0, c1, 0) }; }
#[macro_export]
macro_rules! RCP14_ETMTRIGGER { () => { MRC14!(1, c0, c2, 0) }; }
#[macro_export]
macro_rules! RCP14_ETMASICCR { () => { MRC14!(1, c0, c3, 0) }; }
#[macro_export]
macro_rules! RCP14_ETMSR { () => { MRC14!(1, c0, c4, 0) }; }
#[macro_export]
macro_rules! RCP14_ETMSCR { () => { MRC14!(1, c0, c5, 0) }; }
#[macro_export]
macro_rules! RCP14_ETMTSSCR { () => { MRC14!(1, c0, c6, 0) }; }
#[macro_export]
macro_rules! RCP14_ETMTECR2 { () => { MRC14!(1, c0, c7, 0) }; }
#[macro_export]
macro_rules! RCP14_ETMTEEVR { () => { MRC14!(1, c0, c8, 0) }; }
#[macro_export]
macro_rules! RCP14_ETMTECR1 { () => { MRC14!(1, c0, c9, 0) }; }
#[macro_export]
macro_rules! RCP14_ETMFFRR { () => { MRC14!(1, c0, c10, 0) }; }
#[macro_export]
macro_rules! RCP14_ETMFFLR { () => { MRC14!(1, c0, c11, 0) }; }
#[macro_export]
macro_rules! RCP14_ETMVDEVR { () => { MRC14!(1, c0, c12, 0) }; }
#[macro_export]
macro_rules! RCP14_ETMVDCR1 { () => { MRC14!(1, c0, c13, 0) }; }
#[macro_export]
macro_rules! RCP14_ETMVDCR2 { () => { MRC14!(1, c0, c14, 0) }; }
#[macro_export]
macro_rules! RCP14_ETMVDCR3 { () => { MRC14!(1, c0, c15, 0) }; }
#[macro_export]
macro_rules! RCP14_ETMACVR0 { () => { MRC14!(1, c0, c0, 1) }; }
#[macro_export]
macro_rules! RCP14_ETMACVR1 { () => { MRC14!(1, c0, c1, 1) }; }
#[macro_export]
macro_rules! RCP14_ETMACVR2 { () => { MRC14!(1, c0, c2, 1) }; }
#[macro_export]
macro_rules! RCP14_ETMACVR3 { () => { MRC14!(1, c0, c3, 1) }; }
#[macro_export]
macro_rules! RCP14_ETMACVR4 { () => { MRC14!(1, c0, c4, 1) }; }
#[macro_export]
macro_rules! RCP14_ETMACVR5 { () => { MRC14!(1, c0, c5, 1) }; }
#[macro_export]
macro_rules! RCP14_ETMACVR6 { () => { MRC14!(1, c0, c6, 1) }; }
#[macro_export]
macro_rules! RCP14_ETMACVR7 { () => { MRC14!(1, c0, c7, 1) }; }
#[macro_export]
macro_rules! RCP14_ETMACVR8 { () => { MRC14!(1, c0, c8, 1) }; }
#[macro_export]
macro_rules! RCP14_ETMACVR9 { () => { MRC14!(1, c0, c9, 1) }; }
#[macro_export]
macro_rules! RCP14_ETMACVR10 { () => { MRC14!(1, c0, c10, 1) }; }
#[macro_export]
macro_rules! RCP14_ETMACVR11 { () => { MRC14!(1, c0, c11, 1) }; }
#[macro_export]
macro_rules! RCP14_ETMACVR12 { () => { MRC14!(1, c0, c12, 1) }; }
#[macro_export]
macro_rules! RCP14_ETMACVR13 { () => { MRC14!(1, c0, c13, 1) }; }
#[macro_export]
macro_rules! RCP14_ETMACVR14 { () => { MRC14!(1, c0, c14, 1) }; }
#[macro_export]
macro_rules! RCP14_ETMACVR15 { () => { MRC14!(1, c0, c15, 1) }; }
#[macro_export]
macro_rules! RCP14_ETMACTR0 { () => { MRC14!(1, c0, c0, 2) }; }
#[macro_export]
macro_rules! RCP14_ETMACTR1 { () => { MRC14!(1, c0, c1, 2) }; }
#[macro_export]
macro_rules! RCP14_ETMACTR2 { () => { MRC14!(1, c0, c2, 2) }; }
#[macro_export]
macro_rules! RCP14_ETMACTR3 { () => { MRC14!(1, c0, c3, 2) }; }
#[macro_export]
macro_rules! RCP14_ETMACTR4 { () => { MRC14!(1, c0, c4, 2) }; }
#[macro_export]
macro_rules! RCP14_ETMACTR5 { () => { MRC14!(1, c0, c5, 2) }; }
#[macro_export]
macro_rules! RCP14_ETMACTR6 { () => { MRC14!(1, c0, c6, 2) }; }
#[macro_export]
macro_rules! RCP14_ETMACTR7 { () => { MRC14!(1, c0, c7, 2) }; }
#[macro_export]
macro_rules! RCP14_ETMACTR8 { () => { MRC14!(1, c0, c8, 2) }; }
#[macro_export]
macro_rules! RCP14_ETMACTR9 { () => { MRC14!(1, c0, c9, 2) }; }
#[macro_export]
macro_rules! RCP14_ETMACTR10 { () => { MRC14!(1, c0, c10, 2) }; }
#[macro_export]
macro_rules! RCP14_ETMACTR11 { () => { MRC14!(1, c0, c11, 2) }; }
#[macro_export]
macro_rules! RCP14_ETMACTR12 { () => { MRC14!(1, c0, c12, 2) }; }
#[macro_export]
macro_rules! RCP14_ETMACTR13 { () => { MRC14!(1, c0, c13, 2) }; }
#[macro_export]
macro_rules! RCP14_ETMACTR14 { () => { MRC14!(1, c0, c14, 2) }; }
#[macro_export]
macro_rules! RCP14_ETMACTR15 { () => { MRC14!(1, c0, c15, 2) }; }
#[macro_export]
macro_rules! RCP14_ETMDCVR0 { () => { MRC14!(1, c0, c0, 3) }; }
#[macro_export]
macro_rules! RCP14_ETMDCVR2 { () => { MRC14!(1, c0, c2, 3) }; }
#[macro_export]
macro_rules! RCP14_ETMDCVR4 { () => { MRC14!(1, c0, c4, 3) }; }
#[macro_export]
macro_rules! RCP14_ETMDCVR6 { () => { MRC14!(1, c0, c6, 3) }; }
#[macro_export]
macro_rules! RCP14_ETMDCVR8 { () => { MRC14!(1, c0, c8, 3) }; }
#[macro_export]
macro_rules! RCP14_ETMDCVR10 { () => { MRC14!(1, c0, c10, 3) }; }
#[macro_export]
macro_rules! RCP14_ETMDCVR12 { () => { MRC14!(1, c0, c12, 3) }; }
#[macro_export]
macro_rules! RCP14_ETMDCVR14 { () => { MRC14!(1, c0, c14, 3) }; }
#[macro_export]
macro_rules! RCP14_ETMDCMR0 { () => { MRC14!(1, c0, c0, 4) }; }
#[macro_export]
macro_rules! RCP14_ETMDCMR2 { () => { MRC14!(1, c0, c2, 4) }; }
#[macro_export]
macro_rules! RCP14_ETMDCMR4 { () => { MRC14!(1, c0, c4, 4) }; }
#[macro_export]
macro_rules! RCP14_ETMDCMR6 { () => { MRC14!(1, c0, c6, 4) }; }
#[macro_export]
macro_rules! RCP14_ETMDCMR8 { () => { MRC14!(1, c0, c8, 4) }; }
#[macro_export]
macro_rules! RCP14_ETMDCMR10 { () => { MRC14!(1, c0, c10, 4) }; }
#[macro_export]
macro_rules! RCP14_ETMDCMR12 { () => { MRC14!(1, c0, c12, 4) }; }
#[macro_export]
macro_rules! RCP14_ETMDCMR14 { () => { MRC14!(1, c0, c14, 4) }; }
#[macro_export]
macro_rules! RCP14_ETMCNTRLDVR0 { () => { MRC14!(1, c0, c0, 5) }; }
#[macro_export]
macro_rules! RCP14_ETMCNTRLDVR1 { () => { MRC14!(1, c0, c1, 5) }; }
#[macro_export]
macro_rules! RCP14_ETMCNTRLDVR2 { () => { MRC14!(1, c0, c2, 5) }; }
#[macro_export]
macro_rules! RCP14_ETMCNTRLDVR3 { () => { MRC14!(1, c0, c3, 5) }; }
#[macro_export]
macro_rules! RCP14_ETMCNTENR0 { () => { MRC14!(1, c0, c4, 5) }; }
#[macro_export]
macro_rules! RCP14_ETMCNTENR1 { () => { MRC14!(1, c0, c5, 5) }; }
#[macro_export]
macro_rules! RCP14_ETMCNTENR2 { () => { MRC14!(1, c0, c6, 5) }; }
#[macro_export]
macro_rules! RCP14_ETMCNTENR3 { () => { MRC14!(1, c0, c7, 5) }; }
#[macro_export]
macro_rules! RCP14_ETMCNTRLDEVR0 { () => { MRC14!(1, c0, c8, 5) }; }
#[macro_export]
macro_rules! RCP14_ETMCNTRLDEVR1 { () => { MRC14!(1, c0, c9, 5) }; }
#[macro_export]
macro_rules! RCP14_ETMCNTRLDEVR2 { () => { MRC14!(1, c0, c10, 5) }; }
#[macro_export]
macro_rules! RCP14_ETMCNTRLDEVR3 { () => { MRC14!(1, c0, c11, 5) }; }
#[macro_export]
macro_rules! RCP14_ETMCNTVR0 { () => { MRC14!(1, c0, c12, 5) }; }
#[macro_export]
macro_rules! RCP14_ETMCNTVR1 { () => { MRC14!(1, c0, c13, 5) }; }
#[macro_export]
macro_rules! RCP14_ETMCNTVR2 { () => { MRC14!(1, c0, c14, 5) }; }
#[macro_export]
macro_rules! RCP14_ETMCNTVR3 { () => { MRC14!(1, c0, c15, 5) }; }
#[macro_export]
macro_rules! RCP14_ETMSQ12EVR { () => { MRC14!(1, c0, c0, 6) }; }
#[macro_export]
macro_rules! RCP14_ETMSQ21EVR { () => { MRC14!(1, c0, c1, 6) }; }
#[macro_export]
macro_rules! RCP14_ETMSQ23EVR { () => { MRC14!(1, c0, c2, 6) }; }
#[macro_export]
macro_rules! RCP14_ETMSQ31EVR { () => { MRC14!(1, c0, c3, 6) }; }
#[macro_export]
macro_rules! RCP14_ETMSQ32EVR { () => { MRC14!(1, c0, c4, 6) }; }
#[macro_export]
macro_rules! RCP14_ETMSQ13EVR { () => { MRC14!(1, c0, c5, 6) }; }
#[macro_export]
macro_rules! RCP14_ETMSQR { () => { MRC14!(1, c0, c7, 6) }; }
#[macro_export]
macro_rules! RCP14_ETMEXTOUTEVR0 { () => { MRC14!(1, c0, c8, 6) }; }
#[macro_export]
macro_rules! RCP14_ETMEXTOUTEVR1 { () => { MRC14!(1, c0, c9, 6) }; }
#[macro_export]
macro_rules! RCP14_ETMEXTOUTEVR2 { () => { MRC14!(1, c0, c10, 6) }; }
#[macro_export]
macro_rules! RCP14_ETMEXTOUTEVR3 { () => { MRC14!(1, c0, c11, 6) }; }
#[macro_export]
macro_rules! RCP14_ETMCIDCVR0 { () => { MRC14!(1, c0, c12, 6) }; }
#[macro_export]
macro_rules! RCP14_ETMCIDCVR1 { () => { MRC14!(1, c0, c13, 6) }; }
#[macro_export]
macro_rules! RCP14_ETMCIDCVR2 { () => { MRC14!(1, c0, c14, 6) }; }
#[macro_export]
macro_rules! RCP14_ETMCIDCMR { () => { MRC14!(1, c0, c15, 6) }; }
#[macro_export]
macro_rules! RCP14_ETMIMPSPEC0 { () => { MRC14!(1, c0, c0, 7) }; }
#[macro_export]
macro_rules! RCP14_ETMIMPSPEC1 { () => { MRC14!(1, c0, c1, 7) }; }
#[macro_export]
macro_rules! RCP14_ETMIMPSPEC2 { () => { MRC14!(1, c0, c2, 7) }; }
#[macro_export]
macro_rules! RCP14_ETMIMPSPEC3 { () => { MRC14!(1, c0, c3, 7) }; }
#[macro_export]
macro_rules! RCP14_ETMIMPSPEC4 { () => { MRC14!(1, c0, c4, 7) }; }
#[macro_export]
macro_rules! RCP14_ETMIMPSPEC5 { () => { MRC14!(1, c0, c5, 7) }; }
#[macro_export]
macro_rules! RCP14_ETMIMPSPEC6 { () => { MRC14!(1, c0, c6, 7) }; }
#[macro_export]
macro_rules! RCP14_ETMIMPSPEC7 { () => { MRC14!(1, c0, c7, 7) }; }
#[macro_export]
macro_rules! RCP14_ETMSYNCFR { () => { MRC14!(1, c0, c8, 7) }; }
#[macro_export]
macro_rules! RCP14_ETMIDR { () => { MRC14!(1, c0, c9, 7) }; }
#[macro_export]
macro_rules! RCP14_ETMCCER { () => { MRC14!(1, c0, c10, 7) }; }
#[macro_export]
macro_rules! RCP14_ETMEXTINSELR { () => { MRC14!(1, c0, c11, 7) }; }
#[macro_export]
macro_rules! RCP14_ETMTESSEICR { () => { MRC14!(1, c0, c12, 7) }; }
#[macro_export]
macro_rules! RCP14_ETMEIBCR { () => { MRC14!(1, c0, c13, 7) }; }
#[macro_export]
macro_rules! RCP14_ETMTSEVR { () => { MRC14!(1, c0, c14, 7) }; }
#[macro_export]
macro_rules! RCP14_ETMAUXCR { () => { MRC14!(1, c0, c15, 7) }; }
#[macro_export]
macro_rules! RCP14_ETMTRACEIDR { () => { MRC14!(1, c1, c0, 0) }; }
#[macro_export]
macro_rules! RCP14_ETMIDR2 { () => { MRC14!(1, c1, c2, 0) }; }
#[macro_export]
macro_rules! RCP14_ETMVMIDCVR { () => { MRC14!(1, c1, c0, 1) }; }
#[macro_export]
macro_rules! RCP14_ETMOSLSR { () => { MRC14!(1, c1, c1, 4) }; }
/* Not available in PFTv1.1 */
#[macro_export]
macro_rules! RCP14_ETMOSSRR { () => { MRC14!(1, c1, c2, 4) }; }
#[macro_export]
macro_rules! RCP14_ETMPDCR { () => { MRC14!(1, c1, c4, 4) }; }
#[macro_export]
macro_rules! RCP14_ETMPDSR { () => { MRC14!(1, c1, c5, 4) }; }
#[macro_export]
macro_rules! RCP14_ETMITCTRL { () => { MRC14!(1, c7, c0, 4) }; }
#[macro_export]
macro_rules! RCP14_ETMCLAIMSET { () => { MRC14!(1, c7, c8, 6) }; }
#[macro_export]
macro_rules! RCP14_ETMCLAIMCLR { () => { MRC14!(1, c7, c9, 6) }; }
#[macro_export]
macro_rules! RCP14_ETMLSR { () => { MRC14!(1, c7, c13, 6) }; }
#[macro_export]
macro_rules! RCP14_ETMAUTHSTATUS { () => { MRC14!(1, c7, c14, 6) }; }
#[macro_export]
macro_rules! RCP14_ETMDEVID { () => { MRC14!(1, c7, c2, 7) }; }
#[macro_export]
macro_rules! RCP14_ETMDEVTYPE { () => { MRC14!(1, c7, c3, 7) }; }
#[macro_export]
macro_rules! RCP14_ETMPIDR4 { () => { MRC14!(1, c7, c4, 7) }; }
#[macro_export]
macro_rules! RCP14_ETMPIDR5 { () => { MRC14!(1, c7, c5, 7) }; }
#[macro_export]
macro_rules! RCP14_ETMPIDR6 { () => { MRC14!(1, c7, c6, 7) }; }
#[macro_export]
macro_rules! RCP14_ETMPIDR7 { () => { MRC14!(1, c7, c7, 7) }; }
#[macro_export]
macro_rules! RCP14_ETMPIDR0 { () => { MRC14!(1, c7, c8, 7) }; }
#[macro_export]
macro_rules! RCP14_ETMPIDR1 { () => { MRC14!(1, c7, c9, 7) }; }
#[macro_export]
macro_rules! RCP14_ETMPIDR2 { () => { MRC14!(1, c7, c10, 7) }; }
#[macro_export]
macro_rules! RCP14_ETMPIDR3 { () => { MRC14!(1, c7, c11, 7) }; }
#[macro_export]
macro_rules! RCP14_ETMCIDR0 { () => { MRC14!(1, c7, c12, 7) }; }
#[macro_export]
macro_rules! RCP14_ETMCIDR1 { () => { MRC14!(1, c7, c13, 7) }; }
#[macro_export]
macro_rules! RCP14_ETMCIDR2 { () => { MRC14!(1, c7, c14, 7) }; }
#[macro_export]
macro_rules! RCP14_ETMCIDR3 { () => { MRC14!(1, c7, c15, 7) }; }
#[macro_export]
macro_rules! WCP14_ETMCR { ($val:expr) => { MCR14!(val, 1, c0, c0, 0) }; }
#[macro_export]
macro_rules! WCP14_ETMTRIGGER { ($val:expr) => { MCR14!(val, 1, c0, c2, 0) }; }
#[macro_export]
macro_rules! WCP14_ETMASICCR { ($val:expr) => { MCR14!(val, 1, c0, c3, 0) }; }
#[macro_export]
macro_rules! WCP14_ETMSR { ($val:expr) => { MCR14!(val, 1, c0, c4, 0) }; }
#[macro_export]
macro_rules! WCP14_ETMTSSCR { ($val:expr) => { MCR14!(val, 1, c0, c6, 0) }; }
#[macro_export]
macro_rules! WCP14_ETMTECR2 { ($val:expr) => { MCR14!(val, 1, c0, c7, 0) }; }
#[macro_export]
macro_rules! WCP14_ETMTEEVR { ($val:expr) => { MCR14!(val, 1, c0, c8, 0) }; }
#[macro_export]
macro_rules! WCP14_ETMTECR1 { ($val:expr) => { MCR14!(val, 1, c0, c9, 0) }; }
#[macro_export]
macro_rules! WCP14_ETMFFRR { ($val:expr) => { MCR14!(val, 1, c0, c10, 0) }; }
#[macro_export]
macro_rules! WCP14_ETMFFLR { ($val:expr) => { MCR14!(val, 1, c0, c11, 0) }; }
#[macro_export]
macro_rules! WCP14_ETMVDEVR { ($val:expr) => { MCR14!(val, 1, c0, c12, 0) }; }
#[macro_export]
macro_rules! WCP14_ETMVDCR1 { ($val:expr) => { MCR14!(val, 1, c0, c13, 0) }; }
#[macro_export]
macro_rules! WCP14_ETMVDCR2 { ($val:expr) => { MCR14!(val, 1, c0, c14, 0) }; }
#[macro_export]
macro_rules! WCP14_ETMVDCR3 { ($val:expr) => { MCR14!(val, 1, c0, c15, 0) }; }
#[macro_export]
macro_rules! WCP14_ETMACVR0 { ($val:expr) => { MCR14!(val, 1, c0, c0, 1) }; }
#[macro_export]
macro_rules! WCP14_ETMACVR1 { ($val:expr) => { MCR14!(val, 1, c0, c1, 1) }; }
#[macro_export]
macro_rules! WCP14_ETMACVR2 { ($val:expr) => { MCR14!(val, 1, c0, c2, 1) }; }
#[macro_export]
macro_rules! WCP14_ETMACVR3 { ($val:expr) => { MCR14!(val, 1, c0, c3, 1) }; }
#[macro_export]
macro_rules! WCP14_ETMACVR4 { ($val:expr) => { MCR14!(val, 1, c0, c4, 1) }; }
#[macro_export]
macro_rules! WCP14_ETMACVR5 { ($val:expr) => { MCR14!(val, 1, c0, c5, 1) }; }
#[macro_export]
macro_rules! WCP14_ETMACVR6 { ($val:expr) => { MCR14!(val, 1, c0, c6, 1) }; }
#[macro_export]
macro_rules! WCP14_ETMACVR7 { ($val:expr) => { MCR14!(val, 1, c0, c7, 1) }; }
#[macro_export]
macro_rules! WCP14_ETMACVR8 { ($val:expr) => { MCR14!(val, 1, c0, c8, 1) }; }
#[macro_export]
macro_rules! WCP14_ETMACVR9 { ($val:expr) => { MCR14!(val, 1, c0, c9, 1) }; }
#[macro_export]
macro_rules! WCP14_ETMACVR10 { ($val:expr) => { MCR14!(val, 1, c0, c10, 1) }; }
#[macro_export]
macro_rules! WCP14_ETMACVR11 { ($val:expr) => { MCR14!(val, 1, c0, c11, 1) }; }
#[macro_export]
macro_rules! WCP14_ETMACVR12 { ($val:expr) => { MCR14!(val, 1, c0, c12, 1) }; }
#[macro_export]
macro_rules! WCP14_ETMACVR13 { ($val:expr) => { MCR14!(val, 1, c0, c13, 1) }; }
#[macro_export]
macro_rules! WCP14_ETMACVR14 { ($val:expr) => { MCR14!(val, 1, c0, c14, 1) }; }
#[macro_export]
macro_rules! WCP14_ETMACVR15 { ($val:expr) => { MCR14!(val, 1, c0, c15, 1) }; }
#[macro_export]
macro_rules! WCP14_ETMACTR0 { ($val:expr) => { MCR14!(val, 1, c0, c0, 2) }; }
#[macro_export]
macro_rules! WCP14_ETMACTR1 { ($val:expr) => { MCR14!(val, 1, c0, c1, 2) }; }
#[macro_export]
macro_rules! WCP14_ETMACTR2 { ($val:expr) => { MCR14!(val, 1, c0, c2, 2) }; }
#[macro_export]
macro_rules! WCP14_ETMACTR3 { ($val:expr) => { MCR14!(val, 1, c0, c3, 2) }; }
#[macro_export]
macro_rules! WCP14_ETMACTR4 { ($val:expr) => { MCR14!(val, 1, c0, c4, 2) }; }
#[macro_export]
macro_rules! WCP14_ETMACTR5 { ($val:expr) => { MCR14!(val, 1, c0, c5, 2) }; }
#[macro_export]
macro_rules! WCP14_ETMACTR6 { ($val:expr) => { MCR14!(val, 1, c0, c6, 2) }; }
#[macro_export]
macro_rules! WCP14_ETMACTR7 { ($val:expr) => { MCR14!(val, 1, c0, c7, 2) }; }
#[macro_export]
macro_rules! WCP14_ETMACTR8 { ($val:expr) => { MCR14!(val, 1, c0, c8, 2) }; }
#[macro_export]
macro_rules! WCP14_ETMACTR9 { ($val:expr) => { MCR14!(val, 1, c0, c9, 2) }; }
#[macro_export]
macro_rules! WCP14_ETMACTR10 { ($val:expr) => { MCR14!(val, 1, c0, c10, 2) }; }
#[macro_export]
macro_rules! WCP14_ETMACTR11 { ($val:expr) => { MCR14!(val, 1, c0, c11, 2) }; }
#[macro_export]
macro_rules! WCP14_ETMACTR12 { ($val:expr) => { MCR14!(val, 1, c0, c12, 2) }; }
#[macro_export]
macro_rules! WCP14_ETMACTR13 { ($val:expr) => { MCR14!(val, 1, c0, c13, 2) }; }
#[macro_export]
macro_rules! WCP14_ETMACTR14 { ($val:expr) => { MCR14!(val, 1, c0, c14, 2) }; }
#[macro_export]
macro_rules! WCP14_ETMACTR15 { ($val:expr) => { MCR14!(val, 1, c0, c15, 2) }; }
#[macro_export]
macro_rules! WCP14_ETMDCVR0 { ($val:expr) => { MCR14!(val, 1, c0, c0, 3) }; }
#[macro_export]
macro_rules! WCP14_ETMDCVR2 { ($val:expr) => { MCR14!(val, 1, c0, c2, 3) }; }
#[macro_export]
macro_rules! WCP14_ETMDCVR4 { ($val:expr) => { MCR14!(val, 1, c0, c4, 3) }; }
#[macro_export]
macro_rules! WCP14_ETMDCVR6 { ($val:expr) => { MCR14!(val, 1, c0, c6, 3) }; }
#[macro_export]
macro_rules! WCP14_ETMDCVR8 { ($val:expr) => { MCR14!(val, 1, c0, c8, 3) }; }
#[macro_export]
macro_rules! WCP14_ETMDCVR10 { ($val:expr) => { MCR14!(val, 1, c0, c10, 3) }; }
#[macro_export]
macro_rules! WCP14_ETMDCVR12 { ($val:expr) => { MCR14!(val, 1, c0, c12, 3) }; }
#[macro_export]
macro_rules! WCP14_ETMDCVR14 { ($val:expr) => { MCR14!(val, 1, c0, c14, 3) }; }
#[macro_export]
macro_rules! WCP14_ETMDCMR0 { ($val:expr) => { MCR14!(val, 1, c0, c0, 4) }; }
#[macro_export]
macro_rules! WCP14_ETMDCMR2 { ($val:expr) => { MCR14!(val, 1, c0, c2, 4) }; }
#[macro_export]
macro_rules! WCP14_ETMDCMR4 { ($val:expr) => { MCR14!(val, 1, c0, c4, 4) }; }
#[macro_export]
macro_rules! WCP14_ETMDCMR6 { ($val:expr) => { MCR14!(val, 1, c0, c6, 4) }; }
#[macro_export]
macro_rules! WCP14_ETMDCMR8 { ($val:expr) => { MCR14!(val, 1, c0, c8, 4) }; }
#[macro_export]
macro_rules! WCP14_ETMDCMR10 { ($val:expr) => { MCR14!(val, 1, c0, c10, 4) }; }
#[macro_export]
macro_rules! WCP14_ETMDCMR12 { ($val:expr) => { MCR14!(val, 1, c0, c12, 4) }; }
#[macro_export]
macro_rules! WCP14_ETMDCMR14 { ($val:expr) => { MCR14!(val, 1, c0, c14, 4) }; }
#[macro_export]
macro_rules! WCP14_ETMCNTRLDVR0 { ($val:expr) => { MCR14!(val, 1, c0, c0, 5) }; }
#[macro_export]
macro_rules! WCP14_ETMCNTRLDVR1 { ($val:expr) => { MCR14!(val, 1, c0, c1, 5) }; }
#[macro_export]
macro_rules! WCP14_ETMCNTRLDVR2 { ($val:expr) => { MCR14!(val, 1, c0, c2, 5) }; }
#[macro_export]
macro_rules! WCP14_ETMCNTRLDVR3 { ($val:expr) => { MCR14!(val, 1, c0, c3, 5) }; }
#[macro_export]
macro_rules! WCP14_ETMCNTENR0 { ($val:expr) => { MCR14!(val, 1, c0, c4, 5) }; }
#[macro_export]
macro_rules! WCP14_ETMCNTENR1 { ($val:expr) => { MCR14!(val, 1, c0, c5, 5) }; }
#[macro_export]
macro_rules! WCP14_ETMCNTENR2 { ($val:expr) => { MCR14!(val, 1, c0, c6, 5) }; }
#[macro_export]
macro_rules! WCP14_ETMCNTENR3 { ($val:expr) => { MCR14!(val, 1, c0, c7, 5) }; }
#[macro_export]
macro_rules! WCP14_ETMCNTRLDEVR0 { ($val:expr) => { MCR14!(val, 1, c0, c8, 5) }; }
#[macro_export]
macro_rules! WCP14_ETMCNTRLDEVR1 { ($val:expr) => { MCR14!(val, 1, c0, c9, 5) }; }
#[macro_export]
macro_rules! WCP14_ETMCNTRLDEVR2 { ($val:expr) => { MCR14!(val, 1, c0, c10, 5) }; }
#[macro_export]
macro_rules! WCP14_ETMCNTRLDEVR3 { ($val:expr) => { MCR14!(val, 1, c0, c11, 5) }; }
#[macro_export]
macro_rules! WCP14_ETMCNTVR0 { ($val:expr) => { MCR14!(val, 1, c0, c12, 5) }; }
#[macro_export]
macro_rules! WCP14_ETMCNTVR1 { ($val:expr) => { MCR14!(val, 1, c0, c13, 5) }; }
#[macro_export]
macro_rules! WCP14_ETMCNTVR2 { ($val:expr) => { MCR14!(val, 1, c0, c14, 5) }; }
#[macro_export]
macro_rules! WCP14_ETMCNTVR3 { ($val:expr) => { MCR14!(val, 1, c0, c15, 5) }; }
#[macro_export]
macro_rules! WCP14_ETMSQ12EVR { ($val:expr) => { MCR14!(val, 1, c0, c0, 6) }; }
#[macro_export]
macro_rules! WCP14_ETMSQ21EVR { ($val:expr) => { MCR14!(val, 1, c0, c1, 6) }; }
#[macro_export]
macro_rules! WCP14_ETMSQ23EVR { ($val:expr) => { MCR14!(val, 1, c0, c2, 6) }; }
#[macro_export]
macro_rules! WCP14_ETMSQ31EVR { ($val:expr) => { MCR14!(val, 1, c0, c3, 6) }; }
#[macro_export]
macro_rules! WCP14_ETMSQ32EVR { ($val:expr) => { MCR14!(val, 1, c0, c4, 6) }; }
#[macro_export]
macro_rules! WCP14_ETMSQ13EVR { ($val:expr) => { MCR14!(val, 1, c0, c5, 6) }; }
#[macro_export]
macro_rules! WCP14_ETMSQR { ($val:expr) => { MCR14!(val, 1, c0, c7, 6) }; }
#[macro_export]
macro_rules! WCP14_ETMEXTOUTEVR0 { ($val:expr) => { MCR14!(val, 1, c0, c8, 6) }; }
#[macro_export]
macro_rules! WCP14_ETMEXTOUTEVR1 { ($val:expr) => { MCR14!(val, 1, c0, c9, 6) }; }
#[macro_export]
macro_rules! WCP14_ETMEXTOUTEVR2 { ($val:expr) => { MCR14!(val, 1, c0, c10, 6) }; }
#[macro_export]
macro_rules! WCP14_ETMEXTOUTEVR3 { ($val:expr) => { MCR14!(val, 1, c0, c11, 6) }; }
#[macro_export]
macro_rules! WCP14_ETMCIDCVR0 { ($val:expr) => { MCR14!(val, 1, c0, c12, 6) }; }
#[macro_export]
macro_rules! WCP14_ETMCIDCVR1 { ($val:expr) => { MCR14!(val, 1, c0, c13, 6) }; }
#[macro_export]
macro_rules! WCP14_ETMCIDCVR2 { ($val:expr) => { MCR14!(val, 1, c0, c14, 6) }; }
#[macro_export]
macro_rules! WCP14_ETMCIDCMR { ($val:expr) => { MCR14!(val, 1, c0, c15, 6) }; }
#[macro_export]
macro_rules! WCP14_ETMIMPSPEC0 { ($val:expr) => { MCR14!(val, 1, c0, c0, 7) }; }
#[macro_export]
macro_rules! WCP14_ETMIMPSPEC1 { ($val:expr) => { MCR14!(val, 1, c0, c1, 7) }; }
#[macro_export]
macro_rules! WCP14_ETMIMPSPEC2 { ($val:expr) => { MCR14!(val, 1, c0, c2, 7) }; }
#[macro_export]
macro_rules! WCP14_ETMIMPSPEC3 { ($val:expr) => { MCR14!(val, 1, c0, c3, 7) }; }
#[macro_export]
macro_rules! WCP14_ETMIMPSPEC4 { ($val:expr) => { MCR14!(val, 1, c0, c4, 7) }; }
#[macro_export]
macro_rules! WCP14_ETMIMPSPEC5 { ($val:expr) => { MCR14!(val, 1, c0, c5, 7) }; }
#[macro_export]
macro_rules! WCP14_ETMIMPSPEC6 { ($val:expr) => { MCR14!(val, 1, c0, c6, 7) }; }
#[macro_export]
macro_rules! WCP14_ETMIMPSPEC7 { ($val:expr) => { MCR14!(val, 1, c0, c7, 7) }; }
/* Can be read only in ETMv3.4, ETMv3.5 */
#[macro_export]
macro_rules! WCP14_ETMSYNCFR { ($val:expr) => { MCR14!(val, 1, c0, c8, 7) }; }
#[macro_export]
macro_rules! WCP14_ETMEXTINSELR { ($val:expr) => { MCR14!(val, 1, c0, c11, 7) }; }
#[macro_export]
macro_rules! WCP14_ETMTESSEICR { ($val:expr) => { MCR14!(val, 1, c0, c12, 7) }; }
#[macro_export]
macro_rules! WCP14_ETMEIBCR { ($val:expr) => { MCR14!(val, 1, c0, c13, 7) }; }
#[macro_export]
macro_rules! WCP14_ETMTSEVR { ($val:expr) => { MCR14!(val, 1, c0, c14, 7) }; }
#[macro_export]
macro_rules! WCP14_ETMAUXCR { ($val:expr) => { MCR14!(val, 1, c0, c15, 7) }; }
#[macro_export]
macro_rules! WCP14_ETMTRACEIDR { ($val:expr) => { MCR14!(val, 1, c1, c0, 0) }; }
#[macro_export]
macro_rules! WCP14_ETMIDR2 { ($val:expr) => { MCR14!(val, 1, c1, c2, 0) }; }
#[macro_export]
macro_rules! WCP14_ETMVMIDCVR { ($val:expr) => { MCR14!(val, 1, c1, c0, 1) }; }
#[macro_export]
macro_rules! WCP14_ETMOSLAR { ($val:expr) => { MCR14!(val, 1, c1, c0, 4) }; }
/* Not available in PFTv1.1 */
#[macro_export]
macro_rules! WCP14_ETMOSSRR { ($val:expr) => { MCR14!(val, 1, c1, c2, 4) }; }
#[macro_export]
macro_rules! WCP14_ETMPDCR { ($val:expr) => { MCR14!(val, 1, c1, c4, 4) }; }
#[macro_export]
macro_rules! WCP14_ETMPDSR { ($val:expr) => { MCR14!(val, 1, c1, c5, 4) }; }
#[macro_export]
macro_rules! WCP14_ETMITCTRL { ($val:expr) => { MCR14!(val, 1, c7, c0, 4) }; }
#[macro_export]
macro_rules! WCP14_ETMCLAIMSET { ($val:expr) => { MCR14!(val, 1, c7, c8, 6) }; }
#[macro_export]
macro_rules! WCP14_ETMCLAIMCLR { ($val:expr) => { MCR14!(val, 1, c7, c9, 6) }; }
/* Writes to this from CP14 interface are ignored */
#[macro_export]
macro_rules! WCP14_ETMLAR { ($val:expr) => { MCR14!(val, 1, c7, c12, 6) }; }


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
