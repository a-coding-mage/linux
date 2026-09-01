// SPDX-License-Identifier: GPL-2.0
//
// Mediatek ALSA BT SCO CVSD/MSBC Driver
//
// Copyright (c) 2019 MediaTek Inc.
// Author: KaiChieh Chuang <kaichieh.chuang@mediatek.com>

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

type u8 = u8;
type u16 = u16;
type u32 = u32;
type ssize_t = isize;
type size_t = usize;
type snd_pcm_uframes_t = c_ulong;
type irqreturn_t = c_uint;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}
#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_pcm_substream {
    pub runtime: *mut snd_pcm_runtime,
    pub stream: c_int,
}
#[repr(C)]
pub struct snd_pcm_runtime {
    pub format: c_int,
    pub channels: c_uint,
    pub dma_bytes: size_t,
    pub buffer_size: snd_pcm_uframes_t,
}
#[repr(C)]
pub struct snd_soc_component {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}
#[repr(C)]
pub struct iov_iter {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_kcontrol {
    pub private_value: c_ulong,
}
#[repr(C)]
pub struct snd_ctl_elem_value {
    pub value: snd_ctl_elem_value_value,
}
#[repr(C)]
pub union snd_ctl_elem_value_value {
    pub integer: snd_ctl_elem_value_integer,
    pub enumerated: snd_ctl_elem_value_enumerated,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_elem_value_integer {
    pub value: [c_long; 128],
}
type c_long = isize;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_elem_value_enumerated {
    pub item: [c_uint; 128],
}
#[repr(C)]
pub struct soc_enum {
    pub items: c_uint,
}
#[repr(C)]
pub struct snd_pcm_hardware {
    pub info: c_uint,
    pub formats: u64,
    pub buffer_bytes_max: size_t,
    pub period_bytes_max: size_t,
    pub periods_min: c_uint,
    pub periods_max: c_uint,
    pub fifo_size: size_t,
}
#[repr(C)]
pub struct snd_kcontrol_new {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_component_driver {
    pub name: *const c_char,
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub open: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream) -> c_int>,
    pub close: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream) -> c_int>,
    pub hw_params: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream, *mut snd_pcm_hw_params) -> c_int>,
    pub hw_free: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream) -> c_int>,
    pub prepare: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream) -> c_int>,
    pub trigger: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream, c_int) -> c_int>,
    pub pointer: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream) -> snd_pcm_uframes_t>,
    pub copy: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream, c_int, c_ulong, *mut iov_iter, c_ulong) -> c_int>,
}
#[repr(C)]
pub struct platform_device {
    pub dev: device,
}
#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
}
#[repr(C)]
pub struct platform_driver_driver {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
}
#[repr(C)]
pub struct platform_driver {
    pub driver: platform_driver_driver,
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut platform_device)>,
}

const BTCVSD_SND_NAME: &[u8] = b"mtk-btcvsd-snd\0";

const BT_CVSD_TX_NREADY: c_uint = 1 << 21;
const BT_CVSD_RX_READY: c_uint = 1 << 22;
const BT_CVSD_TX_UNDERFLOW: c_uint = 1 << 23;
const BT_CVSD_RX_OVERFLOW: c_uint = 1 << 24;
const BT_CVSD_INTERRUPT: c_uint = 1 << 31;
const BT_CVSD_CLEAR: c_uint = BT_CVSD_TX_NREADY | BT_CVSD_RX_READY | BT_CVSD_TX_UNDERFLOW | BT_CVSD_RX_OVERFLOW | BT_CVSD_INTERRUPT;

const SCO_TX_ENCODE_SIZE: usize = 60;
const SCO_TX_PACKER_BUF_NUM: usize = 18;
const SCO_RX_PLC_SIZE: usize = 30;
const SCO_RX_PACKER_BUF_NUM: usize = 64;
const SCO_RX_PACKET_MASK: c_int = 0x3f;
const SCO_CVSD_PACKET_VALID_SIZE: usize = 2;
const SCO_PACKET_120: usize = 120;
const SCO_PACKET_180: usize = 180;
const BTCVSD_RX_PACKET_SIZE: usize = SCO_RX_PLC_SIZE + SCO_CVSD_PACKET_VALID_SIZE;
const BTCVSD_TX_PACKET_SIZE: usize = SCO_TX_ENCODE_SIZE;
const BTCVSD_RX_BUF_SIZE: usize = BTCVSD_RX_PACKET_SIZE * SCO_RX_PACKER_BUF_NUM;
const BTCVSD_TX_BUF_SIZE: usize = BTCVSD_TX_PACKET_SIZE * SCO_TX_PACKER_BUF_NUM;

