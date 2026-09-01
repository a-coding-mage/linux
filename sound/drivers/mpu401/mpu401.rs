// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Driver for generic MPU-401 boards (UART mode only)
 *  Copyright (c) by Jaroslav Kysela <perex@perex.cz>
 *  Copyright (c) 2004 by Castet Matthieu <castet.matthieu@free.fr>
 */

// C includes translated as dependency intent:
// linux/init.h, linux/pnp.h, linux/err.h, linux/platform_device.h,
// linux/module.h, sound/core.h, sound/mpu401.h, sound/initval.h

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};
use core::ptr;

// MODULE_AUTHOR("Jaroslav Kysela <perex@perex.cz>");
// MODULE_DESCRIPTION("MPU-401 UART");
// MODULE_LICENSE("GPL");

extern "C" {
    static THIS_MODULE: *mut c_void;

    static SNDRV_CARDS: usize;
    static SNDRV_DEFAULT_STR: [*mut c_char; 0];
    static SNDRV_DEFAULT_ENABLE: [bool; 0];
    static SNDRV_DEFAULT_PORT: [c_long; 0];
    static SNDRV_DEFAULT_IRQ: [c_int; 0];

    static MPU401_HW_MPU401: c_int;
    static SNDRV_AUTO_PORT: c_long;
    static SNDRV_AUTO_IRQ: c_int;

    static EINVAL: c_int;
    static ENODEV: c_int;
    static IORESOURCE_DISABLED: c_uint;

    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn pr_err(fmt: *const c_char, ...);

    fn snd_devm_card_new(
        parent: *mut device,
        idx: c_int,
        xid: *mut c_char,
        module: *mut c_void,
        extra_size: c_int,
        card_ret: *mut *mut snd_card,
    ) -> c_int;
    fn snd_mpu401_uart_new(
        card: *mut snd_card,
        device: c_int,
        hardware: c_int,
        port: c_long,
        integrated: c_int,
        irq: c_int,
        rrawmidi: *mut *mut c_void,
    ) -> c_int;
    fn snd_card_register(card: *mut snd_card) -> c_int;

    fn strscpy(dst: *mut c_char, src: *const c_char) -> isize;
    fn sprintf(buf: *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn strlen(s: *const c_char) -> usize;
    fn strcat(dst: *mut c_char, src: *const c_char) -> *mut c_char;

    fn platform_driver_register(driver: *mut platform_driver) -> c_int;
    fn platform_driver_unregister(driver: *mut platform_driver);
    fn platform_device_register_simple(
        name: *const c_char,
        id: c_int,
        res: *mut c_void,
        num: c_uint,
    ) -> *mut platform_device;
    fn platform_device_unregister(pdev: *mut platform_device);
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut c_void);
    fn platform_get_drvdata(pdev: *mut platform_device) -> *mut c_void;

    fn IS_ERR(ptr: *const c_void) -> bool;

    fn pnp_port_valid(dev: *mut pnp_dev, bar: c_int) -> bool;
    fn pnp_port_flags(dev: *mut pnp_dev, bar: c_int) -> c_uint;
    fn pnp_port_len(dev: *mut pnp_dev, bar: c_int) -> u64;
    fn pnp_port_start(dev: *mut pnp_dev, bar: c_int) -> c_long;
    fn pnp_irq_valid(dev: *mut pnp_dev, bar: c_int) -> bool;
    fn pnp_irq_flags(dev: *mut pnp_dev, bar: c_int) -> c_uint;
    fn pnp_irq(dev: *mut pnp_dev, bar: c_int) -> c_int;
    fn pnp_set_drvdata(pdev: *mut pnp_dev, data: *mut c_void);
    fn pnp_register_driver(driver: *mut pnp_driver) -> c_int;
    fn pnp_unregister_driver(driver: *mut pnp_driver);
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_card {
    pub driver: [c_char; 16],
    pub shortname: [c_char; 32],
    pub longname: [c_char; 80],
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
    pub id: c_int,
}

#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
}

