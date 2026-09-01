// SPDX-License-Identifier: GPL-2.0-only
/*
 * Audio support for PS3
 * Copyright (C) 2007 Sony Computer Entertainment Inc.
 * All rights reserved.
 * Copyright 2006, 2007 Sony Corporation
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

type u32 = u32;
type u64 = u64;
type uint32_t = u32;
type uint64_t = u64;
type dma_addr_t = u64;
type size_t = usize;
type snd_pcm_uframes_t = u64;
type irqreturn_t = c_int;

const IRQ_HANDLED: irqreturn_t = 1;
const ENXIO: c_int = 6;
const ENOMEM: c_int = 12;
const GFP_KERNEL: c_int = 0;
const PAGE_SHIFT: c_uint = 12;
const PAGE_SIZE: size_t = 1usize << PAGE_SHIFT;
const THIS_MODULE: *mut c_void = ptr::null_mut();
const PS3_BINDING_CPU_ANY: c_int = -1;
const SNDRV_DMA_TYPE_DEV: c_int = 0;
const SNDRV_PCM_STREAM_PLAYBACK: c_int = 0;
const SNDRV_PCM_TRIGGER_START: c_int = 0;
const SNDRV_PCM_TRIGGER_STOP: c_int = 1;
const SNDRV_PCM_HW_PARAM_BUFFER_BYTES: c_int = 0;
const SNDRV_CTL_ELEM_TYPE_IEC958: c_int = 0;
const SNDRV_CTL_ELEM_ACCESS_READ: c_uint = 1;
const SNDRV_CTL_ELEM_IFACE_PCM: c_int = 0;
const SNDRV_PCM_INFO_MMAP: c_uint = 1 << 0;
const SNDRV_PCM_INFO_NONINTERLEAVED: c_uint = 1 << 1;
const SNDRV_PCM_INFO_MMAP_VALID: c_uint = 1 << 2;
const SNDRV_PCM_FMTBIT_S16_BE: u64 = 1 << 0;
const SNDRV_PCM_FMTBIT_S24_BE: u64 = 1 << 1;
const SNDRV_PCM_RATE_44100: c_uint = 1 << 0;
const SNDRV_PCM_RATE_48000: c_uint = 1 << 1;
const SNDRV_PCM_RATE_88200: c_uint = 1 << 2;
const SNDRV_PCM_RATE_96000: c_uint = 1 << 3;
const SNDRV_DEFAULT_IDX1: c_int = -1;
static mut SNDRV_DEFAULT_STR1_VALUE: [c_char; 1] = [0];
const CONFIG_SND_PS3_DEFAULT_START_DELAY: c_int = 0;
const SND_PS3_DRIVER_NAME: *const c_char = b"snd_ps3\0".as_ptr() as *const c_char;

const PS3_AUDIO_FIFO_SIZE: size_t = 0x1000;
const PS3_AUDIO_FIFO_STAGE_SIZE: size_t = 0x80;
const PS3_AUDIO_DMAC_BLOCK_SIZE: size_t = 0x80;
const SND_PS3_DMA_REGION_SIZE: size_t = 0x100000;
const SND_PS3_PCM_PREALLOC_SIZE: size_t = 0x100000;
const PS3_AUDIO_IOID: u64 = 0;
const PS3_MATCH_ID_SOUND: u64 = 0;
const PS3_MODULE_ALIAS_SOUND: *const c_char = b"ps3:sound\0".as_ptr() as *const c_char;

const PS3_AUDIO_CONFIG: c_uint = 0;
const PS3_AUDIO_CONFIG_CLEAR: u32 = 0;
const PS3_AUDIO_INTR_0: c_uint = 0;
const PS3_AUDIO_AX_IS: c_uint = 0;
const PS3_AUDIO_AX_IE: c_uint = 0;
const PS3_AUDIO_INTR_EN_0: c_uint = 0;
const PS3_AUDIO_AX_IC: c_uint = 0;
const PS3_AUDIO_AO_3WMCTRL: c_uint = 0;
const PS3_AUDIO_AO_3WCTRL_BASE: c_uint = 0;
const PS3_AUDIO_AO_SPDCTRL_BASE: c_uint = 0;
const PS3_AUDIO_SOURCE_BASE: c_uint = 0;
const PS3_AUDIO_DEST_BASE: c_uint = 0;
const PS3_AUDIO_DMASIZE_BASE: c_uint = 0;
const PS3_AUDIO_KICK_BASE: c_uint = 0;
const PS3_AUDIO_KICK_STATUS_MASK: u32 = 0;
const PS3_AUDIO_KICK_STATUS_DONE: u32 = 0;
const PS3_AUDIO_KICK_STATUS_NOTIFY: u32 = 0;
const PS3_AUDIO_KICK_STATUS_CLEAR: u32 = 0;
const PS3_AUDIO_KICK_STATUS_ERROR: u32 = 0;
const PS3_AUDIO_KICK_REQUEST: u32 = 0;
const PS3_AUDIO_KICK_EVENT_ALWAYS: u32 = 0;
const PS3_AUDIO_KICK_EVENT_SERIALOUT0_EMPTY: u32 = 0;
const PS3_AUDIO_SOURCE_TARGET_SYSTEM_MEMORY: u32 = 0;
const PS3_AUDIO_DEST_TARGET_AUDIOFIFO: u32 = 0;
const PS3_AUDIO_AX_IC_AASOIMD_MASK: u32 = 0;
const PS3_AUDIO_AX_IC_AASOIMD_EVERY4: u32 = 0;
const PS3_AUDIO_AO_3WMCTRL_ASOBCLKD_DISABLED: u32 = 0;
const PS3_AUDIO_AO_3WMCTRL_ASOLRCKD_DISABLED: u32 = 0;
const PS3_AUDIO_AO_3WMCTRL_ASOPLRCK_DEFAULT: u32 = 0;
const PS3_AUDIO_AO_3WCTRL_ASOBRST_RESET: u32 = 0;
const PS3_AUDIO_AO_3WCTRL_ASODF: u32 = 0;
const PS3_AUDIO_AO_3WCTRL_ASODF_LSB: u32 = 0;
const PS3_AUDIO_AO_SPDCTRL_SPODF: u32 = 0;
const PS3_AUDIO_AO_SPDCTRL_SPODF_LSB: u32 = 0;

const PS3AV_CMD_AUDIO_NUM_OF_CH_2: c_int = 0;
const PS3AV_CMD_AUDIO_FS_44K: c_int = 0;
const PS3AV_CMD_AUDIO_FS_48K: c_int = 0;
const PS3AV_CMD_AUDIO_FS_88K: c_int = 0;
const PS3AV_CMD_AUDIO_FS_96K: c_int = 0;
const PS3AV_CMD_AUDIO_WORD_BITS_16: c_int = 0;
const PS3AV_CMD_AUDIO_WORD_BITS_24: c_int = 0;
const PS3AV_CMD_AUDIO_FORMAT_PCM: c_int = 0;
const PS3AV_CMD_AUDIO_SOURCE_SERIAL: c_int = 0;
const FW_FEATURE_PS3_LV1: c_ulong = 0;

const fn PS3_AUDIO_KICK(ch: c_int) -> c_uint { PS3_AUDIO_KICK_BASE + ch as c_uint * 0x10 }
const fn PS3_AUDIO_SOURCE(ch: c_int) -> c_uint { PS3_AUDIO_SOURCE_BASE + ch as c_uint * 0x10 }
const fn PS3_AUDIO_DEST(ch: c_int) -> c_uint { PS3_AUDIO_DEST_BASE + ch as c_uint * 0x10 }
const fn PS3_AUDIO_DMASIZE(ch: c_int) -> c_uint { PS3_AUDIO_DMASIZE_BASE + ch as c_uint * 0x10 }
const fn PS3_AUDIO_AO_3WCTRL(i: c_int) -> c_uint { PS3_AUDIO_AO_3WCTRL_BASE + i as c_uint * 0x10 }
const fn PS3_AUDIO_AO_SPDCTRL(i: c_int) -> c_uint { PS3_AUDIO_AO_SPDCTRL_BASE + i as c_uint * 0x10 }
const fn PS3_AUDIO_AO_3W_LDATA(_i: c_int) -> u32 { 0 }
const fn PS3_AUDIO_AO_3W_RDATA(_i: c_int) -> u32 { 0 }
const fn PS3_AUDIO_KICK_EVENT_AUDIO_DMA(_ch: c_int) -> u32 { 0 }
const fn PS3_AUDIO_AX_IE_ASOBEIE(_i: c_int) -> u32 { 0 }
const fn PS3_AUDIO_AX_IE_ASOBUIE(_i: c_int) -> u32 { 0 }
const fn PS3_AUDIO_AO_3WMCTRL_ASOEN(_i: c_int) -> u32 { 0 }
const fn PS3_AUDIO_AO_3WMCTRL_ASORUN(_i: c_int) -> u32 { 0 }
const fn DMA_BIT_MASK(n: u32) -> u64 { if n == 64 { !0 } else { (1u64 << n) - 1 } }
const fn ALIGN(x: size_t, a: size_t) -> size_t { (x + a - 1) & !(a - 1) }

#[repr(C)]
#[derive(Copy, Clone)]
enum snd_ps3_ch {
    SND_PS3_CH_L = 0,
    SND_PS3_CH_R = 1,
}

#[repr(C)]
#[derive(Copy, Clone)]
enum snd_ps3_dma_filltype {
    SND_PS3_DMA_FILLTYPE_SILENT_FIRSTFILL = 0,
    SND_PS3_DMA_FILLTYPE_FIRSTFILL = 1,
    SND_PS3_DMA_FILLTYPE_SILENT_RUNNING = 2,
    SND_PS3_DMA_FILLTYPE_RUNNING = 3,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct snd_ps3_avsetting_info {
    avs_audio_ch: c_int,
    avs_audio_rate: c_int,
    avs_audio_width: c_int,
    avs_audio_format: c_int,
    avs_audio_source: c_int,
    avs_cs_info: [u8; 8],
}

#[repr(C)]
struct spinlock_t { _private: [u8; 0] }
#[repr(C)]
struct device { dma_mask: *mut u64 }
#[repr(C)]
struct ps3_mmio_region { bus_addr: u64, lpar_addr: u64, len: u64 }
#[repr(C)]
struct ps3_dma_region { bus_addr: u64, ioid: u64 }
#[repr(C)]
struct ps3_system_bus_device {
    m_region: *mut ps3_mmio_region,
    d_region: *mut ps3_dma_region,
    core: device,
    match_id: c_int,
}
#[repr(C)]
struct snd_card { driver: [c_char; 16], shortname: [c_char; 32], longname: [c_char; 80] }
#[repr(C)]
struct snd_pcm { private_data: *mut c_void, name: [c_char; 80], info_flags: c_uint }
#[repr(C)]
struct snd_pcm_runtime {
    hw: snd_pcm_hardware,
    rate: c_uint,
    format: c_int,
    channels: c_uint,
    dma_bytes: size_t,
    dma_area: *mut u8,
    dma_addr: dma_addr_t,
}
#[repr(C)]
struct snd_pcm_substream { runtime: *mut snd_pcm_runtime }
#[repr(C)]
struct snd_kcontrol { _private: [u8; 0] }
#[repr(C)]
struct snd_ctl_elem_info { type_: c_int, count: c_uint }
#[repr(C)]
struct snd_ctl_iec958 { status: [u8; 24] }
#[repr(C)]
union snd_ctl_value_union { iec958: snd_ctl_iec958 }
#[repr(C)]
struct snd_ctl_elem_value { value: snd_ctl_value_union }

#[repr(C)]
#[derive(Copy, Clone)]
struct snd_pcm_hardware {
    info: c_uint,
    formats: u64,
    rates: c_uint,
    rate_min: c_uint,
    rate_max: c_uint,
    channels_min: c_uint,
    channels_max: c_uint,
    buffer_bytes_max: size_t,
    period_bytes_min: size_t,
    period_bytes_max: size_t,
    periods_min: c_uint,
    periods_max: c_uint,
    fifo_size: size_t,
}

#[repr(C)]
struct snd_kcontrol_new {
    access: c_uint,
    iface: c_int,
    name: *const c_char,
    info: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_info) -> c_int>,
    get: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    put: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
}

#[repr(C)]
struct snd_pcm_ops {
    open: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    close: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    prepare: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    trigger: Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_int) -> c_int>,
    pointer: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> snd_pcm_uframes_t>,
}

#[repr(C)]
struct bus_core_driver { name: *const c_char, owner: *mut c_void }
#[repr(C)]
struct ps3_system_bus_driver {
    match_id: u64,
    probe: Option<unsafe extern "C" fn(*mut ps3_system_bus_device) -> c_int>,
    remove: Option<unsafe extern "C" fn(*mut ps3_system_bus_device)>,
    shutdown: Option<unsafe extern "C" fn(*mut ps3_system_bus_device)>,
    core: bus_core_driver,
}

#[repr(C)]
struct snd_ps3_card_info {
    mapped_mmio_vaddr: *mut u8,
    ps3_dev: *mut ps3_system_bus_device,
    dma_lock: spinlock_t,
    dma_start_bus_addr: [dma_addr_t; 2],
    dma_start_vaddr: [*mut u8; 2],
    dma_next_transfer_vaddr: [*mut u8; 2],
    dma_last_transfer_vaddr: [*mut u8; 2],
    dma_buffer_size: size_t,
    null_buffer_start_vaddr: *mut c_void,
    null_buffer_start_dma_addr: dma_addr_t,
    audio_irq_outlet: u64,
    irq_no: c_uint,
    card: *mut snd_card,
    pcm: *mut snd_pcm,
    substream: *mut snd_pcm_substream,
    avs: snd_ps3_avsetting_info,
    start_delay: c_int,
    running: c_int,
    silent: c_int,
}

extern "C" {
    static mut ps3av_mode_cs_info: [u8; 8];
    fn in_be32(addr: *mut u8) -> u32;
    fn out_be32(addr: *mut u8, val: u32);
    fn in_be64(addr: *mut u64) -> u64;
    fn ioremap(addr: u64, size: u64) -> *mut u8;
    fn iounmap(addr: *mut c_void);
    fn udelay(usecs: c_uint);
    fn wmb();
    fn mb();
    fn ps3av_audio_mute(mute_on: c_int) -> c_int;
    fn ps3av_audio_mute_analog(mute_on: c_int) -> c_int;
    fn ps3av_set_audio_mode(ch: c_int, rate: c_int, width: c_int, format: c_int, source: c_int) -> c_int;
    fn snd_pcm_substream_chip(substream: *mut snd_pcm_substream) -> *mut snd_ps3_card_info;
    fn snd_pcm_format_width(format: c_int) -> c_int;
    fn snd_pcm_format_size(format: c_int, samples: c_uint) -> c_int;
    fn snd_pcm_hw_constraint_step(runtime: *mut snd_pcm_runtime, cond: c_uint, var: c_int, step: c_uint) -> c_int;
    fn snd_pcm_period_elapsed(substream: *mut snd_pcm_substream);
    fn bytes_to_frames(runtime: *mut snd_pcm_runtime, bytes: size_t) -> snd_pcm_uframes_t;
    fn lv1_gpu_device_map(id: c_uint, lpar_addr: *mut u64, lpar_size: *mut u64) -> c_int;
    fn lv1_gpu_device_unmap(id: c_uint) -> c_int;
    fn lv1_gpu_attribute(pkg: u64, attr: u64, value: u64) -> c_int;
    fn ps3_irq_plug_setup(cpu: c_int, outlet: u64, virq: *mut c_uint) -> c_int;
    fn ps3_irq_plug_destroy(virq: c_uint) -> c_int;
    fn request_irq(irq: c_uint, handler: unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t, flags: c_ulong, name: *const c_char, dev: *mut c_void) -> c_int;
    fn free_irq(irq: c_uint, dev: *mut c_void);
    fn ps3_open_hv_device(dev: *mut ps3_system_bus_device) -> c_int;
    fn ps3_close_hv_device(dev: *mut ps3_system_bus_device) -> c_int;
    fn ps3_mmio_region_init(dev: *mut ps3_system_bus_device, region: *mut ps3_mmio_region, addr: u64, size: u64, shift: c_uint);
    fn ps3_dma_region_init(dev: *mut ps3_system_bus_device, region: *mut ps3_dma_region, shift: c_uint, typ: c_int, arg: *mut c_void, size: size_t);
    fn ps3_dma_region_create(region: *mut ps3_dma_region) -> c_int;
    fn ps3_dma_region_free(region: *mut ps3_dma_region);
    fn dma_set_coherent_mask(dev: *mut device, mask: u64) -> c_int;
    fn snd_card_new(dev: *mut device, idx: c_int, id: *mut c_char, module: *mut c_void, extra_size: c_int, card: *mut *mut snd_card) -> c_int;
    fn snd_card_free(card: *mut snd_card);
    fn snd_card_register(card: *mut snd_card) -> c_int;
    fn snd_ctl_add(card: *mut snd_card, kcontrol: *mut snd_kcontrol) -> c_int;
    fn snd_ctl_new1(ncontrol: *const snd_kcontrol_new, private_data: *mut c_void) -> *mut snd_kcontrol;
    fn snd_pcm_new(card: *mut snd_card, id: *const c_char, device: c_int, playback_count: c_int, capture_count: c_int, rpcm: *mut *mut snd_pcm) -> c_int;
    fn snd_pcm_set_ops(pcm: *mut snd_pcm, direction: c_int, ops: *const snd_pcm_ops);
    fn snd_pcm_set_managed_buffer_all(pcm: *mut snd_pcm, typ: c_int, data: *mut device, min: size_t, max: size_t);
    fn dma_alloc_coherent(dev: *mut device, size: size_t, dma_handle: *mut dma_addr_t, flag: c_int) -> *mut c_void;
    fn dma_free_coherent(dev: *mut device, size: size_t, cpu_addr: *mut c_void, dma_handle: dma_addr_t);
    fn firmware_has_feature(feature: c_ulong) -> bool;
    fn ps3_system_bus_driver_register(driver: *mut ps3_system_bus_driver) -> c_int;
    fn ps3_system_bus_driver_unregister(driver: *mut ps3_system_bus_driver);
    fn spin_lock_init(lock: *mut spinlock_t);
    fn spin_lock(lock: *mut spinlock_t);
    fn spin_unlock(lock: *mut spinlock_t);
    fn spin_lock_irqsave(lock: *mut spinlock_t) -> c_ulong;
    fn spin_unlock_irqrestore(lock: *mut spinlock_t, flags: c_ulong);
    fn pr_info(fmt: *const c_char, ...);
    fn pr_debug(fmt: *const c_char, ...);
    fn strscpy(dst: *mut c_char, src: *const c_char) -> isize;
    fn memcpy(dst: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn memcmp(s1: *const c_void, s2: *const c_void, n: size_t) -> c_int;
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
}

/*
 * global
 */
