/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 *  include/linux/userfaultfd.h
 *
 *  Copyright (C) 2007  Davide Libenzi <davidel@xmailserver.org>
 *  Copyright (C) 2015  Red Hat, Inc.
 *
 */

/* Depends on Linux UAPI integer types from <linux/types.h>. */
pub type __u8 = u8;
pub type __u16 = u16;
pub type __u32 = u32;
pub type __u64 = u64;
pub type __s64 = i64;

unsafe extern "C" {
    pub fn _IO(type_: core::ffi::c_uint, nr: core::ffi::c_uint) -> core::ffi::c_ulong;
    pub fn _IOR(
        type_: core::ffi::c_uint,
        nr: core::ffi::c_uint,
        size: core::ffi::c_ulong,
    ) -> core::ffi::c_ulong;
    pub fn _IOW(
        type_: core::ffi::c_uint,
        nr: core::ffi::c_uint,
        size: core::ffi::c_ulong,
    ) -> core::ffi::c_ulong;
    pub fn _IOWR(
        type_: core::ffi::c_uint,
        nr: core::ffi::c_uint,
        size: core::ffi::c_ulong,
    ) -> core::ffi::c_ulong;
}

/* ioctls for /dev/userfaultfd */
pub const USERFAULTFD_IOC: core::ffi::c_uint = 0xAA;
pub unsafe fn USERFAULTFD_IOC_NEW() -> core::ffi::c_ulong {
    unsafe { _IO(USERFAULTFD_IOC, 0x00) }
}

/*
 * If the UFFDIO_API is upgraded someday, the UFFDIO_UNREGISTER and
 * UFFDIO_WAKE ioctls should be defined as _IOW and not as _IOR.  In
 * userfaultfd.h we assumed the kernel was reading (instead _IOC_READ
 * means the userland is reading).
 */
pub const UFFD_API: __u64 = 0xAAu64;
pub const UFFD_API_REGISTER_MODES: __u64 = UFFDIO_REGISTER_MODE_MISSING
    | UFFDIO_REGISTER_MODE_WP
    | UFFDIO_REGISTER_MODE_MINOR
    | UFFDIO_REGISTER_MODE_RWP;
pub const UFFD_API_FEATURES: core::ffi::c_uint = UFFD_FEATURE_PAGEFAULT_FLAG_WP
    | UFFD_FEATURE_EVENT_FORK
    | UFFD_FEATURE_EVENT_REMAP
    | UFFD_FEATURE_EVENT_REMOVE
    | UFFD_FEATURE_EVENT_UNMAP
    | UFFD_FEATURE_MISSING_HUGETLBFS
    | UFFD_FEATURE_MISSING_SHMEM
    | UFFD_FEATURE_SIGBUS
    | UFFD_FEATURE_THREAD_ID
    | UFFD_FEATURE_MINOR_HUGETLBFS
    | UFFD_FEATURE_MINOR_SHMEM
    | UFFD_FEATURE_EXACT_ADDRESS
    | UFFD_FEATURE_WP_HUGETLBFS_SHMEM
    | UFFD_FEATURE_WP_UNPOPULATED
    | UFFD_FEATURE_POISON
    | UFFD_FEATURE_WP_ASYNC
    | UFFD_FEATURE_MOVE
    | UFFD_FEATURE_RWP
    | UFFD_FEATURE_RWP_ASYNC;
pub const UFFD_API_IOCTLS: __u64 = (1u64 << _UFFDIO_REGISTER)
    | (1u64 << _UFFDIO_UNREGISTER)
    | (1u64 << _UFFDIO_SET_MODE)
    | (1u64 << _UFFDIO_API);
