// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Copyright (c) by Jaroslav Kysela <perex@perex.cz>
 *  Copyright (c) by Takashi Iwai <tiwai@suse.de>
 *
 *  EMU10K1 memory page allocation (PTB area)
 */

use crate::*;
use core::ffi::c_void;
use core::ptr;

/* C includes:
 * <linux/pci.h>, <linux/gfp.h>, <linux/time.h>, <linux/mutex.h>,
 * <linux/export.h>, <sound/core.h>, <sound/emu10k1.h>
 */

/* page arguments of these two macros are Emu page (4096 bytes), not like
 * aligned pages in others
 */
unsafe fn __set_ptb_entry(emu: *mut snd_emu10k1, page: i32, addr: dma_addr_t) {
    let entry = ((addr << (*emu).address_mode) | page as dma_addr_t) as u32;
    *((*emu).ptb_pages.area as *mut __le32).offset(page as isize) = cpu_to_le32(entry);
}

unsafe fn __get_ptb_entry(emu: *mut snd_emu10k1, page: i32) -> u32 {
    le32_to_cpu(*((*emu).ptb_pages.area as *mut __le32).offset(page as isize))
}

const UNIT_PAGES: usize = PAGE_SIZE / EMUPAGESIZE;
const MAX_ALIGN_PAGES0: usize = MAXPAGES0 / UNIT_PAGES;
const MAX_ALIGN_PAGES1: usize = MAXPAGES1 / UNIT_PAGES;

/* get aligned page from offset address */
fn get_aligned_page(offset: i32) -> i32 {
    offset >> PAGE_SHIFT
}

/* get offset address from aligned page */
fn aligned_page_offset(page: i32) -> i32 {
    page << PAGE_SHIFT
}

/* C condition:
 * #if PAGE_SIZE == EMUPAGESIZE && !IS_ENABLED(CONFIG_DYNAMIC_DEBUG)
 * The runtime translation uses the general implementation so the debug-capable
 * UNIT_PAGES behavior is preserved when those configuration constants differ.
 */
unsafe fn set_ptb_entry(emu: *mut snd_emu10k1, mut page: i32, mut addr: dma_addr_t) {
    let mut i: i32;

    page *= UNIT_PAGES as i32;
    i = 0;
    while i < UNIT_PAGES as i32 {
        __set_ptb_entry(emu, page, addr);
        dev_dbg!(
            (*(*emu).card).dev,
            "mapped page %d to entry %.8x\n",
            page,
            __get_ptb_entry(emu, page) as u32
        );
        addr += EMUPAGESIZE as dma_addr_t;
        i += 1;
        page += 1;
    }
}

unsafe fn set_silent_ptb(emu: *mut snd_emu10k1, mut page: i32) {
    let mut i: i32;

    page *= UNIT_PAGES as i32;
    i = 0;
    while i < UNIT_PAGES as i32 {
        /* do not increment ptr */
        __set_ptb_entry(emu, page, (*emu).silent_page.addr);
        dev_dbg!(
            (*(*emu).card).dev,
            "mapped silent page %d to entry %.8x\n",
            page,
            __get_ptb_entry(emu, page) as u32
        );
        i += 1;
        page += 1;
    }
}

/*
 */
unsafe fn synth_alloc_pages(hw: *mut snd_emu10k1, blk: *mut snd_emu10k1_memblk) -> i32;
unsafe fn synth_free_pages(hw: *mut snd_emu10k1, blk: *mut snd_emu10k1_memblk) -> i32;

macro_rules! get_emu10k1_memblk {
    ($l:expr, $member:tt) => {
        list_entry!($l, snd_emu10k1_memblk, $member)
    };
}

/* initialize emu10k1 part */
unsafe fn emu10k1_memblk_init(blk: *mut snd_emu10k1_memblk) {
    (*blk).mapped_page = -1;
    INIT_LIST_HEAD(&mut (*blk).mapped_link);
    INIT_LIST_HEAD(&mut (*blk).mapped_order_link);
    (*blk).map_locked = 0;

    (*blk).first_page = get_aligned_page((*blk).mem.offset);
    (*blk).last_page = get_aligned_page((*blk).mem.offset + (*blk).mem.size - 1);
    (*blk).pages = (*blk).last_page - (*blk).first_page + 1;
}

/*
 * search empty region on PTB with the given size
 *
 * if an empty region is found, return the page and store the next mapped block
 * in nextp
 * if not found, return a negative error code.
 */
