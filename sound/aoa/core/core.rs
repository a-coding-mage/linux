// SPDX-License-Identifier: GPL-2.0-only
/*
 * Apple Onboard Audio driver core
 *
 * Copyright 2006 Johannes Berg <johannes@sipsolutions.net>
 */

// C dependencies: <linux/init.h>, <linux/module.h>, <linux/list.h>,
// "../aoa.h", and "alsa.h".

use core::ffi::{c_char, c_int, c_void};
use core::mem::offset_of;
use core::ptr;

// MODULE_DESCRIPTION("Apple Onboard Audio Sound Driver");
// MODULE_AUTHOR("Johannes Berg <johannes@sipsolutions.net>");
// MODULE_LICENSE("GPL");

const EBUSY: c_int = 16;
const ENOENT: c_int = 2;
const EALREADY: c_int = 114;
const EEXIST: c_int = 17;
const EINVAL: c_int = 22;

const KERN_ERR: *const c_char = c"\x013".as_ptr();
const SND_AOA_FABRIC_DIDNT_LIKE_CODEC: *const c_char =
    c"\x013snd-aoa: fabric didn't like codec %s\n".as_ptr();
const SND_AOA_CODEC_DIDNT_INIT: *const c_char =
    c"\x013snd-aoa: codec %s didn't init\n".as_ptr();
const SND_AOA_FABRIC_UNASSIGNED: *const c_char =
    c"\x013snd-aoa: fabric unassigned in aoa_fabric_unlink_codec\n".as_ptr();

