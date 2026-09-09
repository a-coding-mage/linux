// SPDX-License-Identifier: GPL-2.0
/*
 * This file implement the Wireless Extensions priv API.
 *
 * Authors : Jean Tourrilhes - HPL - <jt@hpl.hp.com>
 * Copyright (c) 1997-2007 Jean Tourrilhes, All Rights Reserved.
 * Copyright 2009 Johannes Berg <johannes@sipsolutions.net>
 */

// External Linux kernel types, constants, and functions are supplied by the
// surrounding kernel translation.

pub unsafe fn iw_handler_get_private(
    dev: *mut net_device,
    _info: *mut iw_request_info,
    wrqu: *mut iwreq_data,
    extra: *mut c_char,
) -> c_int {
    if (*(*dev).wireless_handlers).num_private_args == 0
        || (*(*dev).wireless_handlers).private_args.is_null()
    {
        return -EOPNOTSUPP;
    }

    if (*wrqu).data.length < (*(*dev).wireless_handlers).num_private_args {
        (*wrqu).data.length = (*(*dev).wireless_handlers).num_private_args;
        return -E2BIG;
    }

    (*wrqu).data.length = (*(*dev).wireless_handlers).num_private_args;
    memcpy(
        extra as *mut c_void,
        (*(*dev).wireless_handlers).private_args as *const c_void,
        core::mem::size_of::<iw_priv_args>() * (*wrqu).data.length as usize,
    );
    0
}

/* Size (in bytes) of the various private data types */
static IW_PRIV_TYPE_SIZE: [u8; 8] = [
    0,                         /* IW_PRIV_TYPE_NONE */
    1,                         /* IW_PRIV_TYPE_BYTE */
    1,                         /* IW_PRIV_TYPE_CHAR */
    0,                         /* Not defined */
    core::mem::size_of::<u32>() as u8, /* IW_PRIV_TYPE_INT */
    core::mem::size_of::<iw_freq>() as u8, /* IW_PRIV_TYPE_FLOAT */
    core::mem::size_of::<sockaddr>() as u8, /* IW_PRIV_TYPE_ADDR */
    0,                         /* Not defined */
];

unsafe fn get_priv_size(args: u16) -> c_int {
    let num = args & IW_PRIV_SIZE_MASK;
    let ty = (args & IW_PRIV_TYPE_MASK) >> 12;
    (num as c_int) * IW_PRIV_TYPE_SIZE[ty as usize] as c_int
}

unsafe fn adjust_priv_size(args: u16, iwp: *mut iw_point) -> c_int {
    let mut num = (*iwp).length;
    let max = args & IW_PRIV_SIZE_MASK;
    let ty = (args & IW_PRIV_TYPE_MASK) >> 12;
    if max < num {
        num = max;
    }
    (num as c_int) * IW_PRIV_TYPE_SIZE[ty as usize] as c_int
}

/* Wrapper to call a private Wireless Extension handler. */
unsafe fn get_priv_descr_and_size(
    dev: *mut net_device,
    cmd: c_uint,
    descrp: *mut *const iw_priv_args,
) -> c_int {
    let mut descr: *const iw_priv_args = core::ptr::null();
    let handlers = (*dev).wireless_handlers;
    for i in 0..(*handlers).num_private_args {
        if cmd == (*handlers).private_args.add(i as usize).read().cmd as c_uint {
            descr = (*handlers).private_args.add(i as usize);
            break;
        }
    }

    let mut extra_size = 0;
    if !descr.is_null() {
        if IW_IS_SET(cmd) {
            let mut offset = 0;
            if (*descr).name[0] == 0 {
                offset = core::mem::size_of::<u32>() as c_int;
            }
            extra_size = get_priv_size((*descr).set_args);
            if ((*descr).set_args & IW_PRIV_SIZE_FIXED) != 0
                && extra_size + offset <= IFNAMSIZ
            {
                extra_size = 0;
            }
        } else {
            extra_size = get_priv_size((*descr).get_args);
            if ((*descr).get_args & IW_PRIV_SIZE_FIXED) != 0 && extra_size <= IFNAMSIZ {
                extra_size = 0;
            }
        }
    }
    *descrp = descr;
    extra_size
}

