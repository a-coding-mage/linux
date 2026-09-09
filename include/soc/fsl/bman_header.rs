/* Copyright 2008 - 2016 Freescale Semiconductor, Inc.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are met:
 *     * Redistributions of source code must retain the above copyright
 *       notice, this list of conditions and the following disclaimer.
 *     * Redistributions in binary form must reproduce the above copyright
 *       notice, this list of conditions and the following disclaimer in the
 *       documentation and/or other materials provided with the distribution.
 *     * Neither the name of Freescale Semiconductor nor the
 *       names of its contributors may be used to endorse or promote products
 *       derived from this software without specific prior written permission.
 *
 * ALTERNATIVELY, this software may be distributed under the terms of the
 * GNU General Public License ("GPL") as published by the Free Software
 * Foundation, either version 2 of that License or (at your option) any later
 * version.
 */

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bm_buffer_fields {
    pub bpid: u16, /* hi 8-bits reserved */
    pub hi: u16,   /* High 16-bits of 48-bit address */
    pub lo: u32,   /* Low 32-bits of 48-bit address */
}

#[repr(C)]
pub union bm_buffer {
    pub fields: bm_buffer_fields,
    pub data: u64,
}

/* Restore the 48 bit address previously stored in BMan hardware pools. */
#[inline]
pub unsafe fn bm_buf_addr(buf: *const bm_buffer) -> u64 {
    (u64::from_be((*buf).data)) & 0xffff_ffff_ffffu64
}

#[inline]
pub unsafe fn bm_buffer_get64(buf: *const bm_buffer) -> u64 {
    (u64::from_be((*buf).data)) & 0xffff_ffff_ffffu64
}

#[inline]
pub unsafe fn bm_buffer_set64(buf: *mut bm_buffer, addr: u64) {
    (*buf).fields.hi = ((addr >> 32) as u16).to_be();
    (*buf).fields.lo = (addr as u32).to_be();
}

#[inline]
pub unsafe fn bm_buffer_get_bpid(buf: *const bm_buffer) -> u8 {
    (u16::from_be((*buf).fields.bpid) & 0xff) as u8
}

#[inline]
pub unsafe fn bm_buffer_set_bpid(buf: *mut bm_buffer, bpid: i32) {
    (*buf).fields.bpid = ((bpid & 0xff) as u16).to_be();
}

/* Managed portal, high-level i/face */

/* Portal and Buffer Pools */
#[repr(C)]
pub struct bman_portal {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bman_pool {
    _private: [u8; 0],
}

pub const BM_POOL_MAX: i32 = 64; /* max # of buffer pools */

extern "C" {
    pub fn bman_new_pool() -> *mut bman_pool;
    pub fn bman_free_pool(pool: *mut bman_pool);
    pub fn bman_get_bpid(pool: *const bman_pool) -> i32;
    pub fn bman_release(pool: *mut bman_pool, bufs: *const bm_buffer, num: u8) -> i32;
    pub fn bman_acquire(pool: *mut bman_pool, bufs: *mut bm_buffer, num: u8) -> i32;
    pub fn bman_is_probed() -> i32;
    pub fn bman_portals_probed() -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
