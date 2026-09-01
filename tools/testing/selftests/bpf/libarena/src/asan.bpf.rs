// SPDX-License-Identifier: LGPL-2.1 OR BSD-2-Clause
/* Copyright (c) 2026 Meta Platforms, Inc. and affiliates. */
/* Rust translation of includes:
 * #include <vmlinux.h>
 * #include <libarena/common.h>
 * #include <libarena/asan.h>
 */

const ASAN_READ: u32 = 0x0u32;
const ASAN_WRITE: u32 = 0x1u32;

/*
 * Address sanitizer (ASAN) for arena-based BPF programs, inspired
 * by KASAN.
 *
 * The API
 * -------
 *
 * The implementation includes two kinds of components: Implementation
 * of ASAN hooks injected by LLVM into the program, and API calls that
 * allocators use to mark memory as valid or invalid. The full list is:
 *
 * LLVM stubs:
 *
 * void __asan_{load, store}<size>(intptr_t addr)
 *	Checks whether an access is valid. All variations covered
 *	by check_region_inline().
 *
 * void __asan_{store, load}((intptr_t addr, ssize_t size)
 *
 * void __asan_report_{load, store}<size>(intptr_t addr)
 *	Report an access violation for the program. Used when LLVM
 *	uses direct code generation for shadow map checks.
 *
 * void *__asan_memcpy(void *d, const void *s, size_t n)
 * void *__asan_memmove(void *d, const void *s, size_t n)
 * void *__asan_memset(void *p, int c, size_t n)
 *	Hooks for ASAN instrumentation of the LLVM mem* builtins.
 *	Currently unimplemented just like the builtins themselves.
 *
 * API methods:
 *
 * asan_init()
 *	Initialize the ASAN map for the arena.
 *
 * asan_poison()
 *	Mark a region of memory as poisoned. Accessing poisoned memory
 *	causes asan_report() to fire. Invoked during free().
 *
 * asan_unpoison()
 *	Mark a region as unpoisoned after alloc().
 *
 * asan_shadow_set()
 *	Check a byte's validity directly.
 *
 * The Algorithm In Brief
 * ----------------------
 * Each group of 8 bytes is mapped to a "granule" in the shadow map. This
 * granule is the size of the byte and describes which bytes are valid.
 * Possible values are:
 *
 * 0: All bytes are valid. Makes checks in the middle of an allocated region
 * (most of them) fast.
 * (0, 7]: How many consecutive bytes are valid, starting from the lowest one.
 * The tradeoff is that we can't poison individual bytes in the middle of a
 * valid region.
 * [0x80, 0xff]: Special poison values, can be used to denote specific error
 * modes (e.g., recently freed vs uninitialized memory).
 *
 * The mapping between a memory location and its shadow is:
 * shadow_addr = shadow_base + (addr >> 3). We retain the 8:1 data:shadow
 * ratio of existing ASAN implementations as a compromise between tracking
 * granularity and space usage/scan overhead.
 */

/* The following items are the Rust translation of the BPF_ARENA_ASAN section. */

type s8 = i8;
type u32 = u32;
type u64 = u64;
type size_t = usize;
type ssize_t = isize;
type intptr_t = isize;

extern "C" {
    static mut zero: size_t;
    static mut can_loop: bool;
    static mut arena: core::ffi::c_void;

    static ASAN_GRANULE_MASK: u64;
    static ASAN_SHADOW_SHIFT: u32;
    static __PAGE_SIZE: u64;
    static NUMA_NO_NODE: i32;
    static EINVAL: i32;
    static ENOMEM: i32;
    static BPF_STDERR: i32;

    fn mem_to_shadow(addr: *mut core::ffi::c_void) -> *mut s8;
    fn bpf_arena_alloc_pages(
        arena: *mut core::ffi::c_void,
        addr: *mut core::ffi::c_void,
        pages: u64,
        numa_node: i32,
        flags: u64,
    ) -> *mut core::ffi::c_void;
    fn arena_stderr(fmt: *const u8, ...);
    fn bpf_stream_print_stack(stream: i32);
}

#[repr(C)]
pub struct asan_init_args {
    pub arena_globals_pages: u64,
    pub arena_all_pages: u64,
}

#[inline(always)]
unsafe fn likely(v: bool) -> bool {
    v
}

