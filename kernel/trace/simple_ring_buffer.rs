// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2025 - Google LLC
 * Author: Vincent Donnefort <vdonnefort@google.com>
 */

// External Linux/kernel definitions and helpers are supplied by other files.

#[repr(C)]
#[derive(Copy, Clone)]
enum simple_rb_link_type {
    SIMPLE_RB_LINK_NORMAL = 0,
    SIMPLE_RB_LINK_HEAD = 1,
    SIMPLE_RB_LINK_HEAD_MOVING,
}

const SIMPLE_RB_LINK_MASK: usize = !(SIMPLE_RB_LINK_HEAD as usize | SIMPLE_RB_LINK_HEAD_MOVING as usize);

unsafe fn simple_bpage_set_head_link(bpage: *mut simple_buffer_page) {
    let mut link = (*bpage).link.next as usize;
    link &= SIMPLE_RB_LINK_MASK;
    link |= SIMPLE_RB_LINK_HEAD as usize;
    smp_store_release(&mut (*bpage).link.next, link as *mut list_head);
}

unsafe fn simple_bpage_unset_head_link(
    bpage: *mut simple_buffer_page,
    dst: *mut simple_buffer_page,
    new_type: simple_rb_link_type,
) -> bool {
    let link = &mut (*bpage).link.next as *mut *mut list_head as *mut usize;
    let mut old = (*link & SIMPLE_RB_LINK_MASK) | SIMPLE_RB_LINK_HEAD as usize;
    let new = (&mut (*dst).link as *mut list_head as usize) | new_type as usize;
    try_cmpxchg(link, &mut old, new)
}

unsafe fn simple_bpage_set_normal_link(bpage: *mut simple_buffer_page) {
    let link = (*bpage).link.next as usize;
    write_once(&mut (*bpage).link.next, (link & SIMPLE_RB_LINK_MASK) as *mut list_head);
}

unsafe fn simple_bpage_from_link(link: *mut list_head) -> *mut simple_buffer_page {
    let ptr = link as usize & SIMPLE_RB_LINK_MASK;
    container_of(ptr as *mut list_head, simple_buffer_page, link)
}

unsafe fn simple_bpage_next_page(bpage: *mut simple_buffer_page) -> *mut simple_buffer_page {
    simple_bpage_from_link((*bpage).link.next)
}

unsafe fn simple_bpage_reset(bpage: *mut simple_buffer_page) {
    (*bpage).write = 0;
    (*bpage).entries = 0;
    local_set(&mut (*(*bpage).page).commit, 0);
}

unsafe fn simple_bpage_init(bpage: *mut simple_buffer_page, page: *mut core::ffi::c_void) {
    INIT_LIST_HEAD(&mut (*bpage).link);
    (*bpage).page = page as *mut buffer_data_page;
    simple_bpage_reset(bpage);
}

unsafe fn simple_rb_meta_inc(meta: *mut u64, inc: u64) {
    write_once(meta, (*meta).wrapping_add(inc));
}

unsafe fn simple_rb_loaded(cpu_buffer: *mut simple_rb_per_cpu) -> bool {
    !(*cpu_buffer).bpages.is_null()
}

unsafe fn simple_rb_find_head(cpu_buffer: *mut simple_rb_per_cpu) -> i32 {
    let mut retry = (*cpu_buffer).nr_pages * 2;
    let mut head = (*cpu_buffer).head_page;
    while retry != 0 {
        retry -= 1;
        let link = smp_load_acquire((*head).link.prev).next as usize;
        match link & !SIMPLE_RB_LINK_MASK {
            x if x == SIMPLE_RB_LINK_HEAD as usize => {
                (*cpu_buffer).head_page = head;
                return 0;
            }
            x if x == SIMPLE_RB_LINK_HEAD_MOVING as usize => continue,
            _ => head = simple_bpage_next_page(head),
        }
    }
    -EBUSY
}

pub unsafe fn simple_ring_buffer_swap_reader_page(cpu_buffer: *mut simple_rb_per_cpu) -> i32 {
    if !simple_rb_loaded(cpu_buffer) { return -ENODEV; }
    let reader = (*cpu_buffer).reader_page;
    let mut retry = 8;
    let overrun;
    loop {
        let ret = simple_rb_find_head(cpu_buffer);
        if ret != 0 { return ret; }
        let head = (*cpu_buffer).head_page;
        (*reader).link.next = (*head).link.next;
        (*reader).link.prev = (*head).link.prev;
        let last = simple_bpage_from_link((*head).link.prev);
        simple_bpage_set_head_link(reader);
        overrun = (*(*cpu_buffer).meta).overrun;
        if simple_bpage_unset_head_link(last, reader, simple_rb_link_type::SIMPLE_RB_LINK_NORMAL) || retry == 0 { break; }
        retry -= 1;
    }
    if retry == 0 { return -EINVAL; }
    (*cpu_buffer).head_page = simple_bpage_from_link((*reader).link.next);
    (*(*cpu_buffer).head_page).link.prev = &mut (*reader).link;
    (*cpu_buffer).reader_page = (*cpu_buffer).head_page;
    (*(*cpu_buffer).meta).reader.lost_events = overrun - (*cpu_buffer).last_overrun;
    (*(*cpu_buffer).meta).reader.id = (*(*cpu_buffer).reader_page).id;
    (*cpu_buffer).last_overrun = overrun;
    0
}

