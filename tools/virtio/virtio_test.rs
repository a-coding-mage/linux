// SPDX-License-Identifier: GPL-2.0
// Translated from virtio/virtio_test.c.
// C include dependency intent: getopt.h, limits.h, string.h, poll.h,
// sys/eventfd.h, stdlib.h, assert.h, unistd.h, sys/ioctl.h, sys/stat.h,
// sys/types.h, fcntl.h, stdbool.h, linux/virtio_types.h, linux/vhost.h,
// linux/virtio.h, linux/virtio_ring.h, ../../drivers/vhost/test.h.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem::{offset_of, size_of};
use core::ptr;

const RANDOM_BATCH: c_int = -1;
const INT_MAX: c_int = 2147483647;

type size_t = usize;
type uint64_t = u64;

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct spinlock_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct virtio_device {
    pub features: u64,
    pub vqs: list_head,
    pub vqs_list_lock: spinlock_t,
}

#[repr(C)]
pub struct virtqueue {
    pub priv_: *mut c_void,
}

#[repr(C)]
pub struct vring {
    pub num: c_uint,
    pub desc: *mut c_void,
    pub avail: *mut c_void,
    pub used: *mut c_void,
}

#[repr(C)]
pub struct scatterlist {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pollfd {
    pub fd: c_int,
    pub events: i16,
    pub revents: i16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vhost_vring_file {
    pub index: c_uint,
    pub fd: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vhost_vring_state {
    pub index: c_uint,
    pub num: c_uint,
}

#[repr(C)]
pub struct vhost_vring_addr {
    pub index: c_uint,
    pub flags: c_uint,
    pub desc_user_addr: uint64_t,
    pub used_user_addr: uint64_t,
    pub avail_user_addr: uint64_t,
    pub log_guest_addr: uint64_t,
}

#[repr(C)]
pub struct vhost_memory_region {
    pub guest_phys_addr: u64,
    pub memory_size: u64,
    pub userspace_addr: u64,
    pub flags_padding: u64,
}

#[repr(C)]
pub struct vhost_memory {
    pub nregions: u32,
    pub padding: u32,
    pub regions: [vhost_memory_region; 0],
}

#[repr(C)]
pub struct option {
    pub name: *const c_char,
    pub has_arg: c_int,
    pub flag: *mut c_int,
    pub val: c_int,
}

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
pub struct vq_info {
    pub kick: c_int,
    pub call: c_int,
    pub num: c_int,
    pub idx: c_int,
    pub ring: *mut c_void,
    /* copy used for control */
    pub vring: vring,
    pub vq: *mut virtqueue,
}

#[repr(C)]
pub struct vdev_info {
    pub vdev: virtio_device,
    pub control: c_int,
    pub fds: [pollfd; 1],
    pub vqs: [vq_info; 1],
    pub nvqs: c_int,
    pub buf: *mut c_void,
    pub buf_size: size_t,
    pub mem: *mut vhost_memory,
}

extern "C" {
    static mut optarg: *mut c_char;
    static mut stderr: *mut FILE;

    static VHOST_SET_FEATURES: c_ulong;
    static VHOST_SET_VRING_NUM: c_ulong;
    static VHOST_SET_VRING_BASE: c_ulong;
    static VHOST_SET_VRING_ADDR: c_ulong;
    static VHOST_SET_VRING_KICK: c_ulong;
    static VHOST_SET_VRING_CALL: c_ulong;
    static VHOST_SET_OWNER: c_ulong;
    static VHOST_SET_MEM_TABLE: c_ulong;
    static VHOST_TEST_RUN: c_ulong;
    static VHOST_TEST_SET_BACKEND: c_ulong;
    static VHOST_GET_VRING_BASE: c_ulong;

    static VIRTIO_RING_F_INDIRECT_DESC: c_int;
    static VIRTIO_RING_F_EVENT_IDX: c_int;
    static VIRTIO_F_VERSION_1: c_int;

    static EFD_NONBLOCK: c_int;
    static O_RDWR: c_int;
    static POLLIN: i16;
    static GFP_ATOMIC: c_uint;
    static ENOSPC: c_int;
    static required_argument: c_int;
    static optional_argument: c_int;

    fn write(fd: c_int, buf: *const c_void, count: size_t) -> isize;
    fn read(fd: c_int, buf: *mut c_void, count: size_t) -> isize;
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
    fn eventfd(initval: c_uint, flags: c_int) -> c_int;
    fn posix_memalign(memptr: *mut *mut c_void, alignment: size_t, size: size_t) -> c_int;
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn malloc(size: size_t) -> *mut c_void;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn poll(fds: *mut pollfd, nfds: c_ulong, timeout: c_int) -> c_int;
    fn random() -> c_long;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strtol(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_long;
    fn getopt_long(
        argc: c_int,
        argv: *mut *mut c_char,
        optstring: *const c_char,
        longopts: *const option,
        longindex: *mut c_int,
    ) -> c_int;
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn exit(status: c_int) -> !;

    fn INIT_LIST_HEAD(list: *mut list_head);
    fn spin_lock_init(lock: *mut spinlock_t);
    fn vring_size(num: c_uint, align: c_ulong) -> c_ulong;
    fn vring_init(vr: *mut vring, num: c_uint, p: *mut c_void, align: c_ulong);
    fn vring_new_virtqueue(
        index: c_uint,
        num: c_uint,
        vring_align: c_uint,
        vdev: *mut virtio_device,
        weak_barriers: bool,
        may_reduce_num: bool,
        pages: *mut c_void,
        notify: extern "C" fn(*mut virtqueue) -> bool,
        callback: extern "C" fn(*mut virtqueue),
        name: *const c_char,
    ) -> *mut virtqueue;
    fn vring_del_virtqueue(vq: *mut virtqueue);
    fn virtqueue_disable_cb(vq: *mut virtqueue);
    fn virtqueue_enable_cb(vq: *mut virtqueue) -> bool;
    fn virtqueue_enable_cb_delayed(vq: *mut virtqueue) -> bool;
    fn virtqueue_add_outbuf(
        vq: *mut virtqueue,
        sg: *mut scatterlist,
        num: c_uint,
        data: *mut c_void,
        gfp: c_uint,
    ) -> c_int;
    fn virtqueue_kick(vq: *mut virtqueue) -> bool;
    fn virtqueue_get_buf(vq: *mut virtqueue, len: *mut c_uint) -> *mut c_void;
    fn sg_init_one(sg: *mut scatterlist, buf: *const c_void, buflen: c_uint);
}

/* Unused */
#[no_mangle]
pub static mut __kmalloc_fake: *mut c_void = ptr::null_mut();
#[no_mangle]
pub static mut __kfree_ignore_start: *mut c_void = ptr::null_mut();
#[no_mangle]
pub static mut __kfree_ignore_end: *mut c_void = ptr::null_mut();

static no_backend: vhost_vring_file = vhost_vring_file { index: 0, fd: -1 };
static backend: vhost_vring_file = vhost_vring_file { index: 0, fd: 1 };
static null_state: vhost_vring_state = vhost_vring_state { index: 0, num: 0 };

#[inline]
fn unlikely(x: bool) -> bool {
    x
}

#[no_mangle]
pub unsafe extern "C" fn vq_notify(vq: *mut virtqueue) -> bool {
    let info = (*vq).priv_ as *mut vq_info;
    let v: u64 = 1;
    let r: c_int = write((*info).kick, &v as *const _ as *const c_void, size_of::<u64>()) as c_int;
    assert!(r == size_of::<u64>() as c_int);
    true
}

#[no_mangle]
pub extern "C" fn vq_callback(_vq: *mut virtqueue) {}

#[no_mangle]
pub unsafe extern "C" fn vhost_vq_setup(dev: *mut vdev_info, info: *mut vq_info) {
    let mut state = vhost_vring_state {
        index: (*info).idx as c_uint,
        num: 0,
    };
    let mut file = vhost_vring_file {
        index: (*info).idx as c_uint,
        fd: 0,
    };
    let mut features: u64 = (*dev).vdev.features;
    let mut addr = vhost_vring_addr {
        index: (*info).idx as c_uint,
        flags: 0,
        desc_user_addr: (*info).vring.desc as c_ulong as uint64_t,
        avail_user_addr: (*info).vring.avail as c_ulong as uint64_t,
        used_user_addr: (*info).vring.used as c_ulong as uint64_t,
        log_guest_addr: 0,
    };
    let mut r: c_int;
    r = ioctl((*dev).control, VHOST_SET_FEATURES, &mut features);
    assert!(r >= 0);
    state.num = (*info).vring.num;
    r = ioctl((*dev).control, VHOST_SET_VRING_NUM, &mut state);
    assert!(r >= 0);
    state.num = 0;
    r = ioctl((*dev).control, VHOST_SET_VRING_BASE, &mut state);
    assert!(r >= 0);
    r = ioctl((*dev).control, VHOST_SET_VRING_ADDR, &mut addr);
    assert!(r >= 0);
    file.fd = (*info).kick;
    r = ioctl((*dev).control, VHOST_SET_VRING_KICK, &mut file);
    assert!(r >= 0);
    file.fd = (*info).call;
    r = ioctl((*dev).control, VHOST_SET_VRING_CALL, &mut file);
    assert!(r >= 0);
}

unsafe fn vq_reset(info: *mut vq_info, num: c_int, vdev: *mut virtio_device) {
    if !(*info).vq.is_null() {
        vring_del_virtqueue((*info).vq);
    }

    memset((*info).ring, 0, vring_size(num as c_uint, 4096) as size_t);
    vring_init(&mut (*info).vring, num as c_uint, (*info).ring, 4096);
    (*info).vq = vring_new_virtqueue(
        (*info).idx as c_uint,
        num as c_uint,
        4096,
        vdev,
        true,
        false,
        (*info).ring,
        vq_notify,
        vq_callback,
        b"test\0".as_ptr() as *const c_char,
    );
    assert!(!(*info).vq.is_null());
    (*(*info).vq).priv_ = info as *mut c_void;
}

unsafe fn vq_info_add(dev: *mut vdev_info, num: c_int) {
    let info = &mut (*dev).vqs[(*dev).nvqs as usize] as *mut vq_info;
    let mut r: c_int;
    (*info).idx = (*dev).nvqs;
    (*info).kick = eventfd(0, EFD_NONBLOCK);
    (*info).call = eventfd(0, EFD_NONBLOCK);
    r = posix_memalign(
        &mut (*info).ring,
        4096,
        vring_size(num as c_uint, 4096) as size_t,
    );
    assert!(r >= 0);
    vq_reset(info, num, &mut (*dev).vdev);
    vhost_vq_setup(dev, info);
    (*dev).fds[(*info).idx as usize].fd = (*info).call;
    (*dev).fds[(*info).idx as usize].events = POLLIN;
    (*dev).nvqs += 1;
}

unsafe fn vdev_info_init(dev: *mut vdev_info, features: u64) {
    let mut r: c_int;
    memset(
        dev as *mut c_void,
        0,
        size_of::<vdev_info>(),
    );
    (*dev).vdev.features = features;
    INIT_LIST_HEAD(&mut (*dev).vdev.vqs);
    spin_lock_init(&mut (*dev).vdev.vqs_list_lock);
    (*dev).buf_size = 1024;
    (*dev).buf = malloc((*dev).buf_size);
    assert!(!(*dev).buf.is_null());
    (*dev).control = open(b"/dev/vhost-test\0".as_ptr() as *const c_char, O_RDWR);
    assert!((*dev).control >= 0);
    r = ioctl((*dev).control, VHOST_SET_OWNER, ptr::null_mut::<c_void>());
    assert!(r >= 0);
    let mem_size = offset_of!(vhost_memory, regions) + size_of::<vhost_memory_region>();
    (*dev).mem = malloc(mem_size) as *mut vhost_memory;
    assert!(!(*dev).mem.is_null());
    memset((*dev).mem as *mut c_void, 0, mem_size);
    (*dev).mem.write(vhost_memory {
        nregions: 1,
        padding: 0,
        regions: [],
    });
    let region0 = ((*dev).mem as *mut u8).add(offset_of!(vhost_memory, regions))
        as *mut vhost_memory_region;
    (*region0).guest_phys_addr = (*dev).buf as c_long as u64;
    (*region0).userspace_addr = (*dev).buf as c_long as u64;
    (*region0).memory_size = (*dev).buf_size as u64;
    r = ioctl((*dev).control, VHOST_SET_MEM_TABLE, (*dev).mem);
    assert!(r >= 0);
}

/* TODO: this is pretty bad: we get a cache line bounce
 * for the wait queue on poll and another one on read,
 * plus the read which is there just to clear the
 * current state. */
unsafe fn wait_for_interrupt(dev: *mut vdev_info) {
    let mut i: c_int;
    let mut val: u64 = 0;
    poll((*dev).fds.as_mut_ptr(), (*dev).nvqs as c_ulong, -1);
    i = 0;
    while i < (*dev).nvqs {
        if ((*dev).fds[i as usize].revents & POLLIN) != 0 {
            read(
                (*dev).fds[i as usize].fd,
                &mut val as *mut _ as *mut c_void,
                size_of::<u64>(),
            );
        }
        i += 1;
    }
}

unsafe fn run_test(
    dev: *mut vdev_info,
    vq: *mut vq_info,
    delayed: bool,
    mut batch: c_long,
    reset_n: c_int,
    bufs: c_int,
) {
    let mut sl: scatterlist = core::mem::zeroed();
    let mut started: c_long = 0;
    let mut completed: c_long = 0;
    let mut next_reset: c_long = reset_n as c_long;
    let mut completed_before: c_long;
    let mut started_before: c_long;
    let mut r: c_int;
    let mut test: c_int = 1;
    let mut len: c_uint = 0;
    let mut spurious: i64 = 0;
    let random_batch: bool = batch == RANDOM_BATCH as c_long;

    r = ioctl((*dev).control, VHOST_TEST_RUN, &mut test);
    assert!(r >= 0);
    if reset_n == 0 {
        next_reset = INT_MAX as c_long;
    }

    loop {
        virtqueue_disable_cb((*vq).vq);
        completed_before = completed;
        started_before = started;
        loop {
            let reset: bool = completed > next_reset;
            if random_batch {
                batch = (random() % (*vq).vring.num as c_long) + 1;
            }

            while started < bufs as c_long && (started - completed) < batch {
                sg_init_one(&mut sl, (*dev).buf, (*dev).buf_size as c_uint);
                r = virtqueue_add_outbuf(
                    (*vq).vq,
                    &mut sl,
                    1,
                    ((*dev).buf as *mut u8).offset(started as isize) as *mut c_void,
                    GFP_ATOMIC,
                );
                if unlikely(r != 0) {
                    if r == -ENOSPC && started > started_before {
                        r = 0;
                    } else {
                        r = -1;
                    }
                    break;
                }

                started += 1;

                if unlikely(!virtqueue_kick((*vq).vq)) {
                    r = -1;
                    break;
                }
            }

            if started >= bufs as c_long {
                r = -1;
            }

            if reset {
                r = ioctl((*dev).control, VHOST_TEST_SET_BACKEND, &no_backend);
                assert!(r == 0);
            }

            /* Flush out completed bufs if any */
            while !virtqueue_get_buf((*vq).vq, &mut len).is_null() {
                completed += 1;
                r = 0;
            }

            if reset {
                let mut s = vhost_vring_state { index: 0, num: 0 };

                vq_reset(vq, (*vq).vring.num as c_int, &mut (*dev).vdev);

                r = ioctl((*dev).control, VHOST_GET_VRING_BASE, &mut s);
                assert!(r == 0);

                s.num = 0;
                r = ioctl((*dev).control, VHOST_SET_VRING_BASE, &null_state);
                assert!(r == 0);

                r = ioctl((*dev).control, VHOST_TEST_SET_BACKEND, &backend);
                assert!(r == 0);

                started = completed;
                while completed > next_reset {
                    next_reset += completed;
                }
            }

            if r != 0 {
                break;
            }
        }
        if completed == completed_before && started == started_before {
            spurious += 1;
        }
        assert!(completed <= bufs as c_long);
        assert!(started <= bufs as c_long);
        if completed == bufs as c_long {
            break;
        }
        if delayed {
            if virtqueue_enable_cb_delayed((*vq).vq) {
                wait_for_interrupt(dev);
            }
        } else if virtqueue_enable_cb((*vq).vq) {
            wait_for_interrupt(dev);
        }
    }
    test = 0;
    r = ioctl((*dev).control, VHOST_TEST_RUN, &mut test);
    assert!(r >= 0);
    fprintf(
        stderr,
        b"spurious wakeups: 0x%llx started=0x%lx completed=0x%lx\n\0".as_ptr()
            as *const c_char,
        spurious,
        started,
        completed,
    );
}

#[no_mangle]
pub static optstring: [c_char; 2] = [b'h' as c_char, 0];

#[no_mangle]
pub static longopts: [option; 14] = [
    option {
        name: b"help\0".as_ptr() as *const c_char,
        has_arg: 0,
        flag: ptr::null_mut(),
        val: b'h' as c_int,
    },
    option {
        name: b"event-idx\0".as_ptr() as *const c_char,
        has_arg: 0,
        flag: ptr::null_mut(),
        val: b'E' as c_int,
    },
    option {
        name: b"no-event-idx\0".as_ptr() as *const c_char,
        has_arg: 0,
        flag: ptr::null_mut(),
        val: b'e' as c_int,
    },
    option {
        name: b"indirect\0".as_ptr() as *const c_char,
        has_arg: 0,
        flag: ptr::null_mut(),
        val: b'I' as c_int,
    },
    option {
        name: b"no-indirect\0".as_ptr() as *const c_char,
        has_arg: 0,
        flag: ptr::null_mut(),
        val: b'i' as c_int,
    },
    option {
        name: b"virtio-1\0".as_ptr() as *const c_char,
        has_arg: 0,
        flag: ptr::null_mut(),
        val: b'1' as c_int,
    },
    option {
        name: b"no-virtio-1\0".as_ptr() as *const c_char,
        has_arg: 0,
        flag: ptr::null_mut(),
        val: b'0' as c_int,
    },
    option {
        name: b"delayed-interrupt\0".as_ptr() as *const c_char,
        has_arg: 0,
        flag: ptr::null_mut(),
        val: b'D' as c_int,
    },
    option {
        name: b"no-delayed-interrupt\0".as_ptr() as *const c_char,
        has_arg: 0,
        flag: ptr::null_mut(),
        val: b'd' as c_int,
    },
    option {
        name: b"batch\0".as_ptr() as *const c_char,
        has_arg: unsafe { required_argument },
        flag: ptr::null_mut(),
        val: b'b' as c_int,
    },
    option {
        name: b"reset\0".as_ptr() as *const c_char,
        has_arg: unsafe { optional_argument },
        flag: ptr::null_mut(),
        val: b'r' as c_int,
    },
    option {
        name: ptr::null(),
        has_arg: 0,
        flag: ptr::null_mut(),
        val: 0,
    },
    option {
        name: ptr::null(),
        has_arg: 0,
        flag: ptr::null_mut(),
        val: 0,
    },
    option {
        name: ptr::null(),
        has_arg: 0,
        flag: ptr::null_mut(),
        val: 0,
    },
];

unsafe fn help(status: c_int) -> ! {
    fprintf(
        stderr,
        b"Usage: virtio_test [--help] [--no-indirect] [--no-event-idx] [--no-virtio-1] [--delayed-interrupt] [--batch=random/N] [--reset=N]\n\0"
            .as_ptr() as *const c_char,
    );

    exit(status);
}

#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut dev: vdev_info = core::mem::zeroed();
    let mut features: u64 = (1_u64 << VIRTIO_RING_F_INDIRECT_DESC)
        | (1_u64 << VIRTIO_RING_F_EVENT_IDX)
        | (1_u64 << VIRTIO_F_VERSION_1);
    let mut batch: c_long = 1;
    let mut reset: c_long = 0;
    let mut o: c_int;
    let mut delayed: bool = false;

    loop {
        o = getopt_long(argc, argv, optstring.as_ptr(), longopts.as_ptr(), ptr::null_mut());
        match o {
            -1 => break,
            x if x == b'?' as c_int => help(2),
            x if x == b'e' as c_int => {
                features &= !(1_u64 << VIRTIO_RING_F_EVENT_IDX);
            }
            x if x == b'h' as c_int => help(0),
            x if x == b'i' as c_int => {
                features &= !(1_u64 << VIRTIO_RING_F_INDIRECT_DESC);
            }
            x if x == b'0' as c_int => {
                features &= !(1_u64 << VIRTIO_F_VERSION_1);
            }
            x if x == b'D' as c_int => {
                delayed = true;
            }
            x if x == b'b' as c_int => {
                if strcmp(optarg, b"random\0".as_ptr() as *const c_char) == 0 {
                    batch = RANDOM_BATCH as c_long;
                } else {
                    batch = strtol(optarg, ptr::null_mut(), 10);
                    assert!(batch > 0);
                    assert!(batch < INT_MAX as c_long + 1);
                }
            }
            x if x == b'r' as c_int => {
                if optarg.is_null() {
                    reset = 1;
                } else {
                    reset = strtol(optarg, ptr::null_mut(), 10);
                    assert!(reset > 0);
                    assert!(reset < INT_MAX as c_long + 1);
                }
            }
            _ => {
                assert!(false);
            }
        }
    }

    vdev_info_init(&mut dev, features);
    vq_info_add(&mut dev, 256);
    run_test(
        &mut dev,
        &mut dev.vqs[0],
        delayed,
        batch,
        reset as c_int,
        0x100000,
    );
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
