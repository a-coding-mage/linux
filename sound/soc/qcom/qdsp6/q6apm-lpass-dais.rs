// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2021, Linaro Limited

// C include dependencies:
// dt-bindings/sound/qcom,q6dsp-lpass-ports.h
// linux/clk.h, linux/err.h, linux/init.h, linux/module.h, linux/device.h,
// linux/of.h, linux/platform_device.h, linux/slab.h
// sound/pcm.h, sound/soc.h, sound/pcm_params.h
// q6dsp-lpass-ports.h, q6dsp-common.h, audioreach.h, q6apm.h, q6prm.h

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem;
use core::ptr;

const AUDIOREACH_BE_PCM_BASE: c_int = 16;

#[repr(C)]
struct q6apm_dai_priv_data {
    mclk: *mut clk,
    bclk: *mut clk,
    mclk_enabled: bool,
    bclk_enabled: bool,
}

#[repr(C)]
struct q6apm_lpass_dai_data {
    graph: [*mut q6apm_graph; APM_PORT_MAX as usize],
    is_port_started: [bool; APM_PORT_MAX as usize],
    module_config: [audioreach_module_config; APM_PORT_MAX as usize],
    priv_: [q6apm_dai_priv_data; APM_PORT_MAX as usize],
}

unsafe fn q6apm_lpass_dai_disable_clocks(dai_data: *mut q6apm_lpass_dai_data, id: c_int) {
    if (*dai_data).priv_[id as usize].mclk_enabled {
        clk_disable_unprepare((*dai_data).priv_[id as usize].mclk);
        (*dai_data).priv_[id as usize].mclk_enabled = false;
    }

    if (*dai_data).priv_[id as usize].bclk_enabled {
        clk_disable_unprepare((*dai_data).priv_[id as usize].bclk);
        (*dai_data).priv_[id as usize].bclk_enabled = false;
    }
}

unsafe fn q6apm_lpass_dai_put_clocks(dai_data: *mut q6apm_lpass_dai_data) {
    let mut i: c_int = 0;

    while i < APM_PORT_MAX {
        q6apm_lpass_dai_disable_clocks(dai_data, i);

        if !(*dai_data).priv_[i as usize].mclk.is_null() {
            clk_put((*dai_data).priv_[i as usize].mclk);
            (*dai_data).priv_[i as usize].mclk = ptr::null_mut();
        }
        if !(*dai_data).priv_[i as usize].bclk.is_null() {
            clk_put((*dai_data).priv_[i as usize].bclk);
            (*dai_data).priv_[i as usize].bclk = ptr::null_mut();
        }

        i += 1;
    }
}

unsafe extern "C" fn q6dma_set_channel_map(
    dai: *mut snd_soc_dai,
    tx_num: c_uint,
    tx_ch_mask: *const c_uint,
    rx_num: c_uint,
    rx_ch_mask: *const c_uint,
) -> c_int {
    let dai_data = dev_get_drvdata((*dai).dev) as *mut q6apm_lpass_dai_data;
    let cfg = &mut (*dai_data).module_config[(*dai).id as usize] as *mut audioreach_module_config;
    let mut i: c_int;

    match (*dai).id {
        WSA_CODEC_DMA_TX_0 | WSA_CODEC_DMA_TX_1 | WSA_CODEC_DMA_TX_2
        | VA_CODEC_DMA_TX_0 | VA_CODEC_DMA_TX_1 | VA_CODEC_DMA_TX_2
        | TX_CODEC_DMA_TX_0 | TX_CODEC_DMA_TX_1 | TX_CODEC_DMA_TX_2
        | TX_CODEC_DMA_TX_3 | TX_CODEC_DMA_TX_4 | TX_CODEC_DMA_TX_5 => {
            if tx_ch_mask.is_null() {
                dev_err((*dai).dev, c"tx slot not found\n".as_ptr());
                return -EINVAL;
            }

            if tx_num > AR_PCM_MAX_NUM_CHANNEL as c_uint {
                dev_err((*dai).dev, c"invalid tx num %d\n".as_ptr(), tx_num);
                return -EINVAL;
            }

            i = 0;
            while (i as c_uint) < tx_num {
                (*cfg).channel_map[i as usize] = *tx_ch_mask.add(i as usize);
                i += 1;
            }
        }
        WSA_CODEC_DMA_RX_0 | WSA_CODEC_DMA_RX_1
        | RX_CODEC_DMA_RX_0 | RX_CODEC_DMA_RX_1 | RX_CODEC_DMA_RX_2
        | RX_CODEC_DMA_RX_3 | RX_CODEC_DMA_RX_4 | RX_CODEC_DMA_RX_5
        | RX_CODEC_DMA_RX_6 | RX_CODEC_DMA_RX_7 => {
            /* rx */
            if rx_ch_mask.is_null() {
                dev_err((*dai).dev, c"rx slot not found\n".as_ptr());
                return -EINVAL;
            }
            if rx_num > APM_PORT_MAX_AUDIO_CHAN_CNT as c_uint {
                dev_err((*dai).dev, c"invalid rx num %d\n".as_ptr(), rx_num);
                return -EINVAL;
            }

            i = 0;
            while (i as c_uint) < rx_num {
                (*cfg).channel_map[i as usize] = *rx_ch_mask.add(i as usize);
                i += 1;
            }
        }
        _ => {
            dev_err(
                (*dai).dev,
                c"%s: invalid dai id 0x%x\n".as_ptr(),
                c"q6dma_set_channel_map".as_ptr(),
                (*dai).id,
            );
            return -EINVAL;
        }
    }

    0
}

