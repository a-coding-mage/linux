// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *   Sound driver for Silicon Graphics O2 Workstations A/V board audio.
 *
 *   Copyright 2003 Vivien Chappelier <vivien.chappelier@linux-mips.org>
 *   Copyright 2008 Thomas Bogendoerfer <tsbogend@alpha.franken.de>
 *   Mxier part taken from mace_audio.c:
 *   Copyright 2007 Thorben Jändling <tj.trevelyan@gmail.com>
 */

// Dependencies from Linux, MIPS IP32, and ALSA headers are declared here as
// external symbols/types; include directives and module macros are C-only.

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

type u64_t = u64;
type s64_t = i64;
type s16_t = i16;
type dma_addr_t = u64;
type snd_pcm_uframes_t = c_ulong;
type irqreturn_t = c_int;
type gfp_t = c_uint;

const fn BIT(n: u32) -> u64 {
    1u64 << n
}

extern "C" {
    static mut index: c_int;
    static mut id: *mut c_char;

    static mut mace: *mut Mace;

    static SNDRV_DEFAULT_IDX1: c_int;
    static mut SNDRV_DEFAULT_STR1: *mut c_char;

    static SNDRV_CTL_ELEM_TYPE_INTEGER: c_int;
    static SNDRV_CTL_ELEM_IFACE_MIXER: c_int;
    static SNDRV_CTL_ELEM_ACCESS_READWRITE: c_uint;
    static AD1843_GAIN_PCM_0: c_ulong;
    static AD1843_GAIN_PCM_1: c_ulong;
    static AD1843_GAIN_RECLEV: c_ulong;
    static AD1843_GAIN_LINE: c_ulong;
    static AD1843_GAIN_LINE_2: c_ulong;
    static AD1843_GAIN_MIC: c_ulong;
    static SNDRV_PCM_STREAM_PLAYBACK: c_int;
    static SNDRV_PCM_STREAM_CAPTURE: c_int;
    static SNDRV_PCM_FORMAT_S16_LE: c_int;
    static SNDRV_PCM_TRIGGER_START: c_int;
    static SNDRV_PCM_TRIGGER_STOP: c_int;
    static SNDRV_PCM_INFO_MMAP: c_uint;
    static SNDRV_PCM_INFO_MMAP_VALID: c_uint;
    static SNDRV_PCM_INFO_INTERLEAVED: c_uint;
    static SNDRV_PCM_INFO_BLOCK_TRANSFER: c_uint;
    static SNDRV_PCM_FMTBIT_S16_BE: u64;
    static SNDRV_PCM_RATE_8000_48000: c_uint;
    static SNDRV_DMA_TYPE_VMALLOC: c_int;
    static SNDRV_DEV_LOWLEVEL: c_int;
    static THIS_MODULE: *mut c_void;
    static GFP_KERNEL: gfp_t;
    static IRQ_HANDLED: irqreturn_t;
    static MACEISA_AUDIO1_DMAT_IRQ: c_int;
    static MACEISA_AUDIO1_OF_IRQ: c_int;
    static MACEISA_AUDIO2_DMAT_IRQ: c_int;
    static MACEISA_AUDIO2_MERR_IRQ: c_int;
    static MACEISA_AUDIO3_DMAT_IRQ: c_int;
    static MACEISA_AUDIO3_MERR_IRQ: c_int;
    static MACEISA_RINGBUFFERS_SIZE: usize;
    static EINVAL: c_int;
    static ENOENT: c_int;
    static ENOMEM: c_int;
    static EBUSY: c_int;
    static KERN_ERR: *const c_char;

    fn writeq(val: u64, addr: *mut u64);
    fn readq(addr: *const u64) -> u64;
    fn wmb();
    fn udelay(usecs: c_uint);
    fn msleep_interruptible(msecs: c_uint);
    fn printk(fmt: *const c_char, ...) -> c_int;
    fn request_irq(
        irq: c_int,
        handler: Option<unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t>,
        flags: c_ulong,
        name: *const c_char,
        dev: *mut c_void,
    ) -> c_int;
    fn free_irq(irq: c_int, dev: *mut c_void);
    fn dma_alloc_coherent(
        dev: *mut device,
        size: usize,
        dma_handle: *mut dma_addr_t,
        flag: gfp_t,
    ) -> *mut c_void;
    fn dma_free_coherent(dev: *mut device, size: usize, cpu_addr: *mut c_void, dma_handle: dma_addr_t);
    fn kzalloc(size: usize, flags: gfp_t) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn spin_lock_init(lock: *mut spinlock_t);
    fn spin_lock_irqsave(lock: *mut spinlock_t, flags: *mut c_ulong);
    fn spin_unlock_irqrestore(lock: *mut spinlock_t, flags: c_ulong);

    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut c_void;
    fn snd_ctl_enum_info(
        uinfo: *mut snd_ctl_elem_info,
        channels: c_uint,
        items: c_uint,
        texts: *const *const c_char,
    ) -> c_int;
    fn snd_ctl_add(card: *mut snd_card, kcontrol: *mut snd_kcontrol) -> c_int;
    fn snd_ctl_new1(ncontrol: *const snd_kcontrol_new, private_data: *mut c_void) -> *mut snd_kcontrol;
    fn snd_pcm_substream_chip(substream: *mut snd_pcm_substream) -> *mut c_void;
    fn frames_to_bytes(runtime: *mut snd_pcm_runtime, size: snd_pcm_uframes_t) -> c_ulong;
    fn bytes_to_frames(runtime: *mut snd_pcm_runtime, size: c_int) -> snd_pcm_uframes_t;
    fn snd_pcm_period_elapsed(substream: *mut snd_pcm_substream);
    fn snd_pcm_new(
        card: *mut snd_card,
        id: *const c_char,
        device: c_int,
        playback_count: c_int,
        capture_count: c_int,
        rpcm: *mut *mut snd_pcm,
    ) -> c_int;
    fn snd_pcm_set_ops(pcm: *mut snd_pcm, direction: c_int, ops: *const snd_pcm_ops);
    fn snd_pcm_set_managed_buffer_all(
        pcm: *mut snd_pcm,
        ty: c_int,
        data: *mut c_void,
        min: usize,
        max: usize,
    );
    fn snd_card_new(
        parent: *mut device,
        idx: c_int,
        xid: *mut c_char,
        module: *mut c_void,
        extra_size: c_int,
        card_ret: *mut *mut snd_card,
    ) -> c_int;
    fn snd_card_free(card: *mut snd_card);
    fn snd_card_register(card: *mut snd_card) -> c_int;
    fn snd_device_new(card: *mut snd_card, ty: c_int, device_data: *mut c_void, ops: *const snd_device_ops) -> c_int;
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut c_void);
    fn platform_get_drvdata(pdev: *mut platform_device) -> *mut c_void;
    fn strscpy(dst: *mut c_char, src: *const c_char) -> isize;
    fn sprintf(buf: *mut c_char, fmt: *const c_char, ...) -> c_int;

    fn ad1843_get_gain(ad1843: *mut snd_ad1843, reg: c_int) -> c_int;
    fn ad1843_get_gain_max(ad1843: *mut snd_ad1843, reg: c_int) -> c_int;
    fn ad1843_set_gain(ad1843: *mut snd_ad1843, reg: c_ulong, gain: c_int) -> c_int;
    fn ad1843_get_recsrc(ad1843: *mut snd_ad1843) -> c_int;
    fn ad1843_set_recsrc(ad1843: *mut snd_ad1843, src: c_ulong) -> c_int;
    fn ad1843_setup_dac(ad1843: *mut snd_ad1843, dac: c_int, rate: c_uint, format: c_int, channels: c_uint);
    fn ad1843_setup_adc(ad1843: *mut snd_ad1843, rate: c_uint, format: c_int, channels: c_uint);
    fn ad1843_init(ad1843: *mut snd_ad1843) -> c_int;
}