unsafe fn search_empty_map_area(
    emu: *mut snd_emu10k1,
    npages: i32,
    nextp: *mut *mut list_head,
) -> i32 {
    let mut page: i32 = 1;
    let mut found_page: i32 = -ENOMEM;
    let mut max_size: i32 = npages;
    let mut size: i32;
    let mut candidate: *mut list_head = &mut (*emu).mapped_link_head;
    let mut pos: *mut list_head;

    pos = (*emu).mapped_link_head.next;
    while pos != &mut (*emu).mapped_link_head {
        let blk: *mut snd_emu10k1_memblk = get_emu10k1_memblk!(pos, mapped_link);
        if (*blk).mapped_page >= 0 {
            size = (*blk).mapped_page - page;
            if size == npages {
                *nextp = pos;
                return page;
            } else if size > max_size {
                /* we look for the maximum empty hole */
                max_size = size;
                candidate = pos;
                found_page = page;
            }
            page = (*blk).mapped_page + (*blk).pages;
        }
        pos = (*pos).next;
    }
    size = (if (*emu).address_mode != 0 {
        MAX_ALIGN_PAGES1
    } else {
        MAX_ALIGN_PAGES0
    }) as i32
        - page;
    if size >= max_size {
        *nextp = pos;
        return page;
    }
    *nextp = candidate;
    found_page
}

/*
 * map a memory block onto emu10k1's PTB
 *
 * call with memblk_lock held
 */
unsafe fn map_memblk(emu: *mut snd_emu10k1, blk: *mut snd_emu10k1_memblk) -> i32 {
    let mut page: i32;
    let mut pg: i32;
    let mut next: *mut list_head = ptr::null_mut();

    page = search_empty_map_area(emu, (*blk).pages, &mut next);
    if page < 0 {
        /* not found */
        return page;
    }
    if page == 0 {
        dev_err!(
            (*(*emu).card).dev,
            "trying to map zero (reserved) page\n"
        );
        return -EINVAL;
    }
    /* insert this block in the proper position of mapped list */
    list_add_tail(&mut (*blk).mapped_link, next);
    /* append this as a newest block in order list */
    list_add_tail(
        &mut (*blk).mapped_order_link,
        &mut (*emu).mapped_order_link_head,
    );
    (*blk).mapped_page = page;
    /* fill PTB */
    pg = (*blk).first_page;
    while pg <= (*blk).last_page {
        set_ptb_entry(emu, page, *(*emu).page_addr_table.offset(pg as isize));
        page += 1;
        pg += 1;
    }
    0
}

/*
 * unmap the block
 * return the size of resultant empty pages
 *
 * call with memblk_lock held
 */
unsafe fn unmap_memblk(emu: *mut snd_emu10k1, blk: *mut snd_emu10k1_memblk) -> i32 {
    let start_page: i32;
    let end_page: i32;
    let mut mpage: i32;
    let mut pg: i32;
    let mut p: *mut list_head;
    let mut q: *mut snd_emu10k1_memblk;

    /* calculate the expected size of empty region */
    p = (*blk).mapped_link.prev;
    if p != &mut (*emu).mapped_link_head {
        q = get_emu10k1_memblk!(p, mapped_link);
        start_page = (*q).mapped_page + (*q).pages;
    } else {
        start_page = 1;
    }
    p = (*blk).mapped_link.next;
    if p != &mut (*emu).mapped_link_head {
        q = get_emu10k1_memblk!(p, mapped_link);
        end_page = (*q).mapped_page;
    } else {
        end_page = if (*emu).address_mode != 0 {
            MAX_ALIGN_PAGES1 as i32
        } else {
            MAX_ALIGN_PAGES0 as i32
        };
    }

    /* remove links */
    list_del(&mut (*blk).mapped_link);
    list_del(&mut (*blk).mapped_order_link);
    /* clear PTB */
    mpage = (*blk).mapped_page;
    pg = (*blk).first_page;
    while pg <= (*blk).last_page {
        set_silent_ptb(emu, mpage);
        mpage += 1;
        pg += 1;
    }
    (*blk).mapped_page = -1;
    end_page - start_page /* return the new empty size */
}

/*
 * search empty pages with the given size, and create a memory block
 *
 * unlike synth_alloc the memory block is aligned to the page start
 */
