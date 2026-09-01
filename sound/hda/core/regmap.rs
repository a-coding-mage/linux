// SPDX-License-Identifier: GPL-2.0-only
/*
 * Regmap support for HD-audio verbs
 *
 * A virtual register is translated to one or more hda verbs for write,
 * vice versa for read.
 *
 * A few limitations:
 * - Provided for not all verbs but only subset standard non-volatile verbs.
 * - For reading, only AC_VERB_GET_* variants can be used.
 * - For writing, mapped to the *corresponding* AC_VERB_SET_* variants,
 *   so can't handle asymmetric verbs for read and write
 */

/* Dependencies from:
 * linux/slab.h, linux/device.h, linux/regmap.h, linux/export.h, linux/pm.h,
 * sound/core.h, sound/hdaudio.h, sound/hda_regmap.h, and "local.h".
 */

unsafe fn codec_pm_lock(codec: *mut hdac_device) -> c_int {
    unsafe { snd_hdac_keep_power_up(codec) }
}

unsafe fn codec_pm_unlock(codec: *mut hdac_device, lock: c_int) {
    if lock == 1 {
        unsafe {
            snd_hdac_power_down_pm(codec);
        }
    }
}

macro_rules! get_verb {
    ($reg:expr) => {
        (($reg >> 8) & 0xfff)
    };
}

unsafe fn hda_volatile_reg(dev: *mut device, reg: c_uint) -> bool {
    let codec: *mut hdac_device = unsafe { dev_to_hdac_dev(dev) };
    let verb: c_uint = get_verb!(reg);

    match verb {
        AC_VERB_GET_PROC_COEF => unsafe { !(*codec).cache_coef },
        AC_VERB_GET_COEF_INDEX
        | AC_VERB_GET_PROC_STATE
        | AC_VERB_GET_POWER_STATE
        | AC_VERB_GET_PIN_SENSE
        | AC_VERB_GET_HDMI_DIP_SIZE
        | AC_VERB_GET_HDMI_ELDD
        | AC_VERB_GET_HDMI_DIP_INDEX
        | AC_VERB_GET_HDMI_DIP_DATA
        | AC_VERB_GET_HDMI_DIP_XMIT
        | AC_VERB_GET_HDMI_CP_CTRL
        | AC_VERB_GET_HDMI_CHAN_SLOT
        | AC_VERB_GET_DEVICE_SEL
        | AC_VERB_GET_DEVICE_LIST => true, /* read-only volatile */
        _ => false,
    }
}

unsafe fn hda_writeable_reg(dev: *mut device, reg: c_uint) -> bool {
    let codec: *mut hdac_device = unsafe { dev_to_hdac_dev(dev) };
    let verb: c_uint = get_verb!(reg);
    let mut v: *const c_uint;
    let mut i: c_int = 0;

    unsafe {
        while snd_array_for_each(&raw mut (*codec).vendor_verbs, &mut i, &mut v) {
            if verb == *v {
                return true;
            }
        }

        if (*codec).caps_overwriting {
            return true;
        }
    }

    match verb & 0xf00 {
        AC_VERB_GET_STREAM_FORMAT | AC_VERB_GET_AMP_GAIN_MUTE => return true,
        AC_VERB_GET_PROC_COEF => return unsafe { (*codec).cache_coef },
        0xf00 => {}
        _ => return false,
    }

    match verb {
        AC_VERB_GET_CONNECT_SEL
        | AC_VERB_GET_SDI_SELECT
        | AC_VERB_GET_PIN_WIDGET_CONTROL
        | AC_VERB_GET_UNSOLICITED_RESPONSE /* only as SET_UNSOLICITED_ENABLE */
        | AC_VERB_GET_BEEP_CONTROL
        | AC_VERB_GET_EAPD_BTLENABLE
        | AC_VERB_GET_DIGI_CONVERT_1
        | AC_VERB_GET_DIGI_CONVERT_2 /* only for beep control */
        | AC_VERB_GET_VOLUME_KNOB_CONTROL
        | AC_VERB_GET_GPIO_MASK
        | AC_VERB_GET_GPIO_DIRECTION
        | AC_VERB_GET_GPIO_DATA /* not for volatile read */
        | AC_VERB_GET_GPIO_WAKE_MASK
        | AC_VERB_GET_GPIO_UNSOLICITED_RSP_MASK
        | AC_VERB_GET_GPIO_STICKY_MASK => true,
        _ => false,
    }
}