const SNDRV_PCM_STREAM_PLAYBACK: c_int = 0;
const SNDRV_PCM_STREAM_CAPTURE: c_int = 1;
const SNDRV_PCM_FORMAT_S32_LE: c_int = 10;
const SNDRV_PCM_FORMAT_U32_LE: c_int = 12;
const SNDRV_PCM_FMTBIT_S16_LE: u64 = 1 << 2;
const SNDRV_PCM_INFO_MMAP: c_uint = 1 << 0;
const SNDRV_PCM_INFO_INTERLEAVED: c_uint = 1 << 1;
const SNDRV_PCM_INFO_RESUME: c_uint = 1 << 4;
const SNDRV_PCM_TRIGGER_START: c_int = 0;
const SNDRV_PCM_TRIGGER_STOP: c_int = 1;
const SNDRV_PCM_TRIGGER_RESUME: c_int = 4;
const SNDRV_PCM_TRIGGER_SUSPEND: c_int = 5;
const IRQ_HANDLED: irqreturn_t = 1;
const IRQF_TRIGGER_LOW: c_ulong = 0x8;
const GFP_KERNEL: c_uint = 0;
const EIO: c_int = 5;
const EINVAL: c_int = 22;
const EFAULT: c_int = 14;
const ETIME: c_int = 62;
const ENOMEM: c_int = 12;
const ENXIO: c_int = 6;
const INT_MAX: c_int = c_int::MAX;
const INT_MIN: c_int = c_int::MIN;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum bt_sco_state {
    BT_SCO_STATE_IDLE,
    BT_SCO_STATE_RUNNING,
    BT_SCO_STATE_ENDING,
    BT_SCO_STATE_LOOPBACK,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum bt_sco_direct {
    BT_SCO_DIRECT_BT2ARM,
    BT_SCO_DIRECT_ARM2BT,
}

#[repr(C)]
#[derive(Copy, Clone)]
enum bt_sco_packet_len {
    BT_SCO_CVSD_30 = 0,
    BT_SCO_CVSD_60,
    BT_SCO_CVSD_90,
    BT_SCO_CVSD_120,
    BT_SCO_CVSD_10,
    BT_SCO_CVSD_20,
    BT_SCO_CVSD_MAX,
}
const BT_SCO_CVSD_MAX_USIZE: usize = bt_sco_packet_len::BT_SCO_CVSD_MAX as usize;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum BT_SCO_BAND {
    BT_SCO_NB,
    BT_SCO_WB,
}

#[repr(C)]
struct mtk_btcvsd_snd_hw_info {
    num_valid_addr: c_uint,
    bt_sram_addr: [c_ulong; 20],
    packet_length: c_uint,
    packet_num: c_uint,
}

#[repr(C)]
struct mtk_btcvsd_snd_stream {
    substream: *mut snd_pcm_substream,
    stream: c_int,
    state: bt_sco_state,
    packet_size: c_uint,
    buf_size: c_uint,
    temp_packet_buf: [u8; SCO_PACKET_180],
    packet_w: c_int,
    packet_r: c_int,
    prev_frame: snd_pcm_uframes_t,
    prev_packet_idx: c_int,
    xrun: c_uint,
    timeout: c_uint,
    mute: c_uint,
    trigger_start: c_uint,
    wait_flag: c_uint,
    rw_cnt: c_uint,
    time_stamp: u64,
    buf_data_equivalent_time: u64,
    buffer_info: mtk_btcvsd_snd_hw_info,
}

#[repr(C)]
struct mtk_btcvsd_snd {
    dev: *mut device,
    irq_id: c_int,
    infra: *mut regmap,
    bt_pkv_base: *mut c_void,
    bt_sram_bank2_base: *mut c_void,
    infra_misc_offset: c_uint,
    conn_bt_cvsd_mask: c_uint,
    cvsd_mcu_read_offset: c_uint,
    cvsd_mcu_write_offset: c_uint,
    cvsd_packet_indicator: c_uint,
    bt_reg_pkt_r: *mut u32,
    bt_reg_pkt_w: *mut u32,
    bt_reg_ctl: *mut u32,
    irq_disabled: c_uint,
    tx_lock: spinlock_t,
    rx_lock: spinlock_t,
    tx_wait: wait_queue_head_t,
    rx_wait: wait_queue_head_t,
    tx: *mut mtk_btcvsd_snd_stream,
    rx: *mut mtk_btcvsd_snd_stream,
    tx_packet_buf: [u8; BTCVSD_TX_BUF_SIZE],
    rx_packet_buf: [u8; BTCVSD_RX_BUF_SIZE],
    band: BT_SCO_BAND,
}

#[repr(C)]
struct mtk_btcvsd_snd_time_buffer_info {
    data_count_equi_time: u64,
    time_stamp_us: u64,
}

#[repr(C)]
pub struct spinlock_t {
    _private: [u8; 0],
}
#[repr(C)]
pub struct wait_queue_head_t {
    _private: [u8; 0],
}

static btsco_packet_valid_mask: [[c_uint; 6]; BT_SCO_CVSD_MAX_USIZE] = [
    [0x1, 0x1 << 1, 0x1 << 2, 0x1 << 3, 0x1 << 4, 0x1 << 5],
    [0x1, 0x1, 0x2, 0x2, 0x4, 0x4],
    [0x1, 0x1, 0x1, 0x2, 0x2, 0x2],
    [0x1, 0x1, 0x1, 0x1, 0x0, 0x0],
    [0x7, 0x7 << 3, 0x7 << 6, 0x7 << 9, 0x7 << 12, 0x7 << 15],
    [0x3, 0x3 << 1, 0x3 << 3, 0x3 << 4, 0x3 << 6, 0x3 << 7],
];

static btsco_packet_info: [[c_uint; 4]; BT_SCO_CVSD_MAX_USIZE] = [
    [30, 6, (SCO_PACKET_180 / SCO_TX_ENCODE_SIZE) as c_uint, (SCO_PACKET_180 / SCO_RX_PLC_SIZE) as c_uint],
    [60, 3, (SCO_PACKET_180 / SCO_TX_ENCODE_SIZE) as c_uint, (SCO_PACKET_180 / SCO_RX_PLC_SIZE) as c_uint],
    [90, 2, (SCO_PACKET_180 / SCO_TX_ENCODE_SIZE) as c_uint, (SCO_PACKET_180 / SCO_RX_PLC_SIZE) as c_uint],
    [120, 1, (SCO_PACKET_120 / SCO_TX_ENCODE_SIZE) as c_uint, (SCO_PACKET_120 / SCO_RX_PLC_SIZE) as c_uint],
    [10, 18, (SCO_PACKET_180 / SCO_TX_ENCODE_SIZE) as c_uint, (SCO_PACKET_180 / SCO_RX_PLC_SIZE) as c_uint],
    [20, 9, (SCO_PACKET_180 / SCO_TX_ENCODE_SIZE) as c_uint, (SCO_PACKET_180 / SCO_RX_PLC_SIZE) as c_uint],
];

static table_msbc_silence: [u8; SCO_PACKET_180] = [
    0x01, 0x38, 0xad, 0x00, 0x00, 0xc5, 0x00, 0x00, 0x00, 0x00,
    0x77, 0x6d, 0xb6, 0xdd, 0xdb, 0x6d, 0xb7, 0x76, 0xdb, 0x6d,
    0xdd, 0xb6, 0xdb, 0x77, 0x6d, 0xb6, 0xdd, 0xdb, 0x6d, 0xb7,
    0x76, 0xdb, 0x6d, 0xdd, 0xb6, 0xdb, 0x77, 0x6d, 0xb6, 0xdd,
    0xdb, 0x6d, 0xb7, 0x76, 0xdb, 0x6d, 0xdd, 0xb6, 0xdb, 0x77,
    0x6d, 0xb6, 0xdd, 0xdb, 0x6d, 0xb7, 0x76, 0xdb, 0x6c, 0x00,
    0x01, 0xc8, 0xad, 0x00, 0x00, 0xc5, 0x00, 0x00, 0x00, 0x00,
    0x77, 0x6d, 0xb6, 0xdd, 0xdb, 0x6d, 0xb7, 0x76, 0xdb, 0x6d,
    0xdd, 0xb6, 0xdb, 0x77, 0x6d, 0xb6, 0xdd, 0xdb, 0x6d, 0xb7,
    0x76, 0xdb, 0x6d, 0xdd, 0xb6, 0xdb, 0x77, 0x6d, 0xb6, 0xdd,
    0xdb, 0x6d, 0xb7, 0x76, 0xdb, 0x6d, 0xdd, 0xb6, 0xdb, 0x77,
    0x6d, 0xb6, 0xdd, 0xdb, 0x6d, 0xb7, 0x76, 0xdb, 0x6c, 0x00,
    0x01, 0xf8, 0xad, 0x00, 0x00, 0xc5, 0x00, 0x00, 0x00, 0x00,
    0x77, 0x6d, 0xb6, 0xdd, 0xdb, 0x6d, 0xb7, 0x76, 0xdb, 0x6d,
    0xdd, 0xb6, 0xdb, 0x77, 0x6d, 0xb6, 0xdd, 0xdb, 0x6d, 0xb7,
    0x76, 0xdb, 0x6d, 0xdd, 0xb6, 0xdb, 0x77, 0x6d, 0xb6, 0xdd,
    0xdb, 0x6d, 0xb7, 0x76, 0xdb, 0x6d, 0xdd, 0xb6, 0xdb, 0x77,
    0x6d, 0xb6, 0xdd, 0xdb, 0x6d, 0xb7, 0x76, 0xdb, 0x6c, 0x00,
];

unsafe extern "C" {
    fn regmap_update_bits(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn disable_irq(irq: c_int);
    fn enable_irq(irq: c_int);
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn sched_clock() -> u64;
    fn nsecs_to_jiffies(n: u64) -> c_ulong;
    fn wait_event_interruptible_timeout(wq: *mut wait_queue_head_t, condition: c_uint, timeout: c_ulong) -> c_long;
    fn wake_up_interruptible(wq: *mut wait_queue_head_t);
    fn snd_pcm_period_elapsed(substream: *mut snd_pcm_substream);
    fn copy_to_iter(addr: *const c_void, bytes: size_t, i: *mut iov_iter) -> size_t;
    fn copy_from_iter(addr: *mut c_void, bytes: size_t, i: *mut iov_iter) -> size_t;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn snd_soc_set_runtime_hwparams(substream: *mut snd_pcm_substream, hw: *const snd_pcm_hardware);
    fn params_buffer_bytes(params: *mut snd_pcm_hw_params) -> c_uint;
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_component;
    fn copy_to_user(to: *mut c_uint, from: *const c_void, n: size_t) -> c_ulong;
    fn snd_soc_add_component_controls(component: *mut snd_soc_component, controls: *const snd_kcontrol_new, num_controls: c_uint) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: size_t, flags: c_uint) -> *mut c_void;
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut c_void);
    fn platform_get_irq(pdev: *mut platform_device, num: c_uint) -> c_int;
    fn devm_request_irq(dev: *mut device, irq: c_int, handler: unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t, flags: c_ulong, name: *const c_char, dev_id: *mut c_void) -> c_int;
    fn of_iomap(node: *mut c_void, index: c_int) -> *mut c_void;
    fn syscon_regmap_lookup_by_phandle(np: *mut c_void, property: *const c_char) -> *mut regmap;
    fn of_property_read_u32_array(np: *mut c_void, propname: *const c_char, out_values: *mut u32, sz: size_t) -> c_int;
    fn devm_snd_soc_register_component(dev: *mut device, cmpnt_drv: *const snd_soc_component_driver, dai_drv: *mut c_void, num_dai: c_int) -> c_int;
    fn iounmap(addr: *mut c_void);
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn spin_lock_init(lock: *mut spinlock_t);
    fn init_waitqueue_head(wq: *mut wait_queue_head_t);
}

