/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 *  include/linux/eventpoll.h ( Efficient event polling implementation )
 *  Copyright (C) 2001,...,2006  Davide Libenzi
 *
 *  Davide Libenzi <davidel@xmailserver.org>
 */

/* Dependencies supplied by the corresponding uapi and kernel translation units. */

#[repr(C)]
pub struct file {
    _private: [u8; 0],
}

#[repr(C)]
pub struct epoll_event {
    pub events: u32,
    pub data: u64,
}

pub type __poll_t = u32;
pub type __u64 = u64;

#[cfg(feature = "CONFIG_KCMP")]
extern "C" {
    pub fn get_epoll_tfile_raw_ptr(
        file: *mut file,
        tfd: core::ffi::c_int,
        toff: core::ffi::c_ulong,
    ) -> *mut file;
}

#[cfg(feature = "CONFIG_EPOLL")]
extern "C" {
    pub fn eventpoll_release_file(file: *mut file);
    pub fn epoll_sendevents(
        file: *mut file,
        events: *mut epoll_event,
        maxevents: core::ffi::c_int,
    ) -> core::ffi::c_int;
    pub fn do_epoll_ctl_file(
        f: *mut file,
        op: core::ffi::c_int,
        tf: *mut epoll_key,
        epds: *mut epoll_event,
        nonblock: bool,
    ) -> core::ffi::c_int;
    pub fn do_epoll_ctl(
        epfd: core::ffi::c_int,
        op: core::ffi::c_int,
        fd: core::ffi::c_int,
        epds: *mut epoll_event,
        nonblock: bool,
    ) -> core::ffi::c_int;
    pub fn is_file_epoll(f: *mut file) -> bool;
}

#[repr(C, packed)]
pub struct epoll_key {
    pub file: *mut file,
    pub fd: core::ffi::c_int,
}

#[cfg(feature = "CONFIG_EPOLL")]
pub unsafe fn eventpoll_release(file: *mut file) {
    /* The f_ep fast-path check depends on the complete kernel `struct file`,
     * supplied by the including translation unit. */
    if file.is_null() {
        return;
    }
    /* TODO: translate READ_ONCE(file->f_ep) once the external file layout is available. */
    eventpoll_release_file(file);
}

#[cfg(not(feature = "CONFIG_EPOLL"))]
pub unsafe fn eventpoll_release(_file: *mut file) {}

#[cfg(feature = "CONFIG_EPOLL")]
#[inline]
pub const fn ep_op_has_event(op: core::ffi::c_int) -> bool {
    op != EPOLL_CTL_DEL
}

/* Supplied by uapi/linux/eventpoll.h. */
pub const EPOLL_CTL_DEL: core::ffi::c_int = 2;

#[cfg(all(target_arch = "arm", feature = "CONFIG_OABI_COMPAT"))]
extern "C" {
    pub fn epoll_put_uevent(
        revents: __poll_t,
        data: __u64,
        uevent: *mut epoll_event,
    ) -> *mut epoll_event;
}

#[cfg(not(all(target_arch = "arm", feature = "CONFIG_OABI_COMPAT")))]
#[inline]
pub unsafe fn epoll_put_uevent(
    revents: __poll_t,
    data: __u64,
    uevent: *mut epoll_event,
) -> *mut epoll_event {
    if uevent.is_null() {
        return core::ptr::null_mut();
    }
    (*uevent).events = revents;
    (*uevent).data = data;
    uevent.add(1)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
