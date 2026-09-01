// SPDX-License-Identifier: GPL-2.0-only
/*****************************************************************************
 *
 * Copyright (C) 2008 Cedric Bregardis <cedric.bregardis@free.fr> and
 * Jean-Christian Hassler <jhassler@free.fr>
 *
 * This file is part of the Audiowerk2 ALSA driver
 *
 *****************************************************************************/

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

/* C includes removed. This translation depends on Linux, ALSA, saa7146,
 * and aw2-saa7146 declarations provided by the surrounding build.
 */

/* MODULE_AUTHOR("Cedric Bregardis <cedric.bregardis@free.fr>, "
 *               "Jean-Christian Hassler <jhassler@free.fr>");
 * MODULE_DESCRIPTION("Emagic Audiowerk 2 sound driver");
 * MODULE_LICENSE("GPL");
 */

/*********************************
 * DEFINES
 ********************************/
const CTL_ROUTE_ANALOG: c_int = 0;
const CTL_ROUTE_DIGITAL: c_int = 1;

/*********************************
 * EXTERNAL DEPENDENCIES
 ********************************/
type snd_pcm_uframes_t = c_ulong;
type dma_addr_t = c_ulong;
type snd_aw2_saa7146_it_cb = Option<unsafe extern "C" fn(*mut c_void)>;

const SNDRV_CARDS: usize = 8;
const NB_STREAM_PLAYBACK: usize = 2;
const NB_STREAM_CAPTURE: usize = 1;
const NUM_STREAM_PLAYBACK_ANA: usize = 0;
const NUM_STREAM_PLAYBACK_DIG: usize = 1;
const NUM_STREAM_CAPTURE_ANA: usize = 0;

const SNDRV_PCM_INFO_MMAP: c_uint = 0x0000_0001;
const SNDRV_PCM_INFO_INTERLEAVED: c_uint = 0x0000_0100;
const SNDRV_PCM_INFO_BLOCK_TRANSFER: c_uint = 0x0001_0000;
const SNDRV_PCM_INFO_MMAP_VALID: c_uint = 0x0000_0002;
const SNDRV_PCM_FMTBIT_S16_LE: u64 = 1 << 2;
const SNDRV_PCM_RATE_44100: c_uint = 1 << 6;
const SNDRV_CTL_ELEM_IFACE_MIXER: c_uint = 2;
const SNDRV_CTL_ELEM_ACCESS_READWRITE: c_uint = 3;
const SNDRV_PCM_TRIGGER_START: c_int = 0;
const SNDRV_PCM_TRIGGER_STOP: c_int = 1;
const SNDRV_PCM_STREAM_PLAYBACK: c_int = 0;
const SNDRV_PCM_STREAM_CAPTURE: c_int = 1;
const SNDRV_DMA_TYPE_DEV: c_int = 2;
const IRQF_SHARED: c_ulong = 0x80;
const ENXIO: c_int = 6;
const EBUSY: c_int = 16;
const ENODEV: c_int = 19;
const ENOENT: c_int = 2;
const EINVAL: c_int = 22;
const PCI_VENDOR_ID_PHILIPS: c_uint = 0x1131;
const PCI_DEVICE_ID_PHILIPS_SAA7146: c_uint = 0x7146;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pci_dev {
    pub dev: device,
    pub irq: c_int,
}

#[repr(C)]
pub struct pci_device_id {
    pub vendor: c_uint,
    pub device: c_uint,
    pub subvendor: c_uint,
    pub subdevice: c_uint,
    pub class: c_uint,
    pub class_mask: c_uint,
    pub driver_data: c_ulong,
}

#[repr(C)]
pub struct snd_pcm_hardware {
    pub info: c_uint,
    pub formats: u64,
    pub rates: c_uint,
    pub rate_min: c_uint,
    pub rate_max: c_uint,
    pub channels_min: c_uint,
    pub channels_max: c_uint,
    pub buffer_bytes_max: c_ulong,
    pub period_bytes_min: c_ulong,
    pub period_bytes_max: c_ulong,
    pub periods_min: c_uint,
    pub periods_max: c_uint,
}

#[repr(C)]
pub struct snd_pcm_runtime {
    pub hw: snd_pcm_hardware,
    pub dma_addr: dma_addr_t,
    pub dma_area: *mut c_void,
    pub buffer_size: snd_pcm_uframes_t,
}

#[repr(C)]
pub struct snd_card {
    pub private_data: *mut c_void,
    pub dev: *mut device,
    pub sync_irq: c_int,
    pub private_free: Option<unsafe extern "C" fn(*mut snd_card)>,
    pub driver: [c_char; 16],
    pub shortname: [c_char; 32],
    pub longname: [c_char; 80],
}

