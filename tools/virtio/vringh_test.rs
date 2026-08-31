// SPDX-License-Identifier: GPL-2.0
/* Simple test of virtio code, entirely in userspace. */
/* C dependencies: sched.h, err.h, linux/kernel.h, linux/err.h, linux/virtio.h,
 * linux/vringh.h, linux/virtio_ring.h, linux/virtio_config.h, linux/uaccess.h,
 * sys/types.h, sys/stat.h, sys/mman.h, sys/wait.h, fcntl.h.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::{size_of, zeroed};
use core::ptr::{null, null_mut};

type bool_t = bool;
type u16_t = u16;
type u64_t = u64;

const USER_MEM: usize = 1024 * 1024;
const RINGSIZE: u32 = 256;
const ALIGN: u32 = 4096;
const NUM_XFERS: c_ulong = 10000000;

const O_RDWR: c_int = 0o2;
const O_CREAT: c_int = 0o100;
const O_TRUNC: c_int = 0o1000;
const PROT_READ: c_int = 0x1;
const PROT_WRITE: c_int = 0x2;
const MAP_SHARED: c_int = 0x01;
const MAP_FAILED: *mut c_void = !0usize as *mut c_void;
const GFP_KERNEL: c_uint = 0;
const ENOSPC: c_int = 28;
const PAGE_SIZE: usize = 4096;

const VIRTIO_RING_F_INDIRECT_DESC: c_uint = 28;
const VIRTIO_RING_F_EVENT_IDX: c_uint = 29;
const VIRTIO_F_VERSION_1: c_uint = 32;
const VRING_DESC_F_NEXT: u16_t = 1;
const VRING_DESC_F_INDIRECT: u16_t = 4;
const VRINGH_IOV_ALLOCATED: c_uint = 0x80000000;

#[repr(C)]
pub struct cpu_set_t {
    bits: [c_ulong; 16],
}

#[repr(C)]
pub struct list_head {
    next: *mut list_head,
    prev: *mut list_head,
}

#[repr(C)]
pub struct spinlock_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct virtio_device {
    features: u64_t,
    vqs: list_head,
    vqs_list_lock: spinlock_t,
}

#[repr(C)]
pub struct virtqueue {
    vdev: *mut virtio_device,
}

#[repr(C)]
pub struct vring_desc {
    addr: u64_t,
    len: u32,
    flags: u16_t,
    next: u16_t,
}

#[repr(C)]
pub struct vring_avail {
    flags: u16_t,
    idx: u16_t,
    ring: [u16_t; 0],
}

#[repr(C)]
pub struct vring_used_elem {
    id: u32,
    len: u32,
}

#[repr(C)]
pub struct vring_used {
    flags: u16_t,
    idx: u16_t,
    ring: [vring_used_elem; 0],
}

#[repr(C)]
pub struct vring {
    num: c_uint,
    desc: *mut vring_desc,
    avail: *mut vring_avail,
    used: *mut vring_used,
}

#[repr(C)]
pub struct vringh {
    vring: vring,
    last_avail_idx: u16_t,
    weak_barriers: bool_t,
}

#[repr(C)]
pub struct vringh_range {
    start: u64_t,
    end_incl: u64_t,
    offset: u64_t,
}

#[repr(C)]
pub struct iovec {
    iov_base: *mut c_void,
    iov_len: usize,
}

#[repr(C)]
pub struct vringh_iov {
    iov: *mut iovec,
    used: c_uint,
    i: c_uint,
    max_num: c_uint,
}

#[repr(C)]
pub struct scatterlist {
    _private: [u8; 0],
}

#[repr(C)]
struct guest_virtio_device {
    vdev: virtio_device,
    to_host_fd: c_int,
    notifies: c_ulong,
}

static mut __user_addr_min: *mut c_void = null_mut();
static mut __user_addr_max: *mut c_void = null_mut();
static mut __kmalloc_fake: *mut c_void = null_mut();
static mut __kfree_ignore_start: *mut c_void = null_mut();
static mut __kfree_ignore_end: *mut c_void = null_mut();
static mut user_addr_offset: u64_t = 0;

unsafe extern "C" {
    fn abort() -> !;
    fn write(fd: c_int, buf: *const c_void, count: usize) -> isize;
    fn read(fd: c_int, buf: *mut c_void, count: usize) -> isize;
    fn open(path: *const c_char, flags: c_int, mode: c_uint) -> c_int;
    fn ftruncate(fd: c_int, length: isize) -> c_int;
    fn mmap(addr: *mut c_void, length: usize, prot: c_int, flags: c_int, fd: c_int, offset: isize) -> *mut c_void;
    fn munmap(addr: *mut c_void, length: usize) -> c_int;
    fn pipe(pipefd: *mut c_int) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn fork() -> c_int;
    fn wait(status: *mut c_int) -> c_int;
    fn getpid() -> c_int;
    fn getpagesize() -> c_int;
    fn sched_setaffinity(pid: c_int, cpusetsize: usize, mask: *const cpu_set_t) -> c_int;
    fn printf(fmt: *const c_char, ...) -> c_int;
    fn fflush(stream: *mut c_void) -> c_int;
    static mut stdout: *mut c_void;
    fn err(eval: c_int, fmt: *const c_char, ...) -> !;
    fn errx(eval: c_int, fmt: *const c_char, ...) -> !;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn memcmp(s1: *const c_void, s2: *const c_void, n: usize) -> c_int;
    fn posix_memalign(memptr: *mut *mut c_void, alignment: usize, size: usize) -> c_int;
    fn free(ptr: *mut c_void);

    fn vring_size(num: c_uint, align: c_ulong) -> c_uint;
    fn vring_init(vr: *mut vring, num: c_uint, p: *mut c_void, align: c_ulong);
    fn vringh_init_user(vrh: *mut vringh, features: u64_t, num: c_uint, weak_barriers: bool_t, desc: *mut vring_desc, avail: *mut vring_avail, used: *mut vring_used);
    fn vringh_need_notify_user(vrh: *mut vringh) -> c_int;
    fn vringh_notify_enable_user(vrh: *mut vringh) -> bool_t;
    fn vringh_notify_disable_user(vrh: *mut vringh);
    fn vringh_getdesc_user(vrh: *mut vringh, riov: *mut vringh_iov, wiov: *mut vringh_iov, getrange: Option<unsafe extern "C" fn(*mut vringh, u64_t, *mut vringh_range) -> bool_t>, head: *mut u16_t) -> c_int;
    fn vringh_iov_init(iov: *mut vringh_iov, iovec: *mut iovec, num: c_uint);
    fn vringh_iov_pull_user(iov: *mut vringh_iov, dst: *mut c_void, len: usize) -> c_int;
    fn vringh_iov_push_user(iov: *mut vringh_iov, src: *const c_void, len: usize) -> c_int;
    fn vringh_iov_cleanup(iov: *mut vringh_iov);
    fn vringh_complete_user(vrh: *mut vringh, head: u16_t, len: u16_t) -> c_int;
    fn vringh_complete_multi_user(vrh: *mut vringh, used: *mut vring_used_elem, num: c_uint) -> c_int;

    fn vring_new_virtqueue(index: c_uint, num: c_uint, vring_align: c_uint, vdev: *mut virtio_device, weak_barriers: bool_t, context: bool_t, pages: *mut c_void, notify: Option<unsafe extern "C" fn(*mut virtqueue) -> bool_t>, callback: Option<unsafe extern "C" fn(*mut virtqueue)>, name: *const c_char) -> *mut virtqueue;
    fn vring_del_virtqueue(vq: *mut virtqueue);
    fn virtqueue_get_buf(vq: *mut virtqueue, len: *mut c_uint) -> *mut c_void;
    fn virtqueue_add_sgs(vq: *mut virtqueue, sgs: *mut *mut scatterlist, out_sgs: c_uint, in_sgs: c_uint, data: *mut c_void, gfp: c_uint) -> c_int;
    fn virtqueue_add_outbuf(vq: *mut virtqueue, sg: *mut scatterlist, num: c_uint, data: *mut c_void, gfp: c_uint) -> c_int;
    fn virtqueue_add_inbuf(vq: *mut virtqueue, sg: *mut scatterlist, num: c_uint, data: *mut c_void, gfp: c_uint) -> c_int;
    fn virtqueue_kick(vq: *mut virtqueue) -> bool_t;
    fn virtqueue_enable_cb_delayed(vq: *mut virtqueue) -> bool_t;
    fn virtqueue_disable_cb(vq: *mut virtqueue);
    fn sg_init_table(sgl: *mut scatterlist, nents: c_uint);
    fn sg_set_buf(sg: *mut scatterlist, buf: *const c_void, buflen: c_uint);
    fn __virtio_set_bit(vdev: *mut virtio_device, bit: c_uint);
    fn __virtio_clear_bit(vdev: *mut virtio_device, bit: c_uint);
    fn __virtio_test_bit(vdev: *const virtio_device, bit: c_uint) -> bool_t;
    fn INIT_LIST_HEAD(list: *mut list_head);
    fn spin_lock_init(lock: *mut spinlock_t);
    fn virtio_rmb(weak_barriers: bool_t);
}

fn CPU_ZERO(set: &mut cpu_set_t) {
    set.bits = [0; 16];
}

fn CPU_SET(cpu: c_uint, set: &mut cpu_set_t) {
    let bits = 8 * size_of::<c_ulong>();
    let idx = cpu as usize / bits;
    let bit = cpu as usize % bits;
    if idx < set.bits.len() {
        set.bits[idx] |= 1usize.wrapping_shl(bit as u32) as c_ulong;
    }
}

unsafe fn WIFEXITED(status: c_int) -> bool {
    (status & 0x7f) == 0
}

unsafe fn WEXITSTATUS(status: c_int) -> c_int {
    (status >> 8) & 0xff
}

unsafe fn WTERMSIG(status: c_int) -> c_int {
    status & 0x7f
}

unsafe fn get_user_u16(dst: *mut u16_t, src: *const u16_t) -> c_int {
    *dst = *src;
    0
}

unsafe extern "C" fn never_notify_host(_vq: *mut virtqueue) -> bool_t {
    abort();
}

unsafe extern "C" fn never_callback_guest(_vq: *mut virtqueue) {
    abort();
}

unsafe extern "C" fn getrange_iov(_vrh: *mut vringh, addr: u64_t, r: *mut vringh_range) -> bool_t {
    if addr < __user_addr_min as c_ulong as u64_t - user_addr_offset {
        return false;
    }
    if addr >= __user_addr_max as c_ulong as u64_t - user_addr_offset {
        return false;
    }

    (*r).start = __user_addr_min as c_ulong as u64_t - user_addr_offset;
    (*r).end_incl = __user_addr_max as c_ulong as u64_t - 1 - user_addr_offset;
    (*r).offset = user_addr_offset;
    true
}

/* We return single byte ranges. */
unsafe extern "C" fn getrange_slow(_vrh: *mut vringh, addr: u64_t, r: *mut vringh_range) -> bool_t {
    if addr < __user_addr_min as c_ulong as u64_t - user_addr_offset {
        return false;
    }
    if addr >= __user_addr_max as c_ulong as u64_t - user_addr_offset {
        return false;
    }

    (*r).start = addr;
    (*r).end_incl = (*r).start;
    (*r).offset = user_addr_offset;
    true
}

