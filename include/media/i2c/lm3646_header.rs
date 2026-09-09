/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * include/media/i2c/lm3646.h
 *
 * Copyright (C) 2014 Texas Instruments
 *
 * Contact: Daniel Jeong <gshark.jeong@gmail.com>
 *          Ldd-Mlp <ldd-mlp@list.ti.com>
 */

// Dependency: <media/v4l2-subdev.h>

pub const LM3646_NAME: &str = "lm3646";
pub const LM3646_I2C_ADDR_REV1: u32 = 0x67;
pub const LM3646_I2C_ADDR_REV0: u32 = 0x63;

/*  TOTAL FLASH Brightness Max
 *     min 93350uA, step 93750uA, max 1499600uA
 */
pub const LM3646_TOTAL_FLASH_BRT_MIN: u32 = 93350;
pub const LM3646_TOTAL_FLASH_BRT_STEP: u32 = 93750;
pub const LM3646_TOTAL_FLASH_BRT_MAX: u32 = 1499600;
pub const fn LM3646_TOTAL_FLASH_BRT_uA_TO_REG(a: u32) -> u32 {
    if a < LM3646_TOTAL_FLASH_BRT_MIN {
        0
    } else {
        (a - LM3646_TOTAL_FLASH_BRT_MIN) / LM3646_TOTAL_FLASH_BRT_STEP
    }
}

/*  TOTAL TORCH Brightness Max
 *     min 23040uA, step 23430uA, max 187100uA
 */
pub const LM3646_TOTAL_TORCH_BRT_MIN: u32 = 23040;
pub const LM3646_TOTAL_TORCH_BRT_STEP: u32 = 23430;
pub const LM3646_TOTAL_TORCH_BRT_MAX: u32 = 187100;
pub const fn LM3646_TOTAL_TORCH_BRT_uA_TO_REG(a: u32) -> u32 {
    if a < LM3646_TOTAL_TORCH_BRT_MIN {
        0
    } else {
        (a - LM3646_TOTAL_TORCH_BRT_MIN) / LM3646_TOTAL_TORCH_BRT_STEP
    }
}

/*  LED1 FLASH Brightness
 *     min 23040uA, step 11718uA, max 1499600uA
 */
pub const LM3646_LED1_FLASH_BRT_MIN: u32 = 23040;
pub const LM3646_LED1_FLASH_BRT_STEP: u32 = 11718;
pub const LM3646_LED1_FLASH_BRT_MAX: u32 = 1499600;
pub const fn LM3646_LED1_FLASH_BRT_uA_TO_REG(a: u32) -> u32 {
    if a <= LM3646_LED1_FLASH_BRT_MIN {
        0
    } else {
        ((a - LM3646_LED1_FLASH_BRT_MIN) / LM3646_LED1_FLASH_BRT_STEP) + 1
    }
}

/*  LED1 TORCH Brightness
 *     min 2530uA, step 1460uA, max 187100uA
 */
pub const LM3646_LED1_TORCH_BRT_MIN: u32 = 2530;
pub const LM3646_LED1_TORCH_BRT_STEP: u32 = 1460;
pub const LM3646_LED1_TORCH_BRT_MAX: u32 = 187100;
pub const fn LM3646_LED1_TORCH_BRT_uA_TO_REG(a: u32) -> u32 {
    if a <= LM3646_LED1_TORCH_BRT_MIN {
        0
    } else {
        ((a - LM3646_LED1_TORCH_BRT_MIN) / LM3646_LED1_TORCH_BRT_STEP) + 1
    }
}

/*  FLASH TIMEOUT DURATION
 *     min 50ms, step 50ms, max 400ms
 */
pub const LM3646_FLASH_TOUT_MIN: u32 = 50;
pub const LM3646_FLASH_TOUT_STEP: u32 = 50;
pub const LM3646_FLASH_TOUT_MAX: u32 = 400;
pub const fn LM3646_FLASH_TOUT_ms_TO_REG(a: u32) -> u32 {
    if a <= LM3646_FLASH_TOUT_MIN {
        0
    } else {
        (a - LM3646_FLASH_TOUT_MIN) / LM3646_FLASH_TOUT_STEP
    }
}

/* struct lm3646_platform_data
 *
 * @flash_timeout: flash timeout
 * @led1_flash_brt: led1 flash mode brightness, uA
 * @led1_torch_brt: led1 torch mode brightness, uA
 */
#[repr(C)]
pub struct lm3646_platform_data {
    pub flash_timeout: u32,
    pub led1_flash_brt: u32,
    pub led1_torch_brt: u32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
