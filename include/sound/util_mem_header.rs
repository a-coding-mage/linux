/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 *  Copyright (C) 2000 Takashi Iwai <tiwai@suse.de>
 *
 *  Generic memory management routines for soundcard memory allocation
 */

use core::ffi::c_void;

/*
 * memory block
 */
#[repr(C)]
pub struct snd_util_memblk {
	pub size: u32,                 /* size of this block */
	pub offset: u32,               /* zero-offset of this block */
	pub list: crate::list_head,    /* link */
}

pub unsafe fn snd_util_memblk_argptr(blk: *mut snd_util_memblk) -> *mut c_void {
	(blk as *mut u8).add(core::mem::size_of::<snd_util_memblk>()) as *mut c_void
}

/*
 * memory management information
 */
#[repr(C)]
pub struct snd_util_memhdr {
	pub size: u32,                 /* size of whole data */
	pub block: crate::list_head,   /* block linked-list header */
	pub nblocks: i32,              /* # of allocated blocks */
	pub used: u32,                 /* used memory size */
	pub block_extra_size: i32,     /* extra data size of chunk */
	pub block_mutex: crate::mutex, /* lock */
}

/*
 * prototypes
 */
extern "C" {
	pub fn snd_util_memhdr_new(memsize: i32) -> *mut snd_util_memhdr;
	pub fn snd_util_memhdr_free(hdr: *mut snd_util_memhdr);
	pub fn snd_util_mem_alloc(hdr: *mut snd_util_memhdr, size: i32)
		-> *mut snd_util_memblk;
	pub fn snd_util_mem_free(hdr: *mut snd_util_memhdr, blk: *mut snd_util_memblk) -> i32;
	pub fn snd_util_mem_avail(hdr: *mut snd_util_memhdr) -> i32;

	/* functions without mutex */
	pub fn __snd_util_mem_alloc(hdr: *mut snd_util_memhdr, size: i32)
		-> *mut snd_util_memblk;
	pub fn __snd_util_mem_free(hdr: *mut snd_util_memhdr, blk: *mut snd_util_memblk);
	pub fn __snd_util_memblk_new(
		hdr: *mut snd_util_memhdr,
		units: u32,
		prev: *mut crate::list_head,
	) -> *mut snd_util_memblk;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
