/* SPDX-License-Identifier: GPL-2.0 */
/* Translation of linux/u64_stats_sync.h. */

#[repr(C)]
pub struct u64_stats_sync {
    #[cfg(target_pointer_width = "32")]
    pub seq: seqcount_t,
}

#[cfg(target_pointer_width = "64")]
#[repr(C)]
pub struct u64_stats_t {
    pub v: local64_t,
}

#[cfg(target_pointer_width = "32")]
#[repr(C)]
pub struct u64_stats_t {
    pub v: u64,
}

#[cfg(target_pointer_width = "64")]
#[inline]
pub unsafe fn u64_stats_read(p: *const u64_stats_t) -> u64 {
    local64_read(&(*p).v)
}

#[cfg(target_pointer_width = "32")]
#[inline]
pub unsafe fn u64_stats_read(p: *const u64_stats_t) -> u64 {
    (*p).v
}

#[cfg(target_pointer_width = "64")]
#[inline]
pub unsafe fn u64_stats_copy(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void, len: usize) -> *mut core::ffi::c_void {
    // BUILD_BUG_ON(len % size_of::<u64_stats_t>());
    for i in 0..(len / core::mem::size_of::<u64_stats_t>()) {
        *((dst as *mut u64).add(i)) = local64_read(&(*(src as *const local64_t).add(i)));
    }
    dst
}

#[cfg(target_pointer_width = "32")]
#[inline]
pub unsafe fn u64_stats_copy(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void, len: usize) -> *mut core::ffi::c_void {
    // BUILD_BUG_ON(len % size_of::<u64_stats_t>());
    core::ptr::copy_nonoverlapping(src as *const u8, dst as *mut u8, len);
    dst
}

#[cfg(target_pointer_width = "64")]
#[inline]
pub unsafe fn u64_stats_set(p: *mut u64_stats_t, val: u64) { local64_set(&mut (*p).v, val); }
#[cfg(target_pointer_width = "32")]
#[inline]
pub unsafe fn u64_stats_set(p: *mut u64_stats_t, val: u64) { (*p).v = val; }

#[cfg(target_pointer_width = "64")]
#[inline]
pub unsafe fn u64_stats_add(p: *mut u64_stats_t, val: libc::c_ulong) { local64_add(val, &mut (*p).v); }
#[cfg(target_pointer_width = "32")]
#[inline]
pub unsafe fn u64_stats_add(p: *mut u64_stats_t, val: libc::c_ulong) { (*p).v = (*p).v.wrapping_add(val as u64); }

#[cfg(target_pointer_width = "64")]
#[inline]
pub unsafe fn u64_stats_sub(p: *mut u64_stats_t, val: i64) { local64_sub(val, &mut (*p).v); }
#[cfg(target_pointer_width = "32")]
#[inline]
pub unsafe fn u64_stats_sub(p: *mut u64_stats_t, val: i64) { (*p).v = (*p).v.wrapping_sub(val as u64); }

#[cfg(target_pointer_width = "64")]
#[inline]
pub unsafe fn u64_stats_inc(p: *mut u64_stats_t) { local64_inc(&mut (*p).v); }
#[cfg(target_pointer_width = "32")]
#[inline]
pub unsafe fn u64_stats_inc(p: *mut u64_stats_t) { (*p).v = (*p).v.wrapping_add(1); }

#[cfg(target_pointer_width = "64")]
#[inline] pub unsafe fn u64_stats_init(_syncp: *mut u64_stats_sync) {}
#[cfg(target_pointer_width = "64")]
#[inline] pub unsafe fn __u64_stats_update_begin(_syncp: *mut u64_stats_sync) {}
#[cfg(target_pointer_width = "64")]
#[inline] pub unsafe fn __u64_stats_update_end(_syncp: *mut u64_stats_sync) {}
#[cfg(target_pointer_width = "64")]
#[inline] pub unsafe fn __u64_stats_irqsave() -> libc::c_ulong { 0 }
#[cfg(target_pointer_width = "64")]
#[inline] pub unsafe fn __u64_stats_irqrestore(_flags: libc::c_ulong) {}
#[cfg(target_pointer_width = "64")]
#[inline] pub unsafe fn __u64_stats_fetch_begin(_syncp: *const u64_stats_sync) -> u32 { 0 }
#[cfg(target_pointer_width = "64")]
#[inline] pub unsafe fn __u64_stats_fetch_retry(_syncp: *const u64_stats_sync, _start: u32) -> bool { false }

#[cfg(target_pointer_width = "32")]
#[inline] pub unsafe fn u64_stats_init(syncp: *mut u64_stats_sync) { seqcount_init(&mut (*syncp).seq); }
#[cfg(target_pointer_width = "32")]
#[inline] pub unsafe fn __u64_stats_update_begin(syncp: *mut u64_stats_sync) { preempt_disable_nested(); write_seqcount_begin(&mut (*syncp).seq); }
#[cfg(target_pointer_width = "32")]
#[inline] pub unsafe fn __u64_stats_update_end(syncp: *mut u64_stats_sync) { write_seqcount_end(&mut (*syncp).seq); preempt_enable_nested(); }
#[cfg(target_pointer_width = "32")]
#[inline] pub unsafe fn __u64_stats_irqsave() -> libc::c_ulong { let mut flags = 0; local_irq_save(&mut flags); flags }
#[cfg(target_pointer_width = "32")]
#[inline] pub unsafe fn __u64_stats_irqrestore(flags: libc::c_ulong) { local_irq_restore(flags); }
#[cfg(target_pointer_width = "32")]
#[inline] pub unsafe fn __u64_stats_fetch_begin(syncp: *const u64_stats_sync) -> u32 { read_seqcount_begin(&(*syncp).seq) }
#[cfg(target_pointer_width = "32")]
#[inline] pub unsafe fn __u64_stats_fetch_retry(syncp: *const u64_stats_sync, start: u32) -> bool { read_seqcount_retry(&(*syncp).seq, start) }

#[inline] pub unsafe fn u64_stats_update_begin(syncp: *mut u64_stats_sync) { __u64_stats_update_begin(syncp); }
#[inline] pub unsafe fn u64_stats_update_end(syncp: *mut u64_stats_sync) { __u64_stats_update_end(syncp); }
#[inline] pub unsafe fn u64_stats_update_begin_irqsave(syncp: *mut u64_stats_sync) -> libc::c_ulong { let flags = __u64_stats_irqsave(); __u64_stats_update_begin(syncp); flags }
#[inline] pub unsafe fn u64_stats_update_end_irqrestore(syncp: *mut u64_stats_sync, flags: libc::c_ulong) { __u64_stats_update_end(syncp); __u64_stats_irqrestore(flags); }
#[inline] pub unsafe fn u64_stats_fetch_begin(syncp: *const u64_stats_sync) -> u32 { __u64_stats_fetch_begin(syncp) }
#[inline] pub unsafe fn u64_stats_fetch_retry(syncp: *const u64_stats_sync, start: u32) -> bool { __u64_stats_fetch_retry(syncp, start) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
