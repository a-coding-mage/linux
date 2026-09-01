// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Copyright (c) by Jaroslav Kysela <perex@perex.cz>
 *  Copyright (c) by Takashi Iwai <tiwai@suse.de>
 *  Copyright (c) by Scott McNab <sdm@fractalgraphics.com.au>
 *
 *  Trident 4DWave-NX memory page allocation (TLB area)
 *  Trident chip can handle only 16MByte of the memory at the same time.
 */

// C dependencies: <linux/io.h>, <linux/pci.h>, <linux/time.h>,
// <linux/mutex.h>, <sound/core.h>, "trident.h"

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_int, c_ulong, c_void};
use core::ptr;

type dma_addr_t = c_ulong;
const NULL: *mut c_void = ptr::null_mut();
const EINVAL: c_int = 22;

extern "C" {
    static PAGE_SIZE: c_ulong;
    static PAGE_SHIFT: c_int;
    static SNDRV_TRIDENT_PAGE_SIZE: c_ulong;
    static SNDRV_TRIDENT_MAX_PAGES: c_int;
    static SNDRV_DMA_TYPE_DEV_SG: c_int;

    fn cpu_to_le32(x: u32) -> u32;
    fn le32_to_cpu(x: u32) -> u32;
    fn dev_err(dev: *mut c_void, fmt: *const i8, ...);
    fn snd_BUG_ON(cond: bool) -> c_int;
    fn snd_util_memblk_argptr(blk: *mut snd_util_memblk) -> *mut c_void;
    fn __snd_util_memblk_new(
        hdr: *mut snd_util_memhdr,
        size: c_ulong,
        prev: *mut list_head,
    ) -> *mut snd_util_memblk;
    fn __snd_util_mem_free(hdr: *mut snd_util_memhdr, blk: *mut snd_util_memblk);
    fn snd_pcm_sgbuf_get_addr(
        substream: *mut snd_pcm_substream,
        ofs: c_ulong,
    ) -> dma_addr_t;
    fn mutex_lock(mutex: *mut mutex);
    fn mutex_unlock(mutex: *mut mutex);
}

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_util_memhdr {
    pub block: list_head,
    pub block_mutex: mutex,
}

#[repr(C)]
pub struct snd_util_memblk {
    pub list: list_head,
    pub offset: c_ulong,
}

#[repr(C)]
pub struct snd_trident_memblk_arg {
    pub first_page: c_int,
    pub last_page: c_int,
}

#[repr(C)]
pub struct snd_dma_buffer_dev {
    pub type_: c_int,
}

#[repr(C)]
pub struct snd_dma_buffer {
    pub dev: snd_dma_buffer_dev,
}

#[repr(C)]
pub struct snd_pcm_runtime {
    pub dma_bytes: c_ulong,
    pub dma_addr: dma_addr_t,
}

#[repr(C)]
pub struct snd_pcm_substream {
    pub runtime: *mut snd_pcm_runtime,
    pub dma_buffer: snd_dma_buffer,
}

#[repr(C)]
pub struct snd_card {
    pub dev: *mut c_void,
}

#[repr(C)]
pub struct snd_trident_tlb_silent_page {
    pub addr: dma_addr_t,
}

#[repr(C)]
pub struct snd_trident_tlb {
    pub entries: *mut u32,
    pub silent_page: *mut snd_trident_tlb_silent_page,
    pub memhdr: *mut snd_util_memhdr,
}

#[repr(C)]
pub struct snd_trident {
    pub tlb: snd_trident_tlb,
    pub card: *mut snd_card,
}

unsafe fn list_entry_snd_util_memblk(ptr: *mut list_head) -> *mut snd_util_memblk {
    let offset = core::mem::offset_of!(snd_util_memblk, list);
    (ptr as *mut u8).sub(offset) as *mut snd_util_memblk
}

