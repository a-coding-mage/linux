// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Driver for Gravis UltraSound Extreme soundcards
 *  Copyright (c) by Jaroslav Kysela <perex@perex.cz>
 */

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

// C includes translated as external dependencies:
// linux/init.h, linux/err.h, linux/isa.h, linux/delay.h, linux/time.h,
// linux/module.h, asm/dma.h, sound/core.h, sound/gus.h, sound/es1688.h,
// sound/mpu401.h, sound/opl3.h, sound/initval.h.
// SNDRV_LEGACY_AUTO_PROBE, SNDRV_LEGACY_FIND_FREE_IRQ, and
// SNDRV_LEGACY_FIND_FREE_DMA were defined before including sound/initval.h.

const CRD_NAME: &[u8] = b"Gravis UltraSound Extreme\0";
const DEV_NAME: &[u8] = b"gusextreme\0";

// MODULE_DESCRIPTION(CRD_NAME);
// MODULE_AUTHOR("Jaroslav Kysela <perex@perex.cz>");
// MODULE_LICENSE("GPL");

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct module {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_card {
    pub private_data: *mut c_void,
    pub dev: *mut device,
    pub longname: [c_char; 80],
}

#[repr(C)]
pub struct snd_es1688 {
    pub mixer_lock: c_void,
    pub reg_lock: c_void,
    pub port: c_long,
    pub irq: c_int,
    pub dma8: c_int,
    pub mpu_port: c_long,
}

#[repr(C)]
pub struct snd_gf1 {
    pub port: c_ulong,
    pub irq: c_int,
    pub dma1: c_int,
}

#[repr(C)]
pub struct snd_gus_card {
    pub card: *mut snd_card,
    pub gf1: snd_gf1,
    pub joystick_dac: c_int,
    pub ess_flag: c_int,
    pub codec_flag: c_int,
}

#[repr(C)]
pub struct snd_opl3 {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_ctl_elem_id {
    pub numid: c_uint,
    pub iface: c_uint,
    pub device: c_uint,
    pub subdevice: c_uint,
    pub name: [c_char; 44],
    pub index: c_uint,
}

#[repr(C)]
pub struct pm_message_t {
    pub event: c_int,
}

#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
}

#[repr(C)]
pub struct isa_driver {
    pub match_: Option<unsafe extern "C" fn(*mut device, c_uint) -> c_int>,
    pub probe: Option<unsafe extern "C" fn(*mut device, c_uint) -> c_int>,
    // CONFIG_PM fields are present in C only when CONFIG_PM is enabled.
    pub suspend: Option<unsafe extern "C" fn(*mut device, c_uint, pm_message_t) -> c_int>,
    pub resume: Option<unsafe extern "C" fn(*mut device, c_uint) -> c_int>,
    pub driver: device_driver,
}

unsafe extern "C" {
    static mut THIS_MODULE: *mut module;

    static SNDRV_CARDS: usize;
    static SNDRV_DEFAULT_IDX: [c_int; SNDRV_CARDS];
    static SNDRV_DEFAULT_STR: [*mut c_char; SNDRV_CARDS];
    static SNDRV_DEFAULT_ENABLE: [bool; SNDRV_CARDS];
    static SNDRV_DEFAULT_PORT: [c_long; SNDRV_CARDS];
    static SNDRV_DEFAULT_IRQ: [c_int; SNDRV_CARDS];
    static SNDRV_DEFAULT_DMA: [c_int; SNDRV_CARDS];
    static SNDRV_AUTO_IRQ: c_int;
    static SNDRV_AUTO_DMA: c_int;
    static SNDRV_AUTO_PORT: c_long;
    static SNDRV_CTL_ELEM_IFACE_MIXER: c_uint;
    static ES1688_HW_1688: c_int;
    static SNDRV_GF1_GB_RESET: c_int;
    static OPL3_HW_OPL3: c_int;
    static MPU401_HW_ES1688: c_int;
    static EBUSY: c_int;
    static EIO: c_int;
    static ENODEV: c_int;

    fn snd_legacy_find_free_irq(irqs: *const c_int) -> c_int;
    fn snd_legacy_find_free_dma(dmas: *const c_int) -> c_int;
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
    fn snd_gus_create(
        card: *mut snd_card,
        port: c_long,
        irq: c_int,
        dma1: c_int,
        dma2: c_int,
        timer_dev: c_int,
        voices: c_int,
        pcm_channels: c_int,
        effect: c_int,
        rgus: *mut *mut snd_gus_card,
    ) -> c_int;
    fn snd_es1688_mixer_write(chip: *mut snd_es1688, reg: c_uchar, data: c_uchar);
    fn outb(value: c_int, port: c_ulong);
    fn ES1688P(chip: *mut snd_es1688, reg: c_int) -> c_ulong;
    fn udelay(usecs: c_ulong);
    fn usleep_range(min: c_ulong, max: c_ulong);
    fn snd_gf1_i_write8(gus: *mut snd_gus_card, reg: c_int, value: c_uchar);
    fn snd_gf1_i_look8(gus: *mut snd_gus_card, reg: c_int) -> c_uchar;
    fn snd_ctl_rename_id(
        card: *mut snd_card,
        src_id: *mut snd_ctl_elem_id,
        dst_id: *mut snd_ctl_elem_id,
    ) -> c_int;
    fn snd_devm_card_new(
        dev: *mut device,
        idx: c_int,
        xid: *mut c_char,
        module: *mut module,
        extra_size: usize,
        card_ret: *mut *mut snd_card,
    ) -> c_int;
    fn snd_gus_initialize(gus: *mut snd_gus_card) -> c_int;
    fn snd_es1688_pcm(card: *mut snd_card, chip: *mut snd_es1688, device: c_int) -> c_int;
    fn snd_es1688_mixer(card: *mut snd_card, chip: *mut snd_es1688) -> c_int;
    fn snd_component_add(card: *mut snd_card, component: *const c_char) -> c_int;
    fn snd_gf1_pcm_new(gus: *mut snd_gus_card, pcm_dev: c_int, control_index: c_int) -> c_int;
    fn snd_gf1_new_mixer(gus: *mut snd_gus_card) -> c_int;
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
        rrawmidi: *mut c_void,
    ) -> c_int;
    fn snd_mpu401_uart_new(
        card: *mut snd_card,
        device: c_int,
        hardware: c_int,
        port: c_long,
        integrated: c_int,
        irq: c_int,
        rrawmidi: *mut c_void,
    ) -> c_int;
    fn snd_card_register(card: *mut snd_card) -> c_int;
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn snd_gus_suspend(gus: *mut snd_gus_card) -> c_int;
    fn snd_es1688_reset(chip: *mut snd_es1688) -> c_int;
    fn snd_gus_resume(gus: *mut snd_gus_card) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn sprintf(buf: *mut c_char, fmt: *const c_char, ...) -> c_int;
}

