/* SPDX-License-Identifier: MIT */

// Translated from a C header that depends on linux/io_uring.h, sys/mman.h,
// sys/syscall.h, stdio.h, string.h, unistd.h, and sys/uio.h.

use core::ffi::{c_int, c_long, c_uint, c_ulong, c_ulonglong, c_void};
use core::mem::size_of;
use core::ptr;
use core::sync::atomic::{compiler_fence, fence, Ordering};

#[repr(C)]
pub struct io_sq_ring {
    pub head: *mut c_uint,
    pub tail: *mut c_uint,
    pub ring_mask: *mut c_uint,
    pub ring_entries: *mut c_uint,
    pub flags: *mut c_uint,
    pub array: *mut c_uint,
}

#[repr(C)]
pub struct io_cq_ring {
    pub head: *mut c_uint,
    pub tail: *mut c_uint,
    pub ring_mask: *mut c_uint,
    pub ring_entries: *mut c_uint,
    pub cqes: *mut io_uring_cqe,
}

#[repr(C)]
pub struct io_uring_sq {
    pub khead: *mut c_uint,
    pub ktail: *mut c_uint,
    pub kring_mask: *mut c_uint,
    pub kring_entries: *mut c_uint,
    pub kflags: *mut c_uint,
    pub kdropped: *mut c_uint,
    pub array: *mut c_uint,
    pub sqes: *mut io_uring_sqe,

    pub sqe_head: c_uint,
    pub sqe_tail: c_uint,

    pub ring_sz: usize,
}

#[repr(C)]
pub struct io_uring_cq {
    pub khead: *mut c_uint,
    pub ktail: *mut c_uint,
    pub kring_mask: *mut c_uint,
    pub kring_entries: *mut c_uint,
    pub koverflow: *mut c_uint,
    pub cqes: *mut io_uring_cqe,

    pub ring_sz: usize,
}

#[repr(C)]
pub struct io_uring {
    pub sq: io_uring_sq,
    pub cq: io_uring_cq,
    pub ring_fd: c_int,
    pub flags: c_uint,
}

// On x86 and i386 the C header used compiler-only asm memory barriers; on
// other targets it used __sync_synchronize().
#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
#[inline]
pub fn read_barrier() {
    compiler_fence(Ordering::SeqCst);
}

#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
#[inline]
pub fn write_barrier() {
    compiler_fence(Ordering::SeqCst);
}

#[cfg(not(any(target_arch = "x86_64", target_arch = "x86")))]
#[inline]
pub fn read_barrier() {
    fence(Ordering::SeqCst);
}

#[cfg(not(any(target_arch = "x86_64", target_arch = "x86")))]
#[inline]
pub fn write_barrier() {
    fence(Ordering::SeqCst);
}

unsafe extern "C" {
    static mut errno: c_int;

    fn mmap(
        addr: *mut c_void,
        length: usize,
        prot: c_int,
        flags: c_int,
        fd: c_int,
        offset: c_long,
    ) -> *mut c_void;
    fn munmap(addr: *mut c_void, length: usize) -> c_int;
    fn syscall(num: c_long, ...) -> c_long;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn close(fd: c_int) -> c_int;
}

#[inline]
const fn map_failed() -> *mut c_void {
    !0usize as *mut c_void
}

