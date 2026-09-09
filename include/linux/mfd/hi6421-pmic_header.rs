/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Header file for device driver Hi6421 PMIC
 *
 * Copyright (c) <2011-2014> HiSilicon Technologies Co., Ltd.
 *              http://www.hisilicon.com
 * Copyright (c) <2013-2014> Linaro Ltd.
 *              https://www.linaro.org
 *
 * Author: Guodong Xu <guodong.xu@linaro.org>
 */

/* Hi6421 registers are mapped to memory bus in 4 bytes stride */
macro_rules! HI6421_REG_TO_BUS_ADDR {
    ($x:expr) => {
        ($x << 2)
    };
}

/* Hi6421 maximum register number */
pub const HI6421_REG_MAX: u32 = 0xFF;

/* Hi6421 OCP (over current protection) and DEB (debounce) control register */
pub const HI6421_OCP_DEB_CTRL_REG: u32 = HI6421_REG_TO_BUS_ADDR!(0x51u32);
pub const HI6421_OCP_DEB_SEL_MASK: u32 = 0x0C;
pub const HI6421_OCP_DEB_SEL_8MS: u32 = 0x00;
pub const HI6421_OCP_DEB_SEL_16MS: u32 = 0x04;
pub const HI6421_OCP_DEB_SEL_32MS: u32 = 0x08;
pub const HI6421_OCP_DEB_SEL_64MS: u32 = 0x0C;
pub const HI6421_OCP_EN_DEBOUNCE_MASK: u32 = 0x02;
pub const HI6421_OCP_EN_DEBOUNCE_ENABLE: u32 = 0x02;
pub const HI6421_OCP_AUTO_STOP_MASK: u32 = 0x01;
pub const HI6421_OCP_AUTO_STOP_ENABLE: u32 = 0x01;

#[repr(C)]
pub struct hi6421_pmic {
    pub regmap: *mut regmap,
}

/* External dependency supplied by the regmap subsystem. */
pub enum regmap {}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum hi6421_type {
    HI6421 = 0,
    HI6421_V530,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
