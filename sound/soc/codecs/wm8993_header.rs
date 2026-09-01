/* SPDX-License-Identifier: GPL-2.0 */

pub const WM8993_SYSCLK_MCLK: u32 = 1;
pub const WM8993_SYSCLK_FLL: u32 = 2;

pub const WM8993_FLL_MCLK: u32 = 1;
pub const WM8993_FLL_BCLK: u32 = 2;
pub const WM8993_FLL_LRCLK: u32 = 3;

/*
 * Register values.
 */
pub const WM8993_SOFTWARE_RESET: u32 = 0x00;
pub const WM8993_POWER_MANAGEMENT_1: u32 = 0x01;
pub const WM8993_POWER_MANAGEMENT_2: u32 = 0x02;
pub const WM8993_POWER_MANAGEMENT_3: u32 = 0x03;
pub const WM8993_AUDIO_INTERFACE_1: u32 = 0x04;
pub const WM8993_AUDIO_INTERFACE_2: u32 = 0x05;
pub const WM8993_CLOCKING_1: u32 = 0x06;
pub const WM8993_CLOCKING_2: u32 = 0x07;
pub const WM8993_AUDIO_INTERFACE_3: u32 = 0x08;
pub const WM8993_AUDIO_INTERFACE_4: u32 = 0x09;
pub const WM8993_DAC_CTRL: u32 = 0x0A;
pub const WM8993_LEFT_DAC_DIGITAL_VOLUME: u32 = 0x0B;
pub const WM8993_RIGHT_DAC_DIGITAL_VOLUME: u32 = 0x0C;
pub const WM8993_DIGITAL_SIDE_TONE: u32 = 0x0D;
pub const WM8993_ADC_CTRL: u32 = 0x0E;
pub const WM8993_LEFT_ADC_DIGITAL_VOLUME: u32 = 0x0F;
pub const WM8993_RIGHT_ADC_DIGITAL_VOLUME: u32 = 0x10;
pub const WM8993_GPIO_CTRL_1: u32 = 0x12;
pub const WM8993_GPIO1: u32 = 0x13;
pub const WM8993_IRQ_DEBOUNCE: u32 = 0x14;
pub const WM8993_INPUTS_CLAMP_REG: u32 = 0x15;
pub const WM8993_GPIOCTRL_2: u32 = 0x16;
pub const WM8993_GPIO_POL: u32 = 0x17;
pub const WM8993_LEFT_LINE_INPUT_1_2_VOLUME: u32 = 0x18;
pub const WM8993_LEFT_LINE_INPUT_3_4_VOLUME: u32 = 0x19;
pub const WM8993_RIGHT_LINE_INPUT_1_2_VOLUME: u32 = 0x1A;
pub const WM8993_RIGHT_LINE_INPUT_3_4_VOLUME: u32 = 0x1B;
pub const WM8993_LEFT_OUTPUT_VOLUME: u32 = 0x1C;
pub const WM8993_RIGHT_OUTPUT_VOLUME: u32 = 0x1D;
pub const WM8993_LINE_OUTPUTS_VOLUME: u32 = 0x1E;
pub const WM8993_HPOUT2_VOLUME: u32 = 0x1F;
pub const WM8993_LEFT_OPGA_VOLUME: u32 = 0x20;
pub const WM8993_RIGHT_OPGA_VOLUME: u32 = 0x21;
pub const WM8993_SPKMIXL_ATTENUATION: u32 = 0x22;
pub const WM8993_SPKMIXR_ATTENUATION: u32 = 0x23;
pub const WM8993_SPKOUT_MIXERS: u32 = 0x24;
pub const WM8993_SPKOUT_BOOST: u32 = 0x25;
pub const WM8993_SPEAKER_VOLUME_LEFT: u32 = 0x26;
pub const WM8993_SPEAKER_VOLUME_RIGHT: u32 = 0x27;
pub const WM8993_INPUT_MIXER2: u32 = 0x28;
pub const WM8993_INPUT_MIXER3: u32 = 0x29;
pub const WM8993_INPUT_MIXER4: u32 = 0x2A;
pub const WM8993_INPUT_MIXER5: u32 = 0x2B;
pub const WM8993_INPUT_MIXER6: u32 = 0x2C;
pub const WM8993_OUTPUT_MIXER1: u32 = 0x2D;
pub const WM8993_OUTPUT_MIXER2: u32 = 0x2E;
pub const WM8993_OUTPUT_MIXER3: u32 = 0x2F;
pub const WM8993_OUTPUT_MIXER4: u32 = 0x30;
pub const WM8993_OUTPUT_MIXER5: u32 = 0x31;
pub const WM8993_OUTPUT_MIXER6: u32 = 0x32;
pub const WM8993_HPOUT2_MIXER: u32 = 0x33;
pub const WM8993_LINE_MIXER1: u32 = 0x34;
pub const WM8993_LINE_MIXER2: u32 = 0x35;
pub const WM8993_SPEAKER_MIXER: u32 = 0x36;
pub const WM8993_ADDITIONAL_CONTROL: u32 = 0x37;
pub const WM8993_ANTIPOP1: u32 = 0x38;
pub const WM8993_ANTIPOP2: u32 = 0x39;
pub const WM8993_MICBIAS: u32 = 0x3A;
pub const WM8993_FLL_CONTROL_1: u32 = 0x3C;
pub const WM8993_FLL_CONTROL_2: u32 = 0x3D;
pub const WM8993_FLL_CONTROL_3: u32 = 0x3E;
pub const WM8993_FLL_CONTROL_4: u32 = 0x3F;
pub const WM8993_FLL_CONTROL_5: u32 = 0x40;
pub const WM8993_CLOCKING_3: u32 = 0x41;
pub const WM8993_CLOCKING_4: u32 = 0x42;
pub const WM8993_MW_SLAVE_CONTROL: u32 = 0x43;
pub const WM8993_BUS_CONTROL_1: u32 = 0x45;
pub const WM8993_WRITE_SEQUENCER_0: u32 = 0x46;
pub const WM8993_WRITE_SEQUENCER_1: u32 = 0x47;
pub const WM8993_WRITE_SEQUENCER_2: u32 = 0x48;
pub const WM8993_WRITE_SEQUENCER_3: u32 = 0x49;
pub const WM8993_WRITE_SEQUENCER_4: u32 = 0x4A;
pub const WM8993_WRITE_SEQUENCER_5: u32 = 0x4B;
pub const WM8993_CHARGE_PUMP_1: u32 = 0x4C;
pub const WM8993_CLASS_W_0: u32 = 0x51;
pub const WM8993_DC_SERVO_0: u32 = 0x54;
pub const WM8993_DC_SERVO_1: u32 = 0x55;
pub const WM8993_DC_SERVO_3: u32 = 0x57;
pub const WM8993_DC_SERVO_READBACK_0: u32 = 0x58;
pub const WM8993_DC_SERVO_READBACK_1: u32 = 0x59;
pub const WM8993_DC_SERVO_READBACK_2: u32 = 0x5A;
pub const WM8993_ANALOGUE_HP_0: u32 = 0x60;
pub const WM8993_EQ1: u32 = 0x62;
pub const WM8993_EQ2: u32 = 0x63;
pub const WM8993_EQ3: u32 = 0x64;
pub const WM8993_EQ4: u32 = 0x65;
pub const WM8993_EQ5: u32 = 0x66;
pub const WM8993_EQ6: u32 = 0x67;
pub const WM8993_EQ7: u32 = 0x68;
pub const WM8993_EQ8: u32 = 0x69;
pub const WM8993_EQ9: u32 = 0x6A;
pub const WM8993_EQ10: u32 = 0x6B;
pub const WM8993_EQ11: u32 = 0x6C;
pub const WM8993_EQ12: u32 = 0x6D;
pub const WM8993_EQ13: u32 = 0x6E;
pub const WM8993_EQ14: u32 = 0x6F;
pub const WM8993_EQ15: u32 = 0x70;
pub const WM8993_EQ16: u32 = 0x71;
pub const WM8993_EQ17: u32 = 0x72;
pub const WM8993_EQ18: u32 = 0x73;
pub const WM8993_EQ19: u32 = 0x74;
pub const WM8993_EQ20: u32 = 0x75;
pub const WM8993_EQ21: u32 = 0x76;
pub const WM8993_EQ22: u32 = 0x77;
pub const WM8993_EQ23: u32 = 0x78;
pub const WM8993_EQ24: u32 = 0x79;
pub const WM8993_DIGITAL_PULLS: u32 = 0x7A;
pub const WM8993_DRC_CONTROL_1: u32 = 0x7B;
pub const WM8993_DRC_CONTROL_2: u32 = 0x7C;
pub const WM8993_DRC_CONTROL_3: u32 = 0x7D;
pub const WM8993_DRC_CONTROL_4: u32 = 0x7E;

pub const WM8993_REGISTER_COUNT: u32 = 0x7F;
pub const WM8993_MAX_REGISTER: u32 = 0x7E;

/*
 * Field Definitions.
 */

/*
 * R0 (0x00) - Software Reset
 */
pub const WM8993_SW_RESET_MASK: u32 = 0xFFFF;   /* SW_RESET - [15:0] */
pub const WM8993_SW_RESET_SHIFT: u32 = 0;   /* SW_RESET - [15:0] */
pub const WM8993_SW_RESET_WIDTH: u32 = 16;   /* SW_RESET - [15:0] */

/*
 * R1 (0x01) - Power Management (1)
 */
pub const WM8993_SPKOUTR_ENA: u32 = 0x2000;   /* SPKOUTR_ENA */
pub const WM8993_SPKOUTR_ENA_MASK: u32 = 0x2000;   /* SPKOUTR_ENA */
pub const WM8993_SPKOUTR_ENA_SHIFT: u32 = 13;   /* SPKOUTR_ENA */
pub const WM8993_SPKOUTR_ENA_WIDTH: u32 = 1;   /* SPKOUTR_ENA */
pub const WM8993_SPKOUTL_ENA: u32 = 0x1000;   /* SPKOUTL_ENA */
pub const WM8993_SPKOUTL_ENA_MASK: u32 = 0x1000;   /* SPKOUTL_ENA */
pub const WM8993_SPKOUTL_ENA_SHIFT: u32 = 12;   /* SPKOUTL_ENA */
pub const WM8993_SPKOUTL_ENA_WIDTH: u32 = 1;   /* SPKOUTL_ENA */
pub const WM8993_HPOUT2_ENA: u32 = 0x0800;   /* HPOUT2_ENA */
pub const WM8993_HPOUT2_ENA_MASK: u32 = 0x0800;   /* HPOUT2_ENA */
pub const WM8993_HPOUT2_ENA_SHIFT: u32 = 11;   /* HPOUT2_ENA */
pub const WM8993_HPOUT2_ENA_WIDTH: u32 = 1;   /* HPOUT2_ENA */
pub const WM8993_HPOUT1L_ENA: u32 = 0x0200;   /* HPOUT1L_ENA */
pub const WM8993_HPOUT1L_ENA_MASK: u32 = 0x0200;   /* HPOUT1L_ENA */
pub const WM8993_HPOUT1L_ENA_SHIFT: u32 = 9;   /* HPOUT1L_ENA */
pub const WM8993_HPOUT1L_ENA_WIDTH: u32 = 1;   /* HPOUT1L_ENA */
pub const WM8993_HPOUT1R_ENA: u32 = 0x0100;   /* HPOUT1R_ENA */
pub const WM8993_HPOUT1R_ENA_MASK: u32 = 0x0100;   /* HPOUT1R_ENA */
pub const WM8993_HPOUT1R_ENA_SHIFT: u32 = 8;   /* HPOUT1R_ENA */
pub const WM8993_HPOUT1R_ENA_WIDTH: u32 = 1;   /* HPOUT1R_ENA */
pub const WM8993_MICB2_ENA: u32 = 0x0020;   /* MICB2_ENA */
pub const WM8993_MICB2_ENA_MASK: u32 = 0x0020;   /* MICB2_ENA */
pub const WM8993_MICB2_ENA_SHIFT: u32 = 5;   /* MICB2_ENA */
pub const WM8993_MICB2_ENA_WIDTH: u32 = 1;   /* MICB2_ENA */
pub const WM8993_MICB1_ENA: u32 = 0x0010;   /* MICB1_ENA */
pub const WM8993_MICB1_ENA_MASK: u32 = 0x0010;   /* MICB1_ENA */
pub const WM8993_MICB1_ENA_SHIFT: u32 = 4;   /* MICB1_ENA */
pub const WM8993_MICB1_ENA_WIDTH: u32 = 1;   /* MICB1_ENA */
pub const WM8993_VMID_SEL_MASK: u32 = 0x0006;   /* VMID_SEL - [2:1] */
pub const WM8993_VMID_SEL_SHIFT: u32 = 1;   /* VMID_SEL - [2:1] */
pub const WM8993_VMID_SEL_WIDTH: u32 = 2;   /* VMID_SEL - [2:1] */
pub const WM8993_BIAS_ENA: u32 = 0x0001;   /* BIAS_ENA */
pub const WM8993_BIAS_ENA_MASK: u32 = 0x0001;   /* BIAS_ENA */
pub const WM8993_BIAS_ENA_SHIFT: u32 = 0;   /* BIAS_ENA */
pub const WM8993_BIAS_ENA_WIDTH: u32 = 1;   /* BIAS_ENA */

/*
 * R2 (0x02) - Power Management (2)
 */
pub const WM8993_TSHUT_ENA: u32 = 0x4000;   /* TSHUT_ENA */
pub const WM8993_TSHUT_ENA_MASK: u32 = 0x4000;   /* TSHUT_ENA */
pub const WM8993_TSHUT_ENA_SHIFT: u32 = 14;   /* TSHUT_ENA */
pub const WM8993_TSHUT_ENA_WIDTH: u32 = 1;   /* TSHUT_ENA */
pub const WM8993_TSHUT_OPDIS: u32 = 0x2000;   /* TSHUT_OPDIS */
pub const WM8993_TSHUT_OPDIS_MASK: u32 = 0x2000;   /* TSHUT_OPDIS */
pub const WM8993_TSHUT_OPDIS_SHIFT: u32 = 13;   /* TSHUT_OPDIS */
pub const WM8993_TSHUT_OPDIS_WIDTH: u32 = 1;   /* TSHUT_OPDIS */
pub const WM8993_OPCLK_ENA: u32 = 0x0800;   /* OPCLK_ENA */
pub const WM8993_OPCLK_ENA_MASK: u32 = 0x0800;   /* OPCLK_ENA */
pub const WM8993_OPCLK_ENA_SHIFT: u32 = 11;   /* OPCLK_ENA */
pub const WM8993_OPCLK_ENA_WIDTH: u32 = 1;   /* OPCLK_ENA */
pub const WM8993_MIXINL_ENA: u32 = 0x0200;   /* MIXINL_ENA */
pub const WM8993_MIXINL_ENA_MASK: u32 = 0x0200;   /* MIXINL_ENA */
pub const WM8993_MIXINL_ENA_SHIFT: u32 = 9;   /* MIXINL_ENA */
pub const WM8993_MIXINL_ENA_WIDTH: u32 = 1;   /* MIXINL_ENA */
pub const WM8993_MIXINR_ENA: u32 = 0x0100;   /* MIXINR_ENA */
pub const WM8993_MIXINR_ENA_MASK: u32 = 0x0100;   /* MIXINR_ENA */
pub const WM8993_MIXINR_ENA_SHIFT: u32 = 8;   /* MIXINR_ENA */
pub const WM8993_MIXINR_ENA_WIDTH: u32 = 1;   /* MIXINR_ENA */
pub const WM8993_IN2L_ENA: u32 = 0x0080;   /* IN2L_ENA */
pub const WM8993_IN2L_ENA_MASK: u32 = 0x0080;   /* IN2L_ENA */
pub const WM8993_IN2L_ENA_SHIFT: u32 = 7;   /* IN2L_ENA */
pub const WM8993_IN2L_ENA_WIDTH: u32 = 1;   /* IN2L_ENA */
pub const WM8993_IN1L_ENA: u32 = 0x0040;   /* IN1L_ENA */
pub const WM8993_IN1L_ENA_MASK: u32 = 0x0040;   /* IN1L_ENA */
pub const WM8993_IN1L_ENA_SHIFT: u32 = 6;   /* IN1L_ENA */
pub const WM8993_IN1L_ENA_WIDTH: u32 = 1;   /* IN1L_ENA */
pub const WM8993_IN2R_ENA: u32 = 0x0020;   /* IN2R_ENA */
pub const WM8993_IN2R_ENA_MASK: u32 = 0x0020;   /* IN2R_ENA */
pub const WM8993_IN2R_ENA_SHIFT: u32 = 5;   /* IN2R_ENA */
pub const WM8993_IN2R_ENA_WIDTH: u32 = 1;   /* IN2R_ENA */
pub const WM8993_IN1R_ENA: u32 = 0x0010;   /* IN1R_ENA */
pub const WM8993_IN1R_ENA_MASK: u32 = 0x0010;   /* IN1R_ENA */
pub const WM8993_IN1R_ENA_SHIFT: u32 = 4;   /* IN1R_ENA */
pub const WM8993_IN1R_ENA_WIDTH: u32 = 1;   /* IN1R_ENA */
pub const WM8993_ADCL_ENA: u32 = 0x0002;   /* ADCL_ENA */
pub const WM8993_ADCL_ENA_MASK: u32 = 0x0002;   /* ADCL_ENA */
pub const WM8993_ADCL_ENA_SHIFT: u32 = 1;   /* ADCL_ENA */
pub const WM8993_ADCL_ENA_WIDTH: u32 = 1;   /* ADCL_ENA */
pub const WM8993_ADCR_ENA: u32 = 0x0001;   /* ADCR_ENA */
pub const WM8993_ADCR_ENA_MASK: u32 = 0x0001;   /* ADCR_ENA */
pub const WM8993_ADCR_ENA_SHIFT: u32 = 0;   /* ADCR_ENA */
pub const WM8993_ADCR_ENA_WIDTH: u32 = 1;   /* ADCR_ENA */

/*
 * R3 (0x03) - Power Management (3)
 */
pub const WM8993_LINEOUT1N_ENA: u32 = 0x2000;   /* LINEOUT1N_ENA */
pub const WM8993_LINEOUT1N_ENA_MASK: u32 = 0x2000;   /* LINEOUT1N_ENA */
pub const WM8993_LINEOUT1N_ENA_SHIFT: u32 = 13;   /* LINEOUT1N_ENA */
pub const WM8993_LINEOUT1N_ENA_WIDTH: u32 = 1;   /* LINEOUT1N_ENA */
pub const WM8993_LINEOUT1P_ENA: u32 = 0x1000;   /* LINEOUT1P_ENA */
pub const WM8993_LINEOUT1P_ENA_MASK: u32 = 0x1000;   /* LINEOUT1P_ENA */
pub const WM8993_LINEOUT1P_ENA_SHIFT: u32 = 12;   /* LINEOUT1P_ENA */
pub const WM8993_LINEOUT1P_ENA_WIDTH: u32 = 1;   /* LINEOUT1P_ENA */
pub const WM8993_LINEOUT2N_ENA: u32 = 0x0800;   /* LINEOUT2N_ENA */
pub const WM8993_LINEOUT2N_ENA_MASK: u32 = 0x0800;   /* LINEOUT2N_ENA */
pub const WM8993_LINEOUT2N_ENA_SHIFT: u32 = 11;   /* LINEOUT2N_ENA */
pub const WM8993_LINEOUT2N_ENA_WIDTH: u32 = 1;   /* LINEOUT2N_ENA */
pub const WM8993_LINEOUT2P_ENA: u32 = 0x0400;   /* LINEOUT2P_ENA */
pub const WM8993_LINEOUT2P_ENA_MASK: u32 = 0x0400;   /* LINEOUT2P_ENA */
pub const WM8993_LINEOUT2P_ENA_SHIFT: u32 = 10;   /* LINEOUT2P_ENA */
pub const WM8993_LINEOUT2P_ENA_WIDTH: u32 = 1;   /* LINEOUT2P_ENA */
pub const WM8993_SPKRVOL_ENA: u32 = 0x0200;   /* SPKRVOL_ENA */
pub const WM8993_SPKRVOL_ENA_MASK: u32 = 0x0200;   /* SPKRVOL_ENA */
pub const WM8993_SPKRVOL_ENA_SHIFT: u32 = 9;   /* SPKRVOL_ENA */
pub const WM8993_SPKRVOL_ENA_WIDTH: u32 = 1;   /* SPKRVOL_ENA */
pub const WM8993_SPKLVOL_ENA: u32 = 0x0100;   /* SPKLVOL_ENA */
pub const WM8993_SPKLVOL_ENA_MASK: u32 = 0x0100;   /* SPKLVOL_ENA */
pub const WM8993_SPKLVOL_ENA_SHIFT: u32 = 8;   /* SPKLVOL_ENA */
pub const WM8993_SPKLVOL_ENA_WIDTH: u32 = 1;   /* SPKLVOL_ENA */
pub const WM8993_MIXOUTLVOL_ENA: u32 = 0x0080;   /* MIXOUTLVOL_ENA */
pub const WM8993_MIXOUTLVOL_ENA_MASK: u32 = 0x0080;   /* MIXOUTLVOL_ENA */
pub const WM8993_MIXOUTLVOL_ENA_SHIFT: u32 = 7;   /* MIXOUTLVOL_ENA */
pub const WM8993_MIXOUTLVOL_ENA_WIDTH: u32 = 1;   /* MIXOUTLVOL_ENA */
pub const WM8993_MIXOUTRVOL_ENA: u32 = 0x0040;   /* MIXOUTRVOL_ENA */
pub const WM8993_MIXOUTRVOL_ENA_MASK: u32 = 0x0040;   /* MIXOUTRVOL_ENA */
pub const WM8993_MIXOUTRVOL_ENA_SHIFT: u32 = 6;   /* MIXOUTRVOL_ENA */
pub const WM8993_MIXOUTRVOL_ENA_WIDTH: u32 = 1;   /* MIXOUTRVOL_ENA */
pub const WM8993_MIXOUTL_ENA: u32 = 0x0020;   /* MIXOUTL_ENA */
pub const WM8993_MIXOUTL_ENA_MASK: u32 = 0x0020;   /* MIXOUTL_ENA */
pub const WM8993_MIXOUTL_ENA_SHIFT: u32 = 5;   /* MIXOUTL_ENA */
pub const WM8993_MIXOUTL_ENA_WIDTH: u32 = 1;   /* MIXOUTL_ENA */
pub const WM8993_MIXOUTR_ENA: u32 = 0x0010;   /* MIXOUTR_ENA */
pub const WM8993_MIXOUTR_ENA_MASK: u32 = 0x0010;   /* MIXOUTR_ENA */
pub const WM8993_MIXOUTR_ENA_SHIFT: u32 = 4;   /* MIXOUTR_ENA */
pub const WM8993_MIXOUTR_ENA_WIDTH: u32 = 1;   /* MIXOUTR_ENA */
pub const WM8993_DACL_ENA: u32 = 0x0002;   /* DACL_ENA */
pub const WM8993_DACL_ENA_MASK: u32 = 0x0002;   /* DACL_ENA */
pub const WM8993_DACL_ENA_SHIFT: u32 = 1;   /* DACL_ENA */
pub const WM8993_DACL_ENA_WIDTH: u32 = 1;   /* DACL_ENA */
pub const WM8993_DACR_ENA: u32 = 0x0001;   /* DACR_ENA */
pub const WM8993_DACR_ENA_MASK: u32 = 0x0001;   /* DACR_ENA */
pub const WM8993_DACR_ENA_SHIFT: u32 = 0;   /* DACR_ENA */
pub const WM8993_DACR_ENA_WIDTH: u32 = 1;   /* DACR_ENA */

/*
 * R4 (0x04) - Audio Interface (1)
 */
pub const WM8993_AIFADCL_SRC: u32 = 0x8000;   /* AIFADCL_SRC */
pub const WM8993_AIFADCL_SRC_MASK: u32 = 0x8000;   /* AIFADCL_SRC */
pub const WM8993_AIFADCL_SRC_SHIFT: u32 = 15;   /* AIFADCL_SRC */
pub const WM8993_AIFADCL_SRC_WIDTH: u32 = 1;   /* AIFADCL_SRC */
pub const WM8993_AIFADCR_SRC: u32 = 0x4000;   /* AIFADCR_SRC */
pub const WM8993_AIFADCR_SRC_MASK: u32 = 0x4000;   /* AIFADCR_SRC */
pub const WM8993_AIFADCR_SRC_SHIFT: u32 = 14;   /* AIFADCR_SRC */
pub const WM8993_AIFADCR_SRC_WIDTH: u32 = 1;   /* AIFADCR_SRC */
pub const WM8993_AIFADC_TDM: u32 = 0x2000;   /* AIFADC_TDM */
pub const WM8993_AIFADC_TDM_MASK: u32 = 0x2000;   /* AIFADC_TDM */
pub const WM8993_AIFADC_TDM_SHIFT: u32 = 13;   /* AIFADC_TDM */
pub const WM8993_AIFADC_TDM_WIDTH: u32 = 1;   /* AIFADC_TDM */
pub const WM8993_AIFADC_TDM_CHAN: u32 = 0x1000;   /* AIFADC_TDM_CHAN */
pub const WM8993_AIFADC_TDM_CHAN_MASK: u32 = 0x1000;   /* AIFADC_TDM_CHAN */
pub const WM8993_AIFADC_TDM_CHAN_SHIFT: u32 = 12;   /* AIFADC_TDM_CHAN */
pub const WM8993_AIFADC_TDM_CHAN_WIDTH: u32 = 1;   /* AIFADC_TDM_CHAN */
pub const WM8993_BCLK_DIR: u32 = 0x0200;   /* BCLK_DIR */
pub const WM8993_BCLK_DIR_MASK: u32 = 0x0200;   /* BCLK_DIR */
pub const WM8993_BCLK_DIR_SHIFT: u32 = 9;   /* BCLK_DIR */
pub const WM8993_BCLK_DIR_WIDTH: u32 = 1;   /* BCLK_DIR */
pub const WM8993_AIF_BCLK_INV: u32 = 0x0100;   /* AIF_BCLK_INV */
pub const WM8993_AIF_BCLK_INV_MASK: u32 = 0x0100;   /* AIF_BCLK_INV */
pub const WM8993_AIF_BCLK_INV_SHIFT: u32 = 8;   /* AIF_BCLK_INV */
pub const WM8993_AIF_BCLK_INV_WIDTH: u32 = 1;   /* AIF_BCLK_INV */
pub const WM8993_AIF_LRCLK_INV: u32 = 0x0080;   /* AIF_LRCLK_INV */
pub const WM8993_AIF_LRCLK_INV_MASK: u32 = 0x0080;   /* AIF_LRCLK_INV */
pub const WM8993_AIF_LRCLK_INV_SHIFT: u32 = 7;   /* AIF_LRCLK_INV */
pub const WM8993_AIF_LRCLK_INV_WIDTH: u32 = 1;   /* AIF_LRCLK_INV */
pub const WM8993_AIF_WL_MASK: u32 = 0x0060;   /* AIF_WL - [6:5] */
pub const WM8993_AIF_WL_SHIFT: u32 = 5;   /* AIF_WL - [6:5] */
pub const WM8993_AIF_WL_WIDTH: u32 = 2;   /* AIF_WL - [6:5] */
pub const WM8993_AIF_FMT_MASK: u32 = 0x0018;   /* AIF_FMT - [4:3] */
pub const WM8993_AIF_FMT_SHIFT: u32 = 3;   /* AIF_FMT - [4:3] */
pub const WM8993_AIF_FMT_WIDTH: u32 = 2;   /* AIF_FMT - [4:3] */

