// SPDX-License-Identifier: GPL-2.0
/*
 * KMSAN runtime library.
 *
 * Copyright (C) 2017-2022 Google LLC
 * Author: Alexander Potapenko <glider@google.com>
 */

// C headers and build-time kernel dependencies are supplied externally.

use core::ffi::c_void;

extern "C" {
    static mut kmsan_enabled: bool;
}

// DEFINE_PER_CPU(struct kmsan_ctx, kmsan_percpu_ctx);

#[allow(non_snake_case)]
pub unsafe extern "C" fn kmsan_internal_task_create(task: *mut task_struct) {
    let ctx = &mut (*task).kmsan_ctx as *mut kmsan_ctx;
    let info = current_thread_info();

    __memset(ctx as *mut c_void, 0, core::mem::size_of::<kmsan_ctx>());
    kmsan_internal_unpoison_memory(
        info as *mut c_void,
        core::mem::size_of::<thread_info>(),
        false,
    );
}

pub unsafe extern "C" fn kmsan_internal_poison_memory(
    address: *mut c_void,
    size: usize,
    flags: gfp_t,
    poison_flags: u32,
) {
    let extra_bits = kmsan_extra_bits(0, poison_flags & KMSAN_POISON_FREE);
    let checked = (poison_flags & KMSAN_POISON_CHECK) != 0;
    let handle = kmsan_save_stack_with_flags(flags, extra_bits);
    kmsan_internal_set_shadow_origin(address, size, -1, handle, checked);
}

pub unsafe extern "C" fn kmsan_internal_unpoison_memory(
    address: *mut c_void,
    size: usize,
    checked: bool,
) {
    kmsan_internal_set_shadow_origin(address, size, 0, 0, checked);
}

pub unsafe extern "C" fn kmsan_save_stack_with_flags(
    flags: gfp_t,
    extra: u32,
) -> depot_stack_handle_t {
    let mut entries = [0UL; KMSAN_STACK_DEPTH];
    let nr_entries = stack_trace_save(entries.as_mut_ptr(), KMSAN_STACK_DEPTH, 0);
    let handle = stack_depot_save(entries.as_mut_ptr(), nr_entries, flags);
    stack_depot_set_extra_bits(handle, extra)
}

/* Copy the metadata following the memmove() behavior. */
pub unsafe extern "C" fn kmsan_internal_memmove_metadata(
    dst: *mut c_void,
    src: *mut c_void,
    n: usize,
) {
    let mut prev_old_origin: depot_stack_handle_t = 0;
    let mut prev_new_origin: depot_stack_handle_t = 0;
    let mut old_origin: depot_stack_handle_t = 0;
    let mut new_origin: depot_stack_handle_t = 0;

    let shadow_dst = kmsan_get_metadata(dst, KMSAN_META_SHADOW) as *mut u8;
    if shadow_dst.is_null() { return; }
    KMSAN_WARN_ON(!kmsan_metadata_is_contiguous(dst, n));
    let align_shadow_dst = ((shadow_dst as u64) & !(KMSAN_ORIGIN_SIZE as u64 - 1)) as *mut u32;

    let shadow_src = kmsan_get_metadata(src, KMSAN_META_SHADOW) as *mut u8;
    if shadow_src.is_null() {
        kmsan_internal_unpoison_memory(dst, n, false);
        return;
    }
    KMSAN_WARN_ON(!kmsan_metadata_is_contiguous(src, n));

    let origin_dst = kmsan_get_metadata(dst, KMSAN_META_ORIGIN) as *mut depot_stack_handle_t;
    let origin_src = kmsan_get_metadata(src, KMSAN_META_ORIGIN) as *mut depot_stack_handle_t;
    KMSAN_WARN_ON(origin_dst.is_null() || origin_src.is_null());

    let backwards = (dst as usize) > (src as usize);
    let step: isize = if backwards { -1 } else { 1 };
    let mut iter: isize = if backwards { n as isize - 1 } else { 0 };
    let src_off = (src as u64) % KMSAN_ORIGIN_SIZE as u64;
    let dst_off = (dst as u64) % KMSAN_ORIGIN_SIZE as u64;

    for _ in 0..n {
        let oiter_src = ((iter as u64 + src_off) / KMSAN_ORIGIN_SIZE as u64) as isize;
        let oiter_dst = ((iter as u64 + dst_off) / KMSAN_ORIGIN_SIZE as u64) as isize;
        let index = iter as usize;
        if *shadow_src.add(index) == 0 {
            *shadow_dst.add(index) = 0;
            if *align_shadow_dst.offset(oiter_dst) == 0 { *origin_dst.offset(oiter_dst) = 0; }
            iter += step;
            continue;
        }
        *shadow_dst.add(index) = *shadow_src.add(index);
        old_origin = *origin_src.offset(oiter_src);
        if old_origin == prev_old_origin { new_origin = prev_new_origin; }
        else {
            new_origin = kmsan_internal_chain_origin(old_origin);
            if new_origin == 0 { new_origin = old_origin; }
        }
        *origin_dst.offset(oiter_dst) = new_origin;
        prev_new_origin = new_origin;
        prev_old_origin = old_origin;
        iter += step;
    }
}

