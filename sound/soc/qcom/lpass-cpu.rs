// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2010-2011,2013-2015 The Linux Foundation. All rights reserved.
 *
 * lpass-cpu.c -- ALSA SoC CPU DAI driver for QTi LPASS
 */

// Rust translation of implementation source. C includes are intentionally not
// executable Rust; referenced kernel/ASoC/LPASS symbols are external
// dependencies supplied by the surrounding repository.

const LPASS_CPU_MAX_MI2S_LINES: usize = 4;
const LPASS_CPU_I2S_SD0_MASK: u32 = BIT(0);
const LPASS_CPU_I2S_SD1_MASK: u32 = BIT(1);
const LPASS_CPU_I2S_SD2_MASK: u32 = BIT(2);
const LPASS_CPU_I2S_SD3_MASK: u32 = BIT(3);
const LPASS_CPU_I2S_SD0_1_MASK: u32 = GENMASK(1, 0);
const LPASS_CPU_I2S_SD2_3_MASK: u32 = GENMASK(3, 2);
const LPASS_CPU_I2S_SD0_1_2_MASK: u32 = GENMASK(2, 0);
const LPASS_CPU_I2S_SD0_1_2_3_MASK: u32 = GENMASK(3, 0);
const LPASS_REG_READ: bool = true;
const LPASS_REG_WRITE: bool = false;

/*
 * Channel maps for Quad channel playbacks on MI2S Secondary
 */
static mut lpass_quad_chmaps: [snd_pcm_chmap_elem; 2] = [
    snd_pcm_chmap_elem {
        channels: 4,
        map: [
            SNDRV_CHMAP_FL,
            SNDRV_CHMAP_RL,
            SNDRV_CHMAP_FR,
            SNDRV_CHMAP_RR,
        ],
    },
    snd_pcm_chmap_elem::default(),
];

unsafe fn lpass_cpu_init_i2sctl_bitfields(
    dev: *mut device,
    i2sctl: *mut lpaif_i2sctl,
    map: *mut regmap,
) -> c_int {
    let drvdata = dev_get_drvdata(dev) as *mut lpass_data;
    let v = (*drvdata).variant;

    (*i2sctl).loopback = devm_regmap_field_alloc(dev, map, (*v).loopback);
    (*i2sctl).spken = devm_regmap_field_alloc(dev, map, (*v).spken);
    (*i2sctl).spkmode = devm_regmap_field_alloc(dev, map, (*v).spkmode);
    (*i2sctl).spkmono = devm_regmap_field_alloc(dev, map, (*v).spkmono);
    (*i2sctl).micen = devm_regmap_field_alloc(dev, map, (*v).micen);
    (*i2sctl).micmode = devm_regmap_field_alloc(dev, map, (*v).micmode);
    (*i2sctl).micmono = devm_regmap_field_alloc(dev, map, (*v).micmono);
    (*i2sctl).wssrc = devm_regmap_field_alloc(dev, map, (*v).wssrc);
    (*i2sctl).bitwidth = devm_regmap_field_alloc(dev, map, (*v).bitwidth);

    if IS_ERR((*i2sctl).loopback)
        || IS_ERR((*i2sctl).spken)
        || IS_ERR((*i2sctl).spkmode)
        || IS_ERR((*i2sctl).spkmono)
        || IS_ERR((*i2sctl).micen)
        || IS_ERR((*i2sctl).micmode)
        || IS_ERR((*i2sctl).micmono)
        || IS_ERR((*i2sctl).wssrc)
        || IS_ERR((*i2sctl).bitwidth)
    {
        return -EINVAL;
    }

    0
}

unsafe fn lpass_cpu_daiops_set_sysclk(
    dai: *mut snd_soc_dai,
    _clk_id: c_int,
    freq: c_uint,
    _dir: c_int,
) -> c_int {
    let drvdata = snd_soc_dai_get_drvdata(dai) as *mut lpass_data;
    let ret = clk_set_rate((*drvdata).mi2s_osr_clk[(*(*dai).driver).id as usize], freq);
    if ret != 0 {
        dev_err(
            (*dai).dev,
            c_str!("error setting mi2s osrclk to %u: %d\n"),
            freq,
            ret,
        );
    }

    ret
}

unsafe fn lpass_cpu_daiops_startup(
    _substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> c_int {
    let drvdata = snd_soc_dai_get_drvdata(dai) as *mut lpass_data;
    let mut ret: c_int;

    ret = clk_prepare_enable((*drvdata).mi2s_osr_clk[(*(*dai).driver).id as usize]);
    if ret != 0 {
        dev_err((*dai).dev, c_str!("error in enabling mi2s osr clk: %d\n"), ret);
        return ret;
    }
    ret = clk_prepare((*drvdata).mi2s_bit_clk[(*(*dai).driver).id as usize]);
    if ret != 0 {
        dev_err((*dai).dev, c_str!("error in enabling mi2s bit clk: %d\n"), ret);
        clk_disable_unprepare((*drvdata).mi2s_osr_clk[(*(*dai).driver).id as usize]);
        return ret;
    }
    0
}

unsafe fn lpass_cpu_daiops_shutdown(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) {
    let drvdata = snd_soc_dai_get_drvdata(dai) as *mut lpass_data;
    let i2sctl = (*drvdata).i2sctl;
    let id = (*(*dai).driver).id as c_uint;

    clk_disable_unprepare((*drvdata).mi2s_osr_clk[(*(*dai).driver).id as usize]);
    /*
     * Ensure LRCLK is disabled even in device node validation.
     * Will not impact if disabled in lpass_cpu_daiops_trigger()
     * suspend.
     */
    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        regmap_fields_write((*i2sctl).spken, id, LPAIF_I2SCTL_SPKEN_DISABLE);
    } else {
        regmap_fields_write((*i2sctl).micen, id, LPAIF_I2SCTL_MICEN_DISABLE);
    }

    /*
     * BCLK may not be enabled if lpass_cpu_daiops_prepare is called before
     * lpass_cpu_daiops_shutdown. It's paired with the clk_enable in
     * lpass_cpu_daiops_prepare.
     */
    if (*drvdata).mi2s_was_prepared[(*(*dai).driver).id as usize] {
        (*drvdata).mi2s_was_prepared[(*(*dai).driver).id as usize] = false;
        clk_disable((*drvdata).mi2s_bit_clk[(*(*dai).driver).id as usize]);
    }

    clk_unprepare((*drvdata).mi2s_bit_clk[(*(*dai).driver).id as usize]);
}

