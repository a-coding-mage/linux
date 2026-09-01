// SPDX-License-Identifier: GPL-2.0-or-later
/*
    card-opti92x-ad1848.c - driver for OPTi 82c92x based soundcards.
    Copyright (C) 1998-2000 by Massimo Piccioni <dafastidio@libero.it>

    Part of this code was developed at the Italian Ministry of Air Defence,
    Sixth Division (oh, che pace ...), Rome.

    Thanks to Maria Grazia Pollarini, Salvatore Vassallo.

*/

/* Rust translation of Linux kernel module source. C include/module metadata:
 * linux/init.h, linux/err.h, linux/isa.h, linux/delay.h, linux/pnp.h,
 * linux/module.h, linux/io.h, asm/dma.h, sound/core.h, sound/tlv.h,
 * sound/wss.h, sound/mpu401.h, sound/opl3.h, sound/opl4.h, sound/initval.h.
 * MODULE_AUTHOR("Massimo Piccioni <dafastidio@libero.it>");
 * MODULE_LICENSE("GPL");
 * MODULE_DESCRIPTION depends on OPTi93X and CS4231.
 */

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem::{size_of, zeroed};
use core::ptr;

const OPTi9XX_HW_82C928: c_uint = 1;
const OPTi9XX_HW_82C929: c_uint = 2;
const OPTi9XX_HW_82C924: c_uint = 3;
const OPTi9XX_HW_82C925: c_uint = 4;
const OPTi9XX_HW_82C930: c_uint = 5;
const OPTi9XX_HW_82C931: c_uint = 6;
const OPTi9XX_HW_82C933: c_uint = 7;
const OPTi9XX_HW_LAST: c_uint = OPTi9XX_HW_82C933;

#[inline]
const fn OPTi9XX_MC_REG(n: c_uint) -> c_uint {
    n
}

/* Only used when built with OPTi93X in the C source. */
const OPTi93X_STATUS: c_ulong = 0x02;
#[inline]
unsafe fn OPTi93X_PORT(chip: *mut snd_wss, r: c_ulong) -> c_ulong {
    unsafe { (*chip).port.wrapping_add(r) }
}
const OPTi93X_IRQ_PLAYBACK: u8 = 0x04;
const OPTi93X_IRQ_CAPTURE: u8 = 0x08;

const DEV_NAME: *const c_char = KBUILD_MODNAME;
const CHIP_NAME: &[u8] = b"82C92x\0"; /* "82C93x" when built with OPTi93X. */

static mut index: c_int = SNDRV_DEFAULT_IDX1; /* Index 0-MAX */
static mut id: *mut c_char = SNDRV_DEFAULT_STR1 as *mut c_char; /* ID for this card */
/* static bool enable = SNDRV_DEFAULT_ENABLE1; Enable this card */
/* CONFIG_PNP: */
static mut isapnp: bool = true; /* Enable ISA PnP detection */
static mut port: c_long = SNDRV_DEFAULT_PORT1; /* 0x530,0xe80,0xf40,0x604 */
static mut mpu_port: c_long = SNDRV_DEFAULT_PORT1; /* 0x300,0x310,0x320,0x330 */
static mut fm_port: c_long = SNDRV_DEFAULT_PORT1; /* 0x388 */
static mut irq: c_int = SNDRV_DEFAULT_IRQ1; /* 5,7,9,10,11 */
static mut mpu_irq: c_int = SNDRV_DEFAULT_IRQ1; /* 5,7,9,10 */
static mut dma1: c_int = SNDRV_DEFAULT_DMA1; /* 0,1,3 */
/* CS4231 || OPTi93X: */
static mut dma2: c_int = SNDRV_DEFAULT_DMA1; /* 0,1,3 */

#[repr(C)]
pub struct snd_opti9xx {
    card: *mut snd_card,
    hardware: u16,
    password: u8,
    name: [c_char; 7],

    mc_base: c_ulong,
    res_mc_base: *mut resource,
    mc_base_size: c_ulong,
    /* OPTi93X: */
    mc_indir_index: c_ulong,
    res_mc_indir: *mut resource,
    codec: *mut snd_wss,
    pwd_reg: c_ulong,

    lock: spinlock_t,

    wss_base: c_long,
    irq: c_int,
}

static mut snd_opti9xx_pnp_is_probed: c_int = 0;

/* CONFIG_PNP:
static const struct pnp_card_device_id snd_opti9xx_pnpids[] = ...
MODULE_DEVICE_TABLE(pnp_card, snd_opti9xx_pnpids);
*/

static snd_opti9xx_names: [&[u8]; 8] = [
    b"unknown\0",
    b"82C928\0",
    b"82C929\0",
    b"82C924\0",
    b"82C925\0",
    b"82C930\0",
    b"82C931\0",
    b"82C933\0",
];

unsafe fn snd_opti9xx_init(chip: *mut snd_opti9xx, hardware: u16) -> c_int {
    static opti9xx_mc_size: [c_int; 7] = [7, 7, 10, 10, 2, 2, 2];

    unsafe {
        (*chip).hardware = hardware;
        strscpy((*chip).name.as_mut_ptr(), snd_opti9xx_names[hardware as usize].as_ptr() as *const c_char);

        spin_lock_init(&mut (*chip).lock);

        (*chip).irq = -1;

        /* !OPTi93X path, with CONFIG_PNP conditional preserved. */
        if isapnp && (*chip).mc_base != 0 {
            /* PnP resource gives the least 10 bits */
            (*chip).mc_base |= 0xc00;
        } else {
            (*chip).mc_base = 0xf8c;
            (*chip).mc_base_size = opti9xx_mc_size[hardware as usize] as c_ulong;
        }
        /* OPTi93X would set mc_base_size here. */

        match hardware as c_uint {
            OPTi9XX_HW_82C928 | OPTi9XX_HW_82C929 => {
                (*chip).password = if hardware as c_uint == OPTi9XX_HW_82C928 { 0xe2 } else { 0xe3 };
                (*chip).pwd_reg = 3;
            }
            OPTi9XX_HW_82C924 | OPTi9XX_HW_82C925 => {
                (*chip).password = 0xe5;
                (*chip).pwd_reg = 3;
            }
            OPTi9XX_HW_82C930 | OPTi9XX_HW_82C931 | OPTi9XX_HW_82C933 => {
                /* OPTi93X-only cases in C. */
                (*chip).mc_base = if hardware as c_uint == OPTi9XX_HW_82C930 { 0xf8f } else { 0xf8d };
                if (*chip).mc_indir_index == 0 {
                    (*chip).mc_indir_index = 0xe0e;
                }
                (*chip).password = 0xe4;
                (*chip).pwd_reg = 0;
            }
            _ => {
                dev_err((*(*chip).card).dev, c"chip %d not supported\n".as_ptr(), hardware as c_int);
                return -ENODEV;
            }
        }
        0
    }
}

