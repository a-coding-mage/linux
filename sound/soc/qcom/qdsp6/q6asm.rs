// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2011-2017, The Linux Foundation. All rights reserved.
// Copyright (c) 2018, Linaro Limited

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::{offset_of, size_of};
use core::ptr;

type u8 = u8;
type u16 = u16;
type u32 = u32;
type uint8_t = u8;
type uint16_t = u16;
type uint32_t = u32;
type int32_t = i32;
type size_t = usize;
type phys_addr_t = usize;
type bool_ = bool;

const ASM_STREAM_CMD_CLOSE: u32 = 0x00010BCD;
const ASM_STREAM_CMD_FLUSH: u32 = 0x00010BCE;
const ASM_SESSION_CMD_PAUSE: u32 = 0x00010BD3;
const ASM_DATA_CMD_EOS: u32 = 0x00010BDB;
const ASM_DATA_EVENT_RENDERED_EOS: u32 = 0x00010C1C;
const ASM_NULL_POPP_TOPOLOGY: u32 = 0x00010C68;
const ASM_STREAM_CMD_FLUSH_READBUFS: u32 = 0x00010C09;
const ASM_STREAM_CMD_SET_ENCDEC_PARAM: u32 = 0x00010C10;
const ASM_STREAM_POSTPROC_TOPO_ID_NONE: u32 = 0x00010C68;
const ASM_CMD_SHARED_MEM_MAP_REGIONS: u32 = 0x00010D92;
const ASM_CMDRSP_SHARED_MEM_MAP_REGIONS: u32 = 0x00010D93;
const ASM_CMD_SHARED_MEM_UNMAP_REGIONS: u32 = 0x00010D94;
const ASM_DATA_CMD_MEDIA_FMT_UPDATE_V2: u32 = 0x00010D98;
const ASM_DATA_EVENT_WRITE_DONE_V2: u32 = 0x00010D99;
const ASM_PARAM_ID_ENCDEC_ENC_CFG_BLK_V2: u32 = 0x00010DA3;
const ASM_SESSION_CMD_RUN_V2: u32 = 0x00010DAA;
const ASM_MEDIA_FMT_MULTI_CHANNEL_PCM_V2: u32 = 0x00010DA5;
const ASM_MEDIA_FMT_MP3: u32 = 0x00010BE9;
const ASM_MEDIA_FMT_FLAC: u32 = 0x00010C16;
const ASM_MEDIA_FMT_WMA_V9: u32 = 0x00010DA8;
const ASM_MEDIA_FMT_WMA_V10: u32 = 0x00010DA7;
const ASM_DATA_CMD_WRITE_V2: u32 = 0x00010DAB;
const ASM_DATA_CMD_READ_V2: u32 = 0x00010DAC;
const ASM_SESSION_CMD_SUSPEND: u32 = 0x00010DEC;
const ASM_STREAM_CMD_OPEN_WRITE_V3: u32 = 0x00010DB3;
const ASM_STREAM_CMD_OPEN_READ_V3: u32 = 0x00010DB4;
const ASM_DATA_EVENT_READ_DONE_V2: u32 = 0x00010D9A;
const ASM_STREAM_CMD_OPEN_READWRITE_V2: u32 = 0x00010D8D;
const ASM_MEDIA_FMT_ALAC: u32 = 0x00012f31;
const ASM_MEDIA_FMT_APE: u32 = 0x00012f32;
const ASM_DATA_CMD_REMOVE_INITIAL_SILENCE: u32 = 0x00010D67;
const ASM_DATA_CMD_REMOVE_TRAILING_SILENCE: u32 = 0x00010D68;

const ASM_LEGACY_STREAM_SESSION: u32 = 0;
/* Bit shift for the stream_perf_mode subfield. */
const ASM_SHIFT_STREAM_PERF_MODE_FLAG_IN_OPEN_READ: u32 = 29;
const ASM_END_POINT_DEVICE_MATRIX: u32 = 0;
const ASM_DEFAULT_APP_TYPE: u32 = 0;
const ASM_SYNC_IO_MODE: u32 = 0x0001;
const ASM_ASYNC_IO_MODE: u32 = 0x0002;
const ASM_TUN_READ_IO_MODE: u32 = 0x0004; /* tunnel read write mode */
const ASM_TUN_WRITE_IO_MODE: u32 = 0x0008; /* tunnel read write mode */
const ASM_SHIFT_GAPLESS_MODE_FLAG: u32 = 31;
const ADSP_MEMORY_MAP_SHMEM8_4K_POOL: u16 = 3;

const ENOMEM: c_int = 12;
const EINVAL: c_int = 22;
const ETIMEDOUT: c_int = 110;
const GFP_KERNEL: c_uint = 0;
const GFP_ATOMIC: c_uint = 0;
const HZ: c_int = 100;
const APR_HDR_SIZE: usize = size_of::<apr_hdr>();
const APR_SEQ_CMD_HDR_FIELD: u32 = 0;
const APR_BASIC_RSP_RESULT: u32 = 0;
const MAX_SESSIONS: usize = 8;
const PCM_MAX_NUM_CHANNEL: usize = 8;
const SNDRV_PCM_STREAM_PLAYBACK: usize = 0;
const SNDRV_PCM_STREAM_CAPTURE: usize = 1;
const ASM_WRITE_TOKEN_MASK: c_int = 0xff;
const ASM_WRITE_TOKEN_LEN_SHIFT: u32 = 16;
const CMD_PAUSE: c_int = 0;
const CMD_SUSPEND: c_int = 1;
const CMD_FLUSH: c_int = 2;
const CMD_OUT_FLUSH: c_int = 3;
const CMD_EOS: c_int = 4;
const CMD_CLOSE: c_int = 5;
const FORMAT_LINEAR_PCM: u32 = 0;
const SND_AUDIOCODEC_MP3: u32 = 1;
const SND_AUDIOCODEC_FLAC: u32 = 2;
const SND_AUDIOCODEC_WMA: u32 = 3;
const SND_AUDIOCODEC_ALAC: u32 = 4;
const SND_AUDIOCODEC_APE: u32 = 5;
const SND_AUDIOPROFILE_WMA9: u32 = 1;
const SND_AUDIOPROFILE_WMA10: u32 = 2;
const SND_AUDIOPROFILE_WMA9_PRO: u32 = 3;
const SND_AUDIOPROFILE_WMA9_LOSSLESS: u32 = 4;
const SND_AUDIOPROFILE_WMA10_LOSSLESS: u32 = 5;
const ASM_CLIENT_EVENT_CMD_PAUSE_DONE: u32 = 0;
const ASM_CLIENT_EVENT_CMD_SUSPEND_DONE: u32 = 1;
const ASM_CLIENT_EVENT_CMD_FLUSH_DONE: u32 = 2;
const ASM_CLIENT_EVENT_CMD_RUN_DONE: u32 = 3;
const ASM_CLIENT_EVENT_CMD_CLOSE_DONE: u32 = 4;
const ASM_CLIENT_EVENT_CMD_OUT_FLUSH_DONE: u32 = 5;
const ASM_CLIENT_EVENT_DATA_WRITE_DONE: u32 = 6;
const ASM_CLIENT_EVENT_DATA_READ_DONE: u32 = 7;
const ASM_CLIENT_EVENT_CMD_EOS_DONE: u32 = 8;

#[repr(C, packed)]
struct avs_cmd_shared_mem_map_regions {
    mem_pool_id: u16,
    num_regions: u16,
    property_flag: u32,
}

#[repr(C, packed)]
struct avs_shared_map_region_payload {
    shm_addr_lsw: u32,
    shm_addr_msw: u32,
    mem_size_bytes: u32,
}

#[repr(C, packed)]
struct avs_cmd_shared_mem_unmap_regions {
    mem_map_handle: u32,
}

#[repr(C, packed)]
struct asm_data_cmd_media_fmt_update_v2 {
    fmt_blk_size: u32,
}

#[repr(C, packed)]
struct asm_multi_channel_pcm_fmt_blk_v2 {
    fmt_blk: asm_data_cmd_media_fmt_update_v2,
    num_channels: u16,
    bits_per_sample: u16,
    sample_rate: u32,
    is_signed: u16,
    reserved: u16,
    channel_mapping: [u8; PCM_MAX_NUM_CHANNEL],
}

#[repr(C, packed)]
struct asm_flac_fmt_blk_v2 {
    fmt_blk: asm_data_cmd_media_fmt_update_v2,
    is_stream_info_present: u16,
    num_channels: u16,
    min_blk_size: u16,
    max_blk_size: u16,
    md5_sum: [u16; 8],
    sample_rate: u32,
    min_frame_size: u32,
    max_frame_size: u32,
    sample_size: u16,
    reserved: u16,
}

#[repr(C, packed)]
struct asm_wmastdv9_fmt_blk_v2 {
    fmt_blk: asm_data_cmd_media_fmt_update_v2,
    fmtag: u16,
    num_channels: u16,
    sample_rate: u32,
    bytes_per_sec: u32,
    blk_align: u16,
    bits_per_sample: u16,
    channel_mask: u32,
    enc_options: u16,
    reserved: u16,
}

#[repr(C, packed)]
struct asm_wmaprov10_fmt_blk_v2 {
    fmt_blk: asm_data_cmd_media_fmt_update_v2,
    fmtag: u16,
    num_channels: u16,
    sample_rate: u32,
    bytes_per_sec: u32,
    blk_align: u16,
    bits_per_sample: u16,
    channel_mask: u32,
    enc_options: u16,
    advanced_enc_options1: u16,
    advanced_enc_options2: u32,
}

#[repr(C, packed)]
struct asm_alac_fmt_blk_v2 {
    fmt_blk: asm_data_cmd_media_fmt_update_v2,
    frame_length: u32,
    compatible_version: u8,
    bit_depth: u8,
    pb: u8,
    mb: u8,
    kb: u8,
    num_channels: u8,
    max_run: u16,
    max_frame_bytes: u32,
    avg_bit_rate: u32,
    sample_rate: u32,
    channel_layout_tag: u32,
}

#[repr(C, packed)]
struct asm_ape_fmt_blk_v2 {
    fmt_blk: asm_data_cmd_media_fmt_update_v2,
    compatible_version: u16,
    compression_level: u16,
    format_flags: u32,
    blocks_per_frame: u32,
    final_frame_blocks: u32,
    total_frames: u32,
    bits_per_sample: u16,
    num_channels: u16,
    sample_rate: u32,
    seek_table_present: u32,
}