#[inline(always)]
unsafe fn unlikely(v: bool) -> bool {
    v
}

#[inline(always)]
unsafe fn ASAN_GRANULE(addr: *mut s8) -> s8 {
    ((addr as u64) & ASAN_GRANULE_MASK) as s8
}

const SHADOW_ALL_ZEROES: u64 = !0u64;

/*
 * Canary variable for ASAN violations. Set to the offending address.
 */
#[no_mangle]
pub static mut asan_violated: u64 = 0;

/*
 * Shadow map occupancy map.
 */
#[no_mangle]
pub static mut __asan_shadow_memory_dynamic_address: u64 = 0;

#[no_mangle]
pub static mut asan_reported: u32 = false as u32;
#[no_mangle]
pub static mut asan_inited: bool = false;

/*
 * Set during program load.
 */
#[no_mangle]
pub static mut asan_report_once: bool = false;

/*
 * BPF does not currently support the memset/memcpy/memcmp intrinsics.
 * For large sequential copies, or assignments of large data structures,
 * the frontend will generate an intrinsic that causes the BPF backend
 * to exit due to a missing implementation. Provide a simple implementation
 * just for memset to use it for poisoning/unpoisoning the map.
 */
#[no_mangle]
pub unsafe extern "C" fn asan_memset(dst: *mut s8, val: s8, size: size_t) -> i32 {
    let mut i: size_t = zero;

    while i < size && can_loop {
        *dst.add(i) = val;
        i += 1;
    }

    0
}

/* Validate a 1-byte access, always within a single byte. */
#[inline(always)]
unsafe fn memory_is_poisoned_1(addr: *mut s8) -> bool {
    let shadow_value: s8 = *(mem_to_shadow(addr as *mut core::ffi::c_void) as *mut s8);

    /* Byte is 0, access is valid. */
    if likely(shadow_value == 0) {
        return false;
    }

    /*
     * Byte is non-zero. Access is valid if granule offset in [0, shadow_value),
     * so the memory is poisoned if shadow_value is negative or smaller than
     * the granule's value.
     */

    ASAN_GRANULE(addr) >= shadow_value
}

/* Validate a 2- 4-, 8-byte access, shadow spans up to 2 bytes. */
#[inline(always)]
unsafe fn memory_is_poisoned_2_4_8(addr: *mut s8, size: u64) -> bool {
    let end: u64 = (addr as u64).wrapping_add(size).wrapping_sub(1);

    /*
     * Region fully within a single byte (addition didn't
     * overflow above ASAN_GRANULE).
     */
    if likely(ASAN_GRANULE(end as *mut s8) as u64 >= size.wrapping_sub(1)) {
        return memory_is_poisoned_1(end as *mut s8);
    }

    /*
     * Otherwise first byte must be fully unpoisoned, and second byte
     * must be unpoisoned up to the end of the accessed region.
     */

    *(mem_to_shadow(addr as *mut core::ffi::c_void) as *mut s8) != 0
        || memory_is_poisoned_1(end as *mut s8)
}

#[no_mangle]
pub unsafe extern "C" fn asan_shadow_set(addr: *mut core::ffi::c_void) -> bool {
    memory_is_poisoned_1(addr as *mut s8)
}

#[inline(always)]
unsafe fn first_nonzero_byte(mut addr: u64, mut size: size_t) -> u64 {
    while size != 0 && can_loop {
        if unlikely(*(addr as *mut s8) != 0) {
            return addr;
        }
        addr = addr.wrapping_add(1);
        size -= 1;
    }

    SHADOW_ALL_ZEROES
}

#[inline(always)]
unsafe fn memory_is_poisoned_n(addr: *mut s8, size: u64) -> bool {
    let ret: u64;
    let start: u64;
    let end: u64;

    /* Size of [start, end] is end - start + 1. */
    start = mem_to_shadow(addr as *mut core::ffi::c_void) as u64;
    end = mem_to_shadow(addr.add(size as usize).sub(1) as *mut core::ffi::c_void) as u64;

    ret = first_nonzero_byte(start, (end - start) as size_t + 1);
    if likely(ret == SHADOW_ALL_ZEROES) {
        return false;
    }

    unlikely(
        ret != end
            || ASAN_GRANULE(addr.add(size as usize).sub(1)) >= *(end as *mut s8),
    )
}

