/* SPDX-License-Identifier: GPL-2.0-only */

// Dependencies supplied by the corresponding Linux I2C and regmap interfaces.

pub const PM886_A1_CHIP_ID: u32 = 0xa1;

pub const PM886_IRQ_ONKEY: u32 = 0;

pub const PM886_PAGE_OFFSET_REGULATORS: u32 = 1;
pub const PM886_PAGE_OFFSET_GPADC: u32 = 2;
pub const PM886_PAGE_OFFSET_BATTERY: u32 = 3;

pub const PM886_REG_ID: u32 = 0x00;

pub const PM886_REG_STATUS1: u32 = 0x01;
pub const PM886_ONKEY_STS1: u32 = 1 << 0;

pub const PM886_REG_INT_STATUS1: u32 = 0x05;

pub const PM886_REG_INT_ENA_1: u32 = 0x0a;
pub const PM886_INT_ENA1_ONKEY: u32 = 1 << 0;

pub const PM886_REG_MISC_CONFIG1: u32 = 0x14;
pub const PM886_SW_PDOWN: u32 = 1 << 5;

pub const PM886_REG_MISC_CONFIG2: u32 = 0x15;
pub const PM886_INT_INV: u32 = 1 << 0;
pub const PM886_INT_CLEAR: u32 = 1 << 1;
pub const PM886_INT_RC: u32 = 0x00;
pub const PM886_INT_WC: u32 = 1 << 1;
pub const PM886_INT_MASK_MODE: u32 = 1 << 2;

pub const PM886_REG_RTC_CNT1: u32 = 0xd1;
pub const PM886_REG_RTC_CNT2: u32 = 0xd2;
pub const PM886_REG_RTC_CNT3: u32 = 0xd3;
pub const PM886_REG_RTC_CNT4: u32 = 0xd4;
pub const PM886_REG_RTC_SPARE1: u32 = 0xea;
pub const PM886_REG_RTC_SPARE2: u32 = 0xeb;
pub const PM886_REG_RTC_SPARE3: u32 = 0xec;
pub const PM886_REG_RTC_SPARE4: u32 = 0xed;
pub const PM886_REG_RTC_SPARE5: u32 = 0xee;
pub const PM886_REG_RTC_SPARE6: u32 = 0xef;

pub const PM886_REG_BUCK_EN: u32 = 0x08;
pub const PM886_REG_LDO_EN1: u32 = 0x09;
pub const PM886_REG_LDO_EN2: u32 = 0x0a;
pub const PM886_REG_LDO1_VOUT: u32 = 0x20;
pub const PM886_REG_LDO2_VOUT: u32 = 0x26;
pub const PM886_REG_LDO3_VOUT: u32 = 0x2c;
pub const PM886_REG_LDO4_VOUT: u32 = 0x32;
pub const PM886_REG_LDO5_VOUT: u32 = 0x38;
pub const PM886_REG_LDO6_VOUT: u32 = 0x3e;
pub const PM886_REG_LDO7_VOUT: u32 = 0x44;
pub const PM886_REG_LDO8_VOUT: u32 = 0x4a;
pub const PM886_REG_LDO9_VOUT: u32 = 0x50;
pub const PM886_REG_LDO10_VOUT: u32 = 0x56;
pub const PM886_REG_LDO11_VOUT: u32 = 0x5c;
pub const PM886_REG_LDO12_VOUT: u32 = 0x62;
pub const PM886_REG_LDO13_VOUT: u32 = 0x68;
pub const PM886_REG_LDO14_VOUT: u32 = 0x6e;
pub const PM886_REG_LDO15_VOUT: u32 = 0x74;
pub const PM886_REG_LDO16_VOUT: u32 = 0x7a;
pub const PM886_REG_BUCK1_VOUT: u32 = 0xa5;
pub const PM886_REG_BUCK2_VOUT: u32 = 0xb3;
pub const PM886_REG_BUCK3_VOUT: u32 = 0xc1;
pub const PM886_REG_BUCK4_VOUT: u32 = 0xcf;
pub const PM886_REG_BUCK5_VOUT: u32 = 0xdd;

pub const PM886_LDO_VSEL_MASK: u32 = 0x0f;
pub const PM886_BUCK_VSEL_MASK: u32 = 0x7f;

