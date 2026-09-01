// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright(c) 2020 Intel Corporation
//
// Author: Cezary Rojewski <cezary.rojewski@intel.com>
//

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem;
use core::ptr;

type u8 = u8;
type u32 = u32;
type bool_ = bool;
type snd_pcm_uframes_t = c_ulong;

// Dependencies originally provided by Linux/ALSA and local catpt headers.
extern "C" {
    static mut catpt_volume_tlv: [u32; 0];

    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn snd_soc_rtd_to_cpu(rtd: *mut snd_soc_pcm_runtime, num: c_int) -> *mut snd_soc_dai;
    fn snd_soc_rtd_to_codec(rtd: *mut snd_soc_pcm_runtime, num: c_int) -> *mut snd_soc_dai;
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn kzalloc(size: usize, flags: c_uint) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn snd_dma_alloc_pages(
        ty: c_int,
        dev: *mut device,
        size: usize,
        dmab: *mut snd_dma_buffer,
    ) -> c_int;
    fn snd_dma_free_pages(dmab: *mut snd_dma_buffer);
    fn catpt_request_region(dram: *mut resource, size: u32) -> *mut resource;
    fn release_resource(res: *mut resource);
    fn catpt_dsp_update_srampge(cdev: *mut catpt_dev, dram: *mut resource, mask: u32);
    fn snd_soc_dai_set_dma_data(
        dai: *mut snd_soc_dai,
        substream: *mut snd_pcm_substream,
        data: *mut c_void,
    );
    fn snd_soc_dai_get_dma_data(
        dai: *mut snd_soc_dai,
        substream: *mut snd_pcm_substream,
    ) -> *mut c_void;
    fn snd_ctl_find_id_mixer(card: *mut snd_card, name: *const c_char) -> *mut snd_kcontrol;
    fn catpt_ipc_mute_loopback(cdev: *mut catpt_dev, stream_id: u8, mute: bool_) -> c_int;
    fn snd_pcm_get_dma_buf(substream: *mut snd_pcm_substream) -> *mut snd_dma_buffer;
    fn snd_sgbuf_aligned_pages(size: usize) -> c_int;
    fn snd_sgbuf_get_addr(dmab: *mut snd_dma_buffer, offset: usize) -> usize;
    fn catpt_ipc_alloc_stream(
        cdev: *mut catpt_dev,
        path_id: catpt_path_id,
        ty: catpt_stream_type,
        afmt: *mut catpt_audio_format,
        rinfo: *mut catpt_ring_info,
        num_entries: u8,
        entries: *mut catpt_module_entry,
        persistent: *mut resource,
        scratch: *mut resource,
        info: *mut catpt_stream_info,
    ) -> c_int;
    fn catpt_ipc_free_stream(cdev: *mut catpt_dev, stream_id: u8) -> c_int;
    fn catpt_ipc_reset_stream(cdev: *mut catpt_dev, stream_id: u8) -> c_int;
    fn catpt_ipc_pause_stream(cdev: *mut catpt_dev, stream_id: u8) -> c_int;
    fn catpt_ipc_resume_stream(cdev: *mut catpt_dev, stream_id: u8) -> c_int;
    fn catpt_ipc_set_write_pos(
        cdev: *mut catpt_dev,
        stream_id: u8,
        pos: snd_pcm_uframes_t,
        eob: bool_,
        ll: bool_,
    ) -> c_int;
    fn catpt_dsp_update_lpclock(cdev: *mut catpt_dev);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...) -> c_int;
    fn snd_pcm_period_elapsed(substream: *mut snd_pcm_substream);
    fn snd_pcm_set_managed_buffer_all(
        pcm: *mut snd_pcm,
        ty: c_int,
        dev: *mut device,
        min: usize,
        max: usize,
    );
    fn snd_soc_set_runtime_hwparams(
        substream: *mut snd_pcm_substream,
        hw: *const snd_pcm_hardware,
    );
    fn memcpy_fromio(dst: *mut c_void, src: *const c_void, size: usize);
    fn readl(addr: *const c_void) -> u32;
    fn pm_runtime_resume_and_get(dev: *mut device) -> c_int;
    fn pm_runtime_put_autosuspend(dev: *mut device);
    fn catpt_ipc_set_device_format(cdev: *mut catpt_dev, devfmt: *mut catpt_ssp_device_format) -> c_int;
    fn snd_kcontrol_chip(kctl: *mut snd_kcontrol) -> *mut c_void;
    fn catpt_ipc_set_volume(
        cdev: *mut catpt_dev,
        stream_id: u8,
        channel: c_int,
        volume: u32,
        curve_duration: c_int,
        curve_type: c_int,
    ) -> c_int;
    fn snd_soc_component_alloc(dev: *mut device) -> *mut snd_soc_component;
    fn snd_soc_component_set_name(component: *mut snd_soc_component, name: *const c_char);
    fn snd_soc_register_component(
        component: *mut snd_soc_component,
        driver: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: usize,
    ) -> c_int;
}

#[repr(C)]
struct list_head {
    next: *mut list_head,
    prev: *mut list_head,
}

#[repr(C)]
struct resource {
    _private: [u8; 0],
}
#[repr(C)]
struct device {
    _private: [u8; 0],
}
#[repr(C)]
struct snd_card {
    _private: [u8; 0],
}
#[repr(C)]
struct snd_pcm {
    _private: [u8; 0],
}
#[repr(C)]
struct snd_pcm_substream {
    runtime: *mut snd_pcm_runtime,
    stream: c_int,
}
#[repr(C)]
struct snd_pcm_runtime {
    dma_bytes: usize,
    start_threshold: snd_pcm_uframes_t,
    buffer_size: snd_pcm_uframes_t,
}
#[repr(C)]
struct snd_dma_buffer {
    area: *mut u8,
    addr: usize,
}
#[repr(C)]
struct snd_soc_pcm_runtime {
    pcm: *mut snd_pcm,
    dai_link: *mut snd_soc_dai_link,
}
#[repr(C)]
struct snd_soc_dai_link {
    no_pcm: bool_,
}
#[repr(C)]
struct snd_soc_dai {
    dev: *mut device,
    driver: *mut snd_soc_dai_driver,
    component: *mut snd_soc_component,
}
#[repr(C)]
struct snd_soc_component {
    dev: *mut device,
    card: *mut snd_soc_card,
}
#[repr(C)]
struct snd_soc_card {
    snd_card: *mut snd_card,
}
#[repr(C)]
struct snd_kcontrol {
    private_value: c_ulong,
}