unsafe fn lpass_cpu_daiops_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let drvdata = snd_soc_dai_get_drvdata(dai) as *mut lpass_data;
    let i2sctl = (*drvdata).i2sctl;
    let id = (*(*dai).driver).id as c_uint;
    let format: snd_pcm_format_t = params_format(params);
    let channels = params_channels(params);
    let rate = params_rate(params);
    let mut mode: c_uint;
    let regval: c_uint;
    let bitwidth = snd_pcm_format_width(format);
    let mut ret: c_int;

    if bitwidth < 0 {
        dev_err((*dai).dev, c_str!("invalid bit width given: %d\n"), bitwidth);
        return bitwidth;
    }

    ret = regmap_fields_write(i2sctl.loopback, id, LPAIF_I2SCTL_LOOPBACK_DISABLE);
    if ret != 0 {
        dev_err((*dai).dev, c_str!("error updating loopback field: %d\n"), ret);
        return ret;
    }

    ret = regmap_fields_write(i2sctl.wssrc, id, LPAIF_I2SCTL_WSSRC_INTERNAL);
    if ret != 0 {
        dev_err((*dai).dev, c_str!("error updating wssrc field: %d\n"), ret);
        return ret;
    }

    match bitwidth {
        16 => regval = LPAIF_I2SCTL_BITWIDTH_16,
        24 => regval = LPAIF_I2SCTL_BITWIDTH_24,
        32 => regval = LPAIF_I2SCTL_BITWIDTH_32,
        _ => {
            dev_err((*dai).dev, c_str!("invalid bitwidth given: %d\n"), bitwidth);
            return -EINVAL;
        }
    }

    ret = regmap_fields_write(i2sctl.bitwidth, id, regval);
    if ret != 0 {
        dev_err((*dai).dev, c_str!("error updating bitwidth field: %d\n"), ret);
        return ret;
    }

    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        mode = (*drvdata).mi2s_playback_sd_mode[id as usize];
    } else {
        mode = (*drvdata).mi2s_capture_sd_mode[id as usize];
    }

    if mode == 0 {
        dev_err((*dai).dev, c_str!("no line is assigned\n"));
        return -EINVAL;
    }

    match channels {
        1 | 2 => match mode {
            LPAIF_I2SCTL_MODE_QUAD01 | LPAIF_I2SCTL_MODE_6CH | LPAIF_I2SCTL_MODE_8CH => {
                mode = LPAIF_I2SCTL_MODE_SD0;
            }
            LPAIF_I2SCTL_MODE_QUAD23 => {
                mode = LPAIF_I2SCTL_MODE_SD2;
            }
            _ => {}
        },
        4 => {
            if mode < LPAIF_I2SCTL_MODE_QUAD01 {
                dev_err(
                    (*dai).dev,
                    c_str!("cannot configure 4 channels with mode %d\n"),
                    mode,
                );
                return -EINVAL;
            }

            match mode {
                LPAIF_I2SCTL_MODE_6CH | LPAIF_I2SCTL_MODE_8CH => mode = LPAIF_I2SCTL_MODE_QUAD01,
                _ => {}
            }
        }
        6 => {
            if mode < LPAIF_I2SCTL_MODE_6CH {
                dev_err(
                    (*dai).dev,
                    c_str!("cannot configure 6 channels with mode %d\n"),
                    mode,
                );
                return -EINVAL;
            }

            match mode {
                LPAIF_I2SCTL_MODE_8CH => mode = LPAIF_I2SCTL_MODE_6CH,
                _ => {}
            }
        }
        8 => {
            if mode < LPAIF_I2SCTL_MODE_8CH {
                dev_err(
                    (*dai).dev,
                    c_str!("cannot configure 8 channels with mode %d\n"),
                    mode,
                );
                return -EINVAL;
            }
        }
        _ => {
            dev_err((*dai).dev, c_str!("invalid channels given: %u\n"), channels);
            return -EINVAL;
        }
    }

    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        ret = regmap_fields_write(i2sctl.spkmode, id, LPAIF_I2SCTL_SPKMODE(mode));
        if ret != 0 {
            dev_err((*dai).dev, c_str!("error writing to i2sctl spkr mode: %d\n"), ret);
            return ret;
        }
        if channels >= 2 {
            ret = regmap_fields_write(i2sctl.spkmono, id, LPAIF_I2SCTL_SPKMONO_STEREO);
        } else {
            ret = regmap_fields_write(i2sctl.spkmono, id, LPAIF_I2SCTL_SPKMONO_MONO);
        }
    } else {
        ret = regmap_fields_write(i2sctl.micmode, id, LPAIF_I2SCTL_MICMODE(mode));
        if ret != 0 {
            dev_err((*dai).dev, c_str!("error writing to i2sctl mic mode: %d\n"), ret);
            return ret;
        }
        if channels >= 2 {
            ret = regmap_fields_write(i2sctl.micmono, id, LPAIF_I2SCTL_MICMONO_STEREO);
        } else {
            ret = regmap_fields_write(i2sctl.micmono, id, LPAIF_I2SCTL_MICMONO_MONO);
        }
    }

    if ret != 0 {
        dev_err((*dai).dev, c_str!("error writing to i2sctl channels mode: %d\n"), ret);
        return ret;
    }

    ret = clk_set_rate(
        (*drvdata).mi2s_bit_clk[id as usize],
        (rate as c_int * bitwidth * 2) as c_uint,
    );
    if ret != 0 {
        dev_err(
            (*dai).dev,
            c_str!("error setting mi2s bitclk to %u: %d\n"),
            (rate as c_int * bitwidth * 2) as c_uint,
            ret,
        );
        return ret;
    }

    0
}

unsafe fn lpass_cpu_daiops_trigger(
    substream: *mut snd_pcm_substream,
    cmd: c_int,
    dai: *mut snd_soc_dai,
) -> c_int {
    let drvdata = snd_soc_dai_get_drvdata(dai) as *mut lpass_data;
    let i2sctl = (*drvdata).i2sctl;
    let id = (*(*dai).driver).id as c_uint;
    let mut ret = -EINVAL;

    match cmd {
        SNDRV_PCM_TRIGGER_START | SNDRV_PCM_TRIGGER_RESUME | SNDRV_PCM_TRIGGER_PAUSE_RELEASE => {
            /*
             * Ensure lpass BCLK/LRCLK is enabled during
             * device resume as lpass_cpu_daiops_prepare() is not called
             * after the device resumes. We don't check mi2s_was_prepared before
             * enable/disable BCLK in trigger events because:
             *  1. These trigger events are paired, so the BCLK
             *     enable_count is balanced.
             *  2. the BCLK can be shared (ex: headset and headset mic),
             *     we need to increase the enable_count so that we don't
             *     turn off the shared BCLK while other devices are using
             *     it.
             */
            if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
                ret = regmap_fields_write(i2sctl.spken, id, LPAIF_I2SCTL_SPKEN_ENABLE);
            } else {
                ret = regmap_fields_write(i2sctl.micen, id, LPAIF_I2SCTL_MICEN_ENABLE);
            }
            if ret != 0 {
                dev_err((*dai).dev, c_str!("error writing to i2sctl reg: %d\n"), ret);
            }

            ret = clk_enable((*drvdata).mi2s_bit_clk[id as usize]);
            if ret != 0 {
                dev_err((*dai).dev, c_str!("error in enabling mi2s bit clk: %d\n"), ret);
                clk_disable((*drvdata).mi2s_osr_clk[id as usize]);
                return ret;
            }
        }
        SNDRV_PCM_TRIGGER_STOP | SNDRV_PCM_TRIGGER_SUSPEND | SNDRV_PCM_TRIGGER_PAUSE_PUSH => {
            /*
             * To ensure lpass BCLK/LRCLK is disabled during
             * device suspend.
             */
            if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
                ret = regmap_fields_write(i2sctl.spken, id, LPAIF_I2SCTL_SPKEN_DISABLE);
            } else {
                ret = regmap_fields_write(i2sctl.micen, id, LPAIF_I2SCTL_MICEN_DISABLE);
            }
            if ret != 0 {
                dev_err((*dai).dev, c_str!("error writing to i2sctl reg: %d\n"), ret);
            }

            clk_disable((*drvdata).mi2s_bit_clk[(*(*dai).driver).id as usize]);
        }
        _ => {}
    }

    ret
}

