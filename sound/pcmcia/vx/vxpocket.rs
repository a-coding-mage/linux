// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Driver for Digigram VXpocket V2/440 soundcards
 *
 * Copyright (c) 2002 by Takashi Iwai <tiwai@suse.de>
 */

/* C dependencies:
 * <linux/init.h>, <linux/module.h>, <linux/slab.h>, <sound/core.h>,
 * "vxpocket.h", <pcmcia/ciscode.h>, <pcmcia/cisreg.h>,
 * <sound/initval.h>, <sound/tlv.h>
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::ptr;

/* MODULE_AUTHOR("Takashi Iwai <tiwai@suse.de>");
 * MODULE_DESCRIPTION("Digigram VXPocket");
 * MODULE_LICENSE("GPL");
 */

type bool_t = bool;
type irqreturn_t = c_int;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct resource {
    pub start: c_ulong,
    pub end: c_ulong,
    pub flags: c_ulong,
}

#[repr(C)]
pub struct pcmcia_device {
    pub irq: c_int,
    pub priv_: *mut c_void,
    pub resource: [*mut resource; 1],
    pub config_flags: c_uint,
    pub config_index: c_uint,
    pub config_regs: c_uint,
    pub prod_id: [*const c_char; 4],
    pub devname: *const c_char,
    pub dev: device,
}

#[repr(C)]
pub struct snd_card {
    pub dev: *mut device,
    pub driver: [c_char; 16],
    pub shortname: [c_char; 32],
    pub longname: [c_char; 80],
    pub sync_irq: c_int,
    pub private_data: *mut c_void,
}

#[repr(C)]
pub struct vx_ibl {
    pub size: c_int,
}

#[repr(C)]
pub struct snd_vx_hardware {
    pub name: *const c_char,
    pub type_: c_uint,
    pub num_codecs: c_uint,
    pub num_ins: c_uint,
    pub num_outs: c_uint,
    pub output_level_max: c_uint,
    pub output_level_db_scale: *const c_uint,
}

#[repr(C)]
pub struct vx_core {
    pub card: *mut snd_card,
    pub hw: *const snd_vx_hardware,
    pub type_: c_uint,
    pub ibl: vx_ibl,
    pub irq: c_int,
    pub chip_status: c_uint,
}

#[repr(C)]
pub struct snd_vxpocket {
    pub core: vx_core,
    pub p_dev: *mut pcmcia_device,
    pub port: c_int,
    pub index: c_int,
}

#[repr(C)]
pub struct pcmcia_device_id {
    pub _private: [usize; 4],
}

#[repr(C)]
pub struct pcmcia_driver {
    pub owner: *mut c_void,
    pub name: *const c_char,
    pub probe: Option<unsafe extern "C" fn(*mut pcmcia_device) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut pcmcia_device)>,
    pub id_table: *const pcmcia_device_id,
    /* CONFIG_PM fields are present in C only when power management is enabled. */
    pub suspend: Option<unsafe extern "C" fn(*mut pcmcia_device) -> c_int>,
    pub resume: Option<unsafe extern "C" fn(*mut pcmcia_device) -> c_int>,
}

unsafe extern "C" {
    static THIS_MODULE: *mut c_void;
    static snd_vxpocket_ops: c_void;

    fn free_irq(irq: c_int, dev_id: *mut c_void);
    fn pcmcia_disable_device(link: *mut pcmcia_device);
    fn snd_vx_create(
        card: *mut snd_card,
        hw: *const snd_vx_hardware,
        ops: *const c_void,
        extra_size: usize,
    ) -> *mut vx_core;
    fn snd_vx_setup_firmware(chip: *mut vx_core) -> c_int;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strscpy(dst: *mut c_char, src: *const c_char) -> isize;
    fn pcmcia_request_io(link: *mut pcmcia_device) -> c_int;
    fn request_threaded_irq(
        irq: c_int,
        handler: Option<unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t>,
        thread_fn: Option<unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t>,
        irqflags: c_ulong,
        devname: *const c_char,
        dev_id: *mut c_void,
    ) -> c_int;
    fn pcmcia_enable_device(link: *mut pcmcia_device) -> c_int;
    fn snd_vx_irq_handler(irq: c_int, dev_id: *mut c_void) -> irqreturn_t;
    fn snd_vx_threaded_irq_handler(irq: c_int, dev_id: *mut c_void) -> irqreturn_t;
    fn pcmcia_dev_present(link: *mut pcmcia_device) -> c_int;
    fn snd_vx_suspend(chip: *mut vx_core);
    fn snd_vx_resume(chip: *mut vx_core);
    fn snd_card_new(
        parent: *mut device,
        idx: c_int,
        xid: *mut c_char,
        module: *mut c_void,
        extra_size: c_int,
        card_ret: *mut *mut snd_card,
    ) -> c_int;
    fn snd_card_free(card: *mut snd_card);
    fn snd_card_disconnect(card: *mut snd_card);
    fn snd_card_free_when_closed(card: *mut snd_card);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn sprintf(buf: *mut c_char, fmt: *const c_char, ...) -> c_int;
}

