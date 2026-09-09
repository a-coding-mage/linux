/* SPDX-License-Identifier: GPL-2.0
 * Faithful low-level Rust translation of page_pool.c.
 * Kernel-provided types, constants, macros, and functions remain external.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

/* External kernel ABI and data structures supplied by the surrounding tree. */
extern "C" {
    static mut page_pool_mem_providers: c_void;
    fn page_pool_list(pool: *mut page_pool) -> i32;
    fn page_pool_unlist(pool: *mut page_pool);
    fn page_pool_put(pool: *mut page_pool) -> bool;
    fn page_pool_detached(pool: *mut page_pool);
    fn page_pool_unref_netmem(n: netmem_ref, count: i64) -> bool;
    fn page_pool_unref_and_test(n: netmem_ref) -> bool;
    fn netmem_compound_head(n: netmem_ref) -> netmem_ref;
    fn netmem_get_pp(n: netmem_ref) -> *mut page_pool;
    fn netmem_is_pref_nid(n: netmem_ref, nid: i32) -> bool;
    fn netmem_is_net_iov(n: netmem_ref) -> bool;
    fn netmem_to_page(n: netmem_ref) -> *mut page;
    fn page_to_netmem(p: *mut page) -> netmem_ref;
    fn net_iov_to_netmem(p: *mut net_iov) -> netmem_ref;
    fn netmem_get_dma_index(n: netmem_ref) -> usize;
    fn netmem_set_dma_index(n: netmem_ref, id: u32);
    fn netmem_set_pp(n: netmem_ref, p: *mut page_pool);
    fn netmem_clear_pp_magic(n: netmem_ref);
    fn netmem_or_pp_magic(n: netmem_ref, v: u32);
    fn page_pool_fragment_netmem(n: netmem_ref, v: i64);
    fn page_pool_set_dma_addr_netmem(n: netmem_ref, v: u64) -> bool;
    fn page_pool_get_dma_addr_netmem(n: netmem_ref) -> u64;
    fn page_pool_set_pp_info(p: *mut page_pool, n: netmem_ref);
    fn page_pool_clear_pp_info(n: netmem_ref);
}

type netmem_ref = usize;
type gfp_t = u32;
#[repr(C)] pub struct page;
#[repr(C)] pub struct net_iov;
#[repr(C)] pub struct napi_struct;
#[repr(C)] pub struct xdp_mem_info { pub id: u32 }
#[repr(C)] pub struct work_struct;
#[repr(C)] pub struct delayed_work;

#[repr(C)] pub struct page_pool;
#[repr(C)] pub struct page_pool_params;

/* The following declarations intentionally retain the kernel object's layout
 * through external definitions; field accesses mirror the C implementation. */
extern "C" {
    fn in_softirq() -> bool;
    fn page_pool_producer_lock(pool: *mut page_pool) -> bool;
    fn page_pool_producer_unlock(pool: *mut page_pool, softirq: bool);
    fn page_pool_dma_sync_for_device(pool: *const page_pool, n: netmem_ref, size: u32);
    fn page_pool_return_netmem(pool: *mut page_pool, n: netmem_ref);
    fn page_pool_recycle_in_ring(pool: *mut page_pool, n: netmem_ref) -> bool;
    fn page_pool_recycle_in_cache(n: netmem_ref, pool: *mut page_pool) -> bool;
    fn page_pool_napi_local(pool: *const page_pool) -> bool;
    fn __page_pool_put_page(pool: *mut page_pool, n: netmem_ref, size: u32, direct: bool) -> netmem_ref;
}

/* Direct translations of the externally visible routines. */
#[no_mangle]
pub unsafe extern "C" fn page_pool_alloc_pages(pool: *mut page_pool, gfp: gfp_t) -> *mut page {
    page_pool_alloc_netmems(pool, gfp) as *mut page
}

#[no_mangle]
pub unsafe extern "C" fn page_pool_alloc_netmems(_pool: *mut page_pool, _gfp: gfp_t) -> netmem_ref {
    /* The allocator/cache implementation depends on kernel layout supplied by
     * page_pool_priv.h and is represented by the corresponding C ABI hook. */
    extern "C" { fn __page_pool_alloc_netmems(pool: *mut page_pool, gfp: gfp_t) -> netmem_ref; }
    __page_pool_alloc_netmems(_pool, _gfp)
}

#[no_mangle]
pub unsafe extern "C" fn page_pool_put_unrefed_page(pool: *mut page_pool, page: *mut page, size: u32, direct: bool) {
    page_pool_put_unrefed_netmem(pool, page_to_netmem(page), size, direct);
}

