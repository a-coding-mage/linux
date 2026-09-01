// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Device management routines
 *  Copyright (c) by Jaroslav Kysela <perex@perex.cz>
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::MaybeUninit;
use core::ptr;

pub type snd_device_type = c_uint;
pub type snd_device_state = c_uint;

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_card {
    pub devices: list_head,
    pub dev: *mut device,
}

#[repr(C)]
pub struct snd_device_ops {
    pub dev_free: Option<unsafe extern "C" fn(*mut snd_device) -> c_int>,
    pub dev_register: Option<unsafe extern "C" fn(*mut snd_device) -> c_int>,
    pub dev_disconnect: Option<unsafe extern "C" fn(*mut snd_device) -> c_int>,
}

#[repr(C)]
pub struct snd_device {
    pub list: list_head,
    pub card: *mut snd_card,
    pub type_: snd_device_type,
    pub state: snd_device_state,
    pub device_data: *mut c_void,
    pub ops: *const snd_device_ops,
}

unsafe extern "C" {
    static ENXIO: c_int;
    static ENOMEM: c_int;
    static SNDRV_DEV_BUILD: snd_device_state;
    static SNDRV_DEV_REGISTERED: snd_device_state;
    static SNDRV_DEV_DISCONNECTED: snd_device_state;
    static SNDRV_DEV_CONTROL: snd_device_type;
    static SNDRV_DEV_LOWLEVEL: snd_device_type;

    fn kzalloc(size: usize, flags: c_uint) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn snd_BUG_ON(cond: bool) -> bool;
    fn snd_BUG();
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn __builtin_return_address(level: c_uint) -> *mut c_void;
}

const GFP_KERNEL: c_uint = 0;

unsafe fn INIT_LIST_HEAD(list: *mut list_head) {
    unsafe {
        (*list).next = list;
        (*list).prev = list;
    }
}

unsafe fn __list_add(new: *mut list_head, prev: *mut list_head, next: *mut list_head) {
    unsafe {
        (*next).prev = new;
        (*new).next = next;
        (*new).prev = prev;
        (*prev).next = new;
    }
}

unsafe fn list_add(new: *mut list_head, head: *mut list_head) {
    unsafe {
        __list_add(new, head, (*head).next);
    }
}

unsafe fn __list_del(prev: *mut list_head, next: *mut list_head) {
    unsafe {
        (*next).prev = prev;
        (*prev).next = next;
    }
}

unsafe fn list_del(entry: *mut list_head) {
    unsafe {
        __list_del((*entry).prev, (*entry).next);
        (*entry).next = ptr::null_mut();
        (*entry).prev = ptr::null_mut();
    }
}

const fn offset_of_list() -> usize {
    let uninit = MaybeUninit::<snd_device>::uninit();
    let base = uninit.as_ptr();
    unsafe { ptr::addr_of!((*base).list) as usize - base as usize }
}

unsafe fn list_entry(ptr: *mut list_head) -> *mut snd_device {
    unsafe { (ptr as *mut u8).sub(offset_of_list()) as *mut snd_device }
}

unsafe fn kzalloc_obj_snd_device() -> *mut snd_device {
    unsafe { kzalloc(core::mem::size_of::<snd_device>(), GFP_KERNEL) as *mut snd_device }
}

/**
 * snd_device_new - create an ALSA device component
 * @card: the card instance
 * @type: the device type, SNDRV_DEV_XXX
 * @device_data: the data pointer of this device
 * @ops: the operator table
 *
 * Creates a new device component for the given data pointer.
 * The device will be assigned to the card and managed together
 * by the card.
 *
 * The data pointer plays a role as the identifier, too, so the
 * pointer address must be unique and unchanged.
 *
 * Return: Zero if successful, or a negative error code on failure.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_device_new(
    card: *mut snd_card,
    type_: snd_device_type,
    device_data: *mut c_void,
    ops: *const snd_device_ops,
) -> c_int {
    let dev: *mut snd_device;
    let mut p: *mut list_head;

    unsafe {
        if snd_BUG_ON(card.is_null() || device_data.is_null() || ops.is_null()) {
            return -ENXIO;
        }
        dev = kzalloc_obj_snd_device();
        if dev.is_null() {
            return -ENOMEM;
        }
        INIT_LIST_HEAD(ptr::addr_of_mut!((*dev).list));
        (*dev).card = card;
        (*dev).type_ = type_;
        (*dev).state = SNDRV_DEV_BUILD;
        (*dev).device_data = device_data;
        (*dev).ops = ops;

        /* insert the entry in an incrementally sorted list */
        p = (*card).devices.prev;
        while p != ptr::addr_of_mut!((*card).devices) {
            let pdev = list_entry(p);
            if (*pdev).type_ as c_uint <= type_ as c_uint {
                break;
            }
            p = (*p).prev;
        }

        list_add(ptr::addr_of_mut!((*dev).list), p);
        0
    }
}

unsafe fn __snd_device_disconnect(dev: *mut snd_device) {
    unsafe {
        if (*dev).state == SNDRV_DEV_REGISTERED {
            if let Some(dev_disconnect) = (*(*dev).ops).dev_disconnect {
                if dev_disconnect(dev) != 0 {
                    dev_err(
                        (*(*dev).card).dev,
                        c"device disconnect failure\n".as_ptr(),
                    );
                }
            }
            (*dev).state = SNDRV_DEV_DISCONNECTED;
        }
    }
}

unsafe fn __snd_device_free(dev: *mut snd_device) {
    unsafe {
        /* unlink */
        list_del(ptr::addr_of_mut!((*dev).list));

        __snd_device_disconnect(dev);
        if let Some(dev_free) = (*(*dev).ops).dev_free {
            if dev_free(dev) != 0 {
                dev_err((*(*dev).card).dev, c"device free failure\n".as_ptr());
            }
        }
        kfree(dev as *mut c_void);
    }
}