/*
 * R5 (0x05) - Audio Interface (2)
 */
pub const WM8993_AIFDACL_SRC: u32 = 0x8000;   /* AIFDACL_SRC */
pub const WM8993_AIFDACL_SRC_MASK: u32 = 0x8000;   /* AIFDACL_SRC */
pub const WM8993_AIFDACL_SRC_SHIFT: u32 = 15;   /* AIFDACL_SRC */
pub const WM8993_AIFDACL_SRC_WIDTH: u32 = 1;   /* AIFDACL_SRC */
pub const WM8993_AIFDACR_SRC: u32 = 0x4000;   /* AIFDACR_SRC */
pub const WM8993_AIFDACR_SRC_MASK: u32 = 0x4000;   /* AIFDACR_SRC */
pub const WM8993_AIFDACR_SRC_SHIFT: u32 = 14;   /* AIFDACR_SRC */
pub const WM8993_AIFDACR_SRC_WIDTH: u32 = 1;   /* AIFDACR_SRC */
pub const WM8993_AIFDAC_TDM: u32 = 0x2000;   /* AIFDAC_TDM */
pub const WM8993_AIFDAC_TDM_MASK: u32 = 0x2000;   /* AIFDAC_TDM */
pub const WM8993_AIFDAC_TDM_SHIFT: u32 = 13;   /* AIFDAC_TDM */
pub const WM8993_AIFDAC_TDM_WIDTH: u32 = 1;   /* AIFDAC_TDM */
pub const WM8993_AIFDAC_TDM_CHAN: u32 = 0x1000;   /* AIFDAC_TDM_CHAN */
pub const WM8993_AIFDAC_TDM_CHAN_MASK: u32 = 0x1000;   /* AIFDAC_TDM_CHAN */
pub const WM8993_AIFDAC_TDM_CHAN_SHIFT: u32 = 12;   /* AIFDAC_TDM_CHAN */
pub const WM8993_AIFDAC_TDM_CHAN_WIDTH: u32 = 1;   /* AIFDAC_TDM_CHAN */
pub const WM8993_DAC_BOOST_MASK: u32 = 0x0C00;   /* DAC_BOOST - [11:10] */
pub const WM8993_DAC_BOOST_SHIFT: u32 = 10;   /* DAC_BOOST - [11:10] */
pub const WM8993_DAC_BOOST_WIDTH: u32 = 2;   /* DAC_BOOST - [11:10] */
pub const WM8993_DAC_COMP: u32 = 0x0010;   /* DAC_COMP */
pub const WM8993_DAC_COMP_MASK: u32 = 0x0010;   /* DAC_COMP */
pub const WM8993_DAC_COMP_SHIFT: u32 = 4;   /* DAC_COMP */
pub const WM8993_DAC_COMP_WIDTH: u32 = 1;   /* DAC_COMP */
pub const WM8993_DAC_COMPMODE: u32 = 0x0008;   /* DAC_COMPMODE */
pub const WM8993_DAC_COMPMODE_MASK: u32 = 0x0008;   /* DAC_COMPMODE */
pub const WM8993_DAC_COMPMODE_SHIFT: u32 = 3;   /* DAC_COMPMODE */
pub const WM8993_DAC_COMPMODE_WIDTH: u32 = 1;   /* DAC_COMPMODE */
pub const WM8993_ADC_COMP: u32 = 0x0004;   /* ADC_COMP */
pub const WM8993_ADC_COMP_MASK: u32 = 0x0004;   /* ADC_COMP */
pub const WM8993_ADC_COMP_SHIFT: u32 = 2;   /* ADC_COMP */
pub const WM8993_ADC_COMP_WIDTH: u32 = 1;   /* ADC_COMP */
pub const WM8993_ADC_COMPMODE: u32 = 0x0002;   /* ADC_COMPMODE */
pub const WM8993_ADC_COMPMODE_MASK: u32 = 0x0002;   /* ADC_COMPMODE */
pub const WM8993_ADC_COMPMODE_SHIFT: u32 = 1;   /* ADC_COMPMODE */
pub const WM8993_ADC_COMPMODE_WIDTH: u32 = 1;   /* ADC_COMPMODE */
pub const WM8993_LOOPBACK: u32 = 0x0001;   /* LOOPBACK */
pub const WM8993_LOOPBACK_MASK: u32 = 0x0001;   /* LOOPBACK */
pub const WM8993_LOOPBACK_SHIFT: u32 = 0;   /* LOOPBACK */
pub const WM8993_LOOPBACK_WIDTH: u32 = 1;   /* LOOPBACK */

/*
 * R6 (0x06) - Clocking 1
 */
pub const WM8993_TOCLK_RATE: u32 = 0x8000;   /* TOCLK_RATE */
pub const WM8993_TOCLK_RATE_MASK: u32 = 0x8000;   /* TOCLK_RATE */
pub const WM8993_TOCLK_RATE_SHIFT: u32 = 15;   /* TOCLK_RATE */
pub const WM8993_TOCLK_RATE_WIDTH: u32 = 1;   /* TOCLK_RATE */
pub const WM8993_TOCLK_ENA: u32 = 0x4000;   /* TOCLK_ENA */
pub const WM8993_TOCLK_ENA_MASK: u32 = 0x4000;   /* TOCLK_ENA */
pub const WM8993_TOCLK_ENA_SHIFT: u32 = 14;   /* TOCLK_ENA */
pub const WM8993_TOCLK_ENA_WIDTH: u32 = 1;   /* TOCLK_ENA */
pub const WM8993_OPCLK_DIV_MASK: u32 = 0x1E00;   /* OPCLK_DIV - [12:9] */
pub const WM8993_OPCLK_DIV_SHIFT: u32 = 9;   /* OPCLK_DIV - [12:9] */
pub const WM8993_OPCLK_DIV_WIDTH: u32 = 4;   /* OPCLK_DIV - [12:9] */
pub const WM8993_DCLK_DIV_MASK: u32 = 0x01C0;   /* DCLK_DIV - [8:6] */
pub const WM8993_DCLK_DIV_SHIFT: u32 = 6;   /* DCLK_DIV - [8:6] */
pub const WM8993_DCLK_DIV_WIDTH: u32 = 3;   /* DCLK_DIV - [8:6] */
pub const WM8993_BCLK_DIV_MASK: u32 = 0x001E;   /* BCLK_DIV - [4:1] */
pub const WM8993_BCLK_DIV_SHIFT: u32 = 1;   /* BCLK_DIV - [4:1] */
pub const WM8993_BCLK_DIV_WIDTH: u32 = 4;   /* BCLK_DIV - [4:1] */

/*
 * R7 (0x07) - Clocking 2
 */
pub const WM8993_MCLK_SRC: u32 = 0x8000;   /* MCLK_SRC */
pub const WM8993_MCLK_SRC_MASK: u32 = 0x8000;   /* MCLK_SRC */
pub const WM8993_MCLK_SRC_SHIFT: u32 = 15;   /* MCLK_SRC */
pub const WM8993_MCLK_SRC_WIDTH: u32 = 1;   /* MCLK_SRC */
pub const WM8993_SYSCLK_SRC: u32 = 0x4000;   /* SYSCLK_SRC */
pub const WM8993_SYSCLK_SRC_MASK: u32 = 0x4000;   /* SYSCLK_SRC */
pub const WM8993_SYSCLK_SRC_SHIFT: u32 = 14;   /* SYSCLK_SRC */
pub const WM8993_SYSCLK_SRC_WIDTH: u32 = 1;   /* SYSCLK_SRC */
pub const WM8993_MCLK_DIV: u32 = 0x1000;   /* MCLK_DIV */
pub const WM8993_MCLK_DIV_MASK: u32 = 0x1000;   /* MCLK_DIV */
pub const WM8993_MCLK_DIV_SHIFT: u32 = 12;   /* MCLK_DIV */
pub const WM8993_MCLK_DIV_WIDTH: u32 = 1;   /* MCLK_DIV */
pub const WM8993_MCLK_INV: u32 = 0x0400;   /* MCLK_INV */
pub const WM8993_MCLK_INV_MASK: u32 = 0x0400;   /* MCLK_INV */
pub const WM8993_MCLK_INV_SHIFT: u32 = 10;   /* MCLK_INV */
pub const WM8993_MCLK_INV_WIDTH: u32 = 1;   /* MCLK_INV */
pub const WM8993_ADC_DIV_MASK: u32 = 0x00E0;   /* ADC_DIV - [7:5] */
pub const WM8993_ADC_DIV_SHIFT: u32 = 5;   /* ADC_DIV - [7:5] */
pub const WM8993_ADC_DIV_WIDTH: u32 = 3;   /* ADC_DIV - [7:5] */
pub const WM8993_DAC_DIV_MASK: u32 = 0x001C;   /* DAC_DIV - [4:2] */
pub const WM8993_DAC_DIV_SHIFT: u32 = 2;   /* DAC_DIV - [4:2] */
pub const WM8993_DAC_DIV_WIDTH: u32 = 3;   /* DAC_DIV - [4:2] */

/*
 * R8 (0x08) - Audio Interface (3)
 */
pub const WM8993_AIF_MSTR1: u32 = 0x8000;   /* AIF_MSTR1 */
pub const WM8993_AIF_MSTR1_MASK: u32 = 0x8000;   /* AIF_MSTR1 */
pub const WM8993_AIF_MSTR1_SHIFT: u32 = 15;   /* AIF_MSTR1 */
pub const WM8993_AIF_MSTR1_WIDTH: u32 = 1;   /* AIF_MSTR1 */

/*
 * R9 (0x09) - Audio Interface (4)
 */
pub const WM8993_AIF_TRIS: u32 = 0x2000;   /* AIF_TRIS */
pub const WM8993_AIF_TRIS_MASK: u32 = 0x2000;   /* AIF_TRIS */
pub const WM8993_AIF_TRIS_SHIFT: u32 = 13;   /* AIF_TRIS */
pub const WM8993_AIF_TRIS_WIDTH: u32 = 1;   /* AIF_TRIS */
pub const WM8993_LRCLK_DIR: u32 = 0x0800;   /* LRCLK_DIR */
pub const WM8993_LRCLK_DIR_MASK: u32 = 0x0800;   /* LRCLK_DIR */
pub const WM8993_LRCLK_DIR_SHIFT: u32 = 11;   /* LRCLK_DIR */
pub const WM8993_LRCLK_DIR_WIDTH: u32 = 1;   /* LRCLK_DIR */
pub const WM8993_LRCLK_RATE_MASK: u32 = 0x07FF;   /* LRCLK_RATE - [10:0] */
pub const WM8993_LRCLK_RATE_SHIFT: u32 = 0;   /* LRCLK_RATE - [10:0] */
pub const WM8993_LRCLK_RATE_WIDTH: u32 = 11;   /* LRCLK_RATE - [10:0] */

/*
 * R10 (0x0A) - DAC CTRL
 */
pub const WM8993_DAC_OSR128: u32 = 0x2000;   /* DAC_OSR128 */
pub const WM8993_DAC_OSR128_MASK: u32 = 0x2000;   /* DAC_OSR128 */
pub const WM8993_DAC_OSR128_SHIFT: u32 = 13;   /* DAC_OSR128 */
pub const WM8993_DAC_OSR128_WIDTH: u32 = 1;   /* DAC_OSR128 */
pub const WM8993_DAC_MONO: u32 = 0x0200;   /* DAC_MONO */
pub const WM8993_DAC_MONO_MASK: u32 = 0x0200;   /* DAC_MONO */
pub const WM8993_DAC_MONO_SHIFT: u32 = 9;   /* DAC_MONO */
pub const WM8993_DAC_MONO_WIDTH: u32 = 1;   /* DAC_MONO */
pub const WM8993_DAC_SB_FILT: u32 = 0x0100;   /* DAC_SB_FILT */
pub const WM8993_DAC_SB_FILT_MASK: u32 = 0x0100;   /* DAC_SB_FILT */
pub const WM8993_DAC_SB_FILT_SHIFT: u32 = 8;   /* DAC_SB_FILT */
pub const WM8993_DAC_SB_FILT_WIDTH: u32 = 1;   /* DAC_SB_FILT */
pub const WM8993_DAC_MUTERATE: u32 = 0x0080;   /* DAC_MUTERATE */
pub const WM8993_DAC_MUTERATE_MASK: u32 = 0x0080;   /* DAC_MUTERATE */
pub const WM8993_DAC_MUTERATE_SHIFT: u32 = 7;   /* DAC_MUTERATE */
pub const WM8993_DAC_MUTERATE_WIDTH: u32 = 1;   /* DAC_MUTERATE */
pub const WM8993_DAC_UNMUTE_RAMP: u32 = 0x0040;   /* DAC_UNMUTE_RAMP */
pub const WM8993_DAC_UNMUTE_RAMP_MASK: u32 = 0x0040;   /* DAC_UNMUTE_RAMP */
pub const WM8993_DAC_UNMUTE_RAMP_SHIFT: u32 = 6;   /* DAC_UNMUTE_RAMP */
pub const WM8993_DAC_UNMUTE_RAMP_WIDTH: u32 = 1;   /* DAC_UNMUTE_RAMP */
pub const WM8993_DEEMPH_MASK: u32 = 0x0030;   /* DEEMPH - [5:4] */
pub const WM8993_DEEMPH_SHIFT: u32 = 4;   /* DEEMPH - [5:4] */
pub const WM8993_DEEMPH_WIDTH: u32 = 2;   /* DEEMPH - [5:4] */
pub const WM8993_DAC_MUTE: u32 = 0x0004;   /* DAC_MUTE */
pub const WM8993_DAC_MUTE_MASK: u32 = 0x0004;   /* DAC_MUTE */
pub const WM8993_DAC_MUTE_SHIFT: u32 = 2;   /* DAC_MUTE */
pub const WM8993_DAC_MUTE_WIDTH: u32 = 1;   /* DAC_MUTE */
pub const WM8993_DACL_DATINV: u32 = 0x0002;   /* DACL_DATINV */
pub const WM8993_DACL_DATINV_MASK: u32 = 0x0002;   /* DACL_DATINV */
pub const WM8993_DACL_DATINV_SHIFT: u32 = 1;   /* DACL_DATINV */
pub const WM8993_DACL_DATINV_WIDTH: u32 = 1;   /* DACL_DATINV */
pub const WM8993_DACR_DATINV: u32 = 0x0001;   /* DACR_DATINV */
pub const WM8993_DACR_DATINV_MASK: u32 = 0x0001;   /* DACR_DATINV */
pub const WM8993_DACR_DATINV_SHIFT: u32 = 0;   /* DACR_DATINV */
pub const WM8993_DACR_DATINV_WIDTH: u32 = 1;   /* DACR_DATINV */

/*
 * R11 (0x0B) - Left DAC Digital Volume
 */
pub const WM8993_DAC_VU: u32 = 0x0100;   /* DAC_VU */
pub const WM8993_DAC_VU_MASK: u32 = 0x0100;   /* DAC_VU */
pub const WM8993_DAC_VU_SHIFT: u32 = 8;   /* DAC_VU */
pub const WM8993_DAC_VU_WIDTH: u32 = 1;   /* DAC_VU */
pub const WM8993_DACL_VOL_MASK: u32 = 0x00FF;   /* DACL_VOL - [7:0] */
pub const WM8993_DACL_VOL_SHIFT: u32 = 0;   /* DACL_VOL - [7:0] */
pub const WM8993_DACL_VOL_WIDTH: u32 = 8;   /* DACL_VOL - [7:0] */

/*
 * R12 (0x0C) - Right DAC Digital Volume
 */
// Duplicate C macro: #define WM8993_DAC_VU 0x0100    /* DAC_VU */
// Duplicate C macro: #define WM8993_DAC_VU_MASK 0x0100    /* DAC_VU */
// Duplicate C macro: #define WM8993_DAC_VU_SHIFT 8    /* DAC_VU */
// Duplicate C macro: #define WM8993_DAC_VU_WIDTH 1    /* DAC_VU */
pub const WM8993_DACR_VOL_MASK: u32 = 0x00FF;   /* DACR_VOL - [7:0] */
pub const WM8993_DACR_VOL_SHIFT: u32 = 0;   /* DACR_VOL - [7:0] */
pub const WM8993_DACR_VOL_WIDTH: u32 = 8;   /* DACR_VOL - [7:0] */

/*
 * R13 (0x0D) - Digital Side Tone
 */
pub const WM8993_ADCL_DAC_SVOL_MASK: u32 = 0x1E00;   /* ADCL_DAC_SVOL - [12:9] */
pub const WM8993_ADCL_DAC_SVOL_SHIFT: u32 = 9;   /* ADCL_DAC_SVOL - [12:9] */
pub const WM8993_ADCL_DAC_SVOL_WIDTH: u32 = 4;   /* ADCL_DAC_SVOL - [12:9] */
pub const WM8993_ADCR_DAC_SVOL_MASK: u32 = 0x01E0;   /* ADCR_DAC_SVOL - [8:5] */
pub const WM8993_ADCR_DAC_SVOL_SHIFT: u32 = 5;   /* ADCR_DAC_SVOL - [8:5] */
pub const WM8993_ADCR_DAC_SVOL_WIDTH: u32 = 4;   /* ADCR_DAC_SVOL - [8:5] */
pub const WM8993_ADC_TO_DACL_MASK: u32 = 0x000C;   /* ADC_TO_DACL - [3:2] */
pub const WM8993_ADC_TO_DACL_SHIFT: u32 = 2;   /* ADC_TO_DACL - [3:2] */
pub const WM8993_ADC_TO_DACL_WIDTH: u32 = 2;   /* ADC_TO_DACL - [3:2] */
pub const WM8993_ADC_TO_DACR_MASK: u32 = 0x0003;   /* ADC_TO_DACR - [1:0] */
pub const WM8993_ADC_TO_DACR_SHIFT: u32 = 0;   /* ADC_TO_DACR - [1:0] */
pub const WM8993_ADC_TO_DACR_WIDTH: u32 = 2;   /* ADC_TO_DACR - [1:0] */

/*
 * R14 (0x0E) - ADC CTRL
 */
pub const WM8993_ADC_OSR128: u32 = 0x0200;   /* ADC_OSR128 */
pub const WM8993_ADC_OSR128_MASK: u32 = 0x0200;   /* ADC_OSR128 */
pub const WM8993_ADC_OSR128_SHIFT: u32 = 9;   /* ADC_OSR128 */
pub const WM8993_ADC_OSR128_WIDTH: u32 = 1;   /* ADC_OSR128 */
pub const WM8993_ADC_HPF: u32 = 0x0100;   /* ADC_HPF */
pub const WM8993_ADC_HPF_MASK: u32 = 0x0100;   /* ADC_HPF */
pub const WM8993_ADC_HPF_SHIFT: u32 = 8;   /* ADC_HPF */
pub const WM8993_ADC_HPF_WIDTH: u32 = 1;   /* ADC_HPF */
pub const WM8993_ADC_HPF_CUT_MASK: u32 = 0x0060;   /* ADC_HPF_CUT - [6:5] */
pub const WM8993_ADC_HPF_CUT_SHIFT: u32 = 5;   /* ADC_HPF_CUT - [6:5] */
pub const WM8993_ADC_HPF_CUT_WIDTH: u32 = 2;   /* ADC_HPF_CUT - [6:5] */
pub const WM8993_ADCL_DATINV: u32 = 0x0002;   /* ADCL_DATINV */
pub const WM8993_ADCL_DATINV_MASK: u32 = 0x0002;   /* ADCL_DATINV */
pub const WM8993_ADCL_DATINV_SHIFT: u32 = 1;   /* ADCL_DATINV */
pub const WM8993_ADCL_DATINV_WIDTH: u32 = 1;   /* ADCL_DATINV */
pub const WM8993_ADCR_DATINV: u32 = 0x0001;   /* ADCR_DATINV */
pub const WM8993_ADCR_DATINV_MASK: u32 = 0x0001;   /* ADCR_DATINV */
pub const WM8993_ADCR_DATINV_SHIFT: u32 = 0;   /* ADCR_DATINV */
pub const WM8993_ADCR_DATINV_WIDTH: u32 = 1;   /* ADCR_DATINV */

/*
 * R15 (0x0F) - Left ADC Digital Volume
 */
pub const WM8993_ADC_VU: u32 = 0x0100;   /* ADC_VU */
pub const WM8993_ADC_VU_MASK: u32 = 0x0100;   /* ADC_VU */
pub const WM8993_ADC_VU_SHIFT: u32 = 8;   /* ADC_VU */
pub const WM8993_ADC_VU_WIDTH: u32 = 1;   /* ADC_VU */
pub const WM8993_ADCL_VOL_MASK: u32 = 0x00FF;   /* ADCL_VOL - [7:0] */
pub const WM8993_ADCL_VOL_SHIFT: u32 = 0;   /* ADCL_VOL - [7:0] */
pub const WM8993_ADCL_VOL_WIDTH: u32 = 8;   /* ADCL_VOL - [7:0] */

/*
 * R16 (0x10) - Right ADC Digital Volume
 */
// Duplicate C macro: #define WM8993_ADC_VU 0x0100    /* ADC_VU */
// Duplicate C macro: #define WM8993_ADC_VU_MASK 0x0100    /* ADC_VU */
// Duplicate C macro: #define WM8993_ADC_VU_SHIFT 8    /* ADC_VU */
// Duplicate C macro: #define WM8993_ADC_VU_WIDTH 1    /* ADC_VU */
pub const WM8993_ADCR_VOL_MASK: u32 = 0x00FF;   /* ADCR_VOL - [7:0] */
pub const WM8993_ADCR_VOL_SHIFT: u32 = 0;   /* ADCR_VOL - [7:0] */
pub const WM8993_ADCR_VOL_WIDTH: u32 = 8;   /* ADCR_VOL - [7:0] */

/*
 * R18 (0x12) - GPIO CTRL 1
 */
pub const WM8993_JD2_SC_EINT: u32 = 0x8000;   /* JD2_SC_EINT */
pub const WM8993_JD2_SC_EINT_MASK: u32 = 0x8000;   /* JD2_SC_EINT */
pub const WM8993_JD2_SC_EINT_SHIFT: u32 = 15;   /* JD2_SC_EINT */
pub const WM8993_JD2_SC_EINT_WIDTH: u32 = 1;   /* JD2_SC_EINT */
pub const WM8993_JD2_EINT: u32 = 0x4000;   /* JD2_EINT */
pub const WM8993_JD2_EINT_MASK: u32 = 0x4000;   /* JD2_EINT */
pub const WM8993_JD2_EINT_SHIFT: u32 = 14;   /* JD2_EINT */
pub const WM8993_JD2_EINT_WIDTH: u32 = 1;   /* JD2_EINT */
pub const WM8993_WSEQ_EINT: u32 = 0x2000;   /* WSEQ_EINT */
pub const WM8993_WSEQ_EINT_MASK: u32 = 0x2000;   /* WSEQ_EINT */
pub const WM8993_WSEQ_EINT_SHIFT: u32 = 13;   /* WSEQ_EINT */
pub const WM8993_WSEQ_EINT_WIDTH: u32 = 1;   /* WSEQ_EINT */
pub const WM8993_IRQ: u32 = 0x1000;   /* IRQ */
pub const WM8993_IRQ_MASK: u32 = 0x1000;   /* IRQ */
pub const WM8993_IRQ_SHIFT: u32 = 12;   /* IRQ */
pub const WM8993_IRQ_WIDTH: u32 = 1;   /* IRQ */
pub const WM8993_TEMPOK_EINT: u32 = 0x0800;   /* TEMPOK_EINT */
pub const WM8993_TEMPOK_EINT_MASK: u32 = 0x0800;   /* TEMPOK_EINT */
pub const WM8993_TEMPOK_EINT_SHIFT: u32 = 11;   /* TEMPOK_EINT */
pub const WM8993_TEMPOK_EINT_WIDTH: u32 = 1;   /* TEMPOK_EINT */
pub const WM8993_JD1_SC_EINT: u32 = 0x0400;   /* JD1_SC_EINT */
pub const WM8993_JD1_SC_EINT_MASK: u32 = 0x0400;   /* JD1_SC_EINT */
pub const WM8993_JD1_SC_EINT_SHIFT: u32 = 10;   /* JD1_SC_EINT */
pub const WM8993_JD1_SC_EINT_WIDTH: u32 = 1;   /* JD1_SC_EINT */
pub const WM8993_JD1_EINT: u32 = 0x0200;   /* JD1_EINT */
pub const WM8993_JD1_EINT_MASK: u32 = 0x0200;   /* JD1_EINT */
pub const WM8993_JD1_EINT_SHIFT: u32 = 9;   /* JD1_EINT */
pub const WM8993_JD1_EINT_WIDTH: u32 = 1;   /* JD1_EINT */
pub const WM8993_FLL_LOCK_EINT: u32 = 0x0100;   /* FLL_LOCK_EINT */
pub const WM8993_FLL_LOCK_EINT_MASK: u32 = 0x0100;   /* FLL_LOCK_EINT */
pub const WM8993_FLL_LOCK_EINT_SHIFT: u32 = 8;   /* FLL_LOCK_EINT */
pub const WM8993_FLL_LOCK_EINT_WIDTH: u32 = 1;   /* FLL_LOCK_EINT */
pub const WM8993_GPI8_EINT: u32 = 0x0080;   /* GPI8_EINT */
pub const WM8993_GPI8_EINT_MASK: u32 = 0x0080;   /* GPI8_EINT */
pub const WM8993_GPI8_EINT_SHIFT: u32 = 7;   /* GPI8_EINT */
pub const WM8993_GPI8_EINT_WIDTH: u32 = 1;   /* GPI8_EINT */
pub const WM8993_GPI7_EINT: u32 = 0x0040;   /* GPI7_EINT */
pub const WM8993_GPI7_EINT_MASK: u32 = 0x0040;   /* GPI7_EINT */
pub const WM8993_GPI7_EINT_SHIFT: u32 = 6;   /* GPI7_EINT */
pub const WM8993_GPI7_EINT_WIDTH: u32 = 1;   /* GPI7_EINT */
pub const WM8993_GPIO1_EINT: u32 = 0x0001;   /* GPIO1_EINT */
pub const WM8993_GPIO1_EINT_MASK: u32 = 0x0001;   /* GPIO1_EINT */
pub const WM8993_GPIO1_EINT_SHIFT: u32 = 0;   /* GPIO1_EINT */
pub const WM8993_GPIO1_EINT_WIDTH: u32 = 1;   /* GPIO1_EINT */