#[repr(C)]
struct catpt_stream_template {
    path_id: catpt_path_id,
    type_: catpt_stream_type,
    persistent_size: u32,
    num_entries: u8,
    entries: [catpt_module_entry; 1],
}

#[repr(C)]
struct catpt_control_data {
    pin_id: catpt_pin_id,
    volumes: [c_long; CATPT_CHANNELS_MAX],
}

static mut SYSTEM_PB: catpt_stream_template = catpt_stream_template {
    path_id: CATPT_PATH_SSP0_OUT,
    type_: CATPT_STRM_TYPE_SYSTEM,
    persistent_size: 0,
    num_entries: 1,
    entries: [catpt_module_entry { module_id: CATPT_MODID_PCM_SYSTEM, entry_point: 0 }],
};
static mut SYSTEM_CP: catpt_stream_template = catpt_stream_template {
    path_id: CATPT_PATH_SSP0_IN,
    type_: CATPT_STRM_TYPE_CAPTURE,
    persistent_size: 0,
    num_entries: 1,
    entries: [catpt_module_entry { module_id: CATPT_MODID_PCM_CAPTURE, entry_point: 0 }],
};
static mut OFFLOAD_PB: catpt_stream_template = catpt_stream_template {
    path_id: CATPT_PATH_SSP0_OUT,
    type_: CATPT_STRM_TYPE_RENDER,
    persistent_size: 0,
    num_entries: 1,
    entries: [catpt_module_entry { module_id: CATPT_MODID_PCM, entry_point: 0 }],
};
static mut LOOPBACK_CP: catpt_stream_template = catpt_stream_template {
    path_id: CATPT_PATH_SSP0_OUT,
    type_: CATPT_STRM_TYPE_LOOPBACK,
    persistent_size: 0,
    num_entries: 1,
    entries: [catpt_module_entry { module_id: CATPT_MODID_PCM_REFERENCE, entry_point: 0 }],
};
static mut BLUETOOTH_PB: catpt_stream_template = catpt_stream_template {
    path_id: CATPT_PATH_SSP1_OUT,
    type_: CATPT_STRM_TYPE_BLUETOOTH_RENDER,
    persistent_size: 0,
    num_entries: 1,
    entries: [catpt_module_entry { module_id: CATPT_MODID_BLUETOOTH_RENDER, entry_point: 0 }],
};
static mut BLUETOOTH_CP: catpt_stream_template = catpt_stream_template {
    path_id: CATPT_PATH_SSP1_IN,
    type_: CATPT_STRM_TYPE_BLUETOOTH_CAPTURE,
    persistent_size: 0,
    num_entries: 1,
    entries: [catpt_module_entry { module_id: CATPT_MODID_BLUETOOTH_CAPTURE, entry_point: 0 }],
};

static mut CATPT_TOPOLOGY: [*mut catpt_stream_template; 6] = [
    &raw mut OFFLOAD_PB,
    &raw mut SYSTEM_PB,
    &raw mut SYSTEM_CP,
    &raw mut LOOPBACK_CP,
    &raw mut BLUETOOTH_PB,
    &raw mut BLUETOOTH_CP,
];

unsafe fn catpt_get_stream_template(substream: *mut snd_pcm_substream) -> *mut catpt_stream_template {
    let rtd = snd_soc_substream_to_rtd(substream);
    let cpu_dai = snd_soc_rtd_to_cpu(rtd, 0);
    let mut type_ = (*(*cpu_dai).driver).id as catpt_stream_type;

    /* account for capture in bidirectional dais */
    match type_ {
        CATPT_STRM_TYPE_SYSTEM => {
            if (*substream).stream == SNDRV_PCM_STREAM_CAPTURE {
                type_ = CATPT_STRM_TYPE_CAPTURE;
            }
        }
        CATPT_STRM_TYPE_BLUETOOTH_RENDER => {
            if (*substream).stream == SNDRV_PCM_STREAM_CAPTURE {
                type_ = CATPT_STRM_TYPE_BLUETOOTH_CAPTURE;
            }
        }
        _ => {}
    }

    CATPT_TOPOLOGY[type_ as usize]
}

/* Caller responsible for holding ->stream_mutex. */
#[no_mangle]
pub unsafe extern "C" fn catpt_stream_find(
    cdev: *mut catpt_dev,
    stream_hw_id: u8,
) -> *mut catpt_stream_runtime {
    let mut pos = (*cdev).stream_list.next as *mut catpt_stream_runtime;

    while !ptr::eq(&raw mut (*pos).node, &raw mut (*cdev).stream_list) {
        if (*pos).info.stream_hw_id == stream_hw_id {
            return pos;
        }
        pos = (*pos).node.next as *mut catpt_stream_runtime;
    }

    ptr::null_mut()
}

/* Caller responsible for holding ->stream_mutex. */
unsafe fn catpt_stream_hw_id(cdev: *mut catpt_dev, pin_id: catpt_pin_id) -> u8 {
    match pin_id {
        CATPT_PIN_ID_MIXER => {
            if !list_empty(&raw mut (*cdev).stream_list) {
                return (*cdev).mixer.mixer_hw_id;
            }
        }
        _ => {
            let stream = catpt_stream_find(cdev, pin_id as u8);
            if !stream.is_null() {
                return (*stream).info.stream_hw_id;
            }
        }
    }

    CATPT_PIN_ID_INVALID as u8
}

/* Caller responsible for holding ->stream_mutex. */
unsafe fn catpt_stream_volume_regs(cdev: *mut catpt_dev, pin_id: catpt_pin_id) -> *mut u32 {
    match pin_id {
        CATPT_PIN_ID_MIXER => {
            if !list_empty(&raw mut (*cdev).stream_list) {
                return (*cdev).mixer.volume_regaddr.as_mut_ptr();
            }
        }
        _ => {
            let stream = catpt_stream_find(cdev, pin_id as u8);
            if !stream.is_null() {
                return (*stream).info.volume_regaddr.as_mut_ptr();
            }
        }
    }

    ptr::null_mut()
}

