/* SPDX-License-Identifier: GPL-2.0-only */

/* ES9356 Implementation-define */
pub const ES9356_FLAGS_HP: u32 = 0x2003;
pub const ES9356_CSM_RESET: u32 = 0x2020;
pub const ES9356_FUC_RESET: u32 = 0x2021;
pub const ES9356_STATE: u32 = 0x2022;
pub const ES9356_VMID_TIME: u32 = 0x2023;
pub const ES9356_STATE_TIME: u32 = 0x2024;
pub const ES9356_HP_SPK_TIME: u32 = 0x2025;
pub const ES9356_WP_ENABLE: u32 = 0x2026;
pub const ES9356_DMIC_GPIO: u32 = 0x2027;
pub const ES9356_ENDPOINT_MODE: u32 = 0x2028;

/* HP DETECT */
pub const ES9356_HP_TYPE: u32 = 0x2029;
pub const ES9356_HP_DETECTTIME: u32 = 0x202A;
pub const ES9356_MICBIAS_SEL: u32 = 0x202B;
pub const ES9356_KEY_PRESS_TIME: u32 = 0x202C;
pub const ES9356_KEY_RELEASE_TIME: u32 = 0x202D;
pub const ES9356_KEY_HOLD_TIME: u32 = 0x202E;
pub const ES9356_BTSEL_REF: u32 = 0x202F;
pub const ES9356_BUTTON_CHARGE: u32 = 0x2030;

pub const ES9356_KEYD_DETECT: u32 = 0x2031;
pub const ES9356_DPEN_TIME: u32 = 0x2032;
pub const ES9356_TIMER_CHECK: u32 = 0x2033;
pub const ES9356_IBIASGEN: u32 = 0x2041;
pub const ES9356_VMID1SEL: u32 = 0x2042;
pub const ES9356_VMID1STL: u32 = 0x2043;
pub const ES9356_VMID2SEL: u32 = 0x2044;
pub const ES9356_VMID2STL: u32 = 0x2045;
pub const ES9356_VSEL: u32 = 0x2046;
pub const ES9356_MICBIAS_CTL: u32 = 0x2047;
pub const ES9356_HPDETECT_CTL: u32 = 0x2048;
pub const ES9356_MICBIAS_RES: u32 = 0x2049;

/* CLK */
pub const ES9356_CLK_SEL: u32 = 0x2050;
pub const ES9356_CLK_CTL: u32 = 0x2051;
pub const ES9356_DETCLK_CTL: u32 = 0x2052;
pub const ES9356_CPCLK_CTL: u32 = 0x2053;
pub const ES9356_SPKCLK_CTL: u32 = 0x2054;
pub const ES9356_PRE_DIV_CTL: u32 = 0x2055;
pub const ES9356_DLL_MODE: u32 = 0x2056;
pub const ES9356_ANACLK_SEL: u32 = 0x2057;
pub const ES9356_OSRCLK_SEL: u32 = 0x2058;
pub const ES9356_DSPCLK_SEL: u32 = 0x2059;
pub const ES9356_SPK9M_MODE: u32 = 0x205a;

/* ADC DIG CTL */
pub const ES9356_DMIC_POL: u32 = 0x2061;
pub const ES9356_ADC_SWAP: u32 = 0x2062;
pub const ES9356_ADC_OSR: u32 = 0x2063;
pub const ES9356_ADC_OSRGAIN: u32 = 0x2064;
pub const ES9356_ADC_CLEARRAM: u32 = 0x2065;
pub const ES9356_ADC_RAMP: u32 = 0x2066;
pub const ES9356_ADC_HPF1: u32 = 0x2067;
pub const ES9356_ADC_HPF2: u32 = 0x2068;
pub const ES9356_ADC_ALC: u32 = 0x206C;
pub const ES9356_ALC_LEVEL: u32 = 0x206D;
pub const ES9356_ALC_RAMP_WINSIZE: u32 = 0x206E;

/* ADC ANA CTL */
pub const ES9356_ADC_REF_EN: u32 = 0x2080;
pub const ES9356_ADC_AMIC_CTL: u32 = 0x2081;
pub const ES9356_ADC_ANA: u32 = 0x2082;
pub const ES9356_PGA_CTL: u32 = 0x2083;
pub const ES9356_ADC_INT: u32 = 0x2084;
pub const ES9356_ADC_VCM: u32 = 0x2085;
pub const ES9356_ADC_VRPBIAS: u32 = 0x2086;
pub const ES9356_ADC_LP: u32 = 0x2087;