macro_rules! dev_dbg { ($($arg:tt)*) => {{ }}; }
macro_rules! dev_info { ($($arg:tt)*) => {{ }}; }
macro_rules! dev_warn { ($($arg:tt)*) => {{ }}; }
macro_rules! dev_err { ($($arg:tt)*) => {{ }}; }

unsafe fn mtk_btcvsd_snd_irq_enable(bt: *mut mtk_btcvsd_snd) {
    unsafe { regmap_update_bits((*bt).infra, (*bt).infra_misc_offset, (*bt).conn_bt_cvsd_mask, 0); }
}

unsafe fn mtk_btcvsd_snd_irq_disable(bt: *mut mtk_btcvsd_snd) {
    unsafe { regmap_update_bits((*bt).infra, (*bt).infra_misc_offset, (*bt).conn_bt_cvsd_mask, (*bt).conn_bt_cvsd_mask); }
}

unsafe fn mtk_btcvsd_snd_set_state(bt: *mut mtk_btcvsd_snd, bt_stream: *mut mtk_btcvsd_snd_stream, state: c_int) {
    dev_dbg!();
    unsafe {
        (*bt_stream).state = core::mem::transmute::<c_int, bt_sco_state>(state);
        if (*(*bt).tx).state == bt_sco_state::BT_SCO_STATE_IDLE && (*(*bt).rx).state == bt_sco_state::BT_SCO_STATE_IDLE {
            if (*bt).irq_disabled == 0 {
                disable_irq((*bt).irq_id);
                mtk_btcvsd_snd_irq_disable(bt);
                (*bt).irq_disabled = 1;
            }
        } else if (*bt).irq_disabled != 0 {
            enable_irq((*bt).irq_id);
            mtk_btcvsd_snd_irq_enable(bt);
            (*bt).irq_disabled = 0;
        }
    }
}

unsafe fn mtk_btcvsd_snd_tx_init(bt: *mut mtk_btcvsd_snd) -> c_int {
    unsafe {
        memset((*bt).tx as *mut c_void, 0, size_of::<mtk_btcvsd_snd_stream>());
        memset((*bt).tx_packet_buf.as_mut_ptr() as *mut c_void, 0, BTCVSD_TX_BUF_SIZE);
        (*(*bt).tx).packet_size = BTCVSD_TX_PACKET_SIZE as c_uint;
        (*(*bt).tx).buf_size = BTCVSD_TX_BUF_SIZE as c_uint;
        (*(*bt).tx).timeout = 0;
        (*(*bt).tx).rw_cnt = 0;
        (*(*bt).tx).stream = SNDRV_PCM_STREAM_PLAYBACK;
    }
    0
}

unsafe fn mtk_btcvsd_snd_rx_init(bt: *mut mtk_btcvsd_snd) -> c_int {
    unsafe {
        memset((*bt).rx as *mut c_void, 0, size_of::<mtk_btcvsd_snd_stream>());
        memset((*bt).rx_packet_buf.as_mut_ptr() as *mut c_void, 0, BTCVSD_RX_BUF_SIZE);
        (*(*bt).rx).packet_size = BTCVSD_RX_PACKET_SIZE as c_uint;
        (*(*bt).rx).buf_size = BTCVSD_RX_BUF_SIZE as c_uint;
        (*(*bt).rx).timeout = 0;
        (*(*bt).rx).rw_cnt = 0;
        (*(*bt).rx).stream = SNDRV_PCM_STREAM_CAPTURE;
    }
    0
}

unsafe fn get_tx_time_stamp(bt: *mut mtk_btcvsd_snd, ts: *mut mtk_btcvsd_snd_time_buffer_info) {
    unsafe {
        (*ts).time_stamp_us = (*(*bt).tx).time_stamp;
        (*ts).data_count_equi_time = (*(*bt).tx).buf_data_equivalent_time;
    }
}

unsafe fn get_rx_time_stamp(bt: *mut mtk_btcvsd_snd, ts: *mut mtk_btcvsd_snd_time_buffer_info) {
    unsafe {
        (*ts).time_stamp_us = (*(*bt).rx).time_stamp;
        (*ts).data_count_equi_time = (*(*bt).rx).buf_data_equivalent_time;
    }
}

unsafe fn btcvsd_bytes_to_frame(substream: *mut snd_pcm_substream, bytes: c_int) -> c_int {
    unsafe {
        let mut count = bytes;
        let runtime = (*substream).runtime;
        if (*runtime).format == SNDRV_PCM_FORMAT_S32_LE || (*runtime).format == SNDRV_PCM_FORMAT_U32_LE {
            count >>= 2;
        } else {
            count >>= 1;
        }
        count /= (*runtime).channels as c_int;
        count
    }
}

unsafe fn mtk_btcvsd_snd_data_transfer(dir: bt_sco_direct, src: *mut u8, dst: *mut u8, blk_size: c_uint, blk_num: c_uint) {
    unsafe {
        if blk_size == 60 || blk_size == 120 || blk_size == 20 {
            let mut src_32 = src as *mut u32;
            let mut dst_32 = dst as *mut u32;
            for _ in 0..(blk_size * blk_num / 4) {
                *dst_32 = *src_32;
                dst_32 = dst_32.add(1);
                src_32 = src_32.add(1);
            }
        } else {
            let mut src_16 = src as *mut u16;
            let mut dst_16 = dst as *mut u16;
            for _j in 0..blk_num {
                for _i in 0..(blk_size / 2) {
                    *dst_16 = *src_16;
                    dst_16 = dst_16.add(1);
                    src_16 = src_16.add(1);
                }
                if dir == bt_sco_direct::BT_SCO_DIRECT_BT2ARM {
                    src_16 = src_16.add(1);
                } else {
                    dst_16 = dst_16.add(1);
                }
            }
        }
    }
}