unsafe extern "C" fn q6hdmi_hw_params(
    _substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let dai_data = dev_get_drvdata((*dai).dev) as *mut q6apm_lpass_dai_data;
    let cfg = &mut (*dai_data).module_config[(*dai).id as usize] as *mut audioreach_module_config;
    let channels: c_int = (*hw_param_interval_c(params, SNDRV_PCM_HW_PARAM_CHANNELS)).max as c_int;
    let mut ret: c_int;

    (*cfg).bit_width = params_width(params);
    (*cfg).sample_rate = params_rate(params);
    (*cfg).num_channels = channels;
    audioreach_set_default_channel_mapping((*cfg).channel_map.as_mut_ptr(), channels);

    match (*dai).id {
        DISPLAY_PORT_RX_0 => {
            (*cfg).dp_idx = 0;
        }
        id if id >= DISPLAY_PORT_RX_1 && id <= DISPLAY_PORT_RX_7 => {
            (*cfg).dp_idx = (*dai).id - DISPLAY_PORT_RX_1 + 1;
        }
        _ => {}
    }

    ret = q6dsp_get_channel_allocation(channels);
    if ret < 0 {
        return ret;
    }

    (*cfg).channel_allocation = ret;

    0
}

unsafe extern "C" fn q6dma_hw_params(
    _substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let dai_data = dev_get_drvdata((*dai).dev) as *mut q6apm_lpass_dai_data;
    let cfg = &mut (*dai_data).module_config[(*dai).id as usize] as *mut audioreach_module_config;
    let channels: c_int = (*hw_param_interval_c(params, SNDRV_PCM_HW_PARAM_CHANNELS)).max as c_int;

    (*cfg).bit_width = params_width(params);
    (*cfg).sample_rate = params_rate(params);
    (*cfg).num_channels = channels;
    audioreach_set_default_channel_mapping((*cfg).channel_map.as_mut_ptr(), channels);

    0
}

unsafe extern "C" fn q6apm_lpass_dai_shutdown(
    _substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) {
    let dai_data = dev_get_drvdata((*dai).dev) as *mut q6apm_lpass_dai_data;
    let mut rc: c_int;

    if (*dai_data).is_port_started[(*dai).id as usize] {
        rc = q6apm_graph_stop((*dai_data).graph[(*dai).id as usize]);
        (*dai_data).is_port_started[(*dai).id as usize] = false;
        if rc < 0 {
            dev_err((*dai).dev, c"failed to stop APM port (%d)\n".as_ptr(), rc);
        }
    }

    if !(*dai_data).graph[(*dai).id as usize].is_null() {
        q6apm_graph_close((*dai_data).graph[(*dai).id as usize]);
        (*dai_data).graph[(*dai).id as usize] = ptr::null_mut();
    }
}

