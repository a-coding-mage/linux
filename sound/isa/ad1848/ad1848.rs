// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Generic driver for AD1848/AD1847/CS4248 chips (0.1 Alpha)
 *  Copyright (c) by Tugrul Galatali <galatalt@stuy.edu>,
 *                   Jaroslav Kysela <perex@perex.cz>
 *  Based on card-4232.c by Jaroslav Kysela <perex@perex.cz>
 */

/*
 * C dependencies:
 * linux/init.h, linux/err.h, linux/isa.h, linux/time.h, linux/wait.h,
 * linux/module.h, sound/core.h, sound/wss.h, sound/initval.h
 */

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};

const CRD_NAME: &[u8] = b"Generic AD1848/AD1847/CS4248\0";
const DEV_NAME: &[u8] = b"ad1848\0";

/*
 * MODULE_DESCRIPTION(CRD_NAME);
 * MODULE_AUTHOR("Tugrul Galatali <galatalt@stuy.edu>, Jaroslav Kysela <perex@perex.cz>");
 * MODULE_LICENSE("GPL");
 */

extern "C" {
    static THIS_MODULE: *mut c_void;

    static SNDRV_DEFAULT_IDX: [c_int; SNDRV_CARDS];
    static SNDRV_DEFAULT_STR: [*mut c_char; SNDRV_CARDS];
    static SNDRV_DEFAULT_ENABLE: [bool; SNDRV_CARDS];
    static SNDRV_DEFAULT_PORT: [c_long; SNDRV_CARDS];
    static SNDRV_DEFAULT_IRQ: [c_int; SNDRV_CARDS];
    static SNDRV_DEFAULT_DMA: [c_int; SNDRV_CARDS];

    static SNDRV_AUTO_PORT: c_long;
    static SNDRV_AUTO_IRQ: c_int;
    static SNDRV_AUTO_DMA: c_int;
    static WSS_HW_THINKPAD: c_int;
    static WSS_HW_DETECT: c_int;
    static SNDRV_CTL_POWER_D3hot: c_int;
    static SNDRV_CTL_POWER_D0: c_int;

    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn snd_devm_card_new(
        dev: *mut device,
        idx: c_int,
        xid: *mut c_char,
        module: *mut c_void,
        extra_size: c_int,
        card_ret: *mut *mut snd_card,
    ) -> c_int;
    fn snd_wss_create(
        card: *mut snd_card,
        port: c_long,
        cport: c_long,
        irq: c_int,
        dma1: c_int,
        dma2: c_int,
        hardware: c_int,
        hwshare: c_int,
        rchip: *mut *mut snd_wss,
    ) -> c_int;
    fn snd_wss_pcm(chip: *mut snd_wss, device: c_int) -> c_int;
    fn snd_wss_mixer(chip: *mut snd_wss) -> c_int;
    fn strscpy(dst: *mut c_char, src: *const c_char, count: usize) -> isize;
    fn scnprintf(buf: *mut c_char, size: usize, fmt: *const c_char, ...) -> c_int;
    fn snd_card_register(card: *mut snd_card) -> c_int;
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn snd_power_change_state(card: *mut snd_card, state: c_int) -> c_int;
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm {
    pub name: *mut c_char,
}

#[repr(C)]
pub struct snd_card {
    pub private_data: *mut c_void,
    pub driver: [c_char; 16],
    pub shortname: [c_char; 32],
    pub longname: [c_char; 80],
}

#[repr(C)]
pub struct snd_wss {
    pub pcm: *mut snd_pcm,
    pub port: c_long,
    pub suspend: Option<unsafe extern "C" fn(*mut snd_wss)>,
    pub resume: Option<unsafe extern "C" fn(*mut snd_wss)>,
}

#[repr(C)]
pub struct pm_message_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
}

#[repr(C)]
pub struct isa_driver {
    pub match_: Option<unsafe extern "C" fn(*mut device, c_uint) -> c_int>,
    pub probe: Option<unsafe extern "C" fn(*mut device, c_uint) -> c_int>,
    /* CONFIG_PM:
     * .suspend = snd_ad1848_suspend,
     * .resume = snd_ad1848_resume,
     */
    pub suspend: Option<unsafe extern "C" fn(*mut device, c_uint, pm_message_t) -> c_int>,
    pub resume: Option<unsafe extern "C" fn(*mut device, c_uint) -> c_int>,
    pub driver: device_driver,
}

static mut index: [c_int; SNDRV_CARDS] = unsafe { SNDRV_DEFAULT_IDX }; /* Index 0-MAX */
static mut id: [*mut c_char; SNDRV_CARDS] = unsafe { SNDRV_DEFAULT_STR }; /* ID for this card */
static mut enable: [bool; SNDRV_CARDS] = unsafe { SNDRV_DEFAULT_ENABLE }; /* Enable this card */
static mut port: [c_long; SNDRV_CARDS] = unsafe { SNDRV_DEFAULT_PORT }; /* PnP setup */
static mut irq: [c_int; SNDRV_CARDS] = unsafe { SNDRV_DEFAULT_IRQ }; /* 5,7,9,11,12,15 */
static mut dma1: [c_int; SNDRV_CARDS] = unsafe { SNDRV_DEFAULT_DMA }; /* 0,1,3,5,6,7 */
static mut thinkpad: [bool; SNDRV_CARDS] = [false; SNDRV_CARDS]; /* Thinkpad special case */

