// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
//
// This file is provided under a dual BSD/GPLv2 license.  When using or
// redistributing this file, you may do so under either license.
//
// Copyright(c) 2022 Intel Corporation

// C dependencies:
// sound/pcm_params.h, sound/hdaudio_ext.h, sound/hda_register.h,
// sound/hda-mlink.h, sound/sof/ipc4/header.h, uapi/sound/sof/header.h,
// ipc4-priv.h, ipc4-topology.h, sof-priv.h, sof-audio.h, hda.h

/* These ops are only applicable for the HDA DAI's in their current form */
// C conditional: #if IS_ENABLED(CONFIG_SND_SOC_SOF_HDA_LINK)

/*
 * This function checks if the host DMA stream corresponding
 * to the link DMA stream_tag argument is assigned to one
 * of the FEs connected to the BE DAI.
 */
unsafe fn hda_check_fes(
    rtd: *mut snd_soc_pcm_runtime,
    dir: c_int,
    stream_tag: c_int,
) -> bool {
    let mut fe_substream: *mut snd_pcm_substream;
    let mut fe_hstream: *mut hdac_stream;
    let mut dpcm: *mut snd_soc_dpcm;

    for_each_dpcm_fe!(rtd, dir, dpcm, {
        fe_substream = snd_soc_dpcm_get_substream((*dpcm).fe, dir);
        fe_hstream = (*(*fe_substream).runtime).private_data as *mut hdac_stream;
        if (*fe_hstream).stream_tag == stream_tag {
            return true;
        }
    });

    false
}

unsafe fn hda_link_stream_assign(
    bus: *mut hdac_bus,
    substream: *mut snd_pcm_substream,
    link_type: hda_bus_ml_link_type,
) -> *mut hdac_ext_stream {
    let rtd: *mut snd_soc_pcm_runtime = snd_soc_substream_to_rtd(substream);
    let sof_hda: *mut sof_intel_hda_dev = bus_to_sof_hda(bus);
    let mut hda_stream: *mut sof_intel_hda_stream;
    let mut chip: *const sof_intel_dsp_desc;
    let mut sdev: *mut snd_sof_dev;
    let mut res: *mut hdac_ext_stream = core::ptr::null_mut();
    let mut hstream: *mut hdac_stream = core::ptr::null_mut();
    let stream_dir: c_int = (*substream).stream;
    let is_multi: bool =
        link_type == HDA_BUS_ML_LINK_HDA || link_type == HDA_BUS_ML_LINK_UAOL;
    let is_play: bool = stream_dir == SNDRV_PCM_STREAM_PLAYBACK;
    let is_sdw: bool = link_type == HDA_BUS_ML_LINK_SDW;
    let is_hda: bool = link_type == HDA_BUS_ML_LINK_HDA;
    let mut concur_block_mask: u32 = 0;
    let mut seq_block_mask: u32 = 0;
    let mut stream_idx: c_uint;

    if (*bus).ppcap == 0 {
        dev_err!((*bus).dev, "stream type not supported\n");
        return core::ptr::null_mut();
    }

    /*
     * On ACE2+ the link DMA stream allocator must avoid two HW errata,
     * see the comment on struct sof_intel_hda_dev.
     *
     * - Concurrent cross-direction: SoundWire conflicts with HDA, iDisp
     *   and UAOL on the same physical stream index; SSP and DMIC are safe.
     * - Sequential playback: a stream index previously used by an HDA/iDisp
     *   link cannot drive any non-HDA/iDisp link in the same direction
     *   until the next controller reset.
     *
     * The masks are protected by bus->reg_lock; sample them inside the
     * lock together with the stream walk to keep the decision atomic
     * with concurrent allocations and releases.
     */
    guard!(spinlock_irq, &mut (*bus).reg_lock);

    if is_sdw {
        concur_block_mask = (*sof_hda).link_dma_active_multi_mask[(!stream_dir) as usize];
    } else if is_multi {
        concur_block_mask = (*sof_hda).link_dma_active_sdw_mask[(!stream_dir) as usize];
    }
    if is_play && !is_hda {
        seq_block_mask = (*sof_hda).link_dma_out_hda_used_mask;
    }

    list_for_each_entry!(hstream, &mut (*bus).stream_list, list, {
        let hext_stream: *mut hdac_ext_stream = stream_to_hdac_ext_stream(hstream);
        if (*hstream).direction != (*substream).stream {
            continue;
        }

        hda_stream = hstream_to_sof_hda_stream(hext_stream);
        sdev = (*hda_stream).sdev;
        chip = get_chip_info((*sdev).pdata);

        stream_idx = ((*hstream).stream_tag - 1) as c_uint;

        /* skip streams blocked by the ACE2+ allocator constraints */
        if ((concur_block_mask | seq_block_mask) & BIT(stream_idx)) != 0 {
            continue;
        }

        /* check if link is available */
        if (*hext_stream).link_locked == 0 {
            /*
             * choose the first available link for platforms that do not have the
             * PROCEN_FMT_QUIRK set.
             */
            if ((*chip).quirks & SOF_INTEL_PROCEN_FMT_QUIRK) == 0 {
                res = hext_stream;
                break;
            }

            if (*hstream).opened != 0 {
                /*
                 * check if the stream tag matches the stream
                 * tag of one of the connected FEs
                 */
                if hda_check_fes(rtd, stream_dir, (*hstream).stream_tag) {
                    res = hext_stream;
                    break;
                }
            } else {
                res = hext_stream;

                /*
                 * This must be a hostless stream.
                 * So reserve the host DMA stream.
                 */
                (*hda_stream).host_reserved = 1;
                break;
            }
        }
    });

    if !res.is_null() {
        /* Make sure that host and link DMA is decoupled. */
        snd_hdac_ext_stream_decouple_locked(bus, res, true);

        (*res).link_locked = 1;
        (*res).link_substream = substream;

        stream_idx = ((*res).hstream.stream_tag - 1) as c_uint;
        if is_sdw {
            (*sof_hda).link_dma_active_sdw_mask[stream_dir as usize] |= BIT(stream_idx);
        } else if is_multi {
            (*sof_hda).link_dma_active_multi_mask[stream_dir as usize] |= BIT(stream_idx);
        }

        /* persistent OUT HDA/iDisp shadow, cleared only on CRST# */
        if is_hda && is_play {
            (*sof_hda).link_dma_out_hda_used_mask |= BIT(stream_idx);
        }
    }

    res
}

