// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2011-2017, The Linux Foundation. All rights reserved.
// Copyright (c) 2018, Linaro Limited
//
// Rust translation of soc/qcom/qdsp6/q6afe-dai.c.
// C include dependencies are intentionally left as external Rust-side symbols.

#[repr(C)]
struct q6afe_dai_priv_data {
    sd_line_mask: u32,
    sync_mode: u32,
    sync_src: u32,
    data_out_enable: u32,
    invert_sync: u32,
    data_delay: u32,
    data_align: u32,
}

#[repr(C)]
struct q6afe_dai_data {
    port: [*mut q6afe_port; AFE_PORT_MAX as usize],
    port_config: [q6afe_port_config; AFE_PORT_MAX as usize],
    is_port_started: [bool; AFE_PORT_MAX as usize],
    priv_: [q6afe_dai_priv_data; AFE_PORT_MAX as usize],
}

unsafe fn q6slim_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> i32 {
    let dai_data = dev_get_drvdata((*dai).dev) as *mut q6afe_dai_data;
    let slim = &mut (*dai_data).port_config[(*dai).id as usize].slim;

    slim.sample_rate = params_rate(params);

    match params_format(params) {
        SNDRV_PCM_FORMAT_S16_LE | SNDRV_PCM_FORMAT_SPECIAL => {
            slim.bit_width = 16;
        }
        SNDRV_PCM_FORMAT_S24_LE => {
            slim.bit_width = 24;
        }
        SNDRV_PCM_FORMAT_S32_LE => {
            slim.bit_width = 32;
        }
        _ => {
            pr_err(c"%s: format %d\n".as_ptr(), c"q6slim_hw_params".as_ptr(), params_format(params));
            return -EINVAL;
        }
    }

    0
}

unsafe fn q6hdmi_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> i32 {
    let dai_data = dev_get_drvdata((*dai).dev) as *mut q6afe_dai_data;
    let channels = params_channels(params);
    let hdmi = &mut (*dai_data).port_config[(*dai).id as usize].hdmi;
    let ret: i32;

    hdmi.sample_rate = params_rate(params);
    match params_format(params) {
        SNDRV_PCM_FORMAT_S16_LE => {
            hdmi.bit_width = 16;
        }
        SNDRV_PCM_FORMAT_S24_LE => {
            hdmi.bit_width = 24;
        }
        _ => {}
    }

    ret = q6dsp_get_channel_allocation(channels);
    if ret < 0 {
        return ret;
    }

    hdmi.channel_allocation = ret as u16;

    0
}

unsafe fn q6afe_usb_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> i32 {
    let dai_data = dev_get_drvdata((*dai).dev) as *mut q6afe_dai_data;
    let channels = params_channels(params);
    let rate = params_rate(params);
    let usb = &mut (*dai_data).port_config[(*dai).id as usize].usb_audio;

    usb.sample_rate = rate;
    usb.num_channels = channels;

    match params_format(params) {
        SNDRV_PCM_FORMAT_U16_LE | SNDRV_PCM_FORMAT_S16_LE => {
            usb.bit_width = 16;
        }
        SNDRV_PCM_FORMAT_S24_LE | SNDRV_PCM_FORMAT_S24_3LE => {
            usb.bit_width = 24;
        }
        SNDRV_PCM_FORMAT_S32_LE => {
            usb.bit_width = 32;
        }
        _ => {
            dev_err(
                (*dai).dev,
                c"%s: invalid format %d\n".as_ptr(),
                c"q6afe_usb_hw_params".as_ptr(),
                params_format(params),
            );
            return -EINVAL;
        }
    }

    0
}

unsafe fn q6i2s_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> i32 {
    let dai_data = dev_get_drvdata((*dai).dev) as *mut q6afe_dai_data;
    let i2s = &mut (*dai_data).port_config[(*dai).id as usize].i2s_cfg;

    i2s.sample_rate = params_rate(params);
    i2s.bit_width = params_width(params);
    i2s.num_channels = params_channels(params);
    i2s.sd_line_mask = (*dai_data).priv_[(*dai).id as usize].sd_line_mask;

    0
}

unsafe fn q6i2s_set_fmt(dai: *mut snd_soc_dai, fmt: u32) -> i32 {
    let dai_data = dev_get_drvdata((*dai).dev) as *mut q6afe_dai_data;
    let i2s = &mut (*dai_data).port_config[(*dai).id as usize].i2s_cfg;

    i2s.fmt = fmt;

    0
}

