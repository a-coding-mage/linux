// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright(c) 2021-2022 Intel Corporation
//
// Authors: Cezary Rojewski <cezary.rojewski@intel.com>
//          Amadeusz Slawinski <amadeuszx.slawinski@linux.intel.com>
//

use crate::*;

unsafe fn avs_dsp_init_probe(
    adev: *mut avs_dev,
    params: *mut snd_compr_params,
    bps: c_int,
    node_id: avs_connector_node_id,
    buffer_size: size_t,
) -> c_int {
    let mut cfg: avs_probe_cfg = unsafe { core::mem::zeroed() };
    let mut mentry: avs_module_entry = unsafe { core::mem::zeroed() };
    let mut dummy: u8 = 0;
    let mut ret: c_int;

    ret = unsafe { avs_get_module_entry(adev, &AVS_PROBE_MOD_UUID, &mut mentry) };
    if ret != 0 {
        return ret;
    }

    /*
     * Probe module uses no cycles, input and output frame sizes are unused.
     * It is also not owned by any pipeline.
     */
    cfg.base.ibs = 1;
    /* BSS module descriptor is always segment of index=2. */
    cfg.base.is_pages = mentry.segments[2].flags.length;
    cfg.base.audio_fmt.sampling_freq = unsafe { (*params).codec.sample_rate };
    cfg.base.audio_fmt.bit_depth = bps;
    cfg.base.audio_fmt.num_channels = unsafe { (*params).codec.ch_out };
    cfg.base.audio_fmt.valid_bit_depth = bps;
    cfg.gtw_cfg.node_id = node_id;
    cfg.gtw_cfg.dma_buffer_size = buffer_size;

    unsafe {
        avs_dsp_init_module(
            adev,
            mentry.module_id,
            INVALID_PIPELINE_ID,
            0,
            0,
            &mut cfg as *mut avs_probe_cfg as *mut c_void,
            core::mem::size_of_val(&cfg),
            &mut dummy,
        )
    }
}

unsafe fn avs_dsp_delete_probe(adev: *mut avs_dev) {
    let mut mentry: avs_module_entry = unsafe { core::mem::zeroed() };
    let ret: c_int;

    ret = unsafe { avs_get_module_entry(adev, &AVS_PROBE_MOD_UUID, &mut mentry) };
    if ret == 0 {
        /* There is only ever one probe module instance. */
        unsafe { avs_dsp_delete_module(adev, mentry.module_id, 0, INVALID_PIPELINE_ID, 0) };
    }
}

#[inline]
unsafe fn avs_compr_get_host_stream(cstream: *mut snd_compr_stream) -> *mut hdac_ext_stream {
    unsafe { (*(*cstream).runtime).private_data as *mut hdac_ext_stream }
}

unsafe extern "C" fn avs_probe_compr_open(
    cstream: *mut snd_compr_stream,
    dai: *mut snd_soc_dai,
) -> c_int {
    let adev: *mut avs_dev = unsafe { to_avs_dev((*dai).dev) };
    let bus: *mut hdac_bus = unsafe { &mut (*adev).base.core };
    let host_stream: *mut hdac_ext_stream;

    if unsafe { !(*adev).extractor.is_null() } {
        unsafe {
            dev_err(
                (*dai).dev,
                c"Cannot open more than one extractor stream\n".as_ptr(),
            )
        };
        return -EEXIST;
    }

    host_stream = unsafe { snd_hdac_ext_cstream_assign(bus, cstream) };
    if host_stream.is_null() {
        unsafe {
            dev_err(
                (*dai).dev,
                c"Failed to assign HDAudio stream for extraction\n".as_ptr(),
            )
        };
        return -EBUSY;
    }

    unsafe {
        (*adev).extractor = host_stream;
        (*hdac_stream(host_stream)).curr_pos = 0;
        (*(*cstream).runtime).private_data = host_stream as *mut c_void;
    }

    0
}