static mut the_card: snd_ps3_card_info = snd_ps3_card_info {
    mapped_mmio_vaddr: ptr::null_mut(),
    ps3_dev: ptr::null_mut(),
    dma_lock: spinlock_t { _private: [] },
    dma_start_bus_addr: [0; 2],
    dma_start_vaddr: [ptr::null_mut(); 2],
    dma_next_transfer_vaddr: [ptr::null_mut(); 2],
    dma_last_transfer_vaddr: [ptr::null_mut(); 2],
    dma_buffer_size: 0,
    null_buffer_start_vaddr: ptr::null_mut(),
    null_buffer_start_dma_addr: 0,
    audio_irq_outlet: 0,
    irq_no: 0,
    card: ptr::null_mut(),
    pcm: ptr::null_mut(),
    substream: ptr::null_mut(),
    avs: snd_ps3_avsetting_info { avs_audio_ch: 0, avs_audio_rate: 0, avs_audio_width: 0, avs_audio_format: 0, avs_audio_source: 0, avs_cs_info: [0; 8] },
    start_delay: 0,
    running: 0,
    silent: 0,
};

static mut snd_ps3_start_delay: c_int = CONFIG_SND_PS3_DEFAULT_START_DELAY;
/* module_param_named(start_delay, snd_ps3_start_delay, uint, 0644);
 * MODULE_PARM_DESC(start_delay, "time to insert silent data in ms");
 */