unsafe fn hda_get_hext_stream(
    _sdev: *mut snd_sof_dev,
    cpu_dai: *mut snd_soc_dai,
    substream: *mut snd_pcm_substream,
) -> *mut hdac_ext_stream {
    snd_soc_dai_get_dma_data(cpu_dai, substream) as *mut hdac_ext_stream
}

unsafe fn hda_ipc4_get_hext_stream(
    _sdev: *mut snd_sof_dev,
    cpu_dai: *mut snd_soc_dai,
    substream: *mut snd_pcm_substream,
) -> *mut hdac_ext_stream {
    let pipe_widget: *mut snd_sof_widget;
    let pipeline: *mut sof_ipc4_pipeline;
    let swidget: *mut snd_sof_widget;
    let w: *mut snd_soc_dapm_widget;

    w = snd_soc_dai_get_widget(cpu_dai, (*substream).stream);
    swidget = (*w).dobj.private as *mut snd_sof_widget;
    pipe_widget = (*(*swidget).spipe).pipe_widget;
    pipeline = (*pipe_widget).private as *mut sof_ipc4_pipeline;

    /* mark pipeline so that it can be skipped during FE trigger */
    (*pipeline).skip_during_fe_trigger = true;

    snd_soc_dai_get_dma_data(cpu_dai, substream) as *mut hdac_ext_stream
}