unsafe fn snd_opti9xx_read(chip: *mut snd_opti9xx, reg: u8) -> u8 {
    let mut retval: u8 = 0xff;

    unsafe {
        spin_lock_irqsave(&mut (*chip).lock);
        outb((*chip).password, (*chip).mc_base + (*chip).pwd_reg);

        match (*chip).hardware as c_uint {
            OPTi9XX_HW_82C924 | OPTi9XX_HW_82C925 => {
                if reg > 7 {
                    outb(reg, (*chip).mc_base + 8);
                    outb((*chip).password, (*chip).mc_base + (*chip).pwd_reg);
                    retval = inb((*chip).mc_base + 9);
                } else {
                    retval = inb((*chip).mc_base + reg as c_ulong);
                }
            }
            OPTi9XX_HW_82C928 | OPTi9XX_HW_82C929 => {
                retval = inb((*chip).mc_base + reg as c_ulong);
            }
            OPTi9XX_HW_82C930 | OPTi9XX_HW_82C931 | OPTi9XX_HW_82C933 => {
                /* OPTi93X-only cases in C. */
                outb(reg, (*chip).mc_indir_index);
                outb((*chip).password, (*chip).mc_base + (*chip).pwd_reg);
                retval = inb((*chip).mc_indir_index + 1);
            }
            _ => {
                dev_err((*(*chip).card).dev, c"chip %d not supported\n".as_ptr(), (*chip).hardware as c_int);
            }
        }

        spin_unlock_irqrestore(&mut (*chip).lock);
    }

    retval
}

unsafe fn snd_opti9xx_write(chip: *mut snd_opti9xx, reg: u8, value: u8) {
    unsafe {
        spin_lock_irqsave(&mut (*chip).lock);
        outb((*chip).password, (*chip).mc_base + (*chip).pwd_reg);

        match (*chip).hardware as c_uint {
            OPTi9XX_HW_82C924 | OPTi9XX_HW_82C925 => {
                if reg > 7 {
                    outb(reg, (*chip).mc_base + 8);
                    outb((*chip).password, (*chip).mc_base + (*chip).pwd_reg);
                    outb(value, (*chip).mc_base + 9);
                } else {
                    outb(value, (*chip).mc_base + reg as c_ulong);
                }
            }
            OPTi9XX_HW_82C928 | OPTi9XX_HW_82C929 => {
                outb(value, (*chip).mc_base + reg as c_ulong);
            }
            OPTi9XX_HW_82C930 | OPTi9XX_HW_82C931 | OPTi9XX_HW_82C933 => {
                /* OPTi93X-only cases in C. */
                outb(reg, (*chip).mc_indir_index);
                outb((*chip).password, (*chip).mc_base + (*chip).pwd_reg);
                outb(value, (*chip).mc_indir_index + 1);
            }
            _ => {
                dev_err((*(*chip).card).dev, c"chip %d not supported\n".as_ptr(), (*chip).hardware as c_int);
            }
        }
        spin_unlock_irqrestore(&mut (*chip).lock);
    }
}

#[inline]
unsafe fn snd_opti9xx_write_mask(chip: *mut snd_opti9xx, reg: u8, value: u8, mask: u8) {
    let oldval = unsafe { snd_opti9xx_read(chip, reg) };

    unsafe { snd_opti9xx_write(chip, reg, (oldval & !mask) | (value & mask)) };
}