static mut index: c_int = SNDRV_DEFAULT_IDX1;
static mut id: *mut c_char = unsafe { SNDRV_DEFAULT_STR1_VALUE.as_mut_ptr() };
/* module_param(index, int, 0444);
 * MODULE_PARM_DESC(index, "Index value for PS3 soundchip.");
 * module_param(id, charp, 0444);
 * MODULE_PARM_DESC(id, "ID string for PS3 soundchip.");
 */

/*
 * PS3 audio register access
 */
unsafe fn read_reg(reg: c_uint) -> u32 {
    in_be32(the_card.mapped_mmio_vaddr.add(reg as usize))
}
unsafe fn write_reg(reg: c_uint, val: u32) {
    out_be32(the_card.mapped_mmio_vaddr.add(reg as usize), val);
}
unsafe fn update_reg(reg: c_uint, or_val: u32) {
    let newval = read_reg(reg) | or_val;
    write_reg(reg, newval);
}
unsafe fn update_mask_reg(reg: c_uint, mask: u32, or_val: u32) {
    let newval = (read_reg(reg) & mask) | or_val;
    write_reg(reg, newval);
}

/*
 * ALSA defs
 */
static snd_ps3_pcm_hw: snd_pcm_hardware = snd_pcm_hardware {
    info: SNDRV_PCM_INFO_MMAP | SNDRV_PCM_INFO_NONINTERLEAVED | SNDRV_PCM_INFO_MMAP_VALID,
    formats: SNDRV_PCM_FMTBIT_S16_BE | SNDRV_PCM_FMTBIT_S24_BE,
    rates: SNDRV_PCM_RATE_44100 | SNDRV_PCM_RATE_48000 | SNDRV_PCM_RATE_88200 | SNDRV_PCM_RATE_96000,
    rate_min: 44100,
    rate_max: 96000,
    channels_min: 2, /* stereo only */
    channels_max: 2,
    buffer_bytes_max: PS3_AUDIO_FIFO_SIZE * 64,
    /* interrupt by four stages */
    period_bytes_min: PS3_AUDIO_FIFO_STAGE_SIZE * 4,
    period_bytes_max: PS3_AUDIO_FIFO_STAGE_SIZE * 4,
    periods_min: 16,
    periods_max: 32, /* buffer_size_max/ period_bytes_max */
    fifo_size: PS3_AUDIO_FIFO_SIZE,
};

unsafe fn snd_ps3_verify_dma_stop(_card: *mut snd_ps3_card_info, count: c_int, force_stop: c_int) -> c_int {
    let mut dma_ch: c_int;
    let mut done: c_int;
    let mut retries: c_int;
    let mut stop_forced: c_int = 0;
    let mut status: uint32_t;

    dma_ch = 0;
    while dma_ch < 8 {
        retries = count;
        loop {
            status = read_reg(PS3_AUDIO_KICK(dma_ch)) & PS3_AUDIO_KICK_STATUS_MASK;
            match status {
                PS3_AUDIO_KICK_STATUS_DONE |
                PS3_AUDIO_KICK_STATUS_NOTIFY |
                PS3_AUDIO_KICK_STATUS_CLEAR |
                PS3_AUDIO_KICK_STATUS_ERROR => {
                    done = 1;
                }
                _ => {
                    done = 0;
                    udelay(10);
                }
            }
            if !(done == 0 && { retries -= 1; retries != 0 }) {
                break;
            }
        }
        if retries == 0 && force_stop != 0 {
            pr_info(b"%s: DMA ch %d is not stopped.\0".as_ptr() as *const c_char, b"snd_ps3_verify_dma_stop\0".as_ptr(), dma_ch);
            /* last resort. force to stop dma.
             *  NOTE: this cause DMA done interrupts
             */
            update_reg(PS3_AUDIO_CONFIG, PS3_AUDIO_CONFIG_CLEAR);
            stop_forced = 1;
        }
        dma_ch += 1;
    }
    stop_forced
}

/*
 * wait for all dma is done.
 * NOTE: caller should reset card->running before call.
 *       If not, the interrupt handler will re-start DMA,
 *       then DMA is never stopped.
 */
