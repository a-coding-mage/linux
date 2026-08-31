/* SPDX-License-Identifier: GPL-2.0 */
/*
 * BPF-side helpers for cids and cmasks. See kernel/sched/ext/cid.h for the
 * authoritative layout and semantics. The BPF-side helpers use the cmask_*
 * naming (no scx_ prefix); cmask is the SCX bitmap type so the prefix is
 * redundant in BPF code. Atomics use __sync_val_compare_and_swap and every
 * helper is inline (no .c counterpart).
 *
 * Included by scx/common.bpf.h; don't include directly.
 *
 * Copyright (c) 2026 Meta Platforms, Inc. and affiliates.
 * Copyright (c) 2026 Tejun Heo <tj@kernel.org>
 */

// Dependency intent from C: #include "bpf_arena_common.bpf.h"

pub const fn BIT_U64(nr: u32) -> u64 {
    1u64 << nr
}

pub const fn GENMASK_U64(h: u32, l: u32) -> u64 {
    (!0u64 << l) & (!0u64 >> (63 - h))
}

/*
 * Storage cap for bounded loops over bits[]. Sized to cover NR_CPUS=8192 with
 * one extra word for head-misalignment. Increase if deployment targets larger
 * NR_CPUS.
 */
pub const CMASK_MAX_WORDS: u32 = 129;

/*
 * Mirrors SCX_CMASK_NR_WORDS in kernel/sched/ext/types.h. The u64 cast keeps
 * the +63 from wrapping when @nr_cids is near U32_MAX, so cmask_reframe()
 * bounds-checking the result against alloc_words catches the overflow instead
 * of seeing a small value.
 */
pub const fn CMASK_NR_WORDS(nr_cids: u32) -> u32 {
    (((nr_cids as u64 + 63) / 64 + 1) as u32)
}

unsafe extern "C" {
    pub type cpumask;

    pub fn scx_bpf_error(fmt: *const core::ffi::c_char, ...);
    pub fn scx_bpf_nr_cpu_ids() -> u32;
    pub fn bpf_cpumask_test_cpu(cpu: i32, cpumask: *const cpumask) -> bool;
    pub fn scx_bpf_cpu_to_cid(cpu: i32) -> i32;
}

#[inline(always)]
unsafe fn ctzll(v: u64) -> u32 {
    v.trailing_zeros()
}

// Provided by included BPF arena/common definitions in the source repository.
#[repr(C)]
pub struct scx_cmask {
    pub base: u32,
    pub nr_cids: u32,
    pub alloc_words: u32,
    pub bits: [u64; 0],
}

#[inline(always)]
pub unsafe fn __cmask_contains(cid: u32, m: *const scx_cmask) -> bool {
    unsafe { cid >= (*m).base && cid < (*m).base.wrapping_add((*m).nr_cids) }
}

#[inline(always)]
pub unsafe fn __cmask_word(cid: u32, m: *const scx_cmask) -> *mut u64 {
    unsafe {
        (*m).bits
            .as_ptr()
            .add((cid / 64).wrapping_sub((*m).base / 64) as usize) as *mut u64
    }
}

/**
 * __cmask_init - Initialize @m with explicit storage capacity
 * @m: cmask to initialize
 * @base: first cid of the active range
 * @nr_cids: number of cids in the active range
 * @alloc_cids: storage capacity in cids, at least @nr_cids
 *
 * Use when storage is sized larger than the initial active range. All of
 * bits[] is zeroed.
 */
#[inline(always)]
pub unsafe fn __cmask_init(m: *mut scx_cmask, base: u32, nr_cids: u32, alloc_cids: u32) {
    let alloc_words: u32;

    if nr_cids > alloc_cids {
        unsafe {
            scx_bpf_error(
                c"__cmask_init: nr_cids=%u exceeds alloc_cids=%u".as_ptr(),
                nr_cids,
                alloc_cids,
            );
        }
        return;
    }
    alloc_words = CMASK_NR_WORDS(alloc_cids);

    unsafe {
        (*m).base = base;
        (*m).nr_cids = nr_cids;
        (*m).alloc_words = alloc_words;
    }

    for i in 0..CMASK_MAX_WORDS {
        if i >= alloc_words {
            break;
        }
        unsafe {
            *__cmask_word((*m).base.wrapping_add(i * 64), m) = 0;
        }
    }
}

