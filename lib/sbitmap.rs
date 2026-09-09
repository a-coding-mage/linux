// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2016 Facebook
 * Copyright (C) 2013-2014 Jens Axboe
 *
 * Linux kernel dependencies, types, constants, macros, and primitives used
 * below are supplied externally by the corresponding kernel headers.
 */

unsafe fn init_alloc_hint(sb: *mut sbitmap, flags: gfp_t) -> i32 {
    let depth = (*sb).depth;
    (*sb).alloc_hint = alloc_percpu_gfp::<u32>(flags);
    if (*sb).alloc_hint.is_null() { return -ENOMEM; }
    if depth != 0 && !(*sb).round_robin {
        for i in for_each_possible_cpu() {
            *per_cpu_ptr((*sb).alloc_hint, i) = get_random_u32_below(depth);
        }
    }
    0
}

unsafe fn update_alloc_hint_before_get(sb: *mut sbitmap, depth: u32) -> u32 {
    let mut hint = this_cpu_read((*sb).alloc_hint);
    if hint >= depth {
        hint = if depth != 0 { get_random_u32_below(depth) } else { 0 };
        this_cpu_write((*sb).alloc_hint, hint);
    }
    hint
}

unsafe fn update_alloc_hint_after_get(sb: *mut sbitmap, depth: u32, mut hint: u32, nr: u32) {
    if nr == u32::MAX {
        this_cpu_write((*sb).alloc_hint, 0);
    } else if nr == hint || (*sb).round_robin {
        hint = nr + 1;
        if hint >= depth - 1 { hint = 0; }
        this_cpu_write((*sb).alloc_hint, hint);
    }
}

unsafe fn sbitmap_deferred_clear(map: *mut sbitmap_word, depth: u32, alloc_hint: u32, wrap: bool) -> bool {
    let _lock = raw_spinlock_irqsave(&mut (*map).swap_lock);
    if (*map).cleared == 0 {
        if depth == 0 { return false; }
        let mut word_mask = (!0usize) >> (BITS_PER_LONG - depth);
        if !wrap && alloc_hint != 0 { word_mask &= !((1usize << alloc_hint) - 1); }
        return ((*map).word & word_mask) != word_mask;
    }
    let mask = xchg(&mut (*map).cleared, 0);
    atomic_long_andnot(mask, &mut (*map).word);
    true
}

pub unsafe fn sbitmap_init_node(sb: *mut sbitmap, depth: u32, mut shift: i32, flags: gfp_t, node: i32, round_robin: bool, alloc_hint: bool) -> i32 {
    if shift < 0 { shift = sbitmap_calculate_shift(depth); }
    let bits_per_word = 1u32 << shift;
    if bits_per_word > BITS_PER_LONG as u32 { return -EINVAL; }
    (*sb).shift = shift;
    (*sb).depth = depth;
    (*sb).map_nr = (depth + bits_per_word - 1) / bits_per_word;
    (*sb).round_robin = round_robin;
    if depth == 0 { (*sb).map = core::ptr::null_mut(); return 0; }
    if alloc_hint {
        if init_alloc_hint(sb, flags) != 0 { return -ENOMEM; }
    } else { (*sb).alloc_hint = core::ptr::null_mut(); }
    (*sb).map = kvzalloc_node((*sb).map_nr, flags, node);
    if (*sb).map.is_null() { free_percpu((*sb).alloc_hint); return -ENOMEM; }
    for i in 0..(*sb).map_nr { raw_spin_lock_init(&mut (*sb).map.add(i as usize).as_mut().unwrap().swap_lock); }
    0
}

pub unsafe fn sbitmap_resize(sb: *mut sbitmap, depth: u32) {
    let bits_per_word = 1u32 << (*sb).shift;
    for i in 0..(*sb).map_nr { sbitmap_deferred_clear((*sb).map.add(i as usize), 0, 0, false); }
    (*sb).depth = depth;
    (*sb).map_nr = (depth + bits_per_word - 1) / bits_per_word;
}

unsafe fn __sbitmap_get_word(word: *mut usize, depth: usize, mut hint: u32, mut wrap: bool) -> i32 {
    let mut nr: i32;
    wrap = wrap && hint != 0;
    loop {
        nr = find_next_zero_bit(word, depth, hint as usize) as i32;
        if nr as usize >= depth {
            if hint != 0 && wrap { hint = 0; continue; }
            return -1;
        }
        if !test_and_set_bit_lock(nr as usize, word) { break; }
        hint = nr as u32 + 1;
        if hint >= depth as u32 - 1 { hint = 0; }
    }
    nr
}