#[repr(C, packed)]
struct asm_stream_cmd_set_encdec_param {
    param_id: u32,
    param_size: u32,
}

#[repr(C, packed)]
struct asm_enc_cfg_blk_param_v2 {
    frames_per_buf: u32,
    enc_cfg_blk_size: u32,
}

#[repr(C, packed)]
struct asm_multi_channel_pcm_enc_cfg_v2 {
    encdec: asm_stream_cmd_set_encdec_param,
    encblk: asm_enc_cfg_blk_param_v2,
    num_channels: uint16_t,
    bits_per_sample: uint16_t,
    sample_rate: uint32_t,
    is_signed: uint16_t,
    reserved: uint16_t,
    channel_mapping: [uint8_t; 8],
}

#[repr(C, packed)]
struct asm_data_cmd_read_v2 {
    buf_addr_lsw: u32,
    buf_addr_msw: u32,
    mem_map_handle: u32,
    buf_size: u32,
    seq_id: u32,
}

#[repr(C)]
struct asm_data_cmd_read_v2_done {
    status: u32,
    buf_addr_lsw: u32,
    buf_addr_msw: u32,
}

#[repr(C, packed)]
struct asm_stream_cmd_open_read_v3 {
    mode_flags: u32,
    src_endpointype: u32,
    preprocopo_id: u32,
    enc_cfg_id: u32,
    bits_per_sample: u16,
    reserved: u16,
}

#[repr(C, packed)]
struct asm_data_cmd_write_v2 {
    buf_addr_lsw: u32,
    buf_addr_msw: u32,
    mem_map_handle: u32,
    buf_size: u32,
    seq_id: u32,
    timestamp_lsw: u32,
    timestamp_msw: u32,
    flags: u32,
}

#[repr(C, packed)]
struct asm_stream_cmd_open_write_v3 {
    mode_flags: uint32_t,
    sink_endpointype: uint16_t,
    bits_per_sample: uint16_t,
    postprocopo_id: uint32_t,
    dec_fmt_id: uint32_t,
}

#[repr(C, packed)]
struct asm_session_cmd_run_v2 {
    flags: u32,
    time_lsw: u32,
    time_msw: u32,
}

#[repr(C)]
struct audio_buffer {
    phys: phys_addr_t,
    size: uint32_t, /* size of buffer */
}

#[repr(C)]
struct audio_port_data {
    buf: *mut audio_buffer,
    num_periods: uint32_t,
    dsp_buf: uint32_t,
    mem_map_handle: uint32_t,
    hw_ptr: atomic_t,
}

#[repr(C)]
struct q6asm {
    adev: *mut apr_device,
    dev: *mut device,
    ainfo: q6core_svc_api_info,
    mem_wait: wait_queue_head_t,
    slock: spinlock_t,
    session: [*mut audio_client; MAX_SESSIONS + 1],
}

#[repr(C)]
struct audio_client {
    session: c_int,
    cb: q6asm_cb,
    priv_: *mut c_void,
    io_mode: uint32_t,
    adev: *mut apr_device,
    cmd_lock: mutex,
    lock: spinlock_t,
    refcount: kref,
    /* idx:1 out port, 0: in port */
    port: [audio_port_data; 2],
    cmd_wait: wait_queue_head_t,
    result: aprv2_ibasic_rsp_result_t,
    perf_mode: c_int,
    q6asm: *mut q6asm,
    dev: *mut device,
}

#[repr(C)]
struct apr_hdr {
    hdr_field: u32,
    src_port: u32,
    dest_port: u32,
    pkt_size: u32,
    token: u32,
    opcode: u32,
}

#[repr(C)]
struct apr_pkt {
    hdr: apr_hdr,
}

#[repr(C)]
struct apr_resp_pkt {
    hdr: apr_hdr,
    payload: *mut c_void,
}

#[repr(C)]
struct aprv2_ibasic_rsp_result_t {
    opcode: u32,
    status: u32,
}

#[repr(C)]
struct apr_device {
    dev: device,
    svc_id: u32,
}

#[repr(C)]
struct device {
    parent: *mut device,
}

#[repr(C)]
struct q6core_svc_api_info {
    _private: [u8; 0],
}

#[repr(C)]
struct wait_queue_head_t {
    _private: [u8; 0],
}

#[repr(C)]
struct spinlock_t {
    _private: [u8; 0],
}

#[repr(C)]
struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
struct kref {
    _private: [u8; 0],
}

#[repr(C)]
struct atomic_t {
    _private: [u8; 0],
}

#[repr(C)]
struct of_device_id {
    compatible: *const c_char,
}

#[repr(C)]
struct driver {
    name: *const c_char,
    of_match_table: *const of_device_id,
}

#[repr(C)]
struct apr_driver {
    probe: Option<unsafe extern "C" fn(*mut apr_device) -> c_int>,
    callback: Option<unsafe extern "C" fn(*mut apr_device, *const apr_resp_pkt) -> c_int>,
    driver: driver,
}

#[repr(C)]
struct q6asm_flac_cfg {
    stream_info_present: u16,
    ch_cfg: u16,
    min_blk_size: u16,
    max_blk_size: u16,
    sample_rate: u32,
    min_frame_size: u32,
    max_frame_size: u32,
    sample_size: u16,
}

#[repr(C)]
struct q6asm_wma_cfg {
    fmtag: u16,
    num_channels: u16,
    sample_rate: u32,
    bytes_per_sec: u32,
    block_align: u16,
    bits_per_sample: u16,
    channel_mask: u32,
    enc_options: u16,
    adv_enc_options: u16,
    adv_enc_options2: u32,
}

#[repr(C)]
struct q6asm_alac_cfg {
    frame_length: u32,
    compatible_version: u8,
    bit_depth: u8,
    pb: u8,
    mb: u8,
    kb: u8,
    num_channels: u8,
    max_run: u16,
    max_frame_bytes: u32,
    avg_bit_rate: u32,
    sample_rate: u32,
    channel_layout_tag: u32,
}

#[repr(C)]
struct q6asm_ape_cfg {
    compatible_version: u16,
    compression_level: u16,
    format_flags: u32,
    blocks_per_frame: u32,
    final_frame_blocks: u32,
    total_frames: u32,
    bits_per_sample: u16,
    num_channels: u16,
    sample_rate: u32,
    seek_table_present: u32,
}

type q6asm_cb = Option<unsafe extern "C" fn(u32, u32, *mut c_void, *mut c_void)>;

unsafe extern "C" {
    fn apr_send_pkt(adev: *mut apr_device, pkt: *mut apr_pkt) -> c_int;
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn kzalloc(size: usize, flags: c_uint) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn mutex_init(lock: *mut mutex);
    fn spin_lock_irqsave(lock: *mut spinlock_t, flags: *mut usize);
    fn spin_unlock_irqrestore(lock: *mut spinlock_t, flags: usize);
    fn spin_lock_init(lock: *mut spinlock_t);
    fn init_waitqueue_head(wq: *mut wait_queue_head_t);
    fn wake_up(wq: *mut wait_queue_head_t);
    fn atomic_read(v: *mut atomic_t) -> c_int;
    fn atomic_set(v: *mut atomic_t, i: c_int);
    fn kref_init(kref: *mut kref);
    fn kref_get(kref: *mut kref);
    fn kref_put(kref: *mut kref, release: unsafe extern "C" fn(*mut kref));
    fn wait_event_timeout_cmd_wait(ac: *mut audio_client, opcode: u32, timeout: c_int) -> c_int;
    fn wait_event_timeout_mem_wait(a: *mut q6asm, opcode: u32, rsp_opcode: u32, timeout: c_int) -> c_int;
    fn q6core_get_svc_api_info(svc_id: u32, info: *mut q6core_svc_api_info);
    fn q6dsp_map_channels(channel_mapping: *mut u8, channels: u32) -> c_int;
    fn devm_of_platform_populate(dev: *mut device) -> c_int;
    fn of_match_ptr(ids: *const of_device_id) -> *const of_device_id;
    fn ERR_PTR(error: isize) -> *mut audio_client;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn pr_err(fmt: *const c_char, ...);
}

const fn BIT(nr: u32) -> u32 {
    1u32 << nr
}

const fn ALIGN(x: usize, a: usize) -> usize {
    (x + (a - 1)) & !(a - 1)
}

const fn lower_32_bits(n: phys_addr_t) -> u32 {
    n as u32
}

const fn upper_32_bits(n: phys_addr_t) -> u32 {
    (n >> 32) as u32
}

unsafe fn zalloc_pkt(pkt_size: c_int, flags: c_uint) -> *mut c_void {
    unsafe { kzalloc(pkt_size as usize, flags) }
}

unsafe fn q6asm_add_hdr(
    ac: *mut audio_client,
    hdr: *mut apr_hdr,
    pkt_size: uint32_t,
    cmd_flg: bool_,
    stream_id: uint32_t,
) {
    unsafe {
        (*hdr).hdr_field = APR_SEQ_CMD_HDR_FIELD;
        (*hdr).src_port = (((*ac).session << 8) as u32 & 0xFF00) | stream_id;
        (*hdr).dest_port = (((*ac).session << 8) as u32 & 0xFF00) | stream_id;
        (*hdr).pkt_size = pkt_size;
        if cmd_flg {
            (*hdr).token = (*ac).session as u32;
        }
    }
}

unsafe fn q6asm_apr_send_session_pkt(
    a: *mut q6asm,
    ac: *mut audio_client,
    pkt: *mut apr_pkt,
    rsp_opcode: uint32_t,
) -> c_int {
    unsafe {
        let hdr = &mut (*pkt).hdr as *mut apr_hdr;
        let mut rc: c_int;

        mutex_lock(&mut (*ac).cmd_lock);
        (*ac).result.opcode = 0;
        (*ac).result.status = 0;
        rc = apr_send_pkt((*a).adev, pkt);
        if rc < 0 {
            goto_err(ac);
            return rc;
        }

        rc = wait_event_timeout_mem_wait(a, (*hdr).opcode, rsp_opcode, 5 * HZ);
        if rc == 0 {
            dev_err((*a).dev, c"CMD %x timeout\n".as_ptr(), (*hdr).opcode);
            rc = -ETIMEDOUT;
        } else if (*ac).result.status > 0 {
            dev_err((*a).dev, c"DSP returned error[%x]\n".as_ptr(), (*ac).result.status);
            rc = -EINVAL;
        }

        goto_err(ac);
        rc
    }
}