type c_uchar = u8;

static mut index: [c_int; SNDRV_CARDS] = SNDRV_DEFAULT_IDX; /* Index 0-MAX */
static mut id: [*mut c_char; SNDRV_CARDS] = SNDRV_DEFAULT_STR; /* ID for this card */
static mut enable: [bool; SNDRV_CARDS] = SNDRV_DEFAULT_ENABLE; /* Enable this card */
static mut port: [c_long; SNDRV_CARDS] = SNDRV_DEFAULT_PORT; /* 0x220,0x240,0x260 */
static mut gf1_port: [c_long; SNDRV_CARDS] = [-1; SNDRV_CARDS]; /* 0x210,0x220,0x230,0x240,0x250,0x260,0x270 */
static mut mpu_port: [c_long; SNDRV_CARDS] = [-1; SNDRV_CARDS]; /* 0x300,0x310,0x320 */
static mut irq: [c_int; SNDRV_CARDS] = SNDRV_DEFAULT_IRQ; /* 5,7,9,10 */
static mut mpu_irq: [c_int; SNDRV_CARDS] = SNDRV_DEFAULT_IRQ; /* 5,7,9,10 */
static mut gf1_irq: [c_int; SNDRV_CARDS] = SNDRV_DEFAULT_IRQ; /* 2,3,5,9,11,12,15 */
static mut dma8: [c_int; SNDRV_CARDS] = SNDRV_DEFAULT_DMA; /* 0,1,3 */
static mut dma1: [c_int; SNDRV_CARDS] = SNDRV_DEFAULT_DMA;
static mut joystick_dac: [c_int; SNDRV_CARDS] = [29; SNDRV_CARDS];
/* 0 to 31, (0.59V-4.52V or 0.389V-2.98V) */
static mut channels: [c_int; SNDRV_CARDS] = [24; SNDRV_CARDS];
static mut pcm_channels: [c_int; SNDRV_CARDS] = [2; SNDRV_CARDS];

