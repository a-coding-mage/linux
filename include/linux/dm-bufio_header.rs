/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2009-2011 Red Hat, Inc.
 *
 * Author: Mikulas Patocka <mpatocka@redhat.com>
 *
 * This file is released under the GPL.
 */

use core::ffi::c_void;

// External kernel types supplied by other translated headers.
#[repr(C)]
pub struct block_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dm_io_client {
    _private: [u8; 0],
}

pub type sector_t = u64;

#[repr(C)]
pub struct dm_bufio_client {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dm_buffer {
    _private: [u8; 0],
}

/* Flags for dm_bufio_client_create */
pub const DM_BUFIO_CLIENT_NO_SLEEP: u32 = 0x1;

/* Create a buffered IO cache on a given device */
extern "C" {
    pub fn dm_bufio_client_create(
        bdev: *mut block_device,
        block_size: u32,
        reserved_buffers: u32,
        aux_size: u32,
        alloc_callback: Option<unsafe extern "C" fn(*mut dm_buffer)>,
        write_callback: Option<unsafe extern "C" fn(*mut dm_buffer)>,
        flags: u32,
    ) -> *mut dm_bufio_client;

    /* Release a buffered IO cache. */
    pub fn dm_bufio_client_destroy(c: *mut dm_bufio_client);

    pub fn dm_bufio_client_reset(c: *mut dm_bufio_client);

    /*
     * Set the sector range.
     * When this function is called, there must be no I/O in progress on the bufio
     * client.
     */
    pub fn dm_bufio_set_sector_offset(c: *mut dm_bufio_client, start: sector_t);

    /*
     * Read a given block from disk. Returns pointer to data. Returns a
     * pointer to dm_buffer that can be used to release the buffer or to make
     * it dirty.
     */
    pub fn dm_bufio_read(
        c: *mut dm_bufio_client,
        block: sector_t,
        bp: *mut *mut dm_buffer,
    ) -> *mut c_void;

    pub fn dm_bufio_read_with_ioprio(
        c: *mut dm_bufio_client,
        block: sector_t,
        bp: *mut *mut dm_buffer,
        ioprio: u16,
    ) -> *mut c_void;

    /* Like dm_bufio_read, but return buffer from cache, don't read it. */
    pub fn dm_bufio_get(
        c: *mut dm_bufio_client,
        block: sector_t,
        bp: *mut *mut dm_buffer,
    ) -> *mut c_void;

    /* Like dm_bufio_read, but don't read anything from the disk. */
    pub fn dm_bufio_new(
        c: *mut dm_bufio_client,
        block: sector_t,
        bp: *mut *mut dm_buffer,
    ) -> *mut c_void;

    pub fn dm_bufio_prefetch(c: *mut dm_bufio_client, block: sector_t, n_blocks: u32);

    pub fn dm_bufio_prefetch_with_ioprio(
        c: *mut dm_bufio_client,
        block: sector_t,
        n_blocks: u32,
        ioprio: u16,
    );

    /* Release a reference obtained with dm_bufio_{read,get,new}. */
    pub fn dm_bufio_release(b: *mut dm_buffer);

    pub fn dm_bufio_mark_buffer_dirty(b: *mut dm_buffer);

    pub fn dm_bufio_mark_partial_buffer_dirty(
        b: *mut dm_buffer,
        start: u32,
        end: u32,
    );

    pub fn dm_bufio_write_dirty_buffers_async(c: *mut dm_bufio_client);

    pub fn dm_bufio_write_dirty_buffers(c: *mut dm_bufio_client) -> i32;

    pub fn dm_bufio_issue_flush(c: *mut dm_bufio_client) -> i32;

    pub fn dm_bufio_issue_discard(
        c: *mut dm_bufio_client,
        block: sector_t,
        count: sector_t,
    ) -> i32;

    pub fn dm_bufio_forget(c: *mut dm_bufio_client, block: sector_t);

    pub fn dm_bufio_forget_buffers(
        c: *mut dm_bufio_client,
        block: sector_t,
        n_blocks: sector_t,
    );

    pub fn dm_bufio_set_minimum_buffers(c: *mut dm_bufio_client, n: u32);

    pub fn dm_bufio_get_block_size(c: *mut dm_bufio_client) -> u32;
    pub fn dm_bufio_get_device_size(c: *mut dm_bufio_client) -> sector_t;
    pub fn dm_bufio_get_dm_io_client(c: *mut dm_bufio_client) -> *mut dm_io_client;
    pub fn dm_bufio_get_block_number(b: *mut dm_buffer) -> sector_t;
    pub fn dm_bufio_get_block_data(b: *mut dm_buffer) -> *mut c_void;
    pub fn dm_bufio_get_aux_data(b: *mut dm_buffer) -> *mut c_void;
    pub fn dm_bufio_get_client(b: *mut dm_buffer) -> *mut dm_bufio_client;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
