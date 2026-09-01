// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *   ALSA driver for ATI IXP 150/200/250/300 AC97 controllers
 *
 *	Copyright (c) 2004 Takashi Iwai <tiwai@suse.de>
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

type u16 = u16;
type u32 = u32;
type __le32 = u32;
type bool_ = bool;
type snd_pcm_uframes_t = c_ulong;
type irqreturn_t = c_int;

const MODULE_AUTHOR_TEXT: &[u8] = b"Takashi Iwai <tiwai@suse.de>\0";
const MODULE_DESCRIPTION_TEXT: &[u8] = b"ATI IXP AC97 controller\0";
const MODULE_LICENSE_TEXT: &[u8] = b"GPL\0";

extern "C" {
    static mut SNDRV_DEFAULT_IDX1: c_int;
    static mut SNDRV_DEFAULT_STR1: *mut c_char;
    static mut THIS_MODULE: *mut c_void;
    static mut KBUILD_MODNAME: *const c_char;

    fn readl(addr: *mut c_void) -> c_uint;
    fn writel(value: c_uint, addr: *mut c_void);
    fn udelay(usecs: c_ulong);
    fn mdelay(msecs: c_ulong);

    fn snd_dma_alloc_pages(
        ty: c_int,
        dev: *mut device,
        size: c_ulong,
        dmab: *mut snd_dma_buffer,
    ) -> c_int;
    fn snd_dma_free_pages(dmab: *mut snd_dma_buffer);
    fn snd_pci_quirk_lookup(
        pci: *mut pci_dev,
        list: *const snd_pci_quirk,
    ) -> *const snd_pci_quirk;
    fn snd_pci_quirk_name(q: *const snd_pci_quirk) -> *const c_char;
    fn snd_pcm_substream_chip(substream: *mut snd_pcm_substream) -> *mut atiixp;
    fn bytes_to_frames(runtime: *mut snd_pcm_runtime, bytes: c_uint) -> snd_pcm_uframes_t;
    fn snd_pcm_stop_xrun(substream: *mut snd_pcm_substream);
    fn snd_pcm_period_elapsed(substream: *mut snd_pcm_substream);
    fn snd_BUG_ON(cond: bool) -> c_int;
    fn params_buffer_bytes(hw_params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_periods(hw_params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_period_bytes(hw_params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_rate(hw_params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_channels(hw_params: *mut snd_pcm_hw_params) -> c_uint;
    fn snd_ac97_pcm_close(pcm: *mut ac97_pcm);
    fn snd_ac97_pcm_open(
        pcm: *mut ac97_pcm,
        rate: c_uint,
        channels: c_uint,
        slots: c_uint,
    ) -> c_int;
    fn snd_pcm_limit_hw_rates(runtime: *mut snd_pcm_runtime);
    fn snd_pcm_hw_constraint_integer(runtime: *mut snd_pcm_runtime, param: c_int) -> c_int;
    fn snd_pcm_hw_constraint_step(
        runtime: *mut snd_pcm_runtime,
        cond: c_uint,
        param: c_int,
        step: c_uint,
    ) -> c_int;
    fn snd_ac97_pcm_assign(
        bus: *mut snd_ac97_bus,
        nums: c_int,
        pcms: *const ac97_pcm,
    ) -> c_int;
    fn snd_pcm_new(
        card: *mut snd_card,
        id: *const c_char,
        device: c_int,
        playback_count: c_int,
        capture_count: c_int,
        rpcm: *mut *mut snd_pcm,
    ) -> c_int;
    fn snd_pcm_set_ops(pcm: *mut snd_pcm, stream: c_int, ops: *const snd_pcm_ops);
    fn strscpy(dst: *mut c_char, src: *const c_char) -> c_ulong;
    fn snd_pcm_set_managed_buffer_all(
        pcm: *mut snd_pcm,
        ty: c_int,
        dev: *mut device,
        min: c_ulong,
        max: c_ulong,
    );
    fn snd_pcm_add_chmap_ctls(
        pcm: *mut snd_pcm,
        stream: c_int,
        chmaps: *const c_void,
        channels: c_int,
        device: c_int,
        chmap: *mut *mut snd_pcm_chmap,
    ) -> c_int;
    static mut snd_pcm_alt_chmaps: *const c_void;
    fn snd_ac97_update_bits(ac97: *mut snd_ac97, reg: c_uint, mask: c_uint, value: c_uint) -> c_int;
    fn snd_ac97_bus(
        card: *mut snd_card,
        num: c_int,
        ops: *const snd_ac97_bus_ops,
        private_data: *mut c_void,
        rbus: *mut *mut snd_ac97_bus,
    ) -> c_int;
    fn snd_ac97_mixer(
        bus: *mut snd_ac97_bus,
        template: *mut snd_ac97_template,
        rac97: *mut *mut snd_ac97,
    ) -> c_int;
    fn snd_ac97_tune_hardware(
        ac97: *mut snd_ac97,
        quirks: *const ac97_quirk,
        override_: *const c_char,
    );
    fn snd_power_change_state(card: *mut snd_card, state: c_int);
    fn snd_ac97_suspend(ac97: *mut snd_ac97);
    fn snd_ac97_resume(ac97: *mut snd_ac97);
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn snd_iprintf(buffer: *mut snd_info_buffer, fmt: *const c_char, ...);
    fn snd_card_ro_proc_new(
        card: *mut snd_card,
        name: *const c_char,
        data: *mut c_void,
        read: Option<unsafe extern "C" fn(*mut snd_info_entry, *mut snd_info_buffer)>,
    );
    fn pcim_enable_device(pci: *mut pci_dev) -> c_int;
    fn spin_lock_init(lock: *mut spinlock_t);
    fn mutex_init(mutex: *mut mutex);
    fn pcim_iomap_region(pci: *mut pci_dev, bar: c_int, name: *const c_char) -> *mut c_void;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn pci_resource_start(pci: *mut pci_dev, bar: c_int) -> c_ulong;
    fn devm_request_irq(
        dev: *mut device,
        irq: c_int,
        handler: Option<unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t>,
        flags: c_ulong,
        name: *const c_char,
        dev_id: *mut c_void,
    ) -> c_int;
    fn pci_set_master(pci: *mut pci_dev);
    fn snd_devm_card_new(
        dev: *mut device,
        idx: c_int,
        xid: *mut c_char,
        module: *mut c_void,
        extra_size: usize,
        card_ret: *mut *mut snd_card,
    ) -> c_int;
    fn snprintf(buf: *mut c_char, size: usize, fmt: *const c_char, ...) -> c_int;
    fn snd_ac97_get_short_name(ac97: *mut snd_ac97) -> *const c_char;
    fn snd_card_register(card: *mut snd_card) -> c_int;
    fn pci_set_drvdata(pci: *mut pci_dev, data: *mut c_void);
    fn snd_card_free_on_error(dev: *mut device, err: c_int) -> c_int;
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_info_buffer {
    _private: [u8; 0],
}
#[repr(C)]
pub struct spinlock_t {
    _private: [u8; 0],
}
#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pci_dev {
    dev: device,
    irq: c_int,
    revision: c_uint,
}

#[repr(C)]
pub struct pci_device_id {
    vendor: c_uint,
    device: c_uint,
    subvendor: c_uint,
    subdevice: c_uint,
    class: c_uint,
    class_mask: c_uint,
    driver_data: c_ulong,
}

#[repr(C)]
pub struct snd_pci_quirk {
    subvendor: c_uint,
    subdevice: c_uint,
    name: *const c_char,
    value: c_int,
}

#[repr(C)]
pub struct snd_dma_buffer {
    area: *mut c_void,
    addr: c_ulong,
}

#[repr(C)]
pub struct snd_pcm_runtime {
    dma_addr: c_uint,
    hw: snd_pcm_hardware,
    private_data: *mut c_void,
    format: c_int,
    channels: c_uint,
}

#[repr(C)]
pub struct snd_pcm_substream {
    runtime: *mut snd_pcm_runtime,
    ops: *const snd_pcm_ops,
}

#[repr(C)]
pub struct snd_pcm_ops {
    open: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    close: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params) -> c_int>,
    hw_free: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    prepare: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    trigger: Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_int) -> c_int>,
    pointer: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> snd_pcm_uframes_t>,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_pcm_hardware {
    info: c_uint,
    formats: c_ulong,
    rates: c_uint,
    rate_min: c_uint,
    rate_max: c_uint,
    channels_min: c_uint,
    channels_max: c_uint,
    buffer_bytes_max: c_ulong,
    period_bytes_min: c_uint,
    period_bytes_max: c_uint,
    periods_min: c_uint,
    periods_max: c_uint,
}

#[repr(C)]
pub struct snd_pcm {
    private_data: *mut c_void,
    name: [c_char; 80],
}

#[repr(C)]
pub struct snd_pcm_chmap {
    channel_mask: c_uint,
}

#[repr(C)]
pub struct ac97_pcm_region {
    slots: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ac97_pcm {
    stream: c_uint,
    exclusive: c_uint,
    spdif: c_uint,
    rates: c_uint,
    r: [ac97_pcm_region; 4],
}

#[repr(C)]
pub struct snd_ac97_bus {
    clock: c_int,
    pcms: *mut ac97_pcm,
}

#[repr(C)]
pub struct snd_ac97 {
    private_data: *mut atiixp,
    num: c_ushort,
    chmaps: [*mut snd_pcm_chmap; 2],
}
type c_ushort = u16;

#[repr(C)]
pub struct snd_ac97_bus_ops {
    write: Option<unsafe extern "C" fn(*mut snd_ac97, c_ushort, c_ushort)>,
    read: Option<unsafe extern "C" fn(*mut snd_ac97, c_ushort) -> c_ushort>,
}

#[repr(C)]
pub struct snd_ac97_template {
    private_data: *mut atiixp,
    pci: *mut pci_dev,
    num: c_int,
    scaps: c_uint,
}

#[repr(C)]
pub struct ac97_quirk {
    subvendor: c_uint,
    subdevice: c_uint,
    name: *const c_char,
    type_: c_int,
}

#[repr(C)]
pub struct snd_info_entry {
    private_data: *mut atiixp,
}

#[repr(C)]
pub struct snd_card {
    dev: *mut device,
    private_data: *mut atiixp,
    sync_irq: c_int,
    private_free: Option<unsafe extern "C" fn(*mut snd_card)>,
    driver: [c_char; 16],
    shortname: [c_char; 32],
    longname: [c_char; 80],
}

static mut index: c_int = 0; /* Index 0-MAX */
static mut id: *mut c_char = ptr::null_mut(); /* ID for this card */
static mut ac97_clock: c_int = 48000;
static mut ac97_quirk: *mut c_char = ptr::null_mut();
static mut spdif_aclink: bool_ = true;
static mut ac97_codec: c_int = -1;

/* just for backward compatibility */
static mut enable: bool_ = false;

const ENOMEM: c_int = 12;
const EBUSY: c_int = 16;
const EINVAL: c_int = 22;
const ENXIO: c_int = 6;
const ENODEV: c_int = 19;
const IRQ_NONE: irqreturn_t = 0;
const IRQ_HANDLED: irqreturn_t = 1;
const IRQF_SHARED: c_ulong = 0x80;

const SNDRV_DMA_TYPE_DEV: c_int = 0;
const SNDRV_PCM_TRIGGER_START: c_int = 0;
const SNDRV_PCM_TRIGGER_STOP: c_int = 1;
const SNDRV_PCM_TRIGGER_PAUSE_PUSH: c_int = 3;
const SNDRV_PCM_TRIGGER_PAUSE_RELEASE: c_int = 4;
const SNDRV_PCM_TRIGGER_SUSPEND: c_int = 5;
const SNDRV_PCM_TRIGGER_RESUME: c_int = 6;
const SNDRV_PCM_FORMAT_S16_LE: c_int = 2;
const SNDRV_PCM_FMTBIT_S16_LE: c_ulong = 1 << SNDRV_PCM_FORMAT_S16_LE;
const SNDRV_PCM_FMTBIT_S32_LE: c_ulong = 1 << 10;
const SNDRV_PCM_FMTBIT_IEC958_SUBFRAME_LE: c_ulong = 1 << 18;
const SNDRV_PCM_RATE_48000: c_uint = 1 << 10;
const SNDRV_PCM_INFO_MMAP: c_uint = 1 << 0;
const SNDRV_PCM_INFO_INTERLEAVED: c_uint = 1 << 1;
const SNDRV_PCM_INFO_BLOCK_TRANSFER: c_uint = 1 << 2;
const SNDRV_PCM_INFO_PAUSE: c_uint = 1 << 3;
const SNDRV_PCM_INFO_RESUME: c_uint = 1 << 4;
const SNDRV_PCM_INFO_MMAP_VALID: c_uint = 1 << 5;
const SNDRV_PCM_HW_PARAM_PERIODS: c_int = 0;
const SNDRV_PCM_HW_PARAM_CHANNELS: c_int = 1;
const SNDRV_PCM_STREAM_PLAYBACK: c_int = 0;
const SNDRV_PCM_STREAM_CAPTURE: c_int = 1;
const SND_PCM_CHMAP_MASK_2468: c_uint = 0;
const SNDRV_CTL_POWER_D3hot: c_int = 3;
const SNDRV_CTL_POWER_D0: c_int = 0;

const AC97_SLOT_PCM_LEFT: c_uint = 3;
const AC97_SLOT_PCM_RIGHT: c_uint = 4;
const AC97_SLOT_PCM_CENTER: c_uint = 6;
const AC97_SLOT_PCM_SLEFT: c_uint = 7;
const AC97_SLOT_PCM_SRIGHT: c_uint = 8;
const AC97_SLOT_LFE: c_uint = 9;
const AC97_SLOT_SPDIF_LEFT2: c_uint = 10;
const AC97_SLOT_SPDIF_RIGHT2: c_uint = 11;
const AC97_EXTENDED_STATUS: c_uint = 0x2a;
const AC97_SCAP_SKIP_MODEM: c_uint = 1 << 0;
const AC97_SCAP_POWER_SAVE: c_uint = 1 << 1;
const AC97_SCAP_NO_SPDIF: c_uint = 1 << 2;
const AC97_TUNE_MUTE_LED: c_int = 1;

const ATI_REG_ISR: c_uint = 0x00; /* interrupt source */
const ATI_REG_ISR_IN_XRUN: c_uint = 1u32 << 0;
const ATI_REG_ISR_IN_STATUS: c_uint = 1u32 << 1;
const ATI_REG_ISR_OUT_XRUN: c_uint = 1u32 << 2;
const ATI_REG_ISR_OUT_STATUS: c_uint = 1u32 << 3;
const ATI_REG_ISR_SPDF_XRUN: c_uint = 1u32 << 4;
const ATI_REG_ISR_SPDF_STATUS: c_uint = 1u32 << 5;
const ATI_REG_ISR_PHYS_INTR: c_uint = 1u32 << 8;
const ATI_REG_ISR_PHYS_MISMATCH: c_uint = 1u32 << 9;
const ATI_REG_ISR_CODEC0_NOT_READY: c_uint = 1u32 << 10;
const ATI_REG_ISR_CODEC1_NOT_READY: c_uint = 1u32 << 11;
const ATI_REG_ISR_CODEC2_NOT_READY: c_uint = 1u32 << 12;
const ATI_REG_ISR_NEW_FRAME: c_uint = 1u32 << 13;

const ATI_REG_IER: c_uint = 0x04; /* interrupt enable */
const ATI_REG_IER_IN_XRUN_EN: c_uint = 1u32 << 0;
const ATI_REG_IER_IO_STATUS_EN: c_uint = 1u32 << 1;
const ATI_REG_IER_OUT_XRUN_EN: c_uint = 1u32 << 2;
const ATI_REG_IER_OUT_XRUN_COND: c_uint = 1u32 << 3;
const ATI_REG_IER_SPDF_XRUN_EN: c_uint = 1u32 << 4;
const ATI_REG_IER_SPDF_STATUS_EN: c_uint = 1u32 << 5;
const ATI_REG_IER_PHYS_INTR_EN: c_uint = 1u32 << 8;
const ATI_REG_IER_PHYS_MISMATCH_EN: c_uint = 1u32 << 9;
const ATI_REG_IER_CODEC0_INTR_EN: c_uint = 1u32 << 10;
const ATI_REG_IER_CODEC1_INTR_EN: c_uint = 1u32 << 11;
const ATI_REG_IER_CODEC2_INTR_EN: c_uint = 1u32 << 12;
const ATI_REG_IER_NEW_FRAME_EN: c_uint = 1u32 << 13; /* (RO */
const ATI_REG_IER_SET_BUS_BUSY: c_uint = 1u32 << 14; /* (WO) audio is running */

const ATI_REG_CMD: c_uint = 0x08; /* command */
const ATI_REG_CMD_POWERDOWN: c_uint = 1u32 << 0;
const ATI_REG_CMD_RECEIVE_EN: c_uint = 1u32 << 1;
const ATI_REG_CMD_SEND_EN: c_uint = 1u32 << 2;
const ATI_REG_CMD_STATUS_MEM: c_uint = 1u32 << 3;
const ATI_REG_CMD_SPDF_OUT_EN: c_uint = 1u32 << 4;
const ATI_REG_CMD_SPDF_STATUS_MEM: c_uint = 1u32 << 5;
const ATI_REG_CMD_SPDF_THRESHOLD: c_uint = 3u32 << 6;
const ATI_REG_CMD_SPDF_THRESHOLD_SHIFT: c_uint = 6;
const ATI_REG_CMD_IN_DMA_EN: c_uint = 1u32 << 8;
const ATI_REG_CMD_OUT_DMA_EN: c_uint = 1u32 << 9;
const ATI_REG_CMD_SPDF_DMA_EN: c_uint = 1u32 << 10;
const ATI_REG_CMD_SPDF_OUT_STOPPED: c_uint = 1u32 << 11;
const ATI_REG_CMD_SPDF_CONFIG_MASK: c_uint = 7u32 << 12;
const ATI_REG_CMD_SPDF_CONFIG_34: c_uint = 1u32 << 12;
const ATI_REG_CMD_SPDF_CONFIG_78: c_uint = 2u32 << 12;
const ATI_REG_CMD_SPDF_CONFIG_69: c_uint = 3u32 << 12;
const ATI_REG_CMD_SPDF_CONFIG_01: c_uint = 4u32 << 12;
const ATI_REG_CMD_INTERLEAVE_SPDF: c_uint = 1u32 << 16;
const ATI_REG_CMD_AUDIO_PRESENT: c_uint = 1u32 << 20;
const ATI_REG_CMD_INTERLEAVE_IN: c_uint = 1u32 << 21;
const ATI_REG_CMD_INTERLEAVE_OUT: c_uint = 1u32 << 22;
const ATI_REG_CMD_LOOPBACK_EN: c_uint = 1u32 << 23;
const ATI_REG_CMD_PACKED_DIS: c_uint = 1u32 << 24;
const ATI_REG_CMD_BURST_EN: c_uint = 1u32 << 25;
const ATI_REG_CMD_PANIC_EN: c_uint = 1u32 << 26;
const ATI_REG_CMD_MODEM_PRESENT: c_uint = 1u32 << 27;
const ATI_REG_CMD_ACLINK_ACTIVE: c_uint = 1u32 << 28;
const ATI_REG_CMD_AC_SOFT_RESET: c_uint = 1u32 << 29;
const ATI_REG_CMD_AC_SYNC: c_uint = 1u32 << 30;
const ATI_REG_CMD_AC_RESET: c_uint = 1u32 << 31;

const ATI_REG_PHYS_OUT_ADDR: c_uint = 0x0c;
const ATI_REG_PHYS_OUT_CODEC_MASK: c_uint = 3u32 << 0;
const ATI_REG_PHYS_OUT_RW: c_uint = 1u32 << 2;
const ATI_REG_PHYS_OUT_ADDR_EN: c_uint = 1u32 << 8;
const ATI_REG_PHYS_OUT_ADDR_SHIFT: c_uint = 9;
const ATI_REG_PHYS_OUT_DATA_SHIFT: c_uint = 16;
const ATI_REG_PHYS_IN_ADDR: c_uint = 0x10;
const ATI_REG_PHYS_IN_READ_FLAG: c_uint = 1u32 << 8;
const ATI_REG_PHYS_IN_ADDR_SHIFT: c_uint = 9;
const ATI_REG_PHYS_IN_DATA_SHIFT: c_uint = 16;
const ATI_REG_SLOTREQ: c_uint = 0x14;
const ATI_REG_COUNTER: c_uint = 0x18;
const ATI_REG_COUNTER_SLOT: c_uint = 3u32 << 0; /* slot # */
const ATI_REG_COUNTER_BITCLOCK: c_uint = 31u32 << 8;
const ATI_REG_IN_FIFO_THRESHOLD: c_uint = 0x1c;
const ATI_REG_IN_DMA_LINKPTR: c_uint = 0x20;
const ATI_REG_IN_DMA_DT_START: c_uint = 0x24; /* RO */
const ATI_REG_IN_DMA_DT_NEXT: c_uint = 0x28; /* RO */
const ATI_REG_IN_DMA_DT_CUR: c_uint = 0x2c; /* RO */
const ATI_REG_IN_DMA_DT_SIZE: c_uint = 0x30;
const ATI_REG_OUT_DMA_SLOT: c_uint = 0x34;
fn ATI_REG_OUT_DMA_SLOT_BIT(x: c_uint) -> c_uint { 1u32 << (x - 3) }
const ATI_REG_OUT_DMA_SLOT_MASK: c_uint = 0x1ff;
const ATI_REG_OUT_DMA_THRESHOLD_MASK: c_uint = 0xf800;
const ATI_REG_OUT_DMA_THRESHOLD_SHIFT: c_uint = 11;
const ATI_REG_OUT_DMA_LINKPTR: c_uint = 0x38;
const ATI_REG_OUT_DMA_DT_START: c_uint = 0x3c; /* RO */
const ATI_REG_OUT_DMA_DT_NEXT: c_uint = 0x40; /* RO */
const ATI_REG_OUT_DMA_DT_CUR: c_uint = 0x44; /* RO */
const ATI_REG_OUT_DMA_DT_SIZE: c_uint = 0x48;
const ATI_REG_SPDF_CMD: c_uint = 0x4c;
const ATI_REG_SPDF_CMD_LFSR: c_uint = 1u32 << 4;
const ATI_REG_SPDF_CMD_SINGLE_CH: c_uint = 1u32 << 5;
const ATI_REG_SPDF_CMD_LFSR_ACC: c_uint = 0xffu32 << 8; /* RO */
const ATI_REG_SPDF_DMA_LINKPTR: c_uint = 0x50;
const ATI_REG_SPDF_DMA_DT_START: c_uint = 0x54; /* RO */
const ATI_REG_SPDF_DMA_DT_NEXT: c_uint = 0x58; /* RO */
const ATI_REG_SPDF_DMA_DT_CUR: c_uint = 0x5c; /* RO */
const ATI_REG_SPDF_DMA_DT_SIZE: c_uint = 0x60;
const ATI_REG_MODEM_MIRROR: c_uint = 0x7c;
const ATI_REG_AUDIO_MIRROR: c_uint = 0x80;
const ATI_REG_6CH_REORDER: c_uint = 0x84; /* reorder slots for 6ch */
const ATI_REG_6CH_REORDER_EN: c_uint = 1u32 << 0; /* 3,4,7,8,6,9 -> 3,4,6,9,7,8 */
const ATI_REG_FIFO_FLUSH: c_uint = 0x88;
const ATI_REG_FIFO_OUT_FLUSH: c_uint = 1u32 << 0;
const ATI_REG_FIFO_IN_FLUSH: c_uint = 1u32 << 1;
const ATI_REG_LINKPTR_EN: c_uint = 1u32 << 0;
const ATI_REG_DMA_DT_SIZE: c_uint = 0xffffu32 << 0;
const ATI_REG_DMA_FIFO_USED: c_uint = 0x1fu32 << 16;
const ATI_REG_DMA_FIFO_FREE: c_uint = 0x1fu32 << 21;
const ATI_REG_DMA_STATE: c_uint = 7u32 << 26;

const ATI_MAX_DESCRIPTORS: c_uint = 256; /* max number of descriptor packets */
const NUM_ATI_CODECS: usize = 3;

const ATI_DMA_PLAYBACK: usize = 0;
const ATI_DMA_CAPTURE: usize = 1;
const ATI_DMA_SPDIF: usize = 2;
const NUM_ATI_DMAS: usize = 3;
const ATI_PCM_OUT: usize = 0;
const ATI_PCM_IN: usize = 1;
const ATI_PCM_SPDIF: usize = 2;
const NUM_ATI_PCMS: usize = 3;
const ATI_PCMDEV_ANALOG: usize = 0;
const ATI_PCMDEV_DIGITAL: usize = 1;
const NUM_ATI_PCMDEVS: usize = 2;

/*
 * DMA packate descriptor
 */
#[repr(C)]
pub struct atiixp_dma_desc {
    addr: __le32, /* DMA buffer address */
    status: u16, /* status bits */
    size: u16,   /* size of the packet in dwords */
    next: __le32, /* address of the next packet descriptor */
}

/*
 * constants and callbacks for each DMA type
 */
#[repr(C)]
pub struct atiixp_dma_ops {
    type_: c_int, /* ATI_DMA_XXX */
    llp_offset: c_uint, /* LINKPTR offset */
    dt_cur: c_uint, /* DT_CUR offset */
    /* called from open callback */
    enable_dma: Option<unsafe extern "C" fn(*mut atiixp, c_int)>,
    /* called from trigger (START/STOP) */
    enable_transfer: Option<unsafe extern "C" fn(*mut atiixp, c_int)>,
    /* called from trigger (STOP only) */
    flush_dma: Option<unsafe extern "C" fn(*mut atiixp)>,
}

/*
 * DMA stream
 */
#[repr(C)]
pub struct atiixp_dma {
    ops: *const atiixp_dma_ops,
    desc_buf: snd_dma_buffer,
    substream: *mut snd_pcm_substream, /* assigned PCM substream */
    buf_addr: c_uint,
    buf_bytes: c_uint, /* DMA buffer address, bytes */
    period_bytes: c_uint,
    periods: c_uint,
    opened: c_int,
    running: c_int,
    suspended: c_int,
    pcm_open_flag: c_int,
    ac97_pcm_type: c_int, /* index # of ac97_pcm to access, -1 = not used */
    saved_curptr: c_uint,
}

/*
 * ATI IXP chip
 */
#[repr(C)]
pub struct atiixp {
    card: *mut snd_card,
    pci: *mut pci_dev,
    addr: c_ulong,
    remap_addr: *mut c_void,
    irq: c_int,
    ac97_bus: *mut snd_ac97_bus,
    ac97: [*mut snd_ac97; NUM_ATI_CODECS],
    reg_lock: spinlock_t,
    dmas: [atiixp_dma; NUM_ATI_DMAS],
    pcms: [*mut ac97_pcm; NUM_ATI_PCMS],
    pcmdevs: [*mut snd_pcm; NUM_ATI_PCMDEVS],
    max_channels: c_int, /* max. channels for PCM out */
    codec_not_ready_bits: c_uint, /* for codec detection */
    spdif_over_aclink: c_int, /* passed from the module option */
    open_mutex: mutex, /* playback open mutex */
}

const fn pci_vdevice_ati(device: c_uint) -> pci_device_id {
    pci_device_id { vendor: 0x1002, device, subvendor: 0, subdevice: 0, class: 0, class_mask: 0, driver_data: 0 }
}

static snd_atiixp_ids: [pci_device_id; 5] = [
    pci_vdevice_ati(0x4341), /* SB200 */
    pci_vdevice_ati(0x4361), /* SB300 */
    pci_vdevice_ati(0x4370), /* SB400 */
    pci_vdevice_ati(0x4382), /* SB600 */
    pci_device_id { vendor: 0, device: 0, subvendor: 0, subdevice: 0, class: 0, class_mask: 0, driver_data: 0 },
];

static atiixp_quirks: [snd_pci_quirk; 3] = [
    snd_pci_quirk { subvendor: 0x105b, subdevice: 0x0c81, name: b"Foxconn RC4107MA-RS2\0".as_ptr() as *const c_char, value: 0 },
    snd_pci_quirk { subvendor: 0x15bd, subdevice: 0x3100, name: b"DFI RS482\0".as_ptr() as *const c_char, value: 0 },
    snd_pci_quirk { subvendor: 0, subdevice: 0, name: ptr::null(), value: 0 }, /* terminator */
];

unsafe fn reg_addr(chip: *mut atiixp, reg: c_uint) -> *mut c_void {
    ((*chip).remap_addr as *mut u8).add(reg as usize) as *mut c_void
}

/*
 * update the bits of the given register.
 * return 1 if the bits changed.
 */
unsafe extern "C" fn snd_atiixp_update_bits(
    chip: *mut atiixp,
    reg: c_uint,
    mask: c_uint,
    value: c_uint,
) -> c_int {
    let addr = reg_addr(chip, reg);
    let old_data;
    let mut data;
    old_data = readl(addr);
    data = old_data;
    data &= !mask;
    data |= value;
    if old_data == data {
        return 0;
    }
    writel(data, addr);
    1
}

unsafe fn atiixp_write(chip: *mut atiixp, reg: c_uint, value: c_uint) {
    writel(value, reg_addr(chip, reg));
}
unsafe fn atiixp_read(chip: *mut atiixp, reg: c_uint) -> c_uint {
    readl(reg_addr(chip, reg))
}
unsafe fn atiixp_update(chip: *mut atiixp, reg: c_uint, mask: c_uint, val: c_uint) -> c_int {
    snd_atiixp_update_bits(chip, reg, mask, val)
}

const fn page_align(x: usize) -> usize { (x + 4095) & !4095 }
const ATI_DESC_LIST_SIZE: usize = page_align(ATI_MAX_DESCRIPTORS as usize * size_of::<atiixp_dma_desc>());

unsafe extern "C" fn atiixp_build_dma_packets(
    chip: *mut atiixp,
    dma: *mut atiixp_dma,
    substream: *mut snd_pcm_substream,
    periods: c_uint,
    period_bytes: c_uint,
) -> c_int {
    let mut i: c_uint;
    let mut addr: u32;
    let mut desc_addr: u32;

    if periods > ATI_MAX_DESCRIPTORS {
        return -ENOMEM;
    }
    if (*dma).desc_buf.area.is_null() {
        if snd_dma_alloc_pages(SNDRV_DMA_TYPE_DEV, &mut (*(*chip).pci).dev, ATI_DESC_LIST_SIZE as c_ulong, &mut (*dma).desc_buf) < 0 {
            return -ENOMEM;
        }
        (*dma).periods = 0;
        (*dma).period_bytes = (*dma).periods;
    }
    if (*dma).periods == periods && (*dma).period_bytes == period_bytes {
        return 0;
    }

    /* reset DMA before changing the descriptor table */
    writel(0, reg_addr(chip, (*(*dma).ops).llp_offset));
    ((*(*dma).ops).enable_dma.unwrap())(chip, 0);
    ((*(*dma).ops).enable_dma.unwrap())(chip, 1);

    /* fill the entries */
    addr = (*(*substream).runtime).dma_addr as u32;
    desc_addr = (*dma).desc_buf.addr as u32;
    i = 0;
    while i < periods {
        let desc = ((*dma).desc_buf.area as *mut atiixp_dma_desc).add(i as usize);
        (*desc).addr = addr.to_le();
        (*desc).status = 0;
        (*desc).size = (period_bytes >> 2) as u16; /* in dwords */
        desc_addr = desc_addr.wrapping_add(size_of::<atiixp_dma_desc>() as u32);
        if i == periods - 1 {
            (*desc).next = ((*dma).desc_buf.addr as u32).to_le();
        } else {
            (*desc).next = desc_addr.to_le();
        }
        addr = addr.wrapping_add(period_bytes);
        i += 1;
    }

    writel((*dma).desc_buf.addr as u32 | ATI_REG_LINKPTR_EN, reg_addr(chip, (*(*dma).ops).llp_offset));
    (*dma).period_bytes = period_bytes;
    (*dma).periods = periods;
    0
}

unsafe extern "C" fn atiixp_clear_dma_packets(
    chip: *mut atiixp,
    dma: *mut atiixp_dma,
    _substream: *mut snd_pcm_substream,
) {
    if !(*dma).desc_buf.area.is_null() {
        writel(0, reg_addr(chip, (*(*dma).ops).llp_offset));
        snd_dma_free_pages(&mut (*dma).desc_buf);
        (*dma).desc_buf.area = ptr::null_mut();
    }
}

unsafe extern "C" fn snd_atiixp_acquire_codec(chip: *mut atiixp) -> c_int {
    let mut timeout = 1000;
    while atiixp_read(chip, ATI_REG_PHYS_OUT_ADDR) & ATI_REG_PHYS_OUT_ADDR_EN != 0 {
        if timeout == 0 {
            dev_warn((*(*chip).card).dev, b"codec acquire timeout\n\0".as_ptr() as *const c_char);
            return -EBUSY;
        }
        timeout -= 1;
        udelay(1);
    }
    0
}

extern "C" {
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
}

unsafe extern "C" fn snd_atiixp_codec_read(chip: *mut atiixp, codec: c_ushort, reg: c_ushort) -> c_ushort {
    let mut data: c_uint;
    let mut timeout: c_int;
    if snd_atiixp_acquire_codec(chip) < 0 {
        return 0xffff;
    }
    data = ((reg as c_uint) << ATI_REG_PHYS_OUT_ADDR_SHIFT) | ATI_REG_PHYS_OUT_ADDR_EN | ATI_REG_PHYS_OUT_RW | codec as c_uint;
    atiixp_write(chip, ATI_REG_PHYS_OUT_ADDR, data);
    if snd_atiixp_acquire_codec(chip) < 0 {
        return 0xffff;
    }
    timeout = 1000;
    loop {
        data = atiixp_read(chip, ATI_REG_PHYS_IN_ADDR);
        if data & ATI_REG_PHYS_IN_READ_FLAG != 0 {
            return (data >> ATI_REG_PHYS_IN_DATA_SHIFT) as c_ushort;
        }
        udelay(1);
        timeout -= 1;
        if timeout == 0 {
            break;
        }
    }
    /* time out may happen during reset */
    if reg < 0x7c {
        dev_warn((*(*chip).card).dev, b"codec read timeout (reg %x)\n\0".as_ptr() as *const c_char, reg as c_uint);
    }
    0xffff
}

unsafe extern "C" fn snd_atiixp_codec_write(chip: *mut atiixp, codec: c_ushort, reg: c_ushort, val: c_ushort) {
    let data: c_uint;
    if snd_atiixp_acquire_codec(chip) < 0 {
        return;
    }
    data = ((val as c_uint) << ATI_REG_PHYS_OUT_DATA_SHIFT)
        | ((reg as c_uint) << ATI_REG_PHYS_OUT_ADDR_SHIFT)
        | ATI_REG_PHYS_OUT_ADDR_EN
        | codec as c_uint;
    atiixp_write(chip, ATI_REG_PHYS_OUT_ADDR, data);
}

unsafe extern "C" fn snd_atiixp_ac97_read(ac97: *mut snd_ac97, reg: c_ushort) -> c_ushort {
    let chip = (*ac97).private_data;
    snd_atiixp_codec_read(chip, (*ac97).num, reg)
}

unsafe extern "C" fn snd_atiixp_ac97_write(ac97: *mut snd_ac97, reg: c_ushort, val: c_ushort) {
    let chip = (*ac97).private_data;
    snd_atiixp_codec_write(chip, (*ac97).num, reg, val);
}

unsafe extern "C" fn snd_atiixp_aclink_reset(chip: *mut atiixp) -> c_int {
    let mut timeout: c_int;
    /* reset powerdoewn */
    if atiixp_update(chip, ATI_REG_CMD, ATI_REG_CMD_POWERDOWN, 0) != 0 {
        udelay(10);
    }
    atiixp_update(chip, ATI_REG_CMD, ATI_REG_CMD_AC_SOFT_RESET, ATI_REG_CMD_AC_SOFT_RESET);
    atiixp_read(chip, ATI_REG_CMD);
    udelay(10);
    atiixp_update(chip, ATI_REG_CMD, ATI_REG_CMD_AC_SOFT_RESET, 0);

    timeout = 10;
    while atiixp_read(chip, ATI_REG_CMD) & ATI_REG_CMD_ACLINK_ACTIVE == 0 {
        atiixp_update(chip, ATI_REG_CMD, ATI_REG_CMD_AC_SYNC | ATI_REG_CMD_AC_RESET, ATI_REG_CMD_AC_SYNC);
        atiixp_read(chip, ATI_REG_CMD);
        mdelay(1);
        atiixp_update(chip, ATI_REG_CMD, ATI_REG_CMD_AC_RESET, ATI_REG_CMD_AC_RESET);
        timeout -= 1;
        if timeout == 0 {
            dev_err((*(*chip).card).dev, b"codec reset timeout\n\0".as_ptr() as *const c_char);
            break;
        }
    }
    atiixp_update(chip, ATI_REG_CMD, ATI_REG_CMD_AC_SYNC | ATI_REG_CMD_AC_RESET, ATI_REG_CMD_AC_SYNC | ATI_REG_CMD_AC_RESET);
    0
}

unsafe extern "C" fn snd_atiixp_aclink_down(chip: *mut atiixp) -> c_int {
    // if (atiixp_read(chip, MODEM_MIRROR) & 0x1) /* modem running, too? */
    //	return -EBUSY;
    atiixp_update(chip, ATI_REG_CMD, ATI_REG_CMD_POWERDOWN | ATI_REG_CMD_AC_RESET, ATI_REG_CMD_POWERDOWN);
    0
}

const ALL_CODEC_NOT_READY: c_uint = ATI_REG_ISR_CODEC0_NOT_READY | ATI_REG_ISR_CODEC1_NOT_READY | ATI_REG_ISR_CODEC2_NOT_READY;
const CODEC_CHECK_BITS: c_uint = ALL_CODEC_NOT_READY | ATI_REG_ISR_NEW_FRAME;

unsafe extern "C" fn ac97_probing_bugs(pci: *mut pci_dev) -> c_int {
    let q = snd_pci_quirk_lookup(pci, atiixp_quirks.as_ptr());
    if !q.is_null() {
        dev_dbg(&mut (*pci).dev, b"atiixp quirk for %s.  Forcing codec %d\n\0".as_ptr() as *const c_char, snd_pci_quirk_name(q), (*q).value);
        return (*q).value;
    }
    -1
}

unsafe extern "C" fn snd_atiixp_codec_detect(chip: *mut atiixp) -> c_int {
    let mut timeout: c_int;
    (*chip).codec_not_ready_bits = 0;
    if ac97_codec == -1 {
        ac97_codec = ac97_probing_bugs((*chip).pci);
    }
    if ac97_codec >= 0 {
        (*chip).codec_not_ready_bits |= CODEC_CHECK_BITS ^ (1u32 << (ac97_codec + 10));
        return 0;
    }
    atiixp_write(chip, ATI_REG_IER, CODEC_CHECK_BITS);
    timeout = 50;
    while timeout > 0 {
        timeout -= 1;
        mdelay(1);
        if (*chip).codec_not_ready_bits != 0 {
            break;
        }
    }
    atiixp_write(chip, ATI_REG_IER, 0); /* disable irqs */
    if ((*chip).codec_not_ready_bits & ALL_CODEC_NOT_READY) == ALL_CODEC_NOT_READY {
        dev_err((*(*chip).card).dev, b"no codec detected!\n\0".as_ptr() as *const c_char);
        return -ENXIO;
    }
    0
}

unsafe extern "C" fn snd_atiixp_chip_start(chip: *mut atiixp) -> c_int {
    let mut reg: c_uint;
    reg = atiixp_read(chip, ATI_REG_CMD);
    reg |= 0x02 << ATI_REG_CMD_SPDF_THRESHOLD_SHIFT;
    reg |= ATI_REG_CMD_BURST_EN;
    atiixp_write(chip, ATI_REG_CMD, reg);
    reg = atiixp_read(chip, ATI_REG_SPDF_CMD);
    reg &= !(ATI_REG_SPDF_CMD_LFSR | ATI_REG_SPDF_CMD_SINGLE_CH);
    atiixp_write(chip, ATI_REG_SPDF_CMD, reg);
    atiixp_write(chip, ATI_REG_ISR, 0xffffffff);
    atiixp_write(chip, ATI_REG_IER, ATI_REG_IER_IO_STATUS_EN | ATI_REG_IER_IN_XRUN_EN | ATI_REG_IER_OUT_XRUN_EN | ATI_REG_IER_SPDF_XRUN_EN | ATI_REG_IER_SPDF_STATUS_EN);
    0
}

unsafe extern "C" fn snd_atiixp_chip_stop(chip: *mut atiixp) -> c_int {
    atiixp_write(chip, ATI_REG_ISR, atiixp_read(chip, ATI_REG_ISR));
    atiixp_write(chip, ATI_REG_IER, 0);
    0
}

unsafe extern "C" fn snd_atiixp_pcm_pointer(substream: *mut snd_pcm_substream) -> snd_pcm_uframes_t {
    let chip = snd_pcm_substream_chip(substream);
    let runtime = (*substream).runtime;
    let dma = (*runtime).private_data as *mut atiixp_dma;
    let mut curptr: c_uint;
    let mut timeout = 1000;
    while timeout != 0 {
        timeout -= 1;
        curptr = readl(reg_addr(chip, (*(*dma).ops).dt_cur));
        if curptr < (*dma).buf_addr {
            continue;
        }
        curptr = curptr.wrapping_sub((*dma).buf_addr);
        if curptr >= (*dma).buf_bytes {
            continue;
        }
        return bytes_to_frames(runtime, curptr);
    }
    dev_dbg((*(*chip).card).dev, b"invalid DMA pointer read 0x%x (buf=%x)\n\0".as_ptr() as *const c_char, readl(reg_addr(chip, (*(*dma).ops).dt_cur)), (*dma).buf_addr);
    0
}

unsafe extern "C" fn snd_atiixp_xrun_dma(chip: *mut atiixp, dma: *mut atiixp_dma) {
    if (*dma).substream.is_null() || (*dma).running == 0 {
        return;
    }
    dev_dbg((*(*chip).card).dev, b"XRUN detected (DMA %d)\n\0".as_ptr() as *const c_char, (*(*dma).ops).type_);
    snd_pcm_stop_xrun((*dma).substream);
}

unsafe extern "C" fn snd_atiixp_update_dma(_chip: *mut atiixp, dma: *mut atiixp_dma) {
    if (*dma).substream.is_null() || (*dma).running == 0 {
        return;
    }
    snd_pcm_period_elapsed((*dma).substream);
}

unsafe extern "C" fn snd_atiixp_check_bus_busy(chip: *mut atiixp) {
    let bus_busy: c_uint;
    if atiixp_read(chip, ATI_REG_CMD) & (ATI_REG_CMD_SEND_EN | ATI_REG_CMD_RECEIVE_EN | ATI_REG_CMD_SPDF_OUT_EN) != 0 {
        bus_busy = ATI_REG_IER_SET_BUS_BUSY;
    } else {
        bus_busy = 0;
    }
    atiixp_update(chip, ATI_REG_IER, ATI_REG_IER_SET_BUS_BUSY, bus_busy);
}

unsafe extern "C" fn snd_atiixp_pcm_trigger(substream: *mut snd_pcm_substream, cmd: c_int) -> c_int {
    let chip = snd_pcm_substream_chip(substream);
    let dma = (*(*substream).runtime).private_data as *mut atiixp_dma;
    let mut err = 0;
    if snd_BUG_ON((*(*dma).ops).enable_transfer.is_none() || (*(*dma).ops).flush_dma.is_none()) != 0 {
        return -EINVAL;
    }
    match cmd {
        SNDRV_PCM_TRIGGER_START | SNDRV_PCM_TRIGGER_PAUSE_RELEASE | SNDRV_PCM_TRIGGER_RESUME => {
            if (*dma).running != 0 && (*dma).suspended != 0 && cmd == SNDRV_PCM_TRIGGER_RESUME {
                writel((*dma).saved_curptr, reg_addr(chip, (*(*dma).ops).dt_cur));
            }
            ((*(*dma).ops).enable_transfer.unwrap())(chip, 1);
            (*dma).running = 1;
            (*dma).suspended = 0;
        }
        SNDRV_PCM_TRIGGER_STOP | SNDRV_PCM_TRIGGER_PAUSE_PUSH | SNDRV_PCM_TRIGGER_SUSPEND => {
            (*dma).suspended = (cmd == SNDRV_PCM_TRIGGER_SUSPEND) as c_int;
            if (*dma).running != 0 && (*dma).suspended != 0 {
                (*dma).saved_curptr = readl(reg_addr(chip, (*(*dma).ops).dt_cur));
            }
            ((*(*dma).ops).enable_transfer.unwrap())(chip, 0);
            (*dma).running = 0;
        }
        _ => err = -EINVAL,
    }
    if err == 0 {
        snd_atiixp_check_bus_busy(chip);
        if cmd == SNDRV_PCM_TRIGGER_STOP {
            ((*(*dma).ops).flush_dma.unwrap())(chip);
            snd_atiixp_check_bus_busy(chip);
        }
    }
    err
}

unsafe extern "C" fn atiixp_out_flush_dma(chip: *mut atiixp) {
    atiixp_write(chip, ATI_REG_FIFO_FLUSH, ATI_REG_FIFO_OUT_FLUSH);
}
unsafe extern "C" fn atiixp_out_enable_dma(chip: *mut atiixp, on: c_int) {
    let mut data = atiixp_read(chip, ATI_REG_CMD);
    if on != 0 {
        if data & ATI_REG_CMD_OUT_DMA_EN != 0 { return; }
        atiixp_out_flush_dma(chip);
        data |= ATI_REG_CMD_OUT_DMA_EN;
    } else {
        data &= !ATI_REG_CMD_OUT_DMA_EN;
    }
    atiixp_write(chip, ATI_REG_CMD, data);
}
unsafe extern "C" fn atiixp_out_enable_transfer(chip: *mut atiixp, on: c_int) {
    atiixp_update(chip, ATI_REG_CMD, ATI_REG_CMD_SEND_EN, if on != 0 { ATI_REG_CMD_SEND_EN } else { 0 });
}
unsafe extern "C" fn atiixp_in_enable_dma(chip: *mut atiixp, on: c_int) {
    atiixp_update(chip, ATI_REG_CMD, ATI_REG_CMD_IN_DMA_EN, if on != 0 { ATI_REG_CMD_IN_DMA_EN } else { 0 });
}
unsafe extern "C" fn atiixp_in_enable_transfer(chip: *mut atiixp, on: c_int) {
    if on != 0 {
        let mut data = atiixp_read(chip, ATI_REG_CMD);
        if data & ATI_REG_CMD_RECEIVE_EN == 0 {
            data |= ATI_REG_CMD_RECEIVE_EN;
            /* #if 0: FIXME: this causes the endless loop; wait until slot 3/4 are finished */
            atiixp_write(chip, ATI_REG_CMD, data);
        }
    } else {
        atiixp_update(chip, ATI_REG_CMD, ATI_REG_CMD_RECEIVE_EN, 0);
    }
}
unsafe extern "C" fn atiixp_in_flush_dma(chip: *mut atiixp) {
    atiixp_write(chip, ATI_REG_FIFO_FLUSH, ATI_REG_FIFO_IN_FLUSH);
}
unsafe extern "C" fn atiixp_spdif_enable_dma(chip: *mut atiixp, on: c_int) {
    atiixp_update(chip, ATI_REG_CMD, ATI_REG_CMD_SPDF_DMA_EN, if on != 0 { ATI_REG_CMD_SPDF_DMA_EN } else { 0 });
}
unsafe extern "C" fn atiixp_spdif_enable_transfer(chip: *mut atiixp, on: c_int) {
    let mut data = atiixp_read(chip, ATI_REG_CMD);
    if on != 0 { data |= ATI_REG_CMD_SPDF_OUT_EN; } else { data &= !ATI_REG_CMD_SPDF_OUT_EN; }
    atiixp_write(chip, ATI_REG_CMD, data);
}
unsafe extern "C" fn atiixp_spdif_flush_dma(chip: *mut atiixp) {
    let mut timeout: c_int;
    atiixp_spdif_enable_dma(chip, 0);
    atiixp_spdif_enable_transfer(chip, 1);
    timeout = 100;
    loop {
        if atiixp_read(chip, ATI_REG_SPDF_DMA_DT_SIZE) & ATI_REG_DMA_FIFO_USED == 0 { break; }
        udelay(1);
        if timeout <= 0 { break; }
        timeout -= 1;
    }
    atiixp_spdif_enable_transfer(chip, 0);
}

unsafe extern "C" fn snd_atiixp_spdif_prepare(substream: *mut snd_pcm_substream) -> c_int {
    let chip = snd_pcm_substream_chip(substream);
    if (*chip).spdif_over_aclink != 0 {
        let mut data: c_uint;
        atiixp_update(chip, ATI_REG_CMD, ATI_REG_CMD_SPDF_CONFIG_MASK, ATI_REG_CMD_SPDF_CONFIG_01);
        data = atiixp_read(chip, ATI_REG_OUT_DMA_SLOT) & !ATI_REG_OUT_DMA_SLOT_MASK;
        data |= ATI_REG_OUT_DMA_SLOT_BIT(10) | ATI_REG_OUT_DMA_SLOT_BIT(11);
        data |= 0x04 << ATI_REG_OUT_DMA_THRESHOLD_SHIFT;
        atiixp_write(chip, ATI_REG_OUT_DMA_SLOT, data);
        atiixp_update(chip, ATI_REG_CMD, ATI_REG_CMD_INTERLEAVE_OUT, if (*(*substream).runtime).format == SNDRV_PCM_FORMAT_S16_LE { ATI_REG_CMD_INTERLEAVE_OUT } else { 0 });
    } else {
        atiixp_update(chip, ATI_REG_CMD, ATI_REG_CMD_SPDF_CONFIG_MASK, 0);
        atiixp_update(chip, ATI_REG_CMD, ATI_REG_CMD_INTERLEAVE_SPDF, 0);
    }
    0
}

unsafe extern "C" fn snd_atiixp_playback_prepare(substream: *mut snd_pcm_substream) -> c_int {
    let chip = snd_pcm_substream_chip(substream);
    let mut data = atiixp_read(chip, ATI_REG_OUT_DMA_SLOT) & !ATI_REG_OUT_DMA_SLOT_MASK;
    match (*(*substream).runtime).channels {
        8 => {
            data |= ATI_REG_OUT_DMA_SLOT_BIT(10) | ATI_REG_OUT_DMA_SLOT_BIT(11);
            data |= ATI_REG_OUT_DMA_SLOT_BIT(7) | ATI_REG_OUT_DMA_SLOT_BIT(8);
            data |= ATI_REG_OUT_DMA_SLOT_BIT(6) | ATI_REG_OUT_DMA_SLOT_BIT(9);
            data |= ATI_REG_OUT_DMA_SLOT_BIT(3) | ATI_REG_OUT_DMA_SLOT_BIT(4);
        }
        6 => {
            data |= ATI_REG_OUT_DMA_SLOT_BIT(7) | ATI_REG_OUT_DMA_SLOT_BIT(8);
            data |= ATI_REG_OUT_DMA_SLOT_BIT(6) | ATI_REG_OUT_DMA_SLOT_BIT(9);
            data |= ATI_REG_OUT_DMA_SLOT_BIT(3) | ATI_REG_OUT_DMA_SLOT_BIT(4);
        }
        4 => {
            data |= ATI_REG_OUT_DMA_SLOT_BIT(6) | ATI_REG_OUT_DMA_SLOT_BIT(9);
            data |= ATI_REG_OUT_DMA_SLOT_BIT(3) | ATI_REG_OUT_DMA_SLOT_BIT(4);
        }
        _ => data |= ATI_REG_OUT_DMA_SLOT_BIT(3) | ATI_REG_OUT_DMA_SLOT_BIT(4),
    }
    data |= 0x04 << ATI_REG_OUT_DMA_THRESHOLD_SHIFT;
    atiixp_write(chip, ATI_REG_OUT_DMA_SLOT, data);
    atiixp_update(chip, ATI_REG_CMD, ATI_REG_CMD_INTERLEAVE_OUT, if (*(*substream).runtime).format == SNDRV_PCM_FORMAT_S16_LE { ATI_REG_CMD_INTERLEAVE_OUT } else { 0 });
    atiixp_update(chip, ATI_REG_6CH_REORDER, ATI_REG_6CH_REORDER_EN, if (*(*substream).runtime).channels >= 6 { ATI_REG_6CH_REORDER_EN } else { 0 });
    0
}

unsafe extern "C" fn snd_atiixp_capture_prepare(substream: *mut snd_pcm_substream) -> c_int {
    let chip = snd_pcm_substream_chip(substream);
    atiixp_update(chip, ATI_REG_CMD, ATI_REG_CMD_INTERLEAVE_IN, if (*(*substream).runtime).format == SNDRV_PCM_FORMAT_S16_LE { ATI_REG_CMD_INTERLEAVE_IN } else { 0 });
    0
}

unsafe extern "C" fn snd_atiixp_pcm_hw_params(substream: *mut snd_pcm_substream, hw_params: *mut snd_pcm_hw_params) -> c_int {
    let chip = snd_pcm_substream_chip(substream);
    let dma = (*(*substream).runtime).private_data as *mut atiixp_dma;
    let mut err: c_int;
    (*dma).buf_addr = (*(*substream).runtime).dma_addr;
    (*dma).buf_bytes = params_buffer_bytes(hw_params);
    err = atiixp_build_dma_packets(chip, dma, substream, params_periods(hw_params), params_period_bytes(hw_params));
    if err < 0 { return err; }
    if (*dma).ac97_pcm_type >= 0 {
        let pcm = (*chip).pcms[(*dma).ac97_pcm_type as usize];
        if (*dma).pcm_open_flag != 0 {
            snd_ac97_pcm_close(pcm);
            (*dma).pcm_open_flag = 0;
        }
        err = snd_ac97_pcm_open(pcm, params_rate(hw_params), params_channels(hw_params), (*(*pcm).r.as_ptr()).slots);
        if err >= 0 { (*dma).pcm_open_flag = 1; }
    }
    err
}

unsafe extern "C" fn snd_atiixp_pcm_hw_free(substream: *mut snd_pcm_substream) -> c_int {
    let chip = snd_pcm_substream_chip(substream);
    let dma = (*(*substream).runtime).private_data as *mut atiixp_dma;
    if (*dma).pcm_open_flag != 0 {
        let pcm = (*chip).pcms[(*dma).ac97_pcm_type as usize];
        snd_ac97_pcm_close(pcm);
        (*dma).pcm_open_flag = 0;
    }
    atiixp_clear_dma_packets(chip, dma, substream);
    0
}

static snd_atiixp_pcm_hw: snd_pcm_hardware = snd_pcm_hardware {
    info: SNDRV_PCM_INFO_MMAP | SNDRV_PCM_INFO_INTERLEAVED | SNDRV_PCM_INFO_BLOCK_TRANSFER | SNDRV_PCM_INFO_PAUSE | SNDRV_PCM_INFO_RESUME | SNDRV_PCM_INFO_MMAP_VALID,
    formats: SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S32_LE,
    rates: SNDRV_PCM_RATE_48000,
    rate_min: 48000,
    rate_max: 48000,
    channels_min: 2,
    channels_max: 2,
    buffer_bytes_max: 256 * 1024,
    period_bytes_min: 32,
    period_bytes_max: 128 * 1024,
    periods_min: 2,
    periods_max: ATI_MAX_DESCRIPTORS,
};

unsafe extern "C" fn snd_atiixp_pcm_open(substream: *mut snd_pcm_substream, dma: *mut atiixp_dma, pcm_type: c_int) -> c_int {
    let chip = snd_pcm_substream_chip(substream);
    let runtime = (*substream).runtime;
    let err: c_int;
    if snd_BUG_ON((*dma).ops.is_null() || (*(*dma).ops).enable_dma.is_none()) != 0 { return -EINVAL; }
    if (*dma).opened != 0 { return -EBUSY; }
    (*dma).substream = substream;
    (*runtime).hw = snd_atiixp_pcm_hw;
    (*dma).ac97_pcm_type = pcm_type;
    if pcm_type >= 0 {
        (*runtime).hw.rates = (*(*chip).pcms[pcm_type as usize]).rates;
        snd_pcm_limit_hw_rates(runtime);
    } else {
        (*runtime).hw.formats = SNDRV_PCM_FMTBIT_IEC958_SUBFRAME_LE;
    }
    err = snd_pcm_hw_constraint_integer(runtime, SNDRV_PCM_HW_PARAM_PERIODS);
    if err < 0 { return err; }
    (*runtime).private_data = dma as *mut c_void;
    ((*(*dma).ops).enable_dma.unwrap())(chip, 1);
    (*dma).opened = 1;
    0
}

unsafe extern "C" fn snd_atiixp_pcm_close(substream: *mut snd_pcm_substream, dma: *mut atiixp_dma) -> c_int {
    let chip = snd_pcm_substream_chip(substream);
    if snd_BUG_ON((*dma).ops.is_null() || (*(*dma).ops).enable_dma.is_none()) != 0 { return -EINVAL; }
    ((*(*dma).ops).enable_dma.unwrap())(chip, 0);
    (*dma).substream = ptr::null_mut();
    (*dma).opened = 0;
    0
}

unsafe extern "C" fn snd_atiixp_playback_open(substream: *mut snd_pcm_substream) -> c_int {
    let chip = snd_pcm_substream_chip(substream);
    let err = snd_atiixp_pcm_open(substream, &mut (*chip).dmas[ATI_DMA_PLAYBACK], 0);
    if err < 0 { return err; }
    (*(*substream).runtime).hw.channels_max = (*chip).max_channels as c_uint;
    if (*chip).max_channels > 2 {
        snd_pcm_hw_constraint_step((*substream).runtime, 0, SNDRV_PCM_HW_PARAM_CHANNELS, 2);
    }
    0
}
unsafe extern "C" fn snd_atiixp_playback_close(substream: *mut snd_pcm_substream) -> c_int {
    let chip = snd_pcm_substream_chip(substream);
    snd_atiixp_pcm_close(substream, &mut (*chip).dmas[ATI_DMA_PLAYBACK])
}
unsafe extern "C" fn snd_atiixp_capture_open(substream: *mut snd_pcm_substream) -> c_int {
    let chip = snd_pcm_substream_chip(substream);
    snd_atiixp_pcm_open(substream, &mut (*chip).dmas[ATI_DMA_CAPTURE], 1)
}
unsafe extern "C" fn snd_atiixp_capture_close(substream: *mut snd_pcm_substream) -> c_int {
    let chip = snd_pcm_substream_chip(substream);
    snd_atiixp_pcm_close(substream, &mut (*chip).dmas[ATI_DMA_CAPTURE])
}
unsafe extern "C" fn snd_atiixp_spdif_open(substream: *mut snd_pcm_substream) -> c_int {
    let chip = snd_pcm_substream_chip(substream);
    if (*chip).spdif_over_aclink != 0 {
        snd_atiixp_pcm_open(substream, &mut (*chip).dmas[ATI_DMA_PLAYBACK], 2)
    } else {
        snd_atiixp_pcm_open(substream, &mut (*chip).dmas[ATI_DMA_SPDIF], -1)
    }
}
unsafe extern "C" fn snd_atiixp_spdif_close(substream: *mut snd_pcm_substream) -> c_int {
    let chip = snd_pcm_substream_chip(substream);
    if (*chip).spdif_over_aclink != 0 {
        snd_atiixp_pcm_close(substream, &mut (*chip).dmas[ATI_DMA_PLAYBACK])
    } else {
        snd_atiixp_pcm_close(substream, &mut (*chip).dmas[ATI_DMA_SPDIF])
    }
}

static snd_atiixp_playback_ops: snd_pcm_ops = snd_pcm_ops { open: Some(snd_atiixp_playback_open), close: Some(snd_atiixp_playback_close), hw_params: Some(snd_atiixp_pcm_hw_params), hw_free: Some(snd_atiixp_pcm_hw_free), prepare: Some(snd_atiixp_playback_prepare), trigger: Some(snd_atiixp_pcm_trigger), pointer: Some(snd_atiixp_pcm_pointer) };
static snd_atiixp_capture_ops: snd_pcm_ops = snd_pcm_ops { open: Some(snd_atiixp_capture_open), close: Some(snd_atiixp_capture_close), hw_params: Some(snd_atiixp_pcm_hw_params), hw_free: Some(snd_atiixp_pcm_hw_free), prepare: Some(snd_atiixp_capture_prepare), trigger: Some(snd_atiixp_pcm_trigger), pointer: Some(snd_atiixp_pcm_pointer) };
static snd_atiixp_spdif_ops: snd_pcm_ops = snd_pcm_ops { open: Some(snd_atiixp_spdif_open), close: Some(snd_atiixp_spdif_close), hw_params: Some(snd_atiixp_pcm_hw_params), hw_free: Some(snd_atiixp_pcm_hw_free), prepare: Some(snd_atiixp_spdif_prepare), trigger: Some(snd_atiixp_pcm_trigger), pointer: Some(snd_atiixp_pcm_pointer) };

static atiixp_pcm_defs: [ac97_pcm; 3] = [
    ac97_pcm { stream: 0, exclusive: 1, spdif: 0, rates: 0, r: [ac97_pcm_region { slots: (1 << AC97_SLOT_PCM_LEFT) | (1 << AC97_SLOT_PCM_RIGHT) | (1 << AC97_SLOT_PCM_CENTER) | (1 << AC97_SLOT_PCM_SLEFT) | (1 << AC97_SLOT_PCM_SRIGHT) | (1 << AC97_SLOT_LFE) }; 4] },
    ac97_pcm { stream: 1, exclusive: 1, spdif: 0, rates: 0, r: [ac97_pcm_region { slots: (1 << AC97_SLOT_PCM_LEFT) | (1 << AC97_SLOT_PCM_RIGHT) }; 4] },
    ac97_pcm { stream: 0, exclusive: 1, spdif: 1, rates: 0, r: [ac97_pcm_region { slots: (1 << AC97_SLOT_SPDIF_LEFT2) | (1 << AC97_SLOT_SPDIF_RIGHT2) }; 4] },
];

static snd_atiixp_playback_dma_ops: atiixp_dma_ops = atiixp_dma_ops { type_: ATI_DMA_PLAYBACK as c_int, llp_offset: ATI_REG_OUT_DMA_LINKPTR, dt_cur: ATI_REG_OUT_DMA_DT_CUR, enable_dma: Some(atiixp_out_enable_dma), enable_transfer: Some(atiixp_out_enable_transfer), flush_dma: Some(atiixp_out_flush_dma) };
static snd_atiixp_capture_dma_ops: atiixp_dma_ops = atiixp_dma_ops { type_: ATI_DMA_CAPTURE as c_int, llp_offset: ATI_REG_IN_DMA_LINKPTR, dt_cur: ATI_REG_IN_DMA_DT_CUR, enable_dma: Some(atiixp_in_enable_dma), enable_transfer: Some(atiixp_in_enable_transfer), flush_dma: Some(atiixp_in_flush_dma) };
static snd_atiixp_spdif_dma_ops: atiixp_dma_ops = atiixp_dma_ops { type_: ATI_DMA_SPDIF as c_int, llp_offset: ATI_REG_SPDF_DMA_LINKPTR, dt_cur: ATI_REG_SPDF_DMA_DT_CUR, enable_dma: Some(atiixp_spdif_enable_dma), enable_transfer: Some(atiixp_spdif_enable_transfer), flush_dma: Some(atiixp_spdif_flush_dma) };

unsafe extern "C" fn snd_atiixp_pcm_new(chip: *mut atiixp) -> c_int {
    let mut pcm: *mut snd_pcm = ptr::null_mut();
    let mut chmap: *mut snd_pcm_chmap = ptr::null_mut();
    let pbus = (*chip).ac97_bus;
    let mut err: c_int;
    let mut i: c_int;
    let num_pcms: c_int;
    (*chip).dmas[ATI_DMA_PLAYBACK].ops = &snd_atiixp_playback_dma_ops;
    (*chip).dmas[ATI_DMA_CAPTURE].ops = &snd_atiixp_capture_dma_ops;
    if (*chip).spdif_over_aclink == 0 { (*chip).dmas[ATI_DMA_SPDIF].ops = &snd_atiixp_spdif_dma_ops; }
    num_pcms = if (*chip).spdif_over_aclink != 0 { 3 } else { 2 };
    err = snd_ac97_pcm_assign(pbus, num_pcms, atiixp_pcm_defs.as_ptr());
    if err < 0 { return err; }
    i = 0;
    while i < num_pcms {
        (*chip).pcms[i as usize] = (*pbus).pcms.add(i as usize);
        i += 1;
    }
    (*chip).max_channels = 2;
    if (*(*pbus).pcms.add(ATI_PCM_OUT)).r[0].slots & (1 << AC97_SLOT_PCM_SLEFT) != 0 {
        if (*(*pbus).pcms.add(ATI_PCM_OUT)).r[0].slots & (1 << AC97_SLOT_LFE) != 0 { (*chip).max_channels = 6; } else { (*chip).max_channels = 4; }
    }
    err = snd_pcm_new((*chip).card, b"ATI IXP AC97\0".as_ptr() as *const c_char, ATI_PCMDEV_ANALOG as c_int, 1, 1, &mut pcm);
    if err < 0 { return err; }
    snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_PLAYBACK, &snd_atiixp_playback_ops);
    snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_CAPTURE, &snd_atiixp_capture_ops);
    (*pcm).private_data = chip as *mut c_void;
    strscpy((*pcm).name.as_mut_ptr(), b"ATI IXP AC97\0".as_ptr() as *const c_char);
    (*chip).pcmdevs[ATI_PCMDEV_ANALOG] = pcm;
    snd_pcm_set_managed_buffer_all(pcm, SNDRV_DMA_TYPE_DEV, &mut (*(*chip).pci).dev, 64 * 1024, 128 * 1024);
    err = snd_pcm_add_chmap_ctls(pcm, SNDRV_PCM_STREAM_PLAYBACK, snd_pcm_alt_chmaps, (*chip).max_channels, 0, &mut chmap);
    if err < 0 { return err; }
    (*chmap).channel_mask = SND_PCM_CHMAP_MASK_2468;
    (*(*chip).ac97[0]).chmaps[SNDRV_PCM_STREAM_PLAYBACK as usize] = chmap;
    if !(*chip).pcms[ATI_PCM_SPDIF].is_null() && (*(*chip).pcms[ATI_PCM_SPDIF]).rates == 0 { return 0; }
    if !(*chip).pcms[ATI_PCM_SPDIF].is_null() { (*(*chip).pcms[ATI_PCM_SPDIF]).rates = SNDRV_PCM_RATE_48000; }
    err = snd_pcm_new((*chip).card, b"ATI IXP IEC958\0".as_ptr() as *const c_char, ATI_PCMDEV_DIGITAL as c_int, 1, 0, &mut pcm);
    if err < 0 { return err; }
    snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_PLAYBACK, &snd_atiixp_spdif_ops);
    (*pcm).private_data = chip as *mut c_void;
    if (*chip).spdif_over_aclink != 0 { strscpy((*pcm).name.as_mut_ptr(), b"ATI IXP IEC958 (AC97)\0".as_ptr() as *const c_char); } else { strscpy((*pcm).name.as_mut_ptr(), b"ATI IXP IEC958 (Direct)\0".as_ptr() as *const c_char); }
    (*chip).pcmdevs[ATI_PCMDEV_DIGITAL] = pcm;
    snd_pcm_set_managed_buffer_all(pcm, SNDRV_DMA_TYPE_DEV, &mut (*(*chip).pci).dev, 64 * 1024, 128 * 1024);
    i = 0;
    while i < NUM_ATI_CODECS as c_int {
        if !(*chip).ac97[i as usize].is_null() {
            snd_ac97_update_bits((*chip).ac97[i as usize], AC97_EXTENDED_STATUS, 0x03 << 4, 0x03 << 4);
        }
        i += 1;
    }
    0
}

unsafe extern "C" fn snd_atiixp_interrupt(_irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    let chip = dev_id as *mut atiixp;
    let status = atiixp_read(chip, ATI_REG_ISR);
    if status == 0 { return IRQ_NONE; }
    if status & ATI_REG_ISR_OUT_XRUN != 0 { snd_atiixp_xrun_dma(chip, &mut (*chip).dmas[ATI_DMA_PLAYBACK]); } else if status & ATI_REG_ISR_OUT_STATUS != 0 { snd_atiixp_update_dma(chip, &mut (*chip).dmas[ATI_DMA_PLAYBACK]); }
    if status & ATI_REG_ISR_IN_XRUN != 0 { snd_atiixp_xrun_dma(chip, &mut (*chip).dmas[ATI_DMA_CAPTURE]); } else if status & ATI_REG_ISR_IN_STATUS != 0 { snd_atiixp_update_dma(chip, &mut (*chip).dmas[ATI_DMA_CAPTURE]); }
    if (*chip).spdif_over_aclink == 0 {
        if status & ATI_REG_ISR_SPDF_XRUN != 0 { snd_atiixp_xrun_dma(chip, &mut (*chip).dmas[ATI_DMA_SPDIF]); } else if status & ATI_REG_ISR_SPDF_STATUS != 0 { snd_atiixp_update_dma(chip, &mut (*chip).dmas[ATI_DMA_SPDIF]); }
    }
    if status & CODEC_CHECK_BITS != 0 {
        let detected = status & CODEC_CHECK_BITS;
        (*chip).codec_not_ready_bits |= detected;
        atiixp_update(chip, ATI_REG_IER, detected, 0);
    }
    atiixp_write(chip, ATI_REG_ISR, status);
    IRQ_HANDLED
}

static ac97_quirks: [ac97_quirk; 4] = [
    ac97_quirk { subvendor: 0x103c, subdevice: 0x006b, name: b"HP Pavilion ZV5030US\0".as_ptr() as *const c_char, type_: AC97_TUNE_MUTE_LED },
    ac97_quirk { subvendor: 0x103c, subdevice: 0x308b, name: b"HP nx6125\0".as_ptr() as *const c_char, type_: AC97_TUNE_MUTE_LED },
    ac97_quirk { subvendor: 0x103c, subdevice: 0x3091, name: b"unknown HP\0".as_ptr() as *const c_char, type_: AC97_TUNE_MUTE_LED },
    ac97_quirk { subvendor: 0, subdevice: 0, name: ptr::null(), type_: 0 },
];

unsafe extern "C" fn snd_atiixp_mixer_new(chip: *mut atiixp, clock: c_int, quirk_override: *const c_char) -> c_int {
    let mut pbus: *mut snd_ac97_bus = ptr::null_mut();
    let mut ac97: snd_ac97_template = core::mem::zeroed();
    let mut i: c_int;
    let mut err: c_int;
    let mut codec_count: c_int;
    static ops: snd_ac97_bus_ops = snd_ac97_bus_ops { write: Some(snd_atiixp_ac97_write), read: Some(snd_atiixp_ac97_read) };
    static codec_skip: [c_uint; NUM_ATI_CODECS] = [ATI_REG_ISR_CODEC0_NOT_READY, ATI_REG_ISR_CODEC1_NOT_READY, ATI_REG_ISR_CODEC2_NOT_READY];
    if snd_atiixp_codec_detect(chip) < 0 { return -ENXIO; }
    err = snd_ac97_bus((*chip).card, 0, &ops, chip as *mut c_void, &mut pbus);
    if err < 0 { return err; }
    (*pbus).clock = clock;
    (*chip).ac97_bus = pbus;
    codec_count = 0;
    i = 0;
    while i < NUM_ATI_CODECS as c_int {
        if (*chip).codec_not_ready_bits & codec_skip[i as usize] != 0 { i += 1; continue; }
        ac97 = core::mem::zeroed();
        ac97.private_data = chip;
        ac97.pci = (*chip).pci;
        ac97.num = i;
        ac97.scaps = AC97_SCAP_SKIP_MODEM | AC97_SCAP_POWER_SAVE;
        if (*chip).spdif_over_aclink == 0 { ac97.scaps |= AC97_SCAP_NO_SPDIF; }
        err = snd_ac97_mixer(pbus, &mut ac97, &mut (*chip).ac97[i as usize]);
        if err < 0 {
            (*chip).ac97[i as usize] = ptr::null_mut();
            dev_dbg((*(*chip).card).dev, b"codec %d not available for audio\n\0".as_ptr() as *const c_char, i);
            i += 1;
            continue;
        }
        codec_count += 1;
        i += 1;
    }
    if codec_count == 0 {
        dev_err((*(*chip).card).dev, b"no codec available\n\0".as_ptr() as *const c_char);
        return -ENODEV;
    }
    snd_ac97_tune_hardware((*chip).ac97[0], ac97_quirks.as_ptr(), quirk_override);
    0
}

unsafe extern "C" fn snd_atiixp_suspend(dev: *mut device) -> c_int {
    let card = dev_get_drvdata(dev) as *mut snd_card;
    let chip = (*card).private_data;
    let mut i = 0;
    snd_power_change_state(card, SNDRV_CTL_POWER_D3hot);
    while i < NUM_ATI_CODECS {
        snd_ac97_suspend((*chip).ac97[i]);
        i += 1;
    }
    snd_atiixp_aclink_down(chip);
    snd_atiixp_chip_stop(chip);
    0
}

unsafe extern "C" fn snd_atiixp_resume(dev: *mut device) -> c_int {
    let card = dev_get_drvdata(dev) as *mut snd_card;
    let chip = (*card).private_data;
    let mut i = 0usize;
    snd_atiixp_aclink_reset(chip);
    snd_atiixp_chip_start(chip);
    while i < NUM_ATI_CODECS {
        snd_ac97_resume((*chip).ac97[i]);
        i += 1;
    }
    i = 0;
    while i < NUM_ATI_PCMDEVS {
        if !(*chip).pcmdevs[i].is_null() {
            let dma = &mut (*chip).dmas[i] as *mut atiixp_dma;
            if !(*dma).substream.is_null() && (*dma).suspended != 0 {
                ((*(*dma).ops).enable_dma.unwrap())(chip, 1);
                ((*(*(*dma).substream).ops).prepare.unwrap())((*dma).substream);
                writel((*dma).desc_buf.addr as u32 | ATI_REG_LINKPTR_EN, reg_addr(chip, (*(*dma).ops).llp_offset));
            }
        }
        i += 1;
    }
    snd_power_change_state(card, SNDRV_CTL_POWER_D0);
    0
}

#[repr(C)]
pub struct dev_pm_ops {
    suspend: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    resume: Option<unsafe extern "C" fn(*mut device) -> c_int>,
}
static snd_atiixp_pm: dev_pm_ops = dev_pm_ops { suspend: Some(snd_atiixp_suspend), resume: Some(snd_atiixp_resume) };

unsafe extern "C" fn snd_atiixp_proc_read(entry: *mut snd_info_entry, buffer: *mut snd_info_buffer) {
    let chip = (*entry).private_data;
    let mut i = 0;
    while i < 256 {
        snd_iprintf(buffer, b"%02x: %08x\n\0".as_ptr() as *const c_char, i, readl(reg_addr(chip, i)));
        i += 4;
    }
}

unsafe extern "C" fn snd_atiixp_proc_init(chip: *mut atiixp) {
    snd_card_ro_proc_new((*chip).card, b"atiixp\0".as_ptr() as *const c_char, chip as *mut c_void, Some(snd_atiixp_proc_read));
}

unsafe extern "C" fn snd_atiixp_free(card: *mut snd_card) {
    snd_atiixp_chip_stop((*card).private_data);
}

unsafe extern "C" fn snd_atiixp_init(card: *mut snd_card, pci: *mut pci_dev) -> c_int {
    let mut err: c_int;
    let chip = (*card).private_data;
    err = pcim_enable_device(pci);
    if err < 0 { return err; }
    spin_lock_init(&mut (*chip).reg_lock);
    mutex_init(&mut (*chip).open_mutex);
    (*chip).card = card;
    (*chip).pci = pci;
    (*chip).irq = -1;
    (*chip).remap_addr = pcim_iomap_region(pci, 0, b"ATI IXP AC97\0".as_ptr() as *const c_char);
    if IS_ERR((*chip).remap_addr) { return PTR_ERR((*chip).remap_addr); }
    (*chip).addr = pci_resource_start(pci, 0);
    if devm_request_irq(&mut (*pci).dev, (*pci).irq, Some(snd_atiixp_interrupt), IRQF_SHARED, KBUILD_MODNAME, chip as *mut c_void) != 0 {
        dev_err((*card).dev, b"unable to grab IRQ %d\n\0".as_ptr() as *const c_char, (*pci).irq);
        return -EBUSY;
    }
    (*chip).irq = (*pci).irq;
    (*card).sync_irq = (*chip).irq;
    (*card).private_free = Some(snd_atiixp_free);
    pci_set_master(pci);
    0
}

unsafe extern "C" fn __snd_atiixp_probe(pci: *mut pci_dev, _pci_id: *const pci_device_id) -> c_int {
    let mut card: *mut snd_card = ptr::null_mut();
    let chip: *mut atiixp;
    let mut err: c_int;
    err = snd_devm_card_new(&mut (*pci).dev, index, id, THIS_MODULE, size_of::<atiixp>(), &mut card);
    if err < 0 { return err; }
    chip = (*card).private_data;
    strscpy((*card).driver.as_mut_ptr(), if spdif_aclink { b"ATIIXP\0".as_ptr() } else { b"ATIIXP-SPDMA\0".as_ptr() } as *const c_char);
    strscpy((*card).shortname.as_mut_ptr(), b"ATI IXP\0".as_ptr() as *const c_char);
    err = snd_atiixp_init(card, pci);
    if err < 0 { return err; }
    err = snd_atiixp_aclink_reset(chip);
    if err < 0 { return err; }
    (*chip).spdif_over_aclink = spdif_aclink as c_int;
    err = snd_atiixp_mixer_new(chip, ac97_clock, ac97_quirk);
    if err < 0 { return err; }
    err = snd_atiixp_pcm_new(chip);
    if err < 0 { return err; }
    snd_atiixp_proc_init(chip);
    snd_atiixp_chip_start(chip);
    snprintf((*card).longname.as_mut_ptr(), (*card).longname.len(), b"%s rev %x with %s at %#lx, irq %i\0".as_ptr() as *const c_char, (*card).shortname.as_ptr(), (*pci).revision, if !(*chip).ac97[0].is_null() { snd_ac97_get_short_name((*chip).ac97[0]) } else { b"?\0".as_ptr() as *const c_char }, (*chip).addr, (*chip).irq);
    err = snd_card_register(card);
    if err < 0 { return err; }
    pci_set_drvdata(pci, card as *mut c_void);
    0
}

unsafe extern "C" fn snd_atiixp_probe(pci: *mut pci_dev, pci_id: *const pci_device_id) -> c_int {
    snd_card_free_on_error(&mut (*pci).dev, __snd_atiixp_probe(pci, pci_id))
}

#[repr(C)]
pub struct driver_inner {
    pm: *const dev_pm_ops,
}

#[repr(C)]
pub struct pci_driver {
    name: *const c_char,
    id_table: *const pci_device_id,
    probe: Option<unsafe extern "C" fn(*mut pci_dev, *const pci_device_id) -> c_int>,
    driver: driver_inner,
}

static mut atiixp_driver: pci_driver = pci_driver {
    name: ptr::null(),
    id_table: snd_atiixp_ids.as_ptr(),
    probe: Some(snd_atiixp_probe),
    driver: driver_inner { pm: &snd_atiixp_pm },
};

/* module_param, MODULE_PARM_DESC, MODULE_DEVICE_TABLE, DEFINE_SIMPLE_DEV_PM_OPS,
 * and module_pci_driver are Linux module registration macros in the C source.
 * Their registration intent is preserved by the globals and driver table above.
 */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
