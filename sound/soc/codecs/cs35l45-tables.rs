// SPDX-License-Identifier: GPL-2.0
//
// cs35l45-tables.c -- CS35L45 ALSA SoC audio driver
//
// Copyright 2019-2022 Cirrus Logic, Inc.
//
// Author: James Schulman <james.schulman@cirrus.com>

// Depends on Linux regmap/module support and cs35l45 register definitions.

static CS35L45_PATCH: [reg_sequence; 19] = [
    reg_sequence { reg: 0x00000040, def: 0x00000055 },
    reg_sequence { reg: 0x00000040, def: 0x000000AA },
    reg_sequence { reg: 0x00000044, def: 0x00000055 },
    reg_sequence { reg: 0x00000044, def: 0x000000AA },
    reg_sequence { reg: 0x00006480, def: 0x0830500A },
    reg_sequence { reg: 0x00007C60, def: 0x1000850B },
    reg_sequence { reg: CS35L45_BOOST_OV_CFG, def: 0x007000D0 },
    reg_sequence { reg: CS35L45_LDPM_CONFIG, def: 0x0001B636 },
    reg_sequence { reg: 0x00002C08, def: 0x00000009 },
    reg_sequence { reg: 0x00006850, def: 0x0A30FFC4 },
    reg_sequence { reg: 0x00003820, def: 0x00040100 },
    reg_sequence { reg: 0x00003824, def: 0x00000000 },
    reg_sequence { reg: 0x00007CFC, def: 0x62870004 },
    reg_sequence { reg: 0x00007C60, def: 0x1001850B },
    reg_sequence { reg: 0x00000040, def: 0x00000000 },
    reg_sequence { reg: 0x00000044, def: 0x00000000 },
    reg_sequence { reg: CS35L45_BOOST_CCM_CFG, def: 0xF0000003 },
    reg_sequence { reg: CS35L45_BOOST_DCM_CFG, def: 0x08710220 },
    reg_sequence { reg: CS35L45_ERROR_RELEASE, def: 0x00200000 },
];

