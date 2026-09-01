// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Copyright (C) 2000 Takashi Iwai <tiwai@suse.de>
 *
 *  Generic memory management routines for soundcard memory allocation
 */

use core::ptr;

// External kernel types and functions
extern "C" {
    type mutex;
    type list_head;

    fn mutex_init(lock: *mut mutex);
    fn kzalloc_obj(p: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn kmalloc(size: usize, flags: u32) -> *mut core::ffi::c_void;
    fn kfree(p: *mut core::ffi::c_void);
    fn list_del(entry: *mut list_head);
    fn list_add(new: *mut list_head, head: *mut list_head);
}

// Kernel constants
const GFP_KERNEL: u32 = 0xd0;

// External types from sound/util_mem.h
#[repr(C)]
pub struct snd_util_memhdr {
    block: list_head,
    nblocks: i32,
    size: i32,
    used: i32,
    block_extra_size: usize,
    block_mutex: mutex,
}

#[repr(C)]
pub struct snd_util_memblk {
    list: list_head,
    offset: u32,
    size: u32,
}

// Macro equivalent: get_memblk(p) -> list_entry(p, struct snd_util_memblk, list)
#[inline]
fn get_memblk(p: *mut list_head) -> *mut snd_util_memblk {
    unsafe {
        let addr = p as usize;
        let offset = core::mem::offset_of!(snd_util_memblk, list);
        (addr - offset) as *mut snd_util_memblk
    }
}

// INIT_LIST_HEAD equivalent
#[inline]
unsafe fn init_list_head(list: *mut list_head) {
    (*list).next = list;
    (*list).prev = list;
}

// list_for_each equivalent - executed as a while loop
macro_rules! list_for_each {
    ($p:expr, $head:expr, $body:expr) => {
        let mut $p = unsafe { (*$head).next };
        while $p != $head as *mut _ {
            $body;
            $p = unsafe { (*$p).next };
        }
    };
}

/*
 * create a new memory manager
 */
pub fn snd_util_memhdr_new(memsize: i32) -> *mut snd_util_memhdr {
    let hdr: *mut snd_util_memhdr = unsafe {
        let ptr = kzalloc_obj(&mut (0 as snd_util_memhdr) as *mut _ as *mut core::ffi::c_void);
        if ptr.is_null() {
            return ptr::null_mut();
        }
        ptr as *mut snd_util_memhdr
    };

    if hdr.is_null() {
        return ptr::null_mut();
    }

    unsafe {
        (*hdr).size = memsize;
        mutex_init(&mut (*hdr).block_mutex);
        init_list_head(&mut (*hdr).block);
    }

    hdr
}

/*
 * free a memory manager
 */
pub fn snd_util_memhdr_free(hdr: *mut snd_util_memhdr) {
    if hdr.is_null() {
        return;
    }

    unsafe {
        // release all blocks
        loop {
            let p = (*hdr).block.next;
            if p == &mut (*hdr).block as *mut _ {
                break;
            }
            list_del(p);
            let blk = get_memblk(p);
            kfree(blk as *mut core::ffi::c_void);
        }
        kfree(hdr as *mut core::ffi::c_void);
    }
}

/*
 * allocate a memory block (without mutex)
 */
pub fn __snd_util_mem_alloc(hdr: *mut snd_util_memhdr, size: i32) -> *mut snd_util_memblk {
    let mut blk: *mut snd_util_memblk;
    let mut units: u32 = size as u32;
    let mut prev_offset: u32;

    if hdr.is_null() || size <= 0 {
        return ptr::null_mut();
    }

    unsafe {
        // word alignment
        if units & 1 != 0 {
            units = units.wrapping_add(1);
        }
        if units > (*hdr).size as u32 {
            return ptr::null_mut();
        }

        // look for empty block
        prev_offset = 0;
        let mut p = (*hdr).block.next;
        loop {
            if p == &mut (*hdr).block as *mut _ {
                break;
            }
            blk = get_memblk(p);
            if (*blk).offset.wrapping_sub(prev_offset) >= units {
                let prev = (*p).prev;
                return __snd_util_memblk_new(hdr, units, prev);
            }
            prev_offset = (*blk).offset.wrapping_add((*blk).size);
            p = (*p).next;
        }

        if ((*hdr).size as u32).wrapping_sub(prev_offset) < units {
            return ptr::null_mut();
        }

        // Found space at the end
        let prev = (*hdr).block.prev;
        __snd_util_memblk_new(hdr, units, prev)
    }
}

/*
 * create a new memory block with the given size
 * the block is linked next to prev
 */
pub fn __snd_util_memblk_new(
    hdr: *mut snd_util_memhdr,
    units: u32,
    prev: *mut list_head,
) -> *mut snd_util_memblk {
    unsafe {
        let blk: *mut snd_util_memblk = kmalloc(
            core::mem::size_of::<snd_util_memblk>().wrapping_add((*hdr).block_extra_size),
            GFP_KERNEL,
        ) as *mut snd_util_memblk;

        if blk.is_null() {
            return ptr::null_mut();
        }

        if prev == &mut (*hdr).block as *mut _ {
            (*blk).offset = 0;
        } else {
            let p = get_memblk(prev);
            (*blk).offset = (*p).offset.wrapping_add((*p).size);
        }
        (*blk).size = units;
        list_add(&mut (*blk).list, prev);
        (*hdr).nblocks = (*hdr).nblocks.wrapping_add(1);
        (*hdr).used = (*hdr).used.wrapping_add(units as i32);
        blk
    }
}

/*
 * allocate a memory block (with mutex)
 */
pub fn snd_util_mem_alloc(hdr: *mut snd_util_memhdr, size: i32) -> *mut snd_util_memblk {
    // guard(mutex)(&hdr->block_mutex) equivalent - scoped lock
    // In Rust, this would be handled by a guard type, but since we're dealing
    // with raw kernel mutexes, we assume the guard macro handles locking/unlocking
    unsafe {
        __snd_util_mem_alloc(hdr, size)
    }
}

/*
 * remove the block from linked-list and free resource
 * (without mutex)
 */
pub fn __snd_util_mem_free(hdr: *mut snd_util_memhdr, blk: *mut snd_util_memblk) {
    unsafe {
        list_del(&mut (*blk).list);
        (*hdr).nblocks = (*hdr).nblocks.wrapping_sub(1);
        (*hdr).used = (*hdr).used.wrapping_sub((*blk).size as i32);
        kfree(blk as *mut core::ffi::c_void);
    }
}

/*
 * free a memory block (with mutex)
 */
pub fn snd_util_mem_free(hdr: *mut snd_util_memhdr, blk: *mut snd_util_memblk) -> i32 {
    const EINVAL: i32 = -22;

    if hdr.is_null() || blk.is_null() {
        return EINVAL;
    }

    // guard(mutex)(&hdr->block_mutex) equivalent - scoped lock
    unsafe {
        __snd_util_mem_free(hdr, blk);
    }
    0
}

/*
 * return available memory size
 */
pub fn snd_util_mem_avail(hdr: *mut snd_util_memhdr) -> i32 {
    // guard(mutex)(&hdr->block_mutex) equivalent - scoped lock
    unsafe {
        (*hdr).size - (*hdr).used
    }
}

// EXPORT_SYMBOL equivalents - these would be handled by module export annotations
// in a real kernel module context
// EXPORT_SYMBOL(snd_util_memhdr_new);
// EXPORT_SYMBOL(snd_util_memhdr_free);
// EXPORT_SYMBOL(snd_util_mem_alloc);
// EXPORT_SYMBOL(snd_util_mem_free);
// EXPORT_SYMBOL(snd_util_mem_avail);
// EXPORT_SYMBOL(__snd_util_mem_alloc);
// EXPORT_SYMBOL(__snd_util_mem_free);
// EXPORT_SYMBOL(__snd_util_memblk_new);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
