/* SPDX-License-Identifier: GPL-2.0-or-later */

/* Translated from the C header; Linux bitfield macros are expressed locally. */
const fn bit(n: u32) -> u32 {
    1u32 << n
}

const fn genmask(high: u32, low: u32) -> u32 {
    ((u32::MAX >> (31 - high)) & (u32::MAX << low))
}

/* REGISTERS */
pub const MAX77541_REG_INT_SRC: u32 = 0x00;
pub const MAX77541_REG_INT_SRC_M: u32 = 0x01;

pub const MAX77541_BIT_INT_SRC_TOPSYS: u32 = bit(0);
pub const MAX77541_BIT_INT_SRC_BUCK: u32 = bit(1);

pub const MAX77541_REG_TOPSYS_INT: u32 = 0x02;
pub const MAX77541_REG_TOPSYS_INT_M: u32 = 0x03;

pub const MAX77541_BIT_TOPSYS_INT_TJ_120C: u32 = bit(0);
pub const MAX77541_BIT_TOPSYS_INT_TJ_140C: u32 = bit(1);
pub const MAX77541_BIT_TOPSYS_INT_TSHDN: u32 = bit(2);
pub const MAX77541_BIT_TOPSYS_INT_UVLO: u32 = bit(3);
pub const MAX77541_BIT_TOPSYS_INT_ALT_SWO: u32 = bit(4);
pub const MAX77541_BIT_TOPSYS_INT_EXT_FREQ_DET: u32 = bit(5);

/* REGULATORS */
pub const MAX77541_REG_BUCK_INT: u32 = 0x20;
pub const MAX77541_REG_BUCK_INT_M: u32 = 0x21;

pub const MAX77541_BIT_BUCK_INT_M1_POK_FLT: u32 = bit(0);
pub const MAX77541_BIT_BUCK_INT_M2_POK_FLT: u32 = bit(1);
pub const MAX77541_BIT_BUCK_INT_M1_SCFLT: u32 = bit(4);
pub const MAX77541_BIT_BUCK_INT_M2_SCFLT: u32 = bit(5);

pub const MAX77541_REG_EN_CTRL: u32 = 0x0B;

pub const MAX77541_BIT_M1_EN: u32 = bit(0);
pub const MAX77541_BIT_M2_EN: u32 = bit(1);

pub const MAX77541_REG_M1_VOUT: u32 = 0x23;
pub const MAX77541_REG_M2_VOUT: u32 = 0x33;

pub const MAX77541_BITS_MX_VOUT: u32 = genmask(7, 0);

pub const MAX77541_REG_M1_CFG1: u32 = 0x25;
pub const MAX77541_REG_M2_CFG1: u32 = 0x35;

pub const MAX77541_BITS_MX_CFG1_RNG: u32 = genmask(7, 6);

/* ADC */
pub const MAX77541_REG_ADC_INT: u32 = 0x70;
pub const MAX77541_REG_ADC_INT_M: u32 = 0x71;

pub const MAX77541_BIT_ADC_INT_CH1_I: u32 = bit(0);
pub const MAX77541_BIT_ADC_INT_CH2_I: u32 = bit(1);
pub const MAX77541_BIT_ADC_INT_CH3_I: u32 = bit(2);
pub const MAX77541_BIT_ADC_INT_CH6_I: u32 = bit(5);

pub const MAX77541_REG_ADC_DATA_CH1: u32 = 0x72;
pub const MAX77541_REG_ADC_DATA_CH2: u32 = 0x73;
pub const MAX77541_REG_ADC_DATA_CH3: u32 = 0x74;
pub const MAX77541_REG_ADC_DATA_CH6: u32 = 0x77;

/* INTERRUPT MASKS */
pub const MAX77541_REG_INT_SRC_MASK: u32 = 0x00;
pub const MAX77541_REG_TOPSYS_INT_MASK: u32 = 0x00;
pub const MAX77541_REG_BUCK_INT_MASK: u32 = 0x00;

pub const MAX77541_MAX_REGULATORS: u32 = 2;

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum max7754x_ids {
    MAX77540 = 1,
    MAX77541,
}

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regmap_irq_chip_data {
    _private: [u8; 0],
}

#[repr(C)]
pub struct i2c_client {
    _private: [u8; 0],
}

#[repr(C)]
pub struct max77541 {
    pub i2c: *mut i2c_client,
    pub regmap: *mut regmap,
    pub id: max7754x_ids,

    pub irq_data: *mut regmap_irq_chip_data,
    pub irq_buck: *mut regmap_irq_chip_data,
    pub irq_topsys: *mut regmap_irq_chip_data,
    pub irq_adc: *mut regmap_irq_chip_data,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
