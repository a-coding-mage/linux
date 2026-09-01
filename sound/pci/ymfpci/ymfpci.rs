// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  The driver for the Yamaha's DS1/DS1E cards
 *  Copyright (c) by Jaroslav Kysela <perex@perex.cz>
 */

// Translated from C implementation source. C include dependencies:
// <linux/init.h>, <linux/pci.h>, <linux/time.h>, <linux/module.h>,
// <sound/core.h>, "ymfpci.h", <sound/mpu401.h>, <sound/opl3.h>,
// <sound/initval.h>

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};

type bool_ = bool;
type u16 = u16;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pci_dev {
    pub dev: device,
    pub device: u16,
}

#[repr(C)]
pub struct pci_device_id {
    pub vendor: u32,
    pub device: u32,
    pub subvendor: u32,
    pub subdevice: u32,
    pub class: u32,
    pub class_mask: u32,
    pub driver_data: usize,
}

#[repr(C)]
pub struct resource {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_card {
    pub dev: *mut device,
    pub private_data: *mut c_void,
    pub driver: [c_char; 16],
    pub shortname: [c_char; 32],
    pub longname: [c_char; 80],
}

#[repr(C)]
pub struct snd_ac97 {
    pub ext_id: c_uint,
}

#[repr(C)]
pub struct snd_ymfpci {
    pub pci: *mut pci_dev,
    pub card: *mut snd_card,
    pub gameport: *mut gameport,
    pub reg_area_phys: c_ulong,
    pub irq: c_int,
    pub ac97: *mut snd_ac97,
    pub rawmidi: *mut c_void,
}

#[repr(C)]
pub struct snd_opl3 {
    _private: [u8; 0],
}

#[repr(C)]
pub struct gameport {
    pub io: c_int,
}

#[repr(C)]
pub struct dev_pm_ops {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pci_driver_inner {
    pub pm: *const dev_pm_ops,
}

#[repr(C)]
pub struct pci_driver {
    pub name: *const c_char,
    pub id_table: *const pci_device_id,
    pub probe: Option<unsafe extern "C" fn(*mut pci_dev, *const pci_device_id) -> c_int>,
    pub driver: pci_driver_inner,
}

type c_ulong = core::ffi::c_ulong;

const SNDRV_CARDS: usize = 32;
const SNDRV_DEFAULT_IDX: [c_int; SNDRV_CARDS] = [0; SNDRV_CARDS];
const SNDRV_DEFAULT_STR: [*mut c_char; SNDRV_CARDS] = [core::ptr::null_mut(); SNDRV_CARDS];
const SNDRV_DEFAULT_ENABLE_PNP: [bool_; SNDRV_CARDS] = [true; SNDRV_CARDS];

const ENODEV: c_int = 19;
const ENOENT: c_int = 2;
const EBUSY: c_int = 16;
const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const ENOSYS: c_int = 38;

const PCI_VENDOR_ID_YAMAHA: u32 = 0x1073;
const YMFPCI_LEGACY_JPEN: u16 = 0;
const YMFPCI_LEGACY_FMEN: u16 = 0;
const YMFPCI_LEGACY_MEN: u16 = 0;
const YMFPCI_LEGACY_MIEN: u16 = 0;
const YMFPCI_LEGACY2_FMIO: u16 = 0;
const YMFPCI_LEGACY2_MPUIO: u16 = 0;
const YMFPCI_LEGACY2_IMOD: u16 = 0;
const PCIR_DSXG_JOYBASE: c_int = 0;
const PCIR_DSXG_LEGACY: c_int = 0;
const PCIR_DSXG_ELEGACY: c_int = 0;
const PCIR_DSXG_FMBASE: c_int = 0;
const PCIR_DSXG_MPU401BASE: c_int = 0;
const AC97_EI_SDAC: c_uint = 0;
const MPU401_HW_YMFPCI: c_int = 0;
const MPU401_INFO_INTEGRATED: c_uint = 0;
const MPU401_INFO_IRQ_HOOK: c_uint = 0;
const OPL3_HW_OPL3: c_int = 0;

unsafe extern "C" {
    static THIS_MODULE: *mut c_void;
    static KBUILD_MODNAME: *const c_char;
    static snd_ymfpci_pm: dev_pm_ops;

    fn pci_resource_start(dev: *mut pci_dev, bar: c_int) -> c_long;
    fn request_region(start: c_int, n: c_int, name: *const c_char) -> *mut resource;
    fn devm_request_region(dev: *mut device, start: c_long, n: c_int, name: *const c_char) -> *mut resource;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn gameport_allocate_port() -> *mut gameport;
    fn gameport_set_name(gp: *mut gameport, name: *const c_char);
    fn gameport_set_phys(gp: *mut gameport, fmt: *const c_char, ...);
    fn pci_name(pci: *mut pci_dev) -> *const c_char;
    fn gameport_set_dev_parent(gp: *mut gameport, dev: *mut device);
    fn pci_write_config_word(pci: *mut pci_dev, pos: c_int, val: u16) -> c_int;
    fn gameport_register_port(gp: *mut gameport);
    fn gameport_unregister_port(gp: *mut gameport);
    fn snd_devm_card_new(dev: *mut device, idx: c_int, id: *mut c_char, module: *mut c_void, extra_size: usize, card_ret: *mut *mut snd_card) -> c_int;
    fn strscpy(dst: *mut c_char, src: *const c_char) -> isize;
    fn sprintf(buf: *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn pci_read_config_word(pci: *mut pci_dev, pos: c_int, val: *mut u16) -> c_int;
    fn snd_ymfpci_create(card: *mut snd_card, pci: *mut pci_dev, old_legacy_ctrl: u16) -> c_int;
    fn snd_ymfpci_pcm(chip: *mut snd_ymfpci, device: c_int) -> c_int;
    fn snd_ymfpci_pcm_spdif(chip: *mut snd_ymfpci, device: c_int) -> c_int;
    fn snd_ymfpci_mixer(chip: *mut snd_ymfpci, rear_switch: bool_) -> c_int;
    fn snd_ymfpci_pcm_4ch(chip: *mut snd_ymfpci, device: c_int) -> c_int;
    fn snd_ymfpci_pcm2(chip: *mut snd_ymfpci, device: c_int) -> c_int;
    fn snd_ymfpci_timer(chip: *mut snd_ymfpci, device: c_int) -> c_int;
    fn snd_mpu401_uart_new(card: *mut snd_card, device: c_int, hardware: c_int, port: c_long, info_flags: c_uint, irq: c_int, rrawmidi: *mut *mut c_void) -> c_int;
    fn snd_opl3_create(card: *mut snd_card, l_port: c_long, r_port: c_long, hardware: c_int, integrated: c_int, ropl3: *mut *mut snd_opl3) -> c_int;
    fn snd_opl3_hwdep_new(opl3: *mut snd_opl3, device: c_int, seq_device: c_int, ops: *mut c_void) -> c_int;
    fn snd_card_register(card: *mut snd_card) -> c_int;
    fn pci_set_drvdata(pci: *mut pci_dev, data: *mut c_void);
    fn snd_card_free_on_error(dev: *mut device, err: c_int) -> c_int;
    fn pm_sleep_ptr(pm: *const dev_pm_ops) -> *const dev_pm_ops;
}

// MODULE_AUTHOR("Jaroslav Kysela <perex@perex.cz>");
// MODULE_DESCRIPTION("Yamaha DS-1 PCI");
// MODULE_LICENSE("GPL");

static mut index: [c_int; SNDRV_CARDS] = SNDRV_DEFAULT_IDX; /* Index 0-MAX */
static mut id: [*mut c_char; SNDRV_CARDS] = SNDRV_DEFAULT_STR; /* ID for this card */
static mut enable: [bool_; SNDRV_CARDS] = SNDRV_DEFAULT_ENABLE_PNP; /* Enable this card */
static mut fm_port: [c_long; SNDRV_CARDS] = [0; SNDRV_CARDS];
static mut mpu_port: [c_long; SNDRV_CARDS] = [0; SNDRV_CARDS];
// #ifdef SUPPORT_JOYSTICK
static mut joystick_port: [c_long; SNDRV_CARDS] = [0; SNDRV_CARDS];
// #endif
static mut rear_switch: [bool_; SNDRV_CARDS] = [false; SNDRV_CARDS];

// module_param_array/module_param_hw_array and MODULE_PARM_DESC declarations
// are Linux module metadata in the original C source.

const fn PCI_VDEVICE_YAMAHA(device: u32) -> pci_device_id {
    pci_device_id {
        vendor: PCI_VENDOR_ID_YAMAHA,
        device,
        subvendor: 0,
        subdevice: 0,
        class: 0,
        class_mask: 0,
        driver_data: 0,
    }
}

static snd_ymfpci_ids: [pci_device_id; 7] = [
    PCI_VDEVICE_YAMAHA(0x0004), /* YMF724 */
    PCI_VDEVICE_YAMAHA(0x000d), /* YMF724F */
    PCI_VDEVICE_YAMAHA(0x000a), /* YMF740 */
    PCI_VDEVICE_YAMAHA(0x000c), /* YMF740C */
    PCI_VDEVICE_YAMAHA(0x0010), /* YMF744 */
    PCI_VDEVICE_YAMAHA(0x0012), /* YMF754 */
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

// MODULE_DEVICE_TABLE(pci, snd_ymfpci_ids);

// #ifdef SUPPORT_JOYSTICK
unsafe fn snd_ymfpci_create_gameport(
    chip: *mut snd_ymfpci,
    dev: c_int,
    mut legacy_ctrl: c_int,
    mut legacy_ctrl2: c_int,
) -> c_int {
    let mut gp: *mut gameport;
    let mut r: *mut resource = core::ptr::null_mut();
    let mut io_port: c_int = joystick_port[dev as usize] as c_int;

    if io_port == 0 {
        return -ENODEV;
    }

    if (*(*chip).pci).device >= 0x0010 {
        /* YMF 744/754 */
        if io_port == 1 {
            /* auto-detect */
            io_port = pci_resource_start((*chip).pci, 2) as c_int;
            if io_port == 0 {
                return -ENODEV;
            }
        }
    } else {
        if io_port == 1 {
            /* auto-detect */
            io_port = 0x201;
            while io_port <= 0x205 {
                if io_port == 0x203 {
                    io_port += 1;
                    continue;
                }
                r = request_region(io_port, 1, c"YMFPCI gameport".as_ptr());
                if !r.is_null() {
                    break;
                }
                io_port += 1;
            }
            if r.is_null() {
                dev_err((*(*chip).card).dev, c"no gameport ports available\n".as_ptr());
                return -EBUSY;
            }
        }
        match io_port {
            0x201 => {
                legacy_ctrl2 |= 0 << 6;
            }
            0x202 => {
                legacy_ctrl2 |= 1 << 6;
            }
            0x204 => {
                legacy_ctrl2 |= 2 << 6;
            }
            0x205 => {
                legacy_ctrl2 |= 3 << 6;
            }
            _ => {
                if io_port > 0 {
                    dev_err(
                        (*(*chip).card).dev,
                        c"The %s does not support arbitrary IO ports for the game port (requested 0x%x)\n".as_ptr(),
                        (*(*chip).card).shortname.as_ptr(),
                        io_port as c_uint,
                    );
                }
                return -EINVAL;
            }
        }
    }

    if r.is_null() {
        r = devm_request_region(
            &mut (*(*chip).pci).dev,
            io_port as c_long,
            1,
            c"YMFPCI gameport".as_ptr(),
        );
        if r.is_null() {
            dev_err(
                (*(*chip).card).dev,
                c"joystick port %#x is in use.\n".as_ptr(),
                io_port,
            );
            return -EBUSY;
        }
    }

    gp = gameport_allocate_port();
    (*chip).gameport = gp;
    if gp.is_null() {
        dev_err(
            (*(*chip).card).dev,
            c"cannot allocate memory for gameport\n".as_ptr(),
        );
        return -ENOMEM;
    }

    gameport_set_name(gp, c"Yamaha YMF Gameport".as_ptr());
    gameport_set_phys(gp, c"pci%s/gameport0".as_ptr(), pci_name((*chip).pci));
    gameport_set_dev_parent(gp, &mut (*(*chip).pci).dev);
    (*gp).io = io_port;

    if (*(*chip).pci).device >= 0x0010 {
        /* YMF 744/754 */
        pci_write_config_word((*chip).pci, PCIR_DSXG_JOYBASE, io_port as u16);
    }

    pci_write_config_word(
        (*chip).pci,
        PCIR_DSXG_LEGACY,
        (legacy_ctrl | YMFPCI_LEGACY_JPEN as c_int) as u16,
    );
    pci_write_config_word((*chip).pci, PCIR_DSXG_ELEGACY, legacy_ctrl2 as u16);

    gameport_register_port((*chip).gameport);

    0
}

unsafe fn snd_ymfpci_free_gameport(chip: *mut snd_ymfpci) {
    if !(*chip).gameport.is_null() {
        gameport_unregister_port((*chip).gameport);
        (*chip).gameport = core::ptr::null_mut();
    }
}

// #else
// static inline int snd_ymfpci_create_gameport(struct snd_ymfpci *chip, int dev, int l, int l2) { return -ENOSYS; }
// void snd_ymfpci_free_gameport(struct snd_ymfpci *chip) { }
// #endif /* SUPPORT_JOYSTICK */

unsafe fn __snd_card_ymfpci_probe(
    pci: *mut pci_dev,
    pci_id: *const pci_device_id,
) -> c_int {
    static mut dev: c_int = 0;
    let mut card: *mut snd_card = core::ptr::null_mut();
    let mut fm_res: *mut resource = core::ptr::null_mut();
    let mut mpu_res: *mut resource = core::ptr::null_mut();
    let mut chip: *mut snd_ymfpci;
    let mut opl3: *mut snd_opl3 = core::ptr::null_mut();
    let str_: *const c_char;
    let model: *const c_char;
    let mut err: c_int;
    let mut legacy_ctrl: u16;
    let mut legacy_ctrl2: u16;
    let mut old_legacy_ctrl: u16 = 0;

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
        core::mem::size_of::<snd_ymfpci>(),
        &mut card,
    );
    if err < 0 {
        return err;
    }
    chip = (*card).private_data as *mut snd_ymfpci;

    match (*pci_id).device {
        0x0004 => {
            str_ = c"YMF724".as_ptr();
            model = c"DS-1".as_ptr();
        }
        0x000d => {
            str_ = c"YMF724F".as_ptr();
            model = c"DS-1".as_ptr();
        }
        0x000a => {
            str_ = c"YMF740".as_ptr();
            model = c"DS-1L".as_ptr();
        }
        0x000c => {
            str_ = c"YMF740C".as_ptr();
            model = c"DS-1L".as_ptr();
        }
        0x0010 => {
            str_ = c"YMF744".as_ptr();
            model = c"DS-1S".as_ptr();
        }
        0x0012 => {
            str_ = c"YMF754".as_ptr();
            model = c"DS-1E".as_ptr();
        }
        _ => {
            model = c"???".as_ptr();
            str_ = model;
        }
    }

    strscpy((*card).driver.as_mut_ptr(), str_);
    sprintf(
        (*card).shortname.as_mut_ptr(),
        c"Yamaha %s (%s)".as_ptr(),
        model,
        str_,
    );
    sprintf(
        (*card).longname.as_mut_ptr(),
        c"%s at 0x%lx, irq %i".as_ptr(),
        (*card).shortname.as_ptr(),
        (*chip).reg_area_phys,
        (*chip).irq,
    );

    legacy_ctrl = 0;
    legacy_ctrl2 = 0x0800; /* SBEN = 0, SMOD = 01, LAD = 0 */

    if (*pci_id).device >= 0x0010 {
        /* YMF 744/754 */
        if fm_port[dev as usize] == 1 {
            /* auto-detect */
            fm_port[dev as usize] = pci_resource_start(pci, 1);
        }
        if fm_port[dev as usize] > 0 {
            fm_res = devm_request_region(
                &mut (*pci).dev,
                fm_port[dev as usize],
                4,
                c"YMFPCI OPL3".as_ptr(),
            );
        }
        if !fm_res.is_null() {
            legacy_ctrl |= YMFPCI_LEGACY_FMEN;
            pci_write_config_word(pci, PCIR_DSXG_FMBASE, fm_port[dev as usize] as u16);
        }
        if mpu_port[dev as usize] == 1 {
            /* auto-detect */
            mpu_port[dev as usize] = pci_resource_start(pci, 1) + 0x20;
        }
        if mpu_port[dev as usize] > 0 {
            mpu_res = devm_request_region(
                &mut (*pci).dev,
                mpu_port[dev as usize],
                2,
                c"YMFPCI MPU401".as_ptr(),
            );
        }
        if !mpu_res.is_null() {
            legacy_ctrl |= YMFPCI_LEGACY_MEN;
            pci_write_config_word(
                pci,
                PCIR_DSXG_MPU401BASE,
                mpu_port[dev as usize] as u16,
            );
        }
    } else {
        match fm_port[dev as usize] {
            0x388 => {
                legacy_ctrl2 |= 0;
            }
            0x398 => {
                legacy_ctrl2 |= 1;
            }
            0x3a0 => {
                legacy_ctrl2 |= 2;
            }
            0x3a8 => {
                legacy_ctrl2 |= 3;
            }
            _ => {
                if fm_port[dev as usize] > 0 {
                    dev_err(
                        (*card).dev,
                        c"The %s does not support arbitrary IO ports for FM (requested 0x%x)\n".as_ptr(),
                        (*card).shortname.as_ptr(),
                        fm_port[dev as usize] as c_uint,
                    );
                }
                fm_port[dev as usize] = 0;
            }
        }
        if fm_port[dev as usize] > 0 {
            fm_res = devm_request_region(
                &mut (*pci).dev,
                fm_port[dev as usize],
                4,
                c"YMFPCI OPL3".as_ptr(),
            );
        }
        if !fm_res.is_null() {
            legacy_ctrl |= YMFPCI_LEGACY_FMEN;
        } else {
            legacy_ctrl2 &= !YMFPCI_LEGACY2_FMIO;
            fm_port[dev as usize] = 0;
        }
        match mpu_port[dev as usize] {
            0x330 => {
                legacy_ctrl2 |= 0 << 4;
            }
            0x300 => {
                legacy_ctrl2 |= 1 << 4;
            }
            0x332 => {
                legacy_ctrl2 |= 2 << 4;
            }
            0x334 => {
                legacy_ctrl2 |= 3 << 4;
            }
            _ => {
                if mpu_port[dev as usize] > 0 {
                    dev_err(
                        (*card).dev,
                        c"The %s does not support arbitrary IO ports for MPU-401 (requested 0x%x)\n".as_ptr(),
                        (*card).shortname.as_ptr(),
                        mpu_port[dev as usize] as c_uint,
                    );
                }
                mpu_port[dev as usize] = 0;
            }
        }
        if mpu_port[dev as usize] > 0 {
            mpu_res = devm_request_region(
                &mut (*pci).dev,
                mpu_port[dev as usize],
                2,
                c"YMFPCI MPU401".as_ptr(),
            );
        }
        if !mpu_res.is_null() {
            legacy_ctrl |= YMFPCI_LEGACY_MEN;
        } else {
            legacy_ctrl2 &= !YMFPCI_LEGACY2_MPUIO;
            mpu_port[dev as usize] = 0;
        }
    }
    if !mpu_res.is_null() {
        legacy_ctrl |= YMFPCI_LEGACY_MIEN;
        legacy_ctrl2 |= YMFPCI_LEGACY2_IMOD;
    }
    pci_read_config_word(pci, PCIR_DSXG_LEGACY, &mut old_legacy_ctrl);
    pci_write_config_word(pci, PCIR_DSXG_LEGACY, legacy_ctrl);
    pci_write_config_word(pci, PCIR_DSXG_ELEGACY, legacy_ctrl2);
    err = snd_ymfpci_create(card, pci, old_legacy_ctrl);
    if err < 0 {
        return err;
    }

    err = snd_ymfpci_pcm(chip, 0);
    if err < 0 {
        return err;
    }

    err = snd_ymfpci_pcm_spdif(chip, 1);
    if err < 0 {
        return err;
    }

    err = snd_ymfpci_mixer(chip, rear_switch[dev as usize]);
    if err < 0 {
        return err;
    }

    if (*(*chip).ac97).ext_id & AC97_EI_SDAC != 0 {
        err = snd_ymfpci_pcm_4ch(chip, 2);
        if err < 0 {
            return err;
        }

        err = snd_ymfpci_pcm2(chip, 3);
        if err < 0 {
            return err;
        }
    }
    err = snd_ymfpci_timer(chip, 0);
    if err < 0 {
        return err;
    }

    if !mpu_res.is_null() {
        err = snd_mpu401_uart_new(
            card,
            0,
            MPU401_HW_YMFPCI,
            mpu_port[dev as usize],
            MPU401_INFO_INTEGRATED | MPU401_INFO_IRQ_HOOK,
            -1,
            &mut (*chip).rawmidi,
        );
        if err < 0 {
            dev_warn(
                (*card).dev,
                c"cannot initialize MPU401 at 0x%lx, skipping...\n".as_ptr(),
                mpu_port[dev as usize],
            );
            legacy_ctrl &= !YMFPCI_LEGACY_MIEN; /* disable MPU401 irq */
            pci_write_config_word(pci, PCIR_DSXG_LEGACY, legacy_ctrl);
        }
    }
    if !fm_res.is_null() {
        err = snd_opl3_create(
            card,
            fm_port[dev as usize],
            fm_port[dev as usize] + 2,
            OPL3_HW_OPL3,
            1,
            &mut opl3,
        );
        if err < 0 {
            dev_warn(
                (*card).dev,
                c"cannot initialize FM OPL3 at 0x%lx, skipping...\n".as_ptr(),
                fm_port[dev as usize],
            );
            legacy_ctrl &= !YMFPCI_LEGACY_FMEN;
            pci_write_config_word(pci, PCIR_DSXG_LEGACY, legacy_ctrl);
        } else {
            err = snd_opl3_hwdep_new(opl3, 0, 1, core::ptr::null_mut());
            if err < 0 {
                dev_err((*card).dev, c"cannot create opl3 hwdep\n".as_ptr());
                return err;
            }
        }
    }

    snd_ymfpci_create_gameport(chip, dev, legacy_ctrl as c_int, legacy_ctrl2 as c_int);

    err = snd_card_register(card);
    if err < 0 {
        return err;
    }

    pci_set_drvdata(pci, card as *mut c_void);
    dev += 1;
    0
}

unsafe extern "C" fn snd_card_ymfpci_probe(
    pci: *mut pci_dev,
    pci_id: *const pci_device_id,
) -> c_int {
    snd_card_free_on_error(&mut (*pci).dev, __snd_card_ymfpci_probe(pci, pci_id))
}

static mut ymfpci_driver: pci_driver = pci_driver {
    name: unsafe { KBUILD_MODNAME },
    id_table: snd_ymfpci_ids.as_ptr(),
    probe: Some(snd_card_ymfpci_probe),
    driver: pci_driver_inner {
        pm: unsafe { pm_sleep_ptr(&snd_ymfpci_pm) },
    },
};

// module_pci_driver(ymfpci_driver);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
