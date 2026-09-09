/* SPDX-License-Identifier: GPL-2.0 */

use core::ffi::c_void;

pub const RING_BUFFER_WRITABLE: u32 = 0x01;

#[repr(C)]
pub struct perf_buffer {
    pub refcount: refcount_t,
    pub rcu_head: rcu_head,
    #[cfg(feature = "CONFIG_PERF_USE_VMALLOC")]
    pub work: work_struct,
    #[cfg(feature = "CONFIG_PERF_USE_VMALLOC")]
    pub page_order: i32,
    pub nr_pages: i32,
    pub overwrite: i32,
    pub paused: i32,
    pub poll: atomic_t,
    pub head: local_t,
    pub nest: u32,
    pub events: local_t,
    pub wakeup: local_t,
    pub lost: local_t,
    pub watermark: i64,
    pub aux_watermark: i64,
    pub event_lock: spinlock_t,
    pub event_list: list_head,
    pub mmap_count: refcount_t,
    pub mmap_locked: usize,
    pub mmap_user: *mut user_struct,
    pub aux_mutex: mutex,
    pub aux_head: i64,
    pub aux_nest: u32,
    pub aux_wakeup: i64,
    pub aux_pgoff: usize,
    pub aux_nr_pages: i32,
    pub aux_overwrite: i32,
    pub aux_mmap_count: refcount_t,
    pub aux_mmap_locked: usize,
    pub free_aux: Option<unsafe extern "C" fn(*mut c_void)>,
    pub aux_refcount: refcount_t,
    pub aux_in_sampling: i32,
    pub aux_in_pause_resume: i32,
    pub aux_pages: *mut *mut c_void,
    pub aux_priv: *mut c_void,
    pub user_page: *mut perf_event_mmap_page,
    pub data_pages: [*mut c_void; 0],
}

extern "C" {
    pub fn rb_free(rb: *mut perf_buffer);
    pub fn free_uid(user: *mut user_struct);
    pub fn perf_event_wakeup(event: *mut perf_event);
    pub fn rb_alloc_aux(rb: *mut perf_buffer, event: *mut perf_event, pgoff: pgoff_t,
                        nr_pages: i32, watermark: i64, flags: i32) -> i32;
    pub fn rb_free_aux(rb: *mut perf_buffer);
    pub fn ring_buffer_get(event: *mut perf_event) -> *mut perf_buffer;
    pub fn ring_buffer_put(rb: *mut perf_buffer);
    pub fn perf_event_aux_event(event: *mut perf_event, head: usize, size: usize, flags: u64);
    pub fn perf_mmap_to_page(rb: *mut perf_buffer, pgoff: usize) -> *mut page;
    pub fn memcpy(dst: *mut c_void, src: *const c_void, n: usize);
    pub fn pagefault_disable();
    pub fn pagefault_enable();
    pub fn __copy_from_user_inatomic(dst: *mut c_void, src: *const c_void, n: usize) -> usize;
    pub fn interrupt_context_level() -> u8;
    pub fn barrier();
}

pub fn rb_free_rcu(rcu_head: *mut rcu_head) {
    unsafe {
        let rb = (rcu_head as *mut u8).sub(core::mem::offset_of!(perf_buffer, rcu_head))
            as *mut perf_buffer;
        free_uid((*rb).mmap_user);
        rb_free(rb);
    }
}

pub unsafe fn rb_toggle_paused(rb: *mut perf_buffer, pause: bool) {
    (*rb).paused = if !pause && (*rb).nr_pages != 0 { 0 } else { 1 };
}

pub unsafe fn rb_has_aux(rb: *mut perf_buffer) -> bool { (*rb).aux_nr_pages != 0 }

#[cfg(feature = "CONFIG_PERF_USE_VMALLOC")]
pub unsafe fn page_order(rb: *mut perf_buffer) -> i32 { (*rb).page_order }

#[cfg(not(feature = "CONFIG_PERF_USE_VMALLOC"))]
pub unsafe fn page_order(_rb: *mut perf_buffer) -> i32 { 0 }

pub unsafe fn data_page_nr(rb: *mut perf_buffer) -> i32 {
    (*rb).nr_pages << page_order(rb)
}

