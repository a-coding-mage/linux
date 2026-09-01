// SPDX-License-Identifier: GPL-2.0
// hdac_component.c - routines for sync between HD-A core and DRM driver

// Rust translation of implementation originally depending on:
// <linux/init.h>, <linux/module.h>, <linux/pci.h>, <linux/component.h>,
// <linux/string_choices.h>, <sound/core.h>, <sound/hdaudio.h>,
// <sound/hda_component.h>, <sound/hda_register.h>

use core::ffi::{c_char, c_int, c_ulong, c_void};
use core::ptr;

pub type bool_ = bool;
pub type size_t = usize;
pub type hda_nid_t = c_uint;
pub type c_uint = u32;

pub const ENODEV: c_int = 19;
pub const EINVAL: c_int = 22;
pub const EBUSY: c_int = 16;
pub const ENOMEM: c_int = 12;
pub const GFP_KERNEL: c_uint = 0;
pub const HDA_CODEC_IDX_CONTROLLER: c_uint = !0;

#[repr(C)]
pub struct device {
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
pub struct completion {
    _private: [u8; 0],
}

#[repr(C)]
pub struct component_match {
    _private: [u8; 0],
}

#[repr(C)]
pub struct hdac_bus {
    pub dev: *mut device,
    pub lock: mutex,
    pub audio_component: *mut drm_audio_component,
    pub display_power_status: c_ulong,
    pub display_power_active: c_ulong,
}

#[repr(C)]
pub struct hdac_device {
    pub bus: *mut hdac_bus,
}

#[repr(C)]
pub struct drm_audio_component {
    pub dev: *mut device,
    pub ops: *const drm_audio_component_ops,
    pub audio_ops: *const drm_audio_component_audio_ops,
    pub master_bind_complete: completion,
}

#[repr(C)]
pub struct drm_audio_component_ops {
    pub owner: *mut module,
    pub get_power: Option<unsafe extern "C" fn(*mut device) -> c_ulong>,
    pub put_power: Option<unsafe extern "C" fn(*mut device, c_ulong)>,
    pub codec_wake_override: Option<unsafe extern "C" fn(*mut device, bool_)>,
    pub sync_audio_rate: Option<unsafe extern "C" fn(*mut device, c_int, c_int, c_int) -> c_int>,
    pub get_eld: Option<
        unsafe extern "C" fn(
            *mut device,
            c_int,
            c_int,
            *mut bool_,
            *mut c_char,
            c_int,
        ) -> c_int,
    >,
}

#[repr(C)]
pub struct drm_audio_component_audio_ops {
    pub pin2port: Option<unsafe extern "C" fn(*mut hdac_device, hda_nid_t) -> c_int>,
    pub master_bind: Option<unsafe extern "C" fn(*mut device, *mut drm_audio_component) -> c_int>,
    pub master_unbind: Option<unsafe extern "C" fn(*mut device, *mut drm_audio_component)>,
}

#[repr(C)]
pub struct component_master_ops {
    pub bind: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    pub unbind: Option<unsafe extern "C" fn(*mut device)>,
}

unsafe extern "C" {
    fn devres_find(
        dev: *mut device,
        release: unsafe extern "C" fn(*mut device, *mut c_void),
        match_: *mut c_void,
        match_data: *mut c_void,
    ) -> *mut c_void;
    fn devres_alloc(
        release: unsafe extern "C" fn(*mut device, *mut c_void),
        size: size_t,
        gfp: c_uint,
    ) -> *mut c_void;
    fn devres_add(dev: *mut device, res: *mut c_void);
    fn devres_destroy(
        dev: *mut device,
        release: unsafe extern "C" fn(*mut device, *mut c_void),
        match_: *mut c_void,
        match_data: *mut c_void,
    ) -> c_int;
    fn component_bind_all(dev: *mut device, data: *mut c_void) -> c_int;
    fn component_unbind_all(dev: *mut device, data: *mut c_void);
    fn component_match_add_typed(
        dev: *mut device,
        match_: *mut *mut component_match,
        compare: Option<unsafe extern "C" fn(*mut device, c_int, *mut c_void) -> c_int>,
        compare_data: *mut c_void,
    );
    fn component_master_add_with_match(
        dev: *mut device,
        ops: *const component_master_ops,
        match_: *mut component_match,
    ) -> c_int;
    fn component_master_del(dev: *mut device, ops: *const component_master_ops);
    fn try_module_get(module: *mut module) -> bool_;
    fn module_put(module: *mut module);
    fn init_completion(x: *mut completion);
    fn complete_all(x: *mut completion);
    fn str_enable_disable(enable: bool_) -> *const c_char;
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_info(dev: *mut device, fmt: *const c_char, ...);
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn set_bit(nr: c_uint, addr: *mut c_ulong);
    fn clear_bit(nr: c_uint, addr: *mut c_ulong);
    fn WARN_ON(condition: bool_) -> bool_;
}

unsafe extern "C" fn hdac_acomp_release(_dev: *mut device, _res: *mut c_void) {}

unsafe fn hdac_get_acomp(dev: *mut device) -> *mut drm_audio_component {
    unsafe { devres_find(dev, hdac_acomp_release, ptr::null_mut(), ptr::null_mut()) as *mut drm_audio_component }
}

/**
 * snd_hdac_set_codec_wakeup - Enable / disable HDMI/DP codec wakeup
 * @bus: HDA core bus
 * @enable: enable or disable the wakeup
 *
 * This function is supposed to be used only by a HD-audio controller
 * driver that needs the interaction with graphics driver.
 *
 * This function should be called during the chip reset, also called at
 * resume for updating STATESTS register read.
 *
 * Returns zero for success or a negative error code.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_hdac_set_codec_wakeup(bus: *mut hdac_bus, enable: bool_) -> c_int {
    let acomp = unsafe { (*bus).audio_component };

    if acomp.is_null() || unsafe { (*acomp).ops.is_null() } {
        return -ENODEV;
    }

    if unsafe { (*(*acomp).ops).codec_wake_override.is_none() } {
        return 0;
    }

    unsafe {
        dev_dbg(
            (*bus).dev,
            c"%s codec wakeup\n".as_ptr(),
            str_enable_disable(enable),
        );
        ((*(*acomp).ops).codec_wake_override.unwrap())((*acomp).dev, enable);
    }

    0
}

/**
 * snd_hdac_display_power - Power up / down the power refcount
 * @bus: HDA core bus
 * @idx: HDA codec address, pass HDA_CODEC_IDX_CONTROLLER for controller
 * @enable: power up or down
 *
 * This function is used by either HD-audio controller or codec driver that
 * needs the interaction with graphics driver.
 *
 * This function updates the power status, and calls the get_power() and
 * put_power() ops accordingly, toggling the codec wakeup, too.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_hdac_display_power(bus: *mut hdac_bus, idx: c_uint, enable: bool_) {
    let acomp = unsafe { (*bus).audio_component };

    unsafe {
        dev_dbg((*bus).dev, c"display power %s\n".as_ptr(), str_enable_disable(enable));
        mutex_lock(&mut (*bus).lock);
    }
    if enable {
        unsafe { set_bit(idx, &mut (*bus).display_power_status) };
    } else {
        unsafe { clear_bit(idx, &mut (*bus).display_power_status) };
    }

    if acomp.is_null() || unsafe { (*acomp).ops.is_null() } {
        unsafe { mutex_unlock(&mut (*bus).lock) };
        return;
    }

    if unsafe { (*bus).display_power_status != 0 } {
        if unsafe { (*bus).display_power_active == 0 } {
            let mut cookie: c_ulong = !0;

            if unsafe { (*(*acomp).ops).get_power.is_some() } {
                cookie = unsafe { ((*(*acomp).ops).get_power.unwrap())((*acomp).dev) };
            }

            unsafe {
                snd_hdac_set_codec_wakeup(bus, true);
                snd_hdac_set_codec_wakeup(bus, false);
                (*bus).display_power_active = cookie;
            }
        }
    } else if unsafe { (*bus).display_power_active != 0 } {
        let cookie = unsafe { (*bus).display_power_active };

        if unsafe { (*(*acomp).ops).put_power.is_some() } {
            unsafe { ((*(*acomp).ops).put_power.unwrap())((*acomp).dev, cookie) };
        }

        unsafe { (*bus).display_power_active = 0 };
    }
    unsafe { mutex_unlock(&mut (*bus).lock) };
}

/**
 * snd_hdac_sync_audio_rate - Set N/CTS based on the sample rate
 * @codec: HDA codec
 * @nid: the pin widget NID
 * @dev_id: device identifier
 * @rate: the sample rate to set
 *
 * This function is supposed to be used only by a HD-audio controller
 * driver that needs the interaction with graphics driver.
 *
 * This function sets N/CTS value based on the given sample rate.
 * Returns zero for success, or a negative error code.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_hdac_sync_audio_rate(
    codec: *mut hdac_device,
    nid: hda_nid_t,
    dev_id: c_int,
    rate: c_int,
) -> c_int {
    let bus = unsafe { (*codec).bus };
    let acomp = unsafe { (*bus).audio_component };
    let mut port: c_int;
    let pipe: c_int;

    if acomp.is_null()
        || unsafe { (*acomp).ops.is_null() }
        || unsafe { (*(*acomp).ops).sync_audio_rate.is_none() }
    {
        return -ENODEV;
    }
    port = nid as c_int;
    if unsafe { !(*acomp).audio_ops.is_null() && (*(*acomp).audio_ops).pin2port.is_some() } {
        port = unsafe { ((*(*acomp).audio_ops).pin2port.unwrap())(codec, nid) };
        if port < 0 {
            return -EINVAL;
        }
    }
    pipe = dev_id;
    unsafe { ((*(*acomp).ops).sync_audio_rate.unwrap())((*acomp).dev, port, pipe, rate) }
}

/**
 * snd_hdac_acomp_get_eld - Get the audio state and ELD via component
 * @codec: HDA codec
 * @nid: the pin widget NID
 * @dev_id: device identifier
 * @audio_enabled: the pointer to store the current audio state
 * @buffer: the buffer pointer to store ELD bytes
 * @max_bytes: the max bytes to be stored on @buffer
 *
 * This function is supposed to be used only by a HD-audio controller
 * driver that needs the interaction with graphics driver.
 *
 * This function queries the current state of the audio on the given
 * digital port and fetches the ELD bytes onto the given buffer.
 * It returns the number of bytes for the total ELD data, zero for
 * invalid ELD, or a negative error code.
 *
 * The return size is the total bytes required for the whole ELD bytes,
 * thus it may be over @max_bytes.  If it's over @max_bytes, it implies
 * that only a part of ELD bytes have been fetched.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_hdac_acomp_get_eld(
    codec: *mut hdac_device,
    nid: hda_nid_t,
    dev_id: c_int,
    audio_enabled: *mut bool_,
    buffer: *mut c_char,
    max_bytes: c_int,
) -> c_int {
    let bus = unsafe { (*codec).bus };
    let acomp = unsafe { (*bus).audio_component };
    let mut port: c_int;
    let pipe: c_int;

    if acomp.is_null()
        || unsafe { (*acomp).ops.is_null() }
        || unsafe { (*(*acomp).ops).get_eld.is_none() }
    {
        return -ENODEV;
    }

    port = nid as c_int;
    if unsafe { !(*acomp).audio_ops.is_null() && (*(*acomp).audio_ops).pin2port.is_some() } {
        port = unsafe { ((*(*acomp).audio_ops).pin2port.unwrap())(codec, nid) };
        if port < 0 {
            return -EINVAL;
        }
    }
    pipe = dev_id;
    unsafe {
        ((*(*acomp).ops).get_eld.unwrap())(
            (*acomp).dev,
            port,
            pipe,
            audio_enabled,
            buffer,
            max_bytes,
        )
    }
}

unsafe extern "C" fn hdac_component_master_bind(dev: *mut device) -> c_int {
    let acomp = unsafe { hdac_get_acomp(dev) };
    let mut ret: c_int;

    if unsafe { WARN_ON(acomp.is_null()) } {
        return -EINVAL;
    }

    ret = unsafe { component_bind_all(dev, acomp as *mut c_void) };
    if ret < 0 {
        return ret;
    }

    if unsafe { WARN_ON(!(!(*acomp).dev.is_null() && !(*acomp).ops.is_null())) } {
        ret = -EINVAL;
        unsafe {
            component_unbind_all(dev, acomp as *mut c_void);
            complete_all(&mut (*acomp).master_bind_complete);
        }
        return ret;
    }

    /* pin the module to avoid dynamic unbinding, but only if given */
    if unsafe { !try_module_get((*(*acomp).ops).owner) } {
        ret = -ENODEV;
        unsafe {
            component_unbind_all(dev, acomp as *mut c_void);
            complete_all(&mut (*acomp).master_bind_complete);
        }
        return ret;
    }