unsafe extern "C" fn avs_probe_compr_free(
    cstream: *mut snd_compr_stream,
    dai: *mut snd_soc_dai,
) -> c_int {
    let host_stream: *mut hdac_ext_stream = unsafe { avs_compr_get_host_stream(cstream) };
    let adev: *mut avs_dev = unsafe { to_avs_dev((*dai).dev) };
    let mut desc: *mut avs_probe_point_desc = core::ptr::null_mut();
    /* Extractor node identifier. */
    let vindex: c_uint = unsafe { INVALID_NODE_ID.vindex };
    let mut num_desc: size_t = 0;
    let mut i: c_int;
    let mut ret: c_int;

    'exit: loop {
        /* Disconnect all probe points. */
        ret = unsafe { avs_ipc_probe_get_points(adev, &mut desc, &mut num_desc) };
        if ret != 0 {
            unsafe { dev_err((*dai).dev, c"get probe points failed: %d\n".as_ptr(), ret) };
            ret = AVS_IPC_RET(ret);
            break 'exit;
        }

        i = 0;
        while (i as size_t) < num_desc {
            unsafe {
                if (*desc.add(i as usize)).node_id.vindex == vindex {
                    avs_ipc_probe_disconnect_points(adev, &mut (*desc.add(i as usize)).id, 1);
                }
            }
            i += 1;
        }
        unsafe { kfree(desc as *const c_void) };
        break 'exit;
    }

    if unsafe { (*adev).num_probe_streams } != 0 {
        unsafe {
            (*adev).num_probe_streams -= 1;
            if (*adev).num_probe_streams == 0 {
                avs_dsp_delete_probe(adev);
                avs_dsp_enable_d0ix(adev);
            }
        }
    }

    unsafe {
        snd_hdac_stream_cleanup(hdac_stream(host_stream));
        (*hdac_stream(host_stream)).prepared = 0;
        snd_hdac_ext_stream_release(host_stream, HDAC_EXT_STREAM_TYPE_HOST);

        snd_compr_free_pages(cstream);
        (*adev).extractor = core::ptr::null_mut();
    }

    ret
}

unsafe extern "C" fn avs_probe_compr_set_params(
    cstream: *mut snd_compr_stream,
    params: *mut snd_compr_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let host_stream: *mut hdac_ext_stream = unsafe { avs_compr_get_host_stream(cstream) };
    let rtd: *mut snd_compr_runtime = unsafe { (*cstream).runtime };
    let adev: *mut avs_dev = unsafe { to_avs_dev((*dai).dev) };
    let mut format_val: c_uint;
    let bps: c_int;
    let mut ret: c_int;

    unsafe {
        (*hdac_stream(host_stream)).bufsize = 0;
        (*hdac_stream(host_stream)).period_bytes = 0;
        (*hdac_stream(host_stream)).format_val = 0;
        (*cstream).dma_buffer.dev.type_ = SNDRV_DMA_TYPE_DEV_SG;
        (*cstream).dma_buffer.dev.dev = (*adev).dev;
    }

    ret = unsafe { snd_compr_malloc_pages(cstream, (*rtd).buffer_size) };
    if ret < 0 {
        return ret;
    }
    bps = unsafe { snd_pcm_format_physical_width((*params).codec.format) };
    if bps < 0 {
        return bps;
    }
    format_val = unsafe {
        snd_hdac_stream_format((*params).codec.ch_out, bps, (*params).codec.sample_rate)
    };
    ret = unsafe { snd_hdac_stream_set_params(hdac_stream(host_stream), format_val) };
    if ret < 0 {
        return ret;
    }
    ret = unsafe { snd_hdac_stream_setup(hdac_stream(host_stream), false) };
    if ret < 0 {
        return ret;
    }

    unsafe { (*hdac_stream(host_stream)).prepared = 1 };

    if unsafe { (*adev).num_probe_streams } == 0 {
        let mut node_id: avs_connector_node_id = unsafe { core::mem::zeroed() };

        /* D0ix not allowed during probing. */
        ret = unsafe { avs_dsp_disable_d0ix(adev) };
        if ret != 0 {
            return ret;
        }

        node_id.vindex = unsafe { (*hdac_stream(host_stream)).stream_tag - 1 };
        node_id.dma_type = AVS_DMA_HDA_HOST_INPUT;

        ret = unsafe { avs_dsp_init_probe(adev, params, bps, node_id, (*rtd).dma_bytes) };
        if ret < 0 {
            unsafe { dev_err((*dai).dev, c"probe init failed: %d\n".as_ptr(), ret) };
            unsafe { avs_dsp_enable_d0ix(adev) };
            return ret;
        }
    }

    unsafe { (*adev).num_probe_streams += 1 };
    0
}

unsafe extern "C" fn avs_probe_compr_trigger(
    cstream: *mut snd_compr_stream,
    cmd: c_int,
    dai: *mut snd_soc_dai,
) -> c_int {
    let host_stream: *mut hdac_ext_stream = unsafe { avs_compr_get_host_stream(cstream) };
    let adev: *mut avs_dev = unsafe { to_avs_dev((*dai).dev) };
    let bus: *mut hdac_bus = unsafe { &mut (*adev).base.core };
    let mut cookie: c_ulong = 0;

    if unsafe { (*hdac_stream(host_stream)).prepared } == 0 {
        return -EPIPE;
    }

    match cmd {
        SNDRV_PCM_TRIGGER_START | SNDRV_PCM_TRIGGER_PAUSE_RELEASE | SNDRV_PCM_TRIGGER_RESUME => {
            unsafe {
                spin_lock_irqsave(&mut (*bus).reg_lock, &mut cookie);
                snd_hdac_stream_start(hdac_stream(host_stream));
                spin_unlock_irqrestore(&mut (*bus).reg_lock, cookie);
            }
        }

        SNDRV_PCM_TRIGGER_PAUSE_PUSH | SNDRV_PCM_TRIGGER_SUSPEND | SNDRV_PCM_TRIGGER_STOP => {
            unsafe {
                spin_lock_irqsave(&mut (*bus).reg_lock, &mut cookie);
                snd_hdac_stream_stop(hdac_stream(host_stream));
                spin_unlock_irqrestore(&mut (*bus).reg_lock, cookie);
            }
        }

        _ => return -EINVAL,
    }

    0
}

