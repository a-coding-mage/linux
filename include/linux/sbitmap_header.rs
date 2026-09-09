/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Fast and scalable bitmaps.
 *
 * Copyright (C) 2016 Facebook
 * Copyright (C) 2013-2014 Jens Axboe
 */

// Translated from linux/sbitmap.h. Kernel includes and build configuration are
// supplied by the surrounding translation unit.

#[repr(C)]
pub struct seq_file {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct sbitmap_word {
    /// word holding free bits
    pub word: ::core::ffi::c_ulong,
    /// word holding cleared bits
    pub cleared: ::core::ffi::c_ulong,
    /// serializes simultaneous updates of ->word and ->cleared
    pub swap_lock: raw_spinlock_t,
}

#[repr(C)]
pub struct sbitmap {
    /// Number of bits used in the whole bitmap.
    pub depth: ::core::ffi::c_uint,
    /// log2(number of bits used per word)
    pub shift: ::core::ffi::c_uint,
    /// Number of words (cachelines) being used for the bitmap.
    pub map_nr: ::core::ffi::c_uint,
    /// Allocate bits in strict round-robin order.
    pub round_robin: bool,
    /// Allocated bitmap.
    pub map: *mut sbitmap_word,
    /// Cache of last successfully allocated or freed bit.
    pub alloc_hint: *mut ::core::ffi::c_uint,
}

pub const SBQ_WAIT_QUEUES: ::core::ffi::c_uint = 8;
pub const SBQ_WAKE_BATCH: ::core::ffi::c_uint = 8;

#[repr(C)]
pub struct sbq_wait_state {
    /// Wait queue.
    pub wait: wait_queue_head_t,
}

#[repr(C)]
pub struct sbitmap_queue {
    pub sb: sbitmap,
    pub wake_batch: ::core::ffi::c_uint,
    pub wake_index: atomic_t,
    pub ws: *mut sbq_wait_state,
    pub ws_active: atomic_t,
    pub min_shallow_depth: ::core::ffi::c_uint,
    pub completion_cnt: atomic_t,
    pub wakeup_cnt: atomic_t,
}

extern "C" {
    pub fn sbitmap_init_node(sb: *mut sbitmap, depth: ::core::ffi::c_uint, shift: ::core::ffi::c_int,
                             flags: gfp_t, node: ::core::ffi::c_int, round_robin: bool,
                             alloc_hint: bool) -> ::core::ffi::c_int;
    pub fn sbitmap_resize(sb: *mut sbitmap, depth: ::core::ffi::c_uint);
    pub fn sbitmap_get(sb: *mut sbitmap) -> ::core::ffi::c_int;
    pub fn sbitmap_any_bit_set(sb: *const sbitmap) -> bool;
    pub fn sbitmap_show(sb: *mut sbitmap, m: *mut seq_file);
    pub fn sbitmap_weight(sb: *const sbitmap) -> ::core::ffi::c_uint;
    pub fn sbitmap_bitmap_show(sb: *mut sbitmap, m: *mut seq_file);
    pub fn sbitmap_queue_init_node(sbq: *mut sbitmap_queue, depth: ::core::ffi::c_uint,
                                   shift: ::core::ffi::c_int, round_robin: bool,
                                   flags: gfp_t, node: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn sbitmap_queue_recalculate_wake_batch(sbq: *mut sbitmap_queue, users: ::core::ffi::c_uint);
    pub fn sbitmap_queue_resize(sbq: *mut sbitmap_queue, depth: ::core::ffi::c_uint);
    pub fn __sbitmap_queue_get(sbq: *mut sbitmap_queue) -> ::core::ffi::c_int;
    pub fn __sbitmap_queue_get_batch(sbq: *mut sbitmap_queue, nr_tags: ::core::ffi::c_int,
                                     offset: *mut ::core::ffi::c_uint) -> ::core::ffi::c_ulong;
    pub fn sbitmap_queue_get_shallow(sbq: *mut sbitmap_queue, shallow_depth: ::core::ffi::c_uint) -> ::core::ffi::c_int;
    pub fn sbitmap_queue_min_shallow_depth(sbq: *mut sbitmap_queue, min_shallow_depth: ::core::ffi::c_uint);
    pub fn sbitmap_queue_clear(sbq: *mut sbitmap_queue, nr: ::core::ffi::c_uint, cpu: ::core::ffi::c_uint);
    pub fn sbitmap_queue_clear_batch(sbq: *mut sbitmap_queue, offset: ::core::ffi::c_int,
                                     tags: *mut ::core::ffi::c_int, nr_tags: ::core::ffi::c_int);
    pub fn sbitmap_queue_wake_all(sbq: *mut sbitmap_queue);
    pub fn sbitmap_queue_wake_up(sbq: *mut sbitmap_queue, nr: ::core::ffi::c_int);
    pub fn sbitmap_queue_show(sbq: *mut sbitmap_queue, m: *mut seq_file);
}

#[inline]
pub unsafe fn __map_depth(sb: *const sbitmap, index: ::core::ffi::c_int) -> ::core::ffi::c_uint {
    if index as ::core::ffi::c_uint == (*sb).map_nr - 1 {
        (*sb).depth - ((index as ::core::ffi::c_uint) << (*sb).shift)
    } else {
        1u32 << (*sb).shift
    }
}

#[inline]
pub unsafe fn sbitmap_free(sb: *mut sbitmap) {
    free_percpu((*sb).alloc_hint);
    kvfree((*sb).map as *mut ::core::ffi::c_void);
    (*sb).map = core::ptr::null_mut();
}

#[inline]
pub unsafe fn sbitmap_queue_free(sbq: *mut sbitmap_queue) {
    kfree((*sbq).ws as *mut ::core::ffi::c_void);
    sbitmap_free(&mut (*sbq).sb);
}

pub const fn SB_NR_TO_INDEX(sb: *const sbitmap, bitnr: ::core::ffi::c_uint) -> ::core::ffi::c_uint { bitnr >> (*sb).shift }
pub const fn SB_NR_TO_BIT(sb: *const sbitmap, bitnr: ::core::ffi::c_uint) -> ::core::ffi::c_uint { bitnr & ((1u32 << (*sb).shift) - 1) }

pub type sb_for_each_fn = Option<unsafe extern "C" fn(*mut sbitmap, ::core::ffi::c_uint, *mut ::core::ffi::c_void) -> bool>;

extern "C" {
    pub fn sbitmap_prepare_to_wait(sbq: *mut sbitmap_queue, ws: *mut sbq_wait_state, sbq_wait: *mut sbq_wait, state: ::core::ffi::c_int);
    pub fn sbitmap_finish_wait(sbq: *mut sbitmap_queue, ws: *mut sbq_wait_state, sbq_wait: *mut sbq_wait);
    pub fn sbitmap_add_wait_queue(sbq: *mut sbitmap_queue, ws: *mut sbq_wait_state, sbq_wait: *mut sbq_wait);
    pub fn sbitmap_del_wait_queue(sbq_wait: *mut sbq_wait);
}

#[repr(C)]
pub struct sbq_wait {
    pub sbq: *mut sbitmap_queue,
    pub wait: wait_queue_entry,
}

#[inline]
pub fn sbq_index_inc(index: ::core::ffi::c_int) -> ::core::ffi::c_int {
    (index + 1) & (SBQ_WAIT_QUEUES as ::core::ffi::c_int - 1)
}

#[inline]
pub unsafe fn sbq_index_atomic_inc(index: *mut atomic_t) {
    let old = atomic_read(index);
    let new = sbq_index_inc(old);
    atomic_cmpxchg(index, old, new);
}

#[inline]
pub unsafe fn sbq_wait_ptr(sbq: *mut sbitmap_queue, wait_index: *mut atomic_t) -> *mut sbq_wait_state {
    let ws = (*sbq).ws.add(atomic_read(wait_index) as usize);
    sbq_index_atomic_inc(wait_index);
    ws
}

#[inline]
pub unsafe fn sbitmap_queue_get(sbq: *mut sbitmap_queue, cpu: *mut ::core::ffi::c_uint) -> ::core::ffi::c_int {
    let nr;
    *cpu = get_cpu() as ::core::ffi::c_uint;
    nr = __sbitmap_queue_get(sbq);
    put_cpu();
    nr
}

#[inline]
pub unsafe fn sbitmap_calculate_shift(depth: ::core::ffi::c_uint) -> ::core::ffi::c_int {
    let mut shift = ilog2(BITS_PER_LONG as ::core::ffi::c_ulong) as ::core::ffi::c_int;
    if depth >= 4 {
        while (4u32 << shift) > depth { shift -= 1; }
    }
    shift
}

#[inline]
pub unsafe fn __sbitmap_word(sb: *mut sbitmap, bitnr: ::core::ffi::c_uint) -> *mut ::core::ffi::c_ulong {
    &mut (*(*sb).map.add(SB_NR_TO_INDEX(sb, bitnr) as usize)).word
}

#[inline]
pub unsafe fn sbitmap_set_bit(sb: *mut sbitmap, bitnr: ::core::ffi::c_uint) {
    set_bit(SB_NR_TO_BIT(sb, bitnr), __sbitmap_word(sb));
}

#[inline]
pub unsafe fn sbitmap_clear_bit(sb: *mut sbitmap, bitnr: ::core::ffi::c_uint) {
    clear_bit(SB_NR_TO_BIT(sb, bitnr), __sbitmap_word(sb));
}

#[inline]
pub unsafe fn sbitmap_deferred_clear_bit(sb: *mut sbitmap, bitnr: ::core::ffi::c_uint) {
    let addr = &mut (*(*sb).map.add(SB_NR_TO_INDEX(sb, bitnr) as usize)).cleared;
    set_bit(SB_NR_TO_BIT(sb, bitnr), addr);
}

#[inline]
pub unsafe fn sbitmap_put(sb: *mut sbitmap, bitnr: ::core::ffi::c_uint) {
    sbitmap_deferred_clear_bit(sb, bitnr);
    if (*sb).alloc_hint != core::ptr::null_mut() && !(*sb).round_robin && bitnr < (*sb).depth {
        *raw_cpu_ptr((*sb).alloc_hint) = bitnr;
    }
}

#[inline]
pub unsafe fn sbitmap_test_bit(sb: *mut sbitmap, bitnr: ::core::ffi::c_uint) -> ::core::ffi::c_int {
    test_bit(SB_NR_TO_BIT(sb, bitnr), __sbitmap_word(sb))
}

#[inline]
pub unsafe fn __sbitmap_for_each_set(sb: *mut sbitmap, mut start: ::core::ffi::c_uint,
                                     fn_: sb_for_each_fn, data: *mut ::core::ffi::c_void) {
    if start >= (*sb).depth { start = 0; }
    let mut index = SB_NR_TO_INDEX(sb, start);
    let mut nr = SB_NR_TO_BIT(sb, start);
    let mut scanned = 0u32;
    while scanned < (*sb).depth {
        let map_depth = __map_depth(sb, index as ::core::ffi::c_int);
        let depth = core::cmp::min(map_depth - nr, (*sb).depth - scanned);
        scanned += depth;
        let mut word = (*(*sb).map.add(index as usize)).word & !(*(*sb).map.add(index as usize)).cleared;
        if word != 0 {
            let search_depth = depth + nr;
            loop {
                nr = find_next_bit(&mut word, search_depth, nr);
                if nr >= search_depth { break; }
                if !(fn_.unwrap()(sb, (index << (*sb).shift) + nr, data)) { return; }
                nr += 1;
            }
        }
        nr = 0;
        index += 1;
        if index >= (*sb).map_nr { index = 0; }
    }
}

#[inline]
pub unsafe fn sbitmap_for_each_set(sb: *mut sbitmap, fn_: sb_for_each_fn, data: *mut ::core::ffi::c_void) {
    __sbitmap_for_each_set(sb, 0, fn_, data);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