pub const UFFD_API_RANGE_IOCTLS: __u64 = (1u64 << _UFFDIO_WAKE)
    | (1u64 << _UFFDIO_COPY)
    | (1u64 << _UFFDIO_ZEROPAGE)
    | (1u64 << _UFFDIO_MOVE)
    | (1u64 << _UFFDIO_WRITEPROTECT)
    | (1u64 << _UFFDIO_CONTINUE)
    | (1u64 << _UFFDIO_POISON)
    | (1u64 << _UFFDIO_RWPROTECT);
pub const UFFD_API_RANGE_IOCTLS_BASIC: __u64 = (1u64 << _UFFDIO_WAKE)
    | (1u64 << _UFFDIO_COPY)
    | (1u64 << _UFFDIO_WRITEPROTECT)
    | (1u64 << _UFFDIO_CONTINUE)
    | (1u64 << _UFFDIO_POISON)
    | (1u64 << _UFFDIO_RWPROTECT);

/*
 * Valid ioctl command number range with this API is from 0x00 to
 * 0x3F.  UFFDIO_API is the fixed number, everything else can be
 * changed by implementing a different UFFD_API. If sticking to the
 * same UFFD_API more ioctl can be added and userland will be aware of
 * which ioctl the running kernel implements through the ioctl command
 * bitmask written by the UFFDIO_API.
 */
pub const _UFFDIO_REGISTER: core::ffi::c_uint = 0x00;
pub const _UFFDIO_UNREGISTER: core::ffi::c_uint = 0x01;
pub const _UFFDIO_WAKE: core::ffi::c_uint = 0x02;
pub const _UFFDIO_COPY: core::ffi::c_uint = 0x03;
pub const _UFFDIO_ZEROPAGE: core::ffi::c_uint = 0x04;
pub const _UFFDIO_MOVE: core::ffi::c_uint = 0x05;
pub const _UFFDIO_WRITEPROTECT: core::ffi::c_uint = 0x06;
pub const _UFFDIO_CONTINUE: core::ffi::c_uint = 0x07;
pub const _UFFDIO_POISON: core::ffi::c_uint = 0x08;
pub const _UFFDIO_RWPROTECT: core::ffi::c_uint = 0x09;
pub const _UFFDIO_SET_MODE: core::ffi::c_uint = 0x0A;
pub const _UFFDIO_API: core::ffi::c_uint = 0x3F;

/* userfaultfd ioctl ids */
pub const UFFDIO: core::ffi::c_uint = 0xAA;
pub unsafe fn UFFDIO_API_IOCTL() -> core::ffi::c_ulong {
    unsafe { _IOWR(UFFDIO, _UFFDIO_API, core::mem::size_of::<uffdio_api>() as core::ffi::c_ulong) }
}
pub unsafe fn UFFDIO_REGISTER() -> core::ffi::c_ulong {
    unsafe { _IOWR(UFFDIO, _UFFDIO_REGISTER, core::mem::size_of::<uffdio_register>() as core::ffi::c_ulong) }
}
pub unsafe fn UFFDIO_UNREGISTER() -> core::ffi::c_ulong {
    unsafe { _IOR(UFFDIO, _UFFDIO_UNREGISTER, core::mem::size_of::<uffdio_range>() as core::ffi::c_ulong) }
}
pub unsafe fn UFFDIO_WAKE() -> core::ffi::c_ulong {
    unsafe { _IOR(UFFDIO, _UFFDIO_WAKE, core::mem::size_of::<uffdio_range>() as core::ffi::c_ulong) }
}
pub unsafe fn UFFDIO_COPY() -> core::ffi::c_ulong {
    unsafe { _IOWR(UFFDIO, _UFFDIO_COPY, core::mem::size_of::<uffdio_copy>() as core::ffi::c_ulong) }
}
pub unsafe fn UFFDIO_ZEROPAGE() -> core::ffi::c_ulong {
    unsafe { _IOWR(UFFDIO, _UFFDIO_ZEROPAGE, core::mem::size_of::<uffdio_zeropage>() as core::ffi::c_ulong) }
}
pub unsafe fn UFFDIO_MOVE() -> core::ffi::c_ulong {
    unsafe { _IOWR(UFFDIO, _UFFDIO_MOVE, core::mem::size_of::<uffdio_move>() as core::ffi::c_ulong) }
}
pub unsafe fn UFFDIO_WRITEPROTECT() -> core::ffi::c_ulong {
    unsafe { _IOWR(UFFDIO, _UFFDIO_WRITEPROTECT, core::mem::size_of::<uffdio_writeprotect>() as core::ffi::c_ulong) }
}
pub unsafe fn UFFDIO_CONTINUE() -> core::ffi::c_ulong {
    unsafe { _IOWR(UFFDIO, _UFFDIO_CONTINUE, core::mem::size_of::<uffdio_continue>() as core::ffi::c_ulong) }
}
pub unsafe fn UFFDIO_POISON() -> core::ffi::c_ulong {
    unsafe { _IOWR(UFFDIO, _UFFDIO_POISON, core::mem::size_of::<uffdio_poison>() as core::ffi::c_ulong) }
}
pub unsafe fn UFFDIO_RWPROTECT() -> core::ffi::c_ulong {
    unsafe { _IOWR(UFFDIO, _UFFDIO_RWPROTECT, core::mem::size_of::<uffdio_rwprotect>() as core::ffi::c_ulong) }
}
pub unsafe fn UFFDIO_SET_MODE() -> core::ffi::c_ulong {
    unsafe { _IOW(UFFDIO, _UFFDIO_SET_MODE, core::mem::size_of::<uffdio_set_mode>() as core::ffi::c_ulong) }
}