unsafe fn btcvsd_tx_clean_buffer(bt: *mut mtk_btcvsd_snd) -> c_int {
    unsafe {
        let band = (*bt).band;
        if band == BT_SCO_BAND::BT_SCO_NB {
            memset((*(*bt).tx).temp_packet_buf.as_mut_ptr() as *mut c_void, 170, SCO_PACKET_180);
        } else {
            memcpy((*(*bt).tx).temp_packet_buf.as_mut_ptr() as *mut c_void, table_msbc_silence.as_ptr() as *const c_void, SCO_PACKET_180);
        }
        let num_valid_addr = (*(*bt).tx).buffer_info.num_valid_addr;
        dev_info!();
        for i in 0..num_valid_addr as usize {
            let dst = (*(*bt).tx).buffer_info.bt_sram_addr[i] as *mut c_void;
            mtk_btcvsd_snd_data_transfer(
                bt_sco_direct::BT_SCO_DIRECT_ARM2BT,
                (*(*bt).tx).temp_packet_buf.as_mut_ptr(),
                dst as *mut u8,
                (*(*bt).tx).buffer_info.packet_length,
                (*(*bt).tx).buffer_info.packet_num,
            );
        }
    }
    0
}

unsafe fn mtk_btcvsd_read_from_bt(bt: *mut mtk_btcvsd_snd, packet_type: bt_sco_packet_len, packet_length: c_uint, packet_num: c_uint, blk_size: c_uint, control: c_uint) -> c_int {
    unsafe {
        let connsys_addr_rx = *(*bt).bt_reg_pkt_r;
        let ap_addr_rx = (*bt).bt_sram_bank2_base as c_ulong + (connsys_addr_rx & 0xffff) as c_ulong;
        if connsys_addr_rx == 0xdeadfeed {
            dev_warn!();
            return -EIO;
        }
        let src = ap_addr_rx as *mut u8;
        mtk_btcvsd_snd_data_transfer(bt_sco_direct::BT_SCO_DIRECT_BT2ARM, src, (*(*bt).rx).temp_packet_buf.as_mut_ptr(), packet_length, packet_num);
        for i in 0..blk_size {
            let mut packet_buf_ofs = (((*(*bt).rx).packet_w & SCO_RX_PACKET_MASK) as usize) * (*(*bt).rx).packet_size as usize;
            memcpy((*bt).rx_packet_buf.as_mut_ptr().add(packet_buf_ofs) as *mut c_void, (*(*bt).rx).temp_packet_buf.as_ptr().add(SCO_RX_PLC_SIZE * i as usize) as *const c_void, SCO_RX_PLC_SIZE);
            let pv: c_int = if (control & btsco_packet_valid_mask[packet_type as usize][i as usize]) == btsco_packet_valid_mask[packet_type as usize][i as usize] { 1 } else { 0 };
            packet_buf_ofs += SCO_RX_PLC_SIZE;
            memcpy((*bt).rx_packet_buf.as_mut_ptr().add(packet_buf_ofs) as *mut c_void, &pv as *const _ as *const c_void, SCO_CVSD_PACKET_VALID_SIZE);
            (*(*bt).rx).packet_w += 1;
        }
    }
    0
}

unsafe fn mtk_btcvsd_write_to_bt(bt: *mut mtk_btcvsd_snd, _packet_type: bt_sco_packet_len, packet_length: c_uint, packet_num: c_uint, blk_size: c_uint) -> c_int {
    unsafe {
        let connsys_addr_tx = *(*bt).bt_reg_pkt_w;
        let ap_addr_tx = (*bt).bt_sram_bank2_base as c_ulong + (connsys_addr_tx & 0xffff) as c_ulong;
        let mut new_ap_addr_tx = true;
        if connsys_addr_tx == 0xdeadfeed {
            dev_warn!();
            return -EIO;
        }
        for i in 0..blk_size as usize {
            let src_ofs = (((*(*bt).tx).packet_r % SCO_TX_PACKER_BUF_NUM as c_int) as usize) * (*(*bt).tx).packet_size as usize;
            memcpy((*(*bt).tx).temp_packet_buf.as_mut_ptr().add((*(*bt).tx).packet_size as usize * i) as *mut c_void, (*bt).tx_packet_buf.as_ptr().add(src_ofs) as *const c_void, (*(*bt).tx).packet_size as usize);
            (*(*bt).tx).packet_r += 1;
        }
        let dst = ap_addr_tx as *mut u8;
        if (*(*bt).tx).mute == 0 {
            mtk_btcvsd_snd_data_transfer(bt_sco_direct::BT_SCO_DIRECT_ARM2BT, (*(*bt).tx).temp_packet_buf.as_mut_ptr(), dst, packet_length, packet_num);
        }
        (*(*bt).tx).buffer_info.packet_length = packet_length;
        (*(*bt).tx).buffer_info.packet_num = packet_num;
        for i in 0..(*(*bt).tx).buffer_info.num_valid_addr as usize {
            if (*(*bt).tx).buffer_info.bt_sram_addr[i] == ap_addr_tx {
                new_ap_addr_tx = false;
                break;
            }
        }
        if new_ap_addr_tx {
            (*(*bt).tx).buffer_info.num_valid_addr += 1;
            let next_idx = ((*(*bt).tx).buffer_info.num_valid_addr - 1) as usize;
            (*(*bt).tx).buffer_info.bt_sram_addr[next_idx] = ap_addr_tx;
            dev_info!();
        }
        if (*(*bt).tx).mute != 0 {
            btcvsd_tx_clean_buffer(bt);
        }
    }
    0
}