unsafe fn hda_assign_hext_stream(
    sdev: *mut snd_sof_dev,
    cpu_dai: *mut snd_soc_dai,
    substream: *mut snd_pcm_substream,
    hlink: *mut hdac_ext_link,
) -> *mut hdac_ext_stream {
    let hext_stream: *mut hdac_ext_stream;
    let link_type: hda_bus_ml_link_type = hda_bus_ml_link_get_type(hlink);

    hext_stream = hda_link_stream_assign(sof_to_bus(sdev), substream, link_type);
    if hext_stream.is_null() {
        return core::ptr::null_mut();
    }

    snd_soc_dai_set_dma_data(cpu_dai, substream, hext_stream as *mut c_void);

    hext_stream
}

unsafe fn hda_release_hext_stream(
    sdev: *mut snd_sof_dev,
    cpu_dai: *mut snd_soc_dai,
    substream: *mut snd_pcm_substream,
) {
    let hext_stream: *mut hdac_ext_stream = hda_get_hext_stream(sdev, cpu_dai, substream);
    let sof_hda: *mut sof_intel_hda_dev = (*(*sdev).pdata).hw_pdata as *mut sof_intel_hda_dev;
    let bus: *mut hdac_bus = sof_to_bus(sdev);
    let dir: c_int = (*substream).stream;
    let stream_idx: c_uint = ((*hext_stream).hstream.stream_tag - 1) as c_uint;

    /*
     * Drop the stream index from the per-direction active concurrency masks.
     * The two masks are mutually exclusive for a given stream/direction
     * (and a stream of the SSP/DMIC kind appears in neither), so a blind
     * clear of both is safe and lets us avoid having to remember the
     * link type at allocation time.
     */
    scoped_guard!(spinlock_irq, &mut (*bus).reg_lock, {
        (*sof_hda).link_dma_active_sdw_mask[dir as usize] &= !BIT(stream_idx);
        (*sof_hda).link_dma_active_multi_mask[dir as usize] &= !BIT(stream_idx);
    });

    snd_soc_dai_set_dma_data(cpu_dai, substream, core::ptr::null_mut());
    snd_hdac_ext_stream_release(hext_stream, HDAC_EXT_STREAM_TYPE_LINK);
}

unsafe fn hda_setup_hext_stream(
    _sdev: *mut snd_sof_dev,
    hext_stream: *mut hdac_ext_stream,
    format_val: c_uint,
) {
    snd_hdac_ext_stream_setup(hext_stream, format_val);
}

unsafe fn hda_reset_hext_stream(_sdev: *mut snd_sof_dev, hext_stream: *mut hdac_ext_stream) {
    snd_hdac_ext_stream_reset(hext_stream);
}

unsafe fn hda_codec_dai_set_stream(
    _sdev: *mut snd_sof_dev,
    substream: *mut snd_pcm_substream,
    hstream: *mut hdac_stream,
) {
    let rtd: *mut snd_soc_pcm_runtime = snd_soc_substream_to_rtd(substream);
    let codec_dai: *mut snd_soc_dai = snd_soc_rtd_to_codec(rtd, 0);

    /* set the hdac_stream in the codec dai */
    snd_soc_dai_set_stream(codec_dai, hstream as *mut c_void, (*substream).stream);
}

unsafe fn hda_calc_stream_format(
    sdev: *mut snd_sof_dev,
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
) -> c_uint {
    let rtd: *mut snd_soc_pcm_runtime = snd_soc_substream_to_rtd(substream);
    let codec_dai: *mut snd_soc_dai = snd_soc_rtd_to_codec(rtd, 0);
    let link_bps: c_uint;
    let format_val: c_uint;
    let bits: c_uint;

    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        link_bps = (*(*(*codec_dai).driver).playback).sig_bits;
    } else {
        link_bps = (*(*(*codec_dai).driver).capture).sig_bits;
    }

    bits = snd_hdac_stream_format_bits(
        params_format(params),
        SNDRV_PCM_SUBFORMAT_STD,
        link_bps,
    );
    format_val = snd_hdac_stream_format(params_channels(params), bits, params_rate(params));

    dev_dbg!(
        (*sdev).dev,
        "format_val=%#x, rate=%d, ch=%d, format=%d\n",
        format_val,
        params_rate(params),
        params_channels(params),
        params_format(params)
    );

    format_val
}

