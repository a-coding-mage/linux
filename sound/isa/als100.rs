// SPDX-License-Identifier: GPL-2.0-or-later

/*
    card-als100.c - driver for Avance Logic ALS100 based soundcards.
    Copyright (C) 1999-2000 by Massimo Piccioni <dafastidio@libero.it>
    Copyright (C) 1999-2002 by Massimo Piccioni <dafastidio@libero.it>

    Thanks to Pierfrancesco 'qM2' Passerini.

    Generalised for soundcards based on DT-0196 and ALS-007 chips
    by Jonathan Woithe <jwoithe@just42.net>: June 2002.

*/

// Dependencies originally included from Linux ALSA/PNP headers:
// linux/init.h, linux/wait.h, linux/time.h, linux/pnp.h, linux/module.h,
// sound/core.h, sound/initval.h, sound/mpu401.h, sound/opl3.h, sound/sb.h.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::ptr;

extern "C" {
    static THIS_MODULE: *mut c_void;

    static mut SNDRV_DEFAULT_IDX: [c_int; SNDRV_CARDS];
    static mut SNDRV_DEFAULT_STR: [*mut c_char; SNDRV_CARDS];
    static mut SNDRV_DEFAULT_ENABLE: [bool; SNDRV_CARDS];
    static mut SNDRV_DEFAULT_PORT: [c_long; SNDRV_CARDS];
    static mut SNDRV_DEFAULT_IRQ: [c_int; SNDRV_CARDS];
    static mut SNDRV_DEFAULT_DMA: [c_int; SNDRV_CARDS];

    fn pnp_request_card_device(
        card: *mut pnp_card_link,
        id: *const c_char,
        from: *mut pnp_dev,
    ) -> *mut pnp_dev;
    fn pnp_activate_dev(dev: *mut pnp_dev) -> c_int;
    fn pnp_port_start(dev: *mut pnp_dev, bar: c_uint) -> c_long;
    fn pnp_dma(dev: *mut pnp_dev, n: c_uint) -> c_int;
    fn pnp_irq(dev: *mut pnp_dev, n: c_uint) -> c_int;
    fn pnp_release_card_device(dev: *mut pnp_dev);
    fn pnp_set_card_drvdata(pcard: *mut pnp_card_link, data: *mut c_void);
    fn pnp_get_card_drvdata(pcard: *mut pnp_card_link) -> *mut c_void;
    fn pnp_register_card_driver(driver: *mut pnp_card_driver) -> c_int;
    fn pnp_unregister_card_driver(driver: *mut pnp_card_driver);

    fn snd_devm_card_new(
        parent: *mut device,
        idx: c_int,
        xid: *mut c_char,
        module: *mut c_void,
        extra_size: usize,
        card_ret: *mut *mut snd_card,
    ) -> c_int;
    fn snd_sbdsp_create(
        card: *mut snd_card,
        port: c_long,
        irq: c_int,
        irq_handler: Option<unsafe extern "C" fn()>,
        dma8: c_int,
        dma16: c_int,
        hardware: c_ulong,
        chip_ret: *mut *mut snd_sb,
    ) -> c_int;
    fn snd_sb16dsp_interrupt();
    fn snd_sb16dsp_pcm(chip: *mut snd_sb, device: c_int) -> c_int;
    fn snd_sbmixer_new(chip: *mut snd_sb) -> c_int;
    fn snd_mpu401_uart_new(
        card: *mut snd_card,
        device: c_int,
        hardware: c_int,
        port: c_long,
        integrated: c_int,
        irq: c_int,
        rrawmidi: *mut c_void,
    ) -> c_int;
    fn snd_opl3_create(
        card: *mut snd_card,
        l_port: c_long,
        r_port: c_long,
        hardware: c_int,
        integrated: c_int,
        opl3_ret: *mut *mut snd_opl3,
    ) -> c_int;
    fn snd_opl3_timer_new(opl3: *mut snd_opl3, timer1_dev: c_int, timer2_dev: c_int) -> c_int;
    fn snd_opl3_hwdep_new(
        opl3: *mut snd_opl3,
        device: c_int,
        seq_device: c_int,
        rhwdep: *mut c_void,
    ) -> c_int;
    fn snd_card_register(card: *mut snd_card) -> c_int;

    fn snd_power_change_state(card: *mut snd_card, state: c_int);
    fn snd_sbmixer_suspend(chip: *mut snd_sb);
    fn snd_sbdsp_reset(chip: *mut snd_sb);
    fn snd_sbmixer_resume(chip: *mut snd_sb);

    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn pr_err(fmt: *const c_char, ...);
    fn strscpy(dest: *mut c_char, src: *const c_char) -> isize;
    fn snprintf(buf: *mut c_char, size: usize, fmt: *const c_char, ...) -> c_int;
}

