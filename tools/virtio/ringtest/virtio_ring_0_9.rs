// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2016 Red Hat, Inc.
 * Author: Michael S. Tsirkin <mst@redhat.com>
 *
 * Partial implementation of virtio 0.9. event index is used for signalling,
 * unconditionally. Design roughly follows linux kernel implementation in order
 * to be able to judge its performance.
 */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

// Dependencies originally provided by main.h, stdlib.h, stdio.h, string.h,
// and linux/virtio_ring.h.

pub const VRING_DESC_F_NEXT: u16 = 1;

#[repr(C)]
pub struct vring_desc {
    pub addr: u64,
    pub len: u32,
    pub flags: u16,
    pub next: u16,
}

#[repr(C)]
pub struct vring_avail {
    pub flags: u16,
    pub idx: u16,
    pub ring: [u16; 0],
}

#[repr(C)]
pub struct vring_used_elem {
    pub id: u32,
    pub len: u32,
}

#[repr(C)]
pub struct vring_used {
    pub flags: u16,
    pub idx: u16,
    pub ring: [vring_used_elem; 0],
}

#[repr(C)]
pub struct vring {
    pub num: c_uint,
    pub desc: *mut vring_desc,
    pub avail: *mut vring_avail,
    pub used: *mut vring_used,
}

unsafe extern "C" {
    static mut ring_size: c_uint;

    fn posix_memalign(memptr: *mut *mut c_void, alignment: usize, size: usize) -> c_int;
    fn perror(s: *const c_char);
    fn exit(status: c_int) -> !;
    fn malloc(size: usize) -> *mut c_void;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;

    fn vring_size(num: c_uint, align: c_ulong) -> c_ulong;
    fn vring_init(vr: *mut vring, num: c_uint, p: *mut c_void, align: c_ulong);
    fn vring_need_event(event_idx: u16, new_idx: u16, old_idx: u16) -> bool;
    fn vring_used_event(vr: *mut vring) -> *mut u16;
    fn vring_avail_event(vr: *mut vring) -> u16;

    fn smp_release();
    fn smp_acquire();
    fn smp_mb();
    fn kick();
}

#[repr(C)]
pub struct data {
    pub data: *mut c_void,
}

pub static mut data: *mut data = ptr::null_mut();

pub static mut ring: vring = vring {
    num: 0,
    desc: ptr::null_mut(),
    avail: ptr::null_mut(),
    used: ptr::null_mut(),
};

/* enabling the below activates experimental ring polling code
 * (which skips index reads on consumer in favor of looking at
 * high bits of ring id ^ 0x8000).
 */
/* #ifdef RING_POLL */
/* enabling the below activates experimental in-order code
 * (which skips ring updates and reads and writes len in descriptor).
 */
/* #ifdef INORDER */

// RING_POLL and INORDER are mutually exclusive in the original C source.
#[cfg(all(feature = "RING_POLL", feature = "INORDER"))]
compile_error!("RING_POLL and INORDER are mutually exclusive");

/* how much padding is needed to avoid false cache sharing */
pub const HOST_GUEST_PADDING: usize = 0x80;

#[repr(C)]
pub struct guest {
    pub avail_idx: u16,
    pub last_used_idx: u16,
    pub num_free: u16,
    pub kicked_avail_idx: u16,
    #[cfg(not(feature = "INORDER"))]
    pub free_head: u16,
    #[cfg(feature = "INORDER")]
    pub reserved_free_head: u16,
    pub reserved: [u8; HOST_GUEST_PADDING - 10],
}

pub static mut guest: guest = guest {
    avail_idx: 0,
    last_used_idx: 0,
    num_free: 0,
    kicked_avail_idx: 0,
    #[cfg(not(feature = "INORDER"))]
    free_head: 0,
    #[cfg(feature = "INORDER")]
    reserved_free_head: 0,
    reserved: [0; HOST_GUEST_PADDING - 10],
};

#[repr(C)]
pub struct host {
    /* we do not need to track last avail index
     * unless we have more than one in flight.
     */
    pub used_idx: u16,
    pub called_used_idx: u16,
    pub reserved: [u8; HOST_GUEST_PADDING - 4],
}

