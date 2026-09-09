/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright (C) 2019-2020 ARM Limited or its affiliates. */

// Dependency intent from <linux/bitops.h>: BIT() and BITS_PER_TYPE().

pub const POWER_DOWN_ENABLE: u32 = 0x01;
pub const POWER_DOWN_DISABLE: u32 = 0x00;

/* hwrng quality: bits of true entropy per 1024 bits of input */
pub const CC_TRNG_QUALITY: u32 = 1024;

/* CryptoCell TRNG HW definitions */
pub const CC_TRNG_NUM_OF_ROSCS: u32 = 4;
/* The number of words generated in the entropy holding register (EHR)
 * 6 words (192 bit) according to HW implementation
 */
pub const CC_TRNG_EHR_IN_WORDS: u32 = 6;
pub const CC_TRNG_EHR_IN_BITS: u32 = CC_TRNG_EHR_IN_WORDS * 32;

pub const CC_HOST_RNG_IRQ_MASK: u64 = 1u64 << CC_HOST_RGF_IRR_RNG_INT_BIT_SHIFT;

/* RNG interrupt mask */
pub const CC_RNG_INT_MASK: u64 = (1u64 << CC_RNG_IMR_EHR_VALID_INT_MASK_BIT_SHIFT)
    | (1u64 << CC_RNG_IMR_AUTOCORR_ERR_INT_MASK_BIT_SHIFT)
    | (1u64 << CC_RNG_IMR_CRNGT_ERR_INT_MASK_BIT_SHIFT)
    | (1u64 << CC_RNG_IMR_VN_ERR_INT_MASK_BIT_SHIFT)
    | (1u64 << CC_RNG_IMR_WATCHDOG_INT_MASK_BIT_SHIFT);

// --------------------------------------
// BLOCK: RNG
// --------------------------------------
pub const CC_RNG_IMR_REG_OFFSET: u64 = 0x0100;
pub const CC_RNG_IMR_EHR_VALID_INT_MASK_BIT_SHIFT: u64 = 0x0;
pub const CC_RNG_IMR_AUTOCORR_ERR_INT_MASK_BIT_SHIFT: u64 = 0x1;
pub const CC_RNG_IMR_CRNGT_ERR_INT_MASK_BIT_SHIFT: u64 = 0x2;
pub const CC_RNG_IMR_VN_ERR_INT_MASK_BIT_SHIFT: u64 = 0x3;
pub const CC_RNG_IMR_WATCHDOG_INT_MASK_BIT_SHIFT: u64 = 0x4;
pub const CC_RNG_ISR_REG_OFFSET: u64 = 0x0104;
pub const CC_RNG_ISR_EHR_VALID_BIT_SHIFT: u64 = 0x0;
pub const CC_RNG_ISR_EHR_VALID_BIT_SIZE: u64 = 0x1;
pub const CC_RNG_ISR_AUTOCORR_ERR_BIT_SHIFT: u64 = 0x1;
pub const CC_RNG_ISR_AUTOCORR_ERR_BIT_SIZE: u64 = 0x1;
pub const CC_RNG_ISR_CRNGT_ERR_BIT_SHIFT: u64 = 0x2;
pub const CC_RNG_ISR_CRNGT_ERR_BIT_SIZE: u64 = 0x1;
pub const CC_RNG_ISR_WATCHDOG_BIT_SHIFT: u64 = 0x4;
pub const CC_RNG_ISR_WATCHDOG_BIT_SIZE: u64 = 0x1;
pub const CC_RNG_ICR_REG_OFFSET: u64 = 0x0108;
pub const CC_TRNG_CONFIG_REG_OFFSET: u64 = 0x010C;
pub const CC_EHR_DATA_0_REG_OFFSET: u64 = 0x0114;
pub const CC_RND_SOURCE_ENABLE_REG_OFFSET: u64 = 0x012C;
pub const CC_SAMPLE_CNT1_REG_OFFSET: u64 = 0x0130;
pub const CC_TRNG_DEBUG_CONTROL_REG_OFFSET: u64 = 0x0138;
pub const CC_RNG_SW_RESET_REG_OFFSET: u64 = 0x0140;
pub const CC_RNG_CLK_ENABLE_REG_OFFSET: u64 = 0x01C4;
pub const CC_RNG_DMA_ENABLE_REG_OFFSET: u64 = 0x01C8;
pub const CC_RNG_WATCHDOG_VAL_REG_OFFSET: u64 = 0x01D8;
// --------------------------------------
// BLOCK: SEC_HOST_RGF
// --------------------------------------
pub const CC_HOST_RGF_IRR_REG_OFFSET: u64 = 0x0A00;
pub const CC_HOST_RGF_IRR_RNG_INT_BIT_SHIFT: u64 = 0xA;
pub const CC_HOST_RGF_IMR_REG_OFFSET: u64 = 0x0A04;
pub const CC_HOST_RGF_ICR_REG_OFFSET: u64 = 0x0A08;

pub const CC_HOST_POWER_DOWN_EN_REG_OFFSET: u64 = 0x0A78;

// --------------------------------------
// BLOCK: NVM
// --------------------------------------
pub const CC_NVM_IS_IDLE_REG_OFFSET: u64 = 0x0F10;
pub const CC_NVM_IS_IDLE_VALUE_BIT_SHIFT: u64 = 0x0;
pub const CC_NVM_IS_IDLE_VALUE_BIT_SIZE: u64 = 0x1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
