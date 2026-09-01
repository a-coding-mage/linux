/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * ALSA SoC WM9090 driver
 *
 * Copyright 2009 Wolfson Microelectronics
 *
 * Author: Mark Brown <broonie@opensource.wolfsonmicro.com>
 */


/*
 * Register values.
 */
pub const WM9090_SOFTWARE_RESET: u16 = 0x00;
pub const WM9090_POWER_MANAGEMENT_1: u16 = 0x01;
pub const WM9090_POWER_MANAGEMENT_2: u16 = 0x02;
pub const WM9090_POWER_MANAGEMENT_3: u16 = 0x03;
pub const WM9090_CLOCKING_1: u16 = 0x06;
pub const WM9090_IN1_LINE_CONTROL: u16 = 0x16;
pub const WM9090_IN2_LINE_CONTROL: u16 = 0x17;
pub const WM9090_IN1_LINE_INPUT_A_VOLUME: u16 = 0x18;
pub const WM9090_IN1_LINE_INPUT_B_VOLUME: u16 = 0x19;
pub const WM9090_IN2_LINE_INPUT_A_VOLUME: u16 = 0x1A;
pub const WM9090_IN2_LINE_INPUT_B_VOLUME: u16 = 0x1B;
pub const WM9090_LEFT_OUTPUT_VOLUME: u16 = 0x1C;
pub const WM9090_RIGHT_OUTPUT_VOLUME: u16 = 0x1D;
pub const WM9090_SPKMIXL_ATTENUATION: u16 = 0x22;
pub const WM9090_SPKOUT_MIXERS: u16 = 0x24;
pub const WM9090_CLASSD3: u16 = 0x25;
pub const WM9090_SPEAKER_VOLUME_LEFT: u16 = 0x26;
pub const WM9090_OUTPUT_MIXER1: u16 = 0x2D;
pub const WM9090_OUTPUT_MIXER2: u16 = 0x2E;
pub const WM9090_OUTPUT_MIXER3: u16 = 0x2F;
pub const WM9090_OUTPUT_MIXER4: u16 = 0x30;
pub const WM9090_SPEAKER_MIXER: u16 = 0x36;
pub const WM9090_ANTIPOP2: u16 = 0x39;
pub const WM9090_WRITE_SEQUENCER_0: u16 = 0x46;
pub const WM9090_WRITE_SEQUENCER_1: u16 = 0x47;
pub const WM9090_WRITE_SEQUENCER_2: u16 = 0x48;
pub const WM9090_WRITE_SEQUENCER_3: u16 = 0x49;
pub const WM9090_WRITE_SEQUENCER_4: u16 = 0x4A;
pub const WM9090_WRITE_SEQUENCER_5: u16 = 0x4B;
pub const WM9090_CHARGE_PUMP_1: u16 = 0x4C;
pub const WM9090_DC_SERVO_0: u16 = 0x54;
pub const WM9090_DC_SERVO_1: u16 = 0x55;
pub const WM9090_DC_SERVO_3: u16 = 0x57;
pub const WM9090_DC_SERVO_READBACK_0: u16 = 0x58;
pub const WM9090_DC_SERVO_READBACK_1: u16 = 0x59;
pub const WM9090_DC_SERVO_READBACK_2: u16 = 0x5A;
pub const WM9090_ANALOGUE_HP_0: u16 = 0x60;
pub const WM9090_AGC_CONTROL_0: u16 = 0x62;
pub const WM9090_AGC_CONTROL_1: u16 = 0x63;
pub const WM9090_AGC_CONTROL_2: u16 = 0x64;

pub const WM9090_REGISTER_COUNT: u16 = 40;
pub const WM9090_MAX_REGISTER: u16 = 0x64;

/*
 * Field Definitions.
 */

/*
 * R0 (0x00) - Software Reset
 */
pub const WM9090_SW_RESET_MASK: u16 = 0xFFFF;  /* SW_RESET - [15:0] */
pub const WM9090_SW_RESET_SHIFT: u16 = 0;  /* SW_RESET - [15:0] */
pub const WM9090_SW_RESET_WIDTH: u16 = 16;  /* SW_RESET - [15:0] */

/*
 * R1 (0x01) - Power Management (1)
 */
pub const WM9090_SPKOUTL_ENA: u16 = 0x1000;  /* SPKOUTL_ENA */
pub const WM9090_SPKOUTL_ENA_MASK: u16 = 0x1000;  /* SPKOUTL_ENA */
pub const WM9090_SPKOUTL_ENA_SHIFT: u16 = 12;  /* SPKOUTL_ENA */
pub const WM9090_SPKOUTL_ENA_WIDTH: u16 = 1;  /* SPKOUTL_ENA */
pub const WM9090_HPOUT1L_ENA: u16 = 0x0200;  /* HPOUT1L_ENA */
pub const WM9090_HPOUT1L_ENA_MASK: u16 = 0x0200;  /* HPOUT1L_ENA */
pub const WM9090_HPOUT1L_ENA_SHIFT: u16 = 9;  /* HPOUT1L_ENA */
pub const WM9090_HPOUT1L_ENA_WIDTH: u16 = 1;  /* HPOUT1L_ENA */
pub const WM9090_HPOUT1R_ENA: u16 = 0x0100;  /* HPOUT1R_ENA */
pub const WM9090_HPOUT1R_ENA_MASK: u16 = 0x0100;  /* HPOUT1R_ENA */
pub const WM9090_HPOUT1R_ENA_SHIFT: u16 = 8;  /* HPOUT1R_ENA */
pub const WM9090_HPOUT1R_ENA_WIDTH: u16 = 1;  /* HPOUT1R_ENA */
pub const WM9090_OSC_ENA: u16 = 0x0008;  /* OSC_ENA */
pub const WM9090_OSC_ENA_MASK: u16 = 0x0008;  /* OSC_ENA */
pub const WM9090_OSC_ENA_SHIFT: u16 = 3;  /* OSC_ENA */
pub const WM9090_OSC_ENA_WIDTH: u16 = 1;  /* OSC_ENA */
pub const WM9090_VMID_RES_MASK: u16 = 0x0006;  /* VMID_RES - [2:1] */
pub const WM9090_VMID_RES_SHIFT: u16 = 1;  /* VMID_RES - [2:1] */
pub const WM9090_VMID_RES_WIDTH: u16 = 2;  /* VMID_RES - [2:1] */
pub const WM9090_BIAS_ENA: u16 = 0x0001;  /* BIAS_ENA */
pub const WM9090_BIAS_ENA_MASK: u16 = 0x0001;  /* BIAS_ENA */
pub const WM9090_BIAS_ENA_SHIFT: u16 = 0;  /* BIAS_ENA */
pub const WM9090_BIAS_ENA_WIDTH: u16 = 1;  /* BIAS_ENA */

/*
 * R2 (0x02) - Power Management (2)
 */
pub const WM9090_TSHUT: u16 = 0x8000;  /* TSHUT */
pub const WM9090_TSHUT_MASK: u16 = 0x8000;  /* TSHUT */
pub const WM9090_TSHUT_SHIFT: u16 = 15;  /* TSHUT */
pub const WM9090_TSHUT_WIDTH: u16 = 1;  /* TSHUT */
pub const WM9090_TSHUT_ENA: u16 = 0x4000;  /* TSHUT_ENA */
pub const WM9090_TSHUT_ENA_MASK: u16 = 0x4000;  /* TSHUT_ENA */
pub const WM9090_TSHUT_ENA_SHIFT: u16 = 14;  /* TSHUT_ENA */
pub const WM9090_TSHUT_ENA_WIDTH: u16 = 1;  /* TSHUT_ENA */
pub const WM9090_TSHUT_OPDIS: u16 = 0x2000;  /* TSHUT_OPDIS */
pub const WM9090_TSHUT_OPDIS_MASK: u16 = 0x2000;  /* TSHUT_OPDIS */
pub const WM9090_TSHUT_OPDIS_SHIFT: u16 = 13;  /* TSHUT_OPDIS */
pub const WM9090_TSHUT_OPDIS_WIDTH: u16 = 1;  /* TSHUT_OPDIS */
pub const WM9090_IN1A_ENA: u16 = 0x0080;  /* IN1A_ENA */
pub const WM9090_IN1A_ENA_MASK: u16 = 0x0080;  /* IN1A_ENA */
pub const WM9090_IN1A_ENA_SHIFT: u16 = 7;  /* IN1A_ENA */
pub const WM9090_IN1A_ENA_WIDTH: u16 = 1;  /* IN1A_ENA */
pub const WM9090_IN1B_ENA: u16 = 0x0040;  /* IN1B_ENA */
pub const WM9090_IN1B_ENA_MASK: u16 = 0x0040;  /* IN1B_ENA */
pub const WM9090_IN1B_ENA_SHIFT: u16 = 6;  /* IN1B_ENA */
pub const WM9090_IN1B_ENA_WIDTH: u16 = 1;  /* IN1B_ENA */
pub const WM9090_IN2A_ENA: u16 = 0x0020;  /* IN2A_ENA */
pub const WM9090_IN2A_ENA_MASK: u16 = 0x0020;  /* IN2A_ENA */
pub const WM9090_IN2A_ENA_SHIFT: u16 = 5;  /* IN2A_ENA */
pub const WM9090_IN2A_ENA_WIDTH: u16 = 1;  /* IN2A_ENA */
pub const WM9090_IN2B_ENA: u16 = 0x0010;  /* IN2B_ENA */
pub const WM9090_IN2B_ENA_MASK: u16 = 0x0010;  /* IN2B_ENA */
pub const WM9090_IN2B_ENA_SHIFT: u16 = 4;  /* IN2B_ENA */
pub const WM9090_IN2B_ENA_WIDTH: u16 = 1;  /* IN2B_ENA */

