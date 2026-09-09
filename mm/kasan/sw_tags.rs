// SPDX-License-Identifier: GPL-2.0
/*
 * This file contains core software tag-based KASAN code.
 *
 * Copyright (c) 2018 Google, Inc.
 * Author: Andrey Konovalov <andreyknvl@google.com>
 */

// Linux kernel headers and the symbols they provide are supplied by other files.

static mut prng_state: u32 = 0;

pub unsafe fn kasan_init_sw_tags() {
    // `for_each_possible_cpu(cpu)` and `per_cpu(prng_state, cpu)` are Linux
    // per-CPU primitives; their implementation is supplied by the kernel.
    for_each_possible_cpu(|cpu: i32| {
        per_cpu_write_prng_state(cpu, get_cycles() as u32);
    });

    kasan_init_tags();
    kasan_enable();

    pr_info(
        "KernelAddressSanitizer initialized (sw-tags, stacktrace=%s)\n",
        str_on_off(kasan_stack_collection_enabled()),
    );
}

/*
 * If a preemption happens between this_cpu_read and this_cpu_write, the only
 * side effect is that we'll give a few allocated in different contexts objects
 * the same tag. Since tag-based KASAN is meant to be used a probabilistic
 * bug-detection debug feature, this doesn't have significant negative impact.
 *
 * Ideally the tags use strong randomness to prevent any attempts to predict
 * them during explicit exploit attempts. But strong randomness is expensive,
 * and we did an intentional trade-off to use a PRNG. This non-atomic RMW
 * sequence has in fact positive effect, since interrupts that randomly skew
 * PRNG at unpredictable points do only good.
 */
pub unsafe fn kasan_random_tag() -> u8 {
    let mut state = this_cpu_read_prng_state();

    state = 1664525u32.wrapping_mul(state).wrapping_add(1013904223u32);
    this_cpu_write_prng_state(state);

    (state % (KASAN_TAG_MAX + 1) as u32) as u8
}

pub unsafe fn kasan_check_range(
    addr: *const core::ffi::c_void,
    size: usize,
    write: bool,
    ret_ip: usize,
) -> bool {
    let tag: u8;
    let shadow_first: *mut u8;
    let shadow_last: *mut u8;
    let mut shadow: *mut u8;
    let untagged_addr: *mut core::ffi::c_void;

    if size == 0 {
        return true;
    }

    if (addr as usize).wrapping_add(size) < addr as usize {
        return !kasan_report(addr, size, write, ret_ip);
    }

    tag = get_tag(addr);

    /* Ignore accesses for pointers tagged with 0xff (native kernel pointer
     * tag) to suppress false positives caused by kmap. */
    if tag == KASAN_TAG_KERNEL {
        return true;
    }

    untagged_addr = kasan_reset_tag(addr);
    if !addr_has_metadata(untagged_addr) {
        return !kasan_report(addr, size, write, ret_ip);
    }
    shadow_first = kasan_mem_to_shadow(untagged_addr);
    shadow = shadow_first;
    shadow_last = kasan_mem_to_shadow(
        (untagged_addr as usize + size - 1) as *const core::ffi::c_void,
    );
    while (shadow as usize) <= shadow_last as usize {
        if core::ptr::read_volatile(shadow) != tag {
            return !kasan_report(addr, size, write, ret_ip);
        }
        shadow = (shadow as usize + 1) as *mut u8;
    }

    true
}

pub unsafe fn kasan_byte_accessible(addr: *const core::ffi::c_void) -> bool {
    let tag = get_tag(addr);
    let untagged_addr = kasan_reset_tag(addr);

    if !addr_has_metadata(untagged_addr) {
        return false;
    }

    let shadow_byte = core::ptr::read_volatile(kasan_mem_to_shadow(untagged_addr));
    tag == KASAN_TAG_KERNEL || tag == shadow_byte
}

pub unsafe fn __hwasan_load1_noabort(addr: *mut core::ffi::c_void) {
    kasan_check_range(addr, 1, false, 0);
}
pub unsafe fn __hwasan_store1_noabort(addr: *mut core::ffi::c_void) {
    kasan_check_range(addr, 1, true, 0);
}
pub unsafe fn __hwasan_load2_noabort(addr: *mut core::ffi::c_void) {
    kasan_check_range(addr, 2, false, 0);
}
pub unsafe fn __hwasan_store2_noabort(addr: *mut core::ffi::c_void) {
    kasan_check_range(addr, 2, true, 0);
}
pub unsafe fn __hwasan_load4_noabort(addr: *mut core::ffi::c_void) {
    kasan_check_range(addr, 4, false, 0);
}
pub unsafe fn __hwasan_store4_noabort(addr: *mut core::ffi::c_void) {
    kasan_check_range(addr, 4, true, 0);
}
pub unsafe fn __hwasan_load8_noabort(addr: *mut core::ffi::c_void) {
    kasan_check_range(addr, 8, false, 0);
}
pub unsafe fn __hwasan_store8_noabort(addr: *mut core::ffi::c_void) {
    kasan_check_range(addr, 8, true, 0);
}
pub unsafe fn __hwasan_load16_noabort(addr: *mut core::ffi::c_void) {
    kasan_check_range(addr, 16, false, 0);
}
pub unsafe fn __hwasan_store16_noabort(addr: *mut core::ffi::c_void) {
    kasan_check_range(addr, 16, true, 0);
}

pub unsafe fn __hwasan_loadN_noabort(addr: *mut core::ffi::c_void, size: isize) {
    kasan_check_range(addr, size as usize, false, 0);
}

pub unsafe fn __hwasan_storeN_noabort(addr: *mut core::ffi::c_void, size: isize) {
    kasan_check_range(addr, size as usize, true, 0);
}

pub unsafe fn __hwasan_tag_memory(
    addr: *mut core::ffi::c_void,
    tag: u8,
    size: isize,
) {
    kasan_poison(addr, size, tag, false);
}

pub unsafe fn kasan_tag_mismatch(addr: *mut core::ffi::c_void, access_info: usize, ret_ip: usize) {
    kasan_report(addr, 1usize << (access_info & 0xf), (access_info & 0x10) != 0, ret_ip);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
