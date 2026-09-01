// SPDX-License-Identifier: GPL-2.0-only
/*
*
* Copyright Adrian McMenamin 2005, 2006, 2007
* <adrian@mcmen.demon.co.uk>
* Requires firmware (BSD licenced) available from:
* http://linuxdc.cvs.sourceforge.net/linuxdc/linux-sh-dc/sound/oss/aica/firmware/
* or the maintainer
*/

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_ulong, c_void};
use core::ptr;

// C dependencies removed from executable Rust:
// linux/init.h, linux/jiffies.h, linux/slab.h, linux/time.h, linux/wait.h,
// linux/module.h, linux/platform_device.h, linux/firmware.h, linux/timer.h,
// linux/delay.h, linux/workqueue.h, linux/io.h, sound/core.h, sound/control.h,
// sound/pcm.h, sound/initval.h, sound/info.h, asm/dma.h, mach/sysasic.h,
// and "aica.h".
//
// MODULE_AUTHOR("Adrian McMenamin <adrian@mcmen.demon.co.uk>");
// MODULE_DESCRIPTION("Dreamcast AICA sound (pcm) driver");
// MODULE_LICENSE("GPL");
// MODULE_FIRMWARE("aica_firmware.bin");

type u32 = u32;
type bool_ = bool;

const CARD_NAME: &[u8] = b"AICA\0";
static mut index: c_int = -1;
static mut id: *mut c_char = ptr::null_mut();
static mut enable: bool_ = true;
// module_param(index, int, 0444);
// MODULE_PARM_DESC(index, "Index value for " CARD_NAME " soundcard.");
// module_param(id, charp, 0444);
// MODULE_PARM_DESC(id, "ID string for " CARD_NAME " soundcard.");
// module_param(enable, bool, 0644);
// MODULE_PARM_DESC(enable, "Enable " CARD_NAME " soundcard.");

#[repr(C)]
pub struct resource {
    pub name: *const c_char,
    pub start: c_ulong,
    pub end: c_ulong,
    pub flags: c_ulong,
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct platform_driver__driver {
    pub name: *const c_char,
}

#[repr(C)]
pub struct platform_driver {
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut platform_device)>,
    pub driver: platform_driver__driver,
}

#[repr(C)]
pub struct firmware {
    pub size: usize,
    pub data: *const u8,
}

#[repr(C)]
pub struct work_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct timer_list {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_card {
    pub driver: [c_char; 0],
    pub shortname: [c_char; 0],
    pub longname: [c_char; 0],
}

#[repr(C)]
pub struct snd_pcm {
    pub private_data: *mut c_void,
    pub name: [c_char; 0],
}

#[repr(C)]
pub struct snd_pcm_hardware {
    pub info: c_ulong,
    pub formats: c_ulong,
    pub rates: c_ulong,
    pub rate_min: c_uint,
    pub rate_max: c_uint,
    pub channels_min: c_uint,
    pub channels_max: c_uint,
    pub buffer_bytes_max: usize,
    pub period_bytes_min: usize,
    pub period_bytes_max: usize,
    pub periods_min: c_uint,
    pub periods_max: c_uint,
}

type c_uint = u32;

#[repr(C)]
pub struct snd_pcm_runtime {
    pub hw: snd_pcm_hardware,
    pub dma_area: *mut u8,
    pub buffer_size: c_ulong,
    pub channels: c_uint,
    pub format: c_int,
    pub rate: c_uint,
}

#[repr(C)]
pub struct snd_pcm_substream {
    pub pcm: *mut snd_pcm,
    pub runtime: *mut snd_pcm_runtime,
}

#[repr(C)]
pub struct snd_pcm_ops {
    pub open: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    pub close: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    pub prepare: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    pub trigger: Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_int) -> c_int>,
    pub pointer: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_ulong>,
    pub sync_stop: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
}

#[repr(C)]
pub struct snd_kcontrol {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_ctl_elem_info__integer {
    pub min: c_long,
    pub max: c_long,
}

#[repr(C)]
pub union snd_ctl_elem_info__value {
    pub integer: core::mem::ManuallyDrop<snd_ctl_elem_info__integer>,
}

#[repr(C)]
pub struct snd_ctl_elem_info {
    pub type_: c_uint,
    pub count: c_uint,
    pub value: snd_ctl_elem_info__value,
}

type c_long = i64;

#[repr(C)]
pub struct snd_ctl_elem_value__integer {
    pub value: [c_long; 1],
}

#[repr(C)]
pub union snd_ctl_elem_value__value {
    pub integer: core::mem::ManuallyDrop<snd_ctl_elem_value__integer>,
}

#[repr(C)]
pub struct snd_ctl_elem_value {
    pub value: snd_ctl_elem_value__value,
}

#[repr(C)]
pub struct snd_kcontrol_new {
    pub iface: c_uint,
    pub name: *const c_char,
    pub index: c_uint,
    pub info: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_info) -> c_int>,
    pub get: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    pub put: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
}