pub unsafe fn perf_data_size(rb: *mut perf_buffer) -> usize {
    ((*rb).nr_pages << (PAGE_SHIFT + page_order(rb))) as usize
}

pub unsafe fn perf_aux_size(rb: *mut perf_buffer) -> usize {
    ((*rb).aux_nr_pages as usize) << PAGE_SHIFT
}

/* The C __DEFINE_OUTPUT_COPY_BODY/DEFINE_OUTPUT_COPY macros are represented by this helper. */
unsafe fn output_copy_body<F>(handle: *mut perf_output_handle, buf: *const c_void,
                             mut len: usize, advance_buf: bool, copy_func: F) -> usize
where F: Fn(*mut c_void, *const c_void, usize) -> usize {
    let mut written;
    loop {
        let size = core::cmp::min((*handle).size, len);
        written = copy_func((*handle).addr, buf, size);
        written = size - written;
        len -= written;
        (*handle).addr = (*handle).addr.add(written);
        if advance_buf { buf = buf.add(written); }
        (*handle).size -= written;
        if (*handle).size == 0 {
            let rb = (*handle).rb;
            (*handle).page += 1;
            (*handle).page &= ((*rb).nr_pages - 1) as usize;
            (*handle).addr = *(*rb).data_pages.as_ptr().add((*handle).page) as *mut c_void;
            (*handle).size = PAGE_SIZE << page_order(rb);
        }
        if len == 0 || written != size { break; }
    }
    len
}

pub unsafe fn memcpy_common(dst: *mut c_void, src: *const c_void, n: usize) -> usize {
    memcpy(dst, src, n); 0
}

pub unsafe fn __output_copy(handle: *mut perf_output_handle, buf: *const c_void, len: usize) -> usize {
    output_copy_body(handle, buf, len, true, memcpy_common)
}

pub unsafe fn memcpy_skip(_dst: *mut c_void, _src: *const c_void, _n: usize) -> usize { 0 }

pub unsafe fn __output_skip(handle: *mut perf_output_handle, buf: *const c_void, len: usize) -> usize {
    output_copy_body(handle, buf, len, true, memcpy_skip)
}

pub unsafe fn arch_perf_out_copy_user(dst: *mut c_void, src: *const c_void, n: usize) -> usize {
    pagefault_disable();
    let ret = __copy_from_user_inatomic(dst, src, n);
    pagefault_enable();
    ret
}

pub unsafe fn __output_copy_user(handle: *mut perf_output_handle, buf: *const c_void, len: usize) -> usize {
    output_copy_body(handle, buf, len, true, arch_perf_out_copy_user)
}

pub unsafe fn __output_custom(handle: *mut perf_output_handle, copy_func: perf_copy_f,
                              buf: *const c_void, len: usize) -> usize {
    let orig_len = len;
    output_copy_body(handle, buf, len, false, |dst, src, n| {
        copy_func(dst, src, orig_len - n, n)
    })
}

pub unsafe fn get_recursion_context(recursion: *mut u8) -> i32 {
    let rctx = interrupt_context_level();
    if *recursion.add(rctx as usize) != 0 { return -1; }
    *recursion.add(rctx as usize) += 1;
    barrier();
    rctx as i32
}

pub unsafe fn put_recursion_context(recursion: *mut u8, rctx: u8) {
    barrier();
    *recursion.add(rctx as usize) -= 1;
}

#[cfg(feature = "CONFIG_HAVE_PERF_USER_STACK_DUMP")]
pub fn arch_perf_have_user_stack_dump() -> bool { true }

#[cfg(not(feature = "CONFIG_HAVE_PERF_USER_STACK_DUMP"))]
pub fn arch_perf_have_user_stack_dump() -> bool { false }

#[cfg(feature = "CONFIG_HAVE_PERF_USER_STACK_DUMP")]
pub unsafe fn perf_user_stack_pointer(regs: *mut c_void) -> usize { user_stack_pointer(regs) }

#[cfg(not(feature = "CONFIG_HAVE_PERF_USER_STACK_DUMP"))]
pub unsafe fn perf_user_stack_pointer(_regs: *mut c_void) -> usize { 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
