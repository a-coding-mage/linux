// SPDX-License-Identifier: GPL-2.0-only
/*
 * helpers for managing a buffer for many packets
 *
 * Copyright (c) Clemens Ladisch <clemens@ladisch.de>
 */

/*
 * Translated from the implementation source. Kernel headers and
 * packets-buffer.h provide the referenced types, constants, macros, and
 * functions in the original C translation unit.
 */

use core::ffi::{c_int, c_uint};

unsafe extern "C" {
    fn kmalloc_objs<T>(obj: T, count: c_uint) -> *mut T;
    fn kfree(ptr: *mut core::ffi::c_void);
    fn fw_iso_buffer_init(
        buffer: *mut fw_iso_buffer,
        card: *mut fw_card,
        pages: c_uint,
        direction: dma_data_direction,
    ) -> c_int;
    fn fw_iso_buffer_destroy(buffer: *mut fw_iso_buffer, card: *mut fw_card);
    fn fw_parent_device(unit: *mut fw_unit) -> *mut fw_device;
    fn page_address(page: *mut page) -> *mut core::ffi::c_void;
    fn L1_CACHE_ALIGN(size: c_uint) -> c_uint;
    fn WARN_ON(condition: bool) -> bool;
    fn DIV_ROUND_UP(n: c_uint, d: c_uint) -> c_uint;
}

unsafe extern "C" {
    static PAGE_SIZE: c_uint;
    static ENOMEM: c_int;
    static EINVAL: c_int;
}

/* External declarations supplied by included kernel headers. */
type dma_data_direction = c_uint;
type fw_card = core::ffi::c_void;
type page = core::ffi::c_void;

#[repr(C)]
pub struct fw_device {
    pub card: *mut fw_card,
}

#[repr(C)]
pub struct fw_unit {
    _private: [u8; 0],
}

#[repr(C)]
pub struct fw_iso_buffer {
    pub pages: *mut *mut page,
}

#[repr(C)]
pub struct iso_packet {
    pub buffer: *mut core::ffi::c_void,
    pub offset: c_uint,
}

#[repr(C)]
pub struct iso_packets_buffer {
    pub packets: *mut iso_packet,
    pub iso_buffer: fw_iso_buffer,
}

/**
 * iso_packets_buffer_init - allocates the memory for packets
 * @b: the buffer structure to initialize
 * @unit: the device at the other end of the stream
 * @count: the number of packets
 * @packet_size: the (maximum) size of a packet, in bytes
 * @direction: %DMA_TO_DEVICE or %DMA_FROM_DEVICE
 */
#[no_mangle]
pub unsafe extern "C" fn iso_packets_buffer_init(
    b: *mut iso_packets_buffer,
    unit: *mut fw_unit,
    count: c_uint,
    mut packet_size: c_uint,
    direction: dma_data_direction,
) -> c_int {
    let packets_per_page: c_uint;
    let pages: c_uint;
    let mut i: c_uint;
    let mut page_index: c_uint;
    let mut offset_in_page: c_uint;
    let mut p: *mut core::ffi::c_void;
    let mut err: c_int;

    (*b).packets = kmalloc_objs((*(*b).packets), count);
    if (*b).packets.is_null() {
        err = -ENOMEM;
        return err;
    }

    packet_size = L1_CACHE_ALIGN(packet_size);
    packets_per_page = PAGE_SIZE / packet_size;
    if WARN_ON(packets_per_page == 0) {
        err = -EINVAL;
        kfree((*b).packets as *mut core::ffi::c_void);
        return err;
    }
    pages = DIV_ROUND_UP(count, packets_per_page);

    err = fw_iso_buffer_init(
        &mut (*b).iso_buffer,
        (*fw_parent_device(unit)).card,
        pages,
        direction,
    );
    if err < 0 {
        kfree((*b).packets as *mut core::ffi::c_void);
        return err;
    }

    i = 0;
    while i < count {
        page_index = i / packets_per_page;
        p = page_address(*(*b).iso_buffer.pages.add(page_index as usize));
        offset_in_page = (i % packets_per_page) * packet_size;
        (*(*b).packets.add(i as usize)).buffer = p.add(offset_in_page as usize);
        (*(*b).packets.add(i as usize)).offset = page_index * PAGE_SIZE + offset_in_page;
        i += 1;
    }

    0
}
/* EXPORT_SYMBOL(iso_packets_buffer_init); */

/**
 * iso_packets_buffer_destroy - frees packet buffer resources
 * @b: the buffer structure to free
 * @unit: the device at the other end of the stream
 */
#[no_mangle]
pub unsafe extern "C" fn iso_packets_buffer_destroy(
    b: *mut iso_packets_buffer,
    unit: *mut fw_unit,
) {
    fw_iso_buffer_destroy(&mut (*b).iso_buffer, (*fw_parent_device(unit)).card);
    kfree((*b).packets as *mut core::ffi::c_void);
}
/* EXPORT_SYMBOL(iso_packets_buffer_destroy); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