unsafe fn snd_ps3_wait_for_dma_stop(card: *mut snd_ps3_card_info) {
    let stop_forced: c_int;
    /*
     * wait for the last dma is done
     */

    /*
     * expected maximum DMA done time is 5.7ms + something (DMA itself).
     * 5.7ms is from 16bit/sample 2ch 44.1Khz; the time next
     * DMA kick event would occur.
     */
    stop_forced = snd_ps3_verify_dma_stop(card, 700, 1);

    /*
     * clear outstanding interrupts.
     */
    update_reg(PS3_AUDIO_INTR_0, 0);
    update_reg(PS3_AUDIO_AX_IS, 0);

    /*
     *revert CLEAR bit since it will not reset automatically after DMA stop
     */
    if stop_forced != 0 {
        update_mask_reg(PS3_AUDIO_CONFIG, !PS3_AUDIO_CONFIG_CLEAR, 0);
    }
    /* ensure the hardware sees changes */
    wmb();
}

unsafe fn snd_ps3_kick_dma(_card: *mut snd_ps3_card_info) {
    update_reg(PS3_AUDIO_KICK(0), PS3_AUDIO_KICK_REQUEST);
    /* ensure the hardware sees the change */
    wmb();
}

/*
 * convert virtual addr to ioif bus addr.
 */
unsafe fn v_to_bus(card: *mut snd_ps3_card_info, paddr: *mut u8, ch: c_int) -> dma_addr_t {
    (*card).dma_start_bus_addr[ch as usize] +
        paddr.offset_from((*card).dma_start_vaddr[ch as usize]) as dma_addr_t
}

/*
 * increment ring buffer pointer.
 * NOTE: caller must hold write spinlock
 */
unsafe fn snd_ps3_bump_buffer(card: *mut snd_ps3_card_info, ch: snd_ps3_ch, byte_count: size_t, stage: c_int) {
    let idx = ch as usize;
    if stage == 0 {
        (*card).dma_last_transfer_vaddr[idx] = (*card).dma_next_transfer_vaddr[idx];
    }
    (*card).dma_next_transfer_vaddr[idx] = (*card).dma_next_transfer_vaddr[idx].add(byte_count);
    if (*card).dma_start_vaddr[idx].add((*card).dma_buffer_size / 2) <= (*card).dma_next_transfer_vaddr[idx] {
        (*card).dma_next_transfer_vaddr[idx] = (*card).dma_start_vaddr[idx];
    }
}

/*
 * setup dmac to send data to audio and attenuate samples on the ring buffer
 */
unsafe fn snd_ps3_program_dma(card: *mut snd_ps3_card_info, filltype: snd_ps3_dma_filltype) -> c_int {
    /* this dmac does not support over 4G */
    let mut dma_addr: uint32_t;
    let fill_stages: c_int;
    let mut dma_ch: c_int;
    let mut stage: c_int;
    let mut ch: c_int;
    let mut ch0_kick_event: uint32_t = 0; /* initialize to mute gcc */
    let mut silent: c_int = 0;

    match filltype {
        snd_ps3_dma_filltype::SND_PS3_DMA_FILLTYPE_SILENT_FIRSTFILL => {
            silent = 1;
            ch0_kick_event = PS3_AUDIO_KICK_EVENT_ALWAYS;
        }
        snd_ps3_dma_filltype::SND_PS3_DMA_FILLTYPE_FIRSTFILL => {
            ch0_kick_event = PS3_AUDIO_KICK_EVENT_ALWAYS;
        }
        snd_ps3_dma_filltype::SND_PS3_DMA_FILLTYPE_SILENT_RUNNING => {
            silent = 1;
            ch0_kick_event = PS3_AUDIO_KICK_EVENT_SERIALOUT0_EMPTY;
        }
        snd_ps3_dma_filltype::SND_PS3_DMA_FILLTYPE_RUNNING => {
            ch0_kick_event = PS3_AUDIO_KICK_EVENT_SERIALOUT0_EMPTY;
        }
    }

    snd_ps3_verify_dma_stop(card, 700, 0);
    fill_stages = 4;
    let flags = spin_lock_irqsave(&mut (*card).dma_lock);
    ch = 0;
    while ch < 2 {
        stage = 0;
        while stage < fill_stages {
            dma_ch = stage * 2 + ch;
            if silent != 0 {
                dma_addr = (*card).null_buffer_start_dma_addr as uint32_t;
            } else {
                dma_addr = v_to_bus(card, (*card).dma_next_transfer_vaddr[ch as usize], ch) as uint32_t;
            }

            write_reg(PS3_AUDIO_SOURCE(dma_ch), PS3_AUDIO_SOURCE_TARGET_SYSTEM_MEMORY | dma_addr);

            /* dst: fixed to 3wire#0 */
            if ch == 0 {
                write_reg(PS3_AUDIO_DEST(dma_ch), PS3_AUDIO_DEST_TARGET_AUDIOFIFO | PS3_AUDIO_AO_3W_LDATA(0));
            } else {
                write_reg(PS3_AUDIO_DEST(dma_ch), PS3_AUDIO_DEST_TARGET_AUDIOFIFO | PS3_AUDIO_AO_3W_RDATA(0));
            }

            /* count always 1 DMA block (1/2 stage = 128 bytes) */
            write_reg(PS3_AUDIO_DMASIZE(dma_ch), 0);
            /* bump pointer if needed */
            if silent == 0 {
                snd_ps3_bump_buffer(card, if ch == 0 { snd_ps3_ch::SND_PS3_CH_L } else { snd_ps3_ch::SND_PS3_CH_R }, PS3_AUDIO_DMAC_BLOCK_SIZE, stage);
            }

            /* kick event  */
            if dma_ch == 0 {
                write_reg(PS3_AUDIO_KICK(dma_ch), ch0_kick_event);
            } else {
                write_reg(PS3_AUDIO_KICK(dma_ch), PS3_AUDIO_KICK_EVENT_AUDIO_DMA(dma_ch - 1) | PS3_AUDIO_KICK_REQUEST);
            }
            stage += 1;
        }
        ch += 1;
    }
    spin_unlock_irqrestore(&mut (*card).dma_lock, flags);
    /* ensure the hardware sees the change */
    wmb();

    0
}

/*
 * Interrupt handler
 */
unsafe extern "C" fn snd_ps3_interrupt(_irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    let port_intr: uint32_t;
    let mut underflow_occured: c_int = 0;
    let card = dev_id as *mut snd_ps3_card_info;

    if (*card).running == 0 {
        update_reg(PS3_AUDIO_AX_IS, 0);
        update_reg(PS3_AUDIO_INTR_0, 0);
        return IRQ_HANDLED;
    }

    port_intr = read_reg(PS3_AUDIO_AX_IS);
    /*
     *serial buffer empty detected (every 4 times),
     *program next dma and kick it
     */
    if (port_intr & PS3_AUDIO_AX_IE_ASOBEIE(0)) != 0 {
        write_reg(PS3_AUDIO_AX_IS, PS3_AUDIO_AX_IE_ASOBEIE(0));
        if (port_intr & PS3_AUDIO_AX_IE_ASOBUIE(0)) != 0 {
            write_reg(PS3_AUDIO_AX_IS, port_intr);
            underflow_occured = 1;
        }
        if (*card).silent != 0 {
            /* we are still in silent time */
            snd_ps3_program_dma(card, if underflow_occured != 0 { snd_ps3_dma_filltype::SND_PS3_DMA_FILLTYPE_SILENT_FIRSTFILL } else { snd_ps3_dma_filltype::SND_PS3_DMA_FILLTYPE_SILENT_RUNNING });
            snd_ps3_kick_dma(card);
            (*card).silent -= 1;
        } else {
            snd_ps3_program_dma(card, if underflow_occured != 0 { snd_ps3_dma_filltype::SND_PS3_DMA_FILLTYPE_FIRSTFILL } else { snd_ps3_dma_filltype::SND_PS3_DMA_FILLTYPE_RUNNING });
            snd_ps3_kick_dma(card);
            snd_pcm_period_elapsed((*card).substream);
        }
    } else if (port_intr & PS3_AUDIO_AX_IE_ASOBUIE(0)) != 0 {
        write_reg(PS3_AUDIO_AX_IS, PS3_AUDIO_AX_IE_ASOBUIE(0));
        /*
         * serial out underflow, but buffer empty not detected.
         * in this case, fill fifo with 0 to recover.  After
         * filling dummy data, serial automatically start to
         * consume them and then will generate normal buffer
         * empty interrupts.
         * If both buffer underflow and buffer empty are occurred,
         * it is better to do nomal data transfer than empty one
         */
        snd_ps3_program_dma(card, snd_ps3_dma_filltype::SND_PS3_DMA_FILLTYPE_SILENT_FIRSTFILL);
        snd_ps3_kick_dma(card);
        snd_ps3_program_dma(card, snd_ps3_dma_filltype::SND_PS3_DMA_FILLTYPE_SILENT_FIRSTFILL);
        snd_ps3_kick_dma(card);
    }
    /* clear interrupt cause */
    IRQ_HANDLED
}