/*
 * R3 (0x03) - Power Management (3)
 */
pub const WM9090_AGC_ENA: u16 = 0x4000;  /* AGC_ENA */
pub const WM9090_AGC_ENA_MASK: u16 = 0x4000;  /* AGC_ENA */
pub const WM9090_AGC_ENA_SHIFT: u16 = 14;  /* AGC_ENA */
pub const WM9090_AGC_ENA_WIDTH: u16 = 1;  /* AGC_ENA */
pub const WM9090_SPKLVOL_ENA: u16 = 0x0100;  /* SPKLVOL_ENA */
pub const WM9090_SPKLVOL_ENA_MASK: u16 = 0x0100;  /* SPKLVOL_ENA */
pub const WM9090_SPKLVOL_ENA_SHIFT: u16 = 8;  /* SPKLVOL_ENA */
pub const WM9090_SPKLVOL_ENA_WIDTH: u16 = 1;  /* SPKLVOL_ENA */
pub const WM9090_MIXOUTL_ENA: u16 = 0x0020;  /* MIXOUTL_ENA */
pub const WM9090_MIXOUTL_ENA_MASK: u16 = 0x0020;  /* MIXOUTL_ENA */
pub const WM9090_MIXOUTL_ENA_SHIFT: u16 = 5;  /* MIXOUTL_ENA */
pub const WM9090_MIXOUTL_ENA_WIDTH: u16 = 1;  /* MIXOUTL_ENA */
pub const WM9090_MIXOUTR_ENA: u16 = 0x0010;  /* MIXOUTR_ENA */
pub const WM9090_MIXOUTR_ENA_MASK: u16 = 0x0010;  /* MIXOUTR_ENA */
pub const WM9090_MIXOUTR_ENA_SHIFT: u16 = 4;  /* MIXOUTR_ENA */
pub const WM9090_MIXOUTR_ENA_WIDTH: u16 = 1;  /* MIXOUTR_ENA */
pub const WM9090_SPKMIX_ENA: u16 = 0x0008;  /* SPKMIX_ENA */
pub const WM9090_SPKMIX_ENA_MASK: u16 = 0x0008;  /* SPKMIX_ENA */
pub const WM9090_SPKMIX_ENA_SHIFT: u16 = 3;  /* SPKMIX_ENA */
pub const WM9090_SPKMIX_ENA_WIDTH: u16 = 1;  /* SPKMIX_ENA */

/*
 * R6 (0x06) - Clocking 1
 */
pub const WM9090_TOCLK_RATE: u16 = 0x8000;  /* TOCLK_RATE */
pub const WM9090_TOCLK_RATE_MASK: u16 = 0x8000;  /* TOCLK_RATE */
pub const WM9090_TOCLK_RATE_SHIFT: u16 = 15;  /* TOCLK_RATE */
pub const WM9090_TOCLK_RATE_WIDTH: u16 = 1;  /* TOCLK_RATE */
pub const WM9090_TOCLK_ENA: u16 = 0x4000;  /* TOCLK_ENA */
pub const WM9090_TOCLK_ENA_MASK: u16 = 0x4000;  /* TOCLK_ENA */
pub const WM9090_TOCLK_ENA_SHIFT: u16 = 14;  /* TOCLK_ENA */
pub const WM9090_TOCLK_ENA_WIDTH: u16 = 1;  /* TOCLK_ENA */

/*
 * R22 (0x16) - IN1 Line Control
 */
pub const WM9090_IN1_DIFF: u16 = 0x0002;  /* IN1_DIFF */
pub const WM9090_IN1_DIFF_MASK: u16 = 0x0002;  /* IN1_DIFF */
pub const WM9090_IN1_DIFF_SHIFT: u16 = 1;  /* IN1_DIFF */
pub const WM9090_IN1_DIFF_WIDTH: u16 = 1;  /* IN1_DIFF */
pub const WM9090_IN1_CLAMP: u16 = 0x0001;  /* IN1_CLAMP */
pub const WM9090_IN1_CLAMP_MASK: u16 = 0x0001;  /* IN1_CLAMP */
pub const WM9090_IN1_CLAMP_SHIFT: u16 = 0;  /* IN1_CLAMP */
pub const WM9090_IN1_CLAMP_WIDTH: u16 = 1;  /* IN1_CLAMP */

/*
 * R23 (0x17) - IN2 Line Control
 */
pub const WM9090_IN2_DIFF: u16 = 0x0002;  /* IN2_DIFF */
pub const WM9090_IN2_DIFF_MASK: u16 = 0x0002;  /* IN2_DIFF */
pub const WM9090_IN2_DIFF_SHIFT: u16 = 1;  /* IN2_DIFF */
pub const WM9090_IN2_DIFF_WIDTH: u16 = 1;  /* IN2_DIFF */
pub const WM9090_IN2_CLAMP: u16 = 0x0001;  /* IN2_CLAMP */
pub const WM9090_IN2_CLAMP_MASK: u16 = 0x0001;  /* IN2_CLAMP */
pub const WM9090_IN2_CLAMP_SHIFT: u16 = 0;  /* IN2_CLAMP */
pub const WM9090_IN2_CLAMP_WIDTH: u16 = 1;  /* IN2_CLAMP */

/*
 * R24 (0x18) - IN1 Line Input A Volume
 */
pub const WM9090_IN1_VU: u16 = 0x0100;  /* IN1_VU */
pub const WM9090_IN1_VU_MASK: u16 = 0x0100;  /* IN1_VU */
pub const WM9090_IN1_VU_SHIFT: u16 = 8;  /* IN1_VU */
pub const WM9090_IN1_VU_WIDTH: u16 = 1;  /* IN1_VU */
pub const WM9090_IN1A_MUTE: u16 = 0x0080;  /* IN1A_MUTE */
pub const WM9090_IN1A_MUTE_MASK: u16 = 0x0080;  /* IN1A_MUTE */
pub const WM9090_IN1A_MUTE_SHIFT: u16 = 7;  /* IN1A_MUTE */
pub const WM9090_IN1A_MUTE_WIDTH: u16 = 1;  /* IN1A_MUTE */
pub const WM9090_IN1A_ZC: u16 = 0x0040;  /* IN1A_ZC */
pub const WM9090_IN1A_ZC_MASK: u16 = 0x0040;  /* IN1A_ZC */
pub const WM9090_IN1A_ZC_SHIFT: u16 = 6;  /* IN1A_ZC */
pub const WM9090_IN1A_ZC_WIDTH: u16 = 1;  /* IN1A_ZC */
pub const WM9090_IN1A_VOL_MASK: u16 = 0x0007;  /* IN1A_VOL - [2:0] */
pub const WM9090_IN1A_VOL_SHIFT: u16 = 0;  /* IN1A_VOL - [2:0] */
pub const WM9090_IN1A_VOL_WIDTH: u16 = 3;  /* IN1A_VOL - [2:0] */

/*
 * R25 (0x19) - IN1  Line Input B Volume
 */
