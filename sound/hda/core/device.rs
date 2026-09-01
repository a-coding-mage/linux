// SPDX-License-Identifier: GPL-2.0-only
/*
 * HD-audio codec core device
 *
 * Rust source-level translation of hda/core/device.c.
 * C include dependencies are represented as external declarations below.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_ushort, c_void};
use core::mem::size_of;
use core::ptr;

type hda_nid_t = c_uint;
type size_t = usize;
type u16 = u16;
type u32 = u32;
type u64 = u64;
type snd_pcm_format_t = c_int;
type snd_pcm_subformat_t = c_int;

#[repr(C)]
pub struct device {
    pub parent: *mut device,
    pub bus: *const bus_type,
    pub release: Option<unsafe extern "C" fn(*mut device)>,
    pub groups: *const *const attribute_group,
}

#[repr(C)]
pub struct hdac_bus {
    pub dev: *mut device,
}

#[repr(C)]
pub struct hdac_device {
    pub dev: device,
    pub bus: *mut hdac_bus,
    pub addr: c_uint,
    pub type_: c_uint,
    pub widget_lock: mutex,
    pub regmap_lock: mutex,
    pub in_pm: atomic_t,
    pub vendor_id: c_uint,
    pub subsystem_id: c_uint,
    pub revision_id: c_uint,
    pub afg: hda_nid_t,
    pub mfg: hda_nid_t,
    pub afg_function_id: c_uint,
    pub mfg_function_id: c_uint,
    pub afg_unsol: c_uint,
    pub mfg_unsol: c_uint,
    pub power_caps: c_uint,
    pub vendor_name: *mut c_char,
    pub chip_name: *mut c_char,
    pub regmap: *mut c_void,
    pub caps_overwriting: bool,
    pub num_nodes: c_int,
    pub start_nid: hda_nid_t,
    pub end_nid: hda_nid_t,
    pub exec_verb: Option<
        unsafe extern "C" fn(*mut hdac_device, c_uint, c_uint, *mut c_uint) -> c_int,
    >,
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct atomic_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bus_type {
    _private: [u8; 0],
}

#[repr(C)]
pub struct attribute_group {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}

unsafe extern "C" {
    static snd_hda_bus_type: bus_type;
    static hdac_dev_attr_groups: *const *const attribute_group;
    static mut jiffies: c_ulong;

    fn dev_to_hdac_dev(dev: *mut device) -> *mut hdac_device;
    fn snd_hdac_device_exit(codec: *mut hdac_device);
    fn device_initialize(dev: *mut device);
    fn dev_set_name(dev: *mut device, fmt: *const c_char, ...);
    fn device_enable_async_suspend(dev: *mut device);
    fn mutex_init(lock: *mut mutex);
    fn pm_runtime_set_active(dev: *mut device);
    fn pm_runtime_get_noresume(dev: *mut device);
    fn atomic_set(v: *mut atomic_t, i: c_int);
    fn snd_hdac_bus_add_device(bus: *mut hdac_bus, codec: *mut hdac_device) -> c_int;
    fn snd_hdac_read_parm(codec: *mut hdac_device, nid: hda_nid_t, parm: c_int) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn put_device(dev: *mut device);
    fn pm_runtime_put_noidle(dev: *mut device);
    fn pm_runtime_set_suspended(dev: *mut device);
    fn snd_hdac_bus_remove_device(bus: *mut hdac_bus, codec: *mut hdac_device);
    fn kfree(p: *const c_void);
    fn device_add(dev: *mut device) -> c_int;
    fn hda_widget_sysfs_init(codec: *mut hdac_device) -> c_int;
    fn device_del(dev: *mut device);
    fn device_is_registered(dev: *mut device) -> bool;
    fn hda_widget_sysfs_exit(codec: *mut hdac_device);
    fn kstrdup(s: *const c_char, flags: c_uint) -> *mut c_char;
    fn kasprintf(flags: c_uint, fmt: *const c_char, ...) -> *mut c_char;
    fn scnprintf(buf: *mut c_char, size: size_t, fmt: *const c_char, ...) -> c_int;
    fn snd_hdac_bus_exec_verb(
        bus: *mut hdac_bus,
        addr: c_uint,
        cmd: c_uint,
        res: *mut c_uint,
    ) -> c_int;
    fn snd_hdac_regmap_encode_verb(nid: hda_nid_t, verb: c_uint) -> c_uint;
    fn snd_hdac_regmap_read_raw(
        codec: *mut hdac_device,
        cmd: c_uint,
        res: *mut c_uint,
    ) -> c_int;
    fn snd_hdac_regmap_read_raw_uncached(
        codec: *mut hdac_device,
        cmd: c_uint,
        res: *mut c_uint,
    ) -> c_int;
    fn snd_hdac_regmap_write_raw(codec: *mut hdac_device, verb: c_uint, val: c_uint) -> c_int;
    fn snd_hdac_get_wcaps(codec: *mut hdac_device, nid: hda_nid_t) -> c_uint;
    fn snd_hdac_get_wcaps_type(wcaps: c_uint) -> c_uint;
    fn hda_widget_sysfs_reinit(
        codec: *mut hdac_device,
        start_nid: hda_nid_t,
        nums: c_int,
    ) -> c_int;
    fn pm_runtime_get_sync(dev: *mut device) -> c_int;
    fn pm_runtime_put_autosuspend(dev: *mut device) -> c_int;
    fn atomic_inc_not_zero(v: *mut atomic_t) -> c_int;
    fn pm_runtime_get_if_active(dev: *mut device) -> c_int;
    fn atomic_dec_if_positive(v: *mut atomic_t) -> c_int;
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn params_set_format(params: *mut snd_pcm_hw_params, format: snd_pcm_format_t);
    fn hw_param_mask(params: *mut snd_pcm_hw_params, var: c_int) -> *mut c_void;
    fn snd_mask_set(mask: *mut c_void, val: snd_pcm_subformat_t);
    fn snd_pcm_hw_params_bits(params: *const snd_pcm_hw_params) -> c_uint;
    fn msecs_to_jiffies(m: c_uint) -> c_ulong;
    fn time_after_eq(a: c_ulong, b: c_ulong) -> bool;
    fn msleep(msecs: c_uint);
}

const GFP_KERNEL: c_uint = 0;
const HDA_DEV_CORE: c_uint = 0;
const ENODEV: c_int = 19;
const ENOMEM: c_int = 12;
const EINVAL: c_int = 22;
const ENOSPC: c_int = 28;
const EIO: c_int = 5;

const AC_NODE_ROOT: hda_nid_t = 0;
const AC_PAR_VENDOR_ID: c_int = 0;
const AC_PAR_SUBSYSTEM_ID: c_int = 0;
const AC_PAR_REV_ID: c_int = 0;
const AC_PAR_POWER_STATE: c_int = 0;
const AC_VERB_GET_SUBSYSTEM_ID: c_uint = 0;
const AC_VERB_PARAMETERS: c_uint = 0;
const AC_PAR_NODE_COUNT: c_int = 0;
const AC_PAR_FUNCTION_TYPE: c_int = 0;
const AC_GRP_AUDIO_FUNCTION: c_int = 1;
const AC_GRP_MODEM_FUNCTION: c_int = 2;
const AC_WCAP_CONN_LIST: c_uint = 0;
const AC_WID_VOL_KNB: c_uint = 0;
const AC_PAR_CONNLIST_LEN: c_int = 0;
const AC_CLIST_LONG: c_uint = 0;
const AC_CLIST_LENGTH: c_uint = 0;
const AC_VERB_GET_CONNECT_LIST: c_uint = 0;
const AC_FMT_BASE_48K: c_uint = 0;
const AC_FMT_BASE_44K: c_uint = 0;
const AC_FMT_MULT_SHIFT: c_uint = 0;
const AC_FMT_DIV_SHIFT: c_uint = 0;
const SNDRV_PCM_RATE_8000: c_uint = 0;
const SNDRV_PCM_RATE_11025: c_uint = 0;
const SNDRV_PCM_RATE_16000: c_uint = 0;
const SNDRV_PCM_RATE_22050: c_uint = 0;
const SNDRV_PCM_RATE_32000: c_uint = 0;
const SNDRV_PCM_RATE_44100: c_uint = 0;
const SNDRV_PCM_RATE_48000: c_uint = 0;
const SNDRV_PCM_RATE_88200: c_uint = 0;
const SNDRV_PCM_RATE_96000: c_uint = 0;
const SNDRV_PCM_RATE_176400: c_uint = 0;
const SNDRV_PCM_RATE_192000: c_uint = 0;
const SNDRV_PCM_RATE_KNOT: c_uint = 0;
const SNDRV_PCM_FORMAT_S20_LE: snd_pcm_format_t = 0;
const SNDRV_PCM_FORMAT_S24_LE: snd_pcm_format_t = 0;
const SNDRV_PCM_FORMAT_S32_LE: snd_pcm_format_t = 0;
const SNDRV_PCM_FORMAT_U20_LE: snd_pcm_format_t = 0;
const SNDRV_PCM_FORMAT_U24_LE: snd_pcm_format_t = 0;
const SNDRV_PCM_FORMAT_U32_LE: snd_pcm_format_t = 0;
const SNDRV_PCM_FORMAT_S20_BE: snd_pcm_format_t = 0;
const SNDRV_PCM_FORMAT_S24_BE: snd_pcm_format_t = 0;
const SNDRV_PCM_FORMAT_S32_BE: snd_pcm_format_t = 0;
const SNDRV_PCM_FORMAT_U20_BE: snd_pcm_format_t = 0;
const SNDRV_PCM_FORMAT_U24_BE: snd_pcm_format_t = 0;
const SNDRV_PCM_FORMAT_U32_BE: snd_pcm_format_t = 0;
const SNDRV_PCM_HW_PARAM_SUBFORMAT: c_int = 0;
const AC_FMT_BITS_8: c_uint = 0;
const AC_FMT_BITS_16: c_uint = 0;
const AC_FMT_BITS_20: c_uint = 0;
const AC_FMT_BITS_24: c_uint = 0;
const AC_FMT_BITS_32: c_uint = 0;
const AC_DIG1_NONAUDIO: c_ushort = 0;
const AC_FMT_TYPE_NON_PCM: c_uint = 0;
const AC_WCAP_FORMAT_OVRD: c_uint = 0;
const AC_PAR_PCM: c_int = 0;
const AC_PAR_STREAM: c_int = 0;
const AC_SUPFMT_PCM: c_uint = 0;
const AC_SUPPCM_BITS_8: c_uint = 0;
const AC_SUPPCM_BITS_16: c_uint = 0;
const AC_SUPPCM_BITS_20: c_uint = 0;
const AC_SUPPCM_BITS_24: c_uint = 0;
const AC_SUPPCM_BITS_32: c_uint = 0;
const AC_WCAP_DIGITAL: c_uint = 0;
const SNDRV_PCM_FMTBIT_U8: u64 = 0;
const SNDRV_PCM_FMTBIT_S16_LE: u64 = 0;
const SNDRV_PCM_FMTBIT_S32_LE: u64 = 0;
const SNDRV_PCM_SUBFMTBIT_MSBITS_20: u32 = 0;
const SNDRV_PCM_SUBFMTBIT_MSBITS_24: u32 = 0;
const SNDRV_PCM_SUBFMTBIT_MSBITS_MAX: u32 = 0;
const SNDRV_PCM_FMTBIT_IEC958_SUBFRAME_LE: u64 = 0;
const AC_SUPFMT_AC3: c_uint = 0;
const AC_PAR_PCM_RATE_BITS: c_uint = 11;
const AC_VERB_GET_POWER_STATE: c_uint = 0;
const AC_PWRST_ERROR: c_uint = 0;

const fn HDA_RATE(base: c_uint, mult: c_uint, div: c_uint) -> c_uint {
    base | ((mult - 1) << AC_FMT_MULT_SHIFT) | ((div - 1) << AC_FMT_DIV_SHIFT)
}

unsafe extern "C" fn default_release(dev: *mut device) {
    unsafe {
        snd_hdac_device_exit(dev_to_hdac_dev(dev));
    }
}

/**
 * snd_hdac_device_init - initialize the HD-audio codec base device
 * @codec: device to initialize
 * @bus: but to attach
 * @name: device name string
 * @addr: codec address
 *
 * Returns zero for success or a negative error code.
 *
 * This function increments the runtime PM counter and marks it active.
 * The caller needs to turn it off appropriately later.
 *
 * The caller needs to set the device's release op properly by itself.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_hdac_device_init(
    codec: *mut hdac_device,
    bus: *mut hdac_bus,
    name: *const c_char,
    addr: c_uint,
) -> c_int {
    unsafe {
        let dev = &mut (*codec).dev as *mut device;
        let mut fg: hda_nid_t;
        let mut err: c_int;

        device_initialize(dev);
        (*dev).parent = (*bus).dev;
        (*dev).bus = &snd_hda_bus_type;
        (*dev).release = Some(default_release);
        (*dev).groups = hdac_dev_attr_groups;
        dev_set_name(dev, c"%s".as_ptr(), name);
        device_enable_async_suspend(dev);

        (*codec).bus = bus;
        (*codec).addr = addr;
        (*codec).type_ = HDA_DEV_CORE;
        mutex_init(&mut (*codec).widget_lock);
        mutex_init(&mut (*codec).regmap_lock);
        pm_runtime_set_active(&mut (*codec).dev);
        pm_runtime_get_noresume(&mut (*codec).dev);
        atomic_set(&mut (*codec).in_pm, 0);

        err = snd_hdac_bus_add_device(bus, codec);
        if err < 0 {
            put_device(&mut (*codec).dev);
            return err;
        }

        /* fill parameters */
        (*codec).vendor_id =
            snd_hdac_read_parm(codec, AC_NODE_ROOT, AC_PAR_VENDOR_ID) as c_uint;
        if (*codec).vendor_id == -1i32 as c_uint {
            /* read again, hopefully the access method was corrected
             * in the last read...
             */
            (*codec).vendor_id =
                snd_hdac_read_parm(codec, AC_NODE_ROOT, AC_PAR_VENDOR_ID) as c_uint;
        }

        (*codec).subsystem_id =
            snd_hdac_read_parm(codec, AC_NODE_ROOT, AC_PAR_SUBSYSTEM_ID) as c_uint;
        (*codec).revision_id = snd_hdac_read_parm(codec, AC_NODE_ROOT, AC_PAR_REV_ID) as c_uint;

        setup_fg_nodes(codec);
        if (*codec).afg == 0 && (*codec).mfg == 0 {
            dev_err(dev, c"no AFG or MFG node found\n".as_ptr());
            err = -ENODEV;
            put_device(&mut (*codec).dev);
            return err;
        }

        fg = if (*codec).afg != 0 { (*codec).afg } else { (*codec).mfg };

        err = snd_hdac_refresh_widgets(codec);
        if err < 0 {
            put_device(&mut (*codec).dev);
            return err;
        }

        (*codec).power_caps = snd_hdac_read_parm(codec, fg, AC_PAR_POWER_STATE) as c_uint;
        /* reread ssid if not set by parameter */
        if (*codec).subsystem_id == -1i32 as c_uint || (*codec).subsystem_id == 0 {
            snd_hdac_read(
                codec,
                fg,
                AC_VERB_GET_SUBSYSTEM_ID,
                0,
                &mut (*codec).subsystem_id,
            );
        }

        err = get_codec_vendor_name(codec);
        if err < 0 {
            put_device(&mut (*codec).dev);
            return err;
        }

        (*codec).chip_name =
            kasprintf(GFP_KERNEL, c"ID %x".as_ptr(), (*codec).vendor_id & 0xffff);
        if (*codec).chip_name.is_null() {
            err = -ENOMEM;
            put_device(&mut (*codec).dev);
            return err;
        }

        0
    }
}