unsafe extern "C" fn q6apm_lpass_dai_trigger(
    _substream: *mut snd_pcm_substream,
    cmd: c_int,
    dai: *mut snd_soc_dai,
) -> c_int {
    let dai_data = dev_get_drvdata((*dai).dev) as *mut q6apm_lpass_dai_data;
    let mut ret: c_int = 0;

    match cmd {
        SNDRV_PCM_TRIGGER_START | SNDRV_PCM_TRIGGER_RESUME | SNDRV_PCM_TRIGGER_PAUSE_RELEASE => {
            if !(*dai_data).is_port_started[(*dai).id as usize] {
                ret = q6apm_graph_start((*dai_data).graph[(*dai).id as usize]);
                if ret < 0 {
                    dev_err((*dai).dev, c"Failed to start APM port %d\n".as_ptr(), (*dai).id);
                } else {
                    (*dai_data).is_port_started[(*dai).id as usize] = true;
                }
            }
        }
        _ => {}
    }

    ret
}

unsafe extern "C" fn q6apm_lpass_dai_prepare(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> c_int {
    let dai_data = dev_get_drvdata((*dai).dev) as *mut q6apm_lpass_dai_data;
    let cfg = &mut (*dai_data).module_config[(*dai).id as usize] as *mut audioreach_module_config;
    let mut graph: *mut q6apm_graph;
    let graph_id: c_int = (*dai).id;
    let mut rc: c_int;

    if (*dai_data).is_port_started[(*dai).id as usize] {
        q6apm_graph_stop((*dai_data).graph[(*dai).id as usize]);
        (*dai_data).is_port_started[(*dai).id as usize] = false;
    }

    /**
     * It is recommend to load DSP with source graph first and then sink
     * graph, so sequence for playback and capture will be different
     */
    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK
        && (*dai_data).graph[(*dai).id as usize].is_null()
    {
        graph = q6apm_graph_open((*dai).dev, ptr::null_mut(), (*dai).dev, graph_id, (*substream).stream);
        if IS_ERR(graph as *const c_void) {
            dev_err((*dai).dev, c"Failed to open graph (%d)\n".as_ptr(), graph_id);
            rc = PTR_ERR(graph as *const c_void) as c_int;
            return rc;
        }
        (*dai_data).graph[graph_id as usize] = graph;
    }

    (*cfg).direction = (*substream).stream;
    rc = q6apm_graph_media_format_pcm((*dai_data).graph[(*dai).id as usize], cfg);
    if rc != 0 {
        dev_err((*dai).dev, c"Failed to set media format %d\n".as_ptr(), rc);
        if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
            q6apm_graph_close((*dai_data).graph[(*dai).id as usize]);
            (*dai_data).graph[(*dai).id as usize] = ptr::null_mut();
        }
        return rc;
    }

    rc = q6apm_graph_prepare((*dai_data).graph[(*dai).id as usize]);
    if rc != 0 {
        dev_err((*dai).dev, c"Failed to prepare Graph %d\n".as_ptr(), rc);
        if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
            q6apm_graph_close((*dai_data).graph[(*dai).id as usize]);
            (*dai_data).graph[(*dai).id as usize] = ptr::null_mut();
        }
        return rc;
    }

    0
}

