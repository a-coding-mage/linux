/* SPDX-License-Identifier: LGPL-2.1 OR MIT */
/*
 * rseq.h
 *
 * (C) Copyright 2016-2018 - Mathieu Desnoyers <mathieu.desnoyers@efficios.com>
 */

/* Translated from the C header rseq.h. C include directives and header guards
 * are omitted; this file expects the ABI types and architecture-specific rseq
 * helpers normally supplied by rseq-abi.h, compiler.h, rseq-thread-pointer.h,
 * and rseq-*.h to be available to the Rust translation unit.
 */

pub type ptrdiff_t = isize;
pub type intptr_t = isize;
pub type int32_t = i32;
pub type uint32_t = u32;
pub type uintptr_t = usize;
pub type size_t = usize;

unsafe extern "C" {
    /* Offset from the thread pointer to the rseq area. */
    pub static mut rseq_offset: ptrdiff_t;

    /*
     * The rseq ABI is composed of extensible feature fields. The extensions
     * are done by appending additional fields at the end of the structure.
     * The rseq_size defines the size of the active feature set which can be
     * used by the application for the current rseq registration. Features
     * starting at offset >= rseq_size are inactive and should not be used.
     *
     * The rseq_size is the intersection between the available allocation
     * size for the rseq area and the feature size supported by the kernel.
     * unsuccessful.
     */
    pub static mut rseq_size: ::core::ffi::c_uint;

    /* Flags used during rseq registration. */
    pub static mut rseq_flags: ::core::ffi::c_uint;

    pub fn rseq_thread_pointer() -> *mut ::core::ffi::c_void;

    /*
     * Register rseq for the current thread. This needs to be called once
     * by any thread which uses restartable sequences, before they start
     * using restartable sequences, to ensure restartable sequences
     * succeed. A restartable sequence executed from a non-registered
     * thread will always fail.
     */
    pub fn __rseq_register_current_thread(nolibc: bool, legacy: bool) -> ::core::ffi::c_int;

    /*
     * Unregister rseq for current thread.
     */
    pub fn rseq_unregister_current_thread() -> ::core::ffi::c_int;

    /*
     * Restartable sequence fallback for reading the current CPU number.
     */
    pub fn rseq_fallback_current_cpu() -> int32_t;

    /*
     * Restartable sequence fallback for reading the current node number.
     */
    pub fn rseq_fallback_current_node() -> int32_t;

    /*
     * Returns true if rseq is supported.
     */
    pub fn rseq_available() -> bool;

    pub fn rseq_cmpeqv_storev_relaxed_cpu_id(
        v: *mut intptr_t,
        expect: intptr_t,
        newv: intptr_t,
        cpu: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    pub fn rseq_cmpeqv_storev_relaxed_mm_cid(
        v: *mut intptr_t,
        expect: intptr_t,
        newv: intptr_t,
        cpu: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    pub fn rseq_cmpnev_storeoffp_load_relaxed_cpu_id(
        v: *mut intptr_t,
        expectnot: intptr_t,
        voffp: ::core::ffi::c_long,
        load: *mut intptr_t,
        cpu: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    pub fn rseq_cmpnev_storeoffp_load_relaxed_mm_cid(
        v: *mut intptr_t,
        expectnot: intptr_t,
        voffp: ::core::ffi::c_long,
        load: *mut intptr_t,
        cpu: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    pub fn rseq_addv_relaxed_cpu_id(
        v: *mut intptr_t,
        count: intptr_t,
        cpu: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    pub fn rseq_addv_relaxed_mm_cid(
        v: *mut intptr_t,
        count: intptr_t,
        cpu: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;

    /* Present only when the architecture defines RSEQ_ARCH_HAS_OFFSET_DEREF_ADDV. */
    pub fn rseq_offset_deref_addv_relaxed_cpu_id(
        ptr: *mut intptr_t,
        off: ::core::ffi::c_long,
        inc: intptr_t,
        cpu: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    pub fn rseq_offset_deref_addv_relaxed_mm_cid(
        ptr: *mut intptr_t,
        off: ::core::ffi::c_long,
        inc: intptr_t,
        cpu: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;

    pub fn rseq_cmpeqv_trystorev_storev_relaxed_cpu_id(
        v: *mut intptr_t,
        expect: intptr_t,
        v2: *mut intptr_t,
        newv2: intptr_t,
        newv: intptr_t,
        cpu: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    pub fn rseq_cmpeqv_trystorev_storev_relaxed_mm_cid(
        v: *mut intptr_t,
        expect: intptr_t,
        v2: *mut intptr_t,
        newv2: intptr_t,
        newv: intptr_t,
        cpu: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    pub fn rseq_cmpeqv_trystorev_storev_release_cpu_id(
        v: *mut intptr_t,
        expect: intptr_t,
        v2: *mut intptr_t,
        newv2: intptr_t,
        newv: intptr_t,
        cpu: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    pub fn rseq_cmpeqv_trystorev_storev_release_mm_cid(
        v: *mut intptr_t,
        expect: intptr_t,
        v2: *mut intptr_t,
        newv2: intptr_t,
        newv: intptr_t,
        cpu: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    pub fn rseq_cmpeqv_cmpeqv_storev_relaxed_cpu_id(
        v: *mut intptr_t,
        expect: intptr_t,
        v2: *mut intptr_t,
        expect2: intptr_t,
        newv: intptr_t,
        cpu: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    pub fn rseq_cmpeqv_cmpeqv_storev_relaxed_mm_cid(
        v: *mut intptr_t,
        expect: intptr_t,
        v2: *mut intptr_t,
        expect2: intptr_t,
        newv: intptr_t,
        cpu: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    pub fn rseq_cmpeqv_trymemcpy_storev_relaxed_cpu_id(
        v: *mut intptr_t,
        expect: intptr_t,
        dst: *mut ::core::ffi::c_void,
        src: *mut ::core::ffi::c_void,
        len: size_t,
        newv: intptr_t,
        cpu: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    pub fn rseq_cmpeqv_trymemcpy_storev_relaxed_mm_cid(
        v: *mut intptr_t,
        expect: intptr_t,
        dst: *mut ::core::ffi::c_void,
        src: *mut ::core::ffi::c_void,
        len: size_t,
        newv: intptr_t,
        cpu: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    pub fn rseq_cmpeqv_trymemcpy_storev_release_cpu_id(
        v: *mut intptr_t,
        expect: intptr_t,
        dst: *mut ::core::ffi::c_void,
        src: *mut ::core::ffi::c_void,
        len: size_t,
        newv: intptr_t,
        cpu: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    pub fn rseq_cmpeqv_trymemcpy_storev_release_mm_cid(
        v: *mut intptr_t,
        expect: intptr_t,
        dst: *mut ::core::ffi::c_void,
        src: *mut ::core::ffi::c_void,
        len: size_t,
        newv: intptr_t,
        cpu: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum rseq_mo {
    RSEQ_MO_RELAXED = 0,
    RSEQ_MO_CONSUME = 1, /* Unused */
    RSEQ_MO_ACQUIRE = 2, /* Unused */
    RSEQ_MO_RELEASE = 3,
    RSEQ_MO_ACQ_REL = 4, /* Unused */
    RSEQ_MO_SEQ_CST = 5, /* Unused */
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum rseq_percpu_mode {
    RSEQ_PERCPU_CPU_ID = 0,
    RSEQ_PERCPU_MM_CID = 1,
}

#[inline]
pub unsafe fn rseq_get_abi() -> *mut rseq_abi {
    (rseq_thread_pointer() as uintptr_t).wrapping_add(rseq_offset as uintptr_t) as *mut rseq_abi
}

#[inline]
pub fn rseq_likely(x: bool) -> bool {
    x
}

#[inline]
pub fn rseq_unlikely(x: bool) -> bool {
    x
}

#[inline]
pub fn rseq_barrier() {
    ::core::sync::atomic::compiler_fence(::core::sync::atomic::Ordering::SeqCst);
}

#[inline]
pub unsafe fn RSEQ_READ_ONCE<T: Copy>(x: *const T) -> T {
    ::core::ptr::read_volatile(x)
}

#[inline]
pub unsafe fn RSEQ_WRITE_ONCE<T>(x: *mut T, v: T) {
    ::core::ptr::write_volatile(x, v);
}

#[inline]
pub unsafe fn RSEQ_ACCESS_ONCE<T: Copy>(x: *mut T) -> T {
    ::core::ptr::read_volatile(x)
}

#[inline]
pub unsafe fn rseq_register_current_thread() -> ::core::ffi::c_int {
    __rseq_register_current_thread(false, false)
}

/*
 * Values returned can be either the current CPU number, -1 (rseq is
 * uninitialized), or -2 (rseq initialization has failed).
 */
#[inline]
pub unsafe fn rseq_current_cpu_raw() -> int32_t {
    RSEQ_READ_ONCE(::core::ptr::addr_of!((*rseq_get_abi()).cpu_id))
}

/*
 * Returns a possible CPU number, which is typically the current CPU.
 * The returned CPU number can be used to prepare for an rseq critical
 * section, which will confirm whether the cpu number is indeed the
 * current one, and whether rseq is initialized.
 *
 * The CPU number returned by rseq_cpu_start should always be validated
 * by passing it to a rseq asm sequence, or by comparing it to the
 * return value of rseq_current_cpu_raw() if the rseq asm sequence
 * does not need to be invoked.
 */
#[inline]
pub unsafe fn rseq_cpu_start() -> uint32_t {
    RSEQ_READ_ONCE(::core::ptr::addr_of!((*rseq_get_abi()).cpu_id_start))
}

#[inline]
pub unsafe fn rseq_current_cpu() -> uint32_t {
    let mut cpu: int32_t;

    cpu = rseq_current_cpu_raw();
    if rseq_unlikely(cpu < 0) {
        cpu = rseq_fallback_current_cpu();
    }
    cpu as uint32_t
}

#[inline]
pub unsafe fn rseq_node_id_available() -> bool {
    (rseq_size as ::core::ffi::c_int) >=
        (::core::mem::offset_of!(rseq_abi, node_id)
            + ::core::mem::size_of_val(&::core::ptr::addr_of!((*rseq_get_abi()).node_id).read())) as ::core::ffi::c_int
}

/*
 * Current NUMA node number.
 */
#[inline]
pub unsafe fn rseq_current_node_id() -> uint32_t {
    assert!(rseq_node_id_available());
    RSEQ_READ_ONCE(::core::ptr::addr_of!((*rseq_get_abi()).node_id))
}

#[inline]
pub unsafe fn rseq_mm_cid_available() -> bool {
    (rseq_size as ::core::ffi::c_int) >=
        (::core::mem::offset_of!(rseq_abi, mm_cid)
            + ::core::mem::size_of_val(&::core::ptr::addr_of!((*rseq_get_abi()).mm_cid).read())) as ::core::ffi::c_int
}

#[inline]
pub unsafe fn rseq_current_mm_cid() -> uint32_t {
    RSEQ_READ_ONCE(::core::ptr::addr_of!((*rseq_get_abi()).mm_cid))
}

#[inline]
pub unsafe fn rseq_clear_rseq_cs() {
    RSEQ_WRITE_ONCE(::core::ptr::addr_of_mut!((*rseq_get_abi()).rseq_cs.arch.ptr), 0);
}

/*
 * rseq_prepare_unload() should be invoked by each thread executing a rseq
 * critical section at least once between their last critical section and
 * library unload of the library defining the rseq critical section (struct
 * rseq_cs) or the code referred to by the struct rseq_cs start_ip and
 * post_commit_offset fields. This also applies to use of rseq in code
 * generated by JIT: rseq_prepare_unload() should be invoked at least once by
 * each thread executing a rseq critical section before reclaim of the memory
 * holding the struct rseq_cs or reclaim of the code pointed to by struct
 * rseq_cs start_ip and post_commit_offset fields.
 */
#[inline]
pub unsafe fn rseq_prepare_unload() {
    rseq_clear_rseq_cs();
}

#[inline(always)]
pub unsafe fn rseq_cmpeqv_storev(
    rseq_mo: rseq_mo,
    percpu_mode: rseq_percpu_mode,
    v: *mut intptr_t,
    expect: intptr_t,
    newv: intptr_t,
    cpu: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if rseq_mo != rseq_mo::RSEQ_MO_RELAXED {
        return -1;
    }
    match percpu_mode {
        rseq_percpu_mode::RSEQ_PERCPU_CPU_ID => rseq_cmpeqv_storev_relaxed_cpu_id(v, expect, newv, cpu),
        rseq_percpu_mode::RSEQ_PERCPU_MM_CID => rseq_cmpeqv_storev_relaxed_mm_cid(v, expect, newv, cpu),
    }
}

/*
 * Compare @v against @expectnot. When it does _not_ match, load @v
 * into @load, and store the content of *@v + voffp into @v.
 */
#[inline(always)]
pub unsafe fn rseq_cmpnev_storeoffp_load(
    rseq_mo: rseq_mo,
    percpu_mode: rseq_percpu_mode,
    v: *mut intptr_t,
    expectnot: intptr_t,
    voffp: ::core::ffi::c_long,
    load: *mut intptr_t,
    cpu: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if rseq_mo != rseq_mo::RSEQ_MO_RELAXED {
        return -1;
    }
    match percpu_mode {
        rseq_percpu_mode::RSEQ_PERCPU_CPU_ID => rseq_cmpnev_storeoffp_load_relaxed_cpu_id(v, expectnot, voffp, load, cpu),
        rseq_percpu_mode::RSEQ_PERCPU_MM_CID => rseq_cmpnev_storeoffp_load_relaxed_mm_cid(v, expectnot, voffp, load, cpu),
    }
}

#[inline(always)]
pub unsafe fn rseq_addv(
    rseq_mo: rseq_mo,
    percpu_mode: rseq_percpu_mode,
    v: *mut intptr_t,
    count: intptr_t,
    cpu: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if rseq_mo != rseq_mo::RSEQ_MO_RELAXED {
        return -1;
    }
    match percpu_mode {
        rseq_percpu_mode::RSEQ_PERCPU_CPU_ID => rseq_addv_relaxed_cpu_id(v, count, cpu),
        rseq_percpu_mode::RSEQ_PERCPU_MM_CID => rseq_addv_relaxed_mm_cid(v, count, cpu),
    }
}

/* Translates the C block guarded by RSEQ_ARCH_HAS_OFFSET_DEREF_ADDV. */
/*
 *   pval = *(ptr+off)
 *  *pval += inc;
 */
#[inline(always)]
pub unsafe fn rseq_offset_deref_addv(
    rseq_mo: rseq_mo,
    percpu_mode: rseq_percpu_mode,
    ptr: *mut intptr_t,
    off: ::core::ffi::c_long,
    inc: intptr_t,
    cpu: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if rseq_mo != rseq_mo::RSEQ_MO_RELAXED {
        return -1;
    }
    match percpu_mode {
        rseq_percpu_mode::RSEQ_PERCPU_CPU_ID => rseq_offset_deref_addv_relaxed_cpu_id(ptr, off, inc, cpu),
        rseq_percpu_mode::RSEQ_PERCPU_MM_CID => rseq_offset_deref_addv_relaxed_mm_cid(ptr, off, inc, cpu),
    }
}

#[inline(always)]
pub unsafe fn rseq_cmpeqv_trystorev_storev(
    rseq_mo: rseq_mo,
    percpu_mode: rseq_percpu_mode,
    v: *mut intptr_t,
    expect: intptr_t,
    v2: *mut intptr_t,
    newv2: intptr_t,
    newv: intptr_t,
    cpu: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    match rseq_mo {
        rseq_mo::RSEQ_MO_RELAXED => {
            match percpu_mode {
                rseq_percpu_mode::RSEQ_PERCPU_CPU_ID => rseq_cmpeqv_trystorev_storev_relaxed_cpu_id(v, expect, v2, newv2, newv, cpu),
                rseq_percpu_mode::RSEQ_PERCPU_MM_CID => rseq_cmpeqv_trystorev_storev_relaxed_mm_cid(v, expect, v2, newv2, newv, cpu),
            }
        }
        rseq_mo::RSEQ_MO_RELEASE => {
            match percpu_mode {
                rseq_percpu_mode::RSEQ_PERCPU_CPU_ID => rseq_cmpeqv_trystorev_storev_release_cpu_id(v, expect, v2, newv2, newv, cpu),
                rseq_percpu_mode::RSEQ_PERCPU_MM_CID => rseq_cmpeqv_trystorev_storev_release_mm_cid(v, expect, v2, newv2, newv, cpu),
            }
        }
        _ => -1,
    }
}

#[inline(always)]
pub unsafe fn rseq_cmpeqv_cmpeqv_storev(
    rseq_mo: rseq_mo,
    percpu_mode: rseq_percpu_mode,
    v: *mut intptr_t,
    expect: intptr_t,
    v2: *mut intptr_t,
    expect2: intptr_t,
    newv: intptr_t,
    cpu: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if rseq_mo != rseq_mo::RSEQ_MO_RELAXED {
        return -1;
    }
    match percpu_mode {
        rseq_percpu_mode::RSEQ_PERCPU_CPU_ID => rseq_cmpeqv_cmpeqv_storev_relaxed_cpu_id(v, expect, v2, expect2, newv, cpu),
        rseq_percpu_mode::RSEQ_PERCPU_MM_CID => rseq_cmpeqv_cmpeqv_storev_relaxed_mm_cid(v, expect, v2, expect2, newv, cpu),
    }
}

#[inline(always)]
pub unsafe fn rseq_cmpeqv_trymemcpy_storev(
    rseq_mo: rseq_mo,
    percpu_mode: rseq_percpu_mode,
    v: *mut intptr_t,
    expect: intptr_t,
    dst: *mut ::core::ffi::c_void,
    src: *mut ::core::ffi::c_void,
    len: size_t,
    newv: intptr_t,
    cpu: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    match rseq_mo {
        rseq_mo::RSEQ_MO_RELAXED => {
            match percpu_mode {
                rseq_percpu_mode::RSEQ_PERCPU_CPU_ID => rseq_cmpeqv_trymemcpy_storev_relaxed_cpu_id(v, expect, dst, src, len, newv, cpu),
                rseq_percpu_mode::RSEQ_PERCPU_MM_CID => rseq_cmpeqv_trymemcpy_storev_relaxed_mm_cid(v, expect, dst, src, len, newv, cpu),
            }
        }
        rseq_mo::RSEQ_MO_RELEASE => {
            match percpu_mode {
                rseq_percpu_mode::RSEQ_PERCPU_CPU_ID => rseq_cmpeqv_trymemcpy_storev_release_cpu_id(v, expect, dst, src, len, newv, cpu),
                rseq_percpu_mode::RSEQ_PERCPU_MM_CID => rseq_cmpeqv_trymemcpy_storev_release_mm_cid(v, expect, dst, src, len, newv, cpu),
            }
        }
        _ => -1,
    }
}