unsafe fn simple_rb_move_tail(cpu_buffer: *mut simple_rb_per_cpu) -> *mut simple_buffer_page {
    let tail = (*cpu_buffer).tail_page;
    let new_tail = simple_bpage_next_page(tail);
    if simple_bpage_unset_head_link(tail, new_tail, simple_rb_link_type::SIMPLE_RB_LINK_HEAD_MOVING) {
        simple_rb_meta_inc(&mut (*(*cpu_buffer).meta).overrun, (*new_tail).entries);
        simple_rb_meta_inc(&mut (*(*cpu_buffer).meta).pages_lost, 1);
        simple_bpage_set_head_link(new_tail);
        simple_bpage_set_normal_link(tail);
    }
    simple_bpage_reset(new_tail);
    (*cpu_buffer).tail_page = new_tail;
    simple_rb_meta_inc(&mut (*(*cpu_buffer).meta).pages_touched, 1);
    new_tail
}

unsafe fn rb_event_size(length: usize) -> usize { length + RB_EVNT_HDR_SIZE + core::mem::size_of::<u32>() }

unsafe fn rb_event_add_ts_extend(event: *mut ring_buffer_event, delta: u64) -> *mut ring_buffer_event {
    (*event).type_len = RINGBUF_TYPE_TIME_EXTEND;
    (*event).time_delta = delta & TS_MASK;
    (*event).array[0] = delta >> TS_SHIFT;
    (event as usize + 8) as *mut ring_buffer_event
}

unsafe fn simple_rb_reserve_next(cpu_buffer: *mut simple_rb_per_cpu, length: usize, timestamp: u64) -> *mut ring_buffer_event {
    let mut ts_ext_size = 0;
    let event_size = rb_event_size(length);
    let mut tail = (*cpu_buffer).tail_page;
    let mut time_delta = timestamp - (*cpu_buffer).write_stamp;
    if test_time_stamp(time_delta) { ts_ext_size = 8; }
    let mut prev_write = (*tail).write;
    let mut write = prev_write + event_size + ts_ext_size;
    if unlikely(write > PAGE_SIZE - BUF_PAGE_HDR_SIZE) { tail = simple_rb_move_tail(cpu_buffer); }
    if (*tail).entries == 0 {
        (*(*tail).page).time_stamp = timestamp;
        time_delta = 0;
        ts_ext_size = 0;
        write = event_size;
        prev_write = 0;
    }
    (*tail).write = write;
    (*tail).entries += 1;
    (*cpu_buffer).write_stamp = timestamp;
    let mut event = ((*tail).page).data.as_mut_ptr().add(prev_write) as *mut ring_buffer_event;
    if ts_ext_size != 0 { event = rb_event_add_ts_extend(event, time_delta); }
    (*event).type_len = 0;
    (*event).time_delta = time_delta;
    (*event).array[0] = event_size - RB_EVNT_HDR_SIZE;
    event
}

pub unsafe fn simple_ring_buffer_reserve(cpu_buffer: *mut simple_rb_per_cpu, length: usize, timestamp: u64) -> *mut core::ffi::c_void {
    if cmpxchg(&mut (*cpu_buffer).status, SIMPLE_RB_READY, SIMPLE_RB_WRITING) != SIMPLE_RB_READY { return core::ptr::null_mut(); }
    let event = simple_rb_reserve_next(cpu_buffer, length, timestamp);
    &mut (*event).array[1] as *mut _ as *mut core::ffi::c_void
}

pub unsafe fn simple_ring_buffer_commit(cpu_buffer: *mut simple_rb_per_cpu) {
    local_set(&mut (*(*(*cpu_buffer).tail_page).page).commit, (*cpu_buffer).tail_page.write);
    simple_rb_meta_inc(&mut (*(*cpu_buffer).meta).entries, 1);
    smp_store_release(&mut (*cpu_buffer).status, SIMPLE_RB_READY);
}

unsafe fn simple_rb_enable_tracing(cpu_buffer: *mut simple_rb_per_cpu, enable: bool) -> u32 {
    if enable { return cmpxchg(&mut (*cpu_buffer).status, SIMPLE_RB_UNAVAILABLE, SIMPLE_RB_READY); }
    let mut prev_status;
    loop {
        prev_status = cmpxchg_acquire(&mut (*cpu_buffer).status, SIMPLE_RB_READY, SIMPLE_RB_UNAVAILABLE);
        if prev_status != SIMPLE_RB_WRITING { return prev_status; }
    }
}

