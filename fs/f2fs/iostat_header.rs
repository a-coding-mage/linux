/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright 2021 Google LLC
 * Author: Daeho Jeong <daehojeong@google.com>
 */

// Forward declaration supplied by another translation unit.
#[repr(C)]
pub struct bio_post_read_ctx {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum iostat_lat_type {
    READ_IO = 0,
    WRITE_SYNC_IO,
    WRITE_ASYNC_IO,
    MAX_IO_TYPE,
}

#[cfg(feature = "CONFIG_F2FS_IOSTAT")]
pub const NUM_PREALLOC_IOSTAT_CTXS: usize = 128;
#[cfg(feature = "CONFIG_F2FS_IOSTAT")]
pub const DEFAULT_IOSTAT_PERIOD_MS: usize = 3000;
#[cfg(feature = "CONFIG_F2FS_IOSTAT")]
pub const MIN_IOSTAT_PERIOD_MS: usize = 100;
/* maximum period of iostat tracing is 1 day */
#[cfg(feature = "CONFIG_F2FS_IOSTAT")]
pub const MAX_IOSTAT_PERIOD_MS: usize = 8640000;

#[cfg(feature = "CONFIG_F2FS_IOSTAT")]
#[repr(C)]
pub struct iostat_lat_info {
    /* sum of io latencies */
    pub sum_lat: [[::core::ffi::c_ulong; NR_PAGE_TYPE]; MAX_IO_TYPE as usize],
    /* peak io latency */
    pub peak_lat: [[::core::ffi::c_ulong; NR_PAGE_TYPE]; MAX_IO_TYPE as usize],
    /* bio count */
    pub bio_cnt: [[::core::ffi::c_uint; NR_PAGE_TYPE]; MAX_IO_TYPE as usize],
}

#[cfg(feature = "CONFIG_F2FS_IOSTAT")]
extern "C" {
    pub fn iostat_info_seq_show(seq: *mut seq_file, offset: *mut ::core::ffi::c_void) -> ::core::ffi::c_int;
    pub fn f2fs_reset_iostat(sbi: *mut f2fs_sb_info);
    pub fn f2fs_update_iostat(
        sbi: *mut f2fs_sb_info,
        inode: *mut inode,
        type_: iostat_type,
        io_bytes: ::core::ffi::c_ulonglong,
    );
    pub fn f2fs_update_read_folio_count(sbi: *mut f2fs_sb_info, folio: *mut folio);
}

#[cfg(feature = "CONFIG_F2FS_IOSTAT")]
#[repr(C)]
pub struct bio_iostat_ctx {
    pub sbi: *mut f2fs_sb_info,
    pub submit_ts: ::core::ffi::c_ulong,
    pub type_: page_type,
    pub post_read_ctx: *mut bio_post_read_ctx,
}

#[cfg(feature = "CONFIG_F2FS_IOSTAT")]
#[inline]
pub unsafe fn iostat_update_submit_ctx(bio: *mut bio, type_: page_type) {
    let iostat_ctx = (*bio).bi_private as *mut bio_iostat_ctx;
    (*iostat_ctx).submit_ts = jiffies;
    (*iostat_ctx).type_ = type_;
}

#[cfg(feature = "CONFIG_F2FS_IOSTAT")]
#[inline]
pub unsafe fn get_post_read_ctx(bio: *mut bio) -> *mut bio_post_read_ctx {
    let iostat_ctx = (*bio).bi_private as *mut bio_iostat_ctx;
    (*iostat_ctx).post_read_ctx
}

#[cfg(feature = "CONFIG_F2FS_IOSTAT")]
extern "C" {
    pub fn iostat_update_and_unbind_ctx(bio: *mut bio);
    pub fn iostat_alloc_and_bind_ctx(
        sbi: *mut f2fs_sb_info,
        bio: *mut bio,
        ctx: *mut bio_post_read_ctx,
    );
    pub fn f2fs_init_iostat_processing() -> ::core::ffi::c_int;
    pub fn f2fs_destroy_iostat_processing();
    pub fn f2fs_init_iostat(sbi: *mut f2fs_sb_info) -> ::core::ffi::c_int;
    pub fn f2fs_destroy_iostat(sbi: *mut f2fs_sb_info);
}

#[cfg(not(feature = "CONFIG_F2FS_IOSTAT"))]
#[inline]
pub unsafe fn f2fs_update_iostat(
    _sbi: *mut f2fs_sb_info,
    _inode: *mut inode,
    _type: iostat_type,
    _io_bytes: ::core::ffi::c_ulonglong,
) {
}

#[cfg(not(feature = "CONFIG_F2FS_IOSTAT"))]
#[inline]
pub unsafe fn f2fs_update_read_folio_count(_sbi: *mut f2fs_sb_info, _folio: *mut folio) {}

#[cfg(not(feature = "CONFIG_F2FS_IOSTAT"))]
#[inline]
pub unsafe fn iostat_update_and_unbind_ctx(_bio: *mut bio) {}

#[cfg(not(feature = "CONFIG_F2FS_IOSTAT"))]
#[inline]
pub unsafe fn iostat_alloc_and_bind_ctx(
    _sbi: *mut f2fs_sb_info,
    _bio: *mut bio,
    _ctx: *mut bio_post_read_ctx,
) {
}

#[cfg(not(feature = "CONFIG_F2FS_IOSTAT"))]
#[inline]
pub unsafe fn iostat_update_submit_ctx(_bio: *mut bio, _type: page_type) {}

#[cfg(not(feature = "CONFIG_F2FS_IOSTAT"))]
#[inline]
pub unsafe fn get_post_read_ctx(bio: *mut bio) -> *mut bio_post_read_ctx {
    (*bio).bi_private as *mut bio_post_read_ctx
}

#[cfg(not(feature = "CONFIG_F2FS_IOSTAT"))]
#[inline]
pub unsafe fn f2fs_init_iostat_processing() -> ::core::ffi::c_int {
    0
}

#[cfg(not(feature = "CONFIG_F2FS_IOSTAT"))]
#[inline]
pub unsafe fn f2fs_destroy_iostat_processing() {}

#[cfg(not(feature = "CONFIG_F2FS_IOSTAT"))]
#[inline]
pub unsafe fn f2fs_init_iostat(_sbi: *mut f2fs_sb_info) -> ::core::ffi::c_int {
    0
}

#[cfg(not(feature = "CONFIG_F2FS_IOSTAT"))]
#[inline]
pub unsafe fn f2fs_destroy_iostat(_sbi: *mut f2fs_sb_info) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