unsafe fn q6tdm_set_tdm_slot(
    dai: *mut snd_soc_dai,
    tx_mask: u32,
    rx_mask: u32,
    slots: i32,
    slot_width: i32,
) -> i32 {
    let dai_data = dev_get_drvdata((*dai).dev) as *mut q6afe_dai_data;
    let tdm = &mut (*dai_data).port_config[(*dai).id as usize].tdm;
    let cap_mask: u32;
    let rc = 0;

    /* HW only supports 16 and 32 bit slot width configuration */
    if slot_width != 16 && slot_width != 32 {
        dev_err((*dai).dev, c"%s: invalid slot_width %d\n".as_ptr(), c"q6tdm_set_tdm_slot".as_ptr(), slot_width);
        return -EINVAL;
    }

    /* HW supports 1-32 slots configuration. Typical: 1, 2, 4, 8, 16, 32 */
    match slots {
        2 => cap_mask = 0x03,
        4 => cap_mask = 0x0f,
        8 => cap_mask = 0xff,
        16 => cap_mask = 0xffff,
        _ => {
            dev_err((*dai).dev, c"%s: invalid slots %d\n".as_ptr(), c"q6tdm_set_tdm_slot".as_ptr(), slots);
            return -EINVAL;
        }
    }

    match (*dai).id {
        id if id >= PRIMARY_TDM_RX_0 && id <= QUINARY_TDM_TX_7 => {
            tdm.nslots_per_frame = slots;
            tdm.slot_width = slot_width;
            /* TDM RX dais ids are even and tx are odd */
            tdm.slot_mask = (if ((*dai).id & 0x1) != 0 { tx_mask } else { rx_mask }) & cap_mask;
        }
        _ => {
            dev_err((*dai).dev, c"%s: invalid dai id 0x%x\n".as_ptr(), c"q6tdm_set_tdm_slot".as_ptr(), (*dai).id);
            return -EINVAL;
        }
    }

    rc
}

unsafe fn q6tdm_set_channel_map(
    dai: *mut snd_soc_dai,
    tx_num: u32,
    tx_slot: *const u32,
    rx_num: u32,
    rx_slot: *const u32,
) -> i32 {
    let dai_data = dev_get_drvdata((*dai).dev) as *mut q6afe_dai_data;
    let tdm = &mut (*dai_data).port_config[(*dai).id as usize].tdm;
    let rc = 0;
    let mut i: i32 = 0;

    match (*dai).id {
        id if id >= PRIMARY_TDM_RX_0 && id <= QUINARY_TDM_TX_7 => {
            if ((*dai).id & 0x1) != 0 {
                if tx_slot.is_null() {
                    dev_err((*dai).dev, c"tx slot not found\n".as_ptr());
                    return -EINVAL;
                }
                if tx_num > AFE_PORT_MAX_AUDIO_CHAN_CNT {
                    dev_err((*dai).dev, c"invalid tx num %d\n".as_ptr(), tx_num);
                    return -EINVAL;
                }

                i = 0;
                while i < tx_num as i32 {
                    tdm.ch_mapping[i as usize] = *tx_slot.add(i as usize);
                    i += 1;
                }
                i = tx_num as i32;
                while i < AFE_PORT_MAX_AUDIO_CHAN_CNT as i32 {
                    tdm.ch_mapping[i as usize] = Q6AFE_CMAP_INVALID;
                    i += 1;
                }

                tdm.num_channels = tx_num;
            } else {
                /* rx */
                if rx_slot.is_null() {
                    dev_err((*dai).dev, c"rx slot not found\n".as_ptr());
                    return -EINVAL;
                }
                if rx_num > AFE_PORT_MAX_AUDIO_CHAN_CNT {
                    dev_err((*dai).dev, c"invalid rx num %d\n".as_ptr(), rx_num);
                    return -EINVAL;
                }

                i = 0;
                while i < rx_num as i32 {
                    tdm.ch_mapping[i as usize] = *rx_slot.add(i as usize);
                    i += 1;
                }
                i = rx_num as i32;
                while i < AFE_PORT_MAX_AUDIO_CHAN_CNT as i32 {
                    tdm.ch_mapping[i as usize] = Q6AFE_CMAP_INVALID;
                    i += 1;
                }

                tdm.num_channels = rx_num;
            }
        }
        _ => {
            dev_err((*dai).dev, c"%s: invalid dai id 0x%x\n".as_ptr(), c"q6tdm_set_channel_map".as_ptr(), (*dai).id);
            return -EINVAL;
        }
    }

    rc
}