#[repr(C)]
pub struct snd_gusextreme {
    pub es1688: snd_es1688,
    pub gus: *mut snd_gus_card,
}

// module_param_array/module_param_hw_array and MODULE_PARM_DESC entries are
// kernel module metadata in C and are intentionally preserved here as comments.

unsafe extern "C" fn snd_gusextreme_match(_dev: *mut device, n: c_uint) -> c_int {
    enable[n as usize] as c_int
}

unsafe extern "C" fn snd_gusextreme_es1688_create(
    card: *mut snd_card,
    chip: *mut snd_es1688,
    dev: *mut device,
    n: c_uint,
) -> c_int {
    static POSSIBLE_PORTS: [c_long; 3] = [0x220, 0x240, 0x260];
    static POSSIBLE_IRQS: [c_int; 5] = [5, 9, 10, 7, -1];
    static POSSIBLE_DMAS: [c_int; 4] = [1, 3, 0, -1];

    let mut i: usize;
    let mut error: c_int;
    let n = n as usize;

    if irq[n] == SNDRV_AUTO_IRQ {
        irq[n] = snd_legacy_find_free_irq(POSSIBLE_IRQS.as_ptr());
        if irq[n] < 0 {
            dev_err(dev, b"unable to find a free IRQ for ES1688\n\0".as_ptr() as *const c_char);
            return -EBUSY;
        }
    }
    if dma8[n] == SNDRV_AUTO_DMA {
        dma8[n] = snd_legacy_find_free_dma(POSSIBLE_DMAS.as_ptr());
        if dma8[n] < 0 {
            dev_err(dev, b"unable to find a free DMA for ES1688\n\0".as_ptr() as *const c_char);
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
            ES1688_HW_1688,
        );
    }

    i = 0;
    loop {
        port[n] = POSSIBLE_PORTS[i];
        error = snd_es1688_create(
            card,
            chip,
            port[n],
            mpu_port[n],
            irq[n],
            mpu_irq[n],
            dma8[n],
            ES1688_HW_1688,
        );
        i += 1;
        if !(error < 0 && i < POSSIBLE_PORTS.len()) {
            break;
        }
    }

    error
}

unsafe extern "C" fn snd_gusextreme_gus_card_create(
    card: *mut snd_card,
    dev: *mut device,
    n: c_uint,
    rgus: *mut *mut snd_gus_card,
) -> c_int {
    static POSSIBLE_IRQS: [c_int; 8] = [11, 12, 15, 9, 5, 7, 3, -1];
    static POSSIBLE_DMAS: [c_int; 6] = [5, 6, 7, 3, 1, -1];
    let n = n as usize;

    if gf1_irq[n] == SNDRV_AUTO_IRQ {
        gf1_irq[n] = snd_legacy_find_free_irq(POSSIBLE_IRQS.as_ptr());
        if gf1_irq[n] < 0 {
            dev_err(dev, b"unable to find a free IRQ for GF1\n\0".as_ptr() as *const c_char);
            return -EBUSY;
        }
    }
    if dma1[n] == SNDRV_AUTO_DMA {
        dma1[n] = snd_legacy_find_free_dma(POSSIBLE_DMAS.as_ptr());
        if dma1[n] < 0 {
            dev_err(dev, b"unable to find a free DMA for GF1\n\0".as_ptr() as *const c_char);
            return -EBUSY;
        }
    }
    snd_gus_create(
        card,
        gf1_port[n],
        gf1_irq[n],
        dma1[n],
        -1,
        0,
        channels[n],
        pcm_channels[n],
        0,
        rgus,
    )
}

unsafe fn snd_gusextreme_enable_gf1(gus: *mut snd_gus_card, es1688: *mut snd_es1688) {
    /*
     * This is main stuff - enable access to GF1 chip...
     * I'm not sure, if this will work for card which have
     * ES1688 chip in another place than 0x220.
     *
     * I used reverse-engineering in DOSEMU. [--jk]
     *
     * ULTRINIT.EXE:
     * 0x230 = 0,2,3
     * 0x240 = 2,0,1
     * 0x250 = 2,0,3
     * 0x260 = 2,2,1
     */

    // scoped_guard(spinlock_irqsave, &es1688->mixer_lock)
    snd_es1688_mixer_write(es1688, 0x40, 0x0b); /* don't change!!! */

    // scoped_guard(spinlock_irqsave, &es1688->reg_lock)
    outb(
        if (*gus).gf1.port & 0x040 != 0 { 2 } else { 0 },
        ES1688P(es1688, INIT1),
    );
    outb(0, 0x201);
    outb(
        if (*gus).gf1.port & 0x020 != 0 { 2 } else { 0 },
        ES1688P(es1688, INIT1),
    );
    outb(0, 0x201);
    outb(
        if (*gus).gf1.port & 0x010 != 0 { 3 } else { 1 },
        ES1688P(es1688, INIT1),
    );
}