/* read() structure */
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct uffd_msg {
    pub event: __u8,
    pub reserved1: __u8,
    pub reserved2: __u16,
    pub reserved3: __u32,
    pub arg: uffd_msg_arg,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union uffd_msg_arg {
    pub pagefault: uffd_msg_pagefault,
    pub fork: uffd_msg_fork,
    pub remap: uffd_msg_remap,
    pub remove: uffd_msg_remove,
    pub reserved: uffd_msg_reserved,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uffd_msg_pagefault {
    pub flags: __u64,
    pub address: __u64,
    pub feat: uffd_msg_pagefault_feat,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union uffd_msg_pagefault_feat {
    pub ptid: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uffd_msg_fork {
    pub ufd: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uffd_msg_remap {
    pub from: __u64,
    pub to: __u64,
    pub len: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uffd_msg_remove {
    pub start: __u64,
    pub end: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uffd_msg_reserved {
    /* unused reserved fields */
    pub reserved1: __u64,
    pub reserved2: __u64,
    pub reserved3: __u64,
}

/*
 * Start at 0x12 and not at 0 to be more strict against bugs.
 */
pub const UFFD_EVENT_PAGEFAULT: core::ffi::c_uint = 0x12;
pub const UFFD_EVENT_FORK: core::ffi::c_uint = 0x13;
pub const UFFD_EVENT_REMAP: core::ffi::c_uint = 0x14;
pub const UFFD_EVENT_REMOVE: core::ffi::c_uint = 0x15;
pub const UFFD_EVENT_UNMAP: core::ffi::c_uint = 0x16;

/* flags for UFFD_EVENT_PAGEFAULT */
pub const UFFD_PAGEFAULT_FLAG_WRITE: core::ffi::c_uint = 1 << 0; /* If this was a write fault */
pub const UFFD_PAGEFAULT_FLAG_WP: core::ffi::c_uint = 1 << 1; /* If reason is VM_UFFD_WP */
pub const UFFD_PAGEFAULT_FLAG_MINOR: core::ffi::c_uint = 1 << 2; /* If reason is VM_UFFD_MINOR */
pub const UFFD_PAGEFAULT_FLAG_RWP: core::ffi::c_uint = 1 << 3; /* If reason is VM_UFFD_RWP */

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uffdio_api {
    /* userland asks for an API number and the features to enable */
    pub api: __u64,
    /*
     * Kernel answers below with the all available features for
     * the API, this notifies userland of which events and/or
     * which flags for each event are enabled in the current
     * kernel.
     *
     * Note: UFFD_EVENT_PAGEFAULT and UFFD_PAGEFAULT_FLAG_WRITE
     * are to be considered implicitly always enabled in all kernels as
     * long as the uffdio_api.api requested matches UFFD_API.
     *
     * UFFD_FEATURE_MISSING_HUGETLBFS means an UFFDIO_REGISTER
     * with UFFDIO_REGISTER_MODE_MISSING mode will succeed on
     * hugetlbfs virtual memory ranges. Adding or not adding
     * UFFD_FEATURE_MISSING_HUGETLBFS to uffdio_api.features has
     * no real functional effect after UFFDIO_API returns, but
     * it's only useful for an initial feature set probe at
     * UFFDIO_API time. There are two ways to use it:
     *
     * 1) by adding UFFD_FEATURE_MISSING_HUGETLBFS to the
     *    uffdio_api.features before calling UFFDIO_API, an error
     *    will be returned by UFFDIO_API on a kernel without
     *    hugetlbfs missing support
     *
     * 2) the UFFD_FEATURE_MISSING_HUGETLBFS can not be added in
     *    uffdio_api.features and instead it will be set by the
     *    kernel in the uffdio_api.features if the kernel supports
     *    it, so userland can later check if the feature flag is
     *    present in uffdio_api.features after UFFDIO_API
     *    succeeded.
     *
     * UFFD_FEATURE_MISSING_SHMEM works the same as
     * UFFD_FEATURE_MISSING_HUGETLBFS, but it applies to shmem
     * (i.e. tmpfs and other shmem based APIs).
     *
     * UFFD_FEATURE_SIGBUS feature means no page-fault
     * (UFFD_EVENT_PAGEFAULT) event will be delivered, instead
     * a SIGBUS signal will be sent to the faulting process.
     *
     * UFFD_FEATURE_THREAD_ID pid of the page faulted task_struct will
     * be returned, if feature is not requested 0 will be returned.
     *
     * UFFD_FEATURE_MINOR_HUGETLBFS indicates that minor faults
     * can be intercepted (via REGISTER_MODE_MINOR) for
     * hugetlbfs-backed pages.
     *
     * UFFD_FEATURE_MINOR_SHMEM indicates the same support as
     * UFFD_FEATURE_MINOR_HUGETLBFS, but for shmem-backed pages instead.
     *
     * UFFD_FEATURE_EXACT_ADDRESS indicates that the exact address of page
     * faults would be provided and the offset within the page would not be
     * masked.
     *
     * UFFD_FEATURE_WP_HUGETLBFS_SHMEM indicates that userfaultfd
     * write-protection mode is supported on both shmem and hugetlbfs.
     *
     * UFFD_FEATURE_WP_UNPOPULATED indicates that userfaultfd
     * write-protection mode will always apply to unpopulated pages
     * (i.e. empty ptes).  This will be the default behavior for shmem
     * & hugetlbfs, so this flag only affects anonymous memory behavior
     * when userfault write-protection mode is registered.
     *
     * UFFD_FEATURE_WP_ASYNC indicates that userfaultfd write-protection
     * asynchronous mode is supported in which the write fault is
     * automatically resolved and write-protection is un-set.
     * It implies UFFD_FEATURE_WP_UNPOPULATED.
     *
     * UFFD_FEATURE_MOVE indicates that the kernel supports moving an
     * existing page contents from userspace.
     *
     * UFFD_FEATURE_RWP indicates that the kernel supports
     * UFFDIO_REGISTER_MODE_RWP for read-write protection tracking.
     * Pages are made inaccessible via UFFDIO_RWPROTECT and faults
     * are delivered when the pages are re-accessed.
     *
     * UFFD_FEATURE_RWP_ASYNC indicates asynchronous mode for
     * UFFDIO_REGISTER_MODE_RWP.  When set, faults on read-write
     * protected pages are auto-resolved by the kernel (PTE
     * permissions restored immediately) without delivering a message
     * to the userfaultfd handler.  Use PAGEMAP_SCAN with inverted
     * PAGE_IS_ACCESSED to find pages that were not re-accessed.
     */
    pub features: __u64,
    pub ioctls: __u64,
}

pub const UFFD_FEATURE_PAGEFAULT_FLAG_WP: core::ffi::c_uint = 1 << 0;
pub const UFFD_FEATURE_EVENT_FORK: core::ffi::c_uint = 1 << 1;
pub const UFFD_FEATURE_EVENT_REMAP: core::ffi::c_uint = 1 << 2;
pub const UFFD_FEATURE_EVENT_REMOVE: core::ffi::c_uint = 1 << 3;
pub const UFFD_FEATURE_MISSING_HUGETLBFS: core::ffi::c_uint = 1 << 4;
pub const UFFD_FEATURE_MISSING_SHMEM: core::ffi::c_uint = 1 << 5;
pub const UFFD_FEATURE_EVENT_UNMAP: core::ffi::c_uint = 1 << 6;
pub const UFFD_FEATURE_SIGBUS: core::ffi::c_uint = 1 << 7;
pub const UFFD_FEATURE_THREAD_ID: core::ffi::c_uint = 1 << 8;
pub const UFFD_FEATURE_MINOR_HUGETLBFS: core::ffi::c_uint = 1 << 9;
pub const UFFD_FEATURE_MINOR_SHMEM: core::ffi::c_uint = 1 << 10;
pub const UFFD_FEATURE_EXACT_ADDRESS: core::ffi::c_uint = 1 << 11;
pub const UFFD_FEATURE_WP_HUGETLBFS_SHMEM: core::ffi::c_uint = 1 << 12;
pub const UFFD_FEATURE_WP_UNPOPULATED: core::ffi::c_uint = 1 << 13;
pub const UFFD_FEATURE_POISON: core::ffi::c_uint = 1 << 14;
pub const UFFD_FEATURE_WP_ASYNC: core::ffi::c_uint = 1 << 15;
pub const UFFD_FEATURE_MOVE: core::ffi::c_uint = 1 << 16;
pub const UFFD_FEATURE_RWP: core::ffi::c_uint = 1 << 17;
pub const UFFD_FEATURE_RWP_ASYNC: core::ffi::c_uint = 1 << 18;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uffdio_range {
    pub start: __u64,
    pub len: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uffdio_register {
    pub range: uffdio_range,
    pub mode: __u64,
    /*
     * kernel answers which ioctl commands are available for the
     * range, keep at the end as the last 8 bytes aren't read.
     */
    pub ioctls: __u64,
}

pub const UFFDIO_REGISTER_MODE_MISSING: __u64 = 1u64 << 0;
pub const UFFDIO_REGISTER_MODE_WP: __u64 = 1u64 << 1;
pub const UFFDIO_REGISTER_MODE_MINOR: __u64 = 1u64 << 2;
pub const UFFDIO_REGISTER_MODE_RWP: __u64 = 1u64 << 3;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uffdio_copy {
    pub dst: __u64,
    pub src: __u64,
    pub len: __u64,
    pub mode: __u64,
    /*
     * "copy" is written by the ioctl and must be at the end: the
     * copy_from_user will not read the last 8 bytes.
     */
    pub copy: __s64,
}

pub const UFFDIO_COPY_MODE_DONTWAKE: __u64 = 1u64 << 0;
/*
 * UFFDIO_COPY_MODE_WP will map the page write protected on
 * the fly.  UFFDIO_COPY_MODE_WP is available only if the
 * write protected ioctl is implemented for the range
 * according to the uffdio_register.ioctls.
 */
pub const UFFDIO_COPY_MODE_WP: __u64 = 1u64 << 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uffdio_zeropage {
    pub range: uffdio_range,
    pub mode: __u64,
    /*
     * "zeropage" is written by the ioctl and must be at the end:
     * the copy_from_user will not read the last 8 bytes.
     */
    pub zeropage: __s64,
}

pub const UFFDIO_ZEROPAGE_MODE_DONTWAKE: __u64 = 1u64 << 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uffdio_writeprotect {
    pub range: uffdio_range,
    pub mode: __u64,
}

/*
 * UFFDIO_WRITEPROTECT_MODE_WP: set the flag to write protect a range,
 * unset the flag to undo protection of a range which was previously
 * write protected.
 *
 * UFFDIO_WRITEPROTECT_MODE_DONTWAKE: set the flag to avoid waking up
 * any wait thread after the operation succeeds.
 *
 * NOTE: Write protecting a region (WP=1) is unrelated to page faults,
 * therefore DONTWAKE flag is meaningless with WP=1.  Removing write
 * protection (WP=0) in response to a page fault wakes the faulting
 * task unless DONTWAKE is set.
 */
pub const UFFDIO_WRITEPROTECT_MODE_WP: __u64 = 1u64 << 0;
pub const UFFDIO_WRITEPROTECT_MODE_DONTWAKE: __u64 = 1u64 << 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uffdio_continue {
    pub range: uffdio_range,
    pub mode: __u64,
    /*
     * Fields below here are written by the ioctl and must be at the end:
     * the copy_from_user will not read past here.
     */
    pub mapped: __s64,
}

pub const UFFDIO_CONTINUE_MODE_DONTWAKE: __u64 = 1u64 << 0;
/*
 * UFFDIO_CONTINUE_MODE_WP will map the page write protected on
 * the fly.  UFFDIO_CONTINUE_MODE_WP is available only if the
 * write protected ioctl is implemented for the range
 * according to the uffdio_register.ioctls.
 */
pub const UFFDIO_CONTINUE_MODE_WP: __u64 = 1u64 << 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uffdio_poison {
    pub range: uffdio_range,
    pub mode: __u64,
    /*
     * Fields below here are written by the ioctl and must be at the end:
     * the copy_from_user will not read past here.
     */
    pub updated: __s64,
}

pub const UFFDIO_POISON_MODE_DONTWAKE: __u64 = 1u64 << 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uffdio_rwprotect {
    pub range: uffdio_range,
    /* !RWP means undo RWP-protection */
    pub mode: __u64,
}

pub const UFFDIO_RWPROTECT_MODE_RWP: __u64 = 1u64 << 0;
pub const UFFDIO_RWPROTECT_MODE_DONTWAKE: __u64 = 1u64 << 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uffdio_move {
    pub dst: __u64,
    pub src: __u64,
    pub len: __u64,
    /*
     * Especially if used to atomically remove memory from the
     * address space the wake on the dst range is not needed.
     */
    pub mode: __u64,
    /*
     * "move" is written by the ioctl and must be at the end: the
     * copy_from_user will not read the last 8 bytes.
     */
    pub move_: __s64,
}

pub const UFFDIO_MOVE_MODE_DONTWAKE: __u64 = 1u64 << 0;
pub const UFFDIO_MOVE_MODE_ALLOW_SRC_HOLES: __u64 = 1u64 << 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uffdio_set_mode {
    /*
     * Toggle async mode for features at runtime.
     * Supported: UFFD_FEATURE_RWP_ASYNC.
     * Setting a bit in both enable and disable is invalid.
     */
    pub enable: __u64,
    pub disable: __u64,
}

/*
 * Flags for the userfaultfd(2) system call itself.
 */

/*
 * Create a userfaultfd that can handle page faults only in user mode.
 */
pub const UFFD_USER_MODE_ONLY: core::ffi::c_uint = 1;