unsafe fn lpass_cpu_daiops_prepare(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> c_int {
    let drvdata = snd_soc_dai_get_drvdata(dai) as *mut lpass_data;
    let i2sctl = (*drvdata).i2sctl;
    let id = (*(*dai).driver).id as c_uint;
    let mut ret: c_int;

    /*
     * Ensure lpass BCLK/LRCLK is enabled bit before playback/capture
     * data flow starts. This allows other codec to have some delay before
     * the data flow.
     * (ex: to drop start up pop noise before capture starts).
     */
    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        ret = regmap_fields_write(i2sctl.spken, id, LPAIF_I2SCTL_SPKEN_ENABLE);
    } else {
        ret = regmap_fields_write(i2sctl.micen, id, LPAIF_I2SCTL_MICEN_ENABLE);
    }

    if ret != 0 {
        dev_err((*dai).dev, c_str!("error writing to i2sctl reg: %d\n"), ret);
        return ret;
    }

    /*
     * Check mi2s_was_prepared before enabling BCLK as lpass_cpu_daiops_prepare can
     * be called multiple times. It's paired with the clk_disable in
     * lpass_cpu_daiops_shutdown.
     */
    if !(*drvdata).mi2s_was_prepared[(*(*dai).driver).id as usize] {
        ret = clk_enable((*drvdata).mi2s_bit_clk[id as usize]);
        if ret != 0 {
            dev_err((*dai).dev, c_str!("error in enabling mi2s bit clk: %d\n"), ret);
            return ret;
        }
        (*drvdata).mi2s_was_prepared[(*(*dai).driver).id as usize] = true;
    }
    0
}

unsafe fn lpass_cpu_daiops_pcm_new(
    rtd: *mut snd_soc_pcm_runtime,
    dai: *mut snd_soc_dai,
) -> c_int {
    let mut ret: c_int;
    let drv = (*dai).driver;
    let drvdata = snd_soc_dai_get_drvdata(dai) as *mut lpass_data;

    if (*drvdata).mi2s_playback_sd_mode[(*dai).id as usize] == LPAIF_I2SCTL_MODE_QUAD01 {
        ret = snd_pcm_add_chmap_ctls(
            (*rtd).pcm,
            SNDRV_PCM_STREAM_PLAYBACK,
            lpass_quad_chmaps.as_mut_ptr(),
            (*drv).playback.channels_max,
            0,
            core::ptr::null_mut(),
        );
        if ret < 0 {
            return ret;
        }
    }

    0
}

unsafe fn lpass_cpu_daiops_probe(dai: *mut snd_soc_dai) -> c_int {
    let drvdata = snd_soc_dai_get_drvdata(dai) as *mut lpass_data;

    /* ensure audio hardware is disabled */
    let ret = regmap_write(
        (*drvdata).lpaif_map,
        LPAIF_I2SCTL_REG((*drvdata).variant, (*(*dai).driver).id),
        0,
    );
    if ret != 0 {
        dev_err((*dai).dev, c_str!("error writing to i2sctl reg: %d\n"), ret);
    }

    ret
}

pub static asoc_qcom_lpass_cpu_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    probe: Some(lpass_cpu_daiops_probe),
    set_sysclk: Some(lpass_cpu_daiops_set_sysclk),
    startup: Some(lpass_cpu_daiops_startup),
    shutdown: Some(lpass_cpu_daiops_shutdown),
    hw_params: Some(lpass_cpu_daiops_hw_params),
    trigger: Some(lpass_cpu_daiops_trigger),
    prepare: Some(lpass_cpu_daiops_prepare),
    ..snd_soc_dai_ops::default()
};
// EXPORT_SYMBOL_GPL(asoc_qcom_lpass_cpu_dai_ops);

pub static asoc_qcom_lpass_cpu_dai_ops2: snd_soc_dai_ops = snd_soc_dai_ops {
    pcm_new: Some(lpass_cpu_daiops_pcm_new),
    probe: Some(lpass_cpu_daiops_probe),
    set_sysclk: Some(lpass_cpu_daiops_set_sysclk),
    startup: Some(lpass_cpu_daiops_startup),
    shutdown: Some(lpass_cpu_daiops_shutdown),
    hw_params: Some(lpass_cpu_daiops_hw_params),
    trigger: Some(lpass_cpu_daiops_trigger),
    prepare: Some(lpass_cpu_daiops_prepare),
    ..snd_soc_dai_ops::default()
};
// EXPORT_SYMBOL_GPL(asoc_qcom_lpass_cpu_dai_ops2);

unsafe fn asoc_qcom_of_xlate_dai_name(
    component: *mut snd_soc_component,
    args: *const of_phandle_args,
    dai_name: *mut *const c_char,
) -> c_int {
    let drvdata = snd_soc_component_get_drvdata(component) as *mut lpass_data;
    let variant = (*drvdata).variant;
    let id = (*args).args[0] as c_int;
    let mut ret = -EINVAL;
    let mut i = 0;

    while i < (*variant).num_dai {
        if (*(*variant).dai_driver.add(i as usize)).id == id {
            *dai_name = (*(*variant).dai_driver.add(i as usize)).name;
            ret = 0;
            break;
        }
        i += 1;
    }

    ret
}

static lpass_cpu_comp_driver: snd_soc_component_driver = snd_soc_component_driver {
    name: c_str!("lpass-cpu"),
    of_xlate_dai_name: Some(asoc_qcom_of_xlate_dai_name),
    legacy_dai_naming: 1,
    ..snd_soc_component_driver::default()
};

unsafe fn lpass_cpu_regmap_writeable(dev: *mut device, reg: c_uint) -> bool {
    let drvdata = dev_get_drvdata(dev) as *mut lpass_data;
    let v = (*drvdata).variant;
    let mut i = 0;

    while i < (*v).i2s_ports {
        if reg == LPAIF_I2SCTL_REG(v, i) {
            return true;
        }
        i += 1;
    }

    i = 0;
    while i < (*v).irq_ports {
        if reg == LPAIF_IRQEN_REG(v, i) {
            return true;
        }
        if reg == LPAIF_IRQCLEAR_REG(v, i) {
            return true;
        }
        i += 1;
    }

    i = 0;
    while i < (*v).rdma_channels {
        if reg == LPAIF_RDMACTL_REG(v, i) {
            return true;
        }
        if reg == LPAIF_RDMABASE_REG(v, i) {
            return true;
        }
        if reg == LPAIF_RDMABUFF_REG(v, i) {
            return true;
        }
        if reg == LPAIF_RDMAPER_REG(v, i) {
            return true;
        }
        i += 1;
    }

    i = 0;
    while i < (*v).wrdma_channels {
        let ch = i + (*v).wrdma_channel_start;
        if reg == LPAIF_WRDMACTL_REG(v, ch) {
            return true;
        }
        if reg == LPAIF_WRDMABASE_REG(v, ch) {
            return true;
        }
        if reg == LPAIF_WRDMABUFF_REG(v, ch) {
            return true;
        }
        if reg == LPAIF_WRDMAPER_REG(v, ch) {
            return true;
        }
        i += 1;
    }

    false
}