/**
 * cmask_init - Initialize @m on tight storage
 * @m: cmask to initialize
 * @base: first cid of the active range
 * @nr_cids: number of cids in the active range
 *
 * All of bits[] is zeroed.
 */
#[inline(always)]
pub unsafe fn cmask_init(m: *mut scx_cmask, base: u32, nr_cids: u32) {
    unsafe { __cmask_init(m, base, nr_cids, nr_cids) };
}

/**
 * cmask_reframe - Reshape @m's active range without resizing storage
 * @m: cmask to reframe
 * @base: new active range base
 * @nr_cids: new active range length, must fit within @m->alloc_words
 *
 * Body bits within the new range become garbage - only the head and tail
 * words are zeroed to keep the padding invariant.
 */
#[inline(always)]
pub unsafe fn cmask_reframe(m: *mut scx_cmask, base: u32, nr_cids: u32) {
    unsafe {
        if CMASK_NR_WORDS(nr_cids) > (*m).alloc_words {
            scx_bpf_error(
                c"cmask_reframe: nr_cids=%u exceeds alloc_words=%u".as_ptr(),
                nr_cids,
                (*m).alloc_words,
            );
            return;
        }
        if nr_cids != 0 {
            let last_word = ((base & 63).wrapping_add(nr_cids).wrapping_sub(1)) / 64;

            *(*m).bits.as_mut_ptr().add(0) = 0;
            *(*m).bits.as_mut_ptr().add(last_word as usize) = 0;
        }
        (*m).base = base;
        (*m).nr_cids = nr_cids;
    }
}

#[inline(always)]
pub unsafe fn cmask_test(cid: u32, m: *const scx_cmask) -> bool {
    unsafe {
        if !__cmask_contains(cid, m) {
            return false;
        }
        (*__cmask_word(cid, m) & BIT_U64(cid & 63)) != 0
    }
}

/*
 * x86 BPF JIT rejects BPF_OR | BPF_FETCH and BPF_AND | BPF_FETCH on arena
 * pointers (see bpf_jit_supports_insn() in arch/x86/net/bpf_jit_comp.c). Only
 * BPF_CMPXCHG / BPF_XCHG / BPF_ADD with FETCH are allowed. Implement
 * test_and_{set,clear} and the atomic set/clear via a cmpxchg loop.
 *
 * CMASK_CAS_TRIES is sized so exhausting it means seconds of real spinning
 * on one word - past any plausible contention. Abort hard.
 */
pub const CMASK_CAS_TRIES: u32 = 1u32 << 23;

#[inline(always)]
unsafe fn __sync_val_compare_and_swap_u64(w: *mut u64, old: u64, new: u64) -> u64 {
    unsafe {
        let current = core::ptr::read_volatile(w);
        if current == old {
            core::ptr::write_volatile(w, new);
        }
        current
    }
}

#[inline(always)]
pub unsafe fn cmask_set(cid: u32, m: *mut scx_cmask) {
    let w: *mut u64;
    let bit: u64;
    let mut old: u64;
    let new: u64;

    unsafe {
        if !__cmask_contains(cid, m) {
            return;
        }
        w = __cmask_word(cid, m);
        bit = BIT_U64(cid & 63);
        for _i in 0..CMASK_CAS_TRIES {
            old = core::ptr::read_volatile(w);
            if (old & bit) != 0 {
                return;
            }
            new = old | bit;
            if __sync_val_compare_and_swap_u64(w, old, new) == old {
                return;
            }
        }
        scx_bpf_error(c"cmask_set CAS exhausted at cid %u".as_ptr(), cid);
    }
}

#[inline(always)]
pub unsafe fn cmask_clear(cid: u32, m: *mut scx_cmask) {
    let w: *mut u64;
    let bit: u64;
    let mut old: u64;
    let new: u64;

    unsafe {
        if !__cmask_contains(cid, m) {
            return;
        }
        w = __cmask_word(cid, m);
        bit = BIT_U64(cid & 63);
        for _i in 0..CMASK_CAS_TRIES {
            old = core::ptr::read_volatile(w);
            if (old & bit) == 0 {
                return;
            }
            new = old & !bit;
            if __sync_val_compare_and_swap_u64(w, old, new) == old {
                return;
            }
        }
        scx_bpf_error(c"cmask_clear CAS exhausted at cid %u".as_ptr(), cid);
    }
}