/*
 * R19 (0x13) - GPIO1
 */
pub const WM8993_GPIO1_PU: u32 = 0x0020;   /* GPIO1_PU */
pub const WM8993_GPIO1_PU_MASK: u32 = 0x0020;   /* GPIO1_PU */
pub const WM8993_GPIO1_PU_SHIFT: u32 = 5;   /* GPIO1_PU */
pub const WM8993_GPIO1_PU_WIDTH: u32 = 1;   /* GPIO1_PU */
pub const WM8993_GPIO1_PD: u32 = 0x0010;   /* GPIO1_PD */
pub const WM8993_GPIO1_PD_MASK: u32 = 0x0010;   /* GPIO1_PD */
pub const WM8993_GPIO1_PD_SHIFT: u32 = 4;   /* GPIO1_PD */
pub const WM8993_GPIO1_PD_WIDTH: u32 = 1;   /* GPIO1_PD */
pub const WM8993_GPIO1_SEL_MASK: u32 = 0x000F;   /* GPIO1_SEL - [3:0] */
pub const WM8993_GPIO1_SEL_SHIFT: u32 = 0;   /* GPIO1_SEL - [3:0] */
pub const WM8993_GPIO1_SEL_WIDTH: u32 = 4;   /* GPIO1_SEL - [3:0] */

/*
 * R20 (0x14) - IRQ_DEBOUNCE
 */
pub const WM8993_JD2_SC_DB: u32 = 0x8000;   /* JD2_SC_DB */
pub const WM8993_JD2_SC_DB_MASK: u32 = 0x8000;   /* JD2_SC_DB */
pub const WM8993_JD2_SC_DB_SHIFT: u32 = 15;   /* JD2_SC_DB */
pub const WM8993_JD2_SC_DB_WIDTH: u32 = 1;   /* JD2_SC_DB */
pub const WM8993_JD2_DB: u32 = 0x4000;   /* JD2_DB */
pub const WM8993_JD2_DB_MASK: u32 = 0x4000;   /* JD2_DB */
pub const WM8993_JD2_DB_SHIFT: u32 = 14;   /* JD2_DB */
pub const WM8993_JD2_DB_WIDTH: u32 = 1;   /* JD2_DB */
pub const WM8993_WSEQ_DB: u32 = 0x2000;   /* WSEQ_DB */
pub const WM8993_WSEQ_DB_MASK: u32 = 0x2000;   /* WSEQ_DB */
pub const WM8993_WSEQ_DB_SHIFT: u32 = 13;   /* WSEQ_DB */
pub const WM8993_WSEQ_DB_WIDTH: u32 = 1;   /* WSEQ_DB */
pub const WM8993_TEMPOK_DB: u32 = 0x0800;   /* TEMPOK_DB */
pub const WM8993_TEMPOK_DB_MASK: u32 = 0x0800;   /* TEMPOK_DB */
pub const WM8993_TEMPOK_DB_SHIFT: u32 = 11;   /* TEMPOK_DB */
pub const WM8993_TEMPOK_DB_WIDTH: u32 = 1;   /* TEMPOK_DB */
pub const WM8993_JD1_SC_DB: u32 = 0x0400;   /* JD1_SC_DB */
pub const WM8993_JD1_SC_DB_MASK: u32 = 0x0400;   /* JD1_SC_DB */
pub const WM8993_JD1_SC_DB_SHIFT: u32 = 10;   /* JD1_SC_DB */
pub const WM8993_JD1_SC_DB_WIDTH: u32 = 1;   /* JD1_SC_DB */
pub const WM8993_JD1_DB: u32 = 0x0200;   /* JD1_DB */
pub const WM8993_JD1_DB_MASK: u32 = 0x0200;   /* JD1_DB */
pub const WM8993_JD1_DB_SHIFT: u32 = 9;   /* JD1_DB */
pub const WM8993_JD1_DB_WIDTH: u32 = 1;   /* JD1_DB */
pub const WM8993_FLL_LOCK_DB: u32 = 0x0100;   /* FLL_LOCK_DB */
pub const WM8993_FLL_LOCK_DB_MASK: u32 = 0x0100;   /* FLL_LOCK_DB */
pub const WM8993_FLL_LOCK_DB_SHIFT: u32 = 8;   /* FLL_LOCK_DB */
pub const WM8993_FLL_LOCK_DB_WIDTH: u32 = 1;   /* FLL_LOCK_DB */
pub const WM8993_GPI8_DB: u32 = 0x0080;   /* GPI8_DB */
pub const WM8993_GPI8_DB_MASK: u32 = 0x0080;   /* GPI8_DB */
pub const WM8993_GPI8_DB_SHIFT: u32 = 7;   /* GPI8_DB */
pub const WM8993_GPI8_DB_WIDTH: u32 = 1;   /* GPI8_DB */
pub const WM8993_GPI7_DB: u32 = 0x0008;   /* GPI7_DB */
pub const WM8993_GPI7_DB_MASK: u32 = 0x0008;   /* GPI7_DB */
pub const WM8993_GPI7_DB_SHIFT: u32 = 3;   /* GPI7_DB */
pub const WM8993_GPI7_DB_WIDTH: u32 = 1;   /* GPI7_DB */
pub const WM8993_GPIO1_DB: u32 = 0x0001;   /* GPIO1_DB */
pub const WM8993_GPIO1_DB_MASK: u32 = 0x0001;   /* GPIO1_DB */
pub const WM8993_GPIO1_DB_SHIFT: u32 = 0;   /* GPIO1_DB */
pub const WM8993_GPIO1_DB_WIDTH: u32 = 1;   /* GPIO1_DB */

/*
 * R21 (0x15) - Inputs Clamp
 */
pub const WM8993_INPUTS_CLAMP: u32 = 0x0040;   /* INPUTS_CLAMP */
pub const WM8993_INPUTS_CLAMP_MASK: u32 = 0x0040;   /* INPUTS_CLAMP */
pub const WM8993_INPUTS_CLAMP_SHIFT: u32 = 7;   /* INPUTS_CLAMP */
pub const WM8993_INPUTS_CLAMP_WIDTH: u32 = 1;   /* INPUTS_CLAMP */

/*
 * R22 (0x16) - GPIOCTRL 2
 */
pub const WM8993_IM_JD2_EINT: u32 = 0x2000;   /* IM_JD2_EINT */
pub const WM8993_IM_JD2_EINT_MASK: u32 = 0x2000;   /* IM_JD2_EINT */
pub const WM8993_IM_JD2_EINT_SHIFT: u32 = 13;   /* IM_JD2_EINT */
pub const WM8993_IM_JD2_EINT_WIDTH: u32 = 1;   /* IM_JD2_EINT */
pub const WM8993_IM_JD2_SC_EINT: u32 = 0x1000;   /* IM_JD2_SC_EINT */
pub const WM8993_IM_JD2_SC_EINT_MASK: u32 = 0x1000;   /* IM_JD2_SC_EINT */
pub const WM8993_IM_JD2_SC_EINT_SHIFT: u32 = 12;   /* IM_JD2_SC_EINT */
pub const WM8993_IM_JD2_SC_EINT_WIDTH: u32 = 1;   /* IM_JD2_SC_EINT */
pub const WM8993_IM_TEMPOK_EINT: u32 = 0x0800;   /* IM_TEMPOK_EINT */
pub const WM8993_IM_TEMPOK_EINT_MASK: u32 = 0x0800;   /* IM_TEMPOK_EINT */
pub const WM8993_IM_TEMPOK_EINT_SHIFT: u32 = 11;   /* IM_TEMPOK_EINT */
pub const WM8993_IM_TEMPOK_EINT_WIDTH: u32 = 1;   /* IM_TEMPOK_EINT */
pub const WM8993_IM_JD1_SC_EINT: u32 = 0x0400;   /* IM_JD1_SC_EINT */
pub const WM8993_IM_JD1_SC_EINT_MASK: u32 = 0x0400;   /* IM_JD1_SC_EINT */
pub const WM8993_IM_JD1_SC_EINT_SHIFT: u32 = 10;   /* IM_JD1_SC_EINT */
pub const WM8993_IM_JD1_SC_EINT_WIDTH: u32 = 1;   /* IM_JD1_SC_EINT */
pub const WM8993_IM_JD1_EINT: u32 = 0x0200;   /* IM_JD1_EINT */
pub const WM8993_IM_JD1_EINT_MASK: u32 = 0x0200;   /* IM_JD1_EINT */
pub const WM8993_IM_JD1_EINT_SHIFT: u32 = 9;   /* IM_JD1_EINT */
pub const WM8993_IM_JD1_EINT_WIDTH: u32 = 1;   /* IM_JD1_EINT */
pub const WM8993_IM_FLL_LOCK_EINT: u32 = 0x0100;   /* IM_FLL_LOCK_EINT */
pub const WM8993_IM_FLL_LOCK_EINT_MASK: u32 = 0x0100;   /* IM_FLL_LOCK_EINT */
pub const WM8993_IM_FLL_LOCK_EINT_SHIFT: u32 = 8;   /* IM_FLL_LOCK_EINT */
pub const WM8993_IM_FLL_LOCK_EINT_WIDTH: u32 = 1;   /* IM_FLL_LOCK_EINT */
pub const WM8993_IM_GPI8_EINT: u32 = 0x0040;   /* IM_GPI8_EINT */
pub const WM8993_IM_GPI8_EINT_MASK: u32 = 0x0040;   /* IM_GPI8_EINT */
pub const WM8993_IM_GPI8_EINT_SHIFT: u32 = 6;   /* IM_GPI8_EINT */
pub const WM8993_IM_GPI8_EINT_WIDTH: u32 = 1;   /* IM_GPI8_EINT */
pub const WM8993_IM_GPIO1_EINT: u32 = 0x0020;   /* IM_GPIO1_EINT */
pub const WM8993_IM_GPIO1_EINT_MASK: u32 = 0x0020;   /* IM_GPIO1_EINT */
pub const WM8993_IM_GPIO1_EINT_SHIFT: u32 = 5;   /* IM_GPIO1_EINT */
pub const WM8993_IM_GPIO1_EINT_WIDTH: u32 = 1;   /* IM_GPIO1_EINT */
pub const WM8993_GPI8_ENA: u32 = 0x0010;   /* GPI8_ENA */
pub const WM8993_GPI8_ENA_MASK: u32 = 0x0010;   /* GPI8_ENA */
pub const WM8993_GPI8_ENA_SHIFT: u32 = 4;   /* GPI8_ENA */
pub const WM8993_GPI8_ENA_WIDTH: u32 = 1;   /* GPI8_ENA */
pub const WM8993_IM_GPI7_EINT: u32 = 0x0004;   /* IM_GPI7_EINT */
pub const WM8993_IM_GPI7_EINT_MASK: u32 = 0x0004;   /* IM_GPI7_EINT */
pub const WM8993_IM_GPI7_EINT_SHIFT: u32 = 2;   /* IM_GPI7_EINT */
pub const WM8993_IM_GPI7_EINT_WIDTH: u32 = 1;   /* IM_GPI7_EINT */
pub const WM8993_IM_WSEQ_EINT: u32 = 0x0002;   /* IM_WSEQ_EINT */
pub const WM8993_IM_WSEQ_EINT_MASK: u32 = 0x0002;   /* IM_WSEQ_EINT */
pub const WM8993_IM_WSEQ_EINT_SHIFT: u32 = 1;   /* IM_WSEQ_EINT */
pub const WM8993_IM_WSEQ_EINT_WIDTH: u32 = 1;   /* IM_WSEQ_EINT */
pub const WM8993_GPI7_ENA: u32 = 0x0001;   /* GPI7_ENA */
pub const WM8993_GPI7_ENA_MASK: u32 = 0x0001;   /* GPI7_ENA */
pub const WM8993_GPI7_ENA_SHIFT: u32 = 0;   /* GPI7_ENA */
pub const WM8993_GPI7_ENA_WIDTH: u32 = 1;   /* GPI7_ENA */

/*
 * R23 (0x17) - GPIO_POL
 */
pub const WM8993_JD2_SC_POL: u32 = 0x8000;   /* JD2_SC_POL */
pub const WM8993_JD2_SC_POL_MASK: u32 = 0x8000;   /* JD2_SC_POL */
pub const WM8993_JD2_SC_POL_SHIFT: u32 = 15;   /* JD2_SC_POL */
pub const WM8993_JD2_SC_POL_WIDTH: u32 = 1;   /* JD2_SC_POL */
pub const WM8993_JD2_POL: u32 = 0x4000;   /* JD2_POL */
pub const WM8993_JD2_POL_MASK: u32 = 0x4000;   /* JD2_POL */
pub const WM8993_JD2_POL_SHIFT: u32 = 14;   /* JD2_POL */
pub const WM8993_JD2_POL_WIDTH: u32 = 1;   /* JD2_POL */
pub const WM8993_WSEQ_POL: u32 = 0x2000;   /* WSEQ_POL */
pub const WM8993_WSEQ_POL_MASK: u32 = 0x2000;   /* WSEQ_POL */
pub const WM8993_WSEQ_POL_SHIFT: u32 = 13;   /* WSEQ_POL */
pub const WM8993_WSEQ_POL_WIDTH: u32 = 1;   /* WSEQ_POL */
pub const WM8993_IRQ_POL: u32 = 0x1000;   /* IRQ_POL */
pub const WM8993_IRQ_POL_MASK: u32 = 0x1000;   /* IRQ_POL */
pub const WM8993_IRQ_POL_SHIFT: u32 = 12;   /* IRQ_POL */
pub const WM8993_IRQ_POL_WIDTH: u32 = 1;   /* IRQ_POL */
pub const WM8993_TEMPOK_POL: u32 = 0x0800;   /* TEMPOK_POL */
pub const WM8993_TEMPOK_POL_MASK: u32 = 0x0800;   /* TEMPOK_POL */
pub const WM8993_TEMPOK_POL_SHIFT: u32 = 11;   /* TEMPOK_POL */
pub const WM8993_TEMPOK_POL_WIDTH: u32 = 1;   /* TEMPOK_POL */
pub const WM8993_JD1_SC_POL: u32 = 0x0400;   /* JD1_SC_POL */
pub const WM8993_JD1_SC_POL_MASK: u32 = 0x0400;   /* JD1_SC_POL */
pub const WM8993_JD1_SC_POL_SHIFT: u32 = 10;   /* JD1_SC_POL */
pub const WM8993_JD1_SC_POL_WIDTH: u32 = 1;   /* JD1_SC_POL */
pub const WM8993_JD1_POL: u32 = 0x0200;   /* JD1_POL */
pub const WM8993_JD1_POL_MASK: u32 = 0x0200;   /* JD1_POL */
pub const WM8993_JD1_POL_SHIFT: u32 = 9;   /* JD1_POL */
pub const WM8993_JD1_POL_WIDTH: u32 = 1;   /* JD1_POL */
pub const WM8993_FLL_LOCK_POL: u32 = 0x0100;   /* FLL_LOCK_POL */
pub const WM8993_FLL_LOCK_POL_MASK: u32 = 0x0100;   /* FLL_LOCK_POL */
pub const WM8993_FLL_LOCK_POL_SHIFT: u32 = 8;   /* FLL_LOCK_POL */
pub const WM8993_FLL_LOCK_POL_WIDTH: u32 = 1;   /* FLL_LOCK_POL */
pub const WM8993_GPI8_POL: u32 = 0x0080;   /* GPI8_POL */
pub const WM8993_GPI8_POL_MASK: u32 = 0x0080;   /* GPI8_POL */
pub const WM8993_GPI8_POL_SHIFT: u32 = 7;   /* GPI8_POL */
pub const WM8993_GPI8_POL_WIDTH: u32 = 1;   /* GPI8_POL */
pub const WM8993_GPI7_POL: u32 = 0x0040;   /* GPI7_POL */
pub const WM8993_GPI7_POL_MASK: u32 = 0x0040;   /* GPI7_POL */
pub const WM8993_GPI7_POL_SHIFT: u32 = 6;   /* GPI7_POL */
pub const WM8993_GPI7_POL_WIDTH: u32 = 1;   /* GPI7_POL */
pub const WM8993_GPIO1_POL: u32 = 0x0001;   /* GPIO1_POL */
pub const WM8993_GPIO1_POL_MASK: u32 = 0x0001;   /* GPIO1_POL */
pub const WM8993_GPIO1_POL_SHIFT: u32 = 0;   /* GPIO1_POL */
pub const WM8993_GPIO1_POL_WIDTH: u32 = 1;   /* GPIO1_POL */

/*
 * R24 (0x18) - Left Line Input 1&2 Volume
 */
pub const WM8993_IN1_VU: u32 = 0x0100;   /* IN1_VU */
pub const WM8993_IN1_VU_MASK: u32 = 0x0100;   /* IN1_VU */
pub const WM8993_IN1_VU_SHIFT: u32 = 8;   /* IN1_VU */
pub const WM8993_IN1_VU_WIDTH: u32 = 1;   /* IN1_VU */
pub const WM8993_IN1L_MUTE: u32 = 0x0080;   /* IN1L_MUTE */
pub const WM8993_IN1L_MUTE_MASK: u32 = 0x0080;   /* IN1L_MUTE */
pub const WM8993_IN1L_MUTE_SHIFT: u32 = 7;   /* IN1L_MUTE */
pub const WM8993_IN1L_MUTE_WIDTH: u32 = 1;   /* IN1L_MUTE */
pub const WM8993_IN1L_ZC: u32 = 0x0040;   /* IN1L_ZC */
pub const WM8993_IN1L_ZC_MASK: u32 = 0x0040;   /* IN1L_ZC */
pub const WM8993_IN1L_ZC_SHIFT: u32 = 6;   /* IN1L_ZC */
pub const WM8993_IN1L_ZC_WIDTH: u32 = 1;   /* IN1L_ZC */
pub const WM8993_IN1L_VOL_MASK: u32 = 0x001F;   /* IN1L_VOL - [4:0] */
pub const WM8993_IN1L_VOL_SHIFT: u32 = 0;   /* IN1L_VOL - [4:0] */
pub const WM8993_IN1L_VOL_WIDTH: u32 = 5;   /* IN1L_VOL - [4:0] */

/*
 * R25 (0x19) - Left Line Input 3&4 Volume
 */
pub const WM8993_IN2_VU: u32 = 0x0100;   /* IN2_VU */
pub const WM8993_IN2_VU_MASK: u32 = 0x0100;   /* IN2_VU */
pub const WM8993_IN2_VU_SHIFT: u32 = 8;   /* IN2_VU */
pub const WM8993_IN2_VU_WIDTH: u32 = 1;   /* IN2_VU */
pub const WM8993_IN2L_MUTE: u32 = 0x0080;   /* IN2L_MUTE */
pub const WM8993_IN2L_MUTE_MASK: u32 = 0x0080;   /* IN2L_MUTE */
pub const WM8993_IN2L_MUTE_SHIFT: u32 = 7;   /* IN2L_MUTE */
pub const WM8993_IN2L_MUTE_WIDTH: u32 = 1;   /* IN2L_MUTE */
pub const WM8993_IN2L_ZC: u32 = 0x0040;   /* IN2L_ZC */
pub const WM8993_IN2L_ZC_MASK: u32 = 0x0040;   /* IN2L_ZC */
pub const WM8993_IN2L_ZC_SHIFT: u32 = 6;   /* IN2L_ZC */
pub const WM8993_IN2L_ZC_WIDTH: u32 = 1;   /* IN2L_ZC */
pub const WM8993_IN2L_VOL_MASK: u32 = 0x001F;   /* IN2L_VOL - [4:0] */
pub const WM8993_IN2L_VOL_SHIFT: u32 = 0;   /* IN2L_VOL - [4:0] */
pub const WM8993_IN2L_VOL_WIDTH: u32 = 5;   /* IN2L_VOL - [4:0] */

/*
 * R26 (0x1A) - Right Line Input 1&2 Volume
 */
// Duplicate C macro: #define WM8993_IN1_VU 0x0100    /* IN1_VU */
// Duplicate C macro: #define WM8993_IN1_VU_MASK 0x0100    /* IN1_VU */
// Duplicate C macro: #define WM8993_IN1_VU_SHIFT 8    /* IN1_VU */
// Duplicate C macro: #define WM8993_IN1_VU_WIDTH 1    /* IN1_VU */
pub const WM8993_IN1R_MUTE: u32 = 0x0080;   /* IN1R_MUTE */
pub const WM8993_IN1R_MUTE_MASK: u32 = 0x0080;   /* IN1R_MUTE */
pub const WM8993_IN1R_MUTE_SHIFT: u32 = 7;   /* IN1R_MUTE */
pub const WM8993_IN1R_MUTE_WIDTH: u32 = 1;   /* IN1R_MUTE */
pub const WM8993_IN1R_ZC: u32 = 0x0040;   /* IN1R_ZC */
pub const WM8993_IN1R_ZC_MASK: u32 = 0x0040;   /* IN1R_ZC */
pub const WM8993_IN1R_ZC_SHIFT: u32 = 6;   /* IN1R_ZC */
pub const WM8993_IN1R_ZC_WIDTH: u32 = 1;   /* IN1R_ZC */
pub const WM8993_IN1R_VOL_MASK: u32 = 0x001F;   /* IN1R_VOL - [4:0] */
pub const WM8993_IN1R_VOL_SHIFT: u32 = 0;   /* IN1R_VOL - [4:0] */
pub const WM8993_IN1R_VOL_WIDTH: u32 = 5;   /* IN1R_VOL - [4:0] */

/*
 * R27 (0x1B) - Right Line Input 3&4 Volume
 */
// Duplicate C macro: #define WM8993_IN2_VU 0x0100    /* IN2_VU */
// Duplicate C macro: #define WM8993_IN2_VU_MASK 0x0100    /* IN2_VU */
// Duplicate C macro: #define WM8993_IN2_VU_SHIFT 8    /* IN2_VU */
// Duplicate C macro: #define WM8993_IN2_VU_WIDTH 1    /* IN2_VU */
pub const WM8993_IN2R_MUTE: u32 = 0x0080;   /* IN2R_MUTE */
pub const WM8993_IN2R_MUTE_MASK: u32 = 0x0080;   /* IN2R_MUTE */
pub const WM8993_IN2R_MUTE_SHIFT: u32 = 7;   /* IN2R_MUTE */
pub const WM8993_IN2R_MUTE_WIDTH: u32 = 1;   /* IN2R_MUTE */
pub const WM8993_IN2R_ZC: u32 = 0x0040;   /* IN2R_ZC */
pub const WM8993_IN2R_ZC_MASK: u32 = 0x0040;   /* IN2R_ZC */
pub const WM8993_IN2R_ZC_SHIFT: u32 = 6;   /* IN2R_ZC */
pub const WM8993_IN2R_ZC_WIDTH: u32 = 1;   /* IN2R_ZC */
pub const WM8993_IN2R_VOL_MASK: u32 = 0x001F;   /* IN2R_VOL - [4:0] */
pub const WM8993_IN2R_VOL_SHIFT: u32 = 0;   /* IN2R_VOL - [4:0] */
pub const WM8993_IN2R_VOL_WIDTH: u32 = 5;   /* IN2R_VOL - [4:0] */

/*
 * R28 (0x1C) - Left Output Volume
 */
pub const WM8993_HPOUT1_VU: u32 = 0x0100;   /* HPOUT1_VU */
pub const WM8993_HPOUT1_VU_MASK: u32 = 0x0100;   /* HPOUT1_VU */
pub const WM8993_HPOUT1_VU_SHIFT: u32 = 8;   /* HPOUT1_VU */
pub const WM8993_HPOUT1_VU_WIDTH: u32 = 1;   /* HPOUT1_VU */
pub const WM8993_HPOUT1L_ZC: u32 = 0x0080;   /* HPOUT1L_ZC */
pub const WM8993_HPOUT1L_ZC_MASK: u32 = 0x0080;   /* HPOUT1L_ZC */
pub const WM8993_HPOUT1L_ZC_SHIFT: u32 = 7;   /* HPOUT1L_ZC */
pub const WM8993_HPOUT1L_ZC_WIDTH: u32 = 1;   /* HPOUT1L_ZC */
pub const WM8993_HPOUT1L_MUTE_N: u32 = 0x0040;   /* HPOUT1L_MUTE_N */
pub const WM8993_HPOUT1L_MUTE_N_MASK: u32 = 0x0040;   /* HPOUT1L_MUTE_N */
pub const WM8993_HPOUT1L_MUTE_N_SHIFT: u32 = 6;   /* HPOUT1L_MUTE_N */
pub const WM8993_HPOUT1L_MUTE_N_WIDTH: u32 = 1;   /* HPOUT1L_MUTE_N */
pub const WM8993_HPOUT1L_VOL_MASK: u32 = 0x003F;   /* HPOUT1L_VOL - [5:0] */
pub const WM8993_HPOUT1L_VOL_SHIFT: u32 = 0;   /* HPOUT1L_VOL - [5:0] */
pub const WM8993_HPOUT1L_VOL_WIDTH: u32 = 6;   /* HPOUT1L_VOL - [5:0] */

