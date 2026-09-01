// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
//
// Copyright 2021 NXP
//
// Author: Daniel Baluta <daniel.baluta@nxp.com>

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

type u8 = u8;
type u32 = u32;
type u64 = u64;
type size_t = usize;

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const EBUSY: c_int = 16;
const GFP_KERNEL: c_uint = 0;

const SNDRV_DMA_TYPE_DEV_SG: c_int = 0;
const SNDRV_PCM_FORMAT_S32: c_int = 0;
const SND_COMPRESS_PLAYBACK: c_int = 0;
const SNDRV_PCM_TRIGGER_START: c_int = 0;
const SNDRV_PCM_TRIGGER_STOP: c_int = 1;
const SNDRV_PCM_TRIGGER_PAUSE_PUSH: c_int = 3;
const SNDRV_PCM_TRIGGER_PAUSE_RELEASE: c_int = 4;

const SOF_AUDIO_PCM_DRV_NAME: *const c_char = b"sof-audio-component\0".as_ptr() as *const c_char;
const SOF_IPC_GLB_STREAM_MSG: u32 = 0;
const SOF_IPC_STREAM_PCM_FREE: u32 = 0;
const SOF_IPC_STREAM_PCM_PARAMS: u32 = 0;
const SOF_IPC_STREAM_TRIG_START: u32 = 0;
const SOF_IPC_STREAM_TRIG_STOP: u32 = 0;
const SOF_IPC_STREAM_TRIG_PAUSE: u32 = 0;
const SOF_IPC_STREAM_TRIG_RELEASE: u32 = 0;
const SOF_IPC_BUFFER_INTERLEAVED: u32 = 0;
const SOF_IPC_FRAME_S32_LE: u32 = 0;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct work_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_component {
    pub dev: *mut device,
}