// Repeated C macro with identical value: #define WM9090_IN1_VU 0x0100
// Repeated C macro with identical value: #define WM9090_IN1_VU_MASK 0x0100
// Repeated C macro with identical value: #define WM9090_IN1_VU_SHIFT 8
// Repeated C macro with identical value: #define WM9090_IN1_VU_WIDTH 1
pub const WM9090_IN1B_MUTE: u16 = 0x0080;  /* IN1B_MUTE */
pub const WM9090_IN1B_MUTE_MASK: u16 = 0x0080;  /* IN1B_MUTE */
pub const WM9090_IN1B_MUTE_SHIFT: u16 = 7;  /* IN1B_MUTE */
pub const WM9090_IN1B_MUTE_WIDTH: u16 = 1;  /* IN1B_MUTE */
pub const WM9090_IN1B_ZC: u16 = 0x0040;  /* IN1B_ZC */
pub const WM9090_IN1B_ZC_MASK: u16 = 0x0040;  /* IN1B_ZC */
pub const WM9090_IN1B_ZC_SHIFT: u16 = 6;  /* IN1B_ZC */
pub const WM9090_IN1B_ZC_WIDTH: u16 = 1;  /* IN1B_ZC */
pub const WM9090_IN1B_VOL_MASK: u16 = 0x0007;  /* IN1B_VOL - [2:0] */
pub const WM9090_IN1B_VOL_SHIFT: u16 = 0;  /* IN1B_VOL - [2:0] */
pub const WM9090_IN1B_VOL_WIDTH: u16 = 3;  /* IN1B_VOL - [2:0] */

/*
 * R26 (0x1A) - IN2 Line Input A Volume
 */
pub const WM9090_IN2_VU: u16 = 0x0100;  /* IN2_VU */
pub const WM9090_IN2_VU_MASK: u16 = 0x0100;  /* IN2_VU */
pub const WM9090_IN2_VU_SHIFT: u16 = 8;  /* IN2_VU */
pub const WM9090_IN2_VU_WIDTH: u16 = 1;  /* IN2_VU */
pub const WM9090_IN2A_MUTE: u16 = 0x0080;  /* IN2A_MUTE */
pub const WM9090_IN2A_MUTE_MASK: u16 = 0x0080;  /* IN2A_MUTE */
pub const WM9090_IN2A_MUTE_SHIFT: u16 = 7;  /* IN2A_MUTE */
pub const WM9090_IN2A_MUTE_WIDTH: u16 = 1;  /* IN2A_MUTE */
pub const WM9090_IN2A_ZC: u16 = 0x0040;  /* IN2A_ZC */
pub const WM9090_IN2A_ZC_MASK: u16 = 0x0040;  /* IN2A_ZC */
pub const WM9090_IN2A_ZC_SHIFT: u16 = 6;  /* IN2A_ZC */
pub const WM9090_IN2A_ZC_WIDTH: u16 = 1;  /* IN2A_ZC */
pub const WM9090_IN2A_VOL_MASK: u16 = 0x0007;  /* IN2A_VOL - [2:0] */
pub const WM9090_IN2A_VOL_SHIFT: u16 = 0;  /* IN2A_VOL - [2:0] */
pub const WM9090_IN2A_VOL_WIDTH: u16 = 3;  /* IN2A_VOL - [2:0] */

/*
 * R27 (0x1B) - IN2 Line Input B Volume
 */
// Repeated C macro with identical value: #define WM9090_IN2_VU 0x0100
// Repeated C macro with identical value: #define WM9090_IN2_VU_MASK 0x0100
// Repeated C macro with identical value: #define WM9090_IN2_VU_SHIFT 8
// Repeated C macro with identical value: #define WM9090_IN2_VU_WIDTH 1
pub const WM9090_IN2B_MUTE: u16 = 0x0080;  /* IN2B_MUTE */
pub const WM9090_IN2B_MUTE_MASK: u16 = 0x0080;  /* IN2B_MUTE */
pub const WM9090_IN2B_MUTE_SHIFT: u16 = 7;  /* IN2B_MUTE */
pub const WM9090_IN2B_MUTE_WIDTH: u16 = 1;  /* IN2B_MUTE */
pub const WM9090_IN2B_ZC: u16 = 0x0040;  /* IN2B_ZC */
pub const WM9090_IN2B_ZC_MASK: u16 = 0x0040;  /* IN2B_ZC */
pub const WM9090_IN2B_ZC_SHIFT: u16 = 6;  /* IN2B_ZC */
pub const WM9090_IN2B_ZC_WIDTH: u16 = 1;  /* IN2B_ZC */
pub const WM9090_IN2B_VOL_MASK: u16 = 0x0007;  /* IN2B_VOL - [2:0] */
pub const WM9090_IN2B_VOL_SHIFT: u16 = 0;  /* IN2B_VOL - [2:0] */
pub const WM9090_IN2B_VOL_WIDTH: u16 = 3;  /* IN2B_VOL - [2:0] */

/*
 * R28 (0x1C) - Left Output Volume
 */
pub const WM9090_HPOUT1_VU: u16 = 0x0100;  /* HPOUT1_VU */
pub const WM9090_HPOUT1_VU_MASK: u16 = 0x0100;  /* HPOUT1_VU */
pub const WM9090_HPOUT1_VU_SHIFT: u16 = 8;  /* HPOUT1_VU */
pub const WM9090_HPOUT1_VU_WIDTH: u16 = 1;  /* HPOUT1_VU */
pub const WM9090_HPOUT1L_ZC: u16 = 0x0080;  /* HPOUT1L_ZC */
pub const WM9090_HPOUT1L_ZC_MASK: u16 = 0x0080;  /* HPOUT1L_ZC */
pub const WM9090_HPOUT1L_ZC_SHIFT: u16 = 7;  /* HPOUT1L_ZC */
pub const WM9090_HPOUT1L_ZC_WIDTH: u16 = 1;  /* HPOUT1L_ZC */
pub const WM9090_HPOUT1L_MUTE: u16 = 0x0040;  /* HPOUT1L_MUTE */
pub const WM9090_HPOUT1L_MUTE_MASK: u16 = 0x0040;  /* HPOUT1L_MUTE */
pub const WM9090_HPOUT1L_MUTE_SHIFT: u16 = 6;  /* HPOUT1L_MUTE */
pub const WM9090_HPOUT1L_MUTE_WIDTH: u16 = 1;  /* HPOUT1L_MUTE */
pub const WM9090_HPOUT1L_VOL_MASK: u16 = 0x003F;  /* HPOUT1L_VOL - [5:0] */
pub const WM9090_HPOUT1L_VOL_SHIFT: u16 = 0;  /* HPOUT1L_VOL - [5:0] */
pub const WM9090_HPOUT1L_VOL_WIDTH: u16 = 6;  /* HPOUT1L_VOL - [5:0] */

/*
 * R29 (0x1D) - Right Output Volume
 */
// Repeated C macro with identical value: #define WM9090_HPOUT1_VU 0x0100
// Repeated C macro with identical value: #define WM9090_HPOUT1_VU_MASK 0x0100
// Repeated C macro with identical value: #define WM9090_HPOUT1_VU_SHIFT 8
// Repeated C macro with identical value: #define WM9090_HPOUT1_VU_WIDTH 1
pub const WM9090_HPOUT1R_ZC: u16 = 0x0080;  /* HPOUT1R_ZC */
pub const WM9090_HPOUT1R_ZC_MASK: u16 = 0x0080;  /* HPOUT1R_ZC */
pub const WM9090_HPOUT1R_ZC_SHIFT: u16 = 7;  /* HPOUT1R_ZC */
pub const WM9090_HPOUT1R_ZC_WIDTH: u16 = 1;  /* HPOUT1R_ZC */
pub const WM9090_HPOUT1R_MUTE: u16 = 0x0040;  /* HPOUT1R_MUTE */
pub const WM9090_HPOUT1R_MUTE_MASK: u16 = 0x0040;  /* HPOUT1R_MUTE */
pub const WM9090_HPOUT1R_MUTE_SHIFT: u16 = 6;  /* HPOUT1R_MUTE */
pub const WM9090_HPOUT1R_MUTE_WIDTH: u16 = 1;  /* HPOUT1R_MUTE */
pub const WM9090_HPOUT1R_VOL_MASK: u16 = 0x003F;  /* HPOUT1R_VOL - [5:0] */
pub const WM9090_HPOUT1R_VOL_SHIFT: u16 = 0;  /* HPOUT1R_VOL - [5:0] */
pub const WM9090_HPOUT1R_VOL_WIDTH: u16 = 6;  /* HPOUT1R_VOL - [5:0] */

