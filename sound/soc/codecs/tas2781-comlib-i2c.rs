// SPDX-License-Identifier: GPL-2.0
//
// TAS2563/TAS2781 Common functions for HDA and ASoC Audio drivers based on I2C
//
// Copyright 2025 Texas Instruments, Inc.
//
// Author: Shenghao Ding <shenghao-ding@ti.com>

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

// C include dependencies:
// linux/cleanup.h, linux/crc8.h, linux/firmware.h, linux/gpio/consumer.h,
// linux/i2c.h, linux/init.h, linux/interrupt.h, linux/module.h, linux/of.h,
// linux/of_irq.h, linux/regmap.h, linux/slab.h, sound/pcm_params.h,
// sound/soc.h, sound/tas2781.h, sound/tas2781-comlib-i2c.h

pub const REGCACHE_NONE: c_uint = 0;
pub const GFP_KERNEL: c_uint = 0;
pub const FW_ACTION_UEVENT: c_uint = 0;
pub const EINVAL: c_int = 22;

pub const TASDEVICE_PAGE_SELECT: c_uint = 0;
pub const TASDEVICE_BOOKCTL_REG: c_uint = 127;
pub const TASDEVICE_REG_SWRESET: c_uint = 0;
pub const TASDEVICE_REG_SWRESET_RESET: c_uint = 1;
pub const TAS5825_REG_SWRESET_RESET: c_uint = 1;
pub const TAS5802: c_uint = 0x5802;
pub const TASDEVICE_CRC8_POLYNOMIAL: u8 = 0;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct gpio_desc {
    _private: [u8; 0],
}

#[repr(C)]
pub struct firmware {
    _private: [u8; 0],
}