unsafe extern "C" fn mtk_btcvsd_snd_irq_handler(_irq_id: c_int, dev: *mut c_void) -> irqreturn_t {
    unsafe {
        let bt = dev as *mut mtk_btcvsd_snd;
        if (*(*bt).rx).state != bt_sco_state::BT_SCO_STATE_RUNNING
            && (*(*bt).rx).state != bt_sco_state::BT_SCO_STATE_ENDING
            && (*(*bt).tx).state != bt_sco_state::BT_SCO_STATE_RUNNING
            && (*(*bt).tx).state != bt_sco_state::BT_SCO_STATE_ENDING
            && (*(*bt).tx).state != bt_sco_state::BT_SCO_STATE_LOOPBACK {
            dev_warn!();
            *(*bt).bt_reg_ctl &= !BT_CVSD_CLEAR;
            return IRQ_HANDLED;
        }
        let control = *(*bt).bt_reg_ctl;
        let packet_type_u = (control >> 18) & 0x7;
        if ((control >> 31) & 1) == 0 {
            dev_warn!();
            *(*bt).bt_reg_ctl &= !BT_CVSD_CLEAR;
            return IRQ_HANDLED;
        }
        if packet_type_u >= bt_sco_packet_len::BT_SCO_CVSD_MAX as c_uint {
            dev_warn!();
            *(*bt).bt_reg_ctl &= !BT_CVSD_CLEAR;
            return IRQ_HANDLED;
        }
        let packet_type: bt_sco_packet_len = core::mem::transmute(packet_type_u as c_int);
        let packet_length = btsco_packet_info[packet_type_u as usize][0];
        let packet_num = btsco_packet_info[packet_type_u as usize][1];
        let buf_cnt_tx = btsco_packet_info[packet_type_u as usize][2];
        let buf_cnt_rx = btsco_packet_info[packet_type_u as usize][3];
        if (*(*bt).tx).state == bt_sco_state::BT_SCO_STATE_LOOPBACK {
            let connsys_addr_rx = *(*bt).bt_reg_pkt_r;
            let ap_addr_rx = (*bt).bt_sram_bank2_base as c_ulong + (connsys_addr_rx & 0xffff) as c_ulong;
            let connsys_addr_tx = *(*bt).bt_reg_pkt_w;
            let ap_addr_tx = (*bt).bt_sram_bank2_base as c_ulong + (connsys_addr_tx & 0xffff) as c_ulong;
            if connsys_addr_tx == 0xdeadfeed || connsys_addr_rx == 0xdeadfeed {
                dev_warn!();
                *(*bt).bt_reg_ctl &= !BT_CVSD_CLEAR;
                return IRQ_HANDLED;
            }
            mtk_btcvsd_snd_data_transfer(bt_sco_direct::BT_SCO_DIRECT_BT2ARM, ap_addr_rx as *mut u8, (*(*bt).tx).temp_packet_buf.as_mut_ptr(), packet_length, packet_num);
            mtk_btcvsd_snd_data_transfer(bt_sco_direct::BT_SCO_DIRECT_ARM2BT, (*(*bt).tx).temp_packet_buf.as_mut_ptr(), ap_addr_tx as *mut u8, packet_length, packet_num);
            (*(*bt).rx).rw_cnt += 1;
            (*(*bt).tx).rw_cnt += 1;
        }
        if (*(*bt).rx).state == bt_sco_state::BT_SCO_STATE_RUNNING || (*(*bt).rx).state == bt_sco_state::BT_SCO_STATE_ENDING {
            if (*(*bt).rx).xrun != 0 && (*(*bt).rx).packet_w - (*(*bt).rx).packet_r <= SCO_RX_PACKER_BUF_NUM as c_int - 2 * buf_cnt_rx as c_int {
                (*(*bt).rx).xrun = 0;
                dev_warn!();
            }
            if (*(*bt).rx).xrun == 0 && ((*(*bt).rx).packet_w - (*(*bt).rx).packet_r <= SCO_RX_PACKER_BUF_NUM as c_int - buf_cnt_rx as c_int) {
                mtk_btcvsd_read_from_bt(bt, packet_type, packet_length, packet_num, buf_cnt_rx, control);
                (*(*bt).rx).rw_cnt += 1;
            } else {
                (*(*bt).rx).xrun = 1;
                dev_warn!();
            }
        }
        (*(*bt).tx).timeout = 0;
        if ((*(*bt).tx).state == bt_sco_state::BT_SCO_STATE_RUNNING || (*(*bt).tx).state == bt_sco_state::BT_SCO_STATE_ENDING) && (*(*bt).tx).trigger_start != 0 {
            if (*(*bt).tx).xrun != 0 && (*(*bt).tx).packet_w - (*(*bt).tx).packet_r >= 2 * buf_cnt_tx as c_int {
                (*(*bt).tx).xrun = 0;
                dev_warn!();
            }
            if ((*(*bt).tx).xrun == 0 && ((*(*bt).tx).packet_w - (*(*bt).tx).packet_r >= buf_cnt_tx as c_int)) || (*(*bt).tx).state == bt_sco_state::BT_SCO_STATE_ENDING {
                mtk_btcvsd_write_to_bt(bt, packet_type, packet_length, packet_num, buf_cnt_tx);
                (*(*bt).tx).rw_cnt += 1;
            } else {
                (*(*bt).tx).xrun = 1;
                dev_warn!();
            }
        }
        *(*bt).bt_reg_ctl &= !BT_CVSD_CLEAR;
        if (*(*bt).rx).state == bt_sco_state::BT_SCO_STATE_RUNNING || (*(*bt).rx).state == bt_sco_state::BT_SCO_STATE_ENDING {
            (*(*bt).rx).wait_flag = 1;
            wake_up_interruptible(&mut (*bt).rx_wait);
            snd_pcm_period_elapsed((*(*bt).rx).substream);
        }
        if (*(*bt).tx).state == bt_sco_state::BT_SCO_STATE_RUNNING || (*(*bt).tx).state == bt_sco_state::BT_SCO_STATE_ENDING {
            (*(*bt).tx).wait_flag = 1;
            wake_up_interruptible(&mut (*bt).tx_wait);
            snd_pcm_period_elapsed((*(*bt).tx).substream);
        }
    }
    IRQ_HANDLED
}

unsafe fn wait_for_bt_irq(bt: *mut mtk_btcvsd_snd, bt_stream: *mut mtk_btcvsd_snd_stream) -> c_int {
    unsafe {
        let timeout_limit: u64 = 22500000;
        let mut max_timeout_trial: c_int = 2;
        (*bt_stream).wait_flag = 0;
        while max_timeout_trial != 0 && (*bt_stream).wait_flag == 0 {
            let t1 = sched_clock();
            let ret = if (*bt_stream).stream == SNDRV_PCM_STREAM_PLAYBACK {
                wait_event_interruptible_timeout(&mut (*bt).tx_wait, (*bt_stream).wait_flag, nsecs_to_jiffies(timeout_limit))
            } else {
                wait_event_interruptible_timeout(&mut (*bt).rx_wait, (*bt_stream).wait_flag, nsecs_to_jiffies(timeout_limit))
            } as c_int;
            let t2 = sched_clock() - t1;
            if t2 > timeout_limit {
                dev_warn!();
            }
            if ret < 0 {
                dev_warn!();
                (*bt_stream).timeout = 1;
                return ret;
            } else if ret == 0 {
                max_timeout_trial -= 1;
                dev_warn!();
                if max_timeout_trial <= 0 {
                    (*bt_stream).timeout = 1;
                    return -ETIME;
                }
            }
        }
    }
    0
}

unsafe fn mtk_btcvsd_snd_read(bt: *mut mtk_btcvsd_snd, buf: *mut iov_iter, mut count: size_t) -> ssize_t {
    unsafe {
        let mut read_count: ssize_t = 0;
        let packet_size = (*(*bt).rx).packet_size as size_t;
        while count != 0 {
            let mut avail = ((*(*bt).rx).packet_w - (*(*bt).rx).packet_r) as c_ulong * packet_size as c_ulong;
            let cur_read_idx = (((*(*bt).rx).packet_r & SCO_RX_PACKET_MASK) as size_t) * packet_size;
            if avail == 0 {
                if wait_for_bt_irq(bt, (*bt).rx) != 0 {
                    return read_count;
                }
                continue;
            }
            if count % packet_size != 0 || avail as size_t % packet_size != 0 {
                dev_warn!();
                count -= count % packet_size;
                avail -= (avail as size_t % packet_size) as c_ulong;
            }
            let mut read_size = if count > avail as size_t { avail as size_t } else { count };
            let cont = (*(*bt).rx).buf_size as size_t - cur_read_idx;
            if read_size > cont {
                read_size = cont;
            }
            if copy_to_iter((*bt).rx_packet_buf.as_ptr().add(cur_read_idx) as *const c_void, read_size, buf) != read_size {
                dev_warn!();
                return -EFAULT as ssize_t;
            }
            (*(*bt).rx).packet_r += (read_size / packet_size) as c_int;
            read_count += read_size as ssize_t;
            count -= read_size;
        }
        (*(*bt).rx).time_stamp = sched_clock();
        (*(*bt).rx).buf_data_equivalent_time = ((*(*bt).rx).packet_w - (*(*bt).rx).packet_r) as u64 * SCO_RX_PLC_SIZE as u64 * 16 * 1000 / 2 / 64;
        (*(*bt).rx).buf_data_equivalent_time += read_count as u64 * SCO_RX_PLC_SIZE as u64 * 16 * 1000 / packet_size as u64 / 2 / 64;
        (*(*bt).rx).buf_data_equivalent_time *= 1000;
        read_count
    }
}