/*
 * R34 (0x22) - SPKMIXL Attenuation
 */
pub const WM9090_SPKMIX_MUTE: u16 = 0x0100;  /* SPKMIX_MUTE */
pub const WM9090_SPKMIX_MUTE_MASK: u16 = 0x0100;  /* SPKMIX_MUTE */
pub const WM9090_SPKMIX_MUTE_SHIFT: u16 = 8;  /* SPKMIX_MUTE */
pub const WM9090_SPKMIX_MUTE_WIDTH: u16 = 1;  /* SPKMIX_MUTE */
pub const WM9090_IN1A_SPKMIX_VOL_MASK: u16 = 0x00C0;  /* IN1A_SPKMIX_VOL - [7:6] */
pub const WM9090_IN1A_SPKMIX_VOL_SHIFT: u16 = 6;  /* IN1A_SPKMIX_VOL - [7:6] */
pub const WM9090_IN1A_SPKMIX_VOL_WIDTH: u16 = 2;  /* IN1A_SPKMIX_VOL - [7:6] */
pub const WM9090_IN1B_SPKMIX_VOL_MASK: u16 = 0x0030;  /* IN1B_SPKMIX_VOL - [5:4] */
pub const WM9090_IN1B_SPKMIX_VOL_SHIFT: u16 = 4;  /* IN1B_SPKMIX_VOL - [5:4] */
pub const WM9090_IN1B_SPKMIX_VOL_WIDTH: u16 = 2;  /* IN1B_SPKMIX_VOL - [5:4] */
pub const WM9090_IN2A_SPKMIX_VOL_MASK: u16 = 0x000C;  /* IN2A_SPKMIX_VOL - [3:2] */
pub const WM9090_IN2A_SPKMIX_VOL_SHIFT: u16 = 2;  /* IN2A_SPKMIX_VOL - [3:2] */
pub const WM9090_IN2A_SPKMIX_VOL_WIDTH: u16 = 2;  /* IN2A_SPKMIX_VOL - [3:2] */
pub const WM9090_IN2B_SPKMIX_VOL_MASK: u16 = 0x0003;  /* IN2B_SPKMIX_VOL - [1:0] */
pub const WM9090_IN2B_SPKMIX_VOL_SHIFT: u16 = 0;  /* IN2B_SPKMIX_VOL - [1:0] */
pub const WM9090_IN2B_SPKMIX_VOL_WIDTH: u16 = 2;  /* IN2B_SPKMIX_VOL - [1:0] */

/*
 * R36 (0x24) - SPKOUT Mixers
 */
pub const WM9090_SPKMIXL_TO_SPKOUTL: u16 = 0x0010;  /* SPKMIXL_TO_SPKOUTL */
pub const WM9090_SPKMIXL_TO_SPKOUTL_MASK: u16 = 0x0010;  /* SPKMIXL_TO_SPKOUTL */
pub const WM9090_SPKMIXL_TO_SPKOUTL_SHIFT: u16 = 4;  /* SPKMIXL_TO_SPKOUTL */
pub const WM9090_SPKMIXL_TO_SPKOUTL_WIDTH: u16 = 1;  /* SPKMIXL_TO_SPKOUTL */

/*
 * R37 (0x25) - ClassD3
 */
pub const WM9090_SPKOUTL_BOOST_MASK: u16 = 0x0038;  /* SPKOUTL_BOOST - [5:3] */
pub const WM9090_SPKOUTL_BOOST_SHIFT: u16 = 3;  /* SPKOUTL_BOOST - [5:3] */
pub const WM9090_SPKOUTL_BOOST_WIDTH: u16 = 3;  /* SPKOUTL_BOOST - [5:3] */

/*
 * R38 (0x26) - Speaker Volume Left
 */
pub const WM9090_SPKOUT_VU: u16 = 0x0100;  /* SPKOUT_VU */
pub const WM9090_SPKOUT_VU_MASK: u16 = 0x0100;  /* SPKOUT_VU */
pub const WM9090_SPKOUT_VU_SHIFT: u16 = 8;  /* SPKOUT_VU */
pub const WM9090_SPKOUT_VU_WIDTH: u16 = 1;  /* SPKOUT_VU */
pub const WM9090_SPKOUTL_ZC: u16 = 0x0080;  /* SPKOUTL_ZC */
pub const WM9090_SPKOUTL_ZC_MASK: u16 = 0x0080;  /* SPKOUTL_ZC */
pub const WM9090_SPKOUTL_ZC_SHIFT: u16 = 7;  /* SPKOUTL_ZC */
pub const WM9090_SPKOUTL_ZC_WIDTH: u16 = 1;  /* SPKOUTL_ZC */
pub const WM9090_SPKOUTL_MUTE: u16 = 0x0040;  /* SPKOUTL_MUTE */
pub const WM9090_SPKOUTL_MUTE_MASK: u16 = 0x0040;  /* SPKOUTL_MUTE */
pub const WM9090_SPKOUTL_MUTE_SHIFT: u16 = 6;  /* SPKOUTL_MUTE */
pub const WM9090_SPKOUTL_MUTE_WIDTH: u16 = 1;  /* SPKOUTL_MUTE */
pub const WM9090_SPKOUTL_VOL_MASK: u16 = 0x003F;  /* SPKOUTL_VOL - [5:0] */
pub const WM9090_SPKOUTL_VOL_SHIFT: u16 = 0;  /* SPKOUTL_VOL - [5:0] */
pub const WM9090_SPKOUTL_VOL_WIDTH: u16 = 6;  /* SPKOUTL_VOL - [5:0] */

/*
 * R45 (0x2D) - Output Mixer1
 */
pub const WM9090_IN1A_TO_MIXOUTL: u16 = 0x0040;  /* IN1A_TO_MIXOUTL */
pub const WM9090_IN1A_TO_MIXOUTL_MASK: u16 = 0x0040;  /* IN1A_TO_MIXOUTL */
pub const WM9090_IN1A_TO_MIXOUTL_SHIFT: u16 = 6;  /* IN1A_TO_MIXOUTL */
pub const WM9090_IN1A_TO_MIXOUTL_WIDTH: u16 = 1;  /* IN1A_TO_MIXOUTL */
pub const WM9090_IN2A_TO_MIXOUTL: u16 = 0x0004;  /* IN2A_TO_MIXOUTL */
pub const WM9090_IN2A_TO_MIXOUTL_MASK: u16 = 0x0004;  /* IN2A_TO_MIXOUTL */
pub const WM9090_IN2A_TO_MIXOUTL_SHIFT: u16 = 2;  /* IN2A_TO_MIXOUTL */
pub const WM9090_IN2A_TO_MIXOUTL_WIDTH: u16 = 1;  /* IN2A_TO_MIXOUTL */

/*
 * R46 (0x2E) - Output Mixer2
 */
pub const WM9090_IN1A_TO_MIXOUTR: u16 = 0x0040;  /* IN1A_TO_MIXOUTR */
pub const WM9090_IN1A_TO_MIXOUTR_MASK: u16 = 0x0040;  /* IN1A_TO_MIXOUTR */
pub const WM9090_IN1A_TO_MIXOUTR_SHIFT: u16 = 6;  /* IN1A_TO_MIXOUTR */
pub const WM9090_IN1A_TO_MIXOUTR_WIDTH: u16 = 1;  /* IN1A_TO_MIXOUTR */
pub const WM9090_IN1B_TO_MIXOUTR: u16 = 0x0010;  /* IN1B_TO_MIXOUTR */
pub const WM9090_IN1B_TO_MIXOUTR_MASK: u16 = 0x0010;  /* IN1B_TO_MIXOUTR */
pub const WM9090_IN1B_TO_MIXOUTR_SHIFT: u16 = 4;  /* IN1B_TO_MIXOUTR */
pub const WM9090_IN1B_TO_MIXOUTR_WIDTH: u16 = 1;  /* IN1B_TO_MIXOUTR */
pub const WM9090_IN2A_TO_MIXOUTR: u16 = 0x0004;  /* IN2A_TO_MIXOUTR */
pub const WM9090_IN2A_TO_MIXOUTR_MASK: u16 = 0x0004;  /* IN2A_TO_MIXOUTR */
pub const WM9090_IN2A_TO_MIXOUTR_SHIFT: u16 = 2;  /* IN2A_TO_MIXOUTR */
pub const WM9090_IN2A_TO_MIXOUTR_WIDTH: u16 = 1;  /* IN2A_TO_MIXOUTR */
pub const WM9090_IN2B_TO_MIXOUTR: u16 = 0x0001;  /* IN2B_TO_MIXOUTR */
pub const WM9090_IN2B_TO_MIXOUTR_MASK: u16 = 0x0001;  /* IN2B_TO_MIXOUTR */
pub const WM9090_IN2B_TO_MIXOUTR_SHIFT: u16 = 0;  /* IN2B_TO_MIXOUTR */
pub const WM9090_IN2B_TO_MIXOUTR_WIDTH: u16 = 1;  /* IN2B_TO_MIXOUTR */

