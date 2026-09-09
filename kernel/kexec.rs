// SPDX-License-Identifier: GPL-2.0-only
/*
 * kexec.c - kexec_load system call
 * Copyright (C) 2002-2004 Eric Biederman  <ebiederm@xmission.com>
 */

// pr_fmt(fmt) KBUILD_MODNAME ": " fmt
// C dependencies: linux/capability.h, linux/mm.h, linux/file.h,
// linux/security.h, linux/kexec.h, linux/mutex.h, linux/list.h,
// linux/syscalls.h, linux/vmalloc.h, linux/slab.h, and kexec_internal.h.

use core::ffi::c_void;

extern "C" {
    fn do_kimage_alloc_init() -> *mut kimage;
    fn phys_to_boot_phys(value: u64) -> u64;
    fn sanity_check_segment_list(image: *mut kimage) -> i32;
    fn kimage_alloc_control_pages(image: *mut kimage, order: i32) -> *mut page;
    fn get_order(size: usize) -> i32;
    fn kimage_free_page_list(list: *mut list_head);
    fn kexec_trylock() -> bool;
    fn kexec_unlock();
    fn arch_kexec_unprotect_crashkres();
    fn kimage_free(image: *mut kimage);
    fn arch_crash_hotplug_support(image: *mut kimage, flags: u64) -> bool;
    fn machine_kexec_prepare(image: *mut kimage) -> i32;
    fn kimage_crash_copy_vmcoreinfo(image: *mut kimage) -> i32;
    fn kimage_load_segment(image: *mut kimage, index: u64) -> i32;
    fn kimage_terminate(image: *mut kimage);
    fn machine_kexec_post_load(image: *mut kimage) -> i32;
    fn arch_kexec_protect_crashkres();
    fn kexec_load_permitted(image_type: i32) -> bool;
    fn security_kernel_load_data(kind: i32, contents: bool) -> i32;
    fn security_locked_down(reason: i32) -> i32;
    fn memdup_array_user(src: *const c_void, n: u64, size: usize) -> *mut kexec_segment;
    fn kmalloc_objs<T>(example: T, n: u64) -> *mut kexec_segment;
    fn copy_from_user(to: *mut c_void, from: *const c_void, n: usize) -> u64;
    fn compat_ptr(value: u32) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
}