/**
 * snd_hdac_device_exit - clean up the HD-audio codec base device
 * @codec: device to clean up
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_hdac_device_exit(codec: *mut hdac_device) {
    unsafe {
        pm_runtime_put_noidle(&mut (*codec).dev);
        /* keep balance of runtime PM child_count in parent device */
        pm_runtime_set_suspended(&mut (*codec).dev);
        snd_hdac_bus_remove_device((*codec).bus, codec);
        kfree((*codec).vendor_name as *const c_void);
        kfree((*codec).chip_name as *const c_void);
    }
}

/**
 * snd_hdac_device_register - register the hd-audio codec base device
 * @codec: the device to register
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_hdac_device_register(codec: *mut hdac_device) -> c_int {
    unsafe {
        let mut err: c_int;

        err = device_add(&mut (*codec).dev);
        if err < 0 {
            return err;
        }
        /* scoped_guard(mutex, &codec->widget_lock) */
        err = hda_widget_sysfs_init(codec);
        if err < 0 {
            device_del(&mut (*codec).dev);
            return err;
        }

        0
    }
}

/**
 * snd_hdac_device_unregister - unregister the hd-audio codec base device
 * @codec: the device to unregister
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_hdac_device_unregister(codec: *mut hdac_device) {
    unsafe {
        if device_is_registered(&mut (*codec).dev) {
            /* scoped_guard(mutex, &codec->widget_lock) */
            hda_widget_sysfs_exit(codec);
            device_del(&mut (*codec).dev);
            snd_hdac_bus_remove_device((*codec).bus, codec);
        }
    }
}