/*
 * R47 (0x2F) - Output Mixer3
 */
pub const WM9090_MIXOUTL_MUTE: u16 = 0x0100;  /* MIXOUTL_MUTE */
pub const WM9090_MIXOUTL_MUTE_MASK: u16 = 0x0100;  /* MIXOUTL_MUTE */
pub const WM9090_MIXOUTL_MUTE_SHIFT: u16 = 8;  /* MIXOUTL_MUTE */
pub const WM9090_MIXOUTL_MUTE_WIDTH: u16 = 1;  /* MIXOUTL_MUTE */
pub const WM9090_IN1A_MIXOUTL_VOL_MASK: u16 = 0x00C0;  /* IN1A_MIXOUTL_VOL - [7:6] */
pub const WM9090_IN1A_MIXOUTL_VOL_SHIFT: u16 = 6;  /* IN1A_MIXOUTL_VOL - [7:6] */
pub const WM9090_IN1A_MIXOUTL_VOL_WIDTH: u16 = 2;  /* IN1A_MIXOUTL_VOL - [7:6] */
pub const WM9090_IN2A_MIXOUTL_VOL_MASK: u16 = 0x000C;  /* IN2A_MIXOUTL_VOL - [3:2] */
pub const WM9090_IN2A_MIXOUTL_VOL_SHIFT: u16 = 2;  /* IN2A_MIXOUTL_VOL - [3:2] */
pub const WM9090_IN2A_MIXOUTL_VOL_WIDTH: u16 = 2;  /* IN2A_MIXOUTL_VOL - [3:2] */

/*
 * R48 (0x30) - Output Mixer4
 */
pub const WM9090_MIXOUTR_MUTE: u16 = 0x0100;  /* MIXOUTR_MUTE */
pub const WM9090_MIXOUTR_MUTE_MASK: u16 = 0x0100;  /* MIXOUTR_MUTE */
pub const WM9090_MIXOUTR_MUTE_SHIFT: u16 = 8;  /* MIXOUTR_MUTE */
pub const WM9090_MIXOUTR_MUTE_WIDTH: u16 = 1;  /* MIXOUTR_MUTE */
pub const WM9090_IN1A_MIXOUTR_VOL_MASK: u16 = 0x00C0;  /* IN1A_MIXOUTR_VOL - [7:6] */
pub const WM9090_IN1A_MIXOUTR_VOL_SHIFT: u16 = 6;  /* IN1A_MIXOUTR_VOL - [7:6] */
pub const WM9090_IN1A_MIXOUTR_VOL_WIDTH: u16 = 2;  /* IN1A_MIXOUTR_VOL - [7:6] */
pub const WM9090_IN1B_MIXOUTR_VOL_MASK: u16 = 0x0030;  /* IN1B_MIXOUTR_VOL - [5:4] */
pub const WM9090_IN1B_MIXOUTR_VOL_SHIFT: u16 = 4;  /* IN1B_MIXOUTR_VOL - [5:4] */
pub const WM9090_IN1B_MIXOUTR_VOL_WIDTH: u16 = 2;  /* IN1B_MIXOUTR_VOL - [5:4] */
pub const WM9090_IN2A_MIXOUTR_VOL_MASK: u16 = 0x000C;  /* IN2A_MIXOUTR_VOL - [3:2] */
pub const WM9090_IN2A_MIXOUTR_VOL_SHIFT: u16 = 2;  /* IN2A_MIXOUTR_VOL - [3:2] */
pub const WM9090_IN2A_MIXOUTR_VOL_WIDTH: u16 = 2;  /* IN2A_MIXOUTR_VOL - [3:2] */
pub const WM9090_IN2B_MIXOUTR_VOL_MASK: u16 = 0x0003;  /* IN2B_MIXOUTR_VOL - [1:0] */
pub const WM9090_IN2B_MIXOUTR_VOL_SHIFT: u16 = 0;  /* IN2B_MIXOUTR_VOL - [1:0] */
pub const WM9090_IN2B_MIXOUTR_VOL_WIDTH: u16 = 2;  /* IN2B_MIXOUTR_VOL - [1:0] */

/*
 * R54 (0x36) - Speaker Mixer
 */
pub const WM9090_IN1A_TO_SPKMIX: u16 = 0x0040;  /* IN1A_TO_SPKMIX */
pub const WM9090_IN1A_TO_SPKMIX_MASK: u16 = 0x0040;  /* IN1A_TO_SPKMIX */
pub const WM9090_IN1A_TO_SPKMIX_SHIFT: u16 = 6;  /* IN1A_TO_SPKMIX */
pub const WM9090_IN1A_TO_SPKMIX_WIDTH: u16 = 1;  /* IN1A_TO_SPKMIX */
pub const WM9090_IN1B_TO_SPKMIX: u16 = 0x0010;  /* IN1B_TO_SPKMIX */
pub const WM9090_IN1B_TO_SPKMIX_MASK: u16 = 0x0010;  /* IN1B_TO_SPKMIX */
pub const WM9090_IN1B_TO_SPKMIX_SHIFT: u16 = 4;  /* IN1B_TO_SPKMIX */
pub const WM9090_IN1B_TO_SPKMIX_WIDTH: u16 = 1;  /* IN1B_TO_SPKMIX */
pub const WM9090_IN2A_TO_SPKMIX: u16 = 0x0004;  /* IN2A_TO_SPKMIX */
pub const WM9090_IN2A_TO_SPKMIX_MASK: u16 = 0x0004;  /* IN2A_TO_SPKMIX */
pub const WM9090_IN2A_TO_SPKMIX_SHIFT: u16 = 2;  /* IN2A_TO_SPKMIX */
pub const WM9090_IN2A_TO_SPKMIX_WIDTH: u16 = 1;  /* IN2A_TO_SPKMIX */
pub const WM9090_IN2B_TO_SPKMIX: u16 = 0x0001;  /* IN2B_TO_SPKMIX */
pub const WM9090_IN2B_TO_SPKMIX_MASK: u16 = 0x0001;  /* IN2B_TO_SPKMIX */
pub const WM9090_IN2B_TO_SPKMIX_SHIFT: u16 = 0;  /* IN2B_TO_SPKMIX */
pub const WM9090_IN2B_TO_SPKMIX_WIDTH: u16 = 1;  /* IN2B_TO_SPKMIX */

/*
 * R57 (0x39) - AntiPOP2
 */
pub const WM9090_VMID_BUF_ENA: u16 = 0x0008;  /* VMID_BUF_ENA */
pub const WM9090_VMID_BUF_ENA_MASK: u16 = 0x0008;  /* VMID_BUF_ENA */
pub const WM9090_VMID_BUF_ENA_SHIFT: u16 = 3;  /* VMID_BUF_ENA */
pub const WM9090_VMID_BUF_ENA_WIDTH: u16 = 1;  /* VMID_BUF_ENA */
pub const WM9090_VMID_ENA: u16 = 0x0001;  /* VMID_ENA */
pub const WM9090_VMID_ENA_MASK: u16 = 0x0001;  /* VMID_ENA */
pub const WM9090_VMID_ENA_SHIFT: u16 = 0;  /* VMID_ENA */
pub const WM9090_VMID_ENA_WIDTH: u16 = 1;  /* VMID_ENA */

/*
 * R70 (0x46) - Write Sequencer 0
 */