/*
 * audio mute on/off
 * mute_on : 0 output enabled
 *           1 mute
 */
unsafe fn snd_ps3_mute(mute_on: c_int) -> c_int {
    ps3av_audio_mute(mute_on)
}

/*
 * av setting
 * NOTE: calling this function may generate audio interrupt.
 */
unsafe fn snd_ps3_change_avsetting(card: *mut snd_ps3_card_info) -> c_int {
    let ret: c_int;
    let mut retries: c_int;
    let mut i: c_int;
    pr_debug(b"%s: start\n\0".as_ptr() as *const c_char, b"snd_ps3_change_avsetting\0".as_ptr());

    ret = ps3av_set_audio_mode((*card).avs.avs_audio_ch, (*card).avs.avs_audio_rate, (*card).avs.avs_audio_width, (*card).avs.avs_audio_format, (*card).avs.avs_audio_source);
    /*
     * Reset the following unwanted settings:
     */

    /* disable all 3wire buffers */
    update_mask_reg(PS3_AUDIO_AO_3WMCTRL,
                    !(PS3_AUDIO_AO_3WMCTRL_ASOEN(0) | PS3_AUDIO_AO_3WMCTRL_ASOEN(1) | PS3_AUDIO_AO_3WMCTRL_ASOEN(2) | PS3_AUDIO_AO_3WMCTRL_ASOEN(3)),
                    0);
    wmb(); /* ensure the hardware sees the change */
    /* wait for actually stopped */
    retries = 1000;
    while (read_reg(PS3_AUDIO_AO_3WMCTRL) &
           (PS3_AUDIO_AO_3WMCTRL_ASORUN(0) | PS3_AUDIO_AO_3WMCTRL_ASORUN(1) | PS3_AUDIO_AO_3WMCTRL_ASORUN(2) | PS3_AUDIO_AO_3WMCTRL_ASORUN(3))) != 0 &&
          { retries -= 1; retries != 0 } {
        udelay(1);
    }

    /* reset buffer pointer */
    i = 0;
    while i < 4 {
        update_reg(PS3_AUDIO_AO_3WCTRL(i), PS3_AUDIO_AO_3WCTRL_ASOBRST_RESET);
        udelay(10);
        i += 1;
    }
    wmb(); /* ensure the hardware actually start resetting */

    /* enable 3wire#0 buffer */
    update_reg(PS3_AUDIO_AO_3WMCTRL, PS3_AUDIO_AO_3WMCTRL_ASOEN(0));

    /* In 24bit mode,ALSA inserts a zero byte at first byte of per sample */
    update_mask_reg(PS3_AUDIO_AO_3WCTRL(0), !PS3_AUDIO_AO_3WCTRL_ASODF, PS3_AUDIO_AO_3WCTRL_ASODF_LSB);
    update_mask_reg(PS3_AUDIO_AO_SPDCTRL(0), !PS3_AUDIO_AO_SPDCTRL_SPODF, PS3_AUDIO_AO_SPDCTRL_SPODF_LSB);
    /* ensure all the setting above is written back to register */
    wmb();
    /* avsetting driver altered AX_IE, caller must reset it if you want */
    pr_debug(b"%s: end\n\0".as_ptr() as *const c_char, b"snd_ps3_change_avsetting\0".as_ptr());
    ret
}

/*
 *  set sampling rate according to the substream
 */
unsafe fn snd_ps3_set_avsetting(substream: *mut snd_pcm_substream) -> c_int {
    let card = snd_pcm_substream_chip(substream);
    let mut avs: snd_ps3_avsetting_info;
    let ret: c_int;

    avs = (*card).avs;

    pr_debug(b"%s: called freq=%d width=%d\n\0".as_ptr() as *const c_char, b"snd_ps3_set_avsetting\0".as_ptr(), (*(*substream).runtime).rate, snd_pcm_format_width((*(*substream).runtime).format));
    pr_debug(b"%s: before freq=%d width=%d\n\0".as_ptr() as *const c_char, b"snd_ps3_set_avsetting\0".as_ptr(), (*card).avs.avs_audio_rate, (*card).avs.avs_audio_width);

    /* sample rate */
    match (*(*substream).runtime).rate {
        44100 => avs.avs_audio_rate = PS3AV_CMD_AUDIO_FS_44K,
        48000 => avs.avs_audio_rate = PS3AV_CMD_AUDIO_FS_48K,
        88200 => avs.avs_audio_rate = PS3AV_CMD_AUDIO_FS_88K,
        96000 => avs.avs_audio_rate = PS3AV_CMD_AUDIO_FS_96K,
        _ => {
            pr_info(b"%s: invalid rate %d\n\0".as_ptr() as *const c_char, b"snd_ps3_set_avsetting\0".as_ptr(), (*(*substream).runtime).rate);
            return 1;
        }
    }

    /* width */
    match snd_pcm_format_width((*(*substream).runtime).format) {
        16 => avs.avs_audio_width = PS3AV_CMD_AUDIO_WORD_BITS_16,
        24 => avs.avs_audio_width = PS3AV_CMD_AUDIO_WORD_BITS_24,
        _ => {
            pr_info(b"%s: invalid width %d\n\0".as_ptr() as *const c_char, b"snd_ps3_set_avsetting\0".as_ptr(), snd_pcm_format_width((*(*substream).runtime).format));
            return 1;
        }
    }

    memcpy(avs.avs_cs_info.as_mut_ptr() as *mut c_void, ps3av_mode_cs_info.as_ptr() as *const c_void, 8);

    if memcmp(&(*card).avs as *const _ as *const c_void, &avs as *const _ as *const c_void, size_of::<snd_ps3_avsetting_info>()) != 0 {
        pr_debug(b"%s: after freq=%d width=%d\n\0".as_ptr() as *const c_char, b"snd_ps3_set_avsetting\0".as_ptr(), (*card).avs.avs_audio_rate, (*card).avs.avs_audio_width);
        (*card).avs = avs;
        snd_ps3_change_avsetting(card);
        ret = 0;
    } else {
        ret = 1;
    }

    /* check CS non-audio bit and mute accordingly */
    if (avs.avs_cs_info[0] & 0x02) != 0 {
        ps3av_audio_mute_analog(1); /* mute if non-audio */
    } else {
        ps3av_audio_mute_analog(0);
    }

    ret
}

/*
 * PCM operators
 */
unsafe extern "C" fn snd_ps3_pcm_open(substream: *mut snd_pcm_substream) -> c_int {
    let runtime = (*substream).runtime;
    let card = snd_pcm_substream_chip(substream);

    /* to retrieve substream/runtime in interrupt handler */
    (*card).substream = substream;

    (*runtime).hw = snd_ps3_pcm_hw;

    (*card).start_delay = snd_ps3_start_delay;

    /* mute off */
    snd_ps3_mute(0); /* this function sleep */

    snd_pcm_hw_constraint_step(runtime, 0, SNDRV_PCM_HW_PARAM_BUFFER_BYTES, (PS3_AUDIO_FIFO_STAGE_SIZE * 4 * 2) as c_uint);
    0
}

unsafe extern "C" fn snd_ps3_pcm_close(_substream: *mut snd_pcm_substream) -> c_int {
    /* mute on */
    snd_ps3_mute(1);
    0
}

unsafe fn snd_ps3_delay_to_bytes(substream: *mut snd_pcm_substream, delay_ms: c_uint) -> c_int {
    let ret: c_int;
    let rate: c_uint;

    rate = (*(*substream).runtime).rate;
    ret = snd_pcm_format_size((*(*substream).runtime).format, rate * delay_ms / 1000) *
        (*(*substream).runtime).channels as c_int;

    pr_debug(b"%s: time=%d rate=%d bytes=%ld, frames=%d, ret=%d\n\0".as_ptr() as *const c_char,
             b"snd_ps3_delay_to_bytes\0".as_ptr(), delay_ms, rate,
             snd_pcm_format_size((*(*substream).runtime).format, rate), rate * delay_ms / 1000, ret);

    ret
}

