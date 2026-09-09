// SPDX-License-Identifier: GPL-2.0
/*
 * This file contains generic KASAN specific error reporting code.
 *
 * Copyright (c) 2014 Samsung Electronics Co., Ltd.
 * Author: Andrey Ryabinin <ryabinin.a.a@gmail.com>
 *
 * Some code borrowed from https://github.com/xairy/kasan-prototype by
 *        Andrey Konovalov <andreyknvl@gmail.com>
 */

// Linux kernel dependencies are supplied by the surrounding translation.

pub unsafe fn kasan_find_first_bad_addr(addr: *const core::ffi::c_void, size: usize) -> *const core::ffi::c_void {
    let mut p = addr as *const u8;
    if !addr_has_metadata(p as *const core::ffi::c_void) {
        return p as *const core::ffi::c_void;
    }
    while (p as usize) < (addr as usize).wrapping_add(size)
        && *(kasan_mem_to_shadow(p as *const core::ffi::c_void) as *const u8) == 0
    {
        p = p.add(KASAN_GRANULE_SIZE as usize);
    }
    p as *const core::ffi::c_void
}

pub unsafe fn kasan_get_alloc_size(object: *mut core::ffi::c_void, cache: *mut kmem_cache) -> usize {
    let mut size = 0usize;
    let mut shadow = kasan_mem_to_shadow(object) as *const u8;
    while size < (*cache).object_size as usize {
        if *shadow == 0 { size += KASAN_GRANULE_SIZE as usize; }
        else if *shadow >= 1 && *shadow <= KASAN_GRANULE_SIZE - 1 { return size + *shadow as usize; }
        else { return size; }
        shadow = shadow.add(1);
    }
    (*cache).object_size as usize
}

unsafe fn get_shadow_bug_type(info: *mut kasan_report_info) -> *const core::ffi::c_char {
    let mut bug_type = b"unknown-crash\0".as_ptr() as *const core::ffi::c_char;
    let mut shadow_addr = kasan_mem_to_shadow((*info).first_bad_addr) as *const u8;
    if *shadow_addr > 0 && *shadow_addr <= KASAN_GRANULE_SIZE - 1 { shadow_addr = shadow_addr.add(1); }
    match *shadow_addr {
        0..=KASAN_GRANULE_SIZE - 1 => bug_type = b"out-of-bounds\0".as_ptr() as _,
        KASAN_PAGE_REDZONE | KASAN_SLAB_REDZONE => bug_type = b"slab-out-of-bounds\0".as_ptr() as _,
        KASAN_GLOBAL_REDZONE => bug_type = b"global-out-of-bounds\0".as_ptr() as _,
        KASAN_STACK_LEFT | KASAN_STACK_MID | KASAN_STACK_RIGHT | KASAN_STACK_PARTIAL => bug_type = b"stack-out-of-bounds\0".as_ptr() as _,
        KASAN_PAGE_FREE => bug_type = b"use-after-free\0".as_ptr() as _,
        KASAN_SLAB_FREE | KASAN_SLAB_FREE_META => bug_type = b"slab-use-after-free\0".as_ptr() as _,
        KASAN_ALLOCA_LEFT | KASAN_ALLOCA_RIGHT => bug_type = b"alloca-out-of-bounds\0".as_ptr() as _,
        KASAN_VMALLOC_INVALID => bug_type = b"vmalloc-out-of-bounds\0".as_ptr() as _,
        _ => {}
    }
    bug_type
}

unsafe fn get_wild_bug_type(info: *mut kasan_report_info) -> *const core::ffi::c_char {
    if (*info).access_addr as usize <= PAGE_SIZE { b"null-ptr-deref\0".as_ptr() as _ }
    else if (*info).access_addr as usize < TASK_SIZE { b"user-memory-access\0".as_ptr() as _ }
    else { b"wild-memory-access\0".as_ptr() as _ }
}

unsafe fn get_bug_type(info: *mut kasan_report_info) -> *const core::ffi::c_char {
    if ((*info).access_addr as usize).wrapping_add((*info).access_size as usize) < (*info).access_addr as usize { return b"out-of-bounds\0".as_ptr() as _; }
    if addr_has_metadata((*info).access_addr) { get_shadow_bug_type(info) } else { get_wild_bug_type(info) }
}

