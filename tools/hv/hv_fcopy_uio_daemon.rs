// SPDX-License-Identifier: GPL-2.0-only
/*
 * An implementation of host to guest copy functionality for Linux.
 *
 * Copyright (C) 2023, Microsoft, Inc.
 *
 * Author : K. Y. Srinivasan <kys@microsoft.com>
 * Author : Saurabh Sengar <ssengar@microsoft.com>
 *
 */

// C dependencies: dirent.h, errno.h, fcntl.h, getopt.h, locale.h, stdbool.h,
// stddef.h, stdint.h, stdio.h, stdlib.h, string.h, syslog.h, unistd.h,
// wchar.h, sys/stat.h, linux/hyperv.h, linux/limits.h, vmbus_bufring.h.

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulonglong, c_void};
use core::mem::size_of;
use core::ptr;

const ICMSGTYPE_NEGOTIATE: c_int = 0;
const ICMSGTYPE_FCOPY: c_int = 7;

const WIN8_SRV_MAJOR: c_int = 1;
const WIN8_SRV_MINOR: c_int = 1;
const WIN8_SRV_VERSION: c_int = (WIN8_SRV_MAJOR << 16) | WIN8_SRV_MINOR;

const FCOPY_UIO_PATH: *const c_char =
    b"/sys/bus/vmbus/devices/eb765408-105f-49b6-b4aa-c123b64d17d4/uio\0".as_ptr()
        as *const c_char;
const FCOPY_CHANNELS_PATH: *const c_char =
    b"/sys/bus/vmbus/devices/eb765408-105f-49b6-b4aa-c123b64d17d4/channels\0".as_ptr()
        as *const c_char;

const FCOPY_VER_COUNT: c_int = 1;
static fcopy_versions: [c_int; 1] = [WIN8_SRV_VERSION];

const FW_VER_COUNT: c_int = 1;
static fw_versions: [c_int; 1] = [UTIL_FW_VERSION];

type size_t = usize;
type ssize_t = isize;
type off_t = i64;
type __u16 = u16;
type __u32 = u32;

// External constants, types, and functions supplied by libc, linux/hyperv.h,
// linux/limits.h, and vmbus_bufring.h.
const PATH_MAX: usize = 4096;
const NAME_MAX: usize = 255;
const DT_DIR: u8 = 4;
const F_OK: c_int = 0;
const O_RDWR: c_int = 0o2;
const O_CREAT: c_int = 0o100;
const O_TRUNC: c_int = 0o1000;
const O_CLOEXEC: c_int = 0o2000000;
const LC_ALL: c_int = 6;
const LOG_ERR: c_int = 3;
const LOG_INFO: c_int = 6;
const LOG_USER: c_int = 1 << 3;
const ENOSPC: c_int = 28;
const ENOBUFS: c_int = 105;
const EINVAL: c_int = 22;
const ENODEV: c_int = 19;
const ENOMEM: c_int = 12;
const EINTR: c_int = 4;
const EAGAIN: c_int = 11;
const HV_E_FAIL: c_int = -2147467259i32;
const HV_ERROR_ALREADY_EXISTS: c_int = -2147024713i32;
const HV_ERROR_DISK_FULL: c_int = -2147024784i32;
const CREATE_PATH: __u32 = 1;
const OVER_WRITE: __u32 = 2;
const START_FILE_COPY: c_int = 0;
const WRITE_TO_FILE: c_int = 1;
const COMPLETE_FCOPY: c_int = 2;
const UTIL_FW_VERSION: c_int = 0;
const ICMSG_HDR: usize = 16;
const IC_VERSION_NEGOTIATION_MAX_VER_COUNT: c_int = 100;
const ICMSGHDRFLAG_TRANSACTION: u8 = 1;
const ICMSGHDRFLAG_RESPONSE: u8 = 2;
const W_MAX_PATH: usize = 260;

#[repr(C)]
struct DIR {
    _private: [u8; 0],
}