unsafe fn lpass_cpu_regmap_readable(dev: *mut device, reg: c_uint) -> bool {
    let drvdata = dev_get_drvdata(dev) as *mut lpass_data;
    let v = (*drvdata).variant;
    let mut i = 0;

    while i < (*v).i2s_ports {
        if reg == LPAIF_I2SCTL_REG(v, i) {
            return true;
        }
        i += 1;
    }

    i = 0;
    while i < (*v).irq_ports {
        if reg == LPAIF_IRQCLEAR_REG(v, i) {
            return true;
        }
        if reg == LPAIF_IRQEN_REG(v, i) {
            return true;
        }
        if reg == LPAIF_IRQSTAT_REG(v, i) {
            return true;
        }
        i += 1;
    }

    i = 0;
    while i < (*v).rdma_channels {
        if reg == LPAIF_RDMACTL_REG(v, i) {
            return true;
        }
        if reg == LPAIF_RDMABASE_REG(v, i) {
            return true;
        }
        if reg == LPAIF_RDMABUFF_REG(v, i) {
            return true;
        }
        if reg == LPAIF_RDMACURR_REG(v, i) {
            return true;
        }
        if reg == LPAIF_RDMAPER_REG(v, i) {
            return true;
        }
        i += 1;
    }

    i = 0;
    while i < (*v).wrdma_channels {
        let ch = i + (*v).wrdma_channel_start;
        if reg == LPAIF_WRDMACTL_REG(v, ch) {
            return true;
        }
        if reg == LPAIF_WRDMABASE_REG(v, ch) {
            return true;
        }
        if reg == LPAIF_WRDMABUFF_REG(v, ch) {
            return true;
        }
        if reg == LPAIF_WRDMACURR_REG(v, ch) {
            return true;
        }
        if reg == LPAIF_WRDMAPER_REG(v, ch) {
            return true;
        }
        i += 1;
    }

    false
}

unsafe fn lpass_cpu_regmap_volatile(dev: *mut device, reg: c_uint) -> bool {
    let drvdata = dev_get_drvdata(dev) as *mut lpass_data;
    let v = (*drvdata).variant;
    let mut i = 0;

    while i < (*v).irq_ports {
        if reg == LPAIF_IRQCLEAR_REG(v, i) {
            return true;
        }
        if reg == LPAIF_IRQSTAT_REG(v, i) {
            return true;
        }
        i += 1;
    }

    i = 0;
    while i < (*v).rdma_channels {
        if reg == LPAIF_RDMACURR_REG(v, i) {
            return true;
        }
        i += 1;
    }

    i = 0;
    while i < (*v).wrdma_channels {
        if reg == LPAIF_WRDMACURR_REG(v, i + (*v).wrdma_channel_start) {
            return true;
        }
        i += 1;
    }

    false
}

static mut lpass_cpu_regmap_config: regmap_config = regmap_config {
    name: c_str!("lpass_cpu"),
    reg_bits: 32,
    reg_stride: 4,
    val_bits: 32,
    writeable_reg: Some(lpass_cpu_regmap_writeable),
    readable_reg: Some(lpass_cpu_regmap_readable),
    volatile_reg: Some(lpass_cpu_regmap_volatile),
    cache_type: REGCACHE_FLAT,
    ..regmap_config::default()
};

unsafe fn lpass_hdmi_init_bitfields(dev: *mut device, map: *mut regmap) -> c_int {
    let drvdata = dev_get_drvdata(dev) as *mut lpass_data;
    let v = (*drvdata).variant;
    let mut i: c_uint;
    let tx_ctl: *mut lpass_hdmi_tx_ctl;
    let mut legacy_en: *mut regmap_field = core::ptr::null_mut();
    let vbit_ctl: *mut lpass_vbit_ctrl;
    let mut tx_parity: *mut regmap_field = core::ptr::null_mut();
    let meta_ctl: *mut lpass_dp_metadata_ctl;
    let sstream_ctl: *mut lpass_sstream_ctl;
    let mut ch_msb: *mut regmap_field = core::ptr::null_mut();
    let mut ch_lsb: *mut regmap_field = core::ptr::null_mut();
    let mut tx_dmactl: *mut lpass_hdmitx_dmactl;
    let mut rval: c_int;

    tx_ctl = devm_kzalloc(dev, core::mem::size_of::<lpass_hdmi_tx_ctl>(), GFP_KERNEL) as *mut lpass_hdmi_tx_ctl;
    if tx_ctl.is_null() {
        return -ENOMEM;
    }

    QCOM_REGMAP_FIELD_ALLOC(dev, map, (*v).soft_reset, &mut (*tx_ctl).soft_reset);
    QCOM_REGMAP_FIELD_ALLOC(dev, map, (*v).force_reset, &mut (*tx_ctl).force_reset);
    (*drvdata).tx_ctl = tx_ctl;

    QCOM_REGMAP_FIELD_ALLOC(dev, map, (*v).legacy_en, &mut legacy_en);
    (*drvdata).hdmitx_legacy_en = legacy_en;

    vbit_ctl = devm_kzalloc(dev, core::mem::size_of::<lpass_vbit_ctrl>(), GFP_KERNEL) as *mut lpass_vbit_ctrl;
    if vbit_ctl.is_null() {
        return -ENOMEM;
    }

    QCOM_REGMAP_FIELD_ALLOC(dev, map, (*v).replace_vbit, &mut (*vbit_ctl).replace_vbit);
    QCOM_REGMAP_FIELD_ALLOC(dev, map, (*v).vbit_stream, &mut (*vbit_ctl).vbit_stream);
    (*drvdata).vbit_ctl = vbit_ctl;

    QCOM_REGMAP_FIELD_ALLOC(dev, map, (*v).calc_en, &mut tx_parity);
    (*drvdata).hdmitx_parity_calc_en = tx_parity;

    meta_ctl = devm_kzalloc(dev, core::mem::size_of::<lpass_dp_metadata_ctl>(), GFP_KERNEL) as *mut lpass_dp_metadata_ctl;
    if meta_ctl.is_null() {
        return -ENOMEM;
    }

    rval = devm_regmap_field_bulk_alloc(dev, map, &mut (*meta_ctl).mute, &(*v).mute, 7);
    if rval != 0 {
        return rval;
    }
    (*drvdata).meta_ctl = meta_ctl;

    sstream_ctl = devm_kzalloc(dev, core::mem::size_of::<lpass_sstream_ctl>(), GFP_KERNEL) as *mut lpass_sstream_ctl;
    if sstream_ctl.is_null() {
        return -ENOMEM;
    }

    rval = devm_regmap_field_bulk_alloc(dev, map, &mut (*sstream_ctl).sstream_en, &(*v).sstream_en, 9);
    if rval != 0 {
        return rval;
    }

    (*drvdata).sstream_ctl = sstream_ctl;

    i = 0;
    while i < LPASS_MAX_HDMI_DMA_CHANNELS {
        QCOM_REGMAP_FIELD_ALLOC(dev, map, (*v).msb_bits, &mut ch_msb);
        (*drvdata).hdmitx_ch_msb[i as usize] = ch_msb;

        QCOM_REGMAP_FIELD_ALLOC(dev, map, (*v).lsb_bits, &mut ch_lsb);
        (*drvdata).hdmitx_ch_lsb[i as usize] = ch_lsb;

        tx_dmactl = devm_kzalloc(dev, core::mem::size_of::<lpass_hdmitx_dmactl>(), GFP_KERNEL) as *mut lpass_hdmitx_dmactl;
        if tx_dmactl.is_null() {
            return -ENOMEM;
        }

        QCOM_REGMAP_FIELD_ALLOC(dev, map, (*v).use_hw_chs, &mut (*tx_dmactl).use_hw_chs);
        QCOM_REGMAP_FIELD_ALLOC(dev, map, (*v).use_hw_usr, &mut (*tx_dmactl).use_hw_usr);
        QCOM_REGMAP_FIELD_ALLOC(dev, map, (*v).hw_chs_sel, &mut (*tx_dmactl).hw_chs_sel);
        QCOM_REGMAP_FIELD_ALLOC(dev, map, (*v).hw_usr_sel, &mut (*tx_dmactl).hw_usr_sel);
        (*drvdata).hdmi_tx_dmactl[i as usize] = tx_dmactl;
        i += 1;
    }
    0
}

