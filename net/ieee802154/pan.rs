// SPDX-License-Identifier: GPL-2.0
/*
 * IEEE 802.15.4 PAN management
 *
 * Copyright (C) 2023 Qorvo US, Inc
 * Authors:
 *   - David Girault <david.girault@qorvo.com>
 *   - Miquel Raynal <miquel.raynal@bootlin.com>
 */

// Linux kernel and IEEE 802.15.4 declarations are supplied by the surrounding
// translation unit.

const IEEE802154_ADDR_SHORT: u8 = 2;
const IEEE802154_ADDR_SHORT_BROADCAST: u16 = 0xffff;
const IEEE802154_ADDR_SHORT_UNSPEC: u16 = 0xfffe;

#[repr(C)]
pub struct ieee802154_addr {
    pub mode: u8,
    pub extended_addr: u64,
}

#[repr(C)]
pub struct ieee802154_pan_device {
    pub node: list_head,
    pub extended_addr: u64,
    pub short_addr: u16,
}

#[repr(C)]
pub struct wpan_dev {
    pub association_lock: mutex,
    pub children: list_head,
    pub parent: *mut ieee802154_pan_device,
    pub short_addr: u16,
    pub max_associations: u32,
}

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

extern "C" {
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn get_random_bytes(buf: *mut core::ffi::c_void, len: usize);
    fn lockdep_assert_held(lock: *mut mutex);
}

#[inline]
unsafe fn cfg802154_pan_device_is_matching(
    pan_dev: *mut ieee802154_pan_device,
    ext_dev: *mut ieee802154_addr,
) -> bool {
    if pan_dev.is_null() || ext_dev.is_null() {
        return false;
    }

    if (*ext_dev).mode == IEEE802154_ADDR_SHORT {
        return false;
    }

    (*pan_dev).extended_addr == (*ext_dev).extended_addr
}

pub unsafe fn cfg802154_device_is_associated(wpan_dev: *mut wpan_dev) -> bool {
    let is_assoc: bool;

    mutex_lock(&mut (*wpan_dev).association_lock);
    is_assoc = (*wpan_dev).children.next != &mut (*wpan_dev).children
        || !(*wpan_dev).parent.is_null();
    mutex_unlock(&mut (*wpan_dev).association_lock);

    is_assoc
}

pub unsafe fn cfg802154_device_is_parent(
    wpan_dev: *mut wpan_dev,
    target: *mut ieee802154_addr,
) -> bool {
    lockdep_assert_held(&mut (*wpan_dev).association_lock);

    cfg802154_pan_device_is_matching((*wpan_dev).parent, target)
}

pub unsafe fn cfg802154_device_is_child(
    wpan_dev: *mut wpan_dev,
    target: *mut ieee802154_addr,
) -> *mut ieee802154_pan_device {
    lockdep_assert_held(&mut (*wpan_dev).association_lock);

    let head = &mut (*wpan_dev).children as *mut list_head;
    let mut pos = (*head).next;
    while pos != head {
        let child = (pos as *mut u8).sub(core::mem::offset_of!(ieee802154_pan_device, node))
            as *mut ieee802154_pan_device;
        if cfg802154_pan_device_is_matching(child, target) {
            return child;
        }
        pos = (*pos).next;
    }

    core::ptr::null_mut()
}

pub unsafe fn cfg802154_get_free_short_addr(wpan_dev: *mut wpan_dev) -> u16 {
    lockdep_assert_held(&mut (*wpan_dev).association_lock);

    loop {
        let mut addr: u16 = 0;
        get_random_bytes(
            &mut addr as *mut u16 as *mut core::ffi::c_void,
            2,
        );
        if addr == IEEE802154_ADDR_SHORT_BROADCAST || addr == IEEE802154_ADDR_SHORT_UNSPEC {
            continue;
        }

        if (*wpan_dev).short_addr == addr {
            continue;
        }

        if !(*wpan_dev).parent.is_null() && (*(*wpan_dev).parent).short_addr == addr {
            continue;
        }

        let head = &mut (*wpan_dev).children as *mut list_head;
        let mut pos = (*head).next;
        let mut used = false;
        while pos != head {
            let child = (pos as *mut u8).sub(core::mem::offset_of!(ieee802154_pan_device, node))
                as *mut ieee802154_pan_device;
            if (*child).short_addr == addr {
                used = true;
                break;
            }
            pos = (*pos).next;
        }
        if used {
            continue;
        }

        return addr;
    }
}

pub unsafe fn cfg802154_set_max_associations(
    wpan_dev: *mut wpan_dev,
    max: u32,
) -> u32 {
    lockdep_assert_held(&mut (*wpan_dev).association_lock);

    let old_max = (*wpan_dev).max_associations;
    (*wpan_dev).max_associations = max;

    old_max
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
