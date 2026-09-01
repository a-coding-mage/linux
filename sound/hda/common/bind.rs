// SPDX-License-Identifier: GPL-2.0-only
/*
 * HD-audio codec driver binding
 * Copyright (c) Takashi Iwai <tiwai@suse.de>
 */

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

type bool_ = bool;
type u32 = u32;
type hda_nid_t = c_uint;

const PM_EVENT_ON: c_int = 0;
const EINVAL: c_int = 22;
const ENODEV: c_int = 19;
const HDA_DEV_LEGACY: c_int = 0;
const HDA_CODEC_ID_GENERIC_HDMI: u32 = 0;
const HDA_CODEC_ID_GENERIC: u32 = 0;
const AC_WID_AUD_IN: c_uint = 0;
const AC_WID_AUD_OUT: c_uint = 0;
const AC_WCAP_DIGITAL: c_uint = 0;

#[repr(C)]
pub struct hdac_device {
    pub dev: device,
    pub vendor_id: u32,
    pub revision_id: u32,
    pub addr: c_uint,
    pub vendor_name: *const c_char,
    pub chip_name: *const c_char,
    pub lazy_cache: bool_,
}

#[repr(C)]
pub struct hdac_driver {
    pub driver: device_driver,
    pub type_: c_int,
    pub match_: Option<unsafe extern "C" fn(*mut hdac_device, *const hdac_driver) -> c_int>,
    pub unsol_event: Option<unsafe extern "C" fn(*mut hdac_device, c_uint)>,
}

#[repr(C)]
pub struct hda_codec {
    pub core: hdac_device,
    pub preset: *const hda_device_id,
    pub probe_id: u32,
    pub card: *mut snd_card,
    pub bus: *mut hda_bus,
    pub wcaps: *mut c_void,
    pub pcm_ref: c_void,
    pub configured: c_int,
    pub modelname: *const c_char,
}

#[repr(C)]
pub struct hda_codec_driver {
    pub core: hdac_driver,
    pub id: *const hda_device_id,
    pub ops: *const hda_codec_driver_ops,
}

#[repr(C)]
pub struct hda_device_id {
    pub vendor_id: u32,
    pub rev_id: u32,
    pub name: *const c_char,
}

#[repr(C)]
pub struct hda_codec_driver_ops {
    pub probe: Option<unsafe extern "C" fn(*mut hda_codec, *const hda_device_id) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut hda_codec)>,
    pub unsol_event: Option<unsafe extern "C" fn(*mut hda_codec, c_uint)>,
}

#[repr(C)]
pub struct device {
    pub driver: *mut device_driver,
    pub power: dev_pm_info,
}

#[repr(C)]
pub struct dev_pm_info {
    pub power_state: pm_message_t,
}

#[repr(C)]
pub struct pm_message_t {
    pub event: c_int,
}

#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
    pub owner: *mut module,
    pub bus: *mut bus_type,
    pub probe: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    pub shutdown: Option<unsafe extern "C" fn(*mut device)>,
    pub pm: *const dev_pm_ops,
}

#[repr(C)]
pub struct hda_bus {
    pub core: hdac_bus,
    pub shutdown: bool_,
    pub mixer_assigned: c_uint,
    pub bus_probing: bool_,
    pub card: *mut snd_card,
}

#[repr(C)]
pub struct hdac_bus {
    pub ext_ops: *const hdac_ext_bus_ops,
}

#[repr(C)]
pub struct hdac_ext_bus_ops {
    pub hdev_attach: Option<unsafe extern "C" fn(*mut hdac_device) -> c_int>,
    pub hdev_detach: Option<unsafe extern "C" fn(*mut hdac_device) -> c_int>,
}

#[repr(C)]
pub struct snd_card {
    pub shutdown: bool_,
    pub mixername: [c_char; 80],
    pub registered: bool_,
}

#[repr(C)]
pub struct module;
#[repr(C)]
pub struct bus_type;
#[repr(C)]
pub struct dev_pm_ops;