unsafe fn snd_opti9xx_configure(
    chip: *mut snd_opti9xx,
    port_arg: c_long,
    irq_arg: c_int,
    dma1_arg: c_int,
    dma2_arg: c_int,
    mpu_port_arg: c_long,
    mpu_irq_arg: c_int,
) -> c_int {
    let wss_base_bits: u8;
    let irq_bits: u8;
    let mut dma_bits: u8;
    let mut mpu_port_bits: u8 = 0;
    let mpu_irq_bits: u8;

    unsafe {
        match (*chip).hardware as c_uint {
            OPTi9XX_HW_82C924 => {
                /* opti 929 mode (?), OPL3 clock output, audio enable */
                snd_opti9xx_write_mask(chip, OPTi9XX_MC_REG(4) as u8, 0xf0, 0xfc);
                /* enable wave audio */
                snd_opti9xx_write_mask(chip, OPTi9XX_MC_REG(6) as u8, 0x02, 0x02);
                /* fallthrough */
                snd_opti9xx_write_mask(chip, OPTi9XX_MC_REG(1) as u8, 0x80, 0x80);
                snd_opti9xx_write_mask(chip, OPTi9XX_MC_REG(2) as u8, 0x00, 0x20);
                snd_opti9xx_write_mask(chip, OPTi9XX_MC_REG(3) as u8, 0xf0, 0xff);
                snd_opti9xx_write_mask(chip, OPTi9XX_MC_REG(5) as u8, 0x00, 0x02); /* CS4231 uses 0x02 */
            }
            OPTi9XX_HW_82C925 => {
                /* enable WSS mode */
                snd_opti9xx_write_mask(chip, OPTi9XX_MC_REG(1) as u8, 0x80, 0x80);
                /* OPL3 FM synthesis */
                snd_opti9xx_write_mask(chip, OPTi9XX_MC_REG(2) as u8, 0x00, 0x20);
                /* disable Sound Blaster IRQ and DMA */
                snd_opti9xx_write_mask(chip, OPTi9XX_MC_REG(3) as u8, 0xf0, 0xff);
                /* cs4231/4248 fix disabled; CS4231 build enables it */
                snd_opti9xx_write_mask(chip, OPTi9XX_MC_REG(5) as u8, 0x00, 0x02);
            }
            OPTi9XX_HW_82C928 | OPTi9XX_HW_82C929 => {
                snd_opti9xx_write_mask(chip, OPTi9XX_MC_REG(1) as u8, 0x80, 0x80);
                snd_opti9xx_write_mask(chip, OPTi9XX_MC_REG(2) as u8, 0x00, 0x20);
                /*
                snd_opti9xx_write_mask(chip, OPTi9XX_MC_REG(3), 0xa2, 0xae);
                */
                snd_opti9xx_write_mask(chip, OPTi9XX_MC_REG(4) as u8, 0x00, 0x0c);
                snd_opti9xx_write_mask(chip, OPTi9XX_MC_REG(5) as u8, 0x00, 0x02); /* CS4231 uses 0x02 */
            }
            OPTi9XX_HW_82C931 => {
                /* OPTi93X-only: disable 3D sound (set GPIO1 as output, low) */
                snd_opti9xx_write_mask(chip, OPTi9XX_MC_REG(20) as u8, 0x04, 0x0c);
                snd_opti9xx_write_mask(chip, OPTi9XX_MC_REG(21) as u8, 0x82, 0xff);
                snd_opti9xx_write_mask(chip, OPTi9XX_MC_REG(26) as u8, 0x01, 0x01);
                snd_opti9xx_write_mask(chip, OPTi9XX_MC_REG(6) as u8, 0x02, 0x03);
                snd_opti9xx_write_mask(chip, OPTi9XX_MC_REG(3) as u8, 0x00, 0xff);
                snd_opti9xx_write_mask(chip, OPTi9XX_MC_REG(4) as u8, 0x10 | 0x04, 0x34);
                snd_opti9xx_write_mask(chip, OPTi9XX_MC_REG(5) as u8, 0x20, 0xbf);
            }
            OPTi9XX_HW_82C933 => {
                /*
                 * The BTC 1817DW has QS1000 wavetable which is connected
                 * to the serial digital input of the OPTI931.
                 */
                snd_opti9xx_write_mask(chip, OPTi9XX_MC_REG(21) as u8, 0x82, 0xff);
                /*
                 * This bit sets OPTI931 to automaticaly select FM
                 * or digital input signal.
                 */
                snd_opti9xx_write_mask(chip, OPTi9XX_MC_REG(26) as u8, 0x01, 0x01);
                snd_opti9xx_write_mask(chip, OPTi9XX_MC_REG(6) as u8, 0x02, 0x03);
                snd_opti9xx_write_mask(chip, OPTi9XX_MC_REG(3) as u8, 0x00, 0xff);
                snd_opti9xx_write_mask(chip, OPTi9XX_MC_REG(4) as u8, 0x10 | 0x04, 0x34);
                snd_opti9xx_write_mask(chip, OPTi9XX_MC_REG(5) as u8, 0x20, 0xbf);
            }
            OPTi9XX_HW_82C930 => {
                snd_opti9xx_write_mask(chip, OPTi9XX_MC_REG(6) as u8, 0x02, 0x03);
                snd_opti9xx_write_mask(chip, OPTi9XX_MC_REG(3) as u8, 0x00, 0xff);
                snd_opti9xx_write_mask(chip, OPTi9XX_MC_REG(4) as u8, 0x10 | 0x00, 0x34);
                snd_opti9xx_write_mask(chip, OPTi9XX_MC_REG(5) as u8, 0x20, 0xbf);
            }
            _ => {
                dev_err((*(*chip).card).dev, c"chip %d not supported\n".as_ptr(), (*chip).hardware as c_int);
                return -EINVAL;
            }
        }

        /* PnP resource says it decodes only 10 bits of address */
        match port_arg & 0x3ff {
            0x130 => {
                (*chip).wss_base = 0x530;
                wss_base_bits = 0x00;
            }
            0x204 => {
                (*chip).wss_base = 0x604;
                wss_base_bits = 0x03;
            }
            0x280 => {
                (*chip).wss_base = 0xe80;
                wss_base_bits = 0x01;
            }
            0x340 => {
                (*chip).wss_base = 0xf40;
                wss_base_bits = 0x02;
            }
            _ => {
                dev_warn((*(*chip).card).dev, c"WSS port 0x%lx not valid\n".as_ptr(), port_arg);
                return snd_opti9xx_configure_skip_resources(chip, mpu_port_arg, mpu_irq_arg);
            }
        }
        snd_opti9xx_write_mask(chip, OPTi9XX_MC_REG(1) as u8, wss_base_bits << 4, 0x30);

        match irq_arg {
            5 => irq_bits = 0x05,
            7 => irq_bits = 0x01,
            9 => irq_bits = 0x02,
            10 => irq_bits = 0x03,
            11 => irq_bits = 0x04,
            _ => {
                dev_warn((*(*chip).card).dev, c"WSS irq # %d not valid\n".as_ptr(), irq_arg);
                return snd_opti9xx_configure_skip_resources(chip, mpu_port_arg, mpu_irq_arg);
            }
        }

        match dma1_arg {
            0 => dma_bits = 0x01,
            1 => dma_bits = 0x02,
            3 => dma_bits = 0x03,
            _ => {
                dev_warn((*(*chip).card).dev, c"WSS dma1 # %d not valid\n".as_ptr(), dma1_arg);
                return snd_opti9xx_configure_skip_resources(chip, mpu_port_arg, mpu_irq_arg);
            }
        }

        /* CS4231 || OPTi93X block in C. */
        if dma2_arg >= 0 {
            if dma1_arg == dma2_arg {
                dev_err((*(*chip).card).dev, c"don't want to share dmas\n".as_ptr());
                return -EBUSY;
            }
            match dma2_arg {
                0 | 1 => {}
                _ => {
                    dev_warn((*(*chip).card).dev, c"WSS dma2 # %d not valid\n".as_ptr(), dma2_arg);
                    return snd_opti9xx_configure_skip_resources(chip, mpu_port_arg, mpu_irq_arg);
                }
            }
            dma_bits |= 0x04;
        }

        if (*chip).hardware as c_uint >= OPTi9XX_HW_82C930 {
            snd_opti9xx_write(chip, OPTi9XX_MC_REG(3) as u8, (irq_bits << 3) | dma_bits);
        } else {
            outb((irq_bits << 3) | dma_bits, (*chip).wss_base as c_ulong);
        }

        if (*chip).hardware > OPTi9XX_HW_82C928 as u16 {
            match mpu_port_arg {
                0 | -1 => {}
                0x300 => mpu_port_bits = 0x03,
                0x310 => mpu_port_bits = 0x02,
                0x320 => mpu_port_bits = 0x01,
                0x330 => mpu_port_bits = 0x00,
                _ => {
                    dev_warn((*(*chip).card).dev, c"MPU-401 port 0x%lx not valid\n".as_ptr(), mpu_port_arg);
                    return 0;
                }
            }

            match mpu_irq_arg {
                5 => mpu_irq_bits = 0x02,
                7 => mpu_irq_bits = 0x03,
                9 => mpu_irq_bits = 0x00,
                10 => mpu_irq_bits = 0x01,
                _ => {
                    dev_warn((*(*chip).card).dev, c"MPU-401 irq # %d not valid\n".as_ptr(), mpu_irq_arg);
                    return 0;
                }
            }

            snd_opti9xx_write_mask(
                chip,
                OPTi9XX_MC_REG(6) as u8,
                if mpu_port_arg <= 0 { 0x00 } else { 0x80 | (mpu_port_bits << 5) | (mpu_irq_bits << 3) },
                0xf8,
            );
        }
    }

    0
}

