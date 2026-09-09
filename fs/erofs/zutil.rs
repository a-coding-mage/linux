// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2018 HUAWEI, Inc.
 *             https://www.huawei.com/
 * Copyright (C) 2024 Alibaba Cloud
 */

// Declarations supplied by internal kernel headers and other translation units

#[repr(C)]
pub struct z_erofs_gbuf {
    pub lock: spinlock_t,
    pub ptr: *mut core::ffi::c_void,
    pub pages: *mut *mut page,
    pub nrpages: core::ffi::c_uint,
}

static mut z_erofs_gbufpool: *mut z_erofs_gbuf = core::ptr::null_mut();
static mut z_erofs_rsvbuf: *mut z_erofs_gbuf = core::ptr::null_mut();
static mut z_erofs_gbuf_count: core::ffi::c_uint = 0;
static mut z_erofs_gbuf_nrpages: core::ffi::c_uint = 0;
static mut z_erofs_rsv_nrpages: core::ffi::c_uint = 0;

pub static mut erofs_global_shrink_cnt: atomic_long_t = atomic_long_t { counter: 0 };

static mut erofs_sb_list_lock: spinlock_t = spinlock_t { raw_lock: 0 };
static mut erofs_sb_list: list_head = list_head { next: core::ptr::null_mut(), prev: core::ptr::null_mut() };
static mut shrinker_run_no: core::ffi::c_uint = 0;
static mut erofs_shrinker_info: *mut shrinker = core::ptr::null_mut();

unsafe fn z_erofs_gbuf_id() -> core::ffi::c_uint {
    raw_smp_processor_id() % z_erofs_gbuf_count
}

pub unsafe fn z_erofs_get_gbuf(requiredpages: core::ffi::c_uint) -> *mut core::ffi::c_void {
    migrate_disable();
    let gbuf = &mut *z_erofs_gbufpool.add(z_erofs_gbuf_id() as usize);
    spin_lock(&mut gbuf.lock);
    // check if the buffer is too small
    if requiredpages > gbuf.nrpages {
        spin_unlock(&mut gbuf.lock);
        migrate_enable();
        // (for sparse checker) pretend gbuf->lock is still taken
        return core::ptr::null_mut();
    }
    gbuf.ptr
}

pub unsafe fn z_erofs_put_gbuf(ptr: *mut core::ffi::c_void) {
    let gbuf = &mut *z_erofs_gbufpool.add(z_erofs_gbuf_id() as usize);
    DBG_BUGON(gbuf.ptr != ptr);
    spin_unlock(&mut gbuf.lock);
    migrate_enable();
}

pub unsafe fn z_erofs_gbuf_growsize(nrpages: core::ffi::c_uint) -> core::ffi::c_int {
    static mut gbuf_resize_mutex: mutex = mutex { count: 0 };
    let mut tmp_pages: *mut *mut page = core::ptr::null_mut();
    let mut gbuf: *mut z_erofs_gbuf = core::ptr::null_mut();
    let mut ptr: *mut core::ffi::c_void;
    let mut old_ptr: *mut core::ffi::c_void;
    let mut last: core::ffi::c_int;
    let mut i: core::ffi::c_int = 0;
    let mut j: core::ffi::c_int;

    mutex_lock(&mut gbuf_resize_mutex);
    // avoid shrinking gbufs, since no idea how many fses rely on
    if nrpages <= z_erofs_gbuf_nrpages {
        mutex_unlock(&mut gbuf_resize_mutex);
        return 0;
    }

    while i < z_erofs_gbuf_count as core::ffi::c_int {
        gbuf = z_erofs_gbufpool.add(i as usize);
        if (*gbuf).nrpages >= nrpages {
            i += 1;
            continue;
        }
        tmp_pages = kzalloc_objs(nrpages);
        if tmp_pages.is_null() { break; }

        j = 0;
        while j < (*gbuf).nrpages as core::ffi::c_int {
            *tmp_pages.add(j as usize) = *(*gbuf).pages.add(j as usize);
            j += 1;
        }
        loop {
            last = j;
            j = alloc_pages_bulk(GFP_KERNEL, nrpages, tmp_pages);
            if last == j { break; }
            if j == nrpages as core::ffi::c_int { break; }
        }
        if j != nrpages as core::ffi::c_int { break; }

        ptr = vmap(tmp_pages, nrpages, VM_MAP, PAGE_KERNEL);
        if ptr.is_null() { break; }

        spin_lock(&mut (*gbuf).lock);
        kfree((*gbuf).pages as *mut core::ffi::c_void);
        old_ptr = (*gbuf).ptr;
        (*gbuf).pages = tmp_pages;
        (*gbuf).ptr = ptr;
        (*gbuf).nrpages = nrpages;
        spin_unlock(&mut (*gbuf).lock);
        vunmap(old_ptr);
        tmp_pages = core::ptr::null_mut();
        i += 1;
    }
    z_erofs_gbuf_nrpages = nrpages;
    if !tmp_pages.is_null() {
        j = 0;
        while j < nrpages as core::ffi::c_int {
            let p = *tmp_pages.add(j as usize);
            if !p.is_null() && (j >= (*gbuf).nrpages as core::ffi::c_int || p != *(*gbuf).pages.add(j as usize)) {
                __free_page(p);
            }
            j += 1;
        }
        kfree(tmp_pages as *mut core::ffi::c_void);
    }
    mutex_unlock(&mut gbuf_resize_mutex);
    if i < z_erofs_gbuf_count as core::ffi::c_int { -ENOMEM } else { 0 }
}

