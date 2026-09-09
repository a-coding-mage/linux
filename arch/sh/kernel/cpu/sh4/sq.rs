// SPDX-License-Identifier: GPL-2.0
/*
 * arch/sh/kernel/cpu/sh4/sq.c
 *
 * General management API for SH-4 integrated Store Queues
 *
 * Copyright (C) 2001 - 2006  Paul Mundt
 * Copyright (C) 2001, 2002  M. R. Brown
 */

// Dependencies supplied by the surrounding kernel translation unit.

#[repr(C)]
struct sq_mapping {
    name: *const core::ffi::c_char,
    sq_addr: usize,
    addr: usize,
    size: u32,
    next: *mut sq_mapping,
}

static mut SQ_MAPPING_LIST: *mut sq_mapping = core::ptr::null_mut();
static mut SQ_MAPPING_LOCK: core::ffi::c_ulong = 0;
static mut SQ_CACHE: *mut core::ffi::c_void = core::ptr::null_mut();
static mut SQ_BITMAP: *mut core::ffi::c_ulong = core::ptr::null_mut();

#[inline]
unsafe fn store_queue_barrier() {
    let _ = __raw_readl(P4SEG_STORE_QUE);
    __raw_writel(0, P4SEG_STORE_QUE + 0);
    __raw_writel(0, P4SEG_STORE_QUE + 8);
}

pub unsafe extern "C" fn sq_flush_range(mut start: usize, mut len: u32) {
    let mut sq = start as *mut usize;
    while { len >>= 5; len != 0 } {
        len -= 1;
        prefetchw(sq);
        sq = sq.add(8);
    }
    store_queue_barrier();
}

unsafe fn sq_mapping_list_add(map: *mut sq_mapping) {
    spin_lock_irq(&raw mut SQ_MAPPING_LOCK);
    let mut p = &raw mut SQ_MAPPING_LIST;
    let mut tmp;
    while {
        tmp = *p;
        !tmp.is_null()
    } {
        p = &raw mut (*tmp).next;
    }
    (*map).next = tmp;
    *p = map;
    spin_unlock_irq(&raw mut SQ_MAPPING_LOCK);
}

unsafe fn sq_mapping_list_del(map: *mut sq_mapping) {
    spin_lock_irq(&raw mut SQ_MAPPING_LOCK);
    let mut p = &raw mut SQ_MAPPING_LIST;
    loop {
        let tmp = *p;
        if tmp.is_null() { break; }
        if tmp == map {
            *p = (*tmp).next;
            break;
        }
        p = &raw mut (*tmp).next;
    }
    spin_unlock_irq(&raw mut SQ_MAPPING_LOCK);
}

unsafe fn __sq_remap(map: *mut sq_mapping, prot: pgprot_t) -> i32 {
    #[cfg(CONFIG_MMU)]
    {
        let vma = __get_vm_area_caller((*map).size as usize, VM_IOREMAP, (*map).sq_addr,
            SQ_ADDRMAX, __builtin_return_address(0));
        if vma.is_null() { return -ENOMEM; }
        (*vma).phys_addr = (*map).addr;
        if ioremap_page_range((*vma).addr as usize, (*vma).addr as usize + (*map).size as usize,
                              (*vma).phys_addr, prot) != 0 {
            vunmap((*vma).addr);
            return -EAGAIN;
        }
    }
    #[cfg(not(CONFIG_MMU))]
    {
        __raw_writel(((((*map).addr >> 26) << 2) & 0x1c) as u32, SQ_QACR0);
        __raw_writel(((((*map).addr >> 26) << 2) & 0x1c) as u32, SQ_QACR1);
    }
    0
}

pub unsafe extern "C" fn sq_remap(mut phys: usize, mut size: u32,
                                   name: *const core::ffi::c_char, prot: pgprot_t) -> usize {
    let end = phys.wrapping_add(size as usize).wrapping_sub(1);
    if size == 0 || end < phys { return (-EINVAL) as usize; }
    if phys < virt_to_phys(high_memory) { return (-EINVAL) as usize; }
    phys &= PAGE_MASK;
    size = (PAGE_ALIGN(end.wrapping_add(1)) - phys) as u32;
    let map = kmem_cache_alloc(SQ_CACHE, GFP_KERNEL) as *mut sq_mapping;
    if map.is_null() { return (-ENOMEM) as usize; }
    (*map).addr = phys;
    (*map).size = size;
    (*map).name = name;
    let page = bitmap_find_free_region(SQ_BITMAP, 0x04000000usize >> PAGE_SHIFT,
                                       get_order(size as usize));
    if page < 0 {
        kmem_cache_free(SQ_CACHE, map as *mut core::ffi::c_void);
        return (-ENOSPC) as usize;
    }
    (*map).sq_addr = P4SEG_STORE_QUE + ((page as usize) << PAGE_SHIFT);
    let ret = __sq_remap(map, prot);
    if ret != 0 {
        kmem_cache_free(SQ_CACHE, map as *mut core::ffi::c_void);
        return ret as usize;
    }
    let _psz = (size as usize + PAGE_SIZE - 1) >> PAGE_SHIFT;
    pr_info("sqremap: %15s  [%4d page%s]  va 0x%08lx   pa 0x%08lx\n",
        if !name.is_null() { name } else { b"???\0".as_ptr() as *const _ },
        _psz, if _psz == 1 { b" \0".as_ptr() } else { b"s\0".as_ptr() },
        (*map).sq_addr, (*map).addr);
    sq_mapping_list_add(map);
    (*map).sq_addr
}