pub const WM9090_WSEQ_ENA: u16 = 0x0100;  /* WSEQ_ENA */
pub const WM9090_WSEQ_ENA_MASK: u16 = 0x0100;  /* WSEQ_ENA */
pub const WM9090_WSEQ_ENA_SHIFT: u16 = 8;  /* WSEQ_ENA */
pub const WM9090_WSEQ_ENA_WIDTH: u16 = 1;  /* WSEQ_ENA */
pub const WM9090_WSEQ_WRITE_INDEX_MASK: u16 = 0x000F;  /* WSEQ_WRITE_INDEX - [3:0] */
pub const WM9090_WSEQ_WRITE_INDEX_SHIFT: u16 = 0;  /* WSEQ_WRITE_INDEX - [3:0] */
pub const WM9090_WSEQ_WRITE_INDEX_WIDTH: u16 = 4;  /* WSEQ_WRITE_INDEX - [3:0] */

/*
 * R71 (0x47) - Write Sequencer 1
 */
pub const WM9090_WSEQ_DATA_WIDTH_MASK: u16 = 0x7000;  /* WSEQ_DATA_WIDTH - [14:12] */
pub const WM9090_WSEQ_DATA_WIDTH_SHIFT: u16 = 12;  /* WSEQ_DATA_WIDTH - [14:12] */
pub const WM9090_WSEQ_DATA_WIDTH_WIDTH: u16 = 3;  /* WSEQ_DATA_WIDTH - [14:12] */
pub const WM9090_WSEQ_DATA_START_MASK: u16 = 0x0F00;  /* WSEQ_DATA_START - [11:8] */
pub const WM9090_WSEQ_DATA_START_SHIFT: u16 = 8;  /* WSEQ_DATA_START - [11:8] */
pub const WM9090_WSEQ_DATA_START_WIDTH: u16 = 4;  /* WSEQ_DATA_START - [11:8] */
pub const WM9090_WSEQ_ADDR_MASK: u16 = 0x00FF;  /* WSEQ_ADDR - [7:0] */
pub const WM9090_WSEQ_ADDR_SHIFT: u16 = 0;  /* WSEQ_ADDR - [7:0] */
pub const WM9090_WSEQ_ADDR_WIDTH: u16 = 8;  /* WSEQ_ADDR - [7:0] */

/*
 * R72 (0x48) - Write Sequencer 2
 */
pub const WM9090_WSEQ_EOS: u16 = 0x4000;  /* WSEQ_EOS */
pub const WM9090_WSEQ_EOS_MASK: u16 = 0x4000;  /* WSEQ_EOS */
pub const WM9090_WSEQ_EOS_SHIFT: u16 = 14;  /* WSEQ_EOS */
pub const WM9090_WSEQ_EOS_WIDTH: u16 = 1;  /* WSEQ_EOS */
pub const WM9090_WSEQ_DELAY_MASK: u16 = 0x0F00;  /* WSEQ_DELAY - [11:8] */
pub const WM9090_WSEQ_DELAY_SHIFT: u16 = 8;  /* WSEQ_DELAY - [11:8] */
pub const WM9090_WSEQ_DELAY_WIDTH: u16 = 4;  /* WSEQ_DELAY - [11:8] */
pub const WM9090_WSEQ_DATA_MASK: u16 = 0x00FF;  /* WSEQ_DATA - [7:0] */
pub const WM9090_WSEQ_DATA_SHIFT: u16 = 0;  /* WSEQ_DATA - [7:0] */
pub const WM9090_WSEQ_DATA_WIDTH: u16 = 8;  /* WSEQ_DATA - [7:0] */

/*
 * R73 (0x49) - Write Sequencer 3
 */
pub const WM9090_WSEQ_ABORT: u16 = 0x0200;  /* WSEQ_ABORT */
pub const WM9090_WSEQ_ABORT_MASK: u16 = 0x0200;  /* WSEQ_ABORT */
pub const WM9090_WSEQ_ABORT_SHIFT: u16 = 9;  /* WSEQ_ABORT */
pub const WM9090_WSEQ_ABORT_WIDTH: u16 = 1;  /* WSEQ_ABORT */
pub const WM9090_WSEQ_START: u16 = 0x0100;  /* WSEQ_START */
pub const WM9090_WSEQ_START_MASK: u16 = 0x0100;  /* WSEQ_START */
pub const WM9090_WSEQ_START_SHIFT: u16 = 8;  /* WSEQ_START */
pub const WM9090_WSEQ_START_WIDTH: u16 = 1;  /* WSEQ_START */
pub const WM9090_WSEQ_START_INDEX_MASK: u16 = 0x003F;  /* WSEQ_START_INDEX - [5:0] */
pub const WM9090_WSEQ_START_INDEX_SHIFT: u16 = 0;  /* WSEQ_START_INDEX - [5:0] */
pub const WM9090_WSEQ_START_INDEX_WIDTH: u16 = 6;  /* WSEQ_START_INDEX - [5:0] */

/*
 * R74 (0x4A) - Write Sequencer 4
 */
pub const WM9090_WSEQ_BUSY: u16 = 0x0001;  /* WSEQ_BUSY */
pub const WM9090_WSEQ_BUSY_MASK: u16 = 0x0001;  /* WSEQ_BUSY */
pub const WM9090_WSEQ_BUSY_SHIFT: u16 = 0;  /* WSEQ_BUSY */
pub const WM9090_WSEQ_BUSY_WIDTH: u16 = 1;  /* WSEQ_BUSY */

/*
 * R75 (0x4B) - Write Sequencer 5
 */
pub const WM9090_WSEQ_CURRENT_INDEX_MASK: u16 = 0x003F;  /* WSEQ_CURRENT_INDEX - [5:0] */
pub const WM9090_WSEQ_CURRENT_INDEX_SHIFT: u16 = 0;  /* WSEQ_CURRENT_INDEX - [5:0] */
pub const WM9090_WSEQ_CURRENT_INDEX_WIDTH: u16 = 6;  /* WSEQ_CURRENT_INDEX - [5:0] */

/*
 * R76 (0x4C) - Charge Pump 1
 */
pub const WM9090_CP_ENA: u16 = 0x8000;  /* CP_ENA */
pub const WM9090_CP_ENA_MASK: u16 = 0x8000;  /* CP_ENA */
pub const WM9090_CP_ENA_SHIFT: u16 = 15;  /* CP_ENA */
pub const WM9090_CP_ENA_WIDTH: u16 = 1;  /* CP_ENA */

/*
 * R84 (0x54) - DC Servo 0
 */
