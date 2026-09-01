// SPDX-License-Identifier: GPL-2.0
/*
 * sysfs support for HD-audio core device
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

type ssize_t = isize;
type size_t = usize;
type umode_t = u16;
type hda_nid_t = c_uint;

const EIO: c_int = 5;
const ENOMEM: c_int = 12;
const GFP_KERNEL: c_uint = 0;
const KOBJ_CHANGE: c_int = 1;

const AC_WID_AUD_OUT: c_uint = 0;
const AC_WID_AUD_IN: c_uint = 1;
const AC_WID_PIN: c_uint = 4;
const AC_WCAP_IN_AMP: c_uint = 1 << 1;
const AC_WCAP_OUT_AMP: c_uint = 1 << 2;
const AC_WCAP_POWER: c_uint = 1 << 10;
const AC_PAR_PIN_CAP: c_uint = 0x0c;
const AC_PAR_PCM: c_uint = 0x0a;
const AC_PAR_STREAM: c_uint = 0x0b;
const AC_PAR_AMP_IN_CAP: c_uint = 0x0d;
const AC_PAR_AMP_OUT_CAP: c_uint = 0x12;
const AC_PAR_POWER_STATE: c_uint = 0x0f;
const AC_PAR_GPIO_CAP: c_uint = 0x11;
const AC_VERB_GET_CONFIG_DEFAULT: c_uint = 0xf1c;

#[repr(C)]
pub struct kobject {
    pub name: *const c_char,
    pub parent: *mut kobject,
}

#[repr(C)]
pub struct device {
    pub kobj: kobject,
}

#[repr(C)]
pub struct attribute {
    pub name: *const c_char,
    pub mode: umode_t,
}

#[repr(C)]
pub struct device_attribute {
    pub attr: attribute,
    pub show: Option<
        unsafe extern "C" fn(*mut device, *mut device_attribute, *mut c_char) -> ssize_t,
    >,
    pub store: Option<
        unsafe extern "C" fn(
            *mut device,
            *mut device_attribute,
            *const c_char,
            size_t,
        ) -> ssize_t,
    >,
}

#[repr(C)]
pub struct attribute_group {
    pub name: *const c_char,
    pub attrs: *mut *mut attribute,
}

#[repr(C)]
pub struct sysfs_ops {
    pub show: Option<unsafe extern "C" fn(*mut kobject, *mut attribute, *mut c_char) -> ssize_t>,
    pub store: Option<
        unsafe extern "C" fn(*mut kobject, *mut attribute, *const c_char, size_t) -> ssize_t,
    >,
}

#[repr(C)]
pub struct kobj_type {
    pub release: Option<unsafe extern "C" fn(*mut kobject)>,
    pub sysfs_ops: *const sysfs_ops,
}

#[repr(C)]
pub struct hdac_widget_tree {
    pub root: *mut kobject,
    pub afg: *mut kobject,
    pub nodes: *mut *mut kobject,
}

#[repr(C)]
pub struct hdac_device {
    pub dev: device,
    pub type_: c_uint,
    pub vendor_id: c_uint,
    pub subsystem_id: c_uint,
    pub revision_id: c_uint,
    pub afg: hda_nid_t,
    pub mfg: hda_nid_t,
    pub vendor_name: *const c_char,
    pub chip_name: *const c_char,
    pub widgets: *mut hdac_widget_tree,
    pub num_nodes: c_int,
    pub start_nid: hda_nid_t,
    pub end_nid: hda_nid_t,
}

#[repr(C)]
pub struct widget_attribute {
    pub attr: attribute,
    pub show: Option<
        unsafe extern "C" fn(
            *mut hdac_device,
            hda_nid_t,
            *mut widget_attribute,
            *mut c_char,
        ) -> ssize_t,
    >,
    pub store: Option<
        unsafe extern "C" fn(
            *mut hdac_device,
            hda_nid_t,
            *mut widget_attribute,
            *const c_char,
            size_t,
        ) -> ssize_t,
    >,
}

unsafe extern "C" {
    fn sysfs_emit(buf: *mut c_char, fmt: *const c_char, ...) -> ssize_t;
    fn sysfs_emit_at(buf: *mut c_char, at: ssize_t, fmt: *const c_char, ...) -> ssize_t;
    fn snd_hdac_codec_modalias(codec: *mut hdac_device, buf: *mut c_char, len: size_t) -> ssize_t;
    fn dev_to_hdac_dev(dev: *mut device) -> *mut hdac_device;
    fn kobj_to_dev(kobj: *mut kobject) -> *mut device;
    fn kstrtoint(s: *const c_char, base: c_uint, res: *mut c_int) -> ssize_t;
    fn kfree(ptr: *const c_void);
    fn kmemdup(src: *const c_void, len: size_t, gfp: c_uint) -> *mut c_void;
    fn kzalloc(size: size_t, gfp: c_uint) -> *mut c_void;
    fn kobject_init(kobj: *mut kobject, ktype: *const kobj_type);
    fn kobject_add(kobj: *mut kobject, parent: *mut kobject, fmt: *const c_char, ...) -> c_int;
    fn kobject_put(kobj: *mut kobject);
    fn kobject_create_and_add(name: *const c_char, parent: *mut kobject) -> *mut kobject;
    fn kobject_uevent(kobj: *mut kobject, action: c_int) -> c_int;
    fn sysfs_create_group(kobj: *mut kobject, grp: *const attribute_group) -> c_int;
    fn sysfs_remove_group(kobj: *mut kobject, grp: *const attribute_group);
    fn snd_hdac_get_wcaps(codec: *mut hdac_device, nid: hda_nid_t) -> c_uint;
    fn snd_hdac_get_wcaps_type(wcaps: c_uint) -> c_uint;
    fn snd_hdac_read_parm(codec: *mut hdac_device, nid: hda_nid_t, parm: c_uint) -> c_uint;
    fn snd_hdac_read(
        codec: *mut hdac_device,
        nid: hda_nid_t,
        verb: c_uint,
        parm: c_uint,
        val: *mut c_uint,
    ) -> c_int;
    fn snd_hdac_get_connections(
        codec: *mut hdac_device,
        nid: hda_nid_t,
        list: *mut hda_nid_t,
        max_conns: c_int,
    ) -> c_int;
}

const fn cstr(bytes: &'static [u8]) -> *const c_char {
    bytes.as_ptr() as *const c_char
}

macro_rules! offset_of {
    ($ty:ty, $field:ident) => {{
        let uninit = core::mem::MaybeUninit::<$ty>::uninit();
        let base = uninit.as_ptr();
        unsafe { (&raw const (*base).$field as usize) - (base as usize) }
    }};
}

unsafe fn container_of_attribute(attr: *mut attribute) -> *mut widget_attribute {
    (attr as *mut u8).sub(offset_of!(widget_attribute, attr)) as *mut widget_attribute
}

macro_rules! codec_attr_show {
    ($name:ident, $field:ident) => {
        unsafe extern "C" fn $name(
            dev: *mut device,
            _attr: *mut device_attribute,
            buf: *mut c_char,
        ) -> ssize_t {
            let codec = dev_to_hdac_dev(dev);
            sysfs_emit(buf, cstr(b"0x%x\n\0"), (*codec).$field)
        }
    };
}

macro_rules! codec_attr_str_show {
    ($name:ident, $field:ident) => {
        unsafe extern "C" fn $name(
            dev: *mut device,
            _attr: *mut device_attribute,
            buf: *mut c_char,
        ) -> ssize_t {
            let codec = dev_to_hdac_dev(dev);
            sysfs_emit(
                buf,
                cstr(b"%s\n\0"),
                if !(*codec).$field.is_null() {
                    (*codec).$field
                } else {
                    cstr(b"\0")
                },
            )
        }
    };
}

codec_attr_show!(type_show, type_);
codec_attr_show!(vendor_id_show, vendor_id);
codec_attr_show!(subsystem_id_show, subsystem_id);
codec_attr_show!(revision_id_show, revision_id);
codec_attr_show!(afg_show, afg);
codec_attr_show!(mfg_show, mfg);
codec_attr_str_show!(vendor_name_show, vendor_name);
codec_attr_str_show!(chip_name_show, chip_name);

unsafe extern "C" fn modalias_show(
    dev: *mut device,
    _attr: *mut device_attribute,
    buf: *mut c_char,
) -> ssize_t {
    snd_hdac_codec_modalias(dev_to_hdac_dev(dev), buf, 256)
}

static mut dev_attr_type: device_attribute = device_attribute {
    attr: attribute { name: cstr(b"type\0"), mode: 0o444 },
    show: Some(type_show),
    store: None,
};
static mut dev_attr_vendor_id: device_attribute = device_attribute {
    attr: attribute { name: cstr(b"vendor_id\0"), mode: 0o444 },
    show: Some(vendor_id_show),
    store: None,
};
static mut dev_attr_subsystem_id: device_attribute = device_attribute {
    attr: attribute { name: cstr(b"subsystem_id\0"), mode: 0o444 },
    show: Some(subsystem_id_show),
    store: None,
};
static mut dev_attr_revision_id: device_attribute = device_attribute {
    attr: attribute { name: cstr(b"revision_id\0"), mode: 0o444 },
    show: Some(revision_id_show),
    store: None,
};
static mut dev_attr_afg: device_attribute = device_attribute {
    attr: attribute { name: cstr(b"afg\0"), mode: 0o444 },
    show: Some(afg_show),
    store: None,
};
static mut dev_attr_mfg: device_attribute = device_attribute {
    attr: attribute { name: cstr(b"mfg\0"), mode: 0o444 },
    show: Some(mfg_show),
    store: None,
};
static mut dev_attr_vendor_name: device_attribute = device_attribute {
    attr: attribute { name: cstr(b"vendor_name\0"), mode: 0o444 },
    show: Some(vendor_name_show),
    store: None,
};
static mut dev_attr_chip_name: device_attribute = device_attribute {
    attr: attribute { name: cstr(b"chip_name\0"), mode: 0o444 },
    show: Some(chip_name_show),
    store: None,
};
static mut dev_attr_modalias: device_attribute = device_attribute {
    attr: attribute { name: cstr(b"modalias\0"), mode: 0o444 },
    show: Some(modalias_show),
    store: None,
};

static mut hdac_dev_attrs: [*mut attribute; 10] = unsafe {
    [
        &raw mut dev_attr_type.attr,
        &raw mut dev_attr_vendor_id.attr,
        &raw mut dev_attr_subsystem_id.attr,
        &raw mut dev_attr_revision_id.attr,
        &raw mut dev_attr_afg.attr,
        &raw mut dev_attr_mfg.attr,
        &raw mut dev_attr_vendor_name.attr,
        &raw mut dev_attr_chip_name.attr,
        &raw mut dev_attr_modalias.attr,
        ptr::null_mut(),
    ]
};

static mut hdac_dev_attr_group: attribute_group = attribute_group {
    name: ptr::null(),
    attrs: unsafe { &raw mut hdac_dev_attrs as *mut *mut attribute },
};

#[unsafe(no_mangle)]
pub static mut hdac_dev_attr_groups: [*const attribute_group; 2] =
    unsafe { [&raw const hdac_dev_attr_group, ptr::null()] };

unsafe extern "C" fn get_codec_nid(kobj: *mut kobject, codecp: *mut *mut hdac_device) -> c_int {
    let dev = kobj_to_dev((*(*kobj).parent).parent);
    let mut nid: c_int = 0;
    let ret = kstrtoint((*kobj).name, 16, &mut nid);

    if ret < 0 {
        return ret as c_int;
    }
    *codecp = dev_to_hdac_dev(dev);
    nid
}

unsafe extern "C" fn widget_attr_show(
    kobj: *mut kobject,
    attr: *mut attribute,
    buf: *mut c_char,
) -> ssize_t {
    let wid_attr = container_of_attribute(attr);
    let mut codec: *mut hdac_device = ptr::null_mut();
    let nid: c_int;

    if (*wid_attr).show.is_none() {
        return -(EIO as ssize_t);
    }
    nid = get_codec_nid(kobj, &mut codec);
    if nid < 0 {
        return nid as ssize_t;
    }
    ((*wid_attr).show.unwrap())(codec, nid as hda_nid_t, wid_attr, buf)
}

unsafe extern "C" fn widget_attr_store(
    kobj: *mut kobject,
    attr: *mut attribute,
    buf: *const c_char,
    count: size_t,
) -> ssize_t {
    let wid_attr = container_of_attribute(attr);
    let mut codec: *mut hdac_device = ptr::null_mut();
    let nid: c_int;

    if (*wid_attr).store.is_none() {
        return -(EIO as ssize_t);
    }
    nid = get_codec_nid(kobj, &mut codec);
    if nid < 0 {
        return nid as ssize_t;
    }
    ((*wid_attr).store.unwrap())(codec, nid as hda_nid_t, wid_attr, buf, count)
}

static widget_sysfs_ops: sysfs_ops = sysfs_ops {
    show: Some(widget_attr_show),
    store: Some(widget_attr_store),
};

unsafe extern "C" fn widget_release(kobj: *mut kobject) {
    kfree(kobj as *const c_void);
}

static widget_ktype: kobj_type = kobj_type {
    release: Some(widget_release),
    sysfs_ops: &widget_sysfs_ops,
};

macro_rules! widget_attr_ro {
    ($var:ident, $name:expr, $show:ident) => {
        static mut $var: widget_attribute = widget_attribute {
            attr: attribute {
                name: cstr($name),
                mode: 0o444,
            },
            show: Some($show),
            store: None,
        };
    };
}

unsafe extern "C" fn caps_show(
    codec: *mut hdac_device,
    nid: hda_nid_t,
    _attr: *mut widget_attribute,
    buf: *mut c_char,
) -> ssize_t {
    sysfs_emit(buf, cstr(b"0x%08x\n\0"), snd_hdac_get_wcaps(codec, nid))
}

unsafe extern "C" fn pin_caps_show(
    codec: *mut hdac_device,
    nid: hda_nid_t,
    _attr: *mut widget_attribute,
    buf: *mut c_char,
) -> ssize_t {
    if snd_hdac_get_wcaps_type(snd_hdac_get_wcaps(codec, nid)) != AC_WID_PIN {
        return 0;
    }
    sysfs_emit(
        buf,
        cstr(b"0x%08x\n\0"),
        snd_hdac_read_parm(codec, nid, AC_PAR_PIN_CAP),
    )
}

unsafe extern "C" fn pin_cfg_show(
    codec: *mut hdac_device,
    nid: hda_nid_t,
    _attr: *mut widget_attribute,
    buf: *mut c_char,
) -> ssize_t {
    let mut val: c_uint = 0;

    if snd_hdac_get_wcaps_type(snd_hdac_get_wcaps(codec, nid)) != AC_WID_PIN {
        return 0;
    }
    if snd_hdac_read(codec, nid, AC_VERB_GET_CONFIG_DEFAULT, 0, &mut val) != 0 {
        return 0;
    }
    sysfs_emit(buf, cstr(b"0x%08x\n\0"), val)
}

unsafe fn has_pcm_cap(codec: *mut hdac_device, nid: hda_nid_t) -> bool {
    if nid == (*codec).afg || nid == (*codec).mfg {
        return true;
    }
    match snd_hdac_get_wcaps_type(snd_hdac_get_wcaps(codec, nid)) {
        AC_WID_AUD_OUT | AC_WID_AUD_IN => true,
        _ => false,
    }
}

unsafe extern "C" fn pcm_caps_show(
    codec: *mut hdac_device,
    nid: hda_nid_t,
    _attr: *mut widget_attribute,
    buf: *mut c_char,
) -> ssize_t {
    if !has_pcm_cap(codec, nid) {
        return 0;
    }
    sysfs_emit(buf, cstr(b"0x%08x\n\0"), snd_hdac_read_parm(codec, nid, AC_PAR_PCM))
}

unsafe extern "C" fn pcm_formats_show(
    codec: *mut hdac_device,
    nid: hda_nid_t,
    _attr: *mut widget_attribute,
    buf: *mut c_char,
) -> ssize_t {
    if !has_pcm_cap(codec, nid) {
        return 0;
    }
    sysfs_emit(
        buf,
        cstr(b"0x%08x\n\0"),
        snd_hdac_read_parm(codec, nid, AC_PAR_STREAM),
    )
}

unsafe extern "C" fn amp_in_caps_show(
    codec: *mut hdac_device,
    nid: hda_nid_t,
    _attr: *mut widget_attribute,
    buf: *mut c_char,
) -> ssize_t {
    if nid != (*codec).afg && (snd_hdac_get_wcaps(codec, nid) & AC_WCAP_IN_AMP) == 0 {
        return 0;
    }
    sysfs_emit(
        buf,
        cstr(b"0x%08x\n\0"),
        snd_hdac_read_parm(codec, nid, AC_PAR_AMP_IN_CAP),
    )
}

unsafe extern "C" fn amp_out_caps_show(
    codec: *mut hdac_device,
    nid: hda_nid_t,
    _attr: *mut widget_attribute,
    buf: *mut c_char,
) -> ssize_t {
    if nid != (*codec).afg && (snd_hdac_get_wcaps(codec, nid) & AC_WCAP_OUT_AMP) == 0 {
        return 0;
    }
    sysfs_emit(
        buf,
        cstr(b"0x%08x\n\0"),
        snd_hdac_read_parm(codec, nid, AC_PAR_AMP_OUT_CAP),
    )
}

unsafe extern "C" fn power_caps_show(
    codec: *mut hdac_device,
    nid: hda_nid_t,
    _attr: *mut widget_attribute,
    buf: *mut c_char,
) -> ssize_t {
    if nid != (*codec).afg && (snd_hdac_get_wcaps(codec, nid) & AC_WCAP_POWER) == 0 {
        return 0;
    }
    sysfs_emit(
        buf,
        cstr(b"0x%08x\n\0"),
        snd_hdac_read_parm(codec, nid, AC_PAR_POWER_STATE),
    )
}

unsafe extern "C" fn gpio_caps_show(
    codec: *mut hdac_device,
    nid: hda_nid_t,
    _attr: *mut widget_attribute,
    buf: *mut c_char,
) -> ssize_t {
    sysfs_emit(
        buf,
        cstr(b"0x%08x\n\0"),
        snd_hdac_read_parm(codec, nid, AC_PAR_GPIO_CAP),
    )
}

unsafe extern "C" fn connections_show(
    codec: *mut hdac_device,
    nid: hda_nid_t,
    _attr: *mut widget_attribute,
    buf: *mut c_char,
) -> ssize_t {
    let mut list: [hda_nid_t; 32] = [0; 32];
    let mut ret: ssize_t = 0;

    let nconns = snd_hdac_get_connections(codec, nid, list.as_mut_ptr(), list.len() as c_int);
    if nconns <= 0 {
        return nconns as ssize_t;
    }
    for i in 0..nconns {
        ret += sysfs_emit_at(
            buf,
            ret,
            cstr(b"%s0x%02x\0"),
            if i != 0 { cstr(b" \0") } else { cstr(b"\0") },
            list[i as usize],
        );
    }
    ret += sysfs_emit_at(buf, ret, cstr(b"\n\0"));
    ret
}

widget_attr_ro!(wid_attr_caps, b"caps\0", caps_show);
widget_attr_ro!(wid_attr_pin_caps, b"pin_caps\0", pin_caps_show);
widget_attr_ro!(wid_attr_pin_cfg, b"pin_cfg\0", pin_cfg_show);
widget_attr_ro!(wid_attr_pcm_caps, b"pcm_caps\0", pcm_caps_show);
widget_attr_ro!(wid_attr_pcm_formats, b"pcm_formats\0", pcm_formats_show);
widget_attr_ro!(wid_attr_amp_in_caps, b"amp_in_caps\0", amp_in_caps_show);
widget_attr_ro!(wid_attr_amp_out_caps, b"amp_out_caps\0", amp_out_caps_show);
widget_attr_ro!(wid_attr_power_caps, b"power_caps\0", power_caps_show);
widget_attr_ro!(wid_attr_gpio_caps, b"gpio_caps\0", gpio_caps_show);
widget_attr_ro!(wid_attr_connections, b"connections\0", connections_show);

static mut widget_node_attrs: [*mut attribute; 10] = unsafe {
    [
        &raw mut wid_attr_caps.attr,
        &raw mut wid_attr_pin_caps.attr,
        &raw mut wid_attr_pin_cfg.attr,
        &raw mut wid_attr_pcm_caps.attr,
        &raw mut wid_attr_pcm_formats.attr,
        &raw mut wid_attr_amp_in_caps.attr,
        &raw mut wid_attr_amp_out_caps.attr,
        &raw mut wid_attr_power_caps.attr,
        &raw mut wid_attr_connections.attr,
        ptr::null_mut(),
    ]
};

static mut widget_afg_attrs: [*mut attribute; 7] = unsafe {
    [
        &raw mut wid_attr_pcm_caps.attr,
        &raw mut wid_attr_pcm_formats.attr,
        &raw mut wid_attr_amp_in_caps.attr,
        &raw mut wid_attr_amp_out_caps.attr,
        &raw mut wid_attr_power_caps.attr,
        &raw mut wid_attr_gpio_caps.attr,
        ptr::null_mut(),
    ]
};

static mut widget_node_group: attribute_group = attribute_group {
    name: ptr::null(),
    attrs: unsafe { &raw mut widget_node_attrs as *mut *mut attribute },
};

static mut widget_afg_group: attribute_group = attribute_group {
    name: ptr::null(),
    attrs: unsafe { &raw mut widget_afg_attrs as *mut *mut attribute },
};

unsafe fn free_widget_node(kobj: *mut kobject, group: *const attribute_group) {
    if !kobj.is_null() {
        sysfs_remove_group(kobj, group);
        kobject_put(kobj);
    }
}

unsafe fn widget_tree_free(codec: *mut hdac_device) {
    let tree = (*codec).widgets;
    let mut p: *mut *mut kobject;

    if tree.is_null() {
        return;
    }
    free_widget_node((*tree).afg, &raw const widget_afg_group);
    if !(*tree).nodes.is_null() {
        p = (*tree).nodes;
        while !(*p).is_null() {
            free_widget_node(*p, &raw const widget_node_group);
            p = p.add(1);
        }
        kfree((*tree).nodes as *const c_void);
    }
    kobject_put((*tree).root);
    kfree(tree as *const c_void);
    (*codec).widgets = ptr::null_mut();
}

unsafe fn add_widget_node(
    parent: *mut kobject,
    nid: hda_nid_t,
    group: *const attribute_group,
    res: *mut *mut kobject,
) -> c_int {
    let kobj = kzalloc(size_of::<kobject>(), GFP_KERNEL) as *mut kobject;
    let mut err: c_int;

    if kobj.is_null() {
        return -ENOMEM;
    }
    kobject_init(kobj, &widget_ktype);
    err = kobject_add(kobj, parent, cstr(b"%02x\0"), nid);
    if err < 0 {
        kobject_put(kobj);
        return err;
    }
    err = sysfs_create_group(kobj, group);
    if err < 0 {
        kobject_put(kobj);
        return err;
    }

    *res = kobj;
    0
}

unsafe fn widget_tree_create(codec: *mut hdac_device) -> c_int {
    let tree: *mut hdac_widget_tree;
    let mut i: c_int;
    let mut err: c_int;
    let mut nid: hda_nid_t;

    tree = kzalloc(size_of::<hdac_widget_tree>(), GFP_KERNEL) as *mut hdac_widget_tree;
    (*codec).widgets = tree;
    if tree.is_null() {
        return -ENOMEM;
    }

    (*tree).root = kobject_create_and_add(cstr(b"widgets\0"), &mut (*codec).dev.kobj);
    if (*tree).root.is_null() {
        return -ENOMEM;
    }

    (*tree).nodes = kzalloc(
        size_of::<*mut kobject>() * (((*codec).num_nodes + 1) as size_t),
        GFP_KERNEL,
    ) as *mut *mut kobject;
    if (*tree).nodes.is_null() {
        return -ENOMEM;
    }

    i = 0;
    nid = (*codec).start_nid;
    while i < (*codec).num_nodes {
        err = add_widget_node(
            (*tree).root,
            nid,
            &raw const widget_node_group,
            (*tree).nodes.add(i as usize),
        );
        if err < 0 {
            return err;
        }
        i += 1;
        nid = nid.wrapping_add(1);
    }

    if (*codec).afg != 0 {
        err = add_widget_node(
            (*tree).root,
            (*codec).afg,
            &raw const widget_afg_group,
            &mut (*tree).afg,
        );
        if err < 0 {
            return err;
        }
    }

    kobject_uevent((*tree).root, KOBJ_CHANGE);
    0
}

/* call with codec->widget_lock held */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn hda_widget_sysfs_init(codec: *mut hdac_device) -> c_int {
    let err: c_int;

    if !(*codec).widgets.is_null() {
        return 0; /* already created */
    }

    err = widget_tree_create(codec);
    if err < 0 {
        widget_tree_free(codec);
        return err;
    }

    0
}

