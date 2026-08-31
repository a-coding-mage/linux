/* SPDX-License-Identifier: (LGPL-2.1 OR BSD-2-Clause) */

/*
 * AF_XDP user-space access library.
 *
 * Copyright (c) 2018 - 2019 Intel Corporation.
 * Copyright (c) 2019 Facebook
 *
 * Author(s): Magnus Karlsson <magnus.karlsson@intel.com>
 */

/*
 * C dependencies removed from executable Rust:
 * <stdio.h>, <stdint.h>, <stdbool.h>, <linux/if_xdp.h>, <bpf/libbpf.h>.
 * The external types and constants referenced here are expected to be supplied
 * by the surrounding translation unit or bindings.
 */

use core::ffi::{c_int, c_void};
use core::sync::atomic::{AtomicU32, Ordering};

pub type __u16 = u16;
pub type __u32 = u32;
pub type __u64 = u64;

unsafe extern "C" {
    pub type xdp_desc;
    pub type xsk_umem;
    pub type xsk_socket;
    pub type bpf_program;
    pub type bpf_map;
}

unsafe extern "C" {
    pub static XDP_RING_NEED_WAKEUP: __u32;
    pub static XSK_UNALIGNED_BUF_ADDR_MASK: __u64;
    pub static XSK_UNALIGNED_BUF_OFFSET_SHIFT: __u32;
}

/* Do not access these members directly. Use the functions below. */
#[repr(C)]
pub struct xsk_ring_prod {
    pub cached_prod: __u32,
    pub cached_cons: __u32,
    pub mask: __u32,
    pub size: __u32,
    pub producer: *mut __u32,
    pub consumer: *mut __u32,
    pub ring: *mut c_void,
    pub flags: *mut __u32,
}

#[repr(C)]
pub struct xsk_ring_cons {
    pub cached_prod: __u32,
    pub cached_cons: __u32,
    pub mask: __u32,
    pub size: __u32,
    pub producer: *mut __u32,
    pub consumer: *mut __u32,
    pub ring: *mut c_void,
    pub flags: *mut __u32,
}

/* For a detailed explanation on the memory barriers associated with the
 * ring, please take a look at net/xdp/xsk_queue.h.
 */

#[inline]
pub unsafe fn xsk_ring_prod__fill_addr(fill: *mut xsk_ring_prod, idx: __u32) -> *mut __u64 {
    let addrs = unsafe { (*fill).ring as *mut __u64 };

    unsafe { addrs.add((idx & (*fill).mask) as usize) }
}

#[inline]
pub unsafe fn xsk_ring_cons__comp_addr(
    comp: *const xsk_ring_cons,
    idx: __u32,
) -> *const __u64 {
    let addrs = unsafe { (*comp).ring as *const __u64 };

    unsafe { addrs.add((idx & (*comp).mask) as usize) }
}

#[inline]
pub unsafe fn xsk_ring_prod__tx_desc(
    tx: *mut xsk_ring_prod,
    idx: __u32,
) -> *mut xdp_desc {
    let descs = unsafe { (*tx).ring as *mut xdp_desc };

    unsafe { descs.add((idx & (*tx).mask) as usize) }
}

#[inline]
pub unsafe fn xsk_ring_cons__rx_desc(
    rx: *const xsk_ring_cons,
    idx: __u32,
) -> *const xdp_desc {
    let descs = unsafe { (*rx).ring as *const xdp_desc };

    unsafe { descs.add((idx & (*rx).mask) as usize) }
}

#[inline]
pub unsafe fn xsk_ring_prod__needs_wakeup(r: *const xsk_ring_prod) -> c_int {
    unsafe { (*(*r).flags & XDP_RING_NEED_WAKEUP) as c_int }
}

