// SPDX-License-Identifier: GPL-2.0
/*
 * StarFive JH7100 Audio Clock Driver
 *
 * Copyright (C) 2021 Emil Renner Berthing <kernel@esmil.dk>
 */

// Linux and device-tree dependencies are supplied by the surrounding kernel
// bindings.  The original C include directives are intentionally omitted.

/* external clocks */
pub const JH7100_AUDCLK_AUDIO_SRC: u32 = JH7100_AUDCLK_END + 0;
pub const JH7100_AUDCLK_AUDIO_12288: u32 = JH7100_AUDCLK_END + 1;
pub const JH7100_AUDCLK_DOM7AHB_BUS: u32 = JH7100_AUDCLK_END + 2;
pub const JH7100_AUDCLK_I2SADC_BCLK_IOPAD: u32 = JH7100_AUDCLK_END + 3;
pub const JH7100_AUDCLK_I2SADC_LRCLK_IOPAD: u32 = JH7100_AUDCLK_END + 4;
pub const JH7100_AUDCLK_I2SDAC_BCLK_IOPAD: u32 = JH7100_AUDCLK_END + 5;
pub const JH7100_AUDCLK_I2SDAC_LRCLK_IOPAD: u32 = JH7100_AUDCLK_END + 6;
pub const JH7100_AUDCLK_VAD_INTMEM: u32 = JH7100_AUDCLK_END + 7;

static JH7100_AUDCLK_DATA: &[JH71X0_CLK_DATA] = &[
    JH71X0__GMD!(JH7100_AUDCLK_ADC_MCLK, "adc_mclk", 0, 15, 2, JH7100_AUDCLK_AUDIO_SRC, JH7100_AUDCLK_AUDIO_12288),
    JH71X0__GMD!(JH7100_AUDCLK_I2S1_MCLK, "i2s1_mclk", 0, 15, 2, JH7100_AUDCLK_AUDIO_SRC, JH7100_AUDCLK_AUDIO_12288),
    JH71X0_GATE!(JH7100_AUDCLK_I2SADC_APB, "i2sadc_apb", 0, JH7100_AUDCLK_APB0_BUS),
    JH71X0_MDIV!(JH7100_AUDCLK_I2SADC_BCLK, "i2sadc_bclk", 31, 2, JH7100_AUDCLK_ADC_MCLK, JH7100_AUDCLK_I2SADC_BCLK_IOPAD),
    JH71X0__INV!(JH7100_AUDCLK_I2SADC_BCLK_N, "i2sadc_bclk_n", JH7100_AUDCLK_I2SADC_BCLK),
    JH71X0_MDIV!(JH7100_AUDCLK_I2SADC_LRCLK, "i2sadc_lrclk", 63, 3, JH7100_AUDCLK_I2SADC_BCLK_N, JH7100_AUDCLK_I2SADC_LRCLK_IOPAD, JH7100_AUDCLK_I2SADC_BCLK),
    JH71X0_GATE!(JH7100_AUDCLK_PDM_APB, "pdm_apb", 0, JH7100_AUDCLK_APB0_BUS),
    JH71X0__GMD!(JH7100_AUDCLK_PDM_MCLK, "pdm_mclk", 0, 15, 2, JH7100_AUDCLK_AUDIO_SRC, JH7100_AUDCLK_AUDIO_12288),
    JH71X0_GATE!(JH7100_AUDCLK_I2SVAD_APB, "i2svad_apb", 0, JH7100_AUDCLK_APB0_BUS),
    JH71X0__GMD!(JH7100_AUDCLK_SPDIF, "spdif", 0, 15, 2, JH7100_AUDCLK_AUDIO_SRC, JH7100_AUDCLK_AUDIO_12288),
    JH71X0_GATE!(JH7100_AUDCLK_SPDIF_APB, "spdif_apb", 0, JH7100_AUDCLK_APB0_BUS),
    JH71X0_GATE!(JH7100_AUDCLK_PWMDAC_APB, "pwmdac_apb", 0, JH7100_AUDCLK_APB0_BUS),
    JH71X0__GMD!(JH7100_AUDCLK_DAC_MCLK, "dac_mclk", 0, 15, 2, JH7100_AUDCLK_AUDIO_SRC, JH7100_AUDCLK_AUDIO_12288),
    JH71X0_GATE!(JH7100_AUDCLK_I2SDAC_APB, "i2sdac_apb", 0, JH7100_AUDCLK_APB0_BUS),
    JH71X0_MDIV!(JH7100_AUDCLK_I2SDAC_BCLK, "i2sdac_bclk", 31, 2, JH7100_AUDCLK_DAC_MCLK, JH7100_AUDCLK_I2SDAC_BCLK_IOPAD),
    JH71X0__INV!(JH7100_AUDCLK_I2SDAC_BCLK_N, "i2sdac_bclk_n", JH7100_AUDCLK_I2SDAC_BCLK),
    JH71X0_MDIV!(JH7100_AUDCLK_I2SDAC_LRCLK, "i2sdac_lrclk", 31, 2, JH7100_AUDCLK_I2S1_MCLK, JH7100_AUDCLK_I2SDAC_BCLK_IOPAD),
    JH71X0_GATE!(JH7100_AUDCLK_I2S1_APB, "i2s1_apb", 0, JH7100_AUDCLK_APB0_BUS),
    JH71X0_MDIV!(JH7100_AUDCLK_I2S1_BCLK, "i2s1_bclk", 31, 2, JH7100_AUDCLK_I2S1_MCLK, JH7100_AUDCLK_I2SDAC_BCLK_IOPAD),
    JH71X0__INV!(JH7100_AUDCLK_I2S1_BCLK_N, "i2s1_bclk_n", JH7100_AUDCLK_I2S1_BCLK),
    JH71X0_MDIV!(JH7100_AUDCLK_I2S1_LRCLK, "i2s1_lrclk", 63, 3, JH7100_AUDCLK_I2S1_BCLK_N, JH7100_AUDCLK_I2SDAC_LRCLK_IOPAD),
    JH71X0_GATE!(JH7100_AUDCLK_I2SDAC16K_APB, "i2s1dac16k_apb", 0, JH7100_AUDCLK_APB0_BUS),
    JH71X0__DIV!(JH7100_AUDCLK_APB0_BUS, "apb0_bus", 8, JH7100_AUDCLK_DOM7AHB_BUS),
    JH71X0_GATE!(JH7100_AUDCLK_DMA1P_AHB, "dma1p_ahb", 0, JH7100_AUDCLK_DOM7AHB_BUS),
    JH71X0_GATE!(JH7100_AUDCLK_USB_APB, "usb_apb", CLK_IGNORE_UNUSED, JH7100_AUDCLK_APB_EN),
    JH71X0_GDIV!(JH7100_AUDCLK_USB_LPM, "usb_lpm", CLK_IGNORE_UNUSED, 4, JH7100_AUDCLK_USB_APB),
    JH71X0_GDIV!(JH7100_AUDCLK_USB_STB, "usb_stb", CLK_IGNORE_UNUSED, 3, JH7100_AUDCLK_USB_APB),
    JH71X0__DIV!(JH7100_AUDCLK_APB_EN, "apb_en", 8, JH7100_AUDCLK_DOM7AHB_BUS),
    JH71X0__MUX!(JH7100_AUDCLK_VAD_MEM, "vad_mem", 0, 2, JH7100_AUDCLK_VAD_INTMEM, JH7100_AUDCLK_AUDIO_12288),
];