/**
 * snd_hdac_device_set_chip_name - set/update the codec name
 * @codec: the HDAC device
 * @name: name string to set
 *
 * Returns 0 if the name is set or updated, or a negative error code.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_hdac_device_set_chip_name(
    codec: *mut hdac_device,
    name: *const c_char,
) -> c_int {
    unsafe {
        let newname: *mut c_char;

        if name.is_null() {
            return 0;
        }
        newname = kstrdup(name, GFP_KERNEL);
        if newname.is_null() {
            return -ENOMEM;
        }
        kfree((*codec).chip_name as *const c_void);
        (*codec).chip_name = newname;
        0
    }
}

/**
 * snd_hdac_codec_modalias - give the module alias name
 * @codec: HDAC device
 * @buf: string buffer to store
 * @size: string buffer size
 *
 * Returns the size of string, like snprintf(), or a negative error code.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_hdac_codec_modalias(
    codec: *const hdac_device,
    buf: *mut c_char,
    size: size_t,
) -> c_int {
    unsafe {
        scnprintf(
            buf,
            size,
            c"hdaudio:v%08Xr%08Xa%02X\n".as_ptr(),
            (*codec).vendor_id,
            (*codec).revision_id,
            (*codec).type_,
        )
    }
}

/**
 * snd_hdac_make_cmd - compose a 32bit command word to be sent to the
 *	HD-audio controller
 * @codec: the codec object
 * @nid: NID to encode
 * @verb: verb to encode
 * @parm: parameter to encode
 *
 * Return an encoded command verb or -1 for error.
 */
unsafe fn snd_hdac_make_cmd(
    codec: *mut hdac_device,
    nid: hda_nid_t,
    verb: c_uint,
    parm: c_uint,
) -> c_uint {
    unsafe {
        let mut val: u32;
        let addr: u32;

        addr = (*codec).addr;
        if (addr & !0xf) != 0 || (nid & !0x7f) != 0 || (verb & !0xfff) != 0 || (parm & !0xffff) != 0
        {
            dev_err(
                &mut (*codec).dev,
                c"out of range cmd %x:%x:%x:%x\n".as_ptr(),
                addr,
                nid,
                verb,
                parm,
            );
            return -1i32 as c_uint;
        }

        val = addr << 28;
        val |= (nid as u32) << 20;
        val |= verb << 8;
        val |= parm;
        val
    }
}