const SNDRV_CARDS: usize = 8;
const ENODEV: c_int = 19;
const SB_HW_DT019X: c_ulong = 0;
const SB_HW_ALS100: c_ulong = 1;
const SNDRV_AUTO_PORT: c_long = -1;
const SNDRV_AUTO_IRQ: c_int = -1;
const MPU401_HW_ALS100: c_int = 0;
const MPU401_HW_MPU401: c_int = 1;
const OPL3_HW_AUTO: c_int = 0;
const SNDRV_CTL_POWER_D3hot: c_int = 0;
const SNDRV_CTL_POWER_D0: c_int = 0;
const PNP_DRIVER_RES_DISABLE: c_uint = 0;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pnp_dev {
    pub dev: device,
}

#[repr(C)]
pub struct pnp_card {
    pub dev: device,
}

#[repr(C)]
pub struct pnp_card_link {
    pub card: *mut pnp_card,
}

#[repr(C)]
pub struct pnp_id {
    pub id: [c_char; 8],
}

#[repr(C)]
pub struct pnp_card_device_id {
    pub id: [c_char; 8],
    pub devs: [pnp_id; 3],
    pub driver_data: c_ulong,
}

#[repr(C)]
pub struct pnp_card_driver {
    pub flags: c_uint,
    pub name: *const c_char,
    pub id_table: *const pnp_card_device_id,
    pub probe:
        Option<unsafe extern "C" fn(*mut pnp_card_link, *const pnp_card_device_id) -> c_int>,
    // CONFIG_PM: suspend/resume fields are present when power management is enabled.
    pub suspend: Option<unsafe extern "C" fn(*mut pnp_card_link, pm_message_t) -> c_int>,
    pub resume: Option<unsafe extern "C" fn(*mut pnp_card_link) -> c_int>,
}