    if unsafe { !(*acomp).audio_ops.is_null() && (*(*acomp).audio_ops).master_bind.is_some() } {
        ret = unsafe { ((*(*acomp).audio_ops).master_bind.unwrap())(dev, acomp) };
        if ret < 0 {
            unsafe {
                module_put((*(*acomp).ops).owner);
                component_unbind_all(dev, acomp as *mut c_void);
                complete_all(&mut (*acomp).master_bind_complete);
            }
            return ret;
        }
    }

    unsafe { complete_all(&mut (*acomp).master_bind_complete) };
    0
}

unsafe extern "C" fn hdac_component_master_unbind(dev: *mut device) {
    let acomp = unsafe { hdac_get_acomp(dev) };

    if unsafe { !(*acomp).audio_ops.is_null() && (*(*acomp).audio_ops).master_unbind.is_some() } {
        unsafe { ((*(*acomp).audio_ops).master_unbind.unwrap())(dev, acomp) };
    }
    unsafe {
        module_put((*(*acomp).ops).owner);
        component_unbind_all(dev, acomp as *mut c_void);
        WARN_ON(!((*acomp).ops.is_null() || (*acomp).dev.is_null()));
    }
}

static HDAC_COMPONENT_MASTER_OPS: component_master_ops = component_master_ops {
    bind: Some(hdac_component_master_bind),
    unbind: Some(hdac_component_master_unbind),
};