/* DAC DIG CTL */
pub const ES9356_DAC_FSMODE: u32 = 0x2090;
pub const ES9356_DAC_OSR: u32 = 0x2091;
pub const ES9356_DAC_INV: u32 = 0x2092;
pub const ES9356_DAC_RAMP: u32 = 0x2093;
pub const ES9356_DAC_VPPSCALE: u32 = 0x2094;
pub const ES9356_DAC_SWAP: u32 = 0x2097;
pub const ES9356_SPKCMP_VPPSC: u32 = 0x20A0;
pub const ES9356_CALIBRATION_TIME: u32 = 0x20A1;
pub const ES9356_CALIBRATION_SETTING: u32 = 0x20A2;
pub const ES9356_DAC_OFFSET_LH: u32 = 0x20A3;
pub const ES9356_DAC_OFFSET_LL: u32 = 0x20A4;
pub const ES9356_DAC_OFFSET_RH: u32 = 0x20A5;
pub const ES9356_DAC_OFFSET_RL: u32 = 0x20A6;

/* DAC ANA CTL */
pub const ES9356_DAC_REF_EN: u32 = 0x20B0;
pub const ES9356_DAC_ENABLE: u32 = 0x20B1;
pub const ES9356_DAC_VROI: u32 = 0x20B2;
pub const ES9356_DAC_LP: u32 = 0x20B3;

/* HP CTL */
pub const ES9356_CHARGEPUMP_CTL: u32 = 0x20C0;
pub const ES9356_CPLDO_CTL: u32 = 0x20C1;
pub const ES9356_HP_REF_CTL: u32 = 0x20C2;
pub const ES9356_HP_IBIAS: u32 = 0x20C3;
pub const ES9356_HP_EN: u32 = 0x20C4;
pub const ES9356_HP_VOLUME: u32 = 0x20C5;
pub const ES9356_HP_LP: u32 = 0x20C6;

/* SPK CTL */
pub const ES9356_SPKLDO_CTL: u32 = 0x20D0;
pub const ES9356_CLASSD_CTL: u32 = 0x20D1;
pub const ES9356_SPK_HBDG: u32 = 0x20D5;
pub const ES9356_SPK_VOLUME: u32 = 0x20D7;
pub const ES9356_SPK_SCP: u32 = 0x20D8;
pub const ES9356_SPK_DT: u32 = 0x20D9;
pub const ES9356_SPK_OTP: u32 = 0x20DA;
pub const ES9356_SPKBIAS_COMP: u32 = 0x20DB;

/* ES9356 SDCA Control - function number */
pub const FUNC_NUM_UAJ: u32 = 0x01;
pub const FUNC_NUM_MIC: u32 = 0x02;
pub const FUNC_NUM_AMP: u32 = 0x03;
pub const FUNC_NUM_HID: u32 = 0x04;

/* ES9356 SDCA entity */
pub const ES9356_SDCA_ENT0: u32 = 0x00;
pub const ES9356_SDCA_ENT_PDE11: u32 = 0x03;
pub const ES9356_SDCA_ENT_FU11: u32 = 0x04;
pub const ES9356_SDCA_ENT_XU12: u32 = 0x05;
pub const ES9356_SDCA_ENT_FU113: u32 = 0x07;
pub const ES9356_SDCA_ENT_CS113: u32 = 0x09;
pub const ES9356_SDCA_ENT_PPU11: u32 = 0x0C;

pub const ES9356_SDCA_ENT_CS21: u32 = 0x02;
pub const ES9356_SDCA_ENT_PPU21: u32 = 0x03;
pub const ES9356_SDCA_ENT_FU21: u32 = 0X04;
pub const ES9356_SDCA_ENT_XU22: u32 = 0x06;
pub const ES9356_SDCA_ENT_SAPU29: u32 = 0x03;
pub const ES9356_SDCA_ENT_PDE23: u32 = 0x0B;
pub const ES9356_SDCA_ENT_HID01: u32 = 0x01;