#[repr(C)]
pub struct module {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct aoa_codec {
    pub list: list_head,
    pub owner: *mut module,
    pub name: *const c_char,
    pub fabric: *mut aoa_fabric,
    pub init: Option<unsafe extern "C" fn(*mut aoa_codec) -> c_int>,
    pub exit: Option<unsafe extern "C" fn(*mut aoa_codec)>,
}

#[repr(C)]
pub struct aoa_fabric {
    pub name: *const c_char,
    pub owner: *mut module,
    pub found_codec: Option<unsafe extern "C" fn(*mut aoa_codec) -> c_int>,
    pub remove_codec: Option<unsafe extern "C" fn(*mut aoa_codec)>,
    pub attached_codec: Option<unsafe extern "C" fn(*mut aoa_codec)>,
}

unsafe extern "C" {
    fn try_module_get(module: *mut module) -> bool;
    fn module_put(module: *mut module);
    fn printk(fmt: *const c_char, ...) -> c_int;
    fn dump_stack();
    fn aoa_alsa_init(name: *const c_char, owner: *mut module, dev: *mut device) -> c_int;
    fn aoa_alsa_cleanup();
}

/* We allow only one fabric. This simplifies things,
 * and more don't really make that much sense */
static mut fabric: *mut aoa_fabric = ptr::null_mut();
static mut codec_list: list_head = list_head {
    next: ptr::addr_of_mut!(codec_list),
    prev: ptr::addr_of_mut!(codec_list),
};

unsafe fn list_add(new: *mut list_head, head: *mut list_head) {
    (*new).next = (*head).next;
    (*new).prev = head;
    (*(*head).next).prev = new;
    (*head).next = new;
}

unsafe fn list_del(entry: *mut list_head) {
    (*(*entry).next).prev = (*entry).prev;
    (*(*entry).prev).next = (*entry).next;
}

unsafe fn list_entry_aoa_codec(ptr: *mut list_head) -> *mut aoa_codec {
    (ptr as *mut u8).sub(offset_of!(aoa_codec, list)) as *mut aoa_codec
}

unsafe fn attach_codec_to_fabric(c: *mut aoa_codec) -> c_int {
    let mut err: c_int;

    if !try_module_get((*c).owner) {
        return -EBUSY;
    }
    /* found_codec has to be assigned */
    err = -ENOENT;
    if let Some(found_codec) = (*fabric).found_codec {
        err = found_codec(c);
    }
    if err != 0 {
        module_put((*c).owner);
        printk(SND_AOA_FABRIC_DIDNT_LIKE_CODEC, (*c).name);
        return err;
    }
    (*c).fabric = fabric;

    err = 0;
    if let Some(init) = (*c).init {
        err = init(c);
    }
    if err != 0 {
        printk(SND_AOA_CODEC_DIDNT_INIT, (*c).name);
        (*c).fabric = ptr::null_mut();
        if let Some(remove_codec) = (*fabric).remove_codec {
            remove_codec(c);
        }
        module_put((*c).owner);
        return err;
    }
    if let Some(attached_codec) = (*fabric).attached_codec {
        attached_codec(c);
    }
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn aoa_codec_register(codec: *mut aoa_codec) -> c_int {
    let mut err: c_int = 0;

    /* if there's a fabric already, we can tell if we
     * will want to have this codec, so propagate error
     * through. Otherwise, this will happen later... */
    if !fabric.is_null() {
        err = attach_codec_to_fabric(codec);
    }
    if err == 0 {
        list_add(ptr::addr_of_mut!((*codec).list), ptr::addr_of_mut!(codec_list));
    }
    err
}
// EXPORT_SYMBOL_GPL(aoa_codec_register);

#[unsafe(no_mangle)]
pub unsafe extern "C" fn aoa_codec_unregister(codec: *mut aoa_codec) {
    list_del(ptr::addr_of_mut!((*codec).list));
    if !(*codec).fabric.is_null() {
        if let Some(exit) = (*codec).exit {
            exit(codec);
        }
    }
    if !fabric.is_null() {
        if let Some(remove_codec) = (*fabric).remove_codec {
            remove_codec(codec);
        }
    }
    (*codec).fabric = ptr::null_mut();
    module_put((*codec).owner);
}
// EXPORT_SYMBOL_GPL(aoa_codec_unregister);

#[unsafe(no_mangle)]
pub unsafe extern "C" fn aoa_fabric_register(new_fabric: *mut aoa_fabric, dev: *mut device) -> c_int {
    let mut c: *mut aoa_codec;
    let mut err: c_int;

    /* allow querying for presence of fabric
     * (i.e. do this test first!) */
    if new_fabric == fabric {
        err = -EALREADY;
    } else {
        if !fabric.is_null() {
            return -EEXIST;
        }
        if new_fabric.is_null() {
            return -EINVAL;
        }

        err = aoa_alsa_init((*new_fabric).name, (*new_fabric).owner, dev);
        if err != 0 {
            return err;
        }

        fabric = new_fabric;
    }

    let mut pos = codec_list.next;
    while pos != ptr::addr_of_mut!(codec_list) {
        c = list_entry_aoa_codec(pos);
        pos = (*pos).next;
        if (*c).fabric != fabric {
            attach_codec_to_fabric(c);
        }
    }
    err
}
// EXPORT_SYMBOL_GPL(aoa_fabric_register);

#[unsafe(no_mangle)]
pub unsafe extern "C" fn aoa_fabric_unregister(old_fabric: *mut aoa_fabric) {
    let mut c: *mut aoa_codec;

    if fabric != old_fabric {
        return;
    }

    let mut pos = codec_list.next;
    while pos != ptr::addr_of_mut!(codec_list) {
        c = list_entry_aoa_codec(pos);
        pos = (*pos).next;
        if !(*c).fabric.is_null() {
            aoa_fabric_unlink_codec(c);
        }
    }

    aoa_alsa_cleanup();

    fabric = ptr::null_mut();
}
// EXPORT_SYMBOL_GPL(aoa_fabric_unregister);

#[unsafe(no_mangle)]
pub unsafe extern "C" fn aoa_fabric_unlink_codec(codec: *mut aoa_codec) {
    if (*codec).fabric.is_null() {
        printk(SND_AOA_FABRIC_UNASSIGNED);
        dump_stack();
        return;
    }
    if let Some(exit) = (*codec).exit {
        exit(codec);
    }
    if let Some(remove_codec) = (*(*codec).fabric).remove_codec {
        remove_codec(codec);
    }
    (*codec).fabric = ptr::null_mut();
    module_put((*codec).owner);
}
// EXPORT_SYMBOL_GPL(aoa_fabric_unlink_codec);

unsafe extern "C" fn aoa_init() -> c_int {
    0
}

unsafe extern "C" fn aoa_exit() {
    aoa_alsa_cleanup();
}

// module_init(aoa_init);
// module_exit(aoa_exit);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
