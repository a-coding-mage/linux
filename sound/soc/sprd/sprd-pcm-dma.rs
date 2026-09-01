// SPDX-License-Identifier: GPL-2.0
// Copyright (C) 2019 Spreadtrum Communications Inc.

// Rust translation of soc/sprd/sprd-pcm-dma.c.
// Kernel headers and "sprd-pcm-dma.h" provide the external types, constants,
// functions, and macros referenced below.

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

const SPRD_PCM_DMA_LINKLIST_SIZE: c_int = 64;
const SPRD_PCM_DMA_BRUST_LEN: c_int = 640;

#[repr(C)]
pub struct sprd_pcm_dma_data {
    chan: *mut dma_chan,
    desc: *mut dma_async_tx_descriptor,
    cookie: dma_cookie_t,
    phys: dma_addr_t,
    virt: *mut c_void,
    pre_pointer: c_int,
}

#[repr(C)]
pub struct sprd_pcm_dma_private {
    substream: *mut snd_pcm_substream,
    params: *mut sprd_pcm_dma_params,
    data: [sprd_pcm_dma_data; SPRD_PCM_CHANNEL_MAX as usize],
    hw_chan: c_int,
    dma_addr_offset: c_int,
}

static sprd_pcm_hardware: snd_pcm_hardware = snd_pcm_hardware {
    info: SNDRV_PCM_INFO_MMAP
        | SNDRV_PCM_INFO_MMAP_VALID
        | SNDRV_PCM_INFO_INTERLEAVED
        | SNDRV_PCM_INFO_PAUSE
        | SNDRV_PCM_INFO_RESUME
        | SNDRV_PCM_INFO_NO_PERIOD_WAKEUP,
    formats: SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE,
    period_bytes_min: 1,
    period_bytes_max: 64 * 1024,
    periods_min: 1,
    periods_max: PAGE_SIZE / SPRD_PCM_DMA_LINKLIST_SIZE,
    buffer_bytes_max: 64 * 1024,
};

