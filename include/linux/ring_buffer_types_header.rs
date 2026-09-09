/* SPDX-License-Identifier: GPL-2.0 */

// Translated from linux/ring_buffer_types.h.
// Dependency: asm/local.h supplies `local_t`.

pub const TS_SHIFT: u32 = 27;
pub const TS_MASK: u64 = (1u64 << TS_SHIFT) - 1;
pub const TS_DELTA_TEST: u64 = !TS_MASK;

/*
 * We need to fit the time_stamp delta into 27 bits.
 */
#[inline]
pub fn test_time_stamp(delta: u64) -> bool {
	(delta & TS_DELTA_TEST) != 0
}

// `offsetof(struct buffer_data_page, data)`.
pub const BUF_PAGE_HDR_SIZE: usize = core::mem::offset_of!(buffer_data_page, data);

// `offsetof(struct ring_buffer_event, array)`.
pub const RB_EVNT_HDR_SIZE: usize = core::mem::offset_of!(ring_buffer_event, array);
pub const RB_ALIGNMENT: u32 = 4;
pub const RB_MAX_SMALL_DATA: u32 = RB_ALIGNMENT * RINGBUF_TYPE_DATA_TYPE_LEN_MAX;
pub const RB_EVNT_MIN_SIZE: u32 = 8; // two 32bit words

// The build-time CONFIG_HAVE_64BIT_ALIGNED_ACCESS condition is preserved here.
#[cfg(not(CONFIG_HAVE_64BIT_ALIGNED_ACCESS))]
pub const RB_FORCE_8BYTE_ALIGNMENT: u32 = 0;
#[cfg(not(CONFIG_HAVE_64BIT_ALIGNED_ACCESS))]
pub const RB_ARCH_ALIGNMENT: u32 = RB_ALIGNMENT;

#[cfg(CONFIG_HAVE_64BIT_ALIGNED_ACCESS)]
pub const RB_FORCE_8BYTE_ALIGNMENT: u32 = 1;
#[cfg(CONFIG_HAVE_64BIT_ALIGNED_ACCESS)]
pub const RB_ARCH_ALIGNMENT: u32 = 8;

// C's `__aligned(RB_ARCH_ALIGNMENT)` is represented by the enclosing C layout
// and the alignment of the flexible-array field; Rust has no field alignment
// attribute, so the zero-length array retains the field and its ordering.
#[repr(C)]
pub struct buffer_data_page {
	pub time_stamp: u64, // page time stamp
	pub commit: local_t, // write committed index
	pub data: [u8; 0], // data of buffer page
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