pub static mut host: host = host {
    used_idx: 0,
    called_used_idx: 0,
    reserved: [0; HOST_GUEST_PADDING - 4],
};

/* implemented by ring */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn alloc_ring() {
    let mut ret: c_int;
    let mut i: c_int;
    let mut p: *mut c_void = ptr::null_mut();

    ret = unsafe { posix_memalign(&mut p, 0x1000, vring_size(unsafe { ring_size }, 0x1000) as usize) };
    if ret != 0 {
        unsafe { perror(c"Unable to allocate ring buffer.\n".as_ptr()) };
        unsafe { exit(3) };
    }
    unsafe {
        memset(
            p,
            0,
            vring_size(ring_size, 0x1000) as usize,
        );
        vring_init(&raw mut ring, ring_size, p, 0x1000);
    }

    unsafe {
        guest.avail_idx = 0;
        guest.kicked_avail_idx = (-1_i32) as u16;
        guest.last_used_idx = 0;
    }
    #[cfg(not(feature = "INORDER"))]
    unsafe {
        /* Put everything in free lists. */
        guest.free_head = 0;
    }
    i = 0;
    while i < unsafe { ring_size as c_int - 1 } {
        unsafe {
            (*ring.desc.add(i as usize)).next = (i + 1) as u16;
        }
        i += 1;
    }
    unsafe {
        host.used_idx = 0;
        host.called_used_idx = (-1_i32) as u16;
        guest.num_free = ring_size as u16;
        data = malloc(ring_size as usize * size_of::<data>()) as *mut data;
    }
    if unsafe { data.is_null() } {
        unsafe { perror(c"Unable to allocate data buffer.\n".as_ptr()) };
        unsafe { exit(3) };
    }
    unsafe {
        memset(data as *mut c_void, 0, ring_size as usize * size_of::<data>());
    }
}