unsafe fn lpass_hdmi_regmap_writeable(dev: *mut device, reg: c_uint) -> bool {
    let drvdata = dev_get_drvdata(dev) as *mut lpass_data;
    let v = (*drvdata).variant;
    let mut i = 0;

    if reg == LPASS_HDMI_TX_CTL_ADDR(v) { return true; }
    if reg == LPASS_HDMI_TX_LEGACY_ADDR(v) { return true; }
    if reg == LPASS_HDMI_TX_VBIT_CTL_ADDR(v) { return true; }
    if reg == LPASS_HDMI_TX_PARITY_ADDR(v) { return true; }
    if reg == LPASS_HDMI_TX_DP_ADDR(v) { return true; }
    if reg == LPASS_HDMI_TX_SSTREAM_ADDR(v) { return true; }
    if reg == LPASS_HDMITX_APP_IRQEN_REG(v) { return true; }
    if reg == LPASS_HDMITX_APP_IRQCLEAR_REG(v) { return true; }

    while i < (*v).hdmi_rdma_channels {
        if reg == LPASS_HDMI_TX_CH_LSB_ADDR(v, i) { return true; }
        if reg == LPASS_HDMI_TX_CH_MSB_ADDR(v, i) { return true; }
        if reg == LPASS_HDMI_TX_DMA_ADDR(v, i) { return true; }
        i += 1;
    }

    i = 0;
    while i < (*v).hdmi_rdma_channels {
        if reg == LPAIF_HDMI_RDMACTL_REG(v, i) { return true; }
        if reg == LPAIF_HDMI_RDMABASE_REG(v, i) { return true; }
        if reg == LPAIF_HDMI_RDMABUFF_REG(v, i) { return true; }
        if reg == LPAIF_HDMI_RDMAPER_REG(v, i) { return true; }
        i += 1;
    }
    false
}

unsafe fn lpass_hdmi_regmap_readable(dev: *mut device, reg: c_uint) -> bool {
    let drvdata = dev_get_drvdata(dev) as *mut lpass_data;
    let v = (*drvdata).variant;
    let mut i = 0;

    if reg == LPASS_HDMI_TX_CTL_ADDR(v) { return true; }
    if reg == LPASS_HDMI_TX_LEGACY_ADDR(v) { return true; }
    if reg == LPASS_HDMI_TX_VBIT_CTL_ADDR(v) { return true; }

    while i < (*v).hdmi_rdma_channels {
        if reg == LPASS_HDMI_TX_CH_LSB_ADDR(v, i) { return true; }
        if reg == LPASS_HDMI_TX_CH_MSB_ADDR(v, i) { return true; }
        if reg == LPASS_HDMI_TX_DMA_ADDR(v, i) { return true; }
        i += 1;
    }

    if reg == LPASS_HDMI_TX_PARITY_ADDR(v) { return true; }
    if reg == LPASS_HDMI_TX_DP_ADDR(v) { return true; }
    if reg == LPASS_HDMI_TX_SSTREAM_ADDR(v) { return true; }
    if reg == LPASS_HDMITX_APP_IRQEN_REG(v) { return true; }
    if reg == LPASS_HDMITX_APP_IRQSTAT_REG(v) { return true; }

    i = 0;
    while i < (*v).hdmi_rdma_channels {
        if reg == LPAIF_HDMI_RDMACTL_REG(v, i) { return true; }
        if reg == LPAIF_HDMI_RDMABASE_REG(v, i) { return true; }
        if reg == LPAIF_HDMI_RDMABUFF_REG(v, i) { return true; }
        if reg == LPAIF_HDMI_RDMAPER_REG(v, i) { return true; }
        if reg == LPAIF_HDMI_RDMACURR_REG(v, i) { return true; }
        i += 1;
    }

    false
}

unsafe fn lpass_hdmi_regmap_volatile(dev: *mut device, reg: c_uint) -> bool {
    let drvdata = dev_get_drvdata(dev) as *mut lpass_data;
    let v = (*drvdata).variant;
    let mut i = 0;

    if reg == LPASS_HDMITX_APP_IRQSTAT_REG(v) { return true; }
    if reg == LPASS_HDMI_TX_LEGACY_ADDR(v) { return true; }
    if reg == LPASS_HDMI_TX_VBIT_CTL_ADDR(v) { return true; }
    if reg == LPASS_HDMI_TX_PARITY_ADDR(v) { return true; }

    while i < (*v).hdmi_rdma_channels {
        if reg == LPAIF_HDMI_RDMACURR_REG(v, i) { return true; }
        if reg == LPASS_HDMI_TX_DMA_ADDR(v, i) { return true; }
        if reg == LPASS_HDMI_TX_CH_LSB_ADDR(v, i) { return true; }
        if reg == LPASS_HDMI_TX_CH_MSB_ADDR(v, i) { return true; }
        i += 1;
    }
    false
}

static mut lpass_hdmi_regmap_config: regmap_config = regmap_config {
    name: c_str!("lpass_hdmi"),
    reg_bits: 32,
    reg_stride: 4,
    val_bits: 32,
    writeable_reg: Some(lpass_hdmi_regmap_writeable),
    readable_reg: Some(lpass_hdmi_regmap_readable),
    volatile_reg: Some(lpass_hdmi_regmap_volatile),
    cache_type: REGCACHE_FLAT,
    ..regmap_config::default()
};