unsafe extern "C" fn q6apm_lpass_dai_startup(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> c_int {
    let dai_data = dev_get_drvdata((*dai).dev) as *mut q6apm_lpass_dai_data;
    let mut graph: *mut q6apm_graph;
    let graph_id: c_int = (*dai).id;

    if (*substream).stream == SNDRV_PCM_STREAM_CAPTURE {
        graph = q6apm_graph_open((*dai).dev, ptr::null_mut(), (*dai).dev, graph_id, (*substream).stream);
        if IS_ERR(graph as *const c_void) {
            dev_err((*dai).dev, c"Failed to open graph (%d)\n".as_ptr(), graph_id);
            return PTR_ERR(graph as *const c_void) as c_int;
        }
        (*dai_data).graph[graph_id as usize] = graph;
    }

    0
}

unsafe extern "C" fn q6i2s_dai_startup(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> c_int {
    q6apm_lpass_dai_startup(substream, dai)
}

unsafe extern "C" fn q6i2s_lpass_dai_shutdown(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) {
    let dai_data = dev_get_drvdata((*dai).dev) as *mut q6apm_lpass_dai_data;

    q6apm_lpass_dai_shutdown(substream, dai);
    q6apm_lpass_dai_disable_clocks(dai_data, (*dai).id);
}

unsafe extern "C" fn q6i2s_set_sysclk(
    dai: *mut snd_soc_dai,
    clk_id: c_int,
    freq: c_uint,
    _dir: c_int,
) -> c_int {
    let dai_data = dev_get_drvdata((*dai).dev) as *mut q6apm_lpass_dai_data;
    let mut sysclk: *mut clk = ptr::null_mut();
    let mut enabled: *mut bool = ptr::null_mut();
    let mut ret: c_int = 0;

    match clk_id {
        LPAIF_MI2S_MCLK => {
            sysclk = (*dai_data).priv_[(*dai).id as usize].mclk;
            enabled = &mut (*dai_data).priv_[(*dai).id as usize].mclk_enabled;
        }
        LPAIF_MI2S_BCLK => {
            sysclk = (*dai_data).priv_[(*dai).id as usize].bclk;
            enabled = &mut (*dai_data).priv_[(*dai).id as usize].bclk_enabled;
        }
        _ => {
            return -EINVAL;
        }
    }

    if !sysclk.is_null() {
        ret = clk_set_rate(sysclk, freq);
        if ret != 0 {
            dev_err(
                (*dai).dev,
                c"Error, Unable to set rate (%d) for sysclk %d\n".as_ptr(),
                freq,
                clk_id,
            );
            return ret;
        }

        if *enabled {
            return 0;
        }

        ret = clk_prepare_enable(sysclk);
        if ret != 0 {
            dev_err(
                (*dai).dev,
                c"Error, Unable to prepare (%d) sysclk\n".as_ptr(),
                clk_id,
            );
            return ret;
        }

        *enabled = true;
    }

    ret
}

unsafe extern "C" fn q6i2s_set_fmt(dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let dai_data = dev_get_drvdata((*dai).dev) as *mut q6apm_lpass_dai_data;
    let cfg = &mut (*dai_data).module_config[(*dai).id as usize] as *mut audioreach_module_config;

    (*cfg).fmt = fmt;

    0
}

unsafe extern "C" fn q6tdm_set_tdm_slot(
    dai: *mut snd_soc_dai,
    tx_mask: c_uint,
    rx_mask: c_uint,
    slots: c_int,
    slot_width: c_int,
) -> c_int {
    let dai_data = dev_get_drvdata((*dai).dev) as *mut q6apm_lpass_dai_data;
    let cfg = &mut (*dai_data).module_config[(*dai).id as usize] as *mut audioreach_module_config;
    let cap_mask: c_uint;
    let slot_mask: c_uint;

    if slot_width != 16 && slot_width != 32 {
        dev_err(
            (*dai).dev,
            c"%s: invalid slot_width %d\n".as_ptr(),
            c"q6tdm_set_tdm_slot".as_ptr(),
            slot_width,
        );
        return -EINVAL;
    }

    match slots {
        2 | 4 | 8 | 16 => {
            cap_mask = GENMASK((slots - 1) as c_uint, 0);
        }
        _ => {
            dev_err(
                (*dai).dev,
                c"%s: invalid slots %d\n".as_ptr(),
                c"q6tdm_set_tdm_slot".as_ptr(),
                slots,
            );
            return -EINVAL;
        }
    }

    match (*dai).id {
        id if id >= PRIMARY_TDM_RX_0 && id <= QUINARY_TDM_TX_7 => {
            slot_mask = if ((*dai).id & 0x1) != 0 { tx_mask } else { rx_mask };
            if (slot_mask & !cap_mask) != 0 {
                dev_err(
                    (*dai).dev,
                    c"%s: invalid slot mask 0x%x for %d slots\n".as_ptr(),
                    c"q6tdm_set_tdm_slot".as_ptr(),
                    slot_mask,
                    slots,
                );
                return -EINVAL;
            }

            (*cfg).nslots_per_frame = slots;
            (*cfg).slot_width = slot_width;
            (*cfg).slot_mask = slot_mask;
        }
        _ => {
            dev_err(
                (*dai).dev,
                c"%s: invalid dai id 0x%x\n".as_ptr(),
                c"q6tdm_set_tdm_slot".as_ptr(),
                (*dai).id,
            );
            return -EINVAL;
        }
    }

    0
}

static q6dma_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    prepare: Some(q6apm_lpass_dai_prepare),
    startup: Some(q6apm_lpass_dai_startup),
    shutdown: Some(q6apm_lpass_dai_shutdown),
    set_channel_map: Some(q6dma_set_channel_map),
    hw_params: Some(q6dma_hw_params),
    trigger: Some(q6apm_lpass_dai_trigger),
    ..unsafe { mem::zeroed() }
};

static q6i2s_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    prepare: Some(q6apm_lpass_dai_prepare),
    startup: Some(q6i2s_dai_startup),
    shutdown: Some(q6i2s_lpass_dai_shutdown),
    set_channel_map: Some(q6dma_set_channel_map),
    hw_params: Some(q6dma_hw_params),
    set_fmt: Some(q6i2s_set_fmt),
    set_sysclk: Some(q6i2s_set_sysclk),
    trigger: Some(q6apm_lpass_dai_trigger),
    ..unsafe { mem::zeroed() }
};