pub unsafe fn kasan_complete_mode_report_info(info: *mut kasan_report_info) {
    if (*info).bug_type.is_null() { (*info).bug_type = get_bug_type(info); }
    if (*info).cache.is_null() || (*info).object.is_null() { return; }
    let alloc_meta = kasan_get_alloc_meta((*info).cache, (*info).object);
    if !alloc_meta.is_null() { core::ptr::copy_nonoverlapping(&(*alloc_meta).alloc_track, &mut (*info).alloc_track, 1); }
    if *(kasan_mem_to_shadow((*info).object) as *const u8) == KASAN_SLAB_FREE_META {
        let free_meta = kasan_get_free_meta((*info).cache, (*info).object);
        core::ptr::copy_nonoverlapping(&(*free_meta).free_track, &mut (*info).free_track, 1);
    }
}

pub unsafe fn kasan_metadata_fetch_row(buffer: *mut i8, row: *mut core::ffi::c_void) {
    core::ptr::copy_nonoverlapping(kasan_mem_to_shadow(row) as *const i8, buffer, META_BYTES_PER_ROW as usize);
}

pub unsafe fn kasan_print_aux_stacks(cache: *mut kmem_cache, object: *const core::ffi::c_void) {
    let alloc_meta = kasan_get_alloc_meta(cache, object);
    if alloc_meta.is_null() { return; }
    if (*alloc_meta).aux_stack[0] != 0 { pr_err!("Last potentially related work creation:\n"); stack_depot_print((*alloc_meta).aux_stack[0]); pr_err!("\n"); }
    if (*alloc_meta).aux_stack[1] != 0 { pr_err!("Second to last potentially related work creation:\n"); stack_depot_print((*alloc_meta).aux_stack[1]); pr_err!("\n"); }
}

// CONFIG_KASAN_STACK conditional section.
unsafe fn tokenize_frame_descr(frame_descr: &mut *const i8, token: *mut i8, max_tok_len: usize, value: *mut usize) -> bool {
    let mut sep = strchr(*frame_descr, b' ' as i32);
    if sep.is_null() { sep = (*frame_descr).add(strlen(*frame_descr)); }
    if !token.is_null() {
        let tok_len = sep.offset_from(*frame_descr) as usize;
        if tok_len + 1 > max_tok_len { pr_err!("internal error: frame description too long: %s\n", *frame_descr); return false; }
        strscpy(token, *frame_descr, (tok_len + 1) as isize);
    }
    *frame_descr = sep.add(1);
    if !value.is_null() && kstrtoul(token, 10, value) != 0 { pr_err!("internal error: not a valid number: %s\n", token); return false; }
    true
}

unsafe fn print_decoded_frame_descr(mut frame_descr: *const i8) {
    let mut token = [0i8; 64];
    let mut num_objects = 0usize;
    if !tokenize_frame_descr(&mut frame_descr, token.as_mut_ptr(), token.len(), &mut num_objects) { return; }
    pr_err!("\n");
    pr_err!("This frame has {} {}:\n", num_objects, if num_objects == 1 { "object" } else { "objects" });
    while num_objects != 0 {
        num_objects -= 1;
        let mut offset = 0usize; let mut size = 0usize;
        if !tokenize_frame_descr(&mut frame_descr, token.as_mut_ptr(), token.len(), &mut offset) { return; }
        if !tokenize_frame_descr(&mut frame_descr, token.as_mut_ptr(), token.len(), &mut size) { return; }
        if !tokenize_frame_descr(&mut frame_descr, core::ptr::null_mut(), 0, core::ptr::null_mut()) { return; }
        if !tokenize_frame_descr(&mut frame_descr, token.as_mut_ptr(), token.len(), core::ptr::null_mut()) { return; }
        strreplace(token.as_mut_ptr(), b':' as i32, b'\0' as i32);
        pr_err!(" [{}, {}) '%s'", offset, offset + size, token.as_mut_ptr());
    }
}