unsafe fn goto_err(ac: *mut audio_client) {
    unsafe {
        mutex_unlock(&mut (*ac).cmd_lock);
    }
}

unsafe fn __q6asm_memory_unmap(ac: *mut audio_client, _buf_add: phys_addr_t, dir: c_int) -> c_int {
    unsafe {
        let mem_unmap: *mut avs_cmd_shared_mem_unmap_regions;
        let a = dev_get_drvdata((*(*ac).dev).parent) as *mut q6asm;
        let pkt: *mut apr_pkt;
        let rc: c_int;
        let pkt_size = (APR_HDR_SIZE + size_of::<avs_cmd_shared_mem_unmap_regions>()) as c_int;

        if (*ac).port[dir as usize].mem_map_handle == 0 {
            dev_err((*ac).dev, c"invalid mem handle\n".as_ptr());
            return -EINVAL;
        }

        let p = zalloc_pkt(pkt_size, GFP_KERNEL);
        if p.is_null() {
            return -ENOMEM;
        }

        pkt = p as *mut apr_pkt;
        mem_unmap = (p as *mut u8).add(APR_HDR_SIZE) as *mut avs_cmd_shared_mem_unmap_regions;

        (*pkt).hdr.hdr_field = APR_SEQ_CMD_HDR_FIELD;
        (*pkt).hdr.src_port = 0;
        (*pkt).hdr.dest_port = 0;
        (*pkt).hdr.pkt_size = pkt_size as u32;
        (*pkt).hdr.token = (((*ac).session << 8) | dir) as u32;

        (*pkt).hdr.opcode = ASM_CMD_SHARED_MEM_UNMAP_REGIONS;
        (*mem_unmap).mem_map_handle = (*ac).port[dir as usize].mem_map_handle;

        rc = q6asm_apr_send_session_pkt(a, ac, pkt, 0);
        kfree(p);
        if rc < 0 {
            return rc;
        }

        (*ac).port[dir as usize].mem_map_handle = 0;
        0
    }
}

unsafe fn q6asm_audio_client_free_buf(ac: *mut audio_client, port: *mut audio_port_data) {
    unsafe {
        let mut flags: usize = 0;

        spin_lock_irqsave(&mut (*ac).lock, &mut flags);
        (*port).num_periods = 0;
        spin_unlock_irqrestore(&mut (*ac).lock, flags);
        kfree((*port).buf as *mut c_void);
        (*port).buf = ptr::null_mut();
    }
}

/**
 * q6asm_unmap_memory_regions() - unmap memory regions in the dsp.
 *
 * @dir: direction of audio stream
 * @ac: audio client instanace
 *
 * Return: Will be an negative value on failure or zero on success
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn q6asm_unmap_memory_regions(dir: c_uint, ac: *mut audio_client) -> c_int {
    unsafe {
        let port: *mut audio_port_data;
        let mut cnt: c_int = 0;
        let mut rc: c_int = 0;

        port = &mut (*ac).port[dir as usize];
        if (*port).buf.is_null() {
            rc = -EINVAL;
            return rc;
        }

        cnt = (*port).num_periods as c_int - 1;
        if cnt >= 0 {
            rc = __q6asm_memory_unmap(ac, (*(*port).buf.add(dir as usize)).phys, dir as c_int);
            if rc < 0 {
                dev_err(
                    (*ac).dev,
                    c"%s: Memory_unmap_regions failed %d\n".as_ptr(),
                    c"q6asm_unmap_memory_regions".as_ptr(),
                    rc,
                );
                return rc;
            }
        }

        q6asm_audio_client_free_buf(ac, port);
        rc
    }
}
// EXPORT_SYMBOL_GPL(q6asm_unmap_memory_regions);

unsafe fn __q6asm_memory_map_regions(
    ac: *mut audio_client,
    dir: c_int,
    period_sz: size_t,
    periods: c_uint,
    is_contiguous: bool_,
) -> c_int {
    unsafe {
        let mut cmd: *mut avs_cmd_shared_mem_map_regions = ptr::null_mut();
        let mut mregions: *mut avs_shared_map_region_payload = ptr::null_mut();
        let a = dev_get_drvdata((*(*ac).dev).parent) as *mut q6asm;
        let mut port: *mut audio_port_data = ptr::null_mut();
        let mut ab: *mut audio_buffer = ptr::null_mut();
        let pkt: *mut apr_pkt;
        let mut flags: usize = 0;
        let num_regions: uint32_t;
        let mut buf_sz: uint32_t;
        let mut i: c_int;

        if is_contiguous {
            num_regions = 1;
            buf_sz = (period_sz * periods as usize) as u32;
        } else {
            buf_sz = period_sz as u32;
            num_regions = periods;
        }

        /* DSP expects size should be aligned to 4K */
        buf_sz = ALIGN(buf_sz as usize, 4096) as u32;

        let pkt_size = APR_HDR_SIZE
            + size_of::<avs_cmd_shared_mem_map_regions>()
            + (size_of::<avs_shared_map_region_payload>() * num_regions as usize);

        let p = zalloc_pkt(pkt_size as c_int, GFP_KERNEL);
        if p.is_null() {
            return -ENOMEM;
        }

        pkt = p as *mut apr_pkt;
        cmd = (p as *mut u8).add(APR_HDR_SIZE) as *mut avs_cmd_shared_mem_map_regions;
        mregions = (p as *mut u8).add(APR_HDR_SIZE + size_of::<avs_cmd_shared_mem_map_regions>())
            as *mut avs_shared_map_region_payload;

        (*pkt).hdr.hdr_field = APR_SEQ_CMD_HDR_FIELD;
        (*pkt).hdr.src_port = 0;
        (*pkt).hdr.dest_port = 0;
        (*pkt).hdr.pkt_size = pkt_size as u32;
        (*pkt).hdr.token = (((*ac).session << 8) | dir) as u32;
        (*pkt).hdr.opcode = ASM_CMD_SHARED_MEM_MAP_REGIONS;

        (*cmd).mem_pool_id = ADSP_MEMORY_MAP_SHMEM8_4K_POOL;
        (*cmd).num_regions = num_regions as u16;
        (*cmd).property_flag = 0x00;

        spin_lock_irqsave(&mut (*ac).lock, &mut flags);
        port = &mut (*ac).port[dir as usize];

        i = 0;
        while i < num_regions as c_int {
            ab = (*port).buf.add(i as usize);
            (*mregions).shm_addr_lsw = lower_32_bits((*ab).phys);
            (*mregions).shm_addr_msw = upper_32_bits((*ab).phys);
            (*mregions).mem_size_bytes = buf_sz;
            mregions = mregions.add(1);
            i += 1;
        }
        spin_unlock_irqrestore(&mut (*ac).lock, flags);

        let rc = q6asm_apr_send_session_pkt(a, ac, pkt, ASM_CMDRSP_SHARED_MEM_MAP_REGIONS);
        kfree(p);
        rc
    }
}