#[repr(C)]
pub struct snd_pcm {
    pub card: *mut snd_card,
    pub name: [c_char; 80],
    pub private_data: *mut c_void,
}

#[repr(C)]
pub struct snd_pcm_substream {
    pub runtime: *mut snd_pcm_runtime,
    pub pcm: *mut snd_pcm,
}

#[repr(C)]
pub struct snd_pcm_ops {
    pub open: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    pub close: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    pub ioctl: Option<unsafe extern "C" fn() -> c_int>,
    pub hw_params: Option<unsafe extern "C" fn() -> c_int>,
    pub hw_free: Option<unsafe extern "C" fn() -> c_int>,
    pub prepare: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    pub trigger: Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_int) -> c_int>,
    pub pointer: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> snd_pcm_uframes_t>,
}

#[repr(C)]
pub struct snd_kcontrol {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_ctl_elem_info {
    _private: [u8; 0],
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_elem_value_enumerated {
    pub item: [c_uint; 128],
}

type c_long = i64;

#[repr(C)]
pub struct snd_kcontrol_new {
    pub iface: c_uint,
    pub name: *const c_char,
    pub index: c_uint,
    pub access: c_uint,
    pub private_value: c_ulong,
    pub info: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_info) -> c_int>,
    pub get: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    pub put: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
}

#[repr(C)]
pub struct pci_driver {
    pub name: *const c_char,
    pub id_table: *const pci_device_id,
    pub probe: Option<unsafe extern "C" fn(*mut pci_dev, *const pci_device_id) -> c_int>,
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
pub struct snd_aw2_saa7146 {
    _private: [u8; 0],
}

unsafe extern "C" {
    static THIS_MODULE: *mut c_void;
    static KBUILD_MODNAME: *const c_char;

    fn pcim_enable_device(pci: *mut pci_dev) -> c_int;
    fn pci_set_master(pci: *mut pci_dev);
    fn dma_set_mask_and_coherent(dev: *mut device, mask: u64) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_info(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn pcim_iomap_region(pci: *mut pci_dev, bar: c_int, name: *const c_char) -> *mut c_void;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn pci_resource_start(pci: *mut pci_dev, bar: c_int) -> c_ulong;
    fn devm_request_irq(
        dev: *mut device,
        irq: c_int,
        handler: unsafe extern "C" fn(c_int, *mut c_void) -> c_int,
        flags: c_ulong,
        name: *const c_char,
        data: *mut c_void,
    ) -> c_int;
    fn snd_devm_card_new(
        dev: *mut device,
        idx: c_int,
        id: *mut c_char,
        module: *mut c_void,
        extra_size: usize,
        card_ret: *mut *mut snd_card,
    ) -> c_int;
    fn mutex_init(lock: *mut mutex);
    fn spin_lock_init(lock: *mut spinlock_t);
    fn strscpy(dst: *mut c_char, src: *const c_char) -> isize;
    fn sprintf(buf: *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn snd_card_register(card: *mut snd_card) -> c_int;
    fn pci_set_drvdata(pci: *mut pci_dev, data: *mut c_void);
    fn snd_card_free(card: *mut snd_card);
    fn snd_pcm_substream_chip(substream: *mut snd_pcm_substream) -> *mut c_void;
    fn snd_pcm_lib_period_bytes(substream: *mut snd_pcm_substream) -> c_ulong;
    fn snd_pcm_lib_buffer_bytes(substream: *mut snd_pcm_substream) -> c_ulong;
    fn snd_pcm_period_elapsed(substream: *mut snd_pcm_substream);
    fn bytes_to_frames(runtime: *mut snd_pcm_runtime, size: c_uint) -> snd_pcm_uframes_t;
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
        dev: *mut device,
        size: usize,
        max: usize,
    );
    fn snd_ctl_add(card: *mut snd_card, kcontrol: *mut snd_kcontrol) -> c_int;
    fn snd_ctl_new1(ncontrol: *const snd_kcontrol_new, private_data: *mut c_void) -> *mut snd_kcontrol;
    fn snd_ctl_enum_info(
        info: *mut snd_ctl_elem_info,
        channels: c_uint,
        items: c_uint,
        names: *const *const c_char,
    ) -> c_int;
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut c_void;

    fn snd_aw2_saa7146_free(saa7146: *mut snd_aw2_saa7146);
    fn snd_aw2_saa7146_setup(saa7146: *mut snd_aw2_saa7146, iobase: *mut c_void);
    fn snd_aw2_saa7146_interrupt(irq: c_int, dev_id: *mut c_void) -> c_int;
    fn snd_aw2_saa7146_pcm_init_playback(
        saa7146: *mut snd_aw2_saa7146,
        stream: c_uint,
        dma_addr: dma_addr_t,
        period_size: c_ulong,
        buffer_size: c_ulong,
    );
    fn snd_aw2_saa7146_pcm_init_capture(
        saa7146: *mut snd_aw2_saa7146,
        stream: c_uint,
        dma_addr: dma_addr_t,
        period_size: c_ulong,
        buffer_size: c_ulong,
    );
    fn snd_aw2_saa7146_define_it_playback_callback(
        stream: c_uint,
        cb: snd_aw2_saa7146_it_cb,
        data: *mut c_void,
    );
    fn snd_aw2_saa7146_define_it_capture_callback(
        stream: c_uint,
        cb: snd_aw2_saa7146_it_cb,
        data: *mut c_void,
    );
    fn snd_aw2_saa7146_pcm_trigger_start_playback(saa7146: *mut snd_aw2_saa7146, stream: c_uint);
    fn snd_aw2_saa7146_pcm_trigger_stop_playback(saa7146: *mut snd_aw2_saa7146, stream: c_uint);
    fn snd_aw2_saa7146_pcm_trigger_start_capture(saa7146: *mut snd_aw2_saa7146, stream: c_uint);
    fn snd_aw2_saa7146_pcm_trigger_stop_capture(saa7146: *mut snd_aw2_saa7146, stream: c_uint);
    fn snd_aw2_saa7146_get_hw_ptr_playback(
        saa7146: *mut snd_aw2_saa7146,
        stream: c_uint,
        dma_area: *mut c_void,
        buffer_size: snd_pcm_uframes_t,
    ) -> c_uint;
    fn snd_aw2_saa7146_get_hw_ptr_capture(
        saa7146: *mut snd_aw2_saa7146,
        stream: c_uint,
        dma_area: *mut c_void,
        buffer_size: snd_pcm_uframes_t,
    ) -> c_uint;
    fn snd_aw2_saa7146_is_using_digital_input(saa7146: *mut snd_aw2_saa7146) -> c_int;
    fn snd_aw2_saa7146_use_digital_input(saa7146: *mut snd_aw2_saa7146, use_digital: c_int);
}

const fn DMA_BIT_MASK(nr: u32) -> u64 {
    if nr == 64 {
        !0u64
    } else {
        (1u64 << nr) - 1
    }
}

/*********************************
 * TYPEDEFS
 ********************************/
/* hardware definition */
static snd_aw2_playback_hw: snd_pcm_hardware = snd_pcm_hardware {
    info: SNDRV_PCM_INFO_MMAP
        | SNDRV_PCM_INFO_INTERLEAVED
        | SNDRV_PCM_INFO_BLOCK_TRANSFER
        | SNDRV_PCM_INFO_MMAP_VALID,
    formats: SNDRV_PCM_FMTBIT_S16_LE,
    rates: SNDRV_PCM_RATE_44100,
    rate_min: 44100,
    rate_max: 44100,
    channels_min: 2,
    channels_max: 4,
    buffer_bytes_max: 32768,
    period_bytes_min: 4096,
    period_bytes_max: 32768,
    periods_min: 1,
    periods_max: 1024,
};

static snd_aw2_capture_hw: snd_pcm_hardware = snd_pcm_hardware {
    info: SNDRV_PCM_INFO_MMAP
        | SNDRV_PCM_INFO_INTERLEAVED
        | SNDRV_PCM_INFO_BLOCK_TRANSFER
        | SNDRV_PCM_INFO_MMAP_VALID,
    formats: SNDRV_PCM_FMTBIT_S16_LE,
    rates: SNDRV_PCM_RATE_44100,
    rate_min: 44100,
    rate_max: 44100,
    channels_min: 2,
    channels_max: 2,
    buffer_bytes_max: 32768,
    period_bytes_min: 4096,
    period_bytes_max: 32768,
    periods_min: 1,
    periods_max: 1024,
};

#[repr(C)]
pub struct aw2_pcm_device {
    pub pcm: *mut snd_pcm,
    pub stream_number: c_uint,
    pub chip: *mut aw2,
}

#[repr(C)]
pub struct aw2 {
    pub saa7146: snd_aw2_saa7146,

    pub pci: *mut pci_dev,
    pub irq: c_int,
    pub reg_lock: spinlock_t,
    pub mtx: mutex,

    pub iobase_phys: c_ulong,
    pub iobase_virt: *mut c_void,

    pub card: *mut snd_card,

    pub device_playback: [aw2_pcm_device; NB_STREAM_PLAYBACK],
    pub device_capture: [aw2_pcm_device; NB_STREAM_CAPTURE],
}

/*********************************
 * VARIABLES
 ********************************/
static mut index: [c_int; SNDRV_CARDS] = [0; SNDRV_CARDS]; /* SNDRV_DEFAULT_IDX */
static mut id: [*mut c_char; SNDRV_CARDS] = [core::ptr::null_mut(); SNDRV_CARDS]; /* SNDRV_DEFAULT_STR */
static mut enable: [bool; SNDRV_CARDS] = [true; SNDRV_CARDS]; /* SNDRV_DEFAULT_ENABLE_PNP */

/* module_param_array(index, int, NULL, 0444);
 * MODULE_PARM_DESC(index, "Index value for Audiowerk2 soundcard.");
 * module_param_array(id, charp, NULL, 0444);
 * MODULE_PARM_DESC(id, "ID string for the Audiowerk2 soundcard.");
 * module_param_array(enable, bool, NULL, 0444);
 * MODULE_PARM_DESC(enable, "Enable Audiowerk2 soundcard.");
 */

static snd_aw2_ids: [pci_device_id; 2] = [
    pci_device_id {
        vendor: PCI_VENDOR_ID_PHILIPS,
        device: PCI_DEVICE_ID_PHILIPS_SAA7146,
        subvendor: 0,
        subdevice: 0,
        class: 0,
        class_mask: 0,
        driver_data: 0,
    },
    pci_device_id {
        vendor: 0,
        device: 0,
        subvendor: 0,
        subdevice: 0,
        class: 0,
        class_mask: 0,
        driver_data: 0,
    },
];

/* MODULE_DEVICE_TABLE(pci, snd_aw2_ids); */

/* pci_driver definition */
static mut aw2_driver: pci_driver = pci_driver {
    name: core::ptr::null(),
    id_table: snd_aw2_ids.as_ptr(),
    probe: Some(snd_aw2_probe),
};

/* module_pci_driver(aw2_driver); */

/* operators for playback PCM alsa interface */
static snd_aw2_playback_ops: snd_pcm_ops = snd_pcm_ops {
    open: Some(snd_aw2_pcm_playback_open),
    close: Some(snd_aw2_pcm_playback_close),
    ioctl: None,
    hw_params: None,
    hw_free: None,
    prepare: Some(snd_aw2_pcm_prepare_playback),
    trigger: Some(snd_aw2_pcm_trigger_playback),
    pointer: Some(snd_aw2_pcm_pointer_playback),
};

/* operators for capture PCM alsa interface */
static snd_aw2_capture_ops: snd_pcm_ops = snd_pcm_ops {
    open: Some(snd_aw2_pcm_capture_open),
    close: Some(snd_aw2_pcm_capture_close),
    ioctl: None,
    hw_params: None,
    hw_free: None,
    prepare: Some(snd_aw2_pcm_prepare_capture),
    trigger: Some(snd_aw2_pcm_trigger_capture),
    pointer: Some(snd_aw2_pcm_pointer_capture),
};

static aw2_control_name: &[u8] = b"PCM Capture Route\0";

static aw2_control: snd_kcontrol_new = snd_kcontrol_new {
    iface: SNDRV_CTL_ELEM_IFACE_MIXER,
    name: aw2_control_name.as_ptr() as *const c_char,
    index: 0,
    access: SNDRV_CTL_ELEM_ACCESS_READWRITE,
    private_value: 0xffff,
    info: Some(snd_aw2_control_switch_capture_info),
    get: Some(snd_aw2_control_switch_capture_get),
    put: Some(snd_aw2_control_switch_capture_put),
};

/*********************************
 * FUNCTION IMPLEMENTATIONS
 ********************************/

/* component-destructor */
unsafe extern "C" fn snd_aw2_free(card: *mut snd_card) {
    let chip = (*card).private_data as *mut aw2;

    /* Free hardware */
    snd_aw2_saa7146_free(core::ptr::addr_of_mut!((*chip).saa7146));
}

/* chip-specific constructor */
unsafe extern "C" fn snd_aw2_create(card: *mut snd_card, pci: *mut pci_dev) -> c_int {
    let chip = (*card).private_data as *mut aw2;
    let mut err: c_int;

    /* initialize the PCI entry */
    err = pcim_enable_device(pci);
    if err < 0 {
        return err;
    }
    pci_set_master(pci);

    /* check PCI availability (32bit DMA) */
    if dma_set_mask_and_coherent(core::ptr::addr_of_mut!((*pci).dev), DMA_BIT_MASK(32)) != 0 {
        dev_err((*card).dev, b"Impossible to set 32bit mask DMA\n\0".as_ptr() as *const c_char);
        return -ENXIO;
    }

    /* initialize the stuff */
    (*chip).card = card;
    (*chip).pci = pci;
    (*chip).irq = -1;

    /* (1) PCI resource allocation */
    (*chip).iobase_virt = pcim_iomap_region(pci, 0, b"Audiowerk2\0".as_ptr() as *const c_char);
    if IS_ERR((*chip).iobase_virt) {
        return PTR_ERR((*chip).iobase_virt);
    }
    (*chip).iobase_phys = pci_resource_start(pci, 0);

    /* (2) initialization of the chip hardware */
    snd_aw2_saa7146_setup(core::ptr::addr_of_mut!((*chip).saa7146), (*chip).iobase_virt);

    if devm_request_irq(
        core::ptr::addr_of_mut!((*pci).dev),
        (*pci).irq,
        snd_aw2_saa7146_interrupt,
        IRQF_SHARED,
        KBUILD_MODNAME,
        chip as *mut c_void,
    ) != 0
    {
        dev_err((*card).dev, b"Cannot grab irq %d\n\0".as_ptr() as *const c_char, (*pci).irq);
        return -EBUSY;
    }
    (*chip).irq = (*pci).irq;
    (*card).sync_irq = (*chip).irq;
    (*card).private_free = Some(snd_aw2_free);

    dev_info(
        (*card).dev,
        b"Audiowerk 2 sound card (saa7146 chipset) detected and managed\n\0".as_ptr()
            as *const c_char,
    );
    0
}

/* constructor */
unsafe extern "C" fn snd_aw2_probe(pci: *mut pci_dev, _pci_id: *const pci_device_id) -> c_int {
    static mut dev: c_int = 0;
    let mut card: *mut snd_card = core::ptr::null_mut();
    let mut chip: *mut aw2;
    let mut err: c_int;

    /* (1) Continue if device is not enabled, else inc dev */
    if dev >= SNDRV_CARDS as c_int {
        return -ENODEV;
    }

    if !enable[dev as usize] {
        dev += 1;
        return -ENOENT;
    }

    /* (2) Create card instance */
    err = snd_devm_card_new(
        core::ptr::addr_of_mut!((*pci).dev),
        index[dev as usize],
        id[dev as usize],
        THIS_MODULE,
        core::mem::size_of::<aw2>(),
        core::ptr::addr_of_mut!(card),
    );
    if err < 0 {
        return err;
    }
    chip = (*card).private_data as *mut aw2;

    /* (3) Create main component */
    err = snd_aw2_create(card, pci);
    if err < 0 {
        snd_card_free(card);
        return err;
    }

    /* initialize mutex */
    mutex_init(core::ptr::addr_of_mut!((*chip).mtx));
    /* init spinlock */
    spin_lock_init(core::ptr::addr_of_mut!((*chip).reg_lock));
    /* (4) Define driver ID and name string */
    strscpy((*card).driver.as_mut_ptr(), b"aw2\0".as_ptr() as *const c_char);
    strscpy((*card).shortname.as_mut_ptr(), b"Audiowerk2\0".as_ptr() as *const c_char);

    sprintf(
        (*card).longname.as_mut_ptr(),
        b"%s with SAA7146 irq %i\0".as_ptr() as *const c_char,
        (*card).shortname.as_ptr(),
        (*chip).irq,
    );

    /* (5) Create other components */
    snd_aw2_new_pcm(chip);

    /* (6) Register card instance */
    err = snd_card_register(card);
    if err < 0 {
        snd_card_free(card);
        return err;
    }

    /* (7) Set PCI driver data */
    pci_set_drvdata(pci, card as *mut c_void);

    dev += 1;
    0
}

/* open callback */
unsafe extern "C" fn snd_aw2_pcm_playback_open(substream: *mut snd_pcm_substream) -> c_int {
    let runtime = (*substream).runtime;

    dev_dbg((*(*(*substream).pcm).card).dev, b"Playback_open\n\0".as_ptr() as *const c_char);
    (*runtime).hw = snd_aw2_playback_hw;
    0
}

/* close callback */
unsafe extern "C" fn snd_aw2_pcm_playback_close(_substream: *mut snd_pcm_substream) -> c_int {
    return 0;
}

unsafe extern "C" fn snd_aw2_pcm_capture_open(substream: *mut snd_pcm_substream) -> c_int {
    let runtime = (*substream).runtime;

    dev_dbg((*(*(*substream).pcm).card).dev, b"Capture_open\n\0".as_ptr() as *const c_char);
    (*runtime).hw = snd_aw2_capture_hw;
    0
}

/* close callback */
unsafe extern "C" fn snd_aw2_pcm_capture_close(_substream: *mut snd_pcm_substream) -> c_int {
    /* TODO: something to do ? */
    0
}

/* prepare callback for playback */
unsafe extern "C" fn snd_aw2_pcm_prepare_playback(substream: *mut snd_pcm_substream) -> c_int {
    let pcm_device = snd_pcm_substream_chip(substream) as *mut aw2_pcm_device;
    let chip = (*pcm_device).chip;
    let runtime = (*substream).runtime;
    let period_size: c_ulong;
    let buffer_size: c_ulong;

    /* guard(mutex)(&chip->mtx); */

    period_size = snd_pcm_lib_period_bytes(substream);
    buffer_size = snd_pcm_lib_buffer_bytes(substream);

    snd_aw2_saa7146_pcm_init_playback(
        core::ptr::addr_of_mut!((*chip).saa7146),
        (*pcm_device).stream_number,
        (*runtime).dma_addr,
        period_size,
        buffer_size,
    );

    /* Define Interrupt callback */
    snd_aw2_saa7146_define_it_playback_callback(
        (*pcm_device).stream_number,
        Some(core::mem::transmute::<
            unsafe extern "C" fn(*mut snd_pcm_substream),
            unsafe extern "C" fn(*mut c_void),
        >(snd_pcm_period_elapsed)),
        substream as *mut c_void,
    );

    0
}

/* prepare callback for capture */
unsafe extern "C" fn snd_aw2_pcm_prepare_capture(substream: *mut snd_pcm_substream) -> c_int {
    let pcm_device = snd_pcm_substream_chip(substream) as *mut aw2_pcm_device;
    let chip = (*pcm_device).chip;
    let runtime = (*substream).runtime;
    let period_size: c_ulong;
    let buffer_size: c_ulong;

    /* guard(mutex)(&chip->mtx); */

    period_size = snd_pcm_lib_period_bytes(substream);
    buffer_size = snd_pcm_lib_buffer_bytes(substream);

    snd_aw2_saa7146_pcm_init_capture(
        core::ptr::addr_of_mut!((*chip).saa7146),
        (*pcm_device).stream_number,
        (*runtime).dma_addr,
        period_size,
        buffer_size,
    );

    /* Define Interrupt callback */
    snd_aw2_saa7146_define_it_capture_callback(
        (*pcm_device).stream_number,
        Some(core::mem::transmute::<
            unsafe extern "C" fn(*mut snd_pcm_substream),
            unsafe extern "C" fn(*mut c_void),
        >(snd_pcm_period_elapsed)),
        substream as *mut c_void,
    );

    0
}

/* playback trigger callback */
unsafe extern "C" fn snd_aw2_pcm_trigger_playback(
    substream: *mut snd_pcm_substream,
    cmd: c_int,
) -> c_int {
    let pcm_device = snd_pcm_substream_chip(substream) as *mut aw2_pcm_device;
    let chip = (*pcm_device).chip;

    /* guard(spinlock)(&chip->reg_lock); */
    match cmd {
        SNDRV_PCM_TRIGGER_START => {
            snd_aw2_saa7146_pcm_trigger_start_playback(
                core::ptr::addr_of_mut!((*chip).saa7146),
                (*pcm_device).stream_number,
            );
        }
        SNDRV_PCM_TRIGGER_STOP => {
            snd_aw2_saa7146_pcm_trigger_stop_playback(
                core::ptr::addr_of_mut!((*chip).saa7146),
                (*pcm_device).stream_number,
            );
        }
        _ => {
            return -EINVAL;
        }
    }
    0
}

/* capture trigger callback */
unsafe extern "C" fn snd_aw2_pcm_trigger_capture(
    substream: *mut snd_pcm_substream,
    cmd: c_int,
) -> c_int {
    let pcm_device = snd_pcm_substream_chip(substream) as *mut aw2_pcm_device;
    let chip = (*pcm_device).chip;

    /* guard(spinlock)(&chip->reg_lock); */
    match cmd {
        SNDRV_PCM_TRIGGER_START => {
            snd_aw2_saa7146_pcm_trigger_start_capture(
                core::ptr::addr_of_mut!((*chip).saa7146),
                (*pcm_device).stream_number,
            );
        }
        SNDRV_PCM_TRIGGER_STOP => {
            snd_aw2_saa7146_pcm_trigger_stop_capture(
                core::ptr::addr_of_mut!((*chip).saa7146),
                (*pcm_device).stream_number,
            );
        }
        _ => {
            return -EINVAL;
        }
    }
    0
}

/* playback pointer callback */
unsafe extern "C" fn snd_aw2_pcm_pointer_playback(
    substream: *mut snd_pcm_substream,
) -> snd_pcm_uframes_t {
    let pcm_device = snd_pcm_substream_chip(substream) as *mut aw2_pcm_device;
    let chip = (*pcm_device).chip;
    let current_ptr: c_uint;

    /* get the current hardware pointer */
    let runtime = (*substream).runtime;
    current_ptr = snd_aw2_saa7146_get_hw_ptr_playback(
        core::ptr::addr_of_mut!((*chip).saa7146),
        (*pcm_device).stream_number,
        (*runtime).dma_area,
        (*runtime).buffer_size,
    );

    bytes_to_frames((*substream).runtime, current_ptr)
}

/* capture pointer callback */
unsafe extern "C" fn snd_aw2_pcm_pointer_capture(
    substream: *mut snd_pcm_substream,
) -> snd_pcm_uframes_t {
    let pcm_device = snd_pcm_substream_chip(substream) as *mut aw2_pcm_device;
    let chip = (*pcm_device).chip;
    let current_ptr: c_uint;

    /* get the current hardware pointer */
    let runtime = (*substream).runtime;
    current_ptr = snd_aw2_saa7146_get_hw_ptr_capture(
        core::ptr::addr_of_mut!((*chip).saa7146),
        (*pcm_device).stream_number,
        (*runtime).dma_area,
        (*runtime).buffer_size,
    );

    bytes_to_frames((*substream).runtime, current_ptr)
}

/* create a pcm device */
unsafe extern "C" fn snd_aw2_new_pcm(chip: *mut aw2) -> c_int {
    let mut pcm_playback_ana: *mut snd_pcm = core::ptr::null_mut();
    let mut pcm_playback_num: *mut snd_pcm = core::ptr::null_mut();
    let mut pcm_capture: *mut snd_pcm = core::ptr::null_mut();
    let mut pcm_device: *mut aw2_pcm_device;
    let mut err: c_int = 0;

    /* Create new Alsa PCM device */

    err = snd_pcm_new(
        (*chip).card,
        b"Audiowerk2 analog playback\0".as_ptr() as *const c_char,
        0,
        1,
        0,
        core::ptr::addr_of_mut!(pcm_playback_ana),
    );
    if err < 0 {
        dev_err((*(*chip).card).dev, b"snd_pcm_new error (0x%X)\n\0".as_ptr() as *const c_char, err);
        return err;
    }

    /* Creation ok */
    pcm_device = core::ptr::addr_of_mut!((*chip).device_playback[NUM_STREAM_PLAYBACK_ANA]);

    /* Set PCM device name */
    strscpy((*pcm_playback_ana).name.as_mut_ptr(), b"Analog playback\0".as_ptr() as *const c_char);
    /* Associate private data to PCM device */
    (*pcm_playback_ana).private_data = pcm_device as *mut c_void;
    /* set operators of PCM device */
    snd_pcm_set_ops(pcm_playback_ana, SNDRV_PCM_STREAM_PLAYBACK, &snd_aw2_playback_ops);
    /* store PCM device */
    (*pcm_device).pcm = pcm_playback_ana;
    /* give base chip pointer to our internal pcm device
       structure */
    (*pcm_device).chip = chip;
    /* Give stream number to PCM device */
    (*pcm_device).stream_number = NUM_STREAM_PLAYBACK_ANA as c_uint;

    /* pre-allocation of buffers */
    /* Preallocate continuous pages. */
    snd_pcm_set_managed_buffer_all(
        pcm_playback_ana,
        SNDRV_DMA_TYPE_DEV,
        core::ptr::addr_of_mut!((*(*chip).pci).dev),
        64 * 1024,
        64 * 1024,
    );

    err = snd_pcm_new(
        (*chip).card,
        b"Audiowerk2 digital playback\0".as_ptr() as *const c_char,
        1,
        1,
        0,
        core::ptr::addr_of_mut!(pcm_playback_num),
    );

    if err < 0 {
        dev_err((*(*chip).card).dev, b"snd_pcm_new error (0x%X)\n\0".as_ptr() as *const c_char, err);
        return err;
    }
    /* Creation ok */
    pcm_device = core::ptr::addr_of_mut!((*chip).device_playback[NUM_STREAM_PLAYBACK_DIG]);

    /* Set PCM device name */
    strscpy((*pcm_playback_num).name.as_mut_ptr(), b"Digital playback\0".as_ptr() as *const c_char);
    /* Associate private data to PCM device */
    (*pcm_playback_num).private_data = pcm_device as *mut c_void;
    /* set operators of PCM device */
    snd_pcm_set_ops(pcm_playback_num, SNDRV_PCM_STREAM_PLAYBACK, &snd_aw2_playback_ops);
    /* store PCM device */
    (*pcm_device).pcm = pcm_playback_num;
    /* give base chip pointer to our internal pcm device
       structure */
    (*pcm_device).chip = chip;
    /* Give stream number to PCM device */
    (*pcm_device).stream_number = NUM_STREAM_PLAYBACK_DIG as c_uint;

    /* pre-allocation of buffers */
    /* Preallocate continuous pages. */
    snd_pcm_set_managed_buffer_all(
        pcm_playback_num,
        SNDRV_DMA_TYPE_DEV,
        core::ptr::addr_of_mut!((*(*chip).pci).dev),
        64 * 1024,
        64 * 1024,
    );

    err = snd_pcm_new(
        (*chip).card,
        b"Audiowerk2 capture\0".as_ptr() as *const c_char,
        2,
        0,
        1,
        core::ptr::addr_of_mut!(pcm_capture),
    );

    if err < 0 {
        dev_err((*(*chip).card).dev, b"snd_pcm_new error (0x%X)\n\0".as_ptr() as *const c_char, err);
        return err;
    }

    /* Creation ok */
    pcm_device = core::ptr::addr_of_mut!((*chip).device_capture[NUM_STREAM_CAPTURE_ANA]);

    /* Set PCM device name */
    strscpy((*pcm_capture).name.as_mut_ptr(), b"Capture\0".as_ptr() as *const c_char);
    /* Associate private data to PCM device */
    (*pcm_capture).private_data = pcm_device as *mut c_void;
    /* set operators of PCM device */
    snd_pcm_set_ops(pcm_capture, SNDRV_PCM_STREAM_CAPTURE, &snd_aw2_capture_ops);
    /* store PCM device */
    (*pcm_device).pcm = pcm_capture;
    /* give base chip pointer to our internal pcm device
       structure */
    (*pcm_device).chip = chip;
    /* Give stream number to PCM device */
    (*pcm_device).stream_number = NUM_STREAM_CAPTURE_ANA as c_uint;

    /* pre-allocation of buffers */
    /* Preallocate continuous pages. */
    snd_pcm_set_managed_buffer_all(
        pcm_capture,
        SNDRV_DMA_TYPE_DEV,
        core::ptr::addr_of_mut!((*(*chip).pci).dev),
        64 * 1024,
        64 * 1024,
    );

    /* Create control */
    err = snd_ctl_add((*chip).card, snd_ctl_new1(&aw2_control, chip as *mut c_void));
    if err < 0 {
        dev_err((*(*chip).card).dev, b"snd_ctl_add error (0x%X)\n\0".as_ptr() as *const c_char, err);
        return err;
    }

    0
}

unsafe extern "C" fn snd_aw2_control_switch_capture_info(
    _kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    static TEXT_ANALOG: &[u8] = b"Analog\0";
    static TEXT_DIGITAL: &[u8] = b"Digital\0";
    static TEXTS: [*const c_char; 2] = [
        TEXT_ANALOG.as_ptr() as *const c_char,
        TEXT_DIGITAL.as_ptr() as *const c_char,
    ];
    snd_ctl_enum_info(uinfo, 1, 2, TEXTS.as_ptr())
}

unsafe extern "C" fn snd_aw2_control_switch_capture_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let chip = snd_kcontrol_chip(kcontrol) as *mut aw2;
    if snd_aw2_saa7146_is_using_digital_input(core::ptr::addr_of_mut!((*chip).saa7146)) != 0 {
        (*ucontrol).value.enumerated.item[0] = CTL_ROUTE_DIGITAL as c_uint;
    } else {
        (*ucontrol).value.enumerated.item[0] = CTL_ROUTE_ANALOG as c_uint;
    }
    0
}

unsafe extern "C" fn snd_aw2_control_switch_capture_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let chip = snd_kcontrol_chip(kcontrol) as *mut aw2;
    let mut changed: c_int = 0;
    let is_disgital = snd_aw2_saa7146_is_using_digital_input(core::ptr::addr_of_mut!((*chip).saa7146));

    if (((*ucontrol).value.integer.value[0] == CTL_ROUTE_DIGITAL as c_long) && is_disgital == 0)
        || (((*ucontrol).value.integer.value[0] == CTL_ROUTE_ANALOG as c_long) && is_disgital != 0)
    {
        snd_aw2_saa7146_use_digital_input(
            core::ptr::addr_of_mut!((*chip).saa7146),
            if is_disgital == 0 { 1 } else { 0 },
        );
        changed = 1;
    }
    changed
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