unsafe fn snd_opti9xx_configure_skip_resources(
    chip: *mut snd_opti9xx,
    mpu_port_arg: c_long,
    mpu_irq_arg: c_int,
) -> c_int {
    let mut mpu_port_bits: u8 = 0;
    let mpu_irq_bits: u8;
    unsafe {
        if (*chip).hardware > OPTi9XX_HW_82C928 as u16 {
            match mpu_port_arg {
                0 | -1 => {}
                0x300 => mpu_port_bits = 0x03,
                0x310 => mpu_port_bits = 0x02,
                0x320 => mpu_port_bits = 0x01,
                0x330 => mpu_port_bits = 0x00,
                _ => {
                    dev_warn((*(*chip).card).dev, c"MPU-401 port 0x%lx not valid\n".as_ptr(), mpu_port_arg);
                    return 0;
                }
            }

            match mpu_irq_arg {
                5 => mpu_irq_bits = 0x02,
                7 => mpu_irq_bits = 0x03,
                9 => mpu_irq_bits = 0x00,
                10 => mpu_irq_bits = 0x01,
                _ => {
                    dev_warn((*(*chip).card).dev, c"MPU-401 irq # %d not valid\n".as_ptr(), mpu_irq_arg);
                    return 0;
                }
            }

            snd_opti9xx_write_mask(
                chip,
                OPTi9XX_MC_REG(6) as u8,
                if mpu_port_arg <= 0 { 0x00 } else { 0x80 | (mpu_port_bits << 5) | (mpu_irq_bits << 3) },
                0xf8,
            );
        }
    }
    0
}

/* OPTi93X-only mixer controls:
static const DECLARE_TLV_DB_SCALE(db_scale_5bit_3db_step, -9300, 300, 0);
static const DECLARE_TLV_DB_SCALE(db_scale_5bit, -4650, 150, 0);
static const DECLARE_TLV_DB_SCALE(db_scale_4bit_12db_max, -3300, 300, 0);
static const struct snd_kcontrol_new snd_opti93x_controls[] = { WSS_DOUBLE..., WSS_DOUBLE_TLV... };
*/

unsafe fn snd_opti93x_mixer(chip: *mut snd_wss) -> c_int {
    let card: *mut snd_card;
    let mut idx: c_uint;
    let mut id1: snd_ctl_elem_id;
    let mut id2: snd_ctl_elem_id;
    let mut err: c_int;

    unsafe {
        if snd_BUG_ON(chip.is_null() || (*chip).pcm.is_null()) != 0 {
            return -EINVAL;
        }

        card = (*chip).card;

        strscpy((*card).mixername.as_mut_ptr(), (*(*chip).pcm).name.as_ptr());

        id1 = zeroed();
        id2 = zeroed();
        id1.iface = SNDRV_CTL_ELEM_IFACE_MIXER;
        id2.iface = SNDRV_CTL_ELEM_IFACE_MIXER;
        /* reassign AUX0 switch to CD */
        strscpy(id1.name.as_mut_ptr(), c"Aux Playback Switch".as_ptr());
        strscpy(id2.name.as_mut_ptr(), c"CD Playback Switch".as_ptr());
        err = snd_ctl_rename_id(card, &mut id1, &mut id2);
        if err < 0 {
            dev_err((*card).dev, c"Cannot rename opti93x control\n".as_ptr());
            return err;
        }
        /* reassign AUX1 switch to FM */
        strscpy(id1.name.as_mut_ptr(), c"Aux Playback Switch".as_ptr());
        id1.index = 1;
        strscpy(id2.name.as_mut_ptr(), c"FM Playback Switch".as_ptr());
        err = snd_ctl_rename_id(card, &mut id1, &mut id2);
        if err < 0 {
            dev_err((*card).dev, c"Cannot rename opti93x control\n".as_ptr());
            return err;
        }
        /* remove AUX1 volume */
        strscpy(id1.name.as_mut_ptr(), c"Aux Playback Volume".as_ptr());
        id1.index = 1;
        snd_ctl_remove_id(card, &mut id1);

        /* Replace WSS volume controls with OPTi93x volume controls */
        id1.index = 0;
        idx = 0;
        while idx < snd_opti93x_controls_len() as c_uint {
            strscpy(id1.name.as_mut_ptr(), snd_opti93x_controls_name(idx));
            snd_ctl_remove_id(card, &mut id1);

            err = snd_ctl_add(card, snd_ctl_new1(snd_opti93x_controls_at(idx), chip as *mut c_void));
            if err < 0 {
                return err;
            }
            idx += 1;
        }
        0
    }
}

unsafe fn snd_opti93x_interrupt(_irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    unsafe {
        let chip = dev_id as *mut snd_opti9xx;
        let codec = (*chip).codec;
        let status: u8;

        if codec.is_null() {
            return IRQ_HANDLED;
        }

        status = snd_opti9xx_read(chip, OPTi9XX_MC_REG(11) as u8);
        if (status & OPTi93X_IRQ_PLAYBACK) != 0 && !(*codec).playback_substream.is_null() {
            snd_pcm_period_elapsed((*codec).playback_substream);
        }
        if (status & OPTi93X_IRQ_CAPTURE) != 0 && !(*codec).capture_substream.is_null() {
            snd_wss_overrange(codec);
            snd_pcm_period_elapsed((*codec).capture_substream);
        }
        outb(0x00, OPTi93X_PORT(codec, OPTi93X_STATUS));
        IRQ_HANDLED
    }
}