/*
 * R29 (0x1D) - Right Output Volume
 */
// Duplicate C macro: #define WM8993_HPOUT1_VU 0x0100    /* HPOUT1_VU */
// Duplicate C macro: #define WM8993_HPOUT1_VU_MASK 0x0100    /* HPOUT1_VU */
// Duplicate C macro: #define WM8993_HPOUT1_VU_SHIFT 8    /* HPOUT1_VU */
// Duplicate C macro: #define WM8993_HPOUT1_VU_WIDTH 1    /* HPOUT1_VU */
pub const WM8993_HPOUT1R_ZC: u32 = 0x0080;   /* HPOUT1R_ZC */
pub const WM8993_HPOUT1R_ZC_MASK: u32 = 0x0080;   /* HPOUT1R_ZC */
pub const WM8993_HPOUT1R_ZC_SHIFT: u32 = 7;   /* HPOUT1R_ZC */
pub const WM8993_HPOUT1R_ZC_WIDTH: u32 = 1;   /* HPOUT1R_ZC */
pub const WM8993_HPOUT1R_MUTE_N: u32 = 0x0040;   /* HPOUT1R_MUTE_N */
pub const WM8993_HPOUT1R_MUTE_N_MASK: u32 = 0x0040;   /* HPOUT1R_MUTE_N */
pub const WM8993_HPOUT1R_MUTE_N_SHIFT: u32 = 6;   /* HPOUT1R_MUTE_N */
pub const WM8993_HPOUT1R_MUTE_N_WIDTH: u32 = 1;   /* HPOUT1R_MUTE_N */
pub const WM8993_HPOUT1R_VOL_MASK: u32 = 0x003F;   /* HPOUT1R_VOL - [5:0] */
pub const WM8993_HPOUT1R_VOL_SHIFT: u32 = 0;   /* HPOUT1R_VOL - [5:0] */
pub const WM8993_HPOUT1R_VOL_WIDTH: u32 = 6;   /* HPOUT1R_VOL - [5:0] */

/*
 * R30 (0x1E) - Line Outputs Volume
 */
pub const WM8993_LINEOUT1N_MUTE: u32 = 0x0040;   /* LINEOUT1N_MUTE */
pub const WM8993_LINEOUT1N_MUTE_MASK: u32 = 0x0040;   /* LINEOUT1N_MUTE */
pub const WM8993_LINEOUT1N_MUTE_SHIFT: u32 = 6;   /* LINEOUT1N_MUTE */
pub const WM8993_LINEOUT1N_MUTE_WIDTH: u32 = 1;   /* LINEOUT1N_MUTE */
pub const WM8993_LINEOUT1P_MUTE: u32 = 0x0020;   /* LINEOUT1P_MUTE */
pub const WM8993_LINEOUT1P_MUTE_MASK: u32 = 0x0020;   /* LINEOUT1P_MUTE */
pub const WM8993_LINEOUT1P_MUTE_SHIFT: u32 = 5;   /* LINEOUT1P_MUTE */
pub const WM8993_LINEOUT1P_MUTE_WIDTH: u32 = 1;   /* LINEOUT1P_MUTE */
pub const WM8993_LINEOUT1_VOL: u32 = 0x0010;   /* LINEOUT1_VOL */
pub const WM8993_LINEOUT1_VOL_MASK: u32 = 0x0010;   /* LINEOUT1_VOL */
pub const WM8993_LINEOUT1_VOL_SHIFT: u32 = 4;   /* LINEOUT1_VOL */
pub const WM8993_LINEOUT1_VOL_WIDTH: u32 = 1;   /* LINEOUT1_VOL */
pub const WM8993_LINEOUT2N_MUTE: u32 = 0x0004;   /* LINEOUT2N_MUTE */
pub const WM8993_LINEOUT2N_MUTE_MASK: u32 = 0x0004;   /* LINEOUT2N_MUTE */
pub const WM8993_LINEOUT2N_MUTE_SHIFT: u32 = 2;   /* LINEOUT2N_MUTE */
pub const WM8993_LINEOUT2N_MUTE_WIDTH: u32 = 1;   /* LINEOUT2N_MUTE */
pub const WM8993_LINEOUT2P_MUTE: u32 = 0x0002;   /* LINEOUT2P_MUTE */
pub const WM8993_LINEOUT2P_MUTE_MASK: u32 = 0x0002;   /* LINEOUT2P_MUTE */
pub const WM8993_LINEOUT2P_MUTE_SHIFT: u32 = 1;   /* LINEOUT2P_MUTE */
pub const WM8993_LINEOUT2P_MUTE_WIDTH: u32 = 1;   /* LINEOUT2P_MUTE */
pub const WM8993_LINEOUT2_VOL: u32 = 0x0001;   /* LINEOUT2_VOL */
pub const WM8993_LINEOUT2_VOL_MASK: u32 = 0x0001;   /* LINEOUT2_VOL */
pub const WM8993_LINEOUT2_VOL_SHIFT: u32 = 0;   /* LINEOUT2_VOL */
pub const WM8993_LINEOUT2_VOL_WIDTH: u32 = 1;   /* LINEOUT2_VOL */

/*
 * R31 (0x1F) - HPOUT2 Volume
 */
pub const WM8993_HPOUT2_MUTE: u32 = 0x0020;   /* HPOUT2_MUTE */
pub const WM8993_HPOUT2_MUTE_MASK: u32 = 0x0020;   /* HPOUT2_MUTE */
pub const WM8993_HPOUT2_MUTE_SHIFT: u32 = 5;   /* HPOUT2_MUTE */
pub const WM8993_HPOUT2_MUTE_WIDTH: u32 = 1;   /* HPOUT2_MUTE */
pub const WM8993_HPOUT2_VOL: u32 = 0x0010;   /* HPOUT2_VOL */
pub const WM8993_HPOUT2_VOL_MASK: u32 = 0x0010;   /* HPOUT2_VOL */
pub const WM8993_HPOUT2_VOL_SHIFT: u32 = 4;   /* HPOUT2_VOL */
pub const WM8993_HPOUT2_VOL_WIDTH: u32 = 1;   /* HPOUT2_VOL */

/*
 * R32 (0x20) - Left OPGA Volume
 */
pub const WM8993_MIXOUT_VU: u32 = 0x0100;   /* MIXOUT_VU */
pub const WM8993_MIXOUT_VU_MASK: u32 = 0x0100;   /* MIXOUT_VU */
pub const WM8993_MIXOUT_VU_SHIFT: u32 = 8;   /* MIXOUT_VU */
pub const WM8993_MIXOUT_VU_WIDTH: u32 = 1;   /* MIXOUT_VU */
pub const WM8993_MIXOUTL_ZC: u32 = 0x0080;   /* MIXOUTL_ZC */
pub const WM8993_MIXOUTL_ZC_MASK: u32 = 0x0080;   /* MIXOUTL_ZC */
pub const WM8993_MIXOUTL_ZC_SHIFT: u32 = 7;   /* MIXOUTL_ZC */
pub const WM8993_MIXOUTL_ZC_WIDTH: u32 = 1;   /* MIXOUTL_ZC */
pub const WM8993_MIXOUTL_MUTE_N: u32 = 0x0040;   /* MIXOUTL_MUTE_N */
pub const WM8993_MIXOUTL_MUTE_N_MASK: u32 = 0x0040;   /* MIXOUTL_MUTE_N */
pub const WM8993_MIXOUTL_MUTE_N_SHIFT: u32 = 6;   /* MIXOUTL_MUTE_N */
pub const WM8993_MIXOUTL_MUTE_N_WIDTH: u32 = 1;   /* MIXOUTL_MUTE_N */
pub const WM8993_MIXOUTL_VOL_MASK: u32 = 0x003F;   /* MIXOUTL_VOL - [5:0] */
pub const WM8993_MIXOUTL_VOL_SHIFT: u32 = 0;   /* MIXOUTL_VOL - [5:0] */
pub const WM8993_MIXOUTL_VOL_WIDTH: u32 = 6;   /* MIXOUTL_VOL - [5:0] */

/*
 * R33 (0x21) - Right OPGA Volume
 */
// Duplicate C macro: #define WM8993_MIXOUT_VU 0x0100    /* MIXOUT_VU */
// Duplicate C macro: #define WM8993_MIXOUT_VU_MASK 0x0100    /* MIXOUT_VU */
// Duplicate C macro: #define WM8993_MIXOUT_VU_SHIFT 8    /* MIXOUT_VU */
// Duplicate C macro: #define WM8993_MIXOUT_VU_WIDTH 1    /* MIXOUT_VU */
pub const WM8993_MIXOUTR_ZC: u32 = 0x0080;   /* MIXOUTR_ZC */
pub const WM8993_MIXOUTR_ZC_MASK: u32 = 0x0080;   /* MIXOUTR_ZC */
pub const WM8993_MIXOUTR_ZC_SHIFT: u32 = 7;   /* MIXOUTR_ZC */
pub const WM8993_MIXOUTR_ZC_WIDTH: u32 = 1;   /* MIXOUTR_ZC */
pub const WM8993_MIXOUTR_MUTE_N: u32 = 0x0040;   /* MIXOUTR_MUTE_N */
pub const WM8993_MIXOUTR_MUTE_N_MASK: u32 = 0x0040;   /* MIXOUTR_MUTE_N */
pub const WM8993_MIXOUTR_MUTE_N_SHIFT: u32 = 6;   /* MIXOUTR_MUTE_N */
pub const WM8993_MIXOUTR_MUTE_N_WIDTH: u32 = 1;   /* MIXOUTR_MUTE_N */
pub const WM8993_MIXOUTR_VOL_MASK: u32 = 0x003F;   /* MIXOUTR_VOL - [5:0] */
pub const WM8993_MIXOUTR_VOL_SHIFT: u32 = 0;   /* MIXOUTR_VOL - [5:0] */
pub const WM8993_MIXOUTR_VOL_WIDTH: u32 = 6;   /* MIXOUTR_VOL - [5:0] */

/*
 * R34 (0x22) - SPKMIXL Attenuation
 */
pub const WM8993_MIXINL_SPKMIXL_VOL: u32 = 0x0020;   /* MIXINL_SPKMIXL_VOL */
pub const WM8993_MIXINL_SPKMIXL_VOL_MASK: u32 = 0x0020;   /* MIXINL_SPKMIXL_VOL */
pub const WM8993_MIXINL_SPKMIXL_VOL_SHIFT: u32 = 5;   /* MIXINL_SPKMIXL_VOL */
pub const WM8993_MIXINL_SPKMIXL_VOL_WIDTH: u32 = 1;   /* MIXINL_SPKMIXL_VOL */
pub const WM8993_IN1LP_SPKMIXL_VOL: u32 = 0x0010;   /* IN1LP_SPKMIXL_VOL */
pub const WM8993_IN1LP_SPKMIXL_VOL_MASK: u32 = 0x0010;   /* IN1LP_SPKMIXL_VOL */
pub const WM8993_IN1LP_SPKMIXL_VOL_SHIFT: u32 = 4;   /* IN1LP_SPKMIXL_VOL */
pub const WM8993_IN1LP_SPKMIXL_VOL_WIDTH: u32 = 1;   /* IN1LP_SPKMIXL_VOL */
pub const WM8993_MIXOUTL_SPKMIXL_VOL: u32 = 0x0008;   /* MIXOUTL_SPKMIXL_VOL */
pub const WM8993_MIXOUTL_SPKMIXL_VOL_MASK: u32 = 0x0008;   /* MIXOUTL_SPKMIXL_VOL */
pub const WM8993_MIXOUTL_SPKMIXL_VOL_SHIFT: u32 = 3;   /* MIXOUTL_SPKMIXL_VOL */
pub const WM8993_MIXOUTL_SPKMIXL_VOL_WIDTH: u32 = 1;   /* MIXOUTL_SPKMIXL_VOL */
pub const WM8993_DACL_SPKMIXL_VOL: u32 = 0x0004;   /* DACL_SPKMIXL_VOL */
pub const WM8993_DACL_SPKMIXL_VOL_MASK: u32 = 0x0004;   /* DACL_SPKMIXL_VOL */
pub const WM8993_DACL_SPKMIXL_VOL_SHIFT: u32 = 2;   /* DACL_SPKMIXL_VOL */
pub const WM8993_DACL_SPKMIXL_VOL_WIDTH: u32 = 1;   /* DACL_SPKMIXL_VOL */
pub const WM8993_SPKMIXL_VOL_MASK: u32 = 0x0003;   /* SPKMIXL_VOL - [1:0] */
pub const WM8993_SPKMIXL_VOL_SHIFT: u32 = 0;   /* SPKMIXL_VOL - [1:0] */
pub const WM8993_SPKMIXL_VOL_WIDTH: u32 = 2;   /* SPKMIXL_VOL - [1:0] */

/*
 * R35 (0x23) - SPKMIXR Attenuation
 */
pub const WM8993_SPKOUT_CLASSAB_MODE: u32 = 0x0100;   /* SPKOUT_CLASSAB_MODE */
pub const WM8993_SPKOUT_CLASSAB_MODE_MASK: u32 = 0x0100;   /* SPKOUT_CLASSAB_MODE */
pub const WM8993_SPKOUT_CLASSAB_MODE_SHIFT: u32 = 8;   /* SPKOUT_CLASSAB_MODE */
pub const WM8993_SPKOUT_CLASSAB_MODE_WIDTH: u32 = 1;   /* SPKOUT_CLASSAB_MODE */
pub const WM8993_MIXINR_SPKMIXR_VOL: u32 = 0x0020;   /* MIXINR_SPKMIXR_VOL */
pub const WM8993_MIXINR_SPKMIXR_VOL_MASK: u32 = 0x0020;   /* MIXINR_SPKMIXR_VOL */
pub const WM8993_MIXINR_SPKMIXR_VOL_SHIFT: u32 = 5;   /* MIXINR_SPKMIXR_VOL */
pub const WM8993_MIXINR_SPKMIXR_VOL_WIDTH: u32 = 1;   /* MIXINR_SPKMIXR_VOL */
pub const WM8993_IN1RP_SPKMIXR_VOL: u32 = 0x0010;   /* IN1RP_SPKMIXR_VOL */
pub const WM8993_IN1RP_SPKMIXR_VOL_MASK: u32 = 0x0010;   /* IN1RP_SPKMIXR_VOL */
pub const WM8993_IN1RP_SPKMIXR_VOL_SHIFT: u32 = 4;   /* IN1RP_SPKMIXR_VOL */
pub const WM8993_IN1RP_SPKMIXR_VOL_WIDTH: u32 = 1;   /* IN1RP_SPKMIXR_VOL */
pub const WM8993_MIXOUTR_SPKMIXR_VOL: u32 = 0x0008;   /* MIXOUTR_SPKMIXR_VOL */
pub const WM8993_MIXOUTR_SPKMIXR_VOL_MASK: u32 = 0x0008;   /* MIXOUTR_SPKMIXR_VOL */
pub const WM8993_MIXOUTR_SPKMIXR_VOL_SHIFT: u32 = 3;   /* MIXOUTR_SPKMIXR_VOL */
pub const WM8993_MIXOUTR_SPKMIXR_VOL_WIDTH: u32 = 1;   /* MIXOUTR_SPKMIXR_VOL */
pub const WM8993_DACR_SPKMIXR_VOL: u32 = 0x0004;   /* DACR_SPKMIXR_VOL */
pub const WM8993_DACR_SPKMIXR_VOL_MASK: u32 = 0x0004;   /* DACR_SPKMIXR_VOL */
pub const WM8993_DACR_SPKMIXR_VOL_SHIFT: u32 = 2;   /* DACR_SPKMIXR_VOL */
pub const WM8993_DACR_SPKMIXR_VOL_WIDTH: u32 = 1;   /* DACR_SPKMIXR_VOL */
pub const WM8993_SPKMIXR_VOL_MASK: u32 = 0x0003;   /* SPKMIXR_VOL - [1:0] */
pub const WM8993_SPKMIXR_VOL_SHIFT: u32 = 0;   /* SPKMIXR_VOL - [1:0] */
pub const WM8993_SPKMIXR_VOL_WIDTH: u32 = 2;   /* SPKMIXR_VOL - [1:0] */

/*
 * R36 (0x24) - SPKOUT Mixers
 */
pub const WM8993_VRX_TO_SPKOUTL: u32 = 0x0020;   /* VRX_TO_SPKOUTL */
pub const WM8993_VRX_TO_SPKOUTL_MASK: u32 = 0x0020;   /* VRX_TO_SPKOUTL */
pub const WM8993_VRX_TO_SPKOUTL_SHIFT: u32 = 5;   /* VRX_TO_SPKOUTL */
pub const WM8993_VRX_TO_SPKOUTL_WIDTH: u32 = 1;   /* VRX_TO_SPKOUTL */
pub const WM8993_SPKMIXL_TO_SPKOUTL: u32 = 0x0010;   /* SPKMIXL_TO_SPKOUTL */
pub const WM8993_SPKMIXL_TO_SPKOUTL_MASK: u32 = 0x0010;   /* SPKMIXL_TO_SPKOUTL */
pub const WM8993_SPKMIXL_TO_SPKOUTL_SHIFT: u32 = 4;   /* SPKMIXL_TO_SPKOUTL */
pub const WM8993_SPKMIXL_TO_SPKOUTL_WIDTH: u32 = 1;   /* SPKMIXL_TO_SPKOUTL */
pub const WM8993_SPKMIXR_TO_SPKOUTL: u32 = 0x0008;   /* SPKMIXR_TO_SPKOUTL */
pub const WM8993_SPKMIXR_TO_SPKOUTL_MASK: u32 = 0x0008;   /* SPKMIXR_TO_SPKOUTL */
pub const WM8993_SPKMIXR_TO_SPKOUTL_SHIFT: u32 = 3;   /* SPKMIXR_TO_SPKOUTL */
pub const WM8993_SPKMIXR_TO_SPKOUTL_WIDTH: u32 = 1;   /* SPKMIXR_TO_SPKOUTL */
pub const WM8993_VRX_TO_SPKOUTR: u32 = 0x0004;   /* VRX_TO_SPKOUTR */
pub const WM8993_VRX_TO_SPKOUTR_MASK: u32 = 0x0004;   /* VRX_TO_SPKOUTR */
pub const WM8993_VRX_TO_SPKOUTR_SHIFT: u32 = 2;   /* VRX_TO_SPKOUTR */
pub const WM8993_VRX_TO_SPKOUTR_WIDTH: u32 = 1;   /* VRX_TO_SPKOUTR */
pub const WM8993_SPKMIXL_TO_SPKOUTR: u32 = 0x0002;   /* SPKMIXL_TO_SPKOUTR */
pub const WM8993_SPKMIXL_TO_SPKOUTR_MASK: u32 = 0x0002;   /* SPKMIXL_TO_SPKOUTR */
pub const WM8993_SPKMIXL_TO_SPKOUTR_SHIFT: u32 = 1;   /* SPKMIXL_TO_SPKOUTR */
pub const WM8993_SPKMIXL_TO_SPKOUTR_WIDTH: u32 = 1;   /* SPKMIXL_TO_SPKOUTR */
pub const WM8993_SPKMIXR_TO_SPKOUTR: u32 = 0x0001;   /* SPKMIXR_TO_SPKOUTR */
pub const WM8993_SPKMIXR_TO_SPKOUTR_MASK: u32 = 0x0001;   /* SPKMIXR_TO_SPKOUTR */
pub const WM8993_SPKMIXR_TO_SPKOUTR_SHIFT: u32 = 0;   /* SPKMIXR_TO_SPKOUTR */
pub const WM8993_SPKMIXR_TO_SPKOUTR_WIDTH: u32 = 1;   /* SPKMIXR_TO_SPKOUTR */

/*
 * R37 (0x25) - SPKOUT Boost
 */
pub const WM8993_SPKOUTL_BOOST_MASK: u32 = 0x0038;   /* SPKOUTL_BOOST - [5:3] */
pub const WM8993_SPKOUTL_BOOST_SHIFT: u32 = 3;   /* SPKOUTL_BOOST - [5:3] */
pub const WM8993_SPKOUTL_BOOST_WIDTH: u32 = 3;   /* SPKOUTL_BOOST - [5:3] */
pub const WM8993_SPKOUTR_BOOST_MASK: u32 = 0x0007;   /* SPKOUTR_BOOST - [2:0] */
pub const WM8993_SPKOUTR_BOOST_SHIFT: u32 = 0;   /* SPKOUTR_BOOST - [2:0] */
pub const WM8993_SPKOUTR_BOOST_WIDTH: u32 = 3;   /* SPKOUTR_BOOST - [2:0] */

/*
 * R38 (0x26) - Speaker Volume Left
 */
pub const WM8993_SPKOUT_VU: u32 = 0x0100;   /* SPKOUT_VU */
pub const WM8993_SPKOUT_VU_MASK: u32 = 0x0100;   /* SPKOUT_VU */
pub const WM8993_SPKOUT_VU_SHIFT: u32 = 8;   /* SPKOUT_VU */
pub const WM8993_SPKOUT_VU_WIDTH: u32 = 1;   /* SPKOUT_VU */
pub const WM8993_SPKOUTL_ZC: u32 = 0x0080;   /* SPKOUTL_ZC */
pub const WM8993_SPKOUTL_ZC_MASK: u32 = 0x0080;   /* SPKOUTL_ZC */
pub const WM8993_SPKOUTL_ZC_SHIFT: u32 = 7;   /* SPKOUTL_ZC */
pub const WM8993_SPKOUTL_ZC_WIDTH: u32 = 1;   /* SPKOUTL_ZC */
pub const WM8993_SPKOUTL_MUTE_N: u32 = 0x0040;   /* SPKOUTL_MUTE_N */
pub const WM8993_SPKOUTL_MUTE_N_MASK: u32 = 0x0040;   /* SPKOUTL_MUTE_N */
pub const WM8993_SPKOUTL_MUTE_N_SHIFT: u32 = 6;   /* SPKOUTL_MUTE_N */
pub const WM8993_SPKOUTL_MUTE_N_WIDTH: u32 = 1;   /* SPKOUTL_MUTE_N */
pub const WM8993_SPKOUTL_VOL_MASK: u32 = 0x003F;   /* SPKOUTL_VOL - [5:0] */
pub const WM8993_SPKOUTL_VOL_SHIFT: u32 = 0;   /* SPKOUTL_VOL - [5:0] */
pub const WM8993_SPKOUTL_VOL_WIDTH: u32 = 6;   /* SPKOUTL_VOL - [5:0] */

/*
 * R39 (0x27) - Speaker Volume Right
 */
// Duplicate C macro: #define WM8993_SPKOUT_VU 0x0100    /* SPKOUT_VU */
// Duplicate C macro: #define WM8993_SPKOUT_VU_MASK 0x0100    /* SPKOUT_VU */
// Duplicate C macro: #define WM8993_SPKOUT_VU_SHIFT 8    /* SPKOUT_VU */
// Duplicate C macro: #define WM8993_SPKOUT_VU_WIDTH 1    /* SPKOUT_VU */
pub const WM8993_SPKOUTR_ZC: u32 = 0x0080;   /* SPKOUTR_ZC */
pub const WM8993_SPKOUTR_ZC_MASK: u32 = 0x0080;   /* SPKOUTR_ZC */
pub const WM8993_SPKOUTR_ZC_SHIFT: u32 = 7;   /* SPKOUTR_ZC */
pub const WM8993_SPKOUTR_ZC_WIDTH: u32 = 1;   /* SPKOUTR_ZC */
pub const WM8993_SPKOUTR_MUTE_N: u32 = 0x0040;   /* SPKOUTR_MUTE_N */
pub const WM8993_SPKOUTR_MUTE_N_MASK: u32 = 0x0040;   /* SPKOUTR_MUTE_N */
pub const WM8993_SPKOUTR_MUTE_N_SHIFT: u32 = 6;   /* SPKOUTR_MUTE_N */
pub const WM8993_SPKOUTR_MUTE_N_WIDTH: u32 = 1;   /* SPKOUTR_MUTE_N */
pub const WM8993_SPKOUTR_VOL_MASK: u32 = 0x003F;   /* SPKOUTR_VOL - [5:0] */
pub const WM8993_SPKOUTR_VOL_SHIFT: u32 = 0;   /* SPKOUTR_VOL - [5:0] */
pub const WM8993_SPKOUTR_VOL_WIDTH: u32 = 6;   /* SPKOUTR_VOL - [5:0] */

/*
 * R40 (0x28) - Input Mixer2
 */
