/* SPDX-License-Identifier: GPL-2.0+ */
/*
 * NILFS Segment buffer prototypes and definitions
 *
 * Translated from segbuf.h.
 */

/* Linux kernel dependencies are supplied by other translated units. */

#[repr(C)]
pub struct nilfs_segsum_info {
    pub flags: ::core::ffi::c_uint,
    pub nfinfo: ::core::ffi::c_ulong,
    pub nblocks: ::core::ffi::c_ulong,
    pub nsumblk: ::core::ffi::c_ulong,
    pub sumbytes: ::core::ffi::c_ulong,
    pub nfileblk: ::core::ffi::c_ulong,
    pub seg_seq: u64,
    pub cno: u64,
    pub ctime: time64_t,
    pub next: sector_t,
}

#[repr(C)]
pub struct nilfs_segment_buffer {
    pub sb_super: *mut super_block,
    pub sb_list: list_head,
    pub sb_sum: nilfs_segsum_info,
    pub sb_segnum: u64,
    pub sb_nextnum: u64,
    pub sb_fseg_start: sector_t,
    pub sb_fseg_end: sector_t,
    pub sb_pseg_start: sector_t,
    pub sb_rest_blocks: ::core::ffi::c_uint,
    pub sb_segsum_buffers: list_head,
    pub sb_payload_buffers: list_head,
    pub sb_super_root: *mut buffer_head,
    pub sb_nbio: ::core::ffi::c_int,
    pub sb_err: atomic_t,
    pub sb_bio_event: completion,
}

#[macro_export]
macro_rules! NILFS_LIST_SEGBUF {
    ($head:expr) => { list_entry!($head, nilfs_segment_buffer, sb_list) };
}
#[macro_export]
macro_rules! NILFS_NEXT_SEGBUF {
    ($segbuf:expr) => { NILFS_LIST_SEGBUF!((*$segbuf).sb_list.next) };
}
#[macro_export]
macro_rules! NILFS_PREV_SEGBUF {
    ($segbuf:expr) => { NILFS_LIST_SEGBUF!((*$segbuf).sb_list.prev) };
}
#[macro_export]
macro_rules! NILFS_LAST_SEGBUF {
    ($head:expr) => { NILFS_LIST_SEGBUF!((*$head).prev) };
}
#[macro_export]
macro_rules! NILFS_FIRST_SEGBUF {
    ($head:expr) => { NILFS_LIST_SEGBUF!((*$head).next) };
}
#[macro_export]
macro_rules! NILFS_SEGBUF_IS_LAST {
    ($segbuf:expr, $head:expr) => { (*$segbuf).sb_list.next == $head };
}

#[macro_export]
macro_rules! nilfs_for_each_segbuf_before {
    ($s:ident, $t:expr, $h:expr) => {
        for $s in std::iter::successors(Some(NILFS_FIRST_SEGBUF!($h)), |p| {
            if *p != $t { Some(NILFS_NEXT_SEGBUF!(*p)) } else { None }
        }) {
            if $s == $t { break; }
        }
    };
}

#[macro_export]
macro_rules! NILFS_SEGBUF_FIRST_BH {
    ($head:expr) => { list_entry!((*$head).next, buffer_head, b_assoc_buffers) };
}
#[macro_export]
macro_rules! NILFS_SEGBUF_NEXT_BH {
    ($bh:expr) => { list_entry!((*$bh).b_assoc_buffers.next, buffer_head, b_assoc_buffers) };
}
#[macro_export]
macro_rules! NILFS_SEGBUF_BH_IS_LAST {
    ($bh:expr, $head:expr) => { (*$bh).b_assoc_buffers.next == $head };
}

extern "C" {
    pub static mut nilfs_segbuf_cachep: *mut kmem_cache;
    pub fn nilfs_segbuf_new(sb: *mut super_block) -> *mut nilfs_segment_buffer;
    pub fn nilfs_segbuf_free(segbuf: *mut nilfs_segment_buffer);
    pub fn nilfs_segbuf_map(segbuf: *mut nilfs_segment_buffer, segnum: u64, start: ::core::ffi::c_ulong, nilfs: *mut the_nilfs);
    pub fn nilfs_segbuf_map_cont(segbuf: *mut nilfs_segment_buffer, prev: *mut nilfs_segment_buffer);
    pub fn nilfs_segbuf_set_next_segnum(segbuf: *mut nilfs_segment_buffer, segnum: u64, nilfs: *mut the_nilfs);
    pub fn nilfs_segbuf_reset(segbuf: *mut nilfs_segment_buffer, flags: ::core::ffi::c_uint, ctime: time64_t, cno: u64) -> ::core::ffi::c_int;
    pub fn nilfs_segbuf_extend_segsum(segbuf: *mut nilfs_segment_buffer) -> ::core::ffi::c_int;
    pub fn nilfs_segbuf_extend_payload(segbuf: *mut nilfs_segment_buffer, bh: *mut *mut buffer_head) -> ::core::ffi::c_int;
    pub fn nilfs_segbuf_fill_in_segsum(segbuf: *mut nilfs_segment_buffer);
    pub fn nilfs_clear_logs(logs: *mut list_head);
    pub fn nilfs_truncate_logs(logs: *mut list_head, last: *mut nilfs_segment_buffer);
    pub fn nilfs_write_logs(logs: *mut list_head, nilfs: *mut the_nilfs) -> ::core::ffi::c_int;
    pub fn nilfs_wait_on_logs(logs: *mut list_head) -> ::core::ffi::c_int;
    pub fn nilfs_add_checksums_on_logs(logs: *mut list_head, seed: u32);
}

#[inline]
pub unsafe fn nilfs_segbuf_simplex(segbuf: *mut nilfs_segment_buffer) -> ::core::ffi::c_int {
    let flags = (*segbuf).sb_sum.flags;
    if (flags & (NILFS_SS_LOGBGN | NILFS_SS_LOGEND)) == (NILFS_SS_LOGBGN | NILFS_SS_LOGEND) { 1 } else { 0 }
}

#[inline]
pub unsafe fn nilfs_segbuf_empty(segbuf: *mut nilfs_segment_buffer) -> ::core::ffi::c_int {
    if (*segbuf).sb_sum.nblocks == (*segbuf).sb_sum.nsumblk { 1 } else { 0 }
}

#[inline]
pub unsafe fn nilfs_segbuf_add_segsum_buffer(segbuf: *mut nilfs_segment_buffer, bh: *mut buffer_head) {
    list_add_tail(&mut (*bh).b_assoc_buffers, &mut (*segbuf).sb_segsum_buffers);
    (*segbuf).sb_sum.nblocks += 1;
    (*segbuf).sb_sum.nsumblk += 1;
}

#[inline]
pub unsafe fn nilfs_segbuf_add_payload_buffer(segbuf: *mut nilfs_segment_buffer, bh: *mut buffer_head) {
    list_add_tail(&mut (*bh).b_assoc_buffers, &mut (*segbuf).sb_payload_buffers);
    (*segbuf).sb_sum.nblocks += 1;
}

#[inline]
pub unsafe fn nilfs_segbuf_add_file_buffer(segbuf: *mut nilfs_segment_buffer, bh: *mut buffer_head) {
    get_bh(bh);
    nilfs_segbuf_add_payload_buffer(segbuf, bh);
    (*segbuf).sb_sum.nfileblk += 1;
}

#[inline]
pub unsafe fn nilfs_destroy_logs(logs: *mut list_head) {
    nilfs_truncate_logs(logs, core::ptr::null_mut());
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
