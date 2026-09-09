// SPDX-License-Identifier: GPL-2.0
/* Use watch_queue API to watch for notifications.
 *
 * Copyright (C) 2020 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

use std::ffi::c_void;
use std::mem::{size_of, MaybeUninit};
use std::os::raw::{c_char, c_int, c_long};
use std::ptr;

// Linux watch_queue and keyctl declarations are supplied by the surrounding
// environment.  These declarations preserve the corresponding C ABI.
#[repr(C)]
#[derive(Copy, Clone)]
struct watch_notification {
    type_: u32,
    subtype: u8,
    info: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct key_notification {
    watch: watch_notification,
    key_id: u32,
    aux: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct watch_notification_filter_entry {
    type_: u32,
    subtype_filter: [u32; 8],
}

#[repr(C)]
struct watch_notification_filter {
    nr_filters: u32,
    filters: [watch_notification_filter_entry; 1],
}

unsafe extern "C" {
    fn syscall(number: c_long, ...) -> c_long;
    fn read(fd: c_int, buf: *mut c_void, count: usize) -> isize;
    fn pipe2(pipefd: *mut c_int, flags: c_int) -> c_int;
    fn ioctl(fd: c_int, request: c_long, ...) -> c_int;
    fn perror(s: *const c_char);
    fn printf(format: *const c_char, ...);
    fn fprintf(stream: *mut c_void, format: *const c_char, ...);
    fn exit(status: c_int) -> !;
}

extern "C" {
    static mut stderr: *mut c_void;
}

const BUF_SIZE: c_int = 256;
const KEYCTL_WATCH_KEY: c_int = -1;
const NR_KEYCTL: c_long = -1;

// Values supplied by <linux/watch_queue.h>, <linux/unistd.h>, and
// <linux/keyctl.h> in the original source.
const WATCH_INFO_LENGTH: u32 = 0x0000_ffff;
const WATCH_INFO_ID: u32 = 0xffff_0000;
const WATCH_INFO_ID_SHIFT: u32 = 16;
const WATCH_TYPE_META: u32 = 0;
const WATCH_TYPE_KEY_NOTIFY: u32 = 1;
const WATCH_META_REMOVAL_NOTIFICATION: u8 = 0;
const WATCH_META_LOSS_NOTIFICATION: u8 = 1;
const KEY_SPEC_SESSION_KEYRING: c_int = -3;
const KEY_SPEC_USER_KEYRING: c_int = -4;
const O_NOTIFICATION_PIPE: c_int = 0x80000;
const IOC_WATCH_QUEUE_SET_SIZE: c_long = 0;
const IOC_WATCH_QUEUE_SET_FILTER: c_long = 0;

const NOTIFY_KEY_INSTANTIATED: usize = 0;
const NOTIFY_KEY_UPDATED: usize = 1;
const NOTIFY_KEY_LINKED: usize = 2;
const NOTIFY_KEY_UNLINKED: usize = 3;
const NOTIFY_KEY_CLEARED: usize = 4;
const NOTIFY_KEY_REVOKED: usize = 5;
const NOTIFY_KEY_INVALIDATED: usize = 6;
const NOTIFY_KEY_SETATTR: usize = 7;

static KEY_SUBTYPES: [&[u8]; 256] = {
    let mut a = [&[] as &[u8]; 256];
    a[NOTIFY_KEY_INSTANTIATED] = b"instantiated\0";
    a[NOTIFY_KEY_UPDATED] = b"updated\0";
    a[NOTIFY_KEY_LINKED] = b"linked\0";
    a[NOTIFY_KEY_UNLINKED] = b"unlinked\0";
    a[NOTIFY_KEY_CLEARED] = b"cleared\0";
    a[NOTIFY_KEY_REVOKED] = b"revoked\0";
    a[NOTIFY_KEY_INVALIDATED] = b"invalidated\0";
    a[NOTIFY_KEY_SETATTR] = b"setattr\0";
    a
};

unsafe fn keyctl_watch_key(key: c_int, watch_fd: c_int, watch_id: c_int) -> c_long {
    syscall(NR_KEYCTL, KEYCTL_WATCH_KEY, key, watch_fd, watch_id)
}

unsafe fn saw_key_change(n: *mut watch_notification, len: usize) {
    let k = n as *mut key_notification;
    if len != size_of::<key_notification>() {
        fprintf(stderr, b"Incorrect key message length\n\0".as_ptr() as *const c_char);
        return;
    }
    let subtype = (*n).subtype as usize;
    printf(
        b"KEY %08x change=%u[%s] aux=%u\n\0".as_ptr() as *const c_char,
        (*k).key_id, (*n).subtype, KEY_SUBTYPES[subtype].as_ptr(), (*k).aux,
    );
}

/* Consume and display events. */
unsafe fn consumer(fd: c_int) {
    let mut buffer = [0u8; 433];
    let mut n: MaybeUninit<watch_notification> = MaybeUninit::uninit();

    loop {
        let buf_len = read(fd, buffer.as_mut_ptr() as *mut c_void, buffer.len());
        if buf_len == -1 {
            perror(b"read\0".as_ptr() as *const c_char);
            exit(1);
        }
        if buf_len == 0 {
            printf(b"-- END --\n\0".as_ptr() as *const c_char);
            return;
        }
        if buf_len as usize > buffer.len() {
            fprintf(stderr, b"Read buffer overrun: %zd\n\0".as_ptr() as *const c_char, buf_len);
            return;
        }
        printf(b"read() = %zd\n\0".as_ptr() as *const c_char, buf_len);

        let mut offset = 0usize;
        while offset < buf_len as usize {
            let largest = (buf_len as usize - offset).min(128);
            if largest < size_of::<watch_notification>() {
                fprintf(stderr, b"Short message header: %zu\n\0".as_ptr() as *const c_char, largest);
                return;
            }
            ptr::copy_nonoverlapping(buffer.as_ptr().add(offset), n.as_mut_ptr() as *mut u8, largest);
            let np = n.as_mut_ptr();
            let info = (*np).info;
            printf(b"NOTIFY[%03zx]: ty=%06x sy=%02x i=%08x\n\0".as_ptr() as *const c_char,
                   offset, (*np).type_, (*np).subtype, info);
            let len = (info & WATCH_INFO_LENGTH) as usize;
            if len < size_of::<watch_notification>() || len > largest {
                fprintf(stderr, b"Bad message length: %zu/%zu\n\0".as_ptr() as *const c_char, len, largest);
                exit(1);
            }
            match (*np).type_ {
                WATCH_TYPE_META => match (*np).subtype {
                    WATCH_META_REMOVAL_NOTIFICATION => printf(b"REMOVAL of watchpoint %08x\n\0".as_ptr() as *const c_char, (info & WATCH_INFO_ID) >> WATCH_INFO_ID_SHIFT),
                    WATCH_META_LOSS_NOTIFICATION => printf(b"-- LOSS --\n\0".as_ptr() as *const c_char),
                    _ => printf(b"other meta record\n\0".as_ptr() as *const c_char),
                },
                WATCH_TYPE_KEY_NOTIFY => saw_key_change(np, len),
                _ => printf(b"other type\n\0".as_ptr() as *const c_char),
            }
            offset += len;
        }
    }
}

