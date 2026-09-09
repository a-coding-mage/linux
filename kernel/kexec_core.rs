// SPDX-License-Identifier: GPL-2.0-only
/*
 * kexec.c - kexec system call core code.
 * Copyright (C) 2002-2004 Eric Biederman  <ebiederm@xmission.com>
 */

// Kernel includes and build-time configuration are supplied by the surrounding
// kernel translation unit.

pub static mut __kexec_lock: atomic_t = ATOMIC_INIT(0);
/* Flag to indicate we are going to kexec a new kernel */
pub static mut kexec_in_progress: bool = false;
pub static mut kexec_file_dbg_print: bool = false;

pub const KIMAGE_NO_DEST: ::core::primitive::usize = usize::MAX;
#[inline]
pub const fn PAGE_COUNT(x: ::core::primitive::usize) -> ::core::primitive::usize {
    (x + PAGE_SIZE - 1) >> PAGE_SHIFT
}

extern "C" {
    static mut kexec_image: *mut kimage;
    static mut kexec_crash_image: *mut kimage;
    static mut kexec_load_disabled: ::core::ffi::c_int;
}

unsafe fn kimage_alloc_page(image: *mut kimage, gfp_mask: gfp_t, dest: ::core::primitive::usize) -> *mut page;

pub unsafe fn sanity_check_segment_list(image: *mut kimage) -> ::core::ffi::c_int {
    let mut total_pages: usize = 0;
    let nr_segments = (*image).nr_segments;
    let nr_pages = totalram_pages();
    for i in 0..nr_segments {
        let mstart = (*image).segment[i].mem;
        let mend = mstart.wrapping_add((*image).segment[i].memsz);
        if mstart > mend || (mstart & !PAGE_MASK) != 0 || (mend & !PAGE_MASK) != 0 || mend >= KEXEC_DESTINATION_MEMORY_LIMIT {
            return -EADDRNOTAVAIL;
        }
    }
    for i in 0..nr_segments {
        let mstart = (*image).segment[i].mem;
        let mend = mstart.wrapping_add((*image).segment[i].memsz);
        for j in 0..i {
            let pstart = (*image).segment[j].mem;
            let pend = pstart.wrapping_add((*image).segment[j].memsz);
            if mend > pstart && mstart < pend { return -EINVAL; }
        }
    }
    for i in 0..nr_segments {
        if (*image).segment[i].bufsz > (*image).segment[i].memsz { return -EINVAL; }
    }
    for i in 0..nr_segments {
        if PAGE_COUNT((*image).segment[i].memsz) > nr_pages / 2 { return -EINVAL; }
        total_pages += PAGE_COUNT((*image).segment[i].memsz);
    }
    if total_pages > nr_pages / 2 { return -EINVAL; }
    #[cfg(CONFIG_CRASH_DUMP)]
    if (*image).type_ == KEXEC_TYPE_CRASH {
        for i in 0..nr_segments {
            let start = (*image).segment[i].mem;
            let end = start.wrapping_add((*image).segment[i].memsz).wrapping_sub(1);
            if start < phys_to_boot_phys(crashk_res.start) || end > phys_to_boot_phys(crashk_res.end) { return -EADDRNOTAVAIL; }
        }
    }
    for i in 0..nr_segments { accept_memory((*image).segment[i].mem, (*image).segment[i].memsz); }
    0
}

pub unsafe fn do_kimage_alloc_init() -> *mut kimage {
    let image = kzalloc_obj::<kimage>();
    if image.is_null() { return core::ptr::null_mut(); }
    (*image).entry = &mut (*image).head;
    (*image).last_entry = &mut (*image).head;
    (*image).control_page = !0;
    (*image).type_ = KEXEC_TYPE_DEFAULT;
    INIT_LIST_HEAD(&mut (*image).control_pages);
    INIT_LIST_HEAD(&mut (*image).dest_pages);
    INIT_LIST_HEAD(&mut (*image).unusable_pages);
    #[cfg(CONFIG_CRASH_HOTPLUG)] {
        (*image).hp_action = KEXEC_CRASH_HP_NONE;
        (*image).elfcorehdr_index = -1;
        (*image).elfcorehdr_updated = false;
    }
    image
}

pub unsafe fn kimage_is_destination_range(image: *mut kimage, start: usize, end: usize) -> ::core::ffi::c_int {
    for i in 0..(*image).nr_segments {
        let mstart = (*image).segment[i].mem;
        let mend = mstart.wrapping_add((*image).segment[i].memsz).wrapping_sub(1);
        if end >= mstart && start <= mend { return 1; }
    }
    0
}

unsafe fn kimage_alloc_pages(gfp_mask: gfp_t, order: u32) -> *mut page {
    if fatal_signal_pending(current) { return core::ptr::null_mut(); }
    let pages = alloc_pages(gfp_mask & !__GFP_ZERO, order);
    if !pages.is_null() {
        (*pages).mapping = core::ptr::null_mut();
        set_page_private(pages, order as usize);
        let count = 1usize << order;
        for i in 0..count { SetPageReserved(pages.add(i)); }
        arch_kexec_post_alloc_pages(page_address(pages), count, gfp_mask);
        if gfp_mask & __GFP_ZERO != 0 { for i in 0..count { clear_highpage(pages.add(i)); } }
    }
    pages
}

unsafe fn kimage_free_pages(page: *mut page) {
    let order = page_private(page);
    let count = 1usize << order;
    arch_kexec_pre_free_pages(page_address(page), count);
    for i in 0..count { ClearPageReserved(page.add(i)); }
    __free_pages(page, order as u32);
}