unsafe fn __set_tlb_bus(trident: *mut snd_trident, page: c_int, addr: dma_addr_t) {
    let mask = !(SNDRV_TRIDENT_PAGE_SIZE - 1) as dma_addr_t;
    *(*trident).tlb.entries.add(page as usize) = cpu_to_le32((addr & mask) as u32);
}

unsafe fn __tlb_to_addr(trident: *mut snd_trident, page: c_int) -> dma_addr_t {
    let mask = !(SNDRV_TRIDENT_PAGE_SIZE - 1) as u32;
    le32_to_cpu(*(*trident).tlb.entries.add(page as usize) & mask) as dma_addr_t
}

unsafe fn unit_pages() -> c_int {
    (PAGE_SIZE / SNDRV_TRIDENT_PAGE_SIZE) as c_int
}

unsafe fn align_page_size() -> c_ulong {
    if PAGE_SIZE == 4096 {
        PAGE_SIZE
    } else if PAGE_SIZE == 8192 {
        PAGE_SIZE
    } else {
        SNDRV_TRIDENT_PAGE_SIZE * unit_pages() as c_ulong
    }
}

unsafe fn max_align_pages() -> c_int {
    if PAGE_SIZE == 4096 {
        SNDRV_TRIDENT_MAX_PAGES
    } else if PAGE_SIZE == 8192 {
        SNDRV_TRIDENT_MAX_PAGES / 2
    } else {
        SNDRV_TRIDENT_MAX_PAGES / unit_pages()
    }
}

unsafe fn get_aligned_page(offset: c_ulong) -> c_int {
    if PAGE_SIZE == 4096 {
        (offset >> 12) as c_int
    } else if PAGE_SIZE == 8192 {
        (offset >> 13) as c_int
    } else {
        (offset / align_page_size()) as c_int
    }
}

unsafe fn aligned_page_offset(page: c_int) -> c_ulong {
    if PAGE_SIZE == 4096 {
        (page as c_ulong) << 12
    } else if PAGE_SIZE == 8192 {
        (page as c_ulong) << 13
    } else {
        (page as c_ulong) * align_page_size()
    }
}

unsafe fn page_to_addr(trident: *mut snd_trident, page: c_int) -> dma_addr_t {
    if PAGE_SIZE == 4096 {
        __tlb_to_addr(trident, page)
    } else if PAGE_SIZE == 8192 {
        __tlb_to_addr(trident, page << 1)
    } else {
        __tlb_to_addr(trident, page * unit_pages())
    }
}

unsafe fn set_tlb_bus(trident: *mut snd_trident, mut page: c_int, mut addr: dma_addr_t) {
    if PAGE_SIZE == 4096 {
        __set_tlb_bus(trident, page, addr);
    } else if PAGE_SIZE == 8192 {
        page <<= 1;
        __set_tlb_bus(trident, page, addr);
        __set_tlb_bus(trident, page + 1, addr + SNDRV_TRIDENT_PAGE_SIZE as dma_addr_t);
    } else {
        let mut i: c_int;
        page *= unit_pages();
        i = 0;
        while i < unit_pages() {
            __set_tlb_bus(trident, page, addr);
            addr += SNDRV_TRIDENT_PAGE_SIZE as dma_addr_t;
            i += 1;
            page += 1;
        }
    }
}

unsafe fn set_silent_tlb(trident: *mut snd_trident, mut page: c_int) {
    if PAGE_SIZE == 4096 {
        __set_tlb_bus(trident, page, (*(*trident).tlb.silent_page).addr);
    } else if PAGE_SIZE == 8192 {
        page <<= 1;
        __set_tlb_bus(trident, page, (*(*trident).tlb.silent_page).addr);
        __set_tlb_bus(trident, page + 1, (*(*trident).tlb.silent_page).addr);
    } else {
        let mut i: c_int;
        page *= unit_pages();
        i = 0;
        while i < unit_pages() {
            __set_tlb_bus(trident, page, (*(*trident).tlb.silent_page).addr);
            i += 1;
            page += 1;
        }
    }
}