unsafe fn search_empty(emu: *mut snd_emu10k1, size: i32) -> *mut snd_emu10k1_memblk {
    let mut p: *mut list_head;
    let mut blk: *mut snd_emu10k1_memblk;
    let mut page: i32;
    let psize: i32;

    psize = get_aligned_page(size + PAGE_SIZE as i32 - 1);
    page = 0;
    p = (*(*emu).memhdr).block.next;
    while p != &mut (*(*emu).memhdr).block {
        blk = get_emu10k1_memblk!(p, mem.list);
        if page + psize <= (*blk).first_page {
            break;
        }
        page = (*blk).last_page + 1;
        p = (*p).next;
    }
    if p == &mut (*(*emu).memhdr).block && page + psize > (*emu).max_cache_pages {
        return ptr::null_mut();
    }

    /* create a new memory block */
    blk = __snd_util_memblk_new((*emu).memhdr, psize << PAGE_SHIFT, (*p).prev)
        as *mut snd_emu10k1_memblk;
    if blk.is_null() {
        return ptr::null_mut();
    }
    (*blk).mem.offset = aligned_page_offset(page); /* set aligned offset */
    emu10k1_memblk_init(blk);
    blk
}

/*
 * check if the given pointer is valid for pages
 */
unsafe fn is_valid_page(emu: *mut snd_emu10k1, addr: dma_addr_t) -> i32 {
    if addr & !(*emu).dma_mask != 0 {
        dev_err_ratelimited!(
            (*(*emu).card).dev,
            "max memory size is 0x%lx (addr = 0x%lx)!!\n",
            (*emu).dma_mask,
            addr as c_ulong
        );
        return 0;
    }
    if addr & (EMUPAGESIZE as dma_addr_t - 1) != 0 {
        dev_err_ratelimited!((*(*emu).card).dev, "page is not aligned\n");
        return 0;
    }
    1
}

/*
 * map the given memory block on PTB.
 * if the block is already mapped, update the link order.
 * if no empty pages are found, tries to release unused memory blocks
 * and retry the mapping.
 */
#[no_mangle]
pub unsafe extern "C" fn snd_emu10k1_memblk_map(
    emu: *mut snd_emu10k1,
    blk: *mut snd_emu10k1_memblk,
) -> i32 {
    let mut err: i32;
    let size: i32;
    let mut p: *mut list_head;
    let mut nextp: *mut list_head;
    let mut deleted: *mut snd_emu10k1_memblk;

    /* guard(spinlock_irqsave)(&emu->memblk_lock); */
    spin_lock_irqsave_guard!(&mut (*emu).memblk_lock);
    if (*blk).mapped_page >= 0 {
        /* update order link */
        list_move_tail(
            &mut (*blk).mapped_order_link,
            &mut (*emu).mapped_order_link_head,
        );
        return 0;
    }
    err = map_memblk(emu, blk);
    if err < 0 {
        /* no enough page - try to unmap some blocks */
        /* starting from the oldest block */
        p = (*emu).mapped_order_link_head.next;
        while p != &mut (*emu).mapped_order_link_head {
            nextp = (*p).next;
            deleted = get_emu10k1_memblk!(p, mapped_order_link);
            if (*deleted).map_locked == 0 {
                size = unmap_memblk(emu, deleted);
                if size >= (*blk).pages {
                    /* ok the empty region is enough large */
                    err = map_memblk(emu, blk);
                    break;
                }
            }
            p = nextp;
        }
    }
    err
}

/*
 * page allocation for DMA
 */
