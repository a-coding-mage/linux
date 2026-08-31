/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright (c) 2022-2024 Red Hat */

/* Translated from testing/selftests/hid/hid_common.h. */
/* Original C dependencies:
 * "kselftest_harness.h", <fcntl.h>, <fnmatch.h>, <dirent.h>, <poll.h>,
 * <pthread.h>, <stdbool.h>, <linux/hidraw.h>, <linux/uhid.h>
 */

use core::ffi::{c_char, c_int, c_long, c_void};
use core::mem::size_of;
use core::ptr;

pub const SHOW_UHID_DEBUG: c_int = 0;

macro_rules! min {
    ($a:expr, $b:expr) => {{
        let _a = $a;
        let _b = $b;
        if _a < _b { _a } else { _b }
    }};
}

macro_rules! ASSERT_OK {
    ($data:expr) => {
        ASSERT_FALSE!($data)
    };
}

macro_rules! ASSERT_OK_PTR {
    ($ptr:expr) => {
        ASSERT_NE!(NULL, $ptr)
    };
}

macro_rules! UHID_LOG {
    ($($arg:tt)*) => {{
        if SHOW_UHID_DEBUG != 0 {
            TH_LOG!($($arg)*);
        }
    }};
}

#[repr(C)]
pub struct uhid_device {
    pub dev_id: c_int, /* uniq (random) number to identify the device */
    pub uhid_fd: c_int,
    pub hid_id: c_int, /* HID device id in the system */
    pub bus: __u16,
    pub vid: __u32,
    pub pid: __u32,
    pub tid: pthread_t, /* thread for reading uhid events */
}

static mut rdesc: [u8; 118] = [
    0x06, 0x00, 0xff, /* Usage Page (Vendor Defined Page 1) */
    0x09, 0x21, /* Usage (Vendor Usage 0x21) */
    0xa1, 0x01, /* COLLECTION (Application) */
    0x09, 0x01, /* Usage (Vendor Usage 0x01) */
    0xa1, 0x00, /* COLLECTION (Physical) */
    0x85, 0x02, /* REPORT_ID (2) */
    0x19, 0x01, /* USAGE_MINIMUM (1) */
    0x29, 0x08, /* USAGE_MAXIMUM (3) */
    0x15, 0x00, /* LOGICAL_MINIMUM (0) */
    0x25, 0xff, /* LOGICAL_MAXIMUM (255) */
    0x95, 0x08, /* REPORT_COUNT (8) */
    0x75, 0x08, /* REPORT_SIZE (8) */
    0x81, 0x02, /* INPUT (Data,Var,Abs) */
    0xc0, /* END_COLLECTION */
    0x09, 0x01, /* Usage (Vendor Usage 0x01) */
    0xa1, 0x00, /* COLLECTION (Physical) */
    0x85, 0x01, /* REPORT_ID (1) */
    0x06, 0x00, 0xff, /* Usage Page (Vendor Defined Page 1) */
    0x19, 0x01, /* USAGE_MINIMUM (1) */
    0x29, 0x03, /* USAGE_MAXIMUM (3) */
    0x15, 0x00, /* LOGICAL_MINIMUM (0) */
    0x25, 0x01, /* LOGICAL_MAXIMUM (1) */
    0x95, 0x03, /* REPORT_COUNT (3) */
    0x75, 0x01, /* REPORT_SIZE (1) */
    0x81, 0x02, /* INPUT (Data,Var,Abs) */
    0x95, 0x01, /* REPORT_COUNT (1) */
    0x75, 0x05, /* REPORT_SIZE (5) */
    0x81, 0x01, /* INPUT (Cnst,Var,Abs) */
    0x05, 0x01, /* USAGE_PAGE (Generic Desktop) */
    0x09, 0x30, /* USAGE (X) */
    0x09, 0x31, /* USAGE (Y) */
    0x15, 0x81, /* LOGICAL_MINIMUM (-127) */
    0x25, 0x7f, /* LOGICAL_MAXIMUM (127) */
    0x75, 0x10, /* REPORT_SIZE (16) */
    0x95, 0x02, /* REPORT_COUNT (2) */
    0x81, 0x06, /* INPUT (Data,Var,Rel) */

    0x06, 0x00, 0xff, /* Usage Page (Vendor Defined Page 1) */
    0x19, 0x01, /* USAGE_MINIMUM (1) */
    0x29, 0x03, /* USAGE_MAXIMUM (3) */
    0x15, 0x00, /* LOGICAL_MINIMUM (0) */
    0x25, 0x01, /* LOGICAL_MAXIMUM (1) */
    0x95, 0x03, /* REPORT_COUNT (3) */
    0x75, 0x01, /* REPORT_SIZE (1) */
    0x91, 0x02, /* Output (Data,Var,Abs) */
    0x95, 0x01, /* REPORT_COUNT (1) */
    0x75, 0x05, /* REPORT_SIZE (5) */
    0x91, 0x01, /* Output (Cnst,Var,Abs) */

    0x06, 0x00, 0xff, /* Usage Page (Vendor Defined Page 1) */
    0x19, 0x06, /* USAGE_MINIMUM (6) */
    0x29, 0x08, /* USAGE_MAXIMUM (8) */
    0x15, 0x00, /* LOGICAL_MINIMUM (0) */
    0x25, 0x01, /* LOGICAL_MAXIMUM (1) */
    0x95, 0x03, /* REPORT_COUNT (3) */
    0x75, 0x01, /* REPORT_SIZE (1) */
    0xb1, 0x02, /* Feature (Data,Var,Abs) */
    0x95, 0x01, /* REPORT_COUNT (1) */
    0x75, 0x05, /* REPORT_SIZE (5) */
    0x91, 0x01, /* Output (Cnst,Var,Abs) */

    0xc0, /* END_COLLECTION */
    0xc0, /* END_COLLECTION */
];