const AUDIO_CONTROL_RESET: u64 = BIT(0); /* 1: reset audio interface */
const AUDIO_CONTROL_CODEC_PRESENT: u64 = BIT(1); /* 1: codec detected */

const CODEC_CONTROL_WORD_SHIFT: u32 = 0;
const CODEC_CONTROL_READ: u64 = BIT(16);
const CODEC_CONTROL_ADDRESS_SHIFT: u32 = 17;

const CHANNEL_CONTROL_RESET: u64 = BIT(10); /* 1: reset channel */
const CHANNEL_DMA_ENABLE: u64 = BIT(9); /* 1: enable DMA transfer */
const CHANNEL_INT_THRESHOLD_DISABLED: u64 = 0 << 5; /* interrupt disabled */
const CHANNEL_INT_THRESHOLD_25: u64 = 1 << 5; /* int on buffer >25% full */
const CHANNEL_INT_THRESHOLD_50: u64 = 2 << 5; /* int on buffer >50% full */
const CHANNEL_INT_THRESHOLD_75: u64 = 3 << 5; /* int on buffer >75% full */
const CHANNEL_INT_THRESHOLD_EMPTY: u64 = 4 << 5; /* int on buffer empty */
const CHANNEL_INT_THRESHOLD_NOT_EMPTY: u64 = 5 << 5; /* int on buffer !empty */
const CHANNEL_INT_THRESHOLD_FULL: u64 = 6 << 5; /* int on buffer empty */
const CHANNEL_INT_THRESHOLD_NOT_FULL: u64 = 7 << 5; /* int on buffer !empty */

const CHANNEL_RING_SHIFT: u32 = 12;
const CHANNEL_RING_SIZE: c_uint = 1 << CHANNEL_RING_SHIFT;
const CHANNEL_RING_MASK: c_ulong = (CHANNEL_RING_SIZE - 1) as c_ulong;

const CHANNEL_LEFT_SHIFT: u32 = 40;
const CHANNEL_RIGHT_SHIFT: u32 = 8;

#[repr(C)]
pub struct snd_sgio2audio_chan {
    idx: c_int,
    substream: *mut snd_pcm_substream,
    pos: c_int,
    size: snd_pcm_uframes_t,
    lock: spinlock_t,
}

/* definition of the chip-specific record */
#[repr(C)]
pub struct snd_sgio2audio {
    card: *mut snd_card,

    /* codec */
    ad1843: snd_ad1843,
    ad1843_lock: spinlock_t,

    /* channels */
    channel: [snd_sgio2audio_chan; 3],

    /* resources */
    ring_base: *mut c_void,
    ring_base_dma: dma_addr_t,
}

