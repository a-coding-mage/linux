/* SPDX-License-Identifier: GPL-2.0 */
pub const CS4245_CHIP_ID: u32 = 0x01;
pub const CS4245_POWER_CTRL: u32 = 0x02;
pub const CS4245_DAC_CTRL_1: u32 = 0x03;
pub const CS4245_ADC_CTRL: u32 = 0x04;
pub const CS4245_MCLK_FREQ: u32 = 0x05;
pub const CS4245_SIGNAL_SEL: u32 = 0x06;
pub const CS4245_PGA_B_CTRL: u32 = 0x07;
pub const CS4245_PGA_A_CTRL: u32 = 0x08;
pub const CS4245_ANALOG_IN: u32 = 0x09;
pub const CS4245_DAC_A_CTRL: u32 = 0x0a;
pub const CS4245_DAC_B_CTRL: u32 = 0x0b;
pub const CS4245_DAC_CTRL_2: u32 = 0x0c;
pub const CS4245_INT_STATUS: u32 = 0x0d;
pub const CS4245_INT_MASK: u32 = 0x0e;
pub const CS4245_INT_MODE_MSB: u32 = 0x0f;
pub const CS4245_INT_MODE_LSB: u32 = 0x10;

/* Chip ID */
pub const CS4245_CHIP_PART_MASK: u32 = 0xf0;
pub const CS4245_CHIP_REV_MASK: u32 = 0x0f;

/* Power Control */
pub const CS4245_FREEZE: u32 = 0x80;
pub const CS4245_PDN_MIC: u32 = 0x08;
pub const CS4245_PDN_ADC: u32 = 0x04;
pub const CS4245_PDN_DAC: u32 = 0x02;
pub const CS4245_PDN: u32 = 0x01;

/* DAC Control */
pub const CS4245_DAC_FM_MASK: u32 = 0xc0;
pub const CS4245_DAC_FM_SINGLE: u32 = 0x00;
pub const CS4245_DAC_FM_DOUBLE: u32 = 0x40;
pub const CS4245_DAC_FM_QUAD: u32 = 0x80;
pub const CS4245_DAC_DIF_MASK: u32 = 0x30;
pub const CS4245_DAC_DIF_LJUST: u32 = 0x00;
pub const CS4245_DAC_DIF_I2S: u32 = 0x10;
pub const CS4245_DAC_DIF_RJUST_16: u32 = 0x20;
pub const CS4245_DAC_DIF_RJUST_24: u32 = 0x30;
pub const CS4245_RESERVED_1: u32 = 0x08;
pub const CS4245_MUTE_DAC: u32 = 0x04;
pub const CS4245_DEEMPH: u32 = 0x02;
pub const CS4245_DAC_MASTER: u32 = 0x01;

/* ADC Control */
pub const CS4245_ADC_FM_MASK: u32 = 0xc0;
pub const CS4245_ADC_FM_SINGLE: u32 = 0x00;
pub const CS4245_ADC_FM_DOUBLE: u32 = 0x40;
pub const CS4245_ADC_FM_QUAD: u32 = 0x80;
pub const CS4245_ADC_DIF_MASK: u32 = 0x10;
pub const CS4245_ADC_DIF_LJUST: u32 = 0x00;
pub const CS4245_ADC_DIF_I2S: u32 = 0x10;
pub const CS4245_MUTE_ADC: u32 = 0x04;
pub const CS4245_HPF_FREEZE: u32 = 0x02;
pub const CS4245_ADC_MASTER: u32 = 0x01;

/* MCLK Frequency */
pub const CS4245_MCLK1_MASK: u32 = 0x70;
pub const CS4245_MCLK1_SHIFT: u32 = 4;
pub const CS4245_MCLK2_MASK: u32 = 0x07;
pub const CS4245_MCLK2_SHIFT: u32 = 0;
pub const CS4245_MCLK_1: u32 = 0;
pub const CS4245_MCLK_1_5: u32 = 1;
pub const CS4245_MCLK_2: u32 = 2;
pub const CS4245_MCLK_3: u32 = 3;
pub const CS4245_MCLK_4: u32 = 4;

/* Signal Selection */
pub const CS4245_A_OUT_SEL_MASK: u32 = 0x60;
pub const CS4245_A_OUT_SEL_HIZ: u32 = 0x00;
pub const CS4245_A_OUT_SEL_DAC: u32 = 0x20;
pub const CS4245_A_OUT_SEL_PGA: u32 = 0x40;
pub const CS4245_LOOP: u32 = 0x02;
pub const CS4245_ASYNCH: u32 = 0x01;

/* Channel B/A PGA Control */
pub const CS4245_PGA_GAIN_MASK: u32 = 0x3f;

/* ADC Input Control */
pub const CS4245_PGA_SOFT: u32 = 0x10;
pub const CS4245_PGA_ZERO: u32 = 0x08;
pub const CS4245_SEL_MASK: u32 = 0x07;
pub const CS4245_SEL_MIC: u32 = 0x00;
pub const CS4245_SEL_INPUT_1: u32 = 0x01;
pub const CS4245_SEL_INPUT_2: u32 = 0x02;
pub const CS4245_SEL_INPUT_3: u32 = 0x03;
pub const CS4245_SEL_INPUT_4: u32 = 0x04;
pub const CS4245_SEL_INPUT_5: u32 = 0x05;
pub const CS4245_SEL_INPUT_6: u32 = 0x06;

/* DAC Channel A/B Volume Control */
pub const CS4245_VOL_MASK: u32 = 0xff;

/* DAC Control 2 */
pub const CS4245_DAC_SOFT: u32 = 0x80;
pub const CS4245_DAC_ZERO: u32 = 0x40;
pub const CS4245_INVERT_DAC: u32 = 0x20;
pub const CS4245_INT_ACTIVE_HIGH: u32 = 0x01;

/* Interrupt Status/Mask/Mode */
pub const CS4245_ADC_CLK_ERR: u32 = 0x08;
pub const CS4245_DAC_CLK_ERR: u32 = 0x04;
pub const CS4245_ADC_OVFL: u32 = 0x02;
pub const CS4245_ADC_UNDRFL: u32 = 0x01;

pub const CS4245_SPI_ADDRESS_S: u32 = 0x9e << 16;
pub const CS4245_SPI_WRITE_S: u32 = 0 << 16;

pub const CS4245_SPI_ADDRESS: u32 = 0x9e;
pub const CS4245_SPI_WRITE: u32 = 0;
pub const CS4245_SPI_READ: u32 = 1;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