pub const WM8993_IN2LP_TO_IN2L: u32 = 0x0080;   /* IN2LP_TO_IN2L */
pub const WM8993_IN2LP_TO_IN2L_MASK: u32 = 0x0080;   /* IN2LP_TO_IN2L */
pub const WM8993_IN2LP_TO_IN2L_SHIFT: u32 = 7;   /* IN2LP_TO_IN2L */
pub const WM8993_IN2LP_TO_IN2L_WIDTH: u32 = 1;   /* IN2LP_TO_IN2L */
pub const WM8993_IN2LN_TO_IN2L: u32 = 0x0040;   /* IN2LN_TO_IN2L */
pub const WM8993_IN2LN_TO_IN2L_MASK: u32 = 0x0040;   /* IN2LN_TO_IN2L */
pub const WM8993_IN2LN_TO_IN2L_SHIFT: u32 = 6;   /* IN2LN_TO_IN2L */
pub const WM8993_IN2LN_TO_IN2L_WIDTH: u32 = 1;   /* IN2LN_TO_IN2L */
pub const WM8993_IN1LP_TO_IN1L: u32 = 0x0020;   /* IN1LP_TO_IN1L */
pub const WM8993_IN1LP_TO_IN1L_MASK: u32 = 0x0020;   /* IN1LP_TO_IN1L */
pub const WM8993_IN1LP_TO_IN1L_SHIFT: u32 = 5;   /* IN1LP_TO_IN1L */
pub const WM8993_IN1LP_TO_IN1L_WIDTH: u32 = 1;   /* IN1LP_TO_IN1L */
pub const WM8993_IN1LN_TO_IN1L: u32 = 0x0010;   /* IN1LN_TO_IN1L */
pub const WM8993_IN1LN_TO_IN1L_MASK: u32 = 0x0010;   /* IN1LN_TO_IN1L */
pub const WM8993_IN1LN_TO_IN1L_SHIFT: u32 = 4;   /* IN1LN_TO_IN1L */
pub const WM8993_IN1LN_TO_IN1L_WIDTH: u32 = 1;   /* IN1LN_TO_IN1L */
pub const WM8993_IN2RP_TO_IN2R: u32 = 0x0008;   /* IN2RP_TO_IN2R */
pub const WM8993_IN2RP_TO_IN2R_MASK: u32 = 0x0008;   /* IN2RP_TO_IN2R */
pub const WM8993_IN2RP_TO_IN2R_SHIFT: u32 = 3;   /* IN2RP_TO_IN2R */
pub const WM8993_IN2RP_TO_IN2R_WIDTH: u32 = 1;   /* IN2RP_TO_IN2R */
pub const WM8993_IN2RN_TO_IN2R: u32 = 0x0004;   /* IN2RN_TO_IN2R */
pub const WM8993_IN2RN_TO_IN2R_MASK: u32 = 0x0004;   /* IN2RN_TO_IN2R */
pub const WM8993_IN2RN_TO_IN2R_SHIFT: u32 = 2;   /* IN2RN_TO_IN2R */
pub const WM8993_IN2RN_TO_IN2R_WIDTH: u32 = 1;   /* IN2RN_TO_IN2R */
pub const WM8993_IN1RP_TO_IN1R: u32 = 0x0002;   /* IN1RP_TO_IN1R */
pub const WM8993_IN1RP_TO_IN1R_MASK: u32 = 0x0002;   /* IN1RP_TO_IN1R */
pub const WM8993_IN1RP_TO_IN1R_SHIFT: u32 = 1;   /* IN1RP_TO_IN1R */
pub const WM8993_IN1RP_TO_IN1R_WIDTH: u32 = 1;   /* IN1RP_TO_IN1R */
pub const WM8993_IN1RN_TO_IN1R: u32 = 0x0001;   /* IN1RN_TO_IN1R */
pub const WM8993_IN1RN_TO_IN1R_MASK: u32 = 0x0001;   /* IN1RN_TO_IN1R */
pub const WM8993_IN1RN_TO_IN1R_SHIFT: u32 = 0;   /* IN1RN_TO_IN1R */
pub const WM8993_IN1RN_TO_IN1R_WIDTH: u32 = 1;   /* IN1RN_TO_IN1R */

/*
 * R41 (0x29) - Input Mixer3
 */
pub const WM8993_IN2L_TO_MIXINL: u32 = 0x0100;   /* IN2L_TO_MIXINL */
pub const WM8993_IN2L_TO_MIXINL_MASK: u32 = 0x0100;   /* IN2L_TO_MIXINL */
pub const WM8993_IN2L_TO_MIXINL_SHIFT: u32 = 8;   /* IN2L_TO_MIXINL */
pub const WM8993_IN2L_TO_MIXINL_WIDTH: u32 = 1;   /* IN2L_TO_MIXINL */
pub const WM8993_IN2L_MIXINL_VOL: u32 = 0x0080;   /* IN2L_MIXINL_VOL */
pub const WM8993_IN2L_MIXINL_VOL_MASK: u32 = 0x0080;   /* IN2L_MIXINL_VOL */
pub const WM8993_IN2L_MIXINL_VOL_SHIFT: u32 = 7;   /* IN2L_MIXINL_VOL */
pub const WM8993_IN2L_MIXINL_VOL_WIDTH: u32 = 1;   /* IN2L_MIXINL_VOL */
pub const WM8993_IN1L_TO_MIXINL: u32 = 0x0020;   /* IN1L_TO_MIXINL */
pub const WM8993_IN1L_TO_MIXINL_MASK: u32 = 0x0020;   /* IN1L_TO_MIXINL */
pub const WM8993_IN1L_TO_MIXINL_SHIFT: u32 = 5;   /* IN1L_TO_MIXINL */
pub const WM8993_IN1L_TO_MIXINL_WIDTH: u32 = 1;   /* IN1L_TO_MIXINL */
pub const WM8993_IN1L_MIXINL_VOL: u32 = 0x0010;   /* IN1L_MIXINL_VOL */
pub const WM8993_IN1L_MIXINL_VOL_MASK: u32 = 0x0010;   /* IN1L_MIXINL_VOL */
pub const WM8993_IN1L_MIXINL_VOL_SHIFT: u32 = 4;   /* IN1L_MIXINL_VOL */
pub const WM8993_IN1L_MIXINL_VOL_WIDTH: u32 = 1;   /* IN1L_MIXINL_VOL */
pub const WM8993_MIXOUTL_MIXINL_VOL_MASK: u32 = 0x0007;   /* MIXOUTL_MIXINL_VOL - [2:0] */
pub const WM8993_MIXOUTL_MIXINL_VOL_SHIFT: u32 = 0;   /* MIXOUTL_MIXINL_VOL - [2:0] */
pub const WM8993_MIXOUTL_MIXINL_VOL_WIDTH: u32 = 3;   /* MIXOUTL_MIXINL_VOL - [2:0] */

/*
 * R42 (0x2A) - Input Mixer4
 */
pub const WM8993_IN2R_TO_MIXINR: u32 = 0x0100;   /* IN2R_TO_MIXINR */
pub const WM8993_IN2R_TO_MIXINR_MASK: u32 = 0x0100;   /* IN2R_TO_MIXINR */
pub const WM8993_IN2R_TO_MIXINR_SHIFT: u32 = 8;   /* IN2R_TO_MIXINR */
pub const WM8993_IN2R_TO_MIXINR_WIDTH: u32 = 1;   /* IN2R_TO_MIXINR */
pub const WM8993_IN2R_MIXINR_VOL: u32 = 0x0080;   /* IN2R_MIXINR_VOL */
pub const WM8993_IN2R_MIXINR_VOL_MASK: u32 = 0x0080;   /* IN2R_MIXINR_VOL */
pub const WM8993_IN2R_MIXINR_VOL_SHIFT: u32 = 7;   /* IN2R_MIXINR_VOL */
pub const WM8993_IN2R_MIXINR_VOL_WIDTH: u32 = 1;   /* IN2R_MIXINR_VOL */
pub const WM8993_IN1R_TO_MIXINR: u32 = 0x0020;   /* IN1R_TO_MIXINR */
pub const WM8993_IN1R_TO_MIXINR_MASK: u32 = 0x0020;   /* IN1R_TO_MIXINR */
pub const WM8993_IN1R_TO_MIXINR_SHIFT: u32 = 5;   /* IN1R_TO_MIXINR */
pub const WM8993_IN1R_TO_MIXINR_WIDTH: u32 = 1;   /* IN1R_TO_MIXINR */
pub const WM8993_IN1R_MIXINR_VOL: u32 = 0x0010;   /* IN1R_MIXINR_VOL */
pub const WM8993_IN1R_MIXINR_VOL_MASK: u32 = 0x0010;   /* IN1R_MIXINR_VOL */
pub const WM8993_IN1R_MIXINR_VOL_SHIFT: u32 = 4;   /* IN1R_MIXINR_VOL */
pub const WM8993_IN1R_MIXINR_VOL_WIDTH: u32 = 1;   /* IN1R_MIXINR_VOL */
pub const WM8993_MIXOUTR_MIXINR_VOL_MASK: u32 = 0x0007;   /* MIXOUTR_MIXINR_VOL - [2:0] */
pub const WM8993_MIXOUTR_MIXINR_VOL_SHIFT: u32 = 0;   /* MIXOUTR_MIXINR_VOL - [2:0] */
pub const WM8993_MIXOUTR_MIXINR_VOL_WIDTH: u32 = 3;   /* MIXOUTR_MIXINR_VOL - [2:0] */

/*
 * R43 (0x2B) - Input Mixer5
 */
pub const WM8993_IN1LP_MIXINL_VOL_MASK: u32 = 0x01C0;   /* IN1LP_MIXINL_VOL - [8:6] */
pub const WM8993_IN1LP_MIXINL_VOL_SHIFT: u32 = 6;   /* IN1LP_MIXINL_VOL - [8:6] */
pub const WM8993_IN1LP_MIXINL_VOL_WIDTH: u32 = 3;   /* IN1LP_MIXINL_VOL - [8:6] */
pub const WM8993_VRX_MIXINL_VOL_MASK: u32 = 0x0007;   /* VRX_MIXINL_VOL - [2:0] */
pub const WM8993_VRX_MIXINL_VOL_SHIFT: u32 = 0;   /* VRX_MIXINL_VOL - [2:0] */
pub const WM8993_VRX_MIXINL_VOL_WIDTH: u32 = 3;   /* VRX_MIXINL_VOL - [2:0] */

/*
 * R44 (0x2C) - Input Mixer6
 */
pub const WM8993_IN1RP_MIXINR_VOL_MASK: u32 = 0x01C0;   /* IN1RP_MIXINR_VOL - [8:6] */
pub const WM8993_IN1RP_MIXINR_VOL_SHIFT: u32 = 6;   /* IN1RP_MIXINR_VOL - [8:6] */
pub const WM8993_IN1RP_MIXINR_VOL_WIDTH: u32 = 3;   /* IN1RP_MIXINR_VOL - [8:6] */
pub const WM8993_VRX_MIXINR_VOL_MASK: u32 = 0x0007;   /* VRX_MIXINR_VOL - [2:0] */
pub const WM8993_VRX_MIXINR_VOL_SHIFT: u32 = 0;   /* VRX_MIXINR_VOL - [2:0] */
pub const WM8993_VRX_MIXINR_VOL_WIDTH: u32 = 3;   /* VRX_MIXINR_VOL - [2:0] */

/*
 * R45 (0x2D) - Output Mixer1
 */
pub const WM8993_DACL_TO_HPOUT1L: u32 = 0x0100;   /* DACL_TO_HPOUT1L */
pub const WM8993_DACL_TO_HPOUT1L_MASK: u32 = 0x0100;   /* DACL_TO_HPOUT1L */
pub const WM8993_DACL_TO_HPOUT1L_SHIFT: u32 = 8;   /* DACL_TO_HPOUT1L */
pub const WM8993_DACL_TO_HPOUT1L_WIDTH: u32 = 1;   /* DACL_TO_HPOUT1L */
pub const WM8993_MIXINR_TO_MIXOUTL: u32 = 0x0080;   /* MIXINR_TO_MIXOUTL */
pub const WM8993_MIXINR_TO_MIXOUTL_MASK: u32 = 0x0080;   /* MIXINR_TO_MIXOUTL */
pub const WM8993_MIXINR_TO_MIXOUTL_SHIFT: u32 = 7;   /* MIXINR_TO_MIXOUTL */
pub const WM8993_MIXINR_TO_MIXOUTL_WIDTH: u32 = 1;   /* MIXINR_TO_MIXOUTL */
pub const WM8993_MIXINL_TO_MIXOUTL: u32 = 0x0040;   /* MIXINL_TO_MIXOUTL */
pub const WM8993_MIXINL_TO_MIXOUTL_MASK: u32 = 0x0040;   /* MIXINL_TO_MIXOUTL */
pub const WM8993_MIXINL_TO_MIXOUTL_SHIFT: u32 = 6;   /* MIXINL_TO_MIXOUTL */
pub const WM8993_MIXINL_TO_MIXOUTL_WIDTH: u32 = 1;   /* MIXINL_TO_MIXOUTL */
pub const WM8993_IN2RN_TO_MIXOUTL: u32 = 0x0020;   /* IN2RN_TO_MIXOUTL */
pub const WM8993_IN2RN_TO_MIXOUTL_MASK: u32 = 0x0020;   /* IN2RN_TO_MIXOUTL */
pub const WM8993_IN2RN_TO_MIXOUTL_SHIFT: u32 = 5;   /* IN2RN_TO_MIXOUTL */
pub const WM8993_IN2RN_TO_MIXOUTL_WIDTH: u32 = 1;   /* IN2RN_TO_MIXOUTL */
pub const WM8993_IN2LN_TO_MIXOUTL: u32 = 0x0010;   /* IN2LN_TO_MIXOUTL */
pub const WM8993_IN2LN_TO_MIXOUTL_MASK: u32 = 0x0010;   /* IN2LN_TO_MIXOUTL */
pub const WM8993_IN2LN_TO_MIXOUTL_SHIFT: u32 = 4;   /* IN2LN_TO_MIXOUTL */
pub const WM8993_IN2LN_TO_MIXOUTL_WIDTH: u32 = 1;   /* IN2LN_TO_MIXOUTL */
pub const WM8993_IN1R_TO_MIXOUTL: u32 = 0x0008;   /* IN1R_TO_MIXOUTL */
pub const WM8993_IN1R_TO_MIXOUTL_MASK: u32 = 0x0008;   /* IN1R_TO_MIXOUTL */
pub const WM8993_IN1R_TO_MIXOUTL_SHIFT: u32 = 3;   /* IN1R_TO_MIXOUTL */
pub const WM8993_IN1R_TO_MIXOUTL_WIDTH: u32 = 1;   /* IN1R_TO_MIXOUTL */
pub const WM8993_IN1L_TO_MIXOUTL: u32 = 0x0004;   /* IN1L_TO_MIXOUTL */
pub const WM8993_IN1L_TO_MIXOUTL_MASK: u32 = 0x0004;   /* IN1L_TO_MIXOUTL */
pub const WM8993_IN1L_TO_MIXOUTL_SHIFT: u32 = 2;   /* IN1L_TO_MIXOUTL */
pub const WM8993_IN1L_TO_MIXOUTL_WIDTH: u32 = 1;   /* IN1L_TO_MIXOUTL */
pub const WM8993_IN2LP_TO_MIXOUTL: u32 = 0x0002;   /* IN2LP_TO_MIXOUTL */
pub const WM8993_IN2LP_TO_MIXOUTL_MASK: u32 = 0x0002;   /* IN2LP_TO_MIXOUTL */
pub const WM8993_IN2LP_TO_MIXOUTL_SHIFT: u32 = 1;   /* IN2LP_TO_MIXOUTL */
pub const WM8993_IN2LP_TO_MIXOUTL_WIDTH: u32 = 1;   /* IN2LP_TO_MIXOUTL */
pub const WM8993_DACL_TO_MIXOUTL: u32 = 0x0001;   /* DACL_TO_MIXOUTL */
pub const WM8993_DACL_TO_MIXOUTL_MASK: u32 = 0x0001;   /* DACL_TO_MIXOUTL */
pub const WM8993_DACL_TO_MIXOUTL_SHIFT: u32 = 0;   /* DACL_TO_MIXOUTL */
pub const WM8993_DACL_TO_MIXOUTL_WIDTH: u32 = 1;   /* DACL_TO_MIXOUTL */

/*
 * R46 (0x2E) - Output Mixer2
 */
pub const WM8993_DACR_TO_HPOUT1R: u32 = 0x0100;   /* DACR_TO_HPOUT1R */
pub const WM8993_DACR_TO_HPOUT1R_MASK: u32 = 0x0100;   /* DACR_TO_HPOUT1R */
pub const WM8993_DACR_TO_HPOUT1R_SHIFT: u32 = 8;   /* DACR_TO_HPOUT1R */
pub const WM8993_DACR_TO_HPOUT1R_WIDTH: u32 = 1;   /* DACR_TO_HPOUT1R */
pub const WM8993_MIXINL_TO_MIXOUTR: u32 = 0x0080;   /* MIXINL_TO_MIXOUTR */
pub const WM8993_MIXINL_TO_MIXOUTR_MASK: u32 = 0x0080;   /* MIXINL_TO_MIXOUTR */
pub const WM8993_MIXINL_TO_MIXOUTR_SHIFT: u32 = 7;   /* MIXINL_TO_MIXOUTR */
pub const WM8993_MIXINL_TO_MIXOUTR_WIDTH: u32 = 1;   /* MIXINL_TO_MIXOUTR */
pub const WM8993_MIXINR_TO_MIXOUTR: u32 = 0x0040;   /* MIXINR_TO_MIXOUTR */
pub const WM8993_MIXINR_TO_MIXOUTR_MASK: u32 = 0x0040;   /* MIXINR_TO_MIXOUTR */
pub const WM8993_MIXINR_TO_MIXOUTR_SHIFT: u32 = 6;   /* MIXINR_TO_MIXOUTR */
pub const WM8993_MIXINR_TO_MIXOUTR_WIDTH: u32 = 1;   /* MIXINR_TO_MIXOUTR */
pub const WM8993_IN2LN_TO_MIXOUTR: u32 = 0x0020;   /* IN2LN_TO_MIXOUTR */
pub const WM8993_IN2LN_TO_MIXOUTR_MASK: u32 = 0x0020;   /* IN2LN_TO_MIXOUTR */
pub const WM8993_IN2LN_TO_MIXOUTR_SHIFT: u32 = 5;   /* IN2LN_TO_MIXOUTR */
pub const WM8993_IN2LN_TO_MIXOUTR_WIDTH: u32 = 1;   /* IN2LN_TO_MIXOUTR */
pub const WM8993_IN2RN_TO_MIXOUTR: u32 = 0x0010;   /* IN2RN_TO_MIXOUTR */
pub const WM8993_IN2RN_TO_MIXOUTR_MASK: u32 = 0x0010;   /* IN2RN_TO_MIXOUTR */
pub const WM8993_IN2RN_TO_MIXOUTR_SHIFT: u32 = 4;   /* IN2RN_TO_MIXOUTR */
pub const WM8993_IN2RN_TO_MIXOUTR_WIDTH: u32 = 1;   /* IN2RN_TO_MIXOUTR */
pub const WM8993_IN1L_TO_MIXOUTR: u32 = 0x0008;   /* IN1L_TO_MIXOUTR */
pub const WM8993_IN1L_TO_MIXOUTR_MASK: u32 = 0x0008;   /* IN1L_TO_MIXOUTR */
pub const WM8993_IN1L_TO_MIXOUTR_SHIFT: u32 = 3;   /* IN1L_TO_MIXOUTR */
pub const WM8993_IN1L_TO_MIXOUTR_WIDTH: u32 = 1;   /* IN1L_TO_MIXOUTR */
pub const WM8993_IN1R_TO_MIXOUTR: u32 = 0x0004;   /* IN1R_TO_MIXOUTR */
pub const WM8993_IN1R_TO_MIXOUTR_MASK: u32 = 0x0004;   /* IN1R_TO_MIXOUTR */
pub const WM8993_IN1R_TO_MIXOUTR_SHIFT: u32 = 2;   /* IN1R_TO_MIXOUTR */
pub const WM8993_IN1R_TO_MIXOUTR_WIDTH: u32 = 1;   /* IN1R_TO_MIXOUTR */
pub const WM8993_IN2RP_TO_MIXOUTR: u32 = 0x0002;   /* IN2RP_TO_MIXOUTR */
pub const WM8993_IN2RP_TO_MIXOUTR_MASK: u32 = 0x0002;   /* IN2RP_TO_MIXOUTR */
pub const WM8993_IN2RP_TO_MIXOUTR_SHIFT: u32 = 1;   /* IN2RP_TO_MIXOUTR */
pub const WM8993_IN2RP_TO_MIXOUTR_WIDTH: u32 = 1;   /* IN2RP_TO_MIXOUTR */
pub const WM8993_DACR_TO_MIXOUTR: u32 = 0x0001;   /* DACR_TO_MIXOUTR */
pub const WM8993_DACR_TO_MIXOUTR_MASK: u32 = 0x0001;   /* DACR_TO_MIXOUTR */
pub const WM8993_DACR_TO_MIXOUTR_SHIFT: u32 = 0;   /* DACR_TO_MIXOUTR */
pub const WM8993_DACR_TO_MIXOUTR_WIDTH: u32 = 1;   /* DACR_TO_MIXOUTR */

/*
 * R47 (0x2F) - Output Mixer3
 */
pub const WM8993_IN2LP_MIXOUTL_VOL_MASK: u32 = 0x0E00;   /* IN2LP_MIXOUTL_VOL - [11:9] */
pub const WM8993_IN2LP_MIXOUTL_VOL_SHIFT: u32 = 9;   /* IN2LP_MIXOUTL_VOL - [11:9] */
pub const WM8993_IN2LP_MIXOUTL_VOL_WIDTH: u32 = 3;   /* IN2LP_MIXOUTL_VOL - [11:9] */
pub const WM8993_IN2LN_MIXOUTL_VOL_MASK: u32 = 0x01C0;   /* IN2LN_MIXOUTL_VOL - [8:6] */
pub const WM8993_IN2LN_MIXOUTL_VOL_SHIFT: u32 = 6;   /* IN2LN_MIXOUTL_VOL - [8:6] */
pub const WM8993_IN2LN_MIXOUTL_VOL_WIDTH: u32 = 3;   /* IN2LN_MIXOUTL_VOL - [8:6] */
pub const WM8993_IN1R_MIXOUTL_VOL_MASK: u32 = 0x0038;   /* IN1R_MIXOUTL_VOL - [5:3] */
pub const WM8993_IN1R_MIXOUTL_VOL_SHIFT: u32 = 3;   /* IN1R_MIXOUTL_VOL - [5:3] */
pub const WM8993_IN1R_MIXOUTL_VOL_WIDTH: u32 = 3;   /* IN1R_MIXOUTL_VOL - [5:3] */
pub const WM8993_IN1L_MIXOUTL_VOL_MASK: u32 = 0x0007;   /* IN1L_MIXOUTL_VOL - [2:0] */
pub const WM8993_IN1L_MIXOUTL_VOL_SHIFT: u32 = 0;   /* IN1L_MIXOUTL_VOL - [2:0] */
pub const WM8993_IN1L_MIXOUTL_VOL_WIDTH: u32 = 3;   /* IN1L_MIXOUTL_VOL - [2:0] */

/*
 * R48 (0x30) - Output Mixer4
 */
pub const WM8993_IN2RP_MIXOUTR_VOL_MASK: u32 = 0x0E00;   /* IN2RP_MIXOUTR_VOL - [11:9] */
pub const WM8993_IN2RP_MIXOUTR_VOL_SHIFT: u32 = 9;   /* IN2RP_MIXOUTR_VOL - [11:9] */
pub const WM8993_IN2RP_MIXOUTR_VOL_WIDTH: u32 = 3;   /* IN2RP_MIXOUTR_VOL - [11:9] */
pub const WM8993_IN2RN_MIXOUTR_VOL_MASK: u32 = 0x01C0;   /* IN2RN_MIXOUTR_VOL - [8:6] */
pub const WM8993_IN2RN_MIXOUTR_VOL_SHIFT: u32 = 6;   /* IN2RN_MIXOUTR_VOL - [8:6] */
pub const WM8993_IN2RN_MIXOUTR_VOL_WIDTH: u32 = 3;   /* IN2RN_MIXOUTR_VOL - [8:6] */
pub const WM8993_IN1L_MIXOUTR_VOL_MASK: u32 = 0x0038;   /* IN1L_MIXOUTR_VOL - [5:3] */
pub const WM8993_IN1L_MIXOUTR_VOL_SHIFT: u32 = 3;   /* IN1L_MIXOUTR_VOL - [5:3] */
pub const WM8993_IN1L_MIXOUTR_VOL_WIDTH: u32 = 3;   /* IN1L_MIXOUTR_VOL - [5:3] */
pub const WM8993_IN1R_MIXOUTR_VOL_MASK: u32 = 0x0007;   /* IN1R_MIXOUTR_VOL - [2:0] */
pub const WM8993_IN1R_MIXOUTR_VOL_SHIFT: u32 = 0;   /* IN1R_MIXOUTR_VOL - [2:0] */
pub const WM8993_IN1R_MIXOUTR_VOL_WIDTH: u32 = 3;   /* IN1R_MIXOUTR_VOL - [2:0] */

/*
 * R49 (0x31) - Output Mixer5
 */
pub const WM8993_DACL_MIXOUTL_VOL_MASK: u32 = 0x0E00;   /* DACL_MIXOUTL_VOL - [11:9] */
pub const WM8993_DACL_MIXOUTL_VOL_SHIFT: u32 = 9;   /* DACL_MIXOUTL_VOL - [11:9] */
pub const WM8993_DACL_MIXOUTL_VOL_WIDTH: u32 = 3;   /* DACL_MIXOUTL_VOL - [11:9] */
pub const WM8993_IN2RN_MIXOUTL_VOL_MASK: u32 = 0x01C0;   /* IN2RN_MIXOUTL_VOL - [8:6] */
pub const WM8993_IN2RN_MIXOUTL_VOL_SHIFT: u32 = 6;   /* IN2RN_MIXOUTL_VOL - [8:6] */
pub const WM8993_IN2RN_MIXOUTL_VOL_WIDTH: u32 = 3;   /* IN2RN_MIXOUTL_VOL - [8:6] */
pub const WM8993_MIXINR_MIXOUTL_VOL_MASK: u32 = 0x0038;   /* MIXINR_MIXOUTL_VOL - [5:3] */
pub const WM8993_MIXINR_MIXOUTL_VOL_SHIFT: u32 = 3;   /* MIXINR_MIXOUTL_VOL - [5:3] */
pub const WM8993_MIXINR_MIXOUTL_VOL_WIDTH: u32 = 3;   /* MIXINR_MIXOUTL_VOL - [5:3] */
pub const WM8993_MIXINL_MIXOUTL_VOL_MASK: u32 = 0x0007;   /* MIXINL_MIXOUTL_VOL - [2:0] */
pub const WM8993_MIXINL_MIXOUTL_VOL_SHIFT: u32 = 0;   /* MIXINL_MIXOUTL_VOL - [2:0] */
pub const WM8993_MIXINL_MIXOUTL_VOL_WIDTH: u32 = 3;   /* MIXINL_MIXOUTL_VOL - [2:0] */