#[repr(C)]
pub struct aica_channel {
    pub sfmt: u32,
    pub cmd: u32,
    pub vol: u32,
    pub pan: u32,
    pub pos: u32,
    pub flags: u32,
    pub freq: c_uint,
}

#[repr(C)]
pub struct snd_card_aica {
    pub card: *mut snd_card,
    pub channel: *mut aica_channel,
    pub spu_dma_work: work_struct,
    pub timer: timer_list,
    pub substream: *mut snd_pcm_substream,
    pub clicks: c_int,
    pub current_period: c_int,
    pub dma_check: c_int,
    pub master_volume: u32,
}

extern "C" {
    static mut pd: *mut platform_device;

    static mut jiffies: c_ulong;
    static ARM_RESET_REGISTER: c_ulong;
    static SPU_MEMORY_BASE: c_ulong;
    static SPU_REGISTER_BASE: c_ulong;
    static G2_FIFO: c_ulong;
    static AICA_CONTROL_POINT: c_ulong;
    static AICA_CONTROL_CHANNEL_SAMPLE_NUMBER: c_ulong;

    static IORESOURCE_MEM: c_ulong;
    static SNDRV_PCM_INFO_NONINTERLEAVED: c_ulong;
    static SNDRV_PCM_FMTBIT_S8: c_ulong;
    static SNDRV_PCM_FMTBIT_S16_LE: c_ulong;
    static SNDRV_PCM_FMTBIT_IMA_ADPCM: c_ulong;
    static SNDRV_PCM_RATE_8000_48000: c_ulong;
    static AICA_BUFFER_SIZE: usize;
    static AICA_PERIOD_SIZE: usize;
    static AICA_PERIOD_NUMBER: c_uint;
    static AICA_DMA_CHANNEL: c_int;
    static AICA_CHANNEL0_OFFSET: c_ulong;
    static CHANNEL_OFFSET: c_ulong;
    static AICA_DMA_MODE: c_int;
    static AICA_CHANNEL0_CONTROL_OFFSET: u32;
    static AICA_CMD_KICK: u32;
    static AICA_CMD_START: u32;
    static AICA_CMD_STOP: u32;
    static SM_8BIT: u32;
    static SM_16BIT: u32;
    static SNDRV_PCM_FORMAT_S16_LE: c_int;
    static SNDRV_PCM_TRIGGER_START: c_int;
    static SNDRV_PCM_TRIGGER_STOP: c_int;
    static SNDRV_PCM_STREAM_PLAYBACK: c_int;
    static SNDRV_DMA_TYPE_CONTINUOUS: c_int;
    static SNDRV_CTL_ELEM_TYPE_INTEGER: c_uint;
    static SNDRV_CTL_ELEM_IFACE_MIXER: c_uint;
    static SND_AICA_DRIVER: *const c_char;
    static THIS_MODULE: *mut c_void;

    fn readl(addr: c_ulong) -> u32;
    fn writel(value: u32, addr: *mut u32);
    fn __raw_writel(value: u32, addr: c_ulong);
    fn local_irq_save(flags: *mut c_ulong);
    fn local_irq_restore(flags: c_ulong);
    fn pr_warn(fmt: *const c_char);
    fn snd_BUG_ON(cond: c_int) -> c_int;
    fn dma_xfer(channel: c_int, from: c_ulong, to: c_ulong, size: c_int, mode: c_int) -> c_int;
    fn dma_wait_for_completion(channel: c_int);
    fn frames_to_bytes(runtime: *mut snd_pcm_runtime, frames: c_ulong) -> c_ulong;
    fn snd_pcm_period_elapsed(substream: *mut snd_pcm_substream);
    fn snd_pcm_running(substream: *mut snd_pcm_substream) -> c_int;
    fn mod_timer(timer: *mut timer_list, expires: c_ulong) -> c_int;
    fn schedule_work(work: *mut work_struct) -> c_int;
    fn timer_delete_sync(timer: *mut timer_list) -> c_int;
    fn cancel_work_sync(work: *mut work_struct) -> bool_;
    fn kmalloc(size: usize, flags: c_uint) -> *mut c_void;
    fn kzalloc(size: usize, flags: c_uint) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn snd_pcm_new(
        card: *mut snd_card,
        id: *const c_char,
        device: c_int,
        playback_count: c_int,
        capture_count: c_int,
        rpcm: *mut *mut snd_pcm,
    ) -> c_int;
    fn strscpy(dst: *mut c_char, src: *const c_char) -> isize;
    fn snd_pcm_set_ops(pcm: *mut snd_pcm, direction: c_int, ops: *const snd_pcm_ops);
    fn snd_pcm_set_managed_buffer_all(
        pcm: *mut snd_pcm,
        type_: c_int,
        data: *mut c_void,
        size: usize,
        max: usize,
    );
    fn snd_ctl_boolean_mono_info(
        kcontrol: *mut snd_kcontrol,
        uinfo: *mut snd_ctl_elem_info,
    ) -> c_int;
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_card_aica;
    fn snd_ctl_add(card: *mut snd_card, kcontrol: *mut snd_kcontrol) -> c_int;
    fn snd_ctl_new1(template: *const snd_kcontrol_new, private_data: *mut c_void) -> *mut snd_kcontrol;
    fn request_firmware(fw: *mut *const firmware, name: *const c_char, device: *mut device) -> c_int;
    fn platform_get_drvdata(dev: *mut platform_device) -> *mut snd_card_aica;
    fn platform_set_drvdata(dev: *mut platform_device, data: *mut snd_card_aica);
    fn snd_card_new(
        parent: *mut device,
        idx: c_int,
        xid: *const c_char,
        module: *mut c_void,
        extra_size: c_int,
        card_ret: *mut *mut snd_card,
    ) -> c_int;
    fn snd_card_free(card: *mut snd_card);
    fn INIT_WORK(work: *mut work_struct, func: unsafe extern "C" fn(*mut work_struct));
    fn timer_setup(
        timer: *mut timer_list,
        func: unsafe extern "C" fn(*mut timer_list),
        flags: c_uint,
    );
    fn snd_card_register(card: *mut snd_card) -> c_int;
    fn dev_info(dev: *mut device, fmt: *const c_char);
    fn platform_driver_register(driver: *mut platform_driver) -> c_int;
    fn platform_driver_unregister(driver: *mut platform_driver);
    fn platform_device_register_simple(
        name: *const c_char,
        id: c_int,
        res: *mut resource,
        num: c_uint,
    ) -> *mut platform_device;
    fn platform_device_unregister(pdev: *mut platform_device);
    fn IS_ERR(ptr: *const c_void) -> c_int;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
}