#[repr(C)]
pub struct snd_soc_pcm_runtime {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_dma_device {
    pub type_: c_int,
    pub dev: *mut device,
}

#[repr(C)]
pub struct snd_dma_buffer {
    pub dev: snd_dma_device,
}

#[repr(C)]
pub struct snd_compr_runtime {
    pub dma_buffer_p: *mut snd_dma_buffer,
    pub buffer_size: size_t,
    pub dma_area: *mut u8,
    pub dma_bytes: size_t,
    pub private_data: *mut c_void,
    pub total_bytes_available: u64,
    pub total_bytes_transferred: u64,
}

#[repr(C)]
pub struct snd_compr_stream {
    pub private_data: *mut snd_soc_pcm_runtime,
    pub runtime: *mut snd_compr_runtime,
    pub direction: c_int,
    pub dma_buffer: snd_dma_buffer,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_codec {
    pub ch_out: u32,
    pub sample_rate: u32,
}

#[repr(C)]
pub struct snd_compr_buffer {
    pub fragment_size: u32,
}

#[repr(C)]
pub struct snd_compr_params {
    pub codec: snd_codec,
    pub buffer: snd_compr_buffer,
}

#[repr(C)]
pub struct snd_compr_tstamp64 {
    pub sampling_rate: u32,
    pub copied_total: u64,
    pub pcm_io_frames: u64,
}

#[repr(C)]
pub struct snd_sof_dev {
    pub ipc: *mut sof_ipc,
    pub fw_ready: sof_ipc_fw_ready,
    pub dev: *mut device,
}

#[repr(C)]
pub struct sof_ipc {
    pub max_payload_size: size_t,
}

#[repr(C)]
pub struct sof_ipc_fw_ready {
    pub version: sof_ipc_fw_version,
}

#[repr(C)]
pub struct sof_ipc_fw_version {
    pub abi_version: u32,
}

#[repr(C)]
pub struct sof_ipc_hdr {
    pub size: u32,
    pub cmd: u32,
}

#[repr(C)]
pub struct sof_ipc_stream {
    pub hdr: sof_ipc_hdr,
    pub comp_id: u32,
}

#[repr(C)]
pub struct sof_ipc_host_buffer {
    pub pages: u32,
    pub phy_addr: u64,
    pub size: size_t,
}

#[repr(C)]
pub struct sof_ipc_pcm_params_hdr {
    pub size: u32,
}

#[repr(C)]
pub struct sof_ipc_pcm_params_data {
    pub hdr: sof_ipc_pcm_params_hdr,
    pub buffer: sof_ipc_host_buffer,
    pub direction: c_int,
    pub channels: u32,
    pub rate: u32,
    pub buffer_fmt: u32,
    pub frame_fmt: u32,
    pub sample_container_bytes: u32,
    pub host_period_bytes: u32,
    pub ext_data_length: size_t,
    pub ext_data: [u8; 0],
}

#[repr(C)]
pub struct sof_ipc_pcm_params {
    pub hdr: sof_ipc_hdr,
    pub comp_id: u32,
    pub params: sof_ipc_pcm_params_data,
}

#[repr(C)]
pub struct sof_ipc_pcm_params_reply {
    pub posn_offset: u32,
}

#[repr(C)]
pub struct sof_page_table {
    pub area: *mut c_void,
    pub addr: u64,
}

#[repr(C)]
pub struct sof_ipc_stream_posn {
    pub host_posn: u64,
    pub dai_posn: u64,
}

#[repr(C)]
pub struct snd_sof_pcm_stream {
    pub cstream: *mut snd_compr_stream,
    pub posn: sof_ipc_stream_posn,
    pub period_elapsed_work: work_struct,
    pub page_table: sof_page_table,
    pub comp_id: u32,
}

#[repr(C)]
pub struct snd_sof_pcm_id {
    pub pcm_id: u32,
}

#[repr(C)]
pub struct snd_sof_pcm {
    pub stream: [snd_sof_pcm_stream; 2],
    pub prepared: [bool; 2],
    pub pcm: snd_sof_pcm_id,
}

#[repr(C)]
pub struct sof_compr_stream {
    pub copied_total: u64,
    pub sampling_rate: u32,
    pub channels: u32,
    pub sample_container_bytes: u32,
    pub codec_params: snd_codec,
}

#[repr(C)]
pub struct snd_compress_ops {
    pub open: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_compr_stream) -> c_int>,
    pub free: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_compr_stream) -> c_int>,
    pub set_params: Option<
        unsafe extern "C" fn(*mut snd_soc_component, *mut snd_compr_stream, *mut snd_compr_params) -> c_int,
    >,
    pub get_params: Option<
        unsafe extern "C" fn(*mut snd_soc_component, *mut snd_compr_stream, *mut snd_codec) -> c_int,
    >,
    pub trigger: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_compr_stream, c_int) -> c_int>,
    pub pointer: Option<
        unsafe extern "C" fn(*mut snd_soc_component, *mut snd_compr_stream, *mut snd_compr_tstamp64) -> c_int,
    >,
    pub copy: Option<
        unsafe extern "C" fn(*mut snd_soc_component, *mut snd_compr_stream, *mut c_char, size_t) -> c_int,
    >,
}

unsafe extern "C" {
    fn snd_compr_fragment_elapsed(cstream: *mut snd_compr_stream);
    fn snd_soc_rtdcom_lookup(rtd: *mut snd_soc_pcm_runtime, name: *const c_char) -> *mut snd_soc_component;
    fn snd_sof_find_spcm_dai(
        component: *mut snd_soc_component,
        rtd: *mut snd_soc_pcm_runtime,
    ) -> *mut snd_sof_pcm;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn schedule_work(work: *mut work_struct) -> bool;
    fn snd_sof_create_page_table(
        dev: *mut device,
        dmab: *mut snd_dma_buffer,
        page_table: *mut c_void,
        size: size_t,
    ) -> c_int;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut snd_sof_dev;
    fn sof_ipc_tx_message_no_reply(ipc: *mut sof_ipc, msg: *const c_void, msg_bytes: size_t) -> c_int;
    fn cancel_work_sync(work: *mut work_struct) -> bool;
    fn snd_sof_boot_dsp_firmware(sdev: *mut snd_sof_dev) -> c_int;
    fn kzalloc(size: size_t, flags: c_uint) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn snd_compr_malloc_pages(cstream: *mut snd_compr_stream, size: size_t) -> c_int;
    fn sof_ipc_tx_message(
        ipc: *mut sof_ipc,
        msg: *const c_void,
        msg_bytes: size_t,
        reply: *mut c_void,
        reply_bytes: size_t,
    ) -> c_int;
    fn snd_sof_set_stream_data_offset(
        sdev: *mut snd_sof_dev,
        stream: *mut snd_sof_pcm_stream,
        offset: u32,
    ) -> c_int;
    fn le32_to_cpu(x: u32) -> u32;
    fn snd_pcm_format_physical_width(format: c_int) -> c_int;
    fn copy_from_user(to: *mut c_void, from: *const c_char, n: size_t) -> c_int;
    fn copy_to_user(to: *mut c_char, from: *const c_void, n: size_t) -> c_int;
}