unsafe fn ioctl_private_iw_point(
    iwp: *mut iw_point,
    cmd: c_uint,
    descr: *const iw_priv_args,
    handler: iw_handler,
    dev: *mut net_device,
    info: *mut iw_request_info,
    extra_size: c_int,
) -> c_int {
    if IW_IS_SET(cmd) {
        if (*iwp).pointer.is_null() && (*iwp).length != 0 {
            return -EFAULT;
        }
        if (*iwp).length > ((*descr).set_args & IW_PRIV_SIZE_MASK) {
            return -E2BIG;
        }
    } else if (*iwp).pointer.is_null() {
        return -EFAULT;
    }

    let extra = kzalloc(extra_size as usize, GFP_KERNEL);
    if extra.is_null() {
        return -ENOMEM;
    }

    let mut err = 0;
    if IW_IS_SET(cmd) && (*iwp).length != 0 {
        if copy_from_user(extra, (*iwp).pointer, extra_size as usize) != 0 {
            err = -EFAULT;
        } else {
            err = handler(dev, info, iwp as *mut iwreq_data, extra as *mut c_char);
        }
    } else {
        err = handler(dev, info, iwp as *mut iwreq_data, extra as *mut c_char);
    }

    if err == 0 && IW_IS_GET(cmd) {
        let mut copy_size = extra_size;
        if ((*descr).get_args & IW_PRIV_SIZE_FIXED) == 0 {
            copy_size = adjust_priv_size((*descr).get_args, iwp);
        }
        if copy_to_user((*iwp).pointer, extra, copy_size as usize) != 0 {
            err = -EFAULT;
        }
    }
    kfree(extra);
    err
}

pub unsafe fn ioctl_private_call(
    dev: *mut net_device,
    iwr: *mut iwreq,
    cmd: c_uint,
    info: *mut iw_request_info,
    handler: iw_handler,
) -> c_int {
    let mut descr: *const iw_priv_args = core::ptr::null();
    let extra_size = get_priv_descr_and_size(dev, cmd, &mut descr);
    let mut ret;
    if extra_size == 0 {
        ret = handler(dev, info, &mut (*iwr).u, &mut (*iwr).u as *mut _ as *mut c_char);
    } else {
        ret = ioctl_private_iw_point(&mut (*iwr).u.data, cmd, descr, handler, dev, info, extra_size);
    }
    if ret == -EIWCOMMIT {
        ret = call_commit_handler(dev);
    }
    ret
}

/* CONFIG_COMPAT is a build-time condition from the original kernel source. */
#[cfg(CONFIG_COMPAT)]
pub unsafe fn compat_private_call(
    dev: *mut net_device,
    iwr: *mut iwreq,
    cmd: c_uint,
    info: *mut iw_request_info,
    handler: iw_handler,
) -> c_int {
    let mut descr: *const iw_priv_args = core::ptr::null();
    let extra_size = get_priv_descr_and_size(dev, cmd, &mut descr);
    let mut ret;
    if extra_size == 0 {
        ret = handler(dev, info, &mut (*iwr).u, &mut (*iwr).u as *mut _ as *mut c_char);
    } else {
        let iwp_compat = &mut (*iwr).u.data as *mut iw_point as *mut compat_iw_point;
        let mut iwp: iw_point = core::mem::zeroed();
        iwp.pointer = compat_ptr((*iwp_compat).pointer);
        iwp.length = (*iwp_compat).length;
        iwp.flags = (*iwp_compat).flags;
        ret = ioctl_private_iw_point(&mut iwp, cmd, descr, handler, dev, info, extra_size);
        (*iwp_compat).pointer = ptr_to_compat(iwp.pointer);
        (*iwp_compat).length = iwp.length;
        (*iwp_compat).flags = iwp.flags;
    }
    if ret == -EIWCOMMIT {
        ret = call_commit_handler(dev);
    }
    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