unsafe extern "C" {
    static mut snd_hda_bus_type: bus_type;
    static hda_codec_driver_pm: dev_pm_ops;

    fn snd_hdac_device_set_chip_name(codec: *mut hdac_device, name: *const c_char) -> c_int;
    fn snprintf(s: *mut c_char, n: usize, format: *const c_char, ...) -> c_int;
    fn snd_hdac_regmap_init(codec: *mut hdac_device) -> c_int;
    fn try_module_get(module: *mut module) -> bool_;
    fn module_put(module: *mut module);
    fn snd_hda_codec_build_pcms(codec: *mut hda_codec) -> c_int;
    fn snd_hda_codec_build_controls(codec: *mut hda_codec) -> c_int;
    fn snd_card_register(card: *mut snd_card) -> c_int;
    fn snd_hda_codec_register(codec: *mut hda_codec);
    fn snd_hda_codec_cleanup_for_unbind(codec: *mut hda_codec);
    fn snd_hda_codec_disconnect_pcms(codec: *mut hda_codec);
    fn snd_hda_jack_tbl_disconnect(codec: *mut hda_codec);
    fn snd_refcount_sync(ref_: *mut c_void);
    fn snd_power_sync_ref(card: *mut snd_card);
    fn snd_hda_codec_shutdown(codec: *mut hda_codec);
    fn driver_register(driver: *mut device_driver) -> c_int;
    fn driver_unregister(driver: *mut device_driver);
    fn device_attach(dev: *mut device) -> c_int;
    fn request_module(name: *const c_char) -> c_int;
    fn snd_hdac_codec_modalias(codec: *mut hdac_device, buf: *mut c_char, size: usize);
    fn get_wcaps(codec: *mut hda_codec, nid: hda_nid_t) -> c_uint;
    fn get_wcaps_type(wcaps: c_uint) -> c_uint;
    fn device_is_registered(dev: *mut device) -> bool_;
    fn snd_hdac_device_register(codec: *mut hdac_device) -> c_int;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn codec_dbg(codec: *mut hda_codec, fmt: *const c_char, ...);
}

unsafe fn dev_to_hda_codec(dev: *mut device) -> *mut hda_codec {
    dev as *mut hda_codec
}

unsafe fn hda_codec_to_driver(codec: *mut hda_codec) -> *mut hda_codec_driver {
    (*(*codec).core.dev.driver) as *mut device_driver as *mut hda_codec_driver
}

unsafe fn hda_codec_dev(codec: *mut hda_codec) -> *mut device {
    &mut (*codec).core.dev
}

unsafe fn pm_ptr(pm: *const dev_pm_ops) -> *const dev_pm_ops {
    pm
}

unsafe fn WARN_ON(condition: bool_) -> bool_ {
    condition
}

/*
 * find a matching codec id
 */
unsafe extern "C" fn hda_codec_match(dev: *mut hdac_device, drv: *const hdac_driver) -> c_int {
    let codec = dev as *mut hda_codec;
    let driver = drv as *const hda_codec_driver;
    let mut list: *const hda_device_id;
    /* check probe_id instead of vendor_id if set */
    let id: u32 = if (*codec).probe_id != 0 {
        (*codec).probe_id
    } else {
        (*codec).core.vendor_id
    };
    let rev_id: u32 = (*codec).core.revision_id;

    list = (*driver).id;
    while (*list).vendor_id != 0 {
        if (*list).vendor_id == id && ((*list).rev_id == 0 || (*list).rev_id == rev_id) {
            (*codec).preset = list;
            return 1;
        }
        list = list.add(1);
    }
    0
}

/* process an unsolicited event */
unsafe extern "C" fn hda_codec_unsol_event(dev: *mut hdac_device, ev: c_uint) {
    let codec = dev as *mut hda_codec;
    let driver = hda_codec_to_driver(codec);

    /* ignore unsol events during shutdown */
    if (*(*codec).card).shutdown || (*(*codec).bus).shutdown {
        return;
    }

    /* ignore unsol events during system suspend/resume */
    if (*codec).core.dev.power.power_state.event != PM_EVENT_ON {
        return;
    }

    if let Some(unsol_event) = (*(*driver).ops).unsol_event {
        unsol_event(codec, ev);
    }
}

/**
 * snd_hda_codec_set_name - set the codec name
 * @codec: the HDA codec
 * @name: name string to set
 */
#[no_mangle]
pub unsafe extern "C" fn snd_hda_codec_set_name(
    codec: *mut hda_codec,
    name: *const c_char,
) -> c_int {
    let mut err: c_int;

    if name.is_null() {
        return 0;
    }
    err = snd_hdac_device_set_chip_name(&mut (*codec).core, name);
    if err < 0 {
        return err;
    }

    /* update the mixer name */
    if (*(*codec).card).mixername[0] == 0
        || (*(*codec).bus).mixer_assigned >= (*codec).core.addr
    {
        snprintf(
            (*(*codec).card).mixername.as_mut_ptr(),
            size_of::<[c_char; 80]>(),
            b"%s %s\0".as_ptr() as *const c_char,
            (*codec).core.vendor_name,
            (*codec).core.chip_name,
        );
        (*(*codec).bus).mixer_assigned = (*codec).core.addr;
    }

    0
}

