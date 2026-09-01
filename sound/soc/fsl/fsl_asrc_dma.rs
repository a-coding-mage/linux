// SPDX-License-Identifier: GPL-2.0
//
// Freescale ASRC ALSA SoC Platform (DMA) driver
//
// Copyright (C) 2014 Freescale Semiconductor, Inc.
//
// Author: Nicolin Chen <nicoleotsuka@gmail.com>

// C dependencies:
// linux/dma-mapping.h, linux/module.h, linux/dma/imx-dma.h,
// sound/dmaengine_pcm.h, sound/pcm_params.h, "fsl_asrc_common.h"

const FSL_ASRC_DMABUF_SIZE: usize = 256 * 1024;

static mut snd_imx_hardware: snd_pcm_hardware = snd_pcm_hardware {
    info: SNDRV_PCM_INFO_INTERLEAVED
        | SNDRV_PCM_INFO_BLOCK_TRANSFER
        | SNDRV_PCM_INFO_MMAP
        | SNDRV_PCM_INFO_MMAP_VALID,
    buffer_bytes_max: FSL_ASRC_DMABUF_SIZE,
    period_bytes_min: 128,
    period_bytes_max: 65535, /* Limited by SDMA engine */
    periods_min: 2,
    periods_max: 255,
    fifo_size: 0,
};

#[inline]
fn logical_not_dir(dir: u8) -> u8 {
    if dir == 0 {
        1
    } else {
        0
    }
}

unsafe extern "C" fn filter(chan: *mut dma_chan, param: *mut c_void) -> bool {
    if !imx_dma_is_general_purpose(chan) {
        return false;
    }

    (*chan).private = param;

    true
}

unsafe extern "C" fn fsl_asrc_dma_complete(arg: *mut c_void) {
    let substream = arg as *mut snd_pcm_substream;
    let runtime = (*substream).runtime;
    let pair = (*runtime).private_data as *mut fsl_asrc_pair;

    (*pair).pos += snd_pcm_lib_period_bytes(substream);
    if (*pair).pos >= snd_pcm_lib_buffer_bytes(substream) {
        (*pair).pos = 0;
    }

    snd_pcm_period_elapsed(substream);
}

unsafe extern "C" fn fsl_asrc_dma_prepare_and_submit(
    substream: *mut snd_pcm_substream,
    component: *mut snd_soc_component,
) -> c_int {
    let dir: u8 = if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        OUT
    } else {
        IN
    };
    let fe_dir = logical_not_dir(dir);
    let runtime = (*substream).runtime;
    let pair = (*runtime).private_data as *mut fsl_asrc_pair;
    let dev = (*component).dev;
    let mut flags: c_ulong = DMA_CTRL_ACK;

    /* Prepare and submit Front-End DMA channel */
    if !(*(*substream).runtime).no_period_wakeup {
        flags |= DMA_PREP_INTERRUPT;
    }

    (*pair).pos = 0;
    (*pair).desc[fe_dir as usize] = dmaengine_prep_dma_cyclic(
        (*pair).dma_chan[fe_dir as usize],
        (*runtime).dma_addr,
        snd_pcm_lib_buffer_bytes(substream),
        snd_pcm_lib_period_bytes(substream),
        if dir == OUT {
            DMA_MEM_TO_DEV
        } else {
            DMA_DEV_TO_MEM
        },
        flags,
    );
    if (*pair).desc[fe_dir as usize].is_null() {
        dev_err(dev, c"failed to prepare slave DMA for Front-End\n".as_ptr());
        return -ENOMEM;
    }

    (*(*pair).desc[fe_dir as usize]).callback = Some(fsl_asrc_dma_complete);
    (*(*pair).desc[fe_dir as usize]).callback_param = substream as *mut c_void;

    dmaengine_submit((*pair).desc[fe_dir as usize]);

    /* Prepare and submit Back-End DMA channel */
    (*pair).desc[dir as usize] = dmaengine_prep_dma_cyclic(
        (*pair).dma_chan[dir as usize],
        0xffff,
        64,
        64,
        DMA_DEV_TO_DEV,
        0,
    );
    if (*pair).desc[dir as usize].is_null() {
        dev_err(dev, c"failed to prepare slave DMA for Back-End\n".as_ptr());
        return -ENOMEM;
    }

    dmaengine_submit((*pair).desc[dir as usize]);

    0
}