unsafe extern "C" fn parallel_notify_host(vq: *mut virtqueue) -> bool_t {
    let rc: c_int;
    let gvdev: *mut guest_virtio_device;

    gvdev = (*vq).vdev as *mut guest_virtio_device;
    rc = write((*gvdev).to_host_fd, c"".as_ptr() as *const c_void, 1) as c_int;
    if rc < 0 {
        return false;
    }
    (*gvdev).notifies += 1;
    true
}

unsafe extern "C" fn no_notify_host(_vq: *mut virtqueue) -> bool_t {
    true
}

/* We aim for two "distant" cpus. */
unsafe fn find_cpus(first: *mut c_uint, last: *mut c_uint) {
    *first = !0u32;
    *last = 0;
    for i in 0..4096u32 {
        let mut set: cpu_set_t = zeroed();
        CPU_ZERO(&mut set);
        CPU_SET(i, &mut set);
        if sched_setaffinity(getpid(), size_of::<cpu_set_t>(), &set) == 0 {
            if i < *first {
                *first = i;
            }
            if i > *last {
                *last = i;
            }
        }
    }
}

/* Opencoded version for fast mode */
unsafe fn vringh_get_head(vrh: *mut vringh, head: *mut u16_t) -> c_int {
    let mut avail_idx: u16_t = 0;
    let i: u16_t;
    let mut err: c_int;

    err = get_user_u16(&mut avail_idx, &(*(*vrh).vring.avail).idx);
    if err != 0 {
        return err;
    }

    if (*vrh).last_avail_idx == avail_idx {
        return 0;
    }

    /* Only get avail ring entries after they have been exposed by guest. */
    virtio_rmb((*vrh).weak_barriers);

    i = (*vrh).last_avail_idx & ((*vrh).vring.num - 1) as u16_t;

    err = get_user_u16(head, (*(*vrh).vring.avail).ring.as_ptr().add(i as usize));
    if err != 0 {
        return err;
    }

    (*vrh).last_avail_idx = (*vrh).last_avail_idx.wrapping_add(1);
    1
}