unsafe fn sbitmap_find_bit_in_word(map: *mut sbitmap_word, depth: u32, alloc_hint: u32, wrap: bool) -> i32 {
    loop {
        let nr = __sbitmap_get_word(&mut (*map).word, depth as usize, alloc_hint, wrap);
        if nr != -1 || !sbitmap_deferred_clear(map, depth, alloc_hint, wrap) { return nr; }
    }
}

unsafe fn __map_depth_with_shallow(sb: *const sbitmap, index: i32, shallow_depth: u32) -> u32 {
    let word_depth = __map_depth(sb, index);
    if shallow_depth >= (*sb).depth { return word_depth; }
    let mut shallow_word_depth = word_depth as u64 * shallow_depth as u64;
    let reminder = shallow_word_depth % (*sb).depth as u64;
    shallow_word_depth /= (*sb).depth as u64;
    if reminder >= ((index + 1) as u32 * word_depth) as u64 { shallow_word_depth += 1; }
    shallow_word_depth as u32
}

unsafe fn sbitmap_find_bit(sb: *mut sbitmap, shallow_depth: u32, mut index: u32, mut alloc_hint: u32, wrap: bool) -> i32 {
    let mut nr = -1;
    for _ in 0..(*sb).map_nr {
        let depth = __map_depth_with_shallow(sb, index as i32, shallow_depth);
        if depth != 0 { nr = sbitmap_find_bit_in_word((*sb).map.add(index as usize), depth, alloc_hint, wrap); }
        if nr != -1 { return nr + (index as i32 << (*sb).shift); }
        alloc_hint = 0;
        index += 1;
        if index >= (*sb).map_nr { index = 0; }
    }
    nr
}

unsafe fn __sbitmap_get(sb: *mut sbitmap, alloc_hint: u32) -> i32 {
    let index = SB_NR_TO_INDEX(sb, alloc_hint);
    let hint = if (*sb).round_robin { SB_NR_TO_BIT(sb, alloc_hint) } else { 0 };
    sbitmap_find_bit(sb, u32::MAX, index, hint, !(*sb).round_robin)
}

pub unsafe fn sbitmap_get(sb: *mut sbitmap) -> i32 {
    if (*sb).alloc_hint.is_null() { return -1; }
    let depth = READ_ONCE((*sb).depth);
    let hint = update_alloc_hint_before_get(sb, depth);
    let nr = __sbitmap_get(sb, hint);
    update_alloc_hint_after_get(sb, depth, hint, nr as u32);
    nr
}

unsafe fn __sbitmap_get_shallow(sb: *mut sbitmap, alloc_hint: u32, shallow_depth: usize) -> i32 {
    let index = SB_NR_TO_INDEX(sb, alloc_hint);
    sbitmap_find_bit(sb, shallow_depth as u32, index, SB_NR_TO_BIT(sb, alloc_hint), true)
}

unsafe fn sbitmap_get_shallow(sb: *mut sbitmap, shallow_depth: usize) -> i32 {
    if (*sb).alloc_hint.is_null() { return -1; }
    let depth = READ_ONCE((*sb).depth);
    let hint = update_alloc_hint_before_get(sb, depth);
    let nr = __sbitmap_get_shallow(sb, hint, shallow_depth);
    update_alloc_hint_after_get(sb, depth, hint, nr as u32);
    nr
}

pub unsafe fn sbitmap_any_bit_set(sb: *const sbitmap) -> bool {
    for i in 0..(*sb).map_nr { let m = &*(*sb).map.add(i as usize); if m.word & !m.cleared != 0 { return true; } }
    false
}

unsafe fn __sbitmap_weight(sb: *const sbitmap, set: bool) -> u32 {
    let mut weight = 0;
    for i in 0..(*sb).map_nr { let word = &*(*sb).map.add(i as usize); let d = __map_depth(sb, i as i32); weight += bitmap_weight(if set { &word.word } else { &word.cleared }, d); }
    weight
}
unsafe fn sbitmap_cleared(sb: *const sbitmap) -> u32 { __sbitmap_weight(sb, false) }
pub unsafe fn sbitmap_weight(sb: *const sbitmap) -> u32 { __sbitmap_weight(sb, true) - sbitmap_cleared(sb) }

pub unsafe fn sbitmap_show(sb: *mut sbitmap, m: *mut seq_file) {
    seq_printf(m, "depth=%u\n", (*sb).depth); seq_printf(m, "busy=%u\n", sbitmap_weight(sb));
    seq_printf(m, "cleared=%u\n", sbitmap_cleared(sb)); seq_printf(m, "bits_per_word=%u\n", 1u32 << (*sb).shift);
    seq_printf(m, "map_nr=%u\n", (*sb).map_nr);
}