#[repr(C)]
struct dirent {
    d_ino: u64,
    d_off: i64,
    d_reclen: u16,
    d_type: u8,
    d_name: [c_char; 256],
}

#[repr(C)]
struct stat {
    _prefix: [u8; 48],
    st_size: off_t,
}

#[repr(C)]
struct option {
    name: *const c_char,
    has_arg: c_int,
    flag: *mut c_int,
    val: c_int,
}

const no_argument: c_int = 0;

#[repr(C)]
struct ic_version {
    major: c_int,
    minor: c_int,
}

#[repr(C)]
struct icmsg_negotiate {
    icframe_vercnt: c_int,
    icmsg_vercnt: c_int,
    reserved: [c_int; 2],
    icversion_data: [ic_version; IC_VERSION_NEGOTIATION_MAX_VER_COUNT as usize * 2],
}

#[repr(C)]
struct icmsg_hdr {
    icmsgtype: u16,
    icmsgsize: u16,
    status: c_int,
    ictransaction_id: u8,
    icflags: u8,
    reserved: [u8; 6],
}

#[repr(C)]
struct hv_fcopy_hdr {
    operation: c_int,
}

#[repr(C)]
struct hv_start_fcopy {
    hdr: hv_fcopy_hdr,
    file_name: [__u16; W_MAX_PATH],
    path_name: [__u16; W_MAX_PATH],
    copy_flags: __u32,
}

#[repr(C)]
struct hv_do_fcopy {
    hdr: hv_fcopy_hdr,
    offset: u64,
    size: u32,
    data: [u8; 0],
}

#[repr(C)]
struct vmbus_chanpkt_hdr {
    _type: u16,
    hlen: u16,
    tlen: u16,
    flags: u16,
    xactid: u64,
}

#[repr(C)]
struct vmbuspipe_hdr {
    flags: u32,
    msgsize: u32,
}

#[repr(C)]
struct vmbus_bufring {
    imask: u32,
}

#[repr(C)]
struct vmbus_br {
    vbr: *mut vmbus_bufring,
}

