// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Driver for Gravis UltraSound Classic soundcard
 *  Copyright (c) by Jaroslav Kysela <perex@perex.cz>
 */

// C dependencies removed from executable Rust:
// linux/init.h, linux/err.h, linux/isa.h, linux/delay.h, linux/time.h,
// linux/module.h, asm/dma.h, sound/core.h, sound/gus.h, sound/initval.h.
// The original file defined SNDRV_LEGACY_FIND_FREE_IRQ and
// SNDRV_LEGACY_FIND_FREE_DMA before including sound/initval.h.

const CRD_NAME: &[u8] = b"Gravis UltraSound Classic\0";
const DEV_NAME: &[u8] = b"gusclassic\0";

// MODULE_DESCRIPTION(CRD_NAME);
// MODULE_AUTHOR("Jaroslav Kysela <perex@perex.cz>");
// MODULE_LICENSE("GPL");

extern "C" {
    static THIS_MODULE: *mut module;

    static SNDRV_DEFAULT_IDX: [::core::ffi::c_int; SNDRV_CARDS];
    static SNDRV_DEFAULT_STR: [*mut ::core::ffi::c_char; SNDRV_CARDS];
    static SNDRV_DEFAULT_ENABLE: [bool; SNDRV_CARDS];
    static SNDRV_DEFAULT_PORT: [::core::ffi::c_long; SNDRV_CARDS];
    static SNDRV_DEFAULT_IRQ: [::core::ffi::c_int; SNDRV_CARDS];
    static SNDRV_DEFAULT_DMA: [::core::ffi::c_int; SNDRV_CARDS];

    fn snd_legacy_find_free_irq(table: *const ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn snd_legacy_find_free_dma(table: *const ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn snd_gus_create(
        card: *mut snd_card,
        port: ::core::ffi::c_long,
        irq: ::core::ffi::c_int,
        dma1: ::core::ffi::c_int,
        dma2: ::core::ffi::c_int,
        timer_dev: ::core::ffi::c_int,
        voices: ::core::ffi::c_int,
        pcm_channels: ::core::ffi::c_int,
        effect: ::core::ffi::c_int,
        rgus: *mut *mut snd_gus_card,
    ) -> ::core::ffi::c_int;
    fn snd_gf1_i_write8(gus: *mut snd_gus_card, reg: ::core::ffi::c_int, data: ::core::ffi::c_uchar);
    fn snd_gf1_i_look8(gus: *mut snd_gus_card, reg: ::core::ffi::c_int) -> ::core::ffi::c_uchar;
    fn udelay(usecs: ::core::ffi::c_ulong);
    fn snd_devm_card_new(
        dev: *mut device,
        idx: ::core::ffi::c_int,
        xid: *mut ::core::ffi::c_char,
        module: *mut module,
        extra_size: ::core::ffi::c_int,
        card_ret: *mut *mut snd_card,
    ) -> ::core::ffi::c_int;
    fn snd_gus_initialize(gus: *mut snd_gus_card) -> ::core::ffi::c_int;
    fn snd_gf1_new_mixer(gus: *mut snd_gus_card) -> ::core::ffi::c_int;
    fn snd_gf1_pcm_new(
        gus: *mut snd_gus_card,
        pcm_dev: ::core::ffi::c_int,
        control_index: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn snd_gf1_rawmidi_new(gus: *mut snd_gus_card, device: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn strlen(s: *const ::core::ffi::c_char) -> usize;
    fn sprintf(s: *mut ::core::ffi::c_char, format: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn snd_card_register(card: *mut snd_card) -> ::core::ffi::c_int;
    fn dev_set_drvdata(dev: *mut device, data: *mut ::core::ffi::c_void);
    fn dev_get_drvdata(dev: *mut device) -> *mut ::core::ffi::c_void;
    fn snd_gus_suspend(private_data: *mut ::core::ffi::c_void) -> ::core::ffi::c_int;
    fn snd_gus_resume(private_data: *mut ::core::ffi::c_void) -> ::core::ffi::c_int;
    fn dev_err(dev: *mut device, format: *const ::core::ffi::c_char, ...);
    fn dev_dbg(dev: *mut device, format: *const ::core::ffi::c_char, ...);
}

extern "C" {
    static SNDRV_CARDS: usize;
    static SNDRV_AUTO_IRQ: ::core::ffi::c_int;
    static SNDRV_AUTO_DMA: ::core::ffi::c_int;
    static SNDRV_AUTO_PORT: ::core::ffi::c_long;
    static SNDRV_GF1_GB_RESET: ::core::ffi::c_int;
    static EBUSY: ::core::ffi::c_int;
    static ENODEV: ::core::ffi::c_int;
}

#[repr(C)]
pub struct module {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct device {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct pm_message_t {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct snd_card {
    pub dev: *mut device,
    pub private_data: *mut ::core::ffi::c_void,
    pub longname: [::core::ffi::c_char; 80],
}

#[repr(C)]
pub struct snd_gus_gf1 {
    pub port: ::core::ffi::c_ulong,
    pub irq: ::core::ffi::c_int,
    pub dma1: ::core::ffi::c_int,
    pub dma2: ::core::ffi::c_int,
}

#[repr(C)]
pub struct snd_gus_card {
    pub card: *mut snd_card,
    pub gf1: snd_gus_gf1,
    pub joystick_dac: ::core::ffi::c_int,
    pub max_flag: ::core::ffi::c_int,
    pub ess_flag: ::core::ffi::c_int,
    pub ace_flag: ::core::ffi::c_int,
}

#[repr(C)]
pub struct isa_driver_inner {
    pub name: *const ::core::ffi::c_char,
}

#[repr(C)]
pub struct isa_driver {
    pub match_: Option<unsafe extern "C" fn(*mut device, ::core::ffi::c_uint) -> ::core::ffi::c_int>,
    pub probe: Option<unsafe extern "C" fn(*mut device, ::core::ffi::c_uint) -> ::core::ffi::c_int>,
    // Present in the original only under CONFIG_PM.
    pub suspend: Option<
        unsafe extern "C" fn(*mut device, ::core::ffi::c_uint, pm_message_t) -> ::core::ffi::c_int,
    >,
    // Present in the original only under CONFIG_PM.
    pub resume: Option<unsafe extern "C" fn(*mut device, ::core::ffi::c_uint) -> ::core::ffi::c_int>,
    pub driver: isa_driver_inner,
}

static mut index: [::core::ffi::c_int; SNDRV_CARDS] = unsafe { SNDRV_DEFAULT_IDX }; /* Index 0-MAX */
static mut id: [*mut ::core::ffi::c_char; SNDRV_CARDS] = unsafe { SNDRV_DEFAULT_STR }; /* ID for this card */
static mut enable: [bool; SNDRV_CARDS] = unsafe { SNDRV_DEFAULT_ENABLE }; /* Enable this card */
static mut port: [::core::ffi::c_long; SNDRV_CARDS] = unsafe { SNDRV_DEFAULT_PORT }; /* 0x220,0x230,0x240,0x250,0x260 */
static mut irq: [::core::ffi::c_int; SNDRV_CARDS] = unsafe { SNDRV_DEFAULT_IRQ }; /* 3,5,9,11,12,15 */
static mut dma1: [::core::ffi::c_int; SNDRV_CARDS] = unsafe { SNDRV_DEFAULT_DMA }; /* 1,3,5,6,7 */
static mut dma2: [::core::ffi::c_int; SNDRV_CARDS] = unsafe { SNDRV_DEFAULT_DMA }; /* 1,3,5,6,7 */
static mut joystick_dac: [::core::ffi::c_int; SNDRV_CARDS] = [29; SNDRV_CARDS];
/* 0 to 31, (0.59V-4.52V or 0.389V-2.98V) */
static mut channels: [::core::ffi::c_int; SNDRV_CARDS] = [24; SNDRV_CARDS];
static mut pcm_channels: [::core::ffi::c_int; SNDRV_CARDS] = [2; SNDRV_CARDS];

// module_param_array(index, int, NULL, 0444);
// MODULE_PARM_DESC(index, "Index value for " CRD_NAME " soundcard.");
// module_param_array(id, charp, NULL, 0444);
// MODULE_PARM_DESC(id, "ID string for " CRD_NAME " soundcard.");
// module_param_array(enable, bool, NULL, 0444);
// MODULE_PARM_DESC(enable, "Enable " CRD_NAME " soundcard.");
// module_param_hw_array(port, long, ioport, NULL, 0444);
// MODULE_PARM_DESC(port, "Port # for " CRD_NAME " driver.");
// module_param_hw_array(irq, int, irq, NULL, 0444);
// MODULE_PARM_DESC(irq, "IRQ # for " CRD_NAME " driver.");
// module_param_hw_array(dma1, int, dma, NULL, 0444);
// MODULE_PARM_DESC(dma1, "DMA1 # for " CRD_NAME " driver.");
// module_param_hw_array(dma2, int, dma, NULL, 0444);
// MODULE_PARM_DESC(dma2, "DMA2 # for " CRD_NAME " driver.");
// module_param_array(joystick_dac, int, NULL, 0444);
// MODULE_PARM_DESC(joystick_dac, "Joystick DAC level 0.59V-4.52V or 0.389V-2.98V for " CRD_NAME " driver.");
// module_param_array(channels, int, NULL, 0444);
// MODULE_PARM_DESC(channels, "GF1 channels for " CRD_NAME " driver.");
// module_param_array(pcm_channels, int, NULL, 0444);
// MODULE_PARM_DESC(pcm_channels, "Reserved PCM channels for " CRD_NAME " driver.");

unsafe extern "C" fn snd_gusclassic_match(
    _dev: *mut device,
    n: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    enable[n as usize] as ::core::ffi::c_int
}

unsafe extern "C" fn snd_gusclassic_create(
    card: *mut snd_card,
    dev: *mut device,
    n: ::core::ffi::c_uint,
    rgus: *mut *mut snd_gus_card,
) -> ::core::ffi::c_int {
    static possible_ports: [::core::ffi::c_long; 5] = [0x220, 0x230, 0x240, 0x250, 0x260];
    static possible_irqs: [::core::ffi::c_int; 9] = [5, 11, 12, 9, 7, 15, 3, 4, -1];
    static possible_dmas: [::core::ffi::c_int; 6] = [5, 6, 7, 1, 3, -1];

    let mut i: ::core::ffi::c_int;
    let mut error: ::core::ffi::c_int;
    let n = n as usize;

    if irq[n] == SNDRV_AUTO_IRQ {
        irq[n] = snd_legacy_find_free_irq(possible_irqs.as_ptr());
        if irq[n] < 0 {
            dev_err(dev, b"unable to find a free IRQ\n\0".as_ptr() as *const _);
            return -EBUSY;
        }
    }
    if dma1[n] == SNDRV_AUTO_DMA {
        dma1[n] = snd_legacy_find_free_dma(possible_dmas.as_ptr());
        if dma1[n] < 0 {
            dev_err(dev, b"unable to find a free DMA1\n\0".as_ptr() as *const _);
            return -EBUSY;
        }
    }
    if dma2[n] == SNDRV_AUTO_DMA {
        dma2[n] = snd_legacy_find_free_dma(possible_dmas.as_ptr());
        if dma2[n] < 0 {
            dev_err(dev, b"unable to find a free DMA2\n\0".as_ptr() as *const _);
            return -EBUSY;
        }
    }

    if port[n] != SNDRV_AUTO_PORT {
        return snd_gus_create(
            card,
            port[n],
            irq[n],
            dma1[n],
            dma2[n],
            0,
            channels[n],
            pcm_channels[n],
            0,
            rgus,
        );
    }

    i = 0;
    loop {
        port[n] = possible_ports[i as usize];
        error = snd_gus_create(
            card,
            port[n],
            irq[n],
            dma1[n],
            dma2[n],
            0,
            channels[n],
            pcm_channels[n],
            0,
            rgus,
        );
        i += 1;
        if !(error < 0 && (i as usize) < possible_ports.len()) {
            break;
        }
    }

    error
}

unsafe extern "C" fn snd_gusclassic_detect(gus: *mut snd_gus_card) -> ::core::ffi::c_int {
    let mut d: ::core::ffi::c_uchar;

    snd_gf1_i_write8(gus, SNDRV_GF1_GB_RESET, 0); /* reset GF1 */
    d = snd_gf1_i_look8(gus, SNDRV_GF1_GB_RESET);
    if (d & 0x07) != 0 {
        dev_dbg(
            (*(*gus).card).dev,
            b"[0x%lx] check 1 failed - 0x%x\n\0".as_ptr() as *const _,
            (*gus).gf1.port,
            d as ::core::ffi::c_int,
        );
        return -ENODEV;
    }
    udelay(160);
    snd_gf1_i_write8(gus, SNDRV_GF1_GB_RESET, 1); /* release reset */
    udelay(160);
    d = snd_gf1_i_look8(gus, SNDRV_GF1_GB_RESET);
    if (d & 0x07) != 1 {
        dev_dbg(
            (*(*gus).card).dev,
            b"[0x%lx] check 2 failed - 0x%x\n\0".as_ptr() as *const _,
            (*gus).gf1.port,
            d as ::core::ffi::c_int,
        );
        return -ENODEV;
    }
    0
}

unsafe extern "C" fn snd_gusclassic_probe(
    dev: *mut device,
    n: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    let mut card: *mut snd_card = ::core::ptr::null_mut();
    let mut gus: *mut snd_gus_card = ::core::ptr::null_mut();
    let mut error: ::core::ffi::c_int;
    let n = n as usize;

    error = snd_devm_card_new(dev, index[n], id[n], THIS_MODULE, 0, &mut card);
    if error < 0 {
        return error;
    }

    if pcm_channels[n] < 2 {
        pcm_channels[n] = 2;
    }

    error = snd_gusclassic_create(card, dev, n as ::core::ffi::c_uint, &mut gus);
    if error < 0 {
        return error;
    }
    (*card).private_data = gus as *mut ::core::ffi::c_void;

    error = snd_gusclassic_detect(gus);
    if error < 0 {
        return error;
    }

    (*gus).joystick_dac = joystick_dac[n];

    error = snd_gus_initialize(gus);
    if error < 0 {
        return error;
    }

    error = -ENODEV;
    if (*gus).max_flag != 0 || (*gus).ess_flag != 0 {
        dev_err(
            dev,
            b"GUS Classic or ACE soundcard was not detected at 0x%lx\n\0".as_ptr() as *const _,
            (*gus).gf1.port,
        );
        return error;
    }

    error = snd_gf1_new_mixer(gus);
    if error < 0 {
        return error;
    }

    error = snd_gf1_pcm_new(gus, 0, 0);
    if error < 0 {
        return error;
    }

    if (*gus).ace_flag == 0 {
        error = snd_gf1_rawmidi_new(gus, 0);
        if error < 0 {
            return error;
        }
    }

    sprintf(
        (*card).longname.as_mut_ptr().add(strlen((*card).longname.as_ptr())),
        b" at 0x%lx, irq %d, dma %d\0".as_ptr() as *const _,
        (*gus).gf1.port,
        (*gus).gf1.irq,
        (*gus).gf1.dma1,
    );

    if (*gus).gf1.dma2 >= 0 {
        sprintf(
            (*card).longname.as_mut_ptr().add(strlen((*card).longname.as_ptr())),
            b"&%d\0".as_ptr() as *const _,
            (*gus).gf1.dma2,
        );
    }

    error = snd_card_register(card);
    if error < 0 {
        return error;
    }

    dev_set_drvdata(dev, card as *mut ::core::ffi::c_void);
    0
}

// Original code conditionally includes these callbacks under CONFIG_PM.
unsafe extern "C" fn snd_gusclassic_suspend(
    dev: *mut device,
    _n: ::core::ffi::c_uint,
    _state: pm_message_t,
) -> ::core::ffi::c_int {
    let card: *mut snd_card = dev_get_drvdata(dev) as *mut snd_card;

    snd_gus_suspend((*card).private_data)
}

// Original code conditionally includes this callback under CONFIG_PM.
unsafe extern "C" fn snd_gusclassic_resume(
    dev: *mut device,
    _n: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    let card: *mut snd_card = dev_get_drvdata(dev) as *mut snd_card;

    snd_gus_resume((*card).private_data)
}

static mut snd_gusclassic_driver: isa_driver = isa_driver {
    match_: Some(snd_gusclassic_match),
    probe: Some(snd_gusclassic_probe),
    // Original initializer includes these fields only under CONFIG_PM.
    suspend: Some(snd_gusclassic_suspend),
    resume: Some(snd_gusclassic_resume),
    driver: isa_driver_inner {
        name: DEV_NAME.as_ptr() as *const ::core::ffi::c_char,
    },
};

// module_isa_driver(snd_gusclassic_driver, SNDRV_CARDS);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