#[repr(C)]
pub struct spinlock_t {
    _private: [u8; 0],
}
#[repr(C)]
pub struct device {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_card {
    dev: *mut device,
    driver: [c_char; 16],
    shortname: [c_char; 32],
    longname: [c_char; 80],
}
#[repr(C)]
pub struct snd_ad1843 {
    read: Option<unsafe extern "C" fn(*mut c_void, c_int) -> c_int>,
    write: Option<unsafe extern "C" fn(*mut c_void, c_int, c_int) -> c_int>,
    chip: *mut c_void,
}
#[repr(C)]
pub struct snd_kcontrol {
    private_value: c_ulong,
}
#[repr(C)]
pub struct snd_ctl_elem_info {
    type_: c_int,
    count: c_uint,
    value: snd_ctl_elem_info_value,
}
#[repr(C)]
pub union snd_ctl_elem_info_value {
    integer: snd_ctl_elem_info_integer,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_elem_info_integer {
    min: i64,
    max: i64,
}
#[repr(C)]
pub struct snd_ctl_elem_value {
    value: snd_ctl_elem_value_value,
}
#[repr(C)]
pub union snd_ctl_elem_value_value {
    integer: snd_ctl_elem_value_integer,
    enumerated: snd_ctl_elem_value_enumerated,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_elem_value_integer {
    value: [i64; 128],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_elem_value_enumerated {
    item: [c_uint; 128],
}
#[repr(C)]
pub struct snd_kcontrol_new {
    iface: c_int,
    name: *const c_char,
    index: c_uint,
    access: c_uint,
    private_value: c_ulong,
    info: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_info) -> c_int>,
    get: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    put: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
}
#[repr(C)]
pub struct snd_pcm_runtime {
    hw: snd_pcm_hardware,
    private_data: *mut c_void,
    dma_area: *mut u8,
    buffer_size: snd_pcm_uframes_t,
    period_size: snd_pcm_uframes_t,
    rate: c_uint,
    channels: c_uint,
}
#[repr(C)]
pub struct snd_pcm_substream {
    runtime: *mut snd_pcm_runtime,
    stream: c_int,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_pcm_hardware {
    info: c_uint,
    formats: u64,
    rates: c_uint,
    rate_min: c_uint,
    rate_max: c_uint,
    channels_min: c_uint,
    channels_max: c_uint,
    buffer_bytes_max: usize,
    period_bytes_min: usize,
    period_bytes_max: usize,
    periods_min: c_uint,
    periods_max: c_uint,
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
pub struct snd_pcm {
    private_data: *mut c_void,
    name: [c_char; 80],
}
#[repr(C)]
pub struct snd_device {
    device_data: *mut c_void,
}
#[repr(C)]
pub struct snd_device_ops {
    dev_free: Option<unsafe extern "C" fn(*mut snd_device) -> c_int>,
}
#[repr(C)]
pub struct platform_device {
    dev: device,
}
#[repr(C)]
pub struct platform_driver_driver {
    name: *const c_char,
}
#[repr(C)]
pub struct platform_driver {
    probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    remove: Option<unsafe extern "C" fn(*mut platform_device)>,
    driver: platform_driver_driver,
}
#[repr(C)]
pub struct Mace {
    perif: MacePerif,
}
#[repr(C)]
pub struct MacePerif {
    audio: MaceAudio,
    ctrl: MaceCtrl,
}
#[repr(C)]
pub struct MaceAudio {
    control: u64,
    codec_control: u64,
    codec_read: u64,
    chan: [MaceAudioChan; 3],
}
#[repr(C)]
pub struct MaceAudioChan {
    control: u64,
    read_ptr: u64,
    write_ptr: u64,
    depth: u64,
}
#[repr(C)]
pub struct MaceCtrl {
    ringbase: u64,
}

unsafe fn with_spinlock_irqsave<T>(lock: *mut spinlock_t, f: impl FnOnce() -> T) -> T {
    let mut flags: c_ulong = 0;
    spin_lock_irqsave(lock, &mut flags);
    let ret = f();
    spin_unlock_irqrestore(lock, flags);
    ret
}

/* AD1843 access */

/*
 * read_ad1843_reg returns the current contents of a 16 bit AD1843 register.
 *
 * Returns unsigned register value on success, -errno on failure.
 */
unsafe extern "C" fn read_ad1843_reg(priv_: *mut c_void, reg: c_int) -> c_int {
    let chip = priv_ as *mut snd_sgio2audio;
    let mut val: c_int;

    with_spinlock_irqsave(&mut (*chip).ad1843_lock, || {
        writeq(
            ((reg as u64) << CODEC_CONTROL_ADDRESS_SHIFT) | CODEC_CONTROL_READ,
            &mut (*mace).perif.audio.codec_control,
        );
        wmb();
        val = readq(&(*mace).perif.audio.codec_control) as c_int; /* flush bus */
        udelay(200);

        val = readq(&(*mace).perif.audio.codec_read) as c_int;
    });

    val
}

/*
 * write_ad1843_reg writes the specified value to a 16 bit AD1843 register.
 */
unsafe extern "C" fn write_ad1843_reg(priv_: *mut c_void, reg: c_int, word: c_int) -> c_int {
    let chip = priv_ as *mut snd_sgio2audio;
    let mut val: c_int;

    with_spinlock_irqsave(&mut (*chip).ad1843_lock, || {
        writeq(
            ((reg as u64) << CODEC_CONTROL_ADDRESS_SHIFT)
                | ((word as u64) << CODEC_CONTROL_WORD_SHIFT),
            &mut (*mace).perif.audio.codec_control,
        );
        wmb();
        val = readq(&(*mace).perif.audio.codec_control) as c_int; /* flush bus */
        udelay(200);
    });

    0
}

unsafe extern "C" fn sgio2audio_gain_info(
    kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    let chip = snd_kcontrol_chip(kcontrol) as *mut snd_sgio2audio;

    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
    (*uinfo).count = 2;
    (*uinfo).value.integer.min = 0;
    (*uinfo).value.integer.max =
        ad1843_get_gain_max(&mut (*chip).ad1843, (*kcontrol).private_value as c_int) as i64;
    0
}

unsafe extern "C" fn sgio2audio_gain_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let chip = snd_kcontrol_chip(kcontrol) as *mut snd_sgio2audio;
    let vol: c_int;

    vol = ad1843_get_gain(&mut (*chip).ad1843, (*kcontrol).private_value as c_int);

    (*ucontrol).value.integer.value[0] = ((vol >> 8) & 0xFF) as i64;
    (*ucontrol).value.integer.value[1] = (vol & 0xFF) as i64;

    0
}

unsafe extern "C" fn sgio2audio_gain_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let chip = snd_kcontrol_chip(kcontrol) as *mut snd_sgio2audio;
    let newvol: c_int;
    let oldvol: c_int;

    oldvol = ad1843_get_gain(&mut (*chip).ad1843, (*kcontrol).private_value as c_int);
    newvol = (((*ucontrol).value.integer.value[0] << 8) | (*ucontrol).value.integer.value[1]) as c_int;

    let newvol = ad1843_set_gain(&mut (*chip).ad1843, (*kcontrol).private_value, newvol);