unsafe fn hda_readable_reg(dev: *mut device, reg: c_uint) -> bool {
    let codec: *mut hdac_device = unsafe { dev_to_hdac_dev(dev) };
    let verb: c_uint = get_verb!(reg);

    if unsafe { (*codec).caps_overwriting } {
        return true;
    }

    match verb {
        AC_VERB_PARAMETERS | AC_VERB_GET_CONNECT_LIST | AC_VERB_GET_SUBSYSTEM_ID => return true,
        /* below are basically writable, but disabled for reducing unnecessary
         * writes at sync
         */
        AC_VERB_GET_CONFIG_DEFAULT /* usually just read */
        | AC_VERB_GET_CONV /* managed in PCM code */
        | AC_VERB_GET_CVT_CHAN_COUNT /* managed in HDMI CA code */ => return true,
        _ => {}
    }

    unsafe { hda_writeable_reg(dev, reg) }
}

/*
 * Stereo amp pseudo register:
 * for making easier to handle the stereo volume control, we provide a
 * fake register to deal both left and right channels by a single
 * (pseudo) register access.  A verb consisting of SET_AMP_GAIN with
 * *both* SET_LEFT and SET_RIGHT bits takes a 16bit value, the lower 8bit
 * for the left and the upper 8bit for the right channel.
 */
fn is_stereo_amp_verb(reg: c_uint) -> bool {
    if ((reg >> 8) & 0x700) != AC_VERB_SET_AMP_GAIN_MUTE {
        return false;
    }
    (reg & (AC_AMP_SET_LEFT | AC_AMP_SET_RIGHT)) == (AC_AMP_SET_LEFT | AC_AMP_SET_RIGHT)
}

/* read a pseudo stereo amp register (16bit left+right) */
unsafe fn hda_reg_read_stereo_amp(
    codec: *mut hdac_device,
    mut reg: c_uint,
    val: *mut c_uint,
) -> c_int {
    let mut left: c_uint = 0;
    let mut right: c_uint = 0;
    let mut err: c_int;

    reg &= !(AC_AMP_SET_LEFT | AC_AMP_SET_RIGHT);
    err = unsafe { snd_hdac_exec_verb(codec, reg | AC_AMP_GET_LEFT, 0, &mut left) };
    if err < 0 {
        return err;
    }
    err = unsafe { snd_hdac_exec_verb(codec, reg | AC_AMP_GET_RIGHT, 0, &mut right) };
    if err < 0 {
        return err;
    }
    unsafe {
        *val = left | (right << 8);
    }
    0
}

/* write a pseudo stereo amp register (16bit left+right) */
unsafe fn hda_reg_write_stereo_amp(
    codec: *mut hdac_device,
    mut reg: c_uint,
    val: c_uint,
) -> c_int {
    let mut err: c_int;
    let mut verb: c_uint;
    let left: c_uint;
    let right: c_uint;

    verb = AC_VERB_SET_AMP_GAIN_MUTE << 8;
    if (reg & AC_AMP_GET_OUTPUT) != 0 {
        verb |= AC_AMP_SET_OUTPUT;
    } else {
        verb |= AC_AMP_SET_INPUT | ((reg & 0xf) << 8);
    }
    reg = (reg & !0xfffff) | verb;

    left = val & 0xff;
    right = (val >> 8) & 0xff;
    if left == right {
        reg |= AC_AMP_SET_LEFT | AC_AMP_SET_RIGHT;
        return unsafe { snd_hdac_exec_verb(codec, reg | left, 0, NULL) };
    }

    err = unsafe { snd_hdac_exec_verb(codec, reg | AC_AMP_SET_LEFT | left, 0, NULL) };
    if err < 0 {
        return err;
    }
    err = unsafe { snd_hdac_exec_verb(codec, reg | AC_AMP_SET_RIGHT | right, 0, NULL) };
    if err < 0 {
        return err;
    }
    0
}