unsafe extern "C" fn hda_codec_driver_probe(dev: *mut device) -> c_int {
    let codec = dev_to_hda_codec(dev);
    let owner = (*(*dev).driver).owner;
    let driver = hda_codec_to_driver(codec);
    let mut err: c_int;

    if !(*(*codec).bus).core.ext_ops.is_null() {
        if WARN_ON((*(*(*codec).bus).core.ext_ops).hdev_attach.is_none()) {
            return -EINVAL;
        }
        return (*(*(*codec).bus).core.ext_ops).hdev_attach.unwrap()(&mut (*codec).core);
    }

    if WARN_ON((*codec).preset.is_null()) {
        return -EINVAL;
    }

    err = snd_hda_codec_set_name(codec, (*(*codec).preset).name);
    if err < 0 {
        goto_error(codec, err);
        return err;
    }
    err = snd_hdac_regmap_init(&mut (*codec).core);
    if err < 0 {
        goto_error(codec, err);
        return err;
    }

    if !try_module_get(owner) {
        err = -EINVAL;
        goto_error(codec, err);
        return err;
    }

    if WARN_ON((*driver).ops.is_null() || (*(*driver).ops).probe.is_none()) {
        err = -EINVAL;
        module_put(owner);
        goto_error(codec, err);
        return err;
    }

    err = (*(*driver).ops).probe.unwrap()(codec, (*codec).preset);
    if err < 0 {
        module_put(owner);
        goto_error(codec, err);
        return err;
    }
    err = snd_hda_codec_build_pcms(codec);
    if err < 0 {
        if let Some(remove) = (*(*driver).ops).remove {
            remove(codec);
        }
        module_put(owner);
        goto_error(codec, err);
        return err;
    }
    err = snd_hda_codec_build_controls(codec);
    if err < 0 {
        if let Some(remove) = (*(*driver).ops).remove {
            remove(codec);
        }
        module_put(owner);
        goto_error(codec, err);
        return err;
    }
    /* only register after the bus probe finished; otherwise it's racy */
    if !(*(*codec).bus).bus_probing && (*(*codec).card).registered {
        err = snd_card_register((*codec).card);
        if err < 0 {
            if let Some(remove) = (*(*driver).ops).remove {
                remove(codec);
            }
            module_put(owner);
            goto_error(codec, err);
            return err;
        }
        snd_hda_codec_register(codec);
    }

    (*codec).core.lazy_cache = true;
    0
}

unsafe fn goto_error(codec: *mut hda_codec, _err: c_int) {
    snd_hda_codec_cleanup_for_unbind(codec);
    (*codec).preset = ptr::null();
}

unsafe extern "C" fn hda_codec_driver_remove(dev: *mut device) -> c_int {
    let codec = dev_to_hda_codec(dev);
    let driver = hda_codec_to_driver(codec);

    if !(*(*codec).bus).core.ext_ops.is_null() {
        if WARN_ON((*(*(*codec).bus).core.ext_ops).hdev_detach.is_none()) {
            return -EINVAL;
        }
        return (*(*(*codec).bus).core.ext_ops).hdev_detach.unwrap()(&mut (*codec).core);
    }

    snd_hda_codec_disconnect_pcms(codec);
    snd_hda_jack_tbl_disconnect(codec);
    snd_refcount_sync(&mut (*codec).pcm_ref);
    snd_power_sync_ref((*(*codec).bus).card);

    if let Some(remove) = (*(*driver).ops).remove {
        remove(codec);
    }
    snd_hda_codec_cleanup_for_unbind(codec);
    (*codec).preset = ptr::null();
    module_put((*(*dev).driver).owner);
    0
}

unsafe extern "C" fn hda_codec_driver_shutdown(dev: *mut device) {
    snd_hda_codec_shutdown(dev_to_hda_codec(dev));
}

#[no_mangle]
pub unsafe extern "C" fn __hda_codec_driver_register(
    drv: *mut hda_codec_driver,
    name: *const c_char,
    owner: *mut module,
) -> c_int {
    (*drv).core.driver.name = name;
    (*drv).core.driver.owner = owner;
    (*drv).core.driver.bus = &mut snd_hda_bus_type;
    (*drv).core.driver.probe = Some(hda_codec_driver_probe);
    (*drv).core.driver.remove = Some(hda_codec_driver_remove);
    (*drv).core.driver.shutdown = Some(hda_codec_driver_shutdown);
    (*drv).core.driver.pm = pm_ptr(&hda_codec_driver_pm);
    (*drv).core.type_ = HDA_DEV_LEGACY;
    (*drv).core.match_ = Some(hda_codec_match);
    (*drv).core.unsol_event = Some(hda_codec_unsol_event);
    driver_register(&mut (*drv).core.driver)
}

