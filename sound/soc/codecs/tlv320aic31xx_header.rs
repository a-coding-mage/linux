// SPDX-License-Identifier: GPL-2.0
/*
 * ALSA SoC TLV320AIC31xx CODEC Driver Definitions
 *
 * Copyright (C) 2014-2017 Texas Instruments Incorporated - https://www.ti.com/
 */

// C header guard removed: _TLV320AIC31XX_H.

pub const fn BIT(nr: u32) -> u32 {
    1u32 << nr
}

pub const fn GENMASK(h: u32, l: u32) -> u32 {
    (!0u32 << l) & (!0u32 >> (31 - h))
}

pub const AIC31XX_RATES: u32 = SNDRV_PCM_RATE_8000_192000;

pub const AIC31XX_FORMATS: u64 = SNDRV_PCM_FMTBIT_S16_LE
    | SNDRV_PCM_FMTBIT_S20_3LE
    | SNDRV_PCM_FMTBIT_S24_3LE
    | SNDRV_PCM_FMTBIT_S24_LE
    | SNDRV_PCM_FMTBIT_S32_LE;

pub const AIC31XX_STEREO_CLASS_D_BIT: u32 = BIT(1);
pub const AIC31XX_MINIDSP_BIT: u32 = BIT(2);
pub const DAC31XX_BIT: u32 = BIT(3);