unsafe fn catpt_stream_read_position(
    cdev: *mut catpt_dev,
    stream: *mut catpt_stream_runtime,
    pos: *mut u32,
) {
    memcpy_fromio(
        pos as *mut c_void,
        (*cdev).lpe_ba.add((*stream).info.read_pos_regaddr as usize),
        mem::size_of_val(&*pos),
    );
}

unsafe fn catpt_arrange_page_table(substream: *mut snd_pcm_substream, pgtbl: *mut snd_dma_buffer) {
    let runtime = (*substream).runtime;
    let databuf = snd_pcm_get_dma_buf(substream);
    let pages = snd_sgbuf_aligned_pages((*runtime).dma_bytes);

    for i in 0..pages {
        let pfn = PFN_DOWN(snd_sgbuf_get_addr(databuf, i as usize * PAGE_SIZE)) as u32;
        /* incrementing by 2 on even and 3 on odd */
        let offset = (((i << 2) + i) >> 1) as usize;
        let page_table = (*pgtbl).area.add(offset) as *mut u32;

        if (i & 1) != 0 {
            *page_table |= pfn << 4;
        } else {
            *page_table |= pfn;
        }
    }
}

fn catpt_get_channel_map(config: catpt_channel_config) -> u32 {
    match config {
        CATPT_CHANNEL_CONFIG_MONO => GENMASK(31, 4) | CATPT_CHANNEL_CENTER,
        CATPT_CHANNEL_CONFIG_STEREO => {
            GENMASK(31, 8) | CATPT_CHANNEL_LEFT | (CATPT_CHANNEL_RIGHT << 4)
        }
        CATPT_CHANNEL_CONFIG_2_POINT_1 => {
            GENMASK(31, 12)
                | CATPT_CHANNEL_LEFT
                | (CATPT_CHANNEL_RIGHT << 4)
                | (CATPT_CHANNEL_LFE << 8)
        }
        CATPT_CHANNEL_CONFIG_3_POINT_0 => {
            GENMASK(31, 12)
                | CATPT_CHANNEL_LEFT
                | (CATPT_CHANNEL_CENTER << 4)
                | (CATPT_CHANNEL_RIGHT << 8)
        }
        CATPT_CHANNEL_CONFIG_3_POINT_1 => {
            GENMASK(31, 16)
                | CATPT_CHANNEL_LEFT
                | (CATPT_CHANNEL_CENTER << 4)
                | (CATPT_CHANNEL_RIGHT << 8)
                | (CATPT_CHANNEL_LFE << 12)
        }
        CATPT_CHANNEL_CONFIG_QUATRO => {
            GENMASK(31, 16)
                | CATPT_CHANNEL_LEFT
                | (CATPT_CHANNEL_RIGHT << 4)
                | (CATPT_CHANNEL_LEFT_SURROUND << 8)
                | (CATPT_CHANNEL_RIGHT_SURROUND << 12)
        }
        CATPT_CHANNEL_CONFIG_4_POINT_0 => {
            GENMASK(31, 16)
                | CATPT_CHANNEL_LEFT
                | (CATPT_CHANNEL_CENTER << 4)
                | (CATPT_CHANNEL_RIGHT << 8)
                | (CATPT_CHANNEL_CENTER_SURROUND << 12)
        }
        CATPT_CHANNEL_CONFIG_5_POINT_0 => {
            GENMASK(31, 20)
                | CATPT_CHANNEL_LEFT
                | (CATPT_CHANNEL_CENTER << 4)
                | (CATPT_CHANNEL_RIGHT << 8)
                | (CATPT_CHANNEL_LEFT_SURROUND << 12)
                | (CATPT_CHANNEL_RIGHT_SURROUND << 16)
        }
        CATPT_CHANNEL_CONFIG_5_POINT_1 => {
            GENMASK(31, 24)
                | CATPT_CHANNEL_CENTER
                | (CATPT_CHANNEL_LEFT << 4)
                | (CATPT_CHANNEL_RIGHT << 8)
                | (CATPT_CHANNEL_LEFT_SURROUND << 12)
                | (CATPT_CHANNEL_RIGHT_SURROUND << 16)
                | (CATPT_CHANNEL_LFE << 20)
        }
        CATPT_CHANNEL_CONFIG_DUAL_MONO => {
            GENMASK(31, 8) | CATPT_CHANNEL_LEFT | (CATPT_CHANNEL_LEFT << 4)
        }
        _ => u32::MAX,
    }
}

fn catpt_get_channel_config(num_channels: u32) -> catpt_channel_config {
    match num_channels {
        6 => CATPT_CHANNEL_CONFIG_5_POINT_1,
        5 => CATPT_CHANNEL_CONFIG_5_POINT_0,
        4 => CATPT_CHANNEL_CONFIG_QUATRO,
        3 => CATPT_CHANNEL_CONFIG_2_POINT_1,
        1 => CATPT_CHANNEL_CONFIG_MONO,
        2 | _ => CATPT_CHANNEL_CONFIG_STEREO,
    }
}

