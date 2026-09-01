// SPDX-License-Identifier: GPL-2.0-only
/*
 * tas2552.c - ALSA SoC Texas Instruments TAS2552 Mono Audio Amplifier
 *
 * Copyright (C) 2014 - 2024 Texas Instruments Incorporated -
 *	https://www.ti.com
 *
 * Author: Dan Murphy <dmurphy@ti.com>
 */

/* Translated from C. Kernel, ALSA SoC, regmap, GPIO, regulator, PM, DT,
 * and TAS2552 register symbols are external dependencies supplied by the
 * surrounding repository.
 */

const TAS2552_NUM_SUPPLIES: usize = 3;

static tas2552_reg_defs: [reg_default; 21] = [
    reg_default { reg: TAS2552_CFG_1, def: 0x22 },
    reg_default { reg: TAS2552_CFG_2, def: 0xef },
    reg_default { reg: TAS2552_CFG_3, def: 0x80 },
    reg_default { reg: TAS2552_DOUT, def: 0x00 },
    reg_default { reg: TAS2552_SER_CTRL_1, def: 0x00 },
    reg_default { reg: TAS2552_SER_CTRL_2, def: 0x00 },
    reg_default { reg: TAS2552_OUTPUT_DATA, def: 0xc0 },
    reg_default { reg: TAS2552_PLL_CTRL_1, def: 0x10 },
    reg_default { reg: TAS2552_PLL_CTRL_2, def: 0x00 },
    reg_default { reg: TAS2552_PLL_CTRL_3, def: 0x00 },
    reg_default { reg: TAS2552_BTIP, def: 0x8f },
    reg_default { reg: TAS2552_BTS_CTRL, def: 0x80 },
    reg_default { reg: TAS2552_RESERVED_0D, def: 0xbe },
    reg_default { reg: TAS2552_LIMIT_RATE_HYS, def: 0x08 },
    reg_default { reg: TAS2552_LIMIT_RELEASE, def: 0x04 },
    reg_default { reg: TAS2552_LIMIT_INT_COUNT, def: 0x00 },
    reg_default { reg: TAS2552_PDM_CFG, def: 0x01 },
    reg_default { reg: TAS2552_PGA_GAIN, def: 0x00 },
    reg_default { reg: TAS2552_EDGE_RATE_CTRL, def: 0x40 },
    reg_default { reg: TAS2552_BOOST_APT_CTRL, def: 0x0f },
    reg_default { reg: TAS2552_VBAT_DATA, def: 0x00 },
];

static tas2552_supply_names: [*const c_char; TAS2552_NUM_SUPPLIES] = [
    c"vbat".as_ptr(),  /* vbat voltage */
    c"iovdd".as_ptr(), /* I/O Voltage */
    c"avdd".as_ptr(),  /* Analog DAC Voltage */
];

#[repr(C)]
struct tas2552_data {
    component: *mut snd_soc_component,
    regmap: *mut regmap,
    tas2552_client: *mut i2c_client,
    supplies: [regulator_bulk_data; TAS2552_NUM_SUPPLIES],
    enable_gpio: *mut gpio_desc,
    regs: [u8; TAS2552_VBAT_DATA as usize],
    pll_clkin: c_uint,
    pll_clk_id: c_int,
    pdm_clk: c_uint,
    pdm_clk_id: c_int,

    dai_fmt: c_uint,
    tdm_delay: c_uint,
}

unsafe extern "C" fn tas2552_post_event(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);

    match event {
        SND_SOC_DAPM_POST_PMU => {
            snd_soc_component_write(component, TAS2552_RESERVED_0D, 0xc0);
            snd_soc_component_update_bits(component, TAS2552_LIMIT_RATE_HYS, 1 << 5, 1 << 5);
            snd_soc_component_update_bits(component, TAS2552_CFG_2, 1, 0);
            snd_soc_component_update_bits(component, TAS2552_CFG_1, TAS2552_SWS, 0);
        }
        SND_SOC_DAPM_POST_PMD => {
            snd_soc_component_update_bits(component, TAS2552_CFG_1, TAS2552_SWS, TAS2552_SWS);
            snd_soc_component_update_bits(component, TAS2552_CFG_2, 1, 1);
            snd_soc_component_update_bits(component, TAS2552_LIMIT_RATE_HYS, 1 << 5, 0);
            snd_soc_component_write(component, TAS2552_RESERVED_0D, 0xbe);
        }
        _ => {}
    }
    0
}