    (newvol != oldvol) as c_int
}

unsafe extern "C" fn sgio2audio_source_info(
    _kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    static TEXT0: &[u8] = b"Cam Mic\0";
    static TEXT1: &[u8] = b"Mic\0";
    static TEXT2: &[u8] = b"Line\0";
    let texts: [*const c_char; 3] = [
        TEXT0.as_ptr() as *const c_char,
        TEXT1.as_ptr() as *const c_char,
        TEXT2.as_ptr() as *const c_char,
    ];
    snd_ctl_enum_info(uinfo, 1, 3, texts.as_ptr())
}

unsafe extern "C" fn sgio2audio_source_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let chip = snd_kcontrol_chip(kcontrol) as *mut snd_sgio2audio;

    (*ucontrol).value.enumerated.item[0] = ad1843_get_recsrc(&mut (*chip).ad1843) as c_uint;
    0
}

unsafe extern "C" fn sgio2audio_source_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let chip = snd_kcontrol_chip(kcontrol) as *mut snd_sgio2audio;
    let newsrc: c_int;
    let oldsrc: c_int;

    oldsrc = ad1843_get_recsrc(&mut (*chip).ad1843);
    newsrc = ad1843_set_recsrc(&mut (*chip).ad1843, (*ucontrol).value.enumerated.item[0] as c_ulong);