#[no_mangle]
pub unsafe extern "C" fn asan_report(addr: *mut s8, sz: size_t, flags: u32) -> i32 {
    let reported: u32 =
        core::intrinsics::atomic_cxchg(&mut asan_reported, false as u32, true as u32).0;

    /* Only report the first ASAN violation. */
    if reported != 0 && asan_report_once {
        return 0;
    }

    asan_violated = addr as u64;

    arena_stderr(
        b"Memory violation for address %p (0x%lx) for %s of size %ld\n\0".as_ptr(),
        addr,
        addr as u64,
        if (flags & ASAN_WRITE) != 0 {
            b"write\0".as_ptr()
        } else {
            b"read\0".as_ptr()
        },
        sz,
    );
    bpf_stream_print_stack(BPF_STDERR);

    0
}

#[inline(always)]
unsafe fn check_asan_args(addr: *mut s8, size: size_t, result: *mut bool) -> bool {
    let mut valid: bool = true;

    /* Size 0 accesses are valid even if the address is invalid. */
    if unlikely(size == 0) {
        *result = valid;
        return true;
    }

    /*
     * Wraparound is possible for values close to the the edge of the
     * 4GiB boundary of the arena (last valid address is 1UL << 32 - 1).
     *
     *
     * The wraparound detection below works for small sizes. check_asan_args is
     * always called from the builtin ASAN checks, so 1 <= size <= 64. Even
     * for storeN/loadN that we do not expect to encounter the intrinsics will
     * not have a large enough size that:
     *
     * - addr + size  > MAX_U32
     * - (u32)(addr + size) > (u32) addr
     *
     * which would defeat wraparound detection.
     */
    if unlikely((addr.add(size) as u64 as u32) < (addr as u64 as u32)) {
        valid = false;
        *result = valid;
        return true;
    }

    false
}

#[inline(always)]
unsafe fn check_region_inline(ptr: intptr_t, size: size_t, flags: u32) -> bool {
    let addr: *mut s8 = ptr as u64 as *mut s8;
    let is_poisoned: bool;
    let mut is_valid: bool = false;

    if check_asan_args(addr, size, &mut is_valid as *mut bool) {
        if !is_valid {
            asan_report(addr, size, flags);
        }
        return is_valid;
    }

    match size {
        1 => {
            is_poisoned = memory_is_poisoned_1(addr);
        }
        2 | 4 | 8 => {
            is_poisoned = memory_is_poisoned_2_4_8(addr, size as u64);
        }
        _ => {
            is_poisoned = memory_is_poisoned_n(addr, size as u64);
        }
    }

    if is_poisoned {
        asan_report(addr, size, flags);
        return false;
    }

    true
}

/*
 * __alias is not supported for BPF so define *__noabort() variants as wrappers.
 */
#[no_mangle]
pub unsafe extern "C" fn __asan_store1(addr: intptr_t) {
    check_region_inline(addr, 1, ASAN_WRITE);
}

#[no_mangle]
pub unsafe extern "C" fn __asan_store1_noabort(addr: intptr_t) {
    check_region_inline(addr, 1, ASAN_WRITE);
}

#[no_mangle]
pub unsafe extern "C" fn __asan_load1(addr: intptr_t) {
    check_region_inline(addr, 1, ASAN_READ);
}

#[no_mangle]
pub unsafe extern "C" fn __asan_load1_noabort(addr: intptr_t) {
    check_region_inline(addr, 1, ASAN_READ);
}

#[no_mangle]
pub unsafe extern "C" fn __asan_report_store1(addr: intptr_t) {
    asan_report(addr as *mut s8, 1, ASAN_WRITE);
}

#[no_mangle]
pub unsafe extern "C" fn __asan_report_store1_noabort(addr: intptr_t) {
    asan_report(addr as *mut s8, 1, ASAN_WRITE);
}

#[no_mangle]
pub unsafe extern "C" fn __asan_report_load1(addr: intptr_t) {
    asan_report(addr as *mut s8, 1, ASAN_READ);
}

#[no_mangle]
pub unsafe extern "C" fn __asan_report_load1_noabort(addr: intptr_t) {
    asan_report(addr as *mut s8, 1, ASAN_READ);
}