#[repr(C)]
struct kimage {
    start: u64,
    nr_segments: u64,
    segment: [kexec_segment; KEXEC_SEGMENT_MAX as usize],
    control_page: u64,
    type_: i32,
    control_code_page: *mut page,
    swap_page: *mut page,
    preserve_context: i32,
    hotplug_support: i32,
    control_pages: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct kexec_segment { buf: *mut c_void, bufsz: u64, mem: u64, memsz: u64 }
#[repr(C)] struct compat_kexec_segment { buf: u32, bufsz: u32, mem: u32, memsz: u32 }
#[repr(C)] struct page;
#[repr(C)] struct list_head { next: *mut list_head, prev: *mut list_head }

extern "C" {
    static mut kexec_image: *mut kimage;
    static mut kexec_crash_image: *mut kimage;
    static mut crashk_res: resource;
}
#[repr(C)] struct resource { start: u64, end: u64 }

const KEXEC_ON_CRASH: u64 = 0x00000001;
const KEXEC_PRESERVE_CONTEXT: u64 = 0x00000002;
const KEXEC_TYPE_CRASH: i32 = 1;
const KEXEC_TYPE_DEFAULT: i32 = 0;
const KEXEC_CONTROL_PAGE_SIZE: usize = 4096;
const KEXEC_SEGMENT_MAX: u64 = 16;
const KEXEC_FLAGS: u64 = 0xffff_ffff;
const KEXEC_ARCH_MASK: u64 = 0xffff_0000;
const KEXEC_ARCH: u64 = 0;
const KEXEC_ARCH_DEFAULT: u64 = 0;
const LOADING_KEXEC_IMAGE: i32 = 0;
const LOCKDOWN_KEXEC: i32 = 0;

unsafe fn kimage_alloc_init(rimage: *mut *mut kimage, entry: u64, nr_segments: u64,
                            segments: *mut kexec_segment, flags: u64) -> i32 {
    let mut ret: i32;
    let image: *mut kimage;
    let kexec_on_panic = (flags & KEXEC_ON_CRASH) != 0;
    if kexec_on_panic && (entry < phys_to_boot_phys(crashk_res.start) ||
        entry > phys_to_boot_phys(crashk_res.end)) { return -99; }
    image = do_kimage_alloc_init();
    if image.is_null() { return -12; }
    (*image).start = entry;
    (*image).nr_segments = nr_segments;
    core::ptr::copy_nonoverlapping(segments, (*image).segment.as_mut_ptr(), nr_segments as usize);
    if kexec_on_panic { (*image).control_page = crashk_res.start; (*image).type_ = KEXEC_TYPE_CRASH; }
    ret = sanity_check_segment_list(image);
    if ret != 0 { kfree(image.cast()); return ret; }
    ret = -12;
    (*image).control_code_page = kimage_alloc_control_pages(image, get_order(KEXEC_CONTROL_PAGE_SIZE));
    if (*image).control_code_page.is_null() { kfree(image.cast()); return ret; }
    if !kexec_on_panic {
        (*image).swap_page = kimage_alloc_control_pages(image, 0);
        if (*image).swap_page.is_null() { kimage_free_page_list(&mut (*image).control_pages); kfree(image.cast()); return ret; }
    }
    *rimage = image; 0
}

unsafe fn do_kexec_load(entry: u64, nr_segments: u64, segments: *mut kexec_segment, flags: u64) -> i32 {
    if !kexec_trylock() { return -16; }
    let dest_image: *mut *mut kimage = if (flags & KEXEC_ON_CRASH) != 0 { &mut kexec_crash_image } else { &mut kexec_image };
    if nr_segments == 0 { kimage_free(core::ptr::replace(dest_image, core::ptr::null_mut())); kexec_unlock(); return 0; }
    if (flags & KEXEC_ON_CRASH) != 0 { kimage_free(core::ptr::replace(&mut kexec_crash_image, core::ptr::null_mut())); }
    let mut image: *mut kimage = core::ptr::null_mut();
    let mut ret = kimage_alloc_init(&mut image, entry, nr_segments, segments, flags);
    if ret != 0 { kexec_unlock(); return ret; }
    if (flags & KEXEC_PRESERVE_CONTEXT) != 0 { (*image).preserve_context = 1; }
    ret = machine_kexec_prepare(image);
    if ret == 0 { ret = kimage_crash_copy_vmcoreinfo(image); }
    let mut i = 0; while ret == 0 && i < nr_segments { ret = kimage_load_segment(image, i); i += 1; }
    if ret == 0 { kimage_terminate(image); ret = machine_kexec_post_load(image); }
    if ret == 0 { image = core::ptr::replace(dest_image, image); }
    kimage_free(image); kexec_unlock(); ret
}

unsafe fn kexec_load_check(nr_segments: u64, flags: u64) -> i32 {
    let image_type = if (flags & KEXEC_ON_CRASH) != 0 { KEXEC_TYPE_CRASH } else { KEXEC_TYPE_DEFAULT };
    if !kexec_load_permitted(image_type) { return -1; }
    let mut result = security_kernel_load_data(LOADING_KEXEC_IMAGE, false); if result < 0 { return result; }
    result = security_locked_down(LOCKDOWN_KEXEC); if result != 0 { return result; }
    if (flags & KEXEC_FLAGS) != (flags & !KEXEC_ARCH_MASK) || nr_segments > KEXEC_SEGMENT_MAX { return -22; }
    0
}

#[no_mangle]
pub unsafe extern "C" fn kexec_load(entry: u64, nr_segments: u64, segments: *mut kexec_segment, flags: u64) -> i32 {
    let result = kexec_load_check(nr_segments, flags); if result != 0 { return result; }
    if (flags & KEXEC_ARCH_MASK) != KEXEC_ARCH && (flags & KEXEC_ARCH_MASK) != KEXEC_ARCH_DEFAULT { return -22; }
    let ksegments = memdup_array_user(segments.cast(), nr_segments, core::mem::size_of::<kexec_segment>());
    if ksegments.is_null() { return -14; }
    let result = do_kexec_load(entry, nr_segments, ksegments, flags); kfree(ksegments.cast()); result
}

#[cfg(CONFIG_COMPAT)]
pub unsafe extern "C" fn compat_kexec_load(entry: u32, nr_segments: u32, segments: *mut compat_kexec_segment, flags: u32) -> i32 {
    let mut result = kexec_load_check(nr_segments as u64, flags as u64); if result != 0 { return result; }
    if (flags as u64 & KEXEC_ARCH_MASK) == KEXEC_ARCH_DEFAULT { return -22; }
    let ksegments = kmalloc_objs(kexec_segment { buf: core::ptr::null_mut(), bufsz: 0, mem: 0, memsz: 0 }, nr_segments as u64);
    if ksegments.is_null() { return -12; }
    let mut input = compat_kexec_segment { buf: 0, bufsz: 0, mem: 0, memsz: 0 };
    for i in 0..nr_segments as usize {
        result = copy_from_user((&mut input).cast(), segments.add(i).cast(), core::mem::size_of::<compat_kexec_segment>()) as i32;
        if result != 0 { kfree(ksegments.cast()); return result; }
        (*ksegments.add(i)).buf = compat_ptr(input.buf); (*ksegments.add(i)).bufsz = input.bufsz as u64;
        (*ksegments.add(i)).mem = input.mem as u64; (*ksegments.add(i)).memsz = input.memsz as u64;
    }
    result = do_kexec_load(entry as u64, nr_segments as u64, ksegments, flags as u64); kfree(ksegments.cast()); result
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