unsafe fn __lpass_rxtx_regmap_accessible(dev: *mut device, reg: c_uint, rw: bool) -> bool {
    let drvdata = dev_get_drvdata(dev) as *mut lpass_data;
    let v = (*drvdata).variant;
    let mut i = 0;

    while i < (*v).rxtx_irq_ports {
        if reg == LPAIF_RXTX_IRQCLEAR_REG(v, i) { return true; }
        if reg == LPAIF_RXTX_IRQEN_REG(v, i) { return true; }
        if reg == LPAIF_RXTX_IRQSTAT_REG(v, i) { return true; }
        i += 1;
    }

    i = 0;
    while i < (*v).rxtx_rdma_channels {
        if reg == LPAIF_CDC_RXTX_RDMACTL_REG(v, i, LPASS_CDC_DMA_RX0) { return true; }
        if reg == LPAIF_CDC_RXTX_RDMABASE_REG(v, i, LPASS_CDC_DMA_RX0) { return true; }
        if reg == LPAIF_CDC_RXTX_RDMABUFF_REG(v, i, LPASS_CDC_DMA_RX0) { return true; }
        if rw == LPASS_REG_READ {
            if reg == LPAIF_CDC_RXTX_RDMACURR_REG(v, i, LPASS_CDC_DMA_RX0) {
                return true;
            }
        }
        if reg == LPAIF_CDC_RXTX_RDMAPER_REG(v, i, LPASS_CDC_DMA_RX0) { return true; }
        if reg == LPAIF_CDC_RXTX_RDMA_INTF_REG(v, i, LPASS_CDC_DMA_RX0) { return true; }
        i += 1;
    }

    i = 0;
    while i < (*v).rxtx_wrdma_channels {
        let ch = i + (*v).rxtx_wrdma_channel_start;
        if reg == LPAIF_CDC_RXTX_WRDMACTL_REG(v, ch, LPASS_CDC_DMA_TX3) { return true; }
        if reg == LPAIF_CDC_RXTX_WRDMABASE_REG(v, ch, LPASS_CDC_DMA_TX3) { return true; }
        if reg == LPAIF_CDC_RXTX_WRDMABUFF_REG(v, ch, LPASS_CDC_DMA_TX3) { return true; }
        if rw == LPASS_REG_READ {
            if reg == LPAIF_CDC_RXTX_WRDMACURR_REG(v, i, LPASS_CDC_DMA_RX0) {
                return true;
            }
        }
        if reg == LPAIF_CDC_RXTX_WRDMAPER_REG(v, ch, LPASS_CDC_DMA_TX3) { return true; }
        if reg == LPAIF_CDC_RXTX_WRDMA_INTF_REG(v, ch, LPASS_CDC_DMA_TX3) { return true; }
        i += 1;
    }
    false
}

unsafe fn lpass_rxtx_regmap_writeable(dev: *mut device, reg: c_uint) -> bool {
    __lpass_rxtx_regmap_accessible(dev, reg, LPASS_REG_WRITE)
}

unsafe fn lpass_rxtx_regmap_readable(dev: *mut device, reg: c_uint) -> bool {
    __lpass_rxtx_regmap_accessible(dev, reg, LPASS_REG_READ)
}

unsafe fn lpass_rxtx_regmap_volatile(dev: *mut device, reg: c_uint) -> bool {
    let drvdata = dev_get_drvdata(dev) as *mut lpass_data;
    let v = (*drvdata).variant;
    let mut i = 0;

    while i < (*v).rxtx_irq_ports {
        if reg == LPAIF_RXTX_IRQCLEAR_REG(v, i) { return true; }
        if reg == LPAIF_RXTX_IRQSTAT_REG(v, i) { return true; }
        i += 1;
    }

    i = 0;
    while i < (*v).rxtx_rdma_channels {
        if reg == LPAIF_CDC_RXTX_RDMACURR_REG(v, i, LPASS_CDC_DMA_RX0) { return true; }
        i += 1;
    }

    i = 0;
    while i < (*v).rxtx_wrdma_channels {
        if reg == LPAIF_CDC_RXTX_WRDMACURR_REG(v, i + (*v).rxtx_wrdma_channel_start, LPASS_CDC_DMA_TX3) {
            return true;
        }
        i += 1;
    }

    false
}

unsafe fn __lpass_va_regmap_accessible(dev: *mut device, reg: c_uint, rw: bool) -> bool {
    let drvdata = dev_get_drvdata(dev) as *mut lpass_data;
    let v = (*drvdata).variant;
    let mut i = 0;

    while i < (*v).va_irq_ports {
        if reg == LPAIF_VA_IRQCLEAR_REG(v, i) { return true; }
        if reg == LPAIF_VA_IRQEN_REG(v, i) { return true; }
        if reg == LPAIF_VA_IRQSTAT_REG(v, i) { return true; }
        i += 1;
    }

    i = 0;
    while i < (*v).va_wrdma_channels {
        let ch = i + (*v).va_wrdma_channel_start;
        if reg == LPAIF_CDC_VA_WRDMACTL_REG(v, ch, LPASS_CDC_DMA_VA_TX0) { return true; }
        if reg == LPAIF_CDC_VA_WRDMABASE_REG(v, ch, LPASS_CDC_DMA_VA_TX0) { return true; }
        if reg == LPAIF_CDC_VA_WRDMABUFF_REG(v, ch, LPASS_CDC_DMA_VA_TX0) { return true; }
        if rw == LPASS_REG_READ {
            if reg == LPAIF_CDC_VA_WRDMACURR_REG(v, ch, LPASS_CDC_DMA_VA_TX0) {
                return true;
            }
        }
        if reg == LPAIF_CDC_VA_WRDMAPER_REG(v, ch, LPASS_CDC_DMA_VA_TX0) { return true; }
        if reg == LPAIF_CDC_VA_WRDMA_INTF_REG(v, ch, LPASS_CDC_DMA_VA_TX0) { return true; }
        i += 1;
    }
    false
}

unsafe fn lpass_va_regmap_writeable(dev: *mut device, reg: c_uint) -> bool {
    __lpass_va_regmap_accessible(dev, reg, LPASS_REG_WRITE)
}

unsafe fn lpass_va_regmap_readable(dev: *mut device, reg: c_uint) -> bool {
    __lpass_va_regmap_accessible(dev, reg, LPASS_REG_READ)
}

unsafe fn lpass_va_regmap_volatile(dev: *mut device, reg: c_uint) -> bool {
    let drvdata = dev_get_drvdata(dev) as *mut lpass_data;
    let v = (*drvdata).variant;
    let mut i = 0;

    while i < (*v).va_irq_ports {
        if reg == LPAIF_VA_IRQCLEAR_REG(v, i) { return true; }
        if reg == LPAIF_VA_IRQSTAT_REG(v, i) { return true; }
        i += 1;
    }

    i = 0;
    while i < (*v).va_wrdma_channels {
        if reg == LPAIF_CDC_VA_WRDMACURR_REG(v, i + (*v).va_wrdma_channel_start, LPASS_CDC_DMA_VA_TX0) {
            return true;
        }
        i += 1;
    }

    false
}

static mut lpass_rxtx_regmap_config: regmap_config = regmap_config {
    reg_bits: 32,
    reg_stride: 4,
    val_bits: 32,
    writeable_reg: Some(lpass_rxtx_regmap_writeable),
    readable_reg: Some(lpass_rxtx_regmap_readable),
    volatile_reg: Some(lpass_rxtx_regmap_volatile),
    cache_type: REGCACHE_FLAT,
    ..regmap_config::default()
};

static mut lpass_va_regmap_config: regmap_config = regmap_config {
    reg_bits: 32,
    reg_stride: 4,
    val_bits: 32,
    writeable_reg: Some(lpass_va_regmap_writeable),
    readable_reg: Some(lpass_va_regmap_readable),
    volatile_reg: Some(lpass_va_regmap_volatile),
    cache_type: REGCACHE_FLAT,
    ..regmap_config::default()
};