static q6hdmi_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    prepare: Some(q6apm_lpass_dai_prepare),
    startup: Some(q6apm_lpass_dai_startup),
    shutdown: Some(q6apm_lpass_dai_shutdown),
    hw_params: Some(q6hdmi_hw_params),
    set_fmt: Some(q6i2s_set_fmt),
    trigger: Some(q6apm_lpass_dai_trigger),
    ..unsafe { mem::zeroed() }
};

static q6tdm_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    prepare: Some(q6apm_lpass_dai_prepare),
    startup: Some(q6apm_lpass_dai_startup),
    shutdown: Some(q6i2s_lpass_dai_shutdown),
    set_tdm_slot: Some(q6tdm_set_tdm_slot),
    hw_params: Some(q6dma_hw_params),
    set_fmt: Some(q6i2s_set_fmt),
    set_sysclk: Some(q6i2s_set_sysclk),
    trigger: Some(q6apm_lpass_dai_trigger),
    ..unsafe { mem::zeroed() }
};

static q6apm_lpass_dai_component: snd_soc_component_driver = snd_soc_component_driver {
    name: c"q6apm-be-dai-component".as_ptr(),
    of_xlate_dai_name: Some(q6dsp_audio_ports_of_xlate_dai_name),
    be_pcm_base: AUDIOREACH_BE_PCM_BASE,
    use_dai_pcm_id: true,
    remove_order: SND_SOC_COMP_ORDER_FIRST,
    ..unsafe { mem::zeroed() }
};

unsafe fn of_q6apm_parse_dai_data(
    dev: *mut device,
    data: *mut q6apm_lpass_dai_data,
) -> c_int {
    let mut ret: c_int;

    for_each_child_of_node_scoped((*dev).of_node, |node| {
        let mut priv_: *mut q6apm_dai_priv_data;
        let mut id: c_int = 0;

        ret = of_property_read_u32(node, c"reg".as_ptr(), &mut id as *mut c_int as *mut u32);
        if ret != 0 || id < 0 || id >= APM_PORT_MAX {
            dev_err(dev, c"valid dai id not found:%d\n".as_ptr(), ret);
            return ForEach::Continue;
        }

        match id {
            /* MI2S specific properties */
            dai_id if (dai_id >= PRIMARY_MI2S_RX && dai_id <= QUATERNARY_MI2S_TX)
                || (dai_id >= QUINARY_MI2S_RX && dai_id <= QUINARY_MI2S_TX)
                || (dai_id >= SENARY_MI2S_RX && dai_id <= SENARY_MI2S_TX)
                || (dai_id >= PRIMARY_TDM_RX_0 && dai_id <= QUINARY_TDM_TX_7) =>
            {
                priv_ = &mut (*data).priv_[id as usize];
                (*priv_).mclk = of_clk_get_by_name(node, c"mclk".as_ptr());
                if IS_ERR((*priv_).mclk as *const c_void) {
                    let err: c_int = PTR_ERR((*priv_).mclk as *const c_void) as c_int;

                    (*priv_).mclk = ptr::null_mut();
                    if err == -EPROBE_DEFER {
                        q6apm_lpass_dai_put_clocks(data);
                        ret = dev_err_probe(dev, err, c"unable to get mi2s mclk\n".as_ptr());
                        return ForEach::Break(ret);
                    }
                }

                (*priv_).bclk = of_clk_get_by_name(node, c"bclk".as_ptr());
                if IS_ERR((*priv_).bclk as *const c_void) {
                    let err: c_int = PTR_ERR((*priv_).bclk as *const c_void) as c_int;

                    (*priv_).bclk = ptr::null_mut();
                    if err == -EPROBE_DEFER {
                        q6apm_lpass_dai_put_clocks(data);
                        ret = dev_err_probe(dev, err, c"unable to get mi2s bclk\n".as_ptr());
                        return ForEach::Break(ret);
                    }
                }
            }
            _ => {}
        }

        ForEach::Continue
    })
    .unwrap_or(0)
}