unsafe fn hda_get_hlink(
    sdev: *mut snd_sof_dev,
    substream: *mut snd_pcm_substream,
) -> *mut hdac_ext_link {
    let rtd: *mut snd_soc_pcm_runtime = snd_soc_substream_to_rtd(substream);
    let codec_dai: *mut snd_soc_dai = snd_soc_rtd_to_codec(rtd, 0);
    let bus: *mut hdac_bus = sof_to_bus(sdev);

    snd_hdac_ext_bus_get_hlink_by_name(bus, (*(*codec_dai).component).name)
}

unsafe fn generic_calc_stream_format(
    sdev: *mut snd_sof_dev,
    _substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
) -> c_uint {
    let format_val: c_uint;
    let bits: c_uint;

    bits = snd_hdac_stream_format_bits(
        params_format(params),
        SNDRV_PCM_SUBFORMAT_STD,
        params_physical_width(params),
    );
    format_val = snd_hdac_stream_format(params_channels(params), bits, params_rate(params));

    dev_dbg!(
        (*sdev).dev,
        "format_val=%#x, rate=%d, ch=%d, format=%d\n",
        format_val,
        params_rate(params),
        params_channels(params),
        params_format(params)
    );

    format_val
}

unsafe fn dmic_calc_stream_format(
    sdev: *mut snd_sof_dev,
    _substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
) -> c_uint {
    let format_val: c_uint;
    let mut format: snd_pcm_format_t;
    let mut channels: c_uint;
    let mut width: c_uint;
    let bits: c_uint;

    channels = params_channels(params);
    format = params_format(params);
    width = params_physical_width(params);

    if format == SNDRV_PCM_FORMAT_S16_LE {
        format = SNDRV_PCM_FORMAT_S32_LE;
        channels /= 2;
        width = 32;
    }

    bits = snd_hdac_stream_format_bits(format, SNDRV_PCM_SUBFORMAT_STD, width);
    format_val = snd_hdac_stream_format(channels, bits, params_rate(params));

    dev_dbg!(
        (*sdev).dev,
        "format_val=%#x, rate=%d, ch=%d, format=%d\n",
        format_val,
        params_rate(params),
        channels,
        format
    );

    format_val
}

unsafe fn ssp_get_hlink(
    sdev: *mut snd_sof_dev,
    _substream: *mut snd_pcm_substream,
) -> *mut hdac_ext_link {
    let bus: *mut hdac_bus = sof_to_bus(sdev);

    hdac_bus_eml_ssp_get_hlink(bus)
}

unsafe fn dmic_get_hlink(
    sdev: *mut snd_sof_dev,
    _substream: *mut snd_pcm_substream,
) -> *mut hdac_ext_link {
    let bus: *mut hdac_bus = sof_to_bus(sdev);

    hdac_bus_eml_dmic_get_hlink(bus)
}

unsafe fn sdw_get_hlink(
    sdev: *mut snd_sof_dev,
    _substream: *mut snd_pcm_substream,
) -> *mut hdac_ext_link {
    let bus: *mut hdac_bus = sof_to_bus(sdev);

    hdac_bus_eml_sdw_get_hlink(bus)
}

unsafe fn hda_ipc4_pre_trigger(
    sdev: *mut snd_sof_dev,
    cpu_dai: *mut snd_soc_dai,
    substream: *mut snd_pcm_substream,
    cmd: c_int,
) -> c_int {
    let ipc4_data: *mut sof_ipc4_fw_data = (*sdev).private as *mut sof_ipc4_fw_data;
    let pipe_widget: *mut snd_sof_widget;
    let pipeline: *mut sof_ipc4_pipeline;
    let swidget: *mut snd_sof_widget;
    let w: *mut snd_soc_dapm_widget;
    let mut ret: c_int = 0;

    w = snd_soc_dai_get_widget(cpu_dai, (*substream).stream);
    swidget = (*w).dobj.private as *mut snd_sof_widget;
    pipe_widget = (*(*swidget).spipe).pipe_widget;
    pipeline = (*pipe_widget).private as *mut sof_ipc4_pipeline;

    if (*pipe_widget).instance_id < 0 {
        return 0;
    }

    guard!(mutex, &mut (*ipc4_data).pipeline_state_mutex);

    match cmd {
        SNDRV_PCM_TRIGGER_START | SNDRV_PCM_TRIGGER_PAUSE_RELEASE => {}
        SNDRV_PCM_TRIGGER_PAUSE_PUSH | SNDRV_PCM_TRIGGER_SUSPEND | SNDRV_PCM_TRIGGER_STOP => {
            ret = sof_ipc4_set_pipeline_state(
                sdev,
                (*pipe_widget).instance_id,
                SOF_IPC4_PIPE_PAUSED,
            );
            if ret < 0 {
                return ret;
            }

            (*pipeline).state = SOF_IPC4_PIPE_PAUSED;
        }
        _ => {
            dev_err!((*sdev).dev, "unknown trigger command %d\n", cmd);
            ret = -EINVAL;
        }
    }

    ret
}

