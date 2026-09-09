/* SPDX-License-Identifier: ((GPL-2.0 WITH Linux-syscall-note) OR MIT) */
/******************************************************************************
 * evtchn.h
 *
 * Interface to /dev/xen/evtchn.
 *
 * Copyright (c) 2003-2005, K A Fraser
 *
 * This program is free software; you can redistribute it and/or
 * modify it under the terms of the GNU General Public License version 2
 * as published by the Free Software Foundation; or, when distributed
 * separately from the Linux kernel or incorporated into other
 * software packages, subject to the following license:
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this source file (the "Software"), to deal in the Software without
 * restriction, including without limitation the rights to use, copy, modify,
 * merge, publish, distribute, sublicense, and/or sell copies of the Software,
 * and to permit persons to whom the Software is furnished to do so, subject to
 * the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
 * FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS
 * IN THE SOFTWARE.
 */

/* C header guard: __LINUX_PUBLIC_EVTCHN_H__ */

/* Bind a fresh port to VIRQ @virq. Return allocated port. */
#[repr(C)]
pub struct ioctl_evtchn_bind_virq {
    pub virq: core::ffi::c_uint,
}

pub const IOCTL_EVTCHN_BIND_VIRQ: _IOC =
    _IOC(_IOC_NONE, b'E' as _, 0, core::mem::size_of::<ioctl_evtchn_bind_virq>());

/* Bind a fresh port to remote <@remote_domain, @remote_port>. Return allocated port. */
#[repr(C)]
pub struct ioctl_evtchn_bind_interdomain {
    pub remote_domain: core::ffi::c_uint,
    pub remote_port: core::ffi::c_uint,
}

pub const IOCTL_EVTCHN_BIND_INTERDOMAIN: _IOC =
    _IOC(_IOC_NONE, b'E' as _, 1, core::mem::size_of::<ioctl_evtchn_bind_interdomain>());

/* Allocate a fresh port for binding to @remote_domain. Return allocated port. */
#[repr(C)]
pub struct ioctl_evtchn_bind_unbound_port {
    pub remote_domain: core::ffi::c_uint,
}

pub const IOCTL_EVTCHN_BIND_UNBOUND_PORT: _IOC =
    _IOC(_IOC_NONE, b'E' as _, 2, core::mem::size_of::<ioctl_evtchn_bind_unbound_port>());

/* Unbind previously allocated @port. */
#[repr(C)]
pub struct ioctl_evtchn_unbind {
    pub port: core::ffi::c_uint,
}

pub const IOCTL_EVTCHN_UNBIND: _IOC =
    _IOC(_IOC_NONE, b'E' as _, 3, core::mem::size_of::<ioctl_evtchn_unbind>());

/* Unbind previously allocated @port. */
#[repr(C)]
pub struct ioctl_evtchn_notify {
    pub port: core::ffi::c_uint,
}

pub const IOCTL_EVTCHN_NOTIFY: _IOC =
    _IOC(_IOC_NONE, b'E' as _, 4, core::mem::size_of::<ioctl_evtchn_notify>());

/* Clear and reinitialise the event buffer. Clear error condition. */
pub const IOCTL_EVTCHN_RESET: _IOC = _IOC(_IOC_NONE, b'E' as _, 5, 0);

/*
 * Restrict this file descriptor so that it can only be used to bind
 * new interdomain events from one domain.
 *
 * Once a file descriptor has been restricted it cannot be
 * de-restricted, and must be closed and re-opened. Event channels
 * which were bound before restricting remain bound afterwards, and
 * can be notified as usual.
 */
#[repr(C)]
pub struct ioctl_evtchn_restrict_domid {
    pub domid: domid_t,
}

pub const IOCTL_EVTCHN_RESTRICT_DOMID: _IOC =
    _IOC(_IOC_NONE, b'E' as _, 6, core::mem::size_of::<ioctl_evtchn_restrict_domid>());

/* Bind statically allocated @port. */
#[repr(C)]
pub struct ioctl_evtchn_bind {
    pub port: core::ffi::c_uint,
}

pub const IOCTL_EVTCHN_BIND_STATIC: _IOC =
    _IOC(_IOC_NONE, b'E' as _, 7, core::mem::size_of::<ioctl_evtchn_bind>());

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