pub const AIC31XX_JACK_MASK: u32 = SND_JACK_HEADPHONE | SND_JACK_HEADSET | SND_JACK_BTN_0;

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum aic31xx_type {
    AIC3100 = 0,
    AIC3110 = AIC31XX_STEREO_CLASS_D_BIT as isize,
    AIC3120 = AIC31XX_MINIDSP_BIT as isize,
    AIC3111 = (AIC31XX_STEREO_CLASS_D_BIT | AIC31XX_MINIDSP_BIT) as isize,
    DAC3100 = DAC31XX_BIT as isize,
    DAC3101 = (DAC31XX_BIT | AIC31XX_STEREO_CLASS_D_BIT) as isize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aic31xx_pdata {
    pub codec_type: aic31xx_type,
    pub gpio_reset: ::core::ffi::c_uint,
    pub micbias_vg: ::core::ffi::c_int,
}

pub const fn AIC31XX_REG(page: u32, reg: u32) -> u32 {
    (page * 128) + reg
}

pub const AIC31XX_PAGECTL: u32 = AIC31XX_REG(0, 0); /* Page Control Register */

/* Page 0 Registers */
pub const AIC31XX_RESET: u32 = AIC31XX_REG(0, 1); /* Software reset register */
pub const AIC31XX_OT_FLAG: u32 = AIC31XX_REG(0, 3); /* OT FLAG register */
pub const AIC31XX_CLKMUX: u32 = AIC31XX_REG(0, 4); /* Clock clock Gen muxing, Multiplexers*/
pub const AIC31XX_PLLPR: u32 = AIC31XX_REG(0, 5); /* PLL P and R-VAL register */
pub const AIC31XX_PLLJ: u32 = AIC31XX_REG(0, 6); /* PLL J-VAL register */
pub const AIC31XX_PLLDMSB: u32 = AIC31XX_REG(0, 7); /* PLL D-VAL MSB register */
pub const AIC31XX_PLLDLSB: u32 = AIC31XX_REG(0, 8); /* PLL D-VAL LSB register */
pub const AIC31XX_NDAC: u32 = AIC31XX_REG(0, 11); /* DAC NDAC_VAL register*/
pub const AIC31XX_MDAC: u32 = AIC31XX_REG(0, 12); /* DAC MDAC_VAL register */
pub const AIC31XX_DOSRMSB: u32 = AIC31XX_REG(0, 13); /* DAC OSR setting register 1, MSB value */
pub const AIC31XX_DOSRLSB: u32 = AIC31XX_REG(0, 14); /* DAC OSR setting register 2, LSB value */
pub const AIC31XX_MINI_DSP_INPOL: u32 = AIC31XX_REG(0, 16);
pub const AIC31XX_NADC: u32 = AIC31XX_REG(0, 18); /* Clock setting register 8, PLL */
pub const AIC31XX_MADC: u32 = AIC31XX_REG(0, 19); /* Clock setting register 9, PLL */
pub const AIC31XX_AOSR: u32 = AIC31XX_REG(0, 20); /* ADC Oversampling (AOSR) Register */
pub const AIC31XX_CLKOUTMUX: u32 = AIC31XX_REG(0, 25); /* Clock setting register 9, Multiplexers */
pub const AIC31XX_CLKOUTMVAL: u32 = AIC31XX_REG(0, 26); /* Clock setting register 10, CLOCKOUT M divider value */
pub const AIC31XX_IFACE1: u32 = AIC31XX_REG(0, 27); /* Audio Interface Setting Register 1 */
pub const AIC31XX_DATA_OFFSET: u32 = AIC31XX_REG(0, 28); /* Audio Data Slot Offset Programming */
pub const AIC31XX_IFACE2: u32 = AIC31XX_REG(0, 29); /* Audio Interface Setting Register 2 */
pub const AIC31XX_BCLKN: u32 = AIC31XX_REG(0, 30); /* Clock setting register 11, BCLK N Divider */
pub const AIC31XX_IFACESEC1: u32 = AIC31XX_REG(0, 31); /* Audio Interface Setting Register 3, Secondary Audio Interface */
pub const AIC31XX_IFACESEC2: u32 = AIC31XX_REG(0, 32); /* Audio Interface Setting Register 4 */
pub const AIC31XX_IFACESEC3: u32 = AIC31XX_REG(0, 33); /* Audio Interface Setting Register 5 */
pub const AIC31XX_I2C: u32 = AIC31XX_REG(0, 34); /* I2C Bus Condition */
pub const AIC31XX_ADCFLAG: u32 = AIC31XX_REG(0, 36); /* ADC FLAG */
pub const AIC31XX_DACFLAG1: u32 = AIC31XX_REG(0, 37); /* DAC Flag Registers */
pub const AIC31XX_DACFLAG2: u32 = AIC31XX_REG(0, 38);
pub const AIC31XX_OFFLAG: u32 = AIC31XX_REG(0, 39); /* Sticky Interrupt flag (overflow) */
pub const AIC31XX_INTRDACFLAG: u32 = AIC31XX_REG(0, 44); /* Sticy DAC Interrupt flags */
pub const AIC31XX_INTRADCFLAG: u32 = AIC31XX_REG(0, 45); /* Sticy ADC Interrupt flags */
pub const AIC31XX_INTRDACFLAG2: u32 = AIC31XX_REG(0, 46); /* DAC Interrupt flags 2 */
pub const AIC31XX_INTRADCFLAG2: u32 = AIC31XX_REG(0, 47); /* ADC Interrupt flags 2 */
pub const AIC31XX_INT1CTRL: u32 = AIC31XX_REG(0, 48); /* INT1 interrupt control */
pub const AIC31XX_INT2CTRL: u32 = AIC31XX_REG(0, 49); /* INT2 interrupt control */
pub const AIC31XX_GPIO1: u32 = AIC31XX_REG(0, 51); /* GPIO1 control */
pub const AIC31XX_DACPRB: u32 = AIC31XX_REG(0, 60);
pub const AIC31XX_ADCPRB: u32 = AIC31XX_REG(0, 61); /* ADC Instruction Set Register */
pub const AIC31XX_DACSETUP: u32 = AIC31XX_REG(0, 63); /* DAC channel setup register */
pub const AIC31XX_DACMUTE: u32 = AIC31XX_REG(0, 64); /* DAC Mute and volume control register */
pub const AIC31XX_LDACVOL: u32 = AIC31XX_REG(0, 65); /* Left DAC channel digital volume control */
pub const AIC31XX_RDACVOL: u32 = AIC31XX_REG(0, 66); /* Right DAC channel digital volume control */
pub const AIC31XX_HSDETECT: u32 = AIC31XX_REG(0, 67); /* Headset detection */
pub const AIC31XX_ADCSETUP: u32 = AIC31XX_REG(0, 81); /* ADC Digital Mic */
pub const AIC31XX_ADCFGA: u32 = AIC31XX_REG(0, 82); /* ADC Digital Volume Control Fine Adjust */
pub const AIC31XX_ADCVOL: u32 = AIC31XX_REG(0, 83); /* ADC Digital Volume Control Coarse Adjust */

/* Page 1 Registers */
pub const AIC31XX_HPDRIVER: u32 = AIC31XX_REG(1, 31); /* Headphone drivers */
pub const AIC31XX_SPKAMP: u32 = AIC31XX_REG(1, 32); /* Class-D Speakear Amplifier */
pub const AIC31XX_HPPOP: u32 = AIC31XX_REG(1, 33); /* HP Output Drivers POP Removal Settings */
pub const AIC31XX_SPPGARAMP: u32 = AIC31XX_REG(1, 34); /* Output Driver PGA Ramp-Down Period Control */
pub const AIC31XX_DACMIXERROUTE: u32 = AIC31XX_REG(1, 35); /* DAC_L and DAC_R Output Mixer Routing */
pub const AIC31XX_LANALOGHPL: u32 = AIC31XX_REG(1, 36); /* Left Analog Vol to HPL */
pub const AIC31XX_RANALOGHPR: u32 = AIC31XX_REG(1, 37); /* Right Analog Vol to HPR */
pub const AIC31XX_LANALOGSPL: u32 = AIC31XX_REG(1, 38); /* Left Analog Vol to SPL */
pub const AIC31XX_RANALOGSPR: u32 = AIC31XX_REG(1, 39); /* Right Analog Vol to SPR */
pub const AIC31XX_HPLGAIN: u32 = AIC31XX_REG(1, 40); /* HPL Driver */
pub const AIC31XX_HPRGAIN: u32 = AIC31XX_REG(1, 41); /* HPR Driver */
pub const AIC31XX_SPLGAIN: u32 = AIC31XX_REG(1, 42); /* SPL Driver */
pub const AIC31XX_SPRGAIN: u32 = AIC31XX_REG(1, 43); /* SPR Driver */
pub const AIC31XX_HPCONTROL: u32 = AIC31XX_REG(1, 44); /* HP Driver Control */
pub const AIC31XX_MICBIAS: u32 = AIC31XX_REG(1, 46); /* MIC Bias Control */
pub const AIC31XX_MICPGA: u32 = AIC31XX_REG(1, 47); /* MIC PGA*/
pub const AIC31XX_MICPGAPI: u32 = AIC31XX_REG(1, 48); /* Delta-Sigma Mono ADC Channel Fine-Gain Input Selection for P-Terminal */
pub const AIC31XX_MICPGAMI: u32 = AIC31XX_REG(1, 49); /* ADC Input Selection for M-Terminal */
pub const AIC31XX_MICPGACM: u32 = AIC31XX_REG(1, 50); /* Input CM Settings */

/* Bits, masks, and shifts */

/* AIC31XX_CLKMUX */
pub const AIC31XX_PLL_CLKIN_MASK: u32 = GENMASK(3, 2);
pub const AIC31XX_PLL_CLKIN_SHIFT: u32 = 2;
pub const AIC31XX_PLL_CLKIN_MCLK: u32 = 0x00;
pub const AIC31XX_PLL_CLKIN_BCLK: u32 = 0x01;
pub const AIC31XX_PLL_CLKIN_GPIO1: u32 = 0x02;
pub const AIC31XX_PLL_CLKIN_DIN: u32 = 0x03;
pub const AIC31XX_CODEC_CLKIN_MASK: u32 = GENMASK(1, 0);
pub const AIC31XX_CODEC_CLKIN_SHIFT: u32 = 0;
pub const AIC31XX_CODEC_CLKIN_MCLK: u32 = 0x00;
pub const AIC31XX_CODEC_CLKIN_BCLK: u32 = 0x01;
pub const AIC31XX_CODEC_CLKIN_GPIO1: u32 = 0x02;
pub const AIC31XX_CODEC_CLKIN_PLL: u32 = 0x03;

/* AIC31XX_PLLPR */
/* AIC31XX_NDAC */
/* AIC31XX_MDAC */
/* AIC31XX_NADC */
/* AIC31XX_MADC */
/* AIC31XX_BCLKN */
pub const AIC31XX_PLL_MASK: u32 = GENMASK(6, 0);
pub const AIC31XX_PM_MASK: u32 = BIT(7);

/* AIC31XX_IFACE1 */
pub const AIC31XX_IFACE1_DATATYPE_MASK: u32 = GENMASK(7, 6);
pub const AIC31XX_IFACE1_DATATYPE_SHIFT: u32 = 6;
pub const AIC31XX_I2S_MODE: u32 = 0x00;
pub const AIC31XX_DSP_MODE: u32 = 0x01;
pub const AIC31XX_RIGHT_JUSTIFIED_MODE: u32 = 0x02;
pub const AIC31XX_LEFT_JUSTIFIED_MODE: u32 = 0x03;
pub const AIC31XX_IFACE1_DATALEN_MASK: u32 = GENMASK(5, 4);
pub const AIC31XX_IFACE1_DATALEN_SHIFT: u32 = 4;
pub const AIC31XX_WORD_LEN_16BITS: u32 = 0x00;
pub const AIC31XX_WORD_LEN_20BITS: u32 = 0x01;
pub const AIC31XX_WORD_LEN_24BITS: u32 = 0x02;
pub const AIC31XX_WORD_LEN_32BITS: u32 = 0x03;
pub const AIC31XX_IFACE1_MASTER_MASK: u32 = GENMASK(3, 2);
pub const AIC31XX_BCLK_MASTER: u32 = BIT(3);
pub const AIC31XX_WCLK_MASTER: u32 = BIT(2);

/* AIC31XX_DATA_OFFSET */
pub const AIC31XX_DATA_OFFSET_MASK: u32 = GENMASK(7, 0);

/* AIC31XX_IFACE2 */
pub const AIC31XX_BCLKINV_MASK: u32 = BIT(3);
pub const AIC31XX_BDIVCLK_MASK: u32 = GENMASK(1, 0);
pub const AIC31XX_DAC2BCLK: u32 = 0x00;
pub const AIC31XX_DACMOD2BCLK: u32 = 0x01;
pub const AIC31XX_ADC2BCLK: u32 = 0x02;
pub const AIC31XX_ADCMOD2BCLK: u32 = 0x03;
pub const AIC31XX_KEEP_I2SCLK: u32 = BIT(2);

/* AIC31XX_ADCFLAG */
pub const AIC31XX_ADCPWRSTATUS_MASK: u32 = BIT(6);

/* AIC31XX_DACFLAG1 */
pub const AIC31XX_LDACPWRSTATUS_MASK: u32 = BIT(7);
pub const AIC31XX_HPLDRVPWRSTATUS_MASK: u32 = BIT(5);
pub const AIC31XX_SPLDRVPWRSTATUS_MASK: u32 = BIT(4);
pub const AIC31XX_RDACPWRSTATUS_MASK: u32 = BIT(3);
pub const AIC31XX_HPRDRVPWRSTATUS_MASK: u32 = BIT(1);
pub const AIC31XX_SPRDRVPWRSTATUS_MASK: u32 = BIT(0);

/* AIC31XX_OFFLAG */
pub const AIC31XX_DAC_OF_LEFT: u32 = BIT(7);
pub const AIC31XX_DAC_OF_RIGHT: u32 = BIT(6);
pub const AIC31XX_DAC_OF_SHIFTER: u32 = BIT(5);
pub const AIC31XX_ADC_OF: u32 = BIT(3);
pub const AIC31XX_ADC_OF_SHIFTER: u32 = BIT(1);

/* AIC31XX_INTRDACFLAG */
pub const AIC31XX_HPLSCDETECT: u32 = BIT(7);
pub const AIC31XX_HPRSCDETECT: u32 = BIT(6);
pub const AIC31XX_BUTTONPRESS: u32 = BIT(5);
pub const AIC31XX_HSPLUG: u32 = BIT(4);
pub const AIC31XX_LDRCTHRES: u32 = BIT(3);
pub const AIC31XX_RDRCTHRES: u32 = BIT(2);
pub const AIC31XX_DACSINT: u32 = BIT(1);
pub const AIC31XX_DACAINT: u32 = BIT(0);

/* AIC31XX_INT1CTRL */
pub const AIC31XX_HSPLUGDET: u32 = BIT(7);
pub const AIC31XX_BUTTONPRESSDET: u32 = BIT(6);
pub const AIC31XX_DRCTHRES: u32 = BIT(5);
pub const AIC31XX_AGCNOISE: u32 = BIT(4);
pub const AIC31XX_SC: u32 = BIT(3);
pub const AIC31XX_ENGINE: u32 = BIT(2);

/* AIC31XX_GPIO1 */
pub const AIC31XX_GPIO1_FUNC_MASK: u32 = GENMASK(5, 2);
pub const AIC31XX_GPIO1_FUNC_SHIFT: u32 = 2;
pub const AIC31XX_GPIO1_DISABLED: u32 = 0x00;
pub const AIC31XX_GPIO1_INPUT: u32 = 0x01;
pub const AIC31XX_GPIO1_GPI: u32 = 0x02;
pub const AIC31XX_GPIO1_GPO: u32 = 0x03;
pub const AIC31XX_GPIO1_CLKOUT: u32 = 0x04;
pub const AIC31XX_GPIO1_INT1: u32 = 0x05;
pub const AIC31XX_GPIO1_INT2: u32 = 0x06;
pub const AIC31XX_GPIO1_ADC_WCLK: u32 = 0x07;
pub const AIC31XX_GPIO1_SBCLK: u32 = 0x08;
pub const AIC31XX_GPIO1_SWCLK: u32 = 0x09;
pub const AIC31XX_GPIO1_ADC_MOD_CLK: u32 = 0x10;
pub const AIC31XX_GPIO1_SDOUT: u32 = 0x11;

/* AIC31XX_DACMUTE */
pub const AIC31XX_DACMUTE_MASK: u32 = GENMASK(3, 2);

/* AIC31XX_HSDETECT */
pub const AIC31XX_HSD_ENABLE: u32 = BIT(7);
pub const AIC31XX_HSD_TYPE_MASK: u32 = GENMASK(6, 5);
pub const AIC31XX_HSD_TYPE_SHIFT: u32 = 5;
pub const AIC31XX_HSD_NONE: u32 = 0x00;
pub const AIC31XX_HSD_HP: u32 = 0x01;
pub const AIC31XX_HSD_HS: u32 = 0x03;

/* AIC31XX_HPDRIVER */
pub const AIC31XX_HPD_OCMV_MASK: u32 = GENMASK(4, 3);
pub const AIC31XX_HPD_OCMV_SHIFT: u32 = 3;
pub const AIC31XX_HPD_OCMV_1_35V: u32 = 0x0;
pub const AIC31XX_HPD_OCMV_1_5V: u32 = 0x1;
pub const AIC31XX_HPD_OCMV_1_65V: u32 = 0x2;
pub const AIC31XX_HPD_OCMV_1_8V: u32 = 0x3;

/* AIC31XX_MICBIAS */
pub const AIC31XX_MICBIAS_MASK: u32 = GENMASK(1, 0);
pub const AIC31XX_MICBIAS_SHIFT: u32 = 0;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