#[repr(C)]
pub struct module {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct i2c_client {
    pub dev: device,
    pub addr: c_uint,
}

#[repr(C)]
pub struct regmap_range_cfg {
    pub range_min: c_uint,
    pub range_max: c_uint,
    pub selector_reg: c_uint,
    pub selector_mask: c_uint,
    pub selector_shift: c_uint,
    pub window_start: c_uint,
    pub window_len: c_uint,
}

#[repr(C)]
pub struct regmap_config {
    pub reg_bits: c_uint,
    pub val_bits: c_uint,
    pub cache_type: c_uint,
    pub ranges: *const regmap_range_cfg,
    pub num_ranges: c_uint,
    pub max_register: c_uint,
}

#[repr(C)]
pub struct tasdevice {
    pub dev_addr: c_uint,
    pub cur_book: c_int,
    pub cur_prog: c_int,
    pub cur_conf: c_int,
}

pub type UpdateBitsFn = unsafe extern "C" fn(
    *mut tasdevice_priv,
    c_ushort,
    c_uint,
    c_uint,
    c_uint,
) -> c_int;
pub type ChangeChnBookFn = unsafe extern "C" fn(*mut tasdevice_priv, c_ushort, c_int) -> c_int;
pub type DevReadFn = unsafe extern "C" fn(*mut tasdevice_priv, c_ushort, c_uint, *mut c_int) -> c_int;
pub type DevBulkReadFn =
    unsafe extern "C" fn(*mut tasdevice_priv, c_ushort, c_uint, *mut c_void, c_uint) -> c_int;

pub type c_ushort = u16;

#[repr(C)]
pub struct tasdevice_priv {
    pub dev: *mut device,
    pub client: *mut c_void,
    pub regmap: *mut regmap,
    pub ndev: c_int,
    pub tasdevice: *mut tasdevice,
    pub cur_prog: c_int,
    pub cur_conf: c_int,
    pub isspi: bool,
    pub update_bits: Option<UpdateBitsFn>,
    pub change_chn_book: Option<ChangeChnBookFn>,
    pub dev_read: Option<DevReadFn>,
    pub dev_bulk_read: Option<DevBulkReadFn>,
    pub codec_lock: mutex,
    pub reset: *mut gpio_desc,
    pub chip_id: c_uint,
    pub name_prefix: *const c_char,
    pub rca_binaryname: [c_char; 64],
    pub dev_name: *const c_char,
    pub crc8_lkp_tbl: *mut u8,
    pub codec: *mut c_void,
}

#[repr(C)]
pub struct snd_ctl_elem_value {
    pub value: snd_ctl_elem_value_value,
}

#[repr(C)]
pub union snd_ctl_elem_value_value {
    pub integer: snd_ctl_elem_value_integer,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_elem_value_integer {
    pub value: [c_long; 128],
}

pub type c_long = i64;

#[repr(C)]
pub struct soc_mixer_control {
    pub reg: c_uint,
    pub shift: c_uint,
    pub max: c_int,
    pub invert: c_uint,
}

unsafe extern "C" {
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn regmap_update_bits(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_regmap_init_i2c(i2c: *mut c_void, config: *const regmap_config) -> *mut regmap;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn mutex_init(lock: *mut mutex);
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn tasdevice_dev_read(
        tas_priv: *mut tasdevice_priv,
        chn: c_ushort,
        reg: c_uint,
        val: *mut c_int,
    ) -> c_int;
    fn tasdevice_dev_bulk_read(
        tas_priv: *mut tasdevice_priv,
        chn: c_ushort,
        reg: c_uint,
        data: *mut c_void,
        len: c_uint,
    ) -> c_int;
    fn tasdevice_dev_write(
        tas_priv: *mut tasdevice_priv,
        chn: c_int,
        reg: c_uint,
        value: c_uint,
    ) -> c_int;
    fn gpiod_set_value_cansleep(desc: *mut gpio_desc, value: c_int);
    fn usleep_range(min: c_ulong, max: c_ulong);
    fn scnprintf(buf: *mut c_char, size: usize, fmt: *const c_char, ...) -> c_int;
    fn crc8_populate_msb(table: *mut u8, polynomial: u8);
    fn request_firmware_nowait(
        module: *mut module,
        uevent: c_int,
        name: *const c_char,
        device: *mut device,
        gfp: c_uint,
        context: *mut c_void,
        cont: Option<unsafe extern "C" fn(*const firmware, *mut c_void)>,
    ) -> c_int;
    fn fls(x: c_int) -> c_int;
    fn TASDEVICE_BOOK_ID(reg: c_uint) -> c_int;
    fn TASDEVICE_PGRG(reg: c_uint) -> c_uint;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
}

static TASDEVICE_RANGES: [regmap_range_cfg; 1] = [regmap_range_cfg {
    range_min: 0,
    range_max: 256 * 128,
    selector_reg: TASDEVICE_PAGE_SELECT,
    selector_mask: 0xff,
    selector_shift: 0,
    window_start: 0,
    window_len: 128,
}];

static TASDEVICE_REGMAP: regmap_config = regmap_config {
    reg_bits: 8,
    val_bits: 8,
    cache_type: REGCACHE_NONE,
    ranges: TASDEVICE_RANGES.as_ptr(),
    num_ranges: TASDEVICE_RANGES.len() as c_uint,
    max_register: 256 * 128,
};

unsafe extern "C" fn tasdevice_change_chn_book(
    tas_priv: *mut tasdevice_priv,
    chn: c_ushort,
    book: c_int,
) -> c_int {
    let client = (*tas_priv).client as *mut i2c_client;
    let mut ret: c_int = 0;

    if (chn as c_int) < (*tas_priv).ndev {
        let tasdev = (*tas_priv).tasdevice.offset(chn as isize);
        let map = (*tas_priv).regmap;

        if (*client).addr != (*tasdev).dev_addr {
            (*client).addr = (*tasdev).dev_addr;
            /* All tas2781s share the same regmap, clear the page
             * inside regmap once switching to another tas2781.
             * Register 0 at any pages and any books inside tas2781
             * is the same one for page-switching.
             */
            ret = regmap_write(map, TASDEVICE_PAGE_SELECT, 0);
            if ret < 0 {
                dev_err(
                    (*tas_priv).dev,
                    c"%s, E=%d channel:%d\n".as_ptr(),
                    c"tasdevice_change_chn_book".as_ptr(),
                    ret,
                    chn as c_int,
                );
                return ret;
            }
        }

        if (*tasdev).cur_book != book {
            ret = regmap_write(map, TASDEVICE_BOOKCTL_REG, book as c_uint);
            if ret < 0 {
                dev_err(
                    (*tas_priv).dev,
                    c"%s, E=%d\n".as_ptr(),
                    c"tasdevice_change_chn_book".as_ptr(),
                    ret,
                );
                return ret;
            }
            (*tasdev).cur_book = book;
        }
    } else {
        ret = -EINVAL;
        dev_err(
            (*tas_priv).dev,
            c"%s, no such channel(%d)\n".as_ptr(),
            c"tasdevice_change_chn_book".as_ptr(),
            chn as c_int,
        );
    }

    ret
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn tasdev_chn_switch(
    tas_priv: *mut tasdevice_priv,
    chn: c_ushort,
) -> c_int {
    let client = (*tas_priv).client as *mut i2c_client;
    let tasdev = (*tas_priv).tasdevice.offset(chn as isize);
    let map = (*tas_priv).regmap;
    let ret: c_int;

    if (*client).addr != (*tasdev).dev_addr {
        (*client).addr = (*tasdev).dev_addr;
        /* All devices share the same regmap, clear the page
         * inside regmap once switching to another device.
         * Register 0 at any pages and any books inside tas2781
         * is the same one for page-switching.
         */
        ret = regmap_write(map, TASDEVICE_PAGE_SELECT, 0);
        if ret < 0 {
            dev_err(
                (*tas_priv).dev,
                c"%s, E=%d\n".as_ptr(),
                c"tasdev_chn_switch".as_ptr(),
                ret,
            );
            return ret;
        }
        return 1;
    }
    0
}
// EXPORT_SYMBOL_GPL(tasdev_chn_switch);

#[unsafe(no_mangle)]
pub unsafe extern "C" fn tasdevice_dev_update_bits(
    tas_priv: *mut tasdevice_priv,
    chn: c_ushort,
    reg: c_uint,
    mask: c_uint,
    value: c_uint,
) -> c_int {
    let mut ret: c_int = 0;

    if (chn as c_int) < (*tas_priv).ndev {
        let map = (*tas_priv).regmap;

        ret = ((*tas_priv).change_chn_book.unwrap())(tas_priv, chn, TASDEVICE_BOOK_ID(reg));
        if ret < 0 {
            return ret;
        }

        ret = regmap_update_bits(map, TASDEVICE_PGRG(reg), mask, value);
        if ret < 0 {
            dev_err(
                (*tas_priv).dev,
                c"%s, E=%d\n".as_ptr(),
                c"tasdevice_dev_update_bits".as_ptr(),
                ret,
            );
        }
    } else {
        dev_err(
            (*tas_priv).dev,
            c"%s, no such channel(%d)\n".as_ptr(),
            c"tasdevice_dev_update_bits".as_ptr(),
            chn as c_int,
        );
        ret = -EINVAL;
    }

    ret
}
// EXPORT_SYMBOL_GPL(tasdevice_dev_update_bits);

#[unsafe(no_mangle)]
pub unsafe extern "C" fn tasdevice_kzalloc(i2c: *mut i2c_client) -> *mut tasdevice_priv {
    let tas_priv: *mut tasdevice_priv;

    tas_priv = devm_kzalloc(
        &mut (*i2c).dev,
        core::mem::size_of::<tasdevice_priv>(),
        GFP_KERNEL,
    ) as *mut tasdevice_priv;
    if tas_priv.is_null() {
        return core::ptr::null_mut();
    }
    (*tas_priv).dev = &mut (*i2c).dev;
    (*tas_priv).client = i2c as *mut c_void;

    tas_priv
}
// EXPORT_SYMBOL_GPL(tasdevice_kzalloc);

#[unsafe(no_mangle)]
pub unsafe extern "C" fn tasdevice_init(tas_priv: *mut tasdevice_priv) -> c_int {
    let mut ret: c_int = 0;
    let mut i: c_int;

    (*tas_priv).regmap = devm_regmap_init_i2c((*tas_priv).client, &TASDEVICE_REGMAP);
    if IS_ERR((*tas_priv).regmap as *const c_void) {
        ret = PTR_ERR((*tas_priv).regmap as *const c_void);
        dev_err(
            (*tas_priv).dev,
            c"Failed to allocate register map: %d\n".as_ptr(),
            ret,
        );
        return ret;
    }

    (*tas_priv).cur_prog = -1;
    (*tas_priv).cur_conf = -1;
    (*tas_priv).isspi = false;

    i = 0;
    while i < (*tas_priv).ndev {
        (*(*tas_priv).tasdevice.offset(i as isize)).cur_book = -1;
        (*(*tas_priv).tasdevice.offset(i as isize)).cur_prog = -1;
        (*(*tas_priv).tasdevice.offset(i as isize)).cur_conf = -1;
        i += 1;
    }

    (*tas_priv).update_bits = Some(tasdevice_dev_update_bits);
    (*tas_priv).change_chn_book = Some(tasdevice_change_chn_book);
    (*tas_priv).dev_read = Some(tasdevice_dev_read);
    (*tas_priv).dev_bulk_read = Some(tasdevice_dev_bulk_read);

    mutex_init(&mut (*tas_priv).codec_lock);

    ret
}
// EXPORT_SYMBOL_GPL(tasdevice_init);

unsafe extern "C" fn tasdevice_clamp(mut val: c_int, max: c_int, invert: c_uint) -> c_int {
    if val > max {
        val = max;
    }
    if invert != 0 {
        val = max - val;
    }
    if val < 0 {
        val = 0;
    }
    val
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn tasdevice_amp_putvol(
    tas_priv: *mut tasdevice_priv,
    ucontrol: *mut snd_ctl_elem_value,
    mc: *mut soc_mixer_control,
) -> c_int {
    let invert = (*mc).invert;
    let mut mask: u8;
    let max = (*mc).max;
    let mut err_cnt: c_int = 0;
    let mut val: c_int;
    let mut i: c_int;
    let mut ret: c_int;

    mask = ((1 << fls(max)) - 1) as u8;
    mask = ((mask as c_uint) << (*mc).shift) as u8;
    val = tasdevice_clamp((*ucontrol).value.integer.value[0] as c_int, max, invert);
    i = 0;
    while i < (*tas_priv).ndev {
        ret = tasdevice_dev_update_bits(
            tas_priv,
            i as c_ushort,
            (*mc).reg,
            mask as c_uint,
            (val << (*mc).shift) as c_uint,
        );
        if ret == 0 {
            i += 1;
            continue;
        }
        err_cnt += 1;
        dev_err(
            (*tas_priv).dev,
            c"set AMP vol error in dev %d\n".as_ptr(),
            i,
        );
        i += 1;
    }

    /* All the devices set error, return 0 */
    if err_cnt == (*tas_priv).ndev { 0 } else { 1 }
}
// EXPORT_SYMBOL_GPL(tasdevice_amp_putvol);

#[unsafe(no_mangle)]
pub unsafe extern "C" fn tasdevice_amp_getvol(
    tas_priv: *mut tasdevice_priv,
    ucontrol: *mut snd_ctl_elem_value,
    mc: *mut soc_mixer_control,
) -> c_int {
    let invert = (*mc).invert;
    let mut mask: u8 = 0;
    let max = (*mc).max;
    let mut ret: c_int = 0;
    let mut val: c_int = 0;

    /* Read the primary device */
    ret = tasdevice_dev_read(tas_priv, 0, (*mc).reg, &mut val);
    if ret != 0 {
        dev_err(
            (*tas_priv).dev,
            c"%s, get AMP vol error\n".as_ptr(),
            c"tasdevice_amp_getvol".as_ptr(),
        );
        return ret;
    }

    mask = ((1 << fls(max)) - 1) as u8;
    mask = ((mask as c_uint) << (*mc).shift) as u8;
    val = ((val & mask as c_int) as c_uint >> (*mc).shift) as c_int;
    val = tasdevice_clamp(val, max, invert);
    (*ucontrol).value.integer.value[0] = val as c_long;

    ret
}
// EXPORT_SYMBOL_GPL(tasdevice_amp_getvol);

#[unsafe(no_mangle)]
pub unsafe extern "C" fn tasdevice_digital_getvol(
    tas_priv: *mut tasdevice_priv,
    ucontrol: *mut snd_ctl_elem_value,
    mc: *mut soc_mixer_control,
) -> c_int {
    let invert = (*mc).invert;
    let max = (*mc).max;
    let ret: c_int;
    let mut val: c_int = 0;

    /* Read the primary device as the whole */
    ret = tasdevice_dev_read(tas_priv, 0, (*mc).reg, &mut val);
    if ret != 0 {
        dev_err(
            (*tas_priv).dev,
            c"%s, get digital vol error\n".as_ptr(),
            c"tasdevice_digital_getvol".as_ptr(),
        );
        return ret;
    }

    val = tasdevice_clamp(val, max, invert);
    (*ucontrol).value.integer.value[0] = val as c_long;

    ret
}
// EXPORT_SYMBOL_GPL(tasdevice_digital_getvol);

#[unsafe(no_mangle)]
pub unsafe extern "C" fn tasdevice_digital_putvol(
    tas_priv: *mut tasdevice_priv,
    ucontrol: *mut snd_ctl_elem_value,
    mc: *mut soc_mixer_control,
) -> c_int {
    let invert = (*mc).invert;
    let max = (*mc).max;
    let mut err_cnt: c_int = 0;
    let mut ret: c_int;
    let mut val: c_int;
    let mut i: c_int;

    val = tasdevice_clamp((*ucontrol).value.integer.value[0] as c_int, max, invert);

    i = 0;
    while i < (*tas_priv).ndev {
        ret = tasdevice_dev_write(tas_priv, i, (*mc).reg, val as c_uint);
        if ret == 0 {
            i += 1;
            continue;
        }
        err_cnt += 1;
        dev_err(
            (*tas_priv).dev,
            c"set digital vol err in dev %d\n".as_ptr(),
            i,
        );
        i += 1;
    }

    /* All the devices set error, return 0 */
    if err_cnt == (*tas_priv).ndev { 0 } else { 1 }
}
// EXPORT_SYMBOL_GPL(tasdevice_digital_putvol);

#[unsafe(no_mangle)]
pub unsafe extern "C" fn tasdevice_reset(tas_dev: *mut tasdevice_priv) {
    let mut ret: c_int;
    let mut i: c_int;

    if !(*tas_dev).reset.is_null() {
        gpiod_set_value_cansleep((*tas_dev).reset, 0);
        usleep_range(500, 1000);
        gpiod_set_value_cansleep((*tas_dev).reset, 1);
    } else {
        i = 0;
        while i < (*tas_dev).ndev {
            ret = tasdevice_dev_write(
                tas_dev,
                i,
                TASDEVICE_REG_SWRESET,
                if (*tas_dev).chip_id >= TAS5802 {
                    TAS5825_REG_SWRESET_RESET
                } else {
                    TASDEVICE_REG_SWRESET_RESET
                },
            );
            if ret < 0 {
                dev_err(
                    (*tas_dev).dev,
                    c"dev %d swreset fail, %d\n".as_ptr(),
                    i,
                    ret,
                );
            }
            i += 1;
        }
    }
    usleep_range(1000, 1050);
}
// EXPORT_SYMBOL_GPL(tasdevice_reset);

#[unsafe(no_mangle)]
pub unsafe extern "C" fn tascodec_init(
    tas_priv: *mut tasdevice_priv,
    codec: *mut c_void,
    module: *mut module,
    cont: Option<unsafe extern "C" fn(*const firmware, *mut c_void)>,
) -> c_int {
    let mut ret: c_int = 0;

    /* Codec Lock Hold to ensure that codec_probe and firmware parsing and
     * loading do not simultaneously execute.
     */
    mutex_lock(&mut (*tas_priv).codec_lock);

    if !(*tas_priv).name_prefix.is_null() {
        scnprintf(
            (*tas_priv).rca_binaryname.as_mut_ptr(),
            64,
            c"%s-%sRCA%d.bin".as_ptr(),
            (*tas_priv).name_prefix,
            (*tas_priv).dev_name,
            (*tas_priv).ndev,
        );
    } else {
        scnprintf(
            (*tas_priv).rca_binaryname.as_mut_ptr(),
            64,
            c"%sRCA%d.bin".as_ptr(),
            (*tas_priv).dev_name,
            (*tas_priv).ndev,
        );
    }
    crc8_populate_msb((*tas_priv).crc8_lkp_tbl, TASDEVICE_CRC8_POLYNOMIAL);
    (*tas_priv).codec = codec;
    ret = request_firmware_nowait(
        module,
        FW_ACTION_UEVENT as c_int,
        (*tas_priv).rca_binaryname.as_ptr(),
        (*tas_priv).dev,
        GFP_KERNEL,
        tas_priv as *mut c_void,
        cont,
    );
    if ret != 0 {
        dev_err(
            (*tas_priv).dev,
            c"request_firmware_nowait err:0x%08x\n".as_ptr(),
            ret,
        );
    }

    mutex_unlock(&mut (*tas_priv).codec_lock);
    ret
}
// EXPORT_SYMBOL_GPL(tascodec_init);

// MODULE_DESCRIPTION("TAS2781 common library for I2C");
// MODULE_AUTHOR("Shenghao Ding, TI, <shenghao-ding@ti.com>");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