unsafe fn q6tdm_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> i32 {
    let dai_data = dev_get_drvdata((*dai).dev) as *mut q6afe_dai_data;
    let tdm = &mut (*dai_data).port_config[(*dai).id as usize].tdm;

    tdm.bit_width = params_width(params);
    tdm.sample_rate = params_rate(params);
    tdm.num_channels = params_channels(params);
    tdm.data_align_type = (*dai_data).priv_[(*dai).id as usize].data_align;
    tdm.sync_src = (*dai_data).priv_[(*dai).id as usize].sync_src;
    tdm.sync_mode = (*dai_data).priv_[(*dai).id as usize].sync_mode;

    0
}

unsafe fn q6dma_set_channel_map(
    dai: *mut snd_soc_dai,
    tx_num: u32,
    tx_ch_mask: *const u32,
    rx_num: u32,
    rx_ch_mask: *const u32,
) -> i32 {
    let dai_data = dev_get_drvdata((*dai).dev) as *mut q6afe_dai_data;
    let cfg = &mut (*dai_data).port_config[(*dai).id as usize].dma_cfg;
    let ch_mask: i32;
    let rc = 0;

    match (*dai).id {
        WSA_CODEC_DMA_TX_0 | WSA_CODEC_DMA_TX_1 | WSA_CODEC_DMA_TX_2
        | VA_CODEC_DMA_TX_0 | VA_CODEC_DMA_TX_1 | VA_CODEC_DMA_TX_2
        | TX_CODEC_DMA_TX_0 | TX_CODEC_DMA_TX_1 | TX_CODEC_DMA_TX_2
        | TX_CODEC_DMA_TX_3 | TX_CODEC_DMA_TX_4 | TX_CODEC_DMA_TX_5 => {
            if tx_ch_mask.is_null() {
                dev_err((*dai).dev, c"tx slot not found\n".as_ptr());
                return -EINVAL;
            }

            if tx_num > AFE_PORT_MAX_AUDIO_CHAN_CNT {
                dev_err((*dai).dev, c"invalid tx num %d\n".as_ptr(), tx_num);
                return -EINVAL;
            }
            ch_mask = *tx_ch_mask as i32;
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
            if rx_num > AFE_PORT_MAX_AUDIO_CHAN_CNT {
                dev_err((*dai).dev, c"invalid rx num %d\n".as_ptr(), rx_num);
                return -EINVAL;
            }
            ch_mask = *rx_ch_mask as i32;
        }
        _ => {
            dev_err((*dai).dev, c"%s: invalid dai id 0x%x\n".as_ptr(), c"q6dma_set_channel_map".as_ptr(), (*dai).id);
            return -EINVAL;
        }
    }

    cfg.active_channels_mask = ch_mask;

    rc
}

unsafe fn q6dma_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> i32 {
    let dai_data = dev_get_drvdata((*dai).dev) as *mut q6afe_dai_data;
    let cfg = &mut (*dai_data).port_config[(*dai).id as usize].dma_cfg;

    cfg.bit_width = params_width(params);
    cfg.sample_rate = params_rate(params);
    cfg.num_channels = params_channels(params);

    0
}

unsafe fn q6afe_dai_shutdown(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) {
    let dai_data = dev_get_drvdata((*dai).dev) as *mut q6afe_dai_data;
    let rc: i32;

    if !(*dai_data).is_port_started[(*dai).id as usize] {
        return;
    }

    rc = q6afe_port_stop((*dai_data).port[(*dai).id as usize]);
    if rc < 0 {
        dev_err((*dai).dev, c"fail to close AFE port (%d)\n".as_ptr(), rc);
    }

    (*dai_data).is_port_started[(*dai).id as usize] = false;
}

