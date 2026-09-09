// SPDX-License-Identifier: GPL-2.0-only
/*
 * VMware vSockets Driver
 *
 * Copyright (C) 2007-2012 VMware, Inc. All rights reserved.
 */

// Dependencies supplied by the surrounding kernel translation.

pub unsafe fn vsock_addr_init(addr: *mut sockaddr_vm, cid: u32, port: u32) {
    core::ptr::write_bytes(addr.cast::<u8>(), 0, core::mem::size_of::<sockaddr_vm>());
    (*addr).svm_family = AF_VSOCK;
    (*addr).svm_cid = cid;
    (*addr).svm_port = port;
}

pub unsafe fn vsock_addr_validate(addr: *const sockaddr_vm) -> i32 {
    let svm_valid_flags: u8 = VMADDR_FLAG_TO_HOST;

    if addr.is_null() {
        return -EFAULT;
    }

    if (*addr).svm_family != AF_VSOCK {
        return -EAFNOSUPPORT;
    }

    if (*addr).svm_flags & !svm_valid_flags != 0 {
        return -EINVAL;
    }

    0
}

pub unsafe fn vsock_addr_bound(addr: *const sockaddr_vm) -> bool {
    (*addr).svm_port != VMADDR_PORT_ANY
}

pub unsafe fn vsock_addr_unbind(addr: *mut sockaddr_vm) {
    vsock_addr_init(addr, VMADDR_CID_ANY, VMADDR_PORT_ANY);
}

pub unsafe fn vsock_addr_equals_addr(
    addr: *const sockaddr_vm,
    other: *const sockaddr_vm,
) -> bool {
    (*addr).svm_cid == (*other).svm_cid && (*addr).svm_port == (*other).svm_port
}

pub unsafe fn vsock_addr_cast(
    addr: *const sockaddr_unsized,
    len: usize,
    out_addr: *mut *mut sockaddr_vm,
) -> i32 {
    if len < core::mem::size_of::<sockaddr_vm>() {
        return -EFAULT;
    }

    *out_addr = addr.cast_mut().cast::<sockaddr_vm>();
    vsock_addr_validate(*out_addr)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