fn SOF_ABI_VER(major: u32, minor: u32, patch: u32) -> u32 {
    (major << 24) | (minor << 12) | patch
}

fn SOF_ABI_VERSION_MAJOR(version: u32) -> u32 {
    version >> 24
}

fn SOF_ABI_VERSION_MINOR(version: u32) -> u32 {
    (version >> 12) & 0xfff
}

fn SOF_ABI_VERSION_PATCH(version: u32) -> u32 {
    version & 0xfff
}

fn PFN_UP(x: size_t) -> u32 {
    ((x + 4095) >> 12) as u32
}

unsafe fn div64_u64_rem(dividend: u64, divisor: u64, remainder: *mut u64) -> u64 {
    *remainder = dividend % divisor;
    dividend / divisor
}

unsafe fn div_u64_rem(dividend: u64, divisor: size_t, remainder: *mut c_uint) -> u64 {
    *remainder = (dividend % divisor as u64) as c_uint;
    dividend / divisor as u64
}

fn div_u64(dividend: u64, divisor: u32) -> u64 {
    dividend / divisor as u64
}

unsafe fn INIT_WORK(work: *mut work_struct, func: unsafe extern "C" fn(*mut work_struct)) {
    let _ = work;
    let _ = func;
}

unsafe fn sof_set_transferred_bytes(sstream: *mut sof_compr_stream, host_pos: u64, buffer_size: u64) {
    let mut prev_pos: u64 = 0;
    let copied: c_uint;

    div64_u64_rem((*sstream).copied_total, buffer_size, &mut prev_pos);

    if host_pos < prev_pos {
        copied = ((buffer_size - prev_pos) + host_pos) as c_uint;
    } else {
        copied = (host_pos - prev_pos) as c_uint;
    }

    (*sstream).copied_total = (*sstream).copied_total.wrapping_add(copied as u64);
}

unsafe extern "C" fn snd_sof_compr_fragment_elapsed_work(work: *mut work_struct) {
    /*
     * C used container_of(work, struct snd_sof_pcm_stream, period_elapsed_work).
     * Keep the dependency explicit; the containing type layout is supplied by external SOF code.
     */
    let sps = (work as *mut u8).offset(-(core::mem::offset_of!(snd_sof_pcm_stream, period_elapsed_work) as isize))
        as *mut snd_sof_pcm_stream;

    snd_compr_fragment_elapsed((*sps).cstream);
}

#[no_mangle]
pub unsafe extern "C" fn snd_sof_compr_init_elapsed_work(work: *mut work_struct) {
    INIT_WORK(work, snd_sof_compr_fragment_elapsed_work);
}

/*
 * sof compr fragment elapse, this could be called in irq thread context
 */