/**
 * q6asm_map_memory_regions() - map memory regions in the dsp.
 *
 * @dir: direction of audio stream
 * @ac: audio client instanace
 * @phys: physical address that needs mapping.
 * @period_sz: audio period size
 * @periods: number of periods
 *
 * Return: Will be an negative value on failure or zero on success
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn q6asm_map_memory_regions(
    dir: c_uint,
    ac: *mut audio_client,
    phys: phys_addr_t,
    period_sz: size_t,
    periods: c_uint,
) -> c_int {
    unsafe {
        let buf: *mut audio_buffer;
        let mut flags: usize = 0;
        let mut cnt: c_int;
        let rc: c_int;

        spin_lock_irqsave(&mut (*ac).lock, &mut flags);
        if !(*ac).port[dir as usize].buf.is_null() {
            dev_err((*ac).dev, c"Buffer already allocated\n".as_ptr());
            spin_unlock_irqrestore(&mut (*ac).lock, flags);
            return 0;
        }

        buf = kzalloc(size_of::<audio_buffer>() * periods as usize, GFP_ATOMIC) as *mut audio_buffer;
        if buf.is_null() {
            spin_unlock_irqrestore(&mut (*ac).lock, flags);
            return -ENOMEM;
        }

        (*ac).port[dir as usize].buf = buf;

        (*buf.add(0)).phys = phys;
        (*buf.add(0)).size = period_sz as u32;

        cnt = 1;
        while cnt < periods as c_int {
            if period_sz > 0 {
                (*buf.add(cnt as usize)).phys = (*buf.add(0)).phys + (cnt as usize * period_sz);
                (*buf.add(cnt as usize)).size = period_sz as u32;
            }
            cnt += 1;
        }
        (*ac).port[dir as usize].num_periods = periods;

        spin_unlock_irqrestore(&mut (*ac).lock, flags);

        rc = __q6asm_memory_map_regions(ac, dir as c_int, period_sz, periods, true);
        if rc < 0 {
            dev_err((*ac).dev, c"Memory_map_regions failed\n".as_ptr());
            q6asm_audio_client_free_buf(ac, &mut (*ac).port[dir as usize]);
        }

        rc
    }
}
// EXPORT_SYMBOL_GPL(q6asm_map_memory_regions);

unsafe extern "C" fn q6asm_audio_client_release(ref_: *mut kref) {
    unsafe {
        let ac = (ref_ as *mut u8).sub(offset_of!(audio_client, refcount)) as *mut audio_client;
        let a = (*ac).q6asm;
        let mut flags: usize = 0;

        spin_lock_irqsave(&mut (*a).slock, &mut flags);
        (*a).session[(*ac).session as usize] = ptr::null_mut();
        spin_unlock_irqrestore(&mut (*a).slock, flags);

        kfree(ac as *mut c_void);
    }
}

/**
 * q6asm_audio_client_free() - Freee allocated audio client
 *
 * @ac: audio client to free
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn q6asm_audio_client_free(ac: *mut audio_client) {
    unsafe {
        kref_put(&mut (*ac).refcount, q6asm_audio_client_release);
    }
}
// EXPORT_SYMBOL_GPL(q6asm_audio_client_free);

unsafe fn q6asm_get_audio_client(a: *mut q6asm, session_id: c_int) -> *mut audio_client {
    unsafe {
        let mut ac: *mut audio_client = ptr::null_mut();
        let mut flags: usize = 0;

        spin_lock_irqsave(&mut (*a).slock, &mut flags);
        if (session_id <= 0) || (session_id > MAX_SESSIONS as c_int) {
            dev_err((*a).dev, c"invalid session: %d\n".as_ptr(), session_id);
        } else if !(*a).session[session_id as usize].is_null()
            && (*(*a).session[session_id as usize]).session == session_id
        {
            ac = (*a).session[session_id as usize];
            kref_get(&mut (*ac).refcount);
        }
        spin_unlock_irqrestore(&mut (*a).slock, flags);
        ac
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn q6asm_get_hw_pointer(ac: *mut audio_client, dir: c_uint) -> c_int {
    unsafe {
        let data = &mut (*ac).port[dir as usize] as *mut audio_port_data;
        atomic_read(&mut (*data).hw_ptr) as c_int
    }
}
// EXPORT_SYMBOL_GPL(q6asm_get_hw_pointer);

unsafe fn q6asm_stream_callback(
    adev: *mut apr_device,
    data: *const apr_resp_pkt,
    session_id: c_int,
) -> int32_t {
    unsafe {
        let q6asm = dev_get_drvdata(&mut (*adev).dev) as *mut q6asm;
        let result: *const aprv2_ibasic_rsp_result_t;
        let hdr = &(*data).hdr as *const apr_hdr;
        let mut port: *mut audio_port_data;
        let ac: *mut audio_client;
        let mut client_event: uint32_t = 0;
        let mut ret: c_int = 0;

        ac = q6asm_get_audio_client(q6asm, session_id);
        if ac.is_null() {
            /* Audio client might already be freed by now */
            return 0;
        }

        result = (*data).payload as *const aprv2_ibasic_rsp_result_t;

        match (*hdr).opcode {
            APR_BASIC_RSP_RESULT => {
                match (*result).opcode {
                    ASM_SESSION_CMD_PAUSE => client_event = ASM_CLIENT_EVENT_CMD_PAUSE_DONE,
                    ASM_SESSION_CMD_SUSPEND => client_event = ASM_CLIENT_EVENT_CMD_SUSPEND_DONE,
                    ASM_STREAM_CMD_FLUSH => client_event = ASM_CLIENT_EVENT_CMD_FLUSH_DONE,
                    ASM_SESSION_CMD_RUN_V2 => client_event = ASM_CLIENT_EVENT_CMD_RUN_DONE,
                    ASM_STREAM_CMD_CLOSE => client_event = ASM_CLIENT_EVENT_CMD_CLOSE_DONE,
                    ASM_STREAM_CMD_FLUSH_READBUFS => {
                        client_event = ASM_CLIENT_EVENT_CMD_OUT_FLUSH_DONE
                    }
                    ASM_STREAM_CMD_OPEN_WRITE_V3
                    | ASM_STREAM_CMD_OPEN_READ_V3
                    | ASM_STREAM_CMD_OPEN_READWRITE_V2
                    | ASM_STREAM_CMD_SET_ENCDEC_PARAM
                    | ASM_DATA_CMD_MEDIA_FMT_UPDATE_V2
                    | ASM_DATA_CMD_REMOVE_INITIAL_SILENCE
                    | ASM_DATA_CMD_REMOVE_TRAILING_SILENCE => {
                        if (*result).status != 0 {
                            dev_err(
                                (*ac).dev,
                                c"cmd = 0x%x returned error = 0x%x\n".as_ptr(),
                                (*result).opcode,
                                (*result).status,
                            );
                            (*ac).result = *result;
                            wake_up(&mut (*ac).cmd_wait);
                            ret = 0;
                            kref_put(&mut (*ac).refcount, q6asm_audio_client_release);
                            return ret;
                        }
                    }
                    ASM_DATA_CMD_EOS | ASM_DATA_CMD_READ_V2 | ASM_DATA_CMD_WRITE_V2 => {
                        /* response as result of close stream */
                        kref_put(&mut (*ac).refcount, q6asm_audio_client_release);
                        return ret;
                    }
                    _ => {
                        dev_err(
                            (*ac).dev,
                            c"command[0x%x] not expecting rsp\n".as_ptr(),
                            (*result).opcode,
                        );
                    }
                }

                (*ac).result = *result;
                wake_up(&mut (*ac).cmd_wait);

                if let Some(cb) = (*ac).cb {
                    cb(client_event, (*hdr).token, (*data).payload, (*ac).priv_);
                }

                ret = 0;
                kref_put(&mut (*ac).refcount, q6asm_audio_client_release);
                return ret;
            }
            ASM_DATA_EVENT_WRITE_DONE_V2 => {
                client_event = ASM_CLIENT_EVENT_DATA_WRITE_DONE;
                if ((*ac).io_mode & ASM_SYNC_IO_MODE) != 0 {
                    let phys: phys_addr_t;
                    let token = ((*hdr).token as c_int) & ASM_WRITE_TOKEN_MASK;
                    let mut flags: usize = 0;

                    spin_lock_irqsave(&mut (*ac).lock, &mut flags);
                    port = &mut (*ac).port[SNDRV_PCM_STREAM_PLAYBACK];

                    if (*port).buf.is_null() {
                        spin_unlock_irqrestore(&mut (*ac).lock, flags);
                        kref_put(&mut (*ac).refcount, q6asm_audio_client_release);
                        return 0;
                    }

                    phys = (*(*port).buf.add(token as usize)).phys;

                    if lower_32_bits(phys) != (*result).opcode || upper_32_bits(phys) != (*result).status {
                        dev_err((*ac).dev, c"Expected addr %pa\n".as_ptr(), &mut (*(*port).buf.add(token as usize)).phys);
                        spin_unlock_irqrestore(&mut (*ac).lock, flags);
                        kref_put(&mut (*ac).refcount, q6asm_audio_client_release);
                        return -EINVAL;
                    }
                    atomic_set(&mut (*port).hw_ptr, token + 1);
                    spin_unlock_irqrestore(&mut (*ac).lock, flags);
                }
            }
            ASM_DATA_EVENT_READ_DONE_V2 => {
                client_event = ASM_CLIENT_EVENT_DATA_READ_DONE;
                if ((*ac).io_mode & ASM_SYNC_IO_MODE) != 0 {
                    let done = (*data).payload as *mut asm_data_cmd_read_v2_done;
                    let phys: phys_addr_t;
                    let mut flags: usize = 0;

                    spin_lock_irqsave(&mut (*ac).lock, &mut flags);
                    port = &mut (*ac).port[SNDRV_PCM_STREAM_CAPTURE];
                    if (*port).buf.is_null() {
                        spin_unlock_irqrestore(&mut (*ac).lock, flags);
                        kref_put(&mut (*ac).refcount, q6asm_audio_client_release);
                        return 0;
                    }

                    phys = (*(*port).buf.add((*hdr).token as usize)).phys;
                    atomic_set(&mut (*port).hw_ptr, (*hdr).token as c_int + 1);

                    if upper_32_bits(phys) != (*done).buf_addr_msw
                        || lower_32_bits(phys) != (*done).buf_addr_lsw
                    {
                        dev_err(
                            (*ac).dev,
                            c"Expected addr %pa %08x-%08x\n".as_ptr(),
                            &mut (*(*port).buf.add((*hdr).token as usize)).phys,
                            (*done).buf_addr_lsw,
                            (*done).buf_addr_msw,
                        );
                        spin_unlock_irqrestore(&mut (*ac).lock, flags);
                        kref_put(&mut (*ac).refcount, q6asm_audio_client_release);
                        return -EINVAL;
                    }
                    spin_unlock_irqrestore(&mut (*ac).lock, flags);
                }
            }
            ASM_DATA_EVENT_RENDERED_EOS => client_event = ASM_CLIENT_EVENT_CMD_EOS_DONE,
            _ => {}
        }

        if let Some(cb) = (*ac).cb {
            cb(client_event, (*hdr).token, (*data).payload, (*ac).priv_);
        }

        kref_put(&mut (*ac).refcount, q6asm_audio_client_release);
        ret
    }
}

unsafe extern "C" fn q6asm_srvc_callback(adev: *mut apr_device, data: *const apr_resp_pkt) -> c_int {
    unsafe {
        let q6asm = dev_get_drvdata(&mut (*adev).dev) as *mut q6asm;
        let mut result: *mut aprv2_ibasic_rsp_result_t;
        let port: *mut audio_port_data;
        let mut ac: *mut audio_client = ptr::null_mut();
        let hdr = &(*data).hdr as *const apr_hdr;
        let a: *mut q6asm;
        let mut sid: uint32_t;
        let dir: uint32_t;
        let session_id: c_int;

        session_id = (((*hdr).dest_port >> 8) & 0xFF) as c_int;
        if session_id != 0 {
            return q6asm_stream_callback(adev, data, session_id);
        }

        sid = ((*hdr).token >> 8) & 0x0F;
        ac = q6asm_get_audio_client(q6asm, sid as c_int);
        if ac.is_null() {
            dev_err(&mut (*adev).dev, c"Audio Client not active\n".as_ptr());
            return 0;
        }

        a = dev_get_drvdata((*(*ac).dev).parent) as *mut q6asm;
        dir = (*hdr).token & 0x0F;
        port = &mut (*ac).port[dir as usize];
        result = (*data).payload as *mut aprv2_ibasic_rsp_result_t;

        match (*hdr).opcode {
            APR_BASIC_RSP_RESULT => {
                match (*result).opcode {
                    ASM_CMD_SHARED_MEM_MAP_REGIONS | ASM_CMD_SHARED_MEM_UNMAP_REGIONS => {
                        (*ac).result = *result;
                        wake_up(&mut (*a).mem_wait);
                    }
                    _ => {
                        dev_err(
                            &mut (*adev).dev,
                            c"command[0x%x] not expecting rsp\n".as_ptr(),
                            (*result).opcode,
                        );
                    }
                }
                kref_put(&mut (*ac).refcount, q6asm_audio_client_release);
                return 0;
            }
            ASM_CMDRSP_SHARED_MEM_MAP_REGIONS => {
                (*ac).result.status = 0;
                (*ac).result.opcode = (*hdr).opcode;
                (*port).mem_map_handle = (*result).opcode;
                wake_up(&mut (*a).mem_wait);
            }
            ASM_CMD_SHARED_MEM_UNMAP_REGIONS => {
                (*ac).result.opcode = (*hdr).opcode;
                (*ac).result.status = 0;
                (*port).mem_map_handle = 0;
                wake_up(&mut (*a).mem_wait);
            }
            _ => {
                dev_dbg(
                    &mut (*adev).dev,
                    c"command[0x%x]success [0x%x]\n".as_ptr(),
                    (*result).opcode,
                    (*result).status,
                );
            }
        }

        if let Some(cb) = (*ac).cb {
            cb((*hdr).opcode, (*hdr).token, (*data).payload, (*ac).priv_);
        }

        kref_put(&mut (*ac).refcount, q6asm_audio_client_release);
        0
    }
}