pub unsafe extern "C" fn kmsan_internal_chain_origin(id: depot_stack_handle_t) -> depot_stack_handle_t {
    if id == 0 { return id; }
    // BUILD_BUG_ON((1 << STACK_DEPOT_EXTRA_BITS) <= (KMSAN_MAX_ORIGIN_DEPTH << 1));
    let old_extra = stack_depot_get_extra_bits(id);
    let mut depth = kmsan_depth_from_eb(old_extra);
    let uaf = kmsan_uaf_from_eb(old_extra);
    if depth == KMSAN_MAX_ORIGIN_DEPTH { return id; }
    depth += 1;
    let extra_bits = kmsan_extra_bits(depth, uaf);
    let mut entries = [0UL; 3];
    entries[0] = KMSAN_CHAIN_MAGIC_ORIGIN;
    entries[1] = kmsan_save_stack_with_flags(__GFP_HIGH, 0);
    entries[2] = id;
    kmsan_internal_unpoison_memory(entries.as_mut_ptr() as *mut c_void, core::mem::size_of_val(&entries), false);
    let handle = stack_depot_save(entries.as_mut_ptr(), 3, __GFP_HIGH);
    stack_depot_set_extra_bits(handle, extra_bits)
}

pub unsafe extern "C" fn kmsan_internal_set_shadow_origin(
    addr: *mut c_void, size: usize, b: i32, origin: u32, checked: bool,
) {
    let mut address = addr as u64;
    let shadow_start = kmsan_get_metadata(addr, KMSAN_META_SHADOW) as *mut u8;
    KMSAN_WARN_ON(!kmsan_metadata_is_contiguous(addr, size));
    if shadow_start.is_null() {
        if checked { pr_err("%s: not memsetting bytes because the shadow is NULL\n"); KMSAN_WARN_ON(true); }
        return;
    }
    __memset(shadow_start as *mut c_void, b, size);
    let (aligned_shadow, pad) = if IS_ALIGNED(address, KMSAN_ORIGIN_SIZE) { (shadow_start, 0) }
        else { let p = address % KMSAN_ORIGIN_SIZE as u64; address -= p; (shadow_start.sub(p as usize), p as usize) };
    let size = ALIGN(size + pad, KMSAN_ORIGIN_SIZE as usize);
    let origin_start = kmsan_get_metadata(address as *mut c_void, KMSAN_META_ORIGIN) as *mut u32;
    for i in 0..(size / KMSAN_ORIGIN_SIZE as usize) { if origin != 0 || *aligned_shadow.add(i) == 0 { *origin_start.add(i) = origin; } }
}

pub unsafe extern "C" fn kmsan_vmalloc_to_page_or_null(vaddr: *mut c_void) -> *mut page {
    if !kmsan_internal_is_vmalloc_addr(vaddr) && !kmsan_internal_is_module_addr(vaddr) { return core::ptr::null_mut(); }
    let page = vmalloc_to_page(vaddr);
    if pfn_valid(page_to_pfn(page)) { page } else { core::ptr::null_mut() }
}