unsafe fn q6afe_dai_prepare(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) -> i32 {
    let dai_data = dev_get_drvdata((*dai).dev) as *mut q6afe_dai_data;
    let mut rc: i32;

    if (*dai_data).is_port_started[(*dai).id as usize] {
        /* stop the port and restart with new port config */
        rc = q6afe_port_stop((*dai_data).port[(*dai).id as usize]);
        if rc < 0 {
            dev_err((*dai).dev, c"fail to close AFE port (%d)\n".as_ptr(), rc);
            return rc;
        }
    }

    match (*dai).id {
        HDMI_RX | DISPLAY_PORT_RX => {
            q6afe_hdmi_port_prepare(
                (*dai_data).port[(*dai).id as usize],
                &mut (*dai_data).port_config[(*dai).id as usize].hdmi,
            );
        }
        id if id >= SLIMBUS_0_RX && id <= SLIMBUS_6_TX => {
            q6afe_slim_port_prepare(
                (*dai_data).port[(*dai).id as usize],
                &mut (*dai_data).port_config[(*dai).id as usize].slim,
            );
        }
        id if (id >= SENARY_MI2S_RX && id <= SENARY_MI2S_TX)
            || (id >= QUINARY_MI2S_RX && id <= QUINARY_MI2S_TX)
            || (id >= PRIMARY_MI2S_RX && id <= QUATERNARY_MI2S_TX)
            || (id >= LPI_MI2S_RX_0 && id <= LPI_MI2S_TX_4)
            || (id >= LPI_MI2S_RX_5 && id <= LPI_MI2S_TX_6) =>
        {
            rc = q6afe_i2s_port_prepare(
                (*dai_data).port[(*dai).id as usize],
                &mut (*dai_data).port_config[(*dai).id as usize].i2s_cfg,
            );
            if rc < 0 {
                dev_err((*dai).dev, c"fail to prepare AFE port %x\n".as_ptr(), (*dai).id);
                return rc;
            }
        }
        id if id >= PRIMARY_TDM_RX_0 && id <= QUINARY_TDM_TX_7 => {
            q6afe_tdm_port_prepare(
                (*dai_data).port[(*dai).id as usize],
                &mut (*dai_data).port_config[(*dai).id as usize].tdm,
            );
        }
        id if id >= WSA_CODEC_DMA_RX_0 && id <= RX_CODEC_DMA_RX_7 => {
            q6afe_cdc_dma_port_prepare(
                (*dai_data).port[(*dai).id as usize],
                &mut (*dai_data).port_config[(*dai).id as usize].dma_cfg,
            );
        }
        USB_RX => {
            q6afe_usb_port_prepare(
                (*dai_data).port[(*dai).id as usize],
                &mut (*dai_data).port_config[(*dai).id as usize].usb_audio,
            );
        }
        _ => return -EINVAL,
    }

    rc = q6afe_port_start((*dai_data).port[(*dai).id as usize]);
    if rc < 0 {
        dev_err((*dai).dev, c"fail to start AFE port %x\n".as_ptr(), (*dai).id);
        return rc;
    }
    (*dai_data).is_port_started[(*dai).id as usize] = true;

    0
}

unsafe fn q6slim_set_channel_map(
    dai: *mut snd_soc_dai,
    tx_num: u32,
    tx_slot: *const u32,
    rx_num: u32,
    rx_slot: *const u32,
) -> i32 {
    let dai_data = dev_get_drvdata((*dai).dev) as *mut q6afe_dai_data;
    let pcfg = &mut (*dai_data).port_config[(*dai).id as usize];
    let mut i: i32;

    if ((*dai).id & 0x1) != 0 {
        /* TX */
        if tx_slot.is_null() {
            pr_err(c"%s: tx slot not found\n".as_ptr(), c"q6slim_set_channel_map".as_ptr());
            return -EINVAL;
        }

        i = 0;
        while i < tx_num as i32 {
            pcfg.slim.ch_mapping[i as usize] = *tx_slot.add(i as usize);
            i += 1;
        }

        pcfg.slim.num_channels = tx_num;
    } else {
        if rx_slot.is_null() {
            pr_err(c"%s: rx slot not found\n".as_ptr(), c"q6slim_set_channel_map".as_ptr());
            return -EINVAL;
        }

        i = 0;
        while i < rx_num as i32 {
            pcfg.slim.ch_mapping[i as usize] = *rx_slot.add(i as usize);
            i += 1;
        }

        pcfg.slim.num_channels = rx_num;
    }

    0
}

