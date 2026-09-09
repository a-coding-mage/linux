// SPDX-License-Identifier: GPL-2.0
/*
 * CMA DebugFS Interface
 *
 * Copyright (c) 2015 Sasha Levin <sasha.levin@oracle.com>
 */

use core::ffi::c_void;

// Declarations supplied by the Linux kernel and cma.h are intentionally kept external.
#[repr(C)]
pub struct CmaMem {
    pub node: HlistNode,
    pub p: *mut Page,
    pub n: usize,
}

#[repr(C)] pub struct HlistNode { pub next: *mut HlistNode, pub pprev: *mut *mut HlistNode }
#[repr(C)] pub struct Page { _private: [u8; 0] }
#[repr(C)] pub struct Dentry { _private: [u8; 0] }
#[repr(C)] pub struct Cma { pub lock: Spinlock, pub count: usize, pub available_count: usize, pub nranges: i32, pub ranges: *mut CmaMemrange, pub order_per_bit: u32, pub mem_head_lock: Spinlock, pub mem_head: HlistHead, pub name: *const i8 }
#[repr(C)] pub struct CmaMemrange { pub bitmap: *mut usize, pub base_pfn: usize, pub dfs_bitmap: U32Array }
#[repr(C)] pub struct Spinlock { _private: [u8; 0] }
#[repr(C)] pub struct HlistHead { pub first: *mut HlistNode }
#[repr(C)] pub struct U32Array { pub array: *mut u32, pub n_elements: u32 }

extern "C" {
    fn spin_lock_irq(lock: *mut Spinlock);
    fn spin_unlock_irq(lock: *mut Spinlock);
    fn hlist_add_head(node: *mut HlistNode, head: *mut HlistHead);
    fn hlist_empty(head: *const HlistHead) -> bool;
    fn hlist_del_init(node: *mut HlistNode);
    fn cma_release(cma: *mut Cma, page: *mut Page, count: usize);
    fn cma_alloc(cma: *mut Cma, count: usize, align: u32, no_warn: bool) -> *mut Page;
    fn kmalloc_zeroed(size: usize) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn cma_bitmap_maxno(cma: *const Cma, range: *const CmaMemrange) -> usize;
    fn debugfs_create_dir(name: *const i8, parent: *mut Dentry) -> *mut Dentry;
    fn debugfs_create_file(name: *const i8, mode: u32, parent: *mut Dentry, data: *mut c_void, fops: *const c_void) -> *mut Dentry;
    fn debugfs_create_u32_array(name: *const i8, mode: u32, parent: *mut Dentry, array: *mut U32Array) -> *mut Dentry;
    fn debugfs_create_symlink(name: *const i8, parent: *mut Dentry, target: *const i8) -> *mut Dentry;
    fn snprintf(buf: *mut i8, size: usize, fmt: *const i8, ...) -> i32;
    fn test_bit(bit: u32, addr: *const usize) -> bool;
    static mut cma_area_count: i32;
    static mut cma_areas: Cma;
}

unsafe fn cma_debugfs_get(data: *mut c_void, val: *mut u64) -> i32 {
    *val = *(data as *mut usize) as u64;
    0
}

unsafe fn cma_used_get(data: *mut c_void, val: *mut u64) -> i32 {
    let cma = data as *mut Cma;
    spin_lock_irq(&mut (*cma).lock);
    *val = ((*cma).count - (*cma).available_count) as u64;
    spin_unlock_irq(&mut (*cma).lock);
    0
}

unsafe fn cma_maxchunk_get(data: *mut c_void, val: *mut u64) -> i32 {
    let cma = data as *mut Cma;
    let mut maxchunk: usize = 0;
    spin_lock_irq(&mut (*cma).lock);
    for r in 0..(*cma).nranges {
        let cmr = (*cma).ranges.add(r as usize);
        let bitmap_maxno = cma_bitmap_maxno(cma, cmr);
        let mut start = 0usize;
        while start < bitmap_maxno {
            let mut end = start;
            while end < bitmap_maxno && (*cmr).bitmap.add(end / (usize::BITS as usize)).read() & (1usize << (end % usize::BITS as usize)) == 0 { end += 1; }
            if end > start { maxchunk = core::cmp::max(end - start, maxchunk); }
            start = if end == start { start + 1 } else { end };
        }
    }
    spin_unlock_irq(&mut (*cma).lock);
    *val = (maxchunk as u64) << (*cma).order_per_bit;
    0
}

unsafe fn cma_add_to_cma_mem_list(cma: *mut Cma, mem: *mut CmaMem) {
    spin_lock_irq(&mut (*cma).mem_head_lock);
    hlist_add_head(&mut (*mem).node, &mut (*cma).mem_head);
    spin_unlock_irq(&mut (*cma).mem_head_lock);
}