pub unsafe fn simple_ring_buffer_reset(cpu_buffer: *mut simple_rb_per_cpu) -> i32 {
    if !simple_rb_loaded(cpu_buffer) { return -ENODEV; }
    let prev_status = simple_rb_enable_tracing(cpu_buffer, false);
    let ret = simple_rb_find_head(cpu_buffer);
    if ret != 0 { return ret; }
    let head = (*cpu_buffer).head_page;
    (*cpu_buffer).tail_page = head;
    let mut bpage = head;
    loop { simple_bpage_reset(bpage); bpage = simple_bpage_next_page(bpage); if bpage == head { break; } }
    simple_bpage_reset((*cpu_buffer).reader_page);
    (*cpu_buffer).last_overrun = 0;
    (*cpu_buffer).write_stamp = 0;
    (*(*cpu_buffer).meta).reader.read = 0;
    (*(*cpu_buffer).meta).reader.lost_events = 0;
    (*(*cpu_buffer).meta).entries = 0;
    (*(*cpu_buffer).meta).overrun = 0;
    (*(*cpu_buffer).meta).read = 0;
    (*(*cpu_buffer).meta).pages_lost = 0;
    (*(*cpu_buffer).meta).pages_touched = 0;
    if prev_status == SIMPLE_RB_READY { simple_rb_enable_tracing(cpu_buffer, true); }
    0
}

pub unsafe fn simple_ring_buffer_init_mm(cpu_buffer: *mut simple_rb_per_cpu, bpages: *mut simple_buffer_page, desc: *const ring_buffer_desc, load_page: unsafe fn(usize) -> *mut core::ffi::c_void, unload_page: unsafe fn(*mut core::ffi::c_void)) -> i32 {
    if (*desc).nr_page_va < 3 { return -EINVAL; }
    core::ptr::write_bytes(cpu_buffer, 0, 1);
    (*cpu_buffer).meta = load_page((*desc).meta_va) as *mut simple_rb_meta;
    if (*cpu_buffer).meta.is_null() { return -EINVAL; }
    core::ptr::write_bytes((*cpu_buffer).meta, 0, 1);
    (*(*cpu_buffer).meta).meta_page_size = PAGE_SIZE;
    let page = load_page((*desc).page_va[0]);
    if page.is_null() { unload_page((*cpu_buffer).meta as *mut _); return -EINVAL; }
    simple_bpage_init(bpages, page);
    (*bpages).id = 0;
    (*cpu_buffer).nr_pages = 1;
    (*cpu_buffer).reader_page = bpages;
    (*cpu_buffer).tail_page = bpages.add(1);
    (*cpu_buffer).head_page = bpages.add(1);
    let mut bpage = bpages;
    for i in 1..(*desc).nr_page_va {
        let page = load_page((*desc).page_va[i]);
        if page.is_null() {
            for j in (0..i).rev() { unload_page((*bpages.add(j)).page as *mut _); }
            unload_page((*cpu_buffer).meta as *mut _);
            return -EINVAL;
        }
        bpage = bpage.add(1);
        simple_bpage_init(bpage, page);
        (*bpage).link.next = &mut (*bpage.add(1)).link;
        (*bpage).link.prev = &mut (*bpage.sub(1)).link;
        (*bpage).id = i;
        (*cpu_buffer).nr_pages = i + 1;
    }
    (*(*cpu_buffer).meta).nr_subbufs = (*cpu_buffer).nr_pages;
    (*bpage).link.next = &mut (*(*cpu_buffer).tail_page).link;
    (*(*cpu_buffer).tail_page).link.prev = &mut (*bpage).link;
    simple_bpage_set_head_link(bpage);
    (*cpu_buffer).bpages = bpages;
    0
}

unsafe fn __load_page(page: usize) -> *mut core::ffi::c_void { page as *mut _ }
unsafe fn __unload_page(_page: *mut core::ffi::c_void) {}

pub unsafe fn simple_ring_buffer_init(cpu_buffer: *mut simple_rb_per_cpu, bpages: *mut simple_buffer_page, desc: *const ring_buffer_desc) -> i32 {
    simple_ring_buffer_init_mm(cpu_buffer, bpages, desc, __load_page, __unload_page)
}

pub unsafe fn simple_ring_buffer_unload_mm(cpu_buffer: *mut simple_rb_per_cpu, unload_page: unsafe fn(*mut core::ffi::c_void)) {
    if !simple_rb_loaded(cpu_buffer) { return; }
    simple_rb_enable_tracing(cpu_buffer, false);
    unload_page((*cpu_buffer).meta as *mut _);
    for p in 0..(*cpu_buffer).nr_pages { unload_page((*(*cpu_buffer).bpages.add(p)).page as *mut _); }
    (*cpu_buffer).bpages = core::ptr::null_mut();
}

pub unsafe fn simple_ring_buffer_unload(cpu_buffer: *mut simple_rb_per_cpu) { simple_ring_buffer_unload_mm(cpu_buffer, __unload_page); }

pub unsafe fn simple_ring_buffer_enable_tracing(cpu_buffer: *mut simple_rb_per_cpu, enable: bool) -> i32 {
    if !simple_rb_loaded(cpu_buffer) { return -ENODEV; }
    simple_rb_enable_tracing(cpu_buffer, enable);
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