unsafe fn q6afe_mi2s_set_sysclk(
    dai: *mut snd_soc_dai,
    clk_id: i32,
    freq: u32,
    dir: i32,
) -> i32 {
    let dai_data = dev_get_drvdata((*dai).dev) as *mut q6afe_dai_data;
    let port = (*dai_data).port[(*dai).id as usize];

    match clk_id {
        LPAIF_DIG_CLK => return q6afe_port_set_sysclk(port, clk_id, 0, 5, freq, dir),
        LPAIF_BIT_CLK | LPAIF_OSR_CLK => {
            return q6afe_port_set_sysclk(
                port,
                clk_id,
                Q6AFE_LPASS_CLK_SRC_INTERNAL,
                Q6AFE_LPASS_CLK_ROOT_DEFAULT,
                freq,
                dir,
            );
        }
        id if (id >= Q6AFE_LPASS_CLK_ID_PRI_MI2S_IBIT && id <= Q6AFE_LPASS_CLK_ID_QUI_MI2S_OSR)
            || (id >= Q6AFE_LPASS_CLK_ID_MCLK_1 && id <= Q6AFE_LPASS_CLK_ID_INT_MCLK_1)
            || (id >= Q6AFE_LPASS_CLK_ID_WSA_CORE_MCLK && id <= Q6AFE_LPASS_CLK_ID_VA_CORE_2X_MCLK) =>
        {
            return q6afe_port_set_sysclk(
                port,
                clk_id,
                Q6AFE_LPASS_CLK_ATTRIBUTE_COUPLE_NO,
                Q6AFE_LPASS_CLK_ROOT_DEFAULT,
                freq,
                dir,
            );
        }
        id if id >= Q6AFE_LPASS_CLK_ID_PRI_TDM_IBIT && id <= Q6AFE_LPASS_CLK_ID_QUIN_TDM_EBIT => {
            return q6afe_port_set_sysclk(
                port,
                clk_id,
                Q6AFE_LPASS_CLK_ATTRIBUTE_INVERT_COUPLE_NO,
                Q6AFE_LPASS_CLK_ROOT_DEFAULT,
                freq,
                dir,
            );
        }
        _ => {}
    }

    0
}

static q6afe_dapm_routes: &[snd_soc_dapm_route] = &[
    snd_soc_dapm_route { sink: c"HDMI Playback".as_ptr(), control: core::ptr::null(), source: c"HDMI_RX".as_ptr() },
    snd_soc_dapm_route { sink: c"DISPLAY_PORT_RX_0 Playback".as_ptr(), control: core::ptr::null(), source: c"DISPLAY_PORT_RX".as_ptr() },
    snd_soc_dapm_route { sink: c"Slimbus Playback".as_ptr(), control: core::ptr::null(), source: c"SLIMBUS_0_RX".as_ptr() },
    snd_soc_dapm_route { sink: c"Slimbus1 Playback".as_ptr(), control: core::ptr::null(), source: c"SLIMBUS_1_RX".as_ptr() },
    snd_soc_dapm_route { sink: c"Slimbus2 Playback".as_ptr(), control: core::ptr::null(), source: c"SLIMBUS_2_RX".as_ptr() },
    snd_soc_dapm_route { sink: c"Slimbus3 Playback".as_ptr(), control: core::ptr::null(), source: c"SLIMBUS_3_RX".as_ptr() },
    snd_soc_dapm_route { sink: c"Slimbus4 Playback".as_ptr(), control: core::ptr::null(), source: c"SLIMBUS_4_RX".as_ptr() },
    snd_soc_dapm_route { sink: c"Slimbus5 Playback".as_ptr(), control: core::ptr::null(), source: c"SLIMBUS_5_RX".as_ptr() },
    snd_soc_dapm_route { sink: c"Slimbus6 Playback".as_ptr(), control: core::ptr::null(), source: c"SLIMBUS_6_RX".as_ptr() },
    snd_soc_dapm_route { sink: c"SLIMBUS_0_TX".as_ptr(), control: core::ptr::null(), source: c"Slimbus Capture".as_ptr() },
    snd_soc_dapm_route { sink: c"SLIMBUS_1_TX".as_ptr(), control: core::ptr::null(), source: c"Slimbus1 Capture".as_ptr() },
    snd_soc_dapm_route { sink: c"SLIMBUS_2_TX".as_ptr(), control: core::ptr::null(), source: c"Slimbus2 Capture".as_ptr() },
    snd_soc_dapm_route { sink: c"SLIMBUS_3_TX".as_ptr(), control: core::ptr::null(), source: c"Slimbus3 Capture".as_ptr() },
    snd_soc_dapm_route { sink: c"SLIMBUS_4_TX".as_ptr(), control: core::ptr::null(), source: c"Slimbus4 Capture".as_ptr() },
    snd_soc_dapm_route { sink: c"SLIMBUS_5_TX".as_ptr(), control: core::ptr::null(), source: c"Slimbus5 Capture".as_ptr() },
    snd_soc_dapm_route { sink: c"SLIMBUS_6_TX".as_ptr(), control: core::ptr::null(), source: c"Slimbus6 Capture".as_ptr() },
    snd_soc_dapm_route { sink: c"Primary MI2S Playback".as_ptr(), control: core::ptr::null(), source: c"PRI_MI2S_RX".as_ptr() },
    snd_soc_dapm_route { sink: c"Secondary MI2S Playback".as_ptr(), control: core::ptr::null(), source: c"SEC_MI2S_RX".as_ptr() },
    snd_soc_dapm_route { sink: c"Tertiary MI2S Playback".as_ptr(), control: core::ptr::null(), source: c"TERT_MI2S_RX".as_ptr() },
    snd_soc_dapm_route { sink: c"Quaternary MI2S Playback".as_ptr(), control: core::ptr::null(), source: c"QUAT_MI2S_RX".as_ptr() },
    snd_soc_dapm_route { sink: c"Quinary MI2S Playback".as_ptr(), control: core::ptr::null(), source: c"QUIN_MI2S_RX".as_ptr() },
    snd_soc_dapm_route { sink: c"Senary MI2S Playback".as_ptr(), control: core::ptr::null(), source: c"SEN_MI2S_RX".as_ptr() },
    route_range_comments! {
        "Primary/Secondary/Tertiary/Quaternary/Quinary TDM playback routes PRIMARY_TDM_RX_0 through QUIN_TDM_RX_7";
        "Primary/Secondary/Tertiary/Quaternary/Quinary TDM capture routes PRIMARY_TDM_TX_0 through QUIN_TDM_TX_7";
    },
    snd_soc_dapm_route { sink: c"TERT_MI2S_TX".as_ptr(), control: core::ptr::null(), source: c"Tertiary MI2S Capture".as_ptr() },
    snd_soc_dapm_route { sink: c"PRI_MI2S_TX".as_ptr(), control: core::ptr::null(), source: c"Primary MI2S Capture".as_ptr() },
    snd_soc_dapm_route { sink: c"SEC_MI2S_TX".as_ptr(), control: core::ptr::null(), source: c"Secondary MI2S Capture".as_ptr() },
    snd_soc_dapm_route { sink: c"QUAT_MI2S_TX".as_ptr(), control: core::ptr::null(), source: c"Quaternary MI2S Capture".as_ptr() },
    snd_soc_dapm_route { sink: c"QUIN_MI2S_TX".as_ptr(), control: core::ptr::null(), source: c"Quinary MI2S Capture".as_ptr() },
    snd_soc_dapm_route { sink: c"SEN_MI2S_TX".as_ptr(), control: core::ptr::null(), source: c"Senary MI2S Capture".as_ptr() },
    dma_route_table_entries!(),
    /* USB playback AFE port receives data for playback, hence use the RX port */
    snd_soc_dapm_route { sink: c"USB Playback".as_ptr(), control: core::ptr::null(), source: c"USB_RX".as_ptr() },
    lpi_mi2s_route_table_entries!(),
];