pub unsafe extern "C" fn kmsan_internal_check_memory(
    addr: *mut c_void, size: usize, user_addr: *const c_void, reason: i32,
) {
    let mut cur_origin: depot_stack_handle_t = 0;
    let mut new_origin: depot_stack_handle_t;
    let addr64 = addr as usize;
    let mut pos = 0usize;
    let mut cur_off_start: isize = -1;
    if size == 0 { return; }
    KMSAN_WARN_ON(!kmsan_metadata_is_contiguous(addr, size));
    while pos < size {
        let chunk_size = core::cmp::min(size - pos, PAGE_SIZE - ((addr64 + pos) % PAGE_SIZE));
        let shadow = kmsan_get_metadata((addr64 + pos) as *mut c_void, KMSAN_META_SHADOW) as *mut u8;
        if shadow.is_null() {
            if cur_origin != 0 { kmsan_report(cur_origin, addr, size, cur_off_start, pos as isize - 1, user_addr, reason); }
            cur_origin = 0; cur_off_start = -1; pos += chunk_size; continue;
        }
        for i in 0..chunk_size {
            if *shadow.add(i) == 0 {
                if cur_origin != 0 { kmsan_report(cur_origin, addr, size, cur_off_start, (pos + i) as isize - 1, user_addr, reason); }
                cur_origin = 0; cur_off_start = -1; continue;
            }
            let origin = kmsan_get_metadata((addr64 + pos + i) as *mut c_void, KMSAN_META_ORIGIN) as *mut depot_stack_handle_t;
            KMSAN_WARN_ON(origin.is_null());
            new_origin = *origin;
            if cur_origin != new_origin {
                if cur_origin != 0 { kmsan_report(cur_origin, addr, size, cur_off_start, (pos + i) as isize - 1, user_addr, reason); }
                cur_origin = new_origin; cur_off_start = (pos + i) as isize;
            }
        }
        pos += chunk_size;
    }
    KMSAN_WARN_ON(pos != size);
    if cur_origin != 0 { kmsan_report(cur_origin, addr, size, cur_off_start, pos as isize - 1, user_addr, reason); }
}

pub unsafe extern "C" fn kmsan_metadata_is_contiguous(addr: *mut c_void, size: usize) -> bool {
    if size == 0 { return true; }
    let cur_addr = addr as u64;
    if ((cur_addr + size as u64 - 1) & !(PAGE_SIZE as u64 - 1)) == (cur_addr & !(PAGE_SIZE as u64 - 1)) { return true; }
    let mut cur_shadow = kmsan_get_metadata(addr, false) as *mut u8;
    let mut cur_origin = kmsan_get_metadata(addr, true) as *mut u8;
    let all_untracked = cur_shadow.is_null();
    if all_untracked && !cur_origin.is_null() { goto_report(addr, cur_addr, cur_shadow, core::ptr::null_mut(), cur_origin, core::ptr::null_mut()); return false; }
    let mut next_addr = cur_addr + PAGE_SIZE as u64;
    while next_addr < cur_addr + size as u64 {
        let next_shadow = kmsan_get_metadata(next_addr as *mut c_void, false) as *mut u8;
        let next_origin = kmsan_get_metadata(next_addr as *mut c_void, true) as *mut u8;
        if all_untracked {
            if !next_shadow.is_null() || !next_origin.is_null() { goto_report(addr, cur_addr, cur_shadow, next_shadow, cur_origin, next_origin); return false; }
            next_addr += PAGE_SIZE as u64; continue;
        }
        if (cur_shadow as u64 != next_shadow as u64 - PAGE_SIZE as u64) || (cur_origin as u64 != next_origin as u64 - PAGE_SIZE as u64) {
            goto_report(addr, next_addr - PAGE_SIZE as u64, cur_shadow, next_shadow, cur_origin, next_origin); return false;
        }
        cur_shadow = next_shadow; cur_origin = next_origin; next_addr += PAGE_SIZE as u64;
    }
    true
}

unsafe fn goto_report(addr: *mut c_void, cur_addr: u64, cur_shadow: *mut u8, next_shadow: *mut u8, cur_origin: *mut u8, next_origin: *mut u8) {
    pr_err("%s: attempting to access two shadow page ranges.\n");
    pr_err("Access at %px.\n", addr);
    pr_err("Addresses belonging to different ranges: %px and %px\n", cur_addr as *mut c_void, (cur_addr + PAGE_SIZE as u64) as *mut c_void);
    pr_err("page[0].shadow: %px, page[1].shadow: %px\n", cur_shadow, next_shadow);
    pr_err("page[0].origin: %px, page[1].origin: %px\n", cur_origin, next_origin);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