#[no_mangle]
pub unsafe extern "C" fn __asan_store2(addr: intptr_t) {
    check_region_inline(addr, 2, ASAN_WRITE);
}

#[no_mangle]
pub unsafe extern "C" fn __asan_store2_noabort(addr: intptr_t) {
    check_region_inline(addr, 2, ASAN_WRITE);
}

#[no_mangle]
pub unsafe extern "C" fn __asan_load2(addr: intptr_t) {
    check_region_inline(addr, 2, ASAN_READ);
}

#[no_mangle]
pub unsafe extern "C" fn __asan_load2_noabort(addr: intptr_t) {
    check_region_inline(addr, 2, ASAN_READ);
}

#[no_mangle]
pub unsafe extern "C" fn __asan_report_store2(addr: intptr_t) {
    asan_report(addr as *mut s8, 2, ASAN_WRITE);
}

#[no_mangle]
pub unsafe extern "C" fn __asan_report_store2_noabort(addr: intptr_t) {
    asan_report(addr as *mut s8, 2, ASAN_WRITE);
}

#[no_mangle]
pub unsafe extern "C" fn __asan_report_load2(addr: intptr_t) {
    asan_report(addr as *mut s8, 2, ASAN_READ);
}

#[no_mangle]
pub unsafe extern "C" fn __asan_report_load2_noabort(addr: intptr_t) {
    asan_report(addr as *mut s8, 2, ASAN_READ);
}

#[no_mangle]
pub unsafe extern "C" fn __asan_store4(addr: intptr_t) {
    check_region_inline(addr, 4, ASAN_WRITE);
}

#[no_mangle]
pub unsafe extern "C" fn __asan_store4_noabort(addr: intptr_t) {
    check_region_inline(addr, 4, ASAN_WRITE);
}

#[no_mangle]
pub unsafe extern "C" fn __asan_load4(addr: intptr_t) {
    check_region_inline(addr, 4, ASAN_READ);
}

#[no_mangle]
pub unsafe extern "C" fn __asan_load4_noabort(addr: intptr_t) {
    check_region_inline(addr, 4, ASAN_READ);
}

#[no_mangle]
pub unsafe extern "C" fn __asan_report_store4(addr: intptr_t) {
    asan_report(addr as *mut s8, 4, ASAN_WRITE);
}

#[no_mangle]
pub unsafe extern "C" fn __asan_report_store4_noabort(addr: intptr_t) {
    asan_report(addr as *mut s8, 4, ASAN_WRITE);
}

#[no_mangle]
pub unsafe extern "C" fn __asan_report_load4(addr: intptr_t) {
    asan_report(addr as *mut s8, 4, ASAN_READ);
}

#[no_mangle]
pub unsafe extern "C" fn __asan_report_load4_noabort(addr: intptr_t) {
    asan_report(addr as *mut s8, 4, ASAN_READ);
}

#[no_mangle]
pub unsafe extern "C" fn __asan_store8(addr: intptr_t) {
    check_region_inline(addr, 8, ASAN_WRITE);
}

#[no_mangle]
pub unsafe extern "C" fn __asan_store8_noabort(addr: intptr_t) {
    check_region_inline(addr, 8, ASAN_WRITE);
}

#[no_mangle]
pub unsafe extern "C" fn __asan_load8(addr: intptr_t) {
    check_region_inline(addr, 8, ASAN_READ);
}

#[no_mangle]
pub unsafe extern "C" fn __asan_load8_noabort(addr: intptr_t) {
    check_region_inline(addr, 8, ASAN_READ);
}

#[no_mangle]
pub unsafe extern "C" fn __asan_report_store8(addr: intptr_t) {
    asan_report(addr as *mut s8, 8, ASAN_WRITE);
}

#[no_mangle]
pub unsafe extern "C" fn __asan_report_store8_noabort(addr: intptr_t) {
    asan_report(addr as *mut s8, 8, ASAN_WRITE);
}

#[no_mangle]
pub unsafe extern "C" fn __asan_report_load8(addr: intptr_t) {
    asan_report(addr as *mut s8, 8, ASAN_READ);
}

#[no_mangle]
pub unsafe extern "C" fn __asan_report_load8_noabort(addr: intptr_t) {
    asan_report(addr as *mut s8, 8, ASAN_READ);
}