#[inline(always)]
pub unsafe fn cmask_test_and_set(cid: u32, m: *mut scx_cmask) -> bool {
    let w: *mut u64;
    let bit: u64;
    let mut old: u64;
    let new: u64;

    unsafe {
        if !__cmask_contains(cid, m) {
            return false;
        }
        w = __cmask_word(cid, m);
        bit = BIT_U64(cid & 63);
        for _i in 0..CMASK_CAS_TRIES {
            old = core::ptr::read_volatile(w);
            if (old & bit) != 0 {
                return true;
            }
            new = old | bit;
            if __sync_val_compare_and_swap_u64(w, old, new) == old {
                return false;
            }
        }
        scx_bpf_error(c"cmask_test_and_set CAS exhausted at cid %u".as_ptr(), cid);
        false
    }
}

#[inline(always)]
pub unsafe fn cmask_test_and_clear(cid: u32, m: *mut scx_cmask) -> bool {
    let w: *mut u64;
    let bit: u64;
    let mut old: u64;
    let new: u64;

    unsafe {
        if !__cmask_contains(cid, m) {
            return false;
        }
        w = __cmask_word(cid, m);
        bit = BIT_U64(cid & 63);
        for _i in 0..CMASK_CAS_TRIES {
            old = core::ptr::read_volatile(w);
            if (old & bit) == 0 {
                return false;
            }
            new = old & !bit;
            if __sync_val_compare_and_swap_u64(w, old, new) == old {
                return true;
            }
        }
        scx_bpf_error(c"cmask_test_and_clear CAS exhausted at cid %u".as_ptr(), cid);
        false
    }
}

#[inline(always)]
pub unsafe fn __cmask_set(cid: u32, m: *mut scx_cmask) {
    unsafe {
        if !__cmask_contains(cid, m) {
            return;
        }
        let w = __cmask_word(cid, m);
        *w |= BIT_U64(cid & 63);
    }
}

#[inline(always)]
pub unsafe fn __cmask_clear(cid: u32, m: *mut scx_cmask) {
    unsafe {
        if !__cmask_contains(cid, m) {
            return;
        }
        let w = __cmask_word(cid, m);
        *w &= !BIT_U64(cid & 63);
    }
}

#[inline(always)]
pub unsafe fn __cmask_test_and_set(cid: u32, m: *mut scx_cmask) -> bool {
    let bit = BIT_U64(cid & 63);
    let w: *mut u64;
    let prev: u64;

    unsafe {
        if !__cmask_contains(cid, m) {
            return false;
        }
        w = __cmask_word(cid, m);
        prev = *w & bit;
        *w |= bit;
        prev != 0
    }
}

#[inline(always)]
pub unsafe fn __cmask_test_and_clear(cid: u32, m: *mut scx_cmask) -> bool {
    let bit = BIT_U64(cid & 63);
    let w: *mut u64;
    let prev: u64;

    unsafe {
        if !__cmask_contains(cid, m) {
            return false;
        }
        w = __cmask_word(cid, m);
        prev = *w & bit;
        *w &= !bit;
        prev != 0
    }
}

#[inline(always)]
pub unsafe fn cmask_zero(m: *mut scx_cmask) {
    unsafe {
        let nr_words = CMASK_NR_WORDS((*m).nr_cids);

        for i in 0..CMASK_MAX_WORDS {
            if i >= nr_words {
                break;
            }
            *(*m).bits.as_mut_ptr().add(i as usize) = 0;
        }
    }
}

/*
 * BPF_-prefixed to avoid colliding with the kernel's anonymous CMASK_OP_*
 * enum in ext/cid.c, which is exported via BTF and reachable through
 * vmlinux.h.
 */
pub const BPF_CMASK_OP_AND: i32 = 0;
pub const BPF_CMASK_OP_OR: i32 = 1;
pub const BPF_CMASK_OP_COPY: i32 = 2;
pub const BPF_CMASK_OP_ANDNOT: i32 = 3;