#[no_mangle]
pub unsafe extern "C" fn snd_emu10k1_alloc_pages(
    emu: *mut snd_emu10k1,
    substream: *mut snd_pcm_substream,
) -> *mut snd_util_memblk {
    let runtime: *mut snd_pcm_runtime = (*substream).runtime;
    let hdr: *mut snd_util_memhdr;
    let mut blk: *mut snd_emu10k1_memblk;
    let mut page: i32;
    let mut err: i32;
    let mut idx: i32;

    if snd_BUG_ON(emu.is_null()) {
        return ptr::null_mut();
    }
    if snd_BUG_ON(
        (*runtime).dma_bytes <= 0
            || (*runtime).dma_bytes
                >= (if (*emu).address_mode != 0 {
                    MAXPAGES1
                } else {
                    MAXPAGES0
                } * EMUPAGESIZE) as i32,
    ) {
        return ptr::null_mut();
    }
    hdr = (*emu).memhdr;
    if snd_BUG_ON(hdr.is_null()) {
        return ptr::null_mut();
    }

    /* guard(mutex)(&hdr->block_mutex); */
    mutex_guard!(&mut (*hdr).block_mutex);
    blk = search_empty(emu, (*runtime).dma_bytes);
    if blk.is_null() {
        return ptr::null_mut();
    }
    /* fill buffer addresses but pointers are not stored so that
     * snd_free_pci_page() is not called in synth_free()
     */
    idx = 0;
    page = (*blk).first_page;
    while page <= (*blk).last_page {
        let ofs: c_ulong = (idx << PAGE_SHIFT) as c_ulong;
        let addr: dma_addr_t;
        if ofs >= (*runtime).dma_bytes as c_ulong {
            addr = (*emu).silent_page.addr;
        } else {
            addr = snd_pcm_sgbuf_get_addr(substream, ofs);
        }
        if is_valid_page(emu, addr) == 0 {
            dev_err_ratelimited!((*(*emu).card).dev, "emu: failure page = %d\n", idx);
            return ptr::null_mut();
        }
        *(*emu).page_addr_table.offset(page as isize) = addr;
        *(*emu).page_ptr_table.offset(page as isize) = ptr::null_mut();
        page += 1;
        idx += 1;
    }

    /* set PTB entries */
    (*blk).map_locked = 1; /* do not unmap this block! */
    err = snd_emu10k1_memblk_map(emu, blk);
    if err < 0 {
        __snd_util_mem_free(hdr, blk as *mut snd_util_memblk);
        return ptr::null_mut();
    }
    blk as *mut snd_util_memblk
}

/*
 * release DMA buffer from page table
 */
#[no_mangle]
pub unsafe extern "C" fn snd_emu10k1_free_pages(
    emu: *mut snd_emu10k1,
    blk: *mut snd_util_memblk,
) -> i32 {
    if snd_BUG_ON(emu.is_null() || blk.is_null()) {
        return -EINVAL;
    }
    snd_emu10k1_synth_free(emu, blk)
}

/*
 * allocate DMA pages, widening the allocation if necessary
 *
 * See the comment above snd_emu10k1_detect_iommu() in emu10k1_main.c why
 * this might be needed.
 *
 * If you modify this function check whether __synth_free_pages() also needs
 * changes.
 */
#[no_mangle]
pub unsafe extern "C" fn snd_emu10k1_alloc_pages_maybe_wider(
    emu: *mut snd_emu10k1,
    mut size: size_t,
    dmab: *mut snd_dma_buffer,
) -> i32 {
    if (*emu).iommu_workaround != 0 {
        let npages: size_t = DIV_ROUND_UP(size, PAGE_SIZE as size_t);
        let size_real: size_t = npages * PAGE_SIZE as size_t;

        /*
         * The device has been observed to accesses up to 256 extra
         * bytes, but use 1k to be safe.
         */
        if size_real < size + 1024 {
            size += PAGE_SIZE as size_t;
        }
    }

    snd_dma_alloc_pages(SNDRV_DMA_TYPE_DEV, &mut (*(*emu).pci).dev, size, dmab)
}

/*
 * memory allocation using multiple pages (for synth)
 * Unlike the DMA allocation above, non-contiguous pages are assined.
 */

/*
 * allocate a synth sample area
 */
#[no_mangle]
pub unsafe extern "C" fn snd_emu10k1_synth_alloc(
    hw: *mut snd_emu10k1,
    size: u32,
) -> *mut snd_util_memblk {
    let mut blk: *mut snd_emu10k1_memblk;
    let hdr: *mut snd_util_memhdr = (*hw).memhdr;

    /* guard(mutex)(&hdr->block_mutex); */
    mutex_guard!(&mut (*hdr).block_mutex);
    blk = __snd_util_mem_alloc(hdr, size) as *mut snd_emu10k1_memblk;
    if blk.is_null() {
        return ptr::null_mut();
    }
    if synth_alloc_pages(hw, blk) != 0 {
        __snd_util_mem_free(hdr, blk as *mut snd_util_memblk);
        return ptr::null_mut();
    }
    snd_emu10k1_memblk_map(hw, blk);
    blk as *mut snd_util_memblk
}

/*
 * free a synth sample area
 */
#[no_mangle]
pub unsafe extern "C" fn snd_emu10k1_synth_free(
    emu: *mut snd_emu10k1,
    memblk: *mut snd_util_memblk,
) -> i32 {
    let hdr: *mut snd_util_memhdr = (*emu).memhdr;
    let blk: *mut snd_emu10k1_memblk = memblk as *mut snd_emu10k1_memblk;

    /* guard(mutex)(&hdr->block_mutex); */
    mutex_guard!(&mut (*hdr).block_mutex);
    /* scoped_guard(spinlock_irqsave, &emu->memblk_lock) */
    {
        spin_lock_irqsave_guard!(&mut (*emu).memblk_lock);
        if (*blk).mapped_page >= 0 {
            unmap_memblk(emu, blk);
        }
    }
    synth_free_pages(emu, blk);
    __snd_util_mem_free(hdr, memblk);
    0
}

