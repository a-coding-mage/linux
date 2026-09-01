// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Driver for PowerMac AWACS
 * Copyright (c) 2001 by Takashi Iwai <tiwai@suse.de>
 *   based on dmasound.c.
 */

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

const CHIP_NAME: &[u8] = b"PMac\0";
const SND_PMAC_DRIVER: &[u8] = b"snd_powermac\0";

/* MODULE_DESCRIPTION("PowerMac"); */
/* MODULE_LICENSE("GPL"); */

static mut index: c_int = SNDRV_DEFAULT_IDX1; /* Index 0-MAX */
static mut id: *mut c_char = SNDRV_DEFAULT_STR1 as *mut c_char; /* ID for this card */
static mut enable_beep: bool = true;

/* module_param(index, int, 0444); */
/* MODULE_PARM_DESC(index, "Index value for " CHIP_NAME " soundchip."); */
/* module_param(id, charp, 0444); */
/* MODULE_PARM_DESC(id, "ID string for " CHIP_NAME " soundchip."); */
/* module_param(enable_beep, bool, 0444); */
/* MODULE_PARM_DESC(enable_beep, "Enable beep using PCM."); */

static mut device: *mut platform_device = ptr::null_mut();

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct platform_driver {
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut platform_device)>,
    pub driver: device_driver,
}

#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
    pub pm: *const dev_pm_ops,
}

#[repr(C)]
pub struct dev_pm_ops {
    pub suspend: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    pub resume: Option<unsafe extern "C" fn(*mut device) -> c_int>,
}

#[repr(C)]
pub struct snd_card {
    pub private_data: *mut c_void,
    pub driver: [c_char; 80],
    pub shortname: [c_char; 80],
    pub longname: [c_char; 80],
}

#[repr(C)]
pub struct snd_pmac {
    pub model: c_int,
    pub device_id: c_int,
    pub subframe: c_int,
    pub revision: c_int,
    pub is_pbook_3400: bool,
    pub is_pbook_G3: bool,
    pub initialized: c_int,
}

extern "C" {
    static SNDRV_DEFAULT_IDX1: c_int;
    static SNDRV_DEFAULT_STR1: *const c_char;
    static THIS_MODULE: *mut c_void;

    static PMAC_BURGUNDY: c_int;
    static PMAC_DACA: c_int;
    static PMAC_TUMBLER: c_int;
    static PMAC_SNAPPER: c_int;
    static PMAC_AWACS: c_int;
    static PMAC_SCREAMER: c_int;
    static EINVAL: c_int;

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

    fn snd_pmac_new(card: *mut snd_card, chip_ret: *mut *mut snd_pmac) -> c_int;
    fn snd_pmac_burgundy_init(chip: *mut snd_pmac) -> c_int;
    fn snd_pmac_daca_init(chip: *mut snd_pmac) -> c_int;
    fn snd_pmac_tumbler_init(chip: *mut snd_pmac) -> c_int;
    fn snd_pmac_tumbler_post_init() -> c_int;
    fn snd_pmac_awacs_init(chip: *mut snd_pmac) -> c_int;
    fn snd_pmac_pcm_new(chip: *mut snd_pmac) -> c_int;
    fn snd_pmac_attach_beep(chip: *mut snd_pmac);
    fn snd_pmac_suspend(chip: *mut c_void);
    fn snd_pmac_resume(chip: *mut c_void);

    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut c_void);
    fn platform_get_drvdata(pdev: *mut platform_device) -> *mut c_void;
    fn platform_driver_register(driver: *mut platform_driver) -> c_int;
    fn platform_driver_unregister(driver: *mut platform_driver);
    fn platform_device_register_simple(
        name: *const c_char,
        id: c_int,
        res: *const c_void,
        num: c_uint,
    ) -> *mut platform_device;
    fn platform_device_unregister(pdev: *mut platform_device);
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;

    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn strscpy(dst: *mut c_char, src: *const c_char);
    fn sprintf(dst: *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn IS_ERR(ptr: *const c_void) -> bool;
}

/*
 */