/* read a pseudo coef register (16bit) */
unsafe fn hda_reg_read_coef(codec: *mut hdac_device, reg: c_uint, val: *mut c_uint) -> c_int {
    let mut verb: c_uint;
    let mut err: c_int;

    if unsafe { !(*codec).cache_coef } {
        return -EINVAL;
    }
    /* LSB 8bit = coef index */
    verb = (reg & !0xfff00) | (AC_VERB_SET_COEF_INDEX << 8);
    err = unsafe { snd_hdac_exec_verb(codec, verb, 0, NULL) };
    if err < 0 {
        return err;
    }
    verb = (reg & !0xfffff) | (AC_VERB_GET_PROC_COEF << 8);
    unsafe { snd_hdac_exec_verb(codec, verb, 0, val) }
}

/* write a pseudo coef register (16bit) */
unsafe fn hda_reg_write_coef(codec: *mut hdac_device, reg: c_uint, val: c_uint) -> c_int {
    let mut verb: c_uint;
    let mut err: c_int;

    if unsafe { !(*codec).cache_coef } {
        return -EINVAL;
    }
    /* LSB 8bit = coef index */
    verb = (reg & !0xfff00) | (AC_VERB_SET_COEF_INDEX << 8);
    err = unsafe { snd_hdac_exec_verb(codec, verb, 0, NULL) };
    if err < 0 {
        return err;
    }
    verb = (reg & !0xfffff) | (AC_VERB_SET_PROC_COEF << 8) | (val & 0xffff);
    unsafe { snd_hdac_exec_verb(codec, verb, 0, NULL) }
}

unsafe fn hda_reg_read(context: *mut c_void, mut reg: c_uint, val: *mut c_uint) -> c_int {
    let codec: *mut hdac_device = context as *mut hdac_device;
    let verb: c_int = get_verb!(reg) as c_int;
    let mut err: c_int;
    let mut pm_lock: c_int = 0;

    if verb != AC_VERB_GET_POWER_STATE as c_int {
        pm_lock = unsafe { codec_pm_lock(codec) };
        if pm_lock < 0 {
            return -EAGAIN;
        }
    }
    reg |= unsafe { (*codec).addr << 28 };
    if is_stereo_amp_verb(reg) {
        err = unsafe { hda_reg_read_stereo_amp(codec, reg, val) };
        unsafe { codec_pm_unlock(codec, pm_lock) };
        return err;
    }
    if verb == AC_VERB_GET_PROC_COEF as c_int {
        err = unsafe { hda_reg_read_coef(codec, reg, val) };
        unsafe { codec_pm_unlock(codec, pm_lock) };
        return err;
    }
    if (verb & 0x700) == AC_VERB_SET_AMP_GAIN_MUTE as c_int {
        reg &= !AC_AMP_FAKE_MUTE;
    }

    err = unsafe { snd_hdac_exec_verb(codec, reg, 0, val) };
    if err >= 0 {
        /* special handling for asymmetric reads */
        if verb == AC_VERB_GET_POWER_STATE as c_int {
            unsafe {
                if (*val & AC_PWRST_ERROR) != 0 {
                    *val = (-1i32) as c_uint;
                } else {
                    /* take only the actual state */
                    *val = (*val >> 4) & 0x0f;
                }
            }
        }
    }
    unsafe { codec_pm_unlock(codec, pm_lock) };
    err
}