unsafe fn parallel_test(
    features: u64_t,
    getrange: Option<unsafe extern "C" fn(*mut vringh, u64_t, *mut vringh_range) -> bool_t>,
    fast_vringh: bool_t,
) -> c_int {
    let host_map: *mut c_void;
    let guest_map: *mut c_void;
    let mut pipe_ret: c_int;
    let fd: c_int;
    let mut mapsize: c_int;
    let mut to_guest = [0 as c_int; 2];
    let mut to_host = [0 as c_int; 2];
    let mut xfers: c_ulong = 0;
    let mut notifies: c_ulong = 0;
    let mut receives: c_ulong = 0;
    let mut first_cpu: c_uint = 0;
    let mut last_cpu: c_uint = 0;
    let mut cpu_set: cpu_set_t = zeroed();
    let mut buf = [0 as c_char; 128];

    /* Create real file to mmap. */
    fd = open(c"/tmp/vringh_test-file".as_ptr(), O_RDWR | O_CREAT | O_TRUNC, 0o600);
    if fd < 0 {
        err(1, c"Opening /tmp/vringh_test-file".as_ptr());
    }

    /* Extra room at the end for some data, and indirects */
    mapsize = vring_size(RINGSIZE, ALIGN as c_ulong) as c_int
        + RINGSIZE as c_int * 2 * size_of::<c_int>() as c_int
        + RINGSIZE as c_int * 6 * size_of::<vring_desc>() as c_int;
    mapsize = (mapsize + getpagesize() - 1) & !(getpagesize() - 1);
    ftruncate(fd, mapsize as isize);

    /* Parent and child use separate addresses, to check our mapping logic! */
    host_map = mmap(null_mut(), mapsize as usize, PROT_READ | PROT_WRITE, MAP_SHARED, fd, 0);
    if host_map == MAP_FAILED {
        err(1, c"mmap host_map".as_ptr());
    }

    guest_map = mmap(null_mut(), mapsize as usize, PROT_READ | PROT_WRITE, MAP_SHARED, fd, 0);
    if guest_map == MAP_FAILED {
        err(1, c"mmap guest_map".as_ptr());
    }

    pipe_ret = pipe(to_guest.as_mut_ptr());
    assert!(pipe_ret == 0);

    pipe_ret = pipe(to_host.as_mut_ptr());
    assert!(pipe_ret == 0);

    CPU_ZERO(&mut cpu_set);
    find_cpus(&mut first_cpu, &mut last_cpu);
    printf(c"Using CPUS %u and %u\n".as_ptr(), first_cpu, last_cpu);
    fflush(stdout);

    if fork() != 0 {
        let mut vrh: vringh = zeroed();
        let mut status: c_int = 0;
        let mut err: c_int;
        let mut rlen: c_int = 0;
        let mut rbuf = [0 as c_char; 5];

        /* We are the host: never access guest addresses! */
        munmap(guest_map, mapsize as usize);

        __user_addr_min = host_map;
        __user_addr_max = (__user_addr_min as *mut u8).add(mapsize as usize) as *mut c_void;
        user_addr_offset = (host_map as isize - guest_map as isize) as u64_t;
        assert!(user_addr_offset != 0);

        close(to_guest[0]);
        close(to_host[1]);

        vring_init(&mut vrh.vring, RINGSIZE, host_map, ALIGN as c_ulong);
        vringh_init_user(&mut vrh, features, RINGSIZE, true, vrh.vring.desc, vrh.vring.avail, vrh.vring.used);
        CPU_SET(first_cpu, &mut cpu_set);
        if sched_setaffinity(getpid(), size_of::<cpu_set_t>(), &cpu_set) != 0 {
            errx(1, c"Could not set affinity to cpu %u".as_ptr(), first_cpu);
        }

        while xfers < NUM_XFERS {
            let mut host_riov: [iovec; 2] = zeroed();
            let mut host_wiov: [iovec; 2] = zeroed();
            let mut riov: vringh_iov = zeroed();
            let mut wiov: vringh_iov = zeroed();
            let mut head: u16_t = 0;
            let written: u16_t;

            if fast_vringh {
                loop {
                    err = vringh_get_head(&mut vrh, &mut head);
                    if err != 0 {
                        break;
                    }
                    err = vringh_need_notify_user(&mut vrh);
                    if err < 0 {
                        errx(1, c"vringh_need_notify_user: %i".as_ptr(), err);
                    }
                    if err != 0 {
                        write(to_guest[1], c"".as_ptr() as *const c_void, 1);
                        notifies += 1;
                    }
                }
                if err != 1 {
                    errx(1, c"vringh_get_head".as_ptr());
                }
                written = 0;
            } else {
                vringh_iov_init(&mut riov, host_riov.as_mut_ptr(), host_riov.len() as c_uint);
                vringh_iov_init(&mut wiov, host_wiov.as_mut_ptr(), host_wiov.len() as c_uint);

                err = vringh_getdesc_user(&mut vrh, &mut riov, &mut wiov, getrange, &mut head);
                if err == 0 {
                    err = vringh_need_notify_user(&mut vrh);
                    if err < 0 {
                        errx(1, c"vringh_need_notify_user: %i".as_ptr(), err);
                    }
                    if err != 0 {
                        write(to_guest[1], c"".as_ptr() as *const c_void, 1);
                        notifies += 1;
                    }

                    if !vringh_notify_enable_user(&mut vrh) {
                        continue;
                    }

                    /* Swallow all notifies at once. */
                    if read(to_host[0], buf.as_mut_ptr() as *mut c_void, size_of::<[c_char; 128]>()) < 1 {
                        break;
                    }

                    vringh_notify_disable_user(&mut vrh);
                    receives += 1;
                    continue;
                }
                if err != 1 {
                    errx(1, c"vringh_getdesc_user: %i".as_ptr(), err);
                }

                /* We simply copy bytes. */
                if riov.used != 0 {
                    rlen = vringh_iov_pull_user(&mut riov, rbuf.as_mut_ptr() as *mut c_void, size_of::<[c_char; 5]>());
                    if rlen != 4 {
                        errx(1, c"vringh_iov_pull_user: %i".as_ptr(), rlen);
                    }
                    assert!(riov.i == riov.used);
                    written = 0;
                } else {
                    err = vringh_iov_push_user(&mut wiov, rbuf.as_ptr() as *const c_void, rlen as usize);
                    if err != rlen {
                        errx(1, c"vringh_iov_push_user: %i".as_ptr(), err);
                    }
                    assert!(wiov.i == wiov.used);
                    written = err as u16_t;
                }
            }

            xfers += 1;

            err = vringh_complete_user(&mut vrh, head, written);
            if err != 0 {
                errx(1, c"vringh_complete_user: %i".as_ptr(), err);
            }
        }

        err = vringh_need_notify_user(&mut vrh);
        if err < 0 {
            errx(1, c"vringh_need_notify_user: %i".as_ptr(), err);
        }
        if err != 0 {
            write(to_guest[1], c"".as_ptr() as *const c_void, 1);
            notifies += 1;
        }
        wait(&mut status);
        if !WIFEXITED(status) {
            errx(1, c"Child died with signal %i?".as_ptr(), WTERMSIG(status));
        }
        if WEXITSTATUS(status) != 0 {
            errx(1, c"Child exited %i?".as_ptr(), WEXITSTATUS(status));
        }
        printf(c"Host: notified %lu, pinged %lu\n".as_ptr(), notifies, receives);
        return 0;
    } else {
        let mut gvdev: guest_virtio_device = zeroed();
        let vq: *mut virtqueue;
        let data: *mut c_uint;
        let indirects: *mut vring_desc;
        let mut finished: c_uint = 0;

        /* We pass sg[]s pointing into here, but we need RINGSIZE+1 */
        data = (guest_map as *mut u8).add(vring_size(RINGSIZE, ALIGN as c_ulong) as usize) as *mut c_uint;
        indirects = (data as *mut u8).add((RINGSIZE as usize + 1) * 2 * size_of::<c_int>()) as *mut vring_desc;

        /* We are the guest. */
        munmap(host_map, mapsize as usize);

        close(to_guest[1]);
        close(to_host[0]);

        gvdev.vdev.features = features;
        INIT_LIST_HEAD(&mut gvdev.vdev.vqs);
        spin_lock_init(&mut gvdev.vdev.vqs_list_lock);
        gvdev.to_host_fd = to_host[1];
        gvdev.notifies = 0;

        CPU_SET(first_cpu, &mut cpu_set);
        if sched_setaffinity(getpid(), size_of::<cpu_set_t>(), &cpu_set) != 0 {
            err(1, c"Could not set affinity to cpu %u".as_ptr(), first_cpu);
        }

        vq = vring_new_virtqueue(
            0,
            RINGSIZE,
            ALIGN,
            &mut gvdev.vdev,
            true,
            false,
            guest_map,
            if fast_vringh { Some(no_notify_host) } else { Some(parallel_notify_host) },
            Some(never_callback_guest),
            c"guest vq".as_ptr(),
        );

        /* Don't kfree indirects. */
        __kfree_ignore_start = indirects as *mut c_void;
        __kfree_ignore_end = indirects.add(RINGSIZE as usize * 6) as *mut c_void;

        while xfers < NUM_XFERS {
            let mut sg: [scatterlist; 4] = zeroed();
            let mut num_sg: c_uint = 0;
            let mut len: c_uint = 0;
            let mut dbuf: *mut c_int;
            let mut err: c_int;
            let output = !(xfers % 2 != 0);

            /* Consume bufs. */
            loop {
                dbuf = virtqueue_get_buf(vq, &mut len) as *mut c_int;
                if dbuf == null_mut() {
                    break;
                }
                if len == 4 {
                    assert!(*dbuf == finished.wrapping_sub(1) as c_int);
                } else if !fast_vringh {
                    assert!(*dbuf == finished as c_int);
                }
                finished = finished.wrapping_add(1);
            }

            /* Produce a buffer. */
            dbuf = data.add((xfers % (RINGSIZE as c_ulong + 1)) as usize) as *mut c_int;

            if output {
                *dbuf = xfers as c_int;
            } else {
                *dbuf = -1;
            }

            match (xfers / size_of::<c_int>() as c_ulong) % 4 {
                0 => {
                    /* Nasty three-element sg list. */
                    num_sg = 3;
                    sg_init_table(sg.as_mut_ptr(), num_sg);
                    sg_set_buf(&mut sg[0], dbuf as *mut c_void, 1);
                    sg_set_buf(&mut sg[1], (dbuf as *mut u8).add(1) as *mut c_void, 2);
                    sg_set_buf(&mut sg[2], (dbuf as *mut u8).add(3) as *mut c_void, 1);
                }
                1 => {
                    num_sg = 2;
                    sg_init_table(sg.as_mut_ptr(), num_sg);
                    sg_set_buf(&mut sg[0], dbuf as *mut c_void, 1);
                    sg_set_buf(&mut sg[1], (dbuf as *mut u8).add(1) as *mut c_void, 3);
                }
                2 => {
                    num_sg = 1;
                    sg_init_table(sg.as_mut_ptr(), num_sg);
                    sg_set_buf(&mut sg[0], dbuf as *mut c_void, 4);
                }
                3 => {
                    num_sg = 4;
                    sg_init_table(sg.as_mut_ptr(), num_sg);
                    sg_set_buf(&mut sg[0], dbuf as *mut c_void, 1);
                    sg_set_buf(&mut sg[1], (dbuf as *mut u8).add(1) as *mut c_void, 1);
                    sg_set_buf(&mut sg[2], (dbuf as *mut u8).add(2) as *mut c_void, 1);
                    sg_set_buf(&mut sg[3], (dbuf as *mut u8).add(3) as *mut c_void, 1);
                }
                _ => {}
            }

            /* May allocate an indirect, so force it to allocate
             * user addr */
            __kmalloc_fake = indirects.add((xfers % RINGSIZE as c_ulong) as usize * 4) as *mut c_void;
            if output {
                err = virtqueue_add_outbuf(vq, sg.as_mut_ptr(), num_sg, dbuf as *mut c_void, GFP_KERNEL);
            } else {
                err = virtqueue_add_inbuf(vq, sg.as_mut_ptr(), num_sg, dbuf as *mut c_void, GFP_KERNEL);
            }

            if err == -ENOSPC {
                if !virtqueue_enable_cb_delayed(vq) {
                    continue;
                }
                /* Swallow all notifies at once. */
                if read(to_guest[0], buf.as_mut_ptr() as *mut c_void, size_of::<[c_char; 128]>()) < 1 {
                    break;
                }

                receives += 1;
                virtqueue_disable_cb(vq);
                continue;
            }

            if err != 0 {
                errx(1, c"virtqueue_add_in/outbuf: %i".as_ptr(), err);
            }

            xfers += 1;
            virtqueue_kick(vq);
        }

        /* Any extra? */
        while finished != xfers as c_uint {
            let mut dbuf: *mut c_int;
            let mut len: c_uint = 0;

            /* Consume bufs. */
            dbuf = virtqueue_get_buf(vq, &mut len) as *mut c_int;
            if !dbuf.is_null() {
                if len == 4 {
                    assert!(*dbuf == finished.wrapping_sub(1) as c_int);
                } else {
                    assert!(len == 0);
                }
                finished = finished.wrapping_add(1);
                continue;
            }

            if !virtqueue_enable_cb_delayed(vq) {
                continue;
            }
            if read(to_guest[0], buf.as_mut_ptr() as *mut c_void, size_of::<[c_char; 128]>()) < 1 {
                break;
            }

            receives += 1;
            virtqueue_disable_cb(vq);
        }

        printf(c"Guest: notified %lu, pinged %lu\n".as_ptr(), gvdev.notifies, receives);
        vring_del_virtqueue(vq);
        return 0;
    }
}