#[no_mangle]
pub unsafe extern "C" fn snd_sof_compr_fragment_elapsed(cstream: *mut snd_compr_stream) {
    let rtd: *mut snd_soc_pcm_runtime;
    let crtd: *mut snd_compr_runtime;
    let component: *mut snd_soc_component;
    let sstream: *mut sof_compr_stream;
    let spcm: *mut snd_sof_pcm;

    if cstream.is_null() {
        return;
    }

    rtd = (*cstream).private_data;
    crtd = (*cstream).runtime;
    sstream = (*crtd).private_data as *mut sof_compr_stream;
    component = snd_soc_rtdcom_lookup(rtd, SOF_AUDIO_PCM_DRV_NAME);

    spcm = snd_sof_find_spcm_dai(component, rtd);
    if spcm.is_null() {
        dev_err((*component).dev, b"fragment elapsed called for unknown stream!\n\0".as_ptr() as *const c_char);
        return;
    }

    sof_set_transferred_bytes(
        sstream,
        (*spcm).stream[(*cstream).direction as usize].posn.host_posn,
        (*crtd).buffer_size as u64,
    );

    /* use the same workqueue-based solution as for PCM, cf. snd_sof_pcm_elapsed */
    schedule_work(&mut (*spcm).stream[(*cstream).direction as usize].period_elapsed_work);
}

unsafe extern "C" fn create_page_table(
    component: *mut snd_soc_component,
    cstream: *mut snd_compr_stream,
    _dma_area: *mut u8,
    size: size_t,
) -> c_int {
    let dmab: *mut snd_dma_buffer = (*(*cstream).runtime).dma_buffer_p;
    let rtd: *mut snd_soc_pcm_runtime = (*cstream).private_data;
    let dir: c_int = (*cstream).direction;
    let spcm: *mut snd_sof_pcm;

    spcm = snd_sof_find_spcm_dai(component, rtd);
    if spcm.is_null() {
        return -EINVAL;
    }

    snd_sof_create_page_table(
        (*component).dev,
        dmab,
        (*spcm).stream[dir as usize].page_table.area,
        size,
    )
}

unsafe extern "C" fn sof_compr_open(
    component: *mut snd_soc_component,
    cstream: *mut snd_compr_stream,
) -> c_int {
    let rtd: *mut snd_soc_pcm_runtime = (*cstream).private_data;
    let crtd: *mut snd_compr_runtime = (*cstream).runtime;
    let sstream: *mut sof_compr_stream;
    let spcm: *mut snd_sof_pcm;
    let dir: c_int;

    sstream = kzalloc(size_of::<sof_compr_stream>(), GFP_KERNEL) as *mut sof_compr_stream;
    if sstream.is_null() {
        return -ENOMEM;
    }

    spcm = snd_sof_find_spcm_dai(component, rtd);
    if spcm.is_null() {
        kfree(sstream as *mut c_void);
        return -EINVAL;
    }

    dir = (*cstream).direction;

    if !(*spcm).stream[dir as usize].cstream.is_null() {
        kfree(sstream as *mut c_void);
        return -EBUSY;
    }

    (*spcm).stream[dir as usize].cstream = cstream;
    (*spcm).stream[dir as usize].posn.host_posn = 0;
    (*spcm).stream[dir as usize].posn.dai_posn = 0;
    (*spcm).prepared[dir as usize] = false;

    (*crtd).private_data = sstream as *mut c_void;

    0
}

unsafe extern "C" fn sof_compr_free(
    component: *mut snd_soc_component,
    cstream: *mut snd_compr_stream,
) -> c_int {
    let sdev: *mut snd_sof_dev = snd_soc_component_get_drvdata(component);
    let sstream: *mut sof_compr_stream = (*(*cstream).runtime).private_data as *mut sof_compr_stream;
    let rtd: *mut snd_soc_pcm_runtime = (*cstream).private_data;
    let mut stream: sof_ipc_stream = core::mem::zeroed();
    let spcm: *mut snd_sof_pcm;
    let mut ret: c_int = 0;

    spcm = snd_sof_find_spcm_dai(component, rtd);
    if spcm.is_null() {
        return -EINVAL;
    }

    stream.hdr.size = size_of::<sof_ipc_stream>() as u32;
    stream.hdr.cmd = SOF_IPC_GLB_STREAM_MSG | SOF_IPC_STREAM_PCM_FREE;
    stream.comp_id = (*spcm).stream[(*cstream).direction as usize].comp_id;

    if (*spcm).prepared[(*cstream).direction as usize] {
        ret = sof_ipc_tx_message_no_reply((*sdev).ipc, &stream as *const _ as *const c_void, size_of::<sof_ipc_stream>());
        if ret == 0 {
            (*spcm).prepared[(*cstream).direction as usize] = false;
        }
    }

    cancel_work_sync(&mut (*spcm).stream[(*cstream).direction as usize].period_elapsed_work);
    (*spcm).stream[(*cstream).direction as usize].cstream = ptr::null_mut();
    kfree(sstream as *mut c_void);

    ret
}