unsafe extern "C" fn snd_gusextreme_detect(
    gus: *mut snd_gus_card,
    es1688: *mut snd_es1688,
) -> c_int {
    let mut d: c_uchar;

    snd_gusextreme_enable_gf1(gus, es1688);
    udelay(100);

    snd_gf1_i_write8(gus, SNDRV_GF1_GB_RESET, 0); /* reset GF1 */
    d = snd_gf1_i_look8(gus, SNDRV_GF1_GB_RESET);
    if (d & 0x07) != 0 {
        dev_dbg(
            (*(*gus).card).dev,
            b"[0x%lx] check 1 failed - 0x%x\n\0".as_ptr() as *const c_char,
            (*gus).gf1.port,
            d as c_int,
        );
        return -EIO;
    }
    udelay(160);
    snd_gf1_i_write8(gus, SNDRV_GF1_GB_RESET, 1); /* release reset */
    udelay(160);
    d = snd_gf1_i_look8(gus, SNDRV_GF1_GB_RESET);
    if (d & 0x07) != 1 {
        dev_dbg(
            (*(*gus).card).dev,
            b"[0x%lx] check 2 failed - 0x%x\n\0".as_ptr() as *const c_char,
            (*gus).gf1.port,
            d as c_int,
        );
        return -EIO;
    }

    0
}

unsafe extern "C" fn snd_gusextreme_mixer(card: *mut snd_card) -> c_int {
    let mut id1: snd_ctl_elem_id = core::mem::zeroed();
    let mut id2: snd_ctl_elem_id = core::mem::zeroed();
    let mut error: c_int;

    id1.iface = SNDRV_CTL_ELEM_IFACE_MIXER;
    id2.iface = SNDRV_CTL_ELEM_IFACE_MIXER;

    /* reassign AUX to SYNTHESIZER */
    strscpy(id1.name.as_mut_ptr(), b"Aux Playback Volume\0".as_ptr() as *const c_char);
    strscpy(id2.name.as_mut_ptr(), b"Synth Playback Volume\0".as_ptr() as *const c_char);
    error = snd_ctl_rename_id(card, &mut id1, &mut id2);
    if error < 0 {
        return error;
    }

    /* reassign Master Playback Switch to Synth Playback Switch */
    strscpy(
        id1.name.as_mut_ptr(),
        b"Master Playback Switch\0".as_ptr() as *const c_char,
    );
    strscpy(
        id2.name.as_mut_ptr(),
        b"Synth Playback Switch\0".as_ptr() as *const c_char,
    );
    error = snd_ctl_rename_id(card, &mut id1, &mut id2);
    if error < 0 {
        return error;
    }

    0
}

unsafe fn strscpy(dst: *mut c_char, src: *const c_char) {
    let mut i = 0usize;
    loop {
        let ch = *src.add(i);
        *dst.add(i) = ch;
        if ch == 0 {
            break;
        }
        i += 1;
    }
}

