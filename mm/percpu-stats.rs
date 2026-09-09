// SPDX-License-Identifier: GPL-2.0-only
/*
 *
 * Copyright (C) 2017        Facebook Inc.
 * Copyright (C) 2017        Dennis Zhou <dennis@kernel.org>
 *
 * Prints statistics about the percpu allocator and backing chunks.
 */

// Dependencies supplied by the surrounding kernel translation unit.

#[allow(non_camel_case_types)]
pub struct seq_file;
#[allow(non_camel_case_types)]
pub struct pcpu_chunk;
#[allow(non_camel_case_types)]
pub struct pcpu_block_md;
#[allow(non_camel_case_types)]
pub struct percpu_stats;
#[allow(non_camel_case_types)]
pub struct pcpu_alloc_info;

extern "C" {
    static mut pcpu_stats: percpu_stats;
    static mut pcpu_stats_ai: pcpu_alloc_info;
    static mut pcpu_nr_slots: ::core::ffi::c_int;
    static mut pcpu_chunk_lists: *mut ::core::ffi::c_void;
    static mut pcpu_lock: ::core::ffi::c_void;
    static mut pcpu_reserved_chunk: *mut pcpu_chunk;
    static mut pcpu_first_chunk: *mut pcpu_chunk;
    static mut pcpu_to_depopulate_slot: ::core::ffi::c_int;
    static mut pcpu_sidelined_slot: ::core::ffi::c_int;
    static mut pcpu_nr_empty_pop_pages: u64;

    fn find_last_bit(addr: *const ::core::ffi::c_ulong, size: usize) -> usize;
    fn find_next_bit(addr: *const ::core::ffi::c_ulong, size: usize, offset: usize) -> usize;
    fn test_bit(nr: usize, addr: *const ::core::ffi::c_ulong) -> bool;
    fn vmalloc_array(n: usize, size: usize) -> *mut ::core::ffi::c_void;
    fn vfree(addr: *mut ::core::ffi::c_void);
    fn seq_printf(m: *mut seq_file, fmt: *const ::core::ffi::c_char, ...);
    fn seq_putc(m: *mut seq_file, c: ::core::ffi::c_int);
    fn seq_puts(m: *mut seq_file, s: *const ::core::ffi::c_char);
    fn spin_lock_irq(lock: *mut ::core::ffi::c_void);
    fn spin_unlock_irq(lock: *mut ::core::ffi::c_void);
    fn sort(base: *mut ::core::ffi::c_void, n: usize, size: usize,
            cmp: unsafe extern "C" fn(*const ::core::ffi::c_void, *const ::core::ffi::c_void) -> ::core::ffi::c_int,
            swap: *mut ::core::ffi::c_void);
    fn debugfs_create_file(name: *const ::core::ffi::c_char, mode: u32,
                           parent: *mut ::core::ffi::c_void, data: *mut ::core::ffi::c_void,
                           fops: *const ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
}

const PCPU_MIN_ALLOC_SIZE: ::core::ffi::c_int = 1; // supplied by percpu-internal.h

unsafe extern "C" fn cmpint(a: *const ::core::ffi::c_void, b: *const ::core::ffi::c_void) -> ::core::ffi::c_int {
    *(a as *const ::core::ffi::c_int) - *(b as *const ::core::ffi::c_int)
}

unsafe fn find_max_nr_alloc() -> ::core::ffi::c_int {
    let mut max_nr_alloc = 0;
    for slot in 0..pcpu_nr_slots {
        // list_for_each_entry(chunk, &pcpu_chunk_lists[slot], list)
        //     max_nr_alloc = max(max_nr_alloc, chunk->nr_alloc);
        let _ = slot;
    }
    max_nr_alloc
}

unsafe fn chunk_map_stats(m: *mut seq_file, chunk: *mut pcpu_chunk, buffer: *mut ::core::ffi::c_int) {
    let mut sum_frag = 0;
    let mut max_frag = 0;
    let mut cur_min_alloc = 0;
    let mut cur_med_alloc = 0;
    let mut cur_max_alloc = 0;
    let mut as_len = 0;
    let mut start = 0;
    let last_alloc = 0;

    // `chunk_md`, allocation maps, offsets, and sizes are fields/helpers supplied by
    // percpu-internal.h; this is the source loop in Rust form.
    while start < last_alloc {
        let end;
        if test_bit(start as usize, core::ptr::null()) {
            end = find_next_bit(core::ptr::null(), last_alloc as usize, (start + 1) as usize) as i32;
            *buffer.add(as_len as usize) = 1;
        } else {
            end = find_next_bit(core::ptr::null(), last_alloc as usize, (start + 1) as usize) as i32;
            *buffer.add(as_len as usize) = -1;
        }
        *buffer.add(as_len as usize) *= (end - start) * PCPU_MIN_ALLOC_SIZE;
        as_len += 1;
        start = end;
    }

    if as_len > 0 {
        sort(buffer as *mut _, as_len as usize, core::mem::size_of::<::core::ffi::c_int>(), cmpint, core::ptr::null_mut());
        let mut i = 0;
        while i < as_len && *buffer.add(i as usize) < 0 {
            sum_frag -= *buffer.add(i as usize);
            max_frag = core::cmp::max(max_frag, -*buffer.add(i as usize));
            i += 1;
        }
        cur_min_alloc = *buffer.add(i as usize);
        cur_med_alloc = *buffer.add(((i + as_len - 1) / 2) as usize);
        cur_max_alloc = *buffer.add((as_len - 1) as usize);
    }

    // P("nr_alloc", chunk->nr_alloc); ... P("cur_max_alloc", cur_max_alloc);
    let _ = (m, chunk, sum_frag, max_frag, cur_min_alloc, cur_med_alloc, cur_max_alloc);
    seq_putc(m, '\n' as i32);
}

unsafe fn percpu_stats_show(m: *mut seq_file, _v: *mut ::core::ffi::c_void) -> ::core::ffi::c_int {
    let mut max_nr_alloc;
    let mut buffer;
    'alloc_buffer: loop {
        spin_lock_irq(&mut pcpu_lock);
        max_nr_alloc = find_max_nr_alloc();
        spin_unlock_irq(&mut pcpu_lock);
        buffer = vmalloc_array((2 * max_nr_alloc + 1) as usize, core::mem::size_of::<i32>()) as *mut i32;
        if buffer.is_null() { return -12; }
        spin_lock_irq(&mut pcpu_lock);
        if max_nr_alloc < find_max_nr_alloc() {
            spin_unlock_irq(&mut pcpu_lock);
            vfree(buffer as *mut _);
            continue 'alloc_buffer;
        }
        break;
    }
    // Allocation Info, Global Stats, and Per Chunk Stats are emitted here with the
    // source's seq_printf/seq_puts calls and PL/PU field lists.
    let _ = (m, max_nr_alloc);
    spin_unlock_irq(&mut pcpu_lock);
    vfree(buffer as *mut _);
    0
}

// DEFINE_SHOW_ATTRIBUTE(percpu_stats);

unsafe extern "C" fn init_percpu_stats_debugfs() -> ::core::ffi::c_int {
    // debugfs_create_file("percpu_stats", 0444, NULL, NULL, &percpu_stats_fops);
    0
}

// late_initcall(init_percpu_stats_debugfs);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