unsafe fn hda_trigger(
    sdev: *mut snd_sof_dev,
    cpu_dai: *mut snd_soc_dai,
    substream: *mut snd_pcm_substream,
    cmd: c_int,
) -> c_int {
    let hext_stream: *mut hdac_ext_stream =
        snd_soc_dai_get_dma_data(cpu_dai, substream) as *mut hdac_ext_stream;

    match cmd {
        SNDRV_PCM_TRIGGER_START | SNDRV_PCM_TRIGGER_PAUSE_RELEASE => {
            snd_hdac_ext_stream_start(hext_stream);
        }
        SNDRV_PCM_TRIGGER_PAUSE_PUSH => {
            /*
             * Save the LLP registers since in case of PAUSE the LLP
             * register are not reset to 0, the delay calculation will use
             * the saved offsets for compensating the delay calculation.
             */
            (*hext_stream).pplcllpl = readl((*hext_stream).pplc_addr.add(AZX_REG_PPLCLLPL));
            (*hext_stream).pplcllpu = readl((*hext_stream).pplc_addr.add(AZX_REG_PPLCLLPU));
            snd_hdac_ext_stream_clear(hext_stream);
        }
        SNDRV_PCM_TRIGGER_SUSPEND | SNDRV_PCM_TRIGGER_STOP => {
            (*hext_stream).pplcllpl = 0;
            (*hext_stream).pplcllpu = 0;
            snd_hdac_ext_stream_clear(hext_stream);
        }
        _ => {
            dev_err!((*sdev).dev, "unknown trigger command %d\n", cmd);
            return -EINVAL;
        }
    }

    0
}

unsafe fn hda_ipc4_post_trigger(
    sdev: *mut snd_sof_dev,
    cpu_dai: *mut snd_soc_dai,
    substream: *mut snd_pcm_substream,
    cmd: c_int,
) -> c_int {
    let ipc4_data: *mut sof_ipc4_fw_data = (*sdev).private as *mut sof_ipc4_fw_data;
    let pipe_widget: *mut snd_sof_widget;
    let pipeline: *mut sof_ipc4_pipeline;
    let swidget: *mut snd_sof_widget;
    let w: *mut snd_soc_dapm_widget;
    let mut ret: c_int = 0;

    w = snd_soc_dai_get_widget(cpu_dai, (*substream).stream);
    swidget = (*w).dobj.private as *mut snd_sof_widget;
    pipe_widget = (*(*swidget).spipe).pipe_widget;
    pipeline = (*pipe_widget).private as *mut sof_ipc4_pipeline;

    if (*pipe_widget).instance_id < 0 {
        return 0;
    }

    guard!(mutex, &mut (*ipc4_data).pipeline_state_mutex);

    match cmd {
        SNDRV_PCM_TRIGGER_START => {
            if (*pipeline).state != SOF_IPC4_PIPE_PAUSED {
                ret = sof_ipc4_set_pipeline_state(
                    sdev,
                    (*pipe_widget).instance_id,
                    SOF_IPC4_PIPE_PAUSED,
                );
                if ret < 0 {
                    return ret;
                }

                (*pipeline).state = SOF_IPC4_PIPE_PAUSED;
            }

            ret = sof_ipc4_set_pipeline_state(
                sdev,
                (*pipe_widget).instance_id,
                SOF_IPC4_PIPE_RUNNING,
            );
            if ret < 0 {
                return ret;
            }

            (*pipeline).state = SOF_IPC4_PIPE_RUNNING;
            (*(*swidget).spipe).started_count += 1;
        }
        SNDRV_PCM_TRIGGER_PAUSE_RELEASE => {
            ret = sof_ipc4_set_pipeline_state(
                sdev,
                (*pipe_widget).instance_id,
                SOF_IPC4_PIPE_RUNNING,
            );
            if ret < 0 {
                return ret;
            }

            (*pipeline).state = SOF_IPC4_PIPE_RUNNING;
        }
        SNDRV_PCM_TRIGGER_SUSPEND | SNDRV_PCM_TRIGGER_STOP => {
            /*
             * STOP/SUSPEND trigger is invoked only once when all users of this pipeline have
             * been stopped. So, clear the started_count so that the pipeline can be reset
             */
            (*(*swidget).spipe).started_count = 0;
        }
        SNDRV_PCM_TRIGGER_PAUSE_PUSH => {}
        _ => {
            dev_err!((*sdev).dev, "unknown trigger command %d\n", cmd);
            ret = -EINVAL;
        }
    }

    ret
}