const ENOENT: c_int = 2;
const ENOMEM: c_int = 12;
const EINVAL: c_int = 22;
const ETXTBSY: c_int = 26;
const GFP_KERNEL: c_uint = 0;

unsafe fn unlikely(x: c_int) -> bool {
    x != 0
}

unsafe fn kmalloc_obj<T>() -> *mut T {
    kmalloc(core::mem::size_of::<T>(), GFP_KERNEL) as *mut T
}

unsafe fn kzalloc_obj<T>() -> *mut T {
    kzalloc(core::mem::size_of::<T>(), GFP_KERNEL) as *mut T
}

unsafe fn DIV_ROUND_UP(n: c_int, d: c_int) -> c_int {
    (n + d - 1) / d
}

static mut aica_memory_space: [resource; 2] = [
    resource {
        name: b"AICA ARM CONTROL\0".as_ptr() as *const c_char,
        start: 0,
        flags: 0,
        end: 0,
    },
    resource {
        name: b"AICA Sound RAM\0".as_ptr() as *const c_char,
        start: 0,
        flags: 0,
        end: 0,
    },
];

/* SPU specific functions */
/* spu_write_wait - wait for G2-SH FIFO to clear */
unsafe extern "C" fn spu_write_wait() {
    let mut time_count: c_int;
    time_count = 0;
    loop {
        if !(readl(G2_FIFO) & 0x11) != 0 {
            break;
        }
        /* To ensure hardware failure doesn't wedge kernel */
        time_count += 1;
        if time_count > 0x10000 {
            pr_warn(b"WARNING: G2 FIFO appears to be blocked.\n\0".as_ptr() as *const c_char);
            break;
        }
    }
}

/* spu_memset - write to memory in SPU address space */
unsafe extern "C" fn spu_memset(mut toi: u32, what: u32, length: c_int) {
    let mut i: c_int;
    let mut flags: c_ulong = 0;
    if snd_BUG_ON(length % 4) != 0 {
        return;
    }
    i = 0;
    while i < length {
        if !(i % 8) != 0 {
            spu_write_wait();
        }
        local_irq_save(&mut flags);
        writel(what, (toi as c_ulong + SPU_MEMORY_BASE) as *mut u32);
        local_irq_restore(flags);
        toi = toi.wrapping_add(1);
        i += 1;
    }
}