#[inline(always)]
pub unsafe fn cmask_op_word(
    dst: *mut scx_cmask,
    src: *const scx_cmask,
    di: u32,
    si: u32,
    mask: u64,
    op: i32,
) {
    unsafe {
        let dv = *(*dst).bits.as_mut_ptr().add(di as usize);
        let sv = *(*src).bits.as_ptr().add(si as usize);
        let rv: u64;

        if op == BPF_CMASK_OP_AND {
            rv = dv & sv;
        } else if op == BPF_CMASK_OP_OR {
            rv = dv | sv;
        } else if op == BPF_CMASK_OP_ANDNOT {
            rv = dv & !sv;
        } else {
            rv = sv;
        }

        *(*dst).bits.as_mut_ptr().add(di as usize) = (dv & !mask) | (rv & mask);
    }
}

#[inline(always)]
pub unsafe fn cmask_op(dst: *mut scx_cmask, src: *const scx_cmask, op: i32) {
    unsafe {
        let d_end = (*dst).base.wrapping_add((*dst).nr_cids);
        let s_end = (*src).base.wrapping_add((*src).nr_cids);
        let lo = if (*dst).base > (*src).base { (*dst).base } else { (*src).base };
        let hi = if d_end < s_end { d_end } else { s_end };
        let d_base = (*dst).base / 64;
        let s_base = (*src).base / 64;
        let lo_word: u32;
        let hi_word: u32;
        let head_mask: u64;
        let tail_mask: u64;

        if lo >= hi {
            return;
        }

        lo_word = lo / 64;
        hi_word = (hi - 1) / 64;
        head_mask = GENMASK_U64(63, lo & 63);
        tail_mask = GENMASK_U64((hi - 1) & 63, 0);

        for i in 0..CMASK_MAX_WORDS {
            let w = lo_word + i;
            let mut m: u64;

            if w > hi_word {
                break;
            }

            m = GENMASK_U64(63, 0);
            if w == lo_word {
                m &= head_mask;
            }
            if w == hi_word {
                m &= tail_mask;
            }

            cmask_op_word(dst, src, w - d_base, w - s_base, m, op);
        }
    }
}

/*
 * cmask_and/or/copy only modify @dst bits that lie in the intersection of
 * [@dst->base, @dst->base + @dst->nr_cids) and [@src->base,
 * @src->base + @src->nr_cids). Bits in @dst outside that window
 * keep their prior values - in particular, cmask_copy() does NOT zero @dst
 * bits that lie outside @src's range.
 */
#[inline(always)]
pub unsafe fn cmask_and(dst: *mut scx_cmask, src: *const scx_cmask) {
    unsafe { cmask_op(dst, src, BPF_CMASK_OP_AND) };
}

#[inline(always)]
pub unsafe fn cmask_or(dst: *mut scx_cmask, src: *const scx_cmask) {
    unsafe { cmask_op(dst, src, BPF_CMASK_OP_OR) };
}

#[inline(always)]
pub unsafe fn cmask_copy(dst: *mut scx_cmask, src: *const scx_cmask) {
    unsafe { cmask_op(dst, src, BPF_CMASK_OP_COPY) };
}

#[inline(always)]
pub unsafe fn cmask_andnot(dst: *mut scx_cmask, src: *const scx_cmask) {
    unsafe { cmask_op(dst, src, BPF_CMASK_OP_ANDNOT) };
}

/*
 * True iff @a and @b have identical bits over their (assumed equal) range.
 * Callers are expected to pass same-shape cmasks; differing shapes always
 * compare unequal.
 */
#[inline(always)]
pub unsafe fn cmask_equal(a: *const scx_cmask, b: *const scx_cmask) -> bool {
    unsafe {
        let nr_words: u32;

        if (*a).base != (*b).base || (*a).nr_cids != (*b).nr_cids {
            return false;
        }
        if (*a).nr_cids == 0 {
            return true;
        }
        nr_words = ((*a).base + (*a).nr_cids - 1) / 64 - (*a).base / 64 + 1;

        for i in 0..CMASK_MAX_WORDS {
            if i >= nr_words {
                break;
            }
            if *(*a).bits.as_ptr().add(i as usize) != *(*b).bits.as_ptr().add(i as usize) {
                return false;
            }
        }
        true
    }
}

/**
 * cmask_next_set - find the first set bit at or after @cid
 * @m: cmask to search
 * @cid: starting cid (clamped to @m->base if below)
 *
 * Returns the smallest set cid in [@cid, @m->base + @m->nr_cids), or
 * @m->base + @m->nr_cids if none (the out-of-range sentinel matches the
 * termination condition used by cmask_for_each()).
 */