unsafe fn get_address_stack_frame_info(addr: *const core::ffi::c_void, offset: *mut usize, frame_descr: *mut *const i8, frame_pc: *mut *const core::ffi::c_void) -> bool {
    let aligned_addr = round_down(addr as usize, core::mem::size_of::<usize>());
    let mut mem_ptr = round_down(aligned_addr, KASAN_GRANULE_SIZE as usize);
    let shadow_bottom = kasan_mem_to_shadow(end_of_stack(current)) as *const u8;
    let mut shadow_ptr = kasan_mem_to_shadow(aligned_addr as *mut _) as *const u8;
    while shadow_ptr >= shadow_bottom && *shadow_ptr != KASAN_STACK_LEFT { shadow_ptr = shadow_ptr.sub(1); mem_ptr -= KASAN_GRANULE_SIZE as usize; }
    while shadow_ptr >= shadow_bottom && *shadow_ptr == KASAN_STACK_LEFT { shadow_ptr = shadow_ptr.sub(1); mem_ptr -= KASAN_GRANULE_SIZE as usize; }
    if shadow_ptr < shadow_bottom { return false; }
    let frame = (mem_ptr + KASAN_GRANULE_SIZE as usize) as *const usize;
    if *frame != KASAN_CURRENT_STACK_FRAME_MAGIC { pr_err!("internal error: frame has invalid marker: {}\n", *frame); return false; }
    *offset = (addr as usize) - frame as usize; *frame_descr = *frame.add(1) as *const i8; *frame_pc = *frame.add(2) as *const _; true
}

pub unsafe fn kasan_print_address_stack_frame(addr: *const core::ffi::c_void) {
    if WARN_ON(!object_is_on_stack(addr)) { return; }
    pr_err!("The buggy address belongs to stack of task %s/%d\n", current.comm, task_pid_nr(current));
    let mut offset = 0usize; let mut frame_descr = core::ptr::null(); let mut frame_pc = core::ptr::null();
    if !get_address_stack_frame_info(addr, &mut offset, &mut frame_descr, &mut frame_pc) { return; }
    pr_err!(" and is located at offset {} in frame:\n", offset); pr_err!(" %pS\n", frame_pc);
    if !frame_descr.is_null() { print_decoded_frame_descr(frame_descr); }
}

pub unsafe fn __asan_report_load1_noabort(addr: *mut core::ffi::c_void) { kasan_report(addr, 1, false, _RET_IP_); }
pub unsafe fn __asan_report_load2_noabort(addr: *mut core::ffi::c_void) { kasan_report(addr, 2, false, _RET_IP_); }
pub unsafe fn __asan_report_load4_noabort(addr: *mut core::ffi::c_void) { kasan_report(addr, 4, false, _RET_IP_); }
pub unsafe fn __asan_report_load8_noabort(addr: *mut core::ffi::c_void) { kasan_report(addr, 8, false, _RET_IP_); }
pub unsafe fn __asan_report_load16_noabort(addr: *mut core::ffi::c_void) { kasan_report(addr, 16, false, _RET_IP_); }
pub unsafe fn __asan_report_store1_noabort(addr: *mut core::ffi::c_void) { kasan_report(addr, 1, true, _RET_IP_); }
pub unsafe fn __asan_report_store2_noabort(addr: *mut core::ffi::c_void) { kasan_report(addr, 2, true, _RET_IP_); }
pub unsafe fn __asan_report_store4_noabort(addr: *mut core::ffi::c_void) { kasan_report(addr, 4, true, _RET_IP_); }
pub unsafe fn __asan_report_store8_noabort(addr: *mut core::ffi::c_void) { kasan_report(addr, 8, true, _RET_IP_); }
pub unsafe fn __asan_report_store16_noabort(addr: *mut core::ffi::c_void) { kasan_report(addr, 16, true, _RET_IP_); }
pub unsafe fn __asan_report_load_n_noabort(addr: *mut core::ffi::c_void, size: isize) { kasan_report(addr, size, false, _RET_IP_); }
pub unsafe fn __asan_report_store_n_noabort(addr: *mut core::ffi::c_void, size: isize) { kasan_report(addr, size, true, _RET_IP_); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
