// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Driver for Gravis UltraSound MAX soundcard
 *  Copyright (c) by Jaroslav Kysela <perex@perex.cz>
 */

// C dependencies: linux/init.h, linux/err.h, linux/isa.h, linux/delay.h,
// linux/time.h, linux/module.h, asm/dma.h, sound/core.h, sound/gus.h,
// sound/wss.h, sound/initval.h.
// SNDRV_LEGACY_FIND_FREE_IRQ and SNDRV_LEGACY_FIND_FREE_DMA are enabled
// before including sound/initval.h in the original C source.

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

const SNDRV_CARDS: usize = 8;
const SNDRV_DEFAULT_IDX: [c_int; SNDRV_CARDS] = [0; SNDRV_CARDS];
const SNDRV_DEFAULT_STR: [*mut c_char; SNDRV_CARDS] = [ptr::null_mut(); SNDRV_CARDS];
const SNDRV_DEFAULT_ENABLE: [bool; SNDRV_CARDS] = [false; SNDRV_CARDS];
const SNDRV_DEFAULT_PORT: [c_long; SNDRV_CARDS] = [-1; SNDRV_CARDS];
const SNDRV_DEFAULT_IRQ: [c_int; SNDRV_CARDS] = [-1; SNDRV_CARDS];
const SNDRV_DEFAULT_DMA: [c_int; SNDRV_CARDS] = [-1; SNDRV_CARDS];
const SNDRV_AUTO_IRQ: c_int = -1;
const SNDRV_AUTO_DMA: c_int = -1;
const SNDRV_AUTO_PORT: c_long = -1;
const SNDRV_CTL_ELEM_IFACE_MIXER: c_int = 2;
const SNDRV_GF1_GB_RESET: c_int = 0;
const MAXCNTRLPORT: c_ulong = 0;
const WSS_HW_DETECT: c_int = 0;
const WSS_HWSHARE_IRQ: c_int = 1 << 0;
const WSS_HWSHARE_DMA1: c_int = 1 << 1;
const WSS_HWSHARE_DMA2: c_int = 1 << 2;
const ENODEV: c_int = 19;
const EBUSY: c_int = 16;
const THIS_MODULE: *mut c_void = ptr::null_mut();
const DEV_NAME: &[u8] = b"gusmax\0";

type IrqReturnT = c_int;
type PmMessageT = c_int;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_card {
    pub dev: *mut device,
    pub private_data: *mut c_void,
    pub sync_irq: c_int,
    pub longname: [c_char; 80],
}

#[repr(C)]
pub struct snd_gf1 {
    pub port: c_ulong,
    pub dma1: c_int,
    pub dma2: c_int,
    pub reg_irqstat: u16,
}

#[repr(C)]
pub struct snd_gus_card {
    pub card: *mut snd_card,
    pub gf1: snd_gf1,
    pub equal_irq: c_int,
    pub codec_flag: c_int,
    pub joystick_dac: c_int,
    pub max_cntrl_val: u8,
    pub max_flag: c_int,
}

#[repr(C)]
pub struct snd_wss {
    pub card: *mut snd_card,
    pub suspend: Option<unsafe extern "C" fn(*mut snd_wss)>,
    pub resume: Option<unsafe extern "C" fn(*mut snd_wss)>,
}

#[repr(C)]
pub struct snd_ctl_elem_id {
    pub numid: c_uint,
    pub iface: c_int,
    pub device: c_uint,
    pub subdevice: c_uint,
    pub name: [c_char; 44],
    pub index: c_uint,
}

#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
}

#[repr(C)]
pub struct isa_driver {
    pub match_: Option<unsafe extern "C" fn(*mut device, c_uint) -> c_int>,
    pub probe: Option<unsafe extern "C" fn(*mut device, c_uint) -> c_int>,
    // CONFIG_PM fields are present in the C initializer only when enabled.
    pub suspend: Option<unsafe extern "C" fn(*mut device, c_uint, PmMessageT) -> c_int>,
    pub resume: Option<unsafe extern "C" fn(*mut device, c_uint) -> c_int>,
    pub driver: device_driver,
}

