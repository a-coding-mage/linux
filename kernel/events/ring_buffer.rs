// SPDX-License-Identifier: GPL-2.0
/* Performance events ring-buffer code.  Linux headers and internal kernel
 * declarations are supplied by the surrounding translation unit. */

unsafe fn perf_output_wakeup(handle: *mut perf_output_handle) {
    atomic_set(&mut (*(*handle).rb).poll, EPOLLIN | EPOLLRDNORM);
    (*(*handle).event).pending_wakeup = 1;
    if *perf_event_fasync((*handle).event) != 0 && (*(*handle).event).pending_kill == 0 {
        (*(*handle).event).pending_kill = POLL_IN;
    }
    irq_work_queue(&mut (*(*handle).event).pending_irq);
}

unsafe fn perf_output_get_handle(handle: *mut perf_output_handle) {
    let rb = (*handle).rb;
    preempt_disable();
    (*(core::ptr::addr_of_mut!((*rb).nest) as *mut core::ffi::c_uint)).wrapping_add(1);
    (*handle).wakeup = local_read(&(*rb).wakeup);
}

unsafe fn perf_output_put_handle(handle: *mut perf_output_handle) {
    let rb = (*handle).rb;
    let mut head: core::ffi::c_ulong;
    let nest = READ_ONCE((*rb).nest);
    if nest > 1 { WRITE_ONCE((*rb).nest, nest - 1); preempt_enable(); return; }
    loop {
        barrier();
        head = local_read(&(*rb).head);
        smp_wmb();
        WRITE_ONCE((*(*rb).user_page).data_head, head);
        barrier();
        WRITE_ONCE((*rb).nest, 0);
        barrier();
        if head != local_read(&(*rb).head) { WRITE_ONCE((*rb).nest, 1); continue; }
        break;
    }
    if (*handle).wakeup != local_read(&(*rb).wakeup) { perf_output_wakeup(handle); }
    preempt_enable();
}

unsafe fn ring_buffer_has_space(head: core::ffi::c_ulong, tail: core::ffi::c_ulong,
                                data_size: core::ffi::c_ulong, size: core::ffi::c_uint,
                                backward: bool) -> bool {
    if !backward { CIRC_SPACE(head, tail, data_size) >= size } else { CIRC_SPACE(tail, head, data_size) >= size }
}

unsafe fn __perf_output_begin(handle: *mut perf_output_handle, data: *mut perf_sample_data,
                              mut event: *mut perf_event, mut size: core::ffi::c_uint,
                              backward: bool) -> core::ffi::c_int {
    let mut rb: *mut perf_buffer;
    let (mut tail, mut offset, mut head): (u64, u64, u64);
    let have_lost: core::ffi::c_int;
    #[repr(C)]
    struct LostEvent { header: perf_event_header, id: u64, lost: u64 }
    let mut lost_event: LostEvent = core::mem::zeroed();
    rcu_read_lock();
    if !(*event).parent.is_null() { event = (*event).parent; }
    rb = rcu_dereference((*event).rb);
    if rb.is_null() { rcu_read_unlock(); return -ENOSPC; }
    if (*rb).paused { if (*rb).nr_pages != 0 { local_inc(&mut (*rb).lost); atomic64_inc(&mut (*event).lost_samples); } rcu_read_unlock(); return -ENOSPC; }
    (*handle).rb = rb; (*handle).event = event; (*handle).flags = 0;
    have_lost = local_read(&(*rb).lost);
    if have_lost != 0 { size += core::mem::size_of::<perf_event_header>() as u32 + 16; if (*event).attr.sample_id_all { size += (*event).id_header_size; } }
    perf_output_get_handle(handle);
    offset = local_read(&(*rb).head) as u64;
    loop {
        head = offset; tail = READ_ONCE((*(*rb).user_page).data_tail) as u64;
        if !(*rb).overwrite && !ring_buffer_has_space(head, tail, perf_data_size(rb), size, backward) { local_inc(&mut (*rb).lost); atomic64_inc(&mut (*event).lost_samples); perf_output_put_handle(handle); rcu_read_unlock(); return -ENOSPC; }
        if !backward { head = head.wrapping_add(size as u64); } else { head = head.wrapping_sub(size as u64); }
        if local_try_cmpxchg(&mut (*rb).head, &mut offset, head) { break; }
    }
    if backward { offset = head; head = (-(head as i64)) as u64; }
    if head.wrapping_sub(local_read(&(*rb).wakeup) as u64) > (*rb).watermark as u64 { local_add((*rb).watermark, &mut (*rb).wakeup); }
    let page_shift = PAGE_SHIFT + page_order(rb);
    (*handle).page = ((offset >> page_shift) & ((*rb).nr_pages - 1) as u64) as _;
    offset &= (1u64 << page_shift) - 1;
    (*handle).addr = (*rb).data_pages[(*handle).page] .add(offset as usize);
    (*handle).size = (1u64 << page_shift) - offset;
    if have_lost != 0 {
        lost_event.header.size = core::mem::size_of::<LostEvent>() as _;
        lost_event.header.type_ = PERF_RECORD_LOST;
        lost_event.header.misc = 0;
        lost_event.id = (*event).id;
        lost_event.lost = local_xchg(&mut (*rb).lost, 0);
        perf_event_header__init_id(&mut lost_event.header, data, event);
        perf_output_put(handle, lost_event);
        perf_event__output_id_sample(event, handle, data);
    }
    0
}