extern "Rust" {
    static SNDRV_CARDS: usize;
    static SNDRV_DEFAULT_IDX: [c_int; SNDRV_CARDS];
    static SNDRV_DEFAULT_STR: [*mut c_char; SNDRV_CARDS];
    static SNDRV_DEFAULT_ENABLE_PNP: [bool_t; SNDRV_CARDS];
}

const ENOMEM: c_int = 12;
const ENODEV: c_int = 19;
const EINVAL: c_int = 22;
const VX_TYPE_VXPOCKET: c_uint = 0;
const VX_TYPE_VXP440: c_uint = 1;
const VX_ANALOG_OUT_LEVEL_MAX: c_uint = 0;
const IO_DATA_PATH_WIDTH_AUTO: c_ulong = 0;
const CONF_ENABLE_IRQ: c_uint = 0;
const PRESENT_OPTION: c_uint = 0;
const IRQF_SHARED: c_ulong = 0;
const VX_STAT_IS_STALE: c_uint = 0;

static mut index: [c_int; SNDRV_CARDS] = SNDRV_DEFAULT_IDX; /* Index 0-MAX */
static mut id: [*mut c_char; SNDRV_CARDS] = SNDRV_DEFAULT_STR; /* ID for this card */
static mut enable: [bool_t; SNDRV_CARDS] = SNDRV_DEFAULT_ENABLE_PNP; /* Enable switches */
static mut ibl: [c_int; SNDRV_CARDS] = [0; SNDRV_CARDS];

/* module_param_array(index, int, NULL, 0444);
 * MODULE_PARM_DESC(index, "Index value for VXPocket soundcard.");
 * module_param_array(id, charp, NULL, 0444);
 * MODULE_PARM_DESC(id, "ID string for VXPocket soundcard.");
 * module_param_array(enable, bool, NULL, 0444);
 * MODULE_PARM_DESC(enable, "Enable VXPocket soundcard.");
 * module_param_array(ibl, int, NULL, 0444);
 * MODULE_PARM_DESC(ibl, "Capture IBL size for VXPocket soundcard.");
 */

static mut card_alloc: c_uint = 0;

unsafe extern "C" fn vxpocket_release(link: *mut pcmcia_device) {
    unsafe {
        free_irq((*link).irq, (*link).priv_);
        pcmcia_disable_device(link);
    }
}

/*
 * Hardware information
 */

/* VX-pocket V2
 *
 * 1 DSP, 1 sync UER
 * 1 programmable clock (NIY)
 * 1 stereo analog input (line/micro)
 * 1 stereo analog output
 * Only output levels can be modified
 */

/* DECLARE_TLV_DB_SCALE(db_scale_old_vol, -11350, 50, 0); */
static db_scale_old_vol: [c_uint; 4] = [0, (-11350i32) as c_uint, 50, 0];

static vxpocket_name: &[u8; 9] = b"VXPocket\0";
static vxp440_name: &[u8; 12] = b"VXPocket440\0";

static vxpocket_hw: snd_vx_hardware = snd_vx_hardware {
    name: vxpocket_name.as_ptr() as *const c_char,
    type_: VX_TYPE_VXPOCKET,

    /* hardware specs */
    num_codecs: 1,
    num_ins: 1,
    num_outs: 1,
    output_level_max: VX_ANALOG_OUT_LEVEL_MAX,
    output_level_db_scale: db_scale_old_vol.as_ptr(),
};

/* VX-pocket 440
 *
 * 1 DSP, 1 sync UER, 1 sync World Clock (NIY)
 * SMPTE (NIY)
 * 2 stereo analog input (line/micro)
 * 2 stereo analog output
 * Only output levels can be modified
 * UER, but only for the first two inputs and outputs.
 */

static vxp440_hw: snd_vx_hardware = snd_vx_hardware {
    name: vxp440_name.as_ptr() as *const c_char,
    type_: VX_TYPE_VXP440,

    /* hardware specs */
    num_codecs: 2,
    num_ins: 2,
    num_outs: 2,
    output_level_max: VX_ANALOG_OUT_LEVEL_MAX,
    output_level_db_scale: db_scale_old_vol.as_ptr(),
};

