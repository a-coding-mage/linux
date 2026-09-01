// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *   ALSA driver for ICEnsemble ICE1712 (Envy24)
 *
 *   AK4524 / AK4528 / AK4529 / AK4355 / AK4381 interface
 *
 *	Copyright (c) 2000 Jaroslav Kysela <perex@perex.cz>
 */

// C dependencies:
// <linux/io.h>, <linux/delay.h>, <linux/interrupt.h>, <linux/slab.h>,
// <linux/init.h>, <linux/module.h>, <sound/core.h>, <sound/initval.h>,
// "ice1712.h"

extern "C" {
    fn snd_ice1712_save_gpio_status(ice: *mut snd_ice1712);
    fn snd_ice1712_restore_gpio_status(ice: *mut snd_ice1712);
    fn snd_ice1712_gpio_read(ice: *mut snd_ice1712) -> c_uint;
    fn snd_ice1712_gpio_write(ice: *mut snd_ice1712, data: c_uint);
    fn udelay(usecs: c_ulong);
    fn snd_BUG_ON(condition: bool) -> bool;
    fn snd_akm4xxx_init(ak: *mut snd_akm4xxx);
    fn snd_akm4xxx_build_controls(ak: *mut snd_akm4xxx) -> c_int;
    fn kmalloc(size: usize, flags: c_uint) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
}

type c_void = core::ffi::c_void;
type c_int = i32;
type c_uint = u32;
type c_ulong = u64;
type c_uchar = u8;

const ENOMEM: c_int = 12;
const GFP_KERNEL: c_uint = 0;