static mut FILTER: watch_notification_filter = watch_notification_filter {
    nr_filters: 1,
    filters: [watch_notification_filter_entry { type_: WATCH_TYPE_KEY_NOTIFY, subtype_filter: [u32::MAX; 8] }],
};

fn main() {
    unsafe {
        let mut pipefd = [0i32; 2];
        if pipe2(pipefd.as_mut_ptr(), O_NOTIFICATION_PIPE) == -1 { perror(b"pipe2\0".as_ptr() as *const c_char); exit(1); }
        let fd = pipefd[0];
        if ioctl(fd, IOC_WATCH_QUEUE_SET_SIZE, BUF_SIZE) == -1 { perror(b"watch_queue(size)\0".as_ptr() as *const c_char); exit(1); }
        if ioctl(fd, IOC_WATCH_QUEUE_SET_FILTER, &raw mut FILTER) == -1 { perror(b"watch_queue(filter)\0".as_ptr() as *const c_char); exit(1); }
        if keyctl_watch_key(KEY_SPEC_SESSION_KEYRING, fd, 0x01) == -1 { perror(b"keyctl\0".as_ptr() as *const c_char); exit(1); }
        if keyctl_watch_key(KEY_SPEC_USER_KEYRING, fd, 0x02) == -1 { perror(b"keyctl\0".as_ptr() as *const c_char); exit(1); }
        consumer(fd);
        exit(0);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