unsafe extern "C" fn snd_ps3_pcm_prepare(substream: *mut snd_pcm_substream) -> c_int {
    let runtime = (*substream).runtime;
    let card = snd_pcm_substream_chip(substream);

    if snd_ps3_set_avsetting(substream) == 0 {
        /* some parameter changed */
        write_reg(PS3_AUDIO_AX_IE, PS3_AUDIO_AX_IE_ASOBEIE(0) | PS3_AUDIO_AX_IE_ASOBUIE(0));
        /*
         * let SPDIF device re-lock with SPDIF signal,
         * start with some silence
         */
        (*card).silent = snd_ps3_delay_to_bytes(substream, (*card).start_delay as c_uint) /
            (PS3_AUDIO_FIFO_STAGE_SIZE * 4) as c_int; /* every 4 times */
    }

    /* restart ring buffer pointer */
    let flags = spin_lock_irqsave(&mut (*card).dma_lock);
    (*card).dma_buffer_size = (*runtime).dma_bytes;

    (*card).dma_start_vaddr[snd_ps3_ch::SND_PS3_CH_L as usize] = (*runtime).dma_area;
    (*card).dma_next_transfer_vaddr[snd_ps3_ch::SND_PS3_CH_L as usize] = (*card).dma_start_vaddr[snd_ps3_ch::SND_PS3_CH_L as usize];
    (*card).dma_last_transfer_vaddr[snd_ps3_ch::SND_PS3_CH_L as usize] = (*card).dma_next_transfer_vaddr[snd_ps3_ch::SND_PS3_CH_L as usize];
    (*card).dma_start_bus_addr[snd_ps3_ch::SND_PS3_CH_L as usize] = (*runtime).dma_addr;

    (*card).dma_start_vaddr[snd_ps3_ch::SND_PS3_CH_R as usize] = (*runtime).dma_area.add((*runtime).dma_bytes / 2);
    (*card).dma_next_transfer_vaddr[snd_ps3_ch::SND_PS3_CH_R as usize] = (*card).dma_start_vaddr[snd_ps3_ch::SND_PS3_CH_R as usize];
    (*card).dma_last_transfer_vaddr[snd_ps3_ch::SND_PS3_CH_R as usize] = (*card).dma_next_transfer_vaddr[snd_ps3_ch::SND_PS3_CH_R as usize];
    (*card).dma_start_bus_addr[snd_ps3_ch::SND_PS3_CH_R as usize] = (*runtime).dma_addr + ((*runtime).dma_bytes / 2) as dma_addr_t;

    pr_debug(b"%s: vaddr=%p bus=%#llx\n\0".as_ptr() as *const c_char, b"snd_ps3_pcm_prepare\0".as_ptr(), (*card).dma_start_vaddr[snd_ps3_ch::SND_PS3_CH_L as usize], (*card).dma_start_bus_addr[snd_ps3_ch::SND_PS3_CH_L as usize]);
    spin_unlock_irqrestore(&mut (*card).dma_lock, flags);

    /* ensure the hardware sees the change */
    mb();

    0
}

unsafe extern "C" fn snd_ps3_pcm_trigger(substream: *mut snd_pcm_substream, cmd: c_int) -> c_int {
    let card = snd_pcm_substream_chip(substream);

    match cmd {
        SNDRV_PCM_TRIGGER_START => {
            /* clear outstanding interrupts  */
            update_reg(PS3_AUDIO_AX_IS, 0);

            spin_lock(&mut (*card).dma_lock);
            (*card).running = 1;
            spin_unlock(&mut (*card).dma_lock);

            snd_ps3_program_dma(card, snd_ps3_dma_filltype::SND_PS3_DMA_FILLTYPE_SILENT_FIRSTFILL);
            snd_ps3_kick_dma(card);
            while (read_reg(PS3_AUDIO_KICK(7)) & PS3_AUDIO_KICK_STATUS_MASK) != 0 {
                udelay(1);
            }
            snd_ps3_program_dma(card, snd_ps3_dma_filltype::SND_PS3_DMA_FILLTYPE_SILENT_RUNNING);
            snd_ps3_kick_dma(card);
        }
        SNDRV_PCM_TRIGGER_STOP => {
            spin_lock(&mut (*card).dma_lock);
            (*card).running = 0;
            spin_unlock(&mut (*card).dma_lock);
            snd_ps3_wait_for_dma_stop(card);
        }
        _ => {}
    }

    0
}

/*
 * report current pointer
 */
unsafe extern "C" fn snd_ps3_pcm_pointer(substream: *mut snd_pcm_substream) -> snd_pcm_uframes_t {
    let card = snd_pcm_substream_chip(substream);
    let bytes: size_t;
    let ret: snd_pcm_uframes_t;

    spin_lock(&mut (*card).dma_lock);
    bytes = (*card).dma_last_transfer_vaddr[snd_ps3_ch::SND_PS3_CH_L as usize]
        .offset_from((*card).dma_start_vaddr[snd_ps3_ch::SND_PS3_CH_L as usize]) as size_t;
    spin_unlock(&mut (*card).dma_lock);

    ret = bytes_to_frames((*substream).runtime, bytes * 2);

    ret
}

/*
 * SPDIF status bits controls
 */
unsafe extern "C" fn snd_ps3_spdif_mask_info(_kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_IEC958;
    (*uinfo).count = 1;
    0
}

/* FIXME: ps3av_set_audio_mode() assumes only consumer mode */
unsafe extern "C" fn snd_ps3_spdif_cmask_get(_kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    memset((*ucontrol).value.iec958.status.as_mut_ptr() as *mut c_void, 0xff, 8);
    0
}

unsafe extern "C" fn snd_ps3_spdif_pmask_get(_kcontrol: *mut snd_kcontrol, _ucontrol: *mut snd_ctl_elem_value) -> c_int {
    0
}

unsafe extern "C" fn snd_ps3_spdif_default_get(_kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    memcpy((*ucontrol).value.iec958.status.as_mut_ptr() as *mut c_void, ps3av_mode_cs_info.as_ptr() as *const c_void, 8);
    0
}

unsafe extern "C" fn snd_ps3_spdif_default_put(_kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    if memcmp(ps3av_mode_cs_info.as_ptr() as *const c_void, (*ucontrol).value.iec958.status.as_ptr() as *const c_void, 8) != 0 {
        memcpy(ps3av_mode_cs_info.as_mut_ptr() as *mut c_void, (*ucontrol).value.iec958.status.as_ptr() as *const c_void, 8);
        return 1;
    }
    0
}

const SND_CTL_NAME_IEC958_PLAYBACK_CON_MASK: *const c_char = b"IEC958 Playback Con Mask\0".as_ptr() as *const c_char;
const SND_CTL_NAME_IEC958_PLAYBACK_PRO_MASK: *const c_char = b"IEC958 Playback Pro Mask\0".as_ptr() as *const c_char;
const SND_CTL_NAME_IEC958_PLAYBACK_DEFAULT: *const c_char = b"IEC958 Playback Default\0".as_ptr() as *const c_char;

static spdif_ctls: [snd_kcontrol_new; 3] = [
    snd_kcontrol_new {
        access: SNDRV_CTL_ELEM_ACCESS_READ,
        iface: SNDRV_CTL_ELEM_IFACE_PCM,
        name: SND_CTL_NAME_IEC958_PLAYBACK_CON_MASK,
        info: Some(snd_ps3_spdif_mask_info),
        get: Some(snd_ps3_spdif_cmask_get),
        put: None,
    },
    snd_kcontrol_new {
        access: SNDRV_CTL_ELEM_ACCESS_READ,
        iface: SNDRV_CTL_ELEM_IFACE_PCM,
        name: SND_CTL_NAME_IEC958_PLAYBACK_PRO_MASK,
        info: Some(snd_ps3_spdif_mask_info),
        get: Some(snd_ps3_spdif_pmask_get),
        put: None,
    },
    snd_kcontrol_new {
        access: 0,
        iface: SNDRV_CTL_ELEM_IFACE_PCM,
        name: SND_CTL_NAME_IEC958_PLAYBACK_DEFAULT,
        info: Some(snd_ps3_spdif_mask_info),
        get: Some(snd_ps3_spdif_default_get),
        put: Some(snd_ps3_spdif_default_put),
    },
];

