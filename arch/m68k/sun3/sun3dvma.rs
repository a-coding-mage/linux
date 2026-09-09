// SPDX-License-Identifier: GPL-2.0
/*
 * linux/arch/m68k/sun3/sun3dvma.c
 *
 * Copyright (C) 2000 Sam Creasey
 *
 * Contains common routines for sun3/sun3x DVMA management.
 */

use core::ptr;

// Kernel declarations supplied by the surrounding tree.
extern "C" {
    static mut iommu_use: *mut c_ulong;
    fn dvma_unmap_iommu(baddr: c_ulong, len: c_ulong);
    fn sun3_dvma_init();
    fn dvma_map_iommu(kaddr: c_ulong, baddr: c_ulong, len: c_ulong) -> c_int;
    fn dvma_map_cpu(kaddr: c_ulong, vaddr: c_ulong, len: c_ulong) -> c_int;
    fn dvma_btov(baddr: c_ulong) -> c_ulong;
    fn __get_free_pages(gfp: c_ulong, order: c_ulong) -> c_ulong;
    fn free_pages(addr: c_ulong, order: c_ulong);
    fn get_order(size: c_ulong) -> c_ulong;
    fn memblock_alloc_or_panic(size: usize, align: usize) -> *mut c_ulong;
}

type c_int = i32;
type c_ulong = usize;

// Values and list primitives are provided by the kernel headers.
extern "C" {
    fn INIT_LIST_HEAD(list: *mut list_head);
    fn list_add(new: *mut list_head, head: *mut list_head);
    fn list_del(entry: *mut list_head);
    fn list_move(entry: *mut list_head, head: *mut list_head);
    fn list_empty(head: *const list_head) -> bool;
    fn pr_debug(fmt: *const u8, ...);
    fn pr_crit(fmt: *const u8, ...);
}

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

const DVMA_START: c_ulong = 0; // supplied by <asm/dvma.h>
const DVMA_END: c_ulong = 0; // supplied by <asm/dvma.h>
const DVMA_SIZE: c_ulong = 0; // supplied by <asm/dvma.h>
const DVMA_PAGE_SHIFT: usize = 0; // supplied by <asm/dvma.h>
const DVMA_PAGE_SIZE: c_ulong = 0; // supplied by <asm/dvma.h>
const DVMA_PAGE_MASK: c_ulong = 0; // supplied by <asm/dvma.h>
const PAGE_MASK: c_ulong = 0; // supplied by <asm/page.h>
const IOMMU_TOTAL_ENTRIES: usize = 0; // supplied by <asm/dvma.h>
const SMP_CACHE_BYTES: usize = 0; // supplied by the kernel
const GFP_ATOMIC: c_ulong = 0; // supplied by <linux/gfp.h>

#[repr(C)]
struct hole {
    start: c_ulong,
    end: c_ulong,
    size: c_ulong,
    list: list_head,
}

static mut HOLE_LIST: list_head = list_head { next: ptr::null_mut(), prev: ptr::null_mut() };
static mut HOLE_CACHE: list_head = list_head { next: ptr::null_mut(), prev: ptr::null_mut() };
static mut INITHOLES: [hole; 64] = [const { hole { start: 0, end: 0, size: 0, list: list_head { next: ptr::null_mut(), prev: ptr::null_mut() } } }; 64];

#[inline]
unsafe fn dvma_index(baddr: c_ulong) -> usize {
    (baddr.wrapping_sub(DVMA_START)) >> DVMA_PAGE_SHIFT
}

#[inline]
unsafe fn dvma_entry_use(baddr: c_ulong) -> *mut c_ulong {
    iommu_use.add(dvma_index(baddr))
}

#[inline]
unsafe fn hole_from_list(p: *mut list_head) -> *mut hole {
    (p as *mut u8).sub(core::mem::offset_of!(hole, list)) as *mut hole
}

#[inline]
unsafe fn refill() -> c_int {
    let mut prev: *mut hole = ptr::null_mut();
    let mut cur = HOLE_LIST.next;
    let mut ret = 0;
    while cur != &mut HOLE_LIST as *mut list_head {
        let h = hole_from_list(cur);
        let next = (*cur).next;
        if prev.is_null() {
            prev = h;
        } else if (*h).end == (*prev).start {
            (*h).size += (*prev).size;
            (*h).end = (*prev).end;
            list_move(&mut (*prev).list, &mut HOLE_CACHE);
            ret += 1;
        }
        cur = next;
    }
    ret
}

#[inline]
unsafe fn rmcache() -> *mut hole {
    if list_empty(&HOLE_CACHE) {
        if refill() == 0 {
            pr_crit(b"out of dvma hole cache!\0".as_ptr());
            panic!("BUG");
        }
    }
    let ret = hole_from_list(HOLE_CACHE.next);
    list_del(&mut (*ret).list);
    ret
}