/* spu_memload - write to SPU address space */
unsafe extern "C" fn spu_memload(toi: u32, from: *const c_void, mut length: c_int) {
    let mut flags: c_ulong = 0;
    let mut froml: *const u32 = from as *const u32;
    let mut to: *mut u32 = (SPU_MEMORY_BASE + toi as c_ulong) as *mut u32;
    let mut i: c_int;
    let mut val: u32;
    length = DIV_ROUND_UP(length, 4);
    spu_write_wait();
    i = 0;
    while i < length {
        if !(i % 8) != 0 {
            spu_write_wait();
        }
        val = *froml;
        local_irq_save(&mut flags);
        writel(val, to);
        local_irq_restore(flags);
        froml = froml.add(1);
        to = to.add(1);
        i += 1;
    }
}

/* spu_disable - set spu registers to stop sound output */
unsafe extern "C" fn spu_disable() {
    let mut i: c_int;
    let mut flags: c_ulong = 0;
    let mut regval: u32;
    spu_write_wait();
    regval = readl(ARM_RESET_REGISTER);
    regval |= 1;
    spu_write_wait();
    local_irq_save(&mut flags);
    writel(regval, ARM_RESET_REGISTER as *mut u32);
    local_irq_restore(flags);
    i = 0;
    while i < 64 {
        spu_write_wait();
        regval = readl(SPU_REGISTER_BASE + (i * 0x80) as c_ulong);
        regval = (regval & !0x4000) | 0x8000;
        spu_write_wait();
        local_irq_save(&mut flags);
        writel(regval, (SPU_REGISTER_BASE + (i * 0x80) as c_ulong) as *mut u32);
        local_irq_restore(flags);
        i += 1;
    }
}

/* spu_enable - set spu registers to enable sound output */
unsafe extern "C" fn spu_enable() {
    let mut flags: c_ulong = 0;
    let mut regval: u32 = readl(ARM_RESET_REGISTER);
    regval &= !1;
    spu_write_wait();
    local_irq_save(&mut flags);
    writel(regval, ARM_RESET_REGISTER as *mut u32);
    local_irq_restore(flags);
}

/*
 * Halt the sound processor, clear the memory,
 * load some default ARM7 code, and then restart ARM7
*/
unsafe extern "C" fn spu_reset() {
    let mut flags: c_ulong = 0;
    spu_disable();
    spu_memset(0, 0, 0x200000 / 4);
    /* Put ARM7 in endless loop */
    local_irq_save(&mut flags);
    __raw_writel(0xea000002, SPU_MEMORY_BASE);
    local_irq_restore(flags);
    spu_enable();
}

/* aica_chn_start - write to spu to start playback */
unsafe extern "C" fn aica_chn_start() {
    let mut flags: c_ulong = 0;
    spu_write_wait();
    local_irq_save(&mut flags);
    writel(AICA_CMD_KICK | AICA_CMD_START, AICA_CONTROL_POINT as *mut u32);
    local_irq_restore(flags);
}

/* aica_chn_halt - write to spu to halt playback */
unsafe extern "C" fn aica_chn_halt() {
    let mut flags: c_ulong = 0;
    spu_write_wait();
    local_irq_save(&mut flags);
    writel(AICA_CMD_KICK | AICA_CMD_STOP, AICA_CONTROL_POINT as *mut u32);
    local_irq_restore(flags);
}

/* ALSA code below */
static snd_pcm_aica_playback_hw: snd_pcm_hardware = snd_pcm_hardware {
    info: 0,
    formats: 0,
    rates: 0,
    rate_min: 8000,
    rate_max: 48000,
    channels_min: 1,
    channels_max: 2,
    buffer_bytes_max: 0,
    period_bytes_min: 0,
    period_bytes_max: 0,
    periods_min: 0,
    periods_max: 0,
};