    (newsrc != oldsrc) as c_int
}

/* dac1/pcm0 mixer control */
static SGIO2AUDIO_CTRL_PCM0: snd_kcontrol_new = snd_kcontrol_new {
    iface: 0,
    name: b"PCM Playback Volume\0".as_ptr() as *const c_char,
    index: 0,
    access: 0,
    private_value: 0,
    info: Some(sgio2audio_gain_info),
    get: Some(sgio2audio_gain_get),
    put: Some(sgio2audio_gain_put),
};

/* dac2/pcm1 mixer control */
static SGIO2AUDIO_CTRL_PCM1: snd_kcontrol_new = snd_kcontrol_new {
    iface: 0,
    name: b"PCM Playback Volume\0".as_ptr() as *const c_char,
    index: 1,
    access: 0,
    private_value: 0,
    info: Some(sgio2audio_gain_info),
    get: Some(sgio2audio_gain_get),
    put: Some(sgio2audio_gain_put),
};

/* record level mixer control */
static SGIO2AUDIO_CTRL_RECLEVEL: snd_kcontrol_new = snd_kcontrol_new {
    iface: 0,
    name: b"Capture Volume\0".as_ptr() as *const c_char,
    index: 0,
    access: 0,
    private_value: 0,
    info: Some(sgio2audio_gain_info),
    get: Some(sgio2audio_gain_get),
    put: Some(sgio2audio_gain_put),
};

/* record level source control */
static SGIO2AUDIO_CTRL_RECSOURCE: snd_kcontrol_new = snd_kcontrol_new {
    iface: 0,
    name: b"Capture Source\0".as_ptr() as *const c_char,
    index: 0,
    access: 0,
    private_value: 0,
    info: Some(sgio2audio_source_info),
    get: Some(sgio2audio_source_get),
    put: Some(sgio2audio_source_put),
};

/* line mixer control */
static SGIO2AUDIO_CTRL_LINE: snd_kcontrol_new = snd_kcontrol_new {
    iface: 0,
    name: b"Line Playback Volume\0".as_ptr() as *const c_char,
    index: 0,
    access: 0,
    private_value: 0,
    info: Some(sgio2audio_gain_info),
    get: Some(sgio2audio_gain_get),
    put: Some(sgio2audio_gain_put),
};

/* cd mixer control */
static SGIO2AUDIO_CTRL_CD: snd_kcontrol_new = snd_kcontrol_new {
    iface: 0,
    name: b"Line Playback Volume\0".as_ptr() as *const c_char,
    index: 1,
    access: 0,
    private_value: 0,
    info: Some(sgio2audio_gain_info),
    get: Some(sgio2audio_gain_get),
    put: Some(sgio2audio_gain_put),
};

/* mic mixer control */
static SGIO2AUDIO_CTRL_MIC: snd_kcontrol_new = snd_kcontrol_new {
    iface: 0,
    name: b"Mic Playback Volume\0".as_ptr() as *const c_char,
    index: 0,
    access: 0,
    private_value: 0,
    info: Some(sgio2audio_gain_info),
    get: Some(sgio2audio_gain_get),
    put: Some(sgio2audio_gain_put),
};

unsafe extern "C" fn snd_sgio2audio_new_mixer(chip: *mut snd_sgio2audio) -> c_int {
    let mut err: c_int;

    err = snd_ctl_add((*chip).card, snd_ctl_new1(&SGIO2AUDIO_CTRL_PCM0, chip as *mut c_void));
    if err < 0 {
        return err;
    }
    err = snd_ctl_add((*chip).card, snd_ctl_new1(&SGIO2AUDIO_CTRL_PCM1, chip as *mut c_void));
    if err < 0 {
        return err;
    }
    err = snd_ctl_add((*chip).card, snd_ctl_new1(&SGIO2AUDIO_CTRL_RECLEVEL, chip as *mut c_void));
    if err < 0 {
        return err;
    }
    err = snd_ctl_add((*chip).card, snd_ctl_new1(&SGIO2AUDIO_CTRL_RECSOURCE, chip as *mut c_void));
    if err < 0 {
        return err;
    }
    err = snd_ctl_add((*chip).card, snd_ctl_new1(&SGIO2AUDIO_CTRL_LINE, chip as *mut c_void));
    if err < 0 {
        return err;
    }
    err = snd_ctl_add((*chip).card, snd_ctl_new1(&SGIO2AUDIO_CTRL_CD, chip as *mut c_void));
    if err < 0 {
        return err;
    }
    err = snd_ctl_add((*chip).card, snd_ctl_new1(&SGIO2AUDIO_CTRL_MIC, chip as *mut c_void));
    if err < 0 {
        return err;
    }

    0
}

/* low-level audio interface DMA */

/* get data out of bounce buffer, count must be a multiple of 32 */
/* returns 1 if a period has elapsed */
unsafe extern "C" fn snd_sgio2audio_dma_pull_frag(
    chip: *mut snd_sgio2audio,
    ch: c_uint,
    mut count: c_uint,
) -> c_int {
    let ch_usize = ch as usize;
    let runtime = (*(*chip).channel[ch_usize].substream).runtime;
    let mut ret: c_int = 0;

    with_spinlock_irqsave(&mut (*chip).channel[ch_usize].lock, || {
        let src_base = ((*chip).ring_base as c_ulong) | ((ch as c_ulong) << CHANNEL_RING_SHIFT);
        let mut src_pos = readq(&(*mace).perif.audio.chan[ch_usize].read_ptr) as c_ulong;
        let dst_base = (*runtime).dma_area;
        let mut dst_pos = (*chip).channel[ch_usize].pos;
        let dst_mask = frames_to_bytes(runtime, (*runtime).buffer_size) - 1;

        /* check if a period has elapsed */
        (*chip).channel[ch_usize].size += (count >> 3) as snd_pcm_uframes_t; /* in frames */
        ret = ((*chip).channel[ch_usize].size >= (*runtime).period_size) as c_int;
        (*chip).channel[ch_usize].size %= (*runtime).period_size;

        while count != 0 {
            let src = (src_base + src_pos) as *mut u64_t;
            let dst = dst_base.add(dst_pos as usize) as *mut s16_t;

            let x = ptr::read(src);
            ptr::write(dst.add(0), ((x >> CHANNEL_LEFT_SHIFT) & 0xffff) as s16_t);
            ptr::write(dst.add(1), ((x >> CHANNEL_RIGHT_SHIFT) & 0xffff) as s16_t);

            src_pos = (src_pos + size_of::<u64_t>() as c_ulong) & CHANNEL_RING_MASK;
            dst_pos = ((dst_pos as c_ulong + (2 * size_of::<s16_t>()) as c_ulong) & dst_mask) as c_int;
            count -= size_of::<u64_t>() as c_uint;
        }

        writeq(src_pos as u64, &mut (*mace).perif.audio.chan[ch_usize].read_ptr); /* in bytes */
        (*chip).channel[ch_usize].pos = dst_pos;
    });

    ret
}

/* put some DMA data in bounce buffer, count must be a multiple of 32 */
/* returns 1 if a period has elapsed */
unsafe extern "C" fn snd_sgio2audio_dma_push_frag(
    chip: *mut snd_sgio2audio,
    ch: c_uint,
    mut count: c_uint,
) -> c_int {
    let ch_usize = ch as usize;
    let runtime = (*(*chip).channel[ch_usize].substream).runtime;
    let mut ret: c_int = 0;

    with_spinlock_irqsave(&mut (*chip).channel[ch_usize].lock, || {
        let dst_base = ((*chip).ring_base as c_ulong) | ((ch as c_ulong) << CHANNEL_RING_SHIFT);
        let mut dst_pos = readq(&(*mace).perif.audio.chan[ch_usize].write_ptr) as c_ulong;
        let src_base = (*runtime).dma_area;
        let mut src_pos = (*chip).channel[ch_usize].pos;
        let src_mask = frames_to_bytes(runtime, (*runtime).buffer_size) - 1;

        /* check if a period has elapsed */
        (*chip).channel[ch_usize].size += (count >> 3) as snd_pcm_uframes_t; /* in frames */
        ret = ((*chip).channel[ch_usize].size >= (*runtime).period_size) as c_int;
        (*chip).channel[ch_usize].size %= (*runtime).period_size;

        while count != 0 {
            let src = src_base.add(src_pos as usize) as *mut s16_t;
            let dst = (dst_base + dst_pos) as *mut u64_t;

            let l: s64_t = ptr::read(src.add(0)) as s64_t; /* sign extend */
            let r: s64_t = ptr::read(src.add(1)) as s64_t; /* sign extend */

            ptr::write(
                dst,
                (((l & 0x00ffffff) as u64) << CHANNEL_LEFT_SHIFT)
                    | (((r & 0x00ffffff) as u64) << CHANNEL_RIGHT_SHIFT),
            );

            dst_pos = (dst_pos + size_of::<u64_t>() as c_ulong) & CHANNEL_RING_MASK;
            src_pos = ((src_pos as c_ulong + (2 * size_of::<s16_t>()) as c_ulong) & src_mask) as c_int;
            count -= size_of::<u64_t>() as c_uint;
        }

        writeq(dst_pos as u64, &mut (*mace).perif.audio.chan[ch_usize].write_ptr); /* in bytes */
        (*chip).channel[ch_usize].pos = src_pos;
    });

    ret
}

unsafe extern "C" fn snd_sgio2audio_dma_start(substream: *mut snd_pcm_substream) -> c_int {
    let chip = snd_pcm_substream_chip(substream) as *mut snd_sgio2audio;
    let chan = (*(*substream).runtime).private_data as *mut snd_sgio2audio_chan;
    let ch = (*chan).idx;

    /* reset DMA channel */
    writeq(CHANNEL_CONTROL_RESET, &mut (*mace).perif.audio.chan[ch as usize].control);
    udelay(10);
    writeq(0, &mut (*mace).perif.audio.chan[ch as usize].control);

    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        /* push a full buffer */
        snd_sgio2audio_dma_push_frag(chip, ch as c_uint, CHANNEL_RING_SIZE - 32);
    }
    /* set DMA to wake on 50% empty and enable interrupt */
    writeq(
        CHANNEL_DMA_ENABLE | CHANNEL_INT_THRESHOLD_50,
        &mut (*mace).perif.audio.chan[ch as usize].control,
    );
    0
}

unsafe extern "C" fn snd_sgio2audio_dma_stop(substream: *mut snd_pcm_substream) -> c_int {
    let chan = (*(*substream).runtime).private_data as *mut snd_sgio2audio_chan;

    writeq(0, &mut (*mace).perif.audio.chan[(*chan).idx as usize].control);
    0
}

