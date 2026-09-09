// SPDX-License-Identifier: GPL-2.0
//
// Copyright (c) 2010 Samsung Electronics Co., Ltd.
//		http://www.samsung.com/
//
// GPIO configuration for S3C64XX KeyPad device

// Dependencies supplied by the surrounding platform code:
// linux/gpio/consumer.h, gpio-cfg.h, keypad.h, and gpio-samsung.h.

extern "C" {
    fn s3c_gpio_cfgrange_nopull(
        gpio: ::core::ffi::c_uint,
        nr: ::core::ffi::c_uint,
        config: ::core::ffi::c_uint,
    );

    // C macros S3C64XX_GPK() and S3C64XX_GPL() are represented by the
    // corresponding platform-provided Rust functions.
    fn s3c64xx_gpk(pin: ::core::ffi::c_uint) -> ::core::ffi::c_uint;
    fn s3c64xx_gpl(pin: ::core::ffi::c_uint) -> ::core::ffi::c_uint;
    fn s3c_gpio_sfn(function: ::core::ffi::c_uint) -> ::core::ffi::c_uint;
}

pub unsafe fn samsung_keypad_cfg_gpio(rows: ::core::ffi::c_uint, cols: ::core::ffi::c_uint) {
    /* Set all the necessary GPK pins to special-function 3: KP_ROW[x] */
    s3c_gpio_cfgrange_nopull(s3c64xx_gpk(8), rows, s3c_gpio_sfn(3));

    /* Set all the necessary GPL pins to special-function 3: KP_COL[x] */
    s3c_gpio_cfgrange_nopull(s3c64xx_gpl(0), cols, s3c_gpio_sfn(3));
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