unsafe extern "C" fn sof_compr_set_params(
    component: *mut snd_soc_component,
    cstream: *mut snd_compr_stream,
    params: *mut snd_compr_params,
) -> c_int {
    let sdev: *mut snd_sof_dev = snd_soc_component_get_drvdata(component);
    let rtd: *mut snd_soc_pcm_runtime = (*cstream).private_data;
    let crtd: *mut snd_compr_runtime = (*cstream).runtime;
    let mut ipc_params_reply: sof_ipc_pcm_params_reply = core::mem::zeroed();
    let ready: *mut sof_ipc_fw_ready = &mut (*sdev).fw_ready;
    let v: *mut sof_ipc_fw_version = &mut (*ready).version;
    let sstream: *mut sof_compr_stream;
    let mut pcm: *mut sof_ipc_pcm_params;
    let spcm: *mut snd_sof_pcm;
    let ext_data_size: size_t;
    let mut ret: c_int;

    if (*v).abi_version < SOF_ABI_VER(3, 22, 0) {
        dev_err(
            (*component).dev,
            b"Compress params not supported with FW ABI version %d:%d:%d\n\0".as_ptr() as *const c_char,
            SOF_ABI_VERSION_MAJOR((*v).abi_version),
            SOF_ABI_VERSION_MINOR((*v).abi_version),
            SOF_ABI_VERSION_PATCH((*v).abi_version),
        );
        return -EINVAL;
    }

    sstream = (*crtd).private_data as *mut sof_compr_stream;

    spcm = snd_sof_find_spcm_dai(component, rtd);

    if spcm.is_null() {
        return -EINVAL;
    }

    ext_data_size = size_of::<snd_codec>();

    if size_of::<sof_ipc_pcm_params>() + ext_data_size > (*(*sdev).ipc).max_payload_size {
        return -EINVAL;
    }

    /*
     * Make sure that the DSP is booted up, which might not be the
     * case if the on-demand DSP boot is used
     */
    ret = snd_sof_boot_dsp_firmware(sdev);
    if ret != 0 {
        return ret;
    }

    pcm = kzalloc(size_of::<sof_ipc_pcm_params>() + ext_data_size, GFP_KERNEL) as *mut sof_ipc_pcm_params;
    if pcm.is_null() {
        return -ENOMEM;
    }

    (*cstream).dma_buffer.dev.type_ = SNDRV_DMA_TYPE_DEV_SG;
    (*cstream).dma_buffer.dev.dev = (*sdev).dev;
    ret = snd_compr_malloc_pages(cstream, (*crtd).buffer_size);
    if ret < 0 {
        kfree(pcm as *mut c_void);
        return ret;
    }

    ret = create_page_table(component, cstream, (*crtd).dma_area, (*crtd).dma_bytes);
    if ret < 0 {
        kfree(pcm as *mut c_void);
        return ret;
    }

    (*pcm).params.buffer.pages = PFN_UP((*crtd).dma_bytes);
    (*pcm).hdr.size = (size_of::<sof_ipc_pcm_params>() + ext_data_size) as u32;
    (*pcm).hdr.cmd = SOF_IPC_GLB_STREAM_MSG | SOF_IPC_STREAM_PCM_PARAMS;

    (*pcm).comp_id = (*spcm).stream[(*cstream).direction as usize].comp_id;
    (*pcm).params.hdr.size = (size_of::<sof_ipc_pcm_params_data>() + ext_data_size) as u32;
    (*pcm).params.buffer.phy_addr = (*spcm).stream[(*cstream).direction as usize].page_table.addr;
    (*pcm).params.buffer.size = (*crtd).dma_bytes;
    (*pcm).params.direction = (*cstream).direction;
    (*pcm).params.channels = (*params).codec.ch_out;
    (*pcm).params.rate = (*params).codec.sample_rate;
    (*pcm).params.buffer_fmt = SOF_IPC_BUFFER_INTERLEAVED;
    (*pcm).params.frame_fmt = SOF_IPC_FRAME_S32_LE;
    (*pcm).params.sample_container_bytes = (snd_pcm_format_physical_width(SNDRV_PCM_FORMAT_S32) >> 3) as u32;
    (*pcm).params.host_period_bytes = (*params).buffer.fragment_size;
    (*pcm).params.ext_data_length = ext_data_size;

    ptr::copy_nonoverlapping(
        &(*params).codec as *const snd_codec as *const u8,
        (*pcm).params.ext_data.as_mut_ptr(),
        ext_data_size,
    );

    ret = sof_ipc_tx_message(
        (*sdev).ipc,
        pcm as *const c_void,
        size_of::<sof_ipc_pcm_params>() + ext_data_size,
        &mut ipc_params_reply as *mut _ as *mut c_void,
        size_of::<sof_ipc_pcm_params_reply>(),
    );
    if ret < 0 {
        dev_err((*component).dev, b"error ipc failed\n\0".as_ptr() as *const c_char);
        kfree(pcm as *mut c_void);
        return ret;
    }

    ret = snd_sof_set_stream_data_offset(
        sdev,
        &mut (*spcm).stream[(*cstream).direction as usize],
        ipc_params_reply.posn_offset,
    );
    if ret < 0 {
        dev_err(
            (*component).dev,
            b"Invalid stream data offset for Compr %u\n\0".as_ptr() as *const c_char,
            le32_to_cpu((*spcm).pcm.pcm_id),
        );
        kfree(pcm as *mut c_void);
        return ret;
    }

    (*sstream).sampling_rate = (*params).codec.sample_rate;
    (*sstream).channels = (*params).codec.ch_out;
    (*sstream).sample_container_bytes = (*pcm).params.sample_container_bytes;
    (*sstream).codec_params = (*params).codec;

    (*spcm).prepared[(*cstream).direction as usize] = true;

    kfree(pcm as *mut c_void);

    ret
}