#[inline]
unsafe fn to_vxpocket(chip: *mut vx_core) -> *mut snd_vxpocket {
    chip as *mut snd_vxpocket
}

/*
 * create vxpocket instance
 */
unsafe extern "C" fn snd_vxpocket_new(
    card: *mut snd_card,
    ibl_arg: c_int,
    link: *mut pcmcia_device,
    chip_ret: *mut *mut snd_vxpocket,
) -> c_int {
    unsafe {
        let chip: *mut vx_core = snd_vx_create(
            card,
            &vxpocket_hw,
            &snd_vxpocket_ops,
            core::mem::size_of::<snd_vxpocket>() - core::mem::size_of::<vx_core>(),
        );
        if chip.is_null() {
            return -ENOMEM;
        }

        (*chip).ibl.size = ibl_arg;

        let vxp: *mut snd_vxpocket = to_vxpocket(chip);

        (*vxp).p_dev = link;
        (*link).priv_ = chip as *mut c_void;

        (*(*link).resource[0]).flags |= IO_DATA_PATH_WIDTH_AUTO;
        (*(*link).resource[0]).end = 16;

        (*link).config_flags |= CONF_ENABLE_IRQ;
        (*link).config_index = 1;
        (*link).config_regs = PRESENT_OPTION;

        *chip_ret = vxp;
        0
    }
}

/**
 * snd_vxpocket_assign_resources - initialize the hardware and card instance.
 * @chip: VX core instance
 * @port: i/o port for the card
 * @irq: irq number for the card
 *
 * this function assigns the specified port and irq, boot the card,
 * create pcm and control instances, and initialize the rest hardware.
 *
 * returns 0 if successful, or a negative error code.
 */
unsafe extern "C" fn snd_vxpocket_assign_resources(
    chip: *mut vx_core,
    port: c_int,
    irq: c_int,
) -> c_int {
    unsafe {
        let card: *mut snd_card = (*chip).card;
        let vxp: *mut snd_vxpocket = to_vxpocket(chip);

        dev_dbg(
            (*(*chip).card).dev,
            b"vxpocket assign resources: port = 0x%x, irq = %d\n\0".as_ptr() as *const c_char,
            port,
            irq,
        );
        (*vxp).port = port;

        sprintf(
            (*card).shortname.as_mut_ptr(),
            b"Digigram %s\0".as_ptr() as *const c_char,
            (*card).driver.as_ptr(),
        );
        sprintf(
            (*card).longname.as_mut_ptr(),
            b"%s at 0x%x, irq %i\0".as_ptr() as *const c_char,
            (*card).shortname.as_ptr(),
            port,
            irq,
        );

        (*chip).irq = irq;
        (*card).sync_irq = (*chip).irq;

        let err: c_int = snd_vx_setup_firmware(chip);
        if err < 0 {
            return err;
        }

        0
    }
}

/*
 * configuration callback
 */

unsafe extern "C" fn vxpocket_config(link: *mut pcmcia_device) -> c_int {
    unsafe {
        let chip: *mut vx_core = (*link).priv_ as *mut vx_core;
        let ret: c_int;

        /* redefine hardware record according to the VERSION1 string */
        if strcmp((*link).prod_id[1], b"VX-POCKET\0".as_ptr() as *const c_char) == 0 {
            dev_dbg((*(*chip).card).dev, b"VX-pocket is detected\n\0".as_ptr() as *const c_char);
        } else {
            dev_dbg(
                (*(*chip).card).dev,
                b"VX-pocket 440 is detected\n\0".as_ptr() as *const c_char,
            );
            /* overwrite the hardware information */
            (*chip).hw = &vxp440_hw;
            (*chip).type_ = vxp440_hw.type_;
            strscpy((*(*chip).card).driver.as_mut_ptr(), vxp440_hw.name);
        }

        ret = pcmcia_request_io(link);
        if ret != 0 {
            goto_failed_preirq(link);
            return -ENODEV;
        }

        ret = request_threaded_irq(
            (*link).irq,
            Some(snd_vx_irq_handler),
            Some(snd_vx_threaded_irq_handler),
            IRQF_SHARED,
            (*link).devname,
            (*link).priv_,
        );
        if ret != 0 {
            goto_failed_preirq(link);
            return -ENODEV;
        }

        ret = pcmcia_enable_device(link);
        if ret != 0 {
            goto_failed(link);
            return -ENODEV;
        }

        if snd_vxpocket_assign_resources(chip, (*(*link).resource[0]).start as c_int, (*link).irq) < 0
        {
            goto_failed(link);
            return -ENODEV;
        }

        0
    }
}