#[repr(C)]
pub struct snd_gusmax {
    pub irq: c_int,
    pub card: *mut snd_card,
    pub gus: *mut snd_gus_card,
    pub wss: *mut snd_wss,
    pub gus_status_reg: u16,
    pub pcm_status_reg: u16,
}

static mut index: [c_int; SNDRV_CARDS] = SNDRV_DEFAULT_IDX; /* Index 0-MAX */
static mut id: [*mut c_char; SNDRV_CARDS] = SNDRV_DEFAULT_STR; /* ID for this card */
static mut enable: [bool; SNDRV_CARDS] = SNDRV_DEFAULT_ENABLE; /* Enable this card */
static mut port: [c_long; SNDRV_CARDS] = SNDRV_DEFAULT_PORT; /* 0x220,0x230,0x240,0x250,0x260 */
static mut irq: [c_int; SNDRV_CARDS] = SNDRV_DEFAULT_IRQ; /* 2,3,5,9,11,12,15 */
static mut dma1: [c_int; SNDRV_CARDS] = SNDRV_DEFAULT_DMA; /* 1,3,5,6,7 */
static mut dma2: [c_int; SNDRV_CARDS] = SNDRV_DEFAULT_DMA; /* 1,3,5,6,7 */
static mut joystick_dac: [c_int; SNDRV_CARDS] = [29; SNDRV_CARDS];
/* 0 to 31, (0.59V-4.52V or 0.389V-2.98V) */
static mut channels: [c_int; SNDRV_CARDS] = [24; SNDRV_CARDS];
static mut pcm_channels: [c_int; SNDRV_CARDS] = [2; SNDRV_CARDS];

// module_param_array/module_param_hw_array and MODULE_PARM_DESC metadata are
// Linux module declarations in the C source.