static snd_ps3_pcm_spdif_ops: snd_pcm_ops = snd_pcm_ops {
    open: Some(snd_ps3_pcm_open),
    close: Some(snd_ps3_pcm_close),
    prepare: Some(snd_ps3_pcm_prepare),
    trigger: Some(snd_ps3_pcm_trigger),
    pointer: Some(snd_ps3_pcm_pointer),
};

unsafe fn snd_ps3_map_mmio() -> c_int {
    the_card.mapped_mmio_vaddr = ioremap((*(*the_card.ps3_dev).m_region).bus_addr, (*(*the_card.ps3_dev).m_region).len);

    if the_card.mapped_mmio_vaddr.is_null() {
        pr_info(b"%s: ioremap 0 failed p=%#lx l=%#lx \n\0".as_ptr() as *const c_char,
                b"snd_ps3_map_mmio\0".as_ptr(), (*(*the_card.ps3_dev).m_region).lpar_addr, (*(*the_card.ps3_dev).m_region).len);
        return -ENXIO;
    }

    0
}

unsafe fn snd_ps3_unmap_mmio() {
    iounmap(the_card.mapped_mmio_vaddr as *mut c_void);
    the_card.mapped_mmio_vaddr = ptr::null_mut();
}

unsafe fn snd_ps3_allocate_irq() -> c_int {
    let mut ret: c_int;
    let mut lpar_addr: u64 = 0;
    let mut lpar_size: u64 = 0;
    let mapped: *mut u64;

    /* FIXME: move this to device_init (H/W probe) */

    /* get irq outlet */
    ret = lv1_gpu_device_map(1, &mut lpar_addr, &mut lpar_size);
    if ret != 0 {
        pr_info(b"%s: device map 1 failed %d\n\0".as_ptr() as *const c_char, b"snd_ps3_allocate_irq\0".as_ptr(), ret);
        return -ENXIO;
    }

    mapped = ioremap(lpar_addr, lpar_size) as *mut u64;
    if mapped.is_null() {
        pr_info(b"%s: ioremap 1 failed \n\0".as_ptr() as *const c_char, b"snd_ps3_allocate_irq\0".as_ptr());
        return -ENXIO;
    }

    the_card.audio_irq_outlet = in_be64(mapped);

    iounmap(mapped as *mut c_void);
    ret = lv1_gpu_device_unmap(1);
    if ret != 0 {
        pr_info(b"%s: unmap 1 failed\n\0".as_ptr() as *const c_char, b"snd_ps3_allocate_irq\0".as_ptr());
    }

    /* irq */
    ret = ps3_irq_plug_setup(PS3_BINDING_CPU_ANY, the_card.audio_irq_outlet, &mut the_card.irq_no);
    if ret != 0 {
        pr_info(b"%s:ps3_alloc_irq failed (%d)\n\0".as_ptr() as *const c_char, b"snd_ps3_allocate_irq\0".as_ptr(), ret);
        return ret;
    }

    ret = request_irq(the_card.irq_no, snd_ps3_interrupt, 0, SND_PS3_DRIVER_NAME, &mut the_card as *mut _ as *mut c_void);
    if ret != 0 {
        pr_info(b"%s: request_irq failed (%d)\n\0".as_ptr() as *const c_char, b"snd_ps3_allocate_irq\0".as_ptr(), ret);
        ps3_irq_plug_destroy(the_card.irq_no);
        return ret;
    }

    0
}

unsafe fn snd_ps3_free_irq() {
    free_irq(the_card.irq_no, &mut the_card as *mut _ as *mut c_void);
    ps3_irq_plug_destroy(the_card.irq_no);
}

unsafe fn snd_ps3_audio_set_base_addr(ioaddr_start: uint64_t) {
    let val: uint64_t;
    let ret: c_int;

    val = ((ioaddr_start & (0x0fu64 << 32)) >> (32 - 20)) |
        (0x03u64 << 24) |
        (0x0fu64 << 12) |
        PS3_AUDIO_IOID;

    ret = lv1_gpu_attribute(0x100, 0x007, val);
    if ret != 0 {
        pr_info(b"%s: gpu_attribute failed %d\n\0".as_ptr() as *const c_char, b"snd_ps3_audio_set_base_addr\0".as_ptr(), ret);
    }
}

unsafe fn snd_ps3_audio_fixup(_card: *mut snd_ps3_card_info) {
    /*
     * avsetting driver seems to never change the following
     * so, init them here once
     */

    /* no dma interrupt needed */
    write_reg(PS3_AUDIO_INTR_EN_0, 0);

    /* use every 4 buffer empty interrupt */
    update_mask_reg(PS3_AUDIO_AX_IC, PS3_AUDIO_AX_IC_AASOIMD_MASK, PS3_AUDIO_AX_IC_AASOIMD_EVERY4);

    /* enable 3wire clocks */
    update_mask_reg(PS3_AUDIO_AO_3WMCTRL,
                    !(PS3_AUDIO_AO_3WMCTRL_ASOBCLKD_DISABLED | PS3_AUDIO_AO_3WMCTRL_ASOLRCKD_DISABLED),
                    0);
    update_reg(PS3_AUDIO_AO_3WMCTRL, PS3_AUDIO_AO_3WMCTRL_ASOPLRCK_DEFAULT);
}

unsafe fn snd_ps3_init_avsetting(card: *mut snd_ps3_card_info) -> c_int {
    let ret: c_int;
    pr_debug(b"%s: start\n\0".as_ptr() as *const c_char, b"snd_ps3_init_avsetting\0".as_ptr());
    (*card).avs.avs_audio_ch = PS3AV_CMD_AUDIO_NUM_OF_CH_2;
    (*card).avs.avs_audio_rate = PS3AV_CMD_AUDIO_FS_48K;
    (*card).avs.avs_audio_width = PS3AV_CMD_AUDIO_WORD_BITS_16;
    (*card).avs.avs_audio_format = PS3AV_CMD_AUDIO_FORMAT_PCM;
    (*card).avs.avs_audio_source = PS3AV_CMD_AUDIO_SOURCE_SERIAL;
    memcpy((*card).avs.avs_cs_info.as_mut_ptr() as *mut c_void, ps3av_mode_cs_info.as_ptr() as *const c_void, 8);

    ret = snd_ps3_change_avsetting(card);

    snd_ps3_audio_fixup(card);

    /* to start to generate SPDIF signal, fill data */
    snd_ps3_program_dma(card, snd_ps3_dma_filltype::SND_PS3_DMA_FILLTYPE_SILENT_FIRSTFILL);
    snd_ps3_kick_dma(card);
    pr_debug(b"%s: end\n\0".as_ptr() as *const c_char, b"snd_ps3_init_avsetting\0".as_ptr());
    ret
}