unsafe extern "C" fn fsl_asrc_dma_trigger(
    component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
    cmd: c_int,
) -> c_int {
    let runtime = (*substream).runtime;
    let pair = (*runtime).private_data as *mut fsl_asrc_pair;
    let mut ret: c_int;

    match cmd {
        SNDRV_PCM_TRIGGER_START | SNDRV_PCM_TRIGGER_RESUME | SNDRV_PCM_TRIGGER_PAUSE_RELEASE => {
            ret = fsl_asrc_dma_prepare_and_submit(substream, component);
            if ret != 0 {
                return ret;
            }
            dma_async_issue_pending((*pair).dma_chan[IN as usize]);
            dma_async_issue_pending((*pair).dma_chan[OUT as usize]);
        }
        SNDRV_PCM_TRIGGER_STOP | SNDRV_PCM_TRIGGER_SUSPEND | SNDRV_PCM_TRIGGER_PAUSE_PUSH => {
            dmaengine_terminate_async((*pair).dma_chan[OUT as usize]);
            dmaengine_terminate_async((*pair).dma_chan[IN as usize]);
        }
        _ => return -EINVAL,
    }

    0
}

unsafe extern "C" fn fsl_asrc_dma_hw_params(
    component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    let mut buswidth: dma_slave_buswidth = DMA_SLAVE_BUSWIDTH_2_BYTES;
    let mut be_peripheral_type: sdma_peripheral_type = IMX_DMATYPE_SSI;
    let rtd = snd_soc_substream_to_rtd(substream);
    let tx = (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK;
    let mut dma_params_fe: *mut snd_dmaengine_dai_dma_data = core::ptr::null_mut();
    let mut dma_params_be: *mut snd_dmaengine_dai_dma_data = core::ptr::null_mut();
    let runtime = (*substream).runtime;
    let pair = (*runtime).private_data as *mut fsl_asrc_pair;
    let mut tmp_chan: *mut dma_chan = core::ptr::null_mut();
    let mut be_chan: *mut dma_chan = core::ptr::null_mut();
    let mut component_be: *mut snd_soc_component = core::ptr::null_mut();
    let asrc = (*pair).asrc;
    let mut config_fe: dma_slave_config = core::mem::zeroed();
    let mut config_be: dma_slave_config = core::mem::zeroed();
    let mut audio_config: sdma_peripheral_config = core::mem::zeroed();
    let index: asrc_pair_index = (*pair).index;
    let dev = (*component).dev;
    let mut of_dma_node: *mut device_node;
    let stream = (*substream).stream;
    let mut tmp_data: *mut imx_dma_data;
    let mut dpcm: *mut snd_soc_dpcm;
    let mut dev_be: *mut device = core::ptr::null_mut();
    let dir: u8 = if tx { OUT } else { IN };
    let fe_dir = logical_not_dir(dir);
    let mut mask: dma_cap_mask_t = core::mem::zeroed();
    let mut ret: c_int;
    let mut width: c_int;

    /* Fetch the Back-End dma_data from DPCM */
    for_each_dpcm_be!(rtd, stream, dpcm, {
        let be = (*dpcm).be;
        let substream_be: *mut snd_pcm_substream;
        let dai_cpu = snd_soc_rtd_to_cpu(be, 0);
        let dai_codec = snd_soc_rtd_to_codec(be, 0);
        let dai: *mut snd_soc_dai;

        if (*dpcm).fe != rtd {
            continue;
        }

        /*
         * With audio graph card, original cpu dai is changed to codec
         * device in backend, so if cpu dai is dummy device in backend,
         * get the codec dai device, which is the real hardware device
         * connected.
         */
        if !snd_soc_dai_is_dummy(dai_cpu) {
            dai = dai_cpu;
        } else {
            dai = dai_codec;
        }

        substream_be = snd_soc_dpcm_get_substream(be, stream);
        dma_params_be = snd_soc_dai_get_dma_data(dai, substream_be);
        dev_be = (*dai).dev;
        break;
    });

    if dma_params_be.is_null() {
        dev_err(dev, c"failed to get the substream of Back-End\n".as_ptr());
        return -EINVAL;
    }

    /* Override dma_data of the Front-End and config its dmaengine */
    dma_params_fe = snd_soc_dai_get_dma_data(snd_soc_rtd_to_cpu(rtd, 0), substream);
    (*dma_params_fe).addr = (*asrc).paddr + ((*asrc).get_fifo_addr)(fe_dir, index);
    (*dma_params_fe).maxburst = (*dma_params_be).maxburst;

    (*pair).dma_chan[fe_dir as usize] = ((*asrc).get_dma_channel)(pair, fe_dir);
    if (*pair).dma_chan[fe_dir as usize].is_null() {
        dev_err(dev, c"failed to request DMA channel\n".as_ptr());
        return -EINVAL;
    }

    ret = snd_dmaengine_pcm_prepare_slave_config(substream, params, &mut config_fe);
    if ret != 0 {
        dev_err(dev, c"failed to prepare DMA config for Front-End\n".as_ptr());
        return ret;
    }

    ret = dmaengine_slave_config((*pair).dma_chan[fe_dir as usize], &mut config_fe);
    if ret != 0 {
        dev_err(dev, c"failed to config DMA channel for Front-End\n".as_ptr());
        return ret;
    }

    /* Request and config DMA channel for Back-End */
    dma_cap_zero(&mut mask);
    dma_cap_set(DMA_SLAVE, &mut mask);
    dma_cap_set(DMA_CYCLIC, &mut mask);

    /*
     * The Back-End device might have already requested a DMA channel,
     * so try to reuse it first, and then request a new one upon NULL.
     */
    component_be = snd_soc_lookup_component_nolocked(dev_be, SND_DMAENGINE_PCM_DRV_NAME);
    if !component_be.is_null() {
        let pcm = snd_soc_component_to_priv(component_be) as *mut dmaengine_pcm;

        be_chan = (*pcm).chan[(*substream).stream as usize];
        tmp_chan = be_chan;
    }
    if tmp_chan.is_null() {
        tmp_chan = dma_request_chan(dev_be, if tx { c"tx".as_ptr() } else { c"rx".as_ptr() });
        if IS_ERR(tmp_chan) {
            dev_err(dev, c"failed to request DMA channel for Back-End\n".as_ptr());
            return -EINVAL;
        }
    }

    /*
     * An EDMA DEV_TO_DEV channel is fixed and bound with DMA event of each
     * peripheral, unlike SDMA channel that is allocated dynamically. So no
     * need to configure dma_request and dma_request2, but get dma_chan of
     * Back-End device directly via dma_request_chan.
     */
    if !(*asrc).use_edma {
        /* Get DMA request of Back-End */
        tmp_data = (*tmp_chan).private as *mut imx_dma_data;
        (*pair).dma_data.dma_request = (*tmp_data).dma_request;
        be_peripheral_type = (*tmp_data).peripheral_type;
        if be_chan.is_null() {
            dma_release_channel(tmp_chan);
        }

        /* Get DMA request of Front-End */
        tmp_chan = ((*asrc).get_dma_channel)(pair, dir);
        tmp_data = (*tmp_chan).private as *mut imx_dma_data;
        (*pair).dma_data.dma_request2 = (*tmp_data).dma_request;
        (*pair).dma_data.peripheral_type = (*tmp_data).peripheral_type;
        (*pair).dma_data.priority = (*tmp_data).priority;
        dma_release_channel(tmp_chan);

        of_dma_node = (*(*(*pair).dma_chan[fe_dir as usize]).device).dev).of_node;
        (*pair).dma_chan[dir as usize] =
            __dma_request_channel(&mut mask, Some(filter), &mut (*pair).dma_data, of_dma_node);
        (*pair).req_dma_chan = true;
    } else {
        (*pair).dma_chan[dir as usize] = tmp_chan;
        /* Do not flag to release if we are reusing the Back-End one */
        (*pair).req_dma_chan = be_chan.is_null();
    }

    if (*pair).dma_chan[dir as usize].is_null() {
        dev_err(dev, c"failed to request DMA channel for Back-End\n".as_ptr());
        return -EINVAL;
    }

    width = snd_pcm_format_physical_width((*asrc).asrc_format);
    if width < 8 || width > 64 {
        return -EINVAL;
    } else if width == 8 {
        buswidth = DMA_SLAVE_BUSWIDTH_1_BYTE;
    } else if width == 16 {
        buswidth = DMA_SLAVE_BUSWIDTH_2_BYTES;
    } else if width == 24 {
        buswidth = DMA_SLAVE_BUSWIDTH_3_BYTES;
    } else if width <= 32 {
        buswidth = DMA_SLAVE_BUSWIDTH_4_BYTES;
    } else {
        buswidth = DMA_SLAVE_BUSWIDTH_8_BYTES;
    }

    config_be.direction = DMA_DEV_TO_DEV;
    config_be.src_addr_width = buswidth;
    config_be.src_maxburst = (*dma_params_be).maxburst;
    config_be.dst_addr_width = buswidth;
    config_be.dst_maxburst = (*dma_params_be).maxburst;

    /*
     * For eDMA, the back-end may report a maxburst size that is not evenly
     * divisible by the channel count. This causes the DMA transfer length
     * to misalign with the FIFO boundary, resulting in wrong data and
     * audible noise. Align maxburst to the nearest valid boundary:
     * - If maxburst >= channel count, override to the channel count so
     *   each transfer equals exactly one audio frame.
     * - If maxburst < channel count, override to 1 to avoid partial-frame
     *   transfers.
     */
    if (*asrc).use_edma && ((*dma_params_be).maxburst % params_channels(params) != 0) {
        if (*dma_params_be).maxburst >= params_channels(params) {
            config_be.src_maxburst = params_channels(params);
            config_be.dst_maxburst = params_channels(params);
        } else {
            config_be.src_maxburst = 1;
            config_be.dst_maxburst = 1;
        }
    }

    core::ptr::write_bytes(
        &mut audio_config as *mut sdma_peripheral_config as *mut u8,
        0,
        core::mem::size_of::<sdma_peripheral_config>(),
    );
    config_be.peripheral_config = &mut audio_config as *mut sdma_peripheral_config as *mut c_void;
    config_be.peripheral_size = core::mem::size_of::<sdma_peripheral_config>();

    if tx && (be_peripheral_type == IMX_DMATYPE_SSI_DUAL || be_peripheral_type == IMX_DMATYPE_SPDIF)
    {
        audio_config.n_fifos_dst = 2;
    }
    if !tx
        && (be_peripheral_type == IMX_DMATYPE_SSI_DUAL
            || be_peripheral_type == IMX_DMATYPE_SPDIF)
    {
        audio_config.n_fifos_src = 2;
    }

    if tx {
        config_be.src_addr = (*asrc).paddr + ((*asrc).get_fifo_addr)(OUT, index);
        config_be.dst_addr = (*dma_params_be).addr;
    } else {
        config_be.dst_addr = (*asrc).paddr + ((*asrc).get_fifo_addr)(IN, index);
        config_be.src_addr = (*dma_params_be).addr;
    }

    ret = dmaengine_slave_config((*pair).dma_chan[dir as usize], &mut config_be);
    if ret != 0 {
        dev_err(dev, c"failed to config DMA channel for Back-End\n".as_ptr());
        if (*pair).req_dma_chan {
            dma_release_channel((*pair).dma_chan[dir as usize]);
        }
        return ret;
    }

    0
}