#[repr(C)]
pub struct platform_driver {
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    pub driver: device_driver,
}

#[repr(C)]
pub struct pnp_dev {
    pub dev: device,
}

#[repr(C)]
pub struct pnp_device_id {
    pub id: [c_char; 8],
}

#[repr(C)]
pub struct pnp_driver {
    pub name: *const c_char,
    pub id_table: *const pnp_device_id,
    pub probe: Option<unsafe extern "C" fn(*mut pnp_dev, *const pnp_device_id) -> c_int>,
}

// static int index[SNDRV_CARDS] = {[0 ... (SNDRV_CARDS - 1)] = -2}; /* exclude the first card */
static mut index: [c_int; SNDRV_CARDS_VALUE] = [-2; SNDRV_CARDS_VALUE];
// static char *id[SNDRV_CARDS] = SNDRV_DEFAULT_STR;	/* ID for this card */
static mut id: [*mut c_char; SNDRV_CARDS_VALUE] = [ptr::null_mut(); SNDRV_CARDS_VALUE];
// static bool enable[SNDRV_CARDS] = SNDRV_DEFAULT_ENABLE;	/* Enable this card */
static mut enable: [bool; SNDRV_CARDS_VALUE] = [false; SNDRV_CARDS_VALUE];
// #ifdef CONFIG_PNP
static mut pnp: [bool; SNDRV_CARDS_VALUE] = [true; SNDRV_CARDS_VALUE];
// #endif
// static long port[SNDRV_CARDS] = SNDRV_DEFAULT_PORT;	/* MPU-401 port number */
static mut port: [c_long; SNDRV_CARDS_VALUE] = [0; SNDRV_CARDS_VALUE];
// static int irq[SNDRV_CARDS] = SNDRV_DEFAULT_IRQ;	/* MPU-401 IRQ */
static mut irq: [c_int; SNDRV_CARDS_VALUE] = [0; SNDRV_CARDS_VALUE];
static mut uart_enter: [bool; SNDRV_CARDS_VALUE] = [true; SNDRV_CARDS_VALUE];

// File-local stand-in for the external array bound used by this isolated translation.
const SNDRV_CARDS_VALUE: usize = 8;

// module_param_array(index, int, NULL, 0444);
// MODULE_PARM_DESC(index, "Index value for MPU-401 device.");
// module_param_array(id, charp, NULL, 0444);
// MODULE_PARM_DESC(id, "ID string for MPU-401 device.");
// module_param_array(enable, bool, NULL, 0444);
// MODULE_PARM_DESC(enable, "Enable MPU-401 device.");
// #ifdef CONFIG_PNP
// module_param_array(pnp, bool, NULL, 0444);
// MODULE_PARM_DESC(pnp, "PnP detection for MPU-401 device.");
// #endif
// module_param_hw_array(port, long, ioport, NULL, 0444);
// MODULE_PARM_DESC(port, "Port # for MPU-401 device.");
// module_param_hw_array(irq, int, irq, NULL, 0444);
// MODULE_PARM_DESC(irq, "IRQ # for MPU-401 device.");
// module_param_array(uart_enter, bool, NULL, 0444);
// MODULE_PARM_DESC(uart_enter, "Issue UART_ENTER command at open.");

static mut platform_devices: [*mut platform_device; SNDRV_CARDS_VALUE] =
    [ptr::null_mut(); SNDRV_CARDS_VALUE];
static mut pnp_registered: c_int = 0;
static mut snd_mpu401_devices: c_uint = 0;

