// SPDX-License-Identifier: GPL-2.0
//
// Socionext UniPhier EVEA ADC/DAC codec driver.
//
// Copyright (c) 2016-2017 Socionext Inc.

// Linux kernel audio driver dependencies:
// #include <linux/clk.h>
// #include <linux/module.h>
// #include <linux/of.h>
// #include <linux/regmap.h>
// #include <linux/reset.h>
// #include <sound/pcm.h>
// #include <sound/soc.h>

const DRV_NAME: &str = "evea";
// EVEA_RATES = SNDRV_PCM_RATE_48000
// EVEA_FORMATS = SNDRV_PCM_FMTBIT_S32_LE

fn aadcpow(n: u32) -> u32 {
    0x0078 + 0x04 * n
}
const AADCPOW_AADC_POWD: u32 = 1 << 0;

const ALINSW1: u32 = 0x0088;
const ALINSW1_SEL1_SHIFT: u32 = 3;

const AHPOUTPOW: u32 = 0x0098;
const AHPOUTPOW_HP_ON: u32 = 1 << 4;

const ALINEPOW: u32 = 0x009c;
const ALINEPOW_LIN2_POWD: u32 = 1 << 3;
const ALINEPOW_LIN1_POWD: u32 = 1 << 4;

const ALO1OUTPOW: u32 = 0x00a8;
const ALO1OUTPOW_LO1_ON: u32 = 1 << 4;

const ALO2OUTPOW: u32 = 0x00ac;
const ALO2OUTPOW_ADAC2_MUTE: u32 = 1 << 0;
const ALO2OUTPOW_LO2_ON: u32 = 1 << 4;

const AANAPOW: u32 = 0x00b8;
const AANAPOW_A_POWD: u32 = 1 << 4;

fn adacseq1(n: u32) -> u32 {
    0x0144 + 0x40 * n
}
const ADACSEQ1_MMUTE: u32 = 1 << 1;

fn adacseq2(n: u32) -> u32 {
    0x0160 + 0x40 * n
}
const ADACSEQ2_ADACIN_FIX: u32 = 1 << 0;

const ADAC1ODC: u32 = 0x0200;
const ADAC1ODC_HP_DIS_RES_MASK: u32 = 0b110;
const ADAC1ODC_HP_DIS_RES_OFF: u32 = 0x0 << 1;
const ADAC1ODC_HP_DIS_RES_ON: u32 = 0x3 << 1;
const ADAC1ODC_ADAC_RAMPCLT_MASK: u32 = 0b110000000;
const ADAC1ODC_ADAC_RAMPCLT_NORMAL: u32 = 0x0 << 7;
const ADAC1ODC_ADAC_RAMPCLT_REDUCE: u32 = 0x1 << 7;

#[repr(C)]
pub struct EvéaPriv {
    pub clk: *mut core::ffi::c_void,
    pub clk_exiv: *mut core::ffi::c_void,
    pub rst: *mut core::ffi::c_void,
    pub rst_exiv: *mut core::ffi::c_void,
    pub rst_adamv: *mut core::ffi::c_void,
    pub regmap: *mut core::ffi::c_void,
    pub switch_lin: i32,
    pub switch_lo: i32,
    pub switch_hp: i32,
}

// linsw1_sel1_text array
const LINSW1_SEL1_TEXT: &[&str] = &["LIN1", "LIN2", "LIN3"];

// SOC_ENUM_SINGLE_DECL(linsw1_sel1_enum, ALINSW1, ALINSW1_SEL1_SHIFT, linsw1_sel1_text)
// linesw1_mux array with SOC_DAPM_ENUM("Line In 1 Source", linsw1_sel1_enum)

// evea_widgets array with SND_SOC_DAPM_* declarations
// ADC, MUX, INPUT, DAC, OUTPUT widgets

// evea_routes array with routing entries

