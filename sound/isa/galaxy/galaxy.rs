// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Aztech AZT1605/AZT2316 Driver
 * Copyright (C) 2007,2010  Rene Herman
 */

// C dependencies:
// linux/kernel.h, linux/module.h, linux/isa.h, linux/delay.h, linux/io.h,
// asm/processor.h, sound/core.h, sound/initval.h, sound/wss.h,
// sound/mpu401.h, sound/opl3.h

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};
use core::ptr;

type U8 = u8;
type U32 = u32;

const DSP_PORT_RESET: usize = 0x6;
const DSP_PORT_READ: usize = 0xa;
const DSP_PORT_COMMAND: usize = 0xc;
const DSP_PORT_STATUS: usize = 0xc;
const DSP_PORT_DATA_AVAIL: usize = 0xe;

const DSP_SIGNATURE: U8 = 0xaa;

const DSP_COMMAND_GET_VERSION: U8 = 0xe1;

const WSS_CONFIG_DMA_0: U8 = 1 << 0;
const WSS_CONFIG_DMA_1: U8 = 2 << 0;
const WSS_CONFIG_DMA_3: U8 = 3 << 0;
const WSS_CONFIG_DUPLEX: U8 = 1 << 2;
const WSS_CONFIG_IRQ_7: U8 = 1 << 3;
const WSS_CONFIG_IRQ_9: U8 = 2 << 3;
const WSS_CONFIG_IRQ_10: U8 = 3 << 3;
const WSS_CONFIG_IRQ_11: U8 = 4 << 3;

const WSS_PORT_CONFIG: usize = 0;
const WSS_PORT_SIGNATURE: usize = 3;

const WSS_SIGNATURE: U8 = 4;

const GALAXY_PORT_CONFIG: c_long = 1024;
const CONFIG_PORT_SET: usize = 4;

const DSP_COMMAND_GALAXY_8: U8 = 8;
const GALAXY_COMMAND_GET_TYPE: U8 = 5;

const DSP_COMMAND_GALAXY_9: U8 = 9;
const GALAXY_COMMAND_WSSMODE: U8 = 0;
const GALAXY_COMMAND_SB8MODE: U8 = 1;

const GALAXY_MODE_WSS: U8 = GALAXY_COMMAND_WSSMODE;
const GALAXY_MODE_SB8: U8 = GALAXY_COMMAND_SB8MODE;

extern "C" {
    static mut index: [c_int; SNDRV_CARDS];
    static mut id: [*mut c_char; SNDRV_CARDS];
    static mut enable: [bool; SNDRV_CARDS];

    static mut port: [c_long; SNDRV_CARDS];
    static mut wss_port: [c_long; SNDRV_CARDS];
    static mut mpu_port: [c_long; SNDRV_CARDS];
    static mut fm_port: [c_long; SNDRV_CARDS];
    static mut irq: [c_int; SNDRV_CARDS];
    static mut mpu_irq: [c_int; SNDRV_CARDS];
    static mut dma1: [c_int; SNDRV_CARDS];
    static mut dma2: [c_int; SNDRV_CARDS];

    static CRD_NAME: [c_char; 0];
    static DRV_NAME: [c_char; 0];
    static DEV_NAME: [c_char; 0];

    fn ioread8(addr: *mut c_void) -> U8;
    fn iowrite8(value: U8, addr: *mut c_void);
    fn udelay(usecs: c_uint);
    fn msleep(msecs: c_uint);
    fn cpu_relax();

    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn dev_info(dev: *mut device, fmt: *const c_char, ...);
    fn snd_devm_card_new(
        dev: *mut device,
        idx: c_int,
        xid: *mut c_char,
        module: *mut c_void,
        extra_size: usize,
        card_ret: *mut *mut snd_card,
    ) -> c_int;
    fn devm_request_region(
        dev: *mut device,
        start: c_long,
        n: c_long,
        name: *const c_char,
    ) -> *mut resource;
    fn devm_ioport_map(dev: *mut device, port: c_long, nr: c_uint) -> *mut c_void;
    fn strscpy(dst: *mut c_char, src: *const c_char) -> isize;
    fn sprintf(buf: *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn snd_wss_create(
        card: *mut snd_card,
        port: c_long,
        cport: c_long,
        irq: c_int,
        dma1: c_int,
        dma2: c_int,
        hardware: c_int,
        hwshare: c_int,
        chip: *mut *mut snd_wss,
    ) -> c_int;
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
        private_data: *mut c_void,
    ) -> c_int;
    fn snd_opl3_create(
        card: *mut snd_card,
        l_port: c_long,
        r_port: c_long,
        hardware: c_int,
        integrated: c_int,
        opl3: *mut *mut snd_opl3,
    ) -> c_int;
    fn snd_opl3_timer_new(opl3: *mut snd_opl3, timer1_dev: c_int, timer2_dev: c_int) -> c_int;
    fn snd_opl3_hwdep_new(
        opl3: *mut snd_opl3,
        device: c_int,
        seq_device: c_int,
        private_data: *mut c_void,
    ) -> c_int;
    fn snd_card_register(card: *mut snd_card) -> c_int;
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn snd_card_free_on_error(dev: *mut device, err: c_int) -> c_int;
}

