/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * tuner.h - definition for different tuners
 *
 * Copyright (C) 1997 Markus Schroeder (schroedm@uni-duesseldorf.de)
 * minor modifications by Ralph Metzler (rjkm@thp.uni-koeln.de)
 */

/* C header guard: _TUNER_H */
/* C conditional: __KERNEL__ */
/* Dependencies: linux/videodev2.h, media/v4l2-mc.h */

pub const ADDR_UNSET: u32 = 255;

pub const TUNER_TEMIC_PAL: u32 = 0;
pub const TUNER_PHILIPS_PAL_I: u32 = 1;
pub const TUNER_PHILIPS_NTSC: u32 = 2;
pub const TUNER_PHILIPS_SECAM: u32 = 3;
pub const TUNER_ABSENT: u32 = 4;
pub const TUNER_PHILIPS_PAL: u32 = 5;
pub const TUNER_TEMIC_NTSC: u32 = 6;
pub const TUNER_TEMIC_PAL_I: u32 = 7;
pub const TUNER_TEMIC_4036FY5_NTSC: u32 = 8;
pub const TUNER_ALPS_TSBH1_NTSC: u32 = 9;
pub const TUNER_ALPS_TSBE1_PAL: u32 = 10;
pub const TUNER_ALPS_TSBB5_PAL_I: u32 = 11;
pub const TUNER_ALPS_TSBE5_PAL: u32 = 12;
pub const TUNER_ALPS_TSBC5_PAL: u32 = 13;
pub const TUNER_TEMIC_4006FH5_PAL: u32 = 14;
pub const TUNER_ALPS_TSHC6_NTSC: u32 = 15;
pub const TUNER_TEMIC_PAL_DK: u32 = 16;
pub const TUNER_PHILIPS_NTSC_M: u32 = 17;
pub const TUNER_TEMIC_4066FY5_PAL_I: u32 = 18;
pub const TUNER_TEMIC_4006FN5_MULTI_PAL: u32 = 19;
pub const TUNER_TEMIC_4009FR5_PAL: u32 = 20;
pub const TUNER_TEMIC_4039FR5_NTSC: u32 = 21;
pub const TUNER_TEMIC_4046FM5: u32 = 22;
pub const TUNER_PHILIPS_PAL_DK: u32 = 23;
pub const TUNER_PHILIPS_FQ1216ME: u32 = 24;
pub const TUNER_LG_PAL_I_FM: u32 = 25;
pub const TUNER_LG_PAL_I: u32 = 26;
pub const TUNER_LG_NTSC_FM: u32 = 27;
pub const TUNER_LG_PAL_FM: u32 = 28;
pub const TUNER_LG_PAL: u32 = 29;
pub const TUNER_TEMIC_4009FN5_MULTI_PAL_FM: u32 = 30;
pub const TUNER_SHARP_2U5JF5540_NTSC: u32 = 31;
pub const TUNER_Samsung_PAL_TCPM9091PD27: u32 = 32;
pub const TUNER_MT2032: u32 = 33;
pub const TUNER_TEMIC_4106FH5: u32 = 34;
pub const TUNER_TEMIC_4012FY5: u32 = 35;
pub const TUNER_TEMIC_4136FY5: u32 = 36;
pub const TUNER_LG_PAL_NEW_TAPC: u32 = 37;
pub const TUNER_PHILIPS_FM1216ME_MK3: u32 = 38;
pub const TUNER_LG_NTSC_NEW_TAPC: u32 = 39;
pub const TUNER_HITACHI_NTSC: u32 = 40;
pub const TUNER_PHILIPS_PAL_MK: u32 = 41;
pub const TUNER_PHILIPS_FCV1236D: u32 = 42;
pub const TUNER_PHILIPS_FM1236_MK3: u32 = 43;
pub const TUNER_PHILIPS_4IN1: u32 = 44;
pub const TUNER_MICROTUNE_4049FM5: u32 = 45;
pub const TUNER_PANASONIC_VP27: u32 = 46;
pub const TUNER_LG_NTSC_TAPE: u32 = 47;
pub const TUNER_TNF_8831BGFF: u32 = 48;
pub const TUNER_MICROTUNE_4042FI5: u32 = 49;
pub const TUNER_TCL_2002N: u32 = 50;
pub const TUNER_PHILIPS_FM1256_IH3: u32 = 51;
pub const TUNER_THOMSON_DTT7610: u32 = 52;
pub const TUNER_PHILIPS_FQ1286: u32 = 53;
pub const TUNER_PHILIPS_TDA8290: u32 = 54;
pub const TUNER_TCL_2002MB: u32 = 55;
pub const TUNER_PHILIPS_FQ1216AME_MK4: u32 = 56;
pub const TUNER_PHILIPS_FQ1236A_MK4: u32 = 57;
pub const TUNER_YMEC_TVF_8531MF: u32 = 58;
pub const TUNER_YMEC_TVF_5533MF: u32 = 59;
pub const TUNER_THOMSON_DTT761X: u32 = 60;
pub const TUNER_TENA_9533_DI: u32 = 61;
pub const TUNER_TEA5767: u32 = 62;
pub const TUNER_PHILIPS_FMD1216ME_MK3: u32 = 63;
pub const TUNER_LG_TDVS_H06XF: u32 = 64;
pub const TUNER_YMEC_TVF66T5_B_DFF: u32 = 65;
pub const TUNER_LG_TALN: u32 = 66;
pub const TUNER_PHILIPS_TD1316: u32 = 67;
pub const TUNER_PHILIPS_TUV1236D: u32 = 68;
pub const TUNER_TNF_5335MF: u32 = 69;
pub const TUNER_SAMSUNG_TCPN_2121P30A: u32 = 70;
pub const TUNER_XC2028: u32 = 71;
pub const TUNER_THOMSON_FE6600: u32 = 72;
pub const TUNER_SAMSUNG_TCPG_6121P30A: u32 = 73;
pub const TUNER_TDA9887: u32 = 74;
pub const TUNER_TEA5761: u32 = 75;
pub const TUNER_XC5000: u32 = 76;
pub const TUNER_TCL_MF02GIP_5N: u32 = 77;
pub const TUNER_PHILIPS_FMD1216MEX_MK3: u32 = 78;
pub const TUNER_PHILIPS_FM1216MK5: u32 = 79;
pub const TUNER_PHILIPS_FQ1216LME_MK3: u32 = 80;
pub const TUNER_PARTSNIC_PTI_5NF05: u32 = 81;
pub const TUNER_PHILIPS_CU1216L: u32 = 82;
pub const TUNER_NXP_TDA18271: u32 = 83;
pub const TUNER_SONY_BTF_PXN01Z: u32 = 84;
pub const TUNER_PHILIPS_FQ1236_MK5: u32 = 85;
pub const TUNER_TENA_TNF_5337: u32 = 86;
pub const TUNER_XC4000: u32 = 87;
pub const TUNER_XC5000C: u32 = 88;
pub const TUNER_SONY_BTF_PG472Z: u32 = 89;
pub const TUNER_SONY_BTF_PK467Z: u32 = 90;
pub const TUNER_SONY_BTF_PB463Z: u32 = 91;
pub const TUNER_SI2157: u32 = 92;
pub const TUNER_TENA_TNF_931D_DFDR1: u32 = 93;