#[inline(always)]
pub unsafe fn cmask_next_set(m: *const scx_cmask, mut cid: u32) -> u32 {
    unsafe {
        let end = (*m).base + (*m).nr_cids;
        let base = (*m).base / 64;
        let last_wi = (end - 1) / 64 - base;
        let start_wi: u32;
        let start_bit: u32;

        if cid < (*m).base {
            cid = (*m).base;
        }
        if cid >= end {
            return end;
        }

        start_wi = cid / 64 - base;
        start_bit = cid & 63;

        for i in 0..CMASK_MAX_WORDS {
            let wi = start_wi + i;
            let mut word: u64;
            let found: u32;

            if wi > last_wi {
                break;
            }

            word = *(*m).bits.as_ptr().add(wi as usize);
            if i == 0 {
                word &= GENMASK_U64(63, start_bit);
            }
            if word == 0 {
                continue;
            }

            found = (base + wi) * 64 + ctzll(word);
            if found >= end {
                return end;
            }
            return found;
        }
        end
    }
}

#[inline(always)]
pub unsafe fn cmask_first_set(m: *const scx_cmask) -> u32 {
    unsafe { cmask_next_set(m, (*m).base) }
}

/*
 * C macro:
 * #define cmask_for_each(cid, m) \
 *      for ((cid) = cmask_first_set(m); \
 *           (cid) < (m)->base + (m)->nr_cids; \
 *           (cid) = cmask_next_set((m), (cid) + 1))
 */

/*
 * True iff every bit set in @a is also set in @b. Matches the kernel-side
 * scx_cmask_subset(): ranges don't need to nest, and set bits of @a outside
 * @b's range fail the test.
 */
#[inline(always)]
pub unsafe fn cmask_subset(a: *const scx_cmask, b: *const scx_cmask) -> bool {
    unsafe {
        let a_end = (*a).base + (*a).nr_cids;
        let b_end = (*b).base + (*b).nr_cids;
        let a_wbase = (*a).base / 64;
        let b_wbase = (*b).base / 64;
        let lo = if (*a).base > (*b).base { (*a).base } else { (*b).base };
        let hi = if a_end < b_end { a_end } else { b_end };
        let lo_word: u32;
        let hi_word: u32;

        /* set bits of @a outside @b's range can't be in @b */
        if (*a).base < (*b).base
            && cmask_next_set(a, (*a).base) < if (*b).base < a_end { (*b).base } else { a_end }
        {
            return false;
        }
        if a_end > b_end
            && cmask_next_set(a, if (*a).base > b_end { (*a).base } else { b_end }) < a_end
        {
            return false;
        }

        if lo >= hi {
            return true;
        }

        /*
         * Walk the words the range intersection spans. Plain word tests
         * suffice: the scans above guarantee @a has no set bit outside @b's
         * range and padding bits are kept clear by all cmask helpers.
         */
        lo_word = lo / 64;
        hi_word = (hi - 1) / 64;

        for i in 0..CMASK_MAX_WORDS {
            let w = lo_word + i;

            if w > hi_word {
                break;
            }
            if (*(*a).bits.as_ptr().add((w - a_wbase) as usize)
                & !*(*b).bits.as_ptr().add((w - b_wbase) as usize))
                != 0
            {
                return false;
            }
        }
        true
    }
}

/*
 * Population count over [base, base + nr_cids). Padding bits in the head/tail
 * words are guaranteed zero by the mutating helpers, so a flat popcount over
 * the words the range spans is correct.
 */
#[inline(always)]
pub unsafe fn cmask_weight(m: *const scx_cmask) -> u32 {
    unsafe {
        let nr_words: u32;
        let mut count: u32 = 0;

        if (*m).nr_cids == 0 {
            return 0;
        }
        nr_words = ((*m).base + (*m).nr_cids - 1) / 64 - (*m).base / 64 + 1;

        for i in 0..CMASK_MAX_WORDS {
            if i >= nr_words {
                break;
            }
            count += (*(*m).bits.as_ptr().add(i as usize)).count_ones();
        }
        count
    }
}

/*
 * True if @a and @b share any set bit. Walk only the intersection of their
 * ranges, matching the semantics of cmask_and().
 */
