// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Copyright (c) by Jaroslav Kysela <perex@perex.cz>
 *                   Uros Bizjak <uros@kss-loka.si>
 *
 *  Lowlevel routines for control of Sound Blaster cards
 */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

// Original C dependencies:
// linux/delay.h, linux/init.h, linux/interrupt.h, linux/slab.h,
// linux/ioport.h, linux/module.h, linux/io.h, sound/core.h, sound/sb.h,
// sound/initval.h, asm/dma.h
//
// MODULE_AUTHOR("Jaroslav Kysela <perex@perex.cz>");
// MODULE_DESCRIPTION("ALSA lowlevel driver for Sound Blaster cards");
// MODULE_LICENSE("GPL");

const BUSY_LOOPS: c_int = 100000;

// IO_DEBUG is undefined in the original C file.

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct resource {
    _private: [u8; 0],
}

#[repr(C)]
pub struct spinlock_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_card {
    pub dev: *mut device,
    pub sync_irq: c_int,
}

#[repr(C)]
pub struct snd_sb {
    pub reg_lock: spinlock_t,
    pub open_lock: spinlock_t,
    pub midi_input_lock: spinlock_t,
    pub mixer_lock: spinlock_t,
    pub irq: c_int,
    pub dma8: c_int,
    pub dma16: c_int,
    pub port: c_ulong,
    pub res_port: *mut resource,
    pub card: *mut snd_card,
    pub hardware: c_ushort,
    pub name: [c_char; 32],
    pub version: c_uint,
}

pub type c_ushort = u16;
pub type irq_handler_t = Option<unsafe extern "C" fn(c_int, *mut c_void) -> c_int>;