unsafe extern "C" fn snd_sgio2audio_dma_in_isr(_irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    let chan = dev_id as *mut snd_sgio2audio_chan;
    let substream = (*chan).substream;
    let chip = snd_pcm_substream_chip(substream) as *mut snd_sgio2audio;
    let ch = (*chan).idx;

    /* empty the ring */
    let count = CHANNEL_RING_SIZE as c_int
        - readq(&(*mace).perif.audio.chan[ch as usize].depth) as c_int
        - 32;
    if snd_sgio2audio_dma_pull_frag(chip, ch as c_uint, count as c_uint) != 0 {
        snd_pcm_period_elapsed(substream);
    }

    IRQ_HANDLED
}

unsafe extern "C" fn snd_sgio2audio_dma_out_isr(_irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    let chan = dev_id as *mut snd_sgio2audio_chan;
    let substream = (*chan).substream;
    let chip = snd_pcm_substream_chip(substream) as *mut snd_sgio2audio;
    let ch = (*chan).idx;
    /* fill the ring */
    let count = CHANNEL_RING_SIZE as c_int
        - readq(&(*mace).perif.audio.chan[ch as usize].depth) as c_int
        - 32;
    if snd_sgio2audio_dma_push_frag(chip, ch as c_uint, count as c_uint) != 0 {
        snd_pcm_period_elapsed(substream);
    }

    IRQ_HANDLED
}

unsafe extern "C" fn snd_sgio2audio_error_isr(_irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    let chan = dev_id as *mut snd_sgio2audio_chan;
    let substream = (*chan).substream;

    snd_sgio2audio_dma_stop(substream);
    snd_sgio2audio_dma_start(substream);
    IRQ_HANDLED
}

/* PCM part */
/* PCM hardware definition */
static mut SND_SGIO2AUDIO_PCM_HW: snd_pcm_hardware = snd_pcm_hardware {
    info: 0,
    formats: 0,
    rates: 0,
    rate_min: 8000,
    rate_max: 48000,
    channels_min: 2,
    channels_max: 2,
    buffer_bytes_max: 65536,
    period_bytes_min: 32768,
    period_bytes_max: 65536,
    periods_min: 1,
    periods_max: 1024,
};

/* PCM playback open callback */
unsafe extern "C" fn snd_sgio2audio_playback1_open(substream: *mut snd_pcm_substream) -> c_int {
    let chip = snd_pcm_substream_chip(substream) as *mut snd_sgio2audio;
    let runtime = (*substream).runtime;

    (*runtime).hw = SND_SGIO2AUDIO_PCM_HW;
    (*runtime).private_data = &mut (*chip).channel[1] as *mut _ as *mut c_void;
    0
}

unsafe extern "C" fn snd_sgio2audio_playback2_open(substream: *mut snd_pcm_substream) -> c_int {
    let chip = snd_pcm_substream_chip(substream) as *mut snd_sgio2audio;
    let runtime = (*substream).runtime;

    (*runtime).hw = SND_SGIO2AUDIO_PCM_HW;
    (*runtime).private_data = &mut (*chip).channel[2] as *mut _ as *mut c_void;
    0
}

/* PCM capture open callback */
unsafe extern "C" fn snd_sgio2audio_capture_open(substream: *mut snd_pcm_substream) -> c_int {
    let chip = snd_pcm_substream_chip(substream) as *mut snd_sgio2audio;
    let runtime = (*substream).runtime;

    (*runtime).hw = SND_SGIO2AUDIO_PCM_HW;
    (*runtime).private_data = &mut (*chip).channel[0] as *mut _ as *mut c_void;
    0
}

/* PCM close callback */
unsafe extern "C" fn snd_sgio2audio_pcm_close(substream: *mut snd_pcm_substream) -> c_int {
    let runtime = (*substream).runtime;

    (*runtime).private_data = ptr::null_mut();
    0
}

/* prepare callback */
unsafe extern "C" fn snd_sgio2audio_pcm_prepare(substream: *mut snd_pcm_substream) -> c_int {
    let chip = snd_pcm_substream_chip(substream) as *mut snd_sgio2audio;
    let runtime = (*substream).runtime;
    let chan = (*(*substream).runtime).private_data as *mut snd_sgio2audio_chan;
    let ch = (*chan).idx;

    with_spinlock_irqsave(&mut (*chip).channel[ch as usize].lock, || {
        /* Setup the pseudo-dma transfer pointers.  */
        (*chip).channel[ch as usize].pos = 0;
        (*chip).channel[ch as usize].size = 0;
        (*chip).channel[ch as usize].substream = substream;

        /* set AD1843 format */
        /* hardware format is always S16_LE */
        if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
            ad1843_setup_dac(
                &mut (*chip).ad1843,
                ch - 1,
                (*runtime).rate,
                SNDRV_PCM_FORMAT_S16_LE,
                (*runtime).channels,
            );
        } else if (*substream).stream == SNDRV_PCM_STREAM_CAPTURE {
            ad1843_setup_adc(
                &mut (*chip).ad1843,
                (*runtime).rate,
                SNDRV_PCM_FORMAT_S16_LE,
                (*runtime).channels,
            );
        }
    });
    0
}

/* trigger callback */
unsafe extern "C" fn snd_sgio2audio_pcm_trigger(
    substream: *mut snd_pcm_substream,
    cmd: c_int,
) -> c_int {
    if cmd == SNDRV_PCM_TRIGGER_START {
        /* start the PCM engine */
        snd_sgio2audio_dma_start(substream);
    } else if cmd == SNDRV_PCM_TRIGGER_STOP {
        /* stop the PCM engine */
        snd_sgio2audio_dma_stop(substream);
    } else {
        return -EINVAL;
    }
    0
}