unsafe extern "C" fn fsl_asrc_dma_hw_free(
    component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> c_int {
    let tx = (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK;
    let runtime = (*substream).runtime;
    let pair = (*runtime).private_data as *mut fsl_asrc_pair;
    let dir: u8 = if tx { OUT } else { IN };
    let fe_dir = logical_not_dir(dir);

    if !(*pair).dma_chan[fe_dir as usize].is_null() {
        dma_release_channel((*pair).dma_chan[fe_dir as usize]);
    }

    /* release dev_to_dev chan if we aren't reusing the Back-End one */
    if !(*pair).dma_chan[dir as usize].is_null() && (*pair).req_dma_chan {
        dma_release_channel((*pair).dma_chan[dir as usize]);
    }

    (*pair).dma_chan[fe_dir as usize] = core::ptr::null_mut();
    (*pair).dma_chan[dir as usize] = core::ptr::null_mut();

    0
}

unsafe extern "C" fn fsl_asrc_dma_startup(
    component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> c_int {
    let tx = (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK;
    let rtd = snd_soc_substream_to_rtd(substream);
    let runtime = (*substream).runtime;
    let mut dma_data: *mut snd_dmaengine_dai_dma_data;
    let dev = (*component).dev;
    let asrc = dev_get_drvdata(dev) as *mut fsl_asrc;
    let mut pair: *mut fsl_asrc_pair;
    let mut tmp_chan: *mut dma_chan = core::ptr::null_mut();
    let dir: u8 = if tx { OUT } else { IN };
    let mut release_pair = true;
    let mut ret: c_int = 0;

    ret = snd_pcm_hw_constraint_integer((*substream).runtime, SNDRV_PCM_HW_PARAM_PERIODS);
    if ret < 0 {
        dev_err(dev, c"failed to set pcm hw params periods\n".as_ptr());
        return ret;
    }

    pair = kzalloc(
        core::mem::size_of::<fsl_asrc_pair>() + (*asrc).pair_priv_size,
        GFP_KERNEL,
    ) as *mut fsl_asrc_pair;
    if pair.is_null() {
        return -ENOMEM;
    }

    (*pair).asrc = asrc;
    (*pair).private =
        (pair as *mut c_void).byte_add(core::mem::size_of::<fsl_asrc_pair>());

    (*runtime).private_data = pair as *mut c_void;

    /* Request a dummy pair, which will be released later.
     * Request pair function needs channel num as input, for this
     * dummy pair, we just request "1" channel temporarily.
     */
    ret = ((*asrc).request_pair)(1, pair);
    if ret < 0 {
        dev_err(dev, c"failed to request asrc pair\n".as_ptr());
    } else {
        /* Request a dummy dma channel, which will be released later. */
        tmp_chan = ((*asrc).get_dma_channel)(pair, dir);
        if tmp_chan.is_null() {
            dev_err(dev, c"failed to get dma channel\n".as_ptr());
            ret = -EINVAL;
        } else {
            dma_data = snd_soc_dai_get_dma_data(snd_soc_rtd_to_cpu(rtd, 0), substream);

            /* Refine the snd_imx_hardware according to caps of DMA. */
            ret = snd_dmaengine_pcm_refine_runtime_hwparams(
                substream,
                dma_data,
                &mut snd_imx_hardware,
                tmp_chan,
            );
            if ret < 0 {
                dev_err(dev, c"failed to refine runtime hwparams\n".as_ptr());
            } else {
                release_pair = false;
                snd_soc_set_runtime_hwparams(substream, &mut snd_imx_hardware);
            }

            dma_release_channel(tmp_chan);
        }

        ((*asrc).release_pair)(pair);
    }

    if release_pair {
        kfree(pair as *mut c_void);
    }

    ret
}

unsafe extern "C" fn fsl_asrc_dma_shutdown(
    component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> c_int {
    let runtime = (*substream).runtime;
    let pair = (*runtime).private_data as *mut fsl_asrc_pair;
    let asrc: *mut fsl_asrc;

    if pair.is_null() {
        return 0;
    }

    asrc = (*pair).asrc;

    if (*asrc).pair[(*pair).index as usize] == pair {
        (*asrc).pair[(*pair).index as usize] = core::ptr::null_mut();
    }

    kfree(pair as *mut c_void);

    0
}

unsafe extern "C" fn fsl_asrc_dma_pcm_pointer(
    component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> snd_pcm_uframes_t {
    let runtime = (*substream).runtime;
    let pair = (*runtime).private_data as *mut fsl_asrc_pair;

    bytes_to_frames((*substream).runtime, (*pair).pos)
}

unsafe extern "C" fn fsl_asrc_dma_pcm_new(
    component: *mut snd_soc_component,
    rtd: *mut snd_soc_pcm_runtime,
) -> c_int {
    let dev = (*component).dev;
    let asrc = dev_get_drvdata(dev) as *mut fsl_asrc;
    let mut pair: *mut fsl_asrc_pair;
    let pcm = (*rtd).pcm;
    let mut chan: *mut dma_chan;
    let mut ret: c_int;

    pair = kzalloc(
        size_add(
            core::mem::size_of::<fsl_asrc_pair>(),
            (*asrc).pair_priv_size,
        ),
        GFP_KERNEL,
    ) as *mut fsl_asrc_pair;
    if pair.is_null() {
        return -ENOMEM;
    }

    (*pair).asrc = asrc;
    (*pair).private =
        (pair as *mut c_void).byte_add(core::mem::size_of::<fsl_asrc_pair>());

    /* Request a pair, which will be released later.
     * Request pair function needs channel num as input, for this
     * pair, we just request "1" channel temporarily.
     */
    ret = ((*asrc).request_pair)(1, pair);
    if ret < 0 {
        kfree(pair as *mut c_void);
        return ret;
    }

    /* Request a dma channel, which will be released later. */
    chan = ((*asrc).get_dma_channel)(pair, IN);
    if chan.is_null() {
        dev_err(dev, c"failed to get dma channel\n".as_ptr());
        ret = -EINVAL;
    } else {
        ret = snd_pcm_set_fixed_buffer_all(
            pcm,
            SNDRV_DMA_TYPE_DEV,
            (*(*chan).device).dev,
            FSL_ASRC_DMABUF_SIZE,
        );

        dma_release_channel(chan);
    }

    ((*asrc).release_pair)(pair);

    kfree(pair as *mut c_void);

    ret
}

#[no_mangle]
pub static mut fsl_asrc_component: snd_soc_component_driver = snd_soc_component_driver {
    name: DRV_NAME,
    hw_params: Some(fsl_asrc_dma_hw_params),
    hw_free: Some(fsl_asrc_dma_hw_free),
    trigger: Some(fsl_asrc_dma_trigger),
    open: Some(fsl_asrc_dma_startup),
    close: Some(fsl_asrc_dma_shutdown),
    pointer: Some(fsl_asrc_dma_pcm_pointer),
    pcm_new: Some(fsl_asrc_dma_pcm_new),
    legacy_dai_naming: 1,
    // CONFIG_DEBUG_FS: .debugfs_prefix = "asrc"
};

EXPORT_SYMBOL_GPL!(fsl_asrc_component);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
