/*
 * This header file describes this specific Xtensa processor's TIE extensions
 * that extend basic Xtensa core functionality.  It is customized to this
 * Xtensa processor configuration.
 *
 * This file is subject to the terms and conditions of version 2.1 of the GNU
 * Lesser General Public License as published by the Free Software Foundation.
 *
 * Copyright (C) 1999-2009 Tensilica Inc.
 */

pub const XCHAL_CP_NUM: u32 = 1;
pub const XCHAL_CP_MAX: u32 = 2;
pub const XCHAL_CP_MASK: u32 = 0x02;
pub const XCHAL_CP_PORT_MASK: u32 = 0x00;

pub const XCHAL_CP1_NAME: &str = "AudioEngineLX";
// C identifier macro value: AudioEngineLX.
pub const XCHAL_CP1_SA_SIZE: u32 = 112;
pub const XCHAL_CP1_SA_ALIGN: u32 = 8;
pub const XCHAL_CP_ID_AUDIOENGINELX: u32 = 1;

pub const XCHAL_CP0_SA_SIZE: u32 = 0;
pub const XCHAL_CP0_SA_ALIGN: u32 = 1;
pub const XCHAL_CP2_SA_SIZE: u32 = 0;
pub const XCHAL_CP2_SA_ALIGN: u32 = 1;
pub const XCHAL_CP3_SA_SIZE: u32 = 0;
pub const XCHAL_CP3_SA_ALIGN: u32 = 1;
pub const XCHAL_CP4_SA_SIZE: u32 = 0;
pub const XCHAL_CP4_SA_ALIGN: u32 = 1;
pub const XCHAL_CP5_SA_SIZE: u32 = 0;
pub const XCHAL_CP5_SA_ALIGN: u32 = 1;
pub const XCHAL_CP6_SA_SIZE: u32 = 0;
pub const XCHAL_CP6_SA_ALIGN: u32 = 1;
pub const XCHAL_CP7_SA_SIZE: u32 = 0;
pub const XCHAL_CP7_SA_ALIGN: u32 = 1;

pub const XCHAL_NCP_SA_SIZE: u32 = 12;
pub const XCHAL_NCP_SA_ALIGN: u32 = 4;
pub const XCHAL_TOTAL_SA_SIZE: u32 = 128;
pub const XCHAL_TOTAL_SA_ALIGN: u32 = 8;

// XCHAL_SA_REG! is supplied by the including translation unit.
#[macro_export]
macro_rules! XCHAL_NCP_SA_LIST {
    ($s:tt) => {
        XCHAL_SA_REG!($s, 0, 0, 0, 1, br, 4, 4, 4, 0x0204, sr, 4, 16, 0, 0, 0);
        XCHAL_SA_REG!($s, 0, 0, 0, 1, scompare1, 4, 4, 4, 0x020C, sr, 12, 32, 0, 0, 0);
        XCHAL_SA_REG!($s, 1, 2, 1, 1, threadptr, 4, 4, 4, 0x03E7, ur, 231, 32, 0, 0, 0);
    };
}

pub const XCHAL_NCP_SA_NUM: u32 = 3;
pub const XCHAL_CP0_SA_NUM: u32 = 0;
pub const XCHAL_CP1_SA_NUM: u32 = 16;
pub const XCHAL_CP2_SA_NUM: u32 = 0;
pub const XCHAL_CP3_SA_NUM: u32 = 0;
pub const XCHAL_CP4_SA_NUM: u32 = 0;
pub const XCHAL_CP5_SA_NUM: u32 = 0;
pub const XCHAL_CP6_SA_NUM: u32 = 0;
pub const XCHAL_CP7_SA_NUM: u32 = 0;

#[macro_export]
macro_rules! XCHAL_CP0_SA_LIST { ($s:tt) => {}; }
#[macro_export]
macro_rules! XCHAL_CP1_SA_LIST {
    ($s:tt) => {
        XCHAL_SA_REG!($s, 0, 0, 1, 0, ae_ovf_sar, 8, 4, 4, 0x03F0, ur, 240, 7, 0, 0, 0);
        XCHAL_SA_REG!($s, 0, 0, 1, 0, ae_bithead, 4, 4, 4, 0x03F1, ur, 241, 32, 0, 0, 0);
        XCHAL_SA_REG!($s, 0, 0, 1, 0, ae_ts_fts_bu_bp, 4, 4, 4, 0x03F2, ur, 242, 16, 0, 0, 0);
        XCHAL_SA_REG!($s, 0, 0, 1, 0, ae_sd_no, 4, 4, 4, 0x03F3, ur, 243, 28, 0, 0, 0);
        XCHAL_SA_REG!($s, 0, 0, 2, 0, aep0, 8, 8, 8, 0x0060, aep, 0, 48, 0, 0, 0);
        XCHAL_SA_REG!($s, 0, 0, 2, 0, aep1, 8, 8, 8, 0x0061, aep, 1, 48, 0, 0, 0);
        XCHAL_SA_REG!($s, 0, 0, 2, 0, aep2, 8, 8, 8, 0x0062, aep, 2, 48, 0, 0, 0);
        XCHAL_SA_REG!($s, 0, 0, 2, 0, aep3, 8, 8, 8, 0x0063, aep, 3, 48, 0, 0, 0);
        XCHAL_SA_REG!($s, 0, 0, 2, 0, aep4, 8, 8, 8, 0x0064, aep, 4, 48, 0, 0, 0);
        XCHAL_SA_REG!($s, 0, 0, 2, 0, aep5, 8, 8, 8, 0x0065, aep, 5, 48, 0, 0, 0);
        XCHAL_SA_REG!($s, 0, 0, 2, 0, aep6, 8, 8, 8, 0x0066, aep, 6, 48, 0, 0, 0);
        XCHAL_SA_REG!($s, 0, 0, 2, 0, aep7, 8, 8, 8, 0x0067, aep, 7, 48, 0, 0, 0);
        XCHAL_SA_REG!($s, 0, 0, 2, 0, aeq0, 8, 8, 8, 0x0068, aeq, 0, 56, 0, 0, 0);
        XCHAL_SA_REG!($s, 0, 0, 2, 0, aeq1, 8, 8, 8, 0x0069, aeq, 1, 56, 0, 0, 0);
        XCHAL_SA_REG!($s, 0, 0, 2, 0, aeq2, 8, 8, 8, 0x006A, aeq, 2, 56, 0, 0, 0);
        XCHAL_SA_REG!($s, 0, 0, 2, 0, aeq3, 8, 8, 8, 0x006B, aeq, 3, 56, 0, 0, 0);
    };
}
#[macro_export] macro_rules! XCHAL_CP2_SA_LIST { ($s:tt) => {}; }
#[macro_export] macro_rules! XCHAL_CP3_SA_LIST { ($s:tt) => {}; }
#[macro_export] macro_rules! XCHAL_CP4_SA_LIST { ($s:tt) => {}; }
#[macro_export] macro_rules! XCHAL_CP5_SA_LIST { ($s:tt) => {}; }
#[macro_export] macro_rules! XCHAL_CP6_SA_LIST { ($s:tt) => {}; }
#[macro_export] macro_rules! XCHAL_CP7_SA_LIST { ($s:tt) => {}; }

pub const XCHAL_OP0_FORMAT_LENGTHS: [u32; 16] = [3, 3, 3, 3, 3, 3, 3, 3, 2, 2, 2, 2, 2, 2, 3, 8];

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
