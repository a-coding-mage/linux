// SPDX-License-Identifier: GPL-2.0-or-later

/*
    card-ad1816a.c - driver for ADI SoundPort AD1816A based soundcards.
    Copyright (C) 2000 by Massimo Piccioni <dafastidio@libero.it>

*/

// C includes translated as external dependency intent:
// linux/init.h, linux/time.h, linux/wait.h, linux/pnp.h, linux/module.h,
// sound/core.h, sound/initval.h, sound/ad1816a.h, sound/mpu401.h, sound/opl3.h

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};

// MODULE_AUTHOR("Massimo Piccioni <dafastidio@libero.it>");
// MODULE_DESCRIPTION("AD1816A, AD1815");
// MODULE_LICENSE("GPL");

extern "C" {
    static THIS_MODULE: *mut c_void;

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
    fn pnp_port_start(dev: *mut pnp_dev, bar: c_uint) -> c_ulong;
    fn pnp_dma(dev: *mut pnp_dev, n: c_uint) -> c_int;
    fn pnp_irq(dev: *mut pnp_dev, n: c_uint) -> c_int;
    fn pnp_set_card_drvdata(card: *mut pnp_card_link, data: *mut c_void);
    fn pnp_get_card_drvdata(card: *mut pnp_card_link) -> *mut c_void;
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
    fn snd_ad1816a_create(
        card: *mut snd_card,
        port: c_long,
        irq: c_int,
        dma1: c_int,
        dma2: c_int,
        chip: *mut snd_ad1816a,
    ) -> c_int;
    fn snd_ad1816a_pcm(chip: *mut snd_ad1816a, device: c_int) -> c_int;
    fn snd_ad1816a_mixer(chip: *mut snd_ad1816a) -> c_int;
    fn snd_ad1816a_timer(chip: *mut snd_ad1816a, device: c_int) -> c_int;
    fn snd_mpu401_uart_new(
        card: *mut snd_card,
        device: c_int,
        hardware: c_int,
        port: c_long,
        integrated: c_int,
        irq: c_int,
        rrawmidi: *mut *mut c_void,
    ) -> c_int;
    fn snd_opl3_create(
        card: *mut snd_card,
        l_port: c_long,
        r_port: c_long,
        hardware: c_int,
        integrated: c_int,
        ropl3: *mut *mut snd_opl3,
    ) -> c_int;
    fn snd_opl3_hwdep_new(
        opl3: *mut snd_opl3,
        device: c_int,
        seq_device: c_int,
        rhwdep: *mut *mut c_void,
    ) -> c_int;
    fn snd_card_register(card: *mut snd_card) -> c_int;
    fn snd_power_change_state(card: *mut snd_card, state: c_int);
    fn snd_ad1816a_suspend(chip: *mut c_void);
    fn snd_ad1816a_resume(chip: *mut c_void);

    fn strscpy(dest: *mut c_char, src: *const c_char) -> isize;
    fn sprintf(buf: *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn pr_warn(fmt: *const c_char, ...);
    fn pr_err(fmt: *const c_char, ...);
}

extern "C" {
    static SNDRV_CARDS: usize;
    static EBUSY: c_int;
    static ENODEV: c_int;
    static MPU401_HW_MPU401: c_int;
    static OPL3_HW_AUTO: c_int;
    static PNP_DRIVER_RES_DISABLE: c_uint;
    static SNDRV_CTL_POWER_D3hot: c_int;
    static SNDRV_CTL_POWER_D0: c_int;
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
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
pub struct pnp_dev {
    pub dev: device,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pnp_device_id {
    pub id: *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pnp_card_device_id {
    pub id: *const c_char,
    pub devs: [pnp_device_id; 2],
}

#[repr(C)]
pub struct pnp_card_driver {
    pub flags: c_uint,
    pub name: *const c_char,
    pub id_table: *const pnp_card_device_id,
    pub probe: Option<
        unsafe extern "C" fn(*mut pnp_card_link, *const pnp_card_device_id) -> c_int,
    >,
    // Present when CONFIG_PM is enabled in the original C build.
    pub suspend: Option<unsafe extern "C" fn(*mut pnp_card_link, pm_message_t) -> c_int>,
    pub resume: Option<unsafe extern "C" fn(*mut pnp_card_link) -> c_int>,
}

pub type pm_message_t = c_int;

#[repr(C)]
pub struct snd_card {
    pub dev: *mut device,
    pub private_data: *mut c_void,
    pub driver: [c_char; 16],
    pub shortname: [c_char; 32],
    pub longname: [c_char; 80],
}

#[repr(C)]
pub struct snd_ad1816a {
    pub port: c_ulong,
    pub clock_freq: c_int,
}

#[repr(C)]
pub struct snd_opl3 {
    _private: [u8; 0],
}

static mut index: [c_int; SNDRV_CARDS] = unsafe { SNDRV_DEFAULT_IDX }; /* Index 1-MAX */
static mut id: [*mut c_char; SNDRV_CARDS] = unsafe { SNDRV_DEFAULT_STR }; /* ID for this card */
static mut enable: [bool; SNDRV_CARDS] = unsafe { SNDRV_DEFAULT_ENABLE_ISAPNP }; /* Enable this card */
static mut port: [c_long; SNDRV_CARDS] = unsafe { SNDRV_DEFAULT_PORT }; /* PnP setup */
static mut mpu_port: [c_long; SNDRV_CARDS] = unsafe { SNDRV_DEFAULT_PORT }; /* PnP setup */
static mut fm_port: [c_long; SNDRV_CARDS] = unsafe { SNDRV_DEFAULT_PORT }; /* PnP setup */
static mut irq: [c_int; SNDRV_CARDS] = unsafe { SNDRV_DEFAULT_IRQ }; /* Pnp setup */
static mut mpu_irq: [c_int; SNDRV_CARDS] = unsafe { SNDRV_DEFAULT_IRQ }; /* Pnp setup */
static mut dma1: [c_int; SNDRV_CARDS] = unsafe { SNDRV_DEFAULT_DMA }; /* PnP setup */
static mut dma2: [c_int; SNDRV_CARDS] = unsafe { SNDRV_DEFAULT_DMA }; /* PnP setup */
static mut clockfreq: [c_int; SNDRV_CARDS] = [0; SNDRV_CARDS];

// module_param_array(index, int, NULL, 0444);
// MODULE_PARM_DESC(index, "Index value for ad1816a based soundcard.");
// module_param_array(id, charp, NULL, 0444);
// MODULE_PARM_DESC(id, "ID string for ad1816a based soundcard.");
// module_param_array(enable, bool, NULL, 0444);
// MODULE_PARM_DESC(enable, "Enable ad1816a based soundcard.");
// module_param_array(clockfreq, int, NULL, 0444);
// MODULE_PARM_DESC(clockfreq, "Clock frequency for ad1816a driver (default = 0).");

static mut snd_ad1816a_pnpids: [pnp_card_device_id; 11] = [
    /* Analog Devices AD1815 */
    pnp_card_device_id { id: b"ADS7150\0".as_ptr() as *const c_char, devs: [pnp_device_id { id: b"ADS7150\0".as_ptr() as *const c_char }, pnp_device_id { id: b"ADS7151\0".as_ptr() as *const c_char }] },
    /* Analog Devices AD1816? */
    pnp_card_device_id { id: b"ADS7180\0".as_ptr() as *const c_char, devs: [pnp_device_id { id: b"ADS7180\0".as_ptr() as *const c_char }, pnp_device_id { id: b"ADS7181\0".as_ptr() as *const c_char }] },
    /* Analog Devices AD1816A - added by Kenneth Platz <kxp@atl.hp.com> */
    pnp_card_device_id { id: b"ADS7181\0".as_ptr() as *const c_char, devs: [pnp_device_id { id: b"ADS7180\0".as_ptr() as *const c_char }, pnp_device_id { id: b"ADS7181\0".as_ptr() as *const c_char }] },
    /* Analog Devices AD1816A - Aztech/Newcom SC-16 3D */
    pnp_card_device_id { id: b"AZT1022\0".as_ptr() as *const c_char, devs: [pnp_device_id { id: b"AZT1018\0".as_ptr() as *const c_char }, pnp_device_id { id: b"AZT2002\0".as_ptr() as *const c_char }] },
    /* Highscreen Sound-Boostar 16 3D - added by Stefan Behnel */
    pnp_card_device_id { id: b"LWC1061\0".as_ptr() as *const c_char, devs: [pnp_device_id { id: b"ADS7180\0".as_ptr() as *const c_char }, pnp_device_id { id: b"ADS7181\0".as_ptr() as *const c_char }] },
    /* Highscreen Sound-Boostar 16 3D */
    pnp_card_device_id { id: b"MDK1605\0".as_ptr() as *const c_char, devs: [pnp_device_id { id: b"ADS7180\0".as_ptr() as *const c_char }, pnp_device_id { id: b"ADS7181\0".as_ptr() as *const c_char }] },
    /* Shark Predator ISA - added by Ken Arromdee */
    pnp_card_device_id { id: b"SMM7180\0".as_ptr() as *const c_char, devs: [pnp_device_id { id: b"ADS7180\0".as_ptr() as *const c_char }, pnp_device_id { id: b"ADS7181\0".as_ptr() as *const c_char }] },
    /* Analog Devices AD1816A - Terratec AudioSystem EWS64 S */
    pnp_card_device_id { id: b"TER1112\0".as_ptr() as *const c_char, devs: [pnp_device_id { id: b"ADS7180\0".as_ptr() as *const c_char }, pnp_device_id { id: b"ADS7181\0".as_ptr() as *const c_char }] },
    /* Analog Devices AD1816A - Terratec AudioSystem EWS64 S */
    pnp_card_device_id { id: b"TER1112\0".as_ptr() as *const c_char, devs: [pnp_device_id { id: b"TER1100\0".as_ptr() as *const c_char }, pnp_device_id { id: b"TER1101\0".as_ptr() as *const c_char }] },
    /* Analog Devices AD1816A - Terratec Base 64 */
    pnp_card_device_id { id: b"TER1411\0".as_ptr() as *const c_char, devs: [pnp_device_id { id: b"ADS7180\0".as_ptr() as *const c_char }, pnp_device_id { id: b"ADS7181\0".as_ptr() as *const c_char }] },
    /* end */
    pnp_card_device_id { id: b"\0".as_ptr() as *const c_char, devs: [pnp_device_id { id: core::ptr::null() }, pnp_device_id { id: core::ptr::null() }] },
];

// MODULE_DEVICE_TABLE(pnp_card, snd_ad1816a_pnpids);

const DRIVER_NAME: *const c_char = b"snd-card-ad1816a\0".as_ptr() as *const c_char;

unsafe extern "C" fn snd_card_ad1816a_pnp(
    dev: c_int,
    card: *mut pnp_card_link,
    id: *const pnp_card_device_id,
) -> c_int {
    let mut pdev: *mut pnp_dev;
    let mut err: c_int;
    let dev_usize = dev as usize;

    pdev = pnp_request_card_device(card, (*id).devs[0].id, core::ptr::null_mut());
    if pdev.is_null() {
        return -EBUSY;
    }

    err = pnp_activate_dev(pdev);
    if err < 0 {
        dev_err(&mut (*pdev).dev, b"AUDIO PnP configure failure\n\0".as_ptr() as *const c_char);
        return -EBUSY;
    }

    port[dev_usize] = pnp_port_start(pdev, 2) as c_long;
    fm_port[dev_usize] = pnp_port_start(pdev, 1) as c_long;
    dma1[dev_usize] = pnp_dma(pdev, 0);
    dma2[dev_usize] = pnp_dma(pdev, 1);
    irq[dev_usize] = pnp_irq(pdev, 0);

    pdev = pnp_request_card_device(card, (*id).devs[1].id, core::ptr::null_mut());
    if pdev.is_null() {
        mpu_port[dev_usize] = -1;
        pr_warn(b"MPU401 device busy, skipping.\n\0".as_ptr() as *const c_char);
        return 0;
    }

    err = pnp_activate_dev(pdev);
    if err < 0 {
        dev_err(&mut (*pdev).dev, b"MPU401 PnP configure failure\n\0".as_ptr() as *const c_char);
        mpu_port[dev_usize] = -1;
    } else {
        mpu_port[dev_usize] = pnp_port_start(pdev, 0) as c_long;
        mpu_irq[dev_usize] = pnp_irq(pdev, 0);
    }

    0
}

unsafe extern "C" fn snd_card_ad1816a_probe(
    dev: c_int,
    pcard: *mut pnp_card_link,
    pid: *const pnp_card_device_id,
) -> c_int {
    let mut error: c_int;
    let mut card: *mut snd_card = core::ptr::null_mut();
    let mut chip: *mut snd_ad1816a;
    let mut opl3: *mut snd_opl3 = core::ptr::null_mut();
    let dev_usize = dev as usize;

    error = snd_devm_card_new(
        &mut (*(*pcard).card).dev,
        index[dev_usize],
        id[dev_usize],
        THIS_MODULE,
        core::mem::size_of::<snd_ad1816a>(),
        &mut card,
    );
    if error < 0 {
        return error;
    }
    chip = (*card).private_data as *mut snd_ad1816a;

    error = snd_card_ad1816a_pnp(dev, pcard, pid);
    if error != 0 {
        return error;
    }

    error = snd_ad1816a_create(
        card,
        port[dev_usize],
        irq[dev_usize],
        dma1[dev_usize],
        dma2[dev_usize],
        chip,
    );
    if error != 0 {
        return error;
    }
    if clockfreq[dev_usize] >= 5000 && clockfreq[dev_usize] <= 100000 {
        (*chip).clock_freq = clockfreq[dev_usize];
    }

    strscpy((*card).driver.as_mut_ptr(), b"AD1816A\0".as_ptr() as *const c_char);
    strscpy((*card).shortname.as_mut_ptr(), b"ADI SoundPort AD1816A\0".as_ptr() as *const c_char);
    sprintf(
        (*card).longname.as_mut_ptr(),
        b"%s, SS at 0x%lx, irq %d, dma %d&%d\0".as_ptr() as *const c_char,
        (*card).shortname.as_ptr(),
        (*chip).port,
        irq[dev_usize],
        dma1[dev_usize],
        dma2[dev_usize],
    );

    error = snd_ad1816a_pcm(chip, 0);
    if error < 0 {
        return error;
    }

    error = snd_ad1816a_mixer(chip);
    if error < 0 {
        return error;
    }

    error = snd_ad1816a_timer(chip, 0);
    if error < 0 {
        return error;
    }

    if mpu_port[dev_usize] > 0 {
        if snd_mpu401_uart_new(
            card,
            0,
            MPU401_HW_MPU401,
            mpu_port[dev_usize],
            0,
            mpu_irq[dev_usize],
            core::ptr::null_mut(),
        ) < 0
        {
            dev_err(
                (*card).dev,
                b"no MPU-401 device at 0x%lx.\n\0".as_ptr() as *const c_char,
                mpu_port[dev_usize],
            );
        }
    }

    if fm_port[dev_usize] > 0 {
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
                b"no OPL device at 0x%lx-0x%lx.\n\0".as_ptr() as *const c_char,
                fm_port[dev_usize],
                fm_port[dev_usize] + 2,
            );
        } else {
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

static mut ad1816a_devices: c_uint = 0;

unsafe extern "C" fn snd_ad1816a_pnp_detect(
    card: *mut pnp_card_link,
    id: *const pnp_card_device_id,
) -> c_int {
    static mut dev: c_int = 0;
    let mut res: c_int;

    while dev < SNDRV_CARDS as c_int {
        if !enable[dev as usize] {
            dev += 1;
            continue;
        }
        res = snd_card_ad1816a_probe(dev, card, id);
        if res < 0 {
            return res;
        }
        dev += 1;
        ad1816a_devices += 1;
        return 0;
    }
    -ENODEV
}

// Original C conditional: #ifdef CONFIG_PM
unsafe extern "C" fn snd_ad1816a_pnp_suspend(
    pcard: *mut pnp_card_link,
    _state: pm_message_t,
) -> c_int {
    let card = pnp_get_card_drvdata(pcard) as *mut snd_card;

    snd_power_change_state(card, SNDRV_CTL_POWER_D3hot);
    snd_ad1816a_suspend((*card).private_data);
    0
}

unsafe extern "C" fn snd_ad1816a_pnp_resume(pcard: *mut pnp_card_link) -> c_int {
    let card = pnp_get_card_drvdata(pcard) as *mut snd_card;

    snd_ad1816a_resume((*card).private_data);
    snd_power_change_state(card, SNDRV_CTL_POWER_D0);
    0
}
// End original C conditional: #endif

static mut ad1816a_pnpc_driver: pnp_card_driver = pnp_card_driver {
    flags: PNP_DRIVER_RES_DISABLE,
    name: b"ad1816a\0".as_ptr() as *const c_char,
    id_table: unsafe { snd_ad1816a_pnpids.as_ptr() },
    probe: Some(snd_ad1816a_pnp_detect),
    // Original C initializes these fields only when CONFIG_PM is enabled.
    suspend: Some(snd_ad1816a_pnp_suspend),
    resume: Some(snd_ad1816a_pnp_resume),
};

unsafe extern "C" fn alsa_card_ad1816a_init() -> c_int {
    let mut err: c_int;

    err = pnp_register_card_driver(&mut ad1816a_pnpc_driver);
    if err != 0 {
        return err;
    }

    if ad1816a_devices == 0 {
        pnp_unregister_card_driver(&mut ad1816a_pnpc_driver);
        // Original C conditional: #ifdef MODULE
        pr_err(b"no AD1816A based soundcards found.\n\0".as_ptr() as *const c_char);
        // #endif /* MODULE */
        return -ENODEV;
    }
    0
}

unsafe extern "C" fn alsa_card_ad1816a_exit() {
    pnp_unregister_card_driver(&mut ad1816a_pnpc_driver);
}

// module_init(alsa_card_ad1816a_init)
// module_exit(alsa_card_ad1816a_exit)

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