unsafe fn emit_byte(m: *mut seq_file, offset: u32, byte: u8) {
    if offset & 0xf == 0 { if offset != 0 { seq_putc(m, '\n'); } seq_printf(m, "%08x:", offset); }
    if offset & 1 == 0 { seq_putc(m, ' '); } seq_printf(m, "%02x", byte);
}

pub unsafe fn sbitmap_bitmap_show(sb: *mut sbitmap, m: *mut seq_file) {
    let mut byte = 0u8; let mut byte_bits = 0u32; let mut offset = 0u32;
    for i in 0..(*sb).map_nr { let map = &*(*sb).map.add(i as usize); let mut word = READ_ONCE(map.word) & !READ_ONCE(map.cleared); let mut word_bits = __map_depth(sb, i as i32);
        while word_bits > 0 { let bits = core::cmp::min(8 - byte_bits, word_bits); byte |= ((word & ((1usize << bits) - 1)) << byte_bits) as u8; byte_bits += bits; if byte_bits == 8 { emit_byte(m, offset, byte); byte=0; byte_bits=0; offset+=1; } word >>= bits; word_bits -= bits; }
    }
    if byte_bits != 0 { emit_byte(m, offset, byte); offset += 1; } if offset != 0 { seq_putc(m, '\n'); }
}

// Remaining queue operations retain their kernel synchronization and waitqueue
// semantics; their declarations and field types are supplied by linux/sbitmap.h.
pub unsafe fn __sbitmap_queue_get(sbq: *mut sbitmap_queue) -> i32 { sbitmap_get(&mut (*sbq).sb) }
pub unsafe fn sbitmap_queue_get_shallow(sbq: *mut sbitmap_queue, shallow_depth: u32) -> i32 { sbitmap_get_shallow(&mut (*sbq).sb, shallow_depth as usize) }

