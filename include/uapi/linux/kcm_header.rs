/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * Kernel Connection Multiplexor
 *
 * Copyright (c) 2016 Tom Herbert <tom@herbertland.com>
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License version 2
 * as published by the Free Software Foundation.
 *
 * User API to clone KCM sockets and attach transport socket to a KCM
 * multiplexor.
 */

#[repr(C)]
pub struct kcm_attach {
    pub fd: i32,
    pub bpf_fd: i32,
}

#[repr(C)]
pub struct kcm_unattach {
    pub fd: i32,
}

#[repr(C)]
pub struct kcm_clone {
    pub fd: i32,
}

pub const SIOCKCMATTACH: i32 = SIOCPROTOPRIVATE + 0;
pub const SIOCKCMUNATTACH: i32 = SIOCPROTOPRIVATE + 1;
pub const SIOCKCMCLONE: i32 = SIOCPROTOPRIVATE + 2;

pub const KCMPROTO_CONNECTED: i32 = 0;

/* Socket options */
pub const KCM_RECV_DISABLE: i32 = 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
