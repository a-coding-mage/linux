/* SPDX-License-Identifier: GPL-2.0 */

// Translated from printk_ringbuffer.h. Linux-provided types and functions are
// intentionally left as external dependencies.

#[repr(C)]
pub struct printk_info {
    pub seq: u64,
    pub ts_nsec: u64,
    pub text_len: u16,
    pub facility: u8,
    pub flags: u8,
    pub level: u8,
    pub caller_id: u32,
    // CONFIG_PRINTK_EXECUTION_CTX: caller_id2, comm[TASK_COMM_LEN]
    pub dev_info: dev_printk_info,
}

#[repr(C)]
pub struct printk_record {
    pub info: *mut printk_info,
    pub text_buf: *mut core::ffi::c_char,
    pub text_buf_size: u32,
}

#[repr(C)]
pub struct prb_data_blk_lpos {
    pub begin: usize,
    pub next: usize,
}

#[repr(C)]
pub struct prb_desc {
    pub state_var: atomic_long_t,
    pub text_blk_lpos: prb_data_blk_lpos,
}

#[repr(C)]
pub struct prb_data_ring {
    pub size_bits: u32,
    pub data: *mut core::ffi::c_char,
    pub head_lpos: atomic_long_t,
    pub tail_lpos: atomic_long_t,
}

#[repr(C)]
pub struct prb_desc_ring {
    pub count_bits: u32,
    pub descs: *mut prb_desc,
    pub infos: *mut printk_info,
    pub head_id: atomic_long_t,
    pub tail_id: atomic_long_t,
    pub last_finalized_seq: atomic_long_t,
}

#[repr(C)]
pub struct printk_ringbuffer {
    pub desc_ring: prb_desc_ring,
    pub text_data_ring: prb_data_ring,
    pub fail: atomic_long_t,
}

#[repr(C)]
pub struct prb_reserved_entry {
    pub rb: *mut printk_ringbuffer,
    pub irqflags: usize,
    pub id: usize,
    pub text_space: u32,
}

#[repr(i32)]
pub enum desc_state {
    desc_miss = -1,
    desc_reserved = 0x0,
    desc_committed = 0x1,
    desc_finalized = 0x2,
    desc_reusable = 0x3,
}

pub const fn _data_size(sz_bits: u32) -> usize { 1usize << sz_bits }
pub const fn _descs_count(ct_bits: u32) -> usize { 1usize << ct_bits }
pub const DESC_SV_BITS: u32 = usize::BITS;
pub const DESC_FLAGS_SHIFT: u32 = DESC_SV_BITS - 2;
pub const DESC_FLAGS_MASK: usize = 3usize << DESC_FLAGS_SHIFT;
pub const fn desc_state_value(sv: usize) -> usize { 3usize & (sv >> DESC_FLAGS_SHIFT) }
pub const fn desc_sv(id: usize, state: usize) -> usize { (state << DESC_FLAGS_SHIFT) | id }
pub const DESC_ID_MASK: usize = !DESC_FLAGS_MASK;
pub const fn desc_id(sv: usize) -> usize { sv & DESC_ID_MASK }

pub const FAILED_LPOS: usize = 0x1;
pub const EMPTY_LINE_LPOS: usize = 0x3;
pub const FAILED_BLK_LPOS: prb_data_blk_lpos = prb_data_blk_lpos {
    begin: FAILED_LPOS,
    next: FAILED_LPOS,
};

pub const fn blk0_lpos(sz_bits: u32) -> usize { 0usize.wrapping_sub(_data_size(sz_bits)) }
pub const fn desc0_id(ct_bits: u32) -> usize {
    desc_id(0usize.wrapping_sub(_descs_count(ct_bits) + 1))
}
pub const fn desc0_sv(ct_bits: u32) -> usize { desc_sv(desc0_id(ct_bits), desc_reusable as usize) }

pub unsafe fn prb_rec_init_wr(r: *mut printk_record, text_buf_size: u32) {
    (*r).info = core::ptr::null_mut();
    (*r).text_buf = core::ptr::null_mut();
    (*r).text_buf_size = text_buf_size;
}

unsafe extern "C" {
    pub fn prb_reserve(e: *mut prb_reserved_entry, rb: *mut printk_ringbuffer,
                       r: *mut printk_record) -> bool;
    pub fn prb_reserve_in_last(e: *mut prb_reserved_entry, rb: *mut printk_ringbuffer,
                               r: *mut printk_record, caller_id: u32, max_size: u32) -> bool;
    pub fn prb_commit(e: *mut prb_reserved_entry);
    pub fn prb_final_commit(e: *mut prb_reserved_entry);
    pub fn prb_init(rb: *mut printk_ringbuffer, text_buf: *mut core::ffi::c_char,
                    text_buf_size: u32, descs: *mut prb_desc, descs_count_bits: u32,
                    infos: *mut printk_info);
    pub fn prb_record_text_space(e: *mut prb_reserved_entry) -> u32;
}

pub unsafe fn prb_rec_init_rd(r: *mut printk_record, info: *mut printk_info,
                              text_buf: *mut core::ffi::c_char, text_buf_size: u32) {
    (*r).info = info;
    (*r).text_buf = text_buf;
    (*r).text_buf_size = text_buf_size;
}

unsafe extern "C" {
    pub fn prb_read_valid(rb: *mut printk_ringbuffer, seq: u64, r: *mut printk_record) -> bool;
    pub fn prb_read_valid_info(rb: *mut printk_ringbuffer, seq: u64,
                               info: *mut printk_info, line_count: *mut u32) -> bool;
    pub fn prb_first_seq(rb: *mut printk_ringbuffer) -> u64;
    pub fn prb_first_valid_seq(rb: *mut printk_ringbuffer) -> u64;
    pub fn prb_next_seq(rb: *mut printk_ringbuffer) -> u64;
    pub fn prb_next_reserve_seq(rb: *mut printk_ringbuffer) -> u64;
}

// prb_for_each_record and prb_for_each_info are C iteration macros; callers
// should use equivalent Rust loops around prb_read_valid/prb_read_valid_info.

// CONFIG_64BIT selects the identity conversions. On 32-bit builds, the
// following helpers preserve the Linux sequence-number folding behavior.
#[cfg(target_pointer_width = "64")]
pub const fn __u64seq_to_ulseq(u64seq: u64) -> usize { u64seq as usize }

#[cfg(target_pointer_width = "64")]
pub unsafe fn __ulseq_to_u64seq(_rb: *mut printk_ringbuffer, ulseq: usize) -> u64 { ulseq as u64 }

#[cfg(target_pointer_width = "64")]
pub const fn ulseq_max(_rb: *mut printk_ringbuffer) -> usize { usize::MAX }

#[cfg(target_pointer_width = "32")]
pub const fn __u64seq_to_ulseq(u64seq: u64) -> u32 { u64seq as u32 }

#[cfg(target_pointer_width = "32")]
pub const fn ulseq_max(rb: *mut printk_ringbuffer) -> u32 {
    (__u64seq_to_ulseq(unsafe { prb_first_seq(rb) }.wrapping_add(0x8000_0000u64)))
}

#[cfg(target_pointer_width = "32")]
pub unsafe fn __ulseq_to_u64seq(rb: *mut printk_ringbuffer, ulseq: u32) -> u64 {
    let rb_first_seq = prb_first_seq(rb);
    rb_first_seq.wrapping_sub((rb_first_seq as u32).wrapping_sub(ulseq) as i32 as i64 as u64)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