static mut feature_data: [__u8; 2] = [1, 2];

static mut uhid_started_mtx: pthread_mutex_t = PTHREAD_MUTEX_INITIALIZER;
static mut uhid_started: pthread_cond_t = PTHREAD_COND_INITIALIZER;

static mut uhid_output_mtx: pthread_mutex_t = PTHREAD_MUTEX_INITIALIZER;
static mut uhid_output_cond: pthread_cond_t = PTHREAD_COND_INITIALIZER;
static mut output_report: [u8; 10] = [0; 10];

/* no need to protect uhid_stopped, only one thread accesses it */
static mut uhid_stopped: bool = false;

unsafe fn uhid_write(_metadata: *mut __test_metadata, fd: c_int, ev: *const uhid_event) -> c_int {
    let ret: ssize_t;

    ret = write(fd, ev as *const c_void, size_of::<uhid_event>());
    if ret < 0 {
        TH_LOG!("Cannot write to uhid: %m");
        return -errno;
    } else if ret as usize != size_of::<uhid_event>() {
        TH_LOG!(
            "Wrong size written to uhid: %zd != %zu",
            ret,
            size_of::<*const uhid_event>()
        );
        return -EFAULT;
    } else {
        return 0;
    }
}

unsafe fn uhid_create(
    _metadata: *mut __test_metadata,
    fd: c_int,
    rand_nb: c_int,
    bus: __u16,
    vid: __u32,
    pid: __u32,
    rdesc: *mut __u8,
    rdesc_size: size_t,
) -> c_int {
    let mut ev: uhid_event = core::mem::zeroed();
    let mut buf: [c_char; 25] = [0; 25];

    sprintf(buf.as_mut_ptr(), c"test-uhid-device-%d".as_ptr(), rand_nb);

    memset(
        &mut ev as *mut uhid_event as *mut c_void,
        0,
        size_of::<uhid_event>(),
    );
    ev.type_ = UHID_CREATE;
    strcpy(ev.u.create.name.as_mut_ptr() as *mut c_char, buf.as_ptr());
    ev.u.create.rd_data = rdesc;
    ev.u.create.rd_size = rdesc_size;
    ev.u.create.bus = bus;
    ev.u.create.vendor = vid;
    ev.u.create.product = pid;
    ev.u.create.version = 0;
    ev.u.create.country = 0;

    sprintf(buf.as_mut_ptr(), c"%d".as_ptr(), rand_nb);
    strcpy(ev.u.create.phys.as_mut_ptr() as *mut c_char, buf.as_ptr());

    return uhid_write(_metadata, fd, &ev);
}

unsafe fn uhid_destroy(_metadata: *mut __test_metadata, hid: *mut uhid_device) {
    let mut ev: uhid_event = core::mem::zeroed();

    memset(
        &mut ev as *mut uhid_event as *mut c_void,
        0,
        size_of::<uhid_event>(),
    );
    ev.type_ = UHID_DESTROY;

    uhid_write(_metadata, (*hid).uhid_fd, &ev);
}