unsafe fn of_lpass_cpu_parse_sd_lines(
    dev: *mut device,
    node: *mut device_node,
    name: *const c_char,
) -> c_uint {
    let mut lines = [0u32; LPASS_CPU_MAX_MI2S_LINES];
    let mut sd_line_mask: c_uint = 0;
    let num_lines: c_int;
    let mut i: c_int;

    num_lines = of_property_read_variable_u32_array(
        node,
        name,
        lines.as_mut_ptr(),
        0,
        LPASS_CPU_MAX_MI2S_LINES,
    );
    if num_lines < 0 {
        return LPAIF_I2SCTL_MODE_NONE;
    }

    i = 0;
    while i < num_lines {
        sd_line_mask |= BIT(lines[i as usize]);
        i += 1;
    }

    match sd_line_mask {
        LPASS_CPU_I2S_SD0_MASK => LPAIF_I2SCTL_MODE_SD0,
        LPASS_CPU_I2S_SD1_MASK => LPAIF_I2SCTL_MODE_SD1,
        LPASS_CPU_I2S_SD2_MASK => LPAIF_I2SCTL_MODE_SD2,
        LPASS_CPU_I2S_SD3_MASK => LPAIF_I2SCTL_MODE_SD3,
        LPASS_CPU_I2S_SD0_1_MASK => LPAIF_I2SCTL_MODE_QUAD01,
        LPASS_CPU_I2S_SD2_3_MASK => LPAIF_I2SCTL_MODE_QUAD23,
        LPASS_CPU_I2S_SD0_1_2_MASK => LPAIF_I2SCTL_MODE_6CH,
        LPASS_CPU_I2S_SD0_1_2_3_MASK => LPAIF_I2SCTL_MODE_8CH,
        _ => {
            dev_err(dev, c_str!("Unsupported SD line mask: %#x\n"), sd_line_mask);
            LPAIF_I2SCTL_MODE_NONE
        }
    }
}

unsafe fn of_lpass_cpu_parse_dai_data(dev: *mut device, data: *mut lpass_data) {
    let mut node: *mut device_node;
    let mut ret: c_int;
    let mut i: c_int;
    let mut id: c_int = 0;

    /* Allow all channels by default for backwards compatibility */
    i = 0;
    while i < (*(*data).variant).num_dai {
        id = (*(*(*data).variant).dai_driver.add(i as usize)).id;
        (*data).mi2s_playback_sd_mode[id as usize] = LPAIF_I2SCTL_MODE_8CH;
        (*data).mi2s_capture_sd_mode[id as usize] = LPAIF_I2SCTL_MODE_8CH;
        i += 1;
    }

    node = core::ptr::null_mut();
    while for_each_child_of_node((*dev).of_node, &mut node) {
        ret = of_property_read_u32(node, c_str!("reg"), &mut id);
        if ret != 0 || id < 0 {
            dev_err(dev, c_str!("valid dai id not found: %d\n"), ret);
            continue;
        }
        if id == LPASS_DP_RX {
            (*data).hdmi_port_enable = 1;
        } else if is_cdc_dma_port(id) {
            (*data).codec_dma_enable = 1;
        } else {
            (*data).mi2s_playback_sd_mode[id as usize] =
                of_lpass_cpu_parse_sd_lines(dev, node, c_str!("qcom,playback-sd-lines"));
            (*data).mi2s_capture_sd_mode[id as usize] =
                of_lpass_cpu_parse_sd_lines(dev, node, c_str!("qcom,capture-sd-lines"));
        }
    }
}

unsafe fn of_lpass_cdc_dma_clks_parse(dev: *mut device, data: *mut lpass_data) -> c_int {
    (*data).codec_mem0 = devm_clk_get(dev, c_str!("audio_cc_codec_mem0"));
    if IS_ERR((*data).codec_mem0) {
        return PTR_ERR((*data).codec_mem0) as c_int;
    }

    (*data).codec_mem1 = devm_clk_get(dev, c_str!("audio_cc_codec_mem1"));
    if IS_ERR((*data).codec_mem1) {
        return PTR_ERR((*data).codec_mem1) as c_int;
    }

    (*data).codec_mem2 = devm_clk_get(dev, c_str!("audio_cc_codec_mem2"));
    if IS_ERR((*data).codec_mem2) {
        return PTR_ERR((*data).codec_mem2) as c_int;
    }

    (*data).va_mem0 = devm_clk_get(dev, c_str!("aon_cc_va_mem0"));
    if IS_ERR((*data).va_mem0) {
        return PTR_ERR((*data).va_mem0) as c_int;
    }

    0
}