pub unsafe fn z_erofs_gbuf_init() -> core::ffi::c_int {
    let mut total = num_possible_cpus();
    if z_erofs_gbuf_count != 0 { total = core::cmp::min(z_erofs_gbuf_count, total); }
    z_erofs_gbuf_count = total;
    // The last (special) global buffer is the reserved buffer
    total += (z_erofs_rsv_nrpages != 0) as core::ffi::c_uint;
    z_erofs_gbufpool = kzalloc_objs(total);
    if z_erofs_gbufpool.is_null() { return -ENOMEM; }
    if z_erofs_rsv_nrpages != 0 {
        z_erofs_rsvbuf = z_erofs_gbufpool.add((total - 1) as usize);
        (*z_erofs_rsvbuf).pages = kzalloc_objs(z_erofs_rsv_nrpages);
        if (*z_erofs_rsvbuf).pages.is_null() { z_erofs_rsvbuf = core::ptr::null_mut(); z_erofs_rsv_nrpages = 0; }
    }
    for i in 0..total { spin_lock_init(&mut (*z_erofs_gbufpool.add(i as usize)).lock); }
    0
}

pub unsafe fn z_erofs_gbuf_exit() {
    let mut i = 0;
    while i < z_erofs_gbuf_count + (!z_erofs_rsvbuf.is_null() as core::ffi::c_uint) {
        let gbuf = &mut *z_erofs_gbufpool.add(i as usize);
        if !gbuf.ptr.is_null() { vunmap(gbuf.ptr); gbuf.ptr = core::ptr::null_mut(); }
        if !gbuf.pages.is_null() {
            for j in 0..gbuf.nrpages { if !(*gbuf.pages.add(j as usize)).is_null() { put_page(*gbuf.pages.add(j as usize)); } }
            kfree(gbuf.pages as *mut core::ffi::c_void); gbuf.pages = core::ptr::null_mut();
        }
        i += 1;
    }
    kfree(z_erofs_gbufpool as *mut core::ffi::c_void);
}

pub unsafe fn __erofs_allocpage(pagepool: *mut *mut page, gfp: gfp_t, tryrsv: bool) -> *mut page {
    let mut p = *pagepool;
    if !p.is_null() { *pagepool = page_private(p) as *mut page; }
    else if tryrsv && !z_erofs_rsvbuf.is_null() && (*z_erofs_rsvbuf).nrpages != 0 {
        spin_lock(&mut (*z_erofs_rsvbuf).lock);
        if (*z_erofs_rsvbuf).nrpages != 0 { (*z_erofs_rsvbuf).nrpages -= 1; p = *(*z_erofs_rsvbuf).pages.add((*z_erofs_rsvbuf).nrpages as usize); }
        spin_unlock(&mut (*z_erofs_rsvbuf).lock);
    }
    if p.is_null() { p = alloc_page(gfp); }
    DBG_BUGON(!p.is_null() && page_ref_count(p) != 1); p
}