#[no_mangle]
pub unsafe extern "C" fn hda_codec_driver_unregister(drv: *mut hda_codec_driver) {
    driver_unregister(&mut (*drv).core.driver);
}

unsafe fn codec_probed(codec: *mut hda_codec) -> bool_ {
    device_attach(hda_codec_dev(codec)) > 0 && !(*codec).preset.is_null()
}

/* try to auto-load codec module */
unsafe fn request_codec_module(codec: *mut hda_codec) {
    /* Original C code is compiled only under #ifdef MODULE. */
    let mut modalias: [c_char; 32] = [0; 32];
    let mut mod_: *const c_char = ptr::null();

    match (*codec).probe_id {
        HDA_CODEC_ID_GENERIC_HDMI => {
            /* Original C code sets this only if IS_MODULE(CONFIG_SND_HDA_CODEC_HDMI). */
            mod_ = ptr::null();
        }
        HDA_CODEC_ID_GENERIC => {
            /* Original C code sets this only if IS_MODULE(CONFIG_SND_HDA_GENERIC). */
            mod_ = ptr::null();
        }
        _ => {
            snd_hdac_codec_modalias(&mut (*codec).core, modalias.as_mut_ptr(), modalias.len());
            mod_ = modalias.as_ptr();
        }
    }

    if !mod_.is_null() {
        request_module(mod_);
    }
}

/* try to auto-load and bind the codec module */
unsafe fn codec_bind_module(codec: *mut hda_codec) {
    /* Original C body is compiled only under #ifdef MODULE. */
    request_codec_module(codec);
    if codec_probed(codec) {
        return;
    }
}

/* if all audio out widgets are digital, let's assume the codec as a HDMI/DP */
unsafe fn is_likely_hdmi_codec(codec: *mut hda_codec) -> bool_ {
    let mut nid: hda_nid_t = 0;

    /*
     * For ASoC users, if snd_hda_hdmi_codec module is denylisted and any
     * event causes i915 enumeration to fail, ->wcaps remains uninitialized.
     */
    if (*codec).wcaps.is_null() {
        return true;
    }

    /* Original C iterates with for_each_hda_codec_node(nid, codec). */
    while nid != 0 {
        let wcaps: c_uint = get_wcaps(codec, nid);
        match get_wcaps_type(wcaps) {
            AC_WID_AUD_IN => {
                return false;
            } /* HDMI parser supports only HDMI out */
            AC_WID_AUD_OUT => {
                if (wcaps & AC_WCAP_DIGITAL) == 0 {
                    return false;
                }
            }
            _ => {}
        }
        nid = nid.wrapping_add(1);
    }
    true
}

unsafe fn codec_bind_generic(codec: *mut hda_codec) -> c_int {
    if (*codec).probe_id != 0 {
        return -ENODEV;
    }

    if is_likely_hdmi_codec(codec) {
        (*codec).probe_id = HDA_CODEC_ID_GENERIC_HDMI;
        request_codec_module(codec);
        if codec_probed(codec) {
            return 0;
        }
    }

    (*codec).probe_id = HDA_CODEC_ID_GENERIC;
    request_codec_module(codec);
    if codec_probed(codec) {
        return 0;
    }
    -ENODEV
}

unsafe fn is_generic_config(codec: *mut hda_codec) -> bool_ {
    /* Original C is enabled only under IS_ENABLED(CONFIG_SND_HDA_GENERIC). */
    !(*codec).modelname.is_null() && strcmp((*codec).modelname, b"generic\0".as_ptr() as *const c_char) == 0
}

/**
 * snd_hda_codec_configure - (Re-)configure the HD-audio codec
 * @codec: the HDA codec
 *
 * Start parsing of the given codec tree and (re-)initialize the whole
 * codec driver binding.
 *
 * Returns 0 if successful or a negative error code.
 */
#[no_mangle]
pub unsafe extern "C" fn snd_hda_codec_configure(codec: *mut hda_codec) -> c_int {
    let mut err: c_int;

    if (*codec).configured != 0 {
        return 0;
    }

    if is_generic_config(codec) {
        (*codec).probe_id = HDA_CODEC_ID_GENERIC;
    } else {
        (*codec).probe_id = 0;
    }

    if !device_is_registered(&mut (*codec).core.dev) {
        err = snd_hdac_device_register(&mut (*codec).core);
        if err < 0 {
            return err;
        }
    }

    if (*codec).preset.is_null() {
        codec_bind_module(codec);
    }
    if (*codec).preset.is_null() {
        err = codec_bind_generic(codec);
        if err < 0 {
            codec_dbg(codec, b"Unable to bind the codec\n\0".as_ptr() as *const c_char);
            return err;
        }
    }

    (*codec).configured = 1;
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