unsafe fn goto_failed(link: *mut pcmcia_device) {
    unsafe {
        free_irq((*link).irq, (*link).priv_);
        goto_failed_preirq(link);
    }
}

unsafe fn goto_failed_preirq(link: *mut pcmcia_device) {
    unsafe {
        pcmcia_disable_device(link);
    }
}

/* CONFIG_PM */

unsafe extern "C" fn vxp_suspend(link: *mut pcmcia_device) -> c_int {
    unsafe {
        let chip: *mut vx_core = (*link).priv_ as *mut vx_core;

        if !chip.is_null() {
            snd_vx_suspend(chip);
        }

        0
    }
}

unsafe extern "C" fn vxp_resume(link: *mut pcmcia_device) -> c_int {
    unsafe {
        let chip: *mut vx_core = (*link).priv_ as *mut vx_core;

        if pcmcia_dev_present(link) != 0 {
            if !chip.is_null() {
                snd_vx_resume(chip);
            }
        }

        0
    }
}

unsafe extern "C" fn vxpocket_probe(p_dev: *mut pcmcia_device) -> c_int {
    unsafe {
        let mut card: *mut snd_card = ptr::null_mut();
        let mut vxp: *mut snd_vxpocket = ptr::null_mut();
        let mut i: c_int;
        let mut err: c_int;

        /* find an empty slot from the card list */
        i = 0;
        while (i as usize) < SNDRV_CARDS {
            if (card_alloc & (1u32 << i)) == 0 {
                break;
            }
            i += 1;
        }
        if (i as usize) >= SNDRV_CARDS {
            dev_err(
                &mut (*p_dev).dev,
                b"vxpocket: too many cards found\n\0".as_ptr() as *const c_char,
            );
            return -EINVAL;
        }
        if !enable[i as usize] {
            return -ENODEV; /* disabled explicitly */
        }

        /* ok, create a card instance */
        err = snd_card_new(
            &mut (*p_dev).dev,
            index[i as usize],
            id[i as usize],
            THIS_MODULE,
            0,
            &mut card,
        );
        if err < 0 {
            dev_err(
                &mut (*p_dev).dev,
                b"vxpocket: cannot create a card instance\n\0".as_ptr() as *const c_char,
            );
            return err;
        }

        err = snd_vxpocket_new(card, ibl[i as usize], p_dev, &mut vxp);
        if err < 0 {
            snd_card_free(card);
            return err;
        }
        (*card).private_data = vxp as *mut c_void;

        (*vxp).index = i;
        card_alloc |= 1u32 << i;

        (*vxp).p_dev = p_dev;

        err = vxpocket_config(p_dev);
        if err < 0 {
            card_alloc &= !(1u32 << i);
            snd_card_free(card);
            return err;
        }
        0
    }
}

unsafe extern "C" fn vxpocket_detach(link: *mut pcmcia_device) {
    unsafe {
        let vxp: *mut snd_vxpocket;
        let chip: *mut vx_core;

        if link.is_null() {
            return;
        }

        vxp = (*link).priv_ as *mut snd_vxpocket;
        chip = vxp as *mut vx_core;
        card_alloc &= !(1u32 << (*vxp).index);

        (*chip).chip_status |= VX_STAT_IS_STALE; /* to be sure */
        snd_card_disconnect((*chip).card);
        vxpocket_release(link);
        snd_card_free_when_closed((*chip).card);
    }
}

/*
 * Module entry points
 */

const fn PCMCIA_DEVICE_MANF_CARD(_manf: c_uint, _card: c_uint) -> pcmcia_device_id {
    pcmcia_device_id { _private: [0; 4] }
}

const PCMCIA_DEVICE_NULL: pcmcia_device_id = pcmcia_device_id { _private: [0; 4] };

static vxp_ids: [pcmcia_device_id; 2] = [
    PCMCIA_DEVICE_MANF_CARD(0x01f1, 0x0100),
    PCMCIA_DEVICE_NULL,
];
/* MODULE_DEVICE_TABLE(pcmcia, vxp_ids); */

static mut vxp_cs_driver: pcmcia_driver = pcmcia_driver {
    owner: unsafe { THIS_MODULE },
    name: b"snd-vxpocket\0".as_ptr() as *const c_char,
    probe: Some(vxpocket_probe),
    remove: Some(vxpocket_detach),
    id_table: vxp_ids.as_ptr(),
    /* CONFIG_PM */
    suspend: Some(vxp_suspend),
    resume: Some(vxp_resume),
};
/* module_pcmcia_driver(vxp_cs_driver); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