pub const WM9090_DCS_TRIG_SINGLE_1: u16 = 0x2000;  /* DCS_TRIG_SINGLE_1 */
pub const WM9090_DCS_TRIG_SINGLE_1_MASK: u16 = 0x2000;  /* DCS_TRIG_SINGLE_1 */
pub const WM9090_DCS_TRIG_SINGLE_1_SHIFT: u16 = 13;  /* DCS_TRIG_SINGLE_1 */
pub const WM9090_DCS_TRIG_SINGLE_1_WIDTH: u16 = 1;  /* DCS_TRIG_SINGLE_1 */
pub const WM9090_DCS_TRIG_SINGLE_0: u16 = 0x1000;  /* DCS_TRIG_SINGLE_0 */
pub const WM9090_DCS_TRIG_SINGLE_0_MASK: u16 = 0x1000;  /* DCS_TRIG_SINGLE_0 */
pub const WM9090_DCS_TRIG_SINGLE_0_SHIFT: u16 = 12;  /* DCS_TRIG_SINGLE_0 */
pub const WM9090_DCS_TRIG_SINGLE_0_WIDTH: u16 = 1;  /* DCS_TRIG_SINGLE_0 */
pub const WM9090_DCS_TRIG_SERIES_1: u16 = 0x0200;  /* DCS_TRIG_SERIES_1 */
pub const WM9090_DCS_TRIG_SERIES_1_MASK: u16 = 0x0200;  /* DCS_TRIG_SERIES_1 */
pub const WM9090_DCS_TRIG_SERIES_1_SHIFT: u16 = 9;  /* DCS_TRIG_SERIES_1 */
pub const WM9090_DCS_TRIG_SERIES_1_WIDTH: u16 = 1;  /* DCS_TRIG_SERIES_1 */
pub const WM9090_DCS_TRIG_SERIES_0: u16 = 0x0100;  /* DCS_TRIG_SERIES_0 */
pub const WM9090_DCS_TRIG_SERIES_0_MASK: u16 = 0x0100;  /* DCS_TRIG_SERIES_0 */
pub const WM9090_DCS_TRIG_SERIES_0_SHIFT: u16 = 8;  /* DCS_TRIG_SERIES_0 */
pub const WM9090_DCS_TRIG_SERIES_0_WIDTH: u16 = 1;  /* DCS_TRIG_SERIES_0 */
pub const WM9090_DCS_TRIG_STARTUP_1: u16 = 0x0020;  /* DCS_TRIG_STARTUP_1 */
pub const WM9090_DCS_TRIG_STARTUP_1_MASK: u16 = 0x0020;  /* DCS_TRIG_STARTUP_1 */
pub const WM9090_DCS_TRIG_STARTUP_1_SHIFT: u16 = 5;  /* DCS_TRIG_STARTUP_1 */
pub const WM9090_DCS_TRIG_STARTUP_1_WIDTH: u16 = 1;  /* DCS_TRIG_STARTUP_1 */
pub const WM9090_DCS_TRIG_STARTUP_0: u16 = 0x0010;  /* DCS_TRIG_STARTUP_0 */
pub const WM9090_DCS_TRIG_STARTUP_0_MASK: u16 = 0x0010;  /* DCS_TRIG_STARTUP_0 */
pub const WM9090_DCS_TRIG_STARTUP_0_SHIFT: u16 = 4;  /* DCS_TRIG_STARTUP_0 */
pub const WM9090_DCS_TRIG_STARTUP_0_WIDTH: u16 = 1;  /* DCS_TRIG_STARTUP_0 */
pub const WM9090_DCS_TRIG_DAC_WR_1: u16 = 0x0008;  /* DCS_TRIG_DAC_WR_1 */
pub const WM9090_DCS_TRIG_DAC_WR_1_MASK: u16 = 0x0008;  /* DCS_TRIG_DAC_WR_1 */
pub const WM9090_DCS_TRIG_DAC_WR_1_SHIFT: u16 = 3;  /* DCS_TRIG_DAC_WR_1 */
pub const WM9090_DCS_TRIG_DAC_WR_1_WIDTH: u16 = 1;  /* DCS_TRIG_DAC_WR_1 */
pub const WM9090_DCS_TRIG_DAC_WR_0: u16 = 0x0004;  /* DCS_TRIG_DAC_WR_0 */
pub const WM9090_DCS_TRIG_DAC_WR_0_MASK: u16 = 0x0004;  /* DCS_TRIG_DAC_WR_0 */
pub const WM9090_DCS_TRIG_DAC_WR_0_SHIFT: u16 = 2;  /* DCS_TRIG_DAC_WR_0 */
pub const WM9090_DCS_TRIG_DAC_WR_0_WIDTH: u16 = 1;  /* DCS_TRIG_DAC_WR_0 */
pub const WM9090_DCS_ENA_CHAN_1: u16 = 0x0002;  /* DCS_ENA_CHAN_1 */
pub const WM9090_DCS_ENA_CHAN_1_MASK: u16 = 0x0002;  /* DCS_ENA_CHAN_1 */
pub const WM9090_DCS_ENA_CHAN_1_SHIFT: u16 = 1;  /* DCS_ENA_CHAN_1 */
pub const WM9090_DCS_ENA_CHAN_1_WIDTH: u16 = 1;  /* DCS_ENA_CHAN_1 */
pub const WM9090_DCS_ENA_CHAN_0: u16 = 0x0001;  /* DCS_ENA_CHAN_0 */
pub const WM9090_DCS_ENA_CHAN_0_MASK: u16 = 0x0001;  /* DCS_ENA_CHAN_0 */
pub const WM9090_DCS_ENA_CHAN_0_SHIFT: u16 = 0;  /* DCS_ENA_CHAN_0 */
pub const WM9090_DCS_ENA_CHAN_0_WIDTH: u16 = 1;  /* DCS_ENA_CHAN_0 */

/*
 * R85 (0x55) - DC Servo 1
 */
pub const WM9090_DCS_SERIES_NO_01_MASK: u16 = 0x0FE0;  /* DCS_SERIES_NO_01 - [11:5] */
pub const WM9090_DCS_SERIES_NO_01_SHIFT: u16 = 5;  /* DCS_SERIES_NO_01 - [11:5] */
pub const WM9090_DCS_SERIES_NO_01_WIDTH: u16 = 7;  /* DCS_SERIES_NO_01 - [11:5] */
pub const WM9090_DCS_TIMER_PERIOD_01_MASK: u16 = 0x000F;  /* DCS_TIMER_PERIOD_01 - [3:0] */
pub const WM9090_DCS_TIMER_PERIOD_01_SHIFT: u16 = 0;  /* DCS_TIMER_PERIOD_01 - [3:0] */
pub const WM9090_DCS_TIMER_PERIOD_01_WIDTH: u16 = 4;  /* DCS_TIMER_PERIOD_01 - [3:0] */

/*
 * R87 (0x57) - DC Servo 3
 */
pub const WM9090_DCS_DAC_WR_VAL_1_MASK: u16 = 0xFF00;  /* DCS_DAC_WR_VAL_1 - [15:8] */
pub const WM9090_DCS_DAC_WR_VAL_1_SHIFT: u16 = 8;  /* DCS_DAC_WR_VAL_1 - [15:8] */
pub const WM9090_DCS_DAC_WR_VAL_1_WIDTH: u16 = 8;  /* DCS_DAC_WR_VAL_1 - [15:8] */
pub const WM9090_DCS_DAC_WR_VAL_0_MASK: u16 = 0x00FF;  /* DCS_DAC_WR_VAL_0 - [7:0] */
pub const WM9090_DCS_DAC_WR_VAL_0_SHIFT: u16 = 0;  /* DCS_DAC_WR_VAL_0 - [7:0] */
pub const WM9090_DCS_DAC_WR_VAL_0_WIDTH: u16 = 8;  /* DCS_DAC_WR_VAL_0 - [7:0] */

/*
 * R88 (0x58) - DC Servo Readback 0
 */
pub const WM9090_DCS_CAL_COMPLETE_MASK: u16 = 0x0300;  /* DCS_CAL_COMPLETE - [9:8] */
pub const WM9090_DCS_CAL_COMPLETE_SHIFT: u16 = 8;  /* DCS_CAL_COMPLETE - [9:8] */
pub const WM9090_DCS_CAL_COMPLETE_WIDTH: u16 = 2;  /* DCS_CAL_COMPLETE - [9:8] */
pub const WM9090_DCS_DAC_WR_COMPLETE_MASK: u16 = 0x0030;  /* DCS_DAC_WR_COMPLETE - [5:4] */
pub const WM9090_DCS_DAC_WR_COMPLETE_SHIFT: u16 = 4;  /* DCS_DAC_WR_COMPLETE - [5:4] */
pub const WM9090_DCS_DAC_WR_COMPLETE_WIDTH: u16 = 2;  /* DCS_DAC_WR_COMPLETE - [5:4] */
pub const WM9090_DCS_STARTUP_COMPLETE_MASK: u16 = 0x0003;  /* DCS_STARTUP_COMPLETE - [1:0] */
pub const WM9090_DCS_STARTUP_COMPLETE_SHIFT: u16 = 0;  /* DCS_STARTUP_COMPLETE - [1:0] */
pub const WM9090_DCS_STARTUP_COMPLETE_WIDTH: u16 = 2;  /* DCS_STARTUP_COMPLETE - [1:0] */

/*
 * R89 (0x59) - DC Servo Readback 1
 */
pub const WM9090_DCS_DAC_WR_VAL_1_RD_MASK: u16 = 0x00FF;  /* DCS_DAC_WR_VAL_1_RD - [7:0] */
pub const WM9090_DCS_DAC_WR_VAL_1_RD_SHIFT: u16 = 0;  /* DCS_DAC_WR_VAL_1_RD - [7:0] */
pub const WM9090_DCS_DAC_WR_VAL_1_RD_WIDTH: u16 = 8;  /* DCS_DAC_WR_VAL_1_RD - [7:0] */

/*
 * R90 (0x5A) - DC Servo Readback 2
 */
pub const WM9090_DCS_DAC_WR_VAL_0_RD_MASK: u16 = 0x00FF;  /* DCS_DAC_WR_VAL_0_RD - [7:0] */
pub const WM9090_DCS_DAC_WR_VAL_0_RD_SHIFT: u16 = 0;  /* DCS_DAC_WR_VAL_0_RD - [7:0] */
pub const WM9090_DCS_DAC_WR_VAL_0_RD_WIDTH: u16 = 8;  /* DCS_DAC_WR_VAL_0_RD - [7:0] */