/*
 * R50 (0x32) - Output Mixer6
 */
pub const WM8993_DACR_MIXOUTR_VOL_MASK: u32 = 0x0E00;   /* DACR_MIXOUTR_VOL - [11:9] */
pub const WM8993_DACR_MIXOUTR_VOL_SHIFT: u32 = 9;   /* DACR_MIXOUTR_VOL - [11:9] */
pub const WM8993_DACR_MIXOUTR_VOL_WIDTH: u32 = 3;   /* DACR_MIXOUTR_VOL - [11:9] */
pub const WM8993_IN2LN_MIXOUTR_VOL_MASK: u32 = 0x01C0;   /* IN2LN_MIXOUTR_VOL - [8:6] */
pub const WM8993_IN2LN_MIXOUTR_VOL_SHIFT: u32 = 6;   /* IN2LN_MIXOUTR_VOL - [8:6] */
pub const WM8993_IN2LN_MIXOUTR_VOL_WIDTH: u32 = 3;   /* IN2LN_MIXOUTR_VOL - [8:6] */
pub const WM8993_MIXINL_MIXOUTR_VOL_MASK: u32 = 0x0038;   /* MIXINL_MIXOUTR_VOL - [5:3] */
pub const WM8993_MIXINL_MIXOUTR_VOL_SHIFT: u32 = 3;   /* MIXINL_MIXOUTR_VOL - [5:3] */
pub const WM8993_MIXINL_MIXOUTR_VOL_WIDTH: u32 = 3;   /* MIXINL_MIXOUTR_VOL - [5:3] */
pub const WM8993_MIXINR_MIXOUTR_VOL_MASK: u32 = 0x0007;   /* MIXINR_MIXOUTR_VOL - [2:0] */
pub const WM8993_MIXINR_MIXOUTR_VOL_SHIFT: u32 = 0;   /* MIXINR_MIXOUTR_VOL - [2:0] */
pub const WM8993_MIXINR_MIXOUTR_VOL_WIDTH: u32 = 3;   /* MIXINR_MIXOUTR_VOL - [2:0] */

/*
 * R51 (0x33) - HPOUT2 Mixer
 */
pub const WM8993_VRX_TO_HPOUT2: u32 = 0x0020;   /* VRX_TO_HPOUT2 */
pub const WM8993_VRX_TO_HPOUT2_MASK: u32 = 0x0020;   /* VRX_TO_HPOUT2 */
pub const WM8993_VRX_TO_HPOUT2_SHIFT: u32 = 5;   /* VRX_TO_HPOUT2 */
pub const WM8993_VRX_TO_HPOUT2_WIDTH: u32 = 1;   /* VRX_TO_HPOUT2 */
pub const WM8993_MIXOUTLVOL_TO_HPOUT2: u32 = 0x0010;   /* MIXOUTLVOL_TO_HPOUT2 */
pub const WM8993_MIXOUTLVOL_TO_HPOUT2_MASK: u32 = 0x0010;   /* MIXOUTLVOL_TO_HPOUT2 */
pub const WM8993_MIXOUTLVOL_TO_HPOUT2_SHIFT: u32 = 4;   /* MIXOUTLVOL_TO_HPOUT2 */
pub const WM8993_MIXOUTLVOL_TO_HPOUT2_WIDTH: u32 = 1;   /* MIXOUTLVOL_TO_HPOUT2 */
pub const WM8993_MIXOUTRVOL_TO_HPOUT2: u32 = 0x0008;   /* MIXOUTRVOL_TO_HPOUT2 */
pub const WM8993_MIXOUTRVOL_TO_HPOUT2_MASK: u32 = 0x0008;   /* MIXOUTRVOL_TO_HPOUT2 */
pub const WM8993_MIXOUTRVOL_TO_HPOUT2_SHIFT: u32 = 3;   /* MIXOUTRVOL_TO_HPOUT2 */
pub const WM8993_MIXOUTRVOL_TO_HPOUT2_WIDTH: u32 = 1;   /* MIXOUTRVOL_TO_HPOUT2 */

/*
 * R52 (0x34) - Line Mixer1
 */
pub const WM8993_MIXOUTL_TO_LINEOUT1N: u32 = 0x0040;   /* MIXOUTL_TO_LINEOUT1N */
pub const WM8993_MIXOUTL_TO_LINEOUT1N_MASK: u32 = 0x0040;   /* MIXOUTL_TO_LINEOUT1N */
pub const WM8993_MIXOUTL_TO_LINEOUT1N_SHIFT: u32 = 6;   /* MIXOUTL_TO_LINEOUT1N */
pub const WM8993_MIXOUTL_TO_LINEOUT1N_WIDTH: u32 = 1;   /* MIXOUTL_TO_LINEOUT1N */
pub const WM8993_MIXOUTR_TO_LINEOUT1N: u32 = 0x0020;   /* MIXOUTR_TO_LINEOUT1N */
pub const WM8993_MIXOUTR_TO_LINEOUT1N_MASK: u32 = 0x0020;   /* MIXOUTR_TO_LINEOUT1N */
pub const WM8993_MIXOUTR_TO_LINEOUT1N_SHIFT: u32 = 5;   /* MIXOUTR_TO_LINEOUT1N */
pub const WM8993_MIXOUTR_TO_LINEOUT1N_WIDTH: u32 = 1;   /* MIXOUTR_TO_LINEOUT1N */
pub const WM8993_LINEOUT1_MODE: u32 = 0x0010;   /* LINEOUT1_MODE */
pub const WM8993_LINEOUT1_MODE_MASK: u32 = 0x0010;   /* LINEOUT1_MODE */
pub const WM8993_LINEOUT1_MODE_SHIFT: u32 = 4;   /* LINEOUT1_MODE */
pub const WM8993_LINEOUT1_MODE_WIDTH: u32 = 1;   /* LINEOUT1_MODE */
pub const WM8993_IN1R_TO_LINEOUT1P: u32 = 0x0004;   /* IN1R_TO_LINEOUT1P */
pub const WM8993_IN1R_TO_LINEOUT1P_MASK: u32 = 0x0004;   /* IN1R_TO_LINEOUT1P */
pub const WM8993_IN1R_TO_LINEOUT1P_SHIFT: u32 = 2;   /* IN1R_TO_LINEOUT1P */
pub const WM8993_IN1R_TO_LINEOUT1P_WIDTH: u32 = 1;   /* IN1R_TO_LINEOUT1P */
pub const WM8993_IN1L_TO_LINEOUT1P: u32 = 0x0002;   /* IN1L_TO_LINEOUT1P */
pub const WM8993_IN1L_TO_LINEOUT1P_MASK: u32 = 0x0002;   /* IN1L_TO_LINEOUT1P */
pub const WM8993_IN1L_TO_LINEOUT1P_SHIFT: u32 = 1;   /* IN1L_TO_LINEOUT1P */
pub const WM8993_IN1L_TO_LINEOUT1P_WIDTH: u32 = 1;   /* IN1L_TO_LINEOUT1P */
pub const WM8993_MIXOUTL_TO_LINEOUT1P: u32 = 0x0001;   /* MIXOUTL_TO_LINEOUT1P */
pub const WM8993_MIXOUTL_TO_LINEOUT1P_MASK: u32 = 0x0001;   /* MIXOUTL_TO_LINEOUT1P */
pub const WM8993_MIXOUTL_TO_LINEOUT1P_SHIFT: u32 = 0;   /* MIXOUTL_TO_LINEOUT1P */
pub const WM8993_MIXOUTL_TO_LINEOUT1P_WIDTH: u32 = 1;   /* MIXOUTL_TO_LINEOUT1P */

/*
 * R53 (0x35) - Line Mixer2
 */
pub const WM8993_MIXOUTR_TO_LINEOUT2N: u32 = 0x0040;   /* MIXOUTR_TO_LINEOUT2N */
pub const WM8993_MIXOUTR_TO_LINEOUT2N_MASK: u32 = 0x0040;   /* MIXOUTR_TO_LINEOUT2N */
pub const WM8993_MIXOUTR_TO_LINEOUT2N_SHIFT: u32 = 6;   /* MIXOUTR_TO_LINEOUT2N */
pub const WM8993_MIXOUTR_TO_LINEOUT2N_WIDTH: u32 = 1;   /* MIXOUTR_TO_LINEOUT2N */
pub const WM8993_MIXOUTL_TO_LINEOUT2N: u32 = 0x0020;   /* MIXOUTL_TO_LINEOUT2N */
pub const WM8993_MIXOUTL_TO_LINEOUT2N_MASK: u32 = 0x0020;   /* MIXOUTL_TO_LINEOUT2N */
pub const WM8993_MIXOUTL_TO_LINEOUT2N_SHIFT: u32 = 5;   /* MIXOUTL_TO_LINEOUT2N */
pub const WM8993_MIXOUTL_TO_LINEOUT2N_WIDTH: u32 = 1;   /* MIXOUTL_TO_LINEOUT2N */
pub const WM8993_LINEOUT2_MODE: u32 = 0x0010;   /* LINEOUT2_MODE */
pub const WM8993_LINEOUT2_MODE_MASK: u32 = 0x0010;   /* LINEOUT2_MODE */
pub const WM8993_LINEOUT2_MODE_SHIFT: u32 = 4;   /* LINEOUT2_MODE */
pub const WM8993_LINEOUT2_MODE_WIDTH: u32 = 1;   /* LINEOUT2_MODE */
pub const WM8993_IN1L_TO_LINEOUT2P: u32 = 0x0004;   /* IN1L_TO_LINEOUT2P */
pub const WM8993_IN1L_TO_LINEOUT2P_MASK: u32 = 0x0004;   /* IN1L_TO_LINEOUT2P */
pub const WM8993_IN1L_TO_LINEOUT2P_SHIFT: u32 = 2;   /* IN1L_TO_LINEOUT2P */
pub const WM8993_IN1L_TO_LINEOUT2P_WIDTH: u32 = 1;   /* IN1L_TO_LINEOUT2P */
pub const WM8993_IN1R_TO_LINEOUT2P: u32 = 0x0002;   /* IN1R_TO_LINEOUT2P */
pub const WM8993_IN1R_TO_LINEOUT2P_MASK: u32 = 0x0002;   /* IN1R_TO_LINEOUT2P */
pub const WM8993_IN1R_TO_LINEOUT2P_SHIFT: u32 = 1;   /* IN1R_TO_LINEOUT2P */
pub const WM8993_IN1R_TO_LINEOUT2P_WIDTH: u32 = 1;   /* IN1R_TO_LINEOUT2P */
pub const WM8993_MIXOUTR_TO_LINEOUT2P: u32 = 0x0001;   /* MIXOUTR_TO_LINEOUT2P */
pub const WM8993_MIXOUTR_TO_LINEOUT2P_MASK: u32 = 0x0001;   /* MIXOUTR_TO_LINEOUT2P */
pub const WM8993_MIXOUTR_TO_LINEOUT2P_SHIFT: u32 = 0;   /* MIXOUTR_TO_LINEOUT2P */
pub const WM8993_MIXOUTR_TO_LINEOUT2P_WIDTH: u32 = 1;   /* MIXOUTR_TO_LINEOUT2P */

/*
 * R54 (0x36) - Speaker Mixer
 */
pub const WM8993_SPKAB_REF_SEL: u32 = 0x0100;   /* SPKAB_REF_SEL */
pub const WM8993_SPKAB_REF_SEL_MASK: u32 = 0x0100;   /* SPKAB_REF_SEL */
pub const WM8993_SPKAB_REF_SEL_SHIFT: u32 = 8;   /* SPKAB_REF_SEL */
pub const WM8993_SPKAB_REF_SEL_WIDTH: u32 = 1;   /* SPKAB_REF_SEL */
pub const WM8993_MIXINL_TO_SPKMIXL: u32 = 0x0080;   /* MIXINL_TO_SPKMIXL */
pub const WM8993_MIXINL_TO_SPKMIXL_MASK: u32 = 0x0080;   /* MIXINL_TO_SPKMIXL */
pub const WM8993_MIXINL_TO_SPKMIXL_SHIFT: u32 = 7;   /* MIXINL_TO_SPKMIXL */
pub const WM8993_MIXINL_TO_SPKMIXL_WIDTH: u32 = 1;   /* MIXINL_TO_SPKMIXL */
pub const WM8993_MIXINR_TO_SPKMIXR: u32 = 0x0040;   /* MIXINR_TO_SPKMIXR */
pub const WM8993_MIXINR_TO_SPKMIXR_MASK: u32 = 0x0040;   /* MIXINR_TO_SPKMIXR */
pub const WM8993_MIXINR_TO_SPKMIXR_SHIFT: u32 = 6;   /* MIXINR_TO_SPKMIXR */
pub const WM8993_MIXINR_TO_SPKMIXR_WIDTH: u32 = 1;   /* MIXINR_TO_SPKMIXR */
pub const WM8993_IN1LP_TO_SPKMIXL: u32 = 0x0020;   /* IN1LP_TO_SPKMIXL */
pub const WM8993_IN1LP_TO_SPKMIXL_MASK: u32 = 0x0020;   /* IN1LP_TO_SPKMIXL */
pub const WM8993_IN1LP_TO_SPKMIXL_SHIFT: u32 = 5;   /* IN1LP_TO_SPKMIXL */
pub const WM8993_IN1LP_TO_SPKMIXL_WIDTH: u32 = 1;   /* IN1LP_TO_SPKMIXL */
pub const WM8993_IN1RP_TO_SPKMIXR: u32 = 0x0010;   /* IN1RP_TO_SPKMIXR */
pub const WM8993_IN1RP_TO_SPKMIXR_MASK: u32 = 0x0010;   /* IN1RP_TO_SPKMIXR */
pub const WM8993_IN1RP_TO_SPKMIXR_SHIFT: u32 = 4;   /* IN1RP_TO_SPKMIXR */
pub const WM8993_IN1RP_TO_SPKMIXR_WIDTH: u32 = 1;   /* IN1RP_TO_SPKMIXR */
pub const WM8993_MIXOUTL_TO_SPKMIXL: u32 = 0x0008;   /* MIXOUTL_TO_SPKMIXL */
pub const WM8993_MIXOUTL_TO_SPKMIXL_MASK: u32 = 0x0008;   /* MIXOUTL_TO_SPKMIXL */
pub const WM8993_MIXOUTL_TO_SPKMIXL_SHIFT: u32 = 3;   /* MIXOUTL_TO_SPKMIXL */
pub const WM8993_MIXOUTL_TO_SPKMIXL_WIDTH: u32 = 1;   /* MIXOUTL_TO_SPKMIXL */
pub const WM8993_MIXOUTR_TO_SPKMIXR: u32 = 0x0004;   /* MIXOUTR_TO_SPKMIXR */
pub const WM8993_MIXOUTR_TO_SPKMIXR_MASK: u32 = 0x0004;   /* MIXOUTR_TO_SPKMIXR */
pub const WM8993_MIXOUTR_TO_SPKMIXR_SHIFT: u32 = 2;   /* MIXOUTR_TO_SPKMIXR */
pub const WM8993_MIXOUTR_TO_SPKMIXR_WIDTH: u32 = 1;   /* MIXOUTR_TO_SPKMIXR */
pub const WM8993_DACL_TO_SPKMIXL: u32 = 0x0002;   /* DACL_TO_SPKMIXL */
pub const WM8993_DACL_TO_SPKMIXL_MASK: u32 = 0x0002;   /* DACL_TO_SPKMIXL */
pub const WM8993_DACL_TO_SPKMIXL_SHIFT: u32 = 1;   /* DACL_TO_SPKMIXL */
pub const WM8993_DACL_TO_SPKMIXL_WIDTH: u32 = 1;   /* DACL_TO_SPKMIXL */
pub const WM8993_DACR_TO_SPKMIXR: u32 = 0x0001;   /* DACR_TO_SPKMIXR */
pub const WM8993_DACR_TO_SPKMIXR_MASK: u32 = 0x0001;   /* DACR_TO_SPKMIXR */
pub const WM8993_DACR_TO_SPKMIXR_SHIFT: u32 = 0;   /* DACR_TO_SPKMIXR */
pub const WM8993_DACR_TO_SPKMIXR_WIDTH: u32 = 1;   /* DACR_TO_SPKMIXR */

/*
 * R55 (0x37) - Additional Control
 */
pub const WM8993_LINEOUT1_FB: u32 = 0x0080;   /* LINEOUT1_FB */
pub const WM8993_LINEOUT1_FB_MASK: u32 = 0x0080;   /* LINEOUT1_FB */
pub const WM8993_LINEOUT1_FB_SHIFT: u32 = 7;   /* LINEOUT1_FB */
pub const WM8993_LINEOUT1_FB_WIDTH: u32 = 1;   /* LINEOUT1_FB */
pub const WM8993_LINEOUT2_FB: u32 = 0x0040;   /* LINEOUT2_FB */
pub const WM8993_LINEOUT2_FB_MASK: u32 = 0x0040;   /* LINEOUT2_FB */
pub const WM8993_LINEOUT2_FB_SHIFT: u32 = 6;   /* LINEOUT2_FB */
pub const WM8993_LINEOUT2_FB_WIDTH: u32 = 1;   /* LINEOUT2_FB */
pub const WM8993_VROI: u32 = 0x0001;   /* VROI */
pub const WM8993_VROI_MASK: u32 = 0x0001;   /* VROI */
pub const WM8993_VROI_SHIFT: u32 = 0;   /* VROI */
pub const WM8993_VROI_WIDTH: u32 = 1;   /* VROI */

/*
 * R56 (0x38) - AntiPOP1
 */
pub const WM8993_LINEOUT_VMID_BUF_ENA: u32 = 0x0080;   /* LINEOUT_VMID_BUF_ENA */
pub const WM8993_LINEOUT_VMID_BUF_ENA_MASK: u32 = 0x0080;   /* LINEOUT_VMID_BUF_ENA */
pub const WM8993_LINEOUT_VMID_BUF_ENA_SHIFT: u32 = 7;   /* LINEOUT_VMID_BUF_ENA */
pub const WM8993_LINEOUT_VMID_BUF_ENA_WIDTH: u32 = 1;   /* LINEOUT_VMID_BUF_ENA */
pub const WM8993_HPOUT2_IN_ENA: u32 = 0x0040;   /* HPOUT2_IN_ENA */
pub const WM8993_HPOUT2_IN_ENA_MASK: u32 = 0x0040;   /* HPOUT2_IN_ENA */
pub const WM8993_HPOUT2_IN_ENA_SHIFT: u32 = 6;   /* HPOUT2_IN_ENA */
pub const WM8993_HPOUT2_IN_ENA_WIDTH: u32 = 1;   /* HPOUT2_IN_ENA */
pub const WM8993_LINEOUT1_DISCH: u32 = 0x0020;   /* LINEOUT1_DISCH */
pub const WM8993_LINEOUT1_DISCH_MASK: u32 = 0x0020;   /* LINEOUT1_DISCH */
pub const WM8993_LINEOUT1_DISCH_SHIFT: u32 = 5;   /* LINEOUT1_DISCH */
pub const WM8993_LINEOUT1_DISCH_WIDTH: u32 = 1;   /* LINEOUT1_DISCH */
pub const WM8993_LINEOUT2_DISCH: u32 = 0x0010;   /* LINEOUT2_DISCH */
pub const WM8993_LINEOUT2_DISCH_MASK: u32 = 0x0010;   /* LINEOUT2_DISCH */
pub const WM8993_LINEOUT2_DISCH_SHIFT: u32 = 4;   /* LINEOUT2_DISCH */
pub const WM8993_LINEOUT2_DISCH_WIDTH: u32 = 1;   /* LINEOUT2_DISCH */

/*
 * R57 (0x39) - AntiPOP2
 */
pub const WM8993_VMID_RAMP_MASK: u32 = 0x0060;   /* VMID_RAMP - [6:5] */
pub const WM8993_VMID_RAMP_SHIFT: u32 = 5;   /* VMID_RAMP - [6:5] */
pub const WM8993_VMID_RAMP_WIDTH: u32 = 2;   /* VMID_RAMP - [6:5] */
pub const WM8993_VMID_BUF_ENA: u32 = 0x0008;   /* VMID_BUF_ENA */
pub const WM8993_VMID_BUF_ENA_MASK: u32 = 0x0008;   /* VMID_BUF_ENA */
pub const WM8993_VMID_BUF_ENA_SHIFT: u32 = 3;   /* VMID_BUF_ENA */
pub const WM8993_VMID_BUF_ENA_WIDTH: u32 = 1;   /* VMID_BUF_ENA */
pub const WM8993_STARTUP_BIAS_ENA: u32 = 0x0004;   /* STARTUP_BIAS_ENA */
pub const WM8993_STARTUP_BIAS_ENA_MASK: u32 = 0x0004;   /* STARTUP_BIAS_ENA */
pub const WM8993_STARTUP_BIAS_ENA_SHIFT: u32 = 2;   /* STARTUP_BIAS_ENA */
pub const WM8993_STARTUP_BIAS_ENA_WIDTH: u32 = 1;   /* STARTUP_BIAS_ENA */
pub const WM8993_BIAS_SRC: u32 = 0x0002;   /* BIAS_SRC */
pub const WM8993_BIAS_SRC_MASK: u32 = 0x0002;   /* BIAS_SRC */
pub const WM8993_BIAS_SRC_SHIFT: u32 = 1;   /* BIAS_SRC */
pub const WM8993_BIAS_SRC_WIDTH: u32 = 1;   /* BIAS_SRC */
pub const WM8993_VMID_DISCH: u32 = 0x0001;   /* VMID_DISCH */
pub const WM8993_VMID_DISCH_MASK: u32 = 0x0001;   /* VMID_DISCH */
pub const WM8993_VMID_DISCH_SHIFT: u32 = 0;   /* VMID_DISCH */
pub const WM8993_VMID_DISCH_WIDTH: u32 = 1;   /* VMID_DISCH */

/*
 * R58 (0x3A) - MICBIAS
 */
pub const WM8993_JD_SCTHR_MASK: u32 = 0x00C0;   /* JD_SCTHR - [7:6] */
pub const WM8993_JD_SCTHR_SHIFT: u32 = 6;   /* JD_SCTHR - [7:6] */
pub const WM8993_JD_SCTHR_WIDTH: u32 = 2;   /* JD_SCTHR - [7:6] */
pub const WM8993_JD_THR_MASK: u32 = 0x0030;   /* JD_THR - [5:4] */
pub const WM8993_JD_THR_SHIFT: u32 = 4;   /* JD_THR - [5:4] */
pub const WM8993_JD_THR_WIDTH: u32 = 2;   /* JD_THR - [5:4] */
pub const WM8993_JD_ENA: u32 = 0x0004;   /* JD_ENA */
pub const WM8993_JD_ENA_MASK: u32 = 0x0004;   /* JD_ENA */
pub const WM8993_JD_ENA_SHIFT: u32 = 2;   /* JD_ENA */
pub const WM8993_JD_ENA_WIDTH: u32 = 1;   /* JD_ENA */
pub const WM8993_MICB2_LVL: u32 = 0x0002;   /* MICB2_LVL */
pub const WM8993_MICB2_LVL_MASK: u32 = 0x0002;   /* MICB2_LVL */
pub const WM8993_MICB2_LVL_SHIFT: u32 = 1;   /* MICB2_LVL */
pub const WM8993_MICB2_LVL_WIDTH: u32 = 1;   /* MICB2_LVL */
pub const WM8993_MICB1_LVL: u32 = 0x0001;   /* MICB1_LVL */
pub const WM8993_MICB1_LVL_MASK: u32 = 0x0001;   /* MICB1_LVL */
pub const WM8993_MICB1_LVL_SHIFT: u32 = 0;   /* MICB1_LVL */
pub const WM8993_MICB1_LVL_WIDTH: u32 = 1;   /* MICB1_LVL */

/*
 * R60 (0x3C) - FLL Control 1
 */
pub const WM8993_FLL_FRAC: u32 = 0x0004;   /* FLL_FRAC */
pub const WM8993_FLL_FRAC_MASK: u32 = 0x0004;   /* FLL_FRAC */
pub const WM8993_FLL_FRAC_SHIFT: u32 = 2;   /* FLL_FRAC */
pub const WM8993_FLL_FRAC_WIDTH: u32 = 1;   /* FLL_FRAC */
pub const WM8993_FLL_OSC_ENA: u32 = 0x0002;   /* FLL_OSC_ENA */
pub const WM8993_FLL_OSC_ENA_MASK: u32 = 0x0002;   /* FLL_OSC_ENA */
pub const WM8993_FLL_OSC_ENA_SHIFT: u32 = 1;   /* FLL_OSC_ENA */
pub const WM8993_FLL_OSC_ENA_WIDTH: u32 = 1;   /* FLL_OSC_ENA */
pub const WM8993_FLL_ENA: u32 = 0x0001;   /* FLL_ENA */
pub const WM8993_FLL_ENA_MASK: u32 = 0x0001;   /* FLL_ENA */
pub const WM8993_FLL_ENA_SHIFT: u32 = 0;   /* FLL_ENA */
pub const WM8993_FLL_ENA_WIDTH: u32 = 1;   /* FLL_ENA */

/*
 * R61 (0x3D) - FLL Control 2
 */
pub const WM8993_FLL_OUTDIV_MASK: u32 = 0x0700;   /* FLL_OUTDIV - [10:8] */
pub const WM8993_FLL_OUTDIV_SHIFT: u32 = 8;   /* FLL_OUTDIV - [10:8] */
pub const WM8993_FLL_OUTDIV_WIDTH: u32 = 3;   /* FLL_OUTDIV - [10:8] */
pub const WM8993_FLL_CTRL_RATE_MASK: u32 = 0x0070;   /* FLL_CTRL_RATE - [6:4] */
pub const WM8993_FLL_CTRL_RATE_SHIFT: u32 = 4;   /* FLL_CTRL_RATE - [6:4] */
pub const WM8993_FLL_CTRL_RATE_WIDTH: u32 = 3;   /* FLL_CTRL_RATE - [6:4] */
pub const WM8993_FLL_FRATIO_MASK: u32 = 0x0007;   /* FLL_FRATIO - [2:0] */
pub const WM8993_FLL_FRATIO_SHIFT: u32 = 0;   /* FLL_FRATIO - [2:0] */
pub const WM8993_FLL_FRATIO_WIDTH: u32 = 3;   /* FLL_FRATIO - [2:0] */