#[no_mangle]
pub unsafe extern "C" fn __asan_storeN(addr: intptr_t, size: ssize_t) {
    check_region_inline(addr, size as size_t, ASAN_WRITE);
}

#[no_mangle]
pub unsafe extern "C" fn __asan_storeN_noabort(addr: intptr_t, size: ssize_t) {
    check_region_inline(addr, size as size_t, ASAN_WRITE);
}

#[no_mangle]
pub unsafe extern "C" fn __asan_loadN(addr: intptr_t, size: ssize_t) {
    check_region_inline(addr, size as size_t, ASAN_READ);
}

#[no_mangle]
pub unsafe extern "C" fn __asan_loadN_noabort(addr: intptr_t, size: ssize_t) {
    check_region_inline(addr, size as size_t, ASAN_READ);
}

/*
 * We currently do not sanitize globals.
 */
#[no_mangle]
pub unsafe extern "C" fn __asan_register_globals(_globals: intptr_t, _n: size_t) {}

#[no_mangle]
pub unsafe extern "C" fn __asan_unregister_globals(_globals: intptr_t, _n: size_t) {}

/*
 * We do not currently have memcpy/memmove/memset intrinsics
 * in LLVM. Do not implement sanitization.
 */
#[no_mangle]
pub unsafe extern "C" fn __asan_memcpy(
    _d: *mut core::ffi::c_void,
    _s: *const core::ffi::c_void,
    _n: size_t,
) -> *mut core::ffi::c_void {
    arena_stderr(b"ASAN: Unexpected %s call\0".as_ptr(), b"__asan_memcpy\0".as_ptr());
    core::ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn __asan_memmove(
    _d: *mut core::ffi::c_void,
    _s: *const core::ffi::c_void,
    _n: size_t,
) -> *mut core::ffi::c_void {
    arena_stderr(
        b"ASAN: Unexpected %s call\0".as_ptr(),
        b"__asan_memmove\0".as_ptr(),
    );
    core::ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn __asan_memset(
    _p: *mut core::ffi::c_void,
    _c: i32,
    _n: size_t,
) -> *mut core::ffi::c_void {
    arena_stderr(b"ASAN: Unexpected %s call\0".as_ptr(), b"__asan_memset\0".as_ptr());
    core::ptr::null_mut()
}

/*
 * Poisoning code, used when we add more freed memory to the allocator by:
 * 	a) pulling memory from the arena segment using bpf_arena_alloc_pages()
 * 	b) freeing memory from application code
 */
#[no_mangle]
pub unsafe extern "C" fn asan_poison(
    addr: *mut core::ffi::c_void,
    val: s8,
    size: size_t,
) -> i32 {
    let shadow: *mut s8;
    let len: size_t;

    /*
     * Poisoning from a non-granule address makes no sense: We can only allocate
     * memory to the application that has a granule-aligned starting address,
     * and bpf_arena_alloc_pages returns page-aligned memory. A non-aligned
     * addr then implies we're freeing a different address than the one we
     * allocated.
     */
    if unlikely((addr as u64 & ASAN_GRANULE_MASK) != 0) {
        return -EINVAL;
    }

    /*
     * We cannot free an unaligned region because it'd be possible that we
     * cannot describe the resulting poisoning state of the granule in
     * the ASAN encoding.
     *
     * Every granule represents a region of memory that looks like the
     * following (P for poisoned bytes, C for clear):
     *
     * <Clear>  <Poisoned>
     * [ C C C ... P P ]
     *
     * The value of the granule's shadow map is the number of clear bytes in
     * it. We cannot represent granules with the following state:
     *
     * [ P P ... C C ... P P ]
     *
     * That would be possible if we could free unaligned regions, so prevent that.
     */
    if unlikely((size as u64 & ASAN_GRANULE_MASK) != 0) {
        return -EINVAL;
    }

    shadow = mem_to_shadow(addr);
    len = size >> ASAN_SHADOW_SHIFT;

    asan_memset(shadow, val, len);

    0
}

/*
 * Unpoisoning code for marking memory as valid during allocation calls.
 *
 * Very similar to asan_poison, except we need to round up instead of
 * down, then partially poison the last granule if necessary.
 *
 * Partial poisoning is useful for keeping the padding poisoned. Allocations
 * are granule-aligned, so we we're reserving granule-aligned sizes for the
 * allocation. However, we want to still treat accesses to the padding as
 * invalid. Partial poisoning takes care of that. Freeing and poisoning the
 * memory is still done in granule-aligned sizes and repoisons the already
 * poisoned padding.
 */
#[no_mangle]
pub unsafe extern "C" fn asan_unpoison(addr: *mut core::ffi::c_void, size: size_t) -> i32 {
    let partial: size_t = size & ASAN_GRANULE_MASK as size_t;
    let shadow: *mut s8;
    let len: size_t;

    /*
     * We cannot allocate in the middle of the granule. The ASAN shadow
     * map encoding only describes regions of memory where every granule
     * follows this format (P for poisoned, C for clear):
     *
     * <Clear>  <Poisoned>
     * [ C C C ... P P ]
     *
     * This is so we can use a single number in [0, ASAN_SHADOW_SCALE)
     * to represent the poison state of the granule.
     */
    if unlikely((addr as u64 & ASAN_GRANULE_MASK) != 0) {
        return -EINVAL;
    }

    shadow = mem_to_shadow(addr);
    len = size >> ASAN_SHADOW_SHIFT;

    asan_memset(shadow, 0, len);

    /*
     * If we are allocating a non-granule aligned region, we need to adjust
     * the last byte of the shadow map to list how many bytes in the granule
     * are unpoisoned. If the region is aligned, then the memset call above
     * was enough.
     */
    if partial != 0 {
        *shadow.add(len) = partial as s8;
    }

    0
}

/*
 * Initialize ASAN state when necessary. Triggered from userspace before
 * allocator startup.
 */
#[no_mangle]
pub unsafe extern "C" fn asan_init(args: *mut asan_init_args) -> i32 {
    let globals_pages: u64 = (*args).arena_globals_pages;
    let all_pages: u64 = (*args).arena_all_pages;
    let shadow_map: u64;
    let shadow_pgoff: u64;
    let mut shadow_pages: u64;

    if asan_inited {
        return 0;
    }

    /*
     * Round up the shadow map size to the nearest page.
     */
    shadow_pages = all_pages >> ASAN_SHADOW_SHIFT;
    if (all_pages & ((1u64 << ASAN_SHADOW_SHIFT) - 1)) != 0 {
        shadow_pages += 1;
    }

    if all_pages > (1u64 << 32) / __PAGE_SIZE {
        arena_stderr(b"error: arena size %lx too large\0".as_ptr(), all_pages);
        return -EINVAL;
    }

    if globals_pages > all_pages {
        arena_stderr(
            b"error: globals %lx do not fit in arena %lx\0".as_ptr(),
            globals_pages,
            all_pages,
        );
        return -EINVAL;
    }

    if globals_pages + shadow_pages >= all_pages {
        arena_stderr(
            b"error: globals %lx do not leave room for shadow map %lx (arena pages %lx)\0"
                .as_ptr(),
            globals_pages,
            shadow_pages,
            all_pages,
        );
        return -EINVAL;
    }

    shadow_pgoff = all_pages - shadow_pages - globals_pages;
    __asan_shadow_memory_dynamic_address = shadow_pgoff * __PAGE_SIZE;

    /*
     * Allocate the last (1/ASAN_SHADOW_SCALE)th of an arena's pages for the map
     * We find the offset and size from the arena map.
     *
     * The allocated map pages are zeroed out, meaning all memory is marked as valid
     * even if it's not allocated already. This is expected: Since the actual memory
     * pages are not allocated, accesses to it will trigger page faults and will be
     * reported through BPF streams. Any pages allocated through bpf_arena_alloc_pages
     * should be poisoned by the allocator right after the call succeeds.
     */
    shadow_map = bpf_arena_alloc_pages(
        &mut arena as *mut core::ffi::c_void,
        __asan_shadow_memory_dynamic_address as *mut core::ffi::c_void,
        shadow_pages,
        NUMA_NO_NODE,
        0,
    ) as u64;
    if shadow_map == 0 {
        arena_stderr(b"Could not allocate shadow map\n\0".as_ptr());

        __asan_shadow_memory_dynamic_address = 0;

        return -ENOMEM;
    }

    asan_inited = true;

    0
}

#[link_section = "license"]
#[no_mangle]
pub static mut _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
