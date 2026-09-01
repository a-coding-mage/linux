// SPDX-License-Identifier: GPL-2.0-or-later
/*
    card-azt2320.c - driver for Aztech Systems AZT2320 based soundcards.
    Copyright (C) 1999-2000 by Massimo Piccioni <dafastidio@libero.it>

*/

/*
    This driver should provide support for most Aztech AZT2320 based cards.
    Several AZT2316 chips are also supported/tested, but autoprobe doesn't
    work: all module option have to be set.

    No docs available for us at Aztech headquarters !!!   Unbelievable ...
    No other help obtained.

    Thanks to Rainer Wiesner <rainer.wiesner@01019freenet.de> for the WSS
    activation method (full-duplex audio!).
*/

/* Original C dependencies:
 * linux/io.h, linux/delay.h, linux/init.h, linux/time.h, linux/wait.h,
 * linux/pnp.h, linux/module.h, sound/core.h, sound/initval.h, sound/wss.h,
 * sound/mpu401.h, sound/opl3.h
 */

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};

extern "C" {
    static mut jiffies: c_ulong;

    static SNDRV_DEFAULT_IDX: [c_int; SNDRV_CARDS];
    static SNDRV_DEFAULT_STR: [*mut c_char; SNDRV_CARDS];
    static SNDRV_DEFAULT_ENABLE_ISAPNP: [bool; SNDRV_CARDS];
    static SNDRV_DEFAULT_PORT: [c_long; SNDRV_CARDS];
    static SNDRV_DEFAULT_IRQ: [c_int; SNDRV_CARDS];
    static SNDRV_DEFAULT_DMA: [c_int; SNDRV_CARDS];

    fn pnp_request_card_device(
        card: *mut pnp_card_link,
        id: *const c_char,
        from: *mut pnp_dev,
    ) -> *mut pnp_dev;
    fn pnp_activate_dev(dev: *mut pnp_dev) -> c_int;
    fn pnp_port_start(dev: *mut pnp_dev, bar: c_uint) -> c_long;
    fn pnp_dma(dev: *mut pnp_dev, bar: c_uint) -> c_int;
    fn pnp_irq(dev: *mut pnp_dev, bar: c_uint) -> c_int;
    fn pnp_release_card_device(dev: *mut pnp_dev);
    fn pnp_set_card_drvdata(pcard: *mut pnp_card_link, data: *mut c_void);
    fn pnp_get_card_drvdata(pcard: *mut pnp_card_link) -> *mut c_void;
    fn pnp_register_card_driver(driver: *mut pnp_card_driver) -> c_int;
    fn pnp_unregister_card_driver(driver: *mut pnp_card_driver);

    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn pr_err(fmt: *const c_char, ...);
    fn inb(port: c_ulong) -> u8;
    fn outb(value: u8, port: c_ulong);
    fn mdelay(msecs: c_ulong);
    fn time_after(a: c_ulong, b: c_ulong) -> bool;

    fn snd_devm_card_new(
        parent: *mut device,
        idx: c_int,
        xid: *mut c_char,
        module: *mut c_void,
        extra_size: usize,
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
    fn strscpy(dst: *mut c_char, src: *const c_char) -> isize;
    fn sprintf(buf: *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn snd_wss_pcm(chip: *mut snd_wss, device: c_int) -> c_int;
    fn snd_wss_mixer(chip: *mut snd_wss) -> c_int;
    fn snd_wss_timer(chip: *mut snd_wss, device: c_int) -> c_int;
    fn snd_mpu401_uart_new(
        card: *mut snd_card,
        device: c_int,
        hardware: c_int,
        port: c_long,
        integrated: c_int,
        irq: c_int,
        info_flags: *mut c_void,
    ) -> c_int;
    fn snd_opl3_create(
        card: *mut snd_card,
        l_port: c_long,
        r_port: c_long,
        hardware: c_int,
        integrated: c_int,
        ropl3: *mut *mut snd_opl3,
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
}

const SNDRV_CARDS: usize = 8;
const ENODEV: c_int = 19;
const EBUSY: c_int = 16;
const HZ: c_ulong = 100;
const SNDRV_AUTO_PORT: c_long = -1;
const WSS_HW_DETECT: c_int = 0;
const MPU401_HW_AZT2320: c_int = 0;
const OPL3_HW_AUTO: c_int = 0;
const PNP_DRIVER_RES_DISABLE: c_int = 1;
const SNDRV_CTL_POWER_D3HOT: c_int = 3;
const SNDRV_CTL_POWER_D0: c_int = 0;
static mut THIS_MODULE: *mut c_void = core::ptr::null_mut();

static mut index: [c_int; SNDRV_CARDS] = unsafe { SNDRV_DEFAULT_IDX }; /* Index 0-MAX */
static mut id: [*mut c_char; SNDRV_CARDS] = unsafe { SNDRV_DEFAULT_STR }; /* ID for this card */
static mut enable: [bool; SNDRV_CARDS] = unsafe { SNDRV_DEFAULT_ENABLE_ISAPNP }; /* Enable this card */
static mut port: [c_long; SNDRV_CARDS] = unsafe { SNDRV_DEFAULT_PORT }; /* PnP setup */
static mut wss_port: [c_long; SNDRV_CARDS] = unsafe { SNDRV_DEFAULT_PORT }; /* PnP setup */
static mut mpu_port: [c_long; SNDRV_CARDS] = unsafe { SNDRV_DEFAULT_PORT }; /* PnP setup */
static mut fm_port: [c_long; SNDRV_CARDS] = unsafe { SNDRV_DEFAULT_PORT }; /* PnP setup */
static mut irq: [c_int; SNDRV_CARDS] = unsafe { SNDRV_DEFAULT_IRQ }; /* Pnp setup */
static mut mpu_irq: [c_int; SNDRV_CARDS] = unsafe { SNDRV_DEFAULT_IRQ }; /* Pnp setup */
static mut dma1: [c_int; SNDRV_CARDS] = unsafe { SNDRV_DEFAULT_DMA }; /* PnP setup */
static mut dma2: [c_int; SNDRV_CARDS] = unsafe { SNDRV_DEFAULT_DMA }; /* PnP setup */

/* module_param_array / MODULE_PARM_DESC declarations are Linux module metadata. */

#[repr(C)]
struct snd_card_azt2320 {
    dev_no: c_int,
    dev: *mut pnp_dev,
    devmpu: *mut pnp_dev,
    chip: *mut snd_wss,
}

#[repr(C)]
struct pnp_card_device_id {
    id: [c_char; 8],
    devs: [pnp_device_id; 2],
}

#[repr(C)]
struct pnp_device_id {
    id: [c_char; 8],
}

#[repr(C)]
struct pnp_card_driver {
    flags: c_int,
    name: *const c_char,
    id_table: *const pnp_card_device_id,
    probe: Option<unsafe extern "C" fn(*mut pnp_card_link, *const pnp_card_device_id) -> c_int>,
    /* CONFIG_PM: suspend/resume fields are present when power management is enabled. */
    suspend: Option<unsafe extern "C" fn(*mut pnp_card_link, pm_message_t) -> c_int>,
    resume: Option<unsafe extern "C" fn(*mut pnp_card_link) -> c_int>,
}

#[repr(C)]
struct pnp_card_link {
    card: *mut pnp_card,
}

#[repr(C)]
struct pnp_card {
    dev: device,
}

#[repr(C)]
struct pnp_dev {
    dev: device,
}

#[repr(C)]
struct device {
    _private: [u8; 0],
}

#[repr(C)]
struct snd_card {
    dev: *mut device,
    private_data: *mut c_void,
    driver: [c_char; 16],
    shortname: [c_char; 32],
    longname: [c_char; 80],
}

#[repr(C)]
struct snd_wss {
    port: c_ulong,
    suspend: Option<unsafe extern "C" fn(*mut snd_wss)>,
    resume: Option<unsafe extern "C" fn(*mut snd_wss)>,
}

#[repr(C)]
struct snd_opl3 {
    _private: [u8; 0],
}

type pm_message_t = c_int;

const fn c8(s: &[u8]) -> [c_char; 8] {
    let mut out = [0 as c_char; 8];
    let mut i = 0;
    while i < s.len() && i < 7 {
        out[i] = s[i] as c_char;
        i += 1;
    }
    out
}

static snd_azt2320_pnpids: [pnp_card_device_id; 7] = [
    /* PRO16V */
    pnp_card_device_id {
        id: c8(b"AZT1008"),
        devs: [pnp_device_id { id: c8(b"AZT1008") }, pnp_device_id { id: c8(b"AZT2001") }],
    },
    /* Aztech Sound Galaxy 16 */
    pnp_card_device_id {
        id: c8(b"AZT2320"),
        devs: [pnp_device_id { id: c8(b"AZT0001") }, pnp_device_id { id: c8(b"AZT0002") }],
    },
    /* Packard Bell Sound III 336 AM/SP */
    pnp_card_device_id {
        id: c8(b"AZT3000"),
        devs: [pnp_device_id { id: c8(b"AZT1003") }, pnp_device_id { id: c8(b"AZT2001") }],
    },
    /* AT3300 */
    pnp_card_device_id {
        id: c8(b"AZT3002"),
        devs: [pnp_device_id { id: c8(b"AZT1004") }, pnp_device_id { id: c8(b"AZT2001") }],
    },
    /* --- */
    pnp_card_device_id {
        id: c8(b"AZT3005"),
        devs: [pnp_device_id { id: c8(b"AZT1003") }, pnp_device_id { id: c8(b"AZT2001") }],
    },
    /* --- */
    pnp_card_device_id {
        id: c8(b"AZT3011"),
        devs: [pnp_device_id { id: c8(b"AZT1003") }, pnp_device_id { id: c8(b"AZT2001") }],
    },
    pnp_card_device_id {
        id: c8(b""),
        devs: [pnp_device_id { id: c8(b"") }, pnp_device_id { id: c8(b"") }],
    }, /* end */
];

/* MODULE_DEVICE_TABLE(pnp_card, snd_azt2320_pnpids); */

const DRIVER_NAME: &[u8] = b"snd-card-azt2320\0";

unsafe extern "C" fn snd_card_azt2320_pnp(
    dev: c_int,
    acard: *mut snd_card_azt2320,
    card: *mut pnp_card_link,
    idp: *const pnp_card_device_id,
) -> c_int {
    let mut pdev: *mut pnp_dev;
    let mut err: c_int;
    let devn = dev as usize;

    (*acard).dev = pnp_request_card_device(card, (*idp).devs[0].id.as_ptr(), core::ptr::null_mut());
    if (*acard).dev.is_null() {
        return -ENODEV;
    }

    (*acard).devmpu =
        pnp_request_card_device(card, (*idp).devs[1].id.as_ptr(), core::ptr::null_mut());

    pdev = (*acard).dev;

    err = pnp_activate_dev(pdev);
    if err < 0 {
        dev_err(
            &mut (*pdev).dev,
            b"AUDIO pnp configure failure\n\0".as_ptr() as *const c_char,
        );
        return err;
    }
    port[devn] = pnp_port_start(pdev, 0);
    fm_port[devn] = pnp_port_start(pdev, 1);
    wss_port[devn] = pnp_port_start(pdev, 2);
    dma1[devn] = pnp_dma(pdev, 0);
    dma2[devn] = pnp_dma(pdev, 1);
    irq[devn] = pnp_irq(pdev, 0);

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
            (*acard).devmpu = core::ptr::null_mut();
            mpu_port[devn] = -1;
        } else {
            mpu_port[devn] = pnp_port_start(pdev, 0);
            mpu_irq[devn] = pnp_irq(pdev, 0);
        }
    } else {
        (*acard).devmpu = core::ptr::null_mut();
        mpu_port[devn] = -1;
    }

    0
}

/* same of snd_sbdsp_command by Jaroslav Kysela */
unsafe extern "C" fn snd_card_azt2320_command(port: c_ulong, val: u8) -> c_int {
    let mut i: c_int;
    let limit: c_ulong;

    limit = jiffies.wrapping_add(HZ / 10);
    i = 50000;
    while i != 0 && time_after(limit, jiffies) {
        if (inb(port.wrapping_add(0x0c)) & 0x80) == 0 {
            outb(val, port.wrapping_add(0x0c));
            return 0;
        }
        i -= 1;
    }
    -EBUSY
}

unsafe extern "C" fn snd_card_azt2320_enable_wss(port: c_ulong) -> c_int {
    let mut error: c_int;

    error = snd_card_azt2320_command(port, 0x09);
    if error != 0 {
        return error;
    }
    error = snd_card_azt2320_command(port, 0x00);
    if error != 0 {
        return error;
    }

    mdelay(5);
    0
}

unsafe extern "C" fn snd_card_azt2320_probe(
    dev: c_int,
    pcard: *mut pnp_card_link,
    pid: *const pnp_card_device_id,
) -> c_int {
    let mut error: c_int;
    let mut card: *mut snd_card = core::ptr::null_mut();
    let mut acard: *mut snd_card_azt2320;
    let mut chip: *mut snd_wss = core::ptr::null_mut();
    let mut opl3: *mut snd_opl3 = core::ptr::null_mut();
    let devn = dev as usize;

    error = snd_devm_card_new(
        &mut (*(*pcard).card).dev,
        index[devn],
        id[devn],
        THIS_MODULE,
        core::mem::size_of::<snd_card_azt2320>(),
        &mut card,
    );
    if error < 0 {
        return error;
    }
    acard = (*card).private_data as *mut snd_card_azt2320;

    error = snd_card_azt2320_pnp(dev, acard, pcard, pid);
    if error != 0 {
        return error;
    }

    error = snd_card_azt2320_enable_wss(port[devn] as c_ulong);
    if error != 0 {
        return error;
    }

    error = snd_wss_create(
        card,
        wss_port[devn],
        -1,
        irq[devn],
        dma1[devn],
        dma2[devn],
        WSS_HW_DETECT,
        0,
        &mut chip,
    );
    if error < 0 {
        return error;
    }

    strscpy((*card).driver.as_mut_ptr(), b"AZT2320\0".as_ptr() as *const c_char);
    strscpy(
        (*card).shortname.as_mut_ptr(),
        b"Aztech AZT2320\0".as_ptr() as *const c_char,
    );
    sprintf(
        (*card).longname.as_mut_ptr(),
        b"%s, WSS at 0x%lx, irq %i, dma %i&%i\0".as_ptr() as *const c_char,
        (*card).shortname.as_ptr(),
        (*chip).port,
        irq[devn],
        dma1[devn],
        dma2[devn],
    );

    error = snd_wss_pcm(chip, 0);
    if error < 0 {
        return error;
    }
    error = snd_wss_mixer(chip);
    if error < 0 {
        return error;
    }
    error = snd_wss_timer(chip, 0);
    if error < 0 {
        return error;
    }

    if mpu_port[devn] > 0 && mpu_port[devn] != SNDRV_AUTO_PORT {
        if snd_mpu401_uart_new(
            card,
            0,
            MPU401_HW_AZT2320,
            mpu_port[devn],
            0,
            mpu_irq[devn],
            core::ptr::null_mut(),
        ) < 0
        {
            dev_err(
                (*card).dev,
                b"no MPU-401 device at 0x%lx\n\0".as_ptr() as *const c_char,
                mpu_port[devn],
            );
        }
    }

    if fm_port[devn] > 0 && fm_port[devn] != SNDRV_AUTO_PORT {
        if snd_opl3_create(
            card,
            fm_port[devn],
            fm_port[devn] + 2,
            OPL3_HW_AUTO,
            0,
            &mut opl3,
        ) < 0
        {
            dev_err(
                (*card).dev,
                b"no OPL device at 0x%lx-0x%lx\n\0".as_ptr() as *const c_char,
                fm_port[devn],
                fm_port[devn] + 2,
            );
        } else {
            error = snd_opl3_timer_new(opl3, 1, 2);
            if error < 0 {
                return error;
            }
            error = snd_opl3_hwdep_new(opl3, 0, 1, core::ptr::null_mut());
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

static mut azt2320_devices: c_uint = 0;

unsafe extern "C" fn snd_azt2320_pnp_detect(
    card: *mut pnp_card_link,
    idp: *const pnp_card_device_id,
) -> c_int {
    static mut DEV: c_int = 0;
    let mut res: c_int;

    while DEV < SNDRV_CARDS as c_int {
        if !enable[DEV as usize] {
            DEV += 1;
            continue;
        }
        res = snd_card_azt2320_probe(DEV, card, idp);
        if res < 0 {
            return res;
        }
        DEV += 1;
        azt2320_devices += 1;
        return 0;
    }
    -ENODEV
}

/* CONFIG_PM */
unsafe extern "C" fn snd_azt2320_pnp_suspend(
    pcard: *mut pnp_card_link,
    _state: pm_message_t,
) -> c_int {
    let card = pnp_get_card_drvdata(pcard) as *mut snd_card;
    let acard = (*card).private_data as *mut snd_card_azt2320;
    let chip = (*acard).chip;

    snd_power_change_state(card, SNDRV_CTL_POWER_D3HOT);
    if let Some(suspend) = (*chip).suspend {
        suspend(chip);
    }
    0
}

unsafe extern "C" fn snd_azt2320_pnp_resume(pcard: *mut pnp_card_link) -> c_int {
    let card = pnp_get_card_drvdata(pcard) as *mut snd_card;
    let acard = (*card).private_data as *mut snd_card_azt2320;
    let chip = (*acard).chip;

    if let Some(resume) = (*chip).resume {
        resume(chip);
    }
    snd_power_change_state(card, SNDRV_CTL_POWER_D0);
    0
}
/* end CONFIG_PM */

static mut azt2320_pnpc_driver: pnp_card_driver = pnp_card_driver {
    flags: PNP_DRIVER_RES_DISABLE,
    name: b"azt2320\0".as_ptr() as *const c_char,
    id_table: snd_azt2320_pnpids.as_ptr(),
    probe: Some(snd_azt2320_pnp_detect),
    /* CONFIG_PM */
    suspend: Some(snd_azt2320_pnp_suspend),
    resume: Some(snd_azt2320_pnp_resume),
};

unsafe extern "C" fn alsa_card_azt2320_init() -> c_int {
    let err: c_int;

    err = pnp_register_card_driver(&mut azt2320_pnpc_driver);
    if err != 0 {
        return err;
    }

    if azt2320_devices == 0 {
        pnp_unregister_card_driver(&mut azt2320_pnpc_driver);
        /* MODULE */
        pr_err(b"no AZT2320 based soundcards found\n\0".as_ptr() as *const c_char);
        /* end MODULE */
        return -ENODEV;
    }
    0
}

unsafe extern "C" fn alsa_card_azt2320_exit() {
    pnp_unregister_card_driver(&mut azt2320_pnpc_driver);
}

/* module_init(alsa_card_azt2320_init) */
/* module_exit(alsa_card_azt2320_exit) */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