unsafe fn mtk_btcvsd_snd_write(bt: *mut mtk_btcvsd_snd, buf: *mut iov_iter, mut count: size_t) -> ssize_t {
    unsafe {
        let written_size = count as ssize_t;
        let packet_size = (*(*bt).tx).packet_size as size_t;
        (*(*bt).tx).time_stamp = sched_clock();
        (*(*bt).tx).buf_data_equivalent_time = ((*(*bt).tx).packet_w - (*(*bt).tx).packet_r) as u64 * packet_size as u64 * 16 * 1000 / 2 / 64;
        (*(*bt).tx).buf_data_equivalent_time *= 1000;
        while count != 0 {
            let mut avail = (*(*bt).tx).buf_size as c_int - ((*(*bt).tx).packet_w - (*(*bt).tx).packet_r) * packet_size as c_int;
            let cur_write_idx = (((*(*bt).tx).packet_w % SCO_TX_PACKER_BUF_NUM as c_int) as size_t) * packet_size;
            if avail == 0 {
                if wait_for_bt_irq(bt, (*bt).tx) != 0 {
                    return written_size;
                }
                continue;
            }
            if count % packet_size != 0 || avail as size_t % packet_size != 0 {
                dev_warn!();
                count -= count % packet_size;
                avail -= (avail as size_t % packet_size) as c_int;
            }
            let mut write_size = if count > avail as size_t { avail as size_t } else { count };
            let cont = (*(*bt).tx).buf_size as size_t - cur_write_idx;
            if write_size > cont {
                write_size = cont;
            }
            if copy_from_iter((*bt).tx_packet_buf.as_mut_ptr().add(cur_write_idx) as *mut c_void, write_size, buf) != write_size {
                dev_warn!();
                return -EFAULT as ssize_t;
            }
            (*(*bt).tx).packet_w += (write_size / packet_size) as c_int;
            count -= write_size;
        }
        written_size
    }
}

unsafe fn get_bt_stream(bt: *mut mtk_btcvsd_snd, substream: *mut snd_pcm_substream) -> *mut mtk_btcvsd_snd_stream {
    unsafe {
        if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
            (*bt).tx
        } else {
            (*bt).rx
        }
    }
}

static mtk_btcvsd_hardware: snd_pcm_hardware = snd_pcm_hardware {
    info: SNDRV_PCM_INFO_MMAP | SNDRV_PCM_INFO_INTERLEAVED | SNDRV_PCM_INFO_RESUME,
    formats: SNDRV_PCM_FMTBIT_S16_LE,
    buffer_bytes_max: 24 * 1024,
    period_bytes_max: 24 * 1024,
    periods_min: 2,
    periods_max: 16,
    fifo_size: 0,
};

unsafe extern "C" fn mtk_pcm_btcvsd_open(component: *mut snd_soc_component, substream: *mut snd_pcm_substream) -> c_int {
    unsafe {
        let bt = snd_soc_component_get_drvdata(component) as *mut mtk_btcvsd_snd;
        dev_dbg!();
        snd_soc_set_runtime_hwparams(substream, &mtk_btcvsd_hardware);
        if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
            let ret = mtk_btcvsd_snd_tx_init(bt);
            (*(*bt).tx).substream = substream;
            ret
        } else {
            let ret = mtk_btcvsd_snd_rx_init(bt);
            (*(*bt).rx).substream = substream;
            ret
        }
    }
}

unsafe extern "C" fn mtk_pcm_btcvsd_close(component: *mut snd_soc_component, substream: *mut snd_pcm_substream) -> c_int {
    unsafe {
        let bt = snd_soc_component_get_drvdata(component) as *mut mtk_btcvsd_snd;
        let bt_stream = get_bt_stream(bt, substream);
        dev_dbg!();
        mtk_btcvsd_snd_set_state(bt, bt_stream, bt_sco_state::BT_SCO_STATE_IDLE as c_int);
        (*bt_stream).substream = ptr::null_mut();
    }
    0
}

unsafe extern "C" fn mtk_pcm_btcvsd_hw_params(component: *mut snd_soc_component, substream: *mut snd_pcm_substream, hw_params: *mut snd_pcm_hw_params) -> c_int {
    unsafe {
        let bt = snd_soc_component_get_drvdata(component) as *mut mtk_btcvsd_snd;
        if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK && params_buffer_bytes(hw_params) % (*(*bt).tx).packet_size != 0 {
            dev_warn!();
            return -EINVAL;
        }
        (*(*substream).runtime).dma_bytes = params_buffer_bytes(hw_params) as size_t;
    }
    0
}

unsafe extern "C" fn mtk_pcm_btcvsd_hw_free(component: *mut snd_soc_component, substream: *mut snd_pcm_substream) -> c_int {
    unsafe {
        let bt = snd_soc_component_get_drvdata(component) as *mut mtk_btcvsd_snd;
        if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
            btcvsd_tx_clean_buffer(bt);
        }
    }
    0
}

unsafe extern "C" fn mtk_pcm_btcvsd_prepare(component: *mut snd_soc_component, substream: *mut snd_pcm_substream) -> c_int {
    unsafe {
        let bt = snd_soc_component_get_drvdata(component) as *mut mtk_btcvsd_snd;
        let bt_stream = get_bt_stream(bt, substream);
        dev_dbg!();
        mtk_btcvsd_snd_set_state(bt, bt_stream, bt_sco_state::BT_SCO_STATE_RUNNING as c_int);
    }
    0
}

unsafe extern "C" fn mtk_pcm_btcvsd_trigger(component: *mut snd_soc_component, substream: *mut snd_pcm_substream, cmd: c_int) -> c_int {
    unsafe {
        let bt = snd_soc_component_get_drvdata(component) as *mut mtk_btcvsd_snd;
        let bt_stream = get_bt_stream(bt, substream);
        let stream = (*substream).stream;
        dev_dbg!();
        match cmd {
            SNDRV_PCM_TRIGGER_START | SNDRV_PCM_TRIGGER_RESUME => {
                let hw_packet_ptr = if stream == SNDRV_PCM_STREAM_PLAYBACK { (*bt_stream).packet_r } else { (*bt_stream).packet_w };
                (*bt_stream).prev_packet_idx = hw_packet_ptr;
                (*bt_stream).prev_frame = 0;
                (*bt_stream).trigger_start = 1;
                0
            }
            SNDRV_PCM_TRIGGER_STOP | SNDRV_PCM_TRIGGER_SUSPEND => {
                (*bt_stream).trigger_start = 0;
                mtk_btcvsd_snd_set_state(bt, bt_stream, bt_sco_state::BT_SCO_STATE_ENDING as c_int);
                0
            }
            _ => -EINVAL,
        }
    }
}

unsafe extern "C" fn mtk_pcm_btcvsd_pointer(component: *mut snd_soc_component, substream: *mut snd_pcm_substream) -> snd_pcm_uframes_t {
    unsafe {
        let bt = snd_soc_component_get_drvdata(component) as *mut mtk_btcvsd_snd;
        let bt_stream = if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK { (*bt).tx } else { (*bt).rx };
        let hw_packet_ptr = if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK { (*(*bt).tx).packet_r } else { (*(*bt).rx).packet_w };
        let packet_diff = if hw_packet_ptr >= (*bt_stream).prev_packet_idx {
            hw_packet_ptr - (*bt_stream).prev_packet_idx
        } else {
            (INT_MAX - (*bt_stream).prev_packet_idx) + (hw_packet_ptr - INT_MIN) + 1
        };
        (*bt_stream).prev_packet_idx = hw_packet_ptr;
        let byte = packet_diff * (*bt_stream).packet_size as c_int;
        let mut frame = btcvsd_bytes_to_frame(substream, byte) as snd_pcm_uframes_t;
        frame += (*bt_stream).prev_frame;
        frame %= (*(*substream).runtime).buffer_size;
        (*bt_stream).prev_frame = frame;
        frame
    }
}