static hda_ipc4_dma_ops: hda_dai_widget_dma_ops = hda_dai_widget_dma_ops {
    get_hext_stream: Some(hda_ipc4_get_hext_stream),
    assign_hext_stream: Some(hda_assign_hext_stream),
    release_hext_stream: Some(hda_release_hext_stream),
    setup_hext_stream: Some(hda_setup_hext_stream),
    reset_hext_stream: Some(hda_reset_hext_stream),
    pre_trigger: Some(hda_ipc4_pre_trigger),
    trigger: Some(hda_trigger),
    post_trigger: Some(hda_ipc4_post_trigger),
    codec_dai_set_stream: Some(hda_codec_dai_set_stream),
    calc_stream_format: Some(hda_calc_stream_format),
    get_hlink: Some(hda_get_hlink),
};

static ssp_ipc4_dma_ops: hda_dai_widget_dma_ops = hda_dai_widget_dma_ops {
    get_hext_stream: Some(hda_ipc4_get_hext_stream),
    assign_hext_stream: Some(hda_assign_hext_stream),
    release_hext_stream: Some(hda_release_hext_stream),
    setup_hext_stream: Some(hda_setup_hext_stream),
    reset_hext_stream: Some(hda_reset_hext_stream),
    pre_trigger: Some(hda_ipc4_pre_trigger),
    trigger: Some(hda_trigger),
    post_trigger: Some(hda_ipc4_post_trigger),
    calc_stream_format: Some(generic_calc_stream_format),
    get_hlink: Some(ssp_get_hlink),
    ..hda_dai_widget_dma_ops::default()
};

static dmic_ipc4_dma_ops: hda_dai_widget_dma_ops = hda_dai_widget_dma_ops {
    get_hext_stream: Some(hda_ipc4_get_hext_stream),
    assign_hext_stream: Some(hda_assign_hext_stream),
    release_hext_stream: Some(hda_release_hext_stream),
    setup_hext_stream: Some(hda_setup_hext_stream),
    reset_hext_stream: Some(hda_reset_hext_stream),
    pre_trigger: Some(hda_ipc4_pre_trigger),
    trigger: Some(hda_trigger),
    post_trigger: Some(hda_ipc4_post_trigger),
    calc_stream_format: Some(dmic_calc_stream_format),
    get_hlink: Some(dmic_get_hlink),
    ..hda_dai_widget_dma_ops::default()
};

static sdw_ipc4_dma_ops: hda_dai_widget_dma_ops = hda_dai_widget_dma_ops {
    get_hext_stream: Some(hda_ipc4_get_hext_stream),
    assign_hext_stream: Some(hda_assign_hext_stream),
    release_hext_stream: Some(hda_release_hext_stream),
    setup_hext_stream: Some(hda_setup_hext_stream),
    reset_hext_stream: Some(hda_reset_hext_stream),
    pre_trigger: Some(hda_ipc4_pre_trigger),
    trigger: Some(hda_trigger),
    post_trigger: Some(hda_ipc4_post_trigger),
    calc_stream_format: Some(generic_calc_stream_format),
    get_hlink: Some(sdw_get_hlink),
    ..hda_dai_widget_dma_ops::default()
};