#[inline(always)]
pub unsafe fn cmask_intersects(a: *const scx_cmask, b: *const scx_cmask) -> bool {
    unsafe {
        let a_end = (*a).base + (*a).nr_cids;
        let b_end = (*b).base + (*b).nr_cids;
        let lo = if (*a).base > (*b).base { (*a).base } else { (*b).base };
        let hi = if a_end < b_end { a_end } else { b_end };
        let a_base = (*a).base / 64;
        let b_base = (*b).base / 64;
        let lo_word: u32;
        let hi_word: u32;
        let head_mask: u64;
        let tail_mask: u64;

        if lo >= hi {
            return false;
        }

        lo_word = lo / 64;
        hi_word = (hi - 1) / 64;
        head_mask = GENMASK_U64(63, lo & 63);
        tail_mask = GENMASK_U64((hi - 1) & 63, 0);

        for i in 0..CMASK_MAX_WORDS {
            let w = lo_word + i;
            let mut mask: u64;
            let av: u64;
            let bv: u64;

            if w > hi_word {
                break;
            }

            mask = GENMASK_U64(63, 0);
            if w == lo_word {
                mask &= head_mask;
            }
            if w == hi_word {
                mask &= tail_mask;
            }

            av = *(*a).bits.as_ptr().add((w - a_base) as usize) & mask;
            bv = *(*b).bits.as_ptr().add((w - b_base) as usize) & mask;
            if (av & bv) != 0 {
                return true;
            }
        }
        false
    }
}

/*
 * Find the next cid set in both @a and @b at or after @start, bounded by the
 * intersection of the two ranges. Return a->base + a->nr_cids if none found.
 *
 * Building block for cmask_next_and_set_wrap(). Callers that want a bounded
 * scan without wrap call this directly.
 */
#[inline(always)]
pub unsafe fn cmask_next_and_set(a: *const scx_cmask, b: *const scx_cmask, mut start: u32) -> u32 {
    unsafe {
        let a_end = (*a).base + (*a).nr_cids;
        let b_end = (*b).base + (*b).nr_cids;
        let a_wbase = (*a).base / 64;
        let b_wbase = (*b).base / 64;
        let lo = if (*a).base > (*b).base { (*a).base } else { (*b).base };
        let hi = if a_end < b_end { a_end } else { b_end };
        let last_wi: u32;
        let start_wi: u32;
        let start_bit: u32;

        if lo >= hi {
            return a_end;
        }
        if start < lo {
            start = lo;
        }
        if start >= hi {
            return a_end;
        }

        last_wi = (hi - 1) / 64;
        start_wi = start / 64;
        start_bit = start & 63;

        for i in 0..CMASK_MAX_WORDS {
            let abs_wi = start_wi + i;
            let mut word: u64;
            let found: u32;

            if abs_wi > last_wi {
                break;
            }

            word = *(*a).bits.as_ptr().add((abs_wi - a_wbase) as usize)
                & *(*b).bits.as_ptr().add((abs_wi - b_wbase) as usize);
            if i == 0 {
                word &= GENMASK_U64(63, start_bit);
            }
            if word == 0 {
                continue;
            }

            found = abs_wi * 64 + ctzll(word);
            if found >= hi {
                return a_end;
            }
            return found;
        }
        a_end
    }
}

/*
 * Find the next set cid in @m at or after @start, wrapping to @m->base if no
 * set bit is found in [start, m->base + m->nr_cids). Return m->base +
 * m->nr_cids if @m is empty.
 *
 * Callers do round-robin distribution by passing (last_cid + 1) as @start.
 */
#[inline(always)]
pub unsafe fn cmask_next_set_wrap(m: *const scx_cmask, start: u32) -> u32 {
    unsafe {
        let end = (*m).base + (*m).nr_cids;
        let mut found: u32;

        found = cmask_next_set(m, start);
        if found < end || start <= (*m).base {
            return found;
        }

        found = cmask_next_set(m, (*m).base);
        if found < start { found } else { end }
    }
}

/*
 * Find the next cid set in both @a and @b at or after @start, wrapping to
 * @a->base if none found in the forward half. Return a->base + a->nr_cids
 * if the intersection is empty.
 *
 * Callers do round-robin distribution by passing (last_cid + 1) as @start.
 */