unsafe extern "C" fn mtk_pcm_btcvsd_copy(component: *mut snd_soc_component, substream: *mut snd_pcm_substream, _channel: c_int, _pos: c_ulong, buf: *mut iov_iter, count: c_ulong) -> c_int {
    unsafe {
        let bt = snd_soc_component_get_drvdata(component) as *mut mtk_btcvsd_snd;
        if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
            mtk_btcvsd_snd_write(bt, buf, count as size_t) as c_int
        } else {
            mtk_btcvsd_snd_read(bt, buf, count as size_t) as c_int
        }
    }
}

static btsco_band_str: [*const c_char; 2] = [b"NB\0".as_ptr() as *const c_char, b"WB\0".as_ptr() as *const c_char];

unsafe extern "C" fn btcvsd_band_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    unsafe {
        let cmpnt = snd_kcontrol_chip(kcontrol);
        let bt = snd_soc_component_get_drvdata(cmpnt) as *mut mtk_btcvsd_snd;
        (*ucontrol).value.integer.value[0] = (*bt).band as c_long;
    }
    0
}

unsafe extern "C" fn btcvsd_band_set(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    unsafe {
        let cmpnt = snd_kcontrol_chip(kcontrol);
        let bt = snd_soc_component_get_drvdata(cmpnt) as *mut mtk_btcvsd_snd;
        let e = (*kcontrol).private_value as *mut soc_enum;
        if (*ucontrol).value.enumerated.item[0] >= (*e).items {
            return -EINVAL;
        }
        (*bt).band = core::mem::transmute::<c_int, BT_SCO_BAND>((*ucontrol).value.integer.value[0] as c_int);
        dev_dbg!();
    }
    0
}

unsafe extern "C" fn btcvsd_loopback_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    unsafe {
        let cmpnt = snd_kcontrol_chip(kcontrol);
        let bt = snd_soc_component_get_drvdata(cmpnt) as *mut mtk_btcvsd_snd;
        let lpbk_en = (*(*bt).tx).state == bt_sco_state::BT_SCO_STATE_LOOPBACK;
        (*ucontrol).value.integer.value[0] = lpbk_en as c_long;
    }
    0
}

unsafe extern "C" fn btcvsd_loopback_set(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    unsafe {
        let cmpnt = snd_kcontrol_chip(kcontrol);
        let bt = snd_soc_component_get_drvdata(cmpnt) as *mut mtk_btcvsd_snd;
        if (*ucontrol).value.integer.value[0] != 0 {
            mtk_btcvsd_snd_set_state(bt, (*bt).tx, bt_sco_state::BT_SCO_STATE_LOOPBACK as c_int);
            mtk_btcvsd_snd_set_state(bt, (*bt).rx, bt_sco_state::BT_SCO_STATE_LOOPBACK as c_int);
        } else {
            mtk_btcvsd_snd_set_state(bt, (*bt).tx, bt_sco_state::BT_SCO_STATE_RUNNING as c_int);
            mtk_btcvsd_snd_set_state(bt, (*bt).rx, bt_sco_state::BT_SCO_STATE_RUNNING as c_int);
        }
    }
    0
}

unsafe extern "C" fn btcvsd_tx_mute_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    unsafe {
        let cmpnt = snd_kcontrol_chip(kcontrol);
        let bt = snd_soc_component_get_drvdata(cmpnt) as *mut mtk_btcvsd_snd;
        if (*bt).tx.is_null() {
            (*ucontrol).value.integer.value[0] = 0;
            return 0;
        }
        (*ucontrol).value.integer.value[0] = (*(*bt).tx).mute as c_long;
    }
    0
}

unsafe extern "C" fn btcvsd_tx_mute_set(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    unsafe {
        let cmpnt = snd_kcontrol_chip(kcontrol);
        let bt = snd_soc_component_get_drvdata(cmpnt) as *mut mtk_btcvsd_snd;
        if (*bt).tx.is_null() {
            return 0;
        }
        (*(*bt).tx).mute = (*ucontrol).value.integer.value[0] as c_uint;
    }
    0
}

unsafe extern "C" fn btcvsd_rx_irq_received_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    unsafe {
        let cmpnt = snd_kcontrol_chip(kcontrol);
        let bt = snd_soc_component_get_drvdata(cmpnt) as *mut mtk_btcvsd_snd;
        if (*bt).rx.is_null() {
            return 0;
        }
        (*ucontrol).value.integer.value[0] = if (*(*bt).rx).rw_cnt != 0 { 1 } else { 0 };
    }
    0
}

unsafe extern "C" fn btcvsd_rx_timeout_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    unsafe {
        let cmpnt = snd_kcontrol_chip(kcontrol);
        let bt = snd_soc_component_get_drvdata(cmpnt) as *mut mtk_btcvsd_snd;
        if (*bt).rx.is_null() {
            return 0;
        }
        (*ucontrol).value.integer.value[0] = (*(*bt).rx).timeout as c_long;
        (*(*bt).rx).timeout = 0;
    }
    0
}

unsafe extern "C" fn btcvsd_rx_timestamp_get(kcontrol: *mut snd_kcontrol, data: *mut c_uint, size: c_uint) -> c_int {
    unsafe {
        let cmpnt = snd_kcontrol_chip(kcontrol);
        let bt = snd_soc_component_get_drvdata(cmpnt) as *mut mtk_btcvsd_snd;
        let mut ret = 0;
        let mut time_buffer_info_rx = mtk_btcvsd_snd_time_buffer_info { data_count_equi_time: 0, time_stamp_us: 0 };
        if size as usize > size_of::<mtk_btcvsd_snd_time_buffer_info>() {
            return -EINVAL;
        }
        get_rx_time_stamp(bt, &mut time_buffer_info_rx);
        dev_dbg!();
        if copy_to_user(data, &time_buffer_info_rx as *const _ as *const c_void, size_of::<mtk_btcvsd_snd_time_buffer_info>()) != 0 {
            dev_warn!();
            ret = -EFAULT;
        }
        ret
    }
}

unsafe extern "C" fn btcvsd_tx_irq_received_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    unsafe {
        let cmpnt = snd_kcontrol_chip(kcontrol);
        let bt = snd_soc_component_get_drvdata(cmpnt) as *mut mtk_btcvsd_snd;
        if (*bt).tx.is_null() {
            return 0;
        }
        (*ucontrol).value.integer.value[0] = if (*(*bt).tx).rw_cnt != 0 { 1 } else { 0 };
    }
    0
}

unsafe extern "C" fn btcvsd_tx_timeout_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    unsafe {
        let cmpnt = snd_kcontrol_chip(kcontrol);
        let bt = snd_soc_component_get_drvdata(cmpnt) as *mut mtk_btcvsd_snd;
        (*ucontrol).value.integer.value[0] = (*(*bt).tx).timeout as c_long;
    }
    0
}

unsafe extern "C" fn btcvsd_tx_timestamp_get(kcontrol: *mut snd_kcontrol, data: *mut c_uint, size: c_uint) -> c_int {
    unsafe {
        let cmpnt = snd_kcontrol_chip(kcontrol);
        let bt = snd_soc_component_get_drvdata(cmpnt) as *mut mtk_btcvsd_snd;
        let mut ret = 0;
        let mut time_buffer_info_tx = mtk_btcvsd_snd_time_buffer_info { data_count_equi_time: 0, time_stamp_us: 0 };
        if size as usize > size_of::<mtk_btcvsd_snd_time_buffer_info>() {
            return -EINVAL;
        }
        get_tx_time_stamp(bt, &mut time_buffer_info_tx);
        dev_dbg!();
        if copy_to_user(data, &time_buffer_info_tx as *const _ as *const c_void, size_of::<mtk_btcvsd_snd_time_buffer_info>()) != 0 {
            dev_warn!();
            ret = -EFAULT;
        }
        ret
    }
}