unsafe extern "C" fn snd_ps3_driver_probe(dev: *mut ps3_system_bus_device) -> c_int {
    let mut i: c_int;
    let mut ret: c_int;
    let mut lpar_addr: u64 = 0;
    let mut lpar_size: u64 = 0;
    static mut dummy_mask: u64 = 0;

    the_card.ps3_dev = dev;

    ret = ps3_open_hv_device(dev);

    if ret != 0 {
        return -ENXIO;
    }

    /* setup MMIO */
    ret = lv1_gpu_device_map(2, &mut lpar_addr, &mut lpar_size);
    if ret != 0 {
        pr_info(b"%s: device map 2 failed %d\n\0".as_ptr() as *const c_char, b"snd_ps3_driver_probe\0".as_ptr(), ret);
        ps3_close_hv_device(dev);
        return ret;
    }
    ps3_mmio_region_init(dev, (*dev).m_region, lpar_addr, lpar_size, PAGE_SHIFT);

    ret = snd_ps3_map_mmio();
    if ret != 0 {
        lv1_gpu_device_unmap(2);
        ps3_close_hv_device(dev);
        return ret;
    }

    /* setup DMA area */
    ps3_dma_region_init(dev, (*dev).d_region,
                        PAGE_SHIFT, /* use system page size */
                        0, /* dma type; not used */
                        ptr::null_mut(),
                        ALIGN(SND_PS3_DMA_REGION_SIZE, PAGE_SIZE));
    (*(*dev).d_region).ioid = PS3_AUDIO_IOID;

    ret = ps3_dma_region_create((*dev).d_region);
    if ret != 0 {
        pr_info(b"%s: region_create\n\0".as_ptr() as *const c_char, b"snd_ps3_driver_probe\0".as_ptr());
        snd_ps3_unmap_mmio();
        lv1_gpu_device_unmap(2);
        ps3_close_hv_device(dev);
        return ret;
    }

    dummy_mask = DMA_BIT_MASK(32);
    (*dev).core.dma_mask = &mut dummy_mask;
    dma_set_coherent_mask(&mut (*dev).core, dummy_mask);

    snd_ps3_audio_set_base_addr((*(*dev).d_region).bus_addr);

    /* CONFIG_SND_PS3_DEFAULT_START_DELAY */
    the_card.start_delay = snd_ps3_start_delay;

    /* irq */
    if snd_ps3_allocate_irq() != 0 {
        ret = -ENXIO;
        ps3_dma_region_free((*dev).d_region);
        snd_ps3_unmap_mmio();
        lv1_gpu_device_unmap(2);
        ps3_close_hv_device(dev);
        return ret;
    }

    /* create card instance */
    ret = snd_card_new(&mut (*dev).core, index, id, THIS_MODULE, 0, &mut the_card.card);
    if ret < 0 {
        snd_ps3_free_irq();
        ps3_dma_region_free((*dev).d_region);
        snd_ps3_unmap_mmio();
        lv1_gpu_device_unmap(2);
        ps3_close_hv_device(dev);
        return ret;
    }

    strscpy((*the_card.card).driver.as_mut_ptr(), b"PS3\0".as_ptr() as *const c_char);
    strscpy((*the_card.card).shortname.as_mut_ptr(), b"PS3\0".as_ptr() as *const c_char);
    strscpy((*the_card.card).longname.as_mut_ptr(), b"PS3 sound\0".as_ptr() as *const c_char);

    /* create control elements */
    i = 0;
    while i < spdif_ctls.len() as c_int {
        ret = snd_ctl_add(the_card.card, snd_ctl_new1(&spdif_ctls[i as usize], &mut the_card as *mut _ as *mut c_void));
        if ret < 0 {
            snd_card_free(the_card.card);
            snd_ps3_free_irq();
            ps3_dma_region_free((*dev).d_region);
            snd_ps3_unmap_mmio();
            lv1_gpu_device_unmap(2);
            ps3_close_hv_device(dev);
            return ret;
        }
        i += 1;
    }

    /* create PCM devices instance */
    /* NOTE:this driver works assuming pcm:substream = 1:1 */
    ret = snd_pcm_new(the_card.card, b"SPDIF\0".as_ptr() as *const c_char,
                      0, /* instance index, will be stored pcm.device*/
                      1, /* output substream */
                      0, /* input substream */
                      &mut the_card.pcm);
    if ret != 0 {
        snd_card_free(the_card.card);
        snd_ps3_free_irq();
        ps3_dma_region_free((*dev).d_region);
        snd_ps3_unmap_mmio();
        lv1_gpu_device_unmap(2);
        ps3_close_hv_device(dev);
        return ret;
    }

    (*the_card.pcm).private_data = &mut the_card as *mut _ as *mut c_void;
    strscpy((*the_card.pcm).name.as_mut_ptr(), b"SPDIF\0".as_ptr() as *const c_char);

    /* set pcm ops */
    snd_pcm_set_ops(the_card.pcm, SNDRV_PCM_STREAM_PLAYBACK, &snd_ps3_pcm_spdif_ops);

    (*the_card.pcm).info_flags = SNDRV_PCM_INFO_NONINTERLEAVED;
    /* pre-alloc PCM DMA buffer*/
    snd_pcm_set_managed_buffer_all(the_card.pcm, SNDRV_DMA_TYPE_DEV, &mut (*dev).core, SND_PS3_PCM_PREALLOC_SIZE, SND_PS3_PCM_PREALLOC_SIZE);

    /*
     * allocate null buffer
     * its size should be lager than PS3_AUDIO_FIFO_STAGE_SIZE * 2
     * PAGE_SIZE is enogh
     */
    the_card.null_buffer_start_vaddr = dma_alloc_coherent(&mut (*the_card.ps3_dev).core,
                                                          PAGE_SIZE,
                                                          &mut the_card.null_buffer_start_dma_addr,
                                                          GFP_KERNEL);
    if the_card.null_buffer_start_vaddr.is_null() {
        pr_info(b"%s: nullbuffer alloc failed\n\0".as_ptr() as *const c_char, b"snd_ps3_driver_probe\0".as_ptr());
        ret = -ENOMEM;
        snd_card_free(the_card.card);
        snd_ps3_free_irq();
        ps3_dma_region_free((*dev).d_region);
        snd_ps3_unmap_mmio();
        lv1_gpu_device_unmap(2);
        ps3_close_hv_device(dev);
        return ret;
    }
    pr_debug(b"%s: null vaddr=%p dma=%#llx\n\0".as_ptr() as *const c_char, b"snd_ps3_driver_probe\0".as_ptr(), the_card.null_buffer_start_vaddr, the_card.null_buffer_start_dma_addr);
    /* set default sample rate/word width */
    snd_ps3_init_avsetting(&mut the_card);

    /* register the card */
    ret = snd_card_register(the_card.card);
    if ret < 0 {
        dma_free_coherent(&mut (*the_card.ps3_dev).core, PAGE_SIZE, the_card.null_buffer_start_vaddr, the_card.null_buffer_start_dma_addr);
        snd_card_free(the_card.card);
        snd_ps3_free_irq();
        ps3_dma_region_free((*dev).d_region);
        snd_ps3_unmap_mmio();
        lv1_gpu_device_unmap(2);
        ps3_close_hv_device(dev);
        return ret;
    }

    pr_info(b"%s started. start_delay=%dms\n\0".as_ptr() as *const c_char, (*the_card.card).longname.as_ptr(), the_card.start_delay);
    0
} /* snd_ps3_probe */

/* called when module removal */
unsafe extern "C" fn snd_ps3_driver_remove(dev: *mut ps3_system_bus_device) {
    pr_info(b"%s:start id=%d\n\0".as_ptr() as *const c_char, b"snd_ps3_driver_remove\0".as_ptr(), (*dev).match_id);

    /*
     * ctl and preallocate buffer will be freed in
     * snd_card_free
     */
    snd_card_free(the_card.card);

    dma_free_coherent(&mut (*dev).core,
                      PAGE_SIZE,
                      the_card.null_buffer_start_vaddr,
                      the_card.null_buffer_start_dma_addr);

    ps3_dma_region_free((*dev).d_region);

    snd_ps3_free_irq();
    snd_ps3_unmap_mmio();

    lv1_gpu_device_unmap(2);
    ps3_close_hv_device(dev);
    pr_info(b"%s:end id=%d\n\0".as_ptr() as *const c_char, b"snd_ps3_driver_remove\0".as_ptr(), (*dev).match_id);
} /* snd_ps3_remove */

static mut snd_ps3_bus_driver_info: ps3_system_bus_driver = ps3_system_bus_driver {
    match_id: PS3_MATCH_ID_SOUND,
    probe: Some(snd_ps3_driver_probe),
    remove: Some(snd_ps3_driver_remove),
    shutdown: Some(snd_ps3_driver_remove),
    core: bus_core_driver {
        name: SND_PS3_DRIVER_NAME,
        owner: THIS_MODULE,
    },
};

/*
 * module/subsystem initialize/terminate
 */
unsafe fn snd_ps3_init() -> c_int {
    let ret: c_int;

    if !firmware_has_feature(FW_FEATURE_PS3_LV1) {
        return -ENXIO;
    }

    memset(&mut the_card as *mut _ as *mut c_void, 0, size_of::<snd_ps3_card_info>());
    spin_lock_init(&mut the_card.dma_lock);

    /* register systembus DRIVER, this calls our probe() func */
    ret = ps3_system_bus_driver_register(&mut snd_ps3_bus_driver_info);

    ret
}
/* module_init(snd_ps3_init); */

unsafe fn snd_ps3_exit() {
    ps3_system_bus_driver_unregister(&mut snd_ps3_bus_driver_info);
}
/* module_exit(snd_ps3_exit);
 *
 * MODULE_LICENSE("GPL v2");
 * MODULE_DESCRIPTION("PS3 sound driver");
 * MODULE_AUTHOR("Sony Computer Entertainment Inc.");
 * MODULE_ALIAS(PS3_MODULE_ALIAS_SOUND);
 */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
