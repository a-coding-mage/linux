// SPDX-License-Identifier: GPL-2.0-only
/*
 * cs42l83-i2c.c -- CS42L83 ALSA SoC audio driver for I2C
 *
 * Based on cs42l42-i2c.c:
 *   Copyright 2016, 2022 Cirrus Logic, Inc.
 */

// C dependencies:
// #include <linux/i2c.h>
// #include <linux/module.h>
// #include <linux/regmap.h>
// #include <linux/slab.h>
// #include <linux/types.h>
// #include "cs42l42.h"

static cs42l83_reg_defaults: [reg_default; 125] = [
    reg_default { reg: CS42L42_FRZ_CTL, r#def: 0x00 },
    reg_default { reg: CS42L42_SRC_CTL, r#def: 0x10 },
    reg_default { reg: CS42L42_MCLK_CTL, r#def: 0x00 }, /* <- only deviation from CS42L42 */
    reg_default { reg: CS42L42_SFTRAMP_RATE, r#def: 0xA4 },
    reg_default { reg: CS42L42_SLOW_START_ENABLE, r#def: 0x70 },
    reg_default { reg: CS42L42_I2C_DEBOUNCE, r#def: 0x88 },
    reg_default { reg: CS42L42_I2C_STRETCH, r#def: 0x03 },
    reg_default { reg: CS42L42_I2C_TIMEOUT, r#def: 0xB7 },
    reg_default { reg: CS42L42_PWR_CTL1, r#def: 0xFF },
    reg_default { reg: CS42L42_PWR_CTL2, r#def: 0x84 },
    reg_default { reg: CS42L42_PWR_CTL3, r#def: 0x20 },
    reg_default { reg: CS42L42_RSENSE_CTL1, r#def: 0x40 },
    reg_default { reg: CS42L42_RSENSE_CTL2, r#def: 0x00 },
    reg_default { reg: CS42L42_OSC_SWITCH, r#def: 0x00 },
    reg_default { reg: CS42L42_RSENSE_CTL3, r#def: 0x1B },
    reg_default { reg: CS42L42_TSENSE_CTL, r#def: 0x1B },
    reg_default { reg: CS42L42_TSRS_INT_DISABLE, r#def: 0x00 },
    reg_default { reg: CS42L42_HSDET_CTL1, r#def: 0x77 },
    reg_default { reg: CS42L42_HSDET_CTL2, r#def: 0x00 },
    reg_default { reg: CS42L42_HS_SWITCH_CTL, r#def: 0xF3 },
    reg_default { reg: CS42L42_HS_CLAMP_DISABLE, r#def: 0x00 },
    reg_default { reg: CS42L42_MCLK_SRC_SEL, r#def: 0x00 },
    reg_default { reg: CS42L42_SPDIF_CLK_CFG, r#def: 0x00 },
    reg_default { reg: CS42L42_FSYNC_PW_LOWER, r#def: 0x00 },
    reg_default { reg: CS42L42_FSYNC_PW_UPPER, r#def: 0x00 },
    reg_default { reg: CS42L42_FSYNC_P_LOWER, r#def: 0xF9 },
    reg_default { reg: CS42L42_FSYNC_P_UPPER, r#def: 0x00 },
    reg_default { reg: CS42L42_ASP_CLK_CFG, r#def: 0x00 },
    reg_default { reg: CS42L42_ASP_FRM_CFG, r#def: 0x10 },
    reg_default { reg: CS42L42_FS_RATE_EN, r#def: 0x00 },
    reg_default { reg: CS42L42_IN_ASRC_CLK, r#def: 0x00 },
    reg_default { reg: CS42L42_OUT_ASRC_CLK, r#def: 0x00 },
    reg_default { reg: CS42L42_PLL_DIV_CFG1, r#def: 0x00 },
    reg_default { reg: CS42L42_ADC_OVFL_INT_MASK, r#def: 0x01 },
    reg_default { reg: CS42L42_MIXER_INT_MASK, r#def: 0x0F },
    reg_default { reg: CS42L42_SRC_INT_MASK, r#def: 0x0F },
    reg_default { reg: CS42L42_ASP_RX_INT_MASK, r#def: 0x1F },
    reg_default { reg: CS42L42_ASP_TX_INT_MASK, r#def: 0x0F },
    reg_default { reg: CS42L42_CODEC_INT_MASK, r#def: 0x03 },
    reg_default { reg: CS42L42_SRCPL_INT_MASK, r#def: 0x7F },
    reg_default { reg: CS42L42_VPMON_INT_MASK, r#def: 0x01 },
    reg_default { reg: CS42L42_PLL_LOCK_INT_MASK, r#def: 0x01 },
    reg_default { reg: CS42L42_TSRS_PLUG_INT_MASK, r#def: 0x0F },
    reg_default { reg: CS42L42_PLL_CTL1, r#def: 0x00 },
    reg_default { reg: CS42L42_PLL_DIV_FRAC0, r#def: 0x00 },
    reg_default { reg: CS42L42_PLL_DIV_FRAC1, r#def: 0x00 },
    reg_default { reg: CS42L42_PLL_DIV_FRAC2, r#def: 0x00 },
    reg_default { reg: CS42L42_PLL_DIV_INT, r#def: 0x40 },
    reg_default { reg: CS42L42_PLL_CTL3, r#def: 0x10 },
    reg_default { reg: CS42L42_PLL_CAL_RATIO, r#def: 0x80 },
    reg_default { reg: CS42L42_PLL_CTL4, r#def: 0x03 },
    reg_default { reg: CS42L42_LOAD_DET_EN, r#def: 0x00 },
    reg_default { reg: CS42L42_HSBIAS_SC_AUTOCTL, r#def: 0x03 },
    reg_default { reg: CS42L42_WAKE_CTL, r#def: 0xC0 },
    reg_default { reg: CS42L42_ADC_DISABLE_MUTE, r#def: 0x00 },
    reg_default { reg: CS42L42_TIPSENSE_CTL, r#def: 0x02 },
    reg_default { reg: CS42L42_MISC_DET_CTL, r#def: 0x03 },
    reg_default { reg: CS42L42_MIC_DET_CTL1, r#def: 0x1F },
    reg_default { reg: CS42L42_MIC_DET_CTL2, r#def: 0x2F },
    reg_default { reg: CS42L42_DET_INT1_MASK, r#def: 0xE0 },
    reg_default { reg: CS42L42_DET_INT2_MASK, r#def: 0xFF },
    reg_default { reg: CS42L42_HS_BIAS_CTL, r#def: 0xC2 },
    reg_default { reg: CS42L42_ADC_CTL, r#def: 0x00 },
    reg_default { reg: CS42L42_ADC_VOLUME, r#def: 0x00 },
    reg_default { reg: CS42L42_ADC_WNF_HPF_CTL, r#def: 0x71 },
    reg_default { reg: CS42L42_DAC_CTL1, r#def: 0x00 },
    reg_default { reg: CS42L42_DAC_CTL2, r#def: 0x02 },
    reg_default { reg: CS42L42_HP_CTL, r#def: 0x0D },
    reg_default { reg: CS42L42_CLASSH_CTL, r#def: 0x07 },
    reg_default { reg: CS42L42_MIXER_CHA_VOL, r#def: 0x3F },
    reg_default { reg: CS42L42_MIXER_ADC_VOL, r#def: 0x3F },
    reg_default { reg: CS42L42_MIXER_CHB_VOL, r#def: 0x3F },
    reg_default { reg: CS42L42_EQ_COEF_IN0, r#def: 0x00 },
    reg_default { reg: CS42L42_EQ_COEF_IN1, r#def: 0x00 },
    reg_default { reg: CS42L42_EQ_COEF_IN2, r#def: 0x00 },
    reg_default { reg: CS42L42_EQ_COEF_IN3, r#def: 0x00 },
    reg_default { reg: CS42L42_EQ_COEF_RW, r#def: 0x00 },
    reg_default { reg: CS42L42_EQ_COEF_OUT0, r#def: 0x00 },
    reg_default { reg: CS42L42_EQ_COEF_OUT1, r#def: 0x00 },
    reg_default { reg: CS42L42_EQ_COEF_OUT2, r#def: 0x00 },
    reg_default { reg: CS42L42_EQ_COEF_OUT3, r#def: 0x00 },
    reg_default { reg: CS42L42_EQ_INIT_STAT, r#def: 0x00 },
    reg_default { reg: CS42L42_EQ_START_FILT, r#def: 0x00 },
    reg_default { reg: CS42L42_EQ_MUTE_CTL, r#def: 0x00 },
    reg_default { reg: CS42L42_SP_RX_CH_SEL, r#def: 0x04 },
    reg_default { reg: CS42L42_SP_RX_ISOC_CTL, r#def: 0x04 },
    reg_default { reg: CS42L42_SP_RX_FS, r#def: 0x8C },
    reg_default { reg: CS42l42_SPDIF_CH_SEL, r#def: 0x0E },
    reg_default { reg: CS42L42_SP_TX_ISOC_CTL, r#def: 0x04 },
    reg_default { reg: CS42L42_SP_TX_FS, r#def: 0xCC },
    reg_default { reg: CS42L42_SPDIF_SW_CTL1, r#def: 0x3F },
    reg_default { reg: CS42L42_SRC_SDIN_FS, r#def: 0x40 },
    reg_default { reg: CS42L42_SRC_SDOUT_FS, r#def: 0x40 },
    reg_default { reg: CS42L42_SPDIF_CTL1, r#def: 0x01 },
    reg_default { reg: CS42L42_SPDIF_CTL2, r#def: 0x00 },
    reg_default { reg: CS42L42_SPDIF_CTL3, r#def: 0x00 },
    reg_default { reg: CS42L42_SPDIF_CTL4, r#def: 0x42 },
    reg_default { reg: CS42L42_ASP_TX_SZ_EN, r#def: 0x00 },
    reg_default { reg: CS42L42_ASP_TX_CH_EN, r#def: 0x00 },
    reg_default { reg: CS42L42_ASP_TX_CH_AP_RES, r#def: 0x0F },
    reg_default { reg: CS42L42_ASP_TX_CH1_BIT_MSB, r#def: 0x00 },
    reg_default { reg: CS42L42_ASP_TX_CH1_BIT_LSB, r#def: 0x00 },
    reg_default { reg: CS42L42_ASP_TX_HIZ_DLY_CFG, r#def: 0x00 },
    reg_default { reg: CS42L42_ASP_TX_CH2_BIT_MSB, r#def: 0x00 },
    reg_default { reg: CS42L42_ASP_TX_CH2_BIT_LSB, r#def: 0x00 },
    reg_default { reg: CS42L42_ASP_RX_DAI0_EN, r#def: 0x00 },
    reg_default { reg: CS42L42_ASP_RX_DAI0_CH1_AP_RES, r#def: 0x03 },
    reg_default { reg: CS42L42_ASP_RX_DAI0_CH1_BIT_MSB, r#def: 0x00 },
    reg_default { reg: CS42L42_ASP_RX_DAI0_CH1_BIT_LSB, r#def: 0x00 },
    reg_default { reg: CS42L42_ASP_RX_DAI0_CH2_AP_RES, r#def: 0x03 },
    reg_default { reg: CS42L42_ASP_RX_DAI0_CH2_BIT_MSB, r#def: 0x00 },
    reg_default { reg: CS42L42_ASP_RX_DAI0_CH2_BIT_LSB, r#def: 0x00 },
    reg_default { reg: CS42L42_ASP_RX_DAI0_CH3_AP_RES, r#def: 0x03 },
    reg_default { reg: CS42L42_ASP_RX_DAI0_CH3_BIT_MSB, r#def: 0x00 },
    reg_default { reg: CS42L42_ASP_RX_DAI0_CH3_BIT_LSB, r#def: 0x00 },
    reg_default { reg: CS42L42_ASP_RX_DAI0_CH4_AP_RES, r#def: 0x03 },
    reg_default { reg: CS42L42_ASP_RX_DAI0_CH4_BIT_MSB, r#def: 0x00 },
    reg_default { reg: CS42L42_ASP_RX_DAI0_CH4_BIT_LSB, r#def: 0x00 },
    reg_default { reg: CS42L42_ASP_RX_DAI1_CH1_AP_RES, r#def: 0x03 },
    reg_default { reg: CS42L42_ASP_RX_DAI1_CH1_BIT_MSB, r#def: 0x00 },
    reg_default { reg: CS42L42_ASP_RX_DAI1_CH1_BIT_LSB, r#def: 0x00 },
    reg_default { reg: CS42L42_ASP_RX_DAI1_CH2_AP_RES, r#def: 0x03 },
    reg_default { reg: CS42L42_ASP_RX_DAI1_CH2_BIT_MSB, r#def: 0x00 },
    reg_default { reg: CS42L42_ASP_RX_DAI1_CH2_BIT_LSB, r#def: 0x00 },
];

/*
 * This is all the same as for CS42L42 but we
 * replace the on-reset register defaults.
 */
static cs42l83_regmap: regmap_config = regmap_config {
    reg_bits: 8,
    val_bits: 8,

    readable_reg: Some(cs42l42_readable_register),
    volatile_reg: Some(cs42l42_volatile_register),

    ranges: unsafe { &cs42l42_page_range as *const _ },
    num_ranges: 1,

    max_register: CS42L42_MAX_REGISTER,
    reg_defaults: cs42l83_reg_defaults.as_ptr(),
    num_reg_defaults: cs42l83_reg_defaults.len() as u32,
    cache_type: REGCACHE_MAPLE,

    use_single_read: true,
    use_single_write: true,
};

unsafe extern "C" fn cs42l83_i2c_probe(i2c_client: *mut i2c_client) -> core::ffi::c_int {
    let dev: *mut device = unsafe { &mut (*i2c_client).dev };
    let cs42l83: *mut cs42l42_private;
    let regmap: *mut regmap;
    let ret: core::ffi::c_int;

    cs42l83 = unsafe {
        devm_kzalloc(
            dev,
            core::mem::size_of::<cs42l42_private>(),
            GFP_KERNEL,
        ) as *mut cs42l42_private
    };
    if cs42l83.is_null() {
        return -ENOMEM;
    }

    regmap = unsafe { devm_regmap_init_i2c(i2c_client, &cs42l83_regmap) };
    if unsafe { IS_ERR(regmap as *const core::ffi::c_void) } {
        return unsafe {
            dev_err_probe(
                &mut (*i2c_client).dev,
                PTR_ERR(regmap as *const core::ffi::c_void),
                c"regmap_init() failed\n".as_ptr(),
            )
        };
    }

    unsafe {
        (*cs42l83).devid = CS42L83_CHIP_ID;
        (*cs42l83).dev = dev;
        (*cs42l83).regmap = regmap;
        (*cs42l83).irq = (*i2c_client).irq;
    }

    ret = unsafe { cs42l42_common_probe(cs42l83, &cs42l42_soc_component, &cs42l42_dai) };
    if ret != 0 {
        return ret;
    }

    unsafe { cs42l42_init(cs42l83) }
}

unsafe extern "C" fn cs42l83_i2c_remove(i2c_client: *mut i2c_client) {
    let cs42l83: *mut cs42l42_private =
        unsafe { dev_get_drvdata(&mut (*i2c_client).dev) as *mut cs42l42_private };

    unsafe {
        cs42l42_common_remove(cs42l83);
    }
}

unsafe extern "C" fn cs42l83_i2c_resume(dev: *mut device) -> core::ffi::c_int {
    let ret: core::ffi::c_int;

    ret = unsafe { cs42l42_resume(dev) };
    if ret != 0 {
        return ret;
    }

    unsafe {
        cs42l42_resume_restore(dev);
    }

    0
}

static cs42l83_i2c_pm_ops: dev_pm_ops = dev_pm_ops {
    // SYSTEM_SLEEP_PM_OPS(cs42l42_suspend, cs42l83_i2c_resume)
    suspend: Some(cs42l42_suspend),
    resume: Some(cs42l83_i2c_resume),
};

static cs42l83_of_match: [of_device_id; 2] = [
    of_device_id {
        compatible: c"cirrus,cs42l83".as_ptr(),
        ..unsafe { core::mem::zeroed() }
    },
    unsafe { core::mem::zeroed() },
];
// MODULE_DEVICE_TABLE(of, cs42l83_of_match);

static mut cs42l83_i2c_driver: i2c_driver = i2c_driver {
    driver: device_driver {
        name: c"cs42l83".as_ptr(),
        pm: pm_ptr(&cs42l83_i2c_pm_ops),
        of_match_table: of_match_ptr(cs42l83_of_match.as_ptr()),
        ..unsafe { core::mem::zeroed() }
    },
    probe: Some(cs42l83_i2c_probe),
    remove: Some(cs42l83_i2c_remove),
    ..unsafe { core::mem::zeroed() }
};

// module_i2c_driver(cs42l83_i2c_driver);

// MODULE_DESCRIPTION("ASoC CS42L83 I2C driver");
// MODULE_AUTHOR("Martin Povišer <povik+lin@cutebit.org>");
// MODULE_LICENSE("GPL");
// MODULE_IMPORT_NS("SND_SOC_CS42L42_CORE");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