pub const TDA9887_PRESENT: u32 = 1 << 0;
pub const TDA9887_PORT1_INACTIVE: u32 = 1 << 1;
pub const TDA9887_PORT2_INACTIVE: u32 = 1 << 2;
pub const TDA9887_QSS: u32 = 1 << 3;
pub const TDA9887_INTERCARRIER: u32 = 1 << 4;
pub const TDA9887_PORT1_ACTIVE: u32 = 1 << 5;
pub const TDA9887_PORT2_ACTIVE: u32 = 1 << 6;
pub const TDA9887_INTERCARRIER_NTSC: u32 = 1 << 7;
pub const TDA9887_TOP_MASK: u32 = 0x3f << 8;
pub const TDA9887_TOP_SET: u32 = 1 << 13;

#[inline]
pub const fn TDA9887_TOP(top: i32) -> u32 {
    TDA9887_TOP_SET | ((((16 + top) & 0x1f) as u32) << 8)
}

pub const TDA9887_DEEMPHASIS_MASK: u32 = 3 << 16;
pub const TDA9887_DEEMPHASIS_NONE: u32 = 1 << 16;
pub const TDA9887_DEEMPHASIS_50: u32 = 2 << 16;
pub const TDA9887_DEEMPHASIS_75: u32 = 3 << 16;
pub const TDA9887_AUTOMUTE: u32 = 1 << 18;
pub const TDA9887_GATING_18: u32 = 1 << 19;
pub const TDA9887_GAIN_NORMAL: u32 = 1 << 20;
pub const TDA9887_RIF_41_3: u32 = 1 << 21;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum tuner_mode {
    T_RADIO = 1 << V4L2_TUNER_RADIO,
    T_ANALOG_TV = 1 << V4L2_TUNER_ANALOG_TV,
}

#[repr(C)]
pub struct tuner_setup {
    pub addr: u16,
    pub type_: u32,
    pub mode_mask: u32,
    pub config: *mut core::ffi::c_void,
    pub tuner_callback: Option<unsafe extern "C" fn(
        dev: *mut core::ffi::c_void,
        component: i32,
        cmd: i32,
        arg: i32,
    ) -> i32>,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