#[repr(C)]
pub struct snd_sb {
    pub name: *mut c_char,
    pub port: c_ulong,
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
pub struct snd_opl3 {
    _private: [u8; 0],
}

pub type pm_message_t = c_int;

// MODULE_DESCRIPTION("Avance Logic ALS007/ALS1X0");
// MODULE_AUTHOR("Massimo Piccioni <dafastidio@libero.it>");
// MODULE_LICENSE("GPL");

static mut index: [c_int; SNDRV_CARDS] = unsafe { SNDRV_DEFAULT_IDX };
static mut id: [*mut c_char; SNDRV_CARDS] = unsafe { SNDRV_DEFAULT_STR };
static mut enable: [bool; SNDRV_CARDS] = unsafe { SNDRV_DEFAULT_ENABLE };
static mut port: [c_long; SNDRV_CARDS] = unsafe { SNDRV_DEFAULT_PORT };
static mut mpu_port: [c_long; SNDRV_CARDS] = unsafe { SNDRV_DEFAULT_PORT };
static mut fm_port: [c_long; SNDRV_CARDS] = unsafe { SNDRV_DEFAULT_PORT };
static mut irq: [c_int; SNDRV_CARDS] = unsafe { SNDRV_DEFAULT_IRQ };
static mut mpu_irq: [c_int; SNDRV_CARDS] = unsafe { SNDRV_DEFAULT_IRQ };
static mut dma8: [c_int; SNDRV_CARDS] = unsafe { SNDRV_DEFAULT_DMA };
static mut dma16: [c_int; SNDRV_CARDS] = unsafe { SNDRV_DEFAULT_DMA };

// module_param_array and MODULE_PARM_DESC declarations are preserved as module metadata intent.
// MODULE_ALIAS("snd-dt019x");

#[repr(C)]
pub struct snd_card_als100 {
    pub dev: *mut pnp_dev,
    pub devmpu: *mut pnp_dev,
    pub devopl: *mut pnp_dev,
    pub chip: *mut snd_sb,
}

const fn c8(s: &[u8]) -> [c_char; 8] {
    let mut out = [0 as c_char; 8];
    let mut i = 0;
    while i < s.len() && i < 8 {
        out[i] = s[i] as c_char;
        i += 1;
    }
    out
}

static snd_als100_pnpids: [pnp_card_device_id; 9] = [
    /* DT197A30 */
    pnp_card_device_id {
        id: c8(b"RWB1688"),
        devs: [pnp_id { id: c8(b"@@@0001") }, pnp_id { id: c8(b"@X@0001") }, pnp_id { id: c8(b"@H@0001") }],
        driver_data: SB_HW_DT019X,
    },
    /* DT0196 / ALS-007 */
    pnp_card_device_id {
        id: c8(b"ALS0007"),
        devs: [pnp_id { id: c8(b"@@@0001") }, pnp_id { id: c8(b"@X@0001") }, pnp_id { id: c8(b"@H@0001") }],
        driver_data: SB_HW_DT019X,
    },
    /* ALS100 - PRO16PNP */
    pnp_card_device_id {
        id: c8(b"ALS0001"),
        devs: [pnp_id { id: c8(b"@@@0001") }, pnp_id { id: c8(b"@X@0001") }, pnp_id { id: c8(b"@H@0001") }],
        driver_data: SB_HW_ALS100,
    },
    /* ALS110 - MF1000 - Digimate 3D Sound */
    pnp_card_device_id {
        id: c8(b"ALS0110"),
        devs: [pnp_id { id: c8(b"@@@1001") }, pnp_id { id: c8(b"@X@1001") }, pnp_id { id: c8(b"@H@1001") }],
        driver_data: SB_HW_ALS100,
    },
    /* ALS120 */
    pnp_card_device_id {
        id: c8(b"ALS0120"),
        devs: [pnp_id { id: c8(b"@@@2001") }, pnp_id { id: c8(b"@X@2001") }, pnp_id { id: c8(b"@H@2001") }],
        driver_data: SB_HW_ALS100,
    },
    /* ALS200 */
    pnp_card_device_id {
        id: c8(b"ALS0200"),
        devs: [pnp_id { id: c8(b"@@@0020") }, pnp_id { id: c8(b"@X@0020") }, pnp_id { id: c8(b"@H@0001") }],
        driver_data: SB_HW_ALS100,
    },
    /* ALS200 OEM */
    pnp_card_device_id {
        id: c8(b"ALS0200"),
        devs: [pnp_id { id: c8(b"@@@0020") }, pnp_id { id: c8(b"@X@0020") }, pnp_id { id: c8(b"@H@0020") }],
        driver_data: SB_HW_ALS100,
    },
    /* RTL3000 */
    pnp_card_device_id {
        id: c8(b"RTL3000"),
        devs: [pnp_id { id: c8(b"@@@2001") }, pnp_id { id: c8(b"@X@2001") }, pnp_id { id: c8(b"@H@2001") }],
        driver_data: SB_HW_ALS100,
    },
    pnp_card_device_id {
        id: c8(b""),
        devs: [pnp_id { id: c8(b"") }, pnp_id { id: c8(b"") }, pnp_id { id: c8(b"") }],
        driver_data: 0,
    }, /* end */
];

// MODULE_DEVICE_TABLE(pnp_card, snd_als100_pnpids);

unsafe extern "C" fn snd_card_als100_pnp(
    dev: c_int,
    acard: *mut snd_card_als100,
    card: *mut pnp_card_link,
    idp: *const pnp_card_device_id,
) -> c_int {
    let mut pdev: *mut pnp_dev;
    let mut err: c_int;
    let dev_usize = dev as usize;

    (*acard).dev = pnp_request_card_device(card, (*idp).devs[0].id.as_ptr(), ptr::null_mut());
    if (*acard).dev.is_null() {
        return -ENODEV;
    }

    (*acard).devmpu =
        pnp_request_card_device(card, (*idp).devs[1].id.as_ptr(), (*acard).dev);
    (*acard).devopl =
        pnp_request_card_device(card, (*idp).devs[2].id.as_ptr(), (*acard).dev);

    pdev = (*acard).dev;

    err = pnp_activate_dev(pdev);
    if err < 0 {
        dev_err(&mut (*pdev).dev, b"AUDIO pnp configure failure\n\0".as_ptr() as *const c_char);
        return err;
    }
    port[dev_usize] = pnp_port_start(pdev, 0);
    if (*idp).driver_data == SB_HW_DT019X {
        dma8[dev_usize] = pnp_dma(pdev, 0);
    } else {
        dma8[dev_usize] = pnp_dma(pdev, 1);
        dma16[dev_usize] = pnp_dma(pdev, 0);
    }
    irq[dev_usize] = pnp_irq(pdev, 0);

    pdev = (*acard).devmpu;
    if !pdev.is_null() {
        err = pnp_activate_dev(pdev);
        if err < 0 {
            if !pdev.is_null() {
                pnp_release_card_device(pdev);
                dev_err(
                    &mut (*pdev).dev,
                    b"MPU401 pnp configure failure, skipping\n\0".as_ptr() as *const c_char,
                );
            }
            (*acard).devmpu = ptr::null_mut();
            mpu_port[dev_usize] = -1;
        } else {
            mpu_port[dev_usize] = pnp_port_start(pdev, 0);
            mpu_irq[dev_usize] = pnp_irq(pdev, 0);
        }
    } else {
        (*acard).devmpu = ptr::null_mut();
        mpu_port[dev_usize] = -1;
    }

    pdev = (*acard).devopl;
    if !pdev.is_null() {
        err = pnp_activate_dev(pdev);
        if err < 0 {
            if !pdev.is_null() {
                pnp_release_card_device(pdev);
                dev_err(
                    &mut (*pdev).dev,
                    b"OPL3 pnp configure failure, skipping\n\0".as_ptr() as *const c_char,
                );
            }
            (*acard).devopl = ptr::null_mut();
            fm_port[dev_usize] = -1;
        } else {
            fm_port[dev_usize] = pnp_port_start(pdev, 0);
        }
    } else {
        (*acard).devopl = ptr::null_mut();
        fm_port[dev_usize] = -1;
    }

    0
}

unsafe extern "C" fn snd_card_als100_probe(
    dev: c_int,
    pcard: *mut pnp_card_link,
    pid: *const pnp_card_device_id,
) -> c_int {
    let mut error: c_int;
    let mut chip: *mut snd_sb = ptr::null_mut();
    let mut card: *mut snd_card = ptr::null_mut();
    let mut acard: *mut snd_card_als100;
    let mut opl3: *mut snd_opl3 = ptr::null_mut();
    let dev_usize = dev as usize;

    error = snd_devm_card_new(
        &mut (*(*pcard).card).dev,
        index[dev_usize],
        id[dev_usize],
        THIS_MODULE,
        core::mem::size_of::<snd_card_als100>(),
        &mut card,
    );
    if error < 0 {
        return error;
    }
    acard = (*card).private_data as *mut snd_card_als100;

    error = snd_card_als100_pnp(dev, acard, pcard, pid);
    if error != 0 {
        return error;
    }

    if (*pid).driver_data == SB_HW_DT019X {
        dma16[dev_usize] = -1;
    }

    error = snd_sbdsp_create(
        card,
        port[dev_usize],
        irq[dev_usize],
        Some(core::mem::transmute(snd_sb16dsp_interrupt as unsafe extern "C" fn())),
        dma8[dev_usize],
        dma16[dev_usize],
        (*pid).driver_data,
        &mut chip,
    );
    if error < 0 {
        return error;
    }
    (*acard).chip = chip;

    if (*pid).driver_data == SB_HW_DT019X {
        strscpy((*card).driver.as_mut_ptr(), b"DT-019X\0".as_ptr() as *const c_char);
        strscpy(
            (*card).shortname.as_mut_ptr(),
            b"Diamond Tech. DT-019X\0".as_ptr() as *const c_char,
        );
        snprintf(
            (*card).longname.as_mut_ptr(),
            (*card).longname.len(),
            b"Diamond Tech. DT-019X, %s at 0x%lx, irq %d, dma %d\0".as_ptr() as *const c_char,
            (*chip).name,
            (*chip).port,
            irq[dev_usize],
            dma8[dev_usize],
        );
    } else {
        strscpy((*card).driver.as_mut_ptr(), b"ALS100\0".as_ptr() as *const c_char);
        strscpy(
            (*card).shortname.as_mut_ptr(),
            b"Avance Logic ALS100\0".as_ptr() as *const c_char,
        );
        snprintf(
            (*card).longname.as_mut_ptr(),
            (*card).longname.len(),
            b"Avance Logic ALS100, %s at 0x%lx, irq %d, dma %d&%d\0".as_ptr() as *const c_char,
            (*chip).name,
            (*chip).port,
            irq[dev_usize],
            dma8[dev_usize],
            dma16[dev_usize],
        );
    }

    error = snd_sb16dsp_pcm(chip, 0);
    if error < 0 {
        return error;
    }

    error = snd_sbmixer_new(chip);
    if error < 0 {
        return error;
    }

    if mpu_port[dev_usize] > 0 && mpu_port[dev_usize] != SNDRV_AUTO_PORT {
        let mut mpu_type = MPU401_HW_ALS100;

        if mpu_irq[dev_usize] == SNDRV_AUTO_IRQ {
            mpu_irq[dev_usize] = -1;
        }

        if (*pid).driver_data == SB_HW_DT019X {
            mpu_type = MPU401_HW_MPU401;
        }

        if snd_mpu401_uart_new(
            card,
            0,
            mpu_type,
            mpu_port[dev_usize],
            0,
            mpu_irq[dev_usize],
            ptr::null_mut(),
        ) < 0
        {
            dev_err(
                (*card).dev,
                b"no MPU-401 device at 0x%lx\n\0".as_ptr() as *const c_char,
                mpu_port[dev_usize],
            );
        }
    }

    if fm_port[dev_usize] > 0 && fm_port[dev_usize] != SNDRV_AUTO_PORT {
        if snd_opl3_create(
            card,
            fm_port[dev_usize],
            fm_port[dev_usize] + 2,
            OPL3_HW_AUTO,
            0,
            &mut opl3,
        ) < 0
        {
            dev_err(
                (*card).dev,
                b"no OPL device at 0x%lx-0x%lx\n\0".as_ptr() as *const c_char,
                fm_port[dev_usize],
                fm_port[dev_usize] + 2,
            );
        } else {
            error = snd_opl3_timer_new(opl3, 0, 1);
            if error < 0 {
                return error;
            }
            error = snd_opl3_hwdep_new(opl3, 0, 1, ptr::null_mut());
            if error < 0 {
                return error;
            }
        }
    }

    error = snd_card_register(card);
    if error < 0 {
        return error;
    }
    pnp_set_card_drvdata(pcard, card as *mut c_void);
    0
}

static mut als100_devices: c_uint = 0;

unsafe extern "C" fn snd_als100_pnp_detect(
    card: *mut pnp_card_link,
    idp: *const pnp_card_device_id,
) -> c_int {
    static mut dev: c_int = 0;
    let mut res: c_int;

    while dev < SNDRV_CARDS as c_int {
        if !enable[dev as usize] {
            dev += 1;
            continue;
        }
        res = snd_card_als100_probe(dev, card, idp);
        if res < 0 {
            return res;
        }
        dev += 1;
        als100_devices += 1;
        return 0;
    }
    -ENODEV
}

// CONFIG_PM
unsafe extern "C" fn snd_als100_pnp_suspend(
    pcard: *mut pnp_card_link,
    _state: pm_message_t,
) -> c_int {
    let card = pnp_get_card_drvdata(pcard) as *mut snd_card;
    let acard = (*card).private_data as *mut snd_card_als100;
    let chip = (*acard).chip;

    snd_power_change_state(card, SNDRV_CTL_POWER_D3hot);
    snd_sbmixer_suspend(chip);
    0
}

unsafe extern "C" fn snd_als100_pnp_resume(pcard: *mut pnp_card_link) -> c_int {
    let card = pnp_get_card_drvdata(pcard) as *mut snd_card;
    let acard = (*card).private_data as *mut snd_card_als100;
    let chip = (*acard).chip;

    snd_sbdsp_reset(chip);
    snd_sbmixer_resume(chip);
    snd_power_change_state(card, SNDRV_CTL_POWER_D0);
    0
}

static mut als100_pnpc_driver: pnp_card_driver = pnp_card_driver {
    flags: PNP_DRIVER_RES_DISABLE,
    name: b"als100\0".as_ptr() as *const c_char,
    id_table: snd_als100_pnpids.as_ptr(),
    probe: Some(snd_als100_pnp_detect),
    // CONFIG_PM
    suspend: Some(snd_als100_pnp_suspend),
    resume: Some(snd_als100_pnp_resume),
};

unsafe extern "C" fn alsa_card_als100_init() -> c_int {
    let mut err: c_int;

    err = pnp_register_card_driver(&mut als100_pnpc_driver);
    if err != 0 {
        return err;
    }

    if als100_devices == 0 {
        pnp_unregister_card_driver(&mut als100_pnpc_driver);
        // MODULE
        pr_err(b"no Avance Logic based soundcards found\n\0".as_ptr() as *const c_char);
        return -ENODEV;
    }
    0
}

unsafe extern "C" fn alsa_card_als100_exit() {
    pnp_unregister_card_driver(&mut als100_pnpc_driver);
}

// module_init(alsa_card_als100_init)
// module_exit(alsa_card_als100_exit)

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