unsafe extern "C" {
    static mut errno: c_int;

    fn opendir(name: *const c_char) -> *mut DIR;
    fn readdir(dirp: *mut DIR) -> *mut dirent;
    fn rewinddir(dirp: *mut DIR);
    fn closedir(dirp: *mut DIR) -> c_int;
    fn stat(pathname: *const c_char, statbuf: *mut stat) -> c_int;
    fn usleep(usec: c_uint) -> c_int;
    fn strerror(errnum: c_int) -> *mut c_char;
    fn syslog(priority: c_int, format: *const c_char, ...);
    fn snprintf(str_: *mut c_char, size: size_t, format: *const c_char, ...) -> c_int;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn access(pathname: *const c_char, mode: c_int) -> c_int;
    fn mkdir(pathname: *const c_char, mode: u32) -> c_int;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn pwrite(fd: c_int, buf: *const c_void, count: size_t, offset: off_t) -> ssize_t;
    fn close(fd: c_int) -> c_int;
    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    static mut stderr: *mut c_void;
    fn setlocale(category: c_int, locale: *const c_char) -> *mut c_char;
    fn getopt_long(
        argc: c_int,
        argv: *const *mut c_char,
        optstring: *const c_char,
        longopts: *const option,
        longindex: *mut c_int,
    ) -> c_int;
    fn daemon(nochdir: c_int, noclose: c_int) -> c_int;
    fn openlog(ident: *const c_char, option: c_int, facility: c_int);
    fn getpid() -> c_int;
    fn malloc(size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn strcpy(dest: *mut c_char, src: *const c_char) -> *mut c_char;
    fn pread(fd: c_int, buf: *mut c_void, count: size_t, offset: off_t) -> ssize_t;
    fn write(fd: c_int, buf: *const c_void, count: size_t) -> ssize_t;
    fn vmbus_uio_map(fd: *mut c_int, size: u32) -> *mut c_void;
    fn vmbus_br_setup(br: *mut vmbus_br, buf: *mut c_void, blen: u32);
    fn rte_vmbus_chan_recv_raw(rxbr: *mut vmbus_br, data: *mut c_void, len: *mut u32) -> c_int;
    fn rte_vmbus_chan_send(
        txbr: *mut vmbus_br,
        typ: u16,
        data: *mut c_void,
        len: u32,
        xactid: u64,
    ) -> c_int;
}

fn ICMSG_NEGOTIATE_PKT_SIZE(frame: c_int, msg: c_int) -> c_uint {
    (ICMSG_HDR + size_of::<icmsg_negotiate>()
        - size_of::<[ic_version; IC_VERSION_NEGOTIATION_MAX_VER_COUNT as usize * 2]>()
        + ((frame + msg) as usize * size_of::<ic_version>())) as c_uint
}

fn offsetof_icmsg_negotiate_reserved() -> usize {
    size_of::<c_int>() * 2
}

fn unlikely<T>(x: T) -> T {
    x
}

unsafe fn get_ring_buffer_size() -> u32 {
    let mut ring_path = [0 as c_char; PATH_MAX];
    let mut st: stat = core::mem::zeroed();
    let mut ring_size: u32 = 0;
    let mut retry_count: c_int = 0;

    /* Find the channel directory */
    let mut dir = opendir(FCOPY_CHANNELS_PATH);
    if dir.is_null() {
        usleep(100 * 1000); /* Avoid race with kernel, wait 100ms and retry once */
        dir = opendir(FCOPY_CHANNELS_PATH);
        if dir.is_null() {
            syslog(
                LOG_ERR,
                b"Failed to open channels directory: %s\0".as_ptr() as *const c_char,
                strerror(errno),
            );
            return 0;
        }
    }

    loop {
        loop {
            let entry = readdir(dir);
            if entry.is_null() {
                break;
            }
            if (*entry).d_type == DT_DIR
                && strcmp((*entry).d_name.as_ptr(), b".\0".as_ptr() as *const c_char) != 0
                && strcmp((*entry).d_name.as_ptr(), b"..\0".as_ptr() as *const c_char) != 0
            {
                snprintf(
                    ring_path.as_mut_ptr(),
                    ring_path.len(),
                    b"%s/%s/ring\0".as_ptr() as *const c_char,
                    FCOPY_CHANNELS_PATH,
                    (*entry).d_name.as_ptr(),
                );

                if stat(ring_path.as_ptr(), &mut st) == 0 {
                    /*
                     * stat returns size of Tx, Rx rings combined,
                     * so take half of it for individual ring size.
                     */
                    ring_size = (st.st_size as u32) / 2;
                    syslog(
                        LOG_INFO,
                        b"Ring buffer size from %s: %u bytes\0".as_ptr() as *const c_char,
                        ring_path.as_ptr(),
                        ring_size,
                    );
                    break;
                }
            }
        }

        if ring_size == 0 && retry_count == 0 {
            retry_count = 1;
            rewinddir(dir);
            usleep(100 * 1000); /* Wait 100ms and retry once */
            continue;
        }
        break;
    }

    closedir(dir);

    if ring_size == 0 {
        syslog(LOG_ERR, b"Could not determine ring size\0".as_ptr() as *const c_char);
    }

    ring_size
}

static mut desc: *mut u8 = ptr::null_mut();

static mut target_fd: c_int = 0;
static mut target_fname: [c_char; PATH_MAX] = [0; PATH_MAX];
static mut filesize: c_ulonglong = 0;

unsafe fn hv_fcopy_create_file(file_name: *mut c_char, path_name: *mut c_char, flags: __u32) -> c_int {
    let mut error: c_int = HV_E_FAIL;
    let mut q: *mut c_char;
    let mut p: *mut c_char;

    filesize = 0;
    p = path_name;
    if snprintf(
        target_fname.as_mut_ptr(),
        target_fname.len(),
        b"%s/%s\0".as_ptr() as *const c_char,
        path_name,
        file_name,
    ) as usize
        >= target_fname.len()
    {
        syslog(
            LOG_ERR,
            b"target file name is too long: %s/%s\0".as_ptr() as *const c_char,
            path_name,
            file_name,
        );
        if error != 0 {
            target_fname[0] = 0;
        }
        return error;
    }

    /*
     * Check to see if the path is already in place; if not,
     * create if required.
     */
    loop {
        q = strchr(p, b'/' as c_int);
        if q.is_null() {
            break;
        }
        if q == p {
            p = p.add(1);
            continue;
        }
        *q = 0;
        if access(path_name, F_OK) != 0 {
            if flags & CREATE_PATH != 0 {
                if mkdir(path_name, 0o755) != 0 {
                    syslog(LOG_ERR, b"Failed to create %s\0".as_ptr() as *const c_char, path_name);
                    if error != 0 {
                        target_fname[0] = 0;
                    }
                    return error;
                }
            } else {
                syslog(LOG_ERR, b"Invalid path: %s\0".as_ptr() as *const c_char, path_name);
                if error != 0 {
                    target_fname[0] = 0;
                }
                return error;
            }
        }
        p = q.add(1);
        *q = b'/' as c_char;
    }

    if access(target_fname.as_ptr(), F_OK) == 0 {
        syslog(LOG_INFO, b"File: %s exists\0".as_ptr() as *const c_char, target_fname.as_ptr());
        if flags & OVER_WRITE == 0 {
            error = HV_ERROR_ALREADY_EXISTS;
            if error != 0 {
                target_fname[0] = 0;
            }
            return error;
        }
    }

    target_fd = open(
        target_fname.as_ptr(),
        O_RDWR | O_CREAT | O_TRUNC | O_CLOEXEC,
        0o744,
    );
    if target_fd == -1 {
        syslog(LOG_INFO, b"Open Failed: %s\0".as_ptr() as *const c_char, strerror(errno));
        if error != 0 {
            target_fname[0] = 0;
        }
        return error;
    }

    error = 0;
    if error != 0 {
        target_fname[0] = 0;
    }
    error
}

/* copy the data into the file */
unsafe fn hv_copy_data(cpmsg: *mut hv_do_fcopy) -> c_int {
    let len: ssize_t;
    let mut ret: c_int = 0;

    len = pwrite(
        target_fd,
        (*cpmsg).data.as_ptr() as *const c_void,
        (*cpmsg).size as size_t,
        (*cpmsg).offset as off_t,
    );

    filesize = filesize.wrapping_add((*cpmsg).size as c_ulonglong);
    if len != (*cpmsg).size as ssize_t {
        match errno {
            ENOSPC => {
                ret = HV_ERROR_DISK_FULL;
            }
            _ => {
                ret = HV_E_FAIL;
            }
        }
        syslog(
            LOG_ERR,
            b"pwrite failed to write %llu bytes: %ld (%s)\0".as_ptr() as *const c_char,
            filesize,
            len as c_long,
            strerror(errno),
        );
    }

    ret
}

unsafe fn hv_copy_finished() -> c_int {
    close(target_fd);
    target_fname[0] = 0;

    0
}

unsafe fn print_usage(argv: *mut *mut c_char) {
    fprintf(
        stderr,
        b"Usage: %s [options]\nOptions are:\n  -n, --no-daemon        stay in foreground, don't daemonize\n  -h, --help             print this help\n\0"
            .as_ptr() as *const c_char,
        *argv,
    );
}

unsafe fn vmbus_prep_negotiate_resp(
    icmsghdrp: *mut icmsg_hdr,
    buf: *mut u8,
    buflen: c_uint,
    fw_version: *const c_int,
    fw_vercnt: c_int,
    srv_version: *const c_int,
    srv_vercnt: c_int,
    nego_fw_version: *mut c_int,
    nego_srv_version: *mut c_int,
) -> bool {
    let mut icframe_major: c_int;
    let mut icframe_minor: c_int;
    let mut icmsg_major: c_int;
    let mut icmsg_minor: c_int;
    let mut fw_major: c_int;
    let mut fw_minor: c_int;
    let mut srv_major: c_int;
    let mut srv_minor: c_int;
    let mut i: c_int;
    let mut j: c_int;
    let mut found_match = false;
    let negop: *mut icmsg_negotiate;

    /* Check that there's enough space for icframe_vercnt, icmsg_vercnt */
    if (buflen as usize) < ICMSG_HDR + offsetof_icmsg_negotiate_reserved() {
        syslog(LOG_ERR, b"Invalid icmsg negotiate\0".as_ptr() as *const c_char);
        return false;
    }

    (*icmsghdrp).icmsgsize = 0x10;
    negop = buf.add(ICMSG_HDR) as *mut icmsg_negotiate;

    icframe_major = (*negop).icframe_vercnt;
    icframe_minor = 0;

    icmsg_major = (*negop).icmsg_vercnt;
    icmsg_minor = 0;

    /* Validate negop packet */
    if icframe_major > IC_VERSION_NEGOTIATION_MAX_VER_COUNT
        || icmsg_major > IC_VERSION_NEGOTIATION_MAX_VER_COUNT
        || ICMSG_NEGOTIATE_PKT_SIZE(icframe_major, icmsg_major) > buflen
    {
        syslog(
            LOG_ERR,
            b"Invalid icmsg negotiate - icframe_major: %u, icmsg_major: %u\n\0".as_ptr()
                as *const c_char,
            icframe_major,
            icmsg_major,
        );
    } else {
        /*
         * Select the framework version number we will
         * support.
         */

        i = 0;
        while i < fw_vercnt {
            fw_major = *fw_version.add(i as usize) >> 16;
            fw_minor = *fw_version.add(i as usize) & 0xFFFF;

            j = 0;
            while j < (*negop).icframe_vercnt {
                if (*negop).icversion_data[j as usize].major == fw_major
                    && (*negop).icversion_data[j as usize].minor == fw_minor
                {
                    icframe_major = (*negop).icversion_data[j as usize].major;
                    icframe_minor = (*negop).icversion_data[j as usize].minor;
                    found_match = true;
                    break;
                }
                j += 1;
            }

            if found_match {
                break;
            }
            i += 1;
        }

        if found_match {
            found_match = false;

            i = 0;
            while i < srv_vercnt {
                srv_major = *srv_version.add(i as usize) >> 16;
                srv_minor = *srv_version.add(i as usize) & 0xFFFF;

                j = (*negop).icframe_vercnt;
                while j < (*negop).icframe_vercnt + (*negop).icmsg_vercnt {
                    if (*negop).icversion_data[j as usize].major == srv_major
                        && (*negop).icversion_data[j as usize].minor == srv_minor
                    {
                        icmsg_major = (*negop).icversion_data[j as usize].major;
                        icmsg_minor = (*negop).icversion_data[j as usize].minor;
                        found_match = true;
                        break;
                    }
                    j += 1;
                }

                if found_match {
                    break;
                }
                i += 1;
            }
        }
    }

    /*
     * Respond with the framework and service
     * version numbers we can support.
     */
    if !found_match {
        (*negop).icframe_vercnt = 0;
        (*negop).icmsg_vercnt = 0;
    } else {
        (*negop).icframe_vercnt = 1;
        (*negop).icmsg_vercnt = 1;
    }

    if !nego_fw_version.is_null() {
        *nego_fw_version = (icframe_major << 16) | icframe_minor;
    }

    if !nego_srv_version.is_null() {
        *nego_srv_version = (icmsg_major << 16) | icmsg_minor;
    }

    (*negop).icversion_data[0].major = icframe_major;
    (*negop).icversion_data[0].minor = icframe_minor;
    (*negop).icversion_data[1].major = icmsg_major;
    (*negop).icversion_data[1].minor = icmsg_minor;

    found_match
}

unsafe fn wcstoutf8(dest: *mut c_char, mut src: *const __u16, dest_size: size_t) {
    let mut len: size_t = 0;

    while len < dest_size && *src != 0 {
        if *src < 0x80 {
            *dest.add(len) = *src as c_char;
            len += 1;
            src = src.add(1);
        } else {
            *dest.add(len) = b'X' as c_char;
            len += 1;
        }
    }

    *dest.add(len) = 0;
}

unsafe fn hv_fcopy_start(smsg_in: *mut hv_start_fcopy) -> c_int {
    /*
     * file_name and path_name should have same length with appropriate
     * member of hv_start_fcopy.
     */
    let mut file_name = [0 as c_char; W_MAX_PATH];
    let mut path_name = [0 as c_char; W_MAX_PATH];

    setlocale(LC_ALL, b"en_US.utf8\0".as_ptr() as *const c_char);
    wcstoutf8(file_name.as_mut_ptr(), (*smsg_in).file_name.as_ptr(), W_MAX_PATH - 1);
    wcstoutf8(path_name.as_mut_ptr(), (*smsg_in).path_name.as_ptr(), W_MAX_PATH - 1);

    hv_fcopy_create_file(file_name.as_mut_ptr(), path_name.as_mut_ptr(), (*smsg_in).copy_flags)
}

unsafe fn hv_fcopy_send_data(fcopy_msg: *mut hv_fcopy_hdr, _recvlen: c_int) -> c_int {
    let operation: c_int = (*fcopy_msg).operation;

    /*
     * The  strings sent from the host are encoded in
     * utf16; convert it to utf8 strings.
     * The host assures us that the utf16 strings will not exceed
     * the max lengths specified. We will however, reserve room
     * for the string terminating character - in the utf16s_utf8s()
     * function we limit the size of the buffer where the converted
     * string is placed to W_MAX_PATH -1 to guarantee
     * that the strings can be properly terminated!
     */

    match operation {
        START_FILE_COPY => hv_fcopy_start(fcopy_msg as *mut hv_start_fcopy),
        WRITE_TO_FILE => hv_copy_data(fcopy_msg as *mut hv_do_fcopy),
        COMPLETE_FCOPY => hv_copy_finished(),
        _ => HV_E_FAIL,
    }
}

/* process the packet recv from host */
unsafe fn fcopy_pkt_process(txbr: *mut vmbus_br) -> c_int {
    let mut ret: c_int;
    let offset: c_int;
    let pktlen: c_int;
    let mut fcopy_srv_version: c_int = 0;
    let pkt: *const vmbus_chanpkt_hdr;
    let fcopy_msg: *mut hv_fcopy_hdr;
    let icmsghdr: *mut icmsg_hdr;

    pkt = desc as *const vmbus_chanpkt_hdr;
    offset = ((*pkt).hlen as c_int) << 3;
    pktlen = (((*pkt).tlen as c_int) << 3) - offset;
    icmsghdr = desc.add(offset as usize + size_of::<vmbuspipe_hdr>()) as *mut icmsg_hdr;
    (*icmsghdr).status = HV_E_FAIL;

    if (*icmsghdr).icmsgtype as c_int == ICMSGTYPE_NEGOTIATE {
        if vmbus_prep_negotiate_resp(
            icmsghdr,
            desc.add(offset as usize),
            pktlen as c_uint,
            fw_versions.as_ptr(),
            FW_VER_COUNT,
            fcopy_versions.as_ptr(),
            FCOPY_VER_COUNT,
            ptr::null_mut(),
            &mut fcopy_srv_version,
        ) {
            syslog(
                LOG_INFO,
                b"FCopy IC version %d.%d\0".as_ptr() as *const c_char,
                fcopy_srv_version >> 16,
                fcopy_srv_version & 0xFFFF,
            );
            (*icmsghdr).status = 0;
        }
    } else if (*icmsghdr).icmsgtype as c_int == ICMSGTYPE_FCOPY {
        /* Ensure recvlen is big enough to contain hv_fcopy_hdr */
        if (pktlen as usize) < ICMSG_HDR + size_of::<hv_fcopy_hdr>() {
            syslog(
                LOG_ERR,
                b"Invalid Fcopy hdr. Packet length too small: %u\0".as_ptr() as *const c_char,
                pktlen,
            );
            return -ENOBUFS;
        }

        fcopy_msg = desc.add(offset as usize + ICMSG_HDR) as *mut hv_fcopy_hdr;
        (*icmsghdr).status = hv_fcopy_send_data(fcopy_msg, pktlen);
    }

    (*icmsghdr).icflags = ICMSGHDRFLAG_TRANSACTION | ICMSGHDRFLAG_RESPONSE;
    ret = rte_vmbus_chan_send(txbr, 0x6, desc.add(offset as usize) as *mut c_void, pktlen as u32, 0);
    if ret != 0 {
        syslog(LOG_ERR, b"Write to ringbuffer failed err: %d\0".as_ptr() as *const c_char, ret);
        return ret;
    }

    0
}

unsafe fn fcopy_get_first_folder(path: *mut c_char, chan_no: *mut c_char) {
    let dir: *mut DIR = opendir(path);
    let mut entry: *mut dirent;

    if dir.is_null() {
        syslog(
            LOG_ERR,
            b"Failed to open directory (errno=%s).\n\0".as_ptr() as *const c_char,
            strerror(errno),
        );
        return;
    }

    loop {
        entry = readdir(dir);
        if entry.is_null() {
            break;
        }
        if (*entry).d_type == DT_DIR
            && strcmp((*entry).d_name.as_ptr(), b".\0".as_ptr() as *const c_char) != 0
            && strcmp((*entry).d_name.as_ptr(), b"..\0".as_ptr() as *const c_char) != 0
        {
            strcpy(chan_no, (*entry).d_name.as_ptr());
            break;
        }
    }

    closedir(dir);
}

unsafe fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut fcopy_fd: c_int = -1;
    let mut tmp: c_int = 1;
    let mut daemonize: c_int = 1;
    let mut long_index: c_int = 0;
    let mut opt: c_int;
    let mut ret: c_int = -EINVAL;
    let mut txbr: vmbus_br = core::mem::zeroed();
    let mut rxbr: vmbus_br = core::mem::zeroed();
    let ring: *mut c_void;
    let mut ring_size: u32;
    let mut len: u32;
    let mut uio_name = [0 as c_char; NAME_MAX];
    let mut uio_dev_path = [0 as c_char; PATH_MAX];

    static long_options: [option; 3] = [
        option {
            name: b"help\0".as_ptr() as *const c_char,
            has_arg: no_argument,
            flag: ptr::null_mut(),
            val: b'h' as c_int,
        },
        option {
            name: b"no-daemon\0".as_ptr() as *const c_char,
            has_arg: no_argument,
            flag: ptr::null_mut(),
            val: b'n' as c_int,
        },
        option {
            name: ptr::null(),
            has_arg: 0,
            flag: ptr::null_mut(),
            val: 0,
        },
    ];

    loop {
        opt = getopt_long(
            argc,
            argv,
            b"hn\0".as_ptr() as *const c_char,
            long_options.as_ptr(),
            &mut long_index,
        );
        if opt == -1 {
            break;
        }
        match opt {
            x if x == b'n' as c_int => {
                daemonize = 0;
            }
            x if x == b'h' as c_int => {
                print_usage(argv);
                return ret;
            }
            _ => {
                print_usage(argv);
                return ret;
            }
        }
    }

    if daemonize != 0 && daemon(1, 0) != 0 {
        syslog(LOG_ERR, b"daemon() failed; error: %s\0".as_ptr() as *const c_char, strerror(errno));
        return ret;
    }

    openlog(b"HV_UIO_FCOPY\0".as_ptr() as *const c_char, 0, LOG_USER);
    syslog(LOG_INFO, b"starting; pid is:%d\0".as_ptr() as *const c_char, getpid());

    ring_size = get_ring_buffer_size();
    if ring_size == 0 {
        ret = -ENODEV;
        return ret;
    }

    desc = malloc(ring_size as size_t * size_of::<u8>()) as *mut u8;
    if desc.is_null() {
        syslog(LOG_ERR, b"malloc failed for desc buffer\0".as_ptr() as *const c_char);
        ret = -ENOMEM;
        return ret;
    }

    fcopy_get_first_folder(FCOPY_UIO_PATH as *mut c_char, uio_name.as_mut_ptr());
    snprintf(
        uio_dev_path.as_mut_ptr(),
        uio_dev_path.len(),
        b"/dev/%s\0".as_ptr() as *const c_char,
        uio_name.as_ptr(),
    );
    fcopy_fd = open(uio_dev_path.as_ptr(), O_RDWR);

    if fcopy_fd < 0 {
        syslog(
            LOG_ERR,
            b"open %s failed; error: %d %s\0".as_ptr() as *const c_char,
            uio_dev_path.as_ptr(),
            errno,
            strerror(errno),
        );
        ret = fcopy_fd;
        free(desc as *mut c_void);
        return ret;
    }

    ring = vmbus_uio_map(&mut fcopy_fd, ring_size);
    if ring.is_null() {
        ret = errno;
        syslog(
            LOG_ERR,
            b"mmap ringbuffer failed; error: %d %s\0".as_ptr() as *const c_char,
            ret,
            strerror(ret),
        );
        close(fcopy_fd);
        free(desc as *mut c_void);
        return ret;
    }
    vmbus_br_setup(&mut txbr, ring, ring_size);
    vmbus_br_setup(&mut rxbr, (ring as *mut c_char).add(ring_size as usize) as *mut c_void, ring_size);

    (*rxbr.vbr).imask = 0;

    loop {
        /*
         * In this loop we process fcopy messages after the
         * handshake is complete.
         */
        ret = pread(
            fcopy_fd,
            &mut tmp as *mut c_int as *mut c_void,
            size_of::<c_int>(),
            0,
        ) as c_int;
        if ret < 0 {
            if errno == EINTR || errno == EAGAIN {
                continue;
            }
            syslog(LOG_ERR, b"pread failed: %s\0".as_ptr() as *const c_char, strerror(errno));
            close(fcopy_fd);
            free(desc as *mut c_void);
            return ret;
        }

        len = ring_size;
        ret = rte_vmbus_chan_recv_raw(&mut rxbr, desc as *mut c_void, &mut len);
        if unlikely(ret <= 0) {
            /* This indicates a failure to communicate (or worse) */
            syslog(LOG_ERR, b"VMBus channel recv error: %d\0".as_ptr() as *const c_char, ret);
        } else {
            ret = fcopy_pkt_process(&mut txbr);
            if ret < 0 {
                close(fcopy_fd);
                free(desc as *mut c_void);
                return ret;
            }

            /* Signal host */
            if write(
                fcopy_fd,
                &tmp as *const c_int as *const c_void,
                size_of::<c_int>(),
            ) != size_of::<c_int>() as ssize_t
            {
                ret = errno;
                syslog(
                    LOG_ERR,
                    b"Signal to host failed: %s\n\0".as_ptr() as *const c_char,
                    strerror(ret),
                );
                close(fcopy_fd);
                free(desc as *mut c_void);
                return ret;
            }
        }
    }
}