unsafe fn hda_reg_write(context: *mut c_void, mut reg: c_uint, mut val: c_uint) -> c_int {
    let codec: *mut hdac_device = context as *mut hdac_device;
    let mut verb: c_uint;
    let mut i: c_int;
    let bytes: c_int;
    let mut err: c_int = 0;
    let mut pm_lock: c_int = 0;

    if unsafe { (*codec).caps_overwriting } {
        return 0;
    }

    reg &= !0x00080000u32; /* drop GET bit */
    reg |= unsafe { (*codec).addr << 28 };
    verb = get_verb!(reg);

    if verb != AC_VERB_SET_POWER_STATE {
        pm_lock = unsafe { codec_pm_lock(codec) };
        if pm_lock < 0 {
            return unsafe { if (*codec).lazy_cache { 0 } else { -EAGAIN } };
        }
    }

    if is_stereo_amp_verb(reg) {
        err = unsafe { hda_reg_write_stereo_amp(codec, reg, val) };
        unsafe { codec_pm_unlock(codec, pm_lock) };
        return err;
    }

    if verb == AC_VERB_SET_PROC_COEF {
        err = unsafe { hda_reg_write_coef(codec, reg, val) };
        unsafe { codec_pm_unlock(codec, pm_lock) };
        return err;
    }

    match verb & 0xf00 {
        AC_VERB_SET_AMP_GAIN_MUTE => {
            if (reg & AC_AMP_FAKE_MUTE) != 0 && (val & AC_AMP_MUTE) != 0 {
                val = 0;
            }
            verb = AC_VERB_SET_AMP_GAIN_MUTE;
            if (reg & AC_AMP_GET_LEFT) != 0 {
                verb |= AC_AMP_SET_LEFT >> 8;
            } else {
                verb |= AC_AMP_SET_RIGHT >> 8;
            }
            if (reg & AC_AMP_GET_OUTPUT) != 0 {
                verb |= AC_AMP_SET_OUTPUT >> 8;
            } else {
                verb |= AC_AMP_SET_INPUT >> 8;
                verb |= reg & 0xf;
            }
        }
        _ => {}
    }

    match verb {
        AC_VERB_SET_DIGI_CONVERT_1 => bytes = 2,
        AC_VERB_SET_CONFIG_DEFAULT_BYTES_0 => bytes = 4,
        _ => bytes = 1,
    }

    i = 0;
    while i < bytes {
        reg &= !0xfffff;
        reg |= ((verb + i as c_uint) << 8) | ((val >> (8 * i)) & 0xff);
        err = unsafe { snd_hdac_exec_verb(codec, reg, 0, NULL) };
        if err < 0 {
            unsafe { codec_pm_unlock(codec, pm_lock) };
            return err;
        }
        i += 1;
    }

    unsafe { codec_pm_unlock(codec, pm_lock) };
    err
}

static hda_regmap_cfg: regmap_config = regmap_config {
    name: b"hdaudio\0".as_ptr() as *const c_char,
    reg_bits: 32,
    val_bits: 32,
    max_register: 0xfffffff,
    writeable_reg: Some(hda_writeable_reg),
    readable_reg: Some(hda_readable_reg),
    volatile_reg: Some(hda_volatile_reg),
    cache_type: REGCACHE_MAPLE,
    reg_read: Some(hda_reg_read),
    reg_write: Some(hda_reg_write),
    use_single_read: true,
    use_single_write: true,
    disable_locking: true,
};

/**
 * snd_hdac_regmap_init - Initialize regmap for HDA register accesses
 * @codec: the codec object
 *
 * Returns zero for success or a negative error code.
 */
pub unsafe extern "C" fn snd_hdac_regmap_init(codec: *mut hdac_device) -> c_int {
    let regmap: *mut regmap;

    regmap = unsafe {
        regmap_init(
            &raw mut (*codec).dev,
            NULL,
            codec as *mut c_void,
            &hda_regmap_cfg,
        )
    };
    if IS_ERR(regmap as *const c_void) {
        return PTR_ERR(regmap as *const c_void);
    }
    unsafe {
        (*codec).regmap = regmap;
        snd_array_init(&raw mut (*codec).vendor_verbs, core::mem::size_of::<c_uint>(), 8);
    }
    0
}
/* EXPORT_SYMBOL_GPL(snd_hdac_regmap_init); */

/**
 * snd_hdac_regmap_exit - Release the regmap from HDA codec
 * @codec: the codec object
 */