unsafe fn firstpg(blk: *mut snd_util_memblk) -> *mut c_int {
    &mut (*(snd_util_memblk_argptr(blk) as *mut snd_trident_memblk_arg)).first_page
}

unsafe fn lastpg(blk: *mut snd_util_memblk) -> *mut c_int {
    &mut (*(snd_util_memblk_argptr(blk) as *mut snd_trident_memblk_arg)).last_page
}

/*
 * search empty pages which may contain given size
 */
unsafe fn search_empty(hdr: *mut snd_util_memhdr, size: c_int) -> *mut snd_util_memblk {
    let mut blk: *mut snd_util_memblk;
    let mut page: c_int;
    let psize: c_int;
    let mut p: *mut list_head;

    psize = get_aligned_page(size as c_ulong + align_page_size() - 1);
    page = 0;
    p = (*hdr).block.next;
    while p != &mut (*hdr).block as *mut list_head {
        blk = list_entry_snd_util_memblk(p);
        if page + psize <= *firstpg(blk) {
            goto_found_pages(hdr, p, page, psize)
        } else {
            page = *lastpg(blk) + 1;
            p = (*p).next;
        }
    }
    if page + psize > max_align_pages() {
        return ptr::null_mut();
    }

    goto_found_pages(hdr, p, page, psize)
}

unsafe fn goto_found_pages(
    hdr: *mut snd_util_memhdr,
    p: *mut list_head,
    page: c_int,
    psize: c_int,
) -> *mut snd_util_memblk {
    /* create a new memory block */
    let blk = __snd_util_memblk_new(
        hdr,
        psize as c_ulong * align_page_size(),
        (*p).prev,
    );
    if blk.is_null() {
        return ptr::null_mut();
    }
    (*blk).offset = aligned_page_offset(page); /* set aligned offset */
    *firstpg(blk) = page;
    *lastpg(blk) = page + psize - 1;
    blk
}

/*
 * check if the given pointer is valid for pages
 */
unsafe fn is_valid_page(trident: *mut snd_trident, ptr_: c_ulong) -> c_int {
    if ptr_ & !(0x3fffffff as c_ulong) != 0 {
        dev_err(
            (*(*trident).card).dev,
            b"max memory size is 1GB!!\n\0".as_ptr() as *const i8,
        );
        return 0;
    }
    if ptr_ & (SNDRV_TRIDENT_PAGE_SIZE - 1) != 0 {
        dev_err(
            (*(*trident).card).dev,
            b"page is not aligned\n\0".as_ptr() as *const i8,
        );
        return 0;
    }
    1
}

/*
 * page allocation for DMA (Scatter-Gather version)
 */
unsafe fn snd_trident_alloc_sg_pages(
    trident: *mut snd_trident,
    substream: *mut snd_pcm_substream,
) -> *mut snd_util_memblk {
    let hdr: *mut snd_util_memhdr;
    let blk: *mut snd_util_memblk;
    let runtime: *mut snd_pcm_runtime = (*substream).runtime;
    let mut idx: c_int;
    let mut page: c_int;

    if snd_BUG_ON(
        (*runtime).dma_bytes <= 0
            || (*runtime).dma_bytes
                > SNDRV_TRIDENT_MAX_PAGES as c_ulong * SNDRV_TRIDENT_PAGE_SIZE,
    ) != 0
    {
        return ptr::null_mut();
    }
    hdr = (*trident).tlb.memhdr;
    if snd_BUG_ON(hdr.is_null()) != 0 {
        return ptr::null_mut();
    }

    mutex_lock(&mut (*hdr).block_mutex);
    blk = search_empty(hdr, (*runtime).dma_bytes as c_int);
    if blk.is_null() {
        mutex_unlock(&mut (*hdr).block_mutex);
        return ptr::null_mut();
    }

    /* set TLB entries */
    idx = 0;
    page = *firstpg(blk);
    while page <= *lastpg(blk) {
        let ofs: c_ulong = (idx as c_ulong) << PAGE_SHIFT;
        let addr: dma_addr_t = snd_pcm_sgbuf_get_addr(substream, ofs);
        if is_valid_page(trident, addr) == 0 {
            __snd_util_mem_free(hdr, blk);
            mutex_unlock(&mut (*hdr).block_mutex);
            return ptr::null_mut();
        }
        set_tlb_bus(trident, page, addr);
        page += 1;
        idx += 1;
    }
    mutex_unlock(&mut (*hdr).block_mutex);
    blk
}