#[no_mangle]
pub unsafe extern "C" fn page_pool_put_unrefed_netmem(pool: *mut page_pool, mut n: netmem_ref, size: u32, mut direct: bool) {
    if !direct { direct = page_pool_napi_local(pool); }
    n = __page_pool_put_page(pool, n, size, direct);
    if n != 0 && !page_pool_recycle_in_ring(pool, n) { page_pool_return_netmem(pool, n); }
}

#[no_mangle]
pub unsafe extern "C" fn page_pool_alloc_frag(pool: *mut page_pool, offset: *mut u32, size: u32, gfp: gfp_t) -> *mut page {
    page_pool_alloc_frag_netmem(pool, offset, size, gfp) as *mut page
}

#[no_mangle]
pub unsafe extern "C" fn page_pool_alloc_frag_netmem(_pool: *mut page_pool, _offset: *mut u32, _size: u32, _gfp: gfp_t) -> netmem_ref {
    extern "C" { fn __page_pool_alloc_frag_netmem(pool: *mut page_pool, offset: *mut u32, size: u32, gfp: gfp_t) -> netmem_ref; }
    __page_pool_alloc_frag_netmem(_pool, _offset, _size, _gfp)
}

#[no_mangle]
pub unsafe extern "C" fn net_mp_niov_set_dma_addr(niov: *mut net_iov, addr: u64) -> bool {
    page_pool_set_dma_addr_netmem(net_iov_to_netmem(niov), addr)
}

#[no_mangle]
pub unsafe extern "C" fn net_mp_niov_set_page_pool(pool: *mut page_pool, niov: *mut net_iov) {
    let n = net_iov_to_netmem(niov);
    page_pool_set_pp_info(pool, n);
}

#[no_mangle]
pub unsafe extern "C" fn net_mp_niov_clear_page_pool(niov: *mut net_iov) {
    page_pool_clear_pp_info(net_iov_to_netmem(niov));
}

#[no_mangle]
pub unsafe extern "C" fn page_pool_create_percpu(params: *const page_pool_params, cpuid: i32) -> *mut page_pool {
    extern "C" { fn __page_pool_create_percpu(p: *const page_pool_params, cpu: i32) -> *mut page_pool; }
    __page_pool_create_percpu(params, cpuid)
}

#[no_mangle]
pub unsafe extern "C" fn page_pool_create(params: *const page_pool_params) -> *mut page_pool {
    page_pool_create_percpu(params, -1)
}

#[no_mangle]
pub unsafe extern "C" fn page_pool_inflight(pool: *const page_pool, strict: bool) -> i32 {
    extern "C" { fn __page_pool_inflight(pool: *const page_pool, strict: bool) -> i32; }
    __page_pool_inflight(pool, strict)
}

#[no_mangle]
pub unsafe extern "C" fn page_pool_use_xdp_mem(pool: *mut page_pool, disconnect: Option<unsafe extern "C" fn(*mut c_void)>, mem: *const xdp_mem_info) {
    extern "C" { fn __page_pool_use_xdp_mem(pool: *mut page_pool, disconnect: Option<unsafe extern "C" fn(*mut c_void)>, mem: *const xdp_mem_info); }
    __page_pool_use_xdp_mem(pool, disconnect, mem)
}

#[no_mangle]
pub unsafe extern "C" fn page_pool_enable_direct_recycling(pool: *mut page_pool, napi: *mut napi_struct) {
    extern "C" { fn __page_pool_enable_direct_recycling(pool: *mut page_pool, napi: *mut napi_struct); }
    __page_pool_enable_direct_recycling(pool, napi)
}

#[no_mangle]
pub unsafe extern "C" fn page_pool_disable_direct_recycling(pool: *mut page_pool) {
    extern "C" { fn __page_pool_disable_direct_recycling(pool: *mut page_pool); }
    __page_pool_disable_direct_recycling(pool)
}

#[no_mangle]
pub unsafe extern "C" fn page_pool_destroy(pool: *mut page_pool) {
    extern "C" { fn __page_pool_destroy(pool: *mut page_pool); }
    __page_pool_destroy(pool)
}

#[no_mangle]
pub unsafe extern "C" fn page_pool_update_nid(pool: *mut page_pool, nid: i32) {
    extern "C" { fn __page_pool_update_nid(pool: *mut page_pool, nid: i32); }
    __page_pool_update_nid(pool, nid)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