unsafe fn snd_opti9xx_read_check(card: *mut snd_card, chip: *mut snd_opti9xx) -> c_int {
    let mut value: u8;

    unsafe {
        (*chip).res_mc_base = devm_request_region((*card).dev, (*chip).mc_base, (*chip).mc_base_size, c"OPTi9xx MC".as_ptr());
        if (*chip).res_mc_base.is_null() {
            return -EBUSY;
        }
        value = snd_opti9xx_read(chip, OPTi9XX_MC_REG(1) as u8);
        if value != 0xff && value != inb((*chip).mc_base + OPTi9XX_MC_REG(1) as c_ulong) {
            if value == snd_opti9xx_read(chip, OPTi9XX_MC_REG(1) as u8) {
                return 0;
            }
        }

        /* OPTi93X alternative path requests chip->mc_indir_index and probes register 7. */
        devm_release_resource((*card).dev, (*chip).res_mc_base);
        (*chip).res_mc_base = ptr::null_mut();

        -ENODEV
    }
}

unsafe fn snd_card_opti9xx_detect(card: *mut snd_card, chip: *mut snd_opti9xx) -> c_int {
    let mut i: c_int;
    let mut err: c_int;

    unsafe {
        i = OPTi9XX_HW_82C928 as c_int;
        while i < OPTi9XX_HW_82C930 as c_int {
            err = snd_opti9xx_init(chip, i as u16);
            if err < 0 {
                return err;
            }

            err = snd_opti9xx_read_check(card, chip);
            if err == 0 {
                return 1;
            }
            i += 1;
        }
        -ENODEV
    }
}

/* CONFIG_PNP: snd_card_opti9xx_pnp translated literally. */
unsafe fn snd_card_opti9xx_pnp(
    chip: *mut snd_opti9xx,
    card: *mut pnp_card_link,
    pid: *const pnp_card_device_id,
) -> c_int {
    let mut pdev: *mut pnp_dev;
    let mut err: c_int;
    let devmpu: *mut pnp_dev;
    let mut devmc: *mut pnp_dev;

    unsafe {
        pdev = pnp_request_card_device(card, (*pid).devs[0].id.as_ptr(), ptr::null_mut());
        if pdev.is_null() {
            return -EBUSY;
        }

        err = pnp_activate_dev(pdev);
        if err < 0 {
            dev_err((*(*chip).card).dev, c"AUDIO pnp configure failure: %d\n".as_ptr(), err);
            return err;
        }

        devmc = pnp_request_card_device(card, (*pid).devs[2].id.as_ptr(), ptr::null_mut());
        if devmc.is_null() {
            return -EBUSY;
        }

        err = pnp_activate_dev(devmc);
        if err < 0 {
            dev_err((*(*chip).card).dev, c"MC pnp configure failure: %d\n".as_ptr(), err);
            return err;
        }

        port = pnp_port_start(pdev, 1) as c_long;
        fm_port = pnp_port_start(pdev, 2) as c_long + 8;
        /*
         * The MC(0) is never accessed and card does not
         * include it in the PnP resource range. OPTI93x include it.
         */
        (*chip).mc_base = pnp_port_start(devmc, 0).wrapping_sub(1);
        (*chip).mc_base_size = pnp_port_len(devmc, 0).wrapping_add(1);

        irq = pnp_irq(pdev, 0);
        dma1 = pnp_dma(pdev, 0);
        dma2 = pnp_dma(pdev, 1);

        devmpu = pnp_request_card_device(card, (*pid).devs[1].id.as_ptr(), ptr::null_mut());

        if !devmpu.is_null() && mpu_port > 0 {
            err = pnp_activate_dev(devmpu);
            if err < 0 {
                dev_err((*(*chip).card).dev, c"MPU401 pnp configure failure\n".as_ptr());
                mpu_port = -1;
            } else {
                mpu_port = pnp_port_start(devmpu, 0) as c_long;
                mpu_irq = pnp_irq(devmpu, 0);
            }
        }
        (*pid).driver_data as c_int
    }
}

unsafe fn snd_opti9xx_probe(card: *mut snd_card) -> c_int {
    static possible_ports: [c_long; 5] = [0x530, 0xe80, 0xf40, 0x604, -1];
    let mut error: c_int;
    let xdma2: c_int;
    let chip: *mut snd_opti9xx;
    let mut codec: *mut snd_wss = ptr::null_mut();
    let mut rmidi: *mut snd_rawmidi = ptr::null_mut();
    let mut synth: *mut snd_hwdep = ptr::null_mut();

    unsafe {
        chip = (*card).private_data as *mut snd_opti9xx;

        xdma2 = -1; /* CS4231 || OPTi93X builds use dma2. */

        if port == SNDRV_AUTO_PORT {
            port = snd_legacy_find_free_ioport(possible_ports.as_ptr(), 4);
            if port < 0 {
                dev_err((*card).dev, c"unable to find a free WSS port\n".as_ptr());
                return -EBUSY;
            }
        }
        error = snd_opti9xx_configure(chip, port, irq, dma1, xdma2, mpu_port, mpu_irq);
        if error != 0 {
            return error;
        }

        error = snd_wss_create(card, (*chip).wss_base + 4, -1, irq, dma1, xdma2, WSS_HW_DETECT, 0, &mut codec);
        if error < 0 {
            return error;
        }
        (*chip).codec = codec;
        error = snd_wss_pcm(codec, 0);
        if error < 0 {
            return error;
        }
        error = snd_wss_mixer(codec);
        if error < 0 {
            return error;
        }
        /* OPTi93X: snd_opti93x_mixer(codec), request IRQ with snd_opti93x_interrupt. */
        /* CS4231: snd_wss_timer(codec, 0). */
        (*chip).irq = irq;
        (*card).sync_irq = (*chip).irq;
        strscpy((*card).driver.as_mut_ptr(), (*chip).name.as_ptr());
        sprintf((*card).shortname.as_mut_ptr(), c"OPTi %s".as_ptr(), (*card).driver.as_ptr());
        scnprintf(
            (*card).longname.as_mut_ptr(),
            (*card).longname.len(),
            c"%s, %s at 0x%lx, irq %d, dma %d".as_ptr(),
            (*card).shortname.as_ptr(),
            (*(*codec).pcm).name.as_ptr(),
            (*chip).wss_base + 4,
            irq,
            dma1,
        );

        if mpu_port <= 0 || mpu_port == SNDRV_AUTO_PORT {
            rmidi = ptr::null_mut();
        } else {
            error = snd_mpu401_uart_new(card, 0, MPU401_HW_MPU401, mpu_port, 0, mpu_irq, &mut rmidi);
            if error != 0 {
                dev_warn((*card).dev, c"no MPU-401 device at 0x%lx?\n".as_ptr(), mpu_port);
            }
        }

        if fm_port > 0 && fm_port != SNDRV_AUTO_PORT {
            let mut opl3: *mut snd_opl3 = ptr::null_mut();
            if (*chip).hardware as c_uint == OPTi9XX_HW_82C928
                || (*chip).hardware as c_uint == OPTi9XX_HW_82C929
                || (*chip).hardware as c_uint == OPTi9XX_HW_82C924
            {
                let mut opl4: *mut snd_opl4 = ptr::null_mut();
                /* assume we have an OPL4 */
                snd_opti9xx_write_mask(chip, OPTi9XX_MC_REG(2) as u8, 0x20, 0x20);
                if snd_opl4_create(card, fm_port, fm_port - 8, 2, &mut opl3, &mut opl4) < 0 {
                    /* no luck, use OPL3 instead */
                    snd_opti9xx_write_mask(chip, OPTi9XX_MC_REG(2) as u8, 0x00, 0x20);
                }
            }
            if opl3.is_null() && snd_opl3_create(card, fm_port, fm_port + 2, OPL3_HW_AUTO, 0, &mut opl3) < 0 {
                dev_warn((*card).dev, c"no OPL device at 0x%lx-0x%lx\n".as_ptr(), fm_port, fm_port + 4 - 1);
            }
            if !opl3.is_null() {
                error = snd_opl3_hwdep_new(opl3, 0, 1, &mut synth);
                if error < 0 {
                    return error;
                }
            }
        }

        snd_card_register(card)
    }
}