pub unsafe fn sbitmap_queue_init_node(sbq: *mut sbitmap_queue, depth: u32, shift: i32, round_robin: bool, flags: gfp_t, node: i32) -> i32 {
    let ret = sbitmap_init_node(&mut (*sbq).sb, depth, shift, flags, node, round_robin, true); if ret != 0 { return ret; }
    (*sbq).min_shallow_depth = u32::MAX; (*sbq).wake_batch = sbq_calc_wake_batch(sbq, depth);
    atomic_set(&mut (*sbq).wake_index, 0); atomic_set(&mut (*sbq).ws_active, 0); atomic_set(&mut (*sbq).completion_cnt, 0); atomic_set(&mut (*sbq).wakeup_cnt, 0);
    (*sbq).ws = kzalloc_node(SBQ_WAIT_QUEUES, flags, node); if (*sbq).ws.is_null() { sbitmap_free(&mut (*sbq).sb); return -ENOMEM; }
    for i in 0..SBQ_WAIT_QUEUES { init_waitqueue_head(&mut (*sbq).ws.add(i).as_mut().unwrap().wait); } 0
}
unsafe fn sbq_calc_wake_batch(sbq: *mut sbitmap_queue, depth: u32) -> u32 { clamp((core::cmp::min(depth, (*sbq).min_shallow_depth) / SBQ_WAIT_QUEUES as u32), 1, SBQ_WAKE_BATCH) }
unsafe fn sbitmap_queue_update_wake_batch(sbq: *mut sbitmap_queue, depth: u32) { (*sbq).wake_batch = sbq_calc_wake_batch(sbq, depth); }
pub unsafe fn sbitmap_queue_resize(sbq: *mut sbitmap_queue, depth: u32) { sbitmap_queue_update_wake_batch(sbq, depth); sbitmap_resize(&mut (*sbq).sb, depth); }
pub unsafe fn sbitmap_queue_min_shallow_depth(sbq: *mut sbitmap_queue, d: u32) { (*sbq).min_shallow_depth=d; sbitmap_queue_update_wake_batch(sbq, (*sbq).sb.depth); }
pub unsafe fn sbitmap_queue_recalculate_wake_batch(sbq: *mut sbitmap_queue, users: u32) { (*sbq).wake_batch = clamp(((*sbq).sb.depth + users - 1) / users / SBQ_WAIT_QUEUES as u32, 1, SBQ_WAKE_BATCH); }
pub unsafe fn sbitmap_queue_wake_up(sbq: *mut sbitmap_queue, nr: i32) { if atomic_read(&(*sbq).ws_active)==0{return;} atomic_add(nr,&mut (*sbq).completion_cnt); let mut wakeups=atomic_read(&(*sbq).wakeup_cnt); loop { if atomic_read(&(*sbq).completion_cnt)-wakeups < (*sbq).wake_batch as i32{return;} if atomic_try_cmpxchg(&mut (*sbq).wakeup_cnt,&mut wakeups,wakeups+(*sbq).wake_batch as i32){break;} } __sbitmap_queue_wake_up(sbq,(*sbq).wake_batch as i32); }
unsafe fn __sbitmap_queue_wake_up(sbq: *mut sbitmap_queue, nr: i32) { if atomic_read(&(*sbq).ws_active)==0{return;} let mut wi=atomic_read(&(*sbq).wake_index); let mut left=nr; for _ in 0..SBQ_WAIT_QUEUES { let ws=&mut *(*sbq).ws.add(wi as usize); wi=sbq_index_inc(wi); if waitqueue_active(&ws.wait){let w=wake_up_nr(&mut ws.wait,left); if w==left{break;} left-=w;} } atomic_set(&mut (*sbq).wake_index,wi); }
pub unsafe fn sbitmap_queue_wake_all(sbq: *mut sbitmap_queue) { smp_mb(); let mut wi=atomic_read(&(*sbq).wake_index); for _ in 0..SBQ_WAIT_QUEUES { let ws=&mut *(*sbq).ws.add(wi as usize); if waitqueue_active(&ws.wait){wake_up(&mut ws.wait);} wi=sbq_index_inc(wi); } }
pub unsafe fn sbitmap_queue_clear(sbq:*mut sbitmap_queue,nr:u32,cpu:u32){smp_mb__before_atomic();sbitmap_deferred_clear_bit(&mut (*sbq).sb,nr);smp_mb__after_atomic();sbitmap_queue_wake_up(sbq,1);sbitmap_update_cpu_hint(&mut (*sbq).sb,cpu,nr);}
unsafe fn sbitmap_update_cpu_hint(sb:*mut sbitmap,cpu:u32,tag:u32){if !(*sb).round_robin&&tag<(*sb).depth{*per_cpu_ptr((*sb).alloc_hint,cpu)=tag;}}
pub unsafe fn sbitmap_add_wait_queue(sbq:*mut sbitmap_queue,ws:*mut sbq_wait_state,w:*mut sbq_wait){if (*w).sbq.is_null(){(*w).sbq=sbq;atomic_inc(&mut (*sbq).ws_active);add_wait_queue(&mut (*ws).wait,&mut (*w).wait);}}
pub unsafe fn sbitmap_del_wait_queue(w:*mut sbq_wait){list_del_init(&mut (*w).wait.entry);if !(*w).sbq.is_null(){atomic_dec(&mut (*(*w).sbq).ws_active);(*w).sbq=core::ptr::null_mut();}}
pub unsafe fn sbitmap_prepare_to_wait(sbq:*mut sbitmap_queue,ws:*mut sbq_wait_state,w:*mut sbq_wait,state:i32){if (*w).sbq.is_null(){atomic_inc(&mut (*sbq).ws_active);(*w).sbq=sbq;}prepare_to_wait_exclusive(&mut (*ws).wait,&mut (*w).wait,state);}
pub unsafe fn sbitmap_finish_wait(sbq:*mut sbitmap_queue,ws:*mut sbq_wait_state,w:*mut sbq_wait){finish_wait(&mut (*ws).wait,&mut (*w).wait);if !(*w).sbq.is_null(){atomic_dec(&mut (*sbq).ws_active);(*w).sbq=core::ptr::null_mut();}}
pub unsafe fn sbitmap_queue_clear_batch(sbq:*mut sbitmap_queue,offset:i32,tags:*const i32,nr_tags:i32){smp_mb__before_atomic();for i in 0..nr_tags{let tag=*tags.add(i as usize)-offset;let map=&mut *(*sbq).sb.map.add(SB_NR_TO_INDEX(&mut (*sbq).sb,tag as u32) as usize);atomic_long_andnot(1usize<<SB_NR_TO_BIT(&mut (*sbq).sb,tag as u32),&mut map.word);}smp_mb__after_atomic();sbitmap_queue_wake_up(sbq,nr_tags);sbitmap_update_cpu_hint(&mut (*sbq).sb,raw_smp_processor_id(),(*tags.add((nr_tags-1) as usize)-offset) as u32);}
pub unsafe fn sbitmap_queue_show(sbq:*mut sbitmap_queue,m:*mut seq_file){sbitmap_show(&mut (*sbq).sb,m);seq_printf(m,"wake_batch=%u\n",(*sbq).wake_batch);seq_printf(m,"wake_index=%d\n",atomic_read(&(*sbq).wake_index));seq_printf(m,"ws_active=%d\n",atomic_read(&(*sbq).ws_active));seq_printf(m,"round_robin=%d\n",(*sbq).sb.round_robin as i32);seq_printf(m,"min_shallow_depth=%u\n",(*sbq).min_shallow_depth);}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