unsafe extern "C" fn aica_dma_transfer(
    channels: c_int,
    buffer_size: c_int,
    substream: *mut snd_pcm_substream,
) -> c_int {
    let mut q: c_int;
    let mut err: c_int;
    let mut period_offset: c_int;
    let mut dreamcastcard: *mut snd_card_aica;
    let mut runtime: *mut snd_pcm_runtime;
    let mut flags: c_ulong = 0;
    err = 0;
    dreamcastcard = (*(*substream).pcm).private_data as *mut snd_card_aica;
    period_offset = (*dreamcastcard).clicks;
    period_offset %= (AICA_PERIOD_NUMBER as c_int / channels);
    runtime = (*substream).runtime;
    q = 0;
    while q < channels {
        local_irq_save(&mut flags);
        err = dma_xfer(
            AICA_DMA_CHANNEL,
            (*runtime)
                .dma_area
                .add((AICA_BUFFER_SIZE * q as usize) / channels as usize)
                .add(AICA_PERIOD_SIZE * period_offset as usize) as c_ulong,
            AICA_CHANNEL0_OFFSET
                + (q as c_ulong * CHANNEL_OFFSET)
                + AICA_PERIOD_SIZE as c_ulong * period_offset as c_ulong,
            buffer_size / channels,
            AICA_DMA_MODE,
        );
        if unlikely((err < 0) as c_int) {
            local_irq_restore(flags);
            break;
        }
        dma_wait_for_completion(AICA_DMA_CHANNEL);
        local_irq_restore(flags);
        q += 1;
    }
    err
}

unsafe extern "C" fn startup_aica(dreamcastcard: *mut snd_card_aica) {
    spu_memload(
        AICA_CHANNEL0_CONTROL_OFFSET,
        (*dreamcastcard).channel as *const c_void,
        core::mem::size_of::<aica_channel>() as c_int,
    );
    aica_chn_start();
}

unsafe extern "C" fn run_spu_dma(work: *mut work_struct) {
    let mut buffer_size: c_int;
    let mut runtime: *mut snd_pcm_runtime;
    let dreamcastcard: *mut snd_card_aica = work as *mut snd_card_aica;
    runtime = (*(*dreamcastcard).substream).runtime;
    if unlikely(((*dreamcastcard).dma_check == 0) as c_int) {
        buffer_size = frames_to_bytes(runtime, (*runtime).buffer_size) as c_int;
        if (*runtime).channels > 1 {
            (*(*dreamcastcard).channel).flags |= 0x01;
        }
        aica_dma_transfer((*runtime).channels as c_int, buffer_size, (*dreamcastcard).substream);
        startup_aica(dreamcastcard);
        (*dreamcastcard).clicks =
            buffer_size / (AICA_PERIOD_SIZE as c_int * (*runtime).channels as c_int);
        return;
    } else {
        aica_dma_transfer(
            (*runtime).channels as c_int,
            AICA_PERIOD_SIZE as c_int * (*runtime).channels as c_int,
            (*dreamcastcard).substream,
        );
        snd_pcm_period_elapsed((*dreamcastcard).substream);
        (*dreamcastcard).clicks += 1;
        if unlikely(((*dreamcastcard).clicks >= AICA_PERIOD_NUMBER as c_int) as c_int) {
            (*dreamcastcard).clicks %= AICA_PERIOD_NUMBER as c_int;
        }
        if snd_pcm_running((*dreamcastcard).substream) != 0 {
            mod_timer(&mut (*dreamcastcard).timer, jiffies + 1);
        }
    }
}

unsafe extern "C" fn aica_period_elapsed(t: *mut timer_list) {
    let mut dreamcastcard: *mut snd_card_aica = t as *mut snd_card_aica;
    let substream: *mut snd_pcm_substream = (*dreamcastcard).substream;
    /*timer function - so cannot sleep */
    let mut play_period: c_int;
    let mut runtime: *mut snd_pcm_runtime;
    if snd_pcm_running(substream) == 0 {
        return;
    }
    runtime = (*substream).runtime;
    dreamcastcard = (*(*substream).pcm).private_data as *mut snd_card_aica;
    /* Have we played out an additional period? */
    play_period = (frames_to_bytes(runtime, readl(AICA_CONTROL_CHANNEL_SAMPLE_NUMBER) as c_ulong)
        / AICA_PERIOD_SIZE as c_ulong) as c_int;
    if play_period == (*dreamcastcard).current_period {
        /* reschedule the timer */
        mod_timer(&mut (*dreamcastcard).timer, jiffies + 1);
        return;
    }
    if (*runtime).channels > 1 {
        (*dreamcastcard).current_period = play_period;
    }
    if unlikely(((*dreamcastcard).dma_check == 0) as c_int) {
        (*dreamcastcard).dma_check = 1;
    }
    schedule_work(&mut (*dreamcastcard).spu_dma_work);
}

unsafe extern "C" fn spu_begin_dma(substream: *mut snd_pcm_substream) {
    let mut dreamcastcard: *mut snd_card_aica;
    dreamcastcard = (*(*substream).pcm).private_data as *mut snd_card_aica;
    /*get the queue to do the work */
    schedule_work(&mut (*dreamcastcard).spu_dma_work);
    mod_timer(&mut (*dreamcastcard).timer, jiffies + 4);
}