#[inline]
pub unsafe fn xsk_prod_nb_free(r: *mut xsk_ring_prod, nb: __u32) -> __u32 {
    let mut free_entries = unsafe { (*r).cached_cons.wrapping_sub((*r).cached_prod) };

    if free_entries >= nb {
        return free_entries;
    }

    /* Refresh the local tail pointer.
     * cached_cons is r->size bigger than the real consumer pointer so
     * that this addition can be avoided in the more frequently
     * executed code that computes free_entries in the beginning of
     * this function. Without this optimization it would have been
     * free_entries = r->cached_prod - r->cached_cons + r->size.
     */
    unsafe {
        (*r).cached_cons = AtomicU32::from_ptr((*r).consumer).load(Ordering::Acquire);
        (*r).cached_cons = (*r).cached_cons.wrapping_add((*r).size);
        free_entries = (*r).cached_cons.wrapping_sub((*r).cached_prod);
    }

    free_entries
}

#[inline]
pub unsafe fn xsk_cons_nb_avail(r: *mut xsk_ring_cons, nb: __u32) -> __u32 {
    let mut entries = unsafe { (*r).cached_prod.wrapping_sub((*r).cached_cons) };

    if entries == 0 {
        unsafe {
            (*r).cached_prod = AtomicU32::from_ptr((*r).producer).load(Ordering::Acquire);
            entries = (*r).cached_prod.wrapping_sub((*r).cached_cons);
        }
    }

    if entries > nb {
        nb
    } else {
        entries
    }
}

#[inline]
pub unsafe fn xsk_ring_prod__reserve(
    prod: *mut xsk_ring_prod,
    nb: __u32,
    idx: *mut __u32,
) -> __u32 {
    if unsafe { xsk_prod_nb_free(prod, nb) } < nb {
        return 0;
    }

    unsafe {
        *idx = (*prod).cached_prod;
        (*prod).cached_prod = (*prod).cached_prod.wrapping_add(nb);
    }

    nb
}

#[inline]
pub unsafe fn xsk_ring_prod__submit(prod: *mut xsk_ring_prod, nb: __u32) {
    /* Make sure everything has been written to the ring before indicating
     * this to the kernel by writing the producer pointer.
     */
    unsafe {
        let producer = (*prod).producer;
        let value = (*producer).wrapping_add(nb);
        AtomicU32::from_ptr(producer).store(value, Ordering::Release);
    }
}

#[inline]
pub unsafe fn xsk_ring_prod__cancel(prod: *mut xsk_ring_prod, nb: __u32) {
    unsafe {
        (*prod).cached_prod = (*prod).cached_prod.wrapping_sub(nb);
    }
}

#[inline]
pub unsafe fn xsk_ring_cons__peek(
    cons: *mut xsk_ring_cons,
    nb: __u32,
    idx: *mut __u32,
) -> __u32 {
    let entries = unsafe { xsk_cons_nb_avail(cons, nb) };

    if entries > 0 {
        unsafe {
            *idx = (*cons).cached_cons;
            (*cons).cached_cons = (*cons).cached_cons.wrapping_add(entries);
        }
    }

    entries
}

#[inline]
pub unsafe fn xsk_ring_cons__cancel(cons: *mut xsk_ring_cons, nb: __u32) {
    unsafe {
        (*cons).cached_cons = (*cons).cached_cons.wrapping_sub(nb);
    }
}

#[inline]
pub unsafe fn xsk_ring_cons__release(cons: *mut xsk_ring_cons, nb: __u32) {
    /* Make sure data has been read before indicating we are done
     * with the entries by updating the consumer pointer.
     */
    unsafe {
        let consumer = (*cons).consumer;
        let value = (*consumer).wrapping_add(nb);
        AtomicU32::from_ptr(consumer).store(value, Ordering::Release);
    }
}

#[inline]
pub unsafe fn xsk_umem__get_data(umem_area: *mut c_void, addr: __u64) -> *mut c_void {
    unsafe { (umem_area as *mut i8).add(addr as usize) as *mut c_void }
}