/* call with codec->widget_lock held */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn hda_widget_sysfs_exit(codec: *mut hdac_device) {
    widget_tree_free(codec);
}

/* call with codec->widget_lock held */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn hda_widget_sysfs_reinit(
    codec: *mut hdac_device,
    start_nid: hda_nid_t,
    num_nodes: c_int,
) -> c_int {
    let tree: *mut hdac_widget_tree;
    let end_nid: hda_nid_t = start_nid.wrapping_add(num_nodes as hda_nid_t);
    let mut nid: hda_nid_t;
    let mut i: c_int;

    if (*codec).widgets.is_null() {
        return 0;
    }

    tree = kmemdup(
        (*codec).widgets as *const c_void,
        size_of::<hdac_widget_tree>(),
        GFP_KERNEL,
    ) as *mut hdac_widget_tree;
    if tree.is_null() {
        return -ENOMEM;
    }

    (*tree).nodes = kzalloc(
        size_of::<*mut kobject>() * ((num_nodes + 1) as size_t),
        GFP_KERNEL,
    ) as *mut *mut kobject;
    if (*tree).nodes.is_null() {
        kfree(tree as *const c_void);
        return -ENOMEM;
    }

    /* prune non-existing nodes */
    i = 0;
    nid = (*codec).start_nid;
    while i < (*codec).num_nodes {
        if nid < start_nid || nid >= end_nid {
            free_widget_node(
                *(*(*codec).widgets).nodes.add(i as usize),
                &raw const widget_node_group,
            );
        }
        i += 1;
        nid = nid.wrapping_add(1);
    }

    /* add new nodes */
    i = 0;
    nid = start_nid;
    while i < num_nodes {
        if nid < (*codec).start_nid || nid >= (*codec).end_nid {
            add_widget_node(
                (*tree).root,
                nid,
                &raw const widget_node_group,
                (*tree).nodes.add(i as usize),
            );
        } else {
            *(*tree).nodes.add(i as usize) =
                *(*(*codec).widgets).nodes.add((nid - (*codec).start_nid) as usize);
        }
        i += 1;
        nid = nid.wrapping_add(1);
    }

    /* replace with the new tree */
    kfree((*(*codec).widgets).nodes as *const c_void);
    kfree((*codec).widgets as *const c_void);
    (*codec).widgets = tree;

    kobject_uevent((*tree).root, KOBJ_CHANGE);
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