pub unsafe fn io_uring_mmap(
    fd: c_int,
    p: *mut io_uring_params,
    sq: *mut io_uring_sq,
    cq: *mut io_uring_cq,
) -> c_int {
    let size: usize;
    let mut ptr_: *mut c_void;
    let mut ret: c_int;

    if (*p).flags & IORING_SETUP_NO_SQARRAY != 0 {
        (*sq).ring_sz = (*p).cq_off.cqes as usize;
        (*sq).ring_sz += (*p).cq_entries as usize * size_of::<io_uring_cqe>();
    } else {
        (*sq).ring_sz = (*p).sq_off.array as usize;
        (*sq).ring_sz += (*p).sq_entries as usize * size_of::<c_uint>();
    }

    ptr_ = mmap(
        ptr::null_mut(),
        (*sq).ring_sz,
        PROT_READ | PROT_WRITE,
        MAP_SHARED | MAP_POPULATE,
        fd,
        IORING_OFF_SQ_RING,
    );
    if ptr_ == map_failed() {
        return -errno;
    }
    (*sq).khead = (ptr_ as *mut u8).add((*p).sq_off.head as usize) as *mut c_uint;
    (*sq).ktail = (ptr_ as *mut u8).add((*p).sq_off.tail as usize) as *mut c_uint;
    (*sq).kring_mask = (ptr_ as *mut u8).add((*p).sq_off.ring_mask as usize) as *mut c_uint;
    (*sq).kring_entries =
        (ptr_ as *mut u8).add((*p).sq_off.ring_entries as usize) as *mut c_uint;
    (*sq).kflags = (ptr_ as *mut u8).add((*p).sq_off.flags as usize) as *mut c_uint;
    (*sq).kdropped = (ptr_ as *mut u8).add((*p).sq_off.dropped as usize) as *mut c_uint;
    if (*p).flags & IORING_SETUP_NO_SQARRAY == 0 {
        (*sq).array = (ptr_ as *mut u8).add((*p).sq_off.array as usize) as *mut c_uint;
    }

    size = (*p).sq_entries as usize * size_of::<io_uring_sqe>();
    (*sq).sqes = mmap(
        ptr::null_mut(),
        size,
        PROT_READ | PROT_WRITE,
        MAP_SHARED | MAP_POPULATE,
        fd,
        IORING_OFF_SQES,
    ) as *mut io_uring_sqe;
    if (*sq).sqes == map_failed() as *mut io_uring_sqe {
        ret = -errno;
        munmap((*sq).khead as *mut c_void, (*sq).ring_sz);
        return ret;
    }

    (*cq).ring_sz = (*p).cq_off.cqes as usize + (*p).cq_entries as usize * size_of::<io_uring_cqe>();
    ptr_ = mmap(
        ptr::null_mut(),
        (*cq).ring_sz,
        PROT_READ | PROT_WRITE,
        MAP_SHARED | MAP_POPULATE,
        fd,
        IORING_OFF_CQ_RING,
    );
    if ptr_ == map_failed() {
        ret = -errno;
        munmap(
            (*sq).sqes as *mut c_void,
            (*p).sq_entries as usize * size_of::<io_uring_sqe>(),
        );
        munmap((*sq).khead as *mut c_void, (*sq).ring_sz);
        return ret;
    }
    (*cq).khead = (ptr_ as *mut u8).add((*p).cq_off.head as usize) as *mut c_uint;
    (*cq).ktail = (ptr_ as *mut u8).add((*p).cq_off.tail as usize) as *mut c_uint;
    (*cq).kring_mask = (ptr_ as *mut u8).add((*p).cq_off.ring_mask as usize) as *mut c_uint;
    (*cq).kring_entries =
        (ptr_ as *mut u8).add((*p).cq_off.ring_entries as usize) as *mut c_uint;
    (*cq).koverflow = (ptr_ as *mut u8).add((*p).cq_off.overflow as usize) as *mut c_uint;
    (*cq).cqes = (ptr_ as *mut u8).add((*p).cq_off.cqes as usize) as *mut io_uring_cqe;
    0
}

pub unsafe fn io_uring_setup(entries: c_uint, p: *mut io_uring_params) -> c_int {
    syscall(__NR_io_uring_setup, entries, p) as c_int
}

pub unsafe fn io_uring_enter(
    fd: c_int,
    to_submit: c_uint,
    min_complete: c_uint,
    flags: c_uint,
    sig: *mut sigset_t,
) -> c_int {
    syscall(
        __NR_io_uring_enter,
        fd,
        to_submit,
        min_complete,
        flags,
        sig,
        _NSIG / 8,
    ) as c_int
}

pub unsafe fn io_uring_queue_init_params(
    entries: c_uint,
    ring: *mut io_uring,
    p: *mut io_uring_params,
) -> c_int {
    let fd: c_int;
    let ret: c_int;

    memset(
        ring as *mut c_void,
        0,
        size_of::<io_uring>(),
    );

    fd = io_uring_setup(entries, p);
    if fd < 0 {
        return fd;
    }
    ret = io_uring_mmap(fd, p, &mut (*ring).sq, &mut (*ring).cq);
    if ret == 0 {
        (*ring).ring_fd = fd;
        (*ring).flags = (*p).flags;
    } else {
        close(fd);
    }
    ret
}

pub unsafe fn io_uring_queue_init(entries: c_uint, ring: *mut io_uring, flags: c_uint) -> c_int {
    let mut p: io_uring_params = core::mem::zeroed();

    memset(
        &mut p as *mut io_uring_params as *mut c_void,
        0,
        size_of::<io_uring_params>(),
    );
    p.flags = flags;

    io_uring_queue_init_params(entries, ring, &mut p)
}

/* Get a sqe */
pub unsafe fn io_uring_get_sqe(ring: *mut io_uring) -> *mut io_uring_sqe {
    let sq: *mut io_uring_sq = &mut (*ring).sq;

    if (*sq).sqe_tail.wrapping_add(1).wrapping_sub((*sq).sqe_head) > *(*sq).kring_entries {
        return ptr::null_mut();
    }
    let idx = (*sq).sqe_tail & *(*sq).kring_mask;
    (*sq).sqe_tail = (*sq).sqe_tail.wrapping_add(1);
    (*sq).sqes.add(idx as usize)
}