unsafe extern "C" fn snd_mpu401_create(
    devptr: *mut device,
    dev: c_int,
    rcard: *mut *mut snd_card,
) -> c_int {
    let mut card: *mut snd_card = ptr::null_mut();
    let mut err: c_int;
    let dev_usize = dev as usize;

    if !uart_enter[dev_usize] {
        dev_err(
            devptr,
            c"the uart_enter option is obsolete; remove it\n".as_ptr(),
        );
    }

    *rcard = ptr::null_mut();
    err = snd_devm_card_new(
        devptr,
        index[dev_usize],
        id[dev_usize],
        THIS_MODULE,
        0,
        &mut card,
    );
    if err < 0 {
        return err;
    }
    strscpy((*card).driver.as_mut_ptr(), c"MPU-401 UART".as_ptr());
    strscpy((*card).shortname.as_mut_ptr(), (*card).driver.as_ptr());
    sprintf(
        (*card).longname.as_mut_ptr(),
        c"%s at %#lx, ".as_ptr(),
        (*card).shortname.as_ptr(),
        port[dev_usize],
    );
    if irq[dev_usize] >= 0 {
        sprintf(
            (*card)
                .longname
                .as_mut_ptr()
                .add(strlen((*card).longname.as_ptr())),
            c"irq %d".as_ptr(),
            irq[dev_usize],
        );
    } else {
        strcat((*card).longname.as_mut_ptr(), c"polled".as_ptr());
    }

    err = snd_mpu401_uart_new(
        card,
        0,
        MPU401_HW_MPU401,
        port[dev_usize],
        0,
        irq[dev_usize],
        ptr::null_mut(),
    );
    if err < 0 {
        dev_err(
            devptr,
            c"MPU401 not detected at 0x%lx\n".as_ptr(),
            port[dev_usize],
        );
        return err;
    }

    *rcard = card;
    0
}

unsafe extern "C" fn snd_mpu401_probe(devptr: *mut platform_device) -> c_int {
    let mut dev: c_int = (*devptr).id;
    let mut err: c_int;
    let mut card: *mut snd_card = ptr::null_mut();

    if dev < 0 || dev >= SNDRV_CARDS_VALUE as c_int {
        dev_warn(
            &mut (*devptr).dev,
            c"Invalid card index %d, using default 0\n".as_ptr(),
            dev,
        );
        dev = 0;
    }

    if port[dev as usize] == SNDRV_AUTO_PORT {
        dev_err(&mut (*devptr).dev, c"specify port\n".as_ptr());
        return -EINVAL;
    }
    if irq[dev as usize] == SNDRV_AUTO_IRQ {
        dev_err(&mut (*devptr).dev, c"specify or disable IRQ\n".as_ptr());
        return -EINVAL;
    }
    err = snd_mpu401_create(&mut (*devptr).dev, dev, &mut card);
    if err < 0 {
        return err;
    }
    err = snd_card_register(card);
    if err < 0 {
        return err;
    }
    platform_set_drvdata(devptr, card as *mut c_void);
    0
}

const SND_MPU401_DRIVER: *const c_char = c"snd_mpu401".as_ptr();

static mut snd_mpu401_driver: platform_driver = platform_driver {
    probe: Some(snd_mpu401_probe),
    driver: device_driver {
        name: SND_MPU401_DRIVER,
    },
};

// #ifdef CONFIG_PNP

const IO_EXTENT: c_int = 2;

static snd_mpu401_pnpids: [pnp_device_id; 2] = [
    pnp_device_id { id: *b"PNPb006\0" },
    pnp_device_id { id: [0; 8] },
];

// MODULE_DEVICE_TABLE(pnp, snd_mpu401_pnpids);

unsafe extern "C" fn snd_mpu401_pnp(
    dev: c_int,
    device: *mut pnp_dev,
    _id: *const pnp_device_id,
) -> c_int {
    if !pnp_port_valid(device, 0) || (pnp_port_flags(device, 0) & IORESOURCE_DISABLED) != 0 {
        dev_err(&mut (*device).dev, c"no PnP port\n".as_ptr());
        return -ENODEV;
    }
    if pnp_port_len(device, 0) < IO_EXTENT as u64 {
        dev_err(
            &mut (*device).dev,
            c"PnP port length is %llu, expected %d\n".as_ptr(),
            pnp_port_len(device, 0) as u64,
            IO_EXTENT,
        );
        return -ENODEV;
    }
    port[dev as usize] = pnp_port_start(device, 0);

    if !pnp_irq_valid(device, 0) || (pnp_irq_flags(device, 0) & IORESOURCE_DISABLED) != 0 {
        dev_warn(&mut (*device).dev, c"no PnP irq, using polling\n".as_ptr());
        irq[dev as usize] = -1;
    } else {
        irq[dev as usize] = pnp_irq(device, 0);
    }
    0
}

