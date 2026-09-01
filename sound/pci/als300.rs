// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  als300.c - driver for Avance Logic ALS300/ALS300+ soundcards.
 *  Copyright (C) 2005 by Ash Willis <ashwillis@programmer.net>
 *
 *  TODO
 *  4 channel playback for ALS300+
 *  gameport
 *  mpu401
 *  opl3
 *
 *  NOTES
 *  The BLOCK_COUNTER registers for the ALS300(+) return a figure related to
 *  the position in the current period, NOT the whole buffer. It is important
 *  to know which period we are in so we can calculate the correct pointer.
 *  This is why we always use 2 periods. We can then use a flip-flop variable
 *  to keep track of what period we are in.
 */

/* Includes translated as external kernel/ALSA dependencies:
 * linux/delay.h, linux/init.h, linux/module.h, linux/pci.h,
 * linux/dma-mapping.h, linux/interrupt.h, linux/slab.h, linux/io.h,
 * sound/core.h, sound/control.h, sound/initval.h, sound/pcm.h,
 * sound/pcm_params.h, sound/ac97_codec.h, sound/opl3.h.
 */

use core::ffi::{c_char, c_int, c_ulong, c_void};
use core::mem;
use core::ptr;

type u8 = u8;
type u16 = u16;
type u32 = u32;
type bool_t = bool;
type snd_pcm_uframes_t = c_ulong;
type irqreturn_t = c_int;
type spinlock_t = c_void;

const IRQ_NONE: irqreturn_t = 0;
const IRQ_HANDLED: irqreturn_t = 1;

const SNDRV_CARDS: usize = 8;
const SNDRV_DEFAULT_IDX: [c_int; SNDRV_CARDS] = [0; SNDRV_CARDS];
const SNDRV_DEFAULT_STR: [*mut c_char; SNDRV_CARDS] = [ptr::null_mut(); SNDRV_CARDS];
const SNDRV_DEFAULT_ENABLE_PNP: [bool_t; SNDRV_CARDS] = [true; SNDRV_CARDS];

const ENOMEM: c_int = 12;
const EINVAL: c_int = 22;
const ENXIO: c_int = 6;
const EBUSY: c_int = 16;
const ENODEV: c_int = 19;
const ENOENT: c_int = 2;

const SNDRV_PCM_INFO_MMAP: u32 = 1 << 0;
const SNDRV_PCM_INFO_INTERLEAVED: u32 = 1 << 1;
const SNDRV_PCM_INFO_PAUSE: u32 = 1 << 2;
const SNDRV_PCM_INFO_MMAP_VALID: u32 = 1 << 3;
const SNDRV_PCM_FMTBIT_S16: u64 = 1 << 0;
const SNDRV_PCM_RATE_48000: u32 = 1 << 0;
const SNDRV_PCM_TRIGGER_START: c_int = 0;
const SNDRV_PCM_TRIGGER_RESUME: c_int = 1;
const SNDRV_PCM_TRIGGER_STOP: c_int = 2;
const SNDRV_PCM_TRIGGER_SUSPEND: c_int = 3;
const SNDRV_PCM_TRIGGER_PAUSE_PUSH: c_int = 4;
const SNDRV_PCM_TRIGGER_PAUSE_RELEASE: c_int = 5;
const SNDRV_PCM_STREAM_PLAYBACK: c_int = 0;
const SNDRV_PCM_STREAM_CAPTURE: c_int = 1;
const SNDRV_DMA_TYPE_DEV: c_int = 0;
const IRQF_SHARED: c_ulong = 0x80;
const SNDRV_CTL_POWER_D3hot: c_int = 3;
const SNDRV_CTL_POWER_D0: c_int = 0;

const THIS_MODULE: *mut c_void = ptr::null_mut();
const KBUILD_MODNAME: *const c_char = b"als300\0".as_ptr() as *const c_char;

/* snd_als300_set_irq_flag */
const IRQ_DISABLE: c_int = 0;
const IRQ_ENABLE: c_int = 1;

/* I/O port layout */
const AC97_ACCESS: c_ulong = 0x00;
const AC97_READ: c_ulong = 0x04;
const AC97_STATUS: c_ulong = 0x06;
const AC97_DATA_AVAIL: u8 = 1 << 6;
const AC97_BUSY: u8 = 1 << 7;
const ALS300_IRQ_STATUS: c_ulong = 0x07; /* ALS300 Only */
const IRQ_PLAYBACK: u8 = 1 << 3;
const IRQ_CAPTURE: u8 = 1 << 2;
const GCR_DATA: c_ulong = 0x08;
const GCR_INDEX: c_ulong = 0x0c;
const ALS300P_DRAM_IRQ_STATUS: c_ulong = 0x0d; /* ALS300+ Only */
const MPU_IRQ_STATUS: c_ulong = 0x0e; /* ALS300 Rev. E+, ALS300+ */
const ALS300P_IRQ_STATUS: c_ulong = 0x0f; /* ALS300+ Only */

