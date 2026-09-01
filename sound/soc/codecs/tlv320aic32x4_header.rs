/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * tlv320aic32x4.h
 */

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regmap_config {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regmap_range_cfg {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum aic32x4_type {
    AIC32X4_TYPE_AIC32X4 = 0,
    AIC32X4_TYPE_AIC32X6 = 1,
    AIC32X4_TYPE_TAS2505 = 2,
}

unsafe extern "C" {
    pub static aic32x4_regmap_pages: [regmap_range_cfg; 0];
    pub fn aic32x4_probe(
        dev: *mut device,
        regmap: *mut regmap,
        type_: aic32x4_type,
    ) -> i32;
    pub fn aic32x4_remove(dev: *mut device);
    pub fn aic32x4_register_clocks(dev: *mut device, mclk_name: *const core::ffi::c_char) -> i32;
}

/* tlv320aic32x4 register space (in decimal to match datasheet) */

pub const fn BIT(nr: u32) -> u32 {
    1u32 << nr
}

pub const fn GENMASK(h: u32, l: u32) -> u32 {
    u32::MAX.wrapping_shl(l) & u32::MAX.wrapping_shr(31 - h)
}

pub const fn AIC32X4_REG(page: u32, reg: u32) -> u32 {
    (page * 128) + reg
}

pub const AIC32X4_PSEL: u32 = AIC32X4_REG(0, 0);

pub const AIC32X4_RESET: u32 = AIC32X4_REG(0, 1);
pub const AIC32X4_CLKMUX: u32 = AIC32X4_REG(0, 4);
pub const AIC32X4_PLLPR: u32 = AIC32X4_REG(0, 5);
pub const AIC32X4_PLLJ: u32 = AIC32X4_REG(0, 6);
pub const AIC32X4_PLLDMSB: u32 = AIC32X4_REG(0, 7);
pub const AIC32X4_PLLDLSB: u32 = AIC32X4_REG(0, 8);
pub const AIC32X4_NDAC: u32 = AIC32X4_REG(0, 11);
pub const AIC32X4_MDAC: u32 = AIC32X4_REG(0, 12);
pub const AIC32X4_DOSRMSB: u32 = AIC32X4_REG(0, 13);
pub const AIC32X4_DOSRLSB: u32 = AIC32X4_REG(0, 14);
pub const AIC32X4_NADC: u32 = AIC32X4_REG(0, 18);
pub const AIC32X4_MADC: u32 = AIC32X4_REG(0, 19);
pub const AIC32X4_AOSR: u32 = AIC32X4_REG(0, 20);
pub const AIC32X4_CLKMUX2: u32 = AIC32X4_REG(0, 25);
pub const AIC32X4_CLKOUTM: u32 = AIC32X4_REG(0, 26);
pub const AIC32X4_IFACE1: u32 = AIC32X4_REG(0, 27);
pub const AIC32X4_IFACE2: u32 = AIC32X4_REG(0, 28);
pub const AIC32X4_IFACE3: u32 = AIC32X4_REG(0, 29);
pub const AIC32X4_BCLKN: u32 = AIC32X4_REG(0, 30);
pub const AIC32X4_IFACE4: u32 = AIC32X4_REG(0, 31);
pub const AIC32X4_IFACE5: u32 = AIC32X4_REG(0, 32);
pub const AIC32X4_IFACE6: u32 = AIC32X4_REG(0, 33);
pub const AIC32X4_GPIOCTL: u32 = AIC32X4_REG(0, 52);
pub const AIC32X4_DOUTCTL: u32 = AIC32X4_REG(0, 53);
pub const AIC32X4_DINCTL: u32 = AIC32X4_REG(0, 54);
pub const AIC32X4_MISOCTL: u32 = AIC32X4_REG(0, 55);
pub const AIC32X4_SCLKCTL: u32 = AIC32X4_REG(0, 56);
pub const AIC32X4_DACSPB: u32 = AIC32X4_REG(0, 60);
pub const AIC32X4_ADCSPB: u32 = AIC32X4_REG(0, 61);
pub const AIC32X4_DACSETUP: u32 = AIC32X4_REG(0, 63);
pub const AIC32X4_DACMUTE: u32 = AIC32X4_REG(0, 64);
pub const AIC32X4_LDACVOL: u32 = AIC32X4_REG(0, 65);
pub const AIC32X4_RDACVOL: u32 = AIC32X4_REG(0, 66);
pub const AIC32X4_ADCSETUP: u32 = AIC32X4_REG(0, 81);
pub const AIC32X4_ADCFGA: u32 = AIC32X4_REG(0, 82);
pub const AIC32X4_LADCVOL: u32 = AIC32X4_REG(0, 83);
pub const AIC32X4_RADCVOL: u32 = AIC32X4_REG(0, 84);
pub const AIC32X4_LAGC1: u32 = AIC32X4_REG(0, 86);
pub const AIC32X4_LAGC2: u32 = AIC32X4_REG(0, 87);
pub const AIC32X4_LAGC3: u32 = AIC32X4_REG(0, 88);
pub const AIC32X4_LAGC4: u32 = AIC32X4_REG(0, 89);
pub const AIC32X4_LAGC5: u32 = AIC32X4_REG(0, 90);
pub const AIC32X4_LAGC6: u32 = AIC32X4_REG(0, 91);
pub const AIC32X4_LAGC7: u32 = AIC32X4_REG(0, 92);
pub const AIC32X4_RAGC1: u32 = AIC32X4_REG(0, 94);
pub const AIC32X4_RAGC2: u32 = AIC32X4_REG(0, 95);
pub const AIC32X4_RAGC3: u32 = AIC32X4_REG(0, 96);
pub const AIC32X4_RAGC4: u32 = AIC32X4_REG(0, 97);
pub const AIC32X4_RAGC5: u32 = AIC32X4_REG(0, 98);
pub const AIC32X4_RAGC6: u32 = AIC32X4_REG(0, 99);
pub const AIC32X4_RAGC7: u32 = AIC32X4_REG(0, 100);

pub const AIC32X4_PWRCFG: u32 = AIC32X4_REG(1, 1);
pub const AIC32X4_LDOCTL: u32 = AIC32X4_REG(1, 2);
pub const AIC32X4_LPLAYBACK: u32 = AIC32X4_REG(1, 3);
pub const AIC32X4_RPLAYBACK: u32 = AIC32X4_REG(1, 4);
pub const AIC32X4_OUTPWRCTL: u32 = AIC32X4_REG(1, 9);
pub const AIC32X4_CMMODE: u32 = AIC32X4_REG(1, 10);
pub const AIC32X4_HPLROUTE: u32 = AIC32X4_REG(1, 12);
pub const AIC32X4_HPRROUTE: u32 = AIC32X4_REG(1, 13);
pub const AIC32X4_LOLROUTE: u32 = AIC32X4_REG(1, 14);
pub const AIC32X4_LORROUTE: u32 = AIC32X4_REG(1, 15);
pub const AIC32X4_HPLGAIN: u32 = AIC32X4_REG(1, 16);
pub const AIC32X4_HPRGAIN: u32 = AIC32X4_REG(1, 17);
pub const AIC32X4_LOLGAIN: u32 = AIC32X4_REG(1, 18);
pub const AIC32X4_LORGAIN: u32 = AIC32X4_REG(1, 19);
pub const AIC32X4_HEADSTART: u32 = AIC32X4_REG(1, 20);
pub const TAS2505_SPK: u32 = AIC32X4_REG(1, 45);
pub const TAS2505_SPKVOL1: u32 = AIC32X4_REG(1, 46);
pub const TAS2505_SPKVOL2: u32 = AIC32X4_REG(1, 48);
pub const AIC32X4_MICBIAS: u32 = AIC32X4_REG(1, 51);
pub const AIC32X4_LMICPGAPIN: u32 = AIC32X4_REG(1, 52);
pub const AIC32X4_LMICPGANIN: u32 = AIC32X4_REG(1, 54);
pub const AIC32X4_RMICPGAPIN: u32 = AIC32X4_REG(1, 55);
pub const AIC32X4_RMICPGANIN: u32 = AIC32X4_REG(1, 57);
pub const AIC32X4_FLOATINGINPUT: u32 = AIC32X4_REG(1, 58);
pub const AIC32X4_LMICPGAVOL: u32 = AIC32X4_REG(1, 59);
pub const AIC32X4_RMICPGAVOL: u32 = AIC32X4_REG(1, 60);
pub const TAS2505_REFPOWERUP: u32 = AIC32X4_REG(1, 122);
pub const AIC32X4_REFPOWERUP: u32 = AIC32X4_REG(1, 123);

/* Bits, masks, and shifts */

/* AIC32X4_CLKMUX */
pub const AIC32X4_PLL_CLKIN_MASK: u32 = GENMASK(3, 2);
pub const AIC32X4_PLL_CLKIN_SHIFT: u32 = 2;
pub const AIC32X4_PLL_CLKIN_MCLK: u32 = 0x00;
pub const AIC32X4_PLL_CLKIN_BCKL: u32 = 0x01;
pub const AIC32X4_PLL_CLKIN_GPIO1: u32 = 0x02;
pub const AIC32X4_PLL_CLKIN_DIN: u32 = 0x03;
pub const AIC32X4_CODEC_CLKIN_MASK: u32 = GENMASK(1, 0);
pub const AIC32X4_CODEC_CLKIN_SHIFT: u32 = 0;
pub const AIC32X4_CODEC_CLKIN_MCLK: u32 = 0x00;
pub const AIC32X4_CODEC_CLKIN_BCLK: u32 = 0x01;
pub const AIC32X4_CODEC_CLKIN_GPIO1: u32 = 0x02;
pub const AIC32X4_CODEC_CLKIN_PLL: u32 = 0x03;

/* AIC32X4_PLLPR */
pub const AIC32X4_PLLEN: u32 = BIT(7);
pub const AIC32X4_PLL_P_MASK: u32 = GENMASK(6, 4);
pub const AIC32X4_PLL_P_SHIFT: u32 = 4;
pub const AIC32X4_PLL_R_MASK: u32 = GENMASK(3, 0);

/* AIC32X4_NDAC */
pub const AIC32X4_NDACEN: u32 = BIT(7);
pub const AIC32X4_NDAC_MASK: u32 = GENMASK(6, 0);

/* AIC32X4_MDAC */
pub const AIC32X4_MDACEN: u32 = BIT(7);
pub const AIC32X4_MDAC_MASK: u32 = GENMASK(6, 0);

/* AIC32X4_NADC */
pub const AIC32X4_NADCEN: u32 = BIT(7);
pub const AIC32X4_NADC_MASK: u32 = GENMASK(6, 0);

/* AIC32X4_MADC */
pub const AIC32X4_MADCEN: u32 = BIT(7);
pub const AIC32X4_MADC_MASK: u32 = GENMASK(6, 0);

/* AIC32X4_BCLKN */
pub const AIC32X4_BCLKEN: u32 = BIT(7);
pub const AIC32X4_BCLK_MASK: u32 = GENMASK(6, 0);

/* AIC32X4_IFACE1 */
pub const AIC32X4_IFACE1_DATATYPE_MASK: u32 = GENMASK(7, 6);
pub const AIC32X4_IFACE1_DATATYPE_SHIFT: u32 = 6;
pub const AIC32X4_I2S_MODE: u32 = 0x00;
pub const AIC32X4_DSP_MODE: u32 = 0x01;
pub const AIC32X4_RIGHT_JUSTIFIED_MODE: u32 = 0x02;
pub const AIC32X4_LEFT_JUSTIFIED_MODE: u32 = 0x03;
pub const AIC32X4_IFACE1_DATALEN_MASK: u32 = GENMASK(5, 4);
pub const AIC32X4_IFACE1_DATALEN_SHIFT: u32 = 4;
pub const AIC32X4_WORD_LEN_16BITS: u32 = 0x00;
pub const AIC32X4_WORD_LEN_20BITS: u32 = 0x01;
pub const AIC32X4_WORD_LEN_24BITS: u32 = 0x02;
pub const AIC32X4_WORD_LEN_32BITS: u32 = 0x03;
pub const AIC32X4_IFACE1_MASTER_MASK: u32 = GENMASK(3, 2);
pub const AIC32X4_BCLKMASTER: u32 = BIT(2);
pub const AIC32X4_WCLKMASTER: u32 = BIT(3);

/* AIC32X4_IFACE2 */
pub const AIC32X4_DATA_OFFSET_MASK: u32 = GENMASK(7, 0);

/* AIC32X4_IFACE3 */
pub const AIC32X4_BCLKINV_MASK: u32 = BIT(3);
pub const AIC32X4_BDIVCLK_MASK: u32 = GENMASK(1, 0);
pub const AIC32X4_BDIVCLK_SHIFT: u32 = 0;
pub const AIC32X4_DAC2BCLK: u32 = 0x00;
pub const AIC32X4_DACMOD2BCLK: u32 = 0x01;
pub const AIC32X4_ADC2BCLK: u32 = 0x02;
pub const AIC32X4_ADCMOD2BCLK: u32 = 0x03;

/* AIC32X4_DACSETUP */
pub const AIC32X4_DAC_CHAN_MASK: u32 = GENMASK(5, 2);
pub const AIC32X4_LDAC2RCHN: u32 = BIT(5);
pub const AIC32X4_LDAC2LCHN: u32 = BIT(4);
pub const AIC32X4_RDAC2LCHN: u32 = BIT(3);
pub const AIC32X4_RDAC2RCHN: u32 = BIT(2);

/* AIC32X4_DACMUTE */
pub const AIC32X4_MUTEON: u32 = 0x0C;

/* AIC32X4_ADCSETUP */
pub const AIC32X4_LADC_EN: u32 = BIT(7);
pub const AIC32X4_RADC_EN: u32 = BIT(6);

/* AIC32X4_PWRCFG */
pub const AIC32X4_AVDDWEAKDISABLE: u32 = BIT(3);

/* AIC32X4_LDOCTL */
pub const AIC32X4_LDOCTLEN: u32 = BIT(0);

/* AIC32X4_CMMODE */
pub const AIC32X4_LDOIN_18_36: u32 = BIT(0);
pub const AIC32X4_LDOIN2HP: u32 = BIT(1);

/* AIC32X4_MICBIAS */
pub const AIC32X4_MICBIAS_LDOIN: u32 = BIT(3);
pub const AIC32X4_MICBIAS_2075V: u32 = 0x60;
pub const AIC32x4_MICBIAS_MASK: u32 = GENMASK(6, 3);

/* AIC32X4_LMICPGANIN */
pub const AIC32X4_LMICPGANIN_IN2R_10K: u32 = 0x10;
pub const AIC32X4_LMICPGANIN_CM1L_10K: u32 = 0x40;

/* AIC32X4_RMICPGANIN */
pub const AIC32X4_RMICPGANIN_IN1L_10K: u32 = 0x10;
pub const AIC32X4_RMICPGANIN_CM1R_10K: u32 = 0x40;

/* AIC32X4_REFPOWERUP */
pub const AIC32X4_REFPOWERUP_SLOW: u32 = 0x04;
pub const AIC32X4_REFPOWERUP_40MS: u32 = 0x05;
pub const AIC32X4_REFPOWERUP_80MS: u32 = 0x06;
pub const AIC32X4_REFPOWERUP_120MS: u32 = 0x07;

/* Common mask and enable for all of the dividers */
pub const AIC32X4_DIVEN: u32 = BIT(7);
pub const AIC32X4_DIV_MASK: u32 = GENMASK(6, 0);
pub const AIC32X4_DIV_MAX: u32 = 128;

/* Clock Limits */
pub const AIC32X4_MAX_DOSR_FREQ: u32 = 6200000;
pub const AIC32X4_MIN_DOSR_FREQ: u32 = 2800000;
pub const AIC32X4_MAX_CODEC_CLKIN_FREQ: u32 = 110000000;
pub const AIC32X4_MAX_PLL_CLKIN: u32 = 20000000;

pub const AIC32X4_PWR_MICBIAS_2075_LDOIN: u32 = 0x00000001;
pub const AIC32X4_PWR_AVDD_DVDD_WEAK_DISABLE: u32 = 0x00000002;
pub const AIC32X4_PWR_AIC32X4_LDO_ENABLE: u32 = 0x00000004;
pub const AIC32X4_PWR_CMMODE_LDOIN_RANGE_18_36: u32 = 0x00000008;
pub const AIC32X4_PWR_CMMODE_HP_LDOIN_POWERED: u32 = 0x00000010;

pub const AIC32X4_MICPGA_ROUTE_LMIC_IN2R_10K: u32 = 0x00000001;
pub const AIC32X4_MICPGA_ROUTE_RMIC_IN1L_10K: u32 = 0x00000002;

/* GPIO API */
pub const AIC32X4_MFPX_DEFAULT_VALUE: u32 = 0xff;

pub const AIC32X4_MFP1_DIN_DISABLED: u32 = 0;
pub const AIC32X4_MFP1_DIN_ENABLED: u32 = 0x2;
pub const AIC32X4_MFP1_GPIO_IN: u32 = 0x4;

pub const AIC32X4_MFP2_GPIO_OUT_LOW: u32 = 0x0;
pub const AIC32X4_MFP2_GPIO_OUT_HIGH: u32 = 0x1;

pub const AIC32X4_MFP_GPIO_ENABLED: u32 = 0x4;

pub const AIC32X4_MFP5_GPIO_DISABLED: u32 = 0x0;
pub const AIC32X4_MFP5_GPIO_INPUT: u32 = 0x8;
pub const AIC32X4_MFP5_GPIO_OUTPUT: u32 = 0xc;
pub const AIC32X4_MFP5_GPIO_OUT_LOW: u32 = 0x0;
pub const AIC32X4_MFP5_GPIO_OUT_HIGH: u32 = 0x1;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