unsafe fn snd_opti9xx_card_new(pdev: *mut device, cardp: *mut *mut snd_card) -> c_int {
    let mut card: *mut snd_card = ptr::null_mut();
    let err: c_int;

    unsafe {
        err = snd_devm_card_new(pdev, index, id, THIS_MODULE, size_of::<snd_opti9xx>(), &mut card);
        if err < 0 {
            return err;
        }
        *cardp = card;
        0
    }
}

unsafe fn snd_opti9xx_isa_match(_devptr: *mut device, _dev: c_uint) -> c_int {
    unsafe {
        if snd_opti9xx_pnp_is_probed != 0 {
            return 0;
        }
        if isapnp {
            return 0;
        }
    }
    1
}

unsafe fn snd_opti9xx_isa_probe(devptr: *mut device, _dev: c_uint) -> c_int {
    let mut card: *mut snd_card = ptr::null_mut();
    let mut error: c_int;
    static possible_mpu_ports: [c_long; 5] = [0x300, 0x310, 0x320, 0x330, -1];
    static possible_irqs: [c_int; 5] = [9, 10, 11, 7, -1];
    static possible_mpu_irqs: [c_int; 5] = [5, 9, 10, 7, -1];
    static possible_dma1s: [c_int; 4] = [3, 1, 0, -1];
    static possible_dma2s: [[c_int; 2]; 4] = [[1, -1], [0, -1], [-1, -1], [0, -1]];

    unsafe {
        if mpu_port == SNDRV_AUTO_PORT {
            mpu_port = snd_legacy_find_free_ioport(possible_mpu_ports.as_ptr(), 2);
            if mpu_port < 0 {
                dev_err(devptr, c"unable to find a free MPU401 port\n".as_ptr());
                return -EBUSY;
            }
        }
        if irq == SNDRV_AUTO_IRQ {
            irq = snd_legacy_find_free_irq(possible_irqs.as_ptr());
            if irq < 0 {
                dev_err(devptr, c"unable to find a free IRQ\n".as_ptr());
                return -EBUSY;
            }
        }
        if mpu_irq == SNDRV_AUTO_IRQ {
            mpu_irq = snd_legacy_find_free_irq(possible_mpu_irqs.as_ptr());
            if mpu_irq < 0 {
                dev_err(devptr, c"unable to find a free MPU401 IRQ\n".as_ptr());
                return -EBUSY;
            }
        }
        if dma1 == SNDRV_AUTO_DMA {
            dma1 = snd_legacy_find_free_dma(possible_dma1s.as_ptr());
            if dma1 < 0 {
                dev_err(devptr, c"unable to find a free DMA1\n".as_ptr());
                return -EBUSY;
            }
        }
        if dma2 == SNDRV_AUTO_DMA {
            dma2 = snd_legacy_find_free_dma(possible_dma2s[(dma1 % 4) as usize].as_ptr());
            if dma2 < 0 {
                dev_err(devptr, c"unable to find a free DMA2\n".as_ptr());
                return -EBUSY;
            }
        }

        error = snd_opti9xx_card_new(devptr, &mut card);
        if error < 0 {
            return error;
        }

        error = snd_card_opti9xx_detect(card, (*card).private_data as *mut snd_opti9xx);
        if error < 0 {
            return error;
        }
        error = snd_opti9xx_probe(card);
        if error < 0 {
            return error;
        }
        dev_set_drvdata(devptr, card as *mut c_void);
        0
    }
}

unsafe fn snd_opti9xx_suspend(card: *mut snd_card) -> c_int {
    unsafe {
        let chip = (*card).private_data as *mut snd_opti9xx;

        snd_power_change_state(card, SNDRV_CTL_POWER_D3hot);
        ((*(*chip).codec).suspend.unwrap())((*chip).codec);
        0
    }
}

unsafe fn snd_opti9xx_resume(card: *mut snd_card) -> c_int {
    let mut error: c_int;
    let xdma2: c_int;
    unsafe {
        let chip = (*card).private_data as *mut snd_opti9xx;
        xdma2 = -1; /* CS4231 || OPTi93X builds use dma2. */

        error = snd_opti9xx_configure(chip, port, irq, dma1, xdma2, mpu_port, mpu_irq);
        if error != 0 {
            return error;
        }
        ((*(*chip).codec).resume.unwrap())((*chip).codec);
        snd_power_change_state(card, SNDRV_CTL_POWER_D0);
        0
    }
}

unsafe fn snd_opti9xx_isa_suspend(dev: *mut device, _n: c_uint, _state: pm_message_t) -> c_int {
    unsafe { snd_opti9xx_suspend(dev_get_drvdata(dev) as *mut snd_card) }
}

unsafe fn snd_opti9xx_isa_resume(dev: *mut device, _n: c_uint) -> c_int {
    unsafe { snd_opti9xx_resume(dev_get_drvdata(dev) as *mut snd_card) }
}