unsafe fn main_impl(_argc: c_int, mut argv: *mut *mut c_char) -> c_int {
    let mut vdev: virtio_device = zeroed();
    let mut vq: *mut virtqueue;
    let mut vrh: vringh = zeroed();
    let mut guest_sg: [scatterlist; RINGSIZE as usize] = zeroed();
    let mut sgs: [*mut scatterlist; 2] = [null_mut(); 2];
    let mut host_riov: [iovec; 2] = zeroed();
    let mut host_wiov: [iovec; 2] = zeroed();
    let mut riov: vringh_iov = zeroed();
    let mut wiov: vringh_iov = zeroed();
    let mut used: [vring_used_elem; RINGSIZE as usize] = zeroed();
    let mut buf = [0 as c_char; 28];
    let mut head: u16_t = 0;
    let mut err: c_int = 0;
    let mut i: c_uint;
    let mut ret: *mut c_void;
    let mut getrange: Option<unsafe extern "C" fn(*mut vringh, u64_t, *mut vringh_range) -> bool_t>;
    let mut fast_vringh = false;
    let mut parallel = false;

    getrange = Some(getrange_iov);
    vdev.features = 0;
    INIT_LIST_HEAD(&mut vdev.vqs);
    spin_lock_init(&mut vdev.vqs_list_lock);

    while !(*argv.add(1)).is_null() {
        if strcmp(*argv.add(1), c"--indirect".as_ptr()) == 0 {
            __virtio_set_bit(&mut vdev, VIRTIO_RING_F_INDIRECT_DESC);
        } else if strcmp(*argv.add(1), c"--eventidx".as_ptr()) == 0 {
            __virtio_set_bit(&mut vdev, VIRTIO_RING_F_EVENT_IDX);
        } else if strcmp(*argv.add(1), c"--virtio-1".as_ptr()) == 0 {
            __virtio_set_bit(&mut vdev, VIRTIO_F_VERSION_1);
        } else if strcmp(*argv.add(1), c"--slow-range".as_ptr()) == 0 {
            getrange = Some(getrange_slow);
        } else if strcmp(*argv.add(1), c"--fast-vringh".as_ptr()) == 0 {
            fast_vringh = true;
        } else if strcmp(*argv.add(1), c"--parallel".as_ptr()) == 0 {
            parallel = true;
        } else {
            errx(1, c"Unknown arg %s".as_ptr(), *argv.add(1));
        }
        argv = argv.add(1);
    }

    if parallel {
        return parallel_test(vdev.features, getrange, fast_vringh);
    }

    if posix_memalign(&mut __user_addr_min, PAGE_SIZE, USER_MEM) != 0 {
        abort();
    }
    __user_addr_max = (__user_addr_min as *mut u8).add(USER_MEM) as *mut c_void;
    memset(__user_addr_min, 0, vring_size(RINGSIZE, ALIGN as c_ulong) as usize);

    /* Set up guest side. */
    vq = vring_new_virtqueue(
        0,
        RINGSIZE,
        ALIGN,
        &mut vdev,
        true,
        false,
        __user_addr_min,
        Some(never_notify_host),
        Some(never_callback_guest),
        c"guest vq".as_ptr(),
    );

    /* Set up host side. */
    vring_init(&mut vrh.vring, RINGSIZE, __user_addr_min, ALIGN as c_ulong);
    vringh_init_user(&mut vrh, vdev.features, RINGSIZE, true, vrh.vring.desc, vrh.vring.avail, vrh.vring.used);

    /* No descriptor to get yet... */
    err = vringh_getdesc_user(&mut vrh, &mut riov, &mut wiov, getrange, &mut head);
    if err != 0 {
        errx(1, c"vringh_getdesc_user: %i".as_ptr(), err);
    }

    /* Guest puts in a descriptor. */
    memcpy((__user_addr_max as *mut u8).sub(1) as *mut c_void, c"a".as_ptr() as *const c_void, 1);
    sg_init_table(guest_sg.as_mut_ptr(), 1);
    sg_set_buf(&mut guest_sg[0], (__user_addr_max as *mut u8).sub(1) as *mut c_void, 1);
    sg_init_table(guest_sg.as_mut_ptr().add(1), 1);
    sg_set_buf(&mut guest_sg[1], (__user_addr_max as *mut u8).sub(3) as *mut c_void, 2);
    sgs[0] = &mut guest_sg[0];
    sgs[1] = &mut guest_sg[1];

    /* May allocate an indirect, so force it to allocate user addr */
    __kmalloc_fake = (__user_addr_min as *mut u8).add(vring_size(RINGSIZE, ALIGN as c_ulong) as usize) as *mut c_void;
    err = virtqueue_add_sgs(vq, sgs.as_mut_ptr(), 1, 1, &mut err as *mut _ as *mut c_void, GFP_KERNEL);
    if err != 0 {
        errx(1, c"virtqueue_add_sgs: %i".as_ptr(), err);
    }
    __kmalloc_fake = null_mut();

    /* Host retrieves it. */
    vringh_iov_init(&mut riov, host_riov.as_mut_ptr(), host_riov.len() as c_uint);
    vringh_iov_init(&mut wiov, host_wiov.as_mut_ptr(), host_wiov.len() as c_uint);

    err = vringh_getdesc_user(&mut vrh, &mut riov, &mut wiov, getrange, &mut head);
    if err != 1 {
        errx(1, c"vringh_getdesc_user: %i".as_ptr(), err);
    }

    assert!(riov.used == 1);
    assert!((*riov.iov.add(0)).iov_base == (__user_addr_max as *mut u8).sub(1) as *mut c_void);
    assert!((*riov.iov.add(0)).iov_len == 1);
    if getrange != Some(getrange_slow) {
        assert!(wiov.used == 1);
        assert!((*wiov.iov.add(0)).iov_base == (__user_addr_max as *mut u8).sub(3) as *mut c_void);
        assert!((*wiov.iov.add(0)).iov_len == 2);
    } else {
        assert!(wiov.used == 2);
        assert!((*wiov.iov.add(0)).iov_base == (__user_addr_max as *mut u8).sub(3) as *mut c_void);
        assert!((*wiov.iov.add(0)).iov_len == 1);
        assert!((*wiov.iov.add(1)).iov_base == (__user_addr_max as *mut u8).sub(2) as *mut c_void);
        assert!((*wiov.iov.add(1)).iov_len == 1);
    }

    err = vringh_iov_pull_user(&mut riov, buf.as_mut_ptr() as *mut c_void, 5);
    if err != 1 {
        errx(1, c"vringh_iov_pull_user: %i".as_ptr(), err);
    }
    assert!(buf[0] == b'a' as c_char);
    assert!(riov.i == 1);
    assert!(vringh_iov_pull_user(&mut riov, buf.as_mut_ptr() as *mut c_void, 5) == 0);

    memcpy(buf.as_mut_ptr() as *mut c_void, c"bcdef".as_ptr() as *const c_void, 5);
    err = vringh_iov_push_user(&mut wiov, buf.as_ptr() as *const c_void, 5);
    if err != 2 {
        errx(1, c"vringh_iov_push_user: %i".as_ptr(), err);
    }
    assert!(memcmp((__user_addr_max as *mut u8).sub(3) as *const c_void, c"bc".as_ptr() as *const c_void, 2) == 0);
    assert!(wiov.i == wiov.used);
    assert!(vringh_iov_push_user(&mut wiov, buf.as_ptr() as *const c_void, 5) == 0);

    /* Host is done. */
    err = vringh_complete_user(&mut vrh, head, err as u16_t);
    if err != 0 {
        errx(1, c"vringh_complete_user: %i".as_ptr(), err);
    }

    /* Guest should see used token now. */
    __kfree_ignore_start = (__user_addr_min as *mut u8).add(vring_size(RINGSIZE, ALIGN as c_ulong) as usize) as *mut c_void;
    __kfree_ignore_end = (__kfree_ignore_start as *mut u8).add(1) as *mut c_void;
    ret = virtqueue_get_buf(vq, &mut i);
    if ret != &mut err as *mut _ as *mut c_void {
        errx(1, c"virtqueue_get_buf: %p".as_ptr(), ret);
    }
    assert!(i == 2);

    /* Guest puts in a huge descriptor. */
    sg_init_table(guest_sg.as_mut_ptr(), RINGSIZE);
    i = 0;
    while i < RINGSIZE {
        sg_set_buf(&mut guest_sg[i as usize], (__user_addr_max as *mut u8).sub(USER_MEM / 4) as *mut c_void, (USER_MEM / 4) as c_uint);
        i += 1;
    }

    /* Fill contents with recognisable garbage. */
    i = 0;
    while i < (USER_MEM / 4) as c_uint {
        *((__user_addr_max as *mut c_char).sub(USER_MEM / 4).add(i as usize)) = i as c_char;
        i += 1;
    }

    /* This will allocate an indirect, so force it to allocate user addr */
    __kmalloc_fake = (__user_addr_min as *mut u8).add(vring_size(RINGSIZE, ALIGN as c_ulong) as usize) as *mut c_void;
    err = virtqueue_add_outbuf(vq, guest_sg.as_mut_ptr(), RINGSIZE, &mut err as *mut _ as *mut c_void, GFP_KERNEL);
    if err != 0 {
        errx(1, c"virtqueue_add_outbuf (large): %i".as_ptr(), err);
    }
    __kmalloc_fake = null_mut();

    /* Host picks it up (allocates new iov). */
    vringh_iov_init(&mut riov, host_riov.as_mut_ptr(), host_riov.len() as c_uint);
    vringh_iov_init(&mut wiov, host_wiov.as_mut_ptr(), host_wiov.len() as c_uint);

    err = vringh_getdesc_user(&mut vrh, &mut riov, &mut wiov, getrange, &mut head);
    if err != 1 {
        errx(1, c"vringh_getdesc_user: %i".as_ptr(), err);
    }

    assert!((riov.max_num & VRINGH_IOV_ALLOCATED) != 0);
    assert!(riov.iov != host_riov.as_mut_ptr());
    if getrange != Some(getrange_slow) {
        assert!(riov.used == RINGSIZE);
    } else {
        assert!(riov.used == RINGSIZE * (USER_MEM / 4) as c_uint);
    }

    assert!((wiov.max_num & VRINGH_IOV_ALLOCATED) == 0);
    assert!(wiov.used == 0);

    /* Pull data back out (in odd chunks), should be as expected. */
    i = 0;
    while i < RINGSIZE * (USER_MEM / 4) as c_uint {
        err = vringh_iov_pull_user(&mut riov, buf.as_mut_ptr() as *mut c_void, 3);
        if err != 3 && i + err as c_uint != RINGSIZE * (USER_MEM / 4) as c_uint {
            errx(1, c"vringh_iov_pull_user large: %i".as_ptr(), err);
        }
        assert!(buf[0] == i as c_char);
        assert!(err < 2 || buf[1] == i.wrapping_add(1) as c_char);
        assert!(err < 3 || buf[2] == i.wrapping_add(2) as c_char);
        i = i.wrapping_add(3);
    }
    assert!(riov.i == riov.used);
    vringh_iov_cleanup(&mut riov);
    vringh_iov_cleanup(&mut wiov);

    /* Complete using multi interface, just because we can. */
    used[0].id = head as u32;
    used[0].len = 0;
    err = vringh_complete_multi_user(&mut vrh, used.as_mut_ptr(), 1);
    if err != 0 {
        errx(1, c"vringh_complete_multi_user(1): %i".as_ptr(), err);
    }

    /* Free up those descriptors. */
    ret = virtqueue_get_buf(vq, &mut i);
    if ret != &mut err as *mut _ as *mut c_void {
        errx(1, c"virtqueue_get_buf: %p".as_ptr(), ret);
    }

    /* Add lots of descriptors. */
    sg_init_table(guest_sg.as_mut_ptr(), 1);
    sg_set_buf(&mut guest_sg[0], (__user_addr_max as *mut u8).sub(1) as *mut c_void, 1);
    i = 0;
    while i < RINGSIZE {
        err = virtqueue_add_outbuf(vq, guest_sg.as_mut_ptr(), 1, &mut err as *mut _ as *mut c_void, GFP_KERNEL);
        if err != 0 {
            errx(1, c"virtqueue_add_outbuf (multiple): %i".as_ptr(), err);
        }
        i += 1;
    }

    /* Now get many, and consume them all at once. */
    vringh_iov_init(&mut riov, host_riov.as_mut_ptr(), host_riov.len() as c_uint);
    vringh_iov_init(&mut wiov, host_wiov.as_mut_ptr(), host_wiov.len() as c_uint);

    i = 0;
    while i < RINGSIZE {
        err = vringh_getdesc_user(&mut vrh, &mut riov, &mut wiov, getrange, &mut head);
        if err != 1 {
            errx(1, c"vringh_getdesc_user: %i".as_ptr(), err);
        }
        used[i as usize].id = head as u32;
        used[i as usize].len = 0;
        i += 1;
    }
    /* Make sure it wraps around ring, to test! */
    assert!((*vrh.vring.used).idx as c_uint % RINGSIZE != 0);
    err = vringh_complete_multi_user(&mut vrh, used.as_mut_ptr(), RINGSIZE);
    if err != 0 {
        errx(1, c"vringh_complete_multi_user: %i".as_ptr(), err);
    }

    /* Free those buffers. */
    i = 0;
    while i < RINGSIZE {
        let mut len: c_uint = 0;
        assert!(virtqueue_get_buf(vq, &mut len) != null_mut());
        i += 1;
    }

    /* Test weird (but legal!) indirect. */
    if __virtio_test_bit(&vdev, VIRTIO_RING_F_INDIRECT_DESC) {
        let data = (__user_addr_max as *mut c_char).sub(USER_MEM / 4);
        let d = (__user_addr_max as *mut u8).sub(USER_MEM / 2) as *mut vring_desc;
        let mut vring: vring = zeroed();

        /* Force creation of direct, which we modify. */
        __virtio_clear_bit(&mut vdev, VIRTIO_RING_F_INDIRECT_DESC);
        vq = vring_new_virtqueue(
            0,
            RINGSIZE,
            ALIGN,
            &mut vdev,
            true,
            false,
            __user_addr_min,
            Some(never_notify_host),
            Some(never_callback_guest),
            c"guest vq".as_ptr(),
        );

        sg_init_table(guest_sg.as_mut_ptr(), 4);
        sg_set_buf(&mut guest_sg[0], d as *mut c_void, (size_of::<vring_desc>() * 2) as c_uint);
        sg_set_buf(&mut guest_sg[1], d.add(2) as *mut c_void, (size_of::<vring_desc>()) as c_uint);
        sg_set_buf(&mut guest_sg[2], data.add(6) as *mut c_void, 4);
        sg_set_buf(&mut guest_sg[3], d.add(3) as *mut c_void, (size_of::<vring_desc>() * 3) as c_uint);

        err = virtqueue_add_outbuf(vq, guest_sg.as_mut_ptr(), 4, &mut err as *mut _ as *mut c_void, GFP_KERNEL);
        if err != 0 {
            errx(1, c"virtqueue_add_outbuf (indirect): %i".as_ptr(), err);
        }

        vring_init(&mut vring, RINGSIZE, __user_addr_min, ALIGN as c_ulong);

        /* They're used in order, but double-check... */
        assert!((*vring.desc.add(0)).addr == d as c_ulong as u64_t);
        assert!((*vring.desc.add(1)).addr == d.add(2) as c_ulong as u64_t);
        assert!((*vring.desc.add(2)).addr == data.add(6) as c_ulong as u64_t);
        assert!((*vring.desc.add(3)).addr == d.add(3) as c_ulong as u64_t);
        (*vring.desc.add(0)).flags |= VRING_DESC_F_INDIRECT;
        (*vring.desc.add(1)).flags |= VRING_DESC_F_INDIRECT;
        (*vring.desc.add(3)).flags |= VRING_DESC_F_INDIRECT;

        /* First indirect */
        (*d.add(0)).addr = data as c_ulong as u64_t;
        (*d.add(0)).len = 1;
        (*d.add(0)).flags = VRING_DESC_F_NEXT;
        (*d.add(0)).next = 1;
        (*d.add(1)).addr = data.add(1) as c_ulong as u64_t;
        (*d.add(1)).len = 2;
        (*d.add(1)).flags = 0;

        /* Second indirect */
        (*d.add(2)).addr = data.add(3) as c_ulong as u64_t;
        (*d.add(2)).len = 3;
        (*d.add(2)).flags = 0;

        /* Third indirect */
        (*d.add(3)).addr = data.add(10) as c_ulong as u64_t;
        (*d.add(3)).len = 5;
        (*d.add(3)).flags = VRING_DESC_F_NEXT;
        (*d.add(3)).next = 1;
        (*d.add(4)).addr = data.add(15) as c_ulong as u64_t;
        (*d.add(4)).len = 6;
        (*d.add(4)).flags = VRING_DESC_F_NEXT;
        (*d.add(4)).next = 2;
        (*d.add(5)).addr = data.add(21) as c_ulong as u64_t;
        (*d.add(5)).len = 7;
        (*d.add(5)).flags = 0;

        /* Host picks it up (allocates new iov). */
        vringh_iov_init(&mut riov, host_riov.as_mut_ptr(), host_riov.len() as c_uint);
        vringh_iov_init(&mut wiov, host_wiov.as_mut_ptr(), host_wiov.len() as c_uint);

        err = vringh_getdesc_user(&mut vrh, &mut riov, &mut wiov, getrange, &mut head);
        if err != 1 {
            errx(1, c"vringh_getdesc_user: %i".as_ptr(), err);
        }

        if head != 0 {
            errx(1, c"vringh_getdesc_user: head %i not 0".as_ptr(), head as c_int);
        }

        assert!((riov.max_num & VRINGH_IOV_ALLOCATED) != 0);
        if getrange != Some(getrange_slow) {
            assert!(riov.used == 7);
        } else {
            assert!(riov.used == 28);
        }
        err = vringh_iov_pull_user(&mut riov, buf.as_mut_ptr() as *mut c_void, 29);
        assert!(err == 28);

        /* Data should be linear. */
        i = 0;
        while i < err as c_uint {
            assert!(buf[i as usize] == i as c_char);
            i += 1;
        }
        vringh_iov_cleanup(&mut riov);
    }

    /* Don't leak memory... */
    vring_del_virtqueue(vq);
    free(__user_addr_min);

    0
}

fn main() {
    unsafe {
        extern "C" {
            static mut __argc: c_int;
            static mut __argv: *mut *mut c_char;
        }
        let _ = main_impl(__argc, __argv);
    }
}