unsafe fn uhid_event(_metadata: *mut __test_metadata, fd: c_int) -> c_int {
    let mut ev: uhid_event = core::mem::zeroed();
    let mut answer: uhid_event = core::mem::zeroed();
    let ret: ssize_t;

    memset(
        &mut ev as *mut uhid_event as *mut c_void,
        0,
        size_of::<uhid_event>(),
    );
    ret = read(fd, &mut ev as *mut uhid_event as *mut c_void, size_of::<uhid_event>());
    if ret == 0 {
        UHID_LOG!("Read HUP on uhid-cdev");
        return -EFAULT;
    } else if ret < 0 {
        UHID_LOG!("Cannot read uhid-cdev: %m");
        return -errno;
    } else if ret as usize != size_of::<uhid_event>() {
        UHID_LOG!(
            "Invalid size read from uhid-dev: %zd != %zu",
            ret,
            size_of::<uhid_event>()
        );
        return -EFAULT;
    }

    match ev.type_ {
        UHID_START => {
            pthread_mutex_lock(ptr::addr_of_mut!(uhid_started_mtx));
            pthread_cond_signal(ptr::addr_of_mut!(uhid_started));
            pthread_mutex_unlock(ptr::addr_of_mut!(uhid_started_mtx));

            UHID_LOG!("UHID_START from uhid-dev");
        }
        UHID_STOP => {
            uhid_stopped = true;

            UHID_LOG!("UHID_STOP from uhid-dev");
        }
        UHID_OPEN => {
            UHID_LOG!("UHID_OPEN from uhid-dev");
        }
        UHID_CLOSE => {
            UHID_LOG!("UHID_CLOSE from uhid-dev");
        }
        UHID_OUTPUT => {
            UHID_LOG!("UHID_OUTPUT from uhid-dev");

            pthread_mutex_lock(ptr::addr_of_mut!(uhid_output_mtx));
            memcpy(
                output_report.as_mut_ptr() as *mut c_void,
                ev.u.output.data.as_ptr() as *const c_void,
                min!(ev.u.output.size as usize, size_of::<[u8; 10]>()),
            );
            pthread_cond_signal(ptr::addr_of_mut!(uhid_output_cond));
            pthread_mutex_unlock(ptr::addr_of_mut!(uhid_output_mtx));
        }
        UHID_GET_REPORT => {
            UHID_LOG!("UHID_GET_REPORT from uhid-dev");

            answer.type_ = UHID_GET_REPORT_REPLY;
            answer.u.get_report_reply.id = ev.u.get_report.id;
            answer.u.get_report_reply.err = if ev.u.get_report.rnum == 1 { 0 } else { -EIO };
            answer.u.get_report_reply.size = size_of::<[__u8; 2]>() as _;
            memcpy(
                answer.u.get_report_reply.data.as_mut_ptr() as *mut c_void,
                feature_data.as_ptr() as *const c_void,
                size_of::<[__u8; 2]>(),
            );

            uhid_write(_metadata, fd, &answer);
        }
        UHID_SET_REPORT => {
            UHID_LOG!("UHID_SET_REPORT from uhid-dev");

            answer.type_ = UHID_SET_REPORT_REPLY;
            answer.u.set_report_reply.id = ev.u.set_report.id;
            answer.u.set_report_reply.err = 0; /* success */

            uhid_write(_metadata, fd, &answer);
        }
        _ => {
            TH_LOG!("Invalid event from uhid-dev: %u", ev.type_);
        }
    }

    return 0;
}

#[repr(C)]
pub struct uhid_thread_args {
    pub fd: c_int,
    pub _metadata: *mut __test_metadata,
}

unsafe extern "C" fn uhid_read_events_thread(arg: *mut c_void) -> *mut c_void {
    let args: *mut uhid_thread_args = arg as *mut uhid_thread_args;
    let _metadata: *mut __test_metadata = (*args)._metadata;
    let mut pfds: [pollfd; 1] = [core::mem::zeroed()];
    let fd: c_int = (*args).fd;
    let mut ret: c_int = 0;

    pfds[0].fd = fd;
    pfds[0].events = POLLIN;

    uhid_stopped = false;

    while !uhid_stopped {
        ret = poll(pfds.as_mut_ptr(), 1, 100);
        if ret < 0 {
            TH_LOG!("Cannot poll for fds: %m");
            break;
        }
        if (pfds[0].revents & POLLIN) != 0 {
            ret = uhid_event(_metadata, fd);
            if ret != 0 {
                break;
            }
        }
    }

    return ret as c_long as *mut c_void;
}