#[inline(always)]
pub unsafe fn cmask_next_and_set_wrap(a: *const scx_cmask, b: *const scx_cmask, start: u32) -> u32 {
    unsafe {
        let a_end = (*a).base + (*a).nr_cids;
        let mut found: u32;

        found = cmask_next_and_set(a, b, start);
        if found < a_end || start <= (*a).base {
            return found;
        }

        found = cmask_next_and_set(a, b, (*a).base);
        if found < start { found } else { a_end }
    }
}

/*
 * Like cmask_next_and_set() but over the intersection of THREE masks. Return
 * a->base + a->nr_cids if no cid is set in all three at or after @start.
 */
#[inline(always)]
pub unsafe fn cmask_next_and2_set(
    a: *const scx_cmask,
    b: *const scx_cmask,
    c: *const scx_cmask,
    mut start: u32,
) -> u32 {
    unsafe {
        let a_end = (*a).base + (*a).nr_cids;
        let b_end = (*b).base + (*b).nr_cids;
        let c_end = (*c).base + (*c).nr_cids;
        let a_wbase = (*a).base / 64;
        let b_wbase = (*b).base / 64;
        let c_wbase = (*c).base / 64;
        let mut lo = if (*a).base > (*b).base { (*a).base } else { (*b).base };
        let mut hi = if a_end < b_end { a_end } else { b_end };
        let last_wi: u32;
        let start_wi: u32;
        let start_bit: u32;

        lo = if lo > (*c).base { lo } else { (*c).base };
        hi = if hi < c_end { hi } else { c_end };

        if lo >= hi {
            return a_end;
        }
        if start < lo {
            start = lo;
        }
        if start >= hi {
            return a_end;
        }

        last_wi = (hi - 1) / 64;
        start_wi = start / 64;
        start_bit = start & 63;

        for i in 0..CMASK_MAX_WORDS {
            let abs_wi = start_wi + i;
            let mut word: u64;
            let found: u32;

            if abs_wi > last_wi {
                break;
            }

            word = *(*a).bits.as_ptr().add((abs_wi - a_wbase) as usize)
                & *(*b).bits.as_ptr().add((abs_wi - b_wbase) as usize)
                & *(*c).bits.as_ptr().add((abs_wi - c_wbase) as usize);
            if i == 0 {
                word &= GENMASK_U64(63, start_bit);
            }
            if word == 0 {
                continue;
            }

            found = abs_wi * 64 + ctzll(word);
            if found >= hi {
                return a_end;
            }
            return found;
        }
        a_end
    }
}

/*
 * Round-robin variant of cmask_next_and2_set(): wrap to @a->base if the
 * three-way intersection has no cid in the forward half. Return a->base +
 * a->nr_cids if empty.
 */
#[inline(always)]
pub unsafe fn cmask_next_and2_set_wrap(
    a: *const scx_cmask,
    b: *const scx_cmask,
    c: *const scx_cmask,
    start: u32,
) -> u32 {
    unsafe {
        let a_end = (*a).base + (*a).nr_cids;
        let mut found: u32;

        found = cmask_next_and2_set(a, b, c, start);
        if found < a_end || start <= (*a).base {
            return found;
        }

        found = cmask_next_and2_set(a, b, c, (*a).base);
        if found < start { found } else { a_end }
    }
}

/**
 * cmask_from_cpumask - translate a kernel cpumask to a cid-space cmask
 * @m: cmask to fill. Zeroed first; only bits within [@m->base, @m->base +
 *     @m->nr_cids) are updated - cpus mapping to cids outside that range
 *     are ignored.
 * @cpumask: kernel cpumask to translate
 *
 * For each cpu in @cpumask, set the cpu's cid in @m. Caller must ensure
 * @cpumask stays stable across the call (e.g. RCU read lock for
 * task->cpus_ptr).
 */
#[inline(always)]
pub unsafe fn cmask_from_cpumask(m: *mut scx_cmask, cpumask: *const cpumask) {
    unsafe {
        let nr_cpu_ids = scx_bpf_nr_cpu_ids();

        cmask_zero(m);
        for cpu in 0..nr_cpu_ids {
            let cid: i32;

            if !bpf_cpumask_test_cpu(cpu as i32, cpumask) {
                continue;
            }
            cid = scx_bpf_cpu_to_cid(cpu as i32);
            if cid >= 0 {
                __cmask_set(cid as u32, m);
            }
        }
    }
}
