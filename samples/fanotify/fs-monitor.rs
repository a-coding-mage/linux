// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright 2021, Collabora Ltd.
 */

use std::ffi::c_char;
use std::mem::{size_of, transmute};
use std::ptr;

const FAN_FS_ERROR: u64 = 0x0000_8000;
const FAN_EVENT_INFO_TYPE_ERROR: u8 = 5;
const FILEID_INO32_GEN: i32 = 1;
const FILEID_INVALID: i32 = 0xff;
const FAN_CLASS_NOTIF: u32 = 0x0000_0000;
const FAN_REPORT_FID: u32 = 0x0000_0200;
const FAN_MARK_ADD: u32 = 0x0000_0001;
const FAN_MARK_FILESYSTEM: u32 = 0x0000_0100;
const O_RDONLY: i32 = 0;
const AT_FDCWD: i32 = -100;
const FAN_NOFD: i32 = -1;
const BUFSIZ: usize = 8192;

#[repr(C)]
struct fanotify_event_metadata {
    event_len: u32,
    vers: u8,
    reserved: u8,
    metadata_len: u16,
    mask: u64,
    fd: i32,
    pid: i32,
}

#[repr(C)]
struct fanotify_event_info_header {
    len: u8,
    info_type: u8,
}

#[repr(C)]
struct file_handle {
    handle_bytes: u32,
    handle_type: i32,
    f_handle: [u8; 0],
}

#[repr(C)]
struct fsid_t {
    val: [i32; 2],
}

#[repr(C)]
struct fanotify_event_info_fid {
    hdr: fanotify_event_info_header,
    fsid: fsid_t,
    handle: [u8; 0],
}

#[repr(C)]
struct fanotify_event_info_error {
    hdr: fanotify_event_info_header,
    error: i32,
    error_count: u32,
}

extern "C" {
    fn printf(format: *const c_char, ...) -> i32;
    fn errx(status: i32, format: *const c_char, ... ) -> !;
    fn fanotify_init(flags: u32, event_f_flags: i32) -> i32;
    fn fanotify_mark(fd: i32, flags: u32, mask: u64, dirfd: i32, path: *const c_char) -> i32;
    fn read(fd: i32, buf: *mut u8, count: usize) -> isize;
}

unsafe fn print_fh(fh: *const file_handle) {
    let h = (*fh).f_handle.as_ptr() as *const u32;

    printf(b"\tfh: \0".as_ptr() as *const c_char);
    for i in 0..(*fh).handle_bytes {
        printf(b"%hhx\0".as_ptr() as *const c_char, *(*fh).f_handle.as_ptr().add(i as usize));
    }
    printf(b"\n\0".as_ptr() as *const c_char);

    printf(b"\tdecoded fh: \0".as_ptr() as *const c_char);
    if (*fh).handle_type == FILEID_INO32_GEN {
        printf(b"inode=%u gen=%u\n\0".as_ptr() as *const c_char, *h, *h.add(1));
    } else if (*fh).handle_type == FILEID_INVALID && (*fh).handle_bytes == 0 {
        printf(b"Type %d (Superblock error)\n\0".as_ptr() as *const c_char, (*fh).handle_type);
    } else {
        printf(b"Type %d (Unknown)\n\0".as_ptr() as *const c_char, (*fh).handle_type);
    }
}

unsafe fn handle_notifications(buffer: *mut c_char, len: i32) {
    let mut event = buffer as *mut fanotify_event_metadata;
    let mut remaining = len;

    while (*event).event_len >= size_of::<fanotify_event_metadata>() as u32
        && (*event).event_len as i32 <= remaining
    {
        if (*event).mask != FAN_FS_ERROR {
            printf(b"unexpected FAN MARK: %llx\n\0".as_ptr() as *const c_char, (*event).mask);
        } else if (*event).fd != FAN_NOFD {
            printf(b"Unexpected fd (!= FAN_NOFD)\n\0".as_ptr() as *const c_char);
        } else {
            printf(b"FAN_FS_ERROR (len=%d)\n\0".as_ptr() as *const c_char, (*event).event_len);
            let mut off = size_of::<fanotify_event_metadata>();
            while off < (*event).event_len as usize {
                let info = (event as *mut u8).add(off) as *mut fanotify_event_info_header;
                match (*info).info_type {
                    FAN_EVENT_INFO_TYPE_ERROR => {
                        let err = info as *mut fanotify_event_info_error;
                        printf(b"\tGeneric Error Record: len=%d\n\0".as_ptr() as *const c_char, (*err).hdr.len);
                        printf(b"\terror: %d\n\0".as_ptr() as *const c_char, (*err).error);
                        printf(b"\terror_count: %d\n\0".as_ptr() as *const c_char, (*err).error_count);
                    }
                    2 => {
                        let fid = info as *mut fanotify_event_info_fid;
                        printf(b"\tfsid: %x%x\n\0".as_ptr() as *const c_char, (*fid).fsid.val[0], (*fid).fsid.val[1]);
                        print_fh((ptr::addr_of!((*fid).handle)) as *const file_handle);
                    }
                    _ => printf(b"\tUnknown info type=%d len=%d:\n\0".as_ptr() as *const c_char, (*info).info_type, (*info).len),
                }
                off += (*info).len as usize;
            }
        }
        printf(b"---\n\n\0".as_ptr() as *const c_char);
        remaining -= (*event).event_len as i32;
        event = (event as *mut u8).add((*event).event_len as usize) as *mut fanotify_event_metadata;
    }
}

fn main() {
    unsafe {
        let args: Vec<*mut c_char> = std::env::args_os().map(|_| ptr::null_mut()).collect();
        let _ = args;
        // The original C entry point receives argc/argv from the runtime.
        // Build integrations provide the corresponding argument vector.
        let argc = std::env::args_os().count();
        if argc < 2 {
            printf(b"Missing path argument\n\0".as_ptr() as *const c_char);
            return;
        }
        let fd = fanotify_init(FAN_CLASS_NOTIF | FAN_REPORT_FID, O_RDONLY);
        if fd < 0 { errx(1, b"fanotify_init\0".as_ptr() as *const c_char); }
        let path = std::env::args_os().nth(1).unwrap();
        let path = std::ffi::CString::new(path.as_encoded_bytes()).unwrap();
        if fanotify_mark(fd, FAN_MARK_ADD | FAN_MARK_FILESYSTEM, FAN_FS_ERROR, AT_FDCWD, path.as_ptr()) != 0 {
            errx(1, b"fanotify_mark\0".as_ptr() as *const c_char);
        }
        let mut buffer = [0u8; BUFSIZ];
        loop {
            let n = read(fd, buffer.as_mut_ptr(), BUFSIZ);
            if n < 0 { errx(1, b"read\0".as_ptr() as *const c_char); }
            handle_notifications(buffer.as_mut_ptr() as *mut c_char, n as i32);
        }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