/*
 * R96 (0x60) - Analogue HP 0
 */
pub const WM9090_HPOUT1L_RMV_SHORT: u16 = 0x0080;  /* HPOUT1L_RMV_SHORT */
pub const WM9090_HPOUT1L_RMV_SHORT_MASK: u16 = 0x0080;  /* HPOUT1L_RMV_SHORT */
pub const WM9090_HPOUT1L_RMV_SHORT_SHIFT: u16 = 7;  /* HPOUT1L_RMV_SHORT */
pub const WM9090_HPOUT1L_RMV_SHORT_WIDTH: u16 = 1;  /* HPOUT1L_RMV_SHORT */
pub const WM9090_HPOUT1L_OUTP: u16 = 0x0040;  /* HPOUT1L_OUTP */
pub const WM9090_HPOUT1L_OUTP_MASK: u16 = 0x0040;  /* HPOUT1L_OUTP */
pub const WM9090_HPOUT1L_OUTP_SHIFT: u16 = 6;  /* HPOUT1L_OUTP */
pub const WM9090_HPOUT1L_OUTP_WIDTH: u16 = 1;  /* HPOUT1L_OUTP */
pub const WM9090_HPOUT1L_DLY: u16 = 0x0020;  /* HPOUT1L_DLY */
pub const WM9090_HPOUT1L_DLY_MASK: u16 = 0x0020;  /* HPOUT1L_DLY */
pub const WM9090_HPOUT1L_DLY_SHIFT: u16 = 5;  /* HPOUT1L_DLY */
pub const WM9090_HPOUT1L_DLY_WIDTH: u16 = 1;  /* HPOUT1L_DLY */
pub const WM9090_HPOUT1R_RMV_SHORT: u16 = 0x0008;  /* HPOUT1R_RMV_SHORT */
pub const WM9090_HPOUT1R_RMV_SHORT_MASK: u16 = 0x0008;  /* HPOUT1R_RMV_SHORT */
pub const WM9090_HPOUT1R_RMV_SHORT_SHIFT: u16 = 3;  /* HPOUT1R_RMV_SHORT */
pub const WM9090_HPOUT1R_RMV_SHORT_WIDTH: u16 = 1;  /* HPOUT1R_RMV_SHORT */
pub const WM9090_HPOUT1R_OUTP: u16 = 0x0004;  /* HPOUT1R_OUTP */
pub const WM9090_HPOUT1R_OUTP_MASK: u16 = 0x0004;  /* HPOUT1R_OUTP */
pub const WM9090_HPOUT1R_OUTP_SHIFT: u16 = 2;  /* HPOUT1R_OUTP */
pub const WM9090_HPOUT1R_OUTP_WIDTH: u16 = 1;  /* HPOUT1R_OUTP */
pub const WM9090_HPOUT1R_DLY: u16 = 0x0002;  /* HPOUT1R_DLY */
pub const WM9090_HPOUT1R_DLY_MASK: u16 = 0x0002;  /* HPOUT1R_DLY */
pub const WM9090_HPOUT1R_DLY_SHIFT: u16 = 1;  /* HPOUT1R_DLY */
pub const WM9090_HPOUT1R_DLY_WIDTH: u16 = 1;  /* HPOUT1R_DLY */

/*
 * R98 (0x62) - AGC Control 0
 */
pub const WM9090_AGC_CLIP_ENA: u16 = 0x8000;  /* AGC_CLIP_ENA */
pub const WM9090_AGC_CLIP_ENA_MASK: u16 = 0x8000;  /* AGC_CLIP_ENA */
pub const WM9090_AGC_CLIP_ENA_SHIFT: u16 = 15;  /* AGC_CLIP_ENA */
pub const WM9090_AGC_CLIP_ENA_WIDTH: u16 = 1;  /* AGC_CLIP_ENA */
pub const WM9090_AGC_CLIP_THR_MASK: u16 = 0x0F00;  /* AGC_CLIP_THR - [11:8] */
pub const WM9090_AGC_CLIP_THR_SHIFT: u16 = 8;  /* AGC_CLIP_THR - [11:8] */
pub const WM9090_AGC_CLIP_THR_WIDTH: u16 = 4;  /* AGC_CLIP_THR - [11:8] */
pub const WM9090_AGC_CLIP_ATK_MASK: u16 = 0x0070;  /* AGC_CLIP_ATK - [6:4] */
pub const WM9090_AGC_CLIP_ATK_SHIFT: u16 = 4;  /* AGC_CLIP_ATK - [6:4] */
pub const WM9090_AGC_CLIP_ATK_WIDTH: u16 = 3;  /* AGC_CLIP_ATK - [6:4] */
pub const WM9090_AGC_CLIP_DCY_MASK: u16 = 0x0007;  /* AGC_CLIP_DCY - [2:0] */
pub const WM9090_AGC_CLIP_DCY_SHIFT: u16 = 0;  /* AGC_CLIP_DCY - [2:0] */
pub const WM9090_AGC_CLIP_DCY_WIDTH: u16 = 3;  /* AGC_CLIP_DCY - [2:0] */

/*
 * R99 (0x63) - AGC Control 1
 */
pub const WM9090_AGC_PWR_ENA: u16 = 0x8000;  /* AGC_PWR_ENA */
pub const WM9090_AGC_PWR_ENA_MASK: u16 = 0x8000;  /* AGC_PWR_ENA */
pub const WM9090_AGC_PWR_ENA_SHIFT: u16 = 15;  /* AGC_PWR_ENA */
pub const WM9090_AGC_PWR_ENA_WIDTH: u16 = 1;  /* AGC_PWR_ENA */
pub const WM9090_AGC_PWR_AVG: u16 = 0x1000;  /* AGC_PWR_AVG */
pub const WM9090_AGC_PWR_AVG_MASK: u16 = 0x1000;  /* AGC_PWR_AVG */
pub const WM9090_AGC_PWR_AVG_SHIFT: u16 = 12;  /* AGC_PWR_AVG */
pub const WM9090_AGC_PWR_AVG_WIDTH: u16 = 1;  /* AGC_PWR_AVG */
pub const WM9090_AGC_PWR_THR_MASK: u16 = 0x0F00;  /* AGC_PWR_THR - [11:8] */
pub const WM9090_AGC_PWR_THR_SHIFT: u16 = 8;  /* AGC_PWR_THR - [11:8] */
pub const WM9090_AGC_PWR_THR_WIDTH: u16 = 4;  /* AGC_PWR_THR - [11:8] */
pub const WM9090_AGC_PWR_ATK_MASK: u16 = 0x0070;  /* AGC_PWR_ATK - [6:4] */
pub const WM9090_AGC_PWR_ATK_SHIFT: u16 = 4;  /* AGC_PWR_ATK - [6:4] */
pub const WM9090_AGC_PWR_ATK_WIDTH: u16 = 3;  /* AGC_PWR_ATK - [6:4] */
pub const WM9090_AGC_PWR_DCY_MASK: u16 = 0x0007;  /* AGC_PWR_DCY - [2:0] */
pub const WM9090_AGC_PWR_DCY_SHIFT: u16 = 0;  /* AGC_PWR_DCY - [2:0] */
pub const WM9090_AGC_PWR_DCY_WIDTH: u16 = 3;  /* AGC_PWR_DCY - [2:0] */

/*
 * R100 (0x64) - AGC Control 2
 */
pub const WM9090_AGC_RAMP: u16 = 0x0100;  /* AGC_RAMP */
pub const WM9090_AGC_RAMP_MASK: u16 = 0x0100;  /* AGC_RAMP */
pub const WM9090_AGC_RAMP_SHIFT: u16 = 8;  /* AGC_RAMP */
pub const WM9090_AGC_RAMP_WIDTH: u16 = 1;  /* AGC_RAMP */
pub const WM9090_AGC_MINGAIN_MASK: u16 = 0x003F;  /* AGC_MINGAIN - [5:0] */
pub const WM9090_AGC_MINGAIN_SHIFT: u16 = 0;  /* AGC_MINGAIN - [5:0] */
pub const WM9090_AGC_MINGAIN_WIDTH: u16 = 6;  /* AGC_MINGAIN - [5:0] */


// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