unsafe fn msm_dai_q6_dai_probe(dai: *mut snd_soc_dai) -> i32 {
    let dai_data = dev_get_drvdata((*dai).dev) as *mut q6afe_dai_data;
    let port: *mut q6afe_port;

    port = q6afe_port_get_from_id((*dai).dev, (*dai).id);
    if IS_ERR(port) {
        dev_err((*dai).dev, c"Unable to get afe port\n".as_ptr());
        return -EINVAL;
    }
    (*dai_data).port[(*dai).id as usize] = port;

    0
}

unsafe fn msm_dai_q6_dai_remove(dai: *mut snd_soc_dai) -> i32 {
    let dai_data = dev_get_drvdata((*dai).dev) as *mut q6afe_dai_data;

    q6afe_port_put((*dai_data).port[(*dai).id as usize]);
    (*dai_data).port[(*dai).id as usize] = core::ptr::null_mut();

    0
}

static q6afe_usb_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    probe: Some(msm_dai_q6_dai_probe),
    prepare: Some(q6afe_dai_prepare),
    hw_params: Some(q6afe_usb_hw_params),
    /*
     * Shutdown callback required to stop the USB AFE port, which is enabled
     * by the prepare() stage.  This stops the audio traffic on the USB AFE
     * port on the Q6DSP.
     */
    shutdown: Some(q6afe_dai_shutdown),
    /*
     * Startup callback not needed, as AFE port start command passes the PCM
     * parameters within the AFE command, which is provided by the PCM core
     * during the prepare() stage.
     */
    ..snd_soc_dai_ops::ZERO
};

static q6hdmi_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    probe: Some(msm_dai_q6_dai_probe),
    remove: Some(msm_dai_q6_dai_remove),
    prepare: Some(q6afe_dai_prepare),
    hw_params: Some(q6hdmi_hw_params),
    shutdown: Some(q6afe_dai_shutdown),
    ..snd_soc_dai_ops::ZERO
};

static q6i2s_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    probe: Some(msm_dai_q6_dai_probe),
    remove: Some(msm_dai_q6_dai_remove),
    prepare: Some(q6afe_dai_prepare),
    hw_params: Some(q6i2s_hw_params),
    set_fmt: Some(q6i2s_set_fmt),
    shutdown: Some(q6afe_dai_shutdown),
    set_sysclk: Some(q6afe_mi2s_set_sysclk),
    ..snd_soc_dai_ops::ZERO
};