/* check new allocation range */
unsafe fn get_single_page_range(
    hdr: *mut snd_util_memhdr,
    blk: *mut snd_emu10k1_memblk,
    first_page_ret: *mut i32,
    last_page_ret: *mut i32,
) {
    let mut p: *mut list_head;
    let mut q: *mut snd_emu10k1_memblk;
    let mut first_page: i32;
    let mut last_page: i32;

    first_page = (*blk).first_page;
    p = (*blk).mem.list.prev;
    if p != &mut (*hdr).block {
        q = get_emu10k1_memblk!(p, mem.list);
        if (*q).last_page == first_page {
            first_page += 1; /* first page was already allocated */
        }
    }
    last_page = (*blk).last_page;
    p = (*blk).mem.list.next;
    if p != &mut (*hdr).block {
        q = get_emu10k1_memblk!(p, mem.list);
        if (*q).first_page == last_page {
            last_page -= 1; /* last page was already allocated */
        }
    }
    *first_page_ret = first_page;
    *last_page_ret = last_page;
}

/* release allocated pages */
unsafe fn __synth_free_pages(emu: *mut snd_emu10k1, first_page: i32, last_page: i32) {
    let mut dmab: snd_dma_buffer = core::mem::zeroed();
    let mut page: i32;

    dmab.dev.type_ = SNDRV_DMA_TYPE_DEV;
    dmab.dev.dev = &mut (*(*emu).pci).dev;

    page = first_page;
    while page <= last_page {
        if (*(*emu).page_ptr_table.offset(page as isize)).is_null() {
            page += 1;
            continue;
        }
        dmab.area = *(*emu).page_ptr_table.offset(page as isize);
        dmab.addr = *(*emu).page_addr_table.offset(page as isize);

        /*
         * please keep me in sync with logic in
         * snd_emu10k1_alloc_pages_maybe_wider()
         */
        dmab.bytes = PAGE_SIZE as size_t;
        if (*emu).iommu_workaround != 0 {
            dmab.bytes *= 2;
        }

        snd_dma_free_pages(&mut dmab);
        *(*emu).page_addr_table.offset(page as isize) = 0;
        *(*emu).page_ptr_table.offset(page as isize) = ptr::null_mut();
        page += 1;
    }
}

/*
 * allocate kernel pages
 */
unsafe fn synth_alloc_pages(emu: *mut snd_emu10k1, blk: *mut snd_emu10k1_memblk) -> i32 {
    let mut page: i32;
    let mut first_page: i32 = 0;
    let mut last_page: i32 = 0;
    let mut dmab: snd_dma_buffer = core::mem::zeroed();

    emu10k1_memblk_init(blk);
    get_single_page_range((*emu).memhdr, blk, &mut first_page, &mut last_page);
    /* allocate kernel pages */
    page = first_page;
    while page <= last_page {
        if snd_emu10k1_alloc_pages_maybe_wider(emu, PAGE_SIZE as size_t, &mut dmab) < 0 {
            break;
        }
        if is_valid_page(emu, dmab.addr) == 0 {
            snd_dma_free_pages(&mut dmab);
            break;
        }
        *(*emu).page_addr_table.offset(page as isize) = dmab.addr;
        *(*emu).page_ptr_table.offset(page as isize) = dmab.area;
        page += 1;
    }
    if page > last_page {
        return 0;
    }

    /* release allocated pages */
    last_page = page - 1;
    __synth_free_pages(emu, first_page, last_page);

    -ENOMEM
}

/*
 * free pages
 */
unsafe fn synth_free_pages(emu: *mut snd_emu10k1, blk: *mut snd_emu10k1_memblk) -> i32 {
    let mut first_page: i32 = 0;
    let mut last_page: i32 = 0;

    get_single_page_range((*emu).memhdr, blk, &mut first_page, &mut last_page);
    __synth_free_pages(emu, first_page, last_page);
    0
}