/**
 * q6asm_get_session_id() - get session id for audio client
 *
 * @c: audio client pointer
 *
 * Return: Will be an session id of the audio client.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn q6asm_get_session_id(c: *mut audio_client) -> c_int {
    unsafe { (*c).session }
}
// EXPORT_SYMBOL_GPL(q6asm_get_session_id);

/**
 * q6asm_audio_client_alloc() - Allocate a new audio client
 *
 * @dev: Pointer to asm child device.
 * @cb: event callback.
 * @priv: private data associated with this client.
 * @session_id: session id
 * @perf_mode: performace mode for this client
 *
 * Return: Will be an error pointer on error or a valid audio client
 * on success.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn q6asm_audio_client_alloc(
    dev: *mut device,
    cb: q6asm_cb,
    priv_: *mut c_void,
    session_id: c_int,
    perf_mode: c_int,
) -> *mut audio_client {
    unsafe {
        let a = dev_get_drvdata((*dev).parent) as *mut q6asm;
        let mut ac: *mut audio_client;
        let mut flags: usize = 0;

        ac = q6asm_get_audio_client(a, session_id + 1);
        if !ac.is_null() {
            dev_err(dev, c"Audio Client already active\n".as_ptr());
            return ac;
        }

        ac = kzalloc(size_of::<audio_client>(), GFP_KERNEL) as *mut audio_client;
        if ac.is_null() {
            return ERR_PTR(-(ENOMEM as isize));
        }

        spin_lock_irqsave(&mut (*a).slock, &mut flags);
        (*a).session[(session_id + 1) as usize] = ac;
        spin_unlock_irqrestore(&mut (*a).slock, flags);
        (*ac).session = session_id + 1;
        (*ac).cb = cb;
        (*ac).dev = dev;
        (*ac).q6asm = a;
        (*ac).priv_ = priv_;
        (*ac).io_mode = ASM_SYNC_IO_MODE;
        (*ac).perf_mode = perf_mode;
        (*ac).adev = (*a).adev;
        kref_init(&mut (*ac).refcount);

        init_waitqueue_head(&mut (*ac).cmd_wait);
        mutex_init(&mut (*ac).cmd_lock);
        spin_lock_init(&mut (*ac).lock);

        ac
    }
}
// EXPORT_SYMBOL_GPL(q6asm_audio_client_alloc);

unsafe fn q6asm_ac_send_cmd_sync(ac: *mut audio_client, pkt: *mut apr_pkt) -> c_int {
    unsafe {
        let hdr = &mut (*pkt).hdr as *mut apr_hdr;
        let mut rc: c_int;

        mutex_lock(&mut (*ac).cmd_lock);
        (*ac).result.opcode = 0;
        (*ac).result.status = 0;

        rc = apr_send_pkt((*ac).adev, pkt);
        if rc < 0 {
            mutex_unlock(&mut (*ac).cmd_lock);
            return rc;
        }

        rc = wait_event_timeout_cmd_wait(ac, (*hdr).opcode, 5 * HZ);
        if rc == 0 {
            dev_err((*ac).dev, c"CMD %x timeout\n".as_ptr(), (*hdr).opcode);
            rc = -ETIMEDOUT;
            mutex_unlock(&mut (*ac).cmd_lock);
            return rc;
        }

        if (*ac).result.status > 0 {
            dev_err((*ac).dev, c"DSP returned error[%x]\n".as_ptr(), (*ac).result.status);
            rc = -EINVAL;
        } else {
            rc = 0;
        }

        mutex_unlock(&mut (*ac).cmd_lock);
        rc
    }
}

/**
 * q6asm_open_write() - Open audio client for writing
 * @ac: audio client pointer
 * @stream_id: stream id of q6asm session
 * @format: audio sample format
 * @codec_profile: compressed format profile
 * @bits_per_sample: bits per sample
 * @is_gapless: flag to indicate if this is a gapless stream
 *
 * Return: Will be an negative value on error or zero on success
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn q6asm_open_write(
    ac: *mut audio_client,
    stream_id: uint32_t,
    format: uint32_t,
    codec_profile: u32,
    bits_per_sample: uint16_t,
    is_gapless: bool_,
) -> c_int {
    unsafe {
        let open: *mut asm_stream_cmd_open_write_v3;
        let pkt: *mut apr_pkt;
        let mut rc: c_int;
        let pkt_size = (APR_HDR_SIZE + size_of::<asm_stream_cmd_open_write_v3>()) as c_int;

        let p = zalloc_pkt(pkt_size, GFP_KERNEL);
        if p.is_null() {
            return -ENOMEM;
        }

        pkt = p as *mut apr_pkt;
        open = (p as *mut u8).add(APR_HDR_SIZE) as *mut asm_stream_cmd_open_write_v3;
        q6asm_add_hdr(ac, &mut (*pkt).hdr, pkt_size as u32, true, stream_id);

        (*pkt).hdr.opcode = ASM_STREAM_CMD_OPEN_WRITE_V3;
        (*open).mode_flags = 0x00;
        (*open).mode_flags |= ASM_LEGACY_STREAM_SESSION;
        if is_gapless {
            (*open).mode_flags |= BIT(ASM_SHIFT_GAPLESS_MODE_FLAG);
        }

        /* source endpoint : matrix */
        (*open).sink_endpointype = ASM_END_POINT_DEVICE_MATRIX as u16;
        (*open).bits_per_sample = bits_per_sample;
        (*open).postprocopo_id = ASM_NULL_POPP_TOPOLOGY;

        match format {
            SND_AUDIOCODEC_MP3 => (*open).dec_fmt_id = ASM_MEDIA_FMT_MP3,
            FORMAT_LINEAR_PCM => (*open).dec_fmt_id = ASM_MEDIA_FMT_MULTI_CHANNEL_PCM_V2,
            SND_AUDIOCODEC_FLAC => (*open).dec_fmt_id = ASM_MEDIA_FMT_FLAC,
            SND_AUDIOCODEC_WMA => match codec_profile {
                SND_AUDIOPROFILE_WMA9 => (*open).dec_fmt_id = ASM_MEDIA_FMT_WMA_V9,
                SND_AUDIOPROFILE_WMA10
                | SND_AUDIOPROFILE_WMA9_PRO
                | SND_AUDIOPROFILE_WMA9_LOSSLESS
                | SND_AUDIOPROFILE_WMA10_LOSSLESS => (*open).dec_fmt_id = ASM_MEDIA_FMT_WMA_V10,
                _ => {
                    dev_err((*ac).dev, c"Invalid codec profile 0x%x\n".as_ptr(), codec_profile);
                    kfree(p);
                    return -EINVAL;
                }
            },
            SND_AUDIOCODEC_ALAC => (*open).dec_fmt_id = ASM_MEDIA_FMT_ALAC,
            SND_AUDIOCODEC_APE => (*open).dec_fmt_id = ASM_MEDIA_FMT_APE,
            _ => {
                dev_err((*ac).dev, c"Invalid format 0x%x\n".as_ptr(), format);
                kfree(p);
                return -EINVAL;
            }
        }

        rc = q6asm_ac_send_cmd_sync(ac, pkt);
        if rc >= 0 {
            (*ac).io_mode |= ASM_TUN_WRITE_IO_MODE;
        }
        kfree(p);
        rc
    }
}
// EXPORT_SYMBOL_GPL(q6asm_open_write);

unsafe fn __q6asm_run(
    ac: *mut audio_client,
    stream_id: uint32_t,
    flags: uint32_t,
    msw_ts: uint32_t,
    lsw_ts: uint32_t,
    wait: bool_,
) -> c_int {
    unsafe {
        let run: *mut asm_session_cmd_run_v2;
        let pkt: *mut apr_pkt;
        let mut rc: c_int;
        let pkt_size = (APR_HDR_SIZE + size_of::<asm_session_cmd_run_v2>()) as c_int;

        let p = zalloc_pkt(pkt_size, GFP_ATOMIC);
        if p.is_null() {
            return -ENOMEM;
        }

        pkt = p as *mut apr_pkt;
        run = (p as *mut u8).add(APR_HDR_SIZE) as *mut asm_session_cmd_run_v2;

        q6asm_add_hdr(ac, &mut (*pkt).hdr, pkt_size as u32, true, stream_id);

        (*pkt).hdr.opcode = ASM_SESSION_CMD_RUN_V2;
        (*run).flags = flags;
        (*run).time_lsw = lsw_ts;
        (*run).time_msw = msw_ts;
        if wait {
            rc = q6asm_ac_send_cmd_sync(ac, pkt);
        } else {
            rc = apr_send_pkt((*ac).adev, pkt);
            if rc == pkt_size {
                rc = 0;
            }
        }

        kfree(p);
        rc
    }
}