static q6slim_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    probe: Some(msm_dai_q6_dai_probe),
    remove: Some(msm_dai_q6_dai_remove),
    prepare: Some(q6afe_dai_prepare),
    hw_params: Some(q6slim_hw_params),
    shutdown: Some(q6afe_dai_shutdown),
    set_channel_map: Some(q6slim_set_channel_map),
    ..snd_soc_dai_ops::ZERO
};

static q6tdm_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    probe: Some(msm_dai_q6_dai_probe),
    remove: Some(msm_dai_q6_dai_remove),
    prepare: Some(q6afe_dai_prepare),
    shutdown: Some(q6afe_dai_shutdown),
    set_sysclk: Some(q6afe_mi2s_set_sysclk),
    set_tdm_slot: Some(q6tdm_set_tdm_slot),
    set_channel_map: Some(q6tdm_set_channel_map),
    hw_params: Some(q6tdm_hw_params),
    ..snd_soc_dai_ops::ZERO
};

static q6dma_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    probe: Some(msm_dai_q6_dai_probe),
    remove: Some(msm_dai_q6_dai_remove),
    prepare: Some(q6afe_dai_prepare),
    shutdown: Some(q6afe_dai_shutdown),
    set_sysclk: Some(q6afe_mi2s_set_sysclk),
    set_channel_map: Some(q6dma_set_channel_map),
    hw_params: Some(q6dma_hw_params),
    ..snd_soc_dai_ops::ZERO
};

static q6afe_dai_widgets: &[snd_soc_dapm_widget] = &[
    SND_SOC_DAPM_AIF_IN!(c"HDMI_RX", core::ptr::null(), 0, SND_SOC_NOPM, 0, 0),
    slimbus_dapm_widget_entries!(),
    mi2s_dapm_widget_entries!(),
    tdm_dapm_widget_entries!(),
    SND_SOC_DAPM_AIF_OUT!(c"DISPLAY_PORT_RX", c"NULL", 0, SND_SOC_NOPM, 0, 0),
    codec_dma_dapm_widget_entries!(),
    SND_SOC_DAPM_AIF_IN!(c"USB_RX", core::ptr::null(), 0, SND_SOC_NOPM, 0, 0),
    lpi_mi2s_dapm_widget_entries!(),
];

static q6afe_dai_component: snd_soc_component_driver = snd_soc_component_driver {
    name: c"q6afe-dai-component".as_ptr(),
    dapm_widgets: q6afe_dai_widgets.as_ptr(),
    num_dapm_widgets: q6afe_dai_widgets.len(),
    dapm_routes: q6afe_dapm_routes.as_ptr(),
    num_dapm_routes: q6afe_dapm_routes.len(),
    of_xlate_dai_name: Some(q6dsp_audio_ports_of_xlate_dai_name),
    ..snd_soc_component_driver::ZERO
};