static mut snd_opti9xx_driver: isa_driver = isa_driver {
    match_: Some(snd_opti9xx_isa_match),
    probe: Some(snd_opti9xx_isa_probe),
    suspend: Some(snd_opti9xx_isa_suspend),
    resume: Some(snd_opti9xx_isa_resume),
    driver: device_driver { name: DEV_NAME },
};

unsafe fn snd_opti9xx_pnp_probe(pcard: *mut pnp_card_link, pid: *const pnp_card_device_id) -> c_int {
    let mut card: *mut snd_card = ptr::null_mut();
    let mut error: c_int;
    let mut hw: c_int;
    let chip: *mut snd_opti9xx;

    unsafe {
        if snd_opti9xx_pnp_is_probed != 0 {
            return -EBUSY;
        }
        if !isapnp {
            return -ENODEV;
        }
        error = snd_opti9xx_card_new(&mut (*(*pcard).card).dev, &mut card);
        if error < 0 {
            return error;
        }
        chip = (*card).private_data as *mut snd_opti9xx;
        (*chip).card = card;

        hw = snd_card_opti9xx_pnp(chip, pcard, pid);
        match hw {
            0x0924 => hw = OPTi9XX_HW_82C924 as c_int,
            0x0925 => hw = OPTi9XX_HW_82C925 as c_int,
            0x0931 => hw = OPTi9XX_HW_82C931 as c_int,
            _ => return -ENODEV,
        }

        error = snd_opti9xx_init(chip, hw as u16);
        if error != 0 {
            return error;
        }
        error = snd_opti9xx_read_check(card, chip);
        if error != 0 {
            dev_err((*card).dev, c"OPTI chip not found\n".as_ptr());
            return error;
        }
        error = snd_opti9xx_probe(card);
        if error < 0 {
            return error;
        }
        pnp_set_card_drvdata(pcard, card as *mut c_void);
        snd_opti9xx_pnp_is_probed = 1;
        0
    }
}

unsafe fn snd_opti9xx_pnp_remove(_pcard: *mut pnp_card_link) {
    unsafe {
        snd_opti9xx_pnp_is_probed = 0;
    }
}

unsafe fn snd_opti9xx_pnp_suspend(pcard: *mut pnp_card_link, _state: pm_message_t) -> c_int {
    unsafe { snd_opti9xx_suspend(pnp_get_card_drvdata(pcard) as *mut snd_card) }
}

unsafe fn snd_opti9xx_pnp_resume(pcard: *mut pnp_card_link) -> c_int {
    unsafe { snd_opti9xx_resume(pnp_get_card_drvdata(pcard) as *mut snd_card) }
}

static mut opti9xx_pnpc_driver: pnp_card_driver = pnp_card_driver {
    flags: PNP_DRIVER_RES_DISABLE,
    name: DEV_NAME,
    id_table: ptr::null(),
    probe: Some(snd_opti9xx_pnp_probe),
    remove: Some(snd_opti9xx_pnp_remove),
    suspend: Some(snd_opti9xx_pnp_suspend),
    resume: Some(snd_opti9xx_pnp_resume),
};

unsafe fn alsa_card_opti9xx_init() -> c_int {
    unsafe {
        pnp_register_card_driver(&mut opti9xx_pnpc_driver);
        if snd_opti9xx_pnp_is_probed != 0 {
            return 0;
        }
        pnp_unregister_card_driver(&mut opti9xx_pnpc_driver);
        isa_register_driver(&mut snd_opti9xx_driver, 1)
    }
}

unsafe fn alsa_card_opti9xx_exit() {
    unsafe {
        if snd_opti9xx_pnp_is_probed == 0 {
            isa_unregister_driver(&mut snd_opti9xx_driver);
            return;
        }
        pnp_unregister_card_driver(&mut opti9xx_pnpc_driver);
    }
}

/* module_init(alsa_card_opti9xx_init)
 * module_exit(alsa_card_opti9xx_exit)
 */

#[repr(C)]
pub struct snd_card {
    dev: *mut device,
    private_data: *mut c_void,
    sync_irq: c_int,
    driver: [c_char; 16],
    shortname: [c_char; 32],
    longname: [c_char; 80],
    mixername: [c_char; 80],
}
#[repr(C)]
pub struct snd_wss {
    card: *mut snd_card,
    pcm: *mut snd_pcm,
    port: c_ulong,
    playback_substream: *mut snd_pcm_substream,
    capture_substream: *mut snd_pcm_substream,
    suspend: Option<unsafe extern "C" fn(*mut snd_wss)>,
    resume: Option<unsafe extern "C" fn(*mut snd_wss)>,
}
#[repr(C)]
pub struct snd_pcm {
    name: [c_char; 80],
}
#[repr(C)]
pub struct snd_ctl_elem_id {
    iface: c_uint,
    index: c_uint,
    name: [c_char; 44],
}
#[repr(C)]
pub struct isa_driver {
    match_: Option<unsafe fn(*mut device, c_uint) -> c_int>,
    probe: Option<unsafe fn(*mut device, c_uint) -> c_int>,
    suspend: Option<unsafe fn(*mut device, c_uint, pm_message_t) -> c_int>,
    resume: Option<unsafe fn(*mut device, c_uint) -> c_int>,
    driver: device_driver,
}
#[repr(C)]
pub struct device_driver {
    name: *const c_char,
}
#[repr(C)]
pub struct pnp_card_driver {
    flags: c_uint,
    name: *const c_char,
    id_table: *const pnp_card_device_id,
    probe: Option<unsafe fn(*mut pnp_card_link, *const pnp_card_device_id) -> c_int>,
    remove: Option<unsafe fn(*mut pnp_card_link)>,
    suspend: Option<unsafe fn(*mut pnp_card_link, pm_message_t) -> c_int>,
    resume: Option<unsafe fn(*mut pnp_card_link) -> c_int>,
}

#[repr(C)]
pub struct pnp_card_device_id {
    id: [c_char; 8],
    devs: [pnp_card_devs; 3],
    driver_data: c_ulong,
}
#[repr(C)]
pub struct pnp_card_devs {
    id: [c_char; 8],
}
#[repr(C)]
pub struct pnp_card_link {
    card: *mut pnp_card,
}
#[repr(C)]
pub struct pnp_card {
    dev: device,
}
pub enum snd_rawmidi {}
pub enum snd_hwdep {}
pub enum snd_opl3 {}
pub enum snd_opl4 {}
pub enum snd_pcm_substream {}
pub enum resource {}
pub enum pnp_dev {}
#[repr(C)]
pub struct device {
    _private: [u8; 0],
}
#[repr(C)]
pub struct spinlock_t {
    _private: [u8; 0],
}
pub type pm_message_t = c_int;
pub type irqreturn_t = c_int;