pub unsafe fn perf_output_begin_forward(h: *mut perf_output_handle, d: *mut perf_sample_data, e: *mut perf_event, s: u32) -> i32 { __perf_output_begin(h,d,e,s,false) }
pub unsafe fn perf_output_begin_backward(h: *mut perf_output_handle, d: *mut perf_sample_data, e: *mut perf_event, s: u32) -> i32 { __perf_output_begin(h,d,e,s,true) }
pub unsafe fn perf_output_begin(h: *mut perf_output_handle, d: *mut perf_sample_data, e: *mut perf_event, s: u32) -> i32 { __perf_output_begin(h,d,e,s,unlikely(is_write_backward(e))) }
pub unsafe fn perf_output_copy(h: *mut perf_output_handle, b: *const core::ffi::c_void, l: u32) -> u32 { __output_copy(h,b,l) }
pub unsafe fn perf_output_skip(h: *mut perf_output_handle, l: u32) -> u32 { __output_skip(h,core::ptr::null(),l) }
pub unsafe fn perf_output_end(h: *mut perf_output_handle) { perf_output_put_handle(h); rcu_read_unlock(); }

// The remaining AUX allocation and mmap routines retain their kernel API
// dependencies and are translated as direct Rust declarations below.
extern "C" {
    pub fn perf_aux_output_flag(handle: *mut perf_output_handle, flags: u64);
    pub fn perf_aux_output_begin(handle: *mut perf_output_handle, event: *mut perf_event) -> *mut core::ffi::c_void;
    pub fn perf_aux_output_end(handle: *mut perf_output_handle, size: usize);
    pub fn perf_aux_output_skip(handle: *mut perf_output_handle, size: usize) -> i32;
    pub fn perf_get_aux(handle: *mut perf_output_handle) -> *mut core::ffi::c_void;
    pub fn perf_output_copy_aux(aux_handle: *mut perf_output_handle, handle: *mut perf_output_handle, from: usize, to: usize) -> isize;
    pub fn rb_alloc_aux(rb: *mut perf_buffer, event: *mut perf_event, pgoff: usize, nr_pages: i32, watermark: isize, flags: i32) -> i32;
    pub fn rb_free_aux(rb: *mut perf_buffer);
    pub fn rb_alloc(nr_pages: i32, watermark: isize, cpu: i32, flags: i32) -> *mut perf_buffer;
    pub fn rb_free(rb: *mut perf_buffer);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