unsafe extern "C" {
    fn snd_gf1_i_write8(gus: *mut snd_gus_card, reg: c_int, data: u8);
    fn snd_gf1_i_look8(gus: *mut snd_gus_card, reg: c_int) -> u8;
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn udelay(usecs: c_ulong);
    fn inb(port: u16) -> u8;
    fn outb(value: u8, port: c_ulong);
    fn snd_gus_interrupt(irq: c_int, gus: *mut snd_gus_card);
    fn snd_wss_interrupt(irq: c_int, wss: *mut snd_wss);
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn strscpy(dest: *mut c_char, src: *const c_char) -> isize;
    fn snd_ctl_rename_id(
        card: *mut snd_card,
        src_id: *mut snd_ctl_elem_id,
        dst_id: *mut snd_ctl_elem_id,
    ) -> c_int;
    fn snd_devm_card_new(
        parent: *mut device,
        idx: c_int,
        xid: *mut c_char,
        module: *mut c_void,
        extra_size: usize,
        card_ret: *mut *mut snd_card,
    ) -> c_int;
    fn snd_legacy_find_free_irq(possible_irqs: *const c_int) -> c_int;
    fn snd_legacy_find_free_dma(possible_dmas: *const c_int) -> c_int;
    fn snd_gus_create(
        card: *mut snd_card,
        port: c_ulong,
        irq: c_int,
        dma1: c_int,
        dma2: c_int,
        timer_dev: c_int,
        voices: c_int,
        pcm_channels: c_int,
        effect: c_int,
        rgus: *mut *mut snd_gus_card,
    ) -> c_int;
    fn snd_gus_initialize(gus: *mut snd_gus_card) -> c_int;
    fn devm_request_irq(
        dev: *mut device,
        irq: c_int,
        handler: unsafe extern "C" fn(c_int, *mut c_void) -> IrqReturnT,
        flags: c_ulong,
        name: *const c_char,
        dev_id: *mut c_void,
    ) -> c_int;
    fn snd_wss_create(
        card: *mut snd_card,
        port: c_ulong,
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
    fn snd_wss_timer(chip: *mut snd_wss, device: c_int) -> c_int;
    fn snd_gf1_pcm_new(gus: *mut snd_gus_card, pcm_dev: c_int, control_index: c_int) -> c_int;
    fn snd_gf1_rawmidi_new(gus: *mut snd_gus_card, device: c_int) -> c_int;
    fn strlen(s: *const c_char) -> usize;
    fn sprintf(s: *mut c_char, format: *const c_char, ...) -> c_int;
    fn snd_card_register(card: *mut snd_card) -> c_int;
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn snd_gus_suspend(gus: *mut snd_gus_card) -> c_int;
    fn snd_gus_resume(gus: *mut snd_gus_card) -> c_int;
    fn module_isa_driver(driver: *mut isa_driver, num: usize);
}

#[inline]
unsafe fn GUSP(gus: *mut snd_gus_card, port: c_ulong) -> c_ulong {
    unsafe { (*gus).gf1.port.wrapping_add(port) }
}

#[inline]
fn IRQ_RETVAL(handled: c_int) -> IrqReturnT {
    if handled != 0 { 1 } else { 0 }
}

unsafe extern "C" fn snd_gusmax_detect(gus: *mut snd_gus_card) -> c_int {
    let mut d: u8;

    unsafe { snd_gf1_i_write8(gus, SNDRV_GF1_GB_RESET, 0) }; /* reset GF1 */
    d = unsafe { snd_gf1_i_look8(gus, SNDRV_GF1_GB_RESET) };
    if (d & 0x07) != 0 {
        unsafe {
            dev_dbg(
                (*(*gus).card).dev,
                b"[0x%lx] check 1 failed - 0x%x\n\0".as_ptr() as *const c_char,
                (*gus).gf1.port,
                d as c_int,
            )
        };
        return -ENODEV;
    }
    unsafe { udelay(160) };
    unsafe { snd_gf1_i_write8(gus, SNDRV_GF1_GB_RESET, 1) }; /* release reset */
    unsafe { udelay(160) };
    d = unsafe { snd_gf1_i_look8(gus, SNDRV_GF1_GB_RESET) };
    if (d & 0x07) != 1 {
        unsafe {
            dev_dbg(
                (*(*gus).card).dev,
                b"[0x%lx] check 2 failed - 0x%x\n\0".as_ptr() as *const c_char,
                (*gus).gf1.port,
                d as c_int,
            )
        };
        return -ENODEV;
    }

    0
}

unsafe extern "C" fn snd_gusmax_interrupt(irq: c_int, dev_id: *mut c_void) -> IrqReturnT {
    let maxcard = dev_id as *mut snd_gusmax;
    let mut loop_: c_int;
    let mut max: c_int = 5;
    let mut handled: c_int = 0;

    loop {
        loop_ = 0;
        if unsafe { inb((*maxcard).gus_status_reg) } != 0 {
            handled = 1;
            unsafe { snd_gus_interrupt(irq, (*maxcard).gus) };
            loop_ += 1;
        }
        if (unsafe { inb((*maxcard).pcm_status_reg) } & 0x01) != 0 {
            /* IRQ bit is set? */
            handled = 1;
            unsafe { snd_wss_interrupt(irq, (*maxcard).wss) };
            loop_ += 1;
        }
        if !(loop_ != 0 && {
            max -= 1;
            max > 0
        }) {
            break;
        }
    }
    IRQ_RETVAL(handled)
}

unsafe extern "C" fn snd_gusmax_init(dev: c_int, _card: *mut snd_card, gus: *mut snd_gus_card) {
    unsafe {
        (*gus).equal_irq = 1;
        (*gus).codec_flag = 1;
        (*gus).joystick_dac = joystick_dac[dev as usize];
        /* init control register */
        (*gus).max_cntrl_val = (((*gus).gf1.port >> 4) & 0x0f) as u8;
        if (*gus).gf1.dma1 > 3 {
            (*gus).max_cntrl_val |= 0x10;
        }
        if (*gus).gf1.dma2 > 3 {
            (*gus).max_cntrl_val |= 0x20;
        }
        (*gus).max_cntrl_val |= 0x40;
        outb((*gus).max_cntrl_val, GUSP(gus, MAXCNTRLPORT));
    }
}

unsafe extern "C" fn snd_gusmax_mixer(chip: *mut snd_wss) -> c_int {
    let card = unsafe { (*chip).card };
    let mut id1: snd_ctl_elem_id = unsafe { core::mem::zeroed() };
    let mut id2: snd_ctl_elem_id = unsafe { core::mem::zeroed() };
    let mut err: c_int;

    unsafe {
        memset(
            &mut id1 as *mut _ as *mut c_void,
            0,
            size_of::<snd_ctl_elem_id>(),
        );
        memset(
            &mut id2 as *mut _ as *mut c_void,
            0,
            size_of::<snd_ctl_elem_id>(),
        );
    }
    id2.iface = SNDRV_CTL_ELEM_IFACE_MIXER;
    id1.iface = id2.iface;
    /* reassign AUXA to SYNTHESIZER */
    unsafe { strscpy(id1.name.as_mut_ptr(), b"Aux Playback Switch\0".as_ptr() as *const c_char) };
    unsafe { strscpy(id2.name.as_mut_ptr(), b"Synth Playback Switch\0".as_ptr() as *const c_char) };
    err = unsafe { snd_ctl_rename_id(card, &mut id1, &mut id2) };
    if err < 0 {
        return err;
    }
    unsafe { strscpy(id1.name.as_mut_ptr(), b"Aux Playback Volume\0".as_ptr() as *const c_char) };
    unsafe { strscpy(id2.name.as_mut_ptr(), b"Synth Playback Volume\0".as_ptr() as *const c_char) };
    err = unsafe { snd_ctl_rename_id(card, &mut id1, &mut id2) };
    if err < 0 {
        return err;
    }
    /* reassign AUXB to CD */
    unsafe { strscpy(id1.name.as_mut_ptr(), b"Aux Playback Switch\0".as_ptr() as *const c_char) };
    id1.index = 1;
    unsafe { strscpy(id2.name.as_mut_ptr(), b"CD Playback Switch\0".as_ptr() as *const c_char) };
    err = unsafe { snd_ctl_rename_id(card, &mut id1, &mut id2) };
    if err < 0 {
        return err;
    }
    unsafe { strscpy(id1.name.as_mut_ptr(), b"Aux Playback Volume\0".as_ptr() as *const c_char) };
    unsafe { strscpy(id2.name.as_mut_ptr(), b"CD Playback Volume\0".as_ptr() as *const c_char) };
    err = unsafe { snd_ctl_rename_id(card, &mut id1, &mut id2) };
    if err < 0 {
        return err;
    }
    /*
     * Original C contains an #if 0 block to reassign Mono Input to MIC using
     * older mixer rename helpers; it is intentionally inactive.
     */
    0
}

unsafe extern "C" fn snd_gusmax_match(_pdev: *mut device, dev: c_uint) -> c_int {
    unsafe { enable[dev as usize] as c_int }
}

unsafe extern "C" fn snd_gusmax_probe(pdev: *mut device, dev: c_uint) -> c_int {
    static POSSIBLE_IRQS: [c_int; 8] = [5, 11, 12, 9, 7, 15, 3, -1];
    static POSSIBLE_DMAS: [c_int; 6] = [5, 6, 7, 1, 3, -1];
    let mut xirq: c_int;
    let mut xdma1: c_int;
    let mut xdma2: c_int;
    let mut err: c_int;
    let mut card: *mut snd_card = ptr::null_mut();
    let mut gus: *mut snd_gus_card = ptr::null_mut();
    let mut wss: *mut snd_wss = ptr::null_mut();
    let maxcard: *mut snd_gusmax;

    unsafe {
        err = snd_devm_card_new(
            pdev,
            index[dev as usize],
            id[dev as usize],
            THIS_MODULE,
            size_of::<snd_gusmax>(),
            &mut card,
        );
    }
    if err < 0 {
        return err;
    }
    maxcard = unsafe { (*card).private_data as *mut snd_gusmax };
    unsafe {
        (*maxcard).card = card;
        (*maxcard).irq = -1;
    }

    xirq = unsafe { irq[dev as usize] };
    if xirq == SNDRV_AUTO_IRQ {
        xirq = unsafe { snd_legacy_find_free_irq(POSSIBLE_IRQS.as_ptr()) };
        if xirq < 0 {
            unsafe { dev_err(pdev, b"unable to find a free IRQ\n\0".as_ptr() as *const c_char) };
            return -EBUSY;
        }
    }
    xdma1 = unsafe { dma1[dev as usize] };
    if xdma1 == SNDRV_AUTO_DMA {
        xdma1 = unsafe { snd_legacy_find_free_dma(POSSIBLE_DMAS.as_ptr()) };
        if xdma1 < 0 {
            unsafe { dev_err(pdev, b"unable to find a free DMA1\n\0".as_ptr() as *const c_char) };
            return -EBUSY;
        }
    }
    xdma2 = unsafe { dma2[dev as usize] };
    if xdma2 == SNDRV_AUTO_DMA {
        xdma2 = unsafe { snd_legacy_find_free_dma(POSSIBLE_DMAS.as_ptr()) };
        if xdma2 < 0 {
            unsafe { dev_err(pdev, b"unable to find a free DMA2\n\0".as_ptr() as *const c_char) };
            return -EBUSY;
        }
    }

    if unsafe { port[dev as usize] } != SNDRV_AUTO_PORT {
        err = unsafe {
            snd_gus_create(
                card,
                port[dev as usize] as c_ulong,
                -xirq,
                xdma1,
                xdma2,
                0,
                channels[dev as usize],
                pcm_channels[dev as usize],
                0,
                &mut gus,
            )
        };
    } else {
        static POSSIBLE_PORTS: [c_ulong; 5] = [0x220, 0x230, 0x240, 0x250, 0x260];
        let mut i: usize = 0;
        while i < POSSIBLE_PORTS.len() {
            err = unsafe {
                snd_gus_create(
                    card,
                    POSSIBLE_PORTS[i],
                    -xirq,
                    xdma1,
                    xdma2,
                    0,
                    channels[dev as usize],
                    pcm_channels[dev as usize],
                    0,
                    &mut gus,
                )
            };
            if err >= 0 {
                unsafe { port[dev as usize] = POSSIBLE_PORTS[i] as c_long };
                break;
            }
            i += 1;
        }
    }
    if err < 0 {
        return err;
    }

    err = unsafe { snd_gusmax_detect(gus) };
    if err < 0 {
        return err;
    }

    unsafe {
        (*maxcard).gus_status_reg = (*gus).gf1.reg_irqstat;
        (*maxcard).pcm_status_reg = ((*gus).gf1.port + 0x10c + 2) as u16;
        snd_gusmax_init(dev as c_int, card, gus);
    }
    err = unsafe { snd_gus_initialize(gus) };
    if err < 0 {
        return err;
    }

    if unsafe { (*gus).max_flag } == 0 {
        unsafe {
            dev_err(
                pdev,
                b"GUS MAX soundcard was not detected at 0x%lx\n\0".as_ptr() as *const c_char,
                (*gus).gf1.port,
            )
        };
        return -ENODEV;
    }

    if unsafe {
        devm_request_irq(
            (*card).dev,
            xirq,
            snd_gusmax_interrupt,
            0,
            b"GUS MAX\0".as_ptr() as *const c_char,
            maxcard as *mut c_void,
        )
    } != 0
    {
        unsafe {
            dev_err(
                pdev,
                b"unable to grab IRQ %d\n\0".as_ptr() as *const c_char,
                xirq,
            )
        };
        return -EBUSY;
    }
    unsafe {
        (*maxcard).irq = xirq;
        (*card).sync_irq = (*maxcard).irq;
    }

    err = unsafe {
        snd_wss_create(
            card,
            (*gus).gf1.port + 0x10c,
            -1,
            xirq,
            if xdma2 < 0 { xdma1 } else { xdma2 },
            xdma1,
            WSS_HW_DETECT,
            WSS_HWSHARE_IRQ | WSS_HWSHARE_DMA1 | WSS_HWSHARE_DMA2,
            &mut wss,
        )
    };
    if err < 0 {
        return err;
    }

    err = unsafe { snd_wss_pcm(wss, 0) };
    if err < 0 {
        return err;
    }

    err = unsafe { snd_wss_mixer(wss) };
    if err < 0 {
        return err;
    }

    err = unsafe { snd_wss_timer(wss, 2) };
    if err < 0 {
        return err;
    }

    if unsafe { pcm_channels[dev as usize] } > 0 {
        err = unsafe { snd_gf1_pcm_new(gus, 1, 1) };
        if err < 0 {
            return err;
        }
    }
    err = unsafe { snd_gusmax_mixer(wss) };
    if err < 0 {
        return err;
    }

    err = unsafe { snd_gf1_rawmidi_new(gus, 0) };
    if err < 0 {
        return err;
    }

    unsafe {
        let end = (*card)
            .longname
            .as_mut_ptr()
            .add(strlen((*card).longname.as_ptr()));
        sprintf(
            end,
            b" at 0x%lx, irq %i, dma %i\0".as_ptr() as *const c_char,
            (*gus).gf1.port,
            xirq,
            xdma1,
        );
        if xdma2 >= 0 {
            let end = (*card)
                .longname
                .as_mut_ptr()
                .add(strlen((*card).longname.as_ptr()));
            sprintf(end, b"&%i\0".as_ptr() as *const c_char, xdma2);
        }
    }

    err = unsafe { snd_card_register(card) };
    if err < 0 {
        return err;
    }

    unsafe {
        (*maxcard).gus = gus;
        (*maxcard).wss = wss;

        dev_set_drvdata(pdev, card as *mut c_void);
    }
    0
}

// CONFIG_PM
unsafe extern "C" fn snd_gusmax_suspend(
    dev: *mut device,
    _n: c_uint,
    _state: PmMessageT,
) -> c_int {
    let card = unsafe { dev_get_drvdata(dev) as *mut snd_card };
    let maxcard = unsafe { (*card).private_data as *mut snd_gusmax };

    unsafe {
        if let Some(suspend) = (*(*maxcard).wss).suspend {
            suspend((*maxcard).wss);
        }
        snd_gus_suspend((*maxcard).gus)
    }
}

// CONFIG_PM
unsafe extern "C" fn snd_gusmax_resume(dev: *mut device, _n: c_uint) -> c_int {
    let card = unsafe { dev_get_drvdata(dev) as *mut snd_card };
    let maxcard = unsafe { (*card).private_data as *mut snd_gusmax };

    unsafe {
        /* Restore the board routing latch before resuming the codec and GF1. */
        outb(
            (*(*maxcard).gus).max_cntrl_val,
            GUSP((*maxcard).gus, MAXCNTRLPORT),
        );
        if let Some(resume) = (*(*maxcard).wss).resume {
            resume((*maxcard).wss);
        }
        snd_gus_resume((*maxcard).gus)
    }
}

static mut snd_gusmax_driver: isa_driver = isa_driver {
    match_: Some(snd_gusmax_match),
    probe: Some(snd_gusmax_probe),
    // CONFIG_PM
    suspend: Some(snd_gusmax_suspend),
    resume: Some(snd_gusmax_resume),
    driver: device_driver {
        name: DEV_NAME.as_ptr() as *const c_char,
    },
};

unsafe fn init_module_isa_driver() {
    unsafe { module_isa_driver(&mut snd_gusmax_driver, SNDRV_CARDS) };
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