/**
 * snd_hdac_exec_verb - execute an encoded verb
 * @codec: the codec object
 * @cmd: encoded verb to execute
 * @flags: optional flags, pass zero for default
 * @res: the pointer to store the result, NULL if running async
 *
 * Returns zero if successful, or a negative error code.
 *
 * This calls the exec_verb op when set in hdac_codec.  If not,
 * call the default snd_hdac_bus_exec_verb().
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_hdac_exec_verb(
    codec: *mut hdac_device,
    cmd: c_uint,
    flags: c_uint,
    res: *mut c_uint,
) -> c_int {
    unsafe {
        if let Some(exec_verb) = (*codec).exec_verb {
            return exec_verb(codec, cmd, flags, res);
        }
        snd_hdac_bus_exec_verb((*codec).bus, (*codec).addr, cmd, res)
    }
}

/**
 * snd_hdac_read - execute a verb
 * @codec: the codec object
 * @nid: NID to execute a verb
 * @verb: verb to execute
 * @parm: parameter for a verb
 * @res: the pointer to store the result, NULL if running async
 *
 * Returns zero if successful, or a negative error code.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_hdac_read(
    codec: *mut hdac_device,
    nid: hda_nid_t,
    verb: c_uint,
    parm: c_uint,
    res: *mut c_uint,
) -> c_int {
    unsafe {
        let cmd: c_uint = snd_hdac_make_cmd(codec, nid, verb, parm);

        snd_hdac_exec_verb(codec, cmd, 0, res)
    }
}

/**
 * _snd_hdac_read_parm - read a parmeter
 * @codec: the codec object
 * @nid: NID to read a parameter
 * @parm: parameter to read
 * @res: pointer to store the read value
 *
 * This function returns zero or an error unlike snd_hdac_read_parm().
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn _snd_hdac_read_parm(
    codec: *mut hdac_device,
    nid: hda_nid_t,
    parm: c_int,
    res: *mut c_uint,
) -> c_int {
    unsafe {
        let cmd: c_uint;

        cmd = snd_hdac_regmap_encode_verb(nid, AC_VERB_PARAMETERS) | parm as c_uint;
        snd_hdac_regmap_read_raw(codec, cmd, res)
    }
}

/**
 * snd_hdac_read_parm_uncached - read a codec parameter without caching
 * @codec: the codec object
 * @nid: NID to read a parameter
 * @parm: parameter to read
 *
 * Returns -1 for error.  If you need to distinguish the error more
 * strictly, use snd_hdac_read() directly.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_hdac_read_parm_uncached(
    codec: *mut hdac_device,
    nid: hda_nid_t,
    parm: c_int,
) -> c_int {
    unsafe {
        let cmd: c_uint;
        let mut val: c_uint = 0;

        cmd = snd_hdac_regmap_encode_verb(nid, AC_VERB_PARAMETERS) | parm as c_uint;
        if snd_hdac_regmap_read_raw_uncached(codec, cmd, &mut val) < 0 {
            return -1;
        }
        val as c_int
    }
}

/**
 * snd_hdac_override_parm - override read-only parameters
 * @codec: the codec object
 * @nid: NID for the parameter
 * @parm: the parameter to change
 * @val: the parameter value to overwrite
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_hdac_override_parm(
    codec: *mut hdac_device,
    nid: hda_nid_t,
    parm: c_uint,
    val: c_uint,
) -> c_int {
    unsafe {
        let verb: c_uint = (AC_VERB_PARAMETERS << 8) | (nid << 20) | parm;
        let err: c_int;

        if (*codec).regmap.is_null() {
            return -EINVAL;
        }

        (*codec).caps_overwriting = true;
        err = snd_hdac_regmap_write_raw(codec, verb, val);
        (*codec).caps_overwriting = false;
        err
    }
}

/**
 * snd_hdac_get_sub_nodes - get start NID and number of subtree nodes
 * @codec: the codec object
 * @nid: NID to inspect
 * @start_id: the pointer to store the starting NID
 *
 * Returns the number of subtree nodes or zero if not found.
 * This function reads parameters always without caching.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_hdac_get_sub_nodes(
    codec: *mut hdac_device,
    nid: hda_nid_t,
    start_id: *mut hda_nid_t,
) -> c_int {
    unsafe {
        let parm: c_uint;

        parm = snd_hdac_read_parm_uncached(codec, nid, AC_PAR_NODE_COUNT) as c_uint;
        if parm == -1i32 as c_uint {
            *start_id = 0;
            return 0;
        }
        *start_id = (parm >> 16) & 0x7fff;
        (parm & 0x7fff) as c_int
    }
}

/*
 * look for an AFG and MFG nodes
 */
unsafe fn setup_fg_nodes(codec: *mut hdac_device) {
    unsafe {
        let mut i: c_int;
        let total_nodes: c_int;
        let function_id: c_int;
        let mut nid: hda_nid_t = 0;

        total_nodes = snd_hdac_get_sub_nodes(codec, AC_NODE_ROOT, &mut nid);
        i = 0;
        while i < total_nodes {
            let function_id = snd_hdac_read_parm(codec, nid, AC_PAR_FUNCTION_TYPE);
            match function_id & 0xff {
                AC_GRP_AUDIO_FUNCTION => {
                    (*codec).afg = nid;
                    (*codec).afg_function_id = (function_id & 0xff) as c_uint;
                    (*codec).afg_unsol = ((function_id >> 8) & 1) as c_uint;
                }
                AC_GRP_MODEM_FUNCTION => {
                    (*codec).mfg = nid;
                    (*codec).mfg_function_id = (function_id & 0xff) as c_uint;
                    (*codec).mfg_unsol = ((function_id >> 8) & 1) as c_uint;
                }
                _ => {}
            }
            i += 1;
            nid += 1;
        }
    }
}

/**
 * snd_hdac_refresh_widgets - Reset the widget start/end nodes
 * @codec: the codec object
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_hdac_refresh_widgets(codec: *mut hdac_device) -> c_int {
    unsafe {
        let mut start_nid: hda_nid_t = 0;
        let nums: c_int;
        let err: c_int;

        /*
         * Serialize against multiple threads trying to update the sysfs
         * widgets array.
         */
        /* guard(mutex)(&codec->widget_lock); */
        nums = snd_hdac_get_sub_nodes(codec, (*codec).afg, &mut start_nid);
        if start_nid == 0 || nums <= 0 || nums >= 0xff {
            dev_err(
                &mut (*codec).dev,
                c"cannot read sub nodes for FG 0x%02x\n".as_ptr(),
                (*codec).afg,
            );
            return -EINVAL;
        }

        err = hda_widget_sysfs_reinit(codec, start_nid, nums);
        if err < 0 {
            return err;
        }

        (*codec).num_nodes = nums;
        (*codec).start_nid = start_nid;
        (*codec).end_nid = start_nid + nums as c_uint;
        0
    }
}

/* return CONNLIST_LEN parameter of the given widget */
unsafe fn get_num_conns(codec: *mut hdac_device, nid: hda_nid_t) -> c_uint {
    unsafe {
        let wcaps: c_uint = snd_hdac_get_wcaps(codec, nid);
        let mut parm: c_uint;

        if (wcaps & AC_WCAP_CONN_LIST) == 0 && snd_hdac_get_wcaps_type(wcaps) != AC_WID_VOL_KNB {
            return 0;
        }

        parm = snd_hdac_read_parm(codec, nid, AC_PAR_CONNLIST_LEN) as c_uint;
        if parm == -1i32 as c_uint {
            parm = 0;
        }
        parm
    }
}

