/* SPDX-License-Identifier: GPL-2.0 */

// Translated from net/dst_ops.h.  Linux-provided types and functions are
// intentionally referenced here as external dependencies.

#[repr(C)]
pub struct dst_entry;
#[repr(C)]
pub struct kmem_cachep;
#[repr(C)]
pub struct net_device;
#[repr(C)]
pub struct sk_buff;
#[repr(C)]
pub struct sock;
#[repr(C)]
pub struct net;
#[repr(C)]
pub struct neighbour;
#[repr(C)]
pub struct kmem_cache;
#[repr(C)]
pub struct percpu_counter;

#[repr(C)]
pub struct dst_ops {
    pub family: u16,
    pub gc_thresh: u32,

    pub gc: Option<unsafe extern "C" fn(ops: *mut dst_ops)>,
    pub check: Option<unsafe extern "C" fn(*mut dst_entry, u32) -> *mut dst_entry>,
    pub default_advmss: Option<unsafe extern "C" fn(*const dst_entry) -> u32>,
    pub mtu: Option<unsafe extern "C" fn(*const dst_entry) -> u32>,
    pub cow_metrics: Option<unsafe extern "C" fn(*mut dst_entry, usize) -> *mut u32>,
    pub destroy: Option<unsafe extern "C" fn(*mut dst_entry)>,
    pub ifdown: Option<unsafe extern "C" fn(*mut dst_entry, *mut net_device)>,
    pub negative_advice: Option<unsafe extern "C" fn(*mut sock, *mut dst_entry)>,
    pub link_failure: Option<unsafe extern "C" fn(*mut sk_buff)>,
    pub update_pmtu: Option<unsafe extern "C" fn(
        *mut dst_entry,
        *mut sock,
        *mut sk_buff,
        u32,
        bool,
    )>,
    pub redirect: Option<unsafe extern "C" fn(*mut dst_entry, *mut sock, *mut sk_buff)>,
    pub local_out: Option<unsafe extern "C" fn(*mut net, *mut sock, *mut sk_buff) -> i32>,
    pub neigh_lookup: Option<unsafe extern "C" fn(
        *const dst_entry,
        *mut sk_buff,
        *const core::ffi::c_void,
    ) -> *mut neighbour>,
    pub confirm_neigh: Option<unsafe extern "C" fn(*const dst_entry, *const core::ffi::c_void)>,

    pub kmem_cachep: *mut kmem_cache,
    // ____cacheline_aligned_in_smp
    pub pcpuc_entries: percpu_counter,
}

pub const DST_PERCPU_COUNTER_BATCH: i32 = 32;

#[inline]
pub unsafe fn dst_entries_get_fast(dst: *mut dst_ops) -> i32 {
    percpu_counter_read_positive(&mut (*dst).pcpuc_entries)
}

#[inline]
pub unsafe fn dst_entries_get_slow(dst: *mut dst_ops) -> i32 {
    percpu_counter_sum_positive(&mut (*dst).pcpuc_entries)
}

#[inline]
pub unsafe fn dst_entries_add(dst: *mut dst_ops, val: i32) {
    percpu_counter_add_batch(
        &mut (*dst).pcpuc_entries,
        val,
        DST_PERCPU_COUNTER_BATCH,
    );
}

#[inline]
pub unsafe fn dst_entries_init(dst: *mut dst_ops) -> i32 {
    percpu_counter_init(&mut (*dst).pcpuc_entries, 0, GFP_KERNEL)
}

#[inline]
pub unsafe fn dst_entries_destroy(dst: *mut dst_ops) {
    percpu_counter_destroy(&mut (*dst).pcpuc_entries);
}

extern "C" {
    fn percpu_counter_read_positive(fbc: *mut percpu_counter) -> i32;
    fn percpu_counter_sum_positive(fbc: *mut percpu_counter) -> i32;
    fn percpu_counter_add_batch(fbc: *mut percpu_counter, amount: i32, batch: i32);
    fn percpu_counter_init(fbc: *mut percpu_counter, amount: i64, gfp: u32) -> i32;
    fn percpu_counter_destroy(fbc: *mut percpu_counter);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
