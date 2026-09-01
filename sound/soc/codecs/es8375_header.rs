/* SPDX-License-Identifier: GPL-2.0-only */
/*
* ES8375.h  --  ES8375 ALSA SoC Audio Codec
*
* Authors:
*
* Based on ES8375.h by Michael Zhang
*/

// Registors
pub const ES8375_RESET1: u32 = 0x00;
pub const ES8375_MCLK_SEL: u32 = 0x01;
pub const ES8375_CLK_MGR2: u32 = 0x02;
pub const ES8375_CLK_MGR3: u32 = 0x03;
pub const ES8375_CLK_MGR4: u32 = 0x04;
pub const ES8375_CLK_MGR5: u32 = 0x05;
pub const ES8375_CLK_MGR6: u32 = 0x06;
pub const ES8375_CLK_MGR7: u32 = 0x07;
pub const ES8375_CLK_MGR8: u32 = 0x08;
pub const ES8375_CLK_MGR9: u32 = 0x09;
pub const ES8375_CLK_MGR10: u32 = 0x0A;
pub const ES8375_CLK_MGR11: u32 = 0x0B;
pub const ES8375_CLK_MGR12: u32 = 0x0C;
pub const ES8375_DIV_SPKCLK: u32 = 0x0E;
pub const ES8375_CSM1: u32 = 0x0F;
pub const ES8375_CSM2: u32 = 0x10;
pub const ES8375_VMID_CHARGE2: u32 = 0x11;
pub const ES8375_VMID_CHARGE3: u32 = 0x12;
pub const ES8375_SDP: u32 = 0x15;
pub const ES8375_SDP2: u32 = 0x16;
pub const ES8375_ADC1: u32 = 0x17;
pub const ES8375_ADC2: u32 = 0x18;
pub const ES8375_ADC_OSR_GAIN: u32 = 0x19;
pub const ES8375_ADC_VOLUME: u32 = 0x1A;
pub const ES8375_ADC_AUTOMUTE: u32 = 0x1B;
pub const ES8375_ADC_AUTOMUTE_ATTN: u32 = 0x1C;
pub const ES8375_HPF1: u32 = 0x1D;
pub const ES8375_DAC1: u32 = 0x1F;
pub const ES8375_DAC2: u32 = 0x20;
pub const ES8375_DAC_VOLUME: u32 = 0x21;
pub const ES8375_DAC_VPPSCALE: u32 = 0x22;
pub const ES8375_DAC_AUTOMUTE1: u32 = 0x23;
pub const ES8375_DAC_AUTOMUTE: u32 = 0x24;
pub const ES8375_DAC_CAL: u32 = 0x25;
pub const ES8375_DAC_OTP: u32 = 0x27;
pub const ES8375_ANALOG_SPK1: u32 = 0x28;
pub const ES8375_ANALOG_SPK2: u32 = 0x29;
pub const ES8375_VMID_SEL: u32 = 0x2D;
pub const ES8375_ANALOG1: u32 = 0x2E;
pub const ES8375_ANALOG2: u32 = 0x32;
pub const ES8375_ANALOG3: u32 = 0x37;
pub const ES8375_ADC2DAC_CLKTRI: u32 = 0xF8;
pub const ES8375_SYS_CTRL2: u32 = 0xF9;
pub const ES8375_FLAGS2: u32 = 0xFB;
pub const ES8375_SPK_OFFSET: u32 = 0xFC;
pub const ES8375_CHIP_ID1: u32 = 0xFD;
pub const ES8375_CHIP_ID0: u32 = 0xFE;
pub const ES8375_CHIP_VERSION: u32 = 0xFF;

// Bit Shifts
pub const ADC_OSR_GAIN_SHIFT_0: u32 = 0;
pub const ADC_RAMPRATE_SHIFT_0: u32 = 0;
pub const ADC_VOLUME_SHIFT_0: u32 = 0;
pub const ADC_AUTOMUTE_NG_SHIFT_0: u32 = 0;
pub const ADC_AUTOMUTE_ATTN_SHIFT_0: u32 = 0;
pub const DAC_RAMPRATE_SHIFT_0: u32 = 0;
pub const DAC_VOLUME_SHIFT_0: u32 = 0;
pub const DAC_VPPSCALE_SHIFT_0: u32 = 0;
pub const DAC_AUTOMUTE_NG_SHIFT_0: u32 = 0;
pub const DAC_AUTOMUTE_ATTN_SHIFT_0: u32 = 0;
pub const DMIC_GAIN_SHIFT_2: u32 = 2;
pub const ADC_AUTOMUTE_WS_SHIFT_3: u32 = 3;
pub const DMIC_POL_SHIFT_4: u32 = 4;
pub const DAC_RAMCLR_SHIFT_4: u32 = 4;
pub const ES8375_EN_MODL_SHIFT_4: u32 = 4;
pub const ADC_RAMCLR_SHIFT_5: u32 = 5;
pub const ADC_HPF_SHIFT_5: u32 = 5;
pub const DAC_INV_SHIFT_5: u32 = 5;
pub const DAC_AUTOMUTE_WS_SHIFT_5: u32 = 5;
pub const ES8375_EN_PGAL_SHIFT_5: u32 = 5;
pub const ES8375_ADC_P2S_MUTE_SHIFT_5: u32 = 5;
pub const ADC_INV_SHIFT_6: u32 = 6;
pub const DAC_DEMMUTE_SHIFT_6: u32 = 6;
pub const ES8375_DAC_S2P_MUTE_SHIFT_6: u32 = 6;
pub const ADC_SRC_SHIFT_7: u32 = 7;
pub const ADC_AUTOMUTE_SHIFT_7: u32 = 7;
pub const DAC_DSMMUTE_SHIFT_7: u32 = 7;
pub const DAC_AUTOMUTE_EN_SHIFT_7: u32 = 7;

// Function values
pub const ES8375_ADC_OSR_GAIN_MAX: u32 = 0x3F;
pub const ES8375_DMIC_GAIN_MAX: u32 = 0x04;
pub const ES8375_ADC_AUTOMUTE_ATTN_MAX: u32 = 0x1F;
pub const ES8375_AUTOMUTE_NG_MAX: u32 = 0x07;
pub const ES8375_ADC_VOLUME_MAX: u32 = 0xFF;
pub const ES8375_DAC_VOLUME_MAX: u32 = 0xFF;
pub const ES8375_DAC_VPPSCALE_MAX: u32 = 0x3F;
pub const ES8375_DAC_AUTOMUTE_ATTN_MAX: u32 = 0x17;
pub const ES8375_REG_MAX: u32 = 0xFF;

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ES8375_supplies {
    ES8375_SUPPLY_VD = 0,
    ES8375_SUPPLY_VA = 1,
}

// Properties
pub const ES8375_3V3: u32 = 1;
pub const ES8375_1V8: u32 = 0;

pub const ES8375_MCLK_PIN: u32 = 0;
pub const ES8375_BCLK_PIN: u32 = 1;
pub const ES8375_MCLK_SOURCE: u32 = ES8375_MCLK_PIN;

pub const DMIC_POSITIVE_EDGE: u32 = 0;
pub const DMIC_NEGATIVE_EDGE: u32 = 1;
pub const DMIC_POL: u32 = DMIC_POSITIVE_EDGE;

pub const PA_SHUTDOWN: u32 = 0;
pub const PA_ENABLE: u32 = 1;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