/* pointer callback */
unsafe extern "C" fn snd_sgio2audio_pcm_pointer(
    substream: *mut snd_pcm_substream,
) -> snd_pcm_uframes_t {
    let chip = snd_pcm_substream_chip(substream) as *mut snd_sgio2audio;
    let chan = (*(*substream).runtime).private_data as *mut snd_sgio2audio_chan;

    /* get the current hardware pointer */
    bytes_to_frames((*substream).runtime, (*chip).channel[(*chan).idx as usize].pos)
}

/* operators */
static SND_SGIO2AUDIO_PLAYBACK1_OPS: snd_pcm_ops = snd_pcm_ops {
    open: Some(snd_sgio2audio_playback1_open),
    close: Some(snd_sgio2audio_pcm_close),
    prepare: Some(snd_sgio2audio_pcm_prepare),
    trigger: Some(snd_sgio2audio_pcm_trigger),
    pointer: Some(snd_sgio2audio_pcm_pointer),
};

static SND_SGIO2AUDIO_PLAYBACK2_OPS: snd_pcm_ops = snd_pcm_ops {
    open: Some(snd_sgio2audio_playback2_open),
    close: Some(snd_sgio2audio_pcm_close),
    prepare: Some(snd_sgio2audio_pcm_prepare),
    trigger: Some(snd_sgio2audio_pcm_trigger),
    pointer: Some(snd_sgio2audio_pcm_pointer),
};

static SND_SGIO2AUDIO_CAPTURE_OPS: snd_pcm_ops = snd_pcm_ops {
    open: Some(snd_sgio2audio_capture_open),
    close: Some(snd_sgio2audio_pcm_close),
    prepare: Some(snd_sgio2audio_pcm_prepare),
    trigger: Some(snd_sgio2audio_pcm_trigger),
    pointer: Some(snd_sgio2audio_pcm_pointer),
};

/*
 *  definitions of capture are omitted here...
 */

/* create a pcm device */
unsafe extern "C" fn snd_sgio2audio_new_pcm(chip: *mut snd_sgio2audio) -> c_int {
    let mut pcm: *mut snd_pcm = ptr::null_mut();
    let mut err: c_int;

    /* create first pcm device with one outputs and one input */
    err = snd_pcm_new((*chip).card, b"SGI O2 Audio\0".as_ptr() as *const c_char, 0, 1, 1, &mut pcm);
    if err < 0 {
        return err;
    }

    (*pcm).private_data = chip as *mut c_void;
    strscpy((*pcm).name.as_mut_ptr(), b"SGI O2 DAC1\0".as_ptr() as *const c_char);

    /* set operators */
    snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_PLAYBACK, &SND_SGIO2AUDIO_PLAYBACK1_OPS);
    snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_CAPTURE, &SND_SGIO2AUDIO_CAPTURE_OPS);
    snd_pcm_set_managed_buffer_all(pcm, SNDRV_DMA_TYPE_VMALLOC, ptr::null_mut(), 0, 0);

    /* create second  pcm device with one outputs and no input */
    err = snd_pcm_new((*chip).card, b"SGI O2 Audio\0".as_ptr() as *const c_char, 1, 1, 0, &mut pcm);
    if err < 0 {
        return err;
    }

    (*pcm).private_data = chip as *mut c_void;
    strscpy((*pcm).name.as_mut_ptr(), b"SGI O2 DAC2\0".as_ptr() as *const c_char);

    /* set operators */
    snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_PLAYBACK, &SND_SGIO2AUDIO_PLAYBACK2_OPS);
    snd_pcm_set_managed_buffer_all(pcm, SNDRV_DMA_TYPE_VMALLOC, ptr::null_mut(), 0, 0);

    0
}

#[repr(C)]
struct snd_sgio2_isr_entry {
    idx: c_int,
    irq: c_int,
    isr: Option<unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t>,
    desc: *const c_char,
}

static mut SND_SGIO2_ISR_TABLE: [snd_sgio2_isr_entry; 6] = [
    snd_sgio2_isr_entry {
        idx: 0,
        irq: 0,
        isr: Some(snd_sgio2audio_dma_in_isr),
        desc: b"Capture DMA Channel 0\0".as_ptr() as *const c_char,
    },
    snd_sgio2_isr_entry {
        idx: 0,
        irq: 0,
        isr: Some(snd_sgio2audio_error_isr),
        desc: b"Capture Overflow\0".as_ptr() as *const c_char,
    },
    snd_sgio2_isr_entry {
        idx: 1,
        irq: 0,
        isr: Some(snd_sgio2audio_dma_out_isr),
        desc: b"Playback DMA Channel 1\0".as_ptr() as *const c_char,
    },
    snd_sgio2_isr_entry {
        idx: 1,
        irq: 0,
        isr: Some(snd_sgio2audio_error_isr),
        desc: b"Memory Error Channel 1\0".as_ptr() as *const c_char,
    },
    snd_sgio2_isr_entry {
        idx: 2,
        irq: 0,
        isr: Some(snd_sgio2audio_dma_out_isr),
        desc: b"Playback DMA Channel 2\0".as_ptr() as *const c_char,
    },
    snd_sgio2_isr_entry {
        idx: 2,
        irq: 0,
        isr: Some(snd_sgio2audio_error_isr),
        desc: b"Memory Error Channel 2\0".as_ptr() as *const c_char,
    },
];

/* ALSA driver */

unsafe extern "C" fn snd_sgio2audio_free(chip: *mut snd_sgio2audio) -> c_int {
    let mut i: usize;

    /* reset interface */
    writeq(AUDIO_CONTROL_RESET, &mut (*mace).perif.audio.control);
    udelay(1);
    writeq(0, &mut (*mace).perif.audio.control);

    /* release IRQ's */
    i = 0;
    while i < SND_SGIO2_ISR_TABLE.len() {
        free_irq(
            SND_SGIO2_ISR_TABLE[i].irq,
            &mut (*chip).channel[SND_SGIO2_ISR_TABLE[i].idx as usize] as *mut _ as *mut c_void,
        );
        i += 1;
    }

    dma_free_coherent(
        (*(*chip).card).dev,
        MACEISA_RINGBUFFERS_SIZE,
        (*chip).ring_base,
        (*chip).ring_base_dma,
    );

    /* release card data */
    kfree(chip as *mut c_void);
    0
}

