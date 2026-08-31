// SPDX-License-Identifier: GPL-2.0-only
/* Industrialio buffer test code.
 *
 * Copyright (c) 2008 Jonathan Cameron
 *
 * This program is primarily intended as an example application.
 * Reads the current buffer setup from sysfs and starts a short capture
 * from the specified device, pretty printing the result after appropriate
 * conversion.
 *
 * Command line parameters
 * generic_buffer -n <device_name> -t <trigger_name>
 * If trigger name is not specified the program assumes you want a dataready
 * trigger associated with the device and goes looking for it.
 */

use std::ffi::c_void;
use std::os::raw::{c_char, c_double, c_float, c_int, c_long, c_ulong, c_ulonglong};
use std::ptr;

type ssize_t = isize;
type size_t = usize;

const EIO: c_int = 5;
const ENOENT: c_int = 2;
const ERANGE: c_int = 34;
const ENOMEM: c_int = 12;
const EFAULT: c_int = 14;
const ENODEV: c_int = 19;
const EINVAL: c_int = 22;
const EBUSY: c_int = 16;
const EAGAIN: c_int = 11;

const O_RDONLY: c_int = 0;
const O_NONBLOCK: c_int = 0o4000;
const POLLIN: c_short = 0x0001;
const SIGINT: c_int = 2;
const SIGTERM: c_int = 15;
const SIGABRT: c_int = 6;
const IIO_MAX_NAME_LENGTH: size_t = 64;

type c_short = i16;
type mode_t = u32;

/* From linux/iio/buffer.h; kept as an external ABI constant equivalent. */
const IIO_BUFFER_GET_FD_IOCTL: c_ulong = 0x80046991;

/* From iio_utils.h. */
const FORMAT_SCAN_ELEMENTS_DIR: &[u8] = b"%siio:device%d/scan_elements\0";

#[repr(C)]
pub struct iio_channel_info {
    pub name: *mut c_char,
    pub generic_name: *mut c_char,
    pub scale: c_float,
    pub offset: c_float,
    pub index: c_uint,
    pub bytes: c_uint,
    pub bits_used: c_uint,
    pub shift: c_uint,
    pub mask: u64,
    pub be: bool,
    pub is_signed: bool,
    pub enabled: bool,
    pub location: c_uint,
    pub format: c_char,
}

type c_uint = u32;

#[repr(C)]
struct DIR(c_void);

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
    st_dev: u64,
    st_ino: u64,
    st_nlink: u64,
    st_mode: mode_t,
    st_uid: u32,
    st_gid: u32,
    __pad0: c_int,
    st_rdev: u64,
    st_size: i64,
    st_blksize: i64,
    st_blocks: i64,
    __unused: [i64; 3],
}

#[repr(C)]
struct pollfd {
    fd: c_int,
    events: c_short,
    revents: c_short,
}

#[repr(C)]
struct sigaction {
    sa_handler: Option<unsafe extern "C" fn(c_int)>,
    sa_flags: c_ulong,
    sa_restorer: Option<unsafe extern "C" fn()>,
    sa_mask: [c_ulong; 16],
}

#[repr(C)]
union Float16Converter {
    u: u16,
    f: u16,
}

#[repr(C)]
union Float32Converter {
    u: u32,
    f: c_float,
}