unsafe extern "C" fn snd_aicapcm_pcm_open(substream: *mut snd_pcm_substream) -> c_int {
    let mut runtime: *mut snd_pcm_runtime;
    let mut channel: *mut aica_channel;
    let mut dreamcastcard: *mut snd_card_aica;
    if !enable {
        return -ENOENT;
    }
    dreamcastcard = (*(*substream).pcm).private_data as *mut snd_card_aica;
    channel = kmalloc_obj::<aica_channel>();
    if channel.is_null() {
        return -ENOMEM;
    }
    /* set defaults for channel */
    (*channel).sfmt = SM_8BIT;
    (*channel).cmd = AICA_CMD_START;
    (*channel).vol = (*dreamcastcard).master_volume;
    (*channel).pan = 0x80;
    (*channel).pos = 0;
    (*channel).flags = 0; /* default to mono */
    (*dreamcastcard).channel = channel;
    runtime = (*substream).runtime;
    (*runtime).hw = snd_pcm_aica_playback_hw;
    spu_enable();
    (*dreamcastcard).clicks = 0;
    (*dreamcastcard).current_period = 0;
    (*dreamcastcard).dma_check = 0;
    0
}

unsafe extern "C" fn snd_aicapcm_pcm_sync_stop(substream: *mut snd_pcm_substream) -> c_int {
    let dreamcastcard: *mut snd_card_aica = (*(*substream).pcm).private_data as *mut snd_card_aica;

    timer_delete_sync(&mut (*dreamcastcard).timer);
    cancel_work_sync(&mut (*dreamcastcard).spu_dma_work);
    0
}

unsafe extern "C" fn snd_aicapcm_pcm_close(substream: *mut snd_pcm_substream) -> c_int {
    let dreamcastcard: *mut snd_card_aica = (*(*substream).pcm).private_data as *mut snd_card_aica;
    (*dreamcastcard).substream = ptr::null_mut();
    kfree((*dreamcastcard).channel as *mut c_void);
    spu_disable();
    0
}

unsafe extern "C" fn snd_aicapcm_pcm_prepare(substream: *mut snd_pcm_substream) -> c_int {
    let dreamcastcard: *mut snd_card_aica = (*(*substream).pcm).private_data as *mut snd_card_aica;
    if (*(*substream).runtime).format == SNDRV_PCM_FORMAT_S16_LE {
        (*(*dreamcastcard).channel).sfmt = SM_16BIT;
    }
    (*(*dreamcastcard).channel).freq = (*(*substream).runtime).rate;
    (*dreamcastcard).substream = substream;
    0
}

unsafe extern "C" fn snd_aicapcm_pcm_trigger(
    substream: *mut snd_pcm_substream,
    cmd: c_int,
) -> c_int {
    match cmd {
        x if x == SNDRV_PCM_TRIGGER_START => {
            spu_begin_dma(substream);
        }
        x if x == SNDRV_PCM_TRIGGER_STOP => {
            aica_chn_halt();
        }
        _ => {
            return -EINVAL;
        }
    }
    0
}

unsafe extern "C" fn snd_aicapcm_pcm_pointer(_substream: *mut snd_pcm_substream) -> c_ulong {
    readl(AICA_CONTROL_CHANNEL_SAMPLE_NUMBER) as c_ulong
}

static snd_aicapcm_playback_ops: snd_pcm_ops = snd_pcm_ops {
    open: Some(snd_aicapcm_pcm_open),
    close: Some(snd_aicapcm_pcm_close),
    prepare: Some(snd_aicapcm_pcm_prepare),
    trigger: Some(snd_aicapcm_pcm_trigger),
    pointer: Some(snd_aicapcm_pcm_pointer),
    sync_stop: Some(snd_aicapcm_pcm_sync_stop),
};

/* TO DO: set up to handle more than one pcm instance */
unsafe extern "C" fn snd_aicapcmchip(
    dreamcastcard: *mut snd_card_aica,
    pcm_index: c_int,
) -> c_int {
    let mut pcm: *mut snd_pcm = ptr::null_mut();
    let mut err: c_int;
    /* AICA has no capture ability */
    err = snd_pcm_new(
        (*dreamcastcard).card,
        b"AICA PCM\0".as_ptr() as *const c_char,
        pcm_index,
        1,
        0,
        &mut pcm,
    );
    if unlikely((err < 0) as c_int) {
        return err;
    }
    (*pcm).private_data = dreamcastcard as *mut c_void;
    strscpy((*pcm).name.as_mut_ptr(), b"AICA PCM\0".as_ptr() as *const c_char);
    snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_PLAYBACK, &snd_aicapcm_playback_ops);
    /* Allocate the DMA buffers */
    snd_pcm_set_managed_buffer_all(
        pcm,
        SNDRV_DMA_TYPE_CONTINUOUS,
        ptr::null_mut(),
        AICA_BUFFER_SIZE,
        AICA_BUFFER_SIZE,
    );
    0
}