unsafe extern "C" fn q6apm_lpass_dai_clocks_action(data: *mut c_void) {
    q6apm_lpass_dai_put_clocks(data as *mut q6apm_lpass_dai_data);
}

unsafe extern "C" fn q6apm_lpass_dai_dev_probe(pdev: *mut platform_device) -> c_int {
    let mut cfg: q6dsp_audio_port_dai_driver_config = mem::zeroed();
    let mut dai_data: *mut q6apm_lpass_dai_data;
    let mut dais: *mut snd_soc_dai_driver;
    let dev: *mut device = &mut (*pdev).dev;
    let mut num_dais: c_int = 0;
    let mut ret: c_int;

    dai_data = devm_kzalloc(dev, mem::size_of::<q6apm_lpass_dai_data>(), GFP_KERNEL)
        as *mut q6apm_lpass_dai_data;
    if dai_data.is_null() {
        return -ENOMEM;
    }

    dev_set_drvdata(dev, dai_data as *mut c_void);
    ret = of_q6apm_parse_dai_data(dev, dai_data);
    if ret != 0 {
        return ret;
    }

    ret = devm_add_action_or_reset(dev, Some(q6apm_lpass_dai_clocks_action), dai_data as *mut c_void);
    if ret != 0 {
        return ret;
    }

    cfg.q6i2s_ops = &q6i2s_ops;
    cfg.q6dma_ops = &q6dma_ops;
    cfg.q6hdmi_ops = &q6hdmi_ops;
    cfg.q6tdm_ops = &q6tdm_ops;
    dais = q6dsp_audio_ports_set_config(dev, &mut cfg, &mut num_dais);

    devm_snd_soc_register_component(dev, &q6apm_lpass_dai_component, dais, num_dais)
}

// CONFIG_OF:
static q6apm_lpass_dai_device_id: [of_device_id; 2] = [
    of_device_id {
        compatible: c"qcom,q6apm-lpass-dais".as_ptr(),
        ..unsafe { mem::zeroed() }
    },
    unsafe { mem::zeroed() },
];
// MODULE_DEVICE_TABLE(of, q6apm_lpass_dai_device_id);

static q6apm_lpass_dai_platform_driver: platform_driver = platform_driver {
    driver: device_driver {
        name: c"q6apm-lpass-dais".as_ptr(),
        of_match_table: of_match_ptr(q6apm_lpass_dai_device_id.as_ptr()),
        ..unsafe { mem::zeroed() }
    },
    probe: Some(q6apm_lpass_dai_dev_probe),
    ..unsafe { mem::zeroed() }
};

module_platform_driver!(q6apm_lpass_dai_platform_driver);

// MODULE_DESCRIPTION("AUDIOREACH APM LPASS dai driver");
// MODULE_LICENSE("GPL");