pub unsafe extern "C" fn sq_unmap(vaddr: usize) {
    let mut p = &raw mut SQ_MAPPING_LIST;
    let mut map = core::ptr::null_mut();
    loop {
        map = *p;
        if map.is_null() || (*map).sq_addr == vaddr { break; }
        p = &raw mut (*map).next;
    }
    if map.is_null() {
        printk("%s: bad store queue address 0x%08lx\n", __func__, vaddr);
        return;
    }
    let page = ((*map).sq_addr - P4SEG_STORE_QUE) >> PAGE_SHIFT;
    bitmap_release_region(SQ_BITMAP, page, get_order((*map).size as usize));
    #[cfg(CONFIG_MMU)]
    {
        let vma = remove_vm_area(((*map).sq_addr & PAGE_MASK) as *mut core::ffi::c_void);
        if vma.is_null() {
            printk(KERN_ERR "%s: bad address 0x%08lx\n", __func__, (*map).sq_addr);
            return;
        }
    }
    sq_mapping_list_del(map);
    kmem_cache_free(SQ_CACHE, map as *mut core::ffi::c_void);
}

#[repr(C)]
struct sq_sysfs_attr {
    attr: attribute,
    show: Option<unsafe extern "C" fn(*mut core::ffi::c_char) -> isize>,
    store: Option<unsafe extern "C" fn(*const core::ffi::c_char, usize) -> isize>,
}

static mut SQ_KOBJECT: [*mut kobject; NR_CPUS] = [core::ptr::null_mut(); NR_CPUS];

unsafe extern "C" fn sq_sysfs_show(_kobj: *mut kobject, attr: *mut attribute,
                                    buf: *mut core::ffi::c_char) -> isize {
    let sattr = container_of_sq_attr(attr);
    match (*sattr).show { Some(show) => show(buf), None => (-EIO) as isize }
}

unsafe extern "C" fn sq_sysfs_store(_kobj: *mut kobject, attr: *mut attribute,
                                     buf: *const core::ffi::c_char, count: usize) -> isize {
    let sattr = container_of_sq_attr(attr);
    match (*sattr).store { Some(store) => store(buf, count), None => (-EIO) as isize }
}

unsafe extern "C" fn mapping_show(buf: *mut core::ffi::c_char) -> isize {
    let mut list = &raw mut SQ_MAPPING_LIST;
    let mut p = buf;
    while !(*list).is_null() {
        let entry = *list;
        p = p.add(sprintf_mapping(p, (*entry).sq_addr, (*entry).sq_addr + (*entry).size as usize,
                                   (*entry).addr, (*entry).name));
        list = &raw mut (*entry).next;
    }
    p.offset_from(buf) as isize
}

unsafe extern "C" fn mapping_store(buf: *const core::ffi::c_char, count: usize) -> isize {
    let mut base = 0usize;
    let mut len = 0usize;
    sscanf_mapping(buf, &mut base, &mut len);
    if base == 0 { return (-EIO) as isize; }
    if len != 0 {
        let ret = sq_remap(base, len as u32, b"Userspace\0".as_ptr() as *const _, PAGE_SHARED);
        if (ret as isize) < 0 { return ret as isize; }
    } else { sq_unmap(base); }
    count as isize
}

extern "C" {
    fn container_of_sq_attr(attr: *mut attribute) -> *mut sq_sysfs_attr;
    fn sprintf_mapping(buf: *mut core::ffi::c_char, va: usize, end: usize, pa: usize,
                       name: *const core::ffi::c_char) -> usize;
    fn sscanf_mapping(buf: *const core::ffi::c_char, base: *mut usize, len: *mut usize);
}

extern "C" {
    fn __raw_readl(addr: usize) -> u32;
    fn __raw_writel(value: u32, addr: usize);
    fn prefetchw(addr: *mut usize);
    fn spin_lock_irq(lock: *mut core::ffi::c_ulong);
    fn spin_unlock_irq(lock: *mut core::ffi::c_ulong);
    fn virt_to_phys(addr: usize) -> usize;
    fn kmem_cache_alloc(cache: *mut core::ffi::c_void, flags: usize) -> *mut core::ffi::c_void;
    fn kmem_cache_free(cache: *mut core::ffi::c_void, obj: *mut core::ffi::c_void);
    fn bitmap_find_free_region(bitmap: *mut core::ffi::c_ulong, bits: usize, order: usize) -> i32;
    fn bitmap_release_region(bitmap: *mut core::ffi::c_ulong, pos: usize, order: usize);
    fn get_order(size: usize) -> usize;
    fn pr_info(fmt: *const core::ffi::c_char, ...);
    fn printk(fmt: *const core::ffi::c_char, ...);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
