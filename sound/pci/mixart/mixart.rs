// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Driver for Digigram miXart soundcards
 *
 * main file with alsa callbacks
 *
 * Copyright (c) 2003 by Digigram <alsa@digigram.com>
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem::{size_of, zeroed};
use core::ptr::{null, null_mut};

const CARD_NAME: &[u8] = b"miXart\0";

type bool_t = bool;
type u32 = u32;
type size_t = usize;
type ssize_t = isize;
type loff_t = i64;
type snd_pcm_format_t = c_int;
type snd_pcm_uframes_t = c_ulong;

#[repr(C)]
pub struct pci_dev {
    pub dev: device,
    pub irq: c_int,
}
#[repr(C)]
pub struct device {
    _private: [u8; 0],
}
#[repr(C)]
pub struct pci_device_id {
    _private: [u8; 0],
}
#[repr(C)]
pub struct file {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_card {
    pub dev: *mut device,
    pub sync_irq: c_int,
    pub driver: [c_char; 16],
    pub shortname: [c_char; 32],
    pub longname: [c_char; 80],
}
#[repr(C)]
pub struct snd_pcm {
    pub card: *mut snd_card,
    pub private_data: *mut c_void,
    pub info_flags: c_uint,
    pub nonatomic: bool,
    pub name: [c_char; 80],
}
#[repr(C)]
pub struct snd_pcm_runtime {
    pub private_data: *mut c_void,
    pub rate: c_uint,
    pub hw: snd_pcm_hardware,
    pub buffer_changed: bool,
    pub dma_addr: u32,
    pub dma_bytes: u32,
    pub period_size: snd_pcm_uframes_t,
}
#[repr(C)]
pub struct snd_pcm_substream {
    pub runtime: *mut snd_pcm_runtime,
    pub pcm: *mut snd_pcm,
    pub number: c_int,
    pub stream: c_int,
    pub next: *mut snd_pcm_substream,
}
#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_pcm_hardware {
    pub info: u32,
    pub formats: u64,
    pub rates: u32,
    pub rate_min: u32,
    pub rate_max: u32,
    pub channels_min: u32,
    pub channels_max: u32,
    pub buffer_bytes_max: usize,
    pub period_bytes_min: usize,
    pub period_bytes_max: usize,
    pub periods_min: u32,
    pub periods_max: u32,
}
#[repr(C)]
pub struct snd_pcm_ops {
    pub open: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    pub close: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    pub prepare: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    pub hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params) -> c_int>,
    pub hw_free: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    pub trigger: Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_int) -> c_int>,
    pub pointer: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> snd_pcm_uframes_t>,
}
#[repr(C)]
pub struct snd_device {
    pub device_data: *mut c_void,
}
#[repr(C)]
pub struct snd_device_ops {
    pub dev_free: Option<unsafe extern "C" fn(*mut snd_device) -> c_int>,
}
#[repr(C)]
pub struct snd_info_entry {
    pub private_data: *mut c_void,
    pub content: c_int,
    pub c: snd_info_entry_union,
    pub size: c_ulong,
}
#[repr(C)]
pub union snd_info_entry_union {
    pub ops: *const snd_info_entry_ops,
}
#[repr(C)]
pub struct snd_info_entry_ops {
    pub read: Option<unsafe extern "C" fn(*mut snd_info_entry, *mut c_void, *mut file, *mut c_char, size_t, loff_t) -> ssize_t>,
}
#[repr(C)]
pub struct snd_info_buffer {
    _private: [u8; 0],
}
#[repr(C)]
pub struct atomic_t {
    _private: [u8; 0],
}
#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}
#[repr(C)]
pub struct wait_queue_head_t {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_dma_buffer {
    pub area: *mut c_void,
    pub addr: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mixart_uid {
    pub object_id: u32,
    pub desc: u32,
}
#[repr(C)]
pub struct mixart_msg {
    pub message_id: u32,
    pub uid: mixart_uid,
    pub data: *mut c_void,
    pub size: usize,
}
#[repr(C)]
pub struct mixart_group_state_req {
    pub pipe_count: u32,
    pub pipe_uid: mixart_uid,
}
#[repr(C)]
pub struct mixart_group_state_resp {
    pub txx_status: u32,
}
#[repr(C)]
pub struct mixart_clock_properties {
    pub clock_generic_type: u32,
    pub clock_mode: u32,
    pub frequency: u32,
    pub nb_callers: u32,
    pub uid_caller: mixart_uid,
}
#[repr(C)]
pub struct mixart_clock_properties_resp {
    pub status: u32,
    pub clock_mode: u32,
}
#[repr(C)]
pub struct mixart_streaming_group_req {
    pub stream_count: c_int,
    pub channel_count: c_int,
    pub latency: c_int,
    pub connector: mixart_uid,
    pub stream_info: [mixart_stream_info; MIXART_PLAYBACK_STREAMS as usize],
    pub flow_entry: [c_int; MIXART_PLAYBACK_STREAMS as usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mixart_stream_info {
    pub size_max_byte_frame: u32,
    pub size_max_sample_frame: u32,
    pub nb_bytes_max_per_sample: u32,
}
#[repr(C)]
pub struct mixart_streaming_group {
    pub status: u32,
    pub group: mixart_uid,
    pub stream_count: c_int,
}
#[repr(C)]
pub struct mixart_flowinfo {
    pub bufferinfo_array_phy_address: u32,
    pub bufferinfo_count: u32,
}
#[repr(C)]
pub struct mixart_bufferinfo {
    pub buffer_address: u32,
    pub available_length: u32,
    pub buffer_id: u32,
}
#[repr(C)]
pub struct mixart_delete_group_resp {
    pub status: u32,
}
#[repr(C)]
pub struct mixart_stream_desc {
    pub uid_pipe: mixart_uid,
    pub stream_idx: c_int,
}
#[repr(C)]
pub struct mixart_stream_state_info {
    pub stream_desc: mixart_stream_desc,
}
#[repr(C)]
pub struct mixart_stream_state_req {
    pub stream_count: u32,
    pub stream_info: mixart_stream_state_info,
}
#[repr(C)]
pub struct mixart_stream_param_desc {
    pub coding_type: u32,
    pub number_of_channel: c_int,
    pub sampling_freq: u32,
    pub sample_type: u32,
    pub sample_size: u32,
    pub pipe_count: u32,
    pub stream_count: u32,
    pub stream_desc: mixart_stream_desc,
}
#[repr(C)]
pub struct mixart_return_uid {
    pub error_code: u32,
}
#[repr(C)]
pub struct mixart_mem {
    pub phys: c_ulong,
    pub virt: *mut c_void,
}
#[repr(C)]
pub struct mixart_mgr {
    pub pci: *mut pci_dev,
    pub irq: c_int,
    pub uid_console_manager: mixart_uid,
    pub chip: [*mut snd_mixart; MIXART_MAX_CARDS as usize],
    pub num_cards: c_uint,
    pub dsp_loaded: u32,
    pub board_type: c_int,
    pub flowinfo: snd_dma_buffer,
    pub bufferinfo: snd_dma_buffer,
    pub mem: [mixart_mem; 2],
    pub ref_count_rate: c_int,
    pub sample_rate: c_uint,
    pub msg_fifo_readptr: c_int,
    pub msg_fifo_writeptr: c_int,
    pub lock: mutex,
    pub msg_lock: mutex,
    pub msg_sleep: wait_queue_head_t,
    pub msg_processed: atomic_t,
    pub setup_mutex: mutex,
}
#[repr(C)]
pub struct mixart_pipe {
    pub status: c_int,
    pub references: c_int,
    pub monitoring: c_int,
    pub group_uid: mixart_uid,
    pub stream_count: c_int,
    pub uid_left_connector: mixart_uid,
}
#[repr(C)]
pub struct mixart_stream {
    pub pipe: *mut mixart_pipe,
    pub pcm_number: c_int,
    pub status: c_int,
    pub substream: *mut snd_pcm_substream,
    pub channels: c_int,
    pub abs_period_elapsed: c_int,
    pub buf_periods: snd_pcm_uframes_t,
    pub buf_period_frag: snd_pcm_uframes_t,
}
#[repr(C)]
pub struct snd_mixart {
    pub card: *mut snd_card,
    pub chip_idx: c_int,
    pub mgr: *mut mixart_mgr,
    pub pipe_in_ana: mixart_pipe,
    pub pipe_in_dig: mixart_pipe,
    pub pipe_out_ana: mixart_pipe,
    pub pipe_out_dig: mixart_pipe,
    pub playback_stream: [[mixart_stream; MIXART_PLAYBACK_STREAMS as usize]; 2],
    pub capture_stream: [mixart_stream; 2],
    pub pcm: *mut snd_pcm,
    pub pcm_dig: *mut snd_pcm,
}

const SNDRV_CARDS: usize = 8;
const SNDRV_DEFAULT_IDX: c_int = -1;
const SNDRV_DEFAULT_ENABLE_PNP: bool = true;
static mut index: [c_int; SNDRV_CARDS] = [SNDRV_DEFAULT_IDX; SNDRV_CARDS];
static mut id: [*mut c_char; SNDRV_CARDS] = [null_mut(); SNDRV_CARDS];
static mut enable: [bool_t; SNDRV_CARDS] = [SNDRV_DEFAULT_ENABLE_PNP; SNDRV_CARDS];

const EINVAL: c_int = 22;
const EBUSY: c_int = 16;
const ENOMEM: c_int = 12;
const ENODEV: c_int = 19;
const ENOENT: c_int = 2;
const ENXIO: c_int = 6;
const EFAULT: c_int = 14;
const HZ: c_ulong = 100;
const IRQF_SHARED: c_ulong = 0x80;
const SNDRV_PCM_STREAM_PLAYBACK: c_int = 0;
const SNDRV_PCM_STREAM_CAPTURE: c_int = 1;
const SNDRV_PCM_TRIGGER_START: c_int = 0;
const SNDRV_PCM_TRIGGER_STOP: c_int = 1;
const SNDRV_PCM_TRIGGER_PAUSE_PUSH: c_int = 3;
const SNDRV_PCM_TRIGGER_PAUSE_RELEASE: c_int = 4;
const SNDRV_PCM_FORMAT_U8: snd_pcm_format_t = 0;
const SNDRV_PCM_FORMAT_S16_LE: snd_pcm_format_t = 2;
const SNDRV_PCM_FORMAT_S16_BE: snd_pcm_format_t = 3;
const SNDRV_PCM_FORMAT_S24_3LE: snd_pcm_format_t = 6;
const SNDRV_PCM_FORMAT_S24_3BE: snd_pcm_format_t = 7;
const SNDRV_PCM_FORMAT_FLOAT_LE: snd_pcm_format_t = 14;
const SNDRV_PCM_FORMAT_FLOAT_BE: snd_pcm_format_t = 15;
const SNDRV_PCM_INFO_MMAP: u32 = 1 << 0;
const SNDRV_PCM_INFO_INTERLEAVED: u32 = 1 << 1;
const SNDRV_PCM_INFO_MMAP_VALID: u32 = 1 << 2;
const SNDRV_PCM_INFO_PAUSE: u32 = 1 << 3;
const SNDRV_PCM_FMTBIT_U8: u64 = 1 << SNDRV_PCM_FORMAT_U8;
const SNDRV_PCM_FMTBIT_S16_LE: u64 = 1 << SNDRV_PCM_FORMAT_S16_LE;
const SNDRV_PCM_FMTBIT_S16_BE: u64 = 1 << SNDRV_PCM_FORMAT_S16_BE;
const SNDRV_PCM_FMTBIT_S24_3LE: u64 = 1 << SNDRV_PCM_FORMAT_S24_3LE;
const SNDRV_PCM_FMTBIT_S24_3BE: u64 = 1 << SNDRV_PCM_FORMAT_S24_3BE;
const SNDRV_PCM_FMTBIT_FLOAT_LE: u64 = 1 << SNDRV_PCM_FORMAT_FLOAT_LE;
const SNDRV_PCM_FMTBIT_FLOAT_BE: u64 = 1 << SNDRV_PCM_FORMAT_FLOAT_BE;
const SNDRV_PCM_RATE_CONTINUOUS: u32 = 1 << 30;
const SNDRV_PCM_RATE_8000_48000: u32 = 0x0000_ffff;
const SNDRV_PCM_RATE_32000: u32 = 1 << 5;
const SNDRV_PCM_RATE_44100: u32 = 1 << 6;
const SNDRV_PCM_RATE_48000: u32 = 1 << 7;
const SNDRV_PCM_HW_PARAM_PERIOD_BYTES: c_int = 0;
const SNDRV_PCM_HW_PARAM_BUFFER_SIZE: c_int = 1;
const SNDRV_DMA_TYPE_DEV: c_int = 0;
const SNDRV_DEV_LOWLEVEL: c_int = 0;
const SNDRV_INFO_CONTENT_DATA: c_int = 1;

const MIXART_PCM_ANALOG: c_int = 0;
const MIXART_PCM_DIGITAL: c_int = 1;
const MIXART_PLAYBACK_STREAMS: c_int = 4;
const MIXART_CAPTURE_STREAMS: c_int = 1;
const MIXART_MAX_STREAM_PER_CARD: c_int = 10;
const MIXART_MAX_CARDS: c_int = 4;
const MIXART_FLOAT_P__4_0_TO_HEX: u32 = 0x4080_0000;
const MIXART_NOTIFY_CARD_OFFSET: c_int = 24;
const MIXART_NOTIFY_PCM_OFFSET: c_int = 16;
const MIXART_NOTIFY_CAPT_MASK: u32 = 0x8000_0000;
const PIPE_RUNNING: c_int = 0;
const PIPE_CLOCK_SET: c_int = 1;
const PIPE_STOPPED: c_int = 2;
const PIPE_UNDEFINED: c_int = 3;
const MIXART_STREAM_STATUS_FREE: c_int = 0;
const MIXART_STREAM_STATUS_OPEN: c_int = 1;
const MIXART_STREAM_STATUS_RUNNING: c_int = 2;
const MIXART_STREAM_STATUS_PAUSE: c_int = 3;
const MIXART_DAUGHTER_TYPE_NONE: c_int = 0;
const MIXART_DAUGHTER_TYPE_AES: c_int = 1;
const MIXART_DAUGHTER_TYPE_COBRANET: c_int = 2;
const MIXART_MOTHERBOARD_ELF_INDEX: c_int = 0;
const MIXART_BA0_SIZE: c_ulong = 0;
const MIXART_BA1_SIZE: c_ulong = 0;
const MIXART_PSEUDOREG_PERF_SYSTEM_LOAD_OFFSET: loff_t = 0;
const MIXART_PSEUDOREG_PERF_MAILBX_LOAD_OFFSET: loff_t = 0;
const MIXART_PSEUDOREG_PERF_STREAM_LOAD_OFFSET: loff_t = 0;
const MIXART_PSEUDOREG_PERF_INTERR_LOAD_OFFSET: loff_t = 0;
const CGT_INTERNAL_CLOCK: u32 = 0;
const CGT_NO_CLOCK: u32 = 1;
const CM_STANDALONE: u32 = 0;
const CT_LINEAR: u32 = 0;
const ST_INTEGER_8: u32 = 0;
const ST_INTEGER_16LE: u32 = 1;
const ST_INTEGER_16BE: u32 = 2;
const ST_INTEGER_24LE: u32 = 3;
const ST_INTEGER_24BE: u32 = 4;
const ST_FLOATING_POINT_32LE: u32 = 5;
const ST_FLOATING_POINT_32BE: u32 = 6;
const MSG_SYSTEM_WAIT_SYNCHRO_CMD: u32 = 0;
const MSG_STREAM_START_STREAM_GRP_PACKET: u32 = 0;
const MSG_STREAM_STOP_STREAM_GRP_PACKET: u32 = 0;
const MSG_SYSTEM_SEND_SYNCHRO_CMD: u32 = 0;
const MSG_CLOCK_SET_PROPERTIES: u32 = 0;
const MSG_STREAM_ADD_OUTPUT_GROUP: u32 = 0;
const MSG_STREAM_ADD_INPUT_GROUP: u32 = 0;
const MSG_STREAM_DELETE_GROUP: u32 = 0;
const MSG_STREAM_START_INPUT_STAGE_PACKET: u32 = 0;
const MSG_STREAM_STOP_INPUT_STAGE_PACKET: u32 = 0;
const MSG_STREAM_START_OUTPUT_STAGE_PACKET: u32 = 0;
const MSG_STREAM_STOP_OUTPUT_STAGE_PACKET: u32 = 0;
const MSG_STREAM_SET_INPUT_STAGE_PARAM: u32 = 0;

unsafe extern "C" {
    static mut jiffies: c_ulong;
    static THIS_MODULE: c_void;
    static KBUILD_MODNAME: c_char;

    fn snd_mixart_send_msg_wait_notif(mgr: *mut mixart_mgr, request: *mut mixart_msg, notif: u32) -> c_int;
    fn snd_mixart_send_msg(mgr: *mut mixart_mgr, request: *mut mixart_msg, resp_size: usize, resp: *mut c_void) -> c_int;
    fn snd_mixart_send_msg_nonblock(mgr: *mut mixart_mgr, request: *mut mixart_msg) -> c_int;
    fn snd_mixart_interrupt(irq: c_int, dev_id: *mut c_void) -> c_int;
    fn snd_mixart_threaded_irq(irq: c_int, dev_id: *mut c_void) -> c_int;
    fn snd_mixart_exit_mailbox(mgr: *mut mixart_mgr);
    fn snd_mixart_reset_board(mgr: *mut mixart_mgr);
    fn snd_mixart_setup_firmware(mgr: *mut mixart_mgr) -> c_int;
    fn mixart_update_playback_stream_level(chip: *mut snd_mixart, is_aes: c_int, idx: c_int);
    fn mixart_update_capture_stream_level(chip: *mut snd_mixart, is_aes: c_int);

    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn kmalloc(size: usize, flags: c_uint) -> *mut c_void;
    fn kzalloc(size: usize, flags: c_uint) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn memset(ptr: *mut c_void, val: c_int, size: usize) -> *mut c_void;
    fn snd_pcm_substream_chip(subs: *mut snd_pcm_substream) -> *mut snd_mixart;
    fn atomic_read(v: *const atomic_t) -> c_int;
    fn schedule_timeout_uninterruptible(timeout: c_long);
    fn time_after(a: c_ulong, b: c_ulong) -> bool;
    fn params_channels(hw: *mut snd_pcm_hw_params) -> c_int;
    fn params_format(hw: *mut snd_pcm_hw_params) -> snd_pcm_format_t;
    fn snd_pcm_hw_constraint_step(runtime: *mut snd_pcm_runtime, cond: c_uint, var: c_int, step: c_ulong) -> c_int;
    fn snd_pcm_set_managed_buffer_all(pcm: *mut snd_pcm, ty: c_int, dev: *mut device, min: usize, max: usize);
    fn snd_pcm_new(card: *mut snd_card, id: *const c_char, device: c_int, playback_count: c_int, capture_count: c_int, rpcm: *mut *mut snd_pcm) -> c_int;
    fn snd_pcm_set_ops(pcm: *mut snd_pcm, direction: c_int, ops: *const snd_pcm_ops);
    fn snd_device_new(card: *mut snd_card, ty: c_int, data: *mut c_void, ops: *const snd_device_ops) -> c_int;
    fn snd_card_free(card: *mut snd_card);
    fn snd_card_new(parent: *mut device, idx: c_int, xid: *const c_char, module: *const c_void, extra_size: c_int, card_ret: *mut *mut snd_card) -> c_int;
    fn snd_card_register(card: *mut snd_card) -> c_int;
    fn snd_card_ro_proc_new(card: *mut snd_card, name: *const c_char, data: *mut c_void, read: unsafe extern "C" fn(*mut snd_info_entry, *mut snd_info_buffer));
    fn snd_card_proc_new(card: *mut snd_card, name: *const c_char, entryp: *mut *mut snd_info_entry) -> c_int;
    fn snd_iprintf(buffer: *mut snd_info_buffer, fmt: *const c_char, ...);
    fn copy_to_user_fromio(dst: *mut c_char, src: *const c_void, count: size_t) -> c_int;
    fn readl_be(addr: *const c_void) -> u32;
    fn pci_enable_device(pci: *mut pci_dev) -> c_int;
    fn pci_disable_device(pci: *mut pci_dev);
    fn pci_set_master(pci: *mut pci_dev);
    fn dma_set_mask(dev: *mut device, mask: u64) -> c_int;
    fn pci_request_regions(pci: *mut pci_dev, name: *const u8) -> c_int;
    fn pci_release_regions(pci: *mut pci_dev);
    fn pci_resource_start(pci: *mut pci_dev, bar: c_uint) -> c_ulong;
    fn pci_ioremap_bar(pci: *mut pci_dev, bar: c_uint) -> *mut c_void;
    fn iounmap(addr: *mut c_void);
    fn request_threaded_irq(irq: c_int, handler: unsafe extern "C" fn(c_int, *mut c_void) -> c_int, thread_fn: unsafe extern "C" fn(c_int, *mut c_void) -> c_int, flags: c_ulong, name: *const c_char, dev: *mut c_void) -> c_int;
    fn free_irq(irq: c_int, dev_id: *mut c_void);
    fn mutex_init(mutex: *mut mutex);
    fn init_waitqueue_head(wq: *mut wait_queue_head_t);
    fn atomic_set(v: *mut atomic_t, i: c_int);
    fn snd_dma_alloc_pages(ty: c_int, dev: *mut device, size: usize, dmab: *mut snd_dma_buffer) -> c_int;
    fn snd_dma_free_pages(dmab: *mut snd_dma_buffer);
    fn pci_set_drvdata(pci: *mut pci_dev, data: *mut c_void);
    fn pci_get_drvdata(pci: *mut pci_dev) -> *mut c_void;
    fn sprintf(buf: *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn snprintf(buf: *mut c_char, size: usize, fmt: *const c_char, ...) -> c_int;
    fn strscpy(dst: *mut c_char, src: *const c_char) -> isize;
}

unsafe fn MIXART_MEM(mgr: *mut mixart_mgr, pos: loff_t) -> *const c_void {
    ((*mgr).mem[0].virt as *const u8).offset(pos as isize) as *const c_void
}
unsafe fn MIXART_REG(mgr: *mut mixart_mgr, pos: loff_t) -> *const c_void {
    ((*mgr).mem[1].virt as *const u8).offset(pos as isize) as *const c_void
}
fn DMA_BIT_MASK(n: u32) -> u64 {
    if n == 64 { !0 } else { (1u64 << n) - 1 }
}
fn PAGE_ALIGN(x: usize) -> usize {
    (x + 4095) & !4095
}
unsafe fn guard_mutex(_m: *mut mutex) {
    /* C used guard(mutex)(&mgr->setup_mutex); unlock is scope based in the kernel macro. */
}

unsafe extern "C" fn mixart_set_pipe_state(mgr: *mut mixart_mgr, pipe: *mut mixart_pipe, start: c_int) -> c_int {
    let mut group_state: mixart_group_state_req = zeroed();
    let mut group_state_resp: mixart_group_state_resp = zeroed();
    let mut request: mixart_msg = zeroed();
    let mut err: c_int;
    let mut system_msg_uid: u32;

    match (*pipe).status {
        PIPE_RUNNING | PIPE_CLOCK_SET => {
            if start != 0 { return 0; }
        }
        PIPE_STOPPED => {
            if start == 0 { return 0; }
        }
        _ => {
            dev_err(&mut (*(*mgr).pci).dev, b"error mixart_set_pipe_state called with wrong pipe->status!\n\0".as_ptr() as *const c_char);
            return -EINVAL;
        }
    }

    system_msg_uid = 0x12345678;
    request.message_id = MSG_SYSTEM_WAIT_SYNCHRO_CMD;
    request.uid = mixart_uid { object_id: 0, desc: 0 };
    request.data = &mut system_msg_uid as *mut _ as *mut c_void;
    request.size = size_of::<u32>();
    err = snd_mixart_send_msg_wait_notif(mgr, &mut request, system_msg_uid);
    if err != 0 {
        dev_err(&mut (*(*mgr).pci).dev, b"error : MSG_SYSTEM_WAIT_SYNCHRO_CMD was not notified !\n\0".as_ptr() as *const c_char);
        return err;
    }

    memset(&mut group_state as *mut _ as *mut c_void, 0, size_of::<mixart_group_state_req>());
    group_state.pipe_count = 1;
    group_state.pipe_uid = (*pipe).group_uid;
    request.message_id = if start != 0 { MSG_STREAM_START_STREAM_GRP_PACKET } else { MSG_STREAM_STOP_STREAM_GRP_PACKET };
    request.uid = (*pipe).group_uid;
    request.data = &mut group_state as *mut _ as *mut c_void;
    request.size = size_of::<mixart_group_state_req>();
    err = snd_mixart_send_msg(mgr, &mut request, size_of::<mixart_group_state_resp>(), &mut group_state_resp as *mut _ as *mut c_void);
    if err < 0 || group_state_resp.txx_status != 0 {
        dev_err(&mut (*(*mgr).pci).dev, b"error MSG_STREAM_ST***_STREAM_GRP_PACKET err=%x stat=%x !\n\0".as_ptr() as *const c_char, err, group_state_resp.txx_status);
        return -EINVAL;
    }

    if start != 0 {
        let mut stat: u32 = 0;
        group_state.pipe_count = 0;
        err = snd_mixart_send_msg(mgr, &mut request, size_of::<mixart_group_state_resp>(), &mut group_state_resp as *mut _ as *mut c_void);
        if err < 0 || group_state_resp.txx_status != 0 {
            dev_err(&mut (*(*mgr).pci).dev, b"error MSG_STREAM_START_STREAM_GRP_PACKET err=%x stat=%x !\n\0".as_ptr() as *const c_char, err, group_state_resp.txx_status);
            return -EINVAL;
        }
        request.message_id = MSG_SYSTEM_SEND_SYNCHRO_CMD;
        request.uid = mixart_uid { object_id: 0, desc: 0 };
        request.data = null_mut();
        request.size = 0;
        err = snd_mixart_send_msg(mgr, &mut request, size_of::<u32>(), &mut stat as *mut _ as *mut c_void);
        if err < 0 || stat != 0 {
            dev_err(&mut (*(*mgr).pci).dev, b"error MSG_SYSTEM_SEND_SYNCHRO_CMD err=%x stat=%x !\n\0".as_ptr() as *const c_char, err, stat);
            return -EINVAL;
        }
        (*pipe).status = PIPE_RUNNING;
    } else {
        (*pipe).status = PIPE_STOPPED;
    }
    0
}

unsafe extern "C" fn mixart_set_clock(mgr: *mut mixart_mgr, pipe: *mut mixart_pipe, rate: c_uint) -> c_int {
    let mut request: mixart_msg = zeroed();
    let mut clock_properties: mixart_clock_properties = zeroed();
    let mut clock_prop_resp: mixart_clock_properties_resp = zeroed();
    let err: c_int;

    match (*pipe).status {
        PIPE_CLOCK_SET => {}
        PIPE_RUNNING => {
            if rate == 0 {
                return 0;
            }
        }
        _ => {
            if rate == 0 {
                return 0;
            } else {
                dev_err(&mut (*(*mgr).pci).dev, b"error mixart_set_clock(%d) called with wrong pipe->status !\n\0".as_ptr() as *const c_char, rate);
                return -EINVAL;
            }
        }
    }

    memset(&mut clock_properties as *mut _ as *mut c_void, 0, size_of::<mixart_clock_properties>());
    clock_properties.clock_generic_type = if rate != 0 { CGT_INTERNAL_CLOCK } else { CGT_NO_CLOCK };
    clock_properties.clock_mode = CM_STANDALONE;
    clock_properties.frequency = rate;
    clock_properties.nb_callers = 1;
    clock_properties.uid_caller = (*pipe).group_uid;
    dev_dbg(&mut (*(*mgr).pci).dev, b"mixart_set_clock to %d kHz\n\0".as_ptr() as *const c_char, rate);
    request.message_id = MSG_CLOCK_SET_PROPERTIES;
    request.uid = (*mgr).uid_console_manager;
    request.data = &mut clock_properties as *mut _ as *mut c_void;
    request.size = size_of::<mixart_clock_properties>();
    err = snd_mixart_send_msg(mgr, &mut request, size_of::<mixart_clock_properties_resp>(), &mut clock_prop_resp as *mut _ as *mut c_void);
    if err < 0 || clock_prop_resp.status != 0 || clock_prop_resp.clock_mode != CM_STANDALONE {
        dev_err(&mut (*(*mgr).pci).dev, b"error MSG_CLOCK_SET_PROPERTIES err=%x stat=%x mod=%x !\n\0".as_ptr() as *const c_char, err, clock_prop_resp.status, clock_prop_resp.clock_mode);
        return -EINVAL;
    }
    (*pipe).status = if rate != 0 { PIPE_CLOCK_SET } else { PIPE_RUNNING };
    0
}

#[repr(C)]
struct add_ref_pipe_buf {
    sgroup_req: mixart_streaming_group_req,
    sgroup_resp: mixart_streaming_group,
}

pub unsafe extern "C" fn snd_mixart_add_ref_pipe(chip: *mut snd_mixart, pcm_number: c_int, capture: c_int, monitoring: c_int) -> *mut mixart_pipe {
    let stream_count: c_int;
    let pipe: *mut mixart_pipe;
    let mut request: mixart_msg = zeroed();

    if capture != 0 {
        pipe = if pcm_number == MIXART_PCM_ANALOG { &mut (*chip).pipe_in_ana } else { &mut (*chip).pipe_in_dig };
        request.message_id = MSG_STREAM_ADD_OUTPUT_GROUP;
        stream_count = MIXART_CAPTURE_STREAMS;
    } else {
        pipe = if pcm_number == MIXART_PCM_ANALOG { &mut (*chip).pipe_out_ana } else { &mut (*chip).pipe_out_dig };
        request.message_id = MSG_STREAM_ADD_INPUT_GROUP;
        stream_count = MIXART_PLAYBACK_STREAMS;
    }
    if monitoring == 0 && (*pipe).references >= stream_count {
        return null_mut();
    }
    if (*pipe).status == PIPE_UNDEFINED {
        let mut i: c_int;
        dev_dbg((*(*chip).card).dev, b"add_ref_pipe audio chip(%d) pcm(%d)\n\0".as_ptr() as *const c_char, (*chip).chip_idx, pcm_number);
        let buf = kmalloc(size_of::<add_ref_pipe_buf>(), 0) as *mut add_ref_pipe_buf;
        if buf.is_null() { return null_mut(); }
        request.uid = mixart_uid { object_id: 0, desc: 0 };
        request.data = &mut (*buf).sgroup_req as *mut _ as *mut c_void;
        request.size = size_of::<mixart_streaming_group_req>();
        memset(&mut (*buf).sgroup_req as *mut _ as *mut c_void, 0, size_of::<mixart_streaming_group_req>());
        (*buf).sgroup_req.stream_count = stream_count;
        (*buf).sgroup_req.channel_count = 2;
        (*buf).sgroup_req.latency = 256;
        (*buf).sgroup_req.connector = (*pipe).uid_left_connector;
        i = 0;
        while i < stream_count {
            let mut j: c_int;
            (*buf).sgroup_req.stream_info[i as usize].size_max_byte_frame = 1024;
            (*buf).sgroup_req.stream_info[i as usize].size_max_sample_frame = 256;
            (*buf).sgroup_req.stream_info[i as usize].nb_bytes_max_per_sample = MIXART_FLOAT_P__4_0_TO_HEX;
            j = ((*chip).chip_idx * MIXART_MAX_STREAM_PER_CARD) + (pcm_number * (MIXART_PLAYBACK_STREAMS + MIXART_CAPTURE_STREAMS)) + i;
            if capture != 0 { j += MIXART_PLAYBACK_STREAMS; }
            (*buf).sgroup_req.flow_entry[i as usize] = j;
            let flowinfo = (*(*chip).mgr).flowinfo.area as *mut mixart_flowinfo;
            (*flowinfo.add(j as usize)).bufferinfo_array_phy_address = (*(*chip).mgr).bufferinfo.addr.wrapping_add((j as usize * size_of::<mixart_bufferinfo>()) as u32);
            (*flowinfo.add(j as usize)).bufferinfo_count = 1;
            let bufferinfo = (*(*chip).mgr).bufferinfo.area as *mut mixart_bufferinfo;
            (*bufferinfo.add(j as usize)).buffer_address = 0;
            (*bufferinfo.add(j as usize)).available_length = 0;
            (*bufferinfo.add(j as usize)).buffer_id = (((*chip).chip_idx << MIXART_NOTIFY_CARD_OFFSET) + (pcm_number << MIXART_NOTIFY_PCM_OFFSET) + i) as u32;
            if capture != 0 {
                (*bufferinfo.add(j as usize)).buffer_id |= MIXART_NOTIFY_CAPT_MASK;
            }
            i += 1;
        }
        let err = snd_mixart_send_msg((*chip).mgr, &mut request, size_of::<mixart_streaming_group>(), &mut (*buf).sgroup_resp as *mut _ as *mut c_void);
        if err < 0 || (*buf).sgroup_resp.status != 0 {
            dev_err((*(*chip).card).dev, b"error MSG_STREAM_ADD_**PUT_GROUP err=%x stat=%x !\n\0".as_ptr() as *const c_char, err, (*buf).sgroup_resp.status);
            kfree(buf as *mut c_void);
            return null_mut();
        }
        (*pipe).group_uid = (*buf).sgroup_resp.group;
        (*pipe).stream_count = (*buf).sgroup_resp.stream_count;
        (*pipe).status = PIPE_STOPPED;
        kfree(buf as *mut c_void);
    }
    if monitoring != 0 { (*pipe).monitoring = 1; } else { (*pipe).references += 1; }
    pipe
}

pub unsafe extern "C" fn snd_mixart_kill_ref_pipe(mgr: *mut mixart_mgr, pipe: *mut mixart_pipe, monitoring: c_int) -> c_int {
    let mut err: c_int = 0;
    if (*pipe).status == PIPE_UNDEFINED { return 0; }
    if monitoring != 0 { (*pipe).monitoring = 0; } else { (*pipe).references -= 1; }
    if (*pipe).references <= 0 && (*pipe).monitoring == 0 {
        let mut request: mixart_msg = zeroed();
        let mut delete_resp: mixart_delete_group_resp = zeroed();
        err = mixart_set_clock(mgr, pipe, 0);
        if err < 0 { dev_err(&mut (*(*mgr).pci).dev, b"mixart_set_clock(0) return error!\n\0".as_ptr() as *const c_char); }
        err = mixart_set_pipe_state(mgr, pipe, 0);
        if err < 0 { dev_err(&mut (*(*mgr).pci).dev, b"error stopping pipe!\n\0".as_ptr() as *const c_char); }
        request.message_id = MSG_STREAM_DELETE_GROUP;
        request.uid = mixart_uid { object_id: 0, desc: 0 };
        request.data = &mut (*pipe).group_uid as *mut _ as *mut c_void;
        request.size = size_of::<mixart_uid>();
        err = snd_mixart_send_msg(mgr, &mut request, size_of::<mixart_delete_group_resp>(), &mut delete_resp as *mut _ as *mut c_void);
        if err < 0 || delete_resp.status != 0 {
            dev_err(&mut (*(*mgr).pci).dev, b"error MSG_STREAM_DELETE_GROUP err(%x), status(%x)\n\0".as_ptr() as *const c_char, err, delete_resp.status);
        }
        (*pipe).group_uid = mixart_uid { object_id: 0, desc: 0 };
        (*pipe).stream_count = 0;
        (*pipe).status = PIPE_UNDEFINED;
    }
    err
}

unsafe extern "C" fn mixart_set_stream_state(stream: *mut mixart_stream, start: c_int) -> c_int {
    let mut stream_state_req: mixart_stream_state_req = zeroed();
    let mut request: mixart_msg = zeroed();
    if (*stream).substream.is_null() { return -EINVAL; }
    memset(&mut stream_state_req as *mut _ as *mut c_void, 0, size_of::<mixart_stream_state_req>());
    stream_state_req.stream_count = 1;
    stream_state_req.stream_info.stream_desc.uid_pipe = (*(*stream).pipe).group_uid;
    stream_state_req.stream_info.stream_desc.stream_idx = (*(*stream).substream).number;
    request.message_id = if (*(*stream).substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        if start != 0 { MSG_STREAM_START_INPUT_STAGE_PACKET } else { MSG_STREAM_STOP_INPUT_STAGE_PACKET }
    } else if start != 0 { MSG_STREAM_START_OUTPUT_STAGE_PACKET } else { MSG_STREAM_STOP_OUTPUT_STAGE_PACKET };
    request.uid = mixart_uid { object_id: 0, desc: 0 };
    request.data = &mut stream_state_req as *mut _ as *mut c_void;
    request.size = size_of::<mixart_stream_state_req>();
    (*stream).abs_period_elapsed = 0;
    (*stream).buf_periods = 0;
    (*stream).buf_period_frag = 0;
    let chip = snd_pcm_substream_chip((*stream).substream);
    snd_mixart_send_msg_nonblock((*chip).mgr, &mut request)
}

unsafe extern "C" fn snd_mixart_trigger(subs: *mut snd_pcm_substream, cmd: c_int) -> c_int {
    let stream = (*(*subs).runtime).private_data as *mut mixart_stream;
    match cmd {
        SNDRV_PCM_TRIGGER_START => {
            dev_dbg((*(*(*subs).pcm).card).dev, b"SNDRV_PCM_TRIGGER_START\n\0".as_ptr() as *const c_char);
            if mixart_set_stream_state(stream, 1) != 0 { return -EINVAL; }
            (*stream).status = MIXART_STREAM_STATUS_RUNNING;
        }
        SNDRV_PCM_TRIGGER_STOP => {
            if mixart_set_stream_state(stream, 0) != 0 { return -EINVAL; }
            (*stream).status = MIXART_STREAM_STATUS_OPEN;
            dev_dbg((*(*(*subs).pcm).card).dev, b"SNDRV_PCM_TRIGGER_STOP\n\0".as_ptr() as *const c_char);
        }
        SNDRV_PCM_TRIGGER_PAUSE_PUSH => {
            (*stream).status = MIXART_STREAM_STATUS_PAUSE;
            dev_dbg((*(*(*subs).pcm).card).dev, b"SNDRV_PCM_PAUSE_PUSH\n\0".as_ptr() as *const c_char);
        }
        SNDRV_PCM_TRIGGER_PAUSE_RELEASE => {
            (*stream).status = MIXART_STREAM_STATUS_RUNNING;
            dev_dbg((*(*(*subs).pcm).card).dev, b"SNDRV_PCM_PAUSE_RELEASE\n\0".as_ptr() as *const c_char);
        }
        _ => return -EINVAL,
    }
    0
}

unsafe extern "C" fn mixart_sync_nonblock_events(mgr: *mut mixart_mgr) -> c_int {
    let timeout = jiffies + HZ;
    while atomic_read(&(*mgr).msg_processed) > 0 {
        if time_after(jiffies, timeout) {
            dev_err(&mut (*(*mgr).pci).dev, b"mixart: cannot process nonblock events!\n\0".as_ptr() as *const c_char);
            return -EBUSY;
        }
        schedule_timeout_uninterruptible(1);
    }
    0
}

unsafe extern "C" fn snd_mixart_prepare(subs: *mut snd_pcm_substream) -> c_int {
    let chip = snd_pcm_substream_chip(subs);
    let stream = (*(*subs).runtime).private_data as *mut mixart_stream;
    dev_dbg((*chip).card.as_ref().unwrap().dev, b"snd_mixart_prepare\n\0".as_ptr() as *const c_char);
    mixart_sync_nonblock_events((*chip).mgr);
    if (*(*chip).mgr).ref_count_rate == 1 {
        (*(*chip).mgr).sample_rate = (*(*subs).runtime).rate;
    }
    if (*(*stream).pipe).references == 1 {
        if mixart_set_clock((*chip).mgr, (*stream).pipe, (*(*subs).runtime).rate) != 0 { return -EINVAL; }
    }
    0
}

unsafe extern "C" fn mixart_set_format(stream: *mut mixart_stream, format: snd_pcm_format_t) -> c_int {
    let chip = snd_pcm_substream_chip((*stream).substream);
    let mut request: mixart_msg = zeroed();
    let mut stream_param: mixart_stream_param_desc = zeroed();
    let mut resp: mixart_return_uid = zeroed();
    stream_param.coding_type = CT_LINEAR;
    stream_param.number_of_channel = (*stream).channels;
    stream_param.sampling_freq = (*(*chip).mgr).sample_rate;
    if stream_param.sampling_freq == 0 { stream_param.sampling_freq = 44100; }
    match format {
        SNDRV_PCM_FORMAT_U8 => { stream_param.sample_type = ST_INTEGER_8; stream_param.sample_size = 8; }
        SNDRV_PCM_FORMAT_S16_LE => { stream_param.sample_type = ST_INTEGER_16LE; stream_param.sample_size = 16; }
        SNDRV_PCM_FORMAT_S16_BE => { stream_param.sample_type = ST_INTEGER_16BE; stream_param.sample_size = 16; }
        SNDRV_PCM_FORMAT_S24_3LE => { stream_param.sample_type = ST_INTEGER_24LE; stream_param.sample_size = 24; }
        SNDRV_PCM_FORMAT_S24_3BE => { stream_param.sample_type = ST_INTEGER_24BE; stream_param.sample_size = 24; }
        SNDRV_PCM_FORMAT_FLOAT_LE => { stream_param.sample_type = ST_FLOATING_POINT_32LE; stream_param.sample_size = 32; }
        SNDRV_PCM_FORMAT_FLOAT_BE => { stream_param.sample_type = ST_FLOATING_POINT_32BE; stream_param.sample_size = 32; }
        _ => {
            dev_err((*(*chip).card).dev, b"error mixart_set_format() : unknown format\n\0".as_ptr() as *const c_char);
            return -EINVAL;
        }
    }
    dev_dbg((*(*chip).card).dev, b"set SNDRV_PCM_FORMAT sample_type(%d) sample_size(%d) freq(%d) channels(%d)\n\0".as_ptr() as *const c_char, stream_param.sample_type, stream_param.sample_size, stream_param.sampling_freq, (*stream).channels);
    stream_param.pipe_count = 1;
    stream_param.stream_count = 1;
    stream_param.stream_desc.uid_pipe = (*(*stream).pipe).group_uid;
    stream_param.stream_desc.stream_idx = (*(*stream).substream).number;
    request.message_id = MSG_STREAM_SET_INPUT_STAGE_PARAM;
    request.uid = mixart_uid { object_id: 0, desc: 0 };
    request.data = &mut stream_param as *mut _ as *mut c_void;
    request.size = size_of::<mixart_stream_param_desc>();
    let err = snd_mixart_send_msg((*chip).mgr, &mut request, size_of::<mixart_return_uid>(), &mut resp as *mut _ as *mut c_void);
    if err < 0 || resp.error_code != 0 {
        dev_err((*(*chip).card).dev, b"MSG_STREAM_SET_INPUT_STAGE_PARAM err=%x; resp=%x\n\0".as_ptr() as *const c_char, err, resp.error_code);
        return -EINVAL;
    }
    0
}

unsafe extern "C" fn snd_mixart_hw_params(subs: *mut snd_pcm_substream, hw: *mut snd_pcm_hw_params) -> c_int {
    let chip = snd_pcm_substream_chip(subs);
    let mgr = (*chip).mgr;
    let stream = (*(*subs).runtime).private_data as *mut mixart_stream;
    let channels = params_channels(hw);
    let format = params_format(hw);
    guard_mutex(&mut (*mgr).setup_mutex);
    if (*stream).pcm_number <= MIXART_PCM_DIGITAL {
        let is_aes = ((*stream).pcm_number > MIXART_PCM_ANALOG) as c_int;
        if (*subs).stream == SNDRV_PCM_STREAM_PLAYBACK {
            mixart_update_playback_stream_level(chip, is_aes, (*subs).number);
        } else {
            mixart_update_capture_stream_level(chip, is_aes);
        }
    }
    (*stream).channels = channels;
    let err = mixart_set_format(stream, format);
    if err < 0 { return err; }
    if (*(*subs).runtime).buffer_changed {
        let mut i = ((*chip).chip_idx * MIXART_MAX_STREAM_PER_CARD) + ((*stream).pcm_number * (MIXART_PLAYBACK_STREAMS + MIXART_CAPTURE_STREAMS)) + (*subs).number;
        if (*subs).stream == SNDRV_PCM_STREAM_CAPTURE { i += MIXART_PLAYBACK_STREAMS; }
        let bufferinfo = (*(*chip).mgr).bufferinfo.area as *mut mixart_bufferinfo;
        (*bufferinfo.add(i as usize)).buffer_address = (*(*subs).runtime).dma_addr;
        (*bufferinfo.add(i as usize)).available_length = (*(*subs).runtime).dma_bytes;
        dev_dbg((*(*chip).card).dev, b"snd_mixart_hw_params(pcm %d) : dma_addr(%x) dma_bytes(%x) subs-number(%d)\n\0".as_ptr() as *const c_char, i, (*bufferinfo.add(i as usize)).buffer_address, (*bufferinfo.add(i as usize)).available_length, (*subs).number);
    }
    0
}

unsafe extern "C" fn snd_mixart_hw_free(subs: *mut snd_pcm_substream) -> c_int {
    let chip = snd_pcm_substream_chip(subs);
    mixart_sync_nonblock_events((*chip).mgr);
    0
}

static snd_mixart_analog_caps: snd_pcm_hardware = snd_pcm_hardware {
    info: SNDRV_PCM_INFO_MMAP | SNDRV_PCM_INFO_INTERLEAVED | SNDRV_PCM_INFO_MMAP_VALID | SNDRV_PCM_INFO_PAUSE,
    formats: SNDRV_PCM_FMTBIT_U8 | SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S16_BE | SNDRV_PCM_FMTBIT_S24_3LE | SNDRV_PCM_FMTBIT_S24_3BE | SNDRV_PCM_FMTBIT_FLOAT_LE | SNDRV_PCM_FMTBIT_FLOAT_BE,
    rates: SNDRV_PCM_RATE_CONTINUOUS | SNDRV_PCM_RATE_8000_48000,
    rate_min: 8000,
    rate_max: 48000,
    channels_min: 1,
    channels_max: 2,
    buffer_bytes_max: 32 * 1024,
    period_bytes_min: 256,
    period_bytes_max: 16 * 1024,
    periods_min: 2,
    periods_max: 32 * 1024 / 256,
};

static snd_mixart_digital_caps: snd_pcm_hardware = snd_pcm_hardware {
    info: SNDRV_PCM_INFO_MMAP | SNDRV_PCM_INFO_INTERLEAVED | SNDRV_PCM_INFO_MMAP_VALID | SNDRV_PCM_INFO_PAUSE,
    formats: SNDRV_PCM_FMTBIT_U8 | SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S16_BE | SNDRV_PCM_FMTBIT_S24_3LE | SNDRV_PCM_FMTBIT_S24_3BE | SNDRV_PCM_FMTBIT_FLOAT_LE | SNDRV_PCM_FMTBIT_FLOAT_BE,
    rates: SNDRV_PCM_RATE_32000 | SNDRV_PCM_RATE_44100 | SNDRV_PCM_RATE_48000,
    rate_min: 32000,
    rate_max: 48000,
    channels_min: 1,
    channels_max: 2,
    buffer_bytes_max: 32 * 1024,
    period_bytes_min: 256,
    period_bytes_max: 16 * 1024,
    periods_min: 2,
    periods_max: 32 * 1024 / 256,
};

unsafe extern "C" fn snd_mixart_playback_open(subs: *mut snd_pcm_substream) -> c_int {
    let chip = snd_pcm_substream_chip(subs);
    let mgr = (*chip).mgr;
    let runtime = (*subs).runtime;
    let pcm = (*subs).pcm;
    let pcm_number: c_int;
    guard_mutex(&mut (*mgr).setup_mutex);
    if pcm == (*chip).pcm {
        pcm_number = MIXART_PCM_ANALOG;
        (*runtime).hw = snd_mixart_analog_caps;
    } else {
        pcm_number = MIXART_PCM_DIGITAL;
        (*runtime).hw = snd_mixart_digital_caps;
    }
    dev_dbg((*(*chip).card).dev, b"snd_mixart_playback_open C%d/P%d/Sub%d\n\0".as_ptr() as *const c_char, (*chip).chip_idx, pcm_number, (*subs).number);
    let stream = &mut (*chip).playback_stream[pcm_number as usize][(*subs).number as usize] as *mut mixart_stream;
    if (*stream).status != MIXART_STREAM_STATUS_FREE {
        dev_err((*(*chip).card).dev, b"snd_mixart_playback_open C%d/P%d/Sub%d in use\n\0".as_ptr() as *const c_char, (*chip).chip_idx, pcm_number, (*subs).number);
        return -EBUSY;
    }
    let pipe = snd_mixart_add_ref_pipe(chip, pcm_number, 0, 0);
    if pipe.is_null() { return -EINVAL; }
    if mixart_set_pipe_state((*chip).mgr, pipe, 1) < 0 {
        dev_err((*(*chip).card).dev, b"error starting pipe!\n\0".as_ptr() as *const c_char);
        snd_mixart_kill_ref_pipe((*chip).mgr, pipe, 0);
        return -EINVAL;
    }
    (*stream).pipe = pipe;
    (*stream).pcm_number = pcm_number;
    (*stream).status = MIXART_STREAM_STATUS_OPEN;
    (*stream).substream = subs;
    (*stream).channels = 0;
    (*runtime).private_data = stream as *mut c_void;
    snd_pcm_hw_constraint_step(runtime, 0, SNDRV_PCM_HW_PARAM_PERIOD_BYTES, 32);
    snd_pcm_hw_constraint_step(runtime, 0, SNDRV_PCM_HW_PARAM_BUFFER_SIZE, 64);
    let old = (*mgr).ref_count_rate;
    (*mgr).ref_count_rate += 1;
    if old != 0 && (*mgr).sample_rate != 0 {
        (*runtime).hw.rate_min = (*mgr).sample_rate;
        (*runtime).hw.rate_max = (*mgr).sample_rate;
    }
    0
}

unsafe extern "C" fn snd_mixart_capture_open(subs: *mut snd_pcm_substream) -> c_int {
    let chip = snd_pcm_substream_chip(subs);
    let mgr = (*chip).mgr;
    let runtime = (*subs).runtime;
    let pcm = (*subs).pcm;
    let pcm_number: c_int;
    guard_mutex(&mut (*mgr).setup_mutex);
    if pcm == (*chip).pcm {
        pcm_number = MIXART_PCM_ANALOG;
        (*runtime).hw = snd_mixart_analog_caps;
    } else {
        pcm_number = MIXART_PCM_DIGITAL;
        (*runtime).hw = snd_mixart_digital_caps;
    }
    (*runtime).hw.channels_min = 2;
    dev_dbg((*(*chip).card).dev, b"snd_mixart_capture_open C%d/P%d/Sub%d\n\0".as_ptr() as *const c_char, (*chip).chip_idx, pcm_number, (*subs).number);
    let stream = &mut (*chip).capture_stream[pcm_number as usize] as *mut mixart_stream;
    if (*stream).status != MIXART_STREAM_STATUS_FREE {
        dev_err((*(*chip).card).dev, b"snd_mixart_capture_open C%d/P%d/Sub%d in use\n\0".as_ptr() as *const c_char, (*chip).chip_idx, pcm_number, (*subs).number);
        return -EBUSY;
    }
    let pipe = snd_mixart_add_ref_pipe(chip, pcm_number, 1, 0);
    if pipe.is_null() { return -EINVAL; }
    if mixart_set_pipe_state((*chip).mgr, pipe, 1) < 0 {
        dev_err((*(*chip).card).dev, b"error starting pipe!\n\0".as_ptr() as *const c_char);
        snd_mixart_kill_ref_pipe((*chip).mgr, pipe, 0);
        return -EINVAL;
    }
    (*stream).pipe = pipe;
    (*stream).pcm_number = pcm_number;
    (*stream).status = MIXART_STREAM_STATUS_OPEN;
    (*stream).substream = subs;
    (*stream).channels = 0;
    (*runtime).private_data = stream as *mut c_void;
    snd_pcm_hw_constraint_step(runtime, 0, SNDRV_PCM_HW_PARAM_PERIOD_BYTES, 32);
    snd_pcm_hw_constraint_step(runtime, 0, SNDRV_PCM_HW_PARAM_BUFFER_SIZE, 64);
    let old = (*mgr).ref_count_rate;
    (*mgr).ref_count_rate += 1;
    if old != 0 && (*mgr).sample_rate != 0 {
        (*runtime).hw.rate_min = (*mgr).sample_rate;
        (*runtime).hw.rate_max = (*mgr).sample_rate;
    }
    0
}

unsafe extern "C" fn snd_mixart_close(subs: *mut snd_pcm_substream) -> c_int {
    let chip = snd_pcm_substream_chip(subs);
    let mgr = (*chip).mgr;
    let stream = (*(*subs).runtime).private_data as *mut mixart_stream;
    guard_mutex(&mut (*mgr).setup_mutex);
    dev_dbg((*(*chip).card).dev, b"snd_mixart_close C%d/P%d/Sub%d\n\0".as_ptr() as *const c_char, (*chip).chip_idx, (*stream).pcm_number, (*subs).number);
    (*mgr).ref_count_rate -= 1;
    if (*mgr).ref_count_rate == 0 { (*mgr).sample_rate = 0; }
    if snd_mixart_kill_ref_pipe(mgr, (*stream).pipe, 0) < 0 {
        dev_err((*(*chip).card).dev, b"error snd_mixart_kill_ref_pipe C%dP%d\n\0".as_ptr() as *const c_char, (*chip).chip_idx, (*stream).pcm_number);
    }
    (*stream).pipe = null_mut();
    (*stream).status = MIXART_STREAM_STATUS_FREE;
    (*stream).substream = null_mut();
    0
}

unsafe extern "C" fn snd_mixart_stream_pointer(subs: *mut snd_pcm_substream) -> snd_pcm_uframes_t {
    let runtime = (*subs).runtime;
    let stream = (*runtime).private_data as *mut mixart_stream;
    ((*stream).buf_periods * (*runtime).period_size) + (*stream).buf_period_frag
}

static snd_mixart_playback_ops: snd_pcm_ops = snd_pcm_ops {
    open: Some(snd_mixart_playback_open),
    close: Some(snd_mixart_close),
    prepare: Some(snd_mixart_prepare),
    hw_params: Some(snd_mixart_hw_params),
    hw_free: Some(snd_mixart_hw_free),
    trigger: Some(snd_mixart_trigger),
    pointer: Some(snd_mixart_stream_pointer),
};
static snd_mixart_capture_ops: snd_pcm_ops = snd_pcm_ops {
    open: Some(snd_mixart_capture_open),
    close: Some(snd_mixart_close),
    prepare: Some(snd_mixart_prepare),
    hw_params: Some(snd_mixart_hw_params),
    hw_free: Some(snd_mixart_hw_free),
    trigger: Some(snd_mixart_trigger),
    pointer: Some(snd_mixart_stream_pointer),
};

unsafe extern "C" fn preallocate_buffers(chip: *mut snd_mixart, pcm: *mut snd_pcm) {
    /* Original C contains an #if 0 block assigning unique DMA device ids. */
    snd_pcm_set_managed_buffer_all(pcm, SNDRV_DMA_TYPE_DEV, &mut (*(*(*chip).mgr).pci).dev, 32 * 1024, 32 * 1024);
}

unsafe extern "C" fn snd_mixart_pcm_analog(chip: *mut snd_mixart) -> c_int {
    let mut pcm: *mut snd_pcm = null_mut();
    let mut name: [c_char; 32] = [0; 32];
    sprintf(name.as_mut_ptr(), b"miXart analog %d\0".as_ptr() as *const c_char, (*chip).chip_idx);
    let err = snd_pcm_new((*chip).card, name.as_ptr(), MIXART_PCM_ANALOG, MIXART_PLAYBACK_STREAMS, MIXART_CAPTURE_STREAMS, &mut pcm);
    if err < 0 {
        dev_err((*(*chip).card).dev, b"cannot create the analog pcm %d\n\0".as_ptr() as *const c_char, (*chip).chip_idx);
        return err;
    }
    (*pcm).private_data = chip as *mut c_void;
    snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_PLAYBACK, &snd_mixart_playback_ops);
    snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_CAPTURE, &snd_mixart_capture_ops);
    (*pcm).info_flags = 0;
    (*pcm).nonatomic = true;
    strscpy((*pcm).name.as_mut_ptr(), name.as_ptr());
    preallocate_buffers(chip, pcm);
    (*chip).pcm = pcm;
    0
}

unsafe extern "C" fn snd_mixart_pcm_digital(chip: *mut snd_mixart) -> c_int {
    let mut pcm: *mut snd_pcm = null_mut();
    let mut name: [c_char; 32] = [0; 32];
    sprintf(name.as_mut_ptr(), b"miXart AES/EBU %d\0".as_ptr() as *const c_char, (*chip).chip_idx);
    let err = snd_pcm_new((*chip).card, name.as_ptr(), MIXART_PCM_DIGITAL, MIXART_PLAYBACK_STREAMS, MIXART_CAPTURE_STREAMS, &mut pcm);
    if err < 0 {
        dev_err((*(*chip).card).dev, b"cannot create the digital pcm %d\n\0".as_ptr() as *const c_char, (*chip).chip_idx);
        return err;
    }
    (*pcm).private_data = chip as *mut c_void;
    snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_PLAYBACK, &snd_mixart_playback_ops);
    snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_CAPTURE, &snd_mixart_capture_ops);
    (*pcm).info_flags = 0;
    (*pcm).nonatomic = true;
    strscpy((*pcm).name.as_mut_ptr(), name.as_ptr());
    preallocate_buffers(chip, pcm);
    (*chip).pcm_dig = pcm;
    0
}

unsafe extern "C" fn snd_mixart_chip_free(chip: *mut snd_mixart) -> c_int {
    kfree(chip as *mut c_void);
    0
}
unsafe extern "C" fn snd_mixart_chip_dev_free(device: *mut snd_device) -> c_int {
    snd_mixart_chip_free((*device).device_data as *mut snd_mixart)
}

unsafe extern "C" fn snd_mixart_create(mgr: *mut mixart_mgr, card: *mut snd_card, idx: c_int) -> c_int {
    static ops: snd_device_ops = snd_device_ops { dev_free: Some(snd_mixart_chip_dev_free) };
    let chip = kzalloc(size_of::<snd_mixart>(), 0) as *mut snd_mixart;
    if chip.is_null() { return -ENOMEM; }
    (*chip).card = card;
    (*chip).chip_idx = idx;
    (*chip).mgr = mgr;
    (*card).sync_irq = (*mgr).irq;
    let err = snd_device_new(card, SNDRV_DEV_LOWLEVEL, chip as *mut c_void, &ops);
    if err < 0 {
        snd_mixart_chip_free(chip);
        return err;
    }
    (*mgr).chip[idx as usize] = chip;
    0
}

pub unsafe extern "C" fn snd_mixart_create_pcm(chip: *mut snd_mixart) -> c_int {
    let mut err = snd_mixart_pcm_analog(chip);
    if err < 0 { return err; }
    if (*(*chip).mgr).board_type == MIXART_DAUGHTER_TYPE_AES {
        err = snd_mixart_pcm_digital(chip);
        if err < 0 { return err; }
    }
    err
}

unsafe extern "C" fn snd_mixart_free(mgr: *mut mixart_mgr) -> c_int {
    let mut i: c_uint = 0;
    while i < (*mgr).num_cards {
        if !(*mgr).chip[i as usize].is_null() { snd_card_free((*(*mgr).chip[i as usize]).card); }
        i += 1;
    }
    snd_mixart_exit_mailbox(mgr);
    if (*mgr).irq >= 0 { free_irq((*mgr).irq, mgr as *mut c_void); }
    if (*mgr).dsp_loaded != 0 {
        snd_mixart_reset_board(mgr);
        dev_dbg(&mut (*(*mgr).pci).dev, b"reset miXart !\n\0".as_ptr() as *const c_char);
    }
    i = 0;
    while i < 2 {
        iounmap((*mgr).mem[i as usize].virt);
        i += 1;
    }
    pci_release_regions((*mgr).pci);
    if !(*mgr).flowinfo.area.is_null() {
        snd_dma_free_pages(&mut (*mgr).flowinfo);
        (*mgr).flowinfo.area = null_mut();
    }
    if !(*mgr).bufferinfo.area.is_null() {
        snd_dma_free_pages(&mut (*mgr).bufferinfo);
        (*mgr).bufferinfo.area = null_mut();
    }
    pci_disable_device((*mgr).pci);
    kfree(mgr as *mut c_void);
    0
}

unsafe extern "C" fn snd_mixart_BA0_read(entry: *mut snd_info_entry, _file_private_data: *mut c_void, _file: *mut file, buf: *mut c_char, mut count: size_t, pos: loff_t) -> ssize_t {
    let mgr = (*entry).private_data as *mut mixart_mgr;
    count &= !3usize;
    if copy_to_user_fromio(buf, MIXART_MEM(mgr, pos), count) != 0 { return -EFAULT as ssize_t; }
    count as ssize_t
}
unsafe extern "C" fn snd_mixart_BA1_read(entry: *mut snd_info_entry, _file_private_data: *mut c_void, _file: *mut file, buf: *mut c_char, mut count: size_t, pos: loff_t) -> ssize_t {
    let mgr = (*entry).private_data as *mut mixart_mgr;
    count &= !3usize;
    if copy_to_user_fromio(buf, MIXART_REG(mgr, pos), count) != 0 { return -EFAULT as ssize_t; }
    count as ssize_t
}

static snd_mixart_proc_ops_BA0: snd_info_entry_ops = snd_info_entry_ops { read: Some(snd_mixart_BA0_read) };
static snd_mixart_proc_ops_BA1: snd_info_entry_ops = snd_info_entry_ops { read: Some(snd_mixart_BA1_read) };

unsafe extern "C" fn snd_mixart_proc_read(entry: *mut snd_info_entry, buffer: *mut snd_info_buffer) {
    let chip = (*entry).private_data as *mut snd_mixart;
    snd_iprintf(buffer, b"Digigram miXart (alsa card %d)\n\n\0".as_ptr() as *const c_char, (*chip).chip_idx);
    if ((*(*chip).mgr).dsp_loaded & (1 << MIXART_MOTHERBOARD_ELF_INDEX)) != 0 {
        snd_iprintf(buffer, b"- hardware -\n\0".as_ptr() as *const c_char);
        match (*(*chip).mgr).board_type {
            MIXART_DAUGHTER_TYPE_NONE => snd_iprintf(buffer, b"\tmiXart8 (no daughter board)\n\n\0".as_ptr() as *const c_char),
            MIXART_DAUGHTER_TYPE_AES => snd_iprintf(buffer, b"\tmiXart8 AES/EBU\n\n\0".as_ptr() as *const c_char),
            MIXART_DAUGHTER_TYPE_COBRANET => snd_iprintf(buffer, b"\tmiXart8 Cobranet\n\n\0".as_ptr() as *const c_char),
            _ => snd_iprintf(buffer, b"\tUNKNOWN!\n\n\0".as_ptr() as *const c_char),
        }
        snd_iprintf(buffer, b"- system load -\n\0".as_ptr() as *const c_char);
        let refv = readl_be(MIXART_MEM((*chip).mgr, MIXART_PSEUDOREG_PERF_SYSTEM_LOAD_OFFSET));
        if refv != 0 {
            let mailbox = 100 * readl_be(MIXART_MEM((*chip).mgr, MIXART_PSEUDOREG_PERF_MAILBX_LOAD_OFFSET)) / refv;
            let streaming = 100 * readl_be(MIXART_MEM((*chip).mgr, MIXART_PSEUDOREG_PERF_STREAM_LOAD_OFFSET)) / refv;
            let interr = 100 * readl_be(MIXART_MEM((*chip).mgr, MIXART_PSEUDOREG_PERF_INTERR_LOAD_OFFSET)) / refv;
            snd_iprintf(buffer, b"\tstreaming          : %d\n\0".as_ptr() as *const c_char, streaming);
            snd_iprintf(buffer, b"\tmailbox            : %d\n\0".as_ptr() as *const c_char, mailbox);
            snd_iprintf(buffer, b"\tinterrupts handling : %d\n\n\0".as_ptr() as *const c_char, interr);
        }
    }
}

unsafe extern "C" fn snd_mixart_proc_init(chip: *mut snd_mixart) {
    let mut entry: *mut snd_info_entry = null_mut();
    snd_card_ro_proc_new((*chip).card, b"board_info\0".as_ptr() as *const c_char, chip as *mut c_void, snd_mixart_proc_read);
    if snd_card_proc_new((*chip).card, b"mixart_BA0\0".as_ptr() as *const c_char, &mut entry) == 0 {
        (*entry).content = SNDRV_INFO_CONTENT_DATA;
        (*entry).private_data = (*chip).mgr as *mut c_void;
        (*entry).c.ops = &snd_mixart_proc_ops_BA0;
        (*entry).size = MIXART_BA0_SIZE;
    }
    if snd_card_proc_new((*chip).card, b"mixart_BA1\0".as_ptr() as *const c_char, &mut entry) == 0 {
        (*entry).content = SNDRV_INFO_CONTENT_DATA;
        (*entry).private_data = (*chip).mgr as *mut c_void;
        (*entry).c.ops = &snd_mixart_proc_ops_BA1;
        (*entry).size = MIXART_BA1_SIZE;
    }
}

unsafe extern "C" fn snd_mixart_probe(pci: *mut pci_dev, _pci_id: *const pci_device_id) -> c_int {
    static mut dev: c_int = 0;
    let mut err: c_int;
    let mut size: usize;
    if dev >= SNDRV_CARDS as c_int { return -ENODEV; }
    if !enable[dev as usize] {
        dev += 1;
        return -ENOENT;
    }
    err = pci_enable_device(pci);
    if err < 0 { return err; }
    pci_set_master(pci);
    if dma_set_mask(&mut (*pci).dev, DMA_BIT_MASK(32)) < 0 {
        dev_err(&mut (*pci).dev, b"architecture does not support 32bit PCI busmaster DMA\n\0".as_ptr() as *const c_char);
        pci_disable_device(pci);
        return -ENXIO;
    }
    let mgr = kzalloc(size_of::<mixart_mgr>(), 0) as *mut mixart_mgr;
    if mgr.is_null() {
        pci_disable_device(pci);
        return -ENOMEM;
    }
    (*mgr).pci = pci;
    (*mgr).irq = -1;
    err = pci_request_regions(pci, CARD_NAME.as_ptr());
    if err < 0 {
        kfree(mgr as *mut c_void);
        pci_disable_device(pci);
        return err;
    }
    let mut i: c_uint = 0;
    while i < 2 {
        (*mgr).mem[i as usize].phys = pci_resource_start(pci, i);
        (*mgr).mem[i as usize].virt = pci_ioremap_bar(pci, i);
        if (*mgr).mem[i as usize].virt.is_null() {
            dev_err(&mut (*pci).dev, b"unable to remap resource 0x%lx\n\0".as_ptr() as *const c_char, (*mgr).mem[i as usize].phys);
            snd_mixart_free(mgr);
            return -EBUSY;
        }
        i += 1;
    }
    if request_threaded_irq((*pci).irq, snd_mixart_interrupt, snd_mixart_threaded_irq, IRQF_SHARED, &KBUILD_MODNAME as *const c_char, mgr as *mut c_void) != 0 {
        dev_err(&mut (*pci).dev, b"unable to grab IRQ %d\n\0".as_ptr() as *const c_char, (*pci).irq);
        snd_mixart_free(mgr);
        return -EBUSY;
    }
    (*mgr).irq = (*pci).irq;
    (*mgr).msg_fifo_readptr = 0;
    (*mgr).msg_fifo_writeptr = 0;
    mutex_init(&mut (*mgr).lock);
    mutex_init(&mut (*mgr).msg_lock);
    init_waitqueue_head(&mut (*mgr).msg_sleep);
    atomic_set(&mut (*mgr).msg_processed, 0);
    mutex_init(&mut (*mgr).setup_mutex);
    (*mgr).num_cards = MIXART_MAX_CARDS as c_uint;
    i = 0;
    while i < (*mgr).num_cards {
        let mut card: *mut snd_card = null_mut();
        let mut tmpid: [c_char; 16] = [0; 16];
        let idx = if index[dev as usize] < 0 { index[dev as usize] } else { index[dev as usize] + i as c_int };
        snprintf(tmpid.as_mut_ptr(), tmpid.len(), b"%s-%d\0".as_ptr() as *const c_char, if id[dev as usize].is_null() { b"MIXART\0".as_ptr() as *const c_char } else { id[dev as usize] }, i);
        err = snd_card_new(&mut (*pci).dev, idx, tmpid.as_ptr(), &THIS_MODULE as *const c_void, 0, &mut card);
        if err < 0 {
            dev_err(&mut (*pci).dev, b"cannot allocate the card %d\n\0".as_ptr() as *const c_char, i);
            snd_mixart_free(mgr);
            return err;
        }
        strscpy((*card).driver.as_mut_ptr(), CARD_NAME.as_ptr() as *const c_char);
        snprintf((*card).shortname.as_mut_ptr(), (*card).shortname.len(), b"Digigram miXart [PCM #%d]\0".as_ptr() as *const c_char, i);
        snprintf((*card).longname.as_mut_ptr(), (*card).longname.len(), b"Digigram miXart at 0x%lx & 0x%lx, irq %i [PCM #%d]\0".as_ptr() as *const c_char, (*mgr).mem[0].phys, (*mgr).mem[1].phys, (*mgr).irq, i);
        err = snd_mixart_create(mgr, card, i as c_int);
        if err < 0 {
            snd_card_free(card);
            snd_mixart_free(mgr);
            return err;
        }
        if i == 0 { snd_mixart_proc_init((*mgr).chip[i as usize]); }
        err = snd_card_register(card);
        if err < 0 {
            snd_mixart_free(mgr);
            return err;
        }
        i += 1;
    }
    (*mgr).board_type = MIXART_DAUGHTER_TYPE_NONE;
    size = PAGE_ALIGN(MIXART_MAX_STREAM_PER_CARD as usize * MIXART_MAX_CARDS as usize * size_of::<mixart_flowinfo>());
    if snd_dma_alloc_pages(SNDRV_DMA_TYPE_DEV, &mut (*pci).dev, size, &mut (*mgr).flowinfo) < 0 {
        snd_mixart_free(mgr);
        return -ENOMEM;
    }
    memset((*mgr).flowinfo.area, 0, size);
    size = PAGE_ALIGN(MIXART_MAX_STREAM_PER_CARD as usize * MIXART_MAX_CARDS as usize * size_of::<mixart_bufferinfo>());
    if snd_dma_alloc_pages(SNDRV_DMA_TYPE_DEV, &mut (*pci).dev, size, &mut (*mgr).bufferinfo) < 0 {
        snd_mixart_free(mgr);
        return -ENOMEM;
    }
    memset((*mgr).bufferinfo.area, 0, size);
    err = snd_mixart_setup_firmware(mgr);
    if err < 0 {
        snd_mixart_free(mgr);
        return err;
    }
    pci_set_drvdata(pci, mgr as *mut c_void);
    dev += 1;
    0
}

unsafe extern "C" fn snd_mixart_remove(pci: *mut pci_dev) {
    snd_mixart_free(pci_get_drvdata(pci) as *mut mixart_mgr);
}

#[repr(C)]
pub struct pci_driver {
    pub name: *const c_char,
    pub id_table: *const pci_device_id,
    pub probe: Option<unsafe extern "C" fn(*mut pci_dev, *const pci_device_id) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut pci_dev)>,
}

static snd_mixart_ids: [pci_device_id; 2] = [pci_device_id { _private: [] }, pci_device_id { _private: [] }];

static mut mixart_driver: pci_driver = pci_driver {
    name: unsafe { &KBUILD_MODNAME as *const c_char },
    id_table: snd_mixart_ids.as_ptr(),
    probe: Some(snd_mixart_probe),
    remove: Some(snd_mixart_remove),
};

/* MODULE_AUTHOR("Digigram <alsa@digigram.com>");
 * MODULE_DESCRIPTION("Digigram " CARD_NAME);
 * MODULE_LICENSE("GPL");
 * module parameters and module_pci_driver(mixart_driver) are preserved as
 * external kernel module metadata intent.
 */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
