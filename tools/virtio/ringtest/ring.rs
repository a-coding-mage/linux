// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2016 Red Hat, Inc.
 * Author: Michael S. Tsirkin <mst@redhat.com>
 *
 * Simple descriptor-based ring. virtio 0.9 compatible event index is used for
 * signalling, unconditionally.
 */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_ulonglong, c_void};
use core::mem::size_of;
use core::ptr::null_mut;

unsafe extern "C" {
    static ring_size: c_uint;

    fn barrier();
    fn smp_release();
    fn smp_acquire();
    fn smp_mb();
    fn kick();
    fn call();

    fn posix_memalign(memptr: *mut *mut c_void, alignment: usize, size: usize) -> c_int;
    fn calloc(nmemb: usize, size: usize) -> *mut c_void;
    fn perror(s: *const c_char);
    fn exit(status: c_int) -> !;
}

/* Next - Where next entry will be written.
 * Prev - "Next" value when event triggered previously.
 * Event - Peer requested event after writing this entry.
 */
#[inline]
fn need_event(event: u16, next: u16, prev: u16) -> bool {
    next.wrapping_sub(event).wrapping_sub(1) < next.wrapping_sub(prev)
}

/* Design:
 * Guest adds descriptors with unique index values and DESC_HW in flags.
 * Host overwrites used descriptors with correct len, index, and DESC_HW clear.
 * Flags are always set last.
 */
const DESC_HW: u16 = 0x1;

#[repr(C)]
#[derive(Copy, Clone)]
struct desc {
    flags: u16,
    index: u16,
    len: c_uint,
    addr: c_ulonglong,
}

/* how much padding is needed to avoid false cache sharing */
const HOST_GUEST_PADDING: usize = 0x80;

/* Mostly read */
#[repr(C)]
struct event {
    kick_index: u16,
    reserved0: [u8; HOST_GUEST_PADDING - 2],
    call_index: u16,
    reserved1: [u8; HOST_GUEST_PADDING - 2],
}

#[repr(C)]
struct data {
    buf: *mut c_void, /* descriptor is writeable, we can't get buf from there */
    data: *mut c_void,
}

static mut data: *mut data = null_mut();

static mut ring: *mut desc = null_mut();
static mut event: *mut event = null_mut();

#[repr(C)]
struct guest {
    avail_idx: c_uint,
    last_used_idx: c_uint,
    num_free: c_uint,
    kicked_avail_idx: c_uint,
    reserved: [u8; HOST_GUEST_PADDING - 12],
}

static mut guest: guest = guest {
    avail_idx: 0,
    last_used_idx: 0,
    num_free: 0,
    kicked_avail_idx: 0,
    reserved: [0; HOST_GUEST_PADDING - 12],
};

#[repr(C)]
struct host {
    /* we do not need to track last avail index
     * unless we have more than one in flight.
     */
    used_idx: c_uint,
    called_used_idx: c_uint,
    reserved: [u8; HOST_GUEST_PADDING - 4],
}

static mut host: host = host {
    used_idx: 0,
    called_used_idx: 0,
    reserved: [0; HOST_GUEST_PADDING - 4],
};

/* implemented by ring */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn alloc_ring() {
    let mut ret: c_int;
    let mut i: c_int;

    ret = unsafe {
        posix_memalign(
            core::ptr::addr_of_mut!(ring).cast::<*mut c_void>(),
            0x1000,
            ring_size as usize * size_of::<desc>(),
        )
    };
    if ret != 0 {
        unsafe {
            perror(c"Unable to allocate ring buffer.\n".as_ptr());
            exit(3);
        }
    }
    unsafe {
        event = calloc(1, size_of::<event>()).cast::<event>();
    }
    if unsafe { event.is_null() } {
        unsafe {
            perror(c"Unable to allocate event buffer.\n".as_ptr());
            exit(3);
        }
    }
    unsafe {
        guest.avail_idx = 0;
        guest.kicked_avail_idx = (-1i32) as c_uint;
        guest.last_used_idx = 0;
        host.used_idx = 0;
        host.called_used_idx = (-1i32) as c_uint;
    }
    i = 0;
    while unsafe { i < ring_size as c_int } {
        let desc = desc {
            flags: 0,
            index: i as u16,
            len: 0,
            addr: 0,
        };
        unsafe {
            *ring.add(i as usize) = desc;
        }
        i += 1;
    }
    unsafe {
        guest.num_free = ring_size;
        data = calloc(ring_size as usize, size_of::<data>()).cast::<data>();
    }
    if unsafe { data.is_null() } {
        unsafe {
            perror(c"Unable to allocate data buffer.\n".as_ptr());
            exit(3);
        }
    }
}