unsafe extern "C" fn snd_pmac_probe(devptr: *mut platform_device) -> c_int {
    let mut card: *mut snd_card = ptr::null_mut();
    let mut chip: *mut snd_pmac = ptr::null_mut();
    let mut name_ext: *const c_char;
    let mut err: c_int;

    err = snd_card_new(
        &mut (*devptr).dev,
        index,
        id,
        THIS_MODULE,
        0,
        &mut card,
    );
    if err < 0 {
        return err;
    }

    err = snd_pmac_new(card, &mut chip);
    if err < 0 {
        snd_card_free(card);
        return err;
    }
    (*card).private_data = chip as *mut c_void;

    if (*chip).model == PMAC_BURGUNDY {
        strscpy((*card).driver.as_mut_ptr(), b"PMac Burgundy\0".as_ptr() as *const c_char);
        strscpy((*card).shortname.as_mut_ptr(), b"PowerMac Burgundy\0".as_ptr() as *const c_char);
        sprintf(
            (*card).longname.as_mut_ptr(),
            b"%s (Dev %d) Sub-frame %d\0".as_ptr() as *const c_char,
            (*card).shortname.as_mut_ptr(),
            (*chip).device_id,
            (*chip).subframe,
        );
        err = snd_pmac_burgundy_init(chip);
        if err < 0 {
            snd_card_free(card);
            return err;
        }
    } else if (*chip).model == PMAC_DACA {
        strscpy((*card).driver.as_mut_ptr(), b"PMac DACA\0".as_ptr() as *const c_char);
        strscpy((*card).shortname.as_mut_ptr(), b"PowerMac DACA\0".as_ptr() as *const c_char);
        sprintf(
            (*card).longname.as_mut_ptr(),
            b"%s (Dev %d) Sub-frame %d\0".as_ptr() as *const c_char,
            (*card).shortname.as_mut_ptr(),
            (*chip).device_id,
            (*chip).subframe,
        );
        err = snd_pmac_daca_init(chip);
        if err < 0 {
            snd_card_free(card);
            return err;
        }
    } else if (*chip).model == PMAC_TUMBLER || (*chip).model == PMAC_SNAPPER {
        name_ext = if (*chip).model == PMAC_TUMBLER {
            b"Tumbler\0".as_ptr() as *const c_char
        } else {
            b"Snapper\0".as_ptr() as *const c_char
        };
        sprintf(
            (*card).driver.as_mut_ptr(),
            b"PMac %s\0".as_ptr() as *const c_char,
            name_ext,
        );
        sprintf(
            (*card).shortname.as_mut_ptr(),
            b"PowerMac %s\0".as_ptr() as *const c_char,
            name_ext,
        );
        sprintf(
            (*card).longname.as_mut_ptr(),
            b"%s (Dev %d) Sub-frame %d\0".as_ptr() as *const c_char,
            (*card).shortname.as_mut_ptr(),
            (*chip).device_id,
            (*chip).subframe,
        );
        err = snd_pmac_tumbler_init(chip);
        if err < 0 {
            snd_card_free(card);
            return err;
        }
        err = snd_pmac_tumbler_post_init();
        if err < 0 {
            snd_card_free(card);
            return err;
        }
    } else if (*chip).model == PMAC_AWACS || (*chip).model == PMAC_SCREAMER {
        name_ext = if (*chip).model == PMAC_SCREAMER {
            b"Screamer\0".as_ptr() as *const c_char
        } else {
            b"AWACS\0".as_ptr() as *const c_char
        };
        sprintf(
            (*card).driver.as_mut_ptr(),
            b"PMac %s\0".as_ptr() as *const c_char,
            name_ext,
        );
        sprintf(
            (*card).shortname.as_mut_ptr(),
            b"PowerMac %s\0".as_ptr() as *const c_char,
            name_ext,
        );
        if (*chip).is_pbook_3400 {
            name_ext = b" [PB3400]\0".as_ptr() as *const c_char;
        } else if (*chip).is_pbook_G3 {
            name_ext = b" [PBG3]\0".as_ptr() as *const c_char;
        } else {
            name_ext = b"\0".as_ptr() as *const c_char;
        }
        sprintf(
            (*card).longname.as_mut_ptr(),
            b"%s%s Rev %d\0".as_ptr() as *const c_char,
            (*card).shortname.as_mut_ptr(),
            name_ext,
            (*chip).revision,
        );
        err = snd_pmac_awacs_init(chip);
        if err < 0 {
            snd_card_free(card);
            return err;
        }
    } else {
        dev_err(
            &mut (*devptr).dev,
            b"unsupported hardware %d\n\0".as_ptr() as *const c_char,
            (*chip).model,
        );
        err = -EINVAL;
        snd_card_free(card);
        return err;
    }

    err = snd_pmac_pcm_new(chip);
    if err < 0 {
        snd_card_free(card);
        return err;
    }

    (*chip).initialized = 1;
    if enable_beep {
        snd_pmac_attach_beep(chip);
    }

    err = snd_card_register(card);
    if err < 0 {
        snd_card_free(card);
        return err;
    }

    platform_set_drvdata(devptr, card as *mut c_void);
    0
}

unsafe extern "C" fn snd_pmac_remove(devptr: *mut platform_device) {
    snd_card_free(platform_get_drvdata(devptr) as *mut snd_card);
}

/* #ifdef CONFIG_PM_SLEEP */
unsafe extern "C" fn snd_pmac_driver_suspend(dev: *mut device) -> c_int {
    let card: *mut snd_card = dev_get_drvdata(dev) as *mut snd_card;
    snd_pmac_suspend((*card).private_data);
    0
}

unsafe extern "C" fn snd_pmac_driver_resume(dev: *mut device) -> c_int {
    let card: *mut snd_card = dev_get_drvdata(dev) as *mut snd_card;
    snd_pmac_resume((*card).private_data);
    0
}

static snd_pmac_pm: dev_pm_ops = dev_pm_ops {
    suspend: Some(snd_pmac_driver_suspend),
    resume: Some(snd_pmac_driver_resume),
};

const SND_PMAC_PM_OPS: *const dev_pm_ops = &snd_pmac_pm;
/* #else
 * #define SND_PMAC_PM_OPS NULL
 * #endif
 */

static mut snd_pmac_driver: platform_driver = platform_driver {
    probe: Some(snd_pmac_probe),
    remove: Some(snd_pmac_remove),
    driver: device_driver {
        name: SND_PMAC_DRIVER.as_ptr() as *const c_char,
        pm: SND_PMAC_PM_OPS,
    },
};

unsafe extern "C" fn alsa_card_pmac_init() -> c_int {
    let mut err: c_int;

    err = platform_driver_register(&mut snd_pmac_driver);
    if err < 0 {
        return err;
    }
    device = platform_device_register_simple(
        SND_PMAC_DRIVER.as_ptr() as *const c_char,
        -1,
        ptr::null(),
        0,
    );
    0
}

unsafe extern "C" fn alsa_card_pmac_exit() {
    if !IS_ERR(device as *const c_void) {
        platform_device_unregister(device);
    }
    platform_driver_unregister(&mut snd_pmac_driver);
}

/* module_init(alsa_card_pmac_init) */
/* module_exit(alsa_card_pmac_exit) */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