extern "C" {
    static mut THIS_MODULE: *mut c_void;
}

const SNDRV_CARDS: usize = 8;
const SNDRV_AUTO_PORT: c_long = -1;
const SNDRV_AUTO_IRQ: c_int = -1;
const SNDRV_AUTO_DMA: c_int = -1;
const EIO: c_int = 5;
const ENODEV: c_int = 19;
const EBUSY: c_int = 16;
const ENOMEM: c_int = 12;
const WSS_HW_DETECT: c_int = 0;
const MPU401_HW_MPU401: c_int = 0;
const OPL3_HW_AUTO: c_int = 0;

extern "C" {
    static GALAXY_CONFIG_SBA_220: U32;
    static GALAXY_CONFIG_SBA_240: U32;
    static GALAXY_CONFIG_SBA_260: U32;
    static GALAXY_CONFIG_SBA_280: U32;
    static GALAXY_CONFIG_WSS_ENABLE: U32;
    static GALAXY_CONFIG_WSSA_530: U32;
    static GALAXY_CONFIG_WSSA_604: U32;
    static GALAXY_CONFIG_WSSA_E80: U32;
    static GALAXY_CONFIG_WSSA_F40: U32;
    static GALAXY_CONFIG_MPU_ENABLE: U32;
    static GALAXY_CONFIG_MPUA_300: U32;
    static GALAXY_CONFIG_MPUA_330: U32;
    static GALAXY_CONFIG_MPUIRQ_2: U32;
    static GALAXY_CONFIG_MPUIRQ_3: U32;
    static GALAXY_CONFIG_MPUIRQ_5: U32;
    static GALAXY_CONFIG_MPUIRQ_7: U32;
    static GALAXY_CONFIG_MPUIRQ_10: U32;
    static GALAXY_CONFIG_GAME_ENABLE: U32;
    static GALAXY_CONFIG_MASK: U32;
    static GALAXY_CONFIG_SIZE: c_int;
    static GALAXY_DSP_MAJOR: U8;
    static GALAXY_DSP_MINOR: U8;
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct resource {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_wss {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_opl3 {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_card {
    pub private_data: *mut c_void,
    pub private_free: Option<unsafe extern "C" fn(*mut snd_card)>,
    pub driver: *mut c_char,
    pub shortname: *mut c_char,
    pub longname: *mut c_char,
}

#[repr(C)]
pub struct isa_driver_driver {
    pub name: *const c_char,
}

#[repr(C)]
pub struct isa_driver {
    pub match_: Option<unsafe extern "C" fn(*mut device, c_uint) -> c_int>,
    pub probe: Option<unsafe extern "C" fn(*mut device, c_uint) -> c_int>,
    pub driver: isa_driver_driver,
}

#[repr(C)]
pub struct snd_galaxy {
    pub port: *mut c_void,
    pub config_port: *mut c_void,
    pub wss_port: *mut c_void,
    pub config: U32,
    pub res_port: *mut resource,
    pub res_config_port: *mut resource,
    pub res_wss_port: *mut resource,
}

static mut config: [U32; SNDRV_CARDS] = [0; SNDRV_CARDS];
static mut wss_config: [U8; SNDRV_CARDS] = [0; SNDRV_CARDS];

unsafe fn ioadd(port: *mut c_void, offset: usize) -> *mut c_void {
    (port as *mut u8).add(offset) as *mut c_void
}

unsafe extern "C" fn dsp_get_byte(port: *mut c_void, val: *mut U8) -> c_int {
    let mut loops: c_int = 1000;

    while ioread8(ioadd(port, DSP_PORT_DATA_AVAIL)) & 0x80 == 0 {
        if loops == 0 {
            return -EIO;
        }
        loops -= 1;
        cpu_relax();
    }
    *val = ioread8(ioadd(port, DSP_PORT_READ));
    0
}

unsafe extern "C" fn dsp_reset(port: *mut c_void) -> c_int {
    let mut val: U8 = 0;

    iowrite8(1, ioadd(port, DSP_PORT_RESET));
    udelay(10);
    iowrite8(0, ioadd(port, DSP_PORT_RESET));

    if dsp_get_byte(port, &mut val) < 0 || val != DSP_SIGNATURE {
        return -ENODEV;
    }

    0
}

unsafe extern "C" fn dsp_command(port: *mut c_void, cmd: U8) -> c_int {
    let mut loops: c_int = 1000;

    while ioread8(ioadd(port, DSP_PORT_STATUS)) & 0x80 != 0 {
        if loops == 0 {
            return -EIO;
        }
        loops -= 1;
        cpu_relax();
    }
    iowrite8(cmd, ioadd(port, DSP_PORT_COMMAND));
    0
}

unsafe extern "C" fn dsp_get_version(
    port: *mut c_void,
    major: *mut U8,
    minor: *mut U8,
) -> c_int {
    let mut err: c_int;

    err = dsp_command(port, DSP_COMMAND_GET_VERSION);
    if err < 0 {
        return err;
    }

    err = dsp_get_byte(port, major);
    if err < 0 {
        return err;
    }

    err = dsp_get_byte(port, minor);
    if err < 0 {
        return err;
    }

    0
}

unsafe extern "C" fn wss_detect(wss_port: *mut c_void) -> c_int {
    if ioread8(ioadd(wss_port, WSS_PORT_SIGNATURE)) & 0x3f != WSS_SIGNATURE {
        return -ENODEV;
    }

    0
}

unsafe extern "C" fn wss_set_config(wss_port: *mut c_void, wss_config: U8) {
    iowrite8(wss_config, ioadd(wss_port, WSS_PORT_CONFIG));
}

unsafe extern "C" fn snd_galaxy_match(dev: *mut device, n: c_uint) -> c_int {
    let n = n as usize;

    if !enable[n] {
        return 0;
    }

    match port[n] {
        SNDRV_AUTO_PORT => {
            dev_err(dev, c"please specify port\n".as_ptr());
            return 0;
        }
        0x220 => config[n] |= GALAXY_CONFIG_SBA_220,
        0x240 => config[n] |= GALAXY_CONFIG_SBA_240,
        0x260 => config[n] |= GALAXY_CONFIG_SBA_260,
        0x280 => config[n] |= GALAXY_CONFIG_SBA_280,
        _ => {
            dev_err(dev, c"invalid port %#lx\n".as_ptr(), port[n]);
            return 0;
        }
    }

    match wss_port[n] {
        SNDRV_AUTO_PORT => {
            dev_err(dev, c"please specify wss_port\n".as_ptr());
            return 0;
        }
        0x530 => config[n] |= GALAXY_CONFIG_WSS_ENABLE | GALAXY_CONFIG_WSSA_530,
        0x604 => config[n] |= GALAXY_CONFIG_WSS_ENABLE | GALAXY_CONFIG_WSSA_604,
        0xe80 => config[n] |= GALAXY_CONFIG_WSS_ENABLE | GALAXY_CONFIG_WSSA_E80,
        0xf40 => config[n] |= GALAXY_CONFIG_WSS_ENABLE | GALAXY_CONFIG_WSSA_F40,
        _ => {
            dev_err(dev, c"invalid WSS port %#lx\n".as_ptr(), wss_port[n]);
            return 0;
        }
    }

    match irq[n] {
        SNDRV_AUTO_IRQ => {
            dev_err(dev, c"please specify irq\n".as_ptr());
            return 0;
        }
        7 => wss_config[n] |= WSS_CONFIG_IRQ_7,
        2 => {
            irq[n] = 9;
            wss_config[n] |= WSS_CONFIG_IRQ_9;
        }
        9 => wss_config[n] |= WSS_CONFIG_IRQ_9,
        10 => wss_config[n] |= WSS_CONFIG_IRQ_10,
        11 => wss_config[n] |= WSS_CONFIG_IRQ_11,
        _ => {
            dev_err(dev, c"invalid IRQ %d\n".as_ptr(), irq[n]);
            return 0;
        }
    }

    match dma1[n] {
        SNDRV_AUTO_DMA => {
            dev_err(dev, c"please specify dma1\n".as_ptr());
            return 0;
        }
        0 => wss_config[n] |= WSS_CONFIG_DMA_0,
        1 => wss_config[n] |= WSS_CONFIG_DMA_1,
        3 => wss_config[n] |= WSS_CONFIG_DMA_3,
        _ => {
            dev_err(dev, c"invalid playback DMA %d\n".as_ptr(), dma1[n]);
            return 0;
        }
    }

    if dma2[n] == SNDRV_AUTO_DMA || dma2[n] == dma1[n] {
        dma2[n] = -1;
    } else {
        wss_config[n] |= WSS_CONFIG_DUPLEX;
        match dma2[n] {
            0 => {}
            1 if dma1[n] == 0 => {}
            _ => {
                dev_err(dev, c"invalid capture DMA %d\n".as_ptr(), dma2[n]);
                return 0;
            }
        }
    }

    match mpu_port[n] {
        SNDRV_AUTO_PORT => {
            dev_warn(dev, c"mpu_port not specified; not using MPU-401\n".as_ptr());
            mpu_port[n] = -1;
        }
        0x300 => config[n] |= GALAXY_CONFIG_MPU_ENABLE | GALAXY_CONFIG_MPUA_300,
        0x330 => config[n] |= GALAXY_CONFIG_MPU_ENABLE | GALAXY_CONFIG_MPUA_330,
        _ => {
            dev_err(dev, c"invalid MPU port %#lx\n".as_ptr(), mpu_port[n]);
            return 0;
        }
    }

    if mpu_port[n] >= 0 {
        match mpu_irq[n] {
            SNDRV_AUTO_IRQ => {
                dev_warn(dev, c"mpu_irq not specified: using polling mode\n".as_ptr());
                mpu_irq[n] = -1;
            }
            2 => {
                mpu_irq[n] = 9;
                config[n] |= GALAXY_CONFIG_MPUIRQ_2;
            }
            9 => config[n] |= GALAXY_CONFIG_MPUIRQ_2,
            // AZT1605: case 3: config[n] |= GALAXY_CONFIG_MPUIRQ_3;
            3 => config[n] |= GALAXY_CONFIG_MPUIRQ_3,
            5 => config[n] |= GALAXY_CONFIG_MPUIRQ_5,
            7 => config[n] |= GALAXY_CONFIG_MPUIRQ_7,
            // AZT2316: case 10: config[n] |= GALAXY_CONFIG_MPUIRQ_10;
            10 => config[n] |= GALAXY_CONFIG_MPUIRQ_10,
            _ => {
                dev_err(dev, c"invalid MPU IRQ %d\n".as_ptr(), mpu_irq[n]);
                return 0;
            }
        }

        if mpu_irq[n] == irq[n] {
            dev_err(dev, c"cannot share IRQ between WSS and MPU-401\n".as_ptr());
            return 0;
        }
    }

    match fm_port[n] {
        SNDRV_AUTO_PORT => {
            dev_warn(dev, c"fm_port not specified: not using OPL3\n".as_ptr());
            fm_port[n] = -1;
        }
        0x388 => {}
        _ => {
            dev_err(dev, c"illegal FM port %#lx\n".as_ptr(), fm_port[n]);
            return 0;
        }
    }

    config[n] |= GALAXY_CONFIG_GAME_ENABLE;
    1
}

unsafe extern "C" fn galaxy_init(galaxy: *mut snd_galaxy, type_: *mut U8) -> c_int {
    let mut major: U8 = 0;
    let mut minor: U8 = 0;
    let mut err: c_int;

    err = dsp_reset((*galaxy).port);
    if err < 0 {
        return err;
    }

    err = dsp_get_version((*galaxy).port, &mut major, &mut minor);
    if err < 0 {
        return err;
    }

    if major != GALAXY_DSP_MAJOR || minor != GALAXY_DSP_MINOR {
        return -ENODEV;
    }

    err = dsp_command((*galaxy).port, DSP_COMMAND_GALAXY_8);
    if err < 0 {
        return err;
    }

    err = dsp_command((*galaxy).port, GALAXY_COMMAND_GET_TYPE);
    if err < 0 {
        return err;
    }

    err = dsp_get_byte((*galaxy).port, type_);
    if err < 0 {
        return err;
    }

    0
}

unsafe extern "C" fn galaxy_set_mode(galaxy: *mut snd_galaxy, mode: U8) -> c_int {
    let mut err: c_int;

    err = dsp_command((*galaxy).port, DSP_COMMAND_GALAXY_9);
    if err < 0 {
        return err;
    }

    err = dsp_command((*galaxy).port, mode);
    if err < 0 {
        return err;
    }

    // AZT1605:
    // Needed for MPU IRQ on AZT1605, but AZT2316 loses WSS again.
    // err = dsp_reset((*galaxy).port); if err < 0 { return err; }

    0
}

unsafe extern "C" fn galaxy_set_config(galaxy: *mut snd_galaxy, mut config: U32) {
    let tmp: U8 = ioread8(ioadd((*galaxy).config_port, CONFIG_PORT_SET));
    let mut i: c_int;

    iowrite8(tmp | 0x80, ioadd((*galaxy).config_port, CONFIG_PORT_SET));
    i = 0;
    while i < GALAXY_CONFIG_SIZE {
        iowrite8(config as U8, ioadd((*galaxy).config_port, i as usize));
        config >>= 8;
        i += 1;
    }
    iowrite8(tmp & 0x7f, ioadd((*galaxy).config_port, CONFIG_PORT_SET));
    msleep(10);
}

unsafe extern "C" fn galaxy_config(galaxy: *mut snd_galaxy, mut config: U32) {
    let mut i: c_int;

    i = GALAXY_CONFIG_SIZE;
    while i != 0 {
        let tmp: U8 = ioread8(ioadd((*galaxy).config_port, (i - 1) as usize));
        (*galaxy).config = ((*galaxy).config << 8) | tmp as U32;
        i -= 1;
    }
    config |= (*galaxy).config & GALAXY_CONFIG_MASK;
    galaxy_set_config(galaxy, config);
}

unsafe extern "C" fn galaxy_wss_config(galaxy: *mut snd_galaxy, wss_config: U8) -> c_int {
    let mut err: c_int;

    err = wss_detect((*galaxy).wss_port);
    if err < 0 {
        return err;
    }

    wss_set_config((*galaxy).wss_port, wss_config);

    err = galaxy_set_mode(galaxy, GALAXY_MODE_WSS);
    if err < 0 {
        return err;
    }

    0
}

unsafe extern "C" fn snd_galaxy_free(card: *mut snd_card) {
    let galaxy = (*card).private_data as *mut snd_galaxy;

    if !(*galaxy).wss_port.is_null() {
        wss_set_config((*galaxy).wss_port, 0);
    }
    if !(*galaxy).config_port.is_null() {
        galaxy_set_config(galaxy, (*galaxy).config);
    }
}

unsafe extern "C" fn __snd_galaxy_probe(dev: *mut device, n: c_uint) -> c_int {
    let n = n as usize;
    let mut galaxy: *mut snd_galaxy;
    let mut chip: *mut snd_wss = ptr::null_mut();
    let mut card: *mut snd_card = ptr::null_mut();
    let mut type_: U8 = 0;
    let mut err: c_int;

    err = snd_devm_card_new(
        dev,
        index[n],
        id[n],
        THIS_MODULE,
        core::mem::size_of::<snd_galaxy>(),
        &mut card,
    );
    if err < 0 {
        return err;
    }

    (*card).private_free = Some(snd_galaxy_free);
    galaxy = (*card).private_data as *mut snd_galaxy;

    (*galaxy).res_port = devm_request_region(dev, port[n], 16, DRV_NAME.as_ptr());
    if (*galaxy).res_port.is_null() {
        dev_err(
            dev,
            c"could not grab ports %#lx-%#lx\n".as_ptr(),
            port[n],
            port[n] + 15,
        );
        return -EBUSY;
    }
    (*galaxy).port = devm_ioport_map(dev, port[n], 16);
    if (*galaxy).port.is_null() {
        return -ENOMEM;
    }

    err = galaxy_init(galaxy, &mut type_);
    if err < 0 {
        dev_err(dev, c"did not find a Sound Galaxy at %#lx\n".as_ptr(), port[n]);
        return err;
    }
    dev_info(
        dev,
        c"Sound Galaxy (type %d) found at %#lx\n".as_ptr(),
        type_ as c_int,
        port[n],
    );

    (*galaxy).res_config_port =
        devm_request_region(dev, port[n] + GALAXY_PORT_CONFIG, 16, DRV_NAME.as_ptr());
    if (*galaxy).res_config_port.is_null() {
        dev_err(
            dev,
            c"could not grab ports %#lx-%#lx\n".as_ptr(),
            port[n] + GALAXY_PORT_CONFIG,
            port[n] + GALAXY_PORT_CONFIG + 15,
        );
        return -EBUSY;
    }
    (*galaxy).config_port = devm_ioport_map(dev, port[n] + GALAXY_PORT_CONFIG, 16);
    if (*galaxy).config_port.is_null() {
        return -ENOMEM;
    }
    galaxy_config(galaxy, config[n]);

    (*galaxy).res_wss_port = devm_request_region(dev, wss_port[n], 4, DRV_NAME.as_ptr());
    if (*galaxy).res_wss_port.is_null() {
        dev_err(
            dev,
            c"could not grab ports %#lx-%#lx\n".as_ptr(),
            wss_port[n],
            wss_port[n] + 3,
        );
        return -EBUSY;
    }
    (*galaxy).wss_port = devm_ioport_map(dev, wss_port[n], 4);
    if (*galaxy).wss_port.is_null() {
        return -ENOMEM;
    }

    err = galaxy_wss_config(galaxy, wss_config[n]);
    if err < 0 {
        dev_err(dev, c"could not configure WSS\n".as_ptr());
        return err;
    }

    strscpy((*card).driver, DRV_NAME.as_ptr());
    strscpy((*card).shortname, DRV_NAME.as_ptr());
    sprintf(
        (*card).longname,
        c"%s at %#lx/%#lx, irq %d, dma %d/%d".as_ptr(),
        (*card).shortname,
        port[n],
        wss_port[n],
        irq[n],
        dma1[n],
        dma2[n],
    );

    err = snd_wss_create(
        card,
        wss_port[n] + 4,
        -1,
        irq[n],
        dma1[n],
        dma2[n],
        WSS_HW_DETECT,
        0,
        &mut chip,
    );
    if err < 0 {
        return err;
    }

    err = snd_wss_pcm(chip, 0);
    if err < 0 {
        return err;
    }

    err = snd_wss_mixer(chip);
    if err < 0 {
        return err;
    }

    err = snd_wss_timer(chip, 0);
    if err < 0 {
        return err;
    }

    if mpu_port[n] >= 0 {
        err = snd_mpu401_uart_new(
            card,
            0,
            MPU401_HW_MPU401,
            mpu_port[n],
            0,
            mpu_irq[n],
            ptr::null_mut(),
        );
        if err < 0 {
            return err;
        }
    }

    if fm_port[n] >= 0 {
        let mut opl3: *mut snd_opl3 = ptr::null_mut();

        err = snd_opl3_create(
            card,
            fm_port[n],
            fm_port[n] + 2,
            OPL3_HW_AUTO,
            0,
            &mut opl3,
        );
        if err < 0 {
            dev_err(dev, c"no OPL device at %#lx\n".as_ptr(), fm_port[n]);
            return err;
        }
        err = snd_opl3_timer_new(opl3, 1, 2);
        if err < 0 {
            return err;
        }

        err = snd_opl3_hwdep_new(opl3, 0, 1, ptr::null_mut());
        if err < 0 {
            return err;
        }
    }

    err = snd_card_register(card);
    if err < 0 {
        return err;
    }

    dev_set_drvdata(dev, card as *mut c_void);
    0
}

unsafe extern "C" fn snd_galaxy_probe(dev: *mut device, n: c_uint) -> c_int {
    snd_card_free_on_error(dev, __snd_galaxy_probe(dev, n))
}

static mut snd_galaxy_driver: isa_driver = isa_driver {
    match_: Some(snd_galaxy_match),
    probe: Some(snd_galaxy_probe),
    driver: isa_driver_driver {
        name: unsafe { DEV_NAME.as_ptr() },
    },
};

// module metadata:
// MODULE_DESCRIPTION(CRD_NAME);
// MODULE_AUTHOR("Rene Herman");
// MODULE_LICENSE("GPL");
// module parameters for index, id, enable, port, wss_port, mpu_port, fm_port,
// irq, mpu_irq, dma1, and dma2 are declared by the original C file.
// module_isa_driver(snd_galaxy_driver, SNDRV_CARDS);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