unsafe extern "C" fn sof_compr_get_params(
    _component: *mut snd_soc_component,
    cstream: *mut snd_compr_stream,
    params: *mut snd_codec,
) -> c_int {
    let sstream: *mut sof_compr_stream = (*(*cstream).runtime).private_data as *mut sof_compr_stream;

    *params = (*sstream).codec_params;

    0
}

unsafe extern "C" fn sof_compr_trigger(
    component: *mut snd_soc_component,
    cstream: *mut snd_compr_stream,
    cmd: c_int,
) -> c_int {
    let sdev: *mut snd_sof_dev = snd_soc_component_get_drvdata(component);
    let rtd: *mut snd_soc_pcm_runtime = (*cstream).private_data;
    let mut stream: sof_ipc_stream = core::mem::zeroed();
    let spcm: *mut snd_sof_pcm;

    spcm = snd_sof_find_spcm_dai(component, rtd);
    if spcm.is_null() {
        return -EINVAL;
    }

    stream.hdr.size = size_of::<sof_ipc_stream>() as u32;
    stream.hdr.cmd = SOF_IPC_GLB_STREAM_MSG;
    stream.comp_id = (*spcm).stream[(*cstream).direction as usize].comp_id;

    match cmd {
        SNDRV_PCM_TRIGGER_START => {
            stream.hdr.cmd |= SOF_IPC_STREAM_TRIG_START;
        }
        SNDRV_PCM_TRIGGER_STOP => {
            stream.hdr.cmd |= SOF_IPC_STREAM_TRIG_STOP;
        }
        SNDRV_PCM_TRIGGER_PAUSE_PUSH => {
            stream.hdr.cmd |= SOF_IPC_STREAM_TRIG_PAUSE;
        }
        SNDRV_PCM_TRIGGER_PAUSE_RELEASE => {
            stream.hdr.cmd |= SOF_IPC_STREAM_TRIG_RELEASE;
        }
        _ => {
            dev_err((*component).dev, b"error: unhandled trigger cmd %d\n\0".as_ptr() as *const c_char, cmd);
        }
    }

    sof_ipc_tx_message_no_reply((*sdev).ipc, &stream as *const _ as *const c_void, size_of::<sof_ipc_stream>())
}