unsafe fn cma_get_entry_from_list(cma: *mut Cma) -> *mut CmaMem {
    let mut mem = core::ptr::null_mut();
    spin_lock_irq(&mut (*cma).mem_head_lock);
    if !hlist_empty(&(*cma).mem_head) {
        let node = (*cma).mem_head.first;
        mem = (node as *mut u8).sub(0) as *mut CmaMem;
        hlist_del_init(&mut (*mem).node);
    }
    spin_unlock_irq(&mut (*cma).mem_head_lock);
    mem
}

unsafe fn cma_free_mem(cma: *mut Cma, mut count: i32) -> i32 {
    while count != 0 {
        let mem = cma_get_entry_from_list(cma);
        if mem.is_null() { return 0; }
        if (*mem).n <= count as usize {
            cma_release(cma, (*mem).p, (*mem).n); count -= (*mem).n as i32; kfree(mem as *mut c_void);
        } else if (*cma).order_per_bit == 0 {
            cma_release(cma, (*mem).p, count as usize); (*mem).p = (*mem).p.add(count as usize); (*mem).n -= count as usize; count = 0; cma_add_to_cma_mem_list(cma, mem);
        } else { cma_add_to_cma_mem_list(cma, mem); break; }
    }
    0
}

unsafe fn cma_free_write(data: *mut c_void, val: u64) -> i32 { cma_free_mem(data as *mut Cma, val as i32) }

unsafe fn cma_alloc_mem(cma: *mut Cma, count: i32) -> i32 {
    let mem = kmalloc_zeroed(core::mem::size_of::<CmaMem>()) as *mut CmaMem;
    if mem.is_null() { return -12; }
    let p = cma_alloc(cma, count as usize, 0, false);
    if p.is_null() { kfree(mem as *mut c_void); return -12; }
    (*mem).p = p; (*mem).n = count as usize; cma_add_to_cma_mem_list(cma, mem); 0
}

unsafe fn cma_alloc_write(data: *mut c_void, val: u64) -> i32 { cma_alloc_mem(data as *mut Cma, val as i32) }

unsafe fn cma_debugfs_add_one(cma: *mut Cma, root_dentry: *mut Dentry) {
    let tmp = debugfs_create_dir((*cma).name, root_dentry);
    debugfs_create_file(b"alloc\0".as_ptr() as *const i8, 0o200, tmp, cma as *mut c_void, cma_alloc_write as *const c_void);
    debugfs_create_file(b"free\0".as_ptr() as *const i8, 0o200, tmp, cma as *mut c_void, cma_free_write as *const c_void);
    debugfs_create_file(b"count\0".as_ptr() as *const i8, 0o444, tmp, &mut (*cma).count as *mut usize as *mut c_void, cma_debugfs_get as *const c_void);
    debugfs_create_file(b"order_per_bit\0".as_ptr() as *const i8, 0o444, tmp, &mut (*cma).order_per_bit as *mut u32 as *mut c_void, cma_debugfs_get as *const c_void);
    debugfs_create_file(b"used\0".as_ptr() as *const i8, 0o444, tmp, cma as *mut c_void, cma_used_get as *const c_void);
    debugfs_create_file(b"maxchunk\0".as_ptr() as *const i8, 0o444, tmp, cma as *mut c_void, cma_maxchunk_get as *const c_void);
    let rangedir = debugfs_create_dir(b"ranges\0".as_ptr() as *const i8, tmp);
    let mut rdirname = [0i8; 12];
    for r in 0..(*cma).nranges {
        let cmr = (*cma).ranges.add(r as usize);
        snprintf(rdirname.as_mut_ptr(), rdirname.len(), b"%d\0".as_ptr() as *const i8, r);
        let dir = debugfs_create_dir(rdirname.as_ptr(), rangedir);
        debugfs_create_file(b"base_pfn\0".as_ptr() as *const i8, 0o444, dir, &mut (*cmr).base_pfn as *mut usize as *mut c_void, cma_debugfs_get as *const c_void);
        (*cmr).dfs_bitmap.array = (*cmr).bitmap as *mut u32;
        (*cmr).dfs_bitmap.n_elements = ((cma_bitmap_maxno(cma, cmr) + 31) / 32) as u32;
        debugfs_create_u32_array(b"bitmap\0".as_ptr() as *const i8, 0o444, dir, &mut (*cmr).dfs_bitmap);
    }
    debugfs_create_symlink(b"base_pfn\0".as_ptr() as *const i8, tmp, b"ranges/0/base_pfn\0".as_ptr() as *const i8);
    debugfs_create_symlink(b"bitmap\0".as_ptr() as *const i8, tmp, b"ranges/0/bitmap\0".as_ptr() as *const i8);
}

unsafe fn cma_debugfs_init() -> i32 {
    let root = debugfs_create_dir(b"cma\0".as_ptr() as *const i8, core::ptr::null_mut());
    for i in 0..cma_area_count {
        let cma = cma_areas.add(i as usize);
        if test_bit(0, &(*cma).count as *const usize) { cma_debugfs_add_one(cma, root); }
    }
    0
}

// DEFINE_DEBUGFS_ATTRIBUTE declarations and late_initcall(cma_debugfs_init) are supplied by the kernel build environment.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