unsafe extern "C" {
    fn devm_kzalloc(dev: *mut device, size: usize, flags: u32) -> *mut jh71x0_clk_priv;
    fn spin_lock_init(lock: *mut spinlock_t);
    fn devm_platform_ioremap_resource(pdev: *mut platform_device, index: u32) -> *mut core::ffi::c_void;
    fn ptr_err(ptr: *mut core::ffi::c_void) -> i32;
    fn devm_clk_hw_register(dev: *mut device, hw: *mut clk_hw) -> i32;
    fn devm_of_clk_add_hw_provider(dev: *mut device, get: unsafe extern "C" fn(), data: *mut jh71x0_clk_priv) -> i32;
}

unsafe fn jh7100_audclk_probe(pdev: *mut platform_device) -> i32 {
    let priv_: *mut jh71x0_clk_priv = devm_kzalloc(&mut (*pdev).dev, core::mem::size_of::<jh71x0_clk_priv>(), GFP_KERNEL);
    if priv_.is_null() { return -ENOMEM; }
    spin_lock_init(&mut (*priv_).rmw_lock);
    (*priv_).num_reg = JH7100_AUDCLK_END;
    (*priv_).dev = &mut (*pdev).dev;
    (*priv_).base = devm_platform_ioremap_resource(pdev, 0);
    if IS_ERR!((*priv_).base) { return ptr_err((*priv_).base); }
    for idx in 0..JH7100_AUDCLK_END {
        let data = &JH7100_AUDCLK_DATA[idx as usize];
        let clk = &mut (*priv_).reg[idx as usize];
        clk.hw.init = data.init();
        clk.idx = idx;
        clk.max_div = data.max & JH71X0_CLK_DIV_MASK;
        let ret = devm_clk_hw_register((*priv_).dev, &mut clk.hw);
        if ret != 0 { return ret; }
    }
    devm_of_clk_add_hw_provider((*priv_).dev, jh71x0_clk_get, priv_)
}

// Device matching, platform-driver registration, and module metadata are
// supplied by the kernel integration layer.
const JH7100_AUDCLK_MATCH: &[&str] = &["starfive,jh7100-audclk", ""];
const JH7100_AUDCLK_DRIVER_NAME: &str = "clk-starfive-jh7100-audio";
const MODULE_AUTHOR_NAME: &str = "Emil Renner Berthing";
const MODULE_DESCRIPTION_TEXT: &str = "StarFive JH7100 audio clock driver";
const MODULE_LICENSE_TEXT: &str = "GPL v2";

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