extern "C" {
    static APM_PORT_MAX: c_int;
    static APM_PORT_MAX_AUDIO_CHAN_CNT: c_int;
    static AR_PCM_MAX_NUM_CHANNEL: c_int;
    static EINVAL: c_int;
    static ENOMEM: c_int;
    static EPROBE_DEFER: c_int;
    static GFP_KERNEL: gfp_t;
    static SNDRV_PCM_HW_PARAM_CHANNELS: c_int;
    static SNDRV_PCM_STREAM_PLAYBACK: c_int;
    static SNDRV_PCM_STREAM_CAPTURE: c_int;
    static SNDRV_PCM_TRIGGER_START: c_int;
    static SNDRV_PCM_TRIGGER_RESUME: c_int;
    static SNDRV_PCM_TRIGGER_PAUSE_RELEASE: c_int;
    static SND_SOC_COMP_ORDER_FIRST: c_int;
    static WSA_CODEC_DMA_TX_0: c_int;
    static WSA_CODEC_DMA_TX_1: c_int;
    static WSA_CODEC_DMA_TX_2: c_int;
    static VA_CODEC_DMA_TX_0: c_int;
    static VA_CODEC_DMA_TX_1: c_int;
    static VA_CODEC_DMA_TX_2: c_int;
    static TX_CODEC_DMA_TX_0: c_int;
    static TX_CODEC_DMA_TX_1: c_int;
    static TX_CODEC_DMA_TX_2: c_int;
    static TX_CODEC_DMA_TX_3: c_int;
    static TX_CODEC_DMA_TX_4: c_int;
    static TX_CODEC_DMA_TX_5: c_int;
    static WSA_CODEC_DMA_RX_0: c_int;
    static WSA_CODEC_DMA_RX_1: c_int;
    static RX_CODEC_DMA_RX_0: c_int;
    static RX_CODEC_DMA_RX_1: c_int;
    static RX_CODEC_DMA_RX_2: c_int;
    static RX_CODEC_DMA_RX_3: c_int;
    static RX_CODEC_DMA_RX_4: c_int;
    static RX_CODEC_DMA_RX_5: c_int;
    static RX_CODEC_DMA_RX_6: c_int;
    static RX_CODEC_DMA_RX_7: c_int;
    static DISPLAY_PORT_RX_0: c_int;
    static DISPLAY_PORT_RX_1: c_int;
    static DISPLAY_PORT_RX_7: c_int;
    static LPAIF_MI2S_MCLK: c_int;
    static LPAIF_MI2S_BCLK: c_int;
    static PRIMARY_TDM_RX_0: c_int;
    static QUINARY_TDM_TX_7: c_int;
    static PRIMARY_MI2S_RX: c_int;
    static QUATERNARY_MI2S_TX: c_int;
    static QUINARY_MI2S_RX: c_int;
    static QUINARY_MI2S_TX: c_int;
    static SENARY_MI2S_RX: c_int;
    static SENARY_MI2S_TX: c_int;

    fn clk_disable_unprepare(clk: *mut clk);
    fn clk_put(clk: *mut clk);
    fn clk_set_rate(clk: *mut clk, rate: c_uint) -> c_int;
    fn clk_prepare_enable(clk: *mut clk) -> c_int;
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...) -> c_int;
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
    fn hw_param_interval_c(
        params: *mut snd_pcm_hw_params,
        var: c_int,
    ) -> *const snd_interval;
    fn params_width(params: *mut snd_pcm_hw_params) -> c_int;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_int;
    fn audioreach_set_default_channel_mapping(map: *mut c_uint, channels: c_int);
    fn q6dsp_get_channel_allocation(channels: c_int) -> c_int;
    fn q6apm_graph_stop(graph: *mut q6apm_graph) -> c_int;
    fn q6apm_graph_close(graph: *mut q6apm_graph);
    fn q6apm_graph_start(graph: *mut q6apm_graph) -> c_int;
    fn q6apm_graph_open(
        dev: *mut device,
        graph_info: *mut c_void,
        graph_dev: *mut device,
        graph_id: c_int,
        dir: c_int,
    ) -> *mut q6apm_graph;
    fn q6apm_graph_media_format_pcm(
        graph: *mut q6apm_graph,
        cfg: *mut audioreach_module_config,
    ) -> c_int;
    fn q6apm_graph_prepare(graph: *mut q6apm_graph) -> c_int;
    fn of_property_read_u32(np: *mut device_node, propname: *const c_char, out_value: *mut u32) -> c_int;
    fn of_clk_get_by_name(np: *mut device_node, name: *const c_char) -> *mut clk;
    fn devm_kzalloc(dev: *mut device, size: usize, gfp: gfp_t) -> *mut c_void;
    fn devm_add_action_or_reset(
        dev: *mut device,
        action: Option<unsafe extern "C" fn(*mut c_void)>,
        data: *mut c_void,
    ) -> c_int;
    fn q6dsp_audio_ports_set_config(
        dev: *mut device,
        cfg: *mut q6dsp_audio_port_dai_driver_config,
        num_dais: *mut c_int,
    ) -> *mut snd_soc_dai_driver;
    fn devm_snd_soc_register_component(
        dev: *mut device,
        cmpnt_drv: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
    fn q6dsp_audio_ports_of_xlate_dai_name(
        component: *mut snd_soc_component,
        args: *const of_phandle_args,
        dai_name: *mut *const c_char,
    ) -> c_int;
}

enum clk {}
enum q6apm_graph {}
enum device_node {}
enum snd_pcm_hw_params {}
enum snd_soc_component {}
enum of_phandle_args {}
type gfp_t = c_uint;