/* calculate buffer pointer from offset address */
unsafe fn offset_ptr(emu: *mut snd_emu10k1, page: i32, offset: i32) -> *mut c_void {
    let mut ptr_: *mut u8;

    if snd_BUG_ON(page < 0 || page >= (*emu).max_cache_pages) {
        return ptr::null_mut();
    }
    ptr_ = *(*emu).page_ptr_table.offset(page as isize) as *mut u8;
    if ptr_.is_null() {
        dev_err!((*(*emu).card).dev, "access to NULL ptr: page = %d\n", page);
        return ptr::null_mut();
    }
    ptr_ = ptr_.offset((offset & (PAGE_SIZE as i32 - 1)) as isize);
    ptr_ as *mut c_void
}

/*
 * memset(blk + offset, value, size)
 */
#[no_mangle]
pub unsafe extern "C" fn snd_emu10k1_synth_memset(
    emu: *mut snd_emu10k1,
    blk: *mut snd_util_memblk,
    mut offset: i32,
    size: i32,
    value: u8,
) -> i32 {
    let mut page: i32;
    let mut nextofs: i32;
    let end_offset: i32;
    let mut temp: i32;
    let temp1: i32;
    let mut ptr_: *mut c_void;
    let p: *mut snd_emu10k1_memblk = blk as *mut snd_emu10k1_memblk;

    if snd_BUG_ON(offset + size > (*p).mem.size) {
        return -EFAULT;
    }

    offset += (*blk).offset & (PAGE_SIZE as i32 - 1);
    end_offset = offset + size;
    page = get_aligned_page(offset);
    loop {
        nextofs = aligned_page_offset(page + 1);
        temp = nextofs - offset;
        temp1 = end_offset - offset;
        if temp1 < temp {
            temp = temp1;
        }
        ptr_ = offset_ptr(emu, page + (*p).first_page, offset);
        if !ptr_.is_null() {
            memset(ptr_, value as i32, temp as usize);
        }
        offset = nextofs;
        page += 1;
        if offset >= end_offset {
            break;
        }
    }
    0
}

// Note that the value is assumed to be suitably repetitive.
unsafe fn xor_range(mut ptr_: *mut c_void, mut size: i32, value: u32) {
    if (ptr_ as isize) & 1 != 0 {
        *(ptr_ as *mut u8) ^= value as u8;
        ptr_ = (ptr_ as *mut u8).offset(1) as *mut c_void;
        size -= 1;
    }
    if size > 1 && ((ptr_ as isize) & 2) != 0 {
        *(ptr_ as *mut u16) ^= value as u16;
        ptr_ = (ptr_ as *mut u8).offset(2) as *mut c_void;
        size -= 2;
    }
    while size > 3 {
        *(ptr_ as *mut u32) ^= value;
        ptr_ = (ptr_ as *mut u8).offset(4) as *mut c_void;
        size -= 4;
    }
    if size > 1 {
        *(ptr_ as *mut u16) ^= value as u16;
        ptr_ = (ptr_ as *mut u8).offset(2) as *mut c_void;
        size -= 2;
    }
    if size > 0 {
        *(ptr_ as *mut u8) ^= value as u8;
    }
}

/*
 * copy_from_user(blk + offset, data, size) ^ xor
 */
#[no_mangle]
pub unsafe extern "C" fn snd_emu10k1_synth_copy_from_user(
    emu: *mut snd_emu10k1,
    blk: *mut snd_util_memblk,
    mut offset: i32,
    mut data: *const c_char,
    size: i32,
    xor: u32,
) -> i32 {
    let mut page: i32;
    let mut nextofs: i32;
    let end_offset: i32;
    let mut temp: i32;
    let temp1: i32;
    let mut ptr_: *mut c_void;
    let p: *mut snd_emu10k1_memblk = blk as *mut snd_emu10k1_memblk;

    if snd_BUG_ON(offset + size > (*p).mem.size) {
        return -EFAULT;
    }

    offset += (*blk).offset & (PAGE_SIZE as i32 - 1);
    end_offset = offset + size;
    page = get_aligned_page(offset);
    loop {
        nextofs = aligned_page_offset(page + 1);
        temp = nextofs - offset;
        temp1 = end_offset - offset;
        if temp1 < temp {
            temp = temp1;
        }
        ptr_ = offset_ptr(emu, page + (*p).first_page, offset);
        if !ptr_.is_null() {
            if copy_from_user(ptr_, data as *const c_void, temp as usize) != 0 {
                return -EFAULT;
            }
            if xor != 0 {
                xor_range(ptr_, temp, xor);
            }
        }
        offset = nextofs;
        data = data.offset(temp as isize);
        page += 1;
        if offset >= end_offset {
            break;
        }
    }
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