#[inline]
pub fn xsk_umem__extract_addr(addr: __u64) -> __u64 {
    unsafe { addr & XSK_UNALIGNED_BUF_ADDR_MASK }
}

#[inline]
pub fn xsk_umem__extract_offset(addr: __u64) -> __u64 {
    unsafe { addr >> XSK_UNALIGNED_BUF_OFFSET_SHIFT }
}

#[inline]
pub fn xsk_umem__add_offset_to_addr(addr: __u64) -> __u64 {
    xsk_umem__extract_addr(addr).wrapping_add(xsk_umem__extract_offset(addr))
}

unsafe extern "C" {
    pub fn xsk_umem__fd(umem: *const xsk_umem) -> c_int;
    pub fn xsk_socket__fd(xsk: *const xsk_socket) -> c_int;
}

pub const XSK_RING_CONS__DEFAULT_NUM_DESCS: __u32 = 2048;
pub const XSK_RING_PROD__DEFAULT_NUM_DESCS: __u32 = 2048;
pub const XSK_UMEM__DEFAULT_FRAME_SHIFT: __u32 = 12; /* 4096 bytes */
pub const XSK_UMEM__DEFAULT_FRAME_SIZE: __u32 = 1 << XSK_UMEM__DEFAULT_FRAME_SHIFT;
pub const XSK_UMEM__DEFAULT_FRAME_HEADROOM: __u32 = 0;
pub const XSK_UMEM__DEFAULT_FLAGS: __u32 = 0;

#[repr(C)]
pub struct xsk_umem_config {
    pub fill_size: __u32,
    pub comp_size: __u32,
    pub frame_size: __u32,
    pub frame_headroom: __u32,
    pub flags: __u32,
    pub tx_metadata_len: __u32,
}

unsafe extern "C" {
    pub fn xsk_attach_xdp_program(
        prog: *mut bpf_program,
        ifindex: c_int,
        xdp_flags: u32,
    ) -> c_int;
    pub fn xsk_detach_xdp_program(ifindex: c_int, xdp_flags: u32);
    pub fn xsk_update_xskmap(map: *mut bpf_map, xsk: *mut xsk_socket, index: u32) -> c_int;
    pub fn xsk_clear_xskmap(map: *mut bpf_map);
    pub fn xsk_is_in_mode(ifindex: u32, mode: c_int) -> bool;
}

#[repr(C)]
pub struct xsk_socket_config {
    pub rx_size: __u32,
    pub tx_size: __u32,
    pub bind_flags: __u16,
}

/* Set config to NULL to get the default configuration. */
unsafe extern "C" {
    pub fn xsk_umem__create(
        umem: *mut *mut xsk_umem,
        umem_area: *mut c_void,
        size: __u64,
        fill: *mut xsk_ring_prod,
        comp: *mut xsk_ring_cons,
        config: *const xsk_umem_config,
    ) -> c_int;
    pub fn xsk_socket__create(
        xsk: *mut *mut xsk_socket,
        ifindex: c_int,
        queue_id: __u32,
        umem: *mut xsk_umem,
        rx: *mut xsk_ring_cons,
        tx: *mut xsk_ring_prod,
        config: *const xsk_socket_config,
    ) -> c_int;
    pub fn xsk_socket__create_shared(
        xsk_ptr: *mut *mut xsk_socket,
        ifindex: c_int,
        queue_id: __u32,
        umem: *mut xsk_umem,
        rx: *mut xsk_ring_cons,
        tx: *mut xsk_ring_prod,
        fill: *mut xsk_ring_prod,
        comp: *mut xsk_ring_cons,
        config: *const xsk_socket_config,
    ) -> c_int;

    /* Returns 0 for success and -EBUSY if the umem is still in use. */
    pub fn xsk_umem__delete(umem: *mut xsk_umem) -> c_int;
    pub fn xsk_socket__delete(xsk: *mut xsk_socket);

    pub fn xsk_set_mtu(ifindex: c_int, mtu: c_int) -> c_int;
}