#[inline]
unsafe fn get_baddr(len: c_int, align: c_ulong) -> c_ulong {
    if list_empty(&HOLE_LIST) {
        panic!("BUG");
    }
    let mut cur = HOLE_LIST.next;
    while cur != &mut HOLE_LIST as *mut list_head {
        let h = hole_from_list(cur);
        let newlen = if align > DVMA_PAGE_SIZE {
            (len as c_ulong) + ((*h).end.wrapping_sub(len as c_ulong) & (align - 1))
        } else { len as c_ulong };
        if (*h).size > newlen {
            (*h).end -= newlen;
            (*h).size -= newlen;
            *dvma_entry_use((*h).end) = newlen;
            return (*h).end;
        } else if (*h).size == newlen {
            list_move(&mut (*h).list, &mut HOLE_CACHE);
            *dvma_entry_use((*h).start) = newlen;
            return (*h).start;
        }
        cur = (*cur).next;
    }
    pr_crit(b"unable to find dvma hole!\0".as_ptr());
    panic!("BUG");
}

#[inline]
unsafe fn free_baddr(mut baddr: c_ulong) -> c_int {
    let len = *dvma_entry_use(baddr);
    *dvma_entry_use(baddr) = 0;
    baddr &= DVMA_PAGE_MASK;
    dvma_unmap_iommu(baddr, len);
    let mut cur = HOLE_LIST.next;
    while cur != &mut HOLE_LIST as *mut list_head {
        let h = hole_from_list(cur);
        if (*h).end == baddr {
            (*h).end += len; (*h).size += len; return 0;
        } else if (*h).start == baddr + len {
            (*h).start = baddr; (*h).size += len; return 0;
        }
        cur = (*cur).next;
    }
    let h = rmcache();
    (*h).start = baddr; (*h).end = baddr + len; (*h).size = len;
    list_add(&mut (*h).list, cur);
    0
}

pub unsafe extern "C" fn dvma_init() {
    INIT_LIST_HEAD(&mut HOLE_LIST); INIT_LIST_HEAD(&mut HOLE_CACHE);
    for i in 0..64 { list_add(&mut INITHOLES[i].list, &mut HOLE_CACHE); }
    let h = rmcache();
    (*h).start = DVMA_START; (*h).end = DVMA_END; (*h).size = DVMA_SIZE;
    list_add(&mut (*h).list, &mut HOLE_LIST);
    iommu_use = memblock_alloc_or_panic(IOMMU_TOTAL_ENTRIES * core::mem::size_of::<c_ulong>(), SMP_CACHE_BYTES);
    dvma_unmap_iommu(DVMA_START, DVMA_SIZE); sun3_dvma_init();
}

pub unsafe extern "C" fn dvma_map_align(mut kaddr: c_ulong, mut len: c_int, mut align: c_int) -> c_ulong {
    if len == 0 { len = 0x800; }
    if kaddr == 0 || len == 0 { return 0; }
    let off = kaddr & !DVMA_PAGE_MASK; kaddr &= PAGE_MASK;
    len += off as c_int; len = ((len as c_ulong + DVMA_PAGE_SIZE - 1) & DVMA_PAGE_MASK) as c_int;
    let alignment = if align == 0 { DVMA_PAGE_SIZE } else { ((align as c_ulong + DVMA_PAGE_SIZE - 1) & DVMA_PAGE_MASK) };
    let baddr = get_baddr(len, alignment);
    if dvma_map_iommu(kaddr, baddr, len as c_ulong) == 0 { return baddr + off; }
    pr_crit(b"dvma_map failed\0".as_ptr()); panic!("BUG");
}

pub unsafe extern "C" fn dvma_unmap(baddr: *mut core::ffi::c_void) {
    let mut addr = baddr as c_ulong;
    if addr & 0x00f00000 == 0 { addr |= 0xf00000; }
    free_baddr(addr);
}

pub unsafe extern "C" fn dvma_malloc_align(len: c_ulong, align: c_ulong) -> *mut core::ffi::c_void {
    if len == 0 { return ptr::null_mut(); }
    let len = (len + DVMA_PAGE_SIZE - 1) & DVMA_PAGE_MASK;
    let kaddr = __get_free_pages(GFP_ATOMIC, get_order(len)); if kaddr == 0 { return ptr::null_mut(); }
    let baddr = dvma_map_align(kaddr, len as c_int, align as c_int); if baddr == 0 { free_pages(kaddr, get_order(len)); return ptr::null_mut(); }
    let vaddr = dvma_btov(baddr);
    if dvma_map_cpu(kaddr, vaddr, len) < 0 { dvma_unmap(baddr as *mut _); free_pages(kaddr, get_order(len)); return ptr::null_mut(); }
    vaddr as *mut core::ffi::c_void
}

pub unsafe extern "C" fn dvma_free(_vaddr: *mut core::ffi::c_void) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