unsafe extern "C" fn sprd_pcm_open(
    component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> c_int {
    let runtime = (*substream).runtime;
    let dev = (*component).dev;
    let mut dma_private: *mut sprd_pcm_dma_private;
    let hw_chan: c_int = SPRD_PCM_CHANNEL_MAX;
    let mut size: c_int;
    let mut ret: c_int;
    let mut i: c_int;

    snd_soc_set_runtime_hwparams(substream, &sprd_pcm_hardware);

    ret = snd_pcm_hw_constraint_step(
        runtime,
        0,
        SNDRV_PCM_HW_PARAM_PERIOD_BYTES,
        SPRD_PCM_DMA_BRUST_LEN,
    );
    if ret < 0 {
        return ret;
    }

    ret = snd_pcm_hw_constraint_step(
        runtime,
        0,
        SNDRV_PCM_HW_PARAM_BUFFER_BYTES,
        SPRD_PCM_DMA_BRUST_LEN,
    );
    if ret < 0 {
        return ret;
    }

    ret = snd_pcm_hw_constraint_integer(runtime, SNDRV_PCM_HW_PARAM_PERIODS);
    if ret < 0 {
        return ret;
    }

    dma_private = devm_kzalloc(dev, size_of::<sprd_pcm_dma_private>(), GFP_KERNEL)
        as *mut sprd_pcm_dma_private;
    if dma_private.is_null() {
        return -ENOMEM;
    }

    size = (*runtime).hw.periods_max * SPRD_PCM_DMA_LINKLIST_SIZE;

    i = 0;
    while i < hw_chan {
        let data = &mut (*dma_private).data[i as usize] as *mut sprd_pcm_dma_data;

        (*data).virt = dmam_alloc_coherent(dev, size as usize, &mut (*data).phys, GFP_KERNEL);
        if (*data).virt.is_null() {
            ret = -ENOMEM;
            goto_error(dev, dma_private, size, hw_chan);
            return ret;
        }

        i += 1;
    }

    (*dma_private).hw_chan = hw_chan;
    (*runtime).private_data = dma_private as *mut c_void;
    (*dma_private).substream = substream;

    0
}

unsafe fn goto_error(
    dev: *mut device,
    dma_private: *mut sprd_pcm_dma_private,
    size: c_int,
    hw_chan: c_int,
) {
    let mut i: c_int = 0;

    while i < hw_chan {
        let data = &mut (*dma_private).data[i as usize] as *mut sprd_pcm_dma_data;

        if !(*data).virt.is_null() {
            dmam_free_coherent(dev, size as usize, (*data).virt, (*data).phys);
        }

        i += 1;
    }

    devm_kfree(dev, dma_private as *mut c_void);
}

unsafe extern "C" fn sprd_pcm_close(
    component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> c_int {
    let runtime = (*substream).runtime;
    let dma_private = (*runtime).private_data as *mut sprd_pcm_dma_private;
    let dev = (*component).dev;
    let size: c_int = (*runtime).hw.periods_max * SPRD_PCM_DMA_LINKLIST_SIZE;
    let mut i: c_int = 0;

    while i < (*dma_private).hw_chan {
        let data = &mut (*dma_private).data[i as usize] as *mut sprd_pcm_dma_data;

        dmam_free_coherent(dev, size as usize, (*data).virt, (*data).phys);

        i += 1;
    }

    devm_kfree(dev, dma_private as *mut c_void);

    0
}

unsafe extern "C" fn sprd_pcm_dma_complete(data: *mut c_void) {
    let dma_private = data as *mut sprd_pcm_dma_private;
    let substream = (*dma_private).substream;

    snd_pcm_period_elapsed(substream);
}

unsafe fn sprd_pcm_release_dma_channel(substream: *mut snd_pcm_substream) {
    let runtime = (*substream).runtime;
    let dma_private = (*runtime).private_data as *mut sprd_pcm_dma_private;
    let mut i: c_int = 0;

    while i < SPRD_PCM_CHANNEL_MAX {
        let data = &mut (*dma_private).data[i as usize] as *mut sprd_pcm_dma_data;

        if !(*data).chan.is_null() {
            dma_release_channel((*data).chan);
            (*data).chan = ptr::null_mut();
        }

        i += 1;
    }
}

unsafe fn sprd_pcm_request_dma_channel(
    component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
    channels: c_int,
) -> c_int {
    let runtime = (*substream).runtime;
    let dma_private = (*runtime).private_data as *mut sprd_pcm_dma_private;
    let dev = (*component).dev;
    let dma_params = (*dma_private).params;
    let mut i: c_int = 0;

    if channels > SPRD_PCM_CHANNEL_MAX {
        dev_err(
            dev,
            b"invalid dma channel number:%d\n\0".as_ptr() as *const c_char,
            channels,
        );
        return -EINVAL;
    }

    while i < channels {
        let data = &mut (*dma_private).data[i as usize] as *mut sprd_pcm_dma_data;

        (*data).chan = dma_request_slave_channel(dev, (*dma_params).chan_name[i as usize]);
        if (*data).chan.is_null() {
            dev_err(
                dev,
                b"failed to request dma channel:%s\n\0".as_ptr() as *const c_char,
                (*dma_params).chan_name[i as usize],
            );
            sprd_pcm_release_dma_channel(substream);
            return -ENODEV;
        }

        i += 1;
    }

    0
}

unsafe extern "C" fn sprd_pcm_hw_params(
    component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    let runtime = (*substream).runtime;
    let dma_private = (*runtime).private_data as *mut sprd_pcm_dma_private;
    let rtd = snd_soc_substream_to_rtd(substream);
    let mut dma_params: *mut sprd_pcm_dma_params;
    let totsize: usize = params_buffer_bytes(params);
    let period: usize = params_period_bytes(params);
    let channels: c_int = params_channels(params);
    let is_playback: c_int = ((*substream).stream == SNDRV_PCM_STREAM_PLAYBACK) as c_int;
    let mut sg: *mut scatterlist;
    let mut flags: c_ulong;
    let mut ret: c_int;
    let mut i: c_int;
    let mut j: c_int;
    let sg_num: c_int;

    dma_params = snd_soc_dai_get_dma_data(snd_soc_rtd_to_cpu(rtd, 0), substream)
        as *mut sprd_pcm_dma_params;
    if dma_params.is_null() {
        dev_warn(
            (*component).dev,
            b"no dma parameters setting\n\0".as_ptr() as *const c_char,
        );
        (*dma_private).params = ptr::null_mut();
        return 0;
    }

    if (*dma_private).params.is_null() {
        (*dma_private).params = dma_params;
        ret = sprd_pcm_request_dma_channel(component, substream, channels);
        if ret != 0 {
            return ret;
        }
    }

    sg_num = (totsize / period) as c_int;
    (*dma_private).dma_addr_offset = (totsize / channels as usize) as c_int;

    sg = devm_kcalloc(
        (*component).dev,
        sg_num as usize,
        size_of::<scatterlist>(),
        GFP_KERNEL,
    ) as *mut scatterlist;
    if sg.is_null() {
        ret = -ENOMEM;
        sprd_pcm_release_dma_channel(substream);
        return ret;
    }

    i = 0;
    while i < channels {
        let data = &mut (*dma_private).data[i as usize] as *mut sprd_pcm_dma_data;
        let chan = (*data).chan;
        let mut config: dma_slave_config = core::mem::zeroed();
        let mut link: sprd_dma_linklist = core::mem::zeroed();
        let dir: dma_transfer_direction;
        let mut sgt = sg;

        config.src_maxburst = (*dma_params).fragment_len[i as usize];
        config.src_addr_width = (*dma_params).datawidth[i as usize];
        config.dst_addr_width = (*dma_params).datawidth[i as usize];
        if is_playback != 0 {
            config.src_addr =
                (*runtime).dma_addr + (i * (*dma_private).dma_addr_offset) as dma_addr_t;
            config.dst_addr = (*dma_params).dev_phys[i as usize];
            dir = DMA_MEM_TO_DEV;
        } else {
            config.src_addr = (*dma_params).dev_phys[i as usize];
            config.dst_addr =
                (*runtime).dma_addr + (i * (*dma_private).dma_addr_offset) as dma_addr_t;
            dir = DMA_DEV_TO_MEM;
        }

        sg_init_table(sgt, sg_num as c_uint);
        j = 0;
        while j < sg_num {
            let sg_len: u32 = (period / channels as usize) as u32;

            sg_dma_len_set(sgt, sg_len);
            sg_dma_address_set(
                sgt,
                (*runtime).dma_addr
                    + (i * (*dma_private).dma_addr_offset) as dma_addr_t
                    + (sg_len as dma_addr_t) * (j as dma_addr_t),
            );

            sgt = sgt.add(1);
            j += 1;
        }

        /*
         * Configure the link-list address for the DMA engine link-list
         * mode.
         */
        link.virt_addr = (*data).virt as c_ulong;
        link.phy_addr = (*data).phys;

        ret = dmaengine_slave_config(chan, &mut config);
        if ret != 0 {
            dev_err(
                (*component).dev,
                b"failed to set slave configuration: %d\n\0".as_ptr() as *const c_char,
                ret,
            );
            devm_kfree((*component).dev, sg as *mut c_void);
            sprd_pcm_release_dma_channel(substream);
            return ret;
        }

        /*
         * We configure the DMA request mode, interrupt mode, channel
         * mode and channel trigger mode by the flags.
         */
        flags = SPRD_DMA_FLAGS(
            SPRD_DMA_CHN_MODE_NONE,
            SPRD_DMA_NO_TRG,
            SPRD_DMA_FRAG_REQ,
            SPRD_DMA_TRANS_INT,
        );
        (*data).desc = ((*(*chan).device).device_prep_slave_sg.unwrap())(
            chan,
            sg,
            sg_num as c_uint,
            dir,
            flags,
            &mut link as *mut sprd_dma_linklist as *mut c_void,
        );
        if (*data).desc.is_null() {
            dev_err(
                (*component).dev,
                b"failed to prepare slave sg\n\0".as_ptr() as *const c_char,
            );
            ret = -ENOMEM;
            devm_kfree((*component).dev, sg as *mut c_void);
            sprd_pcm_release_dma_channel(substream);
            return ret;
        }

        if !(*runtime).no_period_wakeup {
            (*(*data).desc).callback = Some(sprd_pcm_dma_complete);
            (*(*data).desc).callback_param = dma_private as *mut c_void;
        }

        i += 1;
    }

    devm_kfree((*component).dev, sg as *mut c_void);

    0
}

unsafe extern "C" fn sprd_pcm_hw_free(
    _component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> c_int {
    sprd_pcm_release_dma_channel(substream);

    0
}

unsafe extern "C" fn sprd_pcm_trigger(
    component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
    cmd: c_int,
) -> c_int {
    let dma_private = (*(*substream).runtime).private_data as *mut sprd_pcm_dma_private;
    let mut ret: c_int = 0;
    let mut i: c_int;

    match cmd {
        SNDRV_PCM_TRIGGER_START => {
            i = 0;
            while i < (*dma_private).hw_chan {
                let data = &mut (*dma_private).data[i as usize] as *mut sprd_pcm_dma_data;

                if (*data).desc.is_null() {
                    i += 1;
                    continue;
                }

                (*data).cookie = dmaengine_submit((*data).desc);
                ret = dma_submit_error((*data).cookie);
                if ret != 0 {
                    dev_err(
                        (*component).dev,
                        b"failed to submit dma request: %d\n\0".as_ptr() as *const c_char,
                        ret,
                    );
                    return ret;
                }

                dma_async_issue_pending((*data).chan);

                i += 1;
            }
        }
        SNDRV_PCM_TRIGGER_RESUME | SNDRV_PCM_TRIGGER_PAUSE_RELEASE => {
            i = 0;
            while i < (*dma_private).hw_chan {
                let data = &mut (*dma_private).data[i as usize] as *mut sprd_pcm_dma_data;

                if !(*data).chan.is_null() {
                    dmaengine_resume((*data).chan);
                }

                i += 1;
            }
        }
        SNDRV_PCM_TRIGGER_STOP => {
            i = 0;
            while i < (*dma_private).hw_chan {
                let data = &mut (*dma_private).data[i as usize] as *mut sprd_pcm_dma_data;

                if !(*data).chan.is_null() {
                    dmaengine_terminate_async((*data).chan);
                }

                i += 1;
            }
        }
        SNDRV_PCM_TRIGGER_SUSPEND | SNDRV_PCM_TRIGGER_PAUSE_PUSH => {
            i = 0;
            while i < (*dma_private).hw_chan {
                let data = &mut (*dma_private).data[i as usize] as *mut sprd_pcm_dma_data;

                if !(*data).chan.is_null() {
                    dmaengine_pause((*data).chan);
                }

                i += 1;
            }
        }
        _ => {
            ret = -EINVAL;
        }
    }

    ret
}

unsafe extern "C" fn sprd_pcm_pointer(
    component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> snd_pcm_uframes_t {
    let runtime = (*substream).runtime;
    let dma_private = (*runtime).private_data as *mut sprd_pcm_dma_private;
    let mut pointer: [c_int; SPRD_PCM_CHANNEL_MAX as usize] = [0; SPRD_PCM_CHANNEL_MAX as usize];
    let mut bytes_of_pointer: c_int = 0;
    let mut sel_max: c_int = 0;
    let mut i: c_int = 0;
    let mut x: snd_pcm_uframes_t;
    let mut state: dma_tx_state = core::mem::zeroed();
    let mut status: dma_status;

    while i < (*dma_private).hw_chan {
        let data = &mut (*dma_private).data[i as usize] as *mut sprd_pcm_dma_data;

        if (*data).chan.is_null() {
            i += 1;
            continue;
        }

        status = dmaengine_tx_status((*data).chan, (*data).cookie, &mut state);
        if status == DMA_ERROR {
            dev_err(
                (*component).dev,
                b"failed to get dma channel %d status\n\0".as_ptr() as *const c_char,
                i,
            );
            return 0;
        }

        /*
         * We just get current transfer address from the DMA engine, so
         * we need convert to current pointer.
         */
        pointer[i as usize] = (state.residue
            - (*runtime).dma_addr
            - (i * (*dma_private).dma_addr_offset) as dma_addr_t) as c_int;

        if i == 0 {
            bytes_of_pointer = pointer[i as usize];
            sel_max = if pointer[i as usize] < (*data).pre_pointer { 1 } else { 0 };
        } else {
            sel_max ^= if pointer[i as usize] < (*data).pre_pointer { 1 } else { 0 };

            if sel_max != 0 {
                bytes_of_pointer = core::cmp::max(pointer[i as usize], pointer[(i - 1) as usize]) << 1;
            } else {
                bytes_of_pointer = core::cmp::min(pointer[i as usize], pointer[(i - 1) as usize]) << 1;
            }
        }

        (*data).pre_pointer = pointer[i as usize];

        i += 1;
    }

    x = bytes_to_frames(runtime, bytes_of_pointer);
    if x == (*runtime).buffer_size {
        x = 0;
    }

    x
}

unsafe extern "C" fn sprd_pcm_new(
    _component: *mut snd_soc_component,
    rtd: *mut snd_soc_pcm_runtime,
) -> c_int {
    let card = (*(*rtd).card).snd_card;
    let pcm = (*rtd).pcm;
    let mut ret: c_int;

    ret = dma_coerce_mask_and_coherent((*card).dev, DMA_BIT_MASK(32));
    if ret != 0 {
        return ret;
    }

    snd_pcm_set_fixed_buffer_all(
        pcm,
        SNDRV_DMA_TYPE_DEV,
        (*card).dev,
        sprd_pcm_hardware.buffer_bytes_max,
    )
}

static sprd_soc_component: snd_soc_component_driver = snd_soc_component_driver {
    name: DRV_NAME,
    open: Some(sprd_pcm_open),
    close: Some(sprd_pcm_close),
    hw_params: Some(sprd_pcm_hw_params),
    hw_free: Some(sprd_pcm_hw_free),
    trigger: Some(sprd_pcm_trigger),
    pointer: Some(sprd_pcm_pointer),
    pcm_new: Some(sprd_pcm_new),
    compress_ops: unsafe { &sprd_platform_compress_ops },
};

unsafe extern "C" fn sprd_soc_platform_probe(pdev: *mut platform_device) -> c_int {
    let np = (*(*pdev).dev).of_node;
    let mut ret: c_int;

    ret = of_reserved_mem_device_init_by_idx(&mut (*pdev).dev, np, 0);
    if ret != 0 {
        dev_warn(
            &mut (*pdev).dev,
            b"no reserved DMA memory for audio platform device\n\0".as_ptr() as *const c_char,
        );
    }

    ret = devm_snd_soc_register_component(
        &mut (*pdev).dev,
        &sprd_soc_component,
        ptr::null_mut(),
        0,
    );

    ret
}

static sprd_pcm_of_match: [of_device_id; 2] = [
    of_device_id {
        compatible: b"sprd,pcm-platform\0".as_ptr() as *const c_char,
        ..unsafe { core::mem::zeroed() }
    },
    unsafe { core::mem::zeroed() },
];

// MODULE_DEVICE_TABLE(of, sprd_pcm_of_match);

static mut sprd_pcm_driver: platform_driver = platform_driver {
    driver: device_driver {
        name: b"sprd-pcm-audio\0".as_ptr() as *const c_char,
        of_match_table: sprd_pcm_of_match.as_ptr(),
        ..unsafe { core::mem::zeroed() }
    },

    probe: Some(sprd_soc_platform_probe),
};

// module_platform_driver(sprd_pcm_driver);
// MODULE_DESCRIPTION("Spreadtrum ASoC PCM DMA");
// MODULE_LICENSE("GPL v2");
// MODULE_ALIAS("platform:sprd-audio");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