pub unsafe fn erofs_release_pages(pagepool: *mut *mut page) {
    while !(*pagepool).is_null() {
        let p = *pagepool; *pagepool = page_private(p) as *mut page;
        if !z_erofs_rsvbuf.is_null() && (*z_erofs_rsvbuf).nrpages < z_erofs_rsv_nrpages {
            spin_lock(&mut (*z_erofs_rsvbuf).lock);
            if (*z_erofs_rsvbuf).nrpages < z_erofs_rsv_nrpages { *(*z_erofs_rsvbuf).pages.add((*z_erofs_rsvbuf).nrpages as usize) = p; (*z_erofs_rsvbuf).nrpages += 1; spin_unlock(&mut (*z_erofs_rsvbuf).lock); continue; }
            spin_unlock(&mut (*z_erofs_rsvbuf).lock);
        }
        put_page(p);
    }
}

pub unsafe fn erofs_shrinker_register(sb: *mut super_block) {
    let sbi = EROFS_SB(sb); mutex_init(&mut (*sbi).umount_mutex); spin_lock(&mut erofs_sb_list_lock); list_add(&mut (*sbi).list, &mut erofs_sb_list); spin_unlock(&mut erofs_sb_list_lock);
}

pub unsafe fn erofs_shrinker_unregister(sb: *mut super_block) {
    let sbi = EROFS_SB(sb); mutex_lock(&mut (*sbi).umount_mutex);
    while !xa_empty(&mut (*sbi).managed_pslots) { z_erofs_shrink_scan(sbi, !0usize); cond_resched(); }
    spin_lock(&mut erofs_sb_list_lock); list_del(&mut (*sbi).list); spin_unlock(&mut erofs_sb_list_lock); mutex_unlock(&mut (*sbi).umount_mutex);
}

unsafe fn erofs_shrink_count(_shrink: *mut shrinker, _sc: *mut shrink_control) -> usize {
    let n = atomic_long_read(&mut erofs_global_shrink_cnt); if n != 0 { n as usize } else { SHRINK_EMPTY }
}

unsafe fn erofs_shrink_scan(_shrink: *mut shrinker, sc: *mut shrink_control) -> usize {
    let nr = (*sc).nr_to_scan; let mut run_no; let mut freed = 0usize;
    spin_lock(&mut erofs_sb_list_lock); loop { shrinker_run_no = shrinker_run_no.wrapping_add(1); run_no = shrinker_run_no; if run_no != 0 { break; } }
    let mut p = erofs_sb_list.next;
    while p != &mut erofs_sb_list as *mut list_head {
        let sbi = list_entry(p, erofs_sb_info, list);
        if (*sbi).shrinker_run_no == run_no { break; }
        if !mutex_trylock(&mut (*sbi).umount_mutex) { p = (*p).next; continue; }
        spin_unlock(&mut erofs_sb_list_lock); (*sbi).shrinker_run_no = run_no; freed += z_erofs_shrink_scan(sbi, nr - freed); spin_lock(&mut erofs_sb_list_lock); p = (*p).next; list_move_tail(&mut (*sbi).list, &mut erofs_sb_list); mutex_unlock(&mut (*sbi).umount_mutex); if freed >= nr { break; }
    }
    spin_unlock(&mut erofs_sb_list_lock); freed
}

pub unsafe fn erofs_init_shrinker() -> core::ffi::c_int {
    erofs_shrinker_info = shrinker_alloc(0, b"erofs-shrinker\0".as_ptr() as *const core::ffi::c_char); if erofs_shrinker_info.is_null() { return -ENOMEM; }
    (*erofs_shrinker_info).count_objects = Some(erofs_shrink_count); (*erofs_shrinker_info).scan_objects = Some(erofs_shrink_scan); shrinker_register(erofs_shrinker_info); 0
}

pub unsafe fn erofs_exit_shrinker() { shrinker_free(erofs_shrinker_info); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
