// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Driver for generic ESS AudioDrive ESx688 soundcards
 *  Copyright (c) by Jaroslav Kysela <perex@perex.cz>
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};
use core::ptr;

const CRD_NAME: &[u8] = b"Generic ESS ES1688/ES688 AudioDrive\0";
const DEV_NAME: &[u8] = b"es1688\0";

/* Module metadata:
 * MODULE_DESCRIPTION(CRD_NAME);
 * MODULE_AUTHOR("Jaroslav Kysela <perex@perex.cz>");
 * MODULE_LICENSE("GPL");
 * MODULE_ALIAS("snd_es968");
 */

const SNDRV_CARDS: usize = 8;
const SNDRV_AUTO_IRQ: c_int = -1;
const SNDRV_AUTO_DMA: c_int = -1;
const SNDRV_AUTO_PORT: c_long = -1;
const EBUSY: c_int = 16;
const ENODEV: c_int = 19;
const ES1688_HW_AUTO: c_int = 0;
const OPL3_HW_OPL3: c_int = 3;
const MPU401_HW_ES1688: c_int = 0;
const SNDRV_CTL_POWER_D3hot: c_int = 3;
const SNDRV_CTL_POWER_D0: c_int = 0;
const PNP_DRIVER_RES_DISABLE: c_uint = 1;