unsafe extern "C" fn sof_compr_copy_playback(
    rtd: *mut snd_compr_runtime,
    buf: *mut c_char,
    count: size_t,
) -> c_int {
    let ptr_: *mut c_void;
    let mut offset: c_uint = 0;
    let n: c_uint;
    let mut ret: c_int;

    div_u64_rem((*rtd).total_bytes_available, (*rtd).buffer_size, &mut offset);
    ptr_ = (*rtd).dma_area.add(offset as usize) as *mut c_void;
    n = ((*rtd).buffer_size as c_uint).wrapping_sub(offset);

    if count < n as size_t {
        ret = copy_from_user(ptr_, buf, count);
    } else {
        ret = copy_from_user(ptr_, buf, n as size_t);
        ret += copy_from_user((*rtd).dma_area as *mut c_void, buf.add(n as usize), count - n as size_t);
    }

    (count as c_int) - ret
}

unsafe extern "C" fn sof_compr_copy_capture(
    rtd: *mut snd_compr_runtime,
    buf: *mut c_char,
    count: size_t,
) -> c_int {
    let ptr_: *mut c_void;
    let mut offset: c_uint = 0;
    let n: c_uint;
    let mut ret: c_int;

    div_u64_rem((*rtd).total_bytes_transferred, (*rtd).buffer_size, &mut offset);
    ptr_ = (*rtd).dma_area.add(offset as usize) as *mut c_void;
    n = ((*rtd).buffer_size as c_uint).wrapping_sub(offset);

    if count < n as size_t {
        ret = copy_to_user(buf, ptr_, count);
    } else {
        ret = copy_to_user(buf, ptr_, n as size_t);
        ret += copy_to_user(buf.add(n as usize), (*rtd).dma_area as *const c_void, count - n as size_t);
    }

    (count as c_int) - ret
}

unsafe extern "C" fn sof_compr_copy(
    _component: *mut snd_soc_component,
    cstream: *mut snd_compr_stream,
    buf: *mut c_char,
    mut count: size_t,
) -> c_int {
    let rtd: *mut snd_compr_runtime = (*cstream).runtime;

    if count > (*rtd).buffer_size {
        count = (*rtd).buffer_size;
    }

    if (*cstream).direction == SND_COMPRESS_PLAYBACK {
        sof_compr_copy_playback(rtd, buf, count)
    } else {
        sof_compr_copy_capture(rtd, buf, count)
    }
}

unsafe extern "C" fn sof_compr_pointer(
    component: *mut snd_soc_component,
    cstream: *mut snd_compr_stream,
    tstamp: *mut snd_compr_tstamp64,
) -> c_int {
    let spcm: *mut snd_sof_pcm;
    let rtd: *mut snd_soc_pcm_runtime = (*cstream).private_data;
    let sstream: *mut sof_compr_stream = (*(*cstream).runtime).private_data as *mut sof_compr_stream;

    spcm = snd_sof_find_spcm_dai(component, rtd);
    if spcm.is_null() {
        return -EINVAL;
    }

    if (*sstream).channels == 0 || (*sstream).sample_container_bytes == 0 {
        return -EBUSY;
    }

    (*tstamp).sampling_rate = (*sstream).sampling_rate;
    (*tstamp).copied_total = (*sstream).copied_total;
    (*tstamp).pcm_io_frames = div_u64(
        (*spcm).stream[(*cstream).direction as usize].posn.dai_posn,
        (*sstream).channels * (*sstream).sample_container_bytes,
    );

    0
}

#[no_mangle]
pub static mut sof_compressed_ops: snd_compress_ops = snd_compress_ops {
    open: Some(sof_compr_open),
    free: Some(sof_compr_free),
    set_params: Some(sof_compr_set_params),
    get_params: Some(sof_compr_get_params),
    trigger: Some(sof_compr_trigger),
    pointer: Some(sof_compr_pointer),
    copy: Some(sof_compr_copy),
};

/* EXPORT_SYMBOL(sof_compressed_ops); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