/**
 * snd_hdac_get_connections - get a widget connection list
 * @codec: the codec object
 * @nid: NID
 * @conn_list: the array to store the results, can be NULL
 * @max_conns: the max size of the given array
 *
 * Returns the number of connected widgets, zero for no connection, or a
 * negative error code.  When the number of elements don't fit with the
 * given array size, it returns -ENOSPC.
 *
 * When @conn_list is NULL, it just checks the number of connections.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_hdac_get_connections(
    codec: *mut hdac_device,
    nid: hda_nid_t,
    conn_list: *mut hda_nid_t,
    max_conns: c_int,
) -> c_int {
    unsafe {
        let mut parm: c_uint;
        let mut i: c_int;
        let conn_len: c_int;
        let mut conns: c_int;
        let mut err: c_int;
        let shift: c_uint;
        let num_elems: c_uint;
        let mask: c_uint;
        let mut prev_nid: hda_nid_t;
        let mut null_count: c_int = 0;

        parm = get_num_conns(codec, nid);
        if parm == 0 {
            return 0;
        }

        if (parm & AC_CLIST_LONG) != 0 {
            /* long form */
            shift = 16;
            num_elems = 2;
        } else {
            /* short form */
            shift = 8;
            num_elems = 4;
        }
        conn_len = (parm & AC_CLIST_LENGTH) as c_int;
        mask = (1 << (shift - 1)) - 1;

        if conn_len == 0 {
            return 0; /* no connection */
        }

        if conn_len == 1 {
            /* single connection */
            err = snd_hdac_read(codec, nid, AC_VERB_GET_CONNECT_LIST, 0, &mut parm);
            if err < 0 {
                return err;
            }
            if !conn_list.is_null() {
                *conn_list.add(0) = parm & mask;
            }
            return 1;
        }

        /* multi connection */
        conns = 0;
        prev_nid = 0;
        i = 0;
        while i < conn_len {
            let range_val: c_int;
            let val: hda_nid_t;
            let mut n: hda_nid_t;

            if (i as c_uint) % num_elems == 0 {
                err = snd_hdac_read(codec, nid, AC_VERB_GET_CONNECT_LIST, i as c_uint, &mut parm);
                if err < 0 {
                    return -EIO;
                }
            }
            range_val = if (parm & (1 << (shift - 1))) != 0 { 1 } else { 0 };
            val = parm & mask;
            if val == 0 {
                null_count += 1;
                if null_count > 1 {
                    /* no second chance */
                    dev_dbg(
                        &mut (*codec).dev,
                        c"invalid CONNECT_LIST verb %x[%i]:%x\n".as_ptr(),
                        nid,
                        i,
                        parm,
                    );
                    return 0;
                }
            }
            parm >>= shift;
            if range_val != 0 {
                /* ranges between the previous and this one */
                if prev_nid == 0 || prev_nid >= val {
                    dev_warn(
                        &mut (*codec).dev,
                        c"invalid dep_range_val %x:%x\n".as_ptr(),
                        prev_nid,
                        val,
                    );
                    i += 1;
                    prev_nid = val;
                    continue;
                }
                n = prev_nid + 1;
                while n <= val {
                    if !conn_list.is_null() {
                        if conns >= max_conns {
                            return -ENOSPC;
                        }
                        *conn_list.add(conns as usize) = n;
                    }
                    conns += 1;
                    n += 1;
                }
            } else {
                if !conn_list.is_null() {
                    if conns >= max_conns {
                        return -ENOSPC;
                    }
                    *conn_list.add(conns as usize) = val;
                }
                conns += 1;
            }
            prev_nid = val;
            i += 1;
        }
        conns
    }
}

/* CONFIG_PM */
/**
 * snd_hdac_power_up - power up the codec
 * @codec: the codec object
 *
 * This function calls the runtime PM helper to power up the given codec.
 * Unlike snd_hdac_power_up_pm(), you should call this only for the code
 * path that isn't included in PM path.  Otherwise it gets stuck.
 *
 * Returns zero if successful, or a negative error code.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_hdac_power_up(codec: *mut hdac_device) -> c_int {
    unsafe { pm_runtime_get_sync(&mut (*codec).dev) }
}

/**
 * snd_hdac_power_down - power down the codec
 * @codec: the codec object
 *
 * Returns zero if successful, or a negative error code.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_hdac_power_down(codec: *mut hdac_device) -> c_int {
    unsafe {
        let dev: *mut device = &mut (*codec).dev;

        pm_runtime_put_autosuspend(dev)
    }
}

/**
 * snd_hdac_power_up_pm - power up the codec
 * @codec: the codec object
 *
 * This function can be called in a recursive code path like init code
 * which may be called by PM suspend/resume again.  OTOH, if a power-up
 * call must wake up the sleeper (e.g. in a kctl callback), use
 * snd_hdac_power_up() instead.
 *
 * Returns zero if successful, or a negative error code.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_hdac_power_up_pm(codec: *mut hdac_device) -> c_int {
    unsafe {
        if atomic_inc_not_zero(&mut (*codec).in_pm) == 0 {
            return snd_hdac_power_up(codec);
        }
        0
    }
}

/* like snd_hdac_power_up_pm(), but only increment the pm count when
 * already powered up.  Returns -1 if not powered up, 1 if incremented
 * or 0 if unchanged.  Only used in hdac_regmap.c
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_hdac_keep_power_up(codec: *mut hdac_device) -> c_int {
    unsafe {
        if atomic_inc_not_zero(&mut (*codec).in_pm) == 0 {
            let ret: c_int = pm_runtime_get_if_active(&mut (*codec).dev);
            if ret == 0 {
                return -1;
            }
            if ret < 0 {
                return 0;
            }
        }
        1
    }
}

/**
 * snd_hdac_power_down_pm - power down the codec
 * @codec: the codec object
 *
 * Like snd_hdac_power_up_pm(), this function is used in a recursive
 * code path like init code which may be called by PM suspend/resume again.
 *
 * Returns zero if successful, or a negative error code.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_hdac_power_down_pm(codec: *mut hdac_device) -> c_int {
    unsafe {
        if atomic_dec_if_positive(&mut (*codec).in_pm) < 0 {
            return snd_hdac_power_down(codec);
        }
        0
    }
}

/* codec vendor labels */
#[repr(C)]
struct hda_vendor_id {
    id: c_uint,
    name: *const c_char,
}

