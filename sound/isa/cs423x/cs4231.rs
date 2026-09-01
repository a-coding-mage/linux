// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Generic driver for CS4231 chips
 *  Copyright (c) by Jaroslav Kysela <perex@perex.cz>
 *  Originally the CS4232/CS4232A driver, modified for use on CS4231 by
 *  Tugrul Galatali <galatalt@stuy.edu>
 */

// C includes translated as dependency intent:
// linux/init.h, linux/err.h, linux/isa.h, linux/time.h, linux/wait.h,
// linux/module.h, sound/core.h, sound/wss.h, sound/mpu401.h, sound/initval.h

const CRD_NAME: &[u8] = b"Generic CS4231\0";
const DEV_NAME: &[u8] = b"cs4231\0";

// MODULE_DESCRIPTION(CRD_NAME);
// MODULE_AUTHOR("Jaroslav Kysela <perex@perex.cz>");
// MODULE_LICENSE("GPL");

extern "C" {
    static SNDRV_CARDS: usize;
    static SNDRV_DEFAULT_IDX: [::core::ffi::c_int; 0];
    static SNDRV_DEFAULT_STR: [*mut ::core::ffi::c_char; 0];
    static SNDRV_DEFAULT_ENABLE: [bool; 0];
    static SNDRV_DEFAULT_PORT: [::core::ffi::c_long; 0];
    static SNDRV_DEFAULT_IRQ: [::core::ffi::c_int; 0];
    static SNDRV_DEFAULT_DMA: [::core::ffi::c_int; 0];

    static SNDRV_AUTO_PORT: ::core::ffi::c_long;
    static SNDRV_AUTO_IRQ: ::core::ffi::c_int;
    static SNDRV_AUTO_DMA: ::core::ffi::c_int;
    static WSS_HW_DETECT: ::core::ffi::c_int;
    static MPU401_HW_CS4232: ::core::ffi::c_int;
    static SNDRV_CTL_POWER_D3hot: ::core::ffi::c_int;
    static SNDRV_CTL_POWER_D0: ::core::ffi::c_int;
    static THIS_MODULE: *mut module;

    fn dev_err(dev: *mut device, fmt: *const ::core::ffi::c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const ::core::ffi::c_char, ...);
    fn snd_devm_card_new(
        dev: *mut device,
        idx: ::core::ffi::c_int,
        xid: *mut ::core::ffi::c_char,
        module: *mut module,
        extra_size: ::core::ffi::c_int,
        card_ret: *mut *mut snd_card,
    ) -> ::core::ffi::c_int;
    fn snd_wss_create(
        card: *mut snd_card,
        port: ::core::ffi::c_long,
        cport: ::core::ffi::c_int,
        irq: ::core::ffi::c_int,
        dma1: ::core::ffi::c_int,
        dma2: ::core::ffi::c_int,
        hardware: ::core::ffi::c_int,
        hwshare: ::core::ffi::c_int,
        chip_ret: *mut *mut snd_wss,
    ) -> ::core::ffi::c_int;
    fn snd_wss_pcm(chip: *mut snd_wss, device: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn strscpy(
        dest: *mut ::core::ffi::c_char,
        src: *const ::core::ffi::c_char,
        count: usize,
    ) -> isize;
    fn scnprintf(
        buf: *mut ::core::ffi::c_char,
        size: usize,
        fmt: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn snd_wss_mixer(chip: *mut snd_wss) -> ::core::ffi::c_int;
    fn snd_wss_timer(chip: *mut snd_wss, device: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn snd_mpu401_uart_new(
        card: *mut snd_card,
        device: ::core::ffi::c_int,
        hardware: ::core::ffi::c_int,
        port: ::core::ffi::c_long,
        integrated: ::core::ffi::c_int,
        irq: ::core::ffi::c_int,
        private_data: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn snd_card_register(card: *mut snd_card) -> ::core::ffi::c_int;
    fn dev_set_drvdata(dev: *mut device, data: *mut ::core::ffi::c_void);
    fn dev_get_drvdata(dev: *mut device) -> *mut ::core::ffi::c_void;
    fn snd_power_change_state(card: *mut snd_card, state: ::core::ffi::c_int);
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct module {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm {
    pub name: [::core::ffi::c_char; 0],
}

#[repr(C)]
pub struct snd_card {
    pub private_data: *mut ::core::ffi::c_void,
    pub driver: [::core::ffi::c_char; 0],
    pub shortname: [::core::ffi::c_char; 0],
    pub longname: [::core::ffi::c_char; 0],
}

#[repr(C)]
pub struct snd_wss {
    pub pcm: *mut snd_pcm,
    pub port: ::core::ffi::c_ulong,
    pub suspend: unsafe extern "C" fn(chip: *mut snd_wss),
    pub resume: unsafe extern "C" fn(chip: *mut snd_wss),
}

#[repr(C)]
pub struct pm_message_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device_driver {
    pub name: *const ::core::ffi::c_char,
}

#[repr(C)]
pub struct isa_driver {
    pub match_: Option<unsafe extern "C" fn(dev: *mut device, n: ::core::ffi::c_uint) -> ::core::ffi::c_int>,
    pub probe: Option<unsafe extern "C" fn(dev: *mut device, n: ::core::ffi::c_uint) -> ::core::ffi::c_int>,
    // Present when CONFIG_PM is enabled in the original C source.
    #[cfg(CONFIG_PM)]
    pub suspend: Option<
        unsafe extern "C" fn(
            dev: *mut device,
            n: ::core::ffi::c_uint,
            state: pm_message_t,
        ) -> ::core::ffi::c_int,
    >,
    #[cfg(CONFIG_PM)]
    pub resume: Option<unsafe extern "C" fn(dev: *mut device, n: ::core::ffi::c_uint) -> ::core::ffi::c_int>,
    pub driver: device_driver,
}

static mut index: [::core::ffi::c_int; SNDRV_CARDS] = SNDRV_DEFAULT_IDX; /* Index 0-MAX */
static mut id: [*mut ::core::ffi::c_char; SNDRV_CARDS] = SNDRV_DEFAULT_STR; /* ID for this card */
static mut enable: [bool; SNDRV_CARDS] = SNDRV_DEFAULT_ENABLE; /* Enable this card */
static mut port: [::core::ffi::c_long; SNDRV_CARDS] = SNDRV_DEFAULT_PORT; /* PnP setup */
static mut mpu_port: [::core::ffi::c_long; SNDRV_CARDS] = SNDRV_DEFAULT_PORT; /* PnP setup */
static mut irq: [::core::ffi::c_int; SNDRV_CARDS] = SNDRV_DEFAULT_IRQ; /* 5,7,9,11,12,15 */
static mut mpu_irq: [::core::ffi::c_int; SNDRV_CARDS] = SNDRV_DEFAULT_IRQ; /* 9,11,12,15 */
static mut dma1: [::core::ffi::c_int; SNDRV_CARDS] = SNDRV_DEFAULT_DMA; /* 0,1,3,5,6,7 */
static mut dma2: [::core::ffi::c_int; SNDRV_CARDS] = SNDRV_DEFAULT_DMA; /* 0,1,3,5,6,7 */

// module_param_array(index, int, NULL, 0444);
// MODULE_PARM_DESC(index, "Index value for " CRD_NAME " soundcard.");
// module_param_array(id, charp, NULL, 0444);
// MODULE_PARM_DESC(id, "ID string for " CRD_NAME " soundcard.");
// module_param_array(enable, bool, NULL, 0444);
// MODULE_PARM_DESC(enable, "Enable " CRD_NAME " soundcard.");
// module_param_hw_array(port, long, ioport, NULL, 0444);
// MODULE_PARM_DESC(port, "Port # for " CRD_NAME " driver.");
// module_param_hw_array(mpu_port, long, ioport, NULL, 0444);
// MODULE_PARM_DESC(mpu_port, "MPU-401 port # for " CRD_NAME " driver.");
// module_param_hw_array(irq, int, irq, NULL, 0444);
// MODULE_PARM_DESC(irq, "IRQ # for " CRD_NAME " driver.");
// module_param_hw_array(mpu_irq, int, irq, NULL, 0444);
// MODULE_PARM_DESC(mpu_irq, "MPU-401 IRQ # for " CRD_NAME " driver.");
// module_param_hw_array(dma1, int, dma, NULL, 0444);
// MODULE_PARM_DESC(dma1, "DMA1 # for " CRD_NAME " driver.");
// module_param_hw_array(dma2, int, dma, NULL, 0444);
// MODULE_PARM_DESC(dma2, "DMA2 # for " CRD_NAME " driver.");

unsafe extern "C" fn snd_cs4231_match(dev: *mut device, n: ::core::ffi::c_uint) -> ::core::ffi::c_int {
    let n = n as usize;

    if !enable[n] {
        return 0;
    }

    if port[n] == SNDRV_AUTO_PORT {
        dev_err(dev, b"please specify port\n\0".as_ptr() as *const ::core::ffi::c_char);
        return 0;
    }
    if irq[n] == SNDRV_AUTO_IRQ {
        dev_err(dev, b"please specify irq\n\0".as_ptr() as *const ::core::ffi::c_char);
        return 0;
    }
    if dma1[n] == SNDRV_AUTO_DMA {
        dev_err(dev, b"please specify dma1\n\0".as_ptr() as *const ::core::ffi::c_char);
        return 0;
    }
    1
}

unsafe extern "C" fn snd_cs4231_probe(dev: *mut device, n: ::core::ffi::c_uint) -> ::core::ffi::c_int {
    let n = n as usize;
    let mut card: *mut snd_card = ::core::ptr::null_mut();
    let mut chip: *mut snd_wss = ::core::ptr::null_mut();
    let mut error: ::core::ffi::c_int;

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
        dma2[n],
        WSS_HW_DETECT,
        0,
        &mut chip,
    );
    if error < 0 {
        return error;
    }

    (*card).private_data = chip as *mut ::core::ffi::c_void;

    error = snd_wss_pcm(chip, 0);
    if error < 0 {
        return error;
    }

    strscpy(
        (*card).driver.as_mut_ptr(),
        b"CS4231\0".as_ptr() as *const ::core::ffi::c_char,
        ::core::mem::size_of_val(&(*card).driver),
    );
    strscpy(
        (*card).shortname.as_mut_ptr(),
        (*(*chip).pcm).name.as_ptr(),
        ::core::mem::size_of_val(&(*card).shortname),
    );

    if dma2[n] < 0 {
        scnprintf(
            (*card).longname.as_mut_ptr(),
            ::core::mem::size_of_val(&(*card).longname),
            b"%s at 0x%lx, irq %d, dma %d\0".as_ptr() as *const ::core::ffi::c_char,
            (*(*chip).pcm).name.as_ptr(),
            (*chip).port,
            irq[n],
            dma1[n],
        );
    } else {
        scnprintf(
            (*card).longname.as_mut_ptr(),
            ::core::mem::size_of_val(&(*card).longname),
            b"%s at 0x%lx, irq %d, dma %d&%d\0".as_ptr() as *const ::core::ffi::c_char,
            (*(*chip).pcm).name.as_ptr(),
            (*chip).port,
            irq[n],
            dma1[n],
            dma2[n],
        );
    }

    error = snd_wss_mixer(chip);
    if error < 0 {
        return error;
    }

    error = snd_wss_timer(chip, 0);
    if error < 0 {
        return error;
    }

    if mpu_port[n] > 0 && mpu_port[n] != SNDRV_AUTO_PORT {
        if mpu_irq[n] == SNDRV_AUTO_IRQ {
            mpu_irq[n] = -1;
        }
        if snd_mpu401_uart_new(
            card,
            0,
            MPU401_HW_CS4232,
            mpu_port[n],
            0,
            mpu_irq[n],
            ::core::ptr::null_mut(),
        ) < 0
        {
            dev_warn(dev, b"MPU401 not detected\n\0".as_ptr() as *const ::core::ffi::c_char);
        }
    }

    error = snd_card_register(card);
    if error < 0 {
        return error;
    }

    dev_set_drvdata(dev, card as *mut ::core::ffi::c_void);
    0
}

#[cfg(CONFIG_PM)]
unsafe extern "C" fn snd_cs4231_suspend(
    dev: *mut device,
    _n: ::core::ffi::c_uint,
    _state: pm_message_t,
) -> ::core::ffi::c_int {
    let card: *mut snd_card = dev_get_drvdata(dev) as *mut snd_card;
    let chip: *mut snd_wss = (*card).private_data as *mut snd_wss;

    snd_power_change_state(card, SNDRV_CTL_POWER_D3hot);
    ((*chip).suspend)(chip);
    0
}

#[cfg(CONFIG_PM)]
unsafe extern "C" fn snd_cs4231_resume(dev: *mut device, _n: ::core::ffi::c_uint) -> ::core::ffi::c_int {
    let card: *mut snd_card = dev_get_drvdata(dev) as *mut snd_card;
    let chip: *mut snd_wss = (*card).private_data as *mut snd_wss;

    ((*chip).resume)(chip);
    snd_power_change_state(card, SNDRV_CTL_POWER_D0);
    0
}

static mut snd_cs4231_driver: isa_driver = isa_driver {
    match_: Some(snd_cs4231_match),
    probe: Some(snd_cs4231_probe),
    #[cfg(CONFIG_PM)]
    suspend: Some(snd_cs4231_suspend),
    #[cfg(CONFIG_PM)]
    resume: Some(snd_cs4231_resume),
    driver: device_driver {
        name: DEV_NAME.as_ptr() as *const ::core::ffi::c_char,
    },
};

// module_isa_driver(snd_cs4231_driver, SNDRV_CARDS);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