#[repr(C)]
struct snd_interval {
    min: c_uint,
    max: c_uint,
    openmin: c_uint,
    openmax: c_uint,
    integer: c_uint,
    empty: c_uint,
}

#[repr(C)]
struct device {
    of_node: *mut device_node,
}

#[repr(C)]
struct platform_device {
    dev: device,
}

#[repr(C)]
struct snd_pcm_substream {
    stream: c_int,
}

#[repr(C)]
struct snd_soc_dai {
    dev: *mut device,
    id: c_int,
}

#[repr(C)]
struct audioreach_module_config {
    channel_map: [c_uint; APM_PORT_MAX_AUDIO_CHAN_CNT as usize],
    bit_width: c_int,
    sample_rate: c_int,
    num_channels: c_int,
    dp_idx: c_int,
    channel_allocation: c_int,
    direction: c_int,
    fmt: c_uint,
    nslots_per_frame: c_int,
    slot_width: c_int,
    slot_mask: c_uint,
}

#[repr(C)]
struct snd_soc_dai_ops {
    prepare: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
    startup: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
    shutdown: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai)>,
    set_channel_map: Option<
        unsafe extern "C" fn(*mut snd_soc_dai, c_uint, *const c_uint, c_uint, *const c_uint) -> c_int,
    >,
    hw_params: Option<
        unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_soc_dai) -> c_int,
    >,
    trigger: Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_int, *mut snd_soc_dai) -> c_int>,
    set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
    set_sysclk: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_uint, c_int) -> c_int>,
    set_tdm_slot: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint, c_uint, c_int, c_int) -> c_int>,
}

#[repr(C)]
struct snd_soc_component_driver {
    name: *const c_char,
    of_xlate_dai_name: Option<
        unsafe extern "C" fn(*mut snd_soc_component, *const of_phandle_args, *mut *const c_char) -> c_int,
    >,
    be_pcm_base: c_int,
    use_dai_pcm_id: bool,
    remove_order: c_int,
}

#[repr(C)]
struct q6dsp_audio_port_dai_driver_config {
    q6i2s_ops: *const snd_soc_dai_ops,
    q6dma_ops: *const snd_soc_dai_ops,
    q6hdmi_ops: *const snd_soc_dai_ops,
    q6tdm_ops: *const snd_soc_dai_ops,
}

#[repr(C)]
struct snd_soc_dai_driver {
    _private: [u8; 0],
}

#[repr(C)]
struct of_device_id {
    compatible: *const c_char,
}

#[repr(C)]
struct device_driver {
    name: *const c_char,
    of_match_table: *const of_device_id,
}

#[repr(C)]
struct platform_driver {
    driver: device_driver,
    probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
}

enum ForEach<T> {
    Continue,
    Break(T),
}

trait ForEachResult<T> {
    fn unwrap_or(self, default: T) -> T;
}

impl<T> ForEachResult<T> for Option<T> {
    fn unwrap_or(self, default: T) -> T {
        match self {
            Some(value) => value,
            None => default,
        }
    }
}

fn GENMASK(h: c_uint, l: c_uint) -> c_uint {
    (!0u32 << l) & (!0u32 >> (31 - h))
}

unsafe fn IS_ERR(ptr: *const c_void) -> bool {
    (ptr as isize) >= -4095isize
}

unsafe fn PTR_ERR(ptr: *const c_void) -> isize {
    ptr as isize
}

fn of_match_ptr(ptr: *const of_device_id) -> *const of_device_id {
    ptr
}

unsafe fn for_each_child_of_node_scoped<F>(parent: *mut device_node, mut f: F) -> Option<c_int>
where
    F: FnMut(*mut device_node) -> ForEach<c_int>,
{
    extern "C" {
        fn of_get_next_child(parent: *mut device_node, prev: *mut device_node) -> *mut device_node;
        fn of_node_put(node: *mut device_node);
    }

    let mut node = of_get_next_child(parent, ptr::null_mut());
    while !node.is_null() {
        match f(node) {
            ForEach::Continue => {
                node = of_get_next_child(parent, node);
            }
            ForEach::Break(value) => {
                of_node_put(node);
                return Some(value);
            }
        }
    }
    None
}

macro_rules! module_platform_driver {
    ($driver:ident) => {};
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
