/* SPDX-License-Identifier: LGPL-2.1+ WITH Linux-syscall-note */
/* Copyright (C) 2003 Krzysztof Benedyczak & Michal Wronski

   This program is free software; you can redistribute it and/or
   modify it under the terms of the GNU Lesser General Public
   License as published by the Free Software Foundation; either
   version 2.1 of the License, or (at your option) any later version.

   It is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
   Lesser General Public License for more details.  */

pub const MQ_PRIO_MAX: ::core::ffi::c_int = 32768;

/* per-uid limit of kernel memory used by mqueue, in bytes */
pub const MQ_BYTES_MAX: ::core::ffi::c_int = 819200;

#[repr(C)]
pub struct mq_attr {
    pub mq_flags: ::core::ffi::c_long,    /* message queue flags */
    pub mq_maxmsg: ::core::ffi::c_long,   /* maximum number of messages */
    pub mq_msgsize: ::core::ffi::c_long,  /* maximum message size */
    pub mq_curmsgs: ::core::ffi::c_long,  /* number of messages currently queued */
    pub __reserved: [::core::ffi::c_long; 4], /* ignored for input, zeroed for output */
}

/*
 * SIGEV_THREAD implementation:
 * SIGEV_THREAD must be implemented in user space. If SIGEV_THREAD is passed
 * to mq_notify, then
 * - sigev_signo must be the file descriptor of an AF_NETLINK socket. It's not
 *   necessary that the socket is bound.
 * - sigev_value.sival_ptr must point to a cookie that is NOTIFY_COOKIE_LEN
 *   bytes long.
 * If the notification is triggered, then the cookie is sent to the netlink
 * socket. The last byte of the cookie is replaced with the NOTIFY_?? codes:
 * NOTIFY_WOKENUP if the notification got triggered, NOTIFY_REMOVED if it was
 * removed, either due to a close() on the message queue fd or due to a
 * mq_notify() that removed the notification.
 */
pub const NOTIFY_NONE: ::core::ffi::c_int = 0;
pub const NOTIFY_WOKENUP: ::core::ffi::c_int = 1;
pub const NOTIFY_REMOVED: ::core::ffi::c_int = 2;

pub const NOTIFY_COOKIE_LEN: ::core::ffi::c_int = 32;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