// External kernel functions - these are defined in the Linux kernel
extern "C" {
    // Device memory allocation
    fn devm_kzalloc(dev: *mut core::ffi::c_void, size: usize, flags: u32) -> *mut core::ffi::c_void;

    // Clock functions
    fn devm_clk_get(dev: *mut core::ffi::c_void, id: *const u8) -> *mut core::ffi::c_void;
    fn clk_prepare_enable(clk: *mut core::ffi::c_void) -> i32;
    fn clk_disable_unprepare(clk: *mut core::ffi::c_void);

    // Reset control functions
    fn devm_reset_control_get_shared(dev: *mut core::ffi::c_void, id: *const u8) -> *mut core::ffi::c_void;
    fn reset_control_deassert(rstc: *mut core::ffi::c_void) -> i32;
    fn reset_control_assert(rstc: *mut core::ffi::c_void) -> i32;

    // Register map functions
    fn devm_platform_ioremap_resource(pdev: *mut core::ffi::c_void, index: u32) -> *mut core::ffi::c_void;
    fn devm_regmap_init_mmio(dev: *mut core::ffi::c_void, regs: *mut core::ffi::c_void, config: *const core::ffi::c_void) -> *mut core::ffi::c_void;
    fn regmap_update_bits(map: *mut core::ffi::c_void, reg: u32, mask: u32, val: u32) -> i32;

    // SoC codec/DAI registration
    fn devm_snd_soc_register_component(dev: *mut core::ffi::c_void, cmpnt_drv: *const core::ffi::c_void, dai_drv: *const core::ffi::c_void, num_dai: i32) -> i32;

    // Platform driver helpers
    fn platform_set_drvdata(pdev: *mut core::ffi::c_void, data: *mut core::ffi::c_void);
    fn platform_get_drvdata(pdev: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
}

// Inline helper macros translated to functions
fn is_err(ptr: *const core::ffi::c_void) -> bool {
    (ptr as usize) > usize::MAX - 4095
}

fn ptr_err(ptr: *const core::ffi::c_void) -> i32 {
    (ptr as i32)
}

unsafe fn evea_set_power_state_on(evea: *mut EvéaPriv) {
    let map = (*evea).regmap;

    regmap_update_bits(map, AANAPOW, AANAPOW_A_POWD, AANAPOW_A_POWD);

    regmap_update_bits(map, ADAC1ODC, ADAC1ODC_HP_DIS_RES_MASK,
                       ADAC1ODC_HP_DIS_RES_ON);

    regmap_update_bits(map, ADAC1ODC, ADAC1ODC_ADAC_RAMPCLT_MASK,
                       ADAC1ODC_ADAC_RAMPCLT_REDUCE);

    regmap_update_bits(map, adacseq2(0), ADACSEQ2_ADACIN_FIX, 0);
    regmap_update_bits(map, adacseq2(1), ADACSEQ2_ADACIN_FIX, 0);
    regmap_update_bits(map, adacseq2(2), ADACSEQ2_ADACIN_FIX, 0);
}

unsafe fn evea_set_power_state_off(evea: *mut EvéaPriv) {
    let map = (*evea).regmap;

    regmap_update_bits(map, ADAC1ODC, ADAC1ODC_HP_DIS_RES_MASK,
                       ADAC1ODC_HP_DIS_RES_ON);

    regmap_update_bits(map, adacseq1(0), ADACSEQ1_MMUTE,
                       ADACSEQ1_MMUTE);
    regmap_update_bits(map, adacseq1(1), ADACSEQ1_MMUTE,
                       ADACSEQ1_MMUTE);
    regmap_update_bits(map, adacseq1(2), ADACSEQ1_MMUTE,
                       ADACSEQ1_MMUTE);

    regmap_update_bits(map, ALO1OUTPOW, ALO1OUTPOW_LO1_ON, 0);
    regmap_update_bits(map, ALO2OUTPOW, ALO2OUTPOW_LO2_ON, 0);
    regmap_update_bits(map, AHPOUTPOW, AHPOUTPOW_HP_ON, 0);
}

unsafe fn evea_update_switch_lin(evea: *mut EvéaPriv) -> i32 {
    let map = (*evea).regmap;

    if (*evea).switch_lin != 0 {
        regmap_update_bits(map, ALINEPOW,
                           ALINEPOW_LIN2_POWD | ALINEPOW_LIN1_POWD,
                           ALINEPOW_LIN2_POWD | ALINEPOW_LIN1_POWD);

        regmap_update_bits(map, aadcpow(0), AADCPOW_AADC_POWD,
                           AADCPOW_AADC_POWD);
        regmap_update_bits(map, aadcpow(1), AADCPOW_AADC_POWD,
                           AADCPOW_AADC_POWD);
    } else {
        regmap_update_bits(map, aadcpow(0), AADCPOW_AADC_POWD, 0);
        regmap_update_bits(map, aadcpow(1), AADCPOW_AADC_POWD, 0);

        regmap_update_bits(map, ALINEPOW,
                           ALINEPOW_LIN2_POWD | ALINEPOW_LIN1_POWD, 0);
    }

    0
}

unsafe fn evea_update_switch_lo(evea: *mut EvéaPriv) -> i32 {
    let map = (*evea).regmap;

    if (*evea).switch_lo != 0 {
        regmap_update_bits(map, adacseq1(0), ADACSEQ1_MMUTE, 0);
        regmap_update_bits(map, adacseq1(2), ADACSEQ1_MMUTE, 0);

        regmap_update_bits(map, ALO1OUTPOW, ALO1OUTPOW_LO1_ON,
                           ALO1OUTPOW_LO1_ON);
        regmap_update_bits(map, ALO2OUTPOW,
                           ALO2OUTPOW_ADAC2_MUTE | ALO2OUTPOW_LO2_ON,
                           ALO2OUTPOW_ADAC2_MUTE | ALO2OUTPOW_LO2_ON);
    } else {
        regmap_update_bits(map, adacseq1(0), ADACSEQ1_MMUTE,
                           ADACSEQ1_MMUTE);
        regmap_update_bits(map, adacseq1(2), ADACSEQ1_MMUTE,
                           ADACSEQ1_MMUTE);

        regmap_update_bits(map, ALO1OUTPOW, ALO1OUTPOW_LO1_ON, 0);
        regmap_update_bits(map, ALO2OUTPOW,
                           ALO2OUTPOW_ADAC2_MUTE | ALO2OUTPOW_LO2_ON,
                           0);
    }

    0
}

unsafe fn evea_update_switch_hp(evea: *mut EvéaPriv) -> i32 {
    let map = (*evea).regmap;

    if (*evea).switch_hp != 0 {
        regmap_update_bits(map, adacseq1(1), ADACSEQ1_MMUTE, 0);

        regmap_update_bits(map, AHPOUTPOW, AHPOUTPOW_HP_ON,
                           AHPOUTPOW_HP_ON);

        regmap_update_bits(map, ADAC1ODC, ADAC1ODC_HP_DIS_RES_MASK,
                           ADAC1ODC_HP_DIS_RES_OFF);
    } else {
        regmap_update_bits(map, ADAC1ODC, ADAC1ODC_HP_DIS_RES_MASK,
                           ADAC1ODC_HP_DIS_RES_ON);

        regmap_update_bits(map, adacseq1(1), ADACSEQ1_MMUTE,
                           ADACSEQ1_MMUTE);

        regmap_update_bits(map, AHPOUTPOW, AHPOUTPOW_HP_ON, 0);
    }

    0
}

unsafe fn evea_update_switch_all(evea: *mut EvéaPriv) {
    evea_update_switch_lin(evea);
    evea_update_switch_lo(evea);
    evea_update_switch_hp(evea);
}

unsafe extern "C" fn evea_get_switch_lin(kcontrol: *mut core::ffi::c_void,
                                         ucontrol: *mut core::ffi::c_void) -> i32 {
    // struct snd_soc_component *component = snd_kcontrol_chip(kcontrol);
    // struct evea_priv *evea = snd_soc_component_get_drvdata(component);
    // ucontrol->value.integer.value[0] = evea->switch_lin;
    // Return 0 for success
    0
}

unsafe extern "C" fn evea_set_switch_lin(kcontrol: *mut core::ffi::c_void,
                                         ucontrol: *mut core::ffi::c_void) -> i32 {
    // struct snd_soc_component *component = snd_kcontrol_chip(kcontrol);
    // struct evea_priv *evea = snd_soc_component_get_drvdata(component);
    // if (evea->switch_lin == ucontrol->value.integer.value[0])
    //     return 0;
    // evea->switch_lin = ucontrol->value.integer.value[0];
    // return evea_update_switch_lin(evea);
    0
}

unsafe extern "C" fn evea_get_switch_lo(kcontrol: *mut core::ffi::c_void,
                                        ucontrol: *mut core::ffi::c_void) -> i32 {
    0
}

unsafe extern "C" fn evea_set_switch_lo(kcontrol: *mut core::ffi::c_void,
                                        ucontrol: *mut core::ffi::c_void) -> i32 {
    0
}

unsafe extern "C" fn evea_get_switch_hp(kcontrol: *mut core::ffi::c_void,
                                        ucontrol: *mut core::ffi::c_void) -> i32 {
    0
}

unsafe extern "C" fn evea_set_switch_hp(kcontrol: *mut core::ffi::c_void,
                                        ucontrol: *mut core::ffi::c_void) -> i32 {
    0
}

// evea_controls array with SOC_SINGLE_BOOL_EXT declarations

unsafe extern "C" fn evea_codec_probe(component: *mut core::ffi::c_void) -> i32 {
    // struct evea_priv *evea = snd_soc_component_get_drvdata(component);
    // evea->switch_lin = 1;
    // evea->switch_lo = 1;
    // evea->switch_hp = 1;
    // evea_set_power_state_on(evea);
    // evea_update_switch_all(evea);
    // return 0;
    0
}

unsafe extern "C" fn evea_codec_suspend(component: *mut core::ffi::c_void) -> i32 {
    // struct evea_priv *evea = snd_soc_component_get_drvdata(component);
    // evea_set_power_state_off(evea);
    // reset_control_assert(evea->rst_adamv);
    // reset_control_assert(evea->rst_exiv);
    // reset_control_assert(evea->rst);
    // clk_disable_unprepare(evea->clk_exiv);
    // clk_disable_unprepare(evea->clk);
    // return 0;
    0
}

unsafe extern "C" fn evea_codec_resume(component: *mut core::ffi::c_void) -> i32 {
    // struct evea_priv *evea = snd_soc_component_get_drvdata(component);
    // int ret;
    //
    // ret = clk_prepare_enable(evea->clk);
    // if (ret)
    //     return ret;
    //
    // ret = clk_prepare_enable(evea->clk_exiv);
    // if (ret)
    //     goto err_out_clock;
    //
    // ret = reset_control_deassert(evea->rst);
    // if (ret)
    //     goto err_out_clock_exiv;
    //
    // ret = reset_control_deassert(evea->rst_exiv);
    // if (ret)
    //     goto err_out_reset;
    //
    // ret = reset_control_deassert(evea->rst_adamv);
    // if (ret)
    //     goto err_out_reset_exiv;
    //
    // evea_set_power_state_on(evea);
    // evea_update_switch_all(evea);
    //
    // return 0;
    //
    // err_out_reset_exiv:
    //     reset_control_assert(evea->rst_exiv);
    // err_out_reset:
    //     reset_control_assert(evea->rst);
    // err_out_clock_exiv:
    //     clk_disable_unprepare(evea->clk_exiv);
    // err_out_clock:
    //     clk_disable_unprepare(evea->clk);
    //
    // return ret;
    0
}

// soc_codec_evea structure with driver callbacks and widget/route declarations

// soc_dai_evea array with DAI driver descriptors

// evea_regmap_config structure
#[repr(C)]
pub struct EvéaRegmapConfig {
    pub reg_bits: u32,
    pub reg_stride: u32,
    pub val_bits: u32,
    pub max_register: u32,
    pub cache_type: u32,
}

// GFP_KERNEL constant (Linux kernel allocation flag)
const GFP_KERNEL: u32 = 0xd0;

unsafe extern "C" fn evea_probe(pdev: *mut core::ffi::c_void) -> i32 {
    // struct evea_priv *evea;
    // void __iomem *preg;
    // int ret;
    //
    // evea = devm_kzalloc(&pdev->dev, sizeof(struct evea_priv), GFP_KERNEL);
    // if (!evea)
    //     return -ENOMEM;
    //
    // evea->clk = devm_clk_get(&pdev->dev, "evea");
    // if (IS_ERR(evea->clk))
    //     return PTR_ERR(evea->clk);
    //
    // evea->clk_exiv = devm_clk_get(&pdev->dev, "exiv");
    // if (IS_ERR(evea->clk_exiv))
    //     return PTR_ERR(evea->clk_exiv);
    //
    // evea->rst = devm_reset_control_get_shared(&pdev->dev, "evea");
    // if (IS_ERR(evea->rst))
    //     return PTR_ERR(evea->rst);
    //
    // evea->rst_exiv = devm_reset_control_get_shared(&pdev->dev, "exiv");
    // if (IS_ERR(evea->rst_exiv))
    //     return PTR_ERR(evea->rst_exiv);
    //
    // preg = devm_platform_ioremap_resource(pdev, 0);
    // if (IS_ERR(preg))
    //     return PTR_ERR(preg);
    //
    // evea->regmap = devm_regmap_init_mmio(&pdev->dev, preg,
    //                                       &evea_regmap_config);
    // if (IS_ERR(evea->regmap))
    //     return PTR_ERR(evea->regmap);
    //
    // ret = clk_prepare_enable(evea->clk);
    // if (ret)
    //     return ret;
    //
    // ret = clk_prepare_enable(evea->clk_exiv);
    // if (ret)
    //     goto err_out_clock;
    //
    // ret = reset_control_deassert(evea->rst);
    // if (ret)
    //     goto err_out_clock_exiv;
    //
    // ret = reset_control_deassert(evea->rst_exiv);
    // if (ret)
    //     goto err_out_reset;
    //
    // evea->rst_adamv = devm_reset_control_get_shared(&pdev->dev, "adamv");
    // if (IS_ERR(evea->rst_adamv)) {
    //     ret = PTR_ERR(evea->rst_adamv);
    //     goto err_out_reset_exiv;
    // }
    //
    // ret = reset_control_deassert(evea->rst_adamv);
    // if (ret)
    //     goto err_out_reset_exiv;
    //
    // platform_set_drvdata(pdev, evea);
    //
    // ret = devm_snd_soc_register_component(&pdev->dev, &soc_codec_evea,
    //                      soc_dai_evea, ARRAY_SIZE(soc_dai_evea));
    // if (ret)
    //     goto err_out_reset_adamv;
    //
    // return 0;
    //
    // err_out_reset_adamv:
    //     reset_control_assert(evea->rst_adamv);
    // err_out_reset_exiv:
    //     reset_control_assert(evea->rst_exiv);
    // err_out_reset:
    //     reset_control_assert(evea->rst);
    // err_out_clock_exiv:
    //     clk_disable_unprepare(evea->clk_exiv);
    // err_out_clock:
    //     clk_disable_unprepare(evea->clk);
    //
    // return ret;
    0
}

unsafe extern "C" fn evea_remove(pdev: *mut core::ffi::c_void) {
    // struct evea_priv *evea = platform_get_drvdata(pdev);
    //
    // reset_control_assert(evea->rst_adamv);
    // reset_control_assert(evea->rst_exiv);
    // reset_control_assert(evea->rst);
    //
    // clk_disable_unprepare(evea->clk_exiv);
    // clk_disable_unprepare(evea->clk);
}

// evea_of_match array with device tree compatibility entries
// MODULE_DEVICE_TABLE(of, evea_of_match)

// evea_codec_driver platform_driver structure
// module_platform_driver(evea_codec_driver)

// MODULE_AUTHOR("Katsuhiro Suzuki <suzuki.katsuhiro@socionext.com>");
// MODULE_DESCRIPTION("UniPhier EVEA codec driver");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