pub unsafe extern "C" fn snd_hdac_regmap_exit(codec: *mut hdac_device) {
    unsafe {
        if !(*codec).regmap.is_null() {
            regmap_exit((*codec).regmap);
            (*codec).regmap = NULL as *mut regmap;
            snd_array_free(&raw mut (*codec).vendor_verbs);
        }
    }
}
/* EXPORT_SYMBOL_GPL(snd_hdac_regmap_exit); */

/**
 * snd_hdac_regmap_add_vendor_verb - add a vendor-specific verb to regmap
 * @codec: the codec object
 * @verb: verb to allow accessing via regmap
 *
 * Returns zero for success or a negative error code.
 */
pub unsafe extern "C" fn snd_hdac_regmap_add_vendor_verb(
    codec: *mut hdac_device,
    verb: c_uint,
) -> c_int {
    let p: *mut c_uint = unsafe { snd_array_new(&raw mut (*codec).vendor_verbs) };

    if p.is_null() {
        return -ENOMEM;
    }
    unsafe {
        *p = verb | 0x800; /* set GET bit */
    }
    0
}
/* EXPORT_SYMBOL_GPL(snd_hdac_regmap_add_vendor_verb); */

/*
 * helper functions
 */

/* write a pseudo-register value (w/o power sequence) */
unsafe fn reg_raw_write(codec: *mut hdac_device, reg: c_uint, val: c_uint) -> c_int {
    let _guard = unsafe { guard_mutex(&raw mut (*codec).regmap_lock) };
    unsafe {
        if (*codec).regmap.is_null() {
            hda_reg_write(codec as *mut c_void, reg, val)
        } else {
            regmap_write((*codec).regmap, reg, val)
        }
    }
}

/* a helper macro to call @func_call; retry with power-up if failed */
unsafe fn call_raw_func<F>(codec: *mut hdac_device, mut func_call: F) -> c_int
where
    F: FnMut() -> c_int,
{
    let mut err: c_int = func_call();
    if err == -EAGAIN {
        err = unsafe { snd_hdac_power_up_pm(codec) };
        if err >= 0 {
            err = func_call();
        }
        unsafe {
            snd_hdac_power_down_pm(codec);
        }
    }
    err
}

/**
 * snd_hdac_regmap_write_raw - write a pseudo register with power mgmt
 * @codec: the codec object
 * @reg: pseudo register
 * @val: value to write
 *
 * Returns zero if successful or a negative error code.
 */
pub unsafe extern "C" fn snd_hdac_regmap_write_raw(
    codec: *mut hdac_device,
    reg: c_uint,
    val: c_uint,
) -> c_int {
    unsafe { call_raw_func(codec, || reg_raw_write(codec, reg, val)) }
}
/* EXPORT_SYMBOL_GPL(snd_hdac_regmap_write_raw); */

unsafe fn reg_raw_read(
    codec: *mut hdac_device,
    reg: c_uint,
    val: *mut c_uint,
    uncached: bool,
) -> c_int {
    let _guard = unsafe { guard_mutex(&raw mut (*codec).regmap_lock) };
    unsafe {
        if uncached || (*codec).regmap.is_null() {
            hda_reg_read(codec as *mut c_void, reg, val)
        } else {
            regmap_read((*codec).regmap, reg, val)
        }
    }
}

unsafe fn __snd_hdac_regmap_read_raw(
    codec: *mut hdac_device,
    reg: c_uint,
    val: *mut c_uint,
    uncached: bool,
) -> c_int {
    unsafe { call_raw_func(codec, || reg_raw_read(codec, reg, val, uncached)) }
}

/**
 * snd_hdac_regmap_read_raw - read a pseudo register with power mgmt
 * @codec: the codec object
 * @reg: pseudo register
 * @val: pointer to store the read value
 *
 * Returns zero if successful or a negative error code.
 */
pub unsafe extern "C" fn snd_hdac_regmap_read_raw(
    codec: *mut hdac_device,
    reg: c_uint,
    val: *mut c_uint,
) -> c_int {
    unsafe { __snd_hdac_regmap_read_raw(codec, reg, val, false) }
}
/* EXPORT_SYMBOL_GPL(snd_hdac_regmap_read_raw); */