/* guest side */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn add_inbuf(len: c_uint, buf: *mut c_void, datap: *mut c_void) -> c_int {
    let head: c_uint;
    let index: c_uint;

    if unsafe { guest.num_free == 0 } {
        return -1;
    }

    unsafe {
        guest.num_free -= 1;
        head = (ring_size - 1) & guest.avail_idx;
        guest.avail_idx = guest.avail_idx.wrapping_add(1);
    }

    /* Start with a write. On MESI architectures this helps
     * avoid a shared state with consumer that is polling this descriptor.
     */
    unsafe {
        (*ring.add(head as usize)).addr = buf as c_ulong as c_ulonglong;
        (*ring.add(head as usize)).len = len;
    }
    /* read below might bypass write above. That is OK because it's just an
     * optimization. If this happens, we will get the cache line in a
     * shared state which is unfortunate, but probably not worth it to
     * add an explicit full barrier to avoid this.
     */
    unsafe {
        barrier();
        index = (*ring.add(head as usize)).index as c_uint;
        (*data.add(index as usize)).buf = buf;
        (*data.add(index as usize)).data = datap;
    }
    /* Barrier A (for pairing) */
    unsafe {
        smp_release();
        (*ring.add(head as usize)).flags = DESC_HW;
    }

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_buf(lenp: *mut c_uint, bufp: *mut *mut c_void) -> *mut c_void {
    let head: c_uint = unsafe { (ring_size - 1) & guest.last_used_idx };
    let index: c_uint;
    let datap: *mut c_void;

    if unsafe { ((*ring.add(head as usize)).flags & DESC_HW) != 0 } {
        return null_mut();
    }
    /* Barrier B (for pairing) */
    unsafe {
        smp_acquire();
        *lenp = (*ring.add(head as usize)).len;
        index = ((*ring.add(head as usize)).index as c_uint) & (ring_size - 1);
        datap = (*data.add(index as usize)).data;
        *bufp = (*data.add(index as usize)).buf;
        (*data.add(index as usize)).buf = null_mut();
        (*data.add(index as usize)).data = null_mut();
        guest.num_free += 1;
        guest.last_used_idx += 1;
    }
    datap
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn used_empty() -> bool {
    let head: c_uint = unsafe { (ring_size - 1) & guest.last_used_idx };

    unsafe { ((*ring.add(head as usize)).flags & DESC_HW) != 0 }
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
        (*event).call_index = guest.last_used_idx as u16;
    }
    /* Flush call index write */
    /* Barrier D (for pairing) */
    unsafe {
        smp_mb();
        used_empty()
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn kick_available() {
    let need: bool;

    /* Flush in previous flags write */
    /* Barrier C (for pairing) */
    unsafe {
        smp_mb();
        need = need_event(
            (*event).kick_index,
            guest.avail_idx as u16,
            guest.kicked_avail_idx as u16,
        );

        guest.kicked_avail_idx = guest.avail_idx;
        if need {
            kick();
        }
    }
}

/* host side */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn disable_kick() {
    /* Doing nothing to disable kicks might cause
     * extra interrupts, but reduces the number of cache misses.
     */
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn enable_kick() -> bool {
    unsafe {
        (*event).kick_index = host.used_idx as u16;
    }
    /* Barrier C (for pairing) */
    unsafe {
        smp_mb();
        avail_empty()
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn avail_empty() -> bool {
    let head: c_uint = unsafe { (ring_size - 1) & host.used_idx };

    unsafe { !(((*ring.add(head as usize)).flags & DESC_HW) != 0) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn use_buf(lenp: *mut c_uint, bufp: *mut *mut c_void) -> bool {
    let head: c_uint = unsafe { (ring_size - 1) & host.used_idx };

    if unsafe { !(((*ring.add(head as usize)).flags & DESC_HW) != 0) } {
        return false;
    }

    /* make sure length read below is not speculated */
    /* Barrier A (for pairing) */
    unsafe {
        smp_acquire();
    }

    /* simple in-order completion: we don't need
     * to touch index at all. This also means we
     * can just modify the descriptor in-place.
     */
    unsafe {
        (*ring.add(head as usize)).len -= 1;
    }
    /* Make sure len is valid before flags.
     * Note: alternative is to write len and flags in one access -
     * possible on 64 bit architectures but wmb is free on Intel anyway
     * so I have no way to test whether it's a gain.
     */
    /* Barrier B (for pairing) */
    unsafe {
        smp_release();
        (*ring.add(head as usize)).flags = 0;
        host.used_idx += 1;
    }
    true
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn call_used() {
    let need: bool;

    /* Flush in previous flags write */
    /* Barrier D (for pairing) */
    unsafe {
        smp_mb();

        need = need_event(
            (*event).call_index,
            host.used_idx as u16,
            host.called_used_idx as u16,
        );

        host.called_used_idx = host.used_idx;

        if need {
            call();
        }
    }
}