/* guest side */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn add_inbuf(len: c_uint, buf: *mut c_void, datap: *mut c_void) -> c_int {
    let head: c_uint;
    #[cfg(any(feature = "RING_POLL", all(not(feature = "RING_POLL"), not(feature = "INORDER"))))]
    let avail: c_uint;
    let desc: *mut vring_desc;

    if unsafe { guest.num_free } == 0 {
        return -1;
    }

    #[cfg(feature = "INORDER")]
    {
        head = unsafe {
            let old = guest.avail_idx;
            guest.avail_idx = guest.avail_idx.wrapping_add(1);
            (ring_size - 1) & old as c_uint
        };
    }
    #[cfg(not(feature = "INORDER"))]
    {
        head = unsafe { guest.free_head as c_uint };
    }
    unsafe {
        guest.num_free = guest.num_free.wrapping_sub(1);
    }

    desc = unsafe { ring.desc };
    unsafe {
        (*desc.add(head as usize)).flags = VRING_DESC_F_NEXT;
        (*desc.add(head as usize)).addr = buf as c_ulong as u64;
        (*desc.add(head as usize)).len = len;
    }
    /* We do it like this to simulate the way
     * we'd have to flip it if we had multiple
     * descriptors.
     */
    unsafe {
        (*desc.add(head as usize)).flags &= !VRING_DESC_F_NEXT;
    }
    #[cfg(not(feature = "INORDER"))]
    unsafe {
        guest.free_head = (*desc.add(head as usize)).next;
    }

    unsafe {
        (*data.add(head as usize)).data = datap;
    }

    #[cfg(feature = "RING_POLL")]
    {
        /* Barrier A (for pairing) */
        unsafe { smp_release() };
        avail = unsafe {
            let old = guest.avail_idx;
            guest.avail_idx = guest.avail_idx.wrapping_add(1);
            old as c_uint
        };
        unsafe {
            *((*ring.avail).ring.as_mut_ptr()).add((avail & (ring_size - 1)) as usize) =
                ((head | (avail & !(ring_size - 1))) ^ 0x8000) as u16;
        }
    }
    #[cfg(not(feature = "RING_POLL"))]
    {
        #[cfg(not(feature = "INORDER"))]
        {
            /* Barrier A (for pairing) */
            unsafe { smp_release() };
            avail = unsafe {
                let old = guest.avail_idx;
                guest.avail_idx = guest.avail_idx.wrapping_add(1);
                (ring_size - 1) & old as c_uint
            };
            unsafe {
                *((*ring.avail).ring.as_mut_ptr()).add(avail as usize) = head as u16;
            }
        }
        /* Barrier A (for pairing) */
        unsafe { smp_release() };
    }
    unsafe {
        (*ring.avail).idx = guest.avail_idx;
    }
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_buf(lenp: *mut c_uint, bufp: *mut *mut c_void) -> *mut c_void {
    let head: c_uint;
    let mut index: c_uint;
    let datap: *mut c_void;

    #[cfg(feature = "RING_POLL")]
    {
        head = unsafe { (ring_size - 1) & guest.last_used_idx as c_uint };
        index = unsafe { (*((*ring.used).ring.as_ptr()).add(head as usize)).id };
        if ((index ^ unsafe { guest.last_used_idx as c_uint } ^ 0x8000) & unsafe { !(ring_size - 1) }) != 0 {
            return ptr::null_mut();
        }
        /* Barrier B (for pairing) */
        unsafe { smp_acquire() };
        index &= unsafe { ring_size - 1 };
    }
    #[cfg(not(feature = "RING_POLL"))]
    {
        if unsafe { (*ring.used).idx } == unsafe { guest.last_used_idx } {
            return ptr::null_mut();
        }
        /* Barrier B (for pairing) */
        unsafe { smp_acquire() };
        #[cfg(feature = "INORDER")]
        {
            head = unsafe { (ring_size - 1) & guest.last_used_idx as c_uint };
            index = head;
        }
        #[cfg(not(feature = "INORDER"))]
        {
            head = unsafe { (ring_size - 1) & guest.last_used_idx as c_uint };
            index = unsafe { (*((*ring.used).ring.as_ptr()).add(head as usize)).id };
        }
    }
    #[cfg(feature = "INORDER")]
    unsafe {
        *lenp = (*ring.desc.add(index as usize)).len;
    }
    #[cfg(not(feature = "INORDER"))]
    unsafe {
        *lenp = (*((*ring.used).ring.as_ptr()).add(head as usize)).len;
    }
    datap = unsafe { (*data.add(index as usize)).data };
    unsafe {
        *bufp = (*ring.desc.add(index as usize)).addr as c_ulong as *mut c_void;
        (*data.add(index as usize)).data = ptr::null_mut();
    }
    #[cfg(not(feature = "INORDER"))]
    unsafe {
        (*ring.desc.add(index as usize)).next = guest.free_head;
        guest.free_head = index as u16;
    }
    unsafe {
        guest.num_free = guest.num_free.wrapping_add(1);
        guest.last_used_idx = guest.last_used_idx.wrapping_add(1);
    }
    datap
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn used_empty() -> bool {
    let last_used_idx: u16 = unsafe { guest.last_used_idx };
    #[cfg(feature = "RING_POLL")]
    {
        let head: u16 = unsafe { last_used_idx & (ring_size - 1) as u16 };
        let index: c_uint = unsafe { (*((*ring.used).ring.as_ptr()).add(head as usize)).id };

        return ((index ^ last_used_idx as c_uint ^ 0x8000) & unsafe { !(ring_size - 1) }) != 0;
    }
    #[cfg(not(feature = "RING_POLL"))]
    {
        unsafe { (*ring.used).idx == last_used_idx }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn disable_call() {
    /* Doing nothing to disable calls might cause
     * extra interrupts, but reduces the number of cache misses.
     */
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn enable_call() -> bool {
    unsafe {
        *vring_used_event(&raw mut ring) = guest.last_used_idx;
    }
    /* Flush call index write */
    /* Barrier D (for pairing) */
    unsafe { smp_mb() };
    unsafe { used_empty() }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn kick_available() {
    let need: bool;

    /* Flush in previous flags write */
    /* Barrier C (for pairing) */
    unsafe { smp_mb() };
    need = unsafe {
        vring_need_event(
            vring_avail_event(&raw mut ring),
            guest.avail_idx,
            guest.kicked_avail_idx,
        )
    };

    unsafe {
        guest.kicked_avail_idx = guest.avail_idx;
    }
    if need {
        unsafe { kick() };
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