/*
 * module_param_array(index, int, NULL, 0444);
 * MODULE_PARM_DESC(index, "Index value for " CRD_NAME " soundcard.");
 * module_param_array(id, charp, NULL, 0444);
 * MODULE_PARM_DESC(id, "ID string for " CRD_NAME " soundcard.");
 * module_param_array(enable, bool, NULL, 0444);
 * MODULE_PARM_DESC(enable, "Enable " CRD_NAME " soundcard.");
 * module_param_hw_array(port, long, ioport, NULL, 0444);
 * MODULE_PARM_DESC(port, "Port # for " CRD_NAME " driver.");
 * module_param_hw_array(irq, int, irq, NULL, 0444);
 * MODULE_PARM_DESC(irq, "IRQ # for " CRD_NAME " driver.");
 * module_param_hw_array(dma1, int, dma, NULL, 0444);
 * MODULE_PARM_DESC(dma1, "DMA1 # for " CRD_NAME " driver.");
 * module_param_array(thinkpad, bool, NULL, 0444);
 * MODULE_PARM_DESC(thinkpad, "Enable only for the onboard CS4248 of IBM Thinkpad 360/750/755 series.");
 */

unsafe extern "C" fn snd_ad1848_match(dev: *mut device, n: c_uint) -> c_int {
    let n = n as usize;

    if !enable[n] {
        return 0;
    }

    if port[n] == SNDRV_AUTO_PORT {
        dev_err(dev, b"please specify port\n\0".as_ptr() as *const c_char);
        return 0;
    }
    if irq[n] == SNDRV_AUTO_IRQ {
        dev_err(dev, b"please specify irq\n\0".as_ptr() as *const c_char);
        return 0;
    }
    if dma1[n] == SNDRV_AUTO_DMA {
        dev_err(dev, b"please specify dma1\n\0".as_ptr() as *const c_char);
        return 0;
    }
    1
}

unsafe extern "C" fn snd_ad1848_probe(dev: *mut device, n: c_uint) -> c_int {
    let mut card: *mut snd_card = core::ptr::null_mut();
    let mut chip: *mut snd_wss = core::ptr::null_mut();
    let mut error: c_int;
    let n = n as usize;

    error = snd_devm_card_new(dev, index[n], id[n], THIS_MODULE, 0, &mut card);
    if error < 0 {
        return error;
    }

    error = snd_wss_create(
        card,
        port[n],
        -1,
        irq[n],
        dma1[n],
        -1,
        if thinkpad[n] { WSS_HW_THINKPAD } else { WSS_HW_DETECT },
        0,
        &mut chip,
    );
    if error < 0 {
        return error;
    }

    (*card).private_data = chip as *mut c_void;

    error = snd_wss_pcm(chip, 0);
    if error < 0 {
        return error;
    }

    error = snd_wss_mixer(chip);
    if error < 0 {
        return error;
    }

    strscpy(
        (*card).driver.as_mut_ptr(),
        b"AD1848\0".as_ptr() as *const c_char,
        core::mem::size_of_val(&(*card).driver),
    );
    strscpy(
        (*card).shortname.as_mut_ptr(),
        (*(*chip).pcm).name,
        core::mem::size_of_val(&(*card).shortname),
    );

    if !thinkpad[n] {
        scnprintf(
            (*card).longname.as_mut_ptr(),
            core::mem::size_of_val(&(*card).longname),
            b"%s at 0x%lx, irq %d, dma %d\0".as_ptr() as *const c_char,
            (*(*chip).pcm).name,
            (*chip).port,
            irq[n],
            dma1[n],
        );
    } else {
        scnprintf(
            (*card).longname.as_mut_ptr(),
            core::mem::size_of_val(&(*card).longname),
            b"%s at 0x%lx, irq %d, dma %d [Thinkpad]\0".as_ptr() as *const c_char,
            (*(*chip).pcm).name,
            (*chip).port,
            irq[n],
            dma1[n],
        );
    }

    error = snd_card_register(card);
    if error < 0 {
        return error;
    }

    dev_set_drvdata(dev, card as *mut c_void);
    0
}

/* CONFIG_PM */
unsafe extern "C" fn snd_ad1848_suspend(
    dev: *mut device,
    _n: c_uint,
    _state: pm_message_t,
) -> c_int {
    let card: *mut snd_card = dev_get_drvdata(dev) as *mut snd_card;
    let chip: *mut snd_wss = (*card).private_data as *mut snd_wss;

    snd_power_change_state(card, SNDRV_CTL_POWER_D3hot);
    if let Some(suspend) = (*chip).suspend {
        suspend(chip);
    }
    0
}

/* CONFIG_PM */
unsafe extern "C" fn snd_ad1848_resume(dev: *mut device, _n: c_uint) -> c_int {
    let card: *mut snd_card = dev_get_drvdata(dev) as *mut snd_card;
    let chip: *mut snd_wss = (*card).private_data as *mut snd_wss;

    if let Some(resume) = (*chip).resume {
        resume(chip);
    }
    snd_power_change_state(card, SNDRV_CTL_POWER_D0);
    0
}

static mut snd_ad1848_driver: isa_driver = isa_driver {
    match_: Some(snd_ad1848_match),
    probe: Some(snd_ad1848_probe),
    /* CONFIG_PM */
    suspend: Some(snd_ad1848_suspend),
    resume: Some(snd_ad1848_resume),
    driver: device_driver {
        name: DEV_NAME.as_ptr() as *const c_char,
    },
};

/*
 * module_isa_driver(snd_ad1848_driver, SNDRV_CARDS);
 */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