pub unsafe fn io_uring_wait_cqe(
    ring: *mut io_uring,
    cqe_ptr: *mut *mut io_uring_cqe,
) -> c_int {
    let cq: *mut io_uring_cq = &mut (*ring).cq;
    let mask: c_uint = *(*cq).kring_mask;
    let head: c_uint = *(*cq).khead;
    let mut ret: c_int;

    *cqe_ptr = ptr::null_mut();
    loop {
        read_barrier();
        if head != *(*cq).ktail {
            *cqe_ptr = (*cq).cqes.add((head & mask) as usize);
            break;
        }
        ret = io_uring_enter(
            (*ring).ring_fd,
            0,
            1,
            IORING_ENTER_GETEVENTS,
            ptr::null_mut(),
        );
        if ret < 0 {
            return -errno;
        }
    }

    0
}

pub unsafe fn io_uring_submit(ring: *mut io_uring) -> c_int {
    let sq: *mut io_uring_sq = &mut (*ring).sq;
    let mask: c_uint = *(*sq).kring_mask;
    let mut ktail: c_uint;
    let mut submitted: c_uint;
    let to_submit: c_uint;
    let ret: c_int;

    read_barrier();
    if *(*sq).khead != *(*sq).ktail {
        submitted = *(*sq).kring_entries;
    } else {
        if (*sq).sqe_head == (*sq).sqe_tail {
            return 0;
        }

        ktail = *(*sq).ktail;
        to_submit = (*sq).sqe_tail.wrapping_sub((*sq).sqe_head);

        if (*ring).flags & IORING_SETUP_NO_SQARRAY == 0 {
            submitted = 0;
            while submitted < to_submit {
                read_barrier();
                *(*sq).array.add((ktail & mask) as usize) = (*sq).sqe_head & mask;
                ktail = ktail.wrapping_add(1);
                (*sq).sqe_head = (*sq).sqe_head.wrapping_add(1);
                submitted = submitted.wrapping_add(1);
            }
        } else {
            ktail = ktail.wrapping_add(to_submit);
            (*sq).sqe_head = (*sq).sqe_head.wrapping_add(to_submit);
            submitted = to_submit;
        }

        if submitted == 0 {
            return 0;
        }

        if *(*sq).ktail != ktail {
            write_barrier();
            *(*sq).ktail = ktail;
            write_barrier();
        }
    }

    ret = io_uring_enter(
        (*ring).ring_fd,
        submitted,
        0,
        IORING_ENTER_GETEVENTS,
        ptr::null_mut(),
    );
    if ret < 0 {
        -errno
    } else {
        ret
    }
}

pub unsafe fn io_uring_queue_exit(ring: *mut io_uring) {
    let sq: *mut io_uring_sq = &mut (*ring).sq;

    munmap(
        (*sq).sqes as *mut c_void,
        *(*sq).kring_entries as usize * size_of::<io_uring_sqe>(),
    );
    munmap((*sq).khead as *mut c_void, (*sq).ring_sz);
    close((*ring).ring_fd);
}

/* Prepare and send the SQE */
pub unsafe fn io_uring_prep_cmd(
    sqe: *mut io_uring_sqe,
    op: c_int,
    sockfd: c_int,
    level: c_int,
    optname: c_int,
    optval: *const c_void,
    optlen: c_int,
) {
    memset(sqe as *mut c_void, 0, size_of::<io_uring_sqe>());
    (*sqe).opcode = IORING_OP_URING_CMD as __u8;
    (*sqe).fd = sockfd;
    (*sqe).cmd_op = op;

    (*sqe).level = level;
    (*sqe).optname = optname;
    (*sqe).optval = optval as c_ulonglong;
    (*sqe).optlen = optlen;
}

pub unsafe fn io_uring_register_buffers(
    ring: *mut io_uring,
    iovecs: *const iovec,
    nr_iovecs: c_uint,
) -> c_int {
    let ret: c_long;

    ret = syscall(
        __NR_io_uring_register,
        (*ring).ring_fd,
        IORING_REGISTER_BUFFERS,
        iovecs,
        nr_iovecs,
    );
    if ret < 0 {
        -errno
    } else {
        ret as c_int
    }
}

pub unsafe fn io_uring_prep_send(
    sqe: *mut io_uring_sqe,
    sockfd: c_int,
    buf: *const c_void,
    len: usize,
    flags: c_int,
) {
    memset(sqe as *mut c_void, 0, size_of::<io_uring_sqe>());
    (*sqe).opcode = IORING_OP_SEND as __u8;
    (*sqe).fd = sockfd;
    (*sqe).addr = buf as c_ulong;
    (*sqe).len = len;
    (*sqe).msg_flags = flags as __u32;
}

pub unsafe fn io_uring_prep_sendzc(
    sqe: *mut io_uring_sqe,
    sockfd: c_int,
    buf: *const c_void,
    len: usize,
    flags: c_int,
    zc_flags: c_uint,
) {
    io_uring_prep_send(sqe, sockfd, buf, len, flags);
    (*sqe).opcode = IORING_OP_SEND_ZC as __u8;
    (*sqe).ioprio = zc_flags;
}

pub unsafe fn io_uring_cqe_seen(ring: *mut io_uring) {
    let cq = &mut (*ring).cq;
    *cq.khead = (*cq.khead).wrapping_add(1);
    write_barrier();
}