/*
 * page allocation for DMA (contiguous version)
 */
unsafe fn snd_trident_alloc_cont_pages(
    trident: *mut snd_trident,
    substream: *mut snd_pcm_substream,
) -> *mut snd_util_memblk {
    let hdr: *mut snd_util_memhdr;
    let blk: *mut snd_util_memblk;
    let mut page: c_int;
    let runtime: *mut snd_pcm_runtime = (*substream).runtime;
    let mut addr: dma_addr_t;

    if snd_BUG_ON(
        (*runtime).dma_bytes <= 0
            || (*runtime).dma_bytes
                > SNDRV_TRIDENT_MAX_PAGES as c_ulong * SNDRV_TRIDENT_PAGE_SIZE,
    ) != 0
    {
        return ptr::null_mut();
    }
    hdr = (*trident).tlb.memhdr;
    if snd_BUG_ON(hdr.is_null()) != 0 {
        return ptr::null_mut();
    }

    mutex_lock(&mut (*hdr).block_mutex);
    blk = search_empty(hdr, (*runtime).dma_bytes as c_int);
    if blk.is_null() {
        mutex_unlock(&mut (*hdr).block_mutex);
        return ptr::null_mut();
    }

    /* set TLB entries */
    addr = (*runtime).dma_addr;
    page = *firstpg(blk);
    while page <= *lastpg(blk) {
        if is_valid_page(trident, addr) == 0 {
            __snd_util_mem_free(hdr, blk);
            mutex_unlock(&mut (*hdr).block_mutex);
            return ptr::null_mut();
        }
        set_tlb_bus(trident, page, addr);
        page += 1;
        addr += SNDRV_TRIDENT_PAGE_SIZE as dma_addr_t;
    }
    mutex_unlock(&mut (*hdr).block_mutex);
    blk
}

/*
 * page allocation for DMA
 */
#[no_mangle]
pub unsafe extern "C" fn snd_trident_alloc_pages(
    trident: *mut snd_trident,
    substream: *mut snd_pcm_substream,
) -> *mut snd_util_memblk {
    if snd_BUG_ON(trident.is_null() || substream.is_null()) != 0 {
        return ptr::null_mut();
    }
    if (*substream).dma_buffer.dev.type_ == SNDRV_DMA_TYPE_DEV_SG {
        snd_trident_alloc_sg_pages(trident, substream)
    } else {
        snd_trident_alloc_cont_pages(trident, substream)
    }
}

/*
 * release DMA buffer from page table
 */
#[no_mangle]
pub unsafe extern "C" fn snd_trident_free_pages(
    trident: *mut snd_trident,
    blk: *mut snd_util_memblk,
) -> c_int {
    let hdr: *mut snd_util_memhdr;
    let mut page: c_int;

    if snd_BUG_ON(trident.is_null() || blk.is_null()) != 0 {
        return -EINVAL;
    }

    hdr = (*trident).tlb.memhdr;
    mutex_lock(&mut (*hdr).block_mutex);
    /* reset TLB entries */
    page = *firstpg(blk);
    while page <= *lastpg(blk) {
        set_silent_tlb(trident, page);
        page += 1;
    }
    /* free memory block */
    __snd_util_mem_free(hdr, blk);
    mutex_unlock(&mut (*hdr).block_mutex);
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