unsafe extern "C" fn snd_mpu401_pnp_probe(
    pnp_dev: *mut pnp_dev,
    id: *const pnp_device_id,
) -> c_int {
    static mut dev: c_int = 0;
    let mut card: *mut snd_card = ptr::null_mut();
    let mut err: c_int;

    while dev < SNDRV_CARDS_VALUE as c_int {
        if !enable[dev as usize] || !pnp[dev as usize] {
            dev += 1;
            continue;
        }
        err = snd_mpu401_pnp(dev, pnp_dev, id);
        if err < 0 {
            return err;
        }
        err = snd_mpu401_create(&mut (*pnp_dev).dev, dev, &mut card);
        if err < 0 {
            return err;
        }
        err = snd_card_register(card);
        if err < 0 {
            return err;
        }
        pnp_set_drvdata(pnp_dev, card as *mut c_void);
        snd_mpu401_devices = snd_mpu401_devices.wrapping_add(1);
        dev += 1;
        return 0;
    }
    -ENODEV
}

static mut snd_mpu401_pnp_driver: pnp_driver = pnp_driver {
    name: c"mpu401".as_ptr(),
    id_table: snd_mpu401_pnpids.as_ptr(),
    probe: Some(snd_mpu401_pnp_probe),
};

// #else
// static struct pnp_driver snd_mpu401_pnp_driver;
// #endif

unsafe extern "C" fn snd_mpu401_unregister_all() {
    let mut i: usize;

    if pnp_registered != 0 {
        pnp_unregister_driver(&raw mut snd_mpu401_pnp_driver);
    }
    i = 0;
    while i < platform_devices.len() {
        platform_device_unregister(platform_devices[i]);
        i += 1;
    }
    platform_driver_unregister(&raw mut snd_mpu401_driver);
}

// __init
unsafe extern "C" fn alsa_card_mpu401_init() -> c_int {
    let mut i: c_int;
    let mut err: c_int;

    err = platform_driver_register(&raw mut snd_mpu401_driver);
    if err < 0 {
        return err;
    }

    i = 0;
    while i < SNDRV_CARDS_VALUE as c_int {
        let device: *mut platform_device;
        if !enable[i as usize] {
            i += 1;
            continue;
        }
        // #ifdef CONFIG_PNP
        if pnp[i as usize] {
            i += 1;
            continue;
        }
        // #endif
        device = platform_device_register_simple(SND_MPU401_DRIVER, i, ptr::null_mut(), 0);
        if IS_ERR(device as *const c_void) {
            i += 1;
            continue;
        }
        if platform_get_drvdata(device).is_null() {
            platform_device_unregister(device);
            i += 1;
            continue;
        }
        platform_devices[i as usize] = device;
        snd_mpu401_devices = snd_mpu401_devices.wrapping_add(1);
        i += 1;
    }
    err = pnp_register_driver(&raw mut snd_mpu401_pnp_driver);
    if err == 0 {
        pnp_registered = 1;
    }

    if snd_mpu401_devices == 0 {
        // #ifdef MODULE
        pr_err(c"MPU-401 device not found or device busy\n".as_ptr());
        // #endif
        snd_mpu401_unregister_all();
        return -ENODEV;
    }
    0
}

// __exit
unsafe extern "C" fn alsa_card_mpu401_exit() {
    snd_mpu401_unregister_all();
}

// module_init(alsa_card_mpu401_init)
// module_exit(alsa_card_mpu401_exit)

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