/* General Control Registers */
const PLAYBACK_START: u16 = 0x80;
const PLAYBACK_END: u16 = 0x81;
const PLAYBACK_CONTROL: u16 = 0x82;
const TRANSFER_START: u32 = 1 << 16;
const FIFO_PAUSE: u32 = 1 << 17;
const RECORD_START: u16 = 0x83;
const RECORD_END: u16 = 0x84;
const RECORD_CONTROL: u16 = 0x85;
const DRAM_WRITE_CONTROL: u16 = 0x8b;
const WRITE_TRANS_START: u32 = 1 << 16;
const DRAM_MODE_2: u32 = 1 << 17;
const MISC_CONTROL: u16 = 0x8c;
const IRQ_SET_BIT: u32 = 1 << 15;
const VMUTE_NORMAL: u32 = 1 << 20;
const MMUTE_NORMAL: u32 = 1 << 21;
const MUS_VOC_VOL: u16 = 0x8e;
const PLAYBACK_BLOCK_COUNTER: u16 = 0x9a;
const RECORD_BLOCK_COUNTER: u16 = 0x9b;

const DEBUG_PLAY_REC: c_int = 0;

macro_rules! snd_als300_dbgplay {
    ($($arg:tt)*) => {{
        /* DEBUG_PLAY_REC is disabled in the source file. */
    }};
}

const DEVICE_ALS300: c_int = 0;
const DEVICE_ALS300_PLUS: c_int = 1;

/* MODULE_AUTHOR("Ash Willis <ashwillis@programmer.net>"); */
/* MODULE_DESCRIPTION("Avance Logic ALS300"); */
/* MODULE_LICENSE("GPL"); */

static mut index: [c_int; SNDRV_CARDS] = SNDRV_DEFAULT_IDX;
static mut id: [*mut c_char; SNDRV_CARDS] = SNDRV_DEFAULT_STR;
static mut enable: [bool_t; SNDRV_CARDS] = SNDRV_DEFAULT_ENABLE_PNP;

/* module_param_array/module parm descriptions are kernel metadata in C. */

#[repr(C)]
pub struct snd_card {
    private_data: *mut c_void,
    dev: *mut device,
    sync_irq: c_int,
    private_free: Option<unsafe extern "C" fn(*mut snd_card)>,
    driver: [c_char; 16],
    shortname: [c_char; 32],
    longname: [c_char; 80],
}

