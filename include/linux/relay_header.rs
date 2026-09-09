/* SPDX-License-Identifier: GPL-2.0 */
/*
 * linux/include/linux/relay.h
 *
 * CONFIG_RELAY definitions and declarations
 *
 * C includes are intentionally omitted; their supplied kernel types and
 * helpers are external dependencies of this translation.
 */

pub const RELAYFS_CHANNEL_VERSION: u32 = 7;

pub const RELAY_STATS_BUF_FULL: u32 = 1 << 0;
pub const RELAY_STATS_WRT_BIG: u32 = 1 << 1;
pub const RELAY_STATS_LAST: u32 = RELAY_STATS_WRT_BIG;

#[repr(C)]
pub struct rchan_buf_stats {
    pub full_count: ::core::ffi::c_uint,
    pub big_count: ::core::ffi::c_uint,
}

#[repr(C)]
pub struct rchan_buf {
    pub start: *mut ::core::ffi::c_void,
    pub data: *mut ::core::ffi::c_void,
    pub offset: usize,
    pub subbufs_produced: usize,
    pub subbufs_consumed: usize,
    pub chan: *mut rchan,
    pub read_wait: wait_queue_head_t,
    pub wakeup_work: irq_work,
    pub dentry: *mut dentry,
    pub kref: kref,
    pub stats: rchan_buf_stats,
    pub page_array: *mut *mut page,
    pub page_count: ::core::ffi::c_uint,
    pub finalized: ::core::ffi::c_uint,
    pub padding: *mut usize,
    pub bytes_consumed: usize,
    pub early_bytes: usize,
    pub cpu: ::core::ffi::c_uint,
}

#[repr(C)]
pub struct rchan {
    pub version: u32,
    pub subbuf_size: usize,
    pub n_subbufs: usize,
    pub alloc_size: usize,
    pub cb: *const rchan_callbacks,
    pub kref: kref,
    pub private_data: *mut ::core::ffi::c_void,
    pub buf: *mut *mut rchan_buf,
    pub is_global: ::core::ffi::c_int,
    pub list: list_head,
    pub parent: *mut dentry,
    pub has_base_filename: ::core::ffi::c_int,
    pub base_filename: [::core::ffi::c_char; NAME_MAX],
}

#[repr(C)]
pub struct rchan_callbacks {
    pub subbuf_start: Option<unsafe extern "C" fn(
        buf: *mut rchan_buf,
        subbuf: *mut ::core::ffi::c_void,
        prev_subbuf: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int>,
    pub create_buf_file: Option<unsafe extern "C" fn(
        filename: *const ::core::ffi::c_char,
        parent: *mut dentry,
        mode: umode_t,
        buf: *mut rchan_buf,
        is_global: *mut ::core::ffi::c_int,
    ) -> *mut dentry>,
    pub remove_buf_file:
        Option<unsafe extern "C" fn(dentry: *mut dentry) -> ::core::ffi::c_int>,
}

extern "C" {
    pub fn relay_open(
        base_filename: *const ::core::ffi::c_char,
        parent: *mut dentry,
        subbuf_size: usize,
        n_subbufs: usize,
        cb: *const rchan_callbacks,
        private_data: *mut ::core::ffi::c_void,
    ) -> *mut rchan;
    pub fn relay_close(chan: *mut rchan);
    pub fn relay_flush(chan: *mut rchan);
    pub fn relay_stats(chan: *mut rchan, flags: ::core::ffi::c_int) -> usize;
    pub fn relay_subbufs_consumed(chan: *mut rchan, cpu: ::core::ffi::c_uint, consumed: usize);
    pub fn relay_reset(chan: *mut rchan);
    pub fn relay_buf_full(buf: *mut rchan_buf) -> ::core::ffi::c_int;
    pub fn relay_switch_subbuf(buf: *mut rchan_buf, length: usize) -> usize;
}

/* The following helpers/macros are supplied by the kernel environment. */
extern "C" {
    fn local_irq_save(flags: *mut ::core::ffi::c_ulong);
    fn local_irq_restore(flags: ::core::ffi::c_ulong);
    fn this_cpu_ptr(ptr: *mut *mut rchan_buf) -> *mut *mut rchan_buf;
    fn get_cpu_ptr(ptr: *mut *mut rchan_buf) -> *mut *mut rchan_buf;
    fn put_cpu_ptr(ptr: *mut *mut rchan_buf);
    fn memcpy(dst: *mut ::core::ffi::c_void, src: *const ::core::ffi::c_void, n: usize);
}

#[inline]
pub unsafe fn relay_write(chan: *mut rchan, data: *const ::core::ffi::c_void, mut length: usize) {
    let mut flags: ::core::ffi::c_ulong = 0;
    local_irq_save(&mut flags);
    let buf = *this_cpu_ptr((*chan).buf);
    if (*buf).offset.wrapping_add(length) > (*chan).subbuf_size {
        length = relay_switch_subbuf(buf, length);
    }
    memcpy((*buf).data.cast::<u8>().add((*buf).offset).cast(), data, length);
    (*buf).offset = (*buf).offset.wrapping_add(length);
    local_irq_restore(flags);
}

#[inline]
pub unsafe fn __relay_write(
    chan: *mut rchan,
    data: *const ::core::ffi::c_void,
    mut length: usize,
) {
    let buf = *get_cpu_ptr((*chan).buf);
    if (*buf).offset.wrapping_add(length) > (*buf).chan.as_ref().unwrap().subbuf_size {
        length = relay_switch_subbuf(buf, length);
    }
    memcpy((*buf).data.cast::<u8>().add((*buf).offset).cast(), data, length);
    (*buf).offset = (*buf).offset.wrapping_add(length);
    put_cpu_ptr((*chan).buf);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