pub unsafe fn asoc_qcom_lpass_cpu_platform_probe(pdev: *mut platform_device) -> c_int {
    let drvdata: *mut lpass_data;
    let dsp_of_node: *mut device_node;
    let mut res: *mut resource;
    let variant: *const lpass_variant;
    let dev = &mut (*pdev).dev as *mut device;
    let mut ret: c_int;
    let mut i: c_int;
    let mut dai_id: c_int;

    dsp_of_node = of_parse_phandle((*pdev).dev.of_node, c_str!("qcom,adsp"), 0);
    if !dsp_of_node.is_null() {
        dev_err(dev, c_str!("DSP exists and holds audio resources\n"));
        of_node_put(dsp_of_node);
        return -EBUSY;
    }

    drvdata = devm_kzalloc(dev, core::mem::size_of::<lpass_data>(), GFP_KERNEL) as *mut lpass_data;
    if drvdata.is_null() {
        return -ENOMEM;
    }
    platform_set_drvdata(pdev, drvdata as *mut c_void);

    variant = device_get_match_data(dev) as *const lpass_variant;
    if variant.is_null() {
        return -EINVAL;
    }

    if of_device_is_compatible((*dev).of_node, c_str!("qcom,lpass-cpu-apq8016")) != 0 {
        dev_warn(dev, c_str!("qcom,lpass-cpu-apq8016 compatible is deprecated\n"));
    }

    (*drvdata).variant = variant;

    of_lpass_cpu_parse_dai_data(dev, drvdata);

    if (*drvdata).codec_dma_enable != 0 {
        (*drvdata).rxtx_lpaif = devm_platform_ioremap_resource_byname(pdev, c_str!("lpass-rxtx-lpaif"));
        if IS_ERR((*drvdata).rxtx_lpaif) {
            return PTR_ERR((*drvdata).rxtx_lpaif) as c_int;
        }

        (*drvdata).va_lpaif = devm_platform_ioremap_resource_byname(pdev, c_str!("lpass-va-lpaif"));
        if IS_ERR((*drvdata).va_lpaif) {
            return PTR_ERR((*drvdata).va_lpaif) as c_int;
        }

        lpass_rxtx_regmap_config.max_register = LPAIF_CDC_RXTX_WRDMAPER_REG(
            variant,
            (*variant).rxtx_wrdma_channels + (*variant).rxtx_wrdma_channel_start,
            LPASS_CDC_DMA_TX3,
        );

        (*drvdata).rxtx_lpaif_map = devm_regmap_init_mmio(
            dev,
            (*drvdata).rxtx_lpaif,
            &mut lpass_rxtx_regmap_config,
        );
        if IS_ERR((*drvdata).rxtx_lpaif_map) {
            return PTR_ERR((*drvdata).rxtx_lpaif_map) as c_int;
        }

        lpass_va_regmap_config.max_register = LPAIF_CDC_VA_WRDMAPER_REG(
            variant,
            (*variant).va_wrdma_channels + (*variant).va_wrdma_channel_start,
            LPASS_CDC_DMA_VA_TX0,
        );

        (*drvdata).va_lpaif_map = devm_regmap_init_mmio(
            dev,
            (*drvdata).va_lpaif,
            &mut lpass_va_regmap_config,
        );
        if IS_ERR((*drvdata).va_lpaif_map) {
            return PTR_ERR((*drvdata).va_lpaif_map) as c_int;
        }

        ret = of_lpass_cdc_dma_clks_parse(dev, drvdata);
        if ret != 0 {
            dev_err(dev, c_str!("failed to get cdc dma clocks %d\n"), ret);
            return ret;
        }

        res = platform_get_resource_byname(pdev, IORESOURCE_MEM, c_str!("lpass-rxtx-cdc-dma-lpm"));
        if res.is_null() {
            return -EINVAL;
        }
        (*drvdata).rxtx_cdc_dma_lpm_buf = (*res).start;

        res = platform_get_resource_byname(pdev, IORESOURCE_MEM, c_str!("lpass-va-cdc-dma-lpm"));
        if res.is_null() {
            return -EINVAL;
        }
        (*drvdata).va_cdc_dma_lpm_buf = (*res).start;
    }

    (*drvdata).lpaif = devm_platform_ioremap_resource_byname(pdev, c_str!("lpass-lpaif"));
    if IS_ERR((*drvdata).lpaif) {
        return PTR_ERR((*drvdata).lpaif) as c_int;
    }

    lpass_cpu_regmap_config.max_register = LPAIF_WRDMAPER_REG(
        variant,
        (*variant).wrdma_channels + (*variant).wrdma_channel_start,
    );

    (*drvdata).lpaif_map = devm_regmap_init_mmio(dev, (*drvdata).lpaif, &mut lpass_cpu_regmap_config);
    if IS_ERR((*drvdata).lpaif_map) {
        dev_err(
            dev,
            c_str!("error initializing regmap: %ld\n"),
            PTR_ERR((*drvdata).lpaif_map),
        );
        return PTR_ERR((*drvdata).lpaif_map) as c_int;
    }

    if (*drvdata).hdmi_port_enable != 0 {
        (*drvdata).hdmiif = devm_platform_ioremap_resource_byname(pdev, c_str!("lpass-hdmiif"));
        if IS_ERR((*drvdata).hdmiif) {
            return PTR_ERR((*drvdata).hdmiif) as c_int;
        }

        lpass_hdmi_regmap_config.max_register =
            LPAIF_HDMI_RDMAPER_REG(variant, (*variant).hdmi_rdma_channels - 1);
        (*drvdata).hdmiif_map = devm_regmap_init_mmio(
            dev,
            (*drvdata).hdmiif,
            &mut lpass_hdmi_regmap_config,
        );
        if IS_ERR((*drvdata).hdmiif_map) {
            dev_err(
                dev,
                c_str!("error initializing regmap: %ld\n"),
                PTR_ERR((*drvdata).hdmiif_map),
            );
            return PTR_ERR((*drvdata).hdmiif_map) as c_int;
        }
    }

    if (*variant).init.is_some() {
        ret = ((*variant).init.unwrap())(pdev);
        if ret != 0 {
            dev_err(dev, c_str!("error initializing variant: %d\n"), ret);
            return ret;
        }
    }

    i = 0;
    while i < (*variant).num_dai {
        dai_id = (*(*variant).dai_driver.add(i as usize)).id;
        if dai_id == LPASS_DP_RX || is_cdc_dma_port(dai_id) {
            i += 1;
            continue;
        }

        (*drvdata).mi2s_osr_clk[dai_id as usize] =
            devm_clk_get_optional(dev, *(*variant).dai_osr_clk_names.add(i as usize));
        (*drvdata).mi2s_bit_clk[dai_id as usize] =
            devm_clk_get(dev, *(*variant).dai_bit_clk_names.add(i as usize));
        if IS_ERR((*drvdata).mi2s_bit_clk[dai_id as usize]) {
            dev_err(
                dev,
                c_str!("error getting %s: %ld\n"),
                *(*variant).dai_bit_clk_names.add(i as usize),
                PTR_ERR((*drvdata).mi2s_bit_clk[dai_id as usize]),
            );
            return PTR_ERR((*drvdata).mi2s_bit_clk[dai_id as usize]) as c_int;
        }
        if (*drvdata).mi2s_playback_sd_mode[dai_id as usize] == LPAIF_I2SCTL_MODE_QUAD01 {
            (*(*variant).dai_driver.add(dai_id as usize)).playback.channels_min = 4;
            (*(*variant).dai_driver.add(dai_id as usize)).playback.channels_max = 4;
        }
        i += 1;
    }

    /* Allocation for i2sctl regmap fields */
    (*drvdata).i2sctl = devm_kzalloc(
        &mut (*pdev).dev,
        core::mem::size_of::<lpaif_i2sctl>(),
        GFP_KERNEL,
    ) as *mut lpaif_i2sctl;
    if (*drvdata).i2sctl.is_null() {
        return -ENOMEM;
    }

    /* Initialize bitfields for dai I2SCTL register */
    ret = lpass_cpu_init_i2sctl_bitfields(dev, (*drvdata).i2sctl, (*drvdata).lpaif_map);
    if ret != 0 {
        dev_err(dev, c_str!("error init i2sctl field: %d\n"), ret);
        return ret;
    }

    if (*drvdata).hdmi_port_enable != 0 {
        ret = lpass_hdmi_init_bitfields(dev, (*drvdata).hdmiif_map);
        if ret != 0 {
            dev_err(dev, c_str!("%s error  hdmi init failed\n"), __func__);
            return ret;
        }
    }
    ret = devm_snd_soc_register_component(
        dev,
        &lpass_cpu_comp_driver,
        (*variant).dai_driver,
        (*variant).num_dai,
    );
    if ret != 0 {
        dev_err(dev, c_str!("error registering cpu driver: %d\n"), ret);
        return ret;
    }

    ret = asoc_qcom_lpass_platform_register(pdev);
    if ret != 0 {
        dev_err(dev, c_str!("error registering platform driver: %d\n"), ret);
        return ret;
    }

    ret
}
// EXPORT_SYMBOL_GPL(asoc_qcom_lpass_cpu_platform_probe);

pub unsafe fn asoc_qcom_lpass_cpu_platform_remove(pdev: *mut platform_device) {
    let drvdata = platform_get_drvdata(pdev) as *mut lpass_data;

    if (*(*drvdata).variant).exit.is_some() {
        ((*(*drvdata).variant).exit.unwrap())(pdev);
    }
}
// EXPORT_SYMBOL_GPL(asoc_qcom_lpass_cpu_platform_remove);

pub unsafe fn asoc_qcom_lpass_cpu_platform_shutdown(pdev: *mut platform_device) {
    let drvdata = platform_get_drvdata(pdev) as *mut lpass_data;

    if (*(*drvdata).variant).exit.is_some() {
        ((*(*drvdata).variant).exit.unwrap())(pdev);
    }
}
// EXPORT_SYMBOL_GPL(asoc_qcom_lpass_cpu_platform_shutdown);

// MODULE_DESCRIPTION("QTi LPASS CPU Driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