unsafe fn of_q6afe_parse_dai_data(dev: *mut device, data: *mut q6afe_dai_data) {
    let mut node: *mut device_node;
    let mut ret: i32;

    for_each_child_of_node!((*dev).of_node, node, {
        let mut lines: [u32; Q6AFE_MAX_MI2S_LINES as usize] = [0; Q6AFE_MAX_MI2S_LINES as usize];
        let priv_: *mut q6afe_dai_priv_data;
        let mut id: i32 = 0;
        let mut i: i32;
        let num_lines: i32;

        ret = of_property_read_u32(node, c"reg".as_ptr(), &mut id as *mut i32 as *mut u32);
        if ret != 0 || id < 0 || id >= AFE_PORT_MAX as i32 {
            dev_err(dev, c"valid dai id not found:%d\n".as_ptr(), ret);
            continue;
        }

        match id {
            /* MI2S specific properties */
            id if (id >= SENARY_MI2S_RX && id <= SENARY_MI2S_TX)
                || (id >= QUINARY_MI2S_RX && id <= QUINARY_MI2S_TX)
                || (id >= PRIMARY_MI2S_RX && id <= QUATERNARY_MI2S_TX)
                || (id >= LPI_MI2S_RX_0 && id <= LPI_MI2S_TX_4)
                || (id >= LPI_MI2S_RX_5 && id <= LPI_MI2S_TX_6) =>
            {
                priv_ = &mut (*data).priv_[id as usize];
                ret = of_property_read_variable_u32_array(
                    node,
                    c"qcom,sd-lines".as_ptr(),
                    lines.as_mut_ptr(),
                    0,
                    Q6AFE_MAX_MI2S_LINES,
                );
                if ret < 0 {
                    num_lines = 0;
                } else {
                    num_lines = ret;
                }

                (*priv_).sd_line_mask = 0;

                i = 0;
                while i < num_lines {
                    (*priv_).sd_line_mask |= BIT(lines[i as usize]);
                    i += 1;
                }
            }
            id if id >= PRIMARY_TDM_RX_0 && id <= QUINARY_TDM_TX_7 => {
                priv_ = &mut (*data).priv_[id as usize];
                ret = of_property_read_u32(node, c"qcom,tdm-sync-mode".as_ptr(), &mut (*priv_).sync_mode);
                if ret != 0 {
                    dev_err(dev, c"No Sync mode from DT\n".as_ptr());
                    continue;
                }
                ret = of_property_read_u32(node, c"qcom,tdm-sync-src".as_ptr(), &mut (*priv_).sync_src);
                if ret != 0 {
                    dev_err(dev, c"No Sync Src from DT\n".as_ptr());
                    continue;
                }
                ret = of_property_read_u32(node, c"qcom,tdm-data-out".as_ptr(), &mut (*priv_).data_out_enable);
                if ret != 0 {
                    dev_err(dev, c"No Data out enable from DT\n".as_ptr());
                    continue;
                }
                ret = of_property_read_u32(node, c"qcom,tdm-invert-sync".as_ptr(), &mut (*priv_).invert_sync);
                if ret != 0 {
                    dev_err(dev, c"No Invert sync from DT\n".as_ptr());
                    continue;
                }
                ret = of_property_read_u32(node, c"qcom,tdm-data-delay".as_ptr(), &mut (*priv_).data_delay);
                if ret != 0 {
                    dev_err(dev, c"No Data Delay from DT\n".as_ptr());
                    continue;
                }
                ret = of_property_read_u32(node, c"qcom,tdm-data-align".as_ptr(), &mut (*priv_).data_align);
                if ret != 0 {
                    dev_err(dev, c"No Data align from DT\n".as_ptr());
                    continue;
                }
            }
            _ => {}
        }
    });
}

unsafe fn q6afe_dai_dev_probe(pdev: *mut platform_device) -> i32 {
    let mut cfg: q6dsp_audio_port_dai_driver_config = core::mem::zeroed();
    let dais: *mut snd_soc_dai_driver;
    let dai_data: *mut q6afe_dai_data;
    let dev = &mut (*pdev).dev as *mut device;
    let mut num_dais: i32 = 0;

    dai_data = devm_kzalloc(dev, core::mem::size_of::<q6afe_dai_data>(), GFP_KERNEL) as *mut q6afe_dai_data;
    if dai_data.is_null() {
        return -ENOMEM;
    }

    dev_set_drvdata(dev, dai_data as *mut core::ffi::c_void);
    of_q6afe_parse_dai_data(dev, dai_data);

    cfg.q6hdmi_ops = &q6hdmi_ops;
    cfg.q6slim_ops = &q6slim_ops;
    cfg.q6i2s_ops = &q6i2s_ops;
    cfg.q6tdm_ops = &q6tdm_ops;
    cfg.q6dma_ops = &q6dma_ops;
    cfg.q6usb_ops = &q6afe_usb_ops;
    dais = q6dsp_audio_ports_set_config(dev, &mut cfg, &mut num_dais);

    devm_snd_soc_register_component(dev, &q6afe_dai_component, dais, num_dais)
}

// Original C guarded this table with CONFIG_OF.
#[cfg(CONFIG_OF)]
static q6afe_dai_device_id: &[of_device_id] = &[
    of_device_id { compatible: c"qcom,q6afe-dais".as_ptr(), ..of_device_id::ZERO },
    of_device_id::ZERO,
];
#[cfg(CONFIG_OF)]
MODULE_DEVICE_TABLE!(of, q6afe_dai_device_id);

static q6afe_dai_platform_driver: platform_driver = platform_driver {
    driver: device_driver {
        name: c"q6afe-dai".as_ptr(),
        of_match_table: of_match_ptr(q6afe_dai_device_id.as_ptr()),
        ..device_driver::ZERO
    },
    probe: Some(q6afe_dai_dev_probe),
    ..platform_driver::ZERO
};

module_platform_driver!(q6afe_dai_platform_driver);

MODULE_DESCRIPTION!(c"Q6 Audio Frontend dai driver");
MODULE_LICENSE!(c"GPL v2");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