// The C control array uses ALSA macro constructors:
// SOC_ENUM_EXT, SOC_SINGLE_BOOL_EXT, and SND_SOC_BYTES_TLV.
// Preserve the externally visible declaration shape as an empty local array
// placeholder for those dependency-provided initializers.
static mtk_btcvsd_snd_controls: [snd_kcontrol_new; 0] = [];

unsafe extern "C" fn mtk_btcvsd_snd_component_probe(component: *mut snd_soc_component) -> c_int {
    unsafe {
        snd_soc_add_component_controls(component, mtk_btcvsd_snd_controls.as_ptr(), mtk_btcvsd_snd_controls.len() as c_uint)
    }
}

static mtk_btcvsd_snd_platform: snd_soc_component_driver = snd_soc_component_driver {
    name: BTCVSD_SND_NAME.as_ptr() as *const c_char,
    probe: Some(mtk_btcvsd_snd_component_probe),
    open: Some(mtk_pcm_btcvsd_open),
    close: Some(mtk_pcm_btcvsd_close),
    hw_params: Some(mtk_pcm_btcvsd_hw_params),
    hw_free: Some(mtk_pcm_btcvsd_hw_free),
    prepare: Some(mtk_pcm_btcvsd_prepare),
    trigger: Some(mtk_pcm_btcvsd_trigger),
    pointer: Some(mtk_pcm_btcvsd_pointer),
    copy: Some(mtk_pcm_btcvsd_copy),
};

unsafe extern "C" fn mtk_btcvsd_snd_probe(pdev: *mut platform_device) -> c_int {
    unsafe {
        let mut offset: [u32; 5] = [0; 5];
        let dev = &mut (*pdev).dev as *mut device;
        let btcvsd = devm_kzalloc(dev, size_of::<mtk_btcvsd_snd>(), GFP_KERNEL) as *mut mtk_btcvsd_snd;
        if btcvsd.is_null() {
            return -ENOMEM;
        }
        platform_set_drvdata(pdev, btcvsd as *mut c_void);
        (*btcvsd).dev = dev;
        (*btcvsd).rx = devm_kzalloc((*btcvsd).dev, size_of::<mtk_btcvsd_snd_stream>(), GFP_KERNEL) as *mut mtk_btcvsd_snd_stream;
        if (*btcvsd).rx.is_null() {
            return -ENOMEM;
        }
        (*btcvsd).tx = devm_kzalloc((*btcvsd).dev, size_of::<mtk_btcvsd_snd_stream>(), GFP_KERNEL) as *mut mtk_btcvsd_snd_stream;
        if (*btcvsd).tx.is_null() {
            return -ENOMEM;
        }
        spin_lock_init(&mut (*btcvsd).tx_lock);
        spin_lock_init(&mut (*btcvsd).rx_lock);
        init_waitqueue_head(&mut (*btcvsd).tx_wait);
        init_waitqueue_head(&mut (*btcvsd).rx_wait);
        mtk_btcvsd_snd_tx_init(btcvsd);
        mtk_btcvsd_snd_rx_init(btcvsd);
        let irq_id = platform_get_irq(pdev, 0);
        if irq_id <= 0 {
            return if irq_id < 0 { irq_id } else { -ENXIO };
        }
        let mut ret = devm_request_irq(dev, irq_id, mtk_btcvsd_snd_irq_handler, IRQF_TRIGGER_LOW, b"BTCVSD_ISR_Handle\0".as_ptr() as *const c_char, btcvsd as *mut c_void);
        if ret != 0 {
            dev_err!();
            return ret;
        }
        (*btcvsd).irq_id = irq_id;
        (*btcvsd).bt_pkv_base = of_iomap(ptr::null_mut(), 0);
        if (*btcvsd).bt_pkv_base.is_null() {
            dev_err!();
            return -EIO;
        }
        (*btcvsd).bt_sram_bank2_base = of_iomap(ptr::null_mut(), 1);
        if (*btcvsd).bt_sram_bank2_base.is_null() {
            dev_err!();
            ret = -EIO;
            iounmap((*btcvsd).bt_pkv_base);
            return ret;
        }
        (*btcvsd).infra = syscon_regmap_lookup_by_phandle(ptr::null_mut(), b"mediatek,infracfg\0".as_ptr() as *const c_char);
        // IS_ERR/PTR_ERR are Linux pointer-encoded error helpers supplied externally.
        ret = of_property_read_u32_array(ptr::null_mut(), b"mediatek,offset\0".as_ptr() as *const c_char, offset.as_mut_ptr(), offset.len());
        if ret != 0 {
            dev_warn!();
            iounmap((*btcvsd).bt_sram_bank2_base);
            iounmap((*btcvsd).bt_pkv_base);
            return ret;
        }
        (*btcvsd).infra_misc_offset = offset[0];
        (*btcvsd).conn_bt_cvsd_mask = offset[1];
        (*btcvsd).cvsd_mcu_read_offset = offset[2];
        (*btcvsd).cvsd_mcu_write_offset = offset[3];
        (*btcvsd).cvsd_packet_indicator = offset[4];
        (*btcvsd).bt_reg_pkt_r = ((*btcvsd).bt_pkv_base as *mut u8).add((*btcvsd).cvsd_mcu_read_offset as usize) as *mut u32;
        (*btcvsd).bt_reg_pkt_w = ((*btcvsd).bt_pkv_base as *mut u8).add((*btcvsd).cvsd_mcu_write_offset as usize) as *mut u32;
        (*btcvsd).bt_reg_ctl = ((*btcvsd).bt_pkv_base as *mut u8).add((*btcvsd).cvsd_packet_indicator as usize) as *mut u32;
        mtk_btcvsd_snd_set_state(btcvsd, (*btcvsd).tx, bt_sco_state::BT_SCO_STATE_IDLE as c_int);
        mtk_btcvsd_snd_set_state(btcvsd, (*btcvsd).rx, bt_sco_state::BT_SCO_STATE_IDLE as c_int);
        ret = devm_snd_soc_register_component(dev, &mtk_btcvsd_snd_platform, ptr::null_mut(), 0);
        if ret != 0 {
            iounmap((*btcvsd).bt_sram_bank2_base);
            iounmap((*btcvsd).bt_pkv_base);
            return ret;
        }
    }
    0
}

unsafe extern "C" fn mtk_btcvsd_snd_remove(pdev: *mut platform_device) {
    unsafe {
        let btcvsd = dev_get_drvdata(&mut (*pdev).dev) as *mut mtk_btcvsd_snd;
        iounmap((*btcvsd).bt_pkv_base);
        iounmap((*btcvsd).bt_sram_bank2_base);
    }
}

static mtk_btcvsd_snd_dt_match: [of_device_id; 2] = [
    of_device_id { compatible: b"mediatek,mtk-btcvsd-snd\0".as_ptr() as *const c_char },
    of_device_id { compatible: ptr::null() },
];

static mut mtk_btcvsd_snd_driver: platform_driver = platform_driver {
    driver: platform_driver_driver {
        name: b"mtk-btcvsd-snd\0".as_ptr() as *const c_char,
        of_match_table: mtk_btcvsd_snd_dt_match.as_ptr(),
    },
    probe: Some(mtk_btcvsd_snd_probe),
    remove: Some(mtk_btcvsd_snd_remove),
};

// MODULE_DEVICE_TABLE(of, mtk_btcvsd_snd_dt_match);
// module_platform_driver(mtk_btcvsd_snd_driver);
// MODULE_DESCRIPTION("Mediatek ALSA BT SCO CVSD/MSBC Driver");
// MODULE_AUTHOR("KaiChieh Chuang <kaichieh.chuang@mediatek.com>");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