type bool_ = bool;
type pm_message_t = c_int;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm {
    pub name: [c_char; 80],
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
pub struct snd_es1688 {
    pub pcm: *mut snd_pcm,
    pub port: c_long,
    pub irq: c_int,
    pub dma8: c_int,
    pub mpu_port: c_long,
}

#[repr(C)]
pub struct snd_opl3 {
    _private: [u8; 0],
}

#[repr(C)]
pub struct module {
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
    #[cfg(CONFIG_PM)]
    pub suspend: Option<unsafe extern "C" fn(*mut device, c_uint, pm_message_t) -> c_int>,
    #[cfg(CONFIG_PM)]
    pub resume: Option<unsafe extern "C" fn(*mut device, c_uint) -> c_int>,
    pub driver: device_driver,
}

#[repr(C)]
pub struct pnp_dev {
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
#[derive(Copy, Clone)]
pub struct pnp_card_devs_id {
    pub id: *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pnp_card_device_id {
    pub id: *const c_char,
    pub devs: [pnp_card_devs_id; 1],
}

#[repr(C)]
pub struct pnp_card_driver {
    pub flags: c_uint,
    pub name: *const c_char,
    pub id_table: *const pnp_card_device_id,
    pub probe: Option<unsafe extern "C" fn(*mut pnp_card_link, *const pnp_card_device_id) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut pnp_card_link)>,
    #[cfg(CONFIG_PM)]
    pub suspend: Option<unsafe extern "C" fn(*mut pnp_card_link, pm_message_t) -> c_int>,
    #[cfg(CONFIG_PM)]
    pub resume: Option<unsafe extern "C" fn(*mut pnp_card_link) -> c_int>,
}

unsafe extern "C" {
    static THIS_MODULE: *mut module;

    fn snd_legacy_find_free_irq(possible_irqs: *const c_int) -> c_int;
    fn snd_legacy_find_free_dma(possible_dmas: *const c_int) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn snd_es1688_create(
        card: *mut snd_card,
        chip: *mut snd_es1688,
        port: c_long,
        mpu_port: c_long,
        irq: c_int,
        mpu_irq: c_int,
        dma8: c_int,
        hardware: c_int,
    ) -> c_int;
    fn snd_es1688_pcm(card: *mut snd_card, chip: *mut snd_es1688, device: c_int) -> c_int;
    fn snd_es1688_mixer(card: *mut snd_card, chip: *mut snd_es1688) -> c_int;
    fn strscpy(dest: *mut c_char, src: *const c_char, count: usize) -> isize;
    fn scnprintf(buf: *mut c_char, size: usize, fmt: *const c_char, ...) -> c_int;
    fn snd_opl3_create(
        card: *mut snd_card,
        l_port: c_long,
        r_port: c_long,
        hardware: c_int,
        integrated: c_int,
        opl3: *mut *mut snd_opl3,
    ) -> c_int;
    fn snd_opl3_hwdep_new(
        opl3: *mut snd_opl3,
        device: c_int,
        seq_device: c_int,
        info: *mut c_void,
    ) -> c_int;
    fn snd_mpu401_uart_new(
        card: *mut snd_card,
        device: c_int,
        hardware: c_int,
        port: c_long,
        info_flags: c_uint,
        irq: c_int,
        rrawmidi: *mut c_void,
    ) -> c_int;
    fn snd_card_register(card: *mut snd_card) -> c_int;
    fn snd_devm_card_new(
        parent: *mut device,
        idx: c_int,
        xid: *mut c_char,
        module: *mut module,
        extra_size: usize,
        card_ret: *mut *mut snd_card,
    ) -> c_int;
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn snd_power_change_state(card: *mut snd_card, state: c_int);
    fn snd_es1688_reset(chip: *mut snd_es1688) -> c_int;
    fn isa_register_driver(driver: *mut isa_driver, ndev: c_uint) -> c_int;
    fn isa_unregister_driver(driver: *mut isa_driver);

    #[cfg(CONFIG_PNP)]
    fn pnp_request_card_device(
        card: *mut pnp_card_link,
        id: *const c_char,
        from: *mut pnp_dev,
    ) -> *mut pnp_dev;
    #[cfg(CONFIG_PNP)]
    fn pnp_activate_dev(dev: *mut pnp_dev) -> c_int;
    #[cfg(CONFIG_PNP)]
    fn pnp_port_start(dev: *mut pnp_dev, bar: c_uint) -> c_long;
    #[cfg(CONFIG_PNP)]
    fn pnp_dma(dev: *mut pnp_dev, n: c_uint) -> c_int;
    #[cfg(CONFIG_PNP)]
    fn pnp_irq(dev: *mut pnp_dev, n: c_uint) -> c_int;
    #[cfg(CONFIG_PNP)]
    fn pnp_set_card_drvdata(card: *mut pnp_card_link, data: *mut c_void);
    #[cfg(CONFIG_PNP)]
    fn pnp_get_card_drvdata(card: *mut pnp_card_link) -> *mut c_void;
    #[cfg(CONFIG_PNP)]
    fn pnp_register_card_driver(driver: *mut pnp_card_driver) -> c_int;
    #[cfg(CONFIG_PNP)]
    fn pnp_unregister_card_driver(driver: *mut pnp_card_driver);
}

static mut index: [c_int; SNDRV_CARDS] = [-1; SNDRV_CARDS]; /* Index 0-MAX */
static mut id: [*mut c_char; SNDRV_CARDS] = [ptr::null_mut(); SNDRV_CARDS]; /* ID for this card */
#[cfg(CONFIG_PNP)]
static mut isapnp: [bool_; SNDRV_CARDS] = [true; SNDRV_CARDS];
static mut enable: [bool_; SNDRV_CARDS] = [true; SNDRV_CARDS]; /* Enable this card */
static mut port: [c_long; SNDRV_CARDS] = [SNDRV_AUTO_PORT; SNDRV_CARDS]; /* 0x220,0x240,0x260 */
static mut fm_port: [c_long; SNDRV_CARDS] = [SNDRV_AUTO_PORT; SNDRV_CARDS]; /* Usually 0x388 */
static mut mpu_port: [c_long; SNDRV_CARDS] = [-1; SNDRV_CARDS];
static mut irq: [c_int; SNDRV_CARDS] = [SNDRV_AUTO_IRQ; SNDRV_CARDS]; /* 5,7,9,10 */
static mut mpu_irq: [c_int; SNDRV_CARDS] = [SNDRV_AUTO_IRQ; SNDRV_CARDS]; /* 5,7,9,10 */
static mut dma8: [c_int; SNDRV_CARDS] = [SNDRV_AUTO_DMA; SNDRV_CARDS]; /* 0,1,3 */

/* module_param_array/module_param_hw_array and MODULE_PARM_DESC declarations
 * from the C source are Linux module metadata.
 */

#[cfg(CONFIG_PNP)]
unsafe fn is_isapnp_selected(dev: usize) -> c_int {
    isapnp[dev] as c_int
}

#[cfg(not(CONFIG_PNP))]
unsafe fn is_isapnp_selected(_dev: usize) -> c_int {
    0
}

unsafe extern "C" fn snd_es1688_match(_dev: *mut device, n: c_uint) -> c_int {
    (enable[n as usize] && is_isapnp_selected(n as usize) == 0) as c_int
}

unsafe extern "C" fn snd_es1688_legacy_create(
    card: *mut snd_card,
    dev: *mut device,
    n: c_uint,
) -> c_int {
    let chip = (*card).private_data as *mut snd_es1688;
    static possible_ports: [c_long; 3] = [0x220, 0x240, 0x260];
    static possible_irqs: [c_int; 5] = [5, 9, 10, 7, -1];
    static possible_dmas: [c_int; 4] = [1, 3, 0, -1];

    let mut i: usize;
    let mut error: c_int;
    let n = n as usize;

    if irq[n] == SNDRV_AUTO_IRQ {
        irq[n] = snd_legacy_find_free_irq(possible_irqs.as_ptr());
        if irq[n] < 0 {
            dev_err(dev, c"unable to find a free IRQ\n".as_ptr());
            return -EBUSY;
        }
    }
    if dma8[n] == SNDRV_AUTO_DMA {
        dma8[n] = snd_legacy_find_free_dma(possible_dmas.as_ptr());
        if dma8[n] < 0 {
            dev_err(dev, c"unable to find a free DMA\n".as_ptr());
            return -EBUSY;
        }
    }

    if port[n] != SNDRV_AUTO_PORT {
        return snd_es1688_create(
            card,
            chip,
            port[n],
            mpu_port[n],
            irq[n],
            mpu_irq[n],
            dma8[n],
            ES1688_HW_AUTO,
        );
    }

    i = 0;
    loop {
        port[n] = possible_ports[i];
        error = snd_es1688_create(
            card,
            chip,
            port[n],
            mpu_port[n],
            irq[n],
            mpu_irq[n],
            dma8[n],
            ES1688_HW_AUTO,
        );
        i += 1;
        if !(error < 0 && i < possible_ports.len()) {
            break;
        }
    }

    error
}

unsafe extern "C" fn snd_es1688_probe(card: *mut snd_card, n: c_uint) -> c_int {
    let chip = (*card).private_data as *mut snd_es1688;
    let mut opl3: *mut snd_opl3 = ptr::null_mut();
    let mut error: c_int;
    let n = n as usize;

    error = snd_es1688_pcm(card, chip, 0);
    if error < 0 {
        return error;
    }

    error = snd_es1688_mixer(card, chip);
    if error < 0 {
        return error;
    }

    strscpy((*card).driver.as_mut_ptr(), c"ES1688".as_ptr(), (*card).driver.len());
    strscpy(
        (*card).shortname.as_mut_ptr(),
        (*(*chip).pcm).name.as_ptr(),
        (*card).shortname.len(),
    );
    scnprintf(
        (*card).longname.as_mut_ptr(),
        (*card).longname.len(),
        c"%s at 0x%lx, irq %i, dma %i".as_ptr(),
        (*(*chip).pcm).name.as_ptr(),
        (*chip).port,
        (*chip).irq,
        (*chip).dma8,
    );

    if fm_port[n] == SNDRV_AUTO_PORT {
        fm_port[n] = port[n]; /* share the same port */
    }

    if fm_port[n] > 0 {
        if snd_opl3_create(
            card,
            fm_port[n],
            fm_port[n] + 2,
            OPL3_HW_OPL3,
            0,
            &mut opl3,
        ) < 0
        {
            dev_warn((*card).dev, c"opl3 not detected at 0x%lx\n".as_ptr(), fm_port[n]);
        } else {
            error = snd_opl3_hwdep_new(opl3, 0, 1, ptr::null_mut());
            if error < 0 {
                return error;
            }
        }
    }

    if mpu_irq[n] >= 0 && mpu_irq[n] != SNDRV_AUTO_IRQ && (*chip).mpu_port > 0 {
        error = snd_mpu401_uart_new(
            card,
            0,
            MPU401_HW_ES1688,
            (*chip).mpu_port,
            0,
            mpu_irq[n],
            ptr::null_mut(),
        );
        if error < 0 {
            return error;
        }
    }

    snd_card_register(card)
}

unsafe extern "C" fn snd_es1688_isa_probe(dev: *mut device, n: c_uint) -> c_int {
    let mut card: *mut snd_card = ptr::null_mut();
    let mut error: c_int;
    let nu = n as usize;

    error = snd_devm_card_new(
        dev,
        index[nu],
        id[nu],
        THIS_MODULE,
        core::mem::size_of::<snd_es1688>(),
        &mut card,
    );
    if error < 0 {
        return error;
    }

    error = snd_es1688_legacy_create(card, dev, n);
    if error < 0 {
        return error;
    }

    error = snd_es1688_probe(card, n);
    if error < 0 {
        return error;
    }

    dev_set_drvdata(dev, card as *mut c_void);

    0
}

#[cfg(CONFIG_PM)]
unsafe extern "C" fn snd_es1688_card_suspend(card: *mut snd_card) -> c_int {
    snd_power_change_state(card, SNDRV_CTL_POWER_D3hot);
    0
}

#[cfg(CONFIG_PM)]
unsafe extern "C" fn snd_es1688_card_resume(card: *mut snd_card) -> c_int {
    let chip = (*card).private_data as *mut snd_es1688;
    let err: c_int;

    err = snd_es1688_reset(chip);
    if err < 0 {
        return err;
    }

    snd_power_change_state(card, SNDRV_CTL_POWER_D0);
    0
}

#[cfg(CONFIG_PM)]
unsafe extern "C" fn snd_es1688_isa_suspend(
    dev: *mut device,
    _n: c_uint,
    _state: pm_message_t,
) -> c_int {
    snd_es1688_card_suspend(dev_get_drvdata(dev) as *mut snd_card)
}

#[cfg(CONFIG_PM)]
unsafe extern "C" fn snd_es1688_isa_resume(dev: *mut device, _n: c_uint) -> c_int {
    snd_es1688_card_resume(dev_get_drvdata(dev) as *mut snd_card)
}

static mut snd_es1688_driver: isa_driver = isa_driver {
    match_: Some(snd_es1688_match),
    probe: Some(snd_es1688_isa_probe),
    #[cfg(CONFIG_PM)]
    suspend: Some(snd_es1688_isa_suspend),
    #[cfg(CONFIG_PM)]
    resume: Some(snd_es1688_isa_resume),
    driver: device_driver {
        name: DEV_NAME.as_ptr() as *const c_char,
    },
};

static mut snd_es968_pnp_is_probed: c_int = 0;

#[cfg(CONFIG_PNP)]
unsafe extern "C" fn snd_card_es968_pnp(
    card: *mut snd_card,
    n: c_uint,
    pcard: *mut pnp_card_link,
    pid: *const pnp_card_device_id,
) -> c_int {
    let chip = (*card).private_data as *mut snd_es1688;
    let pdev: *mut pnp_dev;
    let mut error: c_int;
    let n = n as usize;

    pdev = pnp_request_card_device(pcard, (*pid).devs[0].id, ptr::null_mut());
    if pdev.is_null() {
        return -ENODEV;
    }

    error = pnp_activate_dev(pdev);
    if error < 0 {
        dev_err((*card).dev, c"ES968 pnp configure failure\n".as_ptr());
        return error;
    }
    port[n] = pnp_port_start(pdev, 0);
    dma8[n] = pnp_dma(pdev, 0);
    irq[n] = pnp_irq(pdev, 0);

    snd_es1688_create(
        card,
        chip,
        port[n],
        mpu_port[n],
        irq[n],
        mpu_irq[n],
        dma8[n],
        ES1688_HW_AUTO,
    )
}

#[cfg(CONFIG_PNP)]
unsafe extern "C" fn snd_es968_pnp_detect(
    pcard: *mut pnp_card_link,
    pid: *const pnp_card_device_id,
) -> c_int {
    let mut card: *mut snd_card = ptr::null_mut();
    static mut dev: c_uint = 0;
    let mut error: c_int;

    if snd_es968_pnp_is_probed != 0 {
        return -EBUSY;
    }
    while dev < SNDRV_CARDS as c_uint {
        if enable[dev as usize] && isapnp[dev as usize] {
            break;
        }
        dev += 1;
    }
    if dev == SNDRV_CARDS as c_uint {
        return -ENODEV;
    }

    error = snd_devm_card_new(
        &mut (*(*pcard).card).dev,
        index[dev as usize],
        id[dev as usize],
        THIS_MODULE,
        core::mem::size_of::<snd_es1688>(),
        &mut card,
    );
    if error < 0 {
        return error;
    }

    error = snd_card_es968_pnp(card, dev, pcard, pid);
    if error < 0 {
        return error;
    }
    error = snd_es1688_probe(card, dev);
    if error < 0 {
        return error;
    }
    pnp_set_card_drvdata(pcard, card as *mut c_void);
    snd_es968_pnp_is_probed = 1;
    0
}

#[cfg(CONFIG_PNP)]
unsafe extern "C" fn snd_es968_pnp_remove(_pcard: *mut pnp_card_link) {
    snd_es968_pnp_is_probed = 0;
}

#[cfg(all(CONFIG_PNP, CONFIG_PM))]
unsafe extern "C" fn snd_es968_pnp_suspend(
    pcard: *mut pnp_card_link,
    _state: pm_message_t,
) -> c_int {
    snd_es1688_card_suspend(pnp_get_card_drvdata(pcard) as *mut snd_card)
}

#[cfg(all(CONFIG_PNP, CONFIG_PM))]
unsafe extern "C" fn snd_es968_pnp_resume(pcard: *mut pnp_card_link) -> c_int {
    snd_es1688_card_resume(pnp_get_card_drvdata(pcard) as *mut snd_card)
}

#[cfg(CONFIG_PNP)]
static snd_es968_pnpids: [pnp_card_device_id; 3] = [
    pnp_card_device_id {
        id: c"ESS0968".as_ptr(),
        devs: [pnp_card_devs_id {
            id: c"@@@0968".as_ptr(),
        }],
    },
    pnp_card_device_id {
        id: c"ESS0968".as_ptr(),
        devs: [pnp_card_devs_id {
            id: c"ESS0968".as_ptr(),
        }],
    },
    pnp_card_device_id {
        id: c"".as_ptr(),
        devs: [pnp_card_devs_id { id: ptr::null() }],
    }, /* end */
];

/* MODULE_DEVICE_TABLE(pnp_card, snd_es968_pnpids); */

#[cfg(CONFIG_PNP)]
static mut es968_pnpc_driver: pnp_card_driver = pnp_card_driver {
    flags: PNP_DRIVER_RES_DISABLE,
    name: c"es1688 PnP".as_ptr(),
    id_table: snd_es968_pnpids.as_ptr(),
    probe: Some(snd_es968_pnp_detect),
    remove: Some(snd_es968_pnp_remove),
    #[cfg(CONFIG_PM)]
    suspend: Some(snd_es968_pnp_suspend),
    #[cfg(CONFIG_PM)]
    resume: Some(snd_es968_pnp_resume),
};

unsafe extern "C" fn alsa_card_es1688_init() -> c_int {
    #[cfg(CONFIG_PNP)]
    {
        pnp_register_card_driver(&mut es968_pnpc_driver);
        if snd_es968_pnp_is_probed != 0 {
            return 0;
        }
        pnp_unregister_card_driver(&mut es968_pnpc_driver);
    }
    isa_register_driver(&mut snd_es1688_driver, SNDRV_CARDS as c_uint)
}

unsafe extern "C" fn alsa_card_es1688_exit() {
    if snd_es968_pnp_is_probed == 0 {
        isa_unregister_driver(&mut snd_es1688_driver);
        return;
    }
    #[cfg(CONFIG_PNP)]
    {
        pnp_unregister_card_driver(&mut es968_pnpc_driver);
    }
}

/* module_init(alsa_card_es1688_init);
 * module_exit(alsa_card_es1688_exit);
 */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