unsafe extern "C" fn avs_probe_compr_pointer(
    cstream: *mut snd_compr_stream,
    tstamp: *mut snd_compr_tstamp64,
    dai: *mut snd_soc_dai,
) -> c_int {
    let host_stream: *mut hdac_ext_stream = unsafe { avs_compr_get_host_stream(cstream) };
    let pstream: *mut snd_soc_pcm_stream;

    pstream = unsafe { &mut (*(*dai).driver).capture };
    unsafe {
        (*tstamp).copied_total = (*hdac_stream(host_stream)).curr_pos;
        (*tstamp).sampling_rate = snd_pcm_rate_bit_to_rate((*pstream).rates);
    }

    0
}

unsafe extern "C" fn avs_probe_compr_copy(
    _comp: *mut snd_soc_component,
    cstream: *mut snd_compr_stream,
    buf: *mut c_char,
    mut count: size_t,
) -> c_int {
    let rtd: *mut snd_compr_runtime = unsafe { (*cstream).runtime };
    let mut offset: c_uint = 0;
    let n: c_uint;
    let ptr: *mut c_void;
    let mut ret: c_int;

    if count > unsafe { (*rtd).buffer_size } {
        count = unsafe { (*rtd).buffer_size };
    }

    unsafe { div_u64_rem((*rtd).total_bytes_transferred, (*rtd).buffer_size, &mut offset) };
    ptr = unsafe { (*rtd).dma_area.add(offset as usize) as *mut c_void };
    n = unsafe { (*rtd).buffer_size as c_uint - offset };

    if count < n as size_t {
        ret = unsafe { copy_to_user(buf as *mut c_void, ptr, count) };
    } else {
        ret = unsafe { copy_to_user(buf as *mut c_void, ptr, n as size_t) };
        ret += unsafe {
            copy_to_user(
                buf.add(n as usize) as *mut c_void,
                (*rtd).dma_area as *mut c_void,
                count - n as size_t,
            )
        };
    }

    if ret != 0 {
        return (count as c_int) - ret;
    }
    count as c_int
}

static avs_probe_cdai_ops: snd_soc_cdai_ops = snd_soc_cdai_ops {
    startup: Some(avs_probe_compr_open),
    shutdown: Some(avs_probe_compr_free),
    set_params: Some(avs_probe_compr_set_params),
    trigger: Some(avs_probe_compr_trigger),
    pointer: Some(avs_probe_compr_pointer),
};

static avs_probe_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    compress_new: Some(snd_soc_new_compress),
};

static avs_probe_compress_ops: snd_compress_ops = snd_compress_ops {
    copy: Some(avs_probe_compr_copy),
};

static mut probe_cpu_dais: [snd_soc_dai_driver; 1] = [snd_soc_dai_driver {
    name: c"Probe Extraction CPU DAI".as_ptr(),
    cops: &avs_probe_cdai_ops,
    ops: &avs_probe_dai_ops,
    capture: snd_soc_pcm_stream {
        stream_name: c"Probe Extraction".as_ptr(),
        channels_min: 1,
        channels_max: 8,
        rates: SNDRV_PCM_RATE_48000,
        rate_min: 48000,
        rate_max: 48000,
    },
}];

static avs_probe_component_driver: snd_soc_component_driver = snd_soc_component_driver {
    name: c"avs-probe-compr".as_ptr(),
    compress_ops: &avs_probe_compress_ops,
    module_get_upon_open: 1, /* increment refcount when a stream is opened */
};

#[no_mangle]
pub unsafe extern "C" fn avs_register_probe_component(
    adev: *mut avs_dev,
    name: *const c_char,
) -> c_int {
    let component: *mut snd_soc_component;
    let comp_name: *const c_char;

    component = unsafe { snd_soc_component_alloc((*adev).dev) };
    if component.is_null() {
        return -ENOMEM;
    }

    comp_name = unsafe { devm_kstrdup((*adev).dev, name, GFP_KERNEL) };
    if comp_name.is_null() {
        return -ENOMEM;
    }

    unsafe { snd_soc_component_set_name(component, comp_name) };

    unsafe {
        snd_soc_register_component(
            component,
            &avs_probe_component_driver,
            probe_cpu_dais.as_mut_ptr(),
            ARRAY_SIZE(&probe_cpu_dais),
        )
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