#[repr(C)]
pub struct pci_dev {
    dev: device,
    irq: c_int,
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm {
    private_data: *mut c_void,
    name: [c_char; 80],
}

#[repr(C)]
pub struct snd_pcm_substream {
    runtime: *mut snd_pcm_runtime,
}

#[repr(C)]
pub struct snd_pcm_runtime {
    hw: snd_pcm_hardware,
    private_data: *mut c_void,
    dma_addr: u32,
}

#[repr(C)]
pub struct snd_ac97 {
    private_data: *mut c_void,
}

#[repr(C)]
pub struct snd_opl3 {
    _private: [u8; 0],
}

#[repr(C)]
pub struct resource {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_ac97_bus {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_ac97_bus_ops {
    write: Option<unsafe extern "C" fn(*mut snd_ac97, u16, u16)>,
    read: Option<unsafe extern "C" fn(*mut snd_ac97, u16) -> u16>,
}

#[repr(C)]
pub struct snd_ac97_template {
    private_data: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_pcm_hardware {
    info: u32,
    formats: u64,
    rates: u32,
    rate_min: u32,
    rate_max: u32,
    channels_min: u32,
    channels_max: u32,
    buffer_bytes_max: usize,
    period_bytes_min: usize,
    period_bytes_max: usize,
    periods_min: u32,
    periods_max: u32,
}

#[repr(C)]
pub struct snd_pcm_ops {
    open: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    close: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    prepare: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    trigger: Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_int) -> c_int>,
    pointer: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> snd_pcm_uframes_t>,
}

#[repr(C)]
pub struct pci_device_id {
    vendor: u32,
    device: u32,
    subvendor: u32,
    subdevice: u32,
    class: u32,
    class_mask: u32,
    driver_data: c_ulong,
}

#[repr(C)]
pub struct dev_pm_ops {
    suspend: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    resume: Option<unsafe extern "C" fn(*mut device) -> c_int>,
}

#[repr(C)]
pub struct device_driver {
    pm: *const dev_pm_ops,
}

#[repr(C)]
pub struct pci_driver {
    name: *const c_char,
    id_table: *const pci_device_id,
    probe: Option<unsafe extern "C" fn(*mut pci_dev, *const pci_device_id) -> c_int>,
    driver: device_driver,
}

#[repr(C)]
pub struct snd_als300 {
    port: c_ulong,
    reg_lock: spinlock_t,
    card: *mut snd_card,
    pci: *mut pci_dev,
    pcm: *mut snd_pcm,
    playback_substream: *mut snd_pcm_substream,
    capture_substream: *mut snd_pcm_substream,
    ac97: *mut snd_ac97,
    opl3: *mut snd_opl3,
    res_port: *mut resource,
    irq: c_int,
    chip_type: c_int, /* ALS300 or ALS300+ */
    revision: c_char,
}

#[repr(C)]
pub struct snd_als300_substream_data {
    period_flipflop: c_int,
    control_register: c_int,
    block_counter_register: c_int,
}

const fn pci_device(vendor: u32, device: u32, driver_data: c_ulong) -> pci_device_id {
    pci_device_id {
        vendor,
        device,
        subvendor: 0,
        subdevice: 0,
        class: 0,
        class_mask: 0,
        driver_data,
    }
}

static snd_als300_ids: [pci_device_id; 3] = [
    pci_device(0x4005, 0x0300, DEVICE_ALS300 as c_ulong),
    pci_device(0x4005, 0x0308, DEVICE_ALS300_PLUS as c_ulong),
    pci_device(0, 0, 0),
];

/* MODULE_DEVICE_TABLE(pci, snd_als300_ids); */

extern "C" {
    fn outb(value: u8, port: c_ulong);
    fn outl(value: u32, port: c_ulong);
    fn inb(port: c_ulong) -> u8;
    fn inw(port: c_ulong) -> u16;
    fn inl(port: c_ulong) -> u32;
    fn udelay(usecs: c_ulong);
    fn snd_pcm_period_elapsed(substream: *mut snd_pcm_substream);
    fn kzalloc(size: usize, flags: c_int) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn snd_pcm_substream_chip(substream: *mut snd_pcm_substream) -> *mut snd_als300;
    fn snd_pcm_lib_period_bytes(substream: *mut snd_pcm_substream) -> usize;
    fn snd_pcm_lib_buffer_bytes(substream: *mut snd_pcm_substream) -> usize;
    fn bytes_to_frames(runtime: *mut snd_pcm_runtime, size: usize) -> snd_pcm_uframes_t;
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
    fn snd_pcm_new(
        card: *mut snd_card,
        id: *const c_char,
        device: c_int,
        playback_count: c_int,
        capture_count: c_int,
        rpcm: *mut *mut snd_pcm,
    ) -> c_int;
    fn strscpy(dest: *mut c_char, src: *const c_char) -> isize;
    fn snd_pcm_set_ops(pcm: *mut snd_pcm, direction: c_int, ops: *const snd_pcm_ops);
    fn snd_pcm_set_managed_buffer_all(
        pcm: *mut snd_pcm,
        type_: c_int,
        data: *mut device,
        size: usize,
        max: usize,
    );
    fn pcim_enable_device(pci: *mut pci_dev) -> c_int;
    fn dma_set_mask_and_coherent(dev: *mut device, mask: u64) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn pci_set_master(pci: *mut pci_dev);
    fn spin_lock_init(lock: *mut spinlock_t);
    fn pcim_request_all_regions(pci: *mut pci_dev, name: *const c_char) -> c_int;
    fn pci_resource_start(pci: *mut pci_dev, bar: c_int) -> c_ulong;
    fn devm_request_irq(
        dev: *mut device,
        irq: c_int,
        handler: *mut c_void,
        flags: c_ulong,
        name: *const c_char,
        dev_id: *mut c_void,
    ) -> c_int;
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn snd_power_change_state(card: *mut snd_card, state: c_int);
    fn snd_ac97_suspend(ac97: *mut snd_ac97);
    fn snd_ac97_resume(ac97: *mut snd_ac97);
    fn snd_devm_card_new(
        dev: *mut device,
        idx: c_int,
        id: *mut c_char,
        module: *mut c_void,
        extra_size: usize,
        card_ret: *mut *mut snd_card,
    ) -> c_int;
    fn sprintf(buf: *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn snd_card_register(card: *mut snd_card) -> c_int;
    fn pci_set_drvdata(pci: *mut pci_dev, data: *mut c_void);
    fn snd_card_free(card: *mut snd_card);
}

const fn DMA_BIT_MASK(n: u32) -> u64 {
    if n == 64 {
        !0u64
    } else {
        (1u64 << n) - 1
    }
}

#[inline]
unsafe fn snd_als300_gcr_read(port: c_ulong, reg: u16) -> u32 {
    unsafe {
        outb(reg as u8, port + GCR_INDEX);
        inl(port + GCR_DATA)
    }
}

#[inline]
unsafe fn snd_als300_gcr_write(port: c_ulong, reg: u16, val: u32) {
    unsafe {
        outb(reg as u8, port + GCR_INDEX);
        outl(val, port + GCR_DATA);
    }
}

/* Enable/Disable Interrupts */
unsafe extern "C" fn snd_als300_set_irq_flag(chip: *mut snd_als300, cmd: c_int) {
    unsafe {
        let mut tmp = snd_als300_gcr_read((*chip).port, MISC_CONTROL);

        /* boolean XOR check, since old vs. new hardware have
           directly reversed bit setting for ENABLE and DISABLE.
           ALS300+ acts like newer versions of ALS300 */
        if (((((*chip).revision > 5) || ((*chip).chip_type == DEVICE_ALS300_PLUS))
            ^ (cmd == IRQ_ENABLE)) as c_int)
            == 0
        {
            tmp |= IRQ_SET_BIT;
        } else {
            tmp &= !IRQ_SET_BIT;
        }
        snd_als300_gcr_write((*chip).port, MISC_CONTROL, tmp);
    }
}

unsafe extern "C" fn snd_als300_free(card: *mut snd_card) {
    unsafe {
        let chip = (*card).private_data as *mut snd_als300;
        snd_als300_set_irq_flag(chip, IRQ_DISABLE);
    }
}

unsafe extern "C" fn snd_als300_interrupt(_irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    unsafe {
        let status: u8;
        let chip = dev_id as *mut snd_als300;
        let mut data: *mut snd_als300_substream_data;

        status = inb((*chip).port + ALS300_IRQ_STATUS);
        if status == 0 {
            /* shared IRQ, for different device?? Exit ASAP! */
            return IRQ_NONE;
        }

        /* ACK everything ASAP */
        outb(status, (*chip).port + ALS300_IRQ_STATUS);
        if (status & IRQ_PLAYBACK) != 0 {
            if !(*chip).pcm.is_null() && !(*chip).playback_substream.is_null() {
                data = (*(*(*chip).playback_substream).runtime).private_data
                    as *mut snd_als300_substream_data;
                (*data).period_flipflop ^= 1;
                snd_pcm_period_elapsed((*chip).playback_substream);
                snd_als300_dbgplay!("IRQ_PLAYBACK\n");
            }
        }
        if (status & IRQ_CAPTURE) != 0 {
            if !(*chip).pcm.is_null() && !(*chip).capture_substream.is_null() {
                data = (*(*(*chip).capture_substream).runtime).private_data
                    as *mut snd_als300_substream_data;
                (*data).period_flipflop ^= 1;
                snd_pcm_period_elapsed((*chip).capture_substream);
                snd_als300_dbgplay!("IRQ_CAPTURE\n");
            }
        }
        IRQ_HANDLED
    }
}

unsafe extern "C" fn snd_als300plus_interrupt(_irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    unsafe {
        let general: u8;
        let mpu: u8;
        let dram: u8;
        let chip = dev_id as *mut snd_als300;
        let mut data: *mut snd_als300_substream_data;

        general = inb((*chip).port + ALS300P_IRQ_STATUS);
        mpu = inb((*chip).port + MPU_IRQ_STATUS);
        dram = inb((*chip).port + ALS300P_DRAM_IRQ_STATUS);

        /* shared IRQ, for different device?? Exit ASAP! */
        if general == 0 && (mpu & 0x80) == 0 && (dram & 0x01) == 0 {
            return IRQ_NONE;
        }

        if (general & IRQ_PLAYBACK) != 0 {
            if !(*chip).pcm.is_null() && !(*chip).playback_substream.is_null() {
                outb(IRQ_PLAYBACK, (*chip).port + ALS300P_IRQ_STATUS);
                data = (*(*(*chip).playback_substream).runtime).private_data
                    as *mut snd_als300_substream_data;
                (*data).period_flipflop ^= 1;
                snd_pcm_period_elapsed((*chip).playback_substream);
                snd_als300_dbgplay!("IRQ_PLAYBACK\n");
            }
        }
        if (general & IRQ_CAPTURE) != 0 {
            if !(*chip).pcm.is_null() && !(*chip).capture_substream.is_null() {
                outb(IRQ_CAPTURE, (*chip).port + ALS300P_IRQ_STATUS);
                data = (*(*(*chip).capture_substream).runtime).private_data
                    as *mut snd_als300_substream_data;
                (*data).period_flipflop ^= 1;
                snd_pcm_period_elapsed((*chip).capture_substream);
                snd_als300_dbgplay!("IRQ_CAPTURE\n");
            }
        }
        /* FIXME: Ack other interrupt types. Not important right now as
         * those other devices aren't enabled. */
        IRQ_HANDLED
    }
}

unsafe extern "C" fn snd_als300_ac97_read(ac97: *mut snd_ac97, reg: u16) -> u16 {
    unsafe {
        let mut i: c_int;
        let chip = (*ac97).private_data as *mut snd_als300;

        i = 0;
        while i < 1000 {
            if (inb((*chip).port + AC97_STATUS) & AC97_BUSY) == 0 {
                break;
            }
            udelay(10);
            i += 1;
        }
        outl(((reg as u32) << 24) | (1 << 31), (*chip).port + AC97_ACCESS);

        i = 0;
        while i < 1000 {
            if (inb((*chip).port + AC97_STATUS) & AC97_DATA_AVAIL) != 0 {
                break;
            }
            udelay(10);
            i += 1;
        }
        inw((*chip).port + AC97_READ)
    }
}

unsafe extern "C" fn snd_als300_ac97_write(ac97: *mut snd_ac97, reg: u16, val: u16) {
    unsafe {
        let mut i: c_int;
        let chip = (*ac97).private_data as *mut snd_als300;

        i = 0;
        while i < 1000 {
            if (inb((*chip).port + AC97_STATUS) & AC97_BUSY) == 0 {
                break;
            }
            udelay(10);
            i += 1;
        }
        outl(((reg as u32) << 24) | val as u32, (*chip).port + AC97_ACCESS);
    }
}

unsafe fn snd_als300_ac97(chip: *mut snd_als300) -> c_int {
    unsafe {
        let mut bus: *mut snd_ac97_bus = ptr::null_mut();
        let mut ac97: snd_ac97_template = mem::zeroed();
        let mut err: c_int;
        static ops: snd_ac97_bus_ops = snd_ac97_bus_ops {
            write: Some(snd_als300_ac97_write),
            read: Some(snd_als300_ac97_read),
        };

        err = snd_ac97_bus((*chip).card, 0, &ops, ptr::null_mut(), &mut bus);
        if err < 0 {
            return err;
        }

        ac97 = mem::zeroed();
        ac97.private_data = chip as *mut c_void;

        snd_ac97_mixer(bus, &mut ac97, &mut (*chip).ac97)
    }
}

/* hardware definition
 *
 * In AC97 mode, we always use 48k/16bit/stereo.
 * Any request to change data type is ignored by
 * the card when it is running outside of legacy
 * mode.
 */
static snd_als300_playback_hw: snd_pcm_hardware = snd_pcm_hardware {
    info: SNDRV_PCM_INFO_MMAP
        | SNDRV_PCM_INFO_INTERLEAVED
        | SNDRV_PCM_INFO_PAUSE
        | SNDRV_PCM_INFO_MMAP_VALID,
    formats: SNDRV_PCM_FMTBIT_S16,
    rates: SNDRV_PCM_RATE_48000,
    rate_min: 48000,
    rate_max: 48000,
    channels_min: 2,
    channels_max: 2,
    buffer_bytes_max: 64 * 1024,
    period_bytes_min: 64,
    period_bytes_max: 32 * 1024,
    periods_min: 2,
    periods_max: 2,
};

static snd_als300_capture_hw: snd_pcm_hardware = snd_pcm_hardware {
    info: SNDRV_PCM_INFO_MMAP
        | SNDRV_PCM_INFO_INTERLEAVED
        | SNDRV_PCM_INFO_PAUSE
        | SNDRV_PCM_INFO_MMAP_VALID,
    formats: SNDRV_PCM_FMTBIT_S16,
    rates: SNDRV_PCM_RATE_48000,
    rate_min: 48000,
    rate_max: 48000,
    channels_min: 2,
    channels_max: 2,
    buffer_bytes_max: 64 * 1024,
    period_bytes_min: 64,
    period_bytes_max: 32 * 1024,
    periods_min: 2,
    periods_max: 2,
};

unsafe extern "C" fn snd_als300_playback_open(substream: *mut snd_pcm_substream) -> c_int {
    unsafe {
        let chip = snd_pcm_substream_chip(substream);
        let runtime = (*substream).runtime;
        let data = kzalloc(mem::size_of::<snd_als300_substream_data>(), 0)
            as *mut snd_als300_substream_data;

        if data.is_null() {
            return -ENOMEM;
        }
        (*chip).playback_substream = substream;
        (*runtime).hw = snd_als300_playback_hw;
        (*runtime).private_data = data as *mut c_void;
        (*data).control_register = PLAYBACK_CONTROL as c_int;
        (*data).block_counter_register = PLAYBACK_BLOCK_COUNTER as c_int;
        0
    }
}

unsafe extern "C" fn snd_als300_playback_close(substream: *mut snd_pcm_substream) -> c_int {
    unsafe {
        let chip = snd_pcm_substream_chip(substream);
        let data: *mut snd_als300_substream_data;

        data = (*(*substream).runtime).private_data as *mut snd_als300_substream_data;
        kfree(data as *mut c_void);
        (*chip).playback_substream = ptr::null_mut();
        0
    }
}

unsafe extern "C" fn snd_als300_capture_open(substream: *mut snd_pcm_substream) -> c_int {
    unsafe {
        let chip = snd_pcm_substream_chip(substream);
        let runtime = (*substream).runtime;
        let data = kzalloc(mem::size_of::<snd_als300_substream_data>(), 0)
            as *mut snd_als300_substream_data;

        if data.is_null() {
            return -ENOMEM;
        }
        (*chip).capture_substream = substream;
        (*runtime).hw = snd_als300_capture_hw;
        (*runtime).private_data = data as *mut c_void;
        (*data).control_register = RECORD_CONTROL as c_int;
        (*data).block_counter_register = RECORD_BLOCK_COUNTER as c_int;
        0
    }
}

unsafe extern "C" fn snd_als300_capture_close(substream: *mut snd_pcm_substream) -> c_int {
    unsafe {
        let chip = snd_pcm_substream_chip(substream);
        let data: *mut snd_als300_substream_data;

        data = (*(*substream).runtime).private_data as *mut snd_als300_substream_data;
        kfree(data as *mut c_void);
        (*chip).capture_substream = ptr::null_mut();
        0
    }
}

unsafe extern "C" fn snd_als300_playback_prepare(substream: *mut snd_pcm_substream) -> c_int {
    unsafe {
        let mut tmp: u32;
        let chip = snd_pcm_substream_chip(substream);
        let runtime = (*substream).runtime;
        let period_bytes = snd_pcm_lib_period_bytes(substream) as u16;
        let buffer_bytes = snd_pcm_lib_buffer_bytes(substream) as u16;

        /* guard(spinlock_irq)(&chip->reg_lock); */
        tmp = snd_als300_gcr_read((*chip).port, PLAYBACK_CONTROL);
        tmp &= !TRANSFER_START;

        snd_als300_dbgplay!(
            "Period bytes: %d Buffer bytes %d\n",
            period_bytes,
            buffer_bytes
        );

        /* set block size */
        tmp &= 0xffff0000;
        tmp |= (period_bytes - 1) as u32;
        snd_als300_gcr_write((*chip).port, PLAYBACK_CONTROL, tmp);

        /* set dma area */
        snd_als300_gcr_write((*chip).port, PLAYBACK_START, (*runtime).dma_addr);
        snd_als300_gcr_write(
            (*chip).port,
            PLAYBACK_END,
            (*runtime).dma_addr + buffer_bytes as u32 - 1,
        );
        0
    }
}

unsafe extern "C" fn snd_als300_capture_prepare(substream: *mut snd_pcm_substream) -> c_int {
    unsafe {
        let mut tmp: u32;
        let chip = snd_pcm_substream_chip(substream);
        let runtime = (*substream).runtime;
        let period_bytes = snd_pcm_lib_period_bytes(substream) as u16;
        let buffer_bytes = snd_pcm_lib_buffer_bytes(substream) as u16;

        /* guard(spinlock_irq)(&chip->reg_lock); */
        tmp = snd_als300_gcr_read((*chip).port, RECORD_CONTROL);
        tmp &= !TRANSFER_START;

        snd_als300_dbgplay!(
            "Period bytes: %d Buffer bytes %d\n",
            period_bytes,
            buffer_bytes
        );

        /* set block size */
        tmp &= 0xffff0000;
        tmp |= (period_bytes - 1) as u32;

        /* set dma area */
        snd_als300_gcr_write((*chip).port, RECORD_CONTROL, tmp);
        snd_als300_gcr_write((*chip).port, RECORD_START, (*runtime).dma_addr);
        snd_als300_gcr_write(
            (*chip).port,
            RECORD_END,
            (*runtime).dma_addr + buffer_bytes as u32 - 1,
        );
        0
    }
}

unsafe extern "C" fn snd_als300_trigger(
    substream: *mut snd_pcm_substream,
    cmd: c_int,
) -> c_int {
    unsafe {
        let chip = snd_pcm_substream_chip(substream);
        let mut tmp: u32;
        let data: *mut snd_als300_substream_data;
        let reg: u16;
        let mut ret: c_int = 0;

        data = (*(*substream).runtime).private_data as *mut snd_als300_substream_data;
        reg = (*data).control_register as u16;

        /* guard(spinlock)(&chip->reg_lock); */
        match cmd {
            SNDRV_PCM_TRIGGER_START | SNDRV_PCM_TRIGGER_RESUME => {
                tmp = snd_als300_gcr_read((*chip).port, reg);
                (*data).period_flipflop = 1;
                snd_als300_gcr_write((*chip).port, reg, tmp | TRANSFER_START);
                snd_als300_dbgplay!("TRIGGER START\n");
            }
            SNDRV_PCM_TRIGGER_STOP | SNDRV_PCM_TRIGGER_SUSPEND => {
                tmp = snd_als300_gcr_read((*chip).port, reg);
                snd_als300_gcr_write((*chip).port, reg, tmp & !TRANSFER_START);
                snd_als300_dbgplay!("TRIGGER STOP\n");
            }
            SNDRV_PCM_TRIGGER_PAUSE_PUSH => {
                tmp = snd_als300_gcr_read((*chip).port, reg);
                snd_als300_gcr_write((*chip).port, reg, tmp | FIFO_PAUSE);
                snd_als300_dbgplay!("TRIGGER PAUSE\n");
            }
            SNDRV_PCM_TRIGGER_PAUSE_RELEASE => {
                tmp = snd_als300_gcr_read((*chip).port, reg);
                snd_als300_gcr_write((*chip).port, reg, tmp & !FIFO_PAUSE);
                snd_als300_dbgplay!("TRIGGER RELEASE\n");
            }
            _ => {
                snd_als300_dbgplay!("TRIGGER INVALID\n");
                ret = -EINVAL;
            }
        }
        ret
    }
}

unsafe extern "C" fn snd_als300_pointer(
    substream: *mut snd_pcm_substream,
) -> snd_pcm_uframes_t {
    unsafe {
        let mut current_ptr: u16;
        let chip = snd_pcm_substream_chip(substream);
        let data: *mut snd_als300_substream_data;
        let period_bytes: u16;

        data = (*(*substream).runtime).private_data as *mut snd_als300_substream_data;
        period_bytes = snd_pcm_lib_period_bytes(substream) as u16;

        /* scoped_guard(spinlock, &chip->reg_lock) */
        current_ptr = snd_als300_gcr_read((*chip).port, (*data).block_counter_register as u16)
            as u16
            + 4;

        if current_ptr > period_bytes {
            current_ptr = 0;
        } else {
            current_ptr = period_bytes - current_ptr;
        }

        if (*data).period_flipflop == 0 {
            current_ptr += period_bytes;
        }
        snd_als300_dbgplay!("Pointer (bytes): %d\n", current_ptr);
        bytes_to_frames((*substream).runtime, current_ptr as usize)
    }
}

static snd_als300_playback_ops: snd_pcm_ops = snd_pcm_ops {
    open: Some(snd_als300_playback_open),
    close: Some(snd_als300_playback_close),
    prepare: Some(snd_als300_playback_prepare),
    trigger: Some(snd_als300_trigger),
    pointer: Some(snd_als300_pointer),
};

static snd_als300_capture_ops: snd_pcm_ops = snd_pcm_ops {
    open: Some(snd_als300_capture_open),
    close: Some(snd_als300_capture_close),
    prepare: Some(snd_als300_capture_prepare),
    trigger: Some(snd_als300_trigger),
    pointer: Some(snd_als300_pointer),
};

unsafe fn snd_als300_new_pcm(chip: *mut snd_als300) -> c_int {
    unsafe {
        let mut pcm: *mut snd_pcm = ptr::null_mut();
        let mut err: c_int;

        err = snd_pcm_new((*chip).card, b"ALS300\0".as_ptr() as *const c_char, 0, 1, 1, &mut pcm);
        if err < 0 {
            return err;
        }
        (*pcm).private_data = chip as *mut c_void;
        strscpy((*pcm).name.as_mut_ptr(), b"ALS300\0".as_ptr() as *const c_char);
        (*chip).pcm = pcm;

        /* set operators */
        snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_PLAYBACK, &snd_als300_playback_ops);
        snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_CAPTURE, &snd_als300_capture_ops);

        /* pre-allocation of buffers */
        snd_pcm_set_managed_buffer_all(
            pcm,
            SNDRV_DMA_TYPE_DEV,
            &mut (*(*chip).pci).dev,
            64 * 1024,
            64 * 1024,
        );
        0
    }
}

unsafe fn snd_als300_init(chip: *mut snd_als300) {
    unsafe {
        let mut tmp: u32;

        /* guard(spinlock_irqsave)(&chip->reg_lock); */
        (*chip).revision =
            ((snd_als300_gcr_read((*chip).port, MISC_CONTROL) >> 16) & 0x0000000f) as c_char;
        /* Setup DRAM */
        tmp = snd_als300_gcr_read((*chip).port, DRAM_WRITE_CONTROL);
        snd_als300_gcr_write(
            (*chip).port,
            DRAM_WRITE_CONTROL,
            (tmp | DRAM_MODE_2) & !WRITE_TRANS_START,
        );

        /* Enable IRQ output */
        snd_als300_set_irq_flag(chip, IRQ_ENABLE);

        /* Unmute hardware devices so their outputs get routed to
         * the onboard mixer */
        tmp = snd_als300_gcr_read((*chip).port, MISC_CONTROL);
        snd_als300_gcr_write((*chip).port, MISC_CONTROL, tmp | VMUTE_NORMAL | MMUTE_NORMAL);

        /* Reset volumes */
        snd_als300_gcr_write((*chip).port, MUS_VOC_VOL, 0);

        /* Make sure playback transfer is stopped */
        tmp = snd_als300_gcr_read((*chip).port, PLAYBACK_CONTROL);
        snd_als300_gcr_write((*chip).port, PLAYBACK_CONTROL, tmp & !TRANSFER_START);
    }
}

unsafe fn snd_als300_create(
    card: *mut snd_card,
    pci: *mut pci_dev,
    chip_type: c_int,
) -> c_int {
    unsafe {
        let chip = (*card).private_data as *mut snd_als300;
        let irq_handler: *mut c_void;
        let mut err: c_int;

        err = pcim_enable_device(pci);
        if err < 0 {
            return err;
        }

        if dma_set_mask_and_coherent(&mut (*pci).dev, DMA_BIT_MASK(28)) != 0 {
            dev_err(
                (*card).dev,
                b"error setting 28bit DMA mask\n\0".as_ptr() as *const c_char,
            );
            return -ENXIO;
        }
        pci_set_master(pci);

        (*chip).card = card;
        (*chip).pci = pci;
        (*chip).irq = -1;
        (*chip).chip_type = chip_type;
        spin_lock_init(&mut (*chip).reg_lock);

        err = pcim_request_all_regions(pci, b"ALS300\0".as_ptr() as *const c_char);
        if err < 0 {
            return err;
        }

        (*chip).port = pci_resource_start(pci, 0);

        if (*chip).chip_type == DEVICE_ALS300_PLUS {
            irq_handler = snd_als300plus_interrupt as *mut c_void;
        } else {
            irq_handler = snd_als300_interrupt as *mut c_void;
        }

        if devm_request_irq(
            &mut (*pci).dev,
            (*pci).irq,
            irq_handler,
            IRQF_SHARED,
            KBUILD_MODNAME,
            chip as *mut c_void,
        ) != 0
        {
            dev_err(
                (*card).dev,
                b"unable to grab IRQ %d\n\0".as_ptr() as *const c_char,
                (*pci).irq,
            );
            return -EBUSY;
        }
        (*chip).irq = (*pci).irq;
        (*card).sync_irq = (*chip).irq;
        (*card).private_free = Some(snd_als300_free);

        snd_als300_init(chip);

        err = snd_als300_ac97(chip);
        if err < 0 {
            dev_err(
                (*card).dev,
                b"Could not create ac97\n\0".as_ptr() as *const c_char,
            );
            return err;
        }

        err = snd_als300_new_pcm(chip);
        if err < 0 {
            dev_err(
                (*card).dev,
                b"Could not create PCM\n\0".as_ptr() as *const c_char,
            );
            return err;
        }

        0
    }
}

unsafe extern "C" fn snd_als300_suspend(dev: *mut device) -> c_int {
    unsafe {
        let card = dev_get_drvdata(dev) as *mut snd_card;
        let chip = (*card).private_data as *mut snd_als300;

        snd_power_change_state(card, SNDRV_CTL_POWER_D3hot);
        snd_ac97_suspend((*chip).ac97);
        0
    }
}

unsafe extern "C" fn snd_als300_resume(dev: *mut device) -> c_int {
    unsafe {
        let card = dev_get_drvdata(dev) as *mut snd_card;
        let chip = (*card).private_data as *mut snd_als300;

        snd_als300_init(chip);
        snd_ac97_resume((*chip).ac97);

        snd_power_change_state(card, SNDRV_CTL_POWER_D0);
        0
    }
}

static snd_als300_pm: dev_pm_ops = dev_pm_ops {
    suspend: Some(snd_als300_suspend),
    resume: Some(snd_als300_resume),
};

unsafe extern "C" fn snd_als300_probe(
    pci: *mut pci_dev,
    pci_id: *const pci_device_id,
) -> c_int {
    unsafe {
        static mut dev: c_int = 0;
        let mut card: *mut snd_card = ptr::null_mut();
        let chip: *mut snd_als300;
        let mut err: c_int;
        let chip_type: c_int;

        if dev >= SNDRV_CARDS as c_int {
            return -ENODEV;
        }
        if !enable[dev as usize] {
            dev += 1;
            return -ENOENT;
        }

        err = snd_devm_card_new(
            &mut (*pci).dev,
            index[dev as usize],
            id[dev as usize],
            THIS_MODULE,
            mem::size_of::<snd_als300>(),
            &mut card,
        );
        if err < 0 {
            return err;
        }
        chip = (*card).private_data as *mut snd_als300;

        chip_type = (*pci_id).driver_data as c_int;

        err = snd_als300_create(card, pci, chip_type);
        if err < 0 {
            snd_card_free(card);
            return err;
        }

        strscpy(
            (*card).driver.as_mut_ptr(),
            b"ALS300\0".as_ptr() as *const c_char,
        );
        if (*chip).chip_type == DEVICE_ALS300_PLUS {
            /* don't know much about ALS300+ yet
             * print revision number for now */
            sprintf(
                (*card).shortname.as_mut_ptr(),
                b"ALS300+ (Rev. %d)\0".as_ptr() as *const c_char,
                (*chip).revision as c_int,
            );
        } else {
            sprintf(
                (*card).shortname.as_mut_ptr(),
                b"ALS300 (Rev. %c)\0".as_ptr() as *const c_char,
                'A' as c_int + (*chip).revision as c_int - 1,
            );
        }
        sprintf(
            (*card).longname.as_mut_ptr(),
            b"%s at 0x%lx irq %i\0".as_ptr() as *const c_char,
            (*card).shortname.as_ptr(),
            (*chip).port,
            (*chip).irq,
        );

        err = snd_card_register(card);
        if err < 0 {
            snd_card_free(card);
            return err;
        }

        pci_set_drvdata(pci, card as *mut c_void);
        dev += 1;
        0
    }
}

static mut als300_driver: pci_driver = pci_driver {
    name: KBUILD_MODNAME,
    id_table: snd_als300_ids.as_ptr(),
    probe: Some(snd_als300_probe),
    driver: device_driver {
        pm: &snd_als300_pm,
    },
};

/* module_pci_driver(als300_driver); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