static hda_ipc4_chain_dma_ops: hda_dai_widget_dma_ops = hda_dai_widget_dma_ops {
    get_hext_stream: Some(hda_get_hext_stream),
    assign_hext_stream: Some(hda_assign_hext_stream),
    release_hext_stream: Some(hda_release_hext_stream),
    setup_hext_stream: Some(hda_setup_hext_stream),
    reset_hext_stream: Some(hda_reset_hext_stream),
    trigger: Some(hda_trigger),
    codec_dai_set_stream: Some(hda_codec_dai_set_stream),
    calc_stream_format: Some(hda_calc_stream_format),
    get_hlink: Some(hda_get_hlink),
    ..hda_dai_widget_dma_ops::default()
};

static sdw_ipc4_chain_dma_ops: hda_dai_widget_dma_ops = hda_dai_widget_dma_ops {
    get_hext_stream: Some(hda_get_hext_stream),
    assign_hext_stream: Some(hda_assign_hext_stream),
    release_hext_stream: Some(hda_release_hext_stream),
    setup_hext_stream: Some(hda_setup_hext_stream),
    reset_hext_stream: Some(hda_reset_hext_stream),
    trigger: Some(hda_trigger),
    calc_stream_format: Some(generic_calc_stream_format),
    get_hlink: Some(sdw_get_hlink),
    ..hda_dai_widget_dma_ops::default()
};

unsafe fn hda_ipc3_post_trigger(
    _sdev: *mut snd_sof_dev,
    cpu_dai: *mut snd_soc_dai,
    substream: *mut snd_pcm_substream,
    cmd: c_int,
) -> c_int {
    let w: *mut snd_soc_dapm_widget = snd_soc_dai_get_widget(cpu_dai, (*substream).stream);

    match cmd {
        SNDRV_PCM_TRIGGER_SUSPEND | SNDRV_PCM_TRIGGER_STOP => {
            let mut data: snd_sof_dai_config_data = core::mem::zeroed();
            let ret: c_int;

            data.dai_data = DMA_CHAN_INVALID;
            ret = hda_dai_config(w, SOF_DAI_CONFIG_FLAGS_HW_FREE, &mut data);
            if ret < 0 {
                return ret;
            }
        }
        SNDRV_PCM_TRIGGER_PAUSE_PUSH => {
            return hda_dai_config(w, SOF_DAI_CONFIG_FLAGS_PAUSE, core::ptr::null_mut());
        }
        _ => {}
    }

    0
}

static hda_ipc3_dma_ops: hda_dai_widget_dma_ops = hda_dai_widget_dma_ops {
    get_hext_stream: Some(hda_get_hext_stream),
    assign_hext_stream: Some(hda_assign_hext_stream),
    release_hext_stream: Some(hda_release_hext_stream),
    setup_hext_stream: Some(hda_setup_hext_stream),
    reset_hext_stream: Some(hda_reset_hext_stream),
    trigger: Some(hda_trigger),
    post_trigger: Some(hda_ipc3_post_trigger),
    codec_dai_set_stream: Some(hda_codec_dai_set_stream),
    calc_stream_format: Some(hda_calc_stream_format),
    get_hlink: Some(hda_get_hlink),
    ..hda_dai_widget_dma_ops::default()
};

unsafe fn hda_dspless_get_hext_stream(
    _sdev: *mut snd_sof_dev,
    _cpu_dai: *mut snd_soc_dai,
    substream: *mut snd_pcm_substream,
) -> *mut hdac_ext_stream {
    let hstream: *mut hdac_stream = (*(*substream).runtime).private_data as *mut hdac_stream;

    stream_to_hdac_ext_stream(hstream)
}

