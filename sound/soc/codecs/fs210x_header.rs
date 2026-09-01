// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * fs210x.h -- Driver for the FS2104/5S Audio Amplifier
 *
 * Copyright (C) 2016-2025 Shanghai FourSemi Semiconductor Co.,Ltd.
 */

pub const fn BIT(nr: u32) -> u32 {
    1u32 << nr
}

pub const fn GENMASK(h: u32, l: u32) -> u32 {
    u32::MAX
        .wrapping_shl(l)
        & u32::MAX.wrapping_shr(31u32.wrapping_sub(h))
}

pub const FS210X_00H_STATUS: u32 = 0x00;
pub const FS210X_03H_DEVID: u32 = 0x03;
pub const FS210X_05H_ANASTAT: u32 = 0x05;
pub const FS210X_06H_DIGSTAT: u32 = 0x06;
pub const FS210X_0BH_ACCKEY: u32 = 0x0B;
pub const FS210X_0FH_I2CADDR: u32 = 0x0F;
pub const FS210X_10H_PWRCTRL: u32 = 0x10;
pub const FS210X_11H_SYSCTRL: u32 = 0x11;
pub const FS210X_17H_I2SCTRL: u32 = 0x17;
pub const FS210X_30H_DACCTRL: u32 = 0x30;
pub const FS210X_39H_LVOLCTRL: u32 = 0x39;
pub const FS210X_3AH_RVOLCTRL: u32 = 0x3A;
pub const FS210X_42H_DACEQWL: u32 = 0x42;
pub const FS210X_46H_DACEQA: u32 = 0x46;
pub const FS210X_A1H_PLLCTRL1: u32 = 0xA1;
pub const FS210X_A2H_PLLCTRL2: u32 = 0xA2;
pub const FS210X_A3H_PLLCTRL3: u32 = 0xA3;
pub const FS210X_ABH_INTSTAT: u32 = 0xAB;
pub const FS210X_ACH_INTSTATR: u32 = 0xAC;

pub const FS210X_05H_PVDD_SHIFT: u32 = 14;
pub const FS210X_05H_PVDD_MASK: u32 = BIT(14);
pub const FS210X_05H_OCDL_SHIFT: u32 = 13;
pub const FS210X_05H_OCDL_MASK: u32 = BIT(13);
pub const FS210X_05H_UVDL_SHIFT: u32 = 12;
pub const FS210X_05H_UVDL_MASK: u32 = BIT(12);
pub const FS210X_05H_OVDL_SHIFT: u32 = 11;
pub const FS210X_05H_OVDL_MASK: u32 = BIT(11);
pub const FS210X_05H_OTPDL_SHIFT: u32 = 10;
pub const FS210X_05H_OTPDL_MASK: u32 = BIT(10);
pub const FS210X_05H_OCRDL_SHIFT: u32 = 9;
pub const FS210X_05H_OCRDL_MASK: u32 = BIT(9);
pub const FS210X_05H_OCLDL_SHIFT: u32 = 8;
pub const FS210X_05H_OCLDL_MASK: u32 = BIT(8);
pub const FS210X_05H_DCRDL_SHIFT: u32 = 7;
pub const FS210X_05H_DCRDL_MASK: u32 = BIT(7);
pub const FS210X_05H_DCLDL_SHIFT: u32 = 6;
pub const FS210X_05H_DCLDL_MASK: u32 = BIT(6);
pub const FS210X_05H_SRDL_SHIFT: u32 = 5;
pub const FS210X_05H_SRDL_MASK: u32 = BIT(5);
pub const FS210X_05H_OTWDL_SHIFT: u32 = 4;
pub const FS210X_05H_OTWDL_MASK: u32 = BIT(4);
pub const FS210X_05H_AMPS_SHIFT: u32 = 3;
pub const FS210X_05H_AMPS_MASK: u32 = BIT(3);
pub const FS210X_05H_PLLS_SHIFT: u32 = 1;
pub const FS210X_05H_PLLS_MASK: u32 = BIT(1);
pub const FS210X_05H_ANAS_SHIFT: u32 = 0;
pub const FS210X_05H_ANAS_MASK: u32 = BIT(0);
pub const FS210X_17H_I2SSR_SHIFT: u32 = 12;
pub const FS210X_17H_I2SSR_MASK: u32 = GENMASK(15, 12);
pub const FS210X_30H_RMUTE_SHIFT: u32 = 8;
pub const FS210X_30H_LMUTE_SHIFT: u32 = 4;

pub const FS210X_0BH_ACCKEY_ON: u32 = 0x0091;
pub const FS210X_0BH_ACCKEY_OFF: u32 = 0x0000;
pub const FS210X_10H_I2C_RESET: u32 = 0x0002;
pub const FS210X_11H_DPS_HIZ: u32 = 0x0100;
pub const FS210X_11H_DPS_PWDN: u32 = 0x0000;
pub const FS210X_11H_DPS_PLAY: u32 = 0x0300;
pub const FS210X_46H_CAM_BURST_L: u32 = 0x8000;
pub const FS210X_46H_CAM_BURST_R: u32 = 0x8200;
pub const FS2105S_46H_CAM_BURST_W: u32 = 0x8400;
pub const FS210X_46H_CAM_CLEAR: u32 = 0x0000;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