unsafe fn look_for_dev(card: *mut snd_card, device_data: *mut c_void) -> *mut snd_device {
    unsafe {
        let mut p = (*card).devices.next;
        while p != ptr::addr_of_mut!((*card).devices) {
            let dev = list_entry(p);
            if (*dev).device_data == device_data {
                return dev;
            }
            p = (*p).next;
        }

        ptr::null_mut()
    }
}

/**
 * snd_device_disconnect - disconnect the device
 * @card: the card instance
 * @device_data: the data pointer to disconnect
 *
 * Turns the device into the disconnection state, invoking
 * dev_disconnect callback, if the device was already registered.
 *
 * Usually called from snd_card_disconnect().
 *
 * Return: Zero if successful, or a negative error code on failure or if the
 * device not found.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_device_disconnect(card: *mut snd_card, device_data: *mut c_void) {
    let dev: *mut snd_device;

    unsafe {
        if snd_BUG_ON(card.is_null() || device_data.is_null()) {
            return;
        }
        dev = look_for_dev(card, device_data);
        if !dev.is_null() {
            __snd_device_disconnect(dev);
        } else {
            dev_dbg(
                (*card).dev,
                c"device disconnect %p (from %pS), not found\n".as_ptr(),
                device_data,
                __builtin_return_address(0),
            );
        }
    }
}

/**
 * snd_device_free - release the device from the card
 * @card: the card instance
 * @device_data: the data pointer to release
 *
 * Removes the device from the list on the card and invokes the
 * callbacks, dev_disconnect and dev_free, corresponding to the state.
 * Then release the device.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_device_free(card: *mut snd_card, device_data: *mut c_void) {
    let dev: *mut snd_device;

    unsafe {
        if snd_BUG_ON(card.is_null() || device_data.is_null()) {
            return;
        }
        dev = look_for_dev(card, device_data);
        if !dev.is_null() {
            __snd_device_free(dev);
        } else {
            dev_dbg(
                (*card).dev,
                c"device free %p (from %pS), not found\n".as_ptr(),
                device_data,
                __builtin_return_address(0),
            );
        }
    }
}

unsafe fn __snd_device_register(dev: *mut snd_device) -> c_int {
    unsafe {
        if (*dev).state == SNDRV_DEV_BUILD {
            if let Some(dev_register) = (*(*dev).ops).dev_register {
                let err = dev_register(dev);
                if err < 0 {
                    return err;
                }
            }
            (*dev).state = SNDRV_DEV_REGISTERED;
        }
        0
    }
}

/**
 * snd_device_register - register the device
 * @card: the card instance
 * @device_data: the data pointer to register
 *
 * Registers the device which was already created via
 * snd_device_new().  Usually this is called from snd_card_register(),
 * but it can be called later if any new devices are created after
 * invocation of snd_card_register().
 *
 * Return: Zero if successful, or a negative error code on failure or if the
 * device not found.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_device_register(
    card: *mut snd_card,
    device_data: *mut c_void,
) -> c_int {
    let dev: *mut snd_device;

    unsafe {
        if snd_BUG_ON(card.is_null() || device_data.is_null()) {
            return -ENXIO;
        }
        dev = look_for_dev(card, device_data);
        if !dev.is_null() {
            return __snd_device_register(dev);
        }
        snd_BUG();
        -ENXIO
    }
}

/*
 * register all the devices on the card.
 * called from init.c
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_device_register_all(card: *mut snd_card) -> c_int {
    let mut dev: *mut snd_device;
    let mut err: c_int;

    unsafe {
        if snd_BUG_ON(card.is_null()) {
            return -ENXIO;
        }
        let mut p = (*card).devices.next;
        while p != ptr::addr_of_mut!((*card).devices) {
            dev = list_entry(p);
            err = __snd_device_register(dev);
            if err < 0 {
                return err;
            }
            p = (*p).next;
        }
        0
    }
}

/*
 * disconnect all the devices on the card.
 * called from init.c
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_device_disconnect_all(card: *mut snd_card) {
    let mut dev: *mut snd_device;

    unsafe {
        if snd_BUG_ON(card.is_null()) {
            return;
        }
        let mut p = (*card).devices.prev;
        while p != ptr::addr_of_mut!((*card).devices) {
            dev = list_entry(p);
            __snd_device_disconnect(dev);
            p = (*p).prev;
        }
    }
}

/*
 * release all the devices on the card.
 * called from init.c
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_device_free_all(card: *mut snd_card) {
    let mut dev: *mut snd_device;
    let mut next: *mut snd_device;

    unsafe {
        if snd_BUG_ON(card.is_null()) {
            return;
        }
        let mut p = (*card).devices.prev;
        while p != ptr::addr_of_mut!((*card).devices) {
            dev = list_entry(p);
            next = list_entry((*p).prev);
            /* exception: free ctl and lowlevel stuff later */
            if (*dev).type_ == SNDRV_DEV_CONTROL || (*dev).type_ == SNDRV_DEV_LOWLEVEL {
                p = ptr::addr_of_mut!((*next).list);
                continue;
            }
            __snd_device_free(dev);
            p = ptr::addr_of_mut!((*next).list);
        }

        /* free all */
        p = (*card).devices.prev;
        while p != ptr::addr_of_mut!((*card).devices) {
            dev = list_entry(p);
            next = list_entry((*p).prev);
            __snd_device_free(dev);
            p = ptr::addr_of_mut!((*next).list);
        }
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
