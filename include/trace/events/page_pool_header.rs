/* SPDX-License-Identifier: GPL-2.0 */

// Translated from trace/events/page_pool.h.
// The C tracepoint includes and tracepoint-generation macros are supplied by
// other parts of the kernel and are intentionally not implemented here.

use core::ffi::c_void;

// External kernel types and symbols supplied by the corresponding dependencies.
#[repr(C)]
pub struct page_pool {
    _private: [u8; 0],
}

pub type netmem_ref = usize;

extern "C" {
    pub fn netmem_pfn_trace(netmem: netmem_ref) -> libc::c_ulong;
}

// NET_IOV is supplied by net/page_pool/types.h.
// Build-time tracepoint configuration is preserved by these entry layouts.

#[repr(C)]
pub struct page_pool_release_entry {
    pub pool: *const page_pool,
    pub inflight: i32,
    pub hold: u32,
    pub release: u32,
    pub cnt: u64,
}

#[repr(C)]
pub struct page_pool_state_release_entry {
    pub pool: *const page_pool,
    pub netmem: libc::c_ulong,
    pub release: u32,
    pub pfn: libc::c_ulong,
}

#[repr(C)]
pub struct page_pool_state_hold_entry {
    pub pool: *const page_pool,
    pub netmem: libc::c_ulong,
    pub hold: u32,
    pub pfn: libc::c_ulong,
}

#[repr(C)]
pub struct page_pool_update_nid_entry {
    pub pool: *const page_pool,
    pub pool_nid: i32,
    pub new_nid: i32,
}

// TRACE_EVENT(page_pool_release)
// TP_PROTO(const struct page_pool *pool, s32 inflight, u32 hold, u32 release)
// TP_printk("page_pool=%p inflight=%d hold=%u release=%u cnt=%llu", ...)
#[inline]
pub unsafe fn page_pool_release_assign(
    entry: *mut page_pool_release_entry,
    pool: *const page_pool,
    inflight: i32,
    hold: u32,
    release: u32,
    destroy_cnt: u64,
) {
    (*entry).pool = pool;
    (*entry).inflight = inflight;
    (*entry).hold = hold;
    (*entry).release = release;
    (*entry).cnt = destroy_cnt;
}

// TRACE_EVENT(page_pool_state_release)
// TP_PROTO(const struct page_pool *pool, netmem_ref netmem, u32 release)
#[inline]
pub unsafe fn page_pool_state_release_assign(
    entry: *mut page_pool_state_release_entry,
    pool: *const page_pool,
    netmem: netmem_ref,
    release: u32,
) {
    (*entry).pool = pool;
    (*entry).netmem = netmem as libc::c_ulong;
    (*entry).release = release;
    (*entry).pfn = netmem_pfn_trace(netmem);
}

// TRACE_EVENT(page_pool_state_hold)
// TP_PROTO(const struct page_pool *pool, netmem_ref netmem, u32 hold)
#[inline]
pub unsafe fn page_pool_state_hold_assign(
    entry: *mut page_pool_state_hold_entry,
    pool: *const page_pool,
    netmem: netmem_ref,
    hold: u32,
) {
    (*entry).pool = pool;
    (*entry).netmem = netmem as libc::c_ulong;
    (*entry).hold = hold;
    (*entry).pfn = netmem_pfn_trace(netmem);
}

// TRACE_EVENT(page_pool_update_nid)
// TP_PROTO(const struct page_pool *pool, int new_nid)
// pool->p.nid is supplied by net/page_pool/types.h.
#[inline]
pub unsafe fn page_pool_update_nid_assign(
    entry: *mut page_pool_update_nid_entry,
    pool: *const page_pool,
    pool_nid: i32,
    new_nid: i32,
) {
    (*entry).pool = pool;
    (*entry).pool_nid = pool_nid;
    (*entry).new_nid = new_nid;
}

// TP_printk formats:
// page_pool=%p netmem=%p is_net_iov=%lu pfn=0x%lx release=%u
// page_pool=%p netmem=%p is_net_iov=%lu, pfn=0x%lx hold=%u
// page_pool=%p pool_nid=%d new_nid=%d


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