unsafe extern "C" fn snd_sgio2audio_dev_free(device: *mut snd_device) -> c_int {
    let chip = (*device).device_data as *mut snd_sgio2audio;

    snd_sgio2audio_free(chip)
}

static OPS: snd_device_ops = snd_device_ops {
    dev_free: Some(snd_sgio2audio_dev_free),
};

unsafe extern "C" fn snd_sgio2audio_create(
    card: *mut snd_card,
    rchip: *mut *mut snd_sgio2audio,
) -> c_int {
    let mut chip: *mut snd_sgio2audio;
    let mut i: usize;
    let mut err: c_int;

    *rchip = ptr::null_mut();

    /* check if a codec is attached to the interface */
    /* (Audio or Audio/Video board present) */
    if (readq(&(*mace).perif.audio.control) & AUDIO_CONTROL_CODEC_PRESENT) == 0 {
        return -ENOENT;
    }

    chip = kzalloc(size_of::<snd_sgio2audio>(), GFP_KERNEL) as *mut snd_sgio2audio;
    if chip.is_null() {
        return -ENOMEM;
    }

    (*chip).card = card;

    (*chip).ring_base = dma_alloc_coherent(
        (*card).dev,
        MACEISA_RINGBUFFERS_SIZE,
        &mut (*chip).ring_base_dma,
        GFP_KERNEL,
    );
    if (*chip).ring_base.is_null() {
        printk(
            b"%ssgio2audio: could not allocate ring buffers\n\0".as_ptr() as *const c_char,
            KERN_ERR,
        );
        kfree(chip as *mut c_void);
        return -ENOMEM;
    }

    spin_lock_init(&mut (*chip).ad1843_lock);

    /* initialize channels */
    i = 0;
    while i < 3 {
        spin_lock_init(&mut (*chip).channel[i].lock);
        (*chip).channel[i].idx = i as c_int;
        i += 1;
    }

    /* allocate IRQs */
    i = 0;
    while i < SND_SGIO2_ISR_TABLE.len() {
        if request_irq(
            SND_SGIO2_ISR_TABLE[i].irq,
            SND_SGIO2_ISR_TABLE[i].isr,
            0,
            SND_SGIO2_ISR_TABLE[i].desc,
            &mut (*chip).channel[SND_SGIO2_ISR_TABLE[i].idx as usize] as *mut _ as *mut c_void,
        ) != 0
        {
            snd_sgio2audio_free(chip);
            printk(
                b"%ssgio2audio: cannot allocate irq %d\n\0".as_ptr() as *const c_char,
                KERN_ERR,
                SND_SGIO2_ISR_TABLE[i].irq,
            );
            return -EBUSY;
        }
        i += 1;
    }

    /* reset the interface */
    writeq(AUDIO_CONTROL_RESET, &mut (*mace).perif.audio.control);
    udelay(1);
    writeq(0, &mut (*mace).perif.audio.control);
    msleep_interruptible(1); /* give time to recover */

    /* set ring base */
    writeq((*chip).ring_base_dma, &mut (*mace).perif.ctrl.ringbase);

    /* attach the AD1843 codec */
    (*chip).ad1843.read = Some(read_ad1843_reg);
    (*chip).ad1843.write = Some(write_ad1843_reg);
    (*chip).ad1843.chip = chip as *mut c_void;

    /* initialize the AD1843 codec */
    err = ad1843_init(&mut (*chip).ad1843);
    if err < 0 {
        snd_sgio2audio_free(chip);
        return err;
    }

    err = snd_device_new(card, SNDRV_DEV_LOWLEVEL, chip as *mut c_void, &OPS);
    if err < 0 {
        snd_sgio2audio_free(chip);
        return err;
    }
    *rchip = chip;
    0
}

unsafe extern "C" fn snd_sgio2audio_probe(pdev: *mut platform_device) -> c_int {
    let mut card: *mut snd_card = ptr::null_mut();
    let mut chip: *mut snd_sgio2audio = ptr::null_mut();
    let mut err: c_int;

    err = snd_card_new(&mut (*pdev).dev, index, id, THIS_MODULE, 0, &mut card);
    if err < 0 {
        return err;
    }

    err = snd_sgio2audio_create(card, &mut chip);
    if err < 0 {
        snd_card_free(card);
        return err;
    }

    err = snd_sgio2audio_new_pcm(chip);
    if err < 0 {
        snd_card_free(card);
        return err;
    }
    err = snd_sgio2audio_new_mixer(chip);
    if err < 0 {
        snd_card_free(card);
        return err;
    }

    strscpy((*card).driver.as_mut_ptr(), b"SGI O2 Audio\0".as_ptr() as *const c_char);
    strscpy((*card).shortname.as_mut_ptr(), b"SGI O2 Audio\0".as_ptr() as *const c_char);
    sprintf(
        (*card).longname.as_mut_ptr(),
        b"%s irq %i-%i\0".as_ptr() as *const c_char,
        (*card).shortname.as_ptr(),
        MACEISA_AUDIO1_DMAT_IRQ,
        MACEISA_AUDIO3_MERR_IRQ,
    );

    err = snd_card_register(card);
    if err < 0 {
        snd_card_free(card);
        return err;
    }
    platform_set_drvdata(pdev, card as *mut c_void);
    0
}

unsafe extern "C" fn snd_sgio2audio_remove(pdev: *mut platform_device) {
    let card = platform_get_drvdata(pdev) as *mut snd_card;

    snd_card_free(card);
}

static mut SGIO2AUDIO_DRIVER: platform_driver = platform_driver {
    probe: Some(snd_sgio2audio_probe),
    remove: Some(snd_sgio2audio_remove),
    driver: platform_driver_driver {
        name: b"sgio2audio\0".as_ptr() as *const c_char,
    },
};

// module_platform_driver(sgio2audio_driver);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