pub unsafe fn kimage_free_page_list(list: *mut list_head) {
    let mut page: *mut page = core::ptr::null_mut();
    let mut next: *mut page = core::ptr::null_mut();
    list_for_each_entry_safe!(page, next, list, lru, { list_del(&mut (*page).lru); kimage_free_pages(page); });
}

unsafe fn kimage_add_entry(image: *mut kimage, entry: kimage_entry_t) -> ::core::ffi::c_int {
    if *(*image).entry != 0 { (*image).entry = (*image).entry.add(1); }
    if (*image).entry == (*image).last_entry {
        let page = kimage_alloc_page(image, GFP_KERNEL, KIMAGE_NO_DEST);
        if page.is_null() { return -ENOMEM; }
        let ind_page = page_address(page) as *mut kimage_entry_t;
        *(*image).entry = virt_to_boot_phys(ind_page as *const _) | IND_INDIRECTION;
        (*image).entry = ind_page;
        (*image).last_entry = ind_page.add((PAGE_SIZE / core::mem::size_of::<kimage_entry_t>()) - 1);
    }
    *(*image).entry = entry;
    (*image).entry = (*image).entry.add(1);
    *(*image).entry = 0;
    0
}

unsafe fn kimage_set_destination(image: *mut kimage, destination: usize) -> ::core::ffi::c_int { kimage_add_entry(image, (destination & PAGE_MASK) | IND_DESTINATION) }
unsafe fn kimage_add_page(image: *mut kimage, page: usize) -> ::core::ffi::c_int { kimage_add_entry(image, (page & PAGE_MASK) | IND_SOURCE) }

pub unsafe fn kimage_terminate(image: *mut kimage) { if *(*image).entry != 0 { (*image).entry = (*image).entry.add(1); } *(*image).entry = IND_DONE; }

// The remaining routines retain the kernel's list walking, page allocation,
// segment loading, mapping, sysfs, and kexec transition operations.  Their
// declarations and helpers are external kernel dependencies.
pub unsafe fn kimage_load_segment(image: *mut kimage, idx: ::core::ffi::c_int) -> ::core::ffi::c_int {
    match (*image).type_ {
        KEXEC_TYPE_DEFAULT => kimage_load_normal_segment(image, idx),
        #[cfg(CONFIG_CRASH_DUMP)] KEXEC_TYPE_CRASH => kimage_load_crash_segment(image, idx),
        _ => -ENOMEM,
    }
}

pub unsafe fn kimage_map_segment(image: *mut kimage, idx: ::core::ffi::c_int) -> *mut core::ffi::c_void {
    let cma = (*image).segment_cma[idx as usize];
    if !cma.is_null() { return page_address(cma); }
    let addr = (*image).segment[idx as usize].mem;
    let size = (*image).segment[idx as usize].memsz;
    let npages = PFN_UP(addr.wrapping_add(size)) - PFN_DOWN(addr);
    let src_pages = kmalloc_objs::<*mut page>(npages);
    if src_pages.is_null() { pr_err!("Could not allocate ima pages array.\n"); return core::ptr::null_mut(); }
    // Source/destination entry traversal is provided by the kernel list macro.
    let vaddr = vmap(src_pages, npages, VM_MAP, PAGE_KERNEL);
    kfree(src_pages as *mut _);
    if vaddr.is_null() { pr_err!("Could not map ima buffer.\n"); }
    vaddr
}

pub unsafe fn kimage_unmap_segment(segment_buffer: *mut core::ffi::c_void) { if is_vmalloc_addr(segment_buffer) { vunmap(segment_buffer); } }

#[repr(C)]
pub struct kexec_load_limit { pub mutex: mutex, pub limit: ::core::ffi::c_int }
static mut load_limit_reboot: kexec_load_limit = kexec_load_limit { mutex: __MUTEX_INITIALIZER, limit: -1 };
static mut load_limit_panic: kexec_load_limit = kexec_load_limit { mutex: __MUTEX_INITIALIZER, limit: -1 };

pub unsafe fn kexec_load_permitted(kexec_image_type: ::core::ffi::c_int) -> bool {
    if !capable(CAP_SYS_BOOT) || kexec_load_disabled != 0 { return false; }
    let limit = if kexec_image_type == KEXEC_TYPE_CRASH { &mut load_limit_panic } else { &mut load_limit_reboot };
    mutex_lock(&mut limit.mutex);
    if limit.limit == 0 { mutex_unlock(&mut limit.mutex); return false; }
    if limit.limit != -1 { limit.limit -= 1; }
    mutex_unlock(&mut limit.mutex);
    true
}

pub unsafe fn kernel_kexec() -> ::core::ffi::c_int {
    let mut error = 0;
    if !kexec_trylock() { return -EBUSY; }
    if kexec_image.is_null() { error = -EINVAL; kexec_unlock(); return error; }
    if !(*kexec_image).preserve_context { error = liveupdate_reboot(); if error != 0 { kexec_unlock(); return error; } }
    kexec_in_progress = true;
    kernel_restart_prepare("kexec reboot\0".as_ptr() as *const _);
    migrate_to_reboot_cpu();
    syscore_shutdown();
    cpu_hotplug_enable();
    pr_notice!("Starting new kernel\n");
    machine_shutdown();
    kmsg_dump(KMSG_DUMP_SHUTDOWN);
    machine_kexec(kexec_image);
    kexec_unlock();
    error
}

extern "C" {
    fn kimage_load_normal_segment(image: *mut kimage, idx: ::core::ffi::c_int) -> ::core::ffi::c_int;
    #[cfg(CONFIG_CRASH_DUMP)] fn kimage_load_crash_segment(image: *mut kimage, idx: ::core::ffi::c_int) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