extern "C" {
    fn regmap_register_patch(
        regmap: *mut regmap,
        regs: *const reg_sequence,
        num_regs: usize,
    ) -> ::core::ffi::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn cs35l45_apply_patch(cs35l45: *mut cs35l45_private) -> ::core::ffi::c_int {
    regmap_register_patch(
        (*cs35l45).regmap,
        CS35L45_PATCH.as_ptr(),
        CS35L45_PATCH.len(),
    )
}
// EXPORT_SYMBOL_NS_GPL(cs35l45_apply_patch, "SND_SOC_CS35L45");

static CS35L45_DEFAULTS: [reg_default; 76] = [
    reg_default { reg: CS35L45_BLOCK_ENABLES, def: 0x00003323 },
    reg_default { reg: CS35L45_BLOCK_ENABLES2, def: 0x00000010 },
    reg_default { reg: CS35L45_SYNC_GPIO1, def: 0x00000007 },
    reg_default { reg: CS35L45_INTB_GPIO2_MCLK_REF, def: 0x00000005 },
    reg_default { reg: CS35L45_GPIO3, def: 0x00000005 },
    reg_default { reg: CS35L45_PWRMGT_CTL, def: 0x00000000 },
    reg_default { reg: CS35L45_WAKESRC_CTL, def: 0x00000008 },
    reg_default { reg: CS35L45_WKI2C_CTL, def: 0x00000030 },
    reg_default { reg: CS35L45_REFCLK_INPUT, def: 0x00000510 },
    reg_default { reg: CS35L45_GLOBAL_SAMPLE_RATE, def: 0x00000003 },
    reg_default { reg: CS35L45_ASP_ENABLES1, def: 0x00000000 },
    reg_default { reg: CS35L45_ASP_CONTROL1, def: 0x00000028 },
    reg_default { reg: CS35L45_ASP_CONTROL2, def: 0x18180200 },
    reg_default { reg: CS35L45_ASP_CONTROL3, def: 0x00000002 },
    reg_default { reg: CS35L45_ASP_FRAME_CONTROL1, def: 0x03020100 },
    reg_default { reg: CS35L45_ASP_FRAME_CONTROL2, def: 0x00000004 },
    reg_default { reg: CS35L45_ASP_FRAME_CONTROL5, def: 0x00000100 },
    reg_default { reg: CS35L45_ASP_DATA_CONTROL1, def: 0x00000018 },
    reg_default { reg: CS35L45_ASP_DATA_CONTROL5, def: 0x00000018 },
    reg_default { reg: CS35L45_DACPCM1_INPUT, def: 0x00000008 },
    reg_default { reg: CS35L45_ASPTX1_INPUT, def: 0x00000018 },
    reg_default { reg: CS35L45_ASPTX2_INPUT, def: 0x00000019 },
    reg_default { reg: CS35L45_ASPTX3_INPUT, def: 0x00000020 },
    reg_default { reg: CS35L45_ASPTX4_INPUT, def: 0x00000028 },
    reg_default { reg: CS35L45_ASPTX5_INPUT, def: 0x00000048 },
    reg_default { reg: CS35L45_DSP1RX1_INPUT, def: 0x00000008 },
    reg_default { reg: CS35L45_DSP1RX2_INPUT, def: 0x00000009 },
    reg_default { reg: CS35L45_DSP1RX3_INPUT, def: 0x00000018 },
    reg_default { reg: CS35L45_DSP1RX4_INPUT, def: 0x00000019 },
    reg_default { reg: CS35L45_DSP1RX5_INPUT, def: 0x00000020 },
    reg_default { reg: CS35L45_DSP1RX6_INPUT, def: 0x00000028 },
    reg_default { reg: CS35L45_DSP1RX7_INPUT, def: 0x0000003A },
    reg_default { reg: CS35L45_DSP1RX8_INPUT, def: 0x00000028 },
    reg_default { reg: CS35L45_AMP_PCM_CONTROL, def: 0x00100000 },
    reg_default { reg: CS35L45_AMP_GAIN, def: 0x00002300 },
    reg_default { reg: CS35L45_IRQ1_CFG, def: 0x00000000 },
    reg_default { reg: CS35L45_IRQ1_MASK_1, def: 0xBFEFFFBF },
    reg_default { reg: CS35L45_IRQ1_MASK_2, def: 0xFFFFFFFF },
    reg_default { reg: CS35L45_IRQ1_MASK_3, def: 0xFFFF87FF },
    reg_default { reg: CS35L45_IRQ1_MASK_4, def: 0xF8FFFFFF },
    reg_default { reg: CS35L45_IRQ1_MASK_5, def: 0x0EF80000 },
    reg_default { reg: CS35L45_IRQ1_MASK_6, def: 0x00000000 },
    reg_default { reg: CS35L45_IRQ1_MASK_7, def: 0xFFFFFF78 },
    reg_default { reg: CS35L45_IRQ1_MASK_8, def: 0x00003FFF },
    reg_default { reg: CS35L45_IRQ1_MASK_9, def: 0x00000000 },
    reg_default { reg: CS35L45_IRQ1_MASK_10, def: 0x00000000 },
    reg_default { reg: CS35L45_IRQ1_MASK_11, def: 0x00000000 },
    reg_default { reg: CS35L45_IRQ1_MASK_12, def: 0x00000000 },
    reg_default { reg: CS35L45_IRQ1_MASK_13, def: 0x00000000 },
    reg_default { reg: CS35L45_IRQ1_MASK_14, def: 0x00000001 },
    reg_default { reg: CS35L45_IRQ1_MASK_15, def: 0x00000000 },
    reg_default { reg: CS35L45_IRQ1_MASK_16, def: 0x00000000 },
    reg_default { reg: CS35L45_IRQ1_MASK_17, def: 0x00000000 },
    reg_default { reg: CS35L45_IRQ1_MASK_18, def: 0x3FE5D0FF },
    reg_default { reg: CS35L45_GPIO1_CTRL1, def: 0x81000001 },
    reg_default { reg: CS35L45_GPIO2_CTRL1, def: 0x81000001 },
    reg_default { reg: CS35L45_GPIO3_CTRL1, def: 0x81000001 },
    reg_default { reg: CS35L45_DSP1_RX1_RATE, def: 0x00000001 },
    reg_default { reg: CS35L45_DSP1_RX2_RATE, def: 0x00000001 },
    reg_default { reg: CS35L45_DSP1_RX3_RATE, def: 0x00000001 },
    reg_default { reg: CS35L45_DSP1_RX4_RATE, def: 0x00000001 },
    reg_default { reg: CS35L45_DSP1_RX5_RATE, def: 0x00000001 },
    reg_default { reg: CS35L45_DSP1_RX6_RATE, def: 0x00000001 },
    reg_default { reg: CS35L45_DSP1_RX7_RATE, def: 0x00000001 },
    reg_default { reg: CS35L45_DSP1_RX8_RATE, def: 0x00000001 },
    reg_default { reg: CS35L45_DSP1_TX1_RATE, def: 0x00000001 },
    reg_default { reg: CS35L45_DSP1_TX2_RATE, def: 0x00000001 },
    reg_default { reg: CS35L45_DSP1_TX3_RATE, def: 0x00000001 },
    reg_default { reg: CS35L45_DSP1_TX4_RATE, def: 0x00000001 },
    reg_default { reg: CS35L45_DSP1_TX5_RATE, def: 0x00000001 },
    reg_default { reg: CS35L45_DSP1_TX6_RATE, def: 0x00000001 },
    reg_default { reg: CS35L45_DSP1_TX7_RATE, def: 0x00000001 },
    reg_default { reg: CS35L45_DSP1_TX8_RATE, def: 0x00000001 },
];

unsafe extern "C" fn cs35l45_readable_reg(_dev: *mut device, reg: ::core::ffi::c_uint) -> bool {
    match reg {
        CS35L45_DEVID..=CS35L45_OTPID
        | CS35L45_SFT_RESET
        | CS35L45_GLOBAL_ENABLES
        | CS35L45_BLOCK_ENABLES
        | CS35L45_BLOCK_ENABLES2
        | CS35L45_ERROR_RELEASE
        | CS35L45_SYNC_GPIO1
        | CS35L45_INTB_GPIO2_MCLK_REF
        | CS35L45_GPIO3
        | CS35L45_PWRMGT_CTL
        | CS35L45_WAKESRC_CTL
        | CS35L45_WKI2C_CTL
        | CS35L45_PWRMGT_STS
        | CS35L45_REFCLK_INPUT
        | CS35L45_GLOBAL_SAMPLE_RATE
        | CS35L45_ASP_ENABLES1
        | CS35L45_ASP_CONTROL1
        | CS35L45_ASP_CONTROL2
        | CS35L45_ASP_CONTROL3
        | CS35L45_ASP_FRAME_CONTROL1
        | CS35L45_ASP_FRAME_CONTROL2
        | CS35L45_ASP_FRAME_CONTROL5
        | CS35L45_ASP_DATA_CONTROL1
        | CS35L45_ASP_DATA_CONTROL5
        | CS35L45_DACPCM1_INPUT
        | CS35L45_ASPTX1_INPUT
        | CS35L45_ASPTX2_INPUT
        | CS35L45_ASPTX3_INPUT
        | CS35L45_ASPTX4_INPUT
        | CS35L45_ASPTX5_INPUT
        | CS35L45_DSP1RX1_INPUT
        | CS35L45_DSP1RX2_INPUT
        | CS35L45_DSP1RX3_INPUT
        | CS35L45_DSP1RX4_INPUT
        | CS35L45_DSP1RX5_INPUT
        | CS35L45_DSP1RX6_INPUT
        | CS35L45_DSP1RX7_INPUT
        | CS35L45_DSP1RX8_INPUT
        | CS35L45_HVLV_CONFIG
        | CS35L45_AMP_PCM_CONTROL
        | CS35L45_AMP_GAIN
        | CS35L45_AMP_PCM_HPF_TST
        | CS35L45_IRQ1_CFG
        | CS35L45_IRQ1_STATUS
        | CS35L45_IRQ1_EINT_1..=CS35L45_IRQ1_EINT_18
        | CS35L45_IRQ1_STS_1..=CS35L45_IRQ1_STS_18
        | CS35L45_IRQ1_MASK_1..=CS35L45_IRQ1_MASK_18
        | CS35L45_GPIO_STATUS1
        | CS35L45_GPIO1_CTRL1
        | CS35L45_GPIO2_CTRL1
        | CS35L45_GPIO3_CTRL1
        | CS35L45_DSP_MBOX_1
        | CS35L45_DSP_MBOX_2
        | CS35L45_DSP_VIRT1_MBOX_1..=CS35L45_DSP_VIRT1_MBOX_4
        | CS35L45_DSP_VIRT2_MBOX_1..=CS35L45_DSP_VIRT2_MBOX_4
        | CS35L45_DSP1_SYS_ID
        | CS35L45_DSP1_CLOCK_FREQ
        | CS35L45_DSP1_RX1_RATE
        | CS35L45_DSP1_RX2_RATE
        | CS35L45_DSP1_RX3_RATE
        | CS35L45_DSP1_RX4_RATE
        | CS35L45_DSP1_RX5_RATE
        | CS35L45_DSP1_RX6_RATE
        | CS35L45_DSP1_RX7_RATE
        | CS35L45_DSP1_RX8_RATE
        | CS35L45_DSP1_TX1_RATE
        | CS35L45_DSP1_TX2_RATE
        | CS35L45_DSP1_TX3_RATE
        | CS35L45_DSP1_TX4_RATE
        | CS35L45_DSP1_TX5_RATE
        | CS35L45_DSP1_TX6_RATE
        | CS35L45_DSP1_TX7_RATE
        | CS35L45_DSP1_TX8_RATE
        | CS35L45_DSP1_SCRATCH1
        | CS35L45_DSP1_SCRATCH2
        | CS35L45_DSP1_SCRATCH3
        | CS35L45_DSP1_SCRATCH4
        | CS35L45_DSP1_CCM_CORE_CONTROL
        | CS35L45_DSP1_XMEM_PACK_0..=CS35L45_DSP1_XMEM_PACK_4607
        | CS35L45_DSP1_XMEM_UNPACK32_0..=CS35L45_DSP1_XMEM_UNPACK32_3071
        | CS35L45_DSP1_XMEM_UNPACK24_0..=CS35L45_DSP1_XMEM_UNPACK24_6143
        | CS35L45_DSP1_YMEM_PACK_0..=CS35L45_DSP1_YMEM_PACK_1532
        | CS35L45_DSP1_YMEM_UNPACK32_0..=CS35L45_DSP1_YMEM_UNPACK32_1022
        | CS35L45_DSP1_YMEM_UNPACK24_0..=CS35L45_DSP1_YMEM_UNPACK24_2043
        | CS35L45_DSP1_PMEM_0..=CS35L45_DSP1_PMEM_3834 => true,
        _ => false,
    }
}

unsafe extern "C" fn cs35l45_volatile_reg(_dev: *mut device, reg: ::core::ffi::c_uint) -> bool {
    match reg {
        CS35L45_DEVID..=CS35L45_OTPID
        | CS35L45_SFT_RESET
        | CS35L45_GLOBAL_ENABLES
        | CS35L45_ERROR_RELEASE
        | CS35L45_AMP_PCM_HPF_TST // not cachable
        | CS35L45_PWRMGT_STS
        | CS35L45_IRQ1_STATUS
        | CS35L45_IRQ1_EINT_1..=CS35L45_IRQ1_EINT_18
        | CS35L45_IRQ1_STS_1..=CS35L45_IRQ1_STS_18
        | CS35L45_GPIO_STATUS1
        | CS35L45_DSP_MBOX_1
        | CS35L45_DSP_MBOX_2
        | CS35L45_DSP_VIRT1_MBOX_1..=CS35L45_DSP_VIRT1_MBOX_4
        | CS35L45_DSP_VIRT2_MBOX_1..=CS35L45_DSP_VIRT2_MBOX_4
        | CS35L45_DSP1_SYS_ID
        | CS35L45_DSP1_CLOCK_FREQ
        | CS35L45_DSP1_SCRATCH1
        | CS35L45_DSP1_SCRATCH2
        | CS35L45_DSP1_SCRATCH3
        | CS35L45_DSP1_SCRATCH4
        | CS35L45_DSP1_CCM_CORE_CONTROL
        | CS35L45_DSP1_XMEM_PACK_0..=CS35L45_DSP1_XMEM_PACK_4607
        | CS35L45_DSP1_XMEM_UNPACK32_0..=CS35L45_DSP1_XMEM_UNPACK32_3071
        | CS35L45_DSP1_XMEM_UNPACK24_0..=CS35L45_DSP1_XMEM_UNPACK24_6143
        | CS35L45_DSP1_YMEM_PACK_0..=CS35L45_DSP1_YMEM_PACK_1532
        | CS35L45_DSP1_YMEM_UNPACK32_0..=CS35L45_DSP1_YMEM_UNPACK32_1022
        | CS35L45_DSP1_YMEM_UNPACK24_0..=CS35L45_DSP1_YMEM_UNPACK24_2043
        | CS35L45_DSP1_PMEM_0..=CS35L45_DSP1_PMEM_3834 => true,
        _ => false,
    }
}

#[no_mangle]
pub static CS35L45_I2C_REGMAP: regmap_config = regmap_config {
    reg_bits: 32,
    val_bits: 32,
    reg_stride: 4,
    reg_format_endian: REGMAP_ENDIAN_BIG,
    val_format_endian: REGMAP_ENDIAN_BIG,
    max_register: CS35L45_LASTREG,
    reg_defaults: CS35L45_DEFAULTS.as_ptr(),
    num_reg_defaults: CS35L45_DEFAULTS.len(),
    volatile_reg: Some(cs35l45_volatile_reg),
    readable_reg: Some(cs35l45_readable_reg),
    cache_type: REGCACHE_MAPLE,
};
// EXPORT_SYMBOL_NS_GPL(cs35l45_i2c_regmap, "SND_SOC_CS35L45");

#[no_mangle]
pub static CS35L45_SPI_REGMAP: regmap_config = regmap_config {
    reg_bits: 32,
    val_bits: 32,
    pad_bits: 16,
    reg_stride: 4,
    reg_format_endian: REGMAP_ENDIAN_BIG,
    val_format_endian: REGMAP_ENDIAN_BIG,
    max_register: CS35L45_LASTREG,
    reg_defaults: CS35L45_DEFAULTS.as_ptr(),
    num_reg_defaults: CS35L45_DEFAULTS.len(),
    volatile_reg: Some(cs35l45_volatile_reg),
    readable_reg: Some(cs35l45_readable_reg),
    cache_type: REGCACHE_MAPLE,
};
// EXPORT_SYMBOL_NS_GPL(cs35l45_spi_regmap, "SND_SOC_CS35L45");

#[repr(C)]
struct cs35l45_pll_refclk_freq_entry {
    cfg_id: u8,
    freq: u32,
}

static CS35L45_PLL_REFCLK_FREQ: [cs35l45_pll_refclk_freq_entry; 31] = [
    cs35l45_pll_refclk_freq_entry { cfg_id: 0x0C, freq: 128000 },
    cs35l45_pll_refclk_freq_entry { cfg_id: 0x0F, freq: 256000 },
    cs35l45_pll_refclk_freq_entry { cfg_id: 0x11, freq: 384000 },
    cs35l45_pll_refclk_freq_entry { cfg_id: 0x12, freq: 512000 },
    cs35l45_pll_refclk_freq_entry { cfg_id: 0x15, freq: 768000 },
    cs35l45_pll_refclk_freq_entry { cfg_id: 0x17, freq: 1024000 },
    cs35l45_pll_refclk_freq_entry { cfg_id: 0x19, freq: 1411200 },
    cs35l45_pll_refclk_freq_entry { cfg_id: 0x1B, freq: 1536000 },
    cs35l45_pll_refclk_freq_entry { cfg_id: 0x1C, freq: 2116800 },
    cs35l45_pll_refclk_freq_entry { cfg_id: 0x1D, freq: 2048000 },
    cs35l45_pll_refclk_freq_entry { cfg_id: 0x1E, freq: 2304000 },
    cs35l45_pll_refclk_freq_entry { cfg_id: 0x1F, freq: 2822400 },
    cs35l45_pll_refclk_freq_entry { cfg_id: 0x21, freq: 3072000 },
    cs35l45_pll_refclk_freq_entry { cfg_id: 0x23, freq: 4233600 },
    cs35l45_pll_refclk_freq_entry { cfg_id: 0x24, freq: 4096000 },
    cs35l45_pll_refclk_freq_entry { cfg_id: 0x25, freq: 4608000 },
    cs35l45_pll_refclk_freq_entry { cfg_id: 0x26, freq: 5644800 },
    cs35l45_pll_refclk_freq_entry { cfg_id: 0x27, freq: 6000000 },
    cs35l45_pll_refclk_freq_entry { cfg_id: 0x28, freq: 6144000 },
    cs35l45_pll_refclk_freq_entry { cfg_id: 0x29, freq: 6350400 },
    cs35l45_pll_refclk_freq_entry { cfg_id: 0x2A, freq: 6912000 },
    cs35l45_pll_refclk_freq_entry { cfg_id: 0x2D, freq: 7526400 },
    cs35l45_pll_refclk_freq_entry { cfg_id: 0x2E, freq: 8467200 },
    cs35l45_pll_refclk_freq_entry { cfg_id: 0x2F, freq: 8192000 },
    cs35l45_pll_refclk_freq_entry { cfg_id: 0x30, freq: 9216000 },
    cs35l45_pll_refclk_freq_entry { cfg_id: 0x31, freq: 11289600 },
    cs35l45_pll_refclk_freq_entry { cfg_id: 0x33, freq: 12288000 },
    cs35l45_pll_refclk_freq_entry { cfg_id: 0x37, freq: 16934400 },
    cs35l45_pll_refclk_freq_entry { cfg_id: 0x38, freq: 18432000 },
    cs35l45_pll_refclk_freq_entry { cfg_id: 0x39, freq: 22579200 },
    cs35l45_pll_refclk_freq_entry { cfg_id: 0x3B, freq: 24576000 },
];

#[no_mangle]
pub extern "C" fn cs35l45_get_clk_freq_id(freq: ::core::ffi::c_uint) -> ::core::ffi::c_int {
    if freq == 0 {
        return -EINVAL;
    }

    for entry in CS35L45_PLL_REFCLK_FREQ.iter() {
        if entry.freq == freq {
            return entry.cfg_id as ::core::ffi::c_int;
        }
    }

    -EINVAL
}
// EXPORT_SYMBOL_NS_GPL(cs35l45_get_clk_freq_id, "SND_SOC_CS35L45");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