/* GPADC enable/disable registers */
#[inline]
pub const fn PM886_REG_GPADC_CONFIG(n: u32) -> u32 { n }

pub const PM886_GPADC_VSC_EN: u32 = 1 << 0;
pub const PM886_GPADC_VBAT_EN: u32 = 1 << 1;
pub const PM886_GPADC_GNDDET1_EN: u32 = 1 << 3;
pub const PM886_GPADC_VBUS_EN: u32 = 1 << 4;
pub const PM886_GPADC_VCHG_PWR_EN: u32 = 1 << 5;
pub const PM886_GPADC_VCF_OUT_EN: u32 = 1 << 6;
pub const PM886_GPADC_CONFIG1_EN_ALL: u32 = PM886_GPADC_VSC_EN
    | PM886_GPADC_VBAT_EN
    | PM886_GPADC_GNDDET1_EN
    | PM886_GPADC_VBUS_EN
    | PM886_GPADC_VCHG_PWR_EN
    | PM886_GPADC_VCF_OUT_EN;

pub const PM886_GPADC_TINT_EN: u32 = 1 << 0;
pub const PM886_GPADC_PMODE_EN: u32 = 1 << 1;
pub const PM886_GPADC_GPADC0_EN: u32 = 1 << 2;
pub const PM886_GPADC_GPADC1_EN: u32 = 1 << 3;
pub const PM886_GPADC_GPADC2_EN: u32 = 1 << 4;
pub const PM886_GPADC_GPADC3_EN: u32 = 1 << 5;
pub const PM886_GPADC_MIC_DET_EN: u32 = 1 << 6;
pub const PM886_GPADC_CONFIG2_EN_ALL: u32 = PM886_GPADC_TINT_EN
    | PM886_GPADC_GPADC0_EN
    | PM886_GPADC_GPADC1_EN
    | PM886_GPADC_GPADC2_EN
    | PM886_GPADC_GPADC3_EN
    | PM886_GPADC_MIC_DET_EN;

/* No CONFIG3_EN_ALL because this is the only bit there. */
pub const PM886_GPADC_GND_DET2_EN: u32 = 1 << 0;

/* GPADC channel registers */
pub const PM886_REG_GPADC_VSC: u32 = 0x40;
pub const PM886_REG_GPADC_VCHG_PWR: u32 = 0x4c;
pub const PM886_REG_GPADC_VCF_OUT: u32 = 0x4e;
pub const PM886_REG_GPADC_TINT: u32 = 0x50;
pub const PM886_REG_GPADC_GPADC0: u32 = 0x54;
pub const PM886_REG_GPADC_GPADC1: u32 = 0x56;
pub const PM886_REG_GPADC_GPADC2: u32 = 0x58;
pub const PM886_REG_GPADC_VBAT: u32 = 0xa0;
pub const PM886_REG_GPADC_GND_DET1: u32 = 0xa4;
pub const PM886_REG_GPADC_GND_DET2: u32 = 0xa6;
pub const PM886_REG_GPADC_VBUS: u32 = 0xa8;
pub const PM886_REG_GPADC_GPADC3: u32 = 0xaa;
pub const PM886_REG_GPADC_MIC_DET: u32 = 0xac;
pub const PM886_REG_GPADC_VBAT_SLP: u32 = 0xb0;

/* VBAT_SLP is the last register and is 2 bytes wide like other channels. */
pub const PM886_GPADC_MAX_REGISTER: u32 = PM886_REG_GPADC_VBAT_SLP + 1;

pub const PM886_GPADC_BIAS_LEVELS: u32 = 16;
#[inline]
pub const fn PM886_GPADC_INDEX_TO_BIAS_uA(i: u32) -> u32 { 1 + i * 5 }

/* Battery block register definitions */
pub const PM886_REG_CLS_CONFIG1: u32 = 0x71;

#[repr(C)]
pub struct pm886_chip {
    pub client: *mut i2c_client,
    pub chip_id: u32,
    pub regmap: *mut regmap,
    pub regmap_battery: *mut regmap,
}

// External types supplied by the included Linux headers.
pub enum i2c_client {}
pub enum regmap {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