unsafe fn uhid_start_listener(
    _metadata: *mut __test_metadata,
    tid: *mut pthread_t,
    uhid_fd: c_int,
) -> c_int {
    let mut args = uhid_thread_args {
        fd: uhid_fd,
        _metadata,
    };
    let err: c_int;

    pthread_mutex_lock(ptr::addr_of_mut!(uhid_started_mtx));
    err = pthread_create(
        tid,
        ptr::null(),
        Some(uhid_read_events_thread),
        &mut args as *mut uhid_thread_args as *mut c_void,
    );
    ASSERT_EQ!(0, err);
    if err != 0 {
        TH_LOG!("Could not start the uhid thread: %d", err);
        pthread_mutex_unlock(ptr::addr_of_mut!(uhid_started_mtx));
        close(uhid_fd);
        return -EIO;
    }
    pthread_cond_wait(
        ptr::addr_of_mut!(uhid_started),
        ptr::addr_of_mut!(uhid_started_mtx),
    );
    pthread_mutex_unlock(ptr::addr_of_mut!(uhid_started_mtx));

    return 0;
}

unsafe fn uhid_send_event(
    _metadata: *mut __test_metadata,
    hid: *mut uhid_device,
    buf: *mut __u8,
    size: size_t,
) -> c_int {
    let mut ev: uhid_event = core::mem::zeroed();

    if size > size_of_val(&ev.u.input.data) {
        return -E2BIG;
    }

    memset(
        &mut ev as *mut uhid_event as *mut c_void,
        0,
        size_of::<uhid_event>(),
    );
    ev.type_ = UHID_INPUT2;
    ev.u.input2.size = size;

    memcpy(
        ev.u.input2.data.as_mut_ptr() as *mut c_void,
        buf as *const c_void,
        size,
    );

    return uhid_write(_metadata, (*hid).uhid_fd, &ev);
}

unsafe fn match_sysfs_device(
    hid: *mut uhid_device,
    workdir: *const c_char,
    dir: *mut dirent,
) -> bool {
    let mut target: [c_char; 20] = [0; 20];
    let mut phys: [c_char; 512] = [0; 512];
    let mut uevent: [c_char; 1024] = [0; 1024];
    let mut temp: [c_char; 512] = [0; 512];
    let fd: c_int;
    let nread: c_int;
    let mut found: bool = false;

    snprintf(
        target.as_mut_ptr(),
        size_of::<[c_char; 20]>(),
        c"%04X:%04X:%04X.*".as_ptr(),
        (*hid).bus as c_int,
        (*hid).vid,
        (*hid).pid,
    );

    if fnmatch(target.as_ptr(), (*dir).d_name.as_ptr(), 0) != 0 {
        return false;
    }

    /* we found the correct VID/PID, now check for phys */
    sprintf(
        uevent.as_mut_ptr(),
        c"%s/%s/uevent".as_ptr(),
        workdir,
        (*dir).d_name.as_ptr(),
    );

    fd = open(uevent.as_ptr(), O_RDONLY | O_NONBLOCK);
    if fd < 0 {
        return false;
    }

    sprintf(phys.as_mut_ptr(), c"PHYS=%d".as_ptr(), (*hid).dev_id);

    nread = read(
        fd,
        temp.as_mut_ptr() as *mut c_void,
        ARRAY_SIZE!(temp),
    ) as c_int;
    if nread > 0 && !strstr(temp.as_ptr(), phys.as_ptr()).is_null() {
        found = true;
    }

    close(fd);

    return found;
}

unsafe fn get_hid_id(hid: *mut uhid_device) -> c_int {
    let workdir: *const c_char = c"/sys/devices/virtual/misc/uhid".as_ptr();
    let mut str_id: *const c_char;
    let mut d: *mut DIR;
    let mut dir: *mut dirent;
    let mut found: c_int = -1;
    let mut attempts: c_int = 3;

    /* it would be nice to be able to use nftw, but the no_alu32 target doesn't support it */

    while found < 0 && attempts > 0 {
        attempts -= 1;
        d = opendir(workdir);
        if !d.is_null() {
            loop {
                dir = readdir(d);
                if dir.is_null() {
                    break;
                }
                if !match_sysfs_device(hid, workdir, dir) {
                    continue;
                }

                str_id = (*dir)
                    .d_name
                    .as_ptr()
                    .add(size_of::<[c_char; 15]>());
                found = strtol(str_id, ptr::null_mut(), 16) as c_int;

                break;
            }
            closedir(d);
        }
        if found < 0 {
            usleep(100000);
        }
    }

    return found;
}

