// SPDX-License-Identifier: GPL-2.0
/*
 * func_utils.h
 *
 * Utility definitions for USB functions
 *
 * Copyright (c) 2013 Samsung Electronics Co., Ltd.
 *		http://www.samsung.com
 *
 * Author: Andrzej Pietrasiewicz <andrzejtp2010@gmail.com>
 */

use core::ffi::c_void;

// Dependencies supplied by the surrounding kernel translation.
pub const SIZE_MAX: usize = usize::MAX;

#[macro_export]
macro_rules! vla_group {
    ($groupname:ident) => {
        paste::paste! { let mut [<$groupname __next>]: usize = 0; }
    };
}

#[macro_export]
macro_rules! vla_group_size {
    ($groupname:ident) => {
        paste::paste! { [<$groupname __next>] }
    };
}

#[macro_export]
macro_rules! vla_item {
    ($groupname:ident, $ty:ty, $name:ident, $n:expr) => {
        paste::paste! { let mut [<$groupname _ $name __offset>]: usize = {
            let mut offset: usize = 0;
            if paste::paste! { [<$groupname __next>] } != $crate::SIZE_MAX {
                let align_mask = core::mem::align_of::<$ty>() - 1;
                let size = $n.checked_mul(core::mem::size_of::<$ty>());
                if let Some(size) = size {
                    if let Some(aligned) = paste::paste! { [<$groupname __next>] }.checked_add(align_mask) {
                        offset = aligned & !align_mask;
                        if let Some(next) = offset.checked_add(size) {
                            paste::paste! { [<$groupname __next>] = next; }
                        } else {
                            paste::paste! { [<$groupname __next>] = $crate::SIZE_MAX; }
                            offset = 0;
                        }
                    } else {
                        paste::paste! { [<$groupname __next>] = $crate::SIZE_MAX; }
                        offset = 0;
                    }
                } else {
                    paste::paste! { [<$groupname __next>] = $crate::SIZE_MAX; }
                    offset = 0;
                }
            }
            offset
        }; }
    };
}

#[macro_export]
macro_rules! vla_item_with_sz {
    ($groupname:ident, $ty:ty, $name:ident, $n:expr) => {
        paste::paste! { let [<$groupname _ $name __sz>]: usize = $n.checked_mul(core::mem::size_of::<$ty>()).unwrap_or($crate::SIZE_MAX);
        let mut [<$groupname _ $name __offset>]: usize = {
            let mut offset: usize = 0;
            if paste::paste! { [<$groupname __next>] } != $crate::SIZE_MAX {
                let align_mask = core::mem::align_of::<$ty>() - 1;
                if let Some(aligned) = paste::paste! { [<$groupname __next>] }.checked_add(align_mask) {
                    offset = aligned & !align_mask;
                    if let Some(next) = offset.checked_add([<$groupname _ $name __sz>]) {
                        paste::paste! { [<$groupname __next>] = next; }
                    } else {
                        paste::paste! { [<$groupname __next>] = $crate::SIZE_MAX; }
                        offset = 0;
                    }
                } else {
                    paste::paste! { [<$groupname __next>] = $crate::SIZE_MAX; }
                    offset = 0;
                }
            }
            offset
        }; }
    };
}

#[macro_export]
macro_rules! vla_ptr {
    ($ptr:expr, $groupname:ident, $name:ident) => {
        paste::paste! { (($ptr as *mut u8).wrapping_add([<$groupname _ $name __offset>])) as *mut c_void }
    };
}

#[repr(C)]
pub struct usb_ep {
    _private: [u8; 0],
}

#[repr(C)]
pub struct usb_request {
    pub buf: *mut c_void,
}

unsafe extern "C" {
    pub fn alloc_ep_req(ep: *mut usb_ep, len: usize) -> *mut usb_request;
    pub fn WARN_ON(condition: bool) -> bool;
    pub fn kfree(ptr: *mut c_void);
    pub fn usb_ep_free_request(ep: *mut usb_ep, req: *mut usb_request);
}

/** Frees a usb_request previously allocated by alloc_ep_req(). */
#[inline]
pub unsafe fn free_ep_req(ep: *mut usb_ep, req: *mut usb_request) {
    WARN_ON((*req).buf.is_null());
    kfree((*req).buf);
    (*req).buf = core::ptr::null_mut();
    usb_ep_free_request(ep, req);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