unsafe extern "C" fn snd_gusextreme_probe(dev: *mut device, n: c_uint) -> c_int {
    let mut card: *mut snd_card = ptr::null_mut();
    let mut gus: *mut snd_gus_card = ptr::null_mut();
    let mut opl3: *mut snd_opl3 = ptr::null_mut();
    let mut error: c_int;
    let n_usize = n as usize;

    error = snd_devm_card_new(
        dev,
        index[n_usize],
        id[n_usize],
        THIS_MODULE,
        size_of::<snd_gusextreme>(),
        &mut card,
    );
    if error < 0 {
        return error;
    }

    let gusextreme = (*card).private_data as *mut snd_gusextreme;
    let es1688 = &mut (*gusextreme).es1688 as *mut snd_es1688;

    if mpu_port[n_usize] == SNDRV_AUTO_PORT {
        mpu_port[n_usize] = 0;
    }

    if mpu_irq[n_usize] > 15 {
        mpu_irq[n_usize] = -1;
    }

    error = snd_gusextreme_es1688_create(card, es1688, dev, n);
    if error < 0 {
        return error;
    }

    if gf1_port[n_usize] < 0 {
        gf1_port[n_usize] = (*es1688).port + 0x20;
    }

    error = snd_gusextreme_gus_card_create(card, dev, n, &mut gus);
    if error < 0 {
        return error;
    }
    (*gusextreme).gus = gus;

    error = snd_gusextreme_detect(gus, es1688);
    if error < 0 {
        return error;
    }

    (*gus).joystick_dac = joystick_dac[n_usize];

    error = snd_gus_initialize(gus);
    if error < 0 {
        return error;
    }

    error = -ENODEV;
    if (*gus).ess_flag == 0 {
        dev_err(
            dev,
            b"GUS Extreme soundcard was not detected at 0x%lx\n\0".as_ptr() as *const c_char,
            (*gus).gf1.port,
        );
        return error;
    }
    (*gus).codec_flag = 1;

    error = snd_es1688_pcm(card, es1688, 0);
    if error < 0 {
        return error;
    }

    error = snd_es1688_mixer(card, es1688);
    if error < 0 {
        return error;
    }

    snd_component_add(card, b"ES1688\0".as_ptr() as *const c_char);

    if pcm_channels[n_usize] > 0 {
        error = snd_gf1_pcm_new(gus, 1, 1);
        if error < 0 {
            return error;
        }
    }

    error = snd_gf1_new_mixer(gus);
    if error < 0 {
        return error;
    }

    error = snd_gusextreme_mixer(card);
    if error < 0 {
        return error;
    }

    if snd_opl3_create(
        card,
        (*es1688).port,
        (*es1688).port + 2,
        OPL3_HW_OPL3,
        0,
        &mut opl3,
    ) < 0
    {
        dev_warn(
            dev,
            b"opl3 not detected at 0x%lx\n\0".as_ptr() as *const c_char,
            (*es1688).port,
        );
    } else {
        error = snd_opl3_hwdep_new(opl3, 0, 2, ptr::null_mut());
        if error < 0 {
            return error;
        }
    }

    if (*es1688).mpu_port >= 0x300 {
        error = snd_mpu401_uart_new(
            card,
            0,
            MPU401_HW_ES1688,
            (*es1688).mpu_port,
            0,
            mpu_irq[n_usize],
            ptr::null_mut(),
        );
        if error < 0 {
            return error;
        }
    }

    sprintf(
        (*card).longname.as_mut_ptr(),
        b"Gravis UltraSound Extreme at 0x%lx, irq %i&%i, dma %i&%i\0".as_ptr() as *const c_char,
        (*es1688).port,
        (*gus).gf1.irq,
        (*es1688).irq,
        (*gus).gf1.dma1,
        (*es1688).dma8,
    );

    error = snd_card_register(card);
    if error < 0 {
        return error;
    }

    dev_set_drvdata(dev, card as *mut c_void);
    0
}

// #ifdef CONFIG_PM
unsafe extern "C" fn snd_gusextreme_suspend(
    dev: *mut device,
    _n: c_uint,
    _state: pm_message_t,
) -> c_int {
    let card = dev_get_drvdata(dev) as *mut snd_card;
    let gusextreme = (*card).private_data as *mut snd_gusextreme;

    snd_gus_suspend((*gusextreme).gus)
}

unsafe extern "C" fn snd_gusextreme_resume(dev: *mut device, _n: c_uint) -> c_int {
    let card = dev_get_drvdata(dev) as *mut snd_card;
    let gusextreme = (*card).private_data as *mut snd_gusextreme;
    let mut err: c_int;

    err = snd_es1688_reset(&mut (*gusextreme).es1688);
    if err < 0 {
        return err;
    }

    snd_gusextreme_enable_gf1((*gusextreme).gus, &mut (*gusextreme).es1688);
    usleep_range(100, 200);
    snd_gus_resume((*gusextreme).gus)
}
// #endif

static mut snd_gusextreme_driver: isa_driver = isa_driver {
    match_: Some(snd_gusextreme_match),
    probe: Some(snd_gusextreme_probe),
    // CONFIG_PM:
    suspend: Some(snd_gusextreme_suspend),
    resume: Some(snd_gusextreme_resume),
    driver: device_driver {
        name: DEV_NAME.as_ptr() as *const c_char,
    },
};

// module_isa_driver(snd_gusextreme_driver, SNDRV_CARDS);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