unsafe extern "C" {
    static mut STATUS: c_int;
    static mut COMMAND: c_int;
    static mut DATA_AVAIL: c_int;
    static mut READ: c_int;
    static mut RESET: c_int;

    static mut SB_DSP_GET_VERSION: c_uchar;

    static mut SB_HW_AUTO: c_ushort;
    static mut SB_HW_10: c_ushort;
    static mut SB_HW_20: c_ushort;
    static mut SB_HW_201: c_ushort;
    static mut SB_HW_PRO: c_ushort;
    static mut SB_HW_16: c_ushort;
    static mut SB_HW_ALS100: c_ushort;
    static mut SB_HW_ALS4000: c_ushort;
    static mut SB_HW_DT019X: c_ushort;
    static mut SB_HW_CS5530: c_ushort;
    static mut SB_HW_JAZZ16: c_ushort;

    static mut ENODEV: c_int;
    static mut EINVAL: c_int;
    static mut ENOMEM: c_int;
    static mut EBUSY: c_int;
    static mut GFP_KERNEL: c_uint;
    static mut IRQF_SHARED: c_ulong;

    fn SBP(chip: *mut snd_sb, reg: c_int) -> c_ulong;
    fn inb(port: c_ulong) -> c_uint;
    fn outb(value: c_uchar, port: c_ulong);
    fn udelay(usecs: c_ulong);

    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_info(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn sprintf(buf: *mut c_char, fmt: *const c_char, ...);

    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_request_irq(
        dev: *mut device,
        irq: c_int,
        handler: irq_handler_t,
        flags: c_ulong,
        name: *const c_char,
        dev_id: *mut c_void,
    ) -> c_int;
    fn devm_request_region(
        dev: *mut device,
        start: c_ulong,
        n: c_ulong,
        name: *const c_char,
    ) -> *mut resource;
    fn snd_devm_request_dma(dev: *mut device, dma: c_int, name: *const c_char) -> c_int;

    fn spin_lock_init(lock: *mut spinlock_t);
    fn spin_lock_irqsave(lock: *mut spinlock_t, flags: *mut c_ulong);
    fn spin_unlock_irqrestore(lock: *mut spinlock_t, flags: c_ulong);

    fn snd_BUG_ON(condition: bool) -> bool;

    fn snd_sbmixer_write();
    fn snd_sbmixer_read();
    fn snd_sbmixer_new();
    fn snd_sbmixer_add_ctl();

    // CONFIG_PM exports in the original C file:
    fn snd_sbmixer_suspend();
    fn snd_sbmixer_resume();
}

pub type c_uchar = u8;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_sbdsp_command(chip: *mut snd_sb, val: c_uchar) -> c_int {
    let mut i: c_int;

    // #ifdef IO_DEBUG
    // dev_dbg(chip->card->dev, "command 0x%x\n", val);
    // #endif
    i = BUSY_LOOPS;
    while i != 0 {
        if (unsafe { inb(SBP(chip, STATUS)) } & 0x80) == 0 {
            unsafe {
                outb(val, SBP(chip, COMMAND));
            }
            return 1;
        }
        i -= 1;
    }
    unsafe {
        dev_dbg(
            (*(*chip).card).dev,
            c"%s [0x%lx]: timeout (0x%x)\n".as_ptr(),
            c"snd_sbdsp_command".as_ptr(),
            (*chip).port,
            val as c_int,
        );
    }
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_sbdsp_get_byte(chip: *mut snd_sb) -> c_int {
    let mut val: c_int;
    let mut i: c_int;

    i = BUSY_LOOPS;
    while i != 0 {
        if unsafe { inb(SBP(chip, DATA_AVAIL)) } & 0x80 != 0 {
            val = unsafe { inb(SBP(chip, READ)) as c_int };
            // #ifdef IO_DEBUG
            // dev_dbg(chip->card->dev, "get_byte 0x%x\n", val);
            // #endif
            return val;
        }
        i -= 1;
    }
    unsafe {
        dev_dbg(
            (*(*chip).card).dev,
            c"%s [0x%lx]: timeout\n".as_ptr(),
            c"snd_sbdsp_get_byte".as_ptr(),
            (*chip).port,
        );
        -ENODEV
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_sbdsp_reset(chip: *mut snd_sb) -> c_int {
    let mut i: c_int;

    unsafe {
        outb(1, SBP(chip, RESET));
        udelay(10);
        outb(0, SBP(chip, RESET));
        udelay(30);
    }
    i = BUSY_LOOPS;
    while i != 0 {
        if unsafe { inb(SBP(chip, DATA_AVAIL)) } & 0x80 != 0 {
            if unsafe { inb(SBP(chip, READ)) } == 0xaa {
                return 0;
            } else {
                break;
            }
        }
        i -= 1;
    }
    if unsafe { !(*chip).card.is_null() } {
        unsafe {
            dev_dbg(
                (*(*chip).card).dev,
                c"%s [0x%lx] failed...\n".as_ptr(),
                c"snd_sbdsp_reset".as_ptr(),
                (*chip).port,
            );
        }
    }
    unsafe { -ENODEV }
}

unsafe extern "C" fn snd_sbdsp_version(chip: *mut snd_sb) -> c_int {
    let mut result: c_uint;

    unsafe {
        snd_sbdsp_command(chip, SB_DSP_GET_VERSION);
        result = ((snd_sbdsp_get_byte(chip) as i16) as c_uint) << 8;
        result |= (snd_sbdsp_get_byte(chip) as i16) as c_uint;
    }
    result as c_int
}

unsafe extern "C" fn snd_sbdsp_probe(chip: *mut snd_sb) -> c_int {
    let version: c_int;
    let major: c_int;
    let minor: c_int;
    let str_: *const c_char;

    /*
     *  initialization sequence
     */

    let mut flags: c_ulong = 0;
    unsafe {
        spin_lock_irqsave(&mut (*chip).reg_lock, &mut flags);
        if snd_sbdsp_reset(chip) < 0 {
            spin_unlock_irqrestore(&mut (*chip).reg_lock, flags);
            return -ENODEV;
        }
        version = snd_sbdsp_version(chip);
        if version < 0 {
            spin_unlock_irqrestore(&mut (*chip).reg_lock, flags);
            return -ENODEV;
        }
        spin_unlock_irqrestore(&mut (*chip).reg_lock, flags);
    }

    major = version >> 8;
    minor = version & 0xff;
    unsafe {
        dev_dbg(
            (*(*chip).card).dev,
            c"SB [0x%lx]: DSP chip found, version = %i.%i\n".as_ptr(),
            (*chip).port,
            major,
            minor,
        );
    }

    unsafe {
        if (*chip).hardware == SB_HW_AUTO {
            match major {
                1 => {
                    (*chip).hardware = SB_HW_10;
                    str_ = c"1.0".as_ptr();
                }
                2 => {
                    if minor != 0 {
                        (*chip).hardware = SB_HW_201;
                        str_ = c"2.01+".as_ptr();
                    } else {
                        (*chip).hardware = SB_HW_20;
                        str_ = c"2.0".as_ptr();
                    }
                }
                3 => {
                    (*chip).hardware = SB_HW_PRO;
                    str_ = c"Pro".as_ptr();
                }
                4 => {
                    (*chip).hardware = SB_HW_16;
                    str_ = c"16".as_ptr();
                }
                _ => {
                    dev_info(
                        (*(*chip).card).dev,
                        c"SB [0x%lx]: unknown DSP chip version %i.%i\n".as_ptr(),
                        (*chip).port,
                        major,
                        minor,
                    );
                    return -ENODEV;
                }
            }
        } else if (*chip).hardware == SB_HW_ALS100 {
            str_ = c"16 (ALS-100)".as_ptr();
        } else if (*chip).hardware == SB_HW_ALS4000 {
            str_ = c"16 (ALS-4000)".as_ptr();
        } else if (*chip).hardware == SB_HW_DT019X {
            str_ = c"(DT019X/ALS007)".as_ptr();
        } else if (*chip).hardware == SB_HW_CS5530 {
            str_ = c"16 (CS5530)".as_ptr();
        } else if (*chip).hardware == SB_HW_JAZZ16 {
            str_ = c"Pro (Jazz16)".as_ptr();
        } else {
            return -ENODEV;
        }

        sprintf(
            (*chip).name.as_mut_ptr(),
            c"Sound Blaster %s".as_ptr(),
            str_,
        );
        (*chip).version = ((major << 8) | minor) as c_uint;
    }
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_sbdsp_create(
    card: *mut snd_card,
    port: c_ulong,
    irq: c_int,
    irq_handler: irq_handler_t,
    dma8: c_int,
    mut dma16: c_int,
    hardware: c_ushort,
    r_chip: *mut *mut snd_sb,
) -> c_int {
    let chip: *mut snd_sb;
    let err: c_int;

    unsafe {
        if snd_BUG_ON(r_chip.is_null()) {
            return -EINVAL;
        }
        *r_chip = core::ptr::null_mut();
        chip = devm_kzalloc((*card).dev, core::mem::size_of::<snd_sb>(), GFP_KERNEL) as *mut snd_sb;
        if chip.is_null() {
            return -ENOMEM;
        }
        spin_lock_init(&mut (*chip).reg_lock);
        spin_lock_init(&mut (*chip).open_lock);
        spin_lock_init(&mut (*chip).midi_input_lock);
        spin_lock_init(&mut (*chip).mixer_lock);
        (*chip).irq = -1;
        (*chip).dma8 = -1;
        (*chip).dma16 = -1;
        (*chip).port = port;

        if devm_request_irq(
            (*card).dev,
            irq,
            irq_handler,
            if hardware == SB_HW_ALS4000 || hardware == SB_HW_CS5530 {
                IRQF_SHARED
            } else {
                0
            },
            c"SoundBlaster".as_ptr(),
            chip as *mut c_void,
        ) != 0
        {
            dev_err((*card).dev, c"sb: can't grab irq %d\n".as_ptr(), irq);
            return -EBUSY;
        }
        (*chip).irq = irq;
        (*card).sync_irq = (*chip).irq;

        if hardware != SB_HW_ALS4000 {
            (*chip).res_port = devm_request_region((*card).dev, port, 16, c"SoundBlaster".as_ptr());
            if (*chip).res_port.is_null() {
                dev_err((*card).dev, c"sb: can't grab port 0x%lx\n".as_ptr(), port);
                return -EBUSY;
            }

            // #ifdef CONFIG_ISA
            if dma8 >= 0 && snd_devm_request_dma((*card).dev, dma8, c"SoundBlaster - 8bit".as_ptr()) != 0 {
                dev_err((*card).dev, c"sb: can't grab DMA8 %d\n".as_ptr(), dma8);
                return -EBUSY;
            }
            (*chip).dma8 = dma8;
            if dma16 >= 0 {
                if hardware != SB_HW_ALS100 && (dma16 < 5 || dma16 > 7) {
                    /* no duplex */
                    dma16 = -1;
                } else if snd_devm_request_dma((*card).dev, dma16, c"SoundBlaster - 16bit".as_ptr()) != 0 {
                    dev_err((*card).dev, c"sb: can't grab DMA16 %d\n".as_ptr(), dma16);
                    return -EBUSY;
                }
            }
            (*chip).dma16 = dma16;
            // #endif
        }

        (*chip).card = card;
        (*chip).hardware = hardware;
        err = snd_sbdsp_probe(chip);
        if err < 0 {
            return err;
        }
        *r_chip = chip;
    }
    0
}

// EXPORT_SYMBOL(snd_sbdsp_command);
// EXPORT_SYMBOL(snd_sbdsp_get_byte);
// EXPORT_SYMBOL(snd_sbdsp_reset);
// EXPORT_SYMBOL(snd_sbdsp_create);
/* sb_mixer.c */
// EXPORT_SYMBOL(snd_sbmixer_write);
// EXPORT_SYMBOL(snd_sbmixer_read);
// EXPORT_SYMBOL(snd_sbmixer_new);
// EXPORT_SYMBOL(snd_sbmixer_add_ctl);
// #ifdef CONFIG_PM
// EXPORT_SYMBOL(snd_sbmixer_suspend);
// EXPORT_SYMBOL(snd_sbmixer_resume);
// #endif

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