/**
 * q6asm_run() - start the audio client
 *
 * @ac: audio client pointer
 * @stream_id: stream id of q6asm session
 * @flags: flags associated with write
 * @msw_ts: timestamp msw
 * @lsw_ts: timestamp lsw
 *
 * Return: Will be an negative value on error or zero on success
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn q6asm_run(
    ac: *mut audio_client,
    stream_id: uint32_t,
    flags: uint32_t,
    msw_ts: uint32_t,
    lsw_ts: uint32_t,
) -> c_int {
    unsafe { __q6asm_run(ac, stream_id, flags, msw_ts, lsw_ts, true) }
}
// EXPORT_SYMBOL_GPL(q6asm_run);

/**
 * q6asm_run_nowait() - start the audio client withou blocking
 *
 * @ac: audio client pointer
 * @stream_id: stream id
 * @flags: flags associated with write
 * @msw_ts: timestamp msw
 * @lsw_ts: timestamp lsw
 *
 * Return: Will be an negative value on error or zero on success
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn q6asm_run_nowait(
    ac: *mut audio_client,
    stream_id: uint32_t,
    flags: uint32_t,
    msw_ts: uint32_t,
    lsw_ts: uint32_t,
) -> c_int {
    unsafe { __q6asm_run(ac, stream_id, flags, msw_ts, lsw_ts, false) }
}
// EXPORT_SYMBOL_GPL(q6asm_run_nowait);

/**
 * q6asm_media_format_block_multi_ch_pcm() - setup pcm configuration
 *
 * @ac: audio client pointer
 * @stream_id: stream id
 * @rate: audio sample rate
 * @channels: number of audio channels.
 * @channel_map: channel map pointer
 * @bits_per_sample: bits per sample
 *
 * Return: Will be an negative value on error or zero on success
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn q6asm_media_format_block_multi_ch_pcm(
    ac: *mut audio_client,
    stream_id: uint32_t,
    rate: uint32_t,
    channels: uint32_t,
    channel_map: *mut u8,
    bits_per_sample: uint16_t,
) -> c_int {
    unsafe {
        let fmt: *mut asm_multi_channel_pcm_fmt_blk_v2;
        let pkt: *mut apr_pkt;
        let channel_mapping: *mut u8;
        let pkt_size = (APR_HDR_SIZE + size_of::<asm_multi_channel_pcm_fmt_blk_v2>()) as c_int;

        let p = zalloc_pkt(pkt_size, GFP_KERNEL);
        if p.is_null() {
            return -ENOMEM;
        }

        pkt = p as *mut apr_pkt;
        fmt = (p as *mut u8).add(APR_HDR_SIZE) as *mut asm_multi_channel_pcm_fmt_blk_v2;

        q6asm_add_hdr(ac, &mut (*pkt).hdr, pkt_size as u32, true, stream_id);

        (*pkt).hdr.opcode = ASM_DATA_CMD_MEDIA_FMT_UPDATE_V2;
        (*fmt).fmt_blk.fmt_blk_size =
            (size_of::<asm_multi_channel_pcm_fmt_blk_v2>() - size_of::<asm_data_cmd_media_fmt_update_v2>()) as u32;
        (*fmt).num_channels = channels as u16;
        (*fmt).bits_per_sample = bits_per_sample;
        (*fmt).sample_rate = rate;
        (*fmt).is_signed = 1;

        channel_mapping = (*fmt).channel_mapping.as_mut_ptr();

        if !channel_map.is_null() {
            ptr::copy_nonoverlapping(channel_map, channel_mapping, PCM_MAX_NUM_CHANNEL);
        } else if q6dsp_map_channels(channel_mapping, channels) != 0 {
            dev_err((*ac).dev, c" map channels failed %d\n".as_ptr(), channels);
            kfree(p);
            return -EINVAL;
        }

        let rc = q6asm_ac_send_cmd_sync(ac, pkt);
        kfree(p);
        rc
    }
}
// EXPORT_SYMBOL_GPL(q6asm_media_format_block_multi_ch_pcm);

#[unsafe(no_mangle)]
pub unsafe extern "C" fn q6asm_stream_media_format_block_flac(
    ac: *mut audio_client,
    stream_id: uint32_t,
    cfg: *mut q6asm_flac_cfg,
) -> c_int {
    unsafe {
        let fmt: *mut asm_flac_fmt_blk_v2;
        let pkt: *mut apr_pkt;
        let pkt_size = (APR_HDR_SIZE + size_of::<asm_flac_fmt_blk_v2>()) as c_int;

        let p = zalloc_pkt(pkt_size, GFP_KERNEL);
        if p.is_null() {
            return -ENOMEM;
        }

        pkt = p as *mut apr_pkt;
        fmt = (p as *mut u8).add(APR_HDR_SIZE) as *mut asm_flac_fmt_blk_v2;

        q6asm_add_hdr(ac, &mut (*pkt).hdr, pkt_size as u32, true, stream_id);

        (*pkt).hdr.opcode = ASM_DATA_CMD_MEDIA_FMT_UPDATE_V2;
        (*fmt).fmt_blk.fmt_blk_size =
            (size_of::<asm_flac_fmt_blk_v2>() - size_of::<asm_data_cmd_media_fmt_update_v2>()) as u32;
        (*fmt).is_stream_info_present = (*cfg).stream_info_present;
        (*fmt).num_channels = (*cfg).ch_cfg;
        (*fmt).min_blk_size = (*cfg).min_blk_size;
        (*fmt).max_blk_size = (*cfg).max_blk_size;
        (*fmt).sample_rate = (*cfg).sample_rate;
        (*fmt).min_frame_size = (*cfg).min_frame_size;
        (*fmt).max_frame_size = (*cfg).max_frame_size;
        (*fmt).sample_size = (*cfg).sample_size;

        let rc = q6asm_ac_send_cmd_sync(ac, pkt);
        kfree(p);
        rc
    }
}
// EXPORT_SYMBOL_GPL(q6asm_stream_media_format_block_flac);

#[unsafe(no_mangle)]
pub unsafe extern "C" fn q6asm_stream_media_format_block_wma_v9(
    ac: *mut audio_client,
    stream_id: uint32_t,
    cfg: *mut q6asm_wma_cfg,
) -> c_int {
    unsafe {
        let fmt: *mut asm_wmastdv9_fmt_blk_v2;
        let pkt: *mut apr_pkt;
        let pkt_size = (APR_HDR_SIZE + size_of::<asm_wmastdv9_fmt_blk_v2>()) as c_int;

        let p = zalloc_pkt(pkt_size, GFP_KERNEL);
        if p.is_null() {
            return -ENOMEM;
        }

        pkt = p as *mut apr_pkt;
        fmt = (p as *mut u8).add(APR_HDR_SIZE) as *mut asm_wmastdv9_fmt_blk_v2;

        q6asm_add_hdr(ac, &mut (*pkt).hdr, pkt_size as u32, true, stream_id);

        (*pkt).hdr.opcode = ASM_DATA_CMD_MEDIA_FMT_UPDATE_V2;
        (*fmt).fmt_blk.fmt_blk_size =
            (size_of::<asm_wmastdv9_fmt_blk_v2>() - size_of::<asm_data_cmd_media_fmt_update_v2>()) as u32;
        (*fmt).fmtag = (*cfg).fmtag;
        (*fmt).num_channels = (*cfg).num_channels;
        (*fmt).sample_rate = (*cfg).sample_rate;
        (*fmt).bytes_per_sec = (*cfg).bytes_per_sec;
        (*fmt).blk_align = (*cfg).block_align;
        (*fmt).bits_per_sample = (*cfg).bits_per_sample;
        (*fmt).channel_mask = (*cfg).channel_mask;
        (*fmt).enc_options = (*cfg).enc_options;
        (*fmt).reserved = 0;

        let rc = q6asm_ac_send_cmd_sync(ac, pkt);
        kfree(p);
        rc
    }
}
// EXPORT_SYMBOL_GPL(q6asm_stream_media_format_block_wma_v9);

#[unsafe(no_mangle)]
pub unsafe extern "C" fn q6asm_stream_media_format_block_wma_v10(
    ac: *mut audio_client,
    stream_id: uint32_t,
    cfg: *mut q6asm_wma_cfg,
) -> c_int {
    unsafe {
        let fmt: *mut asm_wmaprov10_fmt_blk_v2;
        let pkt: *mut apr_pkt;
        let pkt_size = (APR_HDR_SIZE + size_of::<asm_wmaprov10_fmt_blk_v2>()) as c_int;

        let p = zalloc_pkt(pkt_size, GFP_KERNEL);
        if p.is_null() {
            return -ENOMEM;
        }

        pkt = p as *mut apr_pkt;
        fmt = (p as *mut u8).add(APR_HDR_SIZE) as *mut asm_wmaprov10_fmt_blk_v2;

        q6asm_add_hdr(ac, &mut (*pkt).hdr, pkt_size as u32, true, stream_id);

        (*pkt).hdr.opcode = ASM_DATA_CMD_MEDIA_FMT_UPDATE_V2;
        (*fmt).fmt_blk.fmt_blk_size =
            (size_of::<asm_wmaprov10_fmt_blk_v2>() - size_of::<asm_data_cmd_media_fmt_update_v2>()) as u32;
        (*fmt).fmtag = (*cfg).fmtag;
        (*fmt).num_channels = (*cfg).num_channels;
        (*fmt).sample_rate = (*cfg).sample_rate;
        (*fmt).bytes_per_sec = (*cfg).bytes_per_sec;
        (*fmt).blk_align = (*cfg).block_align;
        (*fmt).bits_per_sample = (*cfg).bits_per_sample;
        (*fmt).channel_mask = (*cfg).channel_mask;
        (*fmt).enc_options = (*cfg).enc_options;
        (*fmt).advanced_enc_options1 = (*cfg).adv_enc_options;
        (*fmt).advanced_enc_options2 = (*cfg).adv_enc_options2;

        let rc = q6asm_ac_send_cmd_sync(ac, pkt);
        kfree(p);
        rc
    }
}
// EXPORT_SYMBOL_GPL(q6asm_stream_media_format_block_wma_v10);

#[unsafe(no_mangle)]
pub unsafe extern "C" fn q6asm_stream_media_format_block_alac(
    ac: *mut audio_client,
    stream_id: uint32_t,
    cfg: *mut q6asm_alac_cfg,
) -> c_int {
    unsafe {
        let fmt: *mut asm_alac_fmt_blk_v2;
        let pkt: *mut apr_pkt;
        let pkt_size = (APR_HDR_SIZE + size_of::<asm_alac_fmt_blk_v2>()) as c_int;

        let p = zalloc_pkt(pkt_size, GFP_KERNEL);
        if p.is_null() {
            return -ENOMEM;
        }

        pkt = p as *mut apr_pkt;
        fmt = (p as *mut u8).add(APR_HDR_SIZE) as *mut asm_alac_fmt_blk_v2;

        q6asm_add_hdr(ac, &mut (*pkt).hdr, pkt_size as u32, true, stream_id);

        (*pkt).hdr.opcode = ASM_DATA_CMD_MEDIA_FMT_UPDATE_V2;
        (*fmt).fmt_blk.fmt_blk_size =
            (size_of::<asm_alac_fmt_blk_v2>() - size_of::<asm_data_cmd_media_fmt_update_v2>()) as u32;

        (*fmt).frame_length = (*cfg).frame_length;
        (*fmt).compatible_version = (*cfg).compatible_version;
        (*fmt).bit_depth = (*cfg).bit_depth;
        (*fmt).num_channels = (*cfg).num_channels;
        (*fmt).max_run = (*cfg).max_run;
        (*fmt).max_frame_bytes = (*cfg).max_frame_bytes;
        (*fmt).avg_bit_rate = (*cfg).avg_bit_rate;
        (*fmt).sample_rate = (*cfg).sample_rate;
        (*fmt).channel_layout_tag = (*cfg).channel_layout_tag;
        (*fmt).pb = (*cfg).pb;
        (*fmt).mb = (*cfg).mb;
        (*fmt).kb = (*cfg).kb;

        let rc = q6asm_ac_send_cmd_sync(ac, pkt);
        kfree(p);
        rc
    }
}
// EXPORT_SYMBOL_GPL(q6asm_stream_media_format_block_alac);

#[unsafe(no_mangle)]
pub unsafe extern "C" fn q6asm_stream_media_format_block_ape(
    ac: *mut audio_client,
    stream_id: uint32_t,
    cfg: *mut q6asm_ape_cfg,
) -> c_int {
    unsafe {
        let fmt: *mut asm_ape_fmt_blk_v2;
        let pkt: *mut apr_pkt;
        let pkt_size = (APR_HDR_SIZE + size_of::<asm_ape_fmt_blk_v2>()) as c_int;

        let p = zalloc_pkt(pkt_size, GFP_KERNEL);
        if p.is_null() {
            return -ENOMEM;
        }

        pkt = p as *mut apr_pkt;
        fmt = (p as *mut u8).add(APR_HDR_SIZE) as *mut asm_ape_fmt_blk_v2;

        q6asm_add_hdr(ac, &mut (*pkt).hdr, pkt_size as u32, true, stream_id);

        (*pkt).hdr.opcode = ASM_DATA_CMD_MEDIA_FMT_UPDATE_V2;
        (*fmt).fmt_blk.fmt_blk_size =
            (size_of::<asm_ape_fmt_blk_v2>() - size_of::<asm_data_cmd_media_fmt_update_v2>()) as u32;

        (*fmt).compatible_version = (*cfg).compatible_version;
        (*fmt).compression_level = (*cfg).compression_level;
        (*fmt).format_flags = (*cfg).format_flags;
        (*fmt).blocks_per_frame = (*cfg).blocks_per_frame;
        (*fmt).final_frame_blocks = (*cfg).final_frame_blocks;
        (*fmt).total_frames = (*cfg).total_frames;
        (*fmt).bits_per_sample = (*cfg).bits_per_sample;
        (*fmt).num_channels = (*cfg).num_channels;
        (*fmt).sample_rate = (*cfg).sample_rate;
        (*fmt).seek_table_present = (*cfg).seek_table_present;

        let rc = q6asm_ac_send_cmd_sync(ac, pkt);
        kfree(p);
        rc
    }
}
// EXPORT_SYMBOL_GPL(q6asm_stream_media_format_block_ape);

unsafe fn q6asm_stream_remove_silence(
    ac: *mut audio_client,
    stream_id: uint32_t,
    cmd: uint32_t,
    num_samples: uint32_t,
) -> c_int {
    unsafe {
        let samples: *mut uint32_t;
        let pkt: *mut apr_pkt;
        let mut rc: c_int;
        let pkt_size = (APR_HDR_SIZE + size_of::<uint32_t>()) as c_int;

        let p = zalloc_pkt(pkt_size, GFP_ATOMIC);
        if p.is_null() {
            return -ENOMEM;
        }

        pkt = p as *mut apr_pkt;
        samples = (p as *mut u8).add(APR_HDR_SIZE) as *mut uint32_t;

        q6asm_add_hdr(ac, &mut (*pkt).hdr, pkt_size as u32, true, stream_id);

        (*pkt).hdr.opcode = cmd;
        *samples = num_samples;
        rc = apr_send_pkt((*ac).adev, pkt);
        if rc == pkt_size {
            rc = 0;
        }

        kfree(p);
        rc
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn q6asm_stream_remove_initial_silence(
    ac: *mut audio_client,
    stream_id: uint32_t,
    initial_samples: uint32_t,
) -> c_int {
    unsafe {
        q6asm_stream_remove_silence(
            ac,
            stream_id,
            ASM_DATA_CMD_REMOVE_INITIAL_SILENCE,
            initial_samples,
        )
    }
}
// EXPORT_SYMBOL_GPL(q6asm_stream_remove_initial_silence);

#[unsafe(no_mangle)]
pub unsafe extern "C" fn q6asm_stream_remove_trailing_silence(
    ac: *mut audio_client,
    stream_id: uint32_t,
    trailing_samples: uint32_t,
) -> c_int {
    unsafe {
        q6asm_stream_remove_silence(
            ac,
            stream_id,
            ASM_DATA_CMD_REMOVE_TRAILING_SILENCE,
            trailing_samples,
        )
    }
}
// EXPORT_SYMBOL_GPL(q6asm_stream_remove_trailing_silence);

/**
 * q6asm_enc_cfg_blk_pcm_format_support() - setup pcm configuration for capture
 *
 * @ac: audio client pointer
 * @stream_id: stream id
 * @rate: audio sample rate
 * @channels: number of audio channels.
 * @bits_per_sample: bits per sample
 *
 * Return: Will be an negative value on error or zero on success
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn q6asm_enc_cfg_blk_pcm_format_support(
    ac: *mut audio_client,
    stream_id: uint32_t,
    rate: uint32_t,
    channels: uint32_t,
    bits_per_sample: uint16_t,
) -> c_int {
    unsafe {
        let enc_cfg: *mut asm_multi_channel_pcm_enc_cfg_v2;
        let pkt: *mut apr_pkt;
        let channel_mapping: *mut u8;
        let frames_per_buf: u32 = 0;
        let pkt_size = (APR_HDR_SIZE + size_of::<asm_multi_channel_pcm_enc_cfg_v2>()) as c_int;

        let p = zalloc_pkt(pkt_size, GFP_KERNEL);
        if p.is_null() {
            return -ENOMEM;
        }

        pkt = p as *mut apr_pkt;
        enc_cfg = (p as *mut u8).add(APR_HDR_SIZE) as *mut asm_multi_channel_pcm_enc_cfg_v2;
        q6asm_add_hdr(ac, &mut (*pkt).hdr, pkt_size as u32, true, stream_id);

        (*pkt).hdr.opcode = ASM_STREAM_CMD_SET_ENCDEC_PARAM;
        (*enc_cfg).encdec.param_id = ASM_PARAM_ID_ENCDEC_ENC_CFG_BLK_V2;
        (*enc_cfg).encdec.param_size =
            (size_of::<asm_multi_channel_pcm_enc_cfg_v2>() - size_of::<asm_stream_cmd_set_encdec_param>()) as u32;
        (*enc_cfg).encblk.frames_per_buf = frames_per_buf;
        (*enc_cfg).encblk.enc_cfg_blk_size =
            (*enc_cfg).encdec.param_size - size_of::<asm_enc_cfg_blk_param_v2>() as u32;

        (*enc_cfg).num_channels = channels as u16;
        (*enc_cfg).bits_per_sample = bits_per_sample;
        (*enc_cfg).sample_rate = rate;
        (*enc_cfg).is_signed = 1;
        channel_mapping = (*enc_cfg).channel_mapping.as_mut_ptr();

        if q6dsp_map_channels(channel_mapping, channels) != 0 {
            kfree(p);
            return -EINVAL;
        }

        let rc = q6asm_ac_send_cmd_sync(ac, pkt);
        kfree(p);
        rc
    }
}
// EXPORT_SYMBOL_GPL(q6asm_enc_cfg_blk_pcm_format_support);

/**
 * q6asm_read() - read data of period size from audio client
 *
 * @ac: audio client pointer
 * @stream_id: stream id
 *
 * Return: Will be an negative value on error or zero on success
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn q6asm_read(ac: *mut audio_client, stream_id: uint32_t) -> c_int {
    unsafe {
        let read: *mut asm_data_cmd_read_v2;
        let port: *mut audio_port_data;
        let ab: *mut audio_buffer;
        let pkt: *mut apr_pkt;
        let mut flags: usize = 0;
        let pkt_size = (APR_HDR_SIZE + size_of::<asm_data_cmd_read_v2>()) as c_int;
        let mut rc: c_int = 0;

        let p = zalloc_pkt(pkt_size, GFP_ATOMIC);
        if p.is_null() {
            return -ENOMEM;
        }

        pkt = p as *mut apr_pkt;
        read = (p as *mut u8).add(APR_HDR_SIZE) as *mut asm_data_cmd_read_v2;

        spin_lock_irqsave(&mut (*ac).lock, &mut flags);
        port = &mut (*ac).port[SNDRV_PCM_STREAM_CAPTURE];
        q6asm_add_hdr(ac, &mut (*pkt).hdr, pkt_size as u32, false, stream_id);
        ab = (*port).buf.add((*port).dsp_buf as usize);
        (*pkt).hdr.opcode = ASM_DATA_CMD_READ_V2;
        (*read).buf_addr_lsw = lower_32_bits((*ab).phys);
        (*read).buf_addr_msw = upper_32_bits((*ab).phys);
        (*read).mem_map_handle = (*port).mem_map_handle;

        (*read).buf_size = (*ab).size;
        (*read).seq_id = (*port).dsp_buf;
        (*pkt).hdr.token = (*port).dsp_buf;

        (*port).dsp_buf += 1;

        if (*port).dsp_buf >= (*port).num_periods {
            (*port).dsp_buf = 0;
        }

        spin_unlock_irqrestore(&mut (*ac).lock, flags);
        rc = apr_send_pkt((*ac).adev, pkt);
        if rc == pkt_size {
            rc = 0;
        } else {
            pr_err(c"read op[0x%x]rc[%d]\n".as_ptr(), (*pkt).hdr.opcode, rc);
        }

        kfree(p);
        rc
    }
}
// EXPORT_SYMBOL_GPL(q6asm_read);

unsafe fn __q6asm_open_read(
    ac: *mut audio_client,
    stream_id: uint32_t,
    format: uint32_t,
    bits_per_sample: uint16_t,
) -> c_int {
    unsafe {
        let open: *mut asm_stream_cmd_open_read_v3;
        let pkt: *mut apr_pkt;
        let pkt_size = (APR_HDR_SIZE + size_of::<asm_stream_cmd_open_read_v3>()) as c_int;

        let p = zalloc_pkt(pkt_size, GFP_KERNEL);
        if p.is_null() {
            return -ENOMEM;
        }

        pkt = p as *mut apr_pkt;
        open = (p as *mut u8).add(APR_HDR_SIZE) as *mut asm_stream_cmd_open_read_v3;

        q6asm_add_hdr(ac, &mut (*pkt).hdr, pkt_size as u32, true, stream_id);
        (*pkt).hdr.opcode = ASM_STREAM_CMD_OPEN_READ_V3;
        /* Stream prio : High, provide meta info with encoded frames */
        (*open).src_endpointype = ASM_END_POINT_DEVICE_MATRIX;

        (*open).preprocopo_id = ASM_STREAM_POSTPROC_TOPO_ID_NONE;
        (*open).bits_per_sample = bits_per_sample;
        (*open).mode_flags = 0x0;

        (*open).mode_flags |= ASM_LEGACY_STREAM_SESSION << ASM_SHIFT_STREAM_PERF_MODE_FLAG_IN_OPEN_READ;

        match format {
            FORMAT_LINEAR_PCM => {
                (*open).mode_flags |= 0x00;
                (*open).enc_cfg_id = ASM_MEDIA_FMT_MULTI_CHANNEL_PCM_V2;
            }
            _ => {
                pr_err(c"Invalid format[%d]\n".as_ptr(), format);
            }
        }

        let rc = q6asm_ac_send_cmd_sync(ac, pkt);
        kfree(p);
        rc
    }
}