pub const ES9356_SDCA_ENT_CS41: u32 = 0x02;
pub const ES9356_SDCA_ENT_FU35: u32 = 0x04;
pub const ES9356_SDCA_ENT_XU42: u32 = 0x06;
pub const ES9356_SDCA_ENT_FU41: u32 = 0x07;
pub const ES9356_SDCA_ENT_PDE47: u32 = 0x0E;
pub const ES9356_SDCA_ENT_IT33: u32 = 0x0F;
pub const ES9356_SDCA_ENT_PDE34: u32 = 0x10;
pub const ES9356_SDCA_ENT_FU33: u32 = 0x11;
pub const ES9356_SDCA_ENT_XU36: u32 = 0x13;
pub const ES9356_SDCA_ENT_FU36: u32 = 0x15;
pub const ES9356_SDCA_ENT_CS36: u32 = 0x17;
pub const ES9356_SDCA_ENT_GE35: u32 = 0x18;

/* ES9356 SDCA control */
pub const ES9356_SDCA_CTL_SAMPLE_FREQ_INDEX: u32 = 0x10;
pub const ES9356_SDCA_CTL_FU_MUTE: u32 = 0x01;
pub const ES9356_SDCA_CTL_FU_VOLUME: u32 = 0x02;
pub const ES9356_SDCA_CTL_HIDTX_CURRENT_OWNER: u32 = 0x10;
pub const ES9356_SDCA_CTL_SELECTED_MODE: u32 = 0x01;
pub const ES9356_SDCA_CTL_DETECTED_MODE: u32 = 0x02;
pub const ES9356_SDCA_CTL_REQ_POWER_STATE: u32 = 0x01;
pub const ES9356_SDCA_CTL_FU_CH_GAIN: u32 = 0x0b;
pub const ES9356_SDCA_CTL_FUNC_STATUS: u32 = 0x10;
pub const ES9356_SDCA_CTL_ACTUAL_POWER_STATE: u32 = 0x10;
pub const ES9356_SDCA_CTL_POSTURE_NUMBER: u32 = 0x00;

/* ES9356 SDCA channel */
pub const CH_L: u32 = 0x01;
pub const CH_R: u32 = 0x02;
pub const MBQ: u32 = 0x2000;

/* ES9356 HID */
pub const ES9356_BUF_ADDR_HID: u32 = 0x44000000;
pub const ES9356_HID_BYTE2: u32 = 0x44000001;
pub const ES9356_HID_BYTE3: u32 = 0x44000002;
pub const ES9356_HID_BYTE4: u32 = 0x44000003;

/* ES9356 Volume Setting */
pub const ES9356_VU_BASE: i32 = 768;
pub const ES9356_OFFSET_HIGH: u32 = 0x07F8;
pub const ES9356_OFFSET_LOW: u32 = 0x0007;
pub const ES9356_DEFAULT_VOLUME: i32 = 0x00;
pub const ES9356_VOLUME_STEP: i32 = 32;
pub const ES9356_VOLUME_MIN: i32 = -768;
pub const ES9356_VOLUME_MAX: i32 = 285;
pub const ES9356_AMIC_GAIN_STEP: i32 = 768;
pub const ES9356_DMIC_GAIN_STEP: i32 = 1536;
pub const ES9356_GAIN_MIN: i32 = 0;
pub const ES9356_AMIC_GAIN_MAX: i32 = 10;
pub const ES9356_DMIC_GAIN_MAX: i32 = 3;

pub const ES9356_DMIC: i32 = 1; /* For dmic */
pub const ES9356_JACK_IN: i32 = 2; /* For headset mic */
pub const ES9356_AMP: i32 = 3; /* For speaker */
pub const ES9356_JACK_OUT: i32 = 4; /* For headphone */

pub const ES9356_SDCA_RATE_16000HZ: i32 = 0;
pub const ES9356_SDCA_RATE_24000HZ: i32 = 1;
pub const ES9356_SDCA_RATE_32000HZ: i32 = 2;
pub const ES9356_SDCA_RATE_44100HZ: i32 = 3;
pub const ES9356_SDCA_RATE_48000HZ: i32 = 4;
pub const ES9356_SDCA_RATE_88200HZ: i32 = 5;
pub const ES9356_SDCA_RATE_96000HZ: i32 = 6;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