unsafe fn get_hidraw(hid: *mut uhid_device) -> c_int {
    let workdir: *const c_char = c"/sys/devices/virtual/misc/uhid".as_ptr();
    let mut sysfs: [c_char; 1024] = [0; 1024];
    let mut d: *mut DIR;
    let mut subd: *mut DIR;
    let mut dir: *mut dirent;
    let mut subdir: *mut dirent;
    let mut i: c_int;
    let mut found: c_int = -1;

    /* retry 5 times in case the system is loaded */
    i = 5;
    while i > 0 {
        usleep(10);
        d = opendir(workdir);

        if d.is_null() {
            i -= 1;
            continue;
        }

        loop {
            dir = readdir(d);
            if dir.is_null() {
                break;
            }
            if !match_sysfs_device(hid, workdir, dir) {
                continue;
            }

            sprintf(
                sysfs.as_mut_ptr(),
                c"%s/%s/hidraw".as_ptr(),
                workdir,
                (*dir).d_name.as_ptr(),
            );

            subd = opendir(sysfs.as_ptr());
            if subd.is_null() {
                continue;
            }

            loop {
                subdir = readdir(subd);
                if subdir.is_null() {
                    break;
                }
                if fnmatch(c"hidraw*".as_ptr(), (*subdir).d_name.as_ptr(), 0) != 0 {
                    continue;
                }

                found = atoi((*subdir).d_name.as_ptr().add(strlen(c"hidraw".as_ptr()))) as c_int;
            }

            closedir(subd);

            if found > 0 {
                break;
            }
        }
        closedir(d);
        i -= 1;
    }

    return found;
}

unsafe fn open_hidraw(hid: *mut uhid_device) -> c_int {
    let hidraw_number: c_int;
    let mut hidraw_path: [c_char; 64] = [0; 64];

    hidraw_number = get_hidraw(hid);
    if hidraw_number < 0 {
        return hidraw_number;
    }

    /* open hidraw node to check the other side of the pipe */
    sprintf(
        hidraw_path.as_mut_ptr(),
        c"/dev/hidraw%d".as_ptr(),
        hidraw_number,
    );
    return open(hidraw_path.as_ptr(), O_RDWR | O_NONBLOCK);
}

unsafe fn setup_uhid(
    _metadata: *mut __test_metadata,
    hid: *mut uhid_device,
    bus: __u16,
    vid: __u32,
    pid: __u32,
    rdesc: *const __u8,
    rdesc_size: size_t,
) -> c_int {
    let path: *const c_char = c"/dev/uhid".as_ptr();
    let mut t: time_t = core::mem::zeroed();
    let mut ret: c_int;

    /* initialize random number generator */
    srand(time(&mut t) as c_uint);

    (*hid).dev_id = rand() % 1024;
    (*hid).bus = bus;
    (*hid).vid = vid;
    (*hid).pid = pid;

    (*hid).uhid_fd = open(path, O_RDWR | O_CLOEXEC);
    ASSERT_GE!((*hid).uhid_fd, 0);
    TH_LOG!("open uhid-cdev failed; %d", (*hid).uhid_fd);

    ret = uhid_create(
        _metadata,
        (*hid).uhid_fd,
        (*hid).dev_id,
        bus,
        vid,
        pid,
        rdesc as *mut __u8,
        rdesc_size,
    );
    ASSERT_EQ!(0, ret);
    if ret != 0 {
        TH_LOG!("create uhid device failed: %d", ret);
        close((*hid).uhid_fd);
        return ret;
    }

    /* locate the uevent file of the created device */
    (*hid).hid_id = get_hid_id(hid);
    ASSERT_GT!((*hid).hid_id, 0);
    TH_LOG!("Could not locate uhid device id: %d", (*hid).hid_id);

    ret = uhid_start_listener(_metadata, &mut (*hid).tid, (*hid).uhid_fd);
    ASSERT_EQ!(0, ret);
    if ret != 0 {
        TH_LOG!("could not start udev listener: %d", ret);
        close((*hid).uhid_fd);
        return ret;
    }

    return 0;
}