unsafe extern "C" fn catpt_dai_startup(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> c_int {
    let template = catpt_get_stream_template(substream);
    let cdev = dev_get_drvdata((*dai).dev) as *mut catpt_dev;
    let stream = kzalloc(mem::size_of::<catpt_stream_runtime>(), GFP_KERNEL) as *mut catpt_stream_runtime;
    if stream.is_null() {
        return -ENOMEM;
    }

    let mut ret = snd_dma_alloc_pages(SNDRV_DMA_TYPE_DEV, (*cdev).dev, PAGE_SIZE, &raw mut (*stream).pgtbl);
    if ret != 0 {
        kfree(stream as *mut c_void);
        return ret;
    }

    let res = catpt_request_region(&raw mut (*cdev).dram, (*template).persistent_size);
    if res.is_null() {
        ret = -EBUSY;
        snd_dma_free_pages(&raw mut (*stream).pgtbl);
        kfree(stream as *mut c_void);
        return ret;
    }

    catpt_dsp_update_srampge(cdev, &raw mut (*cdev).dram, (*(*cdev).spec).dram_mask);
    (*stream).template = template;
    (*stream).persistent = res;
    (*stream).substream = substream;
    INIT_LIST_HEAD(&raw mut (*stream).node);
    snd_soc_dai_set_dma_data(dai, substream, stream as *mut c_void);

    0
}

unsafe extern "C" fn catpt_dai_shutdown(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) {
    let stream = snd_soc_dai_get_dma_data(dai, substream) as *mut catpt_stream_runtime;
    let cdev = dev_get_drvdata((*dai).dev) as *mut catpt_dev;

    release_resource((*stream).persistent);
    kfree((*stream).persistent as *mut c_void);
    catpt_dsp_update_srampge(cdev, &raw mut (*cdev).dram, (*(*cdev).spec).dram_mask);
    snd_dma_free_pages(&raw mut (*stream).pgtbl);
    kfree(stream as *mut c_void);
    snd_soc_dai_set_dma_data(dai, substream, ptr::null_mut());
}

unsafe fn catpt_apply_volume(
    cdev: *mut catpt_dev,
    card: *mut snd_soc_card,
    name: *const c_char,
) -> c_int {
    let kctl = snd_ctl_find_id_mixer((*card).snd_card, name);
    if kctl.is_null() {
        return -ENOENT;
    }
    let data = (*kctl).private_value as *mut catpt_control_data;

    catpt_set_dspvol(cdev, (*data).pin_id as u8, (*data).volumes.as_mut_ptr())
}

unsafe fn catpt_apply_mute(cdev: *mut catpt_dev, card: *mut snd_soc_card) -> c_int {
    let kctl = snd_ctl_find_id_mixer((*card).snd_card, c"Loopback Mute".as_ptr());
    if kctl.is_null() {
        return -ENOENT;
    }
    let mute = *((*kctl).private_value as *mut bool_);
    let ret = catpt_ipc_mute_loopback(cdev, CATPT_PIN_ID_REFERENCE as u8, mute);
    CATPT_IPC_RET(ret)
}

unsafe fn catpt_apply_controls(
    cdev: *mut catpt_dev,
    card: *mut snd_soc_card,
    stream: *mut catpt_stream_runtime,
) -> c_int {
    /* Update the master volume when the first stream is opened. */
    if list_empty(&raw mut (*cdev).stream_list) {
        let ret = catpt_apply_volume(cdev, card, c"Master Playback Volume".as_ptr());
        if ret != 0 {
            return ret;
        }
    }

    /* Only selected streams have individual controls. */
    match (*stream).info.stream_hw_id as c_int {
        CATPT_PIN_ID_OFFLOAD1 => catpt_apply_volume(cdev, card, c"Media0 Playback Volume".as_ptr()),
        CATPT_PIN_ID_OFFLOAD2 => catpt_apply_volume(cdev, card, c"Media1 Playback Volume".as_ptr()),
        CATPT_PIN_ID_CAPTURE1 => catpt_apply_volume(cdev, card, c"Mic Capture Volume".as_ptr()),
        CATPT_PIN_ID_REFERENCE => catpt_apply_mute(cdev, card),
        _ => 0,
    }
}

unsafe extern "C" fn catpt_dai_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let runtime = (*substream).runtime;
    let mut afmt: catpt_audio_format = mem::zeroed();
    let mut rinfo: catpt_ring_info = mem::zeroed();
    let cdev = dev_get_drvdata((*dai).dev) as *mut catpt_dev;
    let stream = snd_soc_dai_get_dma_data(dai, substream) as *mut catpt_stream_runtime;

    if (*stream).allocated {
        return 0;
    }

    afmt.sample_rate = params_rate(params);
    afmt.bit_depth = params_physical_width(params);
    afmt.valid_bit_depth = params_width(params);
    afmt.num_channels = params_channels(params);
    afmt.channel_config = catpt_get_channel_config(afmt.num_channels);
    afmt.channel_map = catpt_get_channel_map(afmt.channel_config);
    afmt.interleaving = CATPT_INTERLEAVING_PER_CHANNEL;

    let dmab = snd_pcm_get_dma_buf(substream);
    catpt_arrange_page_table(substream, &raw mut (*stream).pgtbl);

    rinfo.page_table_addr = (*stream).pgtbl.addr;
    rinfo.num_pages = DIV_ROUND_UP((*runtime).dma_bytes, PAGE_SIZE) as u32;
    rinfo.size = (*runtime).dma_bytes as u32;
    rinfo.offset = 0;
    rinfo.ring_first_page_pfn = PFN_DOWN(snd_sgbuf_get_addr(dmab, 0)) as u32;

    let mut ret = catpt_ipc_alloc_stream(
        cdev,
        (*(*stream).template).path_id,
        (*(*stream).template).type_,
        &raw mut afmt,
        &raw mut rinfo,
        (*(*stream).template).num_entries,
        (*(*stream).template).entries.as_mut_ptr(),
        (*stream).persistent,
        (*cdev).scratch,
        &raw mut (*stream).info,
    );
    if ret != 0 {
        return CATPT_IPC_RET(ret);
    }

    mutex_lock(&raw mut (*cdev).stream_mutex);
    ret = catpt_apply_controls(cdev, (*(*dai).component).card, stream);
    if ret != 0 {
        catpt_ipc_free_stream(cdev, (*stream).info.stream_hw_id);
        mutex_unlock(&raw mut (*cdev).stream_mutex);
        return ret;
    }

    list_add_tail(&raw mut (*stream).node, &raw mut (*cdev).stream_list);
    (*stream).allocated = true;
    mutex_unlock(&raw mut (*cdev).stream_mutex);
    0
}

