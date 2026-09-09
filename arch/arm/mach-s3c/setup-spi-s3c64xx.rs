// SPDX-License-Identifier: GPL-2.0
//
// Copyright (C) 2011 Samsung Electronics Ltd.
//		http://www.samsung.com/

// Dependencies supplied by the Linux GPIO and SPI platform-data headers:
// linux/gpio/consumer.h, linux/platform_data/spi-s3c64xx.h, gpio-cfg.h,
// and gpio-samsung.h.

#[cfg(CONFIG_S3C64XX_DEV_SPI0)]
extern "C" {
    fn s3c_gpio_cfgall_range(start: usize, nr: usize, cfg: u32, pull: u32);
}

#[cfg(CONFIG_S3C64XX_DEV_SPI0)]
#[inline]
pub unsafe fn s3c64xx_spi0_cfg_gpio() -> i32 {
    s3c_gpio_cfgall_range(
        S3C64XX_GPC(0),
        3,
        S3C_GPIO_SFN(2),
        S3C_GPIO_PULL_UP,
    );
    0
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