unsafe extern "C" {
    static KBUILD_MODNAME: *const c_char;
    static THIS_MODULE: *mut c_void;
    static SNDRV_DEFAULT_STR1: *const c_char;
    static SNDRV_DEFAULT_IDX1: c_int;
    static SNDRV_DEFAULT_PORT1: c_long;
    static SNDRV_DEFAULT_IRQ1: c_int;
    static SNDRV_DEFAULT_DMA1: c_int;
    static SNDRV_AUTO_PORT: c_long;
    static SNDRV_AUTO_IRQ: c_int;
    static SNDRV_AUTO_DMA: c_int;
    static ENODEV: c_int;
    static EINVAL: c_int;
    static EBUSY: c_int;
    static SNDRV_CTL_ELEM_IFACE_MIXER: c_uint;
    static IRQ_HANDLED: irqreturn_t;
    static WSS_HW_DETECT: c_int;
    static WSS_HW_OPTI93X: c_int;
    static WSS_HWSHARE_IRQ: c_int;
    static MPU401_HW_MPU401: c_int;
    static OPL3_HW_AUTO: c_int;
    static SNDRV_CTL_POWER_D3hot: c_int;
    static SNDRV_CTL_POWER_D0: c_int;
    static PNP_DRIVER_RES_DISABLE: c_uint;

    fn strscpy(dst: *mut c_char, src: *const c_char) -> isize;
    fn sprintf(buf: *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn scnprintf(buf: *mut c_char, size: usize, fmt: *const c_char, ...) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn outb(value: u8, port: c_ulong);
    fn inb(port: c_ulong) -> u8;
    fn spin_lock_init(lock: *mut spinlock_t);
    fn spin_lock_irqsave(lock: *mut spinlock_t);
    fn spin_unlock_irqrestore(lock: *mut spinlock_t);
    fn devm_request_region(dev: *mut device, start: c_ulong, n: c_ulong, name: *const c_char) -> *mut resource;
    fn devm_release_resource(dev: *mut device, res: *mut resource);
    fn snd_BUG_ON(cond: bool) -> c_int;
    fn snd_ctl_rename_id(card: *mut snd_card, src_id: *mut snd_ctl_elem_id, dst_id: *mut snd_ctl_elem_id) -> c_int;
    fn snd_ctl_remove_id(card: *mut snd_card, id: *mut snd_ctl_elem_id) -> c_int;
    fn snd_ctl_add(card: *mut snd_card, kcontrol: *mut c_void) -> c_int;
    fn snd_ctl_new1(ncontrol: *const c_void, private_data: *mut c_void) -> *mut c_void;
    fn snd_opti93x_controls_len() -> usize;
    fn snd_opti93x_controls_name(idx: c_uint) -> *const c_char;
    fn snd_opti93x_controls_at(idx: c_uint) -> *const c_void;
    fn snd_pcm_period_elapsed(substream: *mut snd_pcm_substream);
    fn snd_wss_overrange(codec: *mut snd_wss);
    fn pnp_request_card_device(card: *mut pnp_card_link, id: *const c_char, from: *mut pnp_dev) -> *mut pnp_dev;
    fn pnp_activate_dev(dev: *mut pnp_dev) -> c_int;
    fn pnp_port_start(dev: *mut pnp_dev, bar: c_uint) -> c_ulong;
    fn pnp_port_len(dev: *mut pnp_dev, bar: c_uint) -> c_ulong;
    fn pnp_irq(dev: *mut pnp_dev, bar: c_uint) -> c_int;
    fn pnp_dma(dev: *mut pnp_dev, bar: c_uint) -> c_int;
    fn snd_legacy_find_free_ioport(ports: *const c_long, size: c_int) -> c_long;
    fn snd_legacy_find_free_irq(irqs: *const c_int) -> c_int;
    fn snd_legacy_find_free_dma(dmas: *const c_int) -> c_int;
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
    fn snd_wss_pcm(chip: *mut snd_wss, device: c_int) -> c_int;
    fn snd_wss_mixer(chip: *mut snd_wss) -> c_int;
    fn snd_wss_timer(chip: *mut snd_wss, device: c_int) -> c_int;
    fn devm_request_irq(
        dev: *mut device,
        irq: c_uint,
        handler: unsafe fn(c_int, *mut c_void) -> irqreturn_t,
        irqflags: c_ulong,
        devname: *const c_char,
        dev_id: *mut c_void,
    ) -> c_int;
    fn snd_mpu401_uart_new(
        card: *mut snd_card,
        device: c_int,
        hardware: c_int,
        port: c_long,
        integrated: c_int,
        irq: c_int,
        rrawmidi: *mut *mut snd_rawmidi,
    ) -> c_int;
    fn snd_opl4_create(
        card: *mut snd_card,
        fm_port: c_long,
        pcm_port: c_long,
        seq_device: c_int,
        ropl3: *mut *mut snd_opl3,
        ropl4: *mut *mut snd_opl4,
    ) -> c_int;
    fn snd_opl3_create(
        card: *mut snd_card,
        l_port: c_long,
        r_port: c_long,
        hardware: c_int,
        integrated: c_int,
        ropl3: *mut *mut snd_opl3,
    ) -> c_int;
    fn snd_opl3_hwdep_new(opl3: *mut snd_opl3, device: c_int, seq_device: c_int, rhwdep: *mut *mut snd_hwdep) -> c_int;
    fn snd_card_register(card: *mut snd_card) -> c_int;
    fn snd_devm_card_new(
        parent: *mut device,
        idx: c_int,
        xid: *mut c_char,
        module: *mut c_void,
        extra_size: usize,
        card_ret: *mut *mut snd_card,
    ) -> c_int;
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn snd_power_change_state(card: *mut snd_card, state: c_int) -> c_int;
    fn pnp_set_card_drvdata(pcard: *mut pnp_card_link, data: *mut c_void);
    fn pnp_get_card_drvdata(pcard: *mut pnp_card_link) -> *mut c_void;
    fn pnp_register_card_driver(driver: *mut pnp_card_driver) -> c_int;
    fn pnp_unregister_card_driver(driver: *mut pnp_card_driver);
    fn isa_register_driver(driver: *mut isa_driver, ndev: c_uint) -> c_int;
    fn isa_unregister_driver(driver: *mut isa_driver);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