static hda_vendor_ids: [hda_vendor_id; 26] = [
    hda_vendor_id { id: 0x0014, name: c"Loongson".as_ptr() },
    hda_vendor_id { id: 0x1002, name: c"ATI".as_ptr() },
    hda_vendor_id { id: 0x1013, name: c"Cirrus Logic".as_ptr() },
    hda_vendor_id { id: 0x1057, name: c"Motorola".as_ptr() },
    hda_vendor_id { id: 0x1095, name: c"Silicon Image".as_ptr() },
    hda_vendor_id { id: 0x10de, name: c"Nvidia".as_ptr() },
    hda_vendor_id { id: 0x10ec, name: c"Realtek".as_ptr() },
    hda_vendor_id { id: 0x1102, name: c"Creative".as_ptr() },
    hda_vendor_id { id: 0x1106, name: c"VIA".as_ptr() },
    hda_vendor_id { id: 0x111d, name: c"IDT".as_ptr() },
    hda_vendor_id { id: 0x11c1, name: c"LSI".as_ptr() },
    hda_vendor_id { id: 0x11d4, name: c"Analog Devices".as_ptr() },
    hda_vendor_id { id: 0x13f6, name: c"C-Media".as_ptr() },
    hda_vendor_id { id: 0x14f1, name: c"Conexant".as_ptr() },
    hda_vendor_id { id: 0x17e8, name: c"Chrontel".as_ptr() },
    hda_vendor_id { id: 0x1854, name: c"LG".as_ptr() },
    hda_vendor_id { id: 0x19e5, name: c"Huawei".as_ptr() },
    hda_vendor_id { id: 0x1aec, name: c"Wolfson Microelectronics".as_ptr() },
    hda_vendor_id { id: 0x1af4, name: c"QEMU".as_ptr() },
    hda_vendor_id { id: 0x1fa8, name: c"Senarytech".as_ptr() },
    hda_vendor_id { id: 0x434d, name: c"C-Media".as_ptr() },
    hda_vendor_id { id: 0x4c54, name: c"Lisuan".as_ptr() },
    hda_vendor_id { id: 0x8086, name: c"Intel".as_ptr() },
    hda_vendor_id { id: 0x8384, name: c"SigmaTel".as_ptr() },
    hda_vendor_id { id: 0, name: ptr::null() }, /* terminator */
    hda_vendor_id { id: 0, name: ptr::null() },
];

/* store the codec vendor name */
unsafe fn get_codec_vendor_name(codec: *mut hdac_device) -> c_int {
    unsafe {
        let mut c: *const hda_vendor_id = hda_vendor_ids.as_ptr();
        let vendor_id: u16 = ((*codec).vendor_id >> 16) as u16;

        while (*c).id != 0 {
            if (*c).id == vendor_id as c_uint {
                (*codec).vendor_name = kstrdup((*c).name, GFP_KERNEL);
                return if !(*codec).vendor_name.is_null() { 0 } else { -ENOMEM };
            }
            c = c.add(1);
        }

        (*codec).vendor_name = kasprintf(GFP_KERNEL, c"Generic %04x".as_ptr(), vendor_id as c_uint);
        if !(*codec).vendor_name.is_null() { 0 } else { -ENOMEM }
    }
}

/*
 * stream formats
 */
#[repr(C)]
struct hda_rate_tbl {
    hz: c_uint,
    alsa_bits: c_uint,
    hda_fmt: c_uint,
}

static rate_bits: [hda_rate_tbl; 13] = [
    /* rate in Hz, ALSA rate bitmask, HDA format value */

    /* autodetected value used in snd_hda_query_supported_pcm */
    hda_rate_tbl { hz: 8000, alsa_bits: SNDRV_PCM_RATE_8000, hda_fmt: HDA_RATE(AC_FMT_BASE_48K, 1, 6) },
    hda_rate_tbl { hz: 11025, alsa_bits: SNDRV_PCM_RATE_11025, hda_fmt: HDA_RATE(AC_FMT_BASE_44K, 1, 4) },
    hda_rate_tbl { hz: 16000, alsa_bits: SNDRV_PCM_RATE_16000, hda_fmt: HDA_RATE(AC_FMT_BASE_48K, 1, 3) },
    hda_rate_tbl { hz: 22050, alsa_bits: SNDRV_PCM_RATE_22050, hda_fmt: HDA_RATE(AC_FMT_BASE_44K, 1, 2) },
    hda_rate_tbl { hz: 32000, alsa_bits: SNDRV_PCM_RATE_32000, hda_fmt: HDA_RATE(AC_FMT_BASE_48K, 2, 3) },
    hda_rate_tbl { hz: 44100, alsa_bits: SNDRV_PCM_RATE_44100, hda_fmt: HDA_RATE(AC_FMT_BASE_44K, 1, 1) },
    hda_rate_tbl { hz: 48000, alsa_bits: SNDRV_PCM_RATE_48000, hda_fmt: HDA_RATE(AC_FMT_BASE_48K, 1, 1) },
    hda_rate_tbl { hz: 88200, alsa_bits: SNDRV_PCM_RATE_88200, hda_fmt: HDA_RATE(AC_FMT_BASE_44K, 2, 1) },
    hda_rate_tbl { hz: 96000, alsa_bits: SNDRV_PCM_RATE_96000, hda_fmt: HDA_RATE(AC_FMT_BASE_48K, 2, 1) },
    hda_rate_tbl { hz: 176400, alsa_bits: SNDRV_PCM_RATE_176400, hda_fmt: HDA_RATE(AC_FMT_BASE_44K, 4, 1) },
    hda_rate_tbl { hz: 192000, alsa_bits: SNDRV_PCM_RATE_192000, hda_fmt: HDA_RATE(AC_FMT_BASE_48K, 4, 1) },
    /* up to bits 10, 384kHZ isn't supported properly */

    /* not autodetected value */
    hda_rate_tbl { hz: 9600, alsa_bits: SNDRV_PCM_RATE_KNOT, hda_fmt: HDA_RATE(AC_FMT_BASE_48K, 1, 5) },

    hda_rate_tbl { hz: 0, alsa_bits: 0, hda_fmt: 0 }, /* terminator */
];

fn snd_hdac_format_normalize(format: snd_pcm_format_t) -> snd_pcm_format_t {
    match format {
        SNDRV_PCM_FORMAT_S20_LE | SNDRV_PCM_FORMAT_S24_LE => SNDRV_PCM_FORMAT_S32_LE,
        SNDRV_PCM_FORMAT_U20_LE | SNDRV_PCM_FORMAT_U24_LE => SNDRV_PCM_FORMAT_U32_LE,
        SNDRV_PCM_FORMAT_S20_BE | SNDRV_PCM_FORMAT_S24_BE => SNDRV_PCM_FORMAT_S32_BE,
        SNDRV_PCM_FORMAT_U20_BE | SNDRV_PCM_FORMAT_U24_BE => SNDRV_PCM_FORMAT_U32_BE,
        _ => format,
    }
}

/**
 * snd_hdac_stream_format_bits - obtain bits per sample value.
 * @format:	the PCM format.
 * @subformat:	the PCM subformat.
 * @maxbits:	the maximum bits per sample.
 *
 * Return: The number of bits per sample.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_hdac_stream_format_bits(
    format: snd_pcm_format_t,
    subformat: snd_pcm_subformat_t,
    maxbits: c_uint,
) -> c_uint {
    unsafe {
        let mut params: snd_pcm_hw_params = core::mem::zeroed();
        let bits: c_uint;

        memset(
            &mut params as *mut snd_pcm_hw_params as *mut c_void,
            0,
            size_of::<snd_pcm_hw_params>(),
        );

        params_set_format(&mut params, snd_hdac_format_normalize(format));
        snd_mask_set(hw_param_mask(&mut params, SNDRV_PCM_HW_PARAM_SUBFORMAT), subformat);

        bits = snd_pcm_hw_params_bits(&params);
        if maxbits != 0 {
            return if bits < maxbits { bits } else { maxbits };
        }
        bits
    }
}

/**
 * snd_hdac_stream_format - convert format parameters to SDxFMT value.
 * @channels:	the number of channels.
 * @bits:	bits per sample.
 * @rate:	the sample rate.
 *
 * Return: The format bitset or zero if invalid.
 */