/*
 * R62 (0x3E) - FLL Control 3
 */
pub const WM8993_FLL_K_MASK: u32 = 0xFFFF;   /* FLL_K - [15:0] */
pub const WM8993_FLL_K_SHIFT: u32 = 0;   /* FLL_K - [15:0] */
pub const WM8993_FLL_K_WIDTH: u32 = 16;   /* FLL_K - [15:0] */

/*
 * R63 (0x3F) - FLL Control 4
 */
pub const WM8993_FLL_N_MASK: u32 = 0x7FE0;   /* FLL_N - [14:5] */
pub const WM8993_FLL_N_SHIFT: u32 = 5;   /* FLL_N - [14:5] */
pub const WM8993_FLL_N_WIDTH: u32 = 10;   /* FLL_N - [14:5] */
pub const WM8993_FLL_GAIN_MASK: u32 = 0x000F;   /* FLL_GAIN - [3:0] */
pub const WM8993_FLL_GAIN_SHIFT: u32 = 0;   /* FLL_GAIN - [3:0] */
pub const WM8993_FLL_GAIN_WIDTH: u32 = 4;   /* FLL_GAIN - [3:0] */

/*
 * R64 (0x40) - FLL Control 5
 */
pub const WM8993_FLL_FRC_NCO_VAL_MASK: u32 = 0x1F80;   /* FLL_FRC_NCO_VAL - [12:7] */
pub const WM8993_FLL_FRC_NCO_VAL_SHIFT: u32 = 7;   /* FLL_FRC_NCO_VAL - [12:7] */
pub const WM8993_FLL_FRC_NCO_VAL_WIDTH: u32 = 6;   /* FLL_FRC_NCO_VAL - [12:7] */
pub const WM8993_FLL_FRC_NCO: u32 = 0x0040;   /* FLL_FRC_NCO */
pub const WM8993_FLL_FRC_NCO_MASK: u32 = 0x0040;   /* FLL_FRC_NCO */
pub const WM8993_FLL_FRC_NCO_SHIFT: u32 = 6;   /* FLL_FRC_NCO */
pub const WM8993_FLL_FRC_NCO_WIDTH: u32 = 1;   /* FLL_FRC_NCO */
pub const WM8993_FLL_CLK_REF_DIV_MASK: u32 = 0x0018;   /* FLL_CLK_REF_DIV - [4:3] */
pub const WM8993_FLL_CLK_REF_DIV_SHIFT: u32 = 3;   /* FLL_CLK_REF_DIV - [4:3] */
pub const WM8993_FLL_CLK_REF_DIV_WIDTH: u32 = 2;   /* FLL_CLK_REF_DIV - [4:3] */
pub const WM8993_FLL_CLK_SRC_MASK: u32 = 0x0003;   /* FLL_CLK_SRC - [1:0] */
pub const WM8993_FLL_CLK_SRC_SHIFT: u32 = 0;   /* FLL_CLK_SRC - [1:0] */
pub const WM8993_FLL_CLK_SRC_WIDTH: u32 = 2;   /* FLL_CLK_SRC - [1:0] */

/*
 * R65 (0x41) - Clocking 3
 */
pub const WM8993_CLK_DCS_DIV_MASK: u32 = 0x3C00;   /* CLK_DCS_DIV - [13:10] */
pub const WM8993_CLK_DCS_DIV_SHIFT: u32 = 10;   /* CLK_DCS_DIV - [13:10] */
pub const WM8993_CLK_DCS_DIV_WIDTH: u32 = 4;   /* CLK_DCS_DIV - [13:10] */
pub const WM8993_SAMPLE_RATE_MASK: u32 = 0x0380;   /* SAMPLE_RATE - [9:7] */
pub const WM8993_SAMPLE_RATE_SHIFT: u32 = 7;   /* SAMPLE_RATE - [9:7] */
pub const WM8993_SAMPLE_RATE_WIDTH: u32 = 3;   /* SAMPLE_RATE - [9:7] */
pub const WM8993_CLK_SYS_RATE_MASK: u32 = 0x001E;   /* CLK_SYS_RATE - [4:1] */
pub const WM8993_CLK_SYS_RATE_SHIFT: u32 = 1;   /* CLK_SYS_RATE - [4:1] */
pub const WM8993_CLK_SYS_RATE_WIDTH: u32 = 4;   /* CLK_SYS_RATE - [4:1] */
pub const WM8993_CLK_DSP_ENA: u32 = 0x0001;   /* CLK_DSP_ENA */
pub const WM8993_CLK_DSP_ENA_MASK: u32 = 0x0001;   /* CLK_DSP_ENA */
pub const WM8993_CLK_DSP_ENA_SHIFT: u32 = 0;   /* CLK_DSP_ENA */
pub const WM8993_CLK_DSP_ENA_WIDTH: u32 = 1;   /* CLK_DSP_ENA */

/*
 * R66 (0x42) - Clocking 4
 */
pub const WM8993_DAC_DIV4: u32 = 0x0200;   /* DAC_DIV4 */
pub const WM8993_DAC_DIV4_MASK: u32 = 0x0200;   /* DAC_DIV4 */
pub const WM8993_DAC_DIV4_SHIFT: u32 = 9;   /* DAC_DIV4 */
pub const WM8993_DAC_DIV4_WIDTH: u32 = 1;   /* DAC_DIV4 */
pub const WM8993_CLK_256K_DIV_MASK: u32 = 0x007E;   /* CLK_256K_DIV - [6:1] */
pub const WM8993_CLK_256K_DIV_SHIFT: u32 = 1;   /* CLK_256K_DIV - [6:1] */
pub const WM8993_CLK_256K_DIV_WIDTH: u32 = 6;   /* CLK_256K_DIV - [6:1] */
pub const WM8993_SR_MODE: u32 = 0x0001;   /* SR_MODE */
pub const WM8993_SR_MODE_MASK: u32 = 0x0001;   /* SR_MODE */
pub const WM8993_SR_MODE_SHIFT: u32 = 0;   /* SR_MODE */
pub const WM8993_SR_MODE_WIDTH: u32 = 1;   /* SR_MODE */

/*
 * R67 (0x43) - MW Slave Control
 */
pub const WM8993_MASK_WRITE_ENA: u32 = 0x0001;   /* MASK_WRITE_ENA */
pub const WM8993_MASK_WRITE_ENA_MASK: u32 = 0x0001;   /* MASK_WRITE_ENA */
pub const WM8993_MASK_WRITE_ENA_SHIFT: u32 = 0;   /* MASK_WRITE_ENA */
pub const WM8993_MASK_WRITE_ENA_WIDTH: u32 = 1;   /* MASK_WRITE_ENA */

/*
 * R69 (0x45) - Bus Control 1
 */
pub const WM8993_CLK_SYS_ENA: u32 = 0x0002;   /* CLK_SYS_ENA */
pub const WM8993_CLK_SYS_ENA_MASK: u32 = 0x0002;   /* CLK_SYS_ENA */
pub const WM8993_CLK_SYS_ENA_SHIFT: u32 = 1;   /* CLK_SYS_ENA */
pub const WM8993_CLK_SYS_ENA_WIDTH: u32 = 1;   /* CLK_SYS_ENA */

/*
 * R70 (0x46) - Write Sequencer 0
 */
pub const WM8993_WSEQ_ENA: u32 = 0x0100;   /* WSEQ_ENA */
pub const WM8993_WSEQ_ENA_MASK: u32 = 0x0100;   /* WSEQ_ENA */
pub const WM8993_WSEQ_ENA_SHIFT: u32 = 8;   /* WSEQ_ENA */
pub const WM8993_WSEQ_ENA_WIDTH: u32 = 1;   /* WSEQ_ENA */
pub const WM8993_WSEQ_WRITE_INDEX_MASK: u32 = 0x001F;   /* WSEQ_WRITE_INDEX - [4:0] */
pub const WM8993_WSEQ_WRITE_INDEX_SHIFT: u32 = 0;   /* WSEQ_WRITE_INDEX - [4:0] */
pub const WM8993_WSEQ_WRITE_INDEX_WIDTH: u32 = 5;   /* WSEQ_WRITE_INDEX - [4:0] */

/*
 * R71 (0x47) - Write Sequencer 1
 */
pub const WM8993_WSEQ_DATA_WIDTH_MASK: u32 = 0x7000;   /* WSEQ_DATA_WIDTH - [14:12] */
pub const WM8993_WSEQ_DATA_WIDTH_SHIFT: u32 = 12;   /* WSEQ_DATA_WIDTH - [14:12] */
pub const WM8993_WSEQ_DATA_WIDTH_WIDTH: u32 = 3;   /* WSEQ_DATA_WIDTH - [14:12] */
pub const WM8993_WSEQ_DATA_START_MASK: u32 = 0x0F00;   /* WSEQ_DATA_START - [11:8] */
pub const WM8993_WSEQ_DATA_START_SHIFT: u32 = 8;   /* WSEQ_DATA_START - [11:8] */
pub const WM8993_WSEQ_DATA_START_WIDTH: u32 = 4;   /* WSEQ_DATA_START - [11:8] */
pub const WM8993_WSEQ_ADDR_MASK: u32 = 0x00FF;   /* WSEQ_ADDR - [7:0] */
pub const WM8993_WSEQ_ADDR_SHIFT: u32 = 0;   /* WSEQ_ADDR - [7:0] */
pub const WM8993_WSEQ_ADDR_WIDTH: u32 = 8;   /* WSEQ_ADDR - [7:0] */

/*
 * R72 (0x48) - Write Sequencer 2
 */
pub const WM8993_WSEQ_EOS: u32 = 0x4000;   /* WSEQ_EOS */
pub const WM8993_WSEQ_EOS_MASK: u32 = 0x4000;   /* WSEQ_EOS */
pub const WM8993_WSEQ_EOS_SHIFT: u32 = 14;   /* WSEQ_EOS */
pub const WM8993_WSEQ_EOS_WIDTH: u32 = 1;   /* WSEQ_EOS */
pub const WM8993_WSEQ_DELAY_MASK: u32 = 0x0F00;   /* WSEQ_DELAY - [11:8] */
pub const WM8993_WSEQ_DELAY_SHIFT: u32 = 8;   /* WSEQ_DELAY - [11:8] */
pub const WM8993_WSEQ_DELAY_WIDTH: u32 = 4;   /* WSEQ_DELAY - [11:8] */
pub const WM8993_WSEQ_DATA_MASK: u32 = 0x00FF;   /* WSEQ_DATA - [7:0] */
pub const WM8993_WSEQ_DATA_SHIFT: u32 = 0;   /* WSEQ_DATA - [7:0] */
pub const WM8993_WSEQ_DATA_WIDTH: u32 = 8;   /* WSEQ_DATA - [7:0] */

/*
 * R73 (0x49) - Write Sequencer 3
 */
pub const WM8993_WSEQ_ABORT: u32 = 0x0200;   /* WSEQ_ABORT */
pub const WM8993_WSEQ_ABORT_MASK: u32 = 0x0200;   /* WSEQ_ABORT */
pub const WM8993_WSEQ_ABORT_SHIFT: u32 = 9;   /* WSEQ_ABORT */
pub const WM8993_WSEQ_ABORT_WIDTH: u32 = 1;   /* WSEQ_ABORT */
pub const WM8993_WSEQ_START: u32 = 0x0100;   /* WSEQ_START */
pub const WM8993_WSEQ_START_MASK: u32 = 0x0100;   /* WSEQ_START */
pub const WM8993_WSEQ_START_SHIFT: u32 = 8;   /* WSEQ_START */
pub const WM8993_WSEQ_START_WIDTH: u32 = 1;   /* WSEQ_START */
pub const WM8993_WSEQ_START_INDEX_MASK: u32 = 0x003F;   /* WSEQ_START_INDEX - [5:0] */
pub const WM8993_WSEQ_START_INDEX_SHIFT: u32 = 0;   /* WSEQ_START_INDEX - [5:0] */
pub const WM8993_WSEQ_START_INDEX_WIDTH: u32 = 6;   /* WSEQ_START_INDEX - [5:0] */

/*
 * R74 (0x4A) - Write Sequencer 4
 */
pub const WM8993_WSEQ_BUSY: u32 = 0x0001;   /* WSEQ_BUSY */
pub const WM8993_WSEQ_BUSY_MASK: u32 = 0x0001;   /* WSEQ_BUSY */
pub const WM8993_WSEQ_BUSY_SHIFT: u32 = 0;   /* WSEQ_BUSY */
pub const WM8993_WSEQ_BUSY_WIDTH: u32 = 1;   /* WSEQ_BUSY */

/*
 * R75 (0x4B) - Write Sequencer 5
 */
pub const WM8993_WSEQ_CURRENT_INDEX_MASK: u32 = 0x003F;   /* WSEQ_CURRENT_INDEX - [5:0] */
pub const WM8993_WSEQ_CURRENT_INDEX_SHIFT: u32 = 0;   /* WSEQ_CURRENT_INDEX - [5:0] */
pub const WM8993_WSEQ_CURRENT_INDEX_WIDTH: u32 = 6;   /* WSEQ_CURRENT_INDEX - [5:0] */

/*
 * R76 (0x4C) - Charge Pump 1
 */
pub const WM8993_CP_ENA: u32 = 0x8000;   /* CP_ENA */
pub const WM8993_CP_ENA_MASK: u32 = 0x8000;   /* CP_ENA */
pub const WM8993_CP_ENA_SHIFT: u32 = 15;   /* CP_ENA */
pub const WM8993_CP_ENA_WIDTH: u32 = 1;   /* CP_ENA */

/*
 * R81 (0x51) - Class W 0
 */
pub const WM8993_CP_DYN_FREQ: u32 = 0x0002;   /* CP_DYN_FREQ */
pub const WM8993_CP_DYN_FREQ_MASK: u32 = 0x0002;   /* CP_DYN_FREQ */
pub const WM8993_CP_DYN_FREQ_SHIFT: u32 = 1;   /* CP_DYN_FREQ */
pub const WM8993_CP_DYN_FREQ_WIDTH: u32 = 1;   /* CP_DYN_FREQ */
pub const WM8993_CP_DYN_V: u32 = 0x0001;   /* CP_DYN_V */
pub const WM8993_CP_DYN_V_MASK: u32 = 0x0001;   /* CP_DYN_V */
pub const WM8993_CP_DYN_V_SHIFT: u32 = 0;   /* CP_DYN_V */
pub const WM8993_CP_DYN_V_WIDTH: u32 = 1;   /* CP_DYN_V */

/*
 * R84 (0x54) - DC Servo 0
 */
pub const WM8993_DCS_TRIG_SINGLE_1: u32 = 0x2000;   /* DCS_TRIG_SINGLE_1 */
pub const WM8993_DCS_TRIG_SINGLE_1_MASK: u32 = 0x2000;   /* DCS_TRIG_SINGLE_1 */
pub const WM8993_DCS_TRIG_SINGLE_1_SHIFT: u32 = 13;   /* DCS_TRIG_SINGLE_1 */
pub const WM8993_DCS_TRIG_SINGLE_1_WIDTH: u32 = 1;   /* DCS_TRIG_SINGLE_1 */
pub const WM8993_DCS_TRIG_SINGLE_0: u32 = 0x1000;   /* DCS_TRIG_SINGLE_0 */
pub const WM8993_DCS_TRIG_SINGLE_0_MASK: u32 = 0x1000;   /* DCS_TRIG_SINGLE_0 */
pub const WM8993_DCS_TRIG_SINGLE_0_SHIFT: u32 = 12;   /* DCS_TRIG_SINGLE_0 */
pub const WM8993_DCS_TRIG_SINGLE_0_WIDTH: u32 = 1;   /* DCS_TRIG_SINGLE_0 */
pub const WM8993_DCS_TRIG_SERIES_1: u32 = 0x0200;   /* DCS_TRIG_SERIES_1 */
pub const WM8993_DCS_TRIG_SERIES_1_MASK: u32 = 0x0200;   /* DCS_TRIG_SERIES_1 */
pub const WM8993_DCS_TRIG_SERIES_1_SHIFT: u32 = 9;   /* DCS_TRIG_SERIES_1 */
pub const WM8993_DCS_TRIG_SERIES_1_WIDTH: u32 = 1;   /* DCS_TRIG_SERIES_1 */
pub const WM8993_DCS_TRIG_SERIES_0: u32 = 0x0100;   /* DCS_TRIG_SERIES_0 */
pub const WM8993_DCS_TRIG_SERIES_0_MASK: u32 = 0x0100;   /* DCS_TRIG_SERIES_0 */
pub const WM8993_DCS_TRIG_SERIES_0_SHIFT: u32 = 8;   /* DCS_TRIG_SERIES_0 */
pub const WM8993_DCS_TRIG_SERIES_0_WIDTH: u32 = 1;   /* DCS_TRIG_SERIES_0 */
pub const WM8993_DCS_TRIG_STARTUP_1: u32 = 0x0020;   /* DCS_TRIG_STARTUP_1 */
pub const WM8993_DCS_TRIG_STARTUP_1_MASK: u32 = 0x0020;   /* DCS_TRIG_STARTUP_1 */
pub const WM8993_DCS_TRIG_STARTUP_1_SHIFT: u32 = 5;   /* DCS_TRIG_STARTUP_1 */
pub const WM8993_DCS_TRIG_STARTUP_1_WIDTH: u32 = 1;   /* DCS_TRIG_STARTUP_1 */
pub const WM8993_DCS_TRIG_STARTUP_0: u32 = 0x0010;   /* DCS_TRIG_STARTUP_0 */
pub const WM8993_DCS_TRIG_STARTUP_0_MASK: u32 = 0x0010;   /* DCS_TRIG_STARTUP_0 */
pub const WM8993_DCS_TRIG_STARTUP_0_SHIFT: u32 = 4;   /* DCS_TRIG_STARTUP_0 */
pub const WM8993_DCS_TRIG_STARTUP_0_WIDTH: u32 = 1;   /* DCS_TRIG_STARTUP_0 */
pub const WM8993_DCS_TRIG_DAC_WR_1: u32 = 0x0008;   /* DCS_TRIG_DAC_WR_1 */
pub const WM8993_DCS_TRIG_DAC_WR_1_MASK: u32 = 0x0008;   /* DCS_TRIG_DAC_WR_1 */
pub const WM8993_DCS_TRIG_DAC_WR_1_SHIFT: u32 = 3;   /* DCS_TRIG_DAC_WR_1 */
pub const WM8993_DCS_TRIG_DAC_WR_1_WIDTH: u32 = 1;   /* DCS_TRIG_DAC_WR_1 */
pub const WM8993_DCS_TRIG_DAC_WR_0: u32 = 0x0004;   /* DCS_TRIG_DAC_WR_0 */
pub const WM8993_DCS_TRIG_DAC_WR_0_MASK: u32 = 0x0004;   /* DCS_TRIG_DAC_WR_0 */
pub const WM8993_DCS_TRIG_DAC_WR_0_SHIFT: u32 = 2;   /* DCS_TRIG_DAC_WR_0 */
pub const WM8993_DCS_TRIG_DAC_WR_0_WIDTH: u32 = 1;   /* DCS_TRIG_DAC_WR_0 */
pub const WM8993_DCS_ENA_CHAN_1: u32 = 0x0002;   /* DCS_ENA_CHAN_1 */
pub const WM8993_DCS_ENA_CHAN_1_MASK: u32 = 0x0002;   /* DCS_ENA_CHAN_1 */
pub const WM8993_DCS_ENA_CHAN_1_SHIFT: u32 = 1;   /* DCS_ENA_CHAN_1 */
pub const WM8993_DCS_ENA_CHAN_1_WIDTH: u32 = 1;   /* DCS_ENA_CHAN_1 */
pub const WM8993_DCS_ENA_CHAN_0: u32 = 0x0001;   /* DCS_ENA_CHAN_0 */
pub const WM8993_DCS_ENA_CHAN_0_MASK: u32 = 0x0001;   /* DCS_ENA_CHAN_0 */
pub const WM8993_DCS_ENA_CHAN_0_SHIFT: u32 = 0;   /* DCS_ENA_CHAN_0 */
pub const WM8993_DCS_ENA_CHAN_0_WIDTH: u32 = 1;   /* DCS_ENA_CHAN_0 */

/*
 * R85 (0x55) - DC Servo 1
 */
pub const WM8993_DCS_SERIES_NO_01_MASK: u32 = 0x0FE0;   /* DCS_SERIES_NO_01 - [11:5] */
pub const WM8993_DCS_SERIES_NO_01_SHIFT: u32 = 5;   /* DCS_SERIES_NO_01 - [11:5] */
pub const WM8993_DCS_SERIES_NO_01_WIDTH: u32 = 7;   /* DCS_SERIES_NO_01 - [11:5] */
pub const WM8993_DCS_TIMER_PERIOD_01_MASK: u32 = 0x000F;   /* DCS_TIMER_PERIOD_01 - [3:0] */
pub const WM8993_DCS_TIMER_PERIOD_01_SHIFT: u32 = 0;   /* DCS_TIMER_PERIOD_01 - [3:0] */
pub const WM8993_DCS_TIMER_PERIOD_01_WIDTH: u32 = 4;   /* DCS_TIMER_PERIOD_01 - [3:0] */

/*
 * R87 (0x57) - DC Servo 3
 */
pub const WM8993_DCS_DAC_WR_VAL_1_MASK: u32 = 0xFF00;   /* DCS_DAC_WR_VAL_1 - [15:8] */
pub const WM8993_DCS_DAC_WR_VAL_1_SHIFT: u32 = 8;   /* DCS_DAC_WR_VAL_1 - [15:8] */
pub const WM8993_DCS_DAC_WR_VAL_1_WIDTH: u32 = 8;   /* DCS_DAC_WR_VAL_1 - [15:8] */
pub const WM8993_DCS_DAC_WR_VAL_0_MASK: u32 = 0x00FF;   /* DCS_DAC_WR_VAL_0 - [7:0] */
pub const WM8993_DCS_DAC_WR_VAL_0_SHIFT: u32 = 0;   /* DCS_DAC_WR_VAL_0 - [7:0] */
pub const WM8993_DCS_DAC_WR_VAL_0_WIDTH: u32 = 8;   /* DCS_DAC_WR_VAL_0 - [7:0] */

/*
 * R88 (0x58) - DC Servo Readback 0
 */
pub const WM8993_DCS_DATAPATH_BUSY: u32 = 0x4000;   /* DCS_DATAPATH_BUSY */
pub const WM8993_DCS_DATAPATH_BUSY_MASK: u32 = 0x4000;   /* DCS_DATAPATH_BUSY */
pub const WM8993_DCS_DATAPATH_BUSY_SHIFT: u32 = 14;   /* DCS_DATAPATH_BUSY */
pub const WM8993_DCS_DATAPATH_BUSY_WIDTH: u32 = 1;   /* DCS_DATAPATH_BUSY */
pub const WM8993_DCS_CHANNEL_MASK: u32 = 0x3000;   /* DCS_CHANNEL - [13:12] */
pub const WM8993_DCS_CHANNEL_SHIFT: u32 = 12;   /* DCS_CHANNEL - [13:12] */
pub const WM8993_DCS_CHANNEL_WIDTH: u32 = 2;   /* DCS_CHANNEL - [13:12] */
pub const WM8993_DCS_CAL_COMPLETE_MASK: u32 = 0x0300;   /* DCS_CAL_COMPLETE - [9:8] */
pub const WM8993_DCS_CAL_COMPLETE_SHIFT: u32 = 8;   /* DCS_CAL_COMPLETE - [9:8] */
pub const WM8993_DCS_CAL_COMPLETE_WIDTH: u32 = 2;   /* DCS_CAL_COMPLETE - [9:8] */
pub const WM8993_DCS_DAC_WR_COMPLETE_MASK: u32 = 0x0030;   /* DCS_DAC_WR_COMPLETE - [5:4] */
pub const WM8993_DCS_DAC_WR_COMPLETE_SHIFT: u32 = 4;   /* DCS_DAC_WR_COMPLETE - [5:4] */
pub const WM8993_DCS_DAC_WR_COMPLETE_WIDTH: u32 = 2;   /* DCS_DAC_WR_COMPLETE - [5:4] */
pub const WM8993_DCS_STARTUP_COMPLETE_MASK: u32 = 0x0003;   /* DCS_STARTUP_COMPLETE - [1:0] */
pub const WM8993_DCS_STARTUP_COMPLETE_SHIFT: u32 = 0;   /* DCS_STARTUP_COMPLETE - [1:0] */
pub const WM8993_DCS_STARTUP_COMPLETE_WIDTH: u32 = 2;   /* DCS_STARTUP_COMPLETE - [1:0] */

/*
 * R89 (0x59) - DC Servo Readback 1
 */
pub const WM8993_DCS_INTEG_CHAN_1_MASK: u32 = 0x00FF;   /* DCS_INTEG_CHAN_1 - [7:0] */
pub const WM8993_DCS_INTEG_CHAN_1_SHIFT: u32 = 0;   /* DCS_INTEG_CHAN_1 - [7:0] */
pub const WM8993_DCS_INTEG_CHAN_1_WIDTH: u32 = 8;   /* DCS_INTEG_CHAN_1 - [7:0] */

/*
 * R90 (0x5A) - DC Servo Readback 2
 */
pub const WM8993_DCS_INTEG_CHAN_0_MASK: u32 = 0x00FF;   /* DCS_INTEG_CHAN_0 - [7:0] */
pub const WM8993_DCS_INTEG_CHAN_0_SHIFT: u32 = 0;   /* DCS_INTEG_CHAN_0 - [7:0] */
pub const WM8993_DCS_INTEG_CHAN_0_WIDTH: u32 = 8;   /* DCS_INTEG_CHAN_0 - [7:0] */