/* Input mux controls */
static tas2552_input_texts: [*const c_char; 2] = [c"Digital".as_ptr(), c"Analog".as_ptr()];
static tas2552_input_mux_enum: soc_enum =
    SOC_ENUM_SINGLE_DECL(TAS2552_CFG_3, 7, &tas2552_input_texts);

static tas2552_input_mux_control: snd_kcontrol_new =
    SOC_DAPM_ENUM(c"Route".as_ptr(), &tas2552_input_mux_enum);

static tas2552_dapm_widgets: [snd_soc_dapm_widget; 10] = [
    SND_SOC_DAPM_INPUT(c"IN".as_ptr()),

    /* MUX Controls */
    SND_SOC_DAPM_MUX(
        c"Input selection".as_ptr(),
        SND_SOC_NOPM,
        0,
        0,
        &tas2552_input_mux_control,
    ),

    SND_SOC_DAPM_AIF_IN(c"DAC IN".as_ptr(), c"DAC Playback".as_ptr(), 0, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_AIF_OUT(c"ASI OUT".as_ptr(), c"DAC Capture".as_ptr(), 0, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_DAC(c"DAC".as_ptr(), core::ptr::null(), SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_OUT_DRV(c"ClassD".as_ptr(), TAS2552_CFG_2, 7, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_SUPPLY(c"PLL".as_ptr(), TAS2552_CFG_2, 3, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_POST(c"Post Event".as_ptr(), Some(tas2552_post_event)),

    SND_SOC_DAPM_OUTPUT(c"OUT".as_ptr()),
    SND_SOC_DAPM_INPUT(c"DMIC".as_ptr()),
];

static tas2552_audio_map: [snd_soc_dapm_route; 7] = [
    snd_soc_dapm_route { sink: c"DAC".as_ptr(), control: core::ptr::null(), source: c"DAC IN".as_ptr() },
    snd_soc_dapm_route { sink: c"Input selection".as_ptr(), control: c"Digital".as_ptr(), source: c"DAC".as_ptr() },
    snd_soc_dapm_route { sink: c"Input selection".as_ptr(), control: c"Analog".as_ptr(), source: c"IN".as_ptr() },
    snd_soc_dapm_route { sink: c"ClassD".as_ptr(), control: core::ptr::null(), source: c"Input selection".as_ptr() },
    snd_soc_dapm_route { sink: c"OUT".as_ptr(), control: core::ptr::null(), source: c"ClassD".as_ptr() },
    snd_soc_dapm_route { sink: c"ClassD".as_ptr(), control: core::ptr::null(), source: c"PLL".as_ptr() },
    snd_soc_dapm_route { sink: c"ASI OUT".as_ptr(), control: core::ptr::null(), source: c"DMIC".as_ptr() },
];

unsafe fn tas2552_sw_shutdown(tas2552: *mut tas2552_data, sw_shutdown: c_int) {
    let mut cfg1_reg: u8 = 0;

    if (*tas2552).component.is_null() {
        return;
    }

    if sw_shutdown != 0 {
        cfg1_reg = TAS2552_SWS as u8;
    }

    snd_soc_component_update_bits(
        (*tas2552).component,
        TAS2552_CFG_1,
        TAS2552_SWS,
        cfg1_reg as c_uint,
    );
}

unsafe fn tas2552_setup_pll(
    component: *mut snd_soc_component,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    let tas2552 = dev_get_drvdata((*component).dev) as *mut tas2552_data;
    let mut bypass_pll = false;
    let pll_clk: c_uint = params_rate(params) * 512;
    let mut pll_clkin: c_uint = (*tas2552).pll_clkin;
    let pll_enable: u8;

    if pll_clkin == 0 {
        if (*tas2552).pll_clk_id != TAS2552_PLL_CLKIN_BCLK {
            return -EINVAL;
        }

        pll_clkin = snd_soc_params_to_bclk(params);
        pll_clkin += (*tas2552).tdm_delay;
    }

    pll_enable = (snd_soc_component_read(component, TAS2552_CFG_2) & TAS2552_PLL_ENABLE) as u8;
    snd_soc_component_update_bits(component, TAS2552_CFG_2, TAS2552_PLL_ENABLE, 0);

    if pll_clkin == pll_clk {
        bypass_pll = true;
    }

    if bypass_pll {
        /* By pass the PLL configuration */
        snd_soc_component_update_bits(
            component,
            TAS2552_PLL_CTRL_2,
            TAS2552_PLL_BYPASS,
            TAS2552_PLL_BYPASS,
        );
    } else {
        /* Fill in the PLL control registers for J & D
         * pll_clk = (.5 * pll_clkin * J.D) / 2^p
         * Need to fill in J and D here based on incoming freq
         */
        let mut d: c_uint;
        let mut q: c_uint;
        let mut t: c_uint;
        let mut j: u8;
        let mut pll_sel: u8 = (((*tas2552).pll_clk_id << 3) as c_uint & TAS2552_PLL_SRC_MASK) as u8;
        let mut p: u8 = snd_soc_component_read(component, TAS2552_PLL_CTRL_1) as u8;

        p >>= 7;

        loop {
            t = (pll_clk * 2) << p;
            j = (t / pll_clkin) as u8;
            d = t % pll_clkin;
            t = pll_clkin / 10000;
            q = d / (t + 1);
            d = q + ((9999 - pll_clkin % 10000) * (d / t - q)) / 10000;

            if d != 0 && (pll_clkin < 512000 || pll_clkin > 9200000) {
                if (*tas2552).pll_clk_id == TAS2552_PLL_CLKIN_BCLK {
                    pll_clkin = 1800000;
                    pll_sel = ((TAS2552_PLL_CLKIN_1_8_FIXED << 3) & TAS2552_PLL_SRC_MASK) as u8;
                } else {
                    pll_clkin = snd_soc_params_to_bclk(params);
                    pll_clkin += (*tas2552).tdm_delay;
                    pll_sel = ((TAS2552_PLL_CLKIN_BCLK << 3) & TAS2552_PLL_SRC_MASK) as u8;
                }
                continue;
            }
            break;
        }

        snd_soc_component_update_bits(component, TAS2552_CFG_1, TAS2552_PLL_SRC_MASK, pll_sel as c_uint);

        snd_soc_component_update_bits(component, TAS2552_PLL_CTRL_1, TAS2552_PLL_J_MASK, j as c_uint);
        /* Will clear the PLL_BYPASS bit */
        snd_soc_component_write(component, TAS2552_PLL_CTRL_2, TAS2552_PLL_D_UPPER(d));
        snd_soc_component_write(component, TAS2552_PLL_CTRL_3, TAS2552_PLL_D_LOWER(d));
    }

    /* Restore PLL status */
    snd_soc_component_update_bits(
        component,
        TAS2552_CFG_2,
        TAS2552_PLL_ENABLE,
        pll_enable as c_uint,
    );

    0
}

unsafe extern "C" fn tas2552_hw_params(
    _substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let component = (*dai).component;
    let tas2552 = dev_get_drvdata((*component).dev) as *mut tas2552_data;
    let cpf: c_int;
    let mut ser_ctrl1_reg: u8;
    let wclk_rate: u8;

    match params_width(params) {
        16 => {
            ser_ctrl1_reg = TAS2552_WORDLENGTH_16BIT as u8;
            cpf = 32 + (*tas2552).tdm_delay as c_int;
        }
        20 => {
            ser_ctrl1_reg = TAS2552_WORDLENGTH_20BIT as u8;
            cpf = 64 + (*tas2552).tdm_delay as c_int;
        }
        24 => {
            ser_ctrl1_reg = TAS2552_WORDLENGTH_24BIT as u8;
            cpf = 64 + (*tas2552).tdm_delay as c_int;
        }
        32 => {
            ser_ctrl1_reg = TAS2552_WORDLENGTH_32BIT as u8;
            cpf = 64 + (*tas2552).tdm_delay as c_int;
        }
        _ => {
            dev_err((*component).dev, c"Not supported sample size: %d\n".as_ptr(), params_width(params));
            return -EINVAL;
        }
    }

    if cpf <= 32 {
        ser_ctrl1_reg |= TAS2552_CLKSPERFRAME_32 as u8;
    } else if cpf <= 64 {
        ser_ctrl1_reg |= TAS2552_CLKSPERFRAME_64 as u8;
    } else if cpf <= 128 {
        ser_ctrl1_reg |= TAS2552_CLKSPERFRAME_128 as u8;
    } else {
        ser_ctrl1_reg |= TAS2552_CLKSPERFRAME_256 as u8;
    }

    snd_soc_component_update_bits(
        component,
        TAS2552_SER_CTRL_1,
        TAS2552_WORDLENGTH_MASK | TAS2552_CLKSPERFRAME_MASK,
        ser_ctrl1_reg as c_uint,
    );

    match params_rate(params) {
        8000 => wclk_rate = TAS2552_WCLK_FREQ_8KHZ as u8,
        11025 | 12000 => wclk_rate = TAS2552_WCLK_FREQ_11_12KHZ as u8,
        16000 => wclk_rate = TAS2552_WCLK_FREQ_16KHZ as u8,
        22050 | 24000 => wclk_rate = TAS2552_WCLK_FREQ_22_24KHZ as u8,
        32000 => wclk_rate = TAS2552_WCLK_FREQ_32KHZ as u8,
        44100 | 48000 => wclk_rate = TAS2552_WCLK_FREQ_44_48KHZ as u8,
        88200 | 96000 => wclk_rate = TAS2552_WCLK_FREQ_88_96KHZ as u8,
        176400 | 192000 => wclk_rate = TAS2552_WCLK_FREQ_176_192KHZ as u8,
        _ => {
            dev_err((*component).dev, c"Not supported sample rate: %d\n".as_ptr(), params_rate(params));
            return -EINVAL;
        }
    }

    snd_soc_component_update_bits(component, TAS2552_CFG_3, TAS2552_WCLK_FREQ_MASK, wclk_rate as c_uint);

    tas2552_setup_pll(component, params)
}

const TAS2552_DAI_FMT_MASK: c_uint = TAS2552_BCLKDIR | TAS2552_WCLKDIR | TAS2552_DATAFORMAT_MASK;

unsafe extern "C" fn tas2552_prepare(
    _substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> c_int {
    let component = (*dai).component;
    let tas2552 = snd_soc_component_get_drvdata(component) as *mut tas2552_data;
    let mut delay: c_int = 0;

    /* TDM slot selection only valid in DSP_A/_B mode */
    if (*tas2552).dai_fmt == SND_SOC_DAIFMT_DSP_A {
        delay += ((*tas2552).tdm_delay + 1) as c_int;
    } else if (*tas2552).dai_fmt == SND_SOC_DAIFMT_DSP_B {
        delay += (*tas2552).tdm_delay as c_int;
    }

    /* Configure data delay */
    snd_soc_component_write(component, TAS2552_SER_CTRL_2, delay as c_uint);

    0
}

unsafe extern "C" fn tas2552_set_dai_fmt(dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let component = (*dai).component;
    let tas2552 = dev_get_drvdata((*component).dev) as *mut tas2552_data;
    let mut serial_format: u8;

    match fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK {
        SND_SOC_DAIFMT_CBC_CFC => serial_format = 0x00,
        SND_SOC_DAIFMT_CBC_CFP => serial_format = TAS2552_WCLKDIR as u8,
        SND_SOC_DAIFMT_CBP_CFC => serial_format = TAS2552_BCLKDIR as u8,
        SND_SOC_DAIFMT_CBP_CFP => serial_format = (TAS2552_BCLKDIR | TAS2552_WCLKDIR) as u8,
        _ => {
            dev_vdbg((*component).dev, c"DAI Format master is not found\n".as_ptr());
            return -EINVAL;
        }
    }

    match fmt & (SND_SOC_DAIFMT_FORMAT_MASK | SND_SOC_DAIFMT_INV_MASK) {
        x if x == (SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF) => {}
        x if x == (SND_SOC_DAIFMT_DSP_A | SND_SOC_DAIFMT_IB_NF)
            || x == (SND_SOC_DAIFMT_DSP_B | SND_SOC_DAIFMT_IB_NF) =>
        {
            serial_format |= TAS2552_DATAFORMAT_DSP as u8;
        }
        x if x == (SND_SOC_DAIFMT_RIGHT_J | SND_SOC_DAIFMT_NB_NF) => {
            serial_format |= TAS2552_DATAFORMAT_RIGHT_J as u8;
        }
        x if x == (SND_SOC_DAIFMT_LEFT_J | SND_SOC_DAIFMT_NB_NF) => {
            serial_format |= TAS2552_DATAFORMAT_LEFT_J as u8;
        }
        _ => {
            dev_vdbg((*component).dev, c"DAI Format is not found\n".as_ptr());
            return -EINVAL;
        }
    }
    (*tas2552).dai_fmt = fmt & SND_SOC_DAIFMT_FORMAT_MASK;

    snd_soc_component_update_bits(component, TAS2552_SER_CTRL_1, TAS2552_DAI_FMT_MASK, serial_format as c_uint);
    0
}

unsafe extern "C" fn tas2552_set_dai_sysclk(
    dai: *mut snd_soc_dai,
    mut clk_id: c_int,
    mut freq: c_uint,
    _dir: c_int,
) -> c_int {
    let component = (*dai).component;
    let tas2552 = dev_get_drvdata((*component).dev) as *mut tas2552_data;
    let reg: u8;
    let mask: u8;
    let val: u8;

    match clk_id {
        TAS2552_PLL_CLKIN_MCLK | TAS2552_PLL_CLKIN_IVCLKIN => {
            if freq < 512000 || freq > 24576000 {
                /* out of range PLL_CLKIN, fall back to use BCLK */
                dev_warn((*component).dev, c"Out of range PLL_CLKIN: %u\n".as_ptr(), freq);
                clk_id = TAS2552_PLL_CLKIN_BCLK;
                freq = 0;
            }
            mask = TAS2552_PLL_SRC_MASK as u8;
            val = (((clk_id << 3) as c_uint) & mask as c_uint) as u8; /* bit 4:5 in the register */
            reg = TAS2552_CFG_1 as u8;
            (*tas2552).pll_clk_id = clk_id;
            (*tas2552).pll_clkin = freq;
        }
        TAS2552_PLL_CLKIN_BCLK | TAS2552_PLL_CLKIN_1_8_FIXED => {
            mask = TAS2552_PLL_SRC_MASK as u8;
            val = (((clk_id << 3) as c_uint) & mask as c_uint) as u8; /* bit 4:5 in the register */
            reg = TAS2552_CFG_1 as u8;
            (*tas2552).pll_clk_id = clk_id;
            (*tas2552).pll_clkin = freq;
        }
        TAS2552_PDM_CLK_PLL | TAS2552_PDM_CLK_IVCLKIN | TAS2552_PDM_CLK_BCLK | TAS2552_PDM_CLK_MCLK => {
            mask = TAS2552_PDM_CLK_SEL_MASK as u8;
            val = (((clk_id >> 1) as c_uint) & mask as c_uint) as u8; /* bit 0:1 in the register */
            reg = TAS2552_PDM_CFG as u8;
            (*tas2552).pdm_clk_id = clk_id;
            (*tas2552).pdm_clk = freq;
        }
        _ => {
            dev_err((*component).dev, c"Invalid clk id: %d\n".as_ptr(), clk_id);
            return -EINVAL;
        }
    }

    snd_soc_component_update_bits(component, reg as c_uint, mask as c_uint, val as c_uint);

    0
}

unsafe extern "C" fn tas2552_set_dai_tdm_slot(
    dai: *mut snd_soc_dai,
    tx_mask: c_uint,
    _rx_mask: c_uint,
    _slots: c_int,
    slot_width: c_int,
) -> c_int {
    let component = (*dai).component;
    let tas2552 = snd_soc_component_get_drvdata(component) as *mut tas2552_data;
    let lsb: c_uint;

    if unlikely(tx_mask == 0) {
        dev_err((*component).dev, c"tx masks need to be non 0\n".as_ptr());
        return -EINVAL;
    }

    /* TDM based on DSP mode requires slots to be adjacent */
    lsb = __ffs(tx_mask);
    if (lsb + 1) != __fls(tx_mask) {
        dev_err((*component).dev, c"Invalid mask, slots must be adjacent\n".as_ptr());
        return -EINVAL;
    }

    (*tas2552).tdm_delay = lsb * slot_width as c_uint;

    /* DOUT in high-impedance on inactive bit clocks */
    snd_soc_component_update_bits(component, TAS2552_DOUT, TAS2552_SDOUT_TRISTATE, TAS2552_SDOUT_TRISTATE);

    0
}

unsafe extern "C" fn tas2552_mute(dai: *mut snd_soc_dai, mute: c_int, _direction: c_int) -> c_int {
    let mut cfg1_reg: u8 = 0;
    let component = (*dai).component;

    if mute != 0 {
        cfg1_reg |= TAS2552_MUTE as u8;
    }

    snd_soc_component_update_bits(component, TAS2552_CFG_1, TAS2552_MUTE, cfg1_reg as c_uint);

    0
}

unsafe extern "C" fn tas2552_runtime_suspend(dev: *mut device) -> c_int {
    let tas2552 = dev_get_drvdata(dev) as *mut tas2552_data;

    tas2552_sw_shutdown(tas2552, 1);

    regcache_cache_only((*tas2552).regmap, true);
    regcache_mark_dirty((*tas2552).regmap);

    gpiod_set_value_cansleep((*tas2552).enable_gpio, 0);

    0
}

unsafe extern "C" fn tas2552_runtime_resume(dev: *mut device) -> c_int {
    let tas2552 = dev_get_drvdata(dev) as *mut tas2552_data;
    let ret: c_int;

    gpiod_set_value_cansleep((*tas2552).enable_gpio, 1);

    tas2552_sw_shutdown(tas2552, 0);

    regcache_cache_only((*tas2552).regmap, false);
    ret = regcache_sync((*tas2552).regmap);
    if ret != 0 {
        regcache_cache_only((*tas2552).regmap, true);
        regcache_mark_dirty((*tas2552).regmap);
        tas2552_sw_shutdown(tas2552, 1);
        gpiod_set_value_cansleep((*tas2552).enable_gpio, 0);
        return ret;
    }

    0
}

static tas2552_pm: dev_pm_ops =
    dev_pm_ops { runtime_suspend: Some(tas2552_runtime_suspend), runtime_resume: Some(tas2552_runtime_resume), ..RUNTIME_PM_OPS_NULL() };

static tas2552_speaker_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(tas2552_hw_params),
    prepare: Some(tas2552_prepare),
    set_sysclk: Some(tas2552_set_dai_sysclk),
    set_fmt: Some(tas2552_set_dai_fmt),
    set_tdm_slot: Some(tas2552_set_dai_tdm_slot),
    mute_stream: Some(tas2552_mute),
    no_capture_mute: 1,
    ..SND_SOC_DAI_OPS_DEFAULT()
};

/* Formats supported by TAS2552 driver. */
const TAS2552_FORMATS: c_uint =
    SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S20_3LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE;

/* TAS2552 dai structure. */
static mut tas2552_dai: [snd_soc_dai_driver; 1] = [snd_soc_dai_driver {
    name: c"tas2552-amplifier".as_ptr(),
    playback: snd_soc_pcm_stream {
        stream_name: c"Playback".as_ptr(),
        channels_min: 2,
        channels_max: 2,
        rates: SNDRV_PCM_RATE_8000_192000,
        formats: TAS2552_FORMATS,
        ..SND_SOC_PCM_STREAM_DEFAULT()
    },
    capture: snd_soc_pcm_stream {
        stream_name: c"Capture".as_ptr(),
        channels_min: 2,
        channels_max: 2,
        rates: SNDRV_PCM_RATE_8000_192000,
        formats: TAS2552_FORMATS,
        ..SND_SOC_PCM_STREAM_DEFAULT()
    },
    ops: &tas2552_speaker_dai_ops,
    ..SND_SOC_DAI_DRIVER_DEFAULT()
}];

/*
 * DAC digital volumes. From -7 to 24 dB in 1 dB steps
 */
static dac_tlv: [c_uint; DECLARE_TLV_DB_SCALE_SIZE] = DECLARE_TLV_DB_SCALE(-700, 100, 0);

static tas2552_din_source_select: [*const c_char; 4] = [
    c"Muted".as_ptr(),
    c"Left".as_ptr(),
    c"Right".as_ptr(),
    c"Left + Right average".as_ptr(),
];
static tas2552_din_source_enum: soc_enum =
    SOC_ENUM_SINGLE_DECL(TAS2552_CFG_3, 3, &tas2552_din_source_select);

static tas2552_snd_controls: [snd_kcontrol_new; 2] = [
    SOC_SINGLE_TLV(
        c"Speaker Driver Playback Volume".as_ptr(),
        TAS2552_PGA_GAIN,
        0,
        0x1f,
        0,
        &dac_tlv,
    ),
    SOC_ENUM(c"DIN source".as_ptr(), &tas2552_din_source_enum),
];

unsafe extern "C" fn tas2552_component_probe(component: *mut snd_soc_component) -> c_int {
    let tas2552 = snd_soc_component_get_drvdata(component) as *mut tas2552_data;
    let mut ret: c_int;

    (*tas2552).component = component;

    ret = regulator_bulk_enable(ARRAY_SIZE((*tas2552).supplies.as_ptr()), (*tas2552).supplies.as_mut_ptr());

    if ret != 0 {
        dev_err((*component).dev, c"Failed to enable supplies: %d\n".as_ptr(), ret);
        return ret;
    }

    gpiod_set_value_cansleep((*tas2552).enable_gpio, 1);

    ret = pm_runtime_resume_and_get((*component).dev);
    if ret < 0 {
        dev_err((*component).dev, c"Enabling device failed: %d\n".as_ptr(), ret);
        pm_runtime_put_noidle((*component).dev);
        gpiod_set_value_cansleep((*tas2552).enable_gpio, 0);
        regulator_bulk_disable(ARRAY_SIZE((*tas2552).supplies.as_ptr()), (*tas2552).supplies.as_mut_ptr());
        return ret;
    }

    snd_soc_component_update_bits(component, TAS2552_CFG_1, TAS2552_MUTE, TAS2552_MUTE);
    snd_soc_component_write(component, TAS2552_CFG_3, TAS2552_I2S_OUT_SEL | TAS2552_DIN_SRC_SEL_AVG_L_R);
    snd_soc_component_write(
        component,
        TAS2552_OUTPUT_DATA,
        TAS2552_PDM_DATA_SEL_V_I | TAS2552_R_DATA_OUT(TAS2552_DATA_OUT_V_DATA),
    );
    snd_soc_component_write(
        component,
        TAS2552_BOOST_APT_CTRL,
        TAS2552_APT_DELAY_200 | TAS2552_APT_THRESH_20_17,
    );

    snd_soc_component_write(component, TAS2552_CFG_2, TAS2552_BOOST_EN | TAS2552_APT_EN | TAS2552_LIM_EN);

    0
}

unsafe extern "C" fn tas2552_component_remove(component: *mut snd_soc_component) {
    let tas2552 = snd_soc_component_get_drvdata(component) as *mut tas2552_data;

    pm_runtime_put((*component).dev);

    gpiod_set_value_cansleep((*tas2552).enable_gpio, 0);
}

/* CONFIG_PM conditional in C: suspend/resume callbacks exist only when CONFIG_PM is enabled. */
#[cfg(CONFIG_PM)]
unsafe extern "C" fn tas2552_suspend(component: *mut snd_soc_component) -> c_int {
    let tas2552 = snd_soc_component_get_drvdata(component) as *mut tas2552_data;
    let ret: c_int;

    ret = regulator_bulk_disable(ARRAY_SIZE((*tas2552).supplies.as_ptr()), (*tas2552).supplies.as_mut_ptr());

    if ret != 0 {
        dev_err((*component).dev, c"Failed to disable supplies: %d\n".as_ptr(), ret);
    }
    ret
}

#[cfg(CONFIG_PM)]
unsafe extern "C" fn tas2552_resume(component: *mut snd_soc_component) -> c_int {
    let tas2552 = snd_soc_component_get_drvdata(component) as *mut tas2552_data;
    let ret: c_int;

    ret = regulator_bulk_enable(ARRAY_SIZE((*tas2552).supplies.as_ptr()), (*tas2552).supplies.as_mut_ptr());

    if ret != 0 {
        dev_err((*component).dev, c"Failed to enable supplies: %d\n".as_ptr(), ret);
    }

    ret
}

#[cfg(not(CONFIG_PM))]
const tas2552_suspend: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int> = None;
#[cfg(not(CONFIG_PM))]
const tas2552_resume: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int> = None;

static soc_component_dev_tas2552: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(tas2552_component_probe),
    remove: Some(tas2552_component_remove),
    suspend: Some(tas2552_suspend),
    resume: Some(tas2552_resume),
    controls: tas2552_snd_controls.as_ptr(),
    num_controls: tas2552_snd_controls.len() as c_uint,
    dapm_widgets: tas2552_dapm_widgets.as_ptr(),
    num_dapm_widgets: tas2552_dapm_widgets.len() as c_uint,
    dapm_routes: tas2552_audio_map.as_ptr(),
    num_dapm_routes: tas2552_audio_map.len() as c_uint,
    idle_bias_on: 1,
    endianness: 1,
    ..SND_SOC_COMPONENT_DRIVER_DEFAULT()
};

static tas2552_regmap_config: regmap_config = regmap_config {
    reg_bits: 8,
    val_bits: 8,

    max_register: TAS2552_MAX_REG,
    reg_defaults: tas2552_reg_defs.as_ptr(),
    num_reg_defaults: tas2552_reg_defs.len() as c_uint,
    cache_type: REGCACHE_RBTREE,
    ..REGMAP_CONFIG_DEFAULT()
};

unsafe extern "C" fn tas2552_probe(client: *mut i2c_client) -> c_int {
    let dev: *mut device;
    let data: *mut tas2552_data;
    let mut ret: c_int;
    let mut i: c_int;

    dev = &mut (*client).dev;
    data = devm_kzalloc(&mut (*client).dev, core::mem::size_of::<tas2552_data>(), GFP_KERNEL) as *mut tas2552_data;
    if data.is_null() {
        return -ENOMEM;
    }

    (*data).enable_gpio = devm_gpiod_get_optional(dev, c"enable".as_ptr(), GPIOD_OUT_LOW);
    if IS_ERR((*data).enable_gpio) {
        return PTR_ERR((*data).enable_gpio);
    }

    (*data).tas2552_client = client;
    (*data).regmap = devm_regmap_init_i2c(client, &tas2552_regmap_config);
    if IS_ERR((*data).regmap) {
        ret = PTR_ERR((*data).regmap);
        dev_err(&mut (*client).dev, c"Failed to allocate register map: %d\n".as_ptr(), ret);
        return ret;
    }

    i = 0;
    while i < ARRAY_SIZE((*data).supplies.as_ptr()) as c_int {
        (*data).supplies[i as usize].supply = tas2552_supply_names[i as usize];
        i += 1;
    }

    ret = devm_regulator_bulk_get(dev, ARRAY_SIZE((*data).supplies.as_ptr()), (*data).supplies.as_mut_ptr());
    if ret != 0 {
        dev_err(dev, c"Failed to request supplies: %d\n".as_ptr(), ret);
        return ret;
    }

    pm_runtime_set_active(&mut (*client).dev);
    pm_runtime_set_autosuspend_delay(&mut (*client).dev, 1000);
    pm_runtime_use_autosuspend(&mut (*client).dev);
    pm_runtime_enable(&mut (*client).dev);
    pm_runtime_put_sync_autosuspend(&mut (*client).dev);

    dev_set_drvdata(&mut (*client).dev, data as *mut c_void);

    ret = devm_snd_soc_register_component(
        &mut (*client).dev,
        &soc_component_dev_tas2552,
        tas2552_dai.as_mut_ptr(),
        tas2552_dai.len() as c_int,
    );
    if ret < 0 {
        dev_err(&mut (*client).dev, c"Failed to register component: %d\n".as_ptr(), ret);
        pm_runtime_get_noresume(&mut (*client).dev);
    }

    ret
}

unsafe extern "C" fn tas2552_i2c_remove(client: *mut i2c_client) {
    pm_runtime_disable(&mut (*client).dev);
}

static tas2552_id: [i2c_device_id; 2] = [
    i2c_device_id { name: *b"tas2552\0                         ", driver_data: 0 },
    i2c_device_id { name: [0; I2C_NAME_SIZE], driver_data: 0 },
];
MODULE_DEVICE_TABLE(i2c, tas2552_id);

/* IS_ENABLED(CONFIG_OF) conditional in C: OF match table exists only when enabled. */
#[cfg(CONFIG_OF)]
static tas2552_of_match: [of_device_id; 2] = [
    of_device_id { compatible: c"ti,tas2552".as_ptr(), ..OF_DEVICE_ID_DEFAULT() },
    of_device_id { ..OF_DEVICE_ID_DEFAULT() },
];
#[cfg(CONFIG_OF)]
MODULE_DEVICE_TABLE(of, tas2552_of_match);

static mut tas2552_i2c_driver: i2c_driver = i2c_driver {
    driver: device_driver {
        name: c"tas2552".as_ptr(),
        of_match_table: of_match_ptr(tas2552_of_match.as_ptr()),
        pm: pm_ptr(&tas2552_pm),
        ..DEVICE_DRIVER_DEFAULT()
    },
    probe: Some(tas2552_probe),
    remove: Some(tas2552_i2c_remove),
    id_table: tas2552_id.as_ptr(),
    ..I2C_DRIVER_DEFAULT()
};

module_i2c_driver!(tas2552_i2c_driver);

MODULE_AUTHOR(c"Dan Muprhy <dmurphy@ti.com>".as_ptr());
MODULE_DESCRIPTION(c"TAS2552 Audio amplifier driver".as_ptr());
MODULE_LICENSE(c"GPL".as_ptr());

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