#[unsafe(no_mangle)]
pub extern "C" fn snd_hdac_stream_format(channels: c_uint, bits: c_uint, rate: c_uint) -> c_uint {
    let mut val: c_uint = 0;
    let mut i: usize = 0;

    while rate_bits[i].hz != 0 {
        if rate_bits[i].hz == rate {
            val = rate_bits[i].hda_fmt;
            break;
        }
        i += 1;
    }

    if rate_bits[i].hz == 0 {
        return 0;
    }

    if channels == 0 || channels > 16 {
        return 0;
    }
    val |= channels - 1;

    match bits {
        8 => val |= AC_FMT_BITS_8,
        16 => val |= AC_FMT_BITS_16,
        20 => val |= AC_FMT_BITS_20,
        24 => val |= AC_FMT_BITS_24,
        32 => val |= AC_FMT_BITS_32,
        _ => return 0,
    }

    val
}

/**
 * snd_hdac_spdif_stream_format - convert format parameters to SDxFMT value.
 * @channels:	the number of channels.
 * @bits:	bits per sample.
 * @rate:	the sample rate.
 * @spdif_ctls:	HD-audio SPDIF status bits (0 if irrelevant).
 *
 * Return: The format bitset or zero if invalid.
 */
#[unsafe(no_mangle)]
pub extern "C" fn snd_hdac_spdif_stream_format(
    channels: c_uint,
    bits: c_uint,
    rate: c_uint,
    spdif_ctls: c_ushort,
) -> c_uint {
    let mut val: c_uint = snd_hdac_stream_format(channels, bits, rate);

    if val != 0 && (spdif_ctls & AC_DIG1_NONAUDIO) != 0 {
        val |= AC_FMT_TYPE_NON_PCM;
    }

    val
}

unsafe fn query_pcm_param(codec: *mut hdac_device, nid: hda_nid_t) -> c_uint {
    unsafe {
        let mut val: c_uint = 0;

        if nid != (*codec).afg && (snd_hdac_get_wcaps(codec, nid) & AC_WCAP_FORMAT_OVRD) != 0 {
            val = snd_hdac_read_parm(codec, nid, AC_PAR_PCM) as c_uint;
        }
        if val == 0 || val == -1i32 as c_uint {
            val = snd_hdac_read_parm(codec, (*codec).afg, AC_PAR_PCM) as c_uint;
        }
        if val == 0 || val == -1i32 as c_uint {
            return 0;
        }
        val
    }
}

unsafe fn query_stream_param(codec: *mut hdac_device, nid: hda_nid_t) -> c_uint {
    unsafe {
        let mut streams: c_uint = snd_hdac_read_parm(codec, nid, AC_PAR_STREAM) as c_uint;

        if streams == 0 || streams == -1i32 as c_uint {
            streams = snd_hdac_read_parm(codec, (*codec).afg, AC_PAR_STREAM) as c_uint;
        }
        if streams == 0 || streams == -1i32 as c_uint {
            return 0;
        }
        streams
    }
}

/**
 * snd_hdac_query_supported_pcm - query the supported PCM rates and formats
 * @codec: the codec object
 * @nid: NID to query
 * @ratesp: the pointer to store the detected rate bitflags
 * @formatsp: the pointer to store the detected formats
 * @subformatsp: the pointer to store the detected subformats for S32_LE format
 * @bpsp: the pointer to store the detected format widths
 *
 * Queries the supported PCM rates and formats.  The NULL @ratesp, @formatsp,
 * @subformatsp or @bpsp argument is ignored.
 *
 * Returns 0 if successful, otherwise a negative error code.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_hdac_query_supported_pcm(
    codec: *mut hdac_device,
    nid: hda_nid_t,
    ratesp: *mut u32,
    formatsp: *mut u64,
    subformatsp: *mut u32,
    bpsp: *mut c_uint,
) -> c_int {
    unsafe {
        let mut i: c_uint;
        let val: c_uint;
        let wcaps: c_uint;

        wcaps = snd_hdac_get_wcaps(codec, nid);
        val = query_pcm_param(codec, nid);

        if !ratesp.is_null() {
            let mut rates: u32 = 0;
            i = 0;
            while i < AC_PAR_PCM_RATE_BITS {
                if (val & (1 << i)) != 0 {
                    rates |= rate_bits[i as usize].alsa_bits;
                }
                i += 1;
            }
            if rates == 0 {
                dev_err(
                    &mut (*codec).dev,
                    c"rates == 0 (nid=0x%x, val=0x%x, ovrd=%i)\n".as_ptr(),
                    nid,
                    val,
                    if (wcaps & AC_WCAP_FORMAT_OVRD) != 0 { 1 } else { 0 },
                );
                return -EIO;
            }
            *ratesp = rates;
        }

        if !formatsp.is_null() || !subformatsp.is_null() || !bpsp.is_null() {
            let streams: c_uint;
            let mut bps: c_uint;
            let mut subformats: u32 = 0;
            let mut formats: u64 = 0;

            streams = query_stream_param(codec, nid);
            if streams == 0 {
                return -EIO;
            }

            bps = 0;
            if (streams & AC_SUPFMT_PCM) != 0 {
                if (val & AC_SUPPCM_BITS_8) != 0 {
                    formats |= SNDRV_PCM_FMTBIT_U8;
                    bps = 8;
                }
                if (val & AC_SUPPCM_BITS_16) != 0 {
                    formats |= SNDRV_PCM_FMTBIT_S16_LE;
                    bps = 16;
                }
                if (val & AC_SUPPCM_BITS_20) != 0 {
                    formats |= SNDRV_PCM_FMTBIT_S32_LE;
                    subformats |= SNDRV_PCM_SUBFMTBIT_MSBITS_20;
                    bps = 20;
                }
                if (val & AC_SUPPCM_BITS_24) != 0 {
                    formats |= SNDRV_PCM_FMTBIT_S32_LE;
                    subformats |= SNDRV_PCM_SUBFMTBIT_MSBITS_24;
                    bps = 24;
                }
                if (val & AC_SUPPCM_BITS_32) != 0 {
                    if (wcaps & AC_WCAP_DIGITAL) != 0 {
                        formats |= SNDRV_PCM_FMTBIT_IEC958_SUBFRAME_LE;
                    } else {
                        formats |= SNDRV_PCM_FMTBIT_S32_LE;
                        subformats |= SNDRV_PCM_SUBFMTBIT_MSBITS_MAX;
                        bps = 32;
                    }
                }
            }
            /* #if 0: FIXME: CS4206 doesn't work, which is the only codec supporting float
             * if (streams & AC_SUPFMT_FLOAT32) { ... }
             */
            if streams == AC_SUPFMT_AC3 {
                /* should be exclusive */
                /* temporary hack: we have still no proper support
                 * for the direct AC3 stream...
                 */
                formats |= SNDRV_PCM_FMTBIT_U8;
                bps = 8;
            }
            if formats == 0 {
                dev_err(
                    &mut (*codec).dev,
                    c"formats == 0 (nid=0x%x, val=0x%x, ovrd=%i, streams=0x%x)\n".as_ptr(),
                    nid,
                    val,
                    if (wcaps & AC_WCAP_FORMAT_OVRD) != 0 { 1 } else { 0 },
                    streams,
                );
                return -EIO;
            }
            if !formatsp.is_null() {
                *formatsp = formats;
            }
            if !subformatsp.is_null() {
                *subformatsp = subformats;
            }
            if !bpsp.is_null() {
                *bpsp = bps;
            }
        }

        0
    }
}