/*
 * R96 (0x60) - Analogue HP 0
 */
pub const WM8993_HPOUT1_AUTO_PU: u32 = 0x0100;   /* HPOUT1_AUTO_PU */
pub const WM8993_HPOUT1_AUTO_PU_MASK: u32 = 0x0100;   /* HPOUT1_AUTO_PU */
pub const WM8993_HPOUT1_AUTO_PU_SHIFT: u32 = 8;   /* HPOUT1_AUTO_PU */
pub const WM8993_HPOUT1_AUTO_PU_WIDTH: u32 = 1;   /* HPOUT1_AUTO_PU */
pub const WM8993_HPOUT1L_RMV_SHORT: u32 = 0x0080;   /* HPOUT1L_RMV_SHORT */
pub const WM8993_HPOUT1L_RMV_SHORT_MASK: u32 = 0x0080;   /* HPOUT1L_RMV_SHORT */
pub const WM8993_HPOUT1L_RMV_SHORT_SHIFT: u32 = 7;   /* HPOUT1L_RMV_SHORT */
pub const WM8993_HPOUT1L_RMV_SHORT_WIDTH: u32 = 1;   /* HPOUT1L_RMV_SHORT */
pub const WM8993_HPOUT1L_OUTP: u32 = 0x0040;   /* HPOUT1L_OUTP */
pub const WM8993_HPOUT1L_OUTP_MASK: u32 = 0x0040;   /* HPOUT1L_OUTP */
pub const WM8993_HPOUT1L_OUTP_SHIFT: u32 = 6;   /* HPOUT1L_OUTP */
pub const WM8993_HPOUT1L_OUTP_WIDTH: u32 = 1;   /* HPOUT1L_OUTP */
pub const WM8993_HPOUT1L_DLY: u32 = 0x0020;   /* HPOUT1L_DLY */
pub const WM8993_HPOUT1L_DLY_MASK: u32 = 0x0020;   /* HPOUT1L_DLY */
pub const WM8993_HPOUT1L_DLY_SHIFT: u32 = 5;   /* HPOUT1L_DLY */
pub const WM8993_HPOUT1L_DLY_WIDTH: u32 = 1;   /* HPOUT1L_DLY */
pub const WM8993_HPOUT1R_RMV_SHORT: u32 = 0x0008;   /* HPOUT1R_RMV_SHORT */
pub const WM8993_HPOUT1R_RMV_SHORT_MASK: u32 = 0x0008;   /* HPOUT1R_RMV_SHORT */
pub const WM8993_HPOUT1R_RMV_SHORT_SHIFT: u32 = 3;   /* HPOUT1R_RMV_SHORT */
pub const WM8993_HPOUT1R_RMV_SHORT_WIDTH: u32 = 1;   /* HPOUT1R_RMV_SHORT */
pub const WM8993_HPOUT1R_OUTP: u32 = 0x0004;   /* HPOUT1R_OUTP */
pub const WM8993_HPOUT1R_OUTP_MASK: u32 = 0x0004;   /* HPOUT1R_OUTP */
pub const WM8993_HPOUT1R_OUTP_SHIFT: u32 = 2;   /* HPOUT1R_OUTP */
pub const WM8993_HPOUT1R_OUTP_WIDTH: u32 = 1;   /* HPOUT1R_OUTP */
pub const WM8993_HPOUT1R_DLY: u32 = 0x0002;   /* HPOUT1R_DLY */
pub const WM8993_HPOUT1R_DLY_MASK: u32 = 0x0002;   /* HPOUT1R_DLY */
pub const WM8993_HPOUT1R_DLY_SHIFT: u32 = 1;   /* HPOUT1R_DLY */
pub const WM8993_HPOUT1R_DLY_WIDTH: u32 = 1;   /* HPOUT1R_DLY */

/*
 * R98 (0x62) - EQ1
 */
pub const WM8993_EQ_ENA: u32 = 0x0001;   /* EQ_ENA */
pub const WM8993_EQ_ENA_MASK: u32 = 0x0001;   /* EQ_ENA */
pub const WM8993_EQ_ENA_SHIFT: u32 = 0;   /* EQ_ENA */
pub const WM8993_EQ_ENA_WIDTH: u32 = 1;   /* EQ_ENA */

/*
 * R99 (0x63) - EQ2
 */
pub const WM8993_EQ_B1_GAIN_MASK: u32 = 0x001F;   /* EQ_B1_GAIN - [4:0] */
pub const WM8993_EQ_B1_GAIN_SHIFT: u32 = 0;   /* EQ_B1_GAIN - [4:0] */
pub const WM8993_EQ_B1_GAIN_WIDTH: u32 = 5;   /* EQ_B1_GAIN - [4:0] */

/*
 * R100 (0x64) - EQ3
 */
pub const WM8993_EQ_B2_GAIN_MASK: u32 = 0x001F;   /* EQ_B2_GAIN - [4:0] */
pub const WM8993_EQ_B2_GAIN_SHIFT: u32 = 0;   /* EQ_B2_GAIN - [4:0] */
pub const WM8993_EQ_B2_GAIN_WIDTH: u32 = 5;   /* EQ_B2_GAIN - [4:0] */

/*
 * R101 (0x65) - EQ4
 */
pub const WM8993_EQ_B3_GAIN_MASK: u32 = 0x001F;   /* EQ_B3_GAIN - [4:0] */
pub const WM8993_EQ_B3_GAIN_SHIFT: u32 = 0;   /* EQ_B3_GAIN - [4:0] */
pub const WM8993_EQ_B3_GAIN_WIDTH: u32 = 5;   /* EQ_B3_GAIN - [4:0] */

/*
 * R102 (0x66) - EQ5
 */
pub const WM8993_EQ_B4_GAIN_MASK: u32 = 0x001F;   /* EQ_B4_GAIN - [4:0] */
pub const WM8993_EQ_B4_GAIN_SHIFT: u32 = 0;   /* EQ_B4_GAIN - [4:0] */
pub const WM8993_EQ_B4_GAIN_WIDTH: u32 = 5;   /* EQ_B4_GAIN - [4:0] */

/*
 * R103 (0x67) - EQ6
 */
pub const WM8993_EQ_B5_GAIN_MASK: u32 = 0x001F;   /* EQ_B5_GAIN - [4:0] */
pub const WM8993_EQ_B5_GAIN_SHIFT: u32 = 0;   /* EQ_B5_GAIN - [4:0] */
pub const WM8993_EQ_B5_GAIN_WIDTH: u32 = 5;   /* EQ_B5_GAIN - [4:0] */

/*
 * R104 (0x68) - EQ7
 */
pub const WM8993_EQ_B1_A_MASK: u32 = 0xFFFF;   /* EQ_B1_A - [15:0] */
pub const WM8993_EQ_B1_A_SHIFT: u32 = 0;   /* EQ_B1_A - [15:0] */
pub const WM8993_EQ_B1_A_WIDTH: u32 = 16;   /* EQ_B1_A - [15:0] */

/*
 * R105 (0x69) - EQ8
 */
pub const WM8993_EQ_B1_B_MASK: u32 = 0xFFFF;   /* EQ_B1_B - [15:0] */
pub const WM8993_EQ_B1_B_SHIFT: u32 = 0;   /* EQ_B1_B - [15:0] */
pub const WM8993_EQ_B1_B_WIDTH: u32 = 16;   /* EQ_B1_B - [15:0] */

/*
 * R106 (0x6A) - EQ9
 */
pub const WM8993_EQ_B1_PG_MASK: u32 = 0xFFFF;   /* EQ_B1_PG - [15:0] */
pub const WM8993_EQ_B1_PG_SHIFT: u32 = 0;   /* EQ_B1_PG - [15:0] */
pub const WM8993_EQ_B1_PG_WIDTH: u32 = 16;   /* EQ_B1_PG - [15:0] */

/*
 * R107 (0x6B) - EQ10
 */
pub const WM8993_EQ_B2_A_MASK: u32 = 0xFFFF;   /* EQ_B2_A - [15:0] */
pub const WM8993_EQ_B2_A_SHIFT: u32 = 0;   /* EQ_B2_A - [15:0] */
pub const WM8993_EQ_B2_A_WIDTH: u32 = 16;   /* EQ_B2_A - [15:0] */

/*
 * R108 (0x6C) - EQ11
 */
pub const WM8993_EQ_B2_B_MASK: u32 = 0xFFFF;   /* EQ_B2_B - [15:0] */
pub const WM8993_EQ_B2_B_SHIFT: u32 = 0;   /* EQ_B2_B - [15:0] */
pub const WM8993_EQ_B2_B_WIDTH: u32 = 16;   /* EQ_B2_B - [15:0] */

/*
 * R109 (0x6D) - EQ12
 */
pub const WM8993_EQ_B2_C_MASK: u32 = 0xFFFF;   /* EQ_B2_C - [15:0] */
pub const WM8993_EQ_B2_C_SHIFT: u32 = 0;   /* EQ_B2_C - [15:0] */
pub const WM8993_EQ_B2_C_WIDTH: u32 = 16;   /* EQ_B2_C - [15:0] */

/*
 * R110 (0x6E) - EQ13
 */
pub const WM8993_EQ_B2_PG_MASK: u32 = 0xFFFF;   /* EQ_B2_PG - [15:0] */
pub const WM8993_EQ_B2_PG_SHIFT: u32 = 0;   /* EQ_B2_PG - [15:0] */
pub const WM8993_EQ_B2_PG_WIDTH: u32 = 16;   /* EQ_B2_PG - [15:0] */

/*
 * R111 (0x6F) - EQ14
 */
pub const WM8993_EQ_B3_A_MASK: u32 = 0xFFFF;   /* EQ_B3_A - [15:0] */
pub const WM8993_EQ_B3_A_SHIFT: u32 = 0;   /* EQ_B3_A - [15:0] */
pub const WM8993_EQ_B3_A_WIDTH: u32 = 16;   /* EQ_B3_A - [15:0] */

/*
 * R112 (0x70) - EQ15
 */
pub const WM8993_EQ_B3_B_MASK: u32 = 0xFFFF;   /* EQ_B3_B - [15:0] */
pub const WM8993_EQ_B3_B_SHIFT: u32 = 0;   /* EQ_B3_B - [15:0] */
pub const WM8993_EQ_B3_B_WIDTH: u32 = 16;   /* EQ_B3_B - [15:0] */

/*
 * R113 (0x71) - EQ16
 */
pub const WM8993_EQ_B3_C_MASK: u32 = 0xFFFF;   /* EQ_B3_C - [15:0] */
pub const WM8993_EQ_B3_C_SHIFT: u32 = 0;   /* EQ_B3_C - [15:0] */
pub const WM8993_EQ_B3_C_WIDTH: u32 = 16;   /* EQ_B3_C - [15:0] */

/*
 * R114 (0x72) - EQ17
 */
pub const WM8993_EQ_B3_PG_MASK: u32 = 0xFFFF;   /* EQ_B3_PG - [15:0] */
pub const WM8993_EQ_B3_PG_SHIFT: u32 = 0;   /* EQ_B3_PG - [15:0] */
pub const WM8993_EQ_B3_PG_WIDTH: u32 = 16;   /* EQ_B3_PG - [15:0] */

/*
 * R115 (0x73) - EQ18
 */
pub const WM8993_EQ_B4_A_MASK: u32 = 0xFFFF;   /* EQ_B4_A - [15:0] */
pub const WM8993_EQ_B4_A_SHIFT: u32 = 0;   /* EQ_B4_A - [15:0] */
pub const WM8993_EQ_B4_A_WIDTH: u32 = 16;   /* EQ_B4_A - [15:0] */

/*
 * R116 (0x74) - EQ19
 */
pub const WM8993_EQ_B4_B_MASK: u32 = 0xFFFF;   /* EQ_B4_B - [15:0] */
pub const WM8993_EQ_B4_B_SHIFT: u32 = 0;   /* EQ_B4_B - [15:0] */
pub const WM8993_EQ_B4_B_WIDTH: u32 = 16;   /* EQ_B4_B - [15:0] */

/*
 * R117 (0x75) - EQ20
 */
pub const WM8993_EQ_B4_C_MASK: u32 = 0xFFFF;   /* EQ_B4_C - [15:0] */
pub const WM8993_EQ_B4_C_SHIFT: u32 = 0;   /* EQ_B4_C - [15:0] */
pub const WM8993_EQ_B4_C_WIDTH: u32 = 16;   /* EQ_B4_C - [15:0] */

/*
 * R118 (0x76) - EQ21
 */
pub const WM8993_EQ_B4_PG_MASK: u32 = 0xFFFF;   /* EQ_B4_PG - [15:0] */
pub const WM8993_EQ_B4_PG_SHIFT: u32 = 0;   /* EQ_B4_PG - [15:0] */
pub const WM8993_EQ_B4_PG_WIDTH: u32 = 16;   /* EQ_B4_PG - [15:0] */

/*
 * R119 (0x77) - EQ22
 */
pub const WM8993_EQ_B5_A_MASK: u32 = 0xFFFF;   /* EQ_B5_A - [15:0] */
pub const WM8993_EQ_B5_A_SHIFT: u32 = 0;   /* EQ_B5_A - [15:0] */
pub const WM8993_EQ_B5_A_WIDTH: u32 = 16;   /* EQ_B5_A - [15:0] */

/*
 * R120 (0x78) - EQ23
 */
pub const WM8993_EQ_B5_B_MASK: u32 = 0xFFFF;   /* EQ_B5_B - [15:0] */
pub const WM8993_EQ_B5_B_SHIFT: u32 = 0;   /* EQ_B5_B - [15:0] */
pub const WM8993_EQ_B5_B_WIDTH: u32 = 16;   /* EQ_B5_B - [15:0] */

/*
 * R121 (0x79) - EQ24
 */
pub const WM8993_EQ_B5_PG_MASK: u32 = 0xFFFF;   /* EQ_B5_PG - [15:0] */
pub const WM8993_EQ_B5_PG_SHIFT: u32 = 0;   /* EQ_B5_PG - [15:0] */
pub const WM8993_EQ_B5_PG_WIDTH: u32 = 16;   /* EQ_B5_PG - [15:0] */

/*
 * R122 (0x7A) - Digital Pulls
 */
pub const WM8993_MCLK_PU: u32 = 0x0080;   /* MCLK_PU */
pub const WM8993_MCLK_PU_MASK: u32 = 0x0080;   /* MCLK_PU */
pub const WM8993_MCLK_PU_SHIFT: u32 = 7;   /* MCLK_PU */
pub const WM8993_MCLK_PU_WIDTH: u32 = 1;   /* MCLK_PU */
pub const WM8993_MCLK_PD: u32 = 0x0040;   /* MCLK_PD */
pub const WM8993_MCLK_PD_MASK: u32 = 0x0040;   /* MCLK_PD */
pub const WM8993_MCLK_PD_SHIFT: u32 = 6;   /* MCLK_PD */
pub const WM8993_MCLK_PD_WIDTH: u32 = 1;   /* MCLK_PD */
pub const WM8993_DACDAT_PU: u32 = 0x0020;   /* DACDAT_PU */
pub const WM8993_DACDAT_PU_MASK: u32 = 0x0020;   /* DACDAT_PU */
pub const WM8993_DACDAT_PU_SHIFT: u32 = 5;   /* DACDAT_PU */
pub const WM8993_DACDAT_PU_WIDTH: u32 = 1;   /* DACDAT_PU */
pub const WM8993_DACDAT_PD: u32 = 0x0010;   /* DACDAT_PD */
pub const WM8993_DACDAT_PD_MASK: u32 = 0x0010;   /* DACDAT_PD */
pub const WM8993_DACDAT_PD_SHIFT: u32 = 4;   /* DACDAT_PD */
pub const WM8993_DACDAT_PD_WIDTH: u32 = 1;   /* DACDAT_PD */
pub const WM8993_LRCLK_PU: u32 = 0x0008;   /* LRCLK_PU */
pub const WM8993_LRCLK_PU_MASK: u32 = 0x0008;   /* LRCLK_PU */
pub const WM8993_LRCLK_PU_SHIFT: u32 = 3;   /* LRCLK_PU */
pub const WM8993_LRCLK_PU_WIDTH: u32 = 1;   /* LRCLK_PU */
pub const WM8993_LRCLK_PD: u32 = 0x0004;   /* LRCLK_PD */
pub const WM8993_LRCLK_PD_MASK: u32 = 0x0004;   /* LRCLK_PD */
pub const WM8993_LRCLK_PD_SHIFT: u32 = 2;   /* LRCLK_PD */
pub const WM8993_LRCLK_PD_WIDTH: u32 = 1;   /* LRCLK_PD */
pub const WM8993_BCLK_PU: u32 = 0x0002;   /* BCLK_PU */
pub const WM8993_BCLK_PU_MASK: u32 = 0x0002;   /* BCLK_PU */
pub const WM8993_BCLK_PU_SHIFT: u32 = 1;   /* BCLK_PU */
pub const WM8993_BCLK_PU_WIDTH: u32 = 1;   /* BCLK_PU */
pub const WM8993_BCLK_PD: u32 = 0x0001;   /* BCLK_PD */
pub const WM8993_BCLK_PD_MASK: u32 = 0x0001;   /* BCLK_PD */
pub const WM8993_BCLK_PD_SHIFT: u32 = 0;   /* BCLK_PD */
pub const WM8993_BCLK_PD_WIDTH: u32 = 1;   /* BCLK_PD */

/*
 * R123 (0x7B) - DRC Control 1
 */
pub const WM8993_DRC_ENA: u32 = 0x8000;   /* DRC_ENA */
pub const WM8993_DRC_ENA_MASK: u32 = 0x8000;   /* DRC_ENA */
pub const WM8993_DRC_ENA_SHIFT: u32 = 15;   /* DRC_ENA */
pub const WM8993_DRC_ENA_WIDTH: u32 = 1;   /* DRC_ENA */
pub const WM8993_DRC_DAC_PATH: u32 = 0x4000;   /* DRC_DAC_PATH */
pub const WM8993_DRC_DAC_PATH_MASK: u32 = 0x4000;   /* DRC_DAC_PATH */
pub const WM8993_DRC_DAC_PATH_SHIFT: u32 = 14;   /* DRC_DAC_PATH */
pub const WM8993_DRC_DAC_PATH_WIDTH: u32 = 1;   /* DRC_DAC_PATH */
pub const WM8993_DRC_SMOOTH_ENA: u32 = 0x0800;   /* DRC_SMOOTH_ENA */
pub const WM8993_DRC_SMOOTH_ENA_MASK: u32 = 0x0800;   /* DRC_SMOOTH_ENA */
pub const WM8993_DRC_SMOOTH_ENA_SHIFT: u32 = 11;   /* DRC_SMOOTH_ENA */
pub const WM8993_DRC_SMOOTH_ENA_WIDTH: u32 = 1;   /* DRC_SMOOTH_ENA */
pub const WM8993_DRC_QR_ENA: u32 = 0x0400;   /* DRC_QR_ENA */
pub const WM8993_DRC_QR_ENA_MASK: u32 = 0x0400;   /* DRC_QR_ENA */
pub const WM8993_DRC_QR_ENA_SHIFT: u32 = 10;   /* DRC_QR_ENA */
pub const WM8993_DRC_QR_ENA_WIDTH: u32 = 1;   /* DRC_QR_ENA */
pub const WM8993_DRC_ANTICLIP_ENA: u32 = 0x0200;   /* DRC_ANTICLIP_ENA */
pub const WM8993_DRC_ANTICLIP_ENA_MASK: u32 = 0x0200;   /* DRC_ANTICLIP_ENA */
pub const WM8993_DRC_ANTICLIP_ENA_SHIFT: u32 = 9;   /* DRC_ANTICLIP_ENA */
pub const WM8993_DRC_ANTICLIP_ENA_WIDTH: u32 = 1;   /* DRC_ANTICLIP_ENA */
pub const WM8993_DRC_HYST_ENA: u32 = 0x0100;   /* DRC_HYST_ENA */
pub const WM8993_DRC_HYST_ENA_MASK: u32 = 0x0100;   /* DRC_HYST_ENA */
pub const WM8993_DRC_HYST_ENA_SHIFT: u32 = 8;   /* DRC_HYST_ENA */
pub const WM8993_DRC_HYST_ENA_WIDTH: u32 = 1;   /* DRC_HYST_ENA */
pub const WM8993_DRC_THRESH_HYST_MASK: u32 = 0x0030;   /* DRC_THRESH_HYST - [5:4] */
pub const WM8993_DRC_THRESH_HYST_SHIFT: u32 = 4;   /* DRC_THRESH_HYST - [5:4] */
pub const WM8993_DRC_THRESH_HYST_WIDTH: u32 = 2;   /* DRC_THRESH_HYST - [5:4] */
pub const WM8993_DRC_MINGAIN_MASK: u32 = 0x000C;   /* DRC_MINGAIN - [3:2] */
pub const WM8993_DRC_MINGAIN_SHIFT: u32 = 2;   /* DRC_MINGAIN - [3:2] */
pub const WM8993_DRC_MINGAIN_WIDTH: u32 = 2;   /* DRC_MINGAIN - [3:2] */
pub const WM8993_DRC_MAXGAIN_MASK: u32 = 0x0003;   /* DRC_MAXGAIN - [1:0] */
pub const WM8993_DRC_MAXGAIN_SHIFT: u32 = 0;   /* DRC_MAXGAIN - [1:0] */
pub const WM8993_DRC_MAXGAIN_WIDTH: u32 = 2;   /* DRC_MAXGAIN - [1:0] */

/*
 * R124 (0x7C) - DRC Control 2
 */
pub const WM8993_DRC_ATTACK_RATE_MASK: u32 = 0xF000;   /* DRC_ATTACK_RATE - [15:12] */
pub const WM8993_DRC_ATTACK_RATE_SHIFT: u32 = 12;   /* DRC_ATTACK_RATE - [15:12] */
pub const WM8993_DRC_ATTACK_RATE_WIDTH: u32 = 4;   /* DRC_ATTACK_RATE - [15:12] */
pub const WM8993_DRC_DECAY_RATE_MASK: u32 = 0x0F00;   /* DRC_DECAY_RATE - [11:8] */
pub const WM8993_DRC_DECAY_RATE_SHIFT: u32 = 8;   /* DRC_DECAY_RATE - [11:8] */
pub const WM8993_DRC_DECAY_RATE_WIDTH: u32 = 4;   /* DRC_DECAY_RATE - [11:8] */
pub const WM8993_DRC_THRESH_COMP_MASK: u32 = 0x00FC;   /* DRC_THRESH_COMP - [7:2] */
pub const WM8993_DRC_THRESH_COMP_SHIFT: u32 = 2;   /* DRC_THRESH_COMP - [7:2] */
pub const WM8993_DRC_THRESH_COMP_WIDTH: u32 = 6;   /* DRC_THRESH_COMP - [7:2] */

/*
 * R125 (0x7D) - DRC Control 3
 */
pub const WM8993_DRC_AMP_COMP_MASK: u32 = 0xF800;   /* DRC_AMP_COMP - [15:11] */
pub const WM8993_DRC_AMP_COMP_SHIFT: u32 = 11;   /* DRC_AMP_COMP - [15:11] */
pub const WM8993_DRC_AMP_COMP_WIDTH: u32 = 5;   /* DRC_AMP_COMP - [15:11] */
pub const WM8993_DRC_R0_SLOPE_COMP_MASK: u32 = 0x0700;   /* DRC_R0_SLOPE_COMP - [10:8] */
pub const WM8993_DRC_R0_SLOPE_COMP_SHIFT: u32 = 8;   /* DRC_R0_SLOPE_COMP - [10:8] */
pub const WM8993_DRC_R0_SLOPE_COMP_WIDTH: u32 = 3;   /* DRC_R0_SLOPE_COMP - [10:8] */
pub const WM8993_DRC_FF_DELAY: u32 = 0x0080;   /* DRC_FF_DELAY */
pub const WM8993_DRC_FF_DELAY_MASK: u32 = 0x0080;   /* DRC_FF_DELAY */
pub const WM8993_DRC_FF_DELAY_SHIFT: u32 = 7;   /* DRC_FF_DELAY */
pub const WM8993_DRC_FF_DELAY_WIDTH: u32 = 1;   /* DRC_FF_DELAY */
pub const WM8993_DRC_THRESH_QR_MASK: u32 = 0x000C;   /* DRC_THRESH_QR - [3:2] */
pub const WM8993_DRC_THRESH_QR_SHIFT: u32 = 2;   /* DRC_THRESH_QR - [3:2] */
pub const WM8993_DRC_THRESH_QR_WIDTH: u32 = 2;   /* DRC_THRESH_QR - [3:2] */
pub const WM8993_DRC_RATE_QR_MASK: u32 = 0x0003;   /* DRC_RATE_QR - [1:0] */
pub const WM8993_DRC_RATE_QR_SHIFT: u32 = 0;   /* DRC_RATE_QR - [1:0] */
pub const WM8993_DRC_RATE_QR_WIDTH: u32 = 2;   /* DRC_RATE_QR - [1:0] */

/*
 * R126 (0x7E) - DRC Control 4
 */
pub const WM8993_DRC_R1_SLOPE_COMP_MASK: u32 = 0xE000;   /* DRC_R1_SLOPE_COMP - [15:13] */
pub const WM8993_DRC_R1_SLOPE_COMP_SHIFT: u32 = 13;   /* DRC_R1_SLOPE_COMP - [15:13] */
pub const WM8993_DRC_R1_SLOPE_COMP_WIDTH: u32 = 3;   /* DRC_R1_SLOPE_COMP - [15:13] */
pub const WM8993_DRC_STARTUP_GAIN_MASK: u32 = 0x1F00;   /* DRC_STARTUP_GAIN - [12:8] */
pub const WM8993_DRC_STARTUP_GAIN_SHIFT: u32 = 8;   /* DRC_STARTUP_GAIN - [12:8] */
pub const WM8993_DRC_STARTUP_GAIN_WIDTH: u32 = 5;   /* DRC_STARTUP_GAIN - [12:8] */


// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