/* Mixer controls */
const aica_pcmswitch_info: Option<
    unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_info) -> c_int,
> = Some(snd_ctl_boolean_mono_info);

unsafe extern "C" fn aica_pcmswitch_get(
    _kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    (*(*ucontrol).value.integer).value[0] = 1; /* TO DO: Fix me */
    0
}

unsafe extern "C" fn aica_pcmswitch_put(
    _kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    if (*(*ucontrol).value.integer).value[0] == 1 {
        return 0; /* TO DO: Fix me */
    } else {
        aica_chn_halt();
    }
    0
}

unsafe extern "C" fn aica_pcmvolume_info(
    _kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
    (*uinfo).count = 1;
    (*(*uinfo).value.integer).min = 0;
    (*(*uinfo).value.integer).max = 0xFF;
    0
}

unsafe extern "C" fn aica_pcmvolume_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let dreamcastcard: *mut snd_card_aica = snd_kcontrol_chip(kcontrol);

    if unlikely((*dreamcastcard).channel.is_null() as c_int) {
        return -ETXTBSY; /* we've not yet been set up */
    }
    (*(*ucontrol).value.integer).value[0] = (*(*dreamcastcard).channel).vol as c_long;
    0
}

unsafe extern "C" fn aica_pcmvolume_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let dreamcastcard: *mut snd_card_aica = snd_kcontrol_chip(kcontrol);
    let mut vol: c_uint;

    if unlikely((*dreamcastcard).channel.is_null() as c_int) {
        return -ETXTBSY;
    }
    vol = (*(*ucontrol).value.integer).value[0] as c_uint;
    if vol > 0xff {
        return -EINVAL;
    }
    if unlikely(((*(*dreamcastcard).channel).vol == vol) as c_int) {
        return 0;
    }
    (*(*dreamcastcard).channel).vol = (*(*ucontrol).value.integer).value[0] as u32;
    (*dreamcastcard).master_volume = (*(*ucontrol).value.integer).value[0] as u32;
    spu_memload(
        AICA_CHANNEL0_CONTROL_OFFSET,
        (*dreamcastcard).channel as *const c_void,
        core::mem::size_of::<aica_channel>() as c_int,
    );
    1
}

static snd_aica_pcmswitch_control: snd_kcontrol_new = snd_kcontrol_new {
    iface: 0,
    name: b"PCM Playback Switch\0".as_ptr() as *const c_char,
    index: 0,
    info: aica_pcmswitch_info,
    get: Some(aica_pcmswitch_get),
    put: Some(aica_pcmswitch_put),
};

static snd_aica_pcmvolume_control: snd_kcontrol_new = snd_kcontrol_new {
    iface: 0,
    name: b"PCM Playback Volume\0".as_ptr() as *const c_char,
    index: 0,
    info: Some(aica_pcmvolume_info),
    get: Some(aica_pcmvolume_get),
    put: Some(aica_pcmvolume_put),
};

unsafe extern "C" fn load_aica_firmware() -> c_int {
    let mut err: c_int;
    spu_reset();

    let mut fw_entry: *const firmware = ptr::null();
    err = request_firmware(
        &mut fw_entry,
        b"aica_firmware.bin\0".as_ptr() as *const c_char,
        &mut (*pd).dev,
    );
    if unlikely(err) {
        return err;
    }
    /* write firmware into memory */
    spu_disable();
    spu_memload(0, (*fw_entry).data as *const c_void, (*fw_entry).size as c_int);
    spu_enable();
    err
}

unsafe extern "C" fn add_aicamixer_controls(dreamcastcard: *mut snd_card_aica) -> c_int {
    let mut err: c_int;
    err = snd_ctl_add(
        (*dreamcastcard).card,
        snd_ctl_new1(
            &snd_aica_pcmvolume_control,
            dreamcastcard as *mut c_void,
        ),
    );
    if unlikely((err < 0) as c_int) {
        return err;
    }
    err = snd_ctl_add(
        (*dreamcastcard).card,
        snd_ctl_new1(
            &snd_aica_pcmswitch_control,
            dreamcastcard as *mut c_void,
        ),
    );
    if unlikely((err < 0) as c_int) {
        return err;
    }
    0
}