#[repr(C)]
union Float64Converter {
    u: u64,
    f: c_double,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum autochan {
    AUTOCHANNELS_DISABLED,
    AUTOCHANNELS_ENABLED,
    AUTOCHANNELS_ACTIVE,
}

unsafe extern "C" {
    static mut errno: c_int;
    static iio_dir: *const c_char;
    static mut optarg: *mut c_char;

    fn printf(fmt: *const c_char, ...) -> c_int;
    fn fprintf(stream: *mut c_void, fmt: *const c_char, ...) -> c_int;
    static mut stderr: *mut c_void;
    fn perror(s: *const c_char);
    fn strerror(errnum: c_int) -> *mut c_char;
    fn snprintf(s: *mut c_char, n: size_t, format: *const c_char, ...) -> c_int;
    fn asprintf(strp: *mut *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn malloc(size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn strdup(s: *const c_char) -> *mut c_char;
    fn strtoll(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> i64;
    fn strtoul(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_ulong;
    fn getopt_long(
        argc: c_int,
        argv: *mut *mut c_char,
        optstring: *const c_char,
        longopts: *const option,
        longindex: *mut c_int,
    ) -> c_int;
    fn opendir(name: *const c_char) -> *mut DIR;
    fn readdir(dirp: *mut DIR) -> *mut dirent;
    fn closedir(dirp: *mut DIR) -> c_int;
    fn stat(pathname: *const c_char, statbuf: *mut stat) -> c_int;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: size_t) -> ssize_t;
    fn poll(fds: *mut pollfd, nfds: c_ulong, timeout: c_int) -> c_int;
    fn usleep(usec: c_ulong) -> c_int;
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
    fn sigaction(signum: c_int, act: *const sigaction, oldact: *mut sigaction) -> c_int;
    fn exit(status: c_int) -> !;

    fn iioutils_check_suffix(str_: *const c_char, suffix: *const c_char) -> c_int;
    fn write_sysfs_int(filename: *const c_char, basedir: *const c_char, val: c_int) -> c_int;
    fn write_sysfs_string(filename: *const c_char, basedir: *const c_char, val: *const c_char) -> c_int;
    fn write_sysfs_string_and_verify(
        filename: *const c_char,
        basedir: *const c_char,
        val: *const c_char,
    ) -> c_int;
    fn read_sysfs_string(filename: *const c_char, basedir: *const c_char, str_: *mut c_char) -> c_int;
    fn find_type_by_name(name: *const c_char, type_: *const c_char) -> c_int;
    fn build_channel_array(
        device_dir: *const c_char,
        buffer_idx: c_int,
        channels: *mut *mut iio_channel_info,
        num_channels: *mut c_int,
    ) -> c_int;
}

#[repr(C)]
struct option {
    name: *const c_char,
    has_arg: c_int,
    flag: *mut c_int,
    val: c_int,
}

fn s(bytes: &'static [u8]) -> *const c_char {
    bytes.as_ptr() as *const c_char
}

unsafe fn s_isdir(mode: mode_t) -> bool {
    (mode & 0o170000) == 0o040000
}

unsafe fn size_from_channelarray(channels: *mut iio_channel_info, num_channels: c_int) -> c_uint {
    let mut bytes: c_uint = 0;
    let mut i: c_int = 0;
    let mut max: c_int = 0;
    let misalignment: c_uint;

    while i < num_channels {
        let channel = channels.add(i as usize);
        if (*channel).bytes as c_int > max {
            max = (*channel).bytes as c_int;
        }
        if bytes % (*channel).bytes == 0 {
            (*channel).location = bytes;
        } else {
            (*channel).location = bytes - bytes % (*channel).bytes + (*channel).bytes;
        }

        bytes = (*channel).location + (*channel).bytes;
        i += 1;
    }
    /*
     * We want the data in next sample to also be properly aligned so
     * we'll add padding at the end if needed. Adding padding only
     * works for channel data which size is 2^n bytes.
     */
    misalignment = bytes % max as c_uint;
    if misalignment != 0 {
        bytes += max as c_uint - misalignment;
    }

    bytes
}

unsafe fn print1byte(mut input: u8, info: *mut iio_channel_info) {
    /*
     * Shift before conversion to avoid sign extension
     * of left aligned data
     */
    input >>= (*info).shift;
    input &= (*info).mask as u8;
    match (*info).format as u8 as char {
        's' => {
            let val = ((input << (8 - (*info).bits_used)) as i8) >> (8 - (*info).bits_used);
            printf(s(b"%05f \0"), ((val as c_float + (*info).offset) * (*info).scale) as c_double);
        }
        'u' => {
            printf(s(b"%05f \0"), ((input as c_float + (*info).offset) * (*info).scale) as c_double);
        }
        'f' => {
            printf(s(b"<invalid 1-byte float> \0"));
        }
        _ => {}
    }
}

unsafe fn print2byte(mut input: u16, info: *mut iio_channel_info) {
    /* First swap if incorrect endian */
    if (*info).be {
        input = u16::from_be(input);
    } else {
        input = u16::from_le(input);
    }

    /*
     * Shift before conversion to avoid sign extension
     * of left aligned data
     */
    input >>= (*info).shift;
    input &= (*info).mask as u16;
    match (*info).format as u8 as char {
        's' => {
            let val = ((input << (16 - (*info).bits_used)) as i16) >> (16 - (*info).bits_used);
            printf(s(b"%05f \0"), ((val as c_float + (*info).offset) * (*info).scale) as c_double);
        }
        'u' => {
            printf(s(b"%05f \0"), ((input as c_float + (*info).offset) * (*info).scale) as c_double);
        }
        'f' => {
            /* C uses _Float16 when __FLT16_MAX__ is available; Rust has no stable C _Float16 FFI here. */
            let mut converter = Float16Converter { u: input };
            let _ = unsafe { converter.f };
            printf(s(b"<unsupported 2-byte float> \0"));
        }
        _ => {}
    }
}

unsafe fn print4byte(mut input: u32, info: *mut iio_channel_info) {
    /* First swap if incorrect endian */
    if (*info).be {
        input = u32::from_be(input);
    } else {
        input = u32::from_le(input);
    }

    /*
     * Shift before conversion to avoid sign extension
     * of left aligned data
     */
    input >>= (*info).shift;
    input &= (*info).mask as u32;
    match (*info).format as u8 as char {
        's' => {
            let val = ((input << (32 - (*info).bits_used)) as i32) >> (32 - (*info).bits_used);
            printf(s(b"%05f \0"), ((val as c_float + (*info).offset) * (*info).scale) as c_double);
        }
        'u' => {
            printf(s(b"%05f \0"), ((input as c_float + (*info).offset) * (*info).scale) as c_double);
        }
        'f' => {
            let converter = Float32Converter { u: input };
            printf(s(b"%05f \0"), ((converter.f + (*info).offset) * (*info).scale) as c_double);
        }
        _ => {}
    }
}

unsafe fn print8byte(mut input: u64, info: *mut iio_channel_info) {
    /* First swap if incorrect endian */
    if (*info).be {
        input = u64::from_be(input);
    } else {
        input = u64::from_le(input);
    }

    /*
     * Shift before conversion to avoid sign extension
     * of left aligned data
     */
    input >>= (*info).shift;
    input &= (*info).mask;
    match (*info).format as u8 as char {
        's' => {
            let val = ((input << (64 - (*info).bits_used)) as i64) >> (64 - (*info).bits_used);
            /* special case for timestamp */
            if (*info).scale == 1.0f32 && (*info).offset == 0.0f32 {
                printf(s(b"%ld \0"), val as c_long);
            } else {
                printf(s(b"%05f \0"), ((val as c_float + (*info).offset) * (*info).scale) as c_double);
            }
        }
        'u' => {
            printf(s(b"%05f \0"), ((input as c_float + (*info).offset) * (*info).scale) as c_double);
        }
        'f' => {
            let converter = Float64Converter { u: input };
            printf(s(b"%05f \0"), (converter.f + (*info).offset as c_double) * (*info).scale as c_double);
        }
        _ => {}
    }
}

/**
 * process_scan() - print out the values in SI units
 * @data:		pointer to the start of the scan
 * @channels:		information about the channels.
 *			Note: size_from_channelarray must have been called first
 *			      to fill the location offsets.
 * @num_channels:	number of channels
 **/
unsafe fn process_scan(data: *mut c_char, channels: *mut iio_channel_info, num_channels: c_int) {
    let mut k: c_int = 0;

    while k < num_channels {
        let channel = channels.add(k as usize);
        match (*channel).bytes {
            /* only a few cases implemented so far */
            1 => print1byte(*(data.add((*channel).location as usize) as *mut u8), channel),
            2 => print2byte(*(data.add((*channel).location as usize) as *mut u16), channel),
            4 => print4byte(*(data.add((*channel).location as usize) as *mut u32), channel),
            8 => print8byte(*(data.add((*channel).location as usize) as *mut u64), channel),
            _ => {}
        }
        k += 1;
    }
    printf(s(b"\n\0"));
}

unsafe fn enable_disable_all_channels(dev_dir_name: *mut c_char, buffer_idx: c_int, enable: c_int) -> c_int {
    let mut ent: *const dirent;
    let mut scanelemdir = [0 as c_char; 256];
    let dp: *mut DIR;
    let mut ret: c_int;

    snprintf(
        scanelemdir.as_mut_ptr(),
        scanelemdir.len(),
        FORMAT_SCAN_ELEMENTS_DIR.as_ptr() as *const c_char,
        dev_dir_name,
        buffer_idx,
    );
    scanelemdir[scanelemdir.len() - 1] = 0;

    dp = opendir(scanelemdir.as_ptr());
    if dp.is_null() {
        fprintf(
            stderr,
            s(b"Enabling/disabling channels: can't open %s\n\0"),
            scanelemdir.as_ptr(),
        );
        return -EIO;
    }

    ret = -ENOENT;
    loop {
        ent = readdir(dp);
        if ent.is_null() {
            break;
        }
        if iioutils_check_suffix((*ent).d_name.as_ptr(), s(b"_en\0")) != 0 {
            printf(
                s(b"%sabling: %s\n\0"),
                if enable != 0 { s(b"En\0") } else { s(b"Dis\0") },
                (*ent).d_name.as_ptr(),
            );
            ret = write_sysfs_int((*ent).d_name.as_ptr(), scanelemdir.as_ptr(), enable);
            if ret < 0 {
                fprintf(stderr, s(b"Failed to enable/disable %s\n\0"), (*ent).d_name.as_ptr());
            }
        }
    }

    if closedir(dp) == -1 {
        perror(s(b"Enabling/disabling channels: Failed to close directory\0"));
        return -errno;
    }
    0
}

unsafe fn print_usage() {
    fprintf(
        stderr,
        s(b"Usage: generic_buffer [options]...\nCapture, convert and output data from IIO device buffer\n  -a         Auto-activate all available channels\n  -A         Force-activate ALL channels\n  -b <n>     The buffer which to open (by index), default 0\n  -c <n>     Do n conversions, or loop forever if n < 0\n  -e         Disable wait for event (new data)\n  -g         Use trigger-less mode\n  -l <n>     Set buffer length to n samples\n  --device-name -n <name>\n  --device-num -N <num>\n        Set device by name or number (mandatory)\n  --trigger-name -t <name>\n  --trigger-num -T <num>\n        Set trigger by name or number\n  -w <n>     Set delay between reads in us (event-less mode)\n\0"),
    );
}

static mut autochannels: autochan = autochan::AUTOCHANNELS_DISABLED;
static mut dev_dir_name: *mut c_char = ptr::null_mut();
static mut buf_dir_name: *mut c_char = ptr::null_mut();
static mut buffer_idx: c_int = 0;
static mut current_trigger_set: bool = false;

unsafe fn cleanup() {
    let mut ret: c_int;

    /* Disable trigger */
    if !dev_dir_name.is_null() && current_trigger_set {
        /* Disconnect the trigger - just write a dummy name. */
        ret = write_sysfs_string(s(b"trigger/current_trigger\0"), dev_dir_name, s(b"NULL\0"));
        if ret < 0 {
            fprintf(stderr, s(b"Failed to disable trigger: %s\n\0"), strerror(-ret));
        }
        current_trigger_set = false;
    }

    /* Disable buffer */
    if !buf_dir_name.is_null() {
        ret = write_sysfs_int(s(b"enable\0"), buf_dir_name, 0);
        if ret < 0 {
            fprintf(stderr, s(b"Failed to disable buffer: %s\n\0"), strerror(-ret));
        }
    }

    /* Disable channels if auto-enabled */
    if !dev_dir_name.is_null() && autochannels == autochan::AUTOCHANNELS_ACTIVE {
        ret = enable_disable_all_channels(dev_dir_name, buffer_idx, 0);
        if ret != 0 {
            fprintf(stderr, s(b"Failed to disable all channels\n\0"));
        }
        autochannels = autochan::AUTOCHANNELS_DISABLED;
    }
}

unsafe extern "C" fn sig_handler(signum: c_int) {
    fprintf(stderr, s(b"Caught signal %d\n\0"), signum);
    cleanup();
    exit(-signum);
}

unsafe fn register_cleanup() {
    let mut sa = sigaction {
        sa_handler: Some(sig_handler),
        sa_flags: 0,
        sa_restorer: None,
        sa_mask: [0; 16],
    };
    let signums = [SIGINT, SIGTERM, SIGABRT];
    let mut ret: c_int;
    let mut i: usize = 0;

    while i < signums.len() {
        ret = sigaction(signums[i], &mut sa, ptr::null_mut());
        if ret != 0 {
            perror(s(b"Failed to register signal handler\0"));
            exit(-1);
        }
        i += 1;
    }
}

static longopts: [option; 5] = [
    option { name: b"device-name\0".as_ptr() as *const c_char, has_arg: 1, flag: ptr::null_mut(), val: 'n' as c_int },
    option { name: b"device-num\0".as_ptr() as *const c_char, has_arg: 1, flag: ptr::null_mut(), val: 'N' as c_int },
    option { name: b"trigger-name\0".as_ptr() as *const c_char, has_arg: 1, flag: ptr::null_mut(), val: 't' as c_int },
    option { name: b"trigger-num\0".as_ptr() as *const c_char, has_arg: 1, flag: ptr::null_mut(), val: 'T' as c_int },
    option { name: ptr::null(), has_arg: 0, flag: ptr::null_mut(), val: 0 },
];

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut num_loops: i64 = 2;
    let mut timedelay: c_ulong = 1000000;
    let mut buf_len: c_ulong = 128;

    let mut i: ssize_t;
    let mut j: c_ulonglong;
    let mut toread: c_ulong;
    let mut ret: c_int = 0;
    let mut c: c_int;
    let mut st: stat = std::mem::zeroed();
    let mut fd: c_int = -1;
    let mut buf_fd: c_int = -1;

    let mut num_channels: c_int = 0;
    let mut trigger_name: *mut c_char = ptr::null_mut();
    let mut device_name: *mut c_char = ptr::null_mut();

    let mut data: *mut c_char = ptr::null_mut();
    let mut read_size: ssize_t;
    let mut dev_num: c_int = -1;
    let mut trig_num: c_int = -1;
    let mut buffer_access: *mut c_char = ptr::null_mut();
    let mut scan_size: c_uint;
    let mut noevents: c_int = 0;
    let mut notrigger: c_int = 0;
    let mut dummy: *mut c_char = ptr::null_mut();
    let mut force_autochannels: bool = false;

    let mut channels: *mut iio_channel_info = ptr::null_mut();

    register_cleanup();

    macro_rules! error {
        () => {{
            cleanup();

            if fd >= 0 && close(fd) == -1 {
                perror(s(b"Failed to close character device\0"));
            }
            if buf_fd >= 0 && close(buf_fd) == -1 {
                perror(s(b"Failed to close buffer\0"));
            }
            free(buffer_access as *mut c_void);
            free(data as *mut c_void);
            free(buf_dir_name as *mut c_void);
            i = num_channels as ssize_t - 1;
            while i >= 0 {
                free((*channels.add(i as usize)).name as *mut c_void);
                free((*channels.add(i as usize)).generic_name as *mut c_void);
                i -= 1;
            }
            free(channels as *mut c_void);
            free(trigger_name as *mut c_void);
            free(device_name as *mut c_void);
            free(dev_dir_name as *mut c_void);

            return ret;
        }};
    }
    macro_rules! goto_error {
        ($ret:expr) => {{
            let _ = $ret;
            error!();
        }};
    }

    loop {
        c = getopt_long(argc, argv, s(b"aAb:c:egl:n:N:t:T:w:?\0"), longopts.as_ptr(), ptr::null_mut());
        if c == -1 {
            break;
        }
        match c as u8 as char {
            'a' => autochannels = autochan::AUTOCHANNELS_ENABLED,
            'A' => {
                autochannels = autochan::AUTOCHANNELS_ENABLED;
                force_autochannels = true;
            }
            'b' => {
                errno = 0;
                buffer_idx = strtoll(optarg, &mut dummy, 10) as c_int;
                if errno != 0 {
                    ret = -errno;
                    goto_error!(ret);
                }
                if buffer_idx < 0 {
                    ret = -ERANGE;
                    goto_error!(ret);
                }
            }
            'c' => {
                errno = 0;
                num_loops = strtoll(optarg, &mut dummy, 10);
                if errno != 0 {
                    ret = -errno;
                    goto_error!(ret);
                }
            }
            'e' => noevents = 1,
            'g' => notrigger = 1,
            'l' => {
                errno = 0;
                buf_len = strtoul(optarg, &mut dummy, 10);
                if errno != 0 {
                    ret = -errno;
                    goto_error!(ret);
                }
            }
            'n' => device_name = strdup(optarg),
            'N' => {
                errno = 0;
                dev_num = strtoul(optarg, &mut dummy, 10) as c_int;
                if errno != 0 {
                    ret = -errno;
                    goto_error!(ret);
                }
            }
            't' => trigger_name = strdup(optarg),
            'T' => {
                errno = 0;
                trig_num = strtoul(optarg, &mut dummy, 10) as c_int;
                if errno != 0 {
                    return -errno;
                }
            }
            'w' => {
                errno = 0;
                timedelay = strtoul(optarg, &mut dummy, 10);
                if errno != 0 {
                    ret = -errno;
                    goto_error!(ret);
                }
            }
            '?' => {
                print_usage();
                ret = -1;
                goto_error!(ret);
            }
            _ => {}
        }
    }

    /* Find the device requested */
    if dev_num < 0 && device_name.is_null() {
        fprintf(stderr, s(b"Device not set\n\0"));
        print_usage();
        ret = -1;
        error!();
    } else if dev_num >= 0 && !device_name.is_null() {
        fprintf(stderr, s(b"Only one of --device-num or --device-name needs to be set\n\0"));
        print_usage();
        ret = -1;
        error!();
    } else if dev_num < 0 {
        dev_num = find_type_by_name(device_name, s(b"iio:device\0"));
        if dev_num < 0 {
            fprintf(stderr, s(b"Failed to find the %s\n\0"), device_name);
            ret = dev_num;
            error!();
        }
    }
    printf(s(b"iio device number being used is %d\n\0"), dev_num);

    ret = asprintf(&mut dev_dir_name, s(b"%siio:device%d\0"), iio_dir, dev_num);
    if ret < 0 {
        return -ENOMEM;
    }
    /* Fetch device_name if specified by number */
    if device_name.is_null() {
        device_name = malloc(IIO_MAX_NAME_LENGTH) as *mut c_char;
        if device_name.is_null() {
            ret = -ENOMEM;
            error!();
        }
        ret = read_sysfs_string(s(b"name\0"), dev_dir_name, device_name);
        if ret < 0 {
            fprintf(stderr, s(b"Failed to read name of device %d\n\0"), dev_num);
            error!();
        }
    }

    if notrigger != 0 {
        printf(s(b"trigger-less mode selected\n\0"));
    } else if trig_num >= 0 {
        let mut trig_dev_name: *mut c_char = ptr::null_mut();
        ret = asprintf(&mut trig_dev_name, s(b"%strigger%d\0"), iio_dir, trig_num);
        if ret < 0 {
            return -ENOMEM;
        }
        trigger_name = malloc(IIO_MAX_NAME_LENGTH) as *mut c_char;
        if trigger_name.is_null() {
            ret = -ENOMEM;
            error!();
        }
        ret = read_sysfs_string(s(b"name\0"), trig_dev_name, trigger_name);
        free(trig_dev_name as *mut c_void);
        if ret < 0 {
            fprintf(stderr, s(b"Failed to read trigger%d name from\n\0"), trig_num);
            return ret;
        }
        printf(s(b"iio trigger number being used is %d\n\0"), trig_num);
    } else {
        if trigger_name.is_null() {
            /*
             * Build the trigger name. If it is device associated
             * its name is <device_name>_dev[n] where n matches
             * the device number found above.
             */
            ret = asprintf(&mut trigger_name, s(b"%s-dev%d\0"), device_name, dev_num);
            if ret < 0 {
                ret = -ENOMEM;
                error!();
            }
        }

        /* Look for this "-devN" trigger */
        trig_num = find_type_by_name(trigger_name, s(b"trigger\0"));
        if trig_num < 0 {
            /* OK try the simpler "-trigger" suffix instead */
            free(trigger_name as *mut c_void);
            ret = asprintf(&mut trigger_name, s(b"%s-trigger\0"), device_name);
            if ret < 0 {
                ret = -ENOMEM;
                error!();
            }
        }

        trig_num = find_type_by_name(trigger_name, s(b"trigger\0"));
        if trig_num < 0 {
            fprintf(stderr, s(b"Failed to find the trigger %s\n\0"), trigger_name);
            ret = trig_num;
            error!();
        }

        printf(s(b"iio trigger number being used is %d\n\0"), trig_num);
    }

    /*
     * Parse the files in scan_elements to identify what channels are
     * present
     */
    ret = build_channel_array(dev_dir_name, buffer_idx, &mut channels, &mut num_channels);
    if ret != 0 {
        fprintf(stderr, s(b"Problem reading scan element information\ndiag %s\n\0"), dev_dir_name);
        error!();
    }
    if num_channels != 0 && autochannels == autochan::AUTOCHANNELS_ENABLED && !force_autochannels {
        fprintf(stderr, s(b"Auto-channels selected but some channels are already activated in sysfs\n\0"));
        fprintf(stderr, s(b"Proceeding without activating any channels\n\0"));
    }

    if (num_channels == 0 && autochannels == autochan::AUTOCHANNELS_ENABLED)
        || (autochannels == autochan::AUTOCHANNELS_ENABLED && force_autochannels)
    {
        fprintf(stderr, s(b"Enabling all channels\n\0"));

        ret = enable_disable_all_channels(dev_dir_name, buffer_idx, 1);
        if ret != 0 {
            fprintf(stderr, s(b"Failed to enable all channels\n\0"));
            error!();
        }

        /* This flags that we need to disable the channels again */
        autochannels = autochan::AUTOCHANNELS_ACTIVE;

        ret = build_channel_array(dev_dir_name, buffer_idx, &mut channels, &mut num_channels);
        if ret != 0 {
            fprintf(stderr, s(b"Problem reading scan element information\ndiag %s\n\0"), dev_dir_name);
            error!();
        }
        if num_channels == 0 {
            fprintf(stderr, s(b"Still no channels after auto-enabling, giving up\n\0"));
            error!();
        }
    }

    if num_channels == 0 && autochannels == autochan::AUTOCHANNELS_DISABLED {
        fprintf(stderr, s(b"No channels are enabled, we have nothing to scan.\n\0"));
        fprintf(
            stderr,
            s(b"Enable channels manually in %siio:device%d/scan_elements/*_en or pass -a to autoenable channels and try again.\n\0"),
            dev_dir_name,
            buffer_idx,
        );
        ret = -ENOENT;
        error!();
    }

    /*
     * Construct the directory name for the associated buffer.
     * As we know that the lis3l02dq has only one buffer this may
     * be built rather than found.
     */
    ret = asprintf(&mut buf_dir_name, s(b"%siio:device%d/buffer%d\0"), iio_dir, dev_num, buffer_idx);
    if ret < 0 {
        ret = -ENOMEM;
        error!();
    }

    if stat(buf_dir_name, &mut st) != 0 {
        fprintf(
            stderr,
            s(b"Could not stat() '%s', got error %d: %s\n\0"),
            buf_dir_name,
            errno,
            strerror(errno),
        );
        ret = -errno;
        error!();
    }

    if !s_isdir(st.st_mode) {
        fprintf(stderr, s(b"File '%s' is not a directory\n\0"), buf_dir_name);
        ret = -EFAULT;
        error!();
    }

    if notrigger == 0 {
        printf(s(b"%s %s\n\0"), dev_dir_name, trigger_name);
        /*
         * Set the device trigger to be the data ready trigger found
         * above
         */
        ret = write_sysfs_string_and_verify(s(b"trigger/current_trigger\0"), dev_dir_name, trigger_name);
        if ret < 0 {
            fprintf(stderr, s(b"Failed to write current_trigger file\n\0"));
            error!();
        }
    }

    ret = asprintf(&mut buffer_access, s(b"/dev/iio:device%d\0"), dev_num);
    if ret < 0 {
        ret = -ENOMEM;
        error!();
    }

    /* Attempt to open non blocking the access dev */
    fd = open(buffer_access, O_RDONLY | O_NONBLOCK);
    if fd == -1 {
        /* TODO: If it isn't there make the node */
        ret = -errno;
        fprintf(stderr, s(b"Failed to open %s\n\0"), buffer_access);
        error!();
    }

    /* specify for which buffer index we want an FD */
    buf_fd = buffer_idx;

    ret = ioctl(fd, IIO_BUFFER_GET_FD_IOCTL, &mut buf_fd);
    if ret == -1 || buf_fd == -1 {
        ret = -errno;
        if ret == -ENODEV || ret == -EINVAL {
            fprintf(stderr, s(b"Device does not have this many buffers\n\0"));
        } else {
            fprintf(stderr, s(b"Failed to retrieve buffer fd\n\0"));
        }

        error!();
    }

    /* Setup ring buffer parameters */
    ret = write_sysfs_int(s(b"length\0"), buf_dir_name, buf_len as c_int);
    if ret < 0 {
        error!();
    }

    /* Enable the buffer */
    ret = write_sysfs_int(s(b"enable\0"), buf_dir_name, 1);
    if ret < 0 {
        fprintf(stderr, s(b"Failed to enable buffer '%s': %s\n\0"), buf_dir_name, strerror(-ret));
        error!();
    }

    scan_size = size_from_channelarray(channels, num_channels);

    let total_buf_len: size_t = scan_size as size_t * buf_len as size_t;

    if scan_size > 0 && total_buf_len / scan_size as size_t != buf_len as size_t {
        ret = -EFAULT;
        perror(s(b"Integer overflow happened when calculate scan_size * buf_len\0"));
        error!();
    }

    data = malloc(total_buf_len) as *mut c_char;
    if data.is_null() {
        ret = -ENOMEM;
        error!();
    }

    /**
     * This check is being done here for sanity reasons, however it
     * should be omitted under normal operation.
     * If this is buffer0, we check that we get EBUSY after this point.
     */
    if buffer_idx == 0 {
        errno = 0;
        read_size = read(fd, data as *mut c_void, 1);
        if read_size > -1 || errno != EBUSY {
            ret = -EFAULT;
            perror(s(b"Reading from '%s' should not be possible after ioctl()\0"));
            error!();
        }
    }

    /* close now the main chardev FD and let the buffer FD work */
    if close(fd) == -1 {
        perror(s(b"Failed to close character device file\0"));
    }
    fd = -1;

    j = 0;
    while j < num_loops as c_ulonglong || num_loops < 0 {
        if noevents == 0 {
            let mut pfd = pollfd {
                fd: buf_fd,
                events: POLLIN,
                revents: 0,
            };

            ret = poll(&mut pfd, 1, -1);
            if ret < 0 {
                ret = -errno;
                error!();
            } else if ret == 0 {
                j = j.wrapping_add(1);
                continue;
            }
        } else {
            usleep(timedelay);
        }

        toread = buf_len;

        read_size = read(buf_fd, data as *mut c_void, (toread as size_t) * scan_size as size_t);
        if read_size < 0 {
            if errno == EAGAIN {
                fprintf(stderr, s(b"nothing available\n\0"));
                j = j.wrapping_add(1);
                continue;
            } else {
                break;
            }
        }
        i = 0;
        while i < read_size / scan_size as ssize_t {
            process_scan(data.add(scan_size as usize * i as usize), channels, num_channels);
            i += 1;
        }
        j = j.wrapping_add(1);
    }

    error!();
}