/**
 * q6asm_open_read() - Open audio client for reading
 *
 * @ac: audio client pointer
 * @stream_id: stream id
 * @format: audio sample format
 * @bits_per_sample: bits per sample
 *
 * Return: Will be an negative value on error or zero on success
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn q6asm_open_read(
    ac: *mut audio_client,
    stream_id: uint32_t,
    format: uint32_t,
    bits_per_sample: uint16_t,
) -> c_int {
    unsafe { __q6asm_open_read(ac, stream_id, format, bits_per_sample) }
}
// EXPORT_SYMBOL_GPL(q6asm_open_read);

/**
 * q6asm_write_async() - non blocking write
 *
 * @ac: audio client pointer
 * @stream_id: stream id
 * @len: length in bytes
 * @msw_ts: timestamp msw
 * @lsw_ts: timestamp lsw
 * @wflags: flags associated with write
 *
 * Return: Will be an negative value on error or zero on success
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn q6asm_write_async(
    ac: *mut audio_client,
    stream_id: uint32_t,
    len: uint32_t,
    msw_ts: uint32_t,
    lsw_ts: uint32_t,
    wflags: uint32_t,
) -> c_int {
    unsafe {
        let write: *mut asm_data_cmd_write_v2;
        let port: *mut audio_port_data;
        let ab: *mut audio_buffer;
        let mut flags: usize = 0;
        let pkt: *mut apr_pkt;
        let pkt_size = (APR_HDR_SIZE + size_of::<asm_data_cmd_write_v2>()) as c_int;
        let mut rc: c_int = 0;

        let p = zalloc_pkt(pkt_size, GFP_ATOMIC);
        if p.is_null() {
            return -ENOMEM;
        }

        pkt = p as *mut apr_pkt;
        write = (p as *mut u8).add(APR_HDR_SIZE) as *mut asm_data_cmd_write_v2;

        spin_lock_irqsave(&mut (*ac).lock, &mut flags);
        port = &mut (*ac).port[SNDRV_PCM_STREAM_PLAYBACK];
        q6asm_add_hdr(ac, &mut (*pkt).hdr, pkt_size as u32, false, stream_id);

        ab = (*port).buf.add((*port).dsp_buf as usize);
        (*pkt).hdr.token = (*port).dsp_buf | (len << ASM_WRITE_TOKEN_LEN_SHIFT);
        (*pkt).hdr.opcode = ASM_DATA_CMD_WRITE_V2;
        (*write).buf_addr_lsw = lower_32_bits((*ab).phys);
        (*write).buf_addr_msw = upper_32_bits((*ab).phys);
        (*write).buf_size = len;
        (*write).seq_id = (*port).dsp_buf;
        (*write).timestamp_lsw = lsw_ts;
        (*write).timestamp_msw = msw_ts;
        (*write).mem_map_handle = (*ac).port[SNDRV_PCM_STREAM_PLAYBACK].mem_map_handle;

        (*write).flags = wflags;

        (*port).dsp_buf += 1;

        if (*port).dsp_buf >= (*port).num_periods {
            (*port).dsp_buf = 0;
        }

        spin_unlock_irqrestore(&mut (*ac).lock, flags);
        rc = apr_send_pkt((*ac).adev, pkt);
        if rc == pkt_size {
            rc = 0;
        }

        kfree(p);
        rc
    }
}
// EXPORT_SYMBOL_GPL(q6asm_write_async);

unsafe fn q6asm_reset_buf_state(ac: *mut audio_client) {
    unsafe {
        let mut port: *mut audio_port_data;
        let mut flags: usize = 0;

        spin_lock_irqsave(&mut (*ac).lock, &mut flags);
        port = &mut (*ac).port[SNDRV_PCM_STREAM_PLAYBACK];
        (*port).dsp_buf = 0;
        port = &mut (*ac).port[SNDRV_PCM_STREAM_CAPTURE];
        (*port).dsp_buf = 0;
        spin_unlock_irqrestore(&mut (*ac).lock, flags);
    }
}

unsafe fn __q6asm_cmd(ac: *mut audio_client, stream_id: uint32_t, cmd: c_int, wait: bool_) -> c_int {
    unsafe {
        let mut pkt: apr_pkt = core::mem::zeroed();
        let mut rc: c_int;

        q6asm_add_hdr(ac, &mut pkt.hdr, APR_HDR_SIZE as u32, true, stream_id);

        match cmd {
            CMD_PAUSE => pkt.hdr.opcode = ASM_SESSION_CMD_PAUSE,
            CMD_SUSPEND => pkt.hdr.opcode = ASM_SESSION_CMD_SUSPEND,
            CMD_FLUSH => pkt.hdr.opcode = ASM_STREAM_CMD_FLUSH,
            CMD_OUT_FLUSH => pkt.hdr.opcode = ASM_STREAM_CMD_FLUSH_READBUFS,
            CMD_EOS => pkt.hdr.opcode = ASM_DATA_CMD_EOS,
            CMD_CLOSE => pkt.hdr.opcode = ASM_STREAM_CMD_CLOSE,
            _ => return -EINVAL,
        }

        if wait {
            rc = q6asm_ac_send_cmd_sync(ac, &mut pkt);
        } else {
            return apr_send_pkt((*ac).adev, &mut pkt);
        }

        if rc < 0 {
            return rc;
        }

        if cmd == CMD_FLUSH {
            q6asm_reset_buf_state(ac);
        }

        0
    }
}

/**
 * q6asm_cmd() - run cmd on audio client
 *
 * @ac: audio client pointer
 * @stream_id: stream id
 * @cmd: command to run on audio client.
 *
 * Return: Will be an negative value on error or zero on success
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn q6asm_cmd(ac: *mut audio_client, stream_id: uint32_t, cmd: c_int) -> c_int {
    unsafe { __q6asm_cmd(ac, stream_id, cmd, true) }
}
// EXPORT_SYMBOL_GPL(q6asm_cmd);

/**
 * q6asm_cmd_nowait() - non blocking, run cmd on audio client
 *
 * @ac: audio client pointer
 * @stream_id: stream id
 * @cmd: command to run on audio client.
 *
 * Return: Will be an negative value on error or zero on success
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn q6asm_cmd_nowait(
    ac: *mut audio_client,
    stream_id: uint32_t,
    cmd: c_int,
) -> c_int {
    unsafe { __q6asm_cmd(ac, stream_id, cmd, false) }
}
// EXPORT_SYMBOL_GPL(q6asm_cmd_nowait);

unsafe extern "C" fn q6asm_probe(adev: *mut apr_device) -> c_int {
    unsafe {
        let dev = &mut (*adev).dev as *mut device;
        let q6asm: *mut q6asm;

        q6asm = devm_kzalloc(dev, size_of::<q6asm>(), GFP_KERNEL) as *mut q6asm;
        if q6asm.is_null() {
            return -ENOMEM;
        }

        q6core_get_svc_api_info((*adev).svc_id, &mut (*q6asm).ainfo);

        (*q6asm).dev = dev;
        (*q6asm).adev = adev;
        init_waitqueue_head(&mut (*q6asm).mem_wait);
        spin_lock_init(&mut (*q6asm).slock);
        dev_set_drvdata(dev, q6asm as *mut c_void);

        devm_of_platform_populate(dev)
    }
}

// Original C only defines this table under CONFIG_OF.
static q6asm_device_id: [of_device_id; 2] = [
    of_device_id {
        compatible: c"qcom,q6asm".as_ptr(),
    },
    of_device_id {
        compatible: ptr::null(),
    },
];
// MODULE_DEVICE_TABLE(of, q6asm_device_id);

static qcom_q6asm_driver: apr_driver = apr_driver {
    probe: Some(q6asm_probe),
    callback: Some(q6asm_srvc_callback),
    driver: driver {
        name: c"qcom-q6asm".as_ptr(),
        of_match_table: unsafe { of_match_ptr(q6asm_device_id.as_ptr()) },
    },
};

// module_apr_driver(qcom_q6asm_driver);
// MODULE_DESCRIPTION("Q6 Audio Stream Manager driver");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