/**
 * snd_hdac_is_supported_format - Check the validity of the format
 * @codec: the codec object
 * @nid: NID to check
 * @format: the HD-audio format value to check
 *
 * Check whether the given node supports the format value.
 *
 * Returns true if supported, false if not.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_hdac_is_supported_format(
    codec: *mut hdac_device,
    nid: hda_nid_t,
    format: c_uint,
) -> bool {
    unsafe {
        let mut i: c_int;
        let val: c_uint;
        let rate: c_uint;
        let stream: c_uint;

        val = query_pcm_param(codec, nid);
        if val == 0 {
            return false;
        }

        rate = format & 0xff00;
        i = 0;
        while i < AC_PAR_PCM_RATE_BITS as c_int {
            if rate_bits[i as usize].hda_fmt == rate {
                if (val & (1 << i)) != 0 {
                    break;
                }
                return false;
            }
            i += 1;
        }
        if i >= AC_PAR_PCM_RATE_BITS as c_int {
            return false;
        }

        stream = query_stream_param(codec, nid);
        if stream == 0 {
            return false;
        }

        if (stream & AC_SUPFMT_PCM) != 0 {
            match format & 0xf0 {
                0x00 => {
                    if (val & AC_SUPPCM_BITS_8) == 0 {
                        return false;
                    }
                }
                0x10 => {
                    if (val & AC_SUPPCM_BITS_16) == 0 {
                        return false;
                    }
                }
                0x20 => {
                    if (val & AC_SUPPCM_BITS_20) == 0 {
                        return false;
                    }
                }
                0x30 => {
                    if (val & AC_SUPPCM_BITS_24) == 0 {
                        return false;
                    }
                }
                0x40 => {
                    if (val & AC_SUPPCM_BITS_32) == 0 {
                        return false;
                    }
                }
                _ => return false,
            }
        } else {
            /* FIXME: check for float32 and AC3? */
        }

        true
    }
}

unsafe fn codec_read(
    hdac: *mut hdac_device,
    nid: hda_nid_t,
    flags: c_int,
    verb: c_uint,
    parm: c_uint,
) -> c_uint {
    unsafe {
        let cmd: c_uint = snd_hdac_make_cmd(hdac, nid, verb, parm);
        let mut res: c_uint = 0;

        if snd_hdac_exec_verb(hdac, cmd, flags as c_uint, &mut res) != 0 {
            return -1i32 as c_uint;
        }

        res
    }
}

unsafe fn codec_write(
    hdac: *mut hdac_device,
    nid: hda_nid_t,
    flags: c_int,
    verb: c_uint,
    parm: c_uint,
) -> c_int {
    unsafe {
        let cmd: c_uint = snd_hdac_make_cmd(hdac, nid, verb, parm);

        snd_hdac_exec_verb(hdac, cmd, flags as c_uint, ptr::null_mut())
    }
}

/**
 * snd_hdac_codec_read - send a command and get the response
 * @hdac: the HDAC device
 * @nid: NID to send the command
 * @flags: optional bit flags
 * @verb: the verb to send
 * @parm: the parameter for the verb
 *
 * Send a single command and read the corresponding response.
 *
 * Returns the obtained response value, or -1 for an error.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_hdac_codec_read(
    hdac: *mut hdac_device,
    nid: hda_nid_t,
    flags: c_int,
    verb: c_uint,
    parm: c_uint,
) -> c_int {
    unsafe { codec_read(hdac, nid, flags, verb, parm) as c_int }
}

/**
 * snd_hdac_codec_write - send a single command without waiting for response
 * @hdac: the HDAC device
 * @nid: NID to send the command
 * @flags: optional bit flags
 * @verb: the verb to send
 * @parm: the parameter for the verb
 *
 * Send a single command without waiting for response.
 *
 * Returns 0 if successful, or a negative error code.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_hdac_codec_write(
    hdac: *mut hdac_device,
    nid: hda_nid_t,
    flags: c_int,
    verb: c_uint,
    parm: c_uint,
) -> c_int {
    unsafe { codec_write(hdac, nid, flags, verb, parm) }
}

/**
 * snd_hdac_check_power_state - check whether the actual power state matches
 * with the target state
 *
 * @hdac: the HDAC device
 * @nid: NID to send the command
 * @target_state: target state to check for
 *
 * Return true if state matches, false if not
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_hdac_check_power_state(
    hdac: *mut hdac_device,
    nid: hda_nid_t,
    target_state: c_uint,
) -> bool {
    unsafe {
        let mut state: c_uint = codec_read(hdac, nid, 0, AC_VERB_GET_POWER_STATE, 0);

        if (state & AC_PWRST_ERROR) != 0 {
            return true;
        }
        state = (state >> 4) & 0x0f;
        state == target_state
    }
}

/**
 * snd_hdac_sync_power_state - wait until actual power state matches
 * with the target state
 *
 * @codec: the HDAC device
 * @nid: NID to send the command
 * @power_state: target power state to wait for
 *
 * Return power state or PS_ERROR if codec rejects GET verb.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_hdac_sync_power_state(
    codec: *mut hdac_device,
    nid: hda_nid_t,
    power_state: c_uint,
) -> c_uint {
    unsafe {
        let end_time: c_ulong = jiffies + msecs_to_jiffies(500);
        let mut state: c_uint = 0;
        let mut actual_state: c_uint;
        let mut count: c_uint;

        count = 0;
        while count < 500 {
            state = snd_hdac_codec_read(codec, nid, 0, AC_VERB_GET_POWER_STATE, 0) as c_uint;
            if (state & AC_PWRST_ERROR) != 0 {
                msleep(20);
                break;
            }
            actual_state = (state >> 4) & 0x0f;
            if actual_state == power_state {
                break;
            }
            if time_after_eq(jiffies, end_time) {
                break;
            }
            /* wait until the codec reachs to the target state */
            msleep(1);
            count += 1;
        }
        state
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
