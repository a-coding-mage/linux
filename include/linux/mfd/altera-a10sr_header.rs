/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright Intel Corporation (C) 2014-2016. All Rights Reserved
 *
 * Declarations for Altera Arria10 MAX5 System Resource Chip
 *
 * Adapted from DA9052
 */

/* Dependencies supplied by other translated units. */

/* Write registers are always on even addresses */
pub const WRITE_REG_MASK: u32 = 0xFE;
/* Odd registers are always on odd addresses */
pub const READ_REG_MASK: u32 = 0x01;

pub const ALTR_A10SR_BITS_PER_REGISTER: u32 = 8;
/*
 * To find the correct register, we divide the input GPIO by
 * the number of GPIO in each register. We then need to multiply
 * by 2 because the reads are at odd addresses.
 */
#[inline]
pub const fn altr_a10sr_reg_offset(x: u32) -> u32 {
    ((x / ALTR_A10SR_BITS_PER_REGISTER) << 1)
}

#[inline]
pub const fn altr_a10sr_reg_bit(x: u32) -> u32 {
    x % ALTR_A10SR_BITS_PER_REGISTER
}

#[inline]
pub const fn altr_a10sr_reg_bit_chg(x: u32, y: u32) -> u32 {
    x << altr_a10sr_reg_bit(y)
}

#[inline]
pub const fn altr_a10sr_reg_bit_mask(x: u32) -> u32 {
    1 << altr_a10sr_reg_bit(x)
}

/* Arria10 System Controller Register Defines */
pub const ALTR_A10SR_NOP: u32 = 0x00; /* No Change */
pub const ALTR_A10SR_VERSION_READ: u32 = 0x00; /* MAX5 Version Read */

pub const ALTR_A10SR_LED_REG: u32 = 0x02; /* LED - Upper 4 bits */
/* LED register Bit Definitions */
pub const ALTR_A10SR_LED_VALID_SHIFT: u32 = 4; /* LED - Upper 4 bits valid */
pub const ALTR_A10SR_OUT_VALID_RANGE_LO: u32 = ALTR_A10SR_LED_VALID_SHIFT;
pub const ALTR_A10SR_OUT_VALID_RANGE_HI: u32 = 7;

pub const ALTR_A10SR_PBDSW_REG: u32 = 0x04; /* PB & DIP SW - Input only */
pub const ALTR_A10SR_PBDSW_IRQ_REG: u32 = 0x06; /* PB & DIP SW Flag Clear */
/* Pushbutton & DIP Switch Bit Definitions */
pub const ALTR_A10SR_IN_VALID_RANGE_LO: u32 = 8;
pub const ALTR_A10SR_IN_VALID_RANGE_HI: u32 = 15;

pub const ALTR_A10SR_PWR_GOOD1_REG: u32 = 0x08; /* Power Good1 Read */
pub const ALTR_A10SR_PWR_GOOD2_REG: u32 = 0x0A; /* Power Good2 Read */
pub const ALTR_A10SR_PWR_GOOD3_REG: u32 = 0x0C; /* Power Good3 Read */
pub const ALTR_A10SR_FMCAB_REG: u32 = 0x0E; /* FMCA/B & PCIe Pwr Enable */
pub const ALTR_A10SR_HPS_RST_REG: u32 = 0x10; /* HPS Reset */
pub const ALTR_A10SR_USB_QSPI_REG: u32 = 0x12; /* USB, BQSPI, FILE Reset */
pub const ALTR_A10SR_SFPA_REG: u32 = 0x14; /* SFPA Control Reg */
pub const ALTR_A10SR_SFPB_REG: u32 = 0x16; /* SFPB Control Reg */
pub const ALTR_A10SR_I2C_M_REG: u32 = 0x18; /* I2C Master Select */
pub const ALTR_A10SR_WARM_RST_REG: u32 = 0x1A; /* HPS Warm Reset */
pub const ALTR_A10SR_WR_KEY_REG: u32 = 0x1C; /* HPS Warm Reset Key */
pub const ALTR_A10SR_PMBUS_REG: u32 = 0x1E; /* HPS PM Bus */

/**
 * struct altr_a10sr - Altera Max5 MFD device private data structure
 * @dev:  : this device
 * @regmap: the regmap assigned to the parent device.
 */
#[repr(C)]
pub struct altr_a10sr {
    pub dev: *mut device,
    pub regmap: *mut regmap,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
