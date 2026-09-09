/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * VMware vSockets Driver
 *
 * Copyright (C) 2007-2013 VMware, Inc. All rights reserved.
 */

// Dependency: uapi/linux/vm_sockets.h

extern "C" {
    pub fn vsock_addr_init(addr: *mut sockaddr_vm, cid: u32, port: u32);
    pub fn vsock_addr_validate(addr: *const sockaddr_vm) -> i32;
    pub fn vsock_addr_bound(addr: *const sockaddr_vm) -> bool;
    pub fn vsock_addr_unbind(addr: *mut sockaddr_vm);
    pub fn vsock_addr_equals_addr(
        addr: *const sockaddr_vm,
        other: *const sockaddr_vm,
    ) -> bool;
    pub fn vsock_addr_cast(
        addr: *const sockaddr_unsized,
        len: usize,
        out_addr: *mut *mut sockaddr_vm,
    ) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