unsafe fn hda_dspless_setup_hext_stream(
    _sdev: *mut snd_sof_dev,
    hext_stream: *mut hdac_ext_stream,
    format_val: c_uint,
) {
    /*
     * Save the format_val which was adjusted by the maxbps of the codec.
     * This information is not available on the FE side since there we are
     * using dummy_codec.
     */
    (*hext_stream).hstream.format_val = format_val;
}

static hda_dspless_dma_ops: hda_dai_widget_dma_ops = hda_dai_widget_dma_ops {
    get_hext_stream: Some(hda_dspless_get_hext_stream),
    setup_hext_stream: Some(hda_dspless_setup_hext_stream),
    codec_dai_set_stream: Some(hda_codec_dai_set_stream),
    calc_stream_format: Some(hda_calc_stream_format),
    get_hlink: Some(hda_get_hlink),
    ..hda_dai_widget_dma_ops::default()
};

static sdw_dspless_dma_ops: hda_dai_widget_dma_ops = hda_dai_widget_dma_ops {
    get_hext_stream: Some(hda_dspless_get_hext_stream),
    setup_hext_stream: Some(hda_dspless_setup_hext_stream),
    calc_stream_format: Some(generic_calc_stream_format),
    get_hlink: Some(sdw_get_hlink),
    ..hda_dai_widget_dma_ops::default()
};

// C conditional end: #endif

pub unsafe fn hda_select_dai_widget_ops(
    sdev: *mut snd_sof_dev,
    swidget: *mut snd_sof_widget,
) -> *const hda_dai_widget_dma_ops {
    // C conditional: #if IS_ENABLED(CONFIG_SND_SOC_SOF_HDA_LINK)
    let sdai: *mut snd_sof_dai;
    let chip: *const sof_intel_dsp_desc;

    chip = get_chip_info((*sdev).pdata);
    sdai = (*swidget).private as *mut snd_sof_dai;

    if (*sdev).dspless_mode_selected {
        match (*sdai).type_ {
            SOF_DAI_INTEL_HDA => {
                return &hda_dspless_dma_ops;
            }
            SOF_DAI_INTEL_ALH => {
                if (*chip).hw_ip_version < SOF_INTEL_ACE_2_0 {
                    return core::ptr::null();
                }
                return &sdw_dspless_dma_ops;
            }
            _ => {
                return core::ptr::null();
            }
        }
    }

    match (*(*sdev).pdata).ipc_type {
        SOF_IPC_TYPE_3 => {
            let private: *mut sof_dai_private_data = (*sdai).private as *mut sof_dai_private_data;

            if (*(*private).dai_config).type_ == SOF_DAI_INTEL_HDA {
                return &hda_ipc3_dma_ops;
            }
        }
        SOF_IPC_TYPE_4 => {
            let pipe_widget: *mut snd_sof_widget = (*(*swidget).spipe).pipe_widget;
            let pipeline: *mut sof_ipc4_pipeline = (*pipe_widget).private as *mut sof_ipc4_pipeline;

            match (*sdai).type_ {
                SOF_DAI_INTEL_HDA => {
                    if (*pipeline).use_chain_dma {
                        return &hda_ipc4_chain_dma_ops;
                    }

                    return &hda_ipc4_dma_ops;
                }
                SOF_DAI_INTEL_SSP => {
                    if (*chip).hw_ip_version < SOF_INTEL_ACE_2_0 {
                        return core::ptr::null();
                    }
                    return &ssp_ipc4_dma_ops;
                }
                SOF_DAI_INTEL_DMIC => {
                    if (*chip).hw_ip_version < SOF_INTEL_ACE_2_0 {
                        return core::ptr::null();
                    }
                    return &dmic_ipc4_dma_ops;
                }
                SOF_DAI_INTEL_ALH => {
                    if (*chip).hw_ip_version < SOF_INTEL_ACE_2_0 {
                        return core::ptr::null();
                    }
                    if (*pipeline).use_chain_dma {
                        return &sdw_ipc4_chain_dma_ops;
                    }
                    return &sdw_ipc4_dma_ops;
                }
                _ => {}
            }
        }
        _ => {}
    }
    // C conditional end: #endif
    core::ptr::null()
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