unsafe extern "C" fn catpt_dai_hw_free(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> c_int {
    let cdev = dev_get_drvdata((*dai).dev) as *mut catpt_dev;
    let stream = snd_soc_dai_get_dma_data(dai, substream) as *mut catpt_stream_runtime;
    if !(*stream).allocated {
        return 0;
    }

    mutex_lock(&raw mut (*cdev).stream_mutex);
    list_del(&raw mut (*stream).node);
    mutex_unlock(&raw mut (*cdev).stream_mutex);

    catpt_ipc_reset_stream(cdev, (*stream).info.stream_hw_id);
    catpt_ipc_free_stream(cdev, (*stream).info.stream_hw_id);

    (*stream).allocated = false;
    0
}

unsafe extern "C" fn catpt_dai_prepare(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> c_int {
    let cdev = dev_get_drvdata((*dai).dev) as *mut catpt_dev;
    let stream = snd_soc_dai_get_dma_data(dai, substream) as *mut catpt_stream_runtime;
    if (*stream).prepared {
        return 0;
    }

    let mut ret = catpt_ipc_reset_stream(cdev, (*stream).info.stream_hw_id);
    if ret != 0 {
        return CATPT_IPC_RET(ret);
    }

    ret = catpt_ipc_pause_stream(cdev, (*stream).info.stream_hw_id);
    if ret != 0 {
        return CATPT_IPC_RET(ret);
    }

    (*stream).prepared = true;
    0
}

unsafe extern "C" fn catpt_dai_trigger(
    substream: *mut snd_pcm_substream,
    cmd: c_int,
    dai: *mut snd_soc_dai,
) -> c_int {
    let runtime = (*substream).runtime;
    let cdev = dev_get_drvdata((*dai).dev) as *mut catpt_dev;
    let stream = snd_soc_dai_get_dma_data(dai, substream) as *mut catpt_stream_runtime;

    match cmd {
        SNDRV_PCM_TRIGGER_START => {
            /* only offload is set_write_pos driven */
            if (*(*stream).template).type_ == CATPT_STRM_TYPE_RENDER {
                let pos = frames_to_bytes(runtime, (*runtime).start_threshold);
                /*
                 * Dsp operates on buffer halves, thus max 2x set_write_pos
                 * (entire buffer filled) prior to stream start.
                 */
                let ret = catpt_ipc_set_write_pos(cdev, (*stream).info.stream_hw_id, pos, false, false);
                if ret != 0 {
                    return CATPT_IPC_RET(ret);
                }
            }
            catpt_dsp_update_lpclock(cdev);
            let ret = catpt_ipc_resume_stream(cdev, (*stream).info.stream_hw_id);
            if ret != 0 {
                return CATPT_IPC_RET(ret);
            }
        }
        SNDRV_PCM_TRIGGER_RESUME | SNDRV_PCM_TRIGGER_PAUSE_RELEASE => {
            catpt_dsp_update_lpclock(cdev);
            let ret = catpt_ipc_resume_stream(cdev, (*stream).info.stream_hw_id);
            if ret != 0 {
                return CATPT_IPC_RET(ret);
            }
        }
        SNDRV_PCM_TRIGGER_STOP => {
            (*stream).prepared = false;
            let ret = catpt_ipc_pause_stream(cdev, (*stream).info.stream_hw_id);
            catpt_dsp_update_lpclock(cdev);
            if ret != 0 {
                return CATPT_IPC_RET(ret);
            }
        }
        SNDRV_PCM_TRIGGER_SUSPEND | SNDRV_PCM_TRIGGER_PAUSE_PUSH => {
            let ret = catpt_ipc_pause_stream(cdev, (*stream).info.stream_hw_id);
            catpt_dsp_update_lpclock(cdev);
            if ret != 0 {
                return CATPT_IPC_RET(ret);
            }
        }
        _ => {}
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn catpt_stream_update_position(
    cdev: *mut catpt_dev,
    stream: *mut catpt_stream_runtime,
    pos: *mut catpt_notify_position,
) {
    let substream = (*stream).substream;
    let runtime = (*substream).runtime;
    let dsppos = bytes_to_frames(runtime, (*pos).stream_position);

    if (*stream).prepared && (*(*stream).template).type_ == CATPT_STRM_TYPE_RENDER {
        let newpos = if dsppos >= (*runtime).buffer_size / 2 {
            (*runtime).buffer_size / 2
        } else {
            0
        };
        /*
         * Dsp operates on buffer halves, thus on every notify position
         * (buffer half consumed) update wp to allow stream progression.
         */
        let ret = catpt_ipc_set_write_pos(
            cdev,
            (*stream).info.stream_hw_id,
            frames_to_bytes(runtime, newpos),
            false,
            false,
        );
        if ret != 0 {
            dev_err(
                (*cdev).dev,
                c"update position for stream %d failed: %d\n".as_ptr(),
                (*stream).info.stream_hw_id as c_int,
                ret,
            );
            return;
        }
    }
    snd_pcm_period_elapsed(substream);
}

/* 200 ms for 2 32-bit channels at 48kHz (native format) */
const CATPT_BUFFER_MAX_SIZE: usize = 76800;
const CATPT_PCM_PERIODS_MAX: u32 = 4;
const CATPT_PCM_PERIODS_MIN: u32 = 2;

static CATPT_PCM_HARDWARE: snd_pcm_hardware = snd_pcm_hardware {
    info: SNDRV_PCM_INFO_MMAP
        | SNDRV_PCM_INFO_MMAP_VALID
        | SNDRV_PCM_INFO_INTERLEAVED
        | SNDRV_PCM_INFO_PAUSE
        | SNDRV_PCM_INFO_RESUME
        | SNDRV_PCM_INFO_NO_PERIOD_WAKEUP,
    formats: SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S32_LE,
    subformats: SNDRV_PCM_SUBFMTBIT_MSBITS_24 | SNDRV_PCM_SUBFMTBIT_MSBITS_MAX,
    period_bytes_min: PAGE_SIZE,
    period_bytes_max: CATPT_BUFFER_MAX_SIZE / CATPT_PCM_PERIODS_MIN as usize,
    periods_min: CATPT_PCM_PERIODS_MIN,
    periods_max: CATPT_PCM_PERIODS_MAX,
    buffer_bytes_max: CATPT_BUFFER_MAX_SIZE,
};

unsafe extern "C" fn catpt_component_pcm_new(
    component: *mut snd_soc_component,
    rtd: *mut snd_soc_pcm_runtime,
) -> c_int {
    let cdev = dev_get_drvdata((*component).dev) as *mut catpt_dev;

    snd_pcm_set_managed_buffer_all(
        (*rtd).pcm,
        SNDRV_DMA_TYPE_DEV_SG,
        (*cdev).dev,
        CATPT_PCM_HARDWARE.buffer_bytes_max,
        CATPT_PCM_HARDWARE.buffer_bytes_max,
    );

    0
}

unsafe extern "C" fn catpt_component_open(
    _component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);

    if !(*(*rtd).dai_link).no_pcm {
        snd_soc_set_runtime_hwparams(substream, &raw const CATPT_PCM_HARDWARE);
    }
    0
}

unsafe extern "C" fn catpt_component_pointer(
    component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> snd_pcm_uframes_t {
    let rtd = snd_soc_substream_to_rtd(substream);
    let cpu_dai = snd_soc_rtd_to_cpu(rtd, 0);
    let cdev = dev_get_drvdata((*component).dev) as *mut catpt_dev;
    let mut pos: u32 = 0;

    if (*(*rtd).dai_link).no_pcm {
        return 0;
    }

    let stream = snd_soc_dai_get_dma_data(cpu_dai, substream) as *mut catpt_stream_runtime;
    catpt_stream_read_position(cdev, stream, &raw mut pos);

    bytes_to_frames((*substream).runtime, pos)
}

static CATPT_FE_DAI_OPS: snd_soc_dai_ops = snd_soc_dai_ops {
    startup: Some(catpt_dai_startup),
    shutdown: Some(catpt_dai_shutdown),
    hw_params: Some(catpt_dai_hw_params),
    hw_free: Some(catpt_dai_hw_free),
    prepare: Some(catpt_dai_prepare),
    trigger: Some(catpt_dai_trigger),
    pcm_new: None,
};

unsafe extern "C" fn catpt_dai_pcm_new(
    rtd: *mut snd_soc_pcm_runtime,
    dai: *mut snd_soc_dai,
) -> c_int {
    let codec_dai = snd_soc_rtd_to_codec(rtd, 0);
    let mut devfmt: catpt_ssp_device_format = mem::zeroed();
    let cdev = dev_get_drvdata((*dai).dev) as *mut catpt_dev;

    devfmt.iface = (*(*dai).driver).id;
    devfmt.channels = (*(*codec_dai).driver).capture.channels_max;

    match devfmt.iface {
        CATPT_SSP_IFACE_0 => {
            devfmt.mclk = CATPT_MCLK_FREQ_24_MHZ;
            match devfmt.channels {
                4 => {
                    devfmt.mode = CATPT_SSP_MODE_TDM_PROVIDER;
                    devfmt.clock_divider = 4;
                }
                2 | _ => {
                    devfmt.mode = CATPT_SSP_MODE_I2S_PROVIDER;
                    devfmt.clock_divider = 9;
                }
            }
        }
        CATPT_SSP_IFACE_1 => {
            devfmt.mclk = CATPT_MCLK_OFF;
            devfmt.mode = CATPT_SSP_MODE_I2S_CONSUMER;
            devfmt.clock_divider = 0;
        }
        _ => {}
    }

    /* see if this is a new configuration */
    if memcmp(
        &raw mut (*cdev).devfmt[devfmt.iface as usize] as *const c_void,
        &raw const devfmt as *const c_void,
        mem::size_of_val(&devfmt),
    ) == 0
    {
        return 0;
    }

    let mut ret = pm_runtime_resume_and_get((*cdev).dev);
    if ret != 0 {
        return ret;
    }

    ret = catpt_ipc_set_device_format(cdev, &raw mut devfmt);
    pm_runtime_put_autosuspend((*cdev).dev);
    if ret != 0 {
        return CATPT_IPC_RET(ret);
    }

    /* store device format set for given SSP */
    ptr::copy_nonoverlapping(
        &raw const devfmt as *const u8,
        &raw mut (*cdev).devfmt[devfmt.iface as usize] as *mut u8,
        mem::size_of_val(&devfmt),
    );
    0
}

static CATPT_DAI_OPS: snd_soc_dai_ops = snd_soc_dai_ops {
    startup: None,
    shutdown: None,
    hw_params: None,
    hw_free: None,
    prepare: None,
    trigger: None,
    pcm_new: Some(catpt_dai_pcm_new),
};

static mut DAI_DRIVERS: [snd_soc_dai_driver; 7] = [
    snd_soc_dai_driver { name: c"System Pin".as_ptr(), id: CATPT_STRM_TYPE_SYSTEM as c_int, ops: &raw const CATPT_FE_DAI_OPS, playback: dai_stream("System Playback", 2, 2, SNDRV_PCM_RATE_48000, SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S32_LE, SNDRV_PCM_SUBFMTBIT_MSBITS_24 | SNDRV_PCM_SUBFMTBIT_MSBITS_MAX), capture: dai_stream("Analog Capture", 2, 4, SNDRV_PCM_RATE_48000, SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S32_LE, SNDRV_PCM_SUBFMTBIT_MSBITS_24 | SNDRV_PCM_SUBFMTBIT_MSBITS_MAX) },
    snd_soc_dai_driver { name: c"Offload0 Pin".as_ptr(), id: CATPT_STRM_TYPE_RENDER as c_int, ops: &raw const CATPT_FE_DAI_OPS, playback: dai_stream("Offload0 Playback", 2, 2, SNDRV_PCM_RATE_8000_192000, SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S32_LE, SNDRV_PCM_SUBFMTBIT_MSBITS_24 | SNDRV_PCM_SUBFMTBIT_MSBITS_MAX), capture: snd_soc_pcm_stream::zeroed() },
    snd_soc_dai_driver { name: c"Offload1 Pin".as_ptr(), id: CATPT_STRM_TYPE_RENDER as c_int, ops: &raw const CATPT_FE_DAI_OPS, playback: dai_stream("Offload1 Playback", 2, 2, SNDRV_PCM_RATE_8000_192000, SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S32_LE, SNDRV_PCM_SUBFMTBIT_MSBITS_24 | SNDRV_PCM_SUBFMTBIT_MSBITS_MAX), capture: snd_soc_pcm_stream::zeroed() },
    snd_soc_dai_driver { name: c"Loopback Pin".as_ptr(), id: CATPT_STRM_TYPE_LOOPBACK as c_int, ops: &raw const CATPT_FE_DAI_OPS, playback: snd_soc_pcm_stream::zeroed(), capture: dai_stream("Loopback Capture", 2, 2, SNDRV_PCM_RATE_48000, SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S32_LE, SNDRV_PCM_SUBFMTBIT_MSBITS_24 | SNDRV_PCM_SUBFMTBIT_MSBITS_MAX) },
    snd_soc_dai_driver { name: c"Bluetooth Pin".as_ptr(), id: CATPT_STRM_TYPE_BLUETOOTH_RENDER as c_int, ops: &raw const CATPT_FE_DAI_OPS, playback: dai_stream("Bluetooth Playback", 1, 1, SNDRV_PCM_RATE_8000, SNDRV_PCM_FMTBIT_S16_LE, 0), capture: dai_stream("Bluetooth Capture", 1, 1, SNDRV_PCM_RATE_8000, SNDRV_PCM_FMTBIT_S16_LE, 0) },
    snd_soc_dai_driver { name: c"ssp0-port".as_ptr(), id: CATPT_SSP_IFACE_0, ops: &raw const CATPT_DAI_OPS, playback: dai_stream_null(1, 8), capture: dai_stream_null(1, 8) },
    snd_soc_dai_driver { name: c"ssp1-port".as_ptr(), id: CATPT_SSP_IFACE_1, ops: &raw const CATPT_DAI_OPS, playback: dai_stream_null(1, 8), capture: dai_stream_null(1, 8) },
];

const DSP_VOLUME_MAX: u32 = i32::MAX as u32; /* 0db */
const DSP_VOLUME_STEP_MAX: u32 = 30;

fn ctlvol_to_dspvol(mut value: u32) -> u32 {
    if value > DSP_VOLUME_STEP_MAX {
        value = 0;
    }
    DSP_VOLUME_MAX >> (DSP_VOLUME_STEP_MAX - value)
}

fn dspvol_to_ctlvol(volume: u32) -> u32 {
    if volume > DSP_VOLUME_MAX {
        return DSP_VOLUME_STEP_MAX;
    }
    if volume != 0 { __fls(volume) } else { 0 }
}

unsafe fn catpt_set_dspvol(cdev: *mut catpt_dev, stream_id: u8, ctlvol: *mut c_long) -> c_int {
    let mut i = 1usize;
    while i < CATPT_CHANNELS_MAX {
        if *ctlvol.add(i) != *ctlvol {
            break;
        }
        i += 1;
    }

    let mut ret = 0;
    if i == CATPT_CHANNELS_MAX {
        let dspvol = ctlvol_to_dspvol(*ctlvol as u32);
        ret = catpt_ipc_set_volume(
            cdev,
            stream_id,
            CATPT_ALL_CHANNELS_MASK,
            dspvol,
            0,
            CATPT_AUDIO_CURVE_NONE,
        );
    } else {
        i = 0;
        while i < CATPT_CHANNELS_MAX {
            let dspvol = ctlvol_to_dspvol(*ctlvol.add(i) as u32);
            ret = catpt_ipc_set_volume(
                cdev,
                stream_id,
                i as c_int,
                dspvol,
                0,
                CATPT_AUDIO_CURVE_NONE,
            );
            if ret != 0 {
                break;
            }
            i += 1;
        }
    }

    CATPT_IPC_RET(ret)
}

unsafe extern "C" fn catpt_volume_info(
    _kctl: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
    (*uinfo).count = CATPT_CHANNELS_MAX as c_uint;
    (*uinfo).value.integer.min = 0;
    (*uinfo).value.integer.max = DSP_VOLUME_STEP_MAX as c_long;
    0
}

unsafe extern "C" fn catpt_volume_get(
    kctl: *mut snd_kcontrol,
    uctl: *mut snd_ctl_elem_value,
) -> c_int {
    let component = snd_kcontrol_chip(kctl) as *mut snd_soc_component;
    let cdev = dev_get_drvdata((*component).dev) as *mut catpt_dev;
    let data = (*kctl).private_value as *mut catpt_control_data;
    let uvolumes = (*uctl).value.integer.value.as_mut_ptr();

    mutex_lock(&raw mut (*cdev).stream_mutex);
    let regs = catpt_stream_volume_regs(cdev, (*data).pin_id);
    if !regs.is_null() {
        for i in 0..CATPT_CHANNELS_MAX {
            let dspvol = readl((*cdev).lpe_ba.add(*regs.add(i) as usize));
            (*data).volumes[i] = dspvol_to_ctlvol(dspvol) as c_long;
        }
    }
    mutex_unlock(&raw mut (*cdev).stream_mutex);

    ptr::copy_nonoverlapping((*data).volumes.as_ptr(), uvolumes, CATPT_CHANNELS_MAX);
    0
}

unsafe extern "C" fn catpt_volume_put(
    kctl: *mut snd_kcontrol,
    uctl: *mut snd_ctl_elem_value,
) -> c_int {
    let component = snd_kcontrol_chip(kctl) as *mut snd_soc_component;
    let cdev = dev_get_drvdata((*component).dev) as *mut catpt_dev;
    let data = (*kctl).private_value as *mut catpt_control_data;
    let uvolumes = (*uctl).value.integer.value.as_mut_ptr();

    if memcmp(
        (*data).volumes.as_ptr() as *const c_void,
        uvolumes as *const c_void,
        mem::size_of_val(&(*data).volumes),
    ) == 0
    {
        return 0;
    }

    mutex_lock(&raw mut (*cdev).stream_mutex);
    let stream_hw_id = catpt_stream_hw_id(cdev, (*data).pin_id);
    if stream_hw_id != CATPT_PIN_ID_INVALID as u8 {
        let ret = catpt_set_dspvol(cdev, stream_hw_id, uvolumes);
        if ret != 0 {
            mutex_unlock(&raw mut (*cdev).stream_mutex);
            return ret;
        }
    }
    mutex_unlock(&raw mut (*cdev).stream_mutex);

    ptr::copy_nonoverlapping(uvolumes, (*data).volumes.as_mut_ptr(), CATPT_CHANNELS_MAX);
    1
}

unsafe extern "C" fn catpt_loopback_mute_get(
    kctl: *mut snd_kcontrol,
    uctl: *mut snd_ctl_elem_value,
) -> c_int {
    (*uctl).value.integer.value[0] = *((*kctl).private_value as *mut bool_) as c_long;
    0
}

unsafe extern "C" fn catpt_loopback_mute_put(
    kctl: *mut snd_kcontrol,
    uctl: *mut snd_ctl_elem_value,
) -> c_int {
    let component = snd_kcontrol_chip(kctl) as *mut snd_soc_component;
    let cdev = dev_get_drvdata((*component).dev) as *mut catpt_dev;
    let kmute = (*kctl).private_value as *mut bool_;
    let cmute = (*uctl).value.integer.value[0] as bool_;

    if *kmute == cmute {
        return 0;
    }

    mutex_lock(&raw mut (*cdev).stream_mutex);
    let stream_hw_id = catpt_stream_hw_id(cdev, CATPT_PIN_ID_REFERENCE);
    if stream_hw_id != CATPT_PIN_ID_INVALID as u8 {
        let ret = catpt_ipc_mute_loopback(cdev, stream_hw_id, cmute);
        if ret != 0 {
            mutex_unlock(&raw mut (*cdev).stream_mutex);
            return CATPT_IPC_RET(ret);
        }
    }
    mutex_unlock(&raw mut (*cdev).stream_mutex);

    *kmute = cmute;
    1
}

static mut CATPT_LOOPBACK_MUTE: bool_ = false;

static mut MASTER_PLAYBACK_VOLUME_DATA: catpt_control_data =
    catpt_control_data { pin_id: CATPT_PIN_ID_MIXER, volumes: [0; CATPT_CHANNELS_MAX] };
static mut MEDIA0_PLAYBACK_VOLUME_DATA: catpt_control_data =
    catpt_control_data { pin_id: CATPT_PIN_ID_OFFLOAD1, volumes: [0; CATPT_CHANNELS_MAX] };
static mut MEDIA1_PLAYBACK_VOLUME_DATA: catpt_control_data =
    catpt_control_data { pin_id: CATPT_PIN_ID_OFFLOAD2, volumes: [0; CATPT_CHANNELS_MAX] };
static mut MIC_CAPTURE_VOLUME_DATA: catpt_control_data =
    catpt_control_data { pin_id: CATPT_PIN_ID_CAPTURE1, volumes: [0; CATPT_CHANNELS_MAX] };

static COMPONENT_KCONTROLS: [snd_kcontrol_new; 5] = [
    volume_ctl(c"Master Playback Volume".as_ptr(), &raw mut MASTER_PLAYBACK_VOLUME_DATA as c_ulong),
    volume_ctl(c"Media0 Playback Volume".as_ptr(), &raw mut MEDIA0_PLAYBACK_VOLUME_DATA as c_ulong),
    volume_ctl(c"Media1 Playback Volume".as_ptr(), &raw mut MEDIA1_PLAYBACK_VOLUME_DATA as c_ulong),
    volume_ctl(c"Mic Capture Volume".as_ptr(), &raw mut MIC_CAPTURE_VOLUME_DATA as c_ulong),
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_MIXER,
        name: c"Loopback Mute".as_ptr(),
        access: SNDRV_CTL_ELEM_ACCESS_READWRITE,
        info: None,
        get: Some(catpt_loopback_mute_get),
        put: Some(catpt_loopback_mute_put),
        tlv_p: ptr::null(),
        private_value: &raw mut CATPT_LOOPBACK_MUTE as c_ulong,
    },
];

static COMPONENT_WIDGETS: [snd_soc_dapm_widget; 5] = [
    SND_SOC_DAPM_AIF_IN(c"SSP0 CODEC IN".as_ptr(), ptr::null(), 0, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_AIF_OUT(c"SSP0 CODEC OUT".as_ptr(), ptr::null(), 0, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_AIF_IN(c"SSP1 BT IN".as_ptr(), ptr::null(), 0, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_AIF_OUT(c"SSP1 BT OUT".as_ptr(), ptr::null(), 0, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_MIXER(c"Playback VMixer".as_ptr(), SND_SOC_NOPM, 0, 0, ptr::null(), 0),
];

static COMPONENT_ROUTES: [snd_soc_dapm_route; 8] = [
    route("Playback VMixer", "System Playback"),
    route("Playback VMixer", "Offload0 Playback"),
    route("Playback VMixer", "Offload1 Playback"),
    route("SSP0 CODEC OUT", "Playback VMixer"),
    route("Analog Capture", "SSP0 CODEC IN"),
    route("Loopback Capture", "SSP0 CODEC IN"),
    route("SSP1 BT OUT", "Bluetooth Playback"),
    route("Bluetooth Capture", "SSP1 BT IN"),
];

static CATPT_COMP_DRIVER: snd_soc_component_driver = snd_soc_component_driver {
    name: c"catpt-platform".as_ptr(),
    pcm_new: Some(catpt_component_pcm_new),
    open: Some(catpt_component_open),
    pointer: Some(catpt_component_pointer),
    controls: COMPONENT_KCONTROLS.as_ptr(),
    num_controls: COMPONENT_KCONTROLS.len(),
    dapm_widgets: COMPONENT_WIDGETS.as_ptr(),
    num_dapm_widgets: COMPONENT_WIDGETS.len(),
    dapm_routes: COMPONENT_ROUTES.as_ptr(),
    num_dapm_routes: COMPONENT_ROUTES.len(),
};

#[no_mangle]
pub unsafe extern "C" fn catpt_arm_stream_templates(cdev: *mut catpt_dev) -> c_int {
    let mut scratch_size: u32 = 0;

    for i in 0..CATPT_TOPOLOGY.len() {
        let template = CATPT_TOPOLOGY[i];
        (*template).persistent_size = 0;

        for j in 0..(*template).num_entries as usize {
            let entry = &raw mut (*template).entries[j];
            let type_ = &raw mut (*cdev).modules[(*entry).module_id as usize];

            if !(*type_).loaded {
                return -ENOENT;
            }

            (*entry).entry_point = (*type_).entry_point;
            (*template).persistent_size += (*type_).persistent_size;
            if (*type_).scratch_size > scratch_size {
                scratch_size = (*type_).scratch_size;
            }
        }
    }

    if scratch_size != 0 {
        /* allocate single scratch area for all modules */
        let res = catpt_request_region(&raw mut (*cdev).dram, scratch_size);
        if res.is_null() {
            return -EBUSY;
        }
        (*cdev).scratch = res;
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn catpt_register_plat_component(cdev: *mut catpt_dev) -> c_int {
    let component = snd_soc_component_alloc((*cdev).dev);
    if component.is_null() {
        return -ENOMEM;
    }

    snd_soc_component_set_name(component, CATPT_COMP_DRIVER.name);

    snd_soc_register_component(
        component,
        &raw const CATPT_COMP_DRIVER,
        &raw mut DAI_DRIVERS[0],
        DAI_DRIVERS.len(),
    )
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