#[repr(C)]
pub struct snd_akm4xxx {
    pub card: *mut c_void,
    pub private_value: [c_ulong; 1],
    pub private_data: [*mut c_void; 1],
    pub ops: snd_akm4xxx_ops,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_akm4xxx_ops {
    pub lock: Option<unsafe extern "C" fn(ak: *mut snd_akm4xxx, chip: c_int)>,
    pub unlock: Option<unsafe extern "C" fn(ak: *mut snd_akm4xxx, chip: c_int)>,
    pub write: Option<
        unsafe extern "C" fn(
            ak: *mut snd_akm4xxx,
            chip: c_int,
            addr: c_uchar,
            data: c_uchar,
        ),
    >,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ak4xxx_private {
    pub add_flags: c_uint,
    pub mask_flags: c_uint,
    pub cs_mask: c_uint,
    pub cs_addr: c_uint,
    pub cif: c_int,
    pub caddr: c_uint,
    pub clk_mask: c_uint,
    pub data_mask: c_uint,
    pub cs_none: c_uint,
}

#[repr(C)]
pub struct snd_ice1712 {
    pub card: *mut c_void,
    pub akm: *mut snd_akm4xxx,
    pub akm_codecs: c_uint,
}

unsafe extern "C" fn snd_ice1712_akm4xxx_lock(ak: *mut snd_akm4xxx, _chip: c_int) {
    let ice = (*ak).private_data[0] as *mut snd_ice1712;

    snd_ice1712_save_gpio_status(ice);
}

unsafe extern "C" fn snd_ice1712_akm4xxx_unlock(ak: *mut snd_akm4xxx, _chip: c_int) {
    let ice = (*ak).private_data[0] as *mut snd_ice1712;

    snd_ice1712_restore_gpio_status(ice);
}

/*
 * write AK4xxx register
 */
unsafe extern "C" fn snd_ice1712_akm4xxx_write(
    ak: *mut snd_akm4xxx,
    chip: c_int,
    addr: c_uchar,
    data: c_uchar,
) {
    let mut tmp: c_uint;
    let mut idx: c_int;
    let mut addrdata: c_uint;
    let priv_ = (*ak).private_value[0] as *mut snd_ak4xxx_private;
    let ice = (*ak).private_data[0] as *mut snd_ice1712;

    if snd_BUG_ON(chip < 0 || chip >= 4) {
        return;
    }

    tmp = snd_ice1712_gpio_read(ice);
    tmp |= (*priv_).add_flags;
    tmp &= !(*priv_).mask_flags;
    if (*priv_).cs_mask == (*priv_).cs_addr {
        if (*priv_).cif != 0 {
            tmp |= (*priv_).cs_mask; /* start without chip select */
        } else {
            tmp &= !(*priv_).cs_mask; /* chip select low */
            snd_ice1712_gpio_write(ice, tmp);
            udelay(1);
        }
    } else {
        /* doesn't handle cf=1 yet */
        tmp &= !(*priv_).cs_mask;
        tmp |= (*priv_).cs_addr;
        snd_ice1712_gpio_write(ice, tmp);
        udelay(1);
    }

    /* build I2C address + data byte */
    addrdata = ((*priv_).caddr << 6) | 0x20 | ((addr as c_uint) & 0x1f);
    addrdata = (addrdata << 8) | (data as c_uint);
    idx = 15;
    while idx >= 0 {
        /* drop clock */
        tmp &= !(*priv_).clk_mask;
        snd_ice1712_gpio_write(ice, tmp);
        udelay(1);
        /* set data */
        if (addrdata & (1_u32 << idx)) != 0 {
            tmp |= (*priv_).data_mask;
        } else {
            tmp &= !(*priv_).data_mask;
        }
        snd_ice1712_gpio_write(ice, tmp);
        udelay(1);
        /* raise clock */
        tmp |= (*priv_).clk_mask;
        snd_ice1712_gpio_write(ice, tmp);
        udelay(1);

        idx -= 1;
    }

    if (*priv_).cs_mask == (*priv_).cs_addr {
        if (*priv_).cif != 0 {
            /* assert a cs pulse to trigger */
            tmp &= !(*priv_).cs_mask;
            snd_ice1712_gpio_write(ice, tmp);
            udelay(1);
        }
        tmp |= (*priv_).cs_mask; /* chip select high to trigger */
    } else {
        tmp &= !(*priv_).cs_mask;
        tmp |= (*priv_).cs_none; /* deselect address */
    }
    snd_ice1712_gpio_write(ice, tmp);
    udelay(1);
}

/*
 * initialize the struct snd_akm4xxx record with the template
 */
#[no_mangle]
pub unsafe extern "C" fn snd_ice1712_akm4xxx_init(
    ak: *mut snd_akm4xxx,
    temp: *const snd_akm4xxx,
    _priv: *const snd_ak4xxx_private,
    ice: *mut snd_ice1712,
) -> c_int {
    let priv_: *mut snd_ak4xxx_private;

    if !_priv.is_null() {
        priv_ = kmalloc(core::mem::size_of::<snd_ak4xxx_private>(), GFP_KERNEL)
            as *mut snd_ak4xxx_private;
        if priv_.is_null() {
            return -ENOMEM;
        }
        *priv_ = *_priv;
    } else {
        priv_ = core::ptr::null_mut();
    }
    core::ptr::copy_nonoverlapping(temp, ak, 1);
    (*ak).card = (*ice).card;
    (*ak).private_value[0] = priv_ as c_ulong;
    (*ak).private_data[0] = ice as *mut c_void;
    if (*ak).ops.lock.is_none() {
        (*ak).ops.lock = Some(snd_ice1712_akm4xxx_lock);
    }
    if (*ak).ops.unlock.is_none() {
        (*ak).ops.unlock = Some(snd_ice1712_akm4xxx_unlock);
    }
    if (*ak).ops.write.is_none() {
        (*ak).ops.write = Some(snd_ice1712_akm4xxx_write);
    }
    snd_akm4xxx_init(ak);
    0
}

#[no_mangle]
pub unsafe extern "C" fn snd_ice1712_akm4xxx_free(ice: *mut snd_ice1712) {
    let mut akidx: c_uint;
    if (*ice).akm.is_null() {
        return;
    }
    akidx = 0;
    while akidx < (*ice).akm_codecs {
        let ak = (*ice).akm.add(akidx as usize);
        kfree((*ak).private_value[0] as *mut c_void);
        akidx += 1;
    }
    kfree((*ice).akm as *mut c_void);
}

/*
 * build AK4xxx controls
 */
#[no_mangle]
pub unsafe extern "C" fn snd_ice1712_akm4xxx_build_controls(ice: *mut snd_ice1712) -> c_int {
    let mut akidx: c_uint;
    let mut err: c_int;

    akidx = 0;
    while akidx < (*ice).akm_codecs {
        let ak = (*ice).akm.add(akidx as usize);
        err = snd_akm4xxx_build_controls(ak);
        if err < 0 {
            return err;
        }
        akidx += 1;
    }
    0
}

// EXPORT_SYMBOL(snd_ice1712_akm4xxx_init);
// EXPORT_SYMBOL(snd_ice1712_akm4xxx_free);
// EXPORT_SYMBOL(snd_ice1712_akm4xxx_build_controls);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