unsafe extern "C" fn snd_aica_remove(devptr: *mut platform_device) {
    let mut dreamcastcard: *mut snd_card_aica;
    dreamcastcard = platform_get_drvdata(devptr);
    snd_card_free((*dreamcastcard).card);
    kfree(dreamcastcard as *mut c_void);
}

unsafe extern "C" fn snd_aica_probe(devptr: *mut platform_device) -> c_int {
    let mut err: c_int;
    let mut dreamcastcard: *mut snd_card_aica;
    dreamcastcard = kzalloc_obj::<snd_card_aica>();
    if unlikely(dreamcastcard.is_null() as c_int) {
        return -ENOMEM;
    }
    err = snd_card_new(
        &mut (*devptr).dev,
        index,
        SND_AICA_DRIVER,
        THIS_MODULE,
        0,
        &mut (*dreamcastcard).card,
    );
    if unlikely((err < 0) as c_int) {
        kfree(dreamcastcard as *mut c_void);
        return err;
    }

    strscpy((*(*dreamcastcard).card).driver.as_mut_ptr(), b"snd_aica\0".as_ptr() as *const c_char);
    strscpy((*(*dreamcastcard).card).shortname.as_mut_ptr(), SND_AICA_DRIVER);
    strscpy(
        (*(*dreamcastcard).card).longname.as_mut_ptr(),
        b"Yamaha AICA Super Intelligent Sound Processor for SEGA Dreamcast\0".as_ptr()
            as *const c_char,
    );
    /* Prepare to use the queue */
    INIT_WORK(&mut (*dreamcastcard).spu_dma_work, run_spu_dma);
    timer_setup(&mut (*dreamcastcard).timer, aica_period_elapsed, 0);
    /* Load the PCM 'chip' */
    err = snd_aicapcmchip(dreamcastcard, 0);
    if unlikely((err < 0) as c_int) {
        snd_card_free((*dreamcastcard).card);
        kfree(dreamcastcard as *mut c_void);
        return err;
    }
    /* Add basic controls */
    err = add_aicamixer_controls(dreamcastcard);
    if unlikely((err < 0) as c_int) {
        snd_card_free((*dreamcastcard).card);
        kfree(dreamcastcard as *mut c_void);
        return err;
    }
    /* Register the card with ALSA subsystem */
    err = snd_card_register((*dreamcastcard).card);
    if unlikely((err < 0) as c_int) {
        snd_card_free((*dreamcastcard).card);
        kfree(dreamcastcard as *mut c_void);
        return err;
    }
    platform_set_drvdata(devptr, dreamcastcard);
    dev_info(
        &mut (*devptr).dev,
        b"ALSA Driver for Yamaha AICA Super Intelligent Sound Processor\n\0".as_ptr()
            as *const c_char,
    );
    0
}

static mut snd_aica_driver: platform_driver = platform_driver {
    probe: Some(snd_aica_probe),
    remove: Some(snd_aica_remove),
    driver: platform_driver__driver { name: ptr::null() },
};

unsafe extern "C" fn aica_init() -> c_int {
    let mut err: c_int;
    aica_memory_space[0].start = ARM_RESET_REGISTER;
    aica_memory_space[0].flags = IORESOURCE_MEM;
    aica_memory_space[0].end = ARM_RESET_REGISTER + 3;
    aica_memory_space[1].start = SPU_MEMORY_BASE;
    aica_memory_space[1].flags = IORESOURCE_MEM;
    aica_memory_space[1].end = SPU_MEMORY_BASE + 0x200000 - 1;
    snd_aica_driver.driver.name = SND_AICA_DRIVER;

    err = platform_driver_register(&mut snd_aica_driver);
    if unlikely((err < 0) as c_int) {
        return err;
    }
    pd = platform_device_register_simple(SND_AICA_DRIVER, -1, aica_memory_space.as_mut_ptr(), 2);
    if IS_ERR(pd as *const c_void) != 0 {
        platform_driver_unregister(&mut snd_aica_driver);
        return PTR_ERR(pd as *const c_void);
    }
    /* Load the firmware */
    load_aica_firmware()
}

unsafe extern "C" fn aica_exit() {
    platform_device_unregister(pd);
    platform_driver_unregister(&mut snd_aica_driver);
    /* Kill any sound still playing and reset ARM7 to safe state */
    spu_reset();
}

// module_init(aica_init);
// module_exit(aica_exit);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