/**
 * snd_hdac_acomp_register_notifier - Register audio component ops
 * @bus: HDA core bus
 * @aops: audio component ops
 *
 * This function is supposed to be used only by a HD-audio controller
 * driver that needs the interaction with graphics driver.
 *
 * This function sets the given ops to be called by the graphics driver.
 *
 * Returns zero for success or a negative error code.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_hdac_acomp_register_notifier(
    bus: *mut hdac_bus,
    aops: *const drm_audio_component_audio_ops,
) -> c_int {
    if unsafe { (*bus).audio_component.is_null() } {
        return -ENODEV;
    }

    unsafe { (*(*bus).audio_component).audio_ops = aops };
    0
}

/**
 * snd_hdac_acomp_init - Initialize audio component
 * @bus: HDA core bus
 * @aops: audio component ops
 * @match_master: match function for finding components
 * @extra_size: Extra bytes to allocate
 *
 * This function is supposed to be used only by a HD-audio controller
 * driver that needs the interaction with graphics driver.
 *
 * This function initializes and sets up the audio component to communicate
 * with graphics driver.
 *
 * Unlike snd_hdac_i915_init(), this function doesn't synchronize with the
 * binding with the DRM component.  Each caller needs to sync via master_bind
 * audio_ops.
 *
 * Returns zero for success or a negative error code.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_hdac_acomp_init(
    bus: *mut hdac_bus,
    aops: *const drm_audio_component_audio_ops,
    match_master: Option<unsafe extern "C" fn(*mut device, c_int, *mut c_void) -> c_int>,
    extra_size: size_t,
) -> c_int {
    let mut match_: *mut component_match = ptr::null_mut();
    let dev = unsafe { (*bus).dev };
    let acomp: *mut drm_audio_component;
    let ret: c_int;

    if unsafe { WARN_ON(!hdac_get_acomp(dev).is_null()) } {
        return -EBUSY;
    }

    acomp = unsafe {
        devres_alloc(
            hdac_acomp_release,
            core::mem::size_of::<drm_audio_component>() + extra_size,
            GFP_KERNEL,
        ) as *mut drm_audio_component
    };
    if acomp.is_null() {
        return -ENOMEM;
    }
    unsafe {
        (*acomp).audio_ops = aops;
        init_completion(&mut (*acomp).master_bind_complete);
        (*bus).audio_component = acomp;
        devres_add(dev, acomp as *mut c_void);

        component_match_add_typed(dev, &mut match_, match_master, bus as *mut c_void);
        ret = component_master_add_with_match(dev, &HDAC_COMPONENT_MASTER_OPS, match_);
    }
    if ret < 0 {
        unsafe {
            (*bus).audio_component = ptr::null_mut();
            devres_destroy(dev, hdac_acomp_release, ptr::null_mut(), ptr::null_mut());
            dev_info(
                dev,
                c"failed to add audio component master (%d)\n".as_ptr(),
                ret,
            );
        }

        return ret;
    }

    0
}

/**
 * snd_hdac_acomp_exit - Finalize audio component
 * @bus: HDA core bus
 *
 * This function is supposed to be used only by a HD-audio controller
 * driver that needs the interaction with graphics driver.
 *
 * This function releases the audio component that has been used.
 *
 * Returns zero for success or a negative error code.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_hdac_acomp_exit(bus: *mut hdac_bus) -> c_int {
    let dev = unsafe { (*bus).dev };
    let acomp = unsafe { (*bus).audio_component };

    if acomp.is_null() {
        return 0;
    }

    if unsafe { WARN_ON((*bus).display_power_active != 0) && !(*acomp).ops.is_null() } {
        unsafe { ((*(*acomp).ops).put_power.unwrap())((*acomp).dev, (*bus).display_power_active) };
    }

    unsafe {
        (*bus).display_power_active = 0;
        (*bus).display_power_status = 0;

        component_master_del(dev, &HDAC_COMPONENT_MASTER_OPS);

        (*bus).audio_component = ptr::null_mut();
        devres_destroy(dev, hdac_acomp_release, ptr::null_mut(), ptr::null_mut());
    }

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