/* Works like snd_hdac_regmap_read_raw(), but this doesn't read from the
 * cache but always via hda verbs.
 */
pub unsafe extern "C" fn snd_hdac_regmap_read_raw_uncached(
    codec: *mut hdac_device,
    reg: c_uint,
    val: *mut c_uint,
) -> c_int {
    unsafe { __snd_hdac_regmap_read_raw(codec, reg, val, true) }
}

unsafe fn reg_raw_update(
    codec: *mut hdac_device,
    reg: c_uint,
    mask: c_uint,
    mut val: c_uint,
) -> c_int {
    let mut orig: c_uint = 0;
    let mut change: bool = false;
    let mut err: c_int;

    let _guard = unsafe { guard_mutex(&raw mut (*codec).regmap_lock) };
    unsafe {
        if !(*codec).regmap.is_null() {
            err = regmap_update_bits_check((*codec).regmap, reg, mask, val, &mut change);
            if err == 0 {
                err = if change { 1 } else { 0 };
            }
        } else {
            err = hda_reg_read(codec as *mut c_void, reg, &mut orig);
            if err == 0 {
                val &= mask;
                val |= orig & !mask;
                if val != orig {
                    err = hda_reg_write(codec as *mut c_void, reg, val);
                    if err == 0 {
                        err = 1;
                    }
                }
            }
        }
    }
    err
}

/**
 * snd_hdac_regmap_update_raw - update a pseudo register with power mgmt
 * @codec: the codec object
 * @reg: pseudo register
 * @mask: bit mask to update
 * @val: value to update
 *
 * Returns zero if successful or a negative error code.
 */
pub unsafe extern "C" fn snd_hdac_regmap_update_raw(
    codec: *mut hdac_device,
    reg: c_uint,
    mask: c_uint,
    val: c_uint,
) -> c_int {
    unsafe { call_raw_func(codec, || reg_raw_update(codec, reg, mask, val)) }
}
/* EXPORT_SYMBOL_GPL(snd_hdac_regmap_update_raw); */

unsafe fn reg_raw_update_once(
    codec: *mut hdac_device,
    reg: c_uint,
    mask: c_uint,
    val: c_uint,
) -> c_int {
    unsafe {
        if (*codec).regmap.is_null() {
            return reg_raw_update(codec, reg, mask, val);
        }
    }

    let _guard = unsafe { guard_mutex(&raw mut (*codec).regmap_lock) };
    /* Discard any updates to already initialised registers. */
    unsafe {
        if !regcache_reg_cached((*codec).regmap, reg) {
            return regmap_update_bits((*codec).regmap, reg, mask, val);
        }
    }
    0
}

/**
 * snd_hdac_regmap_update_raw_once - initialize the register value only once
 * @codec: the codec object
 * @reg: pseudo register
 * @mask: bit mask to update
 * @val: value to update
 *
 * Performs the update of the register bits only once when the register
 * hasn't been initialized yet.  Used in HD-audio legacy driver.
 * Returns zero if successful or a negative error code
 */
pub unsafe extern "C" fn snd_hdac_regmap_update_raw_once(
    codec: *mut hdac_device,
    reg: c_uint,
    mask: c_uint,
    val: c_uint,
) -> c_int {
    unsafe { call_raw_func(codec, || reg_raw_update_once(codec, reg, mask, val)) }
}
/* EXPORT_SYMBOL_GPL(snd_hdac_regmap_update_raw_once); */

/**
 * snd_hdac_regmap_sync - sync out the cached values for PM resume
 * @codec: the codec object
 */
pub unsafe extern "C" fn snd_hdac_regmap_sync(codec: *mut hdac_device) {
    let _guard = unsafe { guard_mutex(&raw mut (*codec).regmap_lock) };
    unsafe {
        if !(*codec).regmap.is_null() {
            regcache_sync((*codec).regmap);
        }
    }
}
/* EXPORT_SYMBOL_GPL(snd_hdac_regmap_sync); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
